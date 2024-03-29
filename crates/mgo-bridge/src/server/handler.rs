// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#![allow(clippy::type_complexity)]

use crate::crypto::{BridgeAuthorityKeyPair, BridgeAuthoritySignInfo};
use crate::error::{BridgeError, BridgeResult};
use crate::eth_client::EthClient;
use crate::mgo_client::{MgoClient, MgoClientInner};
use crate::types::{BridgeAction, SignedBridgeAction};
use async_trait::async_trait;
use axum::Json;
use ethers::providers::JsonRpcClient;
use ethers::types::TxHash;
use lru::LruCache;
use std::num::NonZeroUsize;
use std::str::FromStr;
use std::sync::Arc;
use mgo_types::digests::TransactionDigest;
use tap::TapFallible;
use tokio::sync::{oneshot, Mutex};
use tracing::info;
use tracing::instrument;

#[async_trait]
pub trait BridgeRequestHandlerTrait {
    /// Handles a request to sign a BridgeAction that bridges assets
    /// from Ethereum to Mgo. The inputs are a transaction hash on Ethereum
    /// that emitted the bridge event and the Event index in that transaction
    async fn handle_eth_tx_hash(
        &self,
        tx_hash_hex: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;
    /// Handles a request to sign a BridgeAction that bridges assets
    /// from Mgo to Ethereum. The inputs are a transaction digest on Mgo
    /// that emitted the bridge event and the Event index in that transaction
    async fn handle_mgo_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError>;
}

#[async_trait::async_trait]
trait ActionVerifier<K>: Send + Sync {
    async fn verify(&self, key: K) -> BridgeResult<BridgeAction>;
}

struct MgoActionVerifier<C> {
    mgo_client: Arc<MgoClient<C>>,
}

struct EthActionVerifier<P> {
    eth_client: Arc<EthClient<P>>,
}

#[async_trait::async_trait]
impl<C> ActionVerifier<(TransactionDigest, u16)> for MgoActionVerifier<C>
where
    C: MgoClientInner + Send + Sync + 'static,
{
    async fn verify(&self, key: (TransactionDigest, u16)) -> BridgeResult<BridgeAction> {
        let (tx_digest, event_idx) = key;
        self.mgo_client
            .get_bridge_action_by_tx_digest_and_event_idx_maybe(&tx_digest, event_idx)
            .await
            .tap_ok(|action| info!("Mgo action found: {:?}", action))
    }
}

#[async_trait::async_trait]
impl<C> ActionVerifier<(TxHash, u16)> for EthActionVerifier<C>
where
    C: JsonRpcClient + Send + Sync + 'static,
{
    async fn verify(&self, key: (TxHash, u16)) -> BridgeResult<BridgeAction> {
        let (tx_hash, event_idx) = key;
        self.eth_client
            .get_finalized_bridge_action_maybe(tx_hash, event_idx)
            .await
            .tap_ok(|action| info!("Eth action found: {:?}", action))
    }
}

struct SignerWithCache<K> {
    signer: Arc<BridgeAuthorityKeyPair>,
    verifier: Arc<dyn ActionVerifier<K>>,
    mutex: Arc<Mutex<()>>,
    cache: LruCache<K, Arc<Mutex<Option<BridgeResult<SignedBridgeAction>>>>>,
}

impl<K> SignerWithCache<K>
where
    K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
{
    fn new(
        signer: Arc<BridgeAuthorityKeyPair>,
        verifier: impl ActionVerifier<K> + 'static,
    ) -> Self {
        Self {
            signer,
            verifier: Arc::new(verifier),
            mutex: Arc::new(Mutex::new(())),
            cache: LruCache::new(NonZeroUsize::new(1000).unwrap()),
        }
    }

    fn spawn(
        mut self,
        mut rx: mango_metrics::metered_channel::Receiver<(
            K,
            oneshot::Sender<BridgeResult<SignedBridgeAction>>,
        )>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let (key, tx) = rx
                    .recv()
                    .await
                    .unwrap_or_else(|| panic!("Server signer's channel is closed"));
                let result = self.sign(key).await;
                // The receiver may be dropped before the sender (client connection was dropped for example),
                // we ignore the error in that case.
                let _ = tx.send(result);
            }
        })
    }

    async fn get_cache_entry(
        &mut self,
        key: K,
    ) -> Arc<Mutex<Option<BridgeResult<SignedBridgeAction>>>> {
        // This mutex exists to make sure everyone gets the same entry, namely no double insert
        let _ = self.mutex.lock().await;
        self.cache
            .get_or_insert(key, || Arc::new(Mutex::new(None)))
            .clone()
    }

    async fn sign(&mut self, key: K) -> BridgeResult<SignedBridgeAction> {
        let signer = self.signer.clone();
        let verifier = self.verifier.clone();
        let entry = self.get_cache_entry(key.clone()).await;
        let mut guard = entry.lock().await;
        if let Some(result) = &*guard {
            return result.clone();
        }
        match verifier.verify(key.clone()).await {
            Ok(bridge_action) => {
                let sig = BridgeAuthoritySignInfo::new(&bridge_action, &signer);
                let result = SignedBridgeAction::new_from_data_and_sig(bridge_action, sig);
                // Cache result if Ok
                *guard = Some(Ok(result.clone()));
                Ok(result)
            }
            Err(e) => {
                match e {
                    // Only cache non-transient errors
                    BridgeError::BridgeEventInUnrecognizedMgoPackage
                    | BridgeError::BridgeEventInUnrecognizedEthContract
                    | BridgeError::BridgeEventNotActionable
                    | BridgeError::NoBridgeEventsInTxPosition => {
                        *guard = Some(Err(e.clone()));
                    }
                    _ => (),
                }
                Err(e)
            }
        }
    }

    #[cfg(test)]
    async fn get_testing_only(
        &mut self,
        key: K,
    ) -> Option<&Arc<Mutex<Option<BridgeResult<SignedBridgeAction>>>>> {
        let _ = self.mutex.lock().await;
        self.cache.get(&key)
    }
}

