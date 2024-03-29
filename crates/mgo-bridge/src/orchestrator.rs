// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//! `BridgeOrchestrator` is the component that:
//! 1. monitors Mgo and Ethereum events with the help of `MgoSyncer` and `EthSyncer`
//! 2. updates WAL table and cursor tables
//! 2. hands actions to `BridgeExecutor` for execution

use crate::abi::EthBridgeEvent;
use crate::action_executor::{
    submit_to_executor, BridgeActionExecutionWrapper, BridgeActionExecutorTrait,
};
use crate::error::BridgeResult;
use crate::events::MgoBridgeEvent;
use crate::storage::BridgeOrchestratorTables;
use crate::mgo_client::{MgoClient, MgoClientInner};
use crate::types::EthLog;
use ethers::types::Address as EthAddress;
use mango_metrics::spawn_logged_monitored_task;
use std::sync::Arc;
use mgo_json_rpc_types::MgoEvent;
use mgo_types::Identifier;
use tokio::task::JoinHandle;
use tracing::{info, warn};

pub struct BridgeOrchestrator<C> {
    _mgo_client: Arc<MgoClient<C>>,
    mgo_events_rx: mango_metrics::metered_channel::Receiver<(Identifier, Vec<MgoEvent>)>,
    eth_events_rx: mango_metrics::metered_channel::Receiver<(EthAddress, u64, Vec<EthLog>)>,
    store: Arc<BridgeOrchestratorTables>,
}

impl<C> BridgeOrchestrator<C>
where
    C: MgoClientInner + 'static,
{
    pub fn new(
        mgo_client: Arc<MgoClient<C>>,
        mgo_events_rx: mango_metrics::metered_channel::Receiver<(Identifier, Vec<MgoEvent>)>,
        eth_events_rx: mango_metrics::metered_channel::Receiver<(EthAddress, u64, Vec<EthLog>)>,
        store: Arc<BridgeOrchestratorTables>,
    ) -> Self {
        Self {
            _mgo_client: mgo_client,
            mgo_events_rx,
            eth_events_rx,
            store,
        }
    }

    pub fn run(
        self,
        bridge_action_executor: impl BridgeActionExecutorTrait,
    ) -> Vec<JoinHandle<()>> {
        tracing::info!("Starting BridgeOrchestrator");
        let mut task_handles = vec![];
        let store_clone = self.store.clone();

        // Spawn BridgeActionExecutor
        let (handles, executor_sender) = bridge_action_executor.run();
        task_handles.extend(handles);
        let executor_sender_clone = executor_sender.clone();
        task_handles.push(spawn_logged_monitored_task!(Self::run_mgo_watcher(
            store_clone,
            executor_sender_clone,
            self.mgo_events_rx,
        )));
        let store_clone = self.store.clone();
        task_handles.push(spawn_logged_monitored_task!(Self::run_eth_watcher(
            store_clone,
            executor_sender,
            self.eth_events_rx,
        )));
        // TODO: spawn bridge committee change watcher task
        task_handles
    }

    async fn run_mgo_watcher(
        store: Arc<BridgeOrchestratorTables>,
        executor_tx: mango_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        mut mgo_events_rx: mango_metrics::metered_channel::Receiver<(Identifier, Vec<MgoEvent>)>,
    ) {
        info!("Starting mgo watcher task");
        while let Some((identifier, events)) = mgo_events_rx.recv().await {
            if events.is_empty() {
                continue;
            }

            // TODO: skip events that are already processed (in DB and on chain)

            let bridge_events = events
                .iter()
                .map(MgoBridgeEvent::try_from_mgo_event)
                .collect::<BridgeResult<Vec<_>>>()
                .expect("Mgo Event could not be deserialzed to MgoBridgeEvent");

            let mut actions = vec![];
            for (mgo_event, opt_bridge_event) in events.iter().zip(bridge_events) {
                if opt_bridge_event.is_none() {
                    // TODO: we probably should not miss any events, warn for now.
                    warn!("Mgo event not recognized: {:?}", mgo_event);
                    continue;
                }
                // Unwrap safe: checked above
                let bridge_event: MgoBridgeEvent = opt_bridge_event.unwrap();

                if let Some(action) = bridge_event
                    .try_into_bridge_action(mgo_event.id.tx_digest, mgo_event.id.event_seq as u16)
                {
                    actions.push(action);
                }
                // TODO: handle non Action events
            }

            if !actions.is_empty() {
                // Write action to pending WAL
                store
                    .insert_pending_actions(&actions)
                    .expect("Store operation should not fail");
                for action in actions {
                    submit_to_executor(&executor_tx, action)
                        .await
                        .expect("Submit to executor should not fail");
                }
            }

            // Unwrap safe: in the beginning of the loop we checked that events is not empty
            let cursor = events.last().unwrap().id;
            store
                .update_mgo_event_cursor(identifier, cursor)
                .expect("Store operation should not fail");
        }
        panic!("Mgo event channel was closed");
    }

    async fn run_eth_watcher(
        store: Arc<BridgeOrchestratorTables>,
        executor_tx: mango_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        mut eth_events_rx: mango_metrics::metered_channel::Receiver<(
            ethers::types::Address,
            u64,
            Vec<EthLog>,
        )>,
    ) {
        info!("Starting eth watcher task");
        while let Some((contract, end_block, logs)) = eth_events_rx.recv().await {
            if logs.is_empty() {
                store
                    .update_eth_event_cursor(contract, end_block)
                    .expect("Store operation should not fail");
                continue;
            }

            // TODO: skip events that are not already processed (in DB and on chain)
            let bridge_events = logs
                .iter()
                .map(EthBridgeEvent::try_from_eth_log)
                .collect::<Vec<_>>();

            let mut actions = vec![];
            for (log, opt_bridge_event) in logs.iter().zip(bridge_events) {
                if opt_bridge_event.is_none() {
                    // TODO: we probably should not miss any events, warn for now.
                    warn!("Eth event not recognized: {:?}", log);
                    continue;
                }
                // Unwrap safe: checked above
                let bridge_event = opt_bridge_event.unwrap();

                if let Some(action) =
                    bridge_event.try_into_bridge_action(log.tx_hash, log.log_index_in_tx)
                {
                    actions.push(action);
                }
                // TODO: handle non Action events
            }
            if !actions.is_empty() {
                // Write action to pending WAL
                store
                    .insert_pending_actions(&actions)
                    .expect("Store operation should not fail");
                // Execution will remove the pending actions from DB when the action is completed.
                for action in actions {
                    submit_to_executor(&executor_tx, action)
                        .await
                        .expect("Submit to executor should not fail");
                }
            }

            store
                .update_eth_event_cursor(contract, end_block)
                .expect("Store operation should not fail");
        }
        panic!("Eth event channel was closed");
    }
}

