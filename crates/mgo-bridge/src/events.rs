// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

//! This file contains the definition of the MgoBridgeEvent enum, of
//! which each variant is an emitted Event struct defind in the Move
//! Bridge module. We rely on structures in this file to decode
//! the bcs content of the emitted events.

use std::str::FromStr;

use crate::error::BridgeError;
use crate::error::BridgeResult;
use crate::mgo_transaction_builder::get_bridge_package_id;
use crate::types::BridgeAction;
use crate::types::BridgeActionType;
use crate::types::BridgeChainId;
use crate::types::MgoToEthBridgeAction;
use crate::types::TokenId;
use ethers::types::Address as EthAddress;
use fastcrypto::encoding::Encoding;
use fastcrypto::encoding::Hex;
use move_core_types::language_storage::StructTag;
use once_cell::sync::OnceCell;
use serde::{Deserialize, Serialize};
use mgo_json_rpc_types::MgoEvent;
use mgo_types::base_types::MgoAddress;
use mgo_types::digests::TransactionDigest;

// This is the event structure defined and emitted in Move
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct MoveTokenBridgeEvent {
    pub message_type: u8,
    pub seq_num: u64,
    pub source_chain: u8,
    pub sender_address: Vec<u8>,
    pub target_chain: u8,
    pub target_address: Vec<u8>,
    pub token_type: u8,
    pub amount: u64,
}

// Sanitized version of MoveTokenBridgeEvent
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EmittedMgoToEthTokenBridgeV1 {
    pub nonce: u64,
    pub mgo_chain_id: BridgeChainId,
    pub eth_chain_id: BridgeChainId,
    pub mgo_address: MgoAddress,
    pub eth_address: EthAddress,
    pub token_id: TokenId,
    pub amount: u64,
}

impl TryFrom<MoveTokenBridgeEvent> for EmittedMgoToEthTokenBridgeV1 {
    type Error = BridgeError;

    fn try_from(event: MoveTokenBridgeEvent) -> BridgeResult<Self> {
        if event.message_type != BridgeActionType::TokenTransfer as u8 {
            return Err(BridgeError::Generic(format!(
                "Failed to convert MoveTokenBridgeEvent to EmittedMgoToEthTokenBridgeV1. Expected message type {}, got {}",
                BridgeActionType::TokenTransfer as u8,
                event.message_type
            )));
        }
        let token_id = TokenId::try_from(event.token_type).map_err(|_e| {
            BridgeError::Generic(format!(
                "Failed to convert MoveTokenBridgeEvent to EmittedMgoToEthTokenBridgeV1. Failed to convert token type {} to TokenId",
                event.token_type,
            ))
        })?;

        let mgo_chain_id = BridgeChainId::try_from(event.source_chain).map_err(|_e| {
            BridgeError::Generic(format!(
                "Failed to convert MoveTokenBridgeEvent to EmittedMgoToEthTokenBridgeV1. Failed to convert source chain {} to BridgeChainId",
                event.token_type,
            ))
        })?;
        let eth_chain_id = BridgeChainId::try_from(event.target_chain).map_err(|_e| {
            BridgeError::Generic(format!(
                "Failed to convert MoveTokenBridgeEvent to EmittedMgoToEthTokenBridgeV1. Failed to convert target chain {} to BridgeChainId",
                event.token_type,
            ))
        })?;

        match mgo_chain_id {
            BridgeChainId::MgoMainnet
            | BridgeChainId::MgoTestnet
            | BridgeChainId::MgoDevnet
            | BridgeChainId::MgoLocalTest => {}
            _ => {
                return Err(BridgeError::Generic(format!(
                    "Failed to convert MoveTokenBridgeEvent to EmittedMgoToEthTokenBridgeV1. Invalid source chain {}",
                    event.source_chain
                )));
            }
        }
        match eth_chain_id {
            BridgeChainId::EthMainnet | BridgeChainId::EthSepolia | BridgeChainId::EthLocalTest => {
            }
            _ => {
                return Err(BridgeError::Generic(format!(
                    "Failed to convert MoveTokenBridgeEvent to EmittedMgoToEthTokenBridgeV1. Invalid target chain {}",
                    event.target_chain
                )));
            }
        }

        let mgo_address = MgoAddress::from_bytes(event.sender_address)
            .map_err(|e| BridgeError::Generic(format!("Failed to convert MoveTokenBridgeEvent to EmittedMgoToEthTokenBridgeV1. Failed to convert sender_address to MgoAddress: {:?}", e)))?;
        let eth_address = EthAddress::from_str(&Hex::encode(&event.target_address))?;

        Ok(Self {
            nonce: event.seq_num,
            mgo_chain_id,
            eth_chain_id,
            mgo_address,
            eth_address,
            token_id,
            amount: event.amount,
        })
    }
}

// TODO: update this once we have bridge package on mgo framework
pub fn get_bridge_event_struct_tag() -> &'static str {
    static BRIDGE_EVENT_STRUCT_TAG: OnceCell<String> = OnceCell::new();
    BRIDGE_EVENT_STRUCT_TAG.get_or_init(|| {
        let bridge_package_id = *get_bridge_package_id();
        format!("0x{}::bridge::TokenBridgeEvent", bridge_package_id.to_hex())
    })
}

