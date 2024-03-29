// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! `BridgeClient` talks to BridgeNode.

use crate::crypto::{verify_signed_bridge_action, BridgeAuthorityPublicKeyBytes};
use crate::error::{BridgeError, BridgeResult};
use crate::server::APPLICATION_JSON;
use crate::types::{BridgeAction, BridgeCommittee, VerifiedSignedBridgeAction};
use fastcrypto::encoding::{Encoding, Hex};
use std::str::FromStr;
use std::sync::Arc;
use url::Url;

// Note: `base_url` is `Option<Url>` because `quorum_map_then_reduce_with_timeout_and_prefs`
// uses `[]` to get Client based on key. Therefore even when the URL is invalid we need to
// create a Client instance.
// TODO: In the future we can consider change `quorum_map_then_reduce_with_timeout_and_prefs`
// and its callsites to use `get` instead of `[]`.
#[derive(Clone, Debug)]
pub struct BridgeClient {
    inner: reqwest::Client,
    authority: BridgeAuthorityPublicKeyBytes,
    committee: Arc<BridgeCommittee>,
    base_url: Option<Url>,
}

impl BridgeClient {
    pub fn new(
        authority_name: BridgeAuthorityPublicKeyBytes,
        committee: Arc<BridgeCommittee>,
    ) -> BridgeResult<Self> {
        if !committee.is_active_member(&authority_name) {
            return Err(BridgeError::InvalidBridgeAuthority(authority_name));
        }
        // Unwrap safe: we passed the `is_active_member` check above
        let member = committee.member(&authority_name).unwrap();
        Ok(Self {
            inner: reqwest::Client::new(),
            authority: authority_name.clone(),
            base_url: Url::from_str(&member.base_url).ok(),
            committee,
        })
    }

    #[cfg(test)]
    pub fn update_committee(&mut self, committee: Arc<BridgeCommittee>) {
        self.committee = committee;
    }

    // Important: the paths need to match the ones in server.rs
    fn bridge_action_to_path(event: &BridgeAction) -> String {
        match event {
            BridgeAction::MgoToEthBridgeAction(e) => format!(
                "sign/bridge_tx/mgo/eth/{}/{}",
                e.mgo_tx_digest, e.mgo_tx_event_index
            ),
            BridgeAction::EthToMgoBridgeAction(e) => format!(
                "sign/bridge_tx/eth/mgo/{}/{}",
                Hex::encode(e.eth_tx_hash.0),
                e.eth_event_index
            ),
        }
    }

    // Returns Ok(true) if the server is up and running
    pub async fn ping(&self) -> BridgeResult<bool> {
        if self.base_url.is_none() {
            return Err(BridgeError::InvalidAuthorityUrl(self.authority.clone()));
        }
        // Unwrap safe: checked `self.base_url.is_none()` above
        let url = self.base_url.clone().unwrap();
        Ok(self
            .inner
            .get(url)
            .header(reqwest::header::ACCEPT, APPLICATION_JSON)
            .send()
            .await?
            .error_for_status()
            .is_ok())
    }

