// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::error::{BridgeError, BridgeResult};
use crate::types::{BridgeAction, EthToMgoBridgeAction};
use crate::types::{BridgeChainId, EthLog, TokenId};
use ethers::{
    abi::RawLog,
    contract::{abigen, EthLogDecode},
    types::Address as EthAddress,
};
use serde::{Deserialize, Serialize};
use mgo_types::base_types::MgoAddress;

// TODO: write a macro to handle variants

// TODO: Add other events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EthBridgeEvent {
    EthMgoBridgeEvents(EthMgoBridgeEvents),
}

abigen!(
    EthMgoBridge,
    "abi/mgo_bridge.json",
    event_derives(serde::Deserialize, serde::Serialize)
);

impl EthBridgeEvent {
    pub fn try_from_eth_log(log: &EthLog) -> Option<EthBridgeEvent> {
        let raw_log = RawLog {
            topics: log.log.topics.clone(),
            data: log.log.data.to_vec(),
        };

        if let Ok(decoded) = EthMgoBridgeEvents::decode_log(&raw_log) {
            return Some(EthBridgeEvent::EthMgoBridgeEvents(decoded));
        }

        // TODO: try other variants
        None
    }
}

impl EthBridgeEvent {
    pub fn try_into_bridge_action(
        self,
        eth_tx_hash: ethers::types::H256,
        eth_event_index: u16,
    ) -> Option<BridgeAction> {
        match self {
            EthBridgeEvent::EthMgoBridgeEvents(event) => {
                match event {
                    EthMgoBridgeEvents::TokensBridgedToMgoFilter(event) => {
                        let bridge_event = match EthToMgoTokenBridgeV1::try_from(&event) {
                            Ok(bridge_event) => bridge_event,
                            // This only happens when solidity code does not align with rust code.
                            // When this happens in production, there is a risk of stuck bridge transfers.
                            // We log error here.
                            // TODO: add metrics and alert
                            Err(e) => {
                                tracing::error!(?eth_tx_hash, eth_event_index, "Failed to convert TokensBridgedToMgo log to EthToMgoTokenBridgeV1. This indicates incorrect parameters or a bug in the code: {:?}. Err: {:?}", event, e);
                                return None;
                            }
                        };

                        Some(BridgeAction::EthToMgoBridgeAction(EthToMgoBridgeAction {
                            eth_tx_hash,
                            eth_event_index,
                            eth_bridge_event: bridge_event,
                        }))
                    }
                    _ => None,
                }
            }
        }
    }
}

// Sanity checked version of TokensBridgedToMgoFilter
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct EthToMgoTokenBridgeV1 {
    pub nonce: u64,
    pub mgo_chain_id: BridgeChainId,
    pub eth_chain_id: BridgeChainId,
    pub mgo_address: MgoAddress,
    pub eth_address: EthAddress,
    pub token_id: TokenId,
    pub amount: u64,
}

impl TryFrom<&TokensBridgedToMgoFilter> for EthToMgoTokenBridgeV1 {
    type Error = BridgeError;
    fn try_from(event: &TokensBridgedToMgoFilter) -> BridgeResult<Self> {
        Ok(Self {
            nonce: event.nonce,
            mgo_chain_id: BridgeChainId::try_from(event.destination_chain_id)?,
            eth_chain_id: BridgeChainId::try_from(event.source_chain_id)?,
            mgo_address: MgoAddress::from_bytes(event.target_address.as_ref())?,
            eth_address: event.source_address,
            token_id: TokenId::try_from(event.token_code)?,
            amount: event.mgo_adjusted_amount,
        })
    }
}