#[cfg(test)]
mod tests {
    use crate::{test_utils::get_test_log_and_action, types::BridgeActionDigest};
    use ethers::types::{Address as EthAddress, TxHash};
    use prometheus::Registry;
    use std::str::FromStr;

    use super::*;
    use crate::{events::tests::get_test_mgo_event_and_action, mgo_mock_client::MgoMockClient};

    #[tokio::test]
    async fn test_mgo_watcher_task() {
        // Note: this test may fail beacuse of the following reasons:
        // the MgoEvent's struct tag does not match the ones in events.rs

        let (mgo_events_tx, mgo_events_rx, _eth_events_tx, eth_events_rx, mgo_client, store) =
            setup();

        let (executor, mut executor_requested_action_rx) = MockExecutor::new();
        // start orchestrator
        let _handles = BridgeOrchestrator::new(
            Arc::new(mgo_client),
            mgo_events_rx,
            eth_events_rx,
            store.clone(),
        )
        .run(executor);

        let identifier = Identifier::from_str("test_mgo_watcher_task").unwrap();
        let (mgo_event, bridge_action) = get_test_mgo_event_and_action(identifier.clone());
        mgo_events_tx
            .send((identifier.clone(), vec![mgo_event.clone()]))
            .await
            .unwrap();

        let start = std::time::Instant::now();
        // Executor should have received the action
        assert_eq!(
            executor_requested_action_rx.recv().await.unwrap(),
            bridge_action.digest()
        );
        loop {
            let actions = store.get_all_pending_actions().unwrap();
            if actions.is_empty() {
                if start.elapsed().as_secs() > 5 {
                    panic!("Timed out waiting for action to be written to WAL");
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                continue;
            }
            assert_eq!(actions.len(), 1);
            let action = actions.get(&bridge_action.digest()).unwrap();
            assert_eq!(action, &bridge_action);
            assert_eq!(
                store.get_mgo_event_cursors(&[identifier]).unwrap()[0].unwrap(),
                mgo_event.id,
            );
            break;
        }
    }

    #[tokio::test]
    async fn test_eth_watcher_task() {
        // Note: this test may fail beacuse of the following reasons:
        // 1. Log and BridgeAction returned from `get_test_log_and_action` are not in sync
        // 2. Log returned from `get_test_log_and_action` is not parseable log (not abigen!, check abi.rs)

        let (_mgo_events_tx, mgo_events_rx, eth_events_tx, eth_events_rx, mgo_client, store) =
            setup();
        let (executor, mut executor_requested_action_rx) = MockExecutor::new();
        // start orchestrator
        let _handles = BridgeOrchestrator::new(
            Arc::new(mgo_client),
            mgo_events_rx,
            eth_events_rx,
            store.clone(),
        )
        .run(executor);
        let address = EthAddress::random();
        let (log, bridge_action) = get_test_log_and_action(address, TxHash::random(), 10);
        let log_index_in_tx = 10;
        let log_block_num = log.block_number.unwrap().as_u64();
        let eth_log = EthLog {
            log: log.clone(),
            tx_hash: log.transaction_hash.unwrap(),
            block_number: log_block_num,
            log_index_in_tx,
        };
        let end_block_num = log_block_num + 15;

        eth_events_tx
            .send((address, end_block_num, vec![eth_log.clone()]))
            .await
            .unwrap();

        // Executor should have received the action
        assert_eq!(
            executor_requested_action_rx.recv().await.unwrap(),
            bridge_action.digest()
        );
        let start = std::time::Instant::now();
        loop {
            let actions = store.get_all_pending_actions().unwrap();
            if actions.is_empty() {
                if start.elapsed().as_secs() > 5 {
                    panic!("Timed out waiting for action to be written to WAL");
                }
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
                continue;
            }
            assert_eq!(actions.len(), 1);
            let action = actions.get(&bridge_action.digest()).unwrap();
            assert_eq!(action, &bridge_action);
            assert_eq!(
                store.get_eth_event_cursors(&[address]).unwrap()[0].unwrap(),
                end_block_num,
            );
            break;
        }
    }

    #[allow(clippy::type_complexity)]
    fn setup() -> (
        mango_metrics::metered_channel::Sender<(Identifier, Vec<MgoEvent>)>,
        mango_metrics::metered_channel::Receiver<(Identifier, Vec<MgoEvent>)>,
        mango_metrics::metered_channel::Sender<(EthAddress, u64, Vec<EthLog>)>,
        mango_metrics::metered_channel::Receiver<(EthAddress, u64, Vec<EthLog>)>,
        MgoClient<MgoMockClient>,
        Arc<BridgeOrchestratorTables>,
    ) {
        telemetry_subscribers::init_for_testing();
        let registry = Registry::new();
        mango_metrics::init_metrics(&registry);

        // TODO: remove once we don't rely on env var to get package id
        std::env::set_var("BRIDGE_PACKAGE_ID", "0x0b");

        let temp_dir = tempfile::tempdir().unwrap();
        let store = BridgeOrchestratorTables::new(temp_dir.path());

        let mock_client = MgoMockClient::default();
        let mgo_client = MgoClient::new_for_testing(mock_client.clone());

        let (eth_events_tx, eth_events_rx) = mango_metrics::metered_channel::channel(
            100,
            &mango_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["unit_test_eth_events_queue"]),
        );

        let (mgo_events_tx, mgo_events_rx) = mango_metrics::metered_channel::channel(
            100,
            &mango_metrics::get_metrics()
                .unwrap()
                .channels
                .with_label_values(&["unit_test_mgo_events_queue"]),
        );

        (
            mgo_events_tx,
            mgo_events_rx,
            eth_events_tx,
            eth_events_rx,
            mgo_client,
            store,
        )
    }