crate::declare_events!(
    MgoToEthTokenBridgeV1(EmittedMgoToEthTokenBridgeV1) => (get_bridge_event_struct_tag(), MoveTokenBridgeEvent)
    // Add new event types here. Format: EnumVariantName(Struct) => ("StructTagString", CorrespondingMoveStruct)
);

#[macro_export]
macro_rules! declare_events {
    ($($variant:ident($type:path) => ($event_tag:expr, $event_struct:path)),* $(,)?) => {

        #[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
        pub enum MgoBridgeEvent {
            $($variant($type),)*
        }

        #[allow(non_upper_case_globals)]
        $(pub(crate) static $variant: OnceCell<StructTag> = OnceCell::new();)*

        pub(crate) fn init_all_struct_tags() {
            $($variant.get_or_init(|| {
                StructTag::from_str($event_tag).unwrap()
            });)*
        }

        // Try to convert a MgoEvent into MgoBridgeEvent
        impl MgoBridgeEvent {
            pub fn try_from_mgo_event(event: &MgoEvent) -> BridgeResult<Option<MgoBridgeEvent>> {
                init_all_struct_tags(); // Ensure all tags are initialized

                // Unwrap safe: we inited above
                $(
                    if &event.type_ == $variant.get().unwrap() {
                        let event_struct: $event_struct = bcs::from_bytes(&event.bcs).map_err(|e| BridgeError::InternalError(format!("Failed to deserialize event to {}: {:?}", stringify!($event_struct), e)))?;
                        return Ok(Some(MgoBridgeEvent::$variant(event_struct.try_into()?)));
                    }
                )*
                Ok(None)
            }
        }
    };
}

impl MgoBridgeEvent {
    pub fn try_into_bridge_action(
        self,
        mgo_tx_digest: TransactionDigest,
        mgo_tx_event_index: u16,
    ) -> Option<BridgeAction> {
        match self {
            MgoBridgeEvent::MgoToEthTokenBridgeV1(event) => {
                Some(BridgeAction::MgoToEthBridgeAction(MgoToEthBridgeAction {
                    mgo_tx_digest,
                    mgo_tx_event_index,
                    mgo_bridge_event: event.clone(),
                }))
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::get_bridge_event_struct_tag;
    use super::EmittedMgoToEthTokenBridgeV1;
    use super::MoveTokenBridgeEvent;
    use crate::types::BridgeAction;
    use crate::types::BridgeActionType;
    use crate::types::BridgeChainId;
    use crate::types::MgoToEthBridgeAction;
    use crate::types::TokenId;
    use ethers::types::Address as EthAddress;
    use move_core_types::language_storage::StructTag;
    use std::str::FromStr;
    use mgo_json_rpc_types::MgoEvent;
    use mgo_types::base_types::ObjectID;
    use mgo_types::base_types::MgoAddress;
    use mgo_types::digests::TransactionDigest;
    use mgo_types::event::EventID;
    use mgo_types::Identifier;

    /// Returns a test MgoEvent and corresponding BridgeAction
    pub fn get_test_mgo_event_and_action(identifier: Identifier) -> (MgoEvent, BridgeAction) {
        let sanitized_event = EmittedMgoToEthTokenBridgeV1 {
            nonce: 1,
            mgo_chain_id: BridgeChainId::MgoTestnet,
            mgo_address: MgoAddress::random_for_testing_only(),
            eth_chain_id: BridgeChainId::EthSepolia,
            eth_address: EthAddress::random(),
            token_id: TokenId::Mgo,
            amount: 100,
        };
        let emitted_event = MoveTokenBridgeEvent {
            message_type: BridgeActionType::TokenTransfer as u8,
            seq_num: sanitized_event.nonce,
            source_chain: sanitized_event.mgo_chain_id as u8,
            sender_address: sanitized_event.mgo_address.to_vec(),
            target_chain: sanitized_event.eth_chain_id as u8,
            target_address: sanitized_event.eth_address.as_bytes().to_vec(),
            token_type: sanitized_event.token_id as u8,
            amount: sanitized_event.amount,
        };

        let tx_digest = TransactionDigest::random();
        let event_idx = 10u16;
        let bridge_action = BridgeAction::MgoToEthBridgeAction(MgoToEthBridgeAction {
            mgo_tx_digest: tx_digest,
            mgo_tx_event_index: event_idx,
            mgo_bridge_event: sanitized_event.clone(),
        });
        let event = MgoEvent {
            // For this test to pass, match what is in events.rs
            type_: StructTag::from_str(get_bridge_event_struct_tag()).unwrap(),
            bcs: bcs::to_bytes(&emitted_event).unwrap(),
            id: EventID {
                tx_digest,
                event_seq: event_idx as u64,
            },

            // The following fields do not matter as of writing,
            // but if tests start to fail, it's worth checking these fields.
            package_id: ObjectID::ZERO,
            transaction_module: identifier.clone(),
            sender: MgoAddress::random_for_testing_only(),
            parsed_json: serde_json::json!({"test": "test"}),
            timestamp_ms: None,
        };
        (event, bridge_action)
    }
}