    pub async fn request_sign_bridge_action(
        &self,
        action: BridgeAction,
    ) -> BridgeResult<VerifiedSignedBridgeAction> {
        if self.base_url.is_none() {
            return Err(BridgeError::InvalidAuthorityUrl(self.authority.clone()));
        }
        // Unwrap safe: checked `self.base_url.is_none()` above
        let url = self
            .base_url
            .clone()
            .unwrap()
            .join(&Self::bridge_action_to_path(&action))?;
        let resp = self
            .inner
            .get(url)
            .header(reqwest::header::ACCEPT, APPLICATION_JSON)
            .send()
            .await?;
        if !resp.status().is_success() {
            let error_status = format!("{:?}", resp.error_for_status_ref());
            return Err(BridgeError::RestAPIError(format!(
                "request_sign_bridge_action failed with status {:?}: {:?}",
                error_status,
                resp.text().await?
            )));
        }
        let signed_bridge_action = resp.json().await?;
        verify_signed_bridge_action(
            &action,
            signed_bridge_action,
            &self.authority,
            &self.committee,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        abi::EthToMgoTokenBridgeV1,
        crypto::BridgeAuthoritySignInfo,
        events::EmittedMgoToEthTokenBridgeV1,
        server::mock_handler::BridgeRequestMockHandler,
        test_utils::{get_test_authority_and_key, get_test_mgo_to_eth_bridge_action},
        types::{BridgeChainId, SignedBridgeAction, TokenId},
    };
    use fastcrypto::traits::KeyPair;
    use prometheus::Registry;

    use super::*;
    use crate::test_utils::run_mock_bridge_server;
    use ethers::types::Address as EthAddress;
    use ethers::types::TxHash;
    use mgo_types::{base_types::MgoAddress, crypto::get_key_pair, digests::TransactionDigest};

    #[tokio::test]
    async fn test_bridge_client() {
        telemetry_subscribers::init_for_testing();

        let (mut authority, pubkey, _) = get_test_authority_and_key(10000, 12345);

        let pubkey_bytes = BridgeAuthorityPublicKeyBytes::from(&pubkey);
        let committee = Arc::new(BridgeCommittee::new(vec![authority.clone()]).unwrap());
        let action = get_test_mgo_to_eth_bridge_action(None, Some(1), Some(1), Some(100));

        // Ok
        let client = BridgeClient::new(pubkey_bytes.clone(), committee).unwrap();
        assert!(client.base_url.is_some());

        // Ok
        authority.base_url = "https://foo.mgobridge.io".to_string();
        let committee = Arc::new(BridgeCommittee::new(vec![authority.clone()]).unwrap());
        let client = BridgeClient::new(pubkey_bytes.clone(), committee.clone()).unwrap();
        assert!(client.base_url.is_some());

        // Err, not in committee
        let (_, kp2): (_, fastcrypto::secp256k1::Secp256k1KeyPair) = get_key_pair();
        let pubkey2_bytes = BridgeAuthorityPublicKeyBytes::from(kp2.public());
        let err = BridgeClient::new(pubkey2_bytes, committee.clone()).unwrap_err();
        assert!(matches!(err, BridgeError::InvalidBridgeAuthority(_)));

        // invalid base url
        authority.base_url = "127.0.0.1:12345".to_string(); // <-- bad, missing http://
        let committee = Arc::new(BridgeCommittee::new(vec![authority.clone()]).unwrap());
        let client = BridgeClient::new(pubkey_bytes.clone(), committee.clone()).unwrap();
        assert!(client.base_url.is_none());
        assert!(matches!(
            client.ping().await.unwrap_err(),
            BridgeError::InvalidAuthorityUrl(_)
        ));
        assert!(matches!(
            client
                .request_sign_bridge_action(action.clone())
                .await
                .unwrap_err(),
            BridgeError::InvalidAuthorityUrl(_)
        ));

        // invalid base url
        authority.base_url = "http://127.256.0.1:12345".to_string(); // <-- bad, invalid ipv4 address
        let committee = Arc::new(BridgeCommittee::new(vec![authority.clone()]).unwrap());
        let client = BridgeClient::new(pubkey_bytes, committee.clone()).unwrap();
        assert!(client.base_url.is_none());
        assert!(matches!(
            client.ping().await.unwrap_err(),
            BridgeError::InvalidAuthorityUrl(_)
        ));
        assert!(matches!(
            client
                .request_sign_bridge_action(action.clone())
                .await
                .unwrap_err(),
            BridgeError::InvalidAuthorityUrl(_)
        ));
    }

    #[tokio::test]
    async fn test_bridge_client_request_sign_action() {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mango_metrics::init_metrics(&registry);

        let mock_handler = BridgeRequestMockHandler::new();

        // start server
        let (_handles, ports) = run_mock_bridge_server(vec![mock_handler.clone()]);

        let port = ports[0];

        let (authority, _pubkey, secret) = get_test_authority_and_key(5000, port);
        let (authority2, _pubkey2, secret2) = get_test_authority_and_key(5000, port - 1);

        let committee = BridgeCommittee::new(vec![authority.clone(), authority2.clone()]).unwrap();

        let mut client =
            BridgeClient::new(authority.pubkey_bytes(), Arc::new(committee.clone())).unwrap();

        let tx_digest = TransactionDigest::random();
        let event_idx = 4;

        let action =
            get_test_mgo_to_eth_bridge_action(Some(tx_digest), Some(event_idx), Some(1), Some(100));
        let sig = BridgeAuthoritySignInfo::new(&action, &secret);
        let signed_event = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig.clone());
        mock_handler.add_mgo_event_response(tx_digest, event_idx, Ok(signed_event.clone()));

        // success
        client
            .request_sign_bridge_action(action.clone())
            .await
            .unwrap();

        // mismatched action would fail, this could happen when the authority fetched the wrong event
        let action2 =
            get_test_mgo_to_eth_bridge_action(Some(tx_digest), Some(event_idx), Some(2), Some(200));
        let wrong_sig = BridgeAuthoritySignInfo::new(&action2, &secret);
        let wrong_signed_action =
            SignedBridgeAction::new_from_data_and_sig(action2.clone(), wrong_sig.clone());
        mock_handler.add_mgo_event_response(tx_digest, event_idx, Ok(wrong_signed_action));
        let err = client
            .request_sign_bridge_action(action.clone())
            .await
            .unwrap_err();
        assert!(matches!(err, BridgeError::MismatchedAction));

        // The action matches but the signature is wrong, fail
        let wrong_signed_action =
            SignedBridgeAction::new_from_data_and_sig(action.clone(), wrong_sig);
        mock_handler.add_mgo_event_response(tx_digest, event_idx, Ok(wrong_signed_action));
        let err = client
            .request_sign_bridge_action(action.clone())
            .await
            .unwrap_err();
        assert!(matches!(
            err,
            BridgeError::InvalidBridgeAuthoritySignature(..)
        ));

        // sig from blocklisted authority would fail
        let mut authority_blocklisted = authority.clone();
        authority_blocklisted.is_blocklisted = true;
        let committee2 = Arc::new(
            BridgeCommittee::new(vec![authority_blocklisted.clone(), authority2.clone()]).unwrap(),
        );
        client.update_committee(committee2);
        mock_handler.add_mgo_event_response(tx_digest, event_idx, Ok(signed_event));

        let err = client
            .request_sign_bridge_action(action.clone())
            .await
            .unwrap_err();
        assert!(
            matches!(err, BridgeError::InvalidBridgeAuthority(pk) if pk == authority_blocklisted.pubkey_bytes()),
        );

        client.update_committee(committee.into());

        // signed by a different authority in committee would fail
        let sig2 = BridgeAuthoritySignInfo::new(&action, &secret2);
        let signed_event2 = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig2.clone());
        mock_handler.add_mgo_event_response(tx_digest, event_idx, Ok(signed_event2));
        let err = client
            .request_sign_bridge_action(action.clone())
            .await
            .unwrap_err();
        assert!(matches!(err, BridgeError::MismatchedAuthoritySigner));

