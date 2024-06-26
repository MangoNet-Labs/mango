// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

//! A mock implementation of Mgo JSON-RPC client.

use crate::error::{BridgeError, BridgeResult};
use async_trait::async_trait;
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use mgo_json_rpc_types::MgoTransactionBlockResponse;
use mgo_json_rpc_types::{EventFilter, EventPage, MgoEvent};
use mgo_types::base_types::ObjectID;
use mgo_types::base_types::ObjectRef;
use mgo_types::digests::TransactionDigest;
use mgo_types::event::EventID;
use mgo_types::gas_coin::GasCoin;
use mgo_types::object::Owner;
use mgo_types::transaction::Transaction;
use mgo_types::Identifier;

use crate::mgo_client::MgoClientInner;
use crate::types::{BridgeAction, BridgeActionDigest, BridgeActionStatus, MoveTypeBridgeCommittee};

/// Mock client used in test environments.
#[allow(clippy::type_complexity)]
#[derive(Clone, Debug)]
pub struct MgoMockClient {
    // the top two fields do not change during tests so we don't need them to be Arc<Mutex>>
    chain_identifier: String,
    latest_checkpoint_sequence_number: u64,
    events: Arc<Mutex<HashMap<(ObjectID, Identifier, EventID), EventPage>>>,
    past_event_query_params: Arc<Mutex<VecDeque<(ObjectID, Identifier, EventID)>>>,
    events_by_tx_digest:
        Arc<Mutex<HashMap<TransactionDigest, Result<Vec<MgoEvent>, mgo_sdk::error::Error>>>>,
    transaction_responses:
        Arc<Mutex<HashMap<TransactionDigest, BridgeResult<MgoTransactionBlockResponse>>>>,
    wildcard_transaction_response: Arc<Mutex<Option<BridgeResult<MgoTransactionBlockResponse>>>>,
    get_object_info: Arc<Mutex<HashMap<ObjectID, (GasCoin, ObjectRef, Owner)>>>,
    onchain_status: Arc<Mutex<HashMap<BridgeActionDigest, BridgeActionStatus>>>,

    requested_transactions_tx: tokio::sync::broadcast::Sender<TransactionDigest>,
}

impl MgoMockClient {
    pub fn default() -> Self {
        Self {
            chain_identifier: "".to_string(),
            latest_checkpoint_sequence_number: 0,
            events: Default::default(),
            past_event_query_params: Default::default(),
            events_by_tx_digest: Default::default(),
            transaction_responses: Default::default(),
            wildcard_transaction_response: Default::default(),
            get_object_info: Default::default(),
            onchain_status: Default::default(),
            requested_transactions_tx: tokio::sync::broadcast::channel(10000).0,
        }
    }

    pub fn add_event_response(
        &self,
        package: ObjectID,
        module: Identifier,
        cursor: EventID,
        events: EventPage,
    ) {
        self.events
            .lock()
            .unwrap()
            .insert((package, module, cursor), events);
    }

    pub fn add_events_by_tx_digest(&self, tx_digest: TransactionDigest, events: Vec<MgoEvent>) {
        self.events_by_tx_digest
            .lock()
            .unwrap()
            .insert(tx_digest, Ok(events));
    }

    pub fn add_events_by_tx_digest_error(&self, tx_digest: TransactionDigest) {
        self.events_by_tx_digest.lock().unwrap().insert(
            tx_digest,
            Err(mgo_sdk::error::Error::DataError("".to_string())),
        );
    }

    pub fn add_transaction_response(
        &self,
        tx_digest: TransactionDigest,
        response: BridgeResult<MgoTransactionBlockResponse>,
    ) {
        self.transaction_responses
            .lock()
            .unwrap()
            .insert(tx_digest, response);
    }

    pub fn set_action_onchain_status(&self, action: &BridgeAction, status: BridgeActionStatus) {
        self.onchain_status
            .lock()
            .unwrap()
            .insert(action.digest(), status);
    }