    /// A `BridgeActionExecutorTrait` implementation that only tracks the submitted actions.
    struct MockExecutor {
        requested_transactions_tx: tokio::sync::broadcast::Sender<BridgeActionDigest>,
    }

    impl MockExecutor {
        fn new() -> (Self, tokio::sync::broadcast::Receiver<BridgeActionDigest>) {
            let (tx, rx) = tokio::sync::broadcast::channel(100);
            (
                Self {
                    requested_transactions_tx: tx,
                },
                rx,
            )
        }
    }

    impl BridgeActionExecutorTrait for MockExecutor {
        fn run(
            self,
        ) -> (
            Vec<tokio::task::JoinHandle<()>>,
            mango_metrics::metered_channel::Sender<BridgeActionExecutionWrapper>,
        ) {
            let (tx, mut rx) =
                mango_metrics::metered_channel::channel::<BridgeActionExecutionWrapper>(
                    100,
                    &mango_metrics::get_metrics()
                        .unwrap()
                        .channels
                        .with_label_values(&["unit_test_mock_executor"]),
                );

            let handles = tokio::spawn(async move {
                while let Some(action) = rx.recv().await {
                    self.requested_transactions_tx
                        .send(action.0.digest())
                        .unwrap();
                }
            });
            (vec![handles], tx)
        }
    }
}