        // signed by a different key, not in committee, would fail
        let (_, kp3): (_, fastcrypto::secp256k1::Secp256k1KeyPair) = get_key_pair();
        let secret3 = Arc::pin(kp3);
        let sig3 = BridgeAuthoritySignInfo::new(&action, &secret3);
        let signed_event3 = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig3);
        mock_handler.add_mgo_event_response(tx_digest, event_idx, Ok(signed_event3));
        let err = client
            .request_sign_bridge_action(action.clone())
            .await
            .unwrap_err();
        assert!(matches!(err, BridgeError::MismatchedAuthoritySigner));
    }

    #[test]
    fn test_bridge_action_path_regression_tests() {
        let mgo_tx_digest = TransactionDigest::random();
        let mgo_tx_event_index = 5;
        let action = BridgeAction::MgoToEthBridgeAction(crate::types::MgoToEthBridgeAction {
            mgo_tx_digest,
            mgo_tx_event_index,
            mgo_bridge_event: EmittedMgoToEthTokenBridgeV1 {
                mgo_chain_id: BridgeChainId::MgoDevnet,
                nonce: 1,
                mgo_address: MgoAddress::random_for_testing_only(),
                eth_chain_id: BridgeChainId::EthSepolia,
                eth_address: EthAddress::random(),
                token_id: TokenId::USDT,
                amount: 1,
            },
        });
        assert_eq!(
            BridgeClient::bridge_action_to_path(&action),
            format!(
                "sign/bridge_tx/mgo/eth/{}/{}",
                mgo_tx_digest, mgo_tx_event_index
            )
        );

        let eth_tx_hash = TxHash::random();
        let eth_event_index = 6;
        let action = BridgeAction::EthToMgoBridgeAction(crate::types::EthToMgoBridgeAction {
            eth_tx_hash,
            eth_event_index,
            eth_bridge_event: EthToMgoTokenBridgeV1 {
                eth_chain_id: BridgeChainId::EthSepolia,
                nonce: 1,
                eth_address: EthAddress::random(),
                mgo_chain_id: BridgeChainId::MgoDevnet,
                mgo_address: MgoAddress::random_for_testing_only(),
                token_id: TokenId::USDT,
                amount: 1,
            },
        });

        assert_eq!(
            BridgeClient::bridge_action_to_path(&action),
            format!(
                "sign/bridge_tx/eth/mgo/{}/{}",
                Hex::encode(eth_tx_hash.0),
                eth_event_index
            )
        );
    }
}
