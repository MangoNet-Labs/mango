// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use crate::abi::EthToMgoTokenBridgeV1;
use crate::crypto::BridgeAuthorityPublicKeyBytes;
use crate::crypto::{
    BridgeAuthorityPublicKey, BridgeAuthorityRecoverableSignature, BridgeAuthoritySignInfo,
};
use crate::error::{BridgeError, BridgeResult};
use crate::events::EmittedMgoToEthTokenBridgeV1;
use ethers::types::Address as EthAddress;
use ethers::types::Log;
use ethers::types::H256;
pub use ethers::types::H256 as EthTransactionHash;
use fastcrypto::hash::{HashFunction, Keccak256};
use num_enum::TryFromPrimitive;
use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Deserialize, Serialize};
use shared_crypto::intent::IntentScope;
use std::collections::{BTreeMap, BTreeSet};
use mgo_types::base_types::MgoAddress;
use mgo_types::collection_types::{Bag, LinkedTable, LinkedTableNode, VecMap};
use mgo_types::committee::CommitteeTrait;
use mgo_types::committee::StakeUnit;
use mgo_types::digests::{Digest, TransactionDigest};
use mgo_types::dynamic_field::Field;
use mgo_types::error::MgoResult;
use mgo_types::message_envelope::{Envelope, Message, VerifiedEnvelope};
use mgo_types::{base_types::MGO_ADDRESS_LENGTH, committee::EpochId};

pub const BRIDGE_AUTHORITY_TOTAL_VOTING_POWER: u64 = 10000;

pub type BridgeInnerDynamicField = Field<u64, MoveTypeBridgeInner>;
pub type BridgeRecordDyanmicField = Field<
    MoveTypeBridgeMessageKey,
    LinkedTableNode<MoveTypeBridgeMessageKey, MoveTypeBridgeRecord>,
>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BridgeAuthority {
    pub pubkey: BridgeAuthorityPublicKey,
    pub voting_power: u64,
    pub base_url: String,
    pub is_blocklisted: bool,
}

impl BridgeAuthority {
    pub fn pubkey_bytes(&self) -> BridgeAuthorityPublicKeyBytes {
        BridgeAuthorityPublicKeyBytes::from(&self.pubkey)
    }
}

// A static Bridge committee implementation
#[derive(Debug, Clone)]
pub struct BridgeCommittee {
    members: BTreeMap<BridgeAuthorityPublicKeyBytes, BridgeAuthority>,
    total_blocklisted_stake: StakeUnit,
}

impl BridgeCommittee {
    pub fn new(members: Vec<BridgeAuthority>) -> BridgeResult<Self> {
        let mut members_map = BTreeMap::new();
        let mut total_stake = 0;
        let mut total_blocklisted_stake = 0;
        for member in members {
            let public_key = BridgeAuthorityPublicKeyBytes::from(&member.pubkey);
            if members_map.contains_key(&public_key) {
                return Err(BridgeError::InvalidBridgeCommittee(
                    "Duplicate BridgeAuthority Public key".into(),
                ));
            }
            // TODO: should we disallow identical network addresses?
            total_stake += member.voting_power;
            if member.is_blocklisted {
                total_blocklisted_stake += member.voting_power;
            }
            members_map.insert(public_key, member);
        }
        if total_stake != BRIDGE_AUTHORITY_TOTAL_VOTING_POWER {
            return Err(BridgeError::InvalidBridgeCommittee(
                "Total voting power does not equal to 10000".into(),
            ));
        }
        Ok(Self {
            members: members_map,
            total_blocklisted_stake,
        })
    }

    pub fn is_active_member(&self, member: &BridgeAuthorityPublicKeyBytes) -> bool {
        self.members.contains_key(member) && !self.members.get(member).unwrap().is_blocklisted
    }

    pub fn members(&self) -> &BTreeMap<BridgeAuthorityPublicKeyBytes, BridgeAuthority> {
        &self.members
    }

    pub fn member(&self, member: &BridgeAuthorityPublicKeyBytes) -> Option<&BridgeAuthority> {
        self.members.get(member)
    }

    pub fn total_blocklisted_stake(&self) -> StakeUnit {
        self.total_blocklisted_stake
    }
}