pub struct BridgeRequestHandler {
    mgo_signer_tx: mango_metrics::metered_channel::Sender<(
        (TransactionDigest, u16),
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
    eth_signer_tx: mango_metrics::metered_channel::Sender<(
        (TxHash, u16),
        oneshot::Sender<BridgeResult<SignedBridgeAction>>,
    )>,
}

impl BridgeRequestHandler {
    pub fn new<
        SC: MgoClientInner + Send + Sync + 'static,
        EP: JsonRpcClient + Send + Sync + 'static,
    >(
        signer: BridgeAuthorityKeyPair,
        mgo_client: Arc<MgoClient<SC>>,
        eth_client: Arc<EthClient<EP>>,
    ) -> Self {
        let (mgo_signer_tx, mgo_rx) = mango_metrics::metered_channel::channel(
            1000,
            &mango_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["server_mgo_action_signing_queue"]),
        );
        let (eth_signer_tx, eth_rx) = mango_metrics::metered_channel::channel(
            1000,
            &mango_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["server_eth_action_signing_queue"]),
        );
        let signer = Arc::new(signer);

        SignerWithCache::new(signer.clone(), MgoActionVerifier { mgo_client }).spawn(mgo_rx);
        SignerWithCache::new(signer.clone(), EthActionVerifier { eth_client }).spawn(eth_rx);

        Self {
            mgo_signer_tx,
            eth_signer_tx,
        }
    }
}

#[async_trait]
impl BridgeRequestHandlerTrait for BridgeRequestHandler {
    #[instrument(level = "info", skip(self))]
    async fn handle_eth_tx_hash(
        &self,
        tx_hash_hex: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        info!("Received handle eth tx request");
        let tx_hash = TxHash::from_str(&tx_hash_hex).map_err(|_| BridgeError::InvalidTxHash)?;

        let (tx, rx) = oneshot::channel();
        self.eth_signer_tx
            .send(((tx_hash, event_idx), tx))
            .await
            .unwrap_or_else(|_| panic!("Server eth signing channel is closed"));
        let signed_action = rx
            .blocking_recv()
            .unwrap_or_else(|_| panic!("Server signing task's oneshot channel is dropped"))?;
        Ok(Json(signed_action))
    }