    pub fn set_wildcard_transaction_response(
        &self,
        response: BridgeResult<MgoTransactionBlockResponse>,
    ) {
        *self.wildcard_transaction_response.lock().unwrap() = Some(response);
    }

    pub fn add_gas_object_info(&self, gas_coin: GasCoin, object_ref: ObjectRef, owner: Owner) {
        self.get_object_info
            .lock()
            .unwrap()
            .insert(object_ref.0, (gas_coin, object_ref, owner));
    }

    pub fn subscribe_to_requested_transactions(
        &self,
    ) -> tokio::sync::broadcast::Receiver<TransactionDigest> {
        self.requested_transactions_tx.subscribe()
    }
}

#[async_trait]
impl MgoClientInner for MgoMockClient {
    type Error = mgo_sdk::error::Error;

    // Unwraps in this function: We assume the responses are pre-populated
    // by the test before calling into this function.
    async fn query_events(
        &self,
        query: EventFilter,
        cursor: EventID,
    ) -> Result<EventPage, Self::Error> {
        let events = self.events.lock().unwrap();
        match query {
            EventFilter::MoveEventModule { package, module } => {
                self.past_event_query_params.lock().unwrap().push_back((
                    package,
                    module.clone(),
                    cursor,
                ));
                Ok(events
                    .get(&(package, module.clone(), cursor))
                    .cloned()
                    .unwrap_or_else(|| {
                        panic!(
                            "No preset events found for package: {:?}, module: {:?}, cursor: {:?}",
                            package, module, cursor
                        )
                    }))
            }
            _ => unimplemented!(),
        }
    }

    async fn get_events_by_tx_digest(
        &self,
        tx_digest: TransactionDigest,
    ) -> Result<Vec<MgoEvent>, Self::Error> {
        let events = self.events_by_tx_digest.lock().unwrap();

        match events
            .get(&tx_digest)
            .unwrap_or_else(|| panic!("No preset events found for tx_digest: {:?}", tx_digest))
        {
            Ok(events) => Ok(events.clone()),
            // mgo_sdk::error::Error is not Clone
            Err(_) => Err(mgo_sdk::error::Error::DataError("".to_string())),
        }
    }

    async fn get_chain_identifier(&self) -> Result<String, Self::Error> {
        Ok(self.chain_identifier.clone())
    }

    async fn get_latest_checkpoint_sequence_number(&self) -> Result<u64, Self::Error> {
        Ok(self.latest_checkpoint_sequence_number)
    }

    async fn get_bridge_committee(&self) -> Result<MoveTypeBridgeCommittee, Self::Error> {
        unimplemented!()
    }

    async fn get_action_onchain_status(
        &self,
        action: &BridgeAction,
    ) -> Result<BridgeActionStatus, BridgeError> {
        Ok(self
            .onchain_status
            .lock()
            .unwrap()
            .get(&action.digest())
            .cloned()
            .unwrap_or(BridgeActionStatus::Pending))
    }

    async fn execute_transaction_block_with_effects(
        &self,
        tx: Transaction,
    ) -> Result<MgoTransactionBlockResponse, BridgeError> {
        self.requested_transactions_tx.send(*tx.digest()).unwrap();
        match self.transaction_responses.lock().unwrap().get(tx.digest()) {
            Some(response) => response.clone(),
            None => self
                .wildcard_transaction_response
                .lock()
                .unwrap()
                .clone()
                .unwrap_or_else(|| panic!("No preset transaction response found for tx: {:?}", tx)),
        }
    }

    async fn get_gas_data_panic_if_not_gas(
        &self,
        gas_object_id: ObjectID,
    ) -> (GasCoin, ObjectRef, Owner) {
        self.get_object_info
            .lock()
            .unwrap()
            .get(&gas_object_id)
            .cloned()
            .unwrap_or_else(|| {
                panic!(
                    "No preset gas object info found for gas_object_id: {:?}",
                    gas_object_id
                )
            })
    }
}