impl CommitteeTrait<BridgeAuthorityPublicKeyBytes> for BridgeCommittee {
    // Note:
    // 1. preference is not supported today.
    // 2. blocklisted members are always excluded.
    fn shuffle_by_stake_with_rng(
        &self,
        // preference is not supported today
        _preferences: Option<&BTreeSet<BridgeAuthorityPublicKeyBytes>>,
        // only attempt from these authorities.
        restrict_to: Option<&BTreeSet<BridgeAuthorityPublicKeyBytes>>,
        rng: &mut impl Rng,
    ) -> Vec<BridgeAuthorityPublicKeyBytes> {
        let candidates = self
            .members
            .iter()
            .filter_map(|(name, a)| {
                // Remove blocklisted members
                if a.is_blocklisted {
                    return None;
                }
                // exclude non-allowlisted members
                if let Some(restrict_to) = restrict_to {
                    match restrict_to.contains(name) {
                        true => Some((name.clone(), a.voting_power)),
                        false => None,
                    }
                } else {
                    Some((name.clone(), a.voting_power))
                }
            })
            .collect::<Vec<_>>();

        candidates
            .choose_multiple_weighted(rng, candidates.len(), |(_, weight)| *weight as f64)
            // Unwrap safe: it panics when the third parameter is larger than the size of the slice
            .unwrap()
            .map(|(name, _)| name)
            .cloned()
            .collect()
    }

    fn weight(&self, author: &BridgeAuthorityPublicKeyBytes) -> StakeUnit {
        self.members
            .get(author)
            .map(|a| a.voting_power)
            .unwrap_or(0)
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum BridgeActionType {
    TokenTransfer = 0,
    UpdateCommitteeBlocklist = 1,
    EmergencyButton = 2,
}

pub const MGO_TX_DIGEST_LENGTH: usize = 32;
pub const ETH_TX_HASH_LENGTH: usize = 32;

pub const BRIDGE_MESSAGE_PREFIX: &[u8] = b"MGO_BRIDGE_MESSAGE";

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy, TryFromPrimitive)]
#[repr(u8)]
pub enum BridgeChainId {
    MgoMainnet = 0,
    MgoTestnet = 1,
    MgoDevnet = 2,
    MgoLocalTest = 3,