    #[instrument(level = "info", skip(self))]
    async fn handle_mgo_tx_digest(
        &self,
        tx_digest_base58: String,
        event_idx: u16,
    ) -> Result<Json<SignedBridgeAction>, BridgeError> {
        info!("Received handle mgo tx request");
        let tx_digest = TransactionDigest::from_str(&tx_digest_base58)
            .map_err(|_e| BridgeError::InvalidTxHash)?;
        let (tx, rx) = oneshot::channel();
        self.mgo_signer_tx
            .send(((tx_digest, event_idx), tx))
            .await
            .unwrap_or_else(|_| panic!("Server mgo signing channel is closed"));
        let signed_action = rx
            .blocking_recv()
            .unwrap_or_else(|_| panic!("Server signing task's oneshot channel is dropped"))?;
        Ok(Json(signed_action))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use crate::{
        eth_mock_provider::EthMockProvider,
        events::{init_all_struct_tags, MoveTokenBridgeEvent, MgoToEthTokenBridgeV1},
        mgo_mock_client::MgoMockClient,
        test_utils::{
            get_test_log_and_action, get_test_mgo_to_eth_bridge_action, mock_last_finalized_block,
        },
        types::{BridgeActionType, BridgeChainId, TokenId},
    };
    use ethers::types::{Address as EthAddress, TransactionReceipt};
    use mgo_json_rpc_types::MgoEvent;
    use mgo_types::{base_types::MgoAddress, crypto::get_key_pair};

    #[tokio::test]
    async fn test_mgo_signer_with_cache() {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let mgo_client_mock = MgoMockClient::default();
        let mgo_verifier = MgoActionVerifier {
            mgo_client: Arc::new(MgoClient::new_for_testing(mgo_client_mock.clone())),
        };
        let mut mgo_signer_with_cache = SignerWithCache::new(signer.clone(), mgo_verifier);

        // Test `get_cache_entry` creates a new entry if not exist
        let mgo_tx_digest = TransactionDigest::random();
        let mgo_event_idx = 42;
        assert!(mgo_signer_with_cache
            .get_testing_only((mgo_tx_digest, mgo_event_idx))
            .await
            .is_none());
        let entry = mgo_signer_with_cache
            .get_cache_entry((mgo_tx_digest, mgo_event_idx))
            .await;
        let entry_ = mgo_signer_with_cache
            .get_testing_only((mgo_tx_digest, mgo_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_none());

        let action =
            get_test_mgo_to_eth_bridge_action(Some(mgo_tx_digest), Some(mgo_event_idx), None, None);
        let sig = BridgeAuthoritySignInfo::new(&action, &signer);
        let signed_action = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig);
        entry.lock().await.replace(Ok(signed_action));
        let entry_ = mgo_signer_with_cache
            .get_testing_only((mgo_tx_digest, mgo_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_some());

        // Test `sign` caches Err result
        let mgo_tx_digest = TransactionDigest::random();
        let mgo_event_idx = 0;

        // Mock an non-cacheable error such as rpc error
        mgo_client_mock.add_events_by_tx_digest_error(mgo_tx_digest);
        mgo_signer_with_cache
            .sign((mgo_tx_digest, mgo_event_idx))
            .await
            .unwrap_err();
        let entry_ = mgo_signer_with_cache
            .get_testing_only((mgo_tx_digest, mgo_event_idx))
            .await;
        assert!(entry_.unwrap().lock().await.is_none());

        // Mock a cacheable error such as no bridge events in tx position (empty event list)
        mgo_client_mock.add_events_by_tx_digest(mgo_tx_digest, vec![]);
        assert!(matches!(
            mgo_signer_with_cache
                .sign((mgo_tx_digest, mgo_event_idx))
                .await,
            Err(BridgeError::NoBridgeEventsInTxPosition)
        ));
        let entry_ = mgo_signer_with_cache
            .get_testing_only((mgo_tx_digest, mgo_event_idx))
            .await;
        assert_eq!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap_err(),
            BridgeError::NoBridgeEventsInTxPosition,
        );

        // TODO: test BridgeEventInUnrecognizedMgoPackage, MgoBridgeEvent::try_from_mgo_event
        // and BridgeEventNotActionable to be cached

        // Test `sign` caches Ok result
        let emitted_event_1 = MoveTokenBridgeEvent {
            message_type: BridgeActionType::TokenTransfer as u8,
            seq_num: 1,
            source_chain: BridgeChainId::MgoLocalTest as u8,
            sender_address: MgoAddress::random_for_testing_only().to_vec(),
            target_chain: BridgeChainId::EthLocalTest as u8,
            target_address: EthAddress::random().as_bytes().to_vec(),
            token_type: TokenId::USDC as u8,
            amount: 12345,
        };

        // TODO: remove once we don't rely on env var to get package id
        std::env::set_var("BRIDGE_PACKAGE_ID", "0x0b");
        init_all_struct_tags();

        let mut mgo_event_1 = MgoEvent::random_for_testing();
        mgo_event_1.type_ = MgoToEthTokenBridgeV1.get().unwrap().clone();
        mgo_event_1.bcs = bcs::to_bytes(&emitted_event_1).unwrap();
        let mgo_tx_digest = mgo_event_1.id.tx_digest;

        let mut mgo_event_2 = MgoEvent::random_for_testing();
        mgo_event_2.type_ = MgoToEthTokenBridgeV1.get().unwrap().clone();
        mgo_event_2.bcs = bcs::to_bytes(&emitted_event_1).unwrap();
        let mgo_event_idx_2 = 1;
        mgo_client_mock.add_events_by_tx_digest(mgo_tx_digest, vec![mgo_event_2.clone()]);

        mgo_client_mock.add_events_by_tx_digest(
            mgo_tx_digest,
            vec![mgo_event_1.clone(), mgo_event_2.clone()],
        );
        let signed_1 = mgo_signer_with_cache
            .sign((mgo_tx_digest, mgo_event_idx))
            .await
            .unwrap();
        let signed_2 = mgo_signer_with_cache
            .sign((mgo_tx_digest, mgo_event_idx_2))
            .await
            .unwrap();

        // Because the result is cached now, the verifier should not be called again.
        // Even though we remove the `add_events_by_tx_digest` mock, we will still get the same result.
        mgo_client_mock.add_events_by_tx_digest(mgo_tx_digest, vec![]);
        assert_eq!(
            mgo_signer_with_cache
                .sign((mgo_tx_digest, mgo_event_idx))
                .await
                .unwrap(),
            signed_1
        );
        assert_eq!(
            mgo_signer_with_cache
                .sign((mgo_tx_digest, mgo_event_idx_2))
                .await
                .unwrap(),
            signed_2
        );
    }

    #[tokio::test]
    async fn test_eth_signer_with_cache() {
        let (_, kp): (_, BridgeAuthorityKeyPair) = get_key_pair();
        let signer = Arc::new(kp);
        let eth_mock_provider = EthMockProvider::default();
        let contract_address = EthAddress::random();
        let eth_client = EthClient::new_mocked(
            eth_mock_provider.clone(),
            HashSet::from_iter(vec![contract_address]),
        );
        let eth_verifier = EthActionVerifier {
            eth_client: Arc::new(eth_client),
        };
        let mut eth_signer_with_cache = SignerWithCache::new(signer.clone(), eth_verifier);

        // Test `get_cache_entry` creates a new entry if not exist
        let eth_tx_hash = TxHash::random();
        let eth_event_idx = 42;
        assert!(eth_signer_with_cache
            .get_testing_only((eth_tx_hash, eth_event_idx))
            .await
            .is_none());
        let entry = eth_signer_with_cache
            .get_cache_entry((eth_tx_hash, eth_event_idx))
            .await;
        let entry_ = eth_signer_with_cache
            .get_testing_only((eth_tx_hash, eth_event_idx))
            .await;
        // first unwrap should not pacic because the entry should have been inserted by `get_cache_entry`
        assert!(entry_.unwrap().lock().await.is_none());

        let (_, action) = get_test_log_and_action(contract_address, eth_tx_hash, eth_event_idx);
        let sig = BridgeAuthoritySignInfo::new(&action, &signer);
        let signed_action = SignedBridgeAction::new_from_data_and_sig(action.clone(), sig);
        entry.lock().await.replace(Ok(signed_action.clone()));
        let entry_ = eth_signer_with_cache
            .get_testing_only((eth_tx_hash, eth_event_idx))
            .await;
        assert_eq!(
            entry_.unwrap().lock().await.clone().unwrap().unwrap(),
            signed_action
        );

        // Test `sign` caches Ok result
        let eth_tx_hash = TxHash::random();
        let eth_event_idx = 0;
        let (log, _action) = get_test_log_and_action(contract_address, eth_tx_hash, eth_event_idx);
        eth_mock_provider
            .add_response::<[TxHash; 1], TransactionReceipt, TransactionReceipt>(
                "eth_getTransactionReceipt",
                [log.transaction_hash.unwrap()],
                TransactionReceipt {
                    block_number: log.block_number,
                    logs: vec![log.clone()],
                    ..Default::default()
                },
            )
            .unwrap();
        mock_last_finalized_block(&eth_mock_provider, log.block_number.unwrap().as_u64());

        eth_signer_with_cache
            .sign((eth_tx_hash, eth_event_idx))
            .await
            .unwrap();
        let entry_ = eth_signer_with_cache
            .get_testing_only((eth_tx_hash, eth_event_idx))
            .await;
        entry_.unwrap().lock().await.clone().unwrap().unwrap();
    }
}