    EthMainnet = 10,
    EthSepolia = 11,
    EthLocalTest = 12,
}

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    TryFromPrimitive,
    Hash,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum TokenId {
    Mgo = 0,
    BTC = 1,
    ETH = 2,
    USDC = 3,
    USDT = 4,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BridgeActionStatus {
    RecordNotFound,
    Pending,
    Approved,
    Claimed,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct MgoToEthBridgeAction {
    // Digest of the transaction where the event was emitted
    pub mgo_tx_digest: TransactionDigest,
    // The index of the event in the transaction
    pub mgo_tx_event_index: u16,
    pub mgo_bridge_event: EmittedMgoToEthTokenBridgeV1,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct EthToMgoBridgeAction {
    // Digest of the transaction where the event was emitted
    pub eth_tx_hash: EthTransactionHash,
    // The index of the event in the transaction
    pub eth_event_index: u16,
    pub eth_bridge_event: EthToMgoTokenBridgeV1,
}

/// The type of actions Bridge Committee verify and sign off to execution.
/// Its relationship with BridgeEvent is similar to the relationship between
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BridgeAction {
    /// Mgo to Eth bridge action
    MgoToEthBridgeAction(MgoToEthBridgeAction),
    /// Eth to mgo bridge action
    EthToMgoBridgeAction(EthToMgoBridgeAction),
    // TODO: add other bridge actions such as blocklist & emergency button
}

pub const TOKEN_TRANSFER_MESSAGE_VERSION: u8 = 1;

impl BridgeAction {
    /// Convert to message bytes that are verified in Move and Solidity
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        // Add prefix
        bytes.extend_from_slice(BRIDGE_MESSAGE_PREFIX);
        match self {
            BridgeAction::MgoToEthBridgeAction(a) => {
                let e = &a.mgo_bridge_event;
                // Add message type
                bytes.push(BridgeActionType::TokenTransfer as u8);
                // Add message version
                bytes.push(TOKEN_TRANSFER_MESSAGE_VERSION);
                // Add nonce
                bytes.extend_from_slice(&e.nonce.to_be_bytes());
                // Add source chain id
                bytes.push(e.mgo_chain_id as u8);

                // Add source address length
                bytes.push(MGO_ADDRESS_LENGTH as u8);
                // Add source address
                bytes.extend_from_slice(&e.mgo_address.to_vec());
                // Add dest chain id
                bytes.push(e.eth_chain_id as u8);
                // Add dest address length
                bytes.push(EthAddress::len_bytes() as u8);
                // Add dest address
                bytes.extend_from_slice(e.eth_address.as_bytes());

                // Add token id
                bytes.push(e.token_id as u8);

                // Add token amount
                bytes.extend_from_slice(&e.amount.to_be_bytes());
            }
            BridgeAction::EthToMgoBridgeAction(a) => {
                let e = &a.eth_bridge_event;
                // Add message type
                bytes.push(BridgeActionType::TokenTransfer as u8);
                // Add message version
                bytes.push(TOKEN_TRANSFER_MESSAGE_VERSION);
                // Add nonce
                bytes.extend_from_slice(&e.nonce.to_be_bytes());
                // Add source chain id
                bytes.push(e.eth_chain_id as u8);

                // Add source address length
                bytes.push(EthAddress::len_bytes() as u8);
                // Add source address
                bytes.extend_from_slice(e.eth_address.as_bytes());
                // Add dest chain id
                bytes.push(e.mgo_chain_id as u8);
                // Add dest address length
                bytes.push(MGO_ADDRESS_LENGTH as u8);
                // Add dest address
                bytes.extend_from_slice(&e.mgo_address.to_vec());

                // Add token id
                bytes.push(e.token_id as u8);

                // Add token amount
                bytes.extend_from_slice(&e.amount.to_be_bytes());
            } // TODO add formats for other events
        }
        bytes
    }

    // Digest of BridgeAction (with Keccak256 hasher)
    pub fn digest(&self) -> BridgeActionDigest {
        let mut hasher = Keccak256::default();
        hasher.update(&self.to_bytes());
        BridgeActionDigest::new(hasher.finalize().into())
    }

    pub fn source_chain_id(&self) -> BridgeChainId {
        match self {
            BridgeAction::MgoToEthBridgeAction(a) => a.mgo_bridge_event.mgo_chain_id,
            BridgeAction::EthToMgoBridgeAction(a) => a.eth_bridge_event.eth_chain_id,
        }
    }

    // Also called `message_type`
    pub fn action_type(&self) -> BridgeActionType {
        match self {
            BridgeAction::MgoToEthBridgeAction(_) => BridgeActionType::TokenTransfer,
            BridgeAction::EthToMgoBridgeAction(_) => BridgeActionType::TokenTransfer,
        }
    }

    // Also called `nonce`
    pub fn seq_number(&self) -> u64 {
        match self {
            BridgeAction::MgoToEthBridgeAction(a) => a.mgo_bridge_event.nonce,
            BridgeAction::EthToMgoBridgeAction(a) => a.eth_bridge_event.nonce,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct BridgeActionDigest(Digest);

impl BridgeActionDigest {
    pub const fn new(digest: [u8; 32]) -> Self {
        Self(Digest::new(digest))
    }
}

#[derive(Debug, Clone)]
pub struct BridgeCommitteeValiditySignInfo {
    pub signatures: BTreeMap<BridgeAuthorityPublicKeyBytes, BridgeAuthorityRecoverableSignature>,
}

pub type SignedBridgeAction = Envelope<BridgeAction, BridgeAuthoritySignInfo>;
pub type VerifiedSignedBridgeAction = VerifiedEnvelope<BridgeAction, BridgeAuthoritySignInfo>;
pub type CertifiedBridgeAction = Envelope<BridgeAction, BridgeCommitteeValiditySignInfo>;
pub type VerifiedCertifiedBridgeAction =
    VerifiedEnvelope<BridgeAction, BridgeCommitteeValiditySignInfo>;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct BridgeEventDigest(Digest);

impl BridgeEventDigest {
    pub const fn new(digest: [u8; 32]) -> Self {
        Self(Digest::new(digest))
    }
}

impl Message for BridgeAction {
    type DigestType = BridgeEventDigest;

    // this is not encoded in message today
    const SCOPE: IntentScope = IntentScope::BridgeEventUnused;

    // this is not used today
    fn digest(&self) -> Self::DigestType {
        unreachable!("BridgeEventDigest is not used today")
    }

    fn verify_user_input(&self) -> MgoResult {
        Ok(())
    }

    fn verify_epoch(&self, _epoch: EpochId) -> MgoResult {
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EthLog {
    pub block_number: u64,
    pub tx_hash: H256,
    pub log_index_in_tx: u16,
    // TODO: pull necessary fields from `Log`.
    pub log: Log,
}

/////////////////////////// Move Types Start //////////////////////////

/// Rust version of the Move bridge::BridgeInner type.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveTypeBridgeInner {
    pub bridge_version: u64,
    pub chain_id: u8,
    pub sequence_nums: VecMap<u8, u64>,
    pub committee: MoveTypeBridgeCommittee,
    pub treasury: MoveTypeBridgeTreasury,
    pub bridge_records: LinkedTable<MoveTypeBridgeMessageKey>,
    pub frozen: bool,
}

/// Rust version of the Move treasury::BridgeTreasury type.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveTypeBridgeTreasury {
    pub treasuries: Bag,
}

/// Rust version of the Move committee::BridgeCommittee type.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveTypeBridgeCommittee {
    pub members: VecMap<Vec<u8>, MoveTypeCommitteeMember>,
    pub thresholds: VecMap<u8, u64>,
}

/// Rust version of the Move committee::CommitteeMember type.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveTypeCommitteeMember {
    pub mgo_address: MgoAddress,
    pub bridge_pubkey_bytes: Vec<u8>,
    pub voting_power: u64,
    pub http_rest_url: Vec<u8>,
    pub blocklisted: bool,
}

/// Rust version of the Move message::BridgeMessageKey type.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveTypeBridgeMessageKey {
    pub source_chain: u8,
    pub message_type: u8,
    pub bridge_seq_num: u64,
}

/// Rust version of the Move message::BridgeMessage type.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveTypeBridgeMessage {
    pub message_type: u8,
    pub message_version: u8,
    pub seq_num: u64,
    pub source_chain: u8,
    pub payload: Vec<u8>,
}

/// Rust version of the Move message::BridgeMessage type.
#[derive(Debug, Serialize, Deserialize)]
pub struct MoveTypeBridgeRecord {
    pub message: MoveTypeBridgeMessage,
    pub verified_signatures: Option<Vec<Vec<u8>>>,
    pub claimed: bool,
}

/////////////////////////// Move Types End //////////////////////////

#[cfg(test)]
mod tests {
    use crate::{test_utils::get_test_authority_and_key, types::TokenId};
    use ethers::types::{Address as EthAddress, TxHash};
    use fastcrypto::encoding::Hex;
    use fastcrypto::hash::HashFunction;
    use fastcrypto::{encoding::Encoding, traits::KeyPair};
    use prometheus::Registry;
    use std::{collections::HashSet, str::FromStr};
    use mgo_types::{
        base_types::{MgoAddress, TransactionDigest},
        crypto::get_key_pair,
    };

    use super::*;

    #[test]
    fn test_bridge_message_encoding() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mango_metrics::init_metrics(&registry);
        let nonce = 54321u64;
        let mgo_tx_digest = TransactionDigest::random();
        let mgo_chain_id = BridgeChainId::MgoTestnet;
        let mgo_tx_event_index = 1u16;
        let eth_chain_id = BridgeChainId::EthSepolia;
        let mgo_address = MgoAddress::random_for_testing_only();
        let eth_address = EthAddress::random();
        let token_id = TokenId::USDC;
        let amount = 1_000_000;

        let mgo_bridge_event = EmittedMgoToEthTokenBridgeV1 {
            nonce,
            mgo_chain_id,
            eth_chain_id,
            mgo_address,
            eth_address,
            token_id,
            amount,
        };

        let encoded_bytes = BridgeAction::MgoToEthBridgeAction(MgoToEthBridgeAction {
            mgo_tx_digest,
            mgo_tx_event_index,
            mgo_bridge_event,
        })
        .to_bytes();

        // Construct the expected bytes
        let prefix_bytes = BRIDGE_MESSAGE_PREFIX.to_vec(); // len: 18
        let message_type = vec![BridgeActionType::TokenTransfer as u8]; // len: 1
        let message_version = vec![TOKEN_TRANSFER_MESSAGE_VERSION]; // len: 1
        let nonce_bytes = nonce.to_be_bytes().to_vec(); // len: 8
        let source_chain_id_bytes = vec![mgo_chain_id as u8]; // len: 1

        let mgo_address_length_bytes = vec![MGO_ADDRESS_LENGTH as u8]; // len: 1
        let mgo_address_bytes = mgo_address.to_vec(); // len: 32
        let dest_chain_id_bytes = vec![eth_chain_id as u8]; // len: 1
        let eth_address_length_bytes = vec![EthAddress::len_bytes() as u8]; // len: 1
        let eth_address_bytes = eth_address.as_bytes().to_vec(); // len: 20

        let token_id_bytes = vec![token_id as u8]; // len: 1
        let token_amount_bytes = amount.to_be_bytes().to_vec(); // len: 8

        let mut combined_bytes = Vec::new();
        combined_bytes.extend_from_slice(&prefix_bytes);
        combined_bytes.extend_from_slice(&message_type);
        combined_bytes.extend_from_slice(&message_version);
        combined_bytes.extend_from_slice(&nonce_bytes);
        combined_bytes.extend_from_slice(&source_chain_id_bytes);
        combined_bytes.extend_from_slice(&mgo_address_length_bytes);
        combined_bytes.extend_from_slice(&mgo_address_bytes);
        combined_bytes.extend_from_slice(&dest_chain_id_bytes);
        combined_bytes.extend_from_slice(&eth_address_length_bytes);
        combined_bytes.extend_from_slice(&eth_address_bytes);
        combined_bytes.extend_from_slice(&token_id_bytes);
        combined_bytes.extend_from_slice(&token_amount_bytes);

        assert_eq!(combined_bytes, encoded_bytes);

        // Assert fixed length
        // TODO: for each action type add a test to assert the length
        assert_eq!(
            combined_bytes.len(),
            18 + 1 + 1 + 8 + 1 + 1 + 32 + 1 + 20 + 1 + 1 + 8
        );
        Ok(())
    }

    #[test]
    fn test_bridge_message_encoding_regression_emitted_mgo_to_eth_token_bridge_v1(
    ) -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mango_metrics::init_metrics(&registry);
        let mgo_tx_digest = TransactionDigest::random();
        let mgo_tx_event_index = 1u16;

        let nonce = 10u64;
        let mgo_chain_id = BridgeChainId::MgoTestnet;
        let eth_chain_id = BridgeChainId::EthSepolia;
        let mgo_address = MgoAddress::from_str(
            "0x0000000000000000000000000000000000000000000000000000000000000064",
        )
        .unwrap();
        let eth_address =
            EthAddress::from_str("0x00000000000000000000000000000000000000c8").unwrap();
        let token_id = TokenId::USDC;
        let amount = 12345;

        let mgo_bridge_event = EmittedMgoToEthTokenBridgeV1 {
            nonce,
            mgo_chain_id,
            eth_chain_id,
            mgo_address,
            eth_address,
            token_id,
            amount,
        };
        let encoded_bytes = BridgeAction::MgoToEthBridgeAction(MgoToEthBridgeAction {
            mgo_tx_digest,
            mgo_tx_event_index,
            mgo_bridge_event,
        })
        .to_bytes();
        assert_eq!(
            encoded_bytes,
            Hex::decode("5355495f4252494447455f4d4553534147450001000000000000000a012000000000000000000000000000000000000000000000000000000000000000640b1400000000000000000000000000000000000000c8030000000000003039").unwrap(),
        );

        let hash = Keccak256::digest(encoded_bytes).digest;
        assert_eq!(
            hash.to_vec(),
            Hex::decode("6ab34c52b6264cbc12fe8c3874f9b08f8481d2e81530d136386646dbe2f8baf4")
                .unwrap(),
        );
        Ok(())
    }

    #[test]
    fn test_bridge_message_encoding_regression_eth_to_mgo_token_bridge_v1() -> anyhow::Result<()> {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mango_metrics::init_metrics(&registry);
        let eth_tx_hash = TxHash::random();
        let eth_event_index = 1u16;

        let nonce = 10u64;
        let mgo_chain_id = BridgeChainId::MgoTestnet;
        let eth_chain_id = BridgeChainId::EthSepolia;
        let mgo_address = MgoAddress::from_str(
            "0x0000000000000000000000000000000000000000000000000000000000000064",
        )
        .unwrap();
        let eth_address =
            EthAddress::from_str("0x00000000000000000000000000000000000000c8").unwrap();
        let token_id = TokenId::USDC;
        let amount = 12345;

        let eth_bridge_event = EthToMgoTokenBridgeV1 {
            nonce,
            mgo_chain_id,
            eth_chain_id,
            mgo_address,
            eth_address,
            token_id,
            amount,
        };
        let encoded_bytes = BridgeAction::EthToMgoBridgeAction(EthToMgoBridgeAction {
            eth_tx_hash,
            eth_event_index,
            eth_bridge_event,
        })
        .to_bytes();

        assert_eq!(
            encoded_bytes,
            Hex::decode("5355495f4252494447455f4d4553534147450001000000000000000a0b1400000000000000000000000000000000000000c801200000000000000000000000000000000000000000000000000000000000000064030000000000003039").unwrap(),
        );

        let hash = Keccak256::digest(encoded_bytes).digest;
        assert_eq!(
            hash.to_vec(),
            Hex::decode("b352508c301a37bb1b68a75dd0fc42b6f692b2650818631c8f8a4d4d3e5bef46")
                .unwrap(),
        );
        Ok(())
    }

    #[test]
    fn test_bridge_committee_construction() -> anyhow::Result<()> {
        let (mut authority, _, _) = get_test_authority_and_key(10000, 9999);
        // This is ok
        let _ = BridgeCommittee::new(vec![authority.clone()]).unwrap();

        // This is not ok - total voting power != 10000
        authority.voting_power = 9999;
        let _ = BridgeCommittee::new(vec![authority.clone()]).unwrap_err();

        // This is not ok - total voting power != 10000
        authority.voting_power = 10001;
        let _ = BridgeCommittee::new(vec![authority.clone()]).unwrap_err();

        // This is ok
        authority.voting_power = 5000;
        let mut authority_2 = authority.clone();
        let (_, kp): (_, fastcrypto::secp256k1::Secp256k1KeyPair) = get_key_pair();
        let pubkey = kp.public().clone();
        authority_2.pubkey = pubkey.clone();
        let _ = BridgeCommittee::new(vec![authority.clone(), authority_2.clone()]).unwrap();

        // This is not ok - duplicate pub key
        authority_2.pubkey = authority.pubkey.clone();
        let _ = BridgeCommittee::new(vec![authority.clone(), authority.clone()]).unwrap_err();
        Ok(())
    }

    #[test]
    fn test_bridge_committee_total_blocklisted_stake() -> anyhow::Result<()> {
        let (mut authority1, _, _) = get_test_authority_and_key(10000, 9999);
        assert_eq!(
            BridgeCommittee::new(vec![authority1.clone()])
                .unwrap()
                .total_blocklisted_stake(),
            0
        );
        authority1.voting_power = 6000;

        let (mut authority2, _, _) = get_test_authority_and_key(4000, 9999);
        authority2.is_blocklisted = true;
        assert_eq!(
            BridgeCommittee::new(vec![authority1.clone(), authority2.clone()])
                .unwrap()
                .total_blocklisted_stake(),
            4000
        );

        authority1.voting_power = 7000;
        authority2.voting_power = 2000;
        let (mut authority3, _, _) = get_test_authority_and_key(1000, 9999);
        authority3.is_blocklisted = true;
        assert_eq!(
            BridgeCommittee::new(vec![authority1, authority2, authority3])
                .unwrap()
                .total_blocklisted_stake(),
            3000
        );

        Ok(())
    }

    #[test]
    fn test_bridge_committee_filter_blocklisted_authorities() -> anyhow::Result<()> {
        // Note: today BridgeCommitte does not shuffle authorities
        let (authority1, _, _) = get_test_authority_and_key(5000, 9999);
        let (mut authority2, _, _) = get_test_authority_and_key(3000, 9999);
        authority2.is_blocklisted = true;
        let (authority3, _, _) = get_test_authority_and_key(2000, 9999);
        let committee = BridgeCommittee::new(vec![
            authority1.clone(),
            authority2.clone(),
            authority3.clone(),
        ])
        .unwrap();

        // exclude authority2
        let result = committee
            .shuffle_by_stake(None, None)
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(
            HashSet::from_iter(vec![authority1.pubkey_bytes(), authority3.pubkey_bytes()]),
            result
        );

        // exclude authority2 and authority3
        let result = committee
            .shuffle_by_stake(
                None,
                Some(
                    &[authority1.pubkey_bytes(), authority2.pubkey_bytes()]
                        .iter()
                        .cloned()
                        .collect(),
                ),
            )
            .into_iter()
            .collect::<HashSet<_>>();
        assert_eq!(HashSet::from_iter(vec![authority1.pubkey_bytes()]), result);

        Ok(())
    }
}
