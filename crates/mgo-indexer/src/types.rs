// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use itertools::Itertools;

use mgo_json_rpc_types::{
    BalanceChange, ObjectChange, MgoCommand, MgoTransactionBlock, MgoTransactionBlockDataAPI,
    MgoTransactionBlockEffects, MgoTransactionBlockEffectsAPI, MgoTransactionBlockEvents,
    MgoTransactionBlockKind, MgoTransactionBlockResponse, MgoTransactionBlockResponseOptions,
};
use mgo_types::digests::TransactionDigest;
use mgo_types::messages_checkpoint::CheckpointSequenceNumber;
use mgo_types::object::Owner;
use mgo_types::storage::WriteKind;
use mgo_types::transaction::{SenderSignedData, TransactionDataAPI};

use crate::errors::IndexerError;
use crate::models::transaction_index::{ChangedObject, InputObject, MoveCall, Recipient};

const CREATED_OBJECT_CHANGE_TYPE: &str = "created";
const MUTATED_OBJECT_CHANGE_TYPE: &str = "mutated";
const UNWRAPPED_OBJECT_CHANGE_TYPE: &str = "unwrapped";

pub fn write_kind_to_str(write_kind: WriteKind) -> &'static str {
    match write_kind {
        WriteKind::Mutate => MUTATED_OBJECT_CHANGE_TYPE,
        WriteKind::Create => CREATED_OBJECT_CHANGE_TYPE,
        WriteKind::Unwrap => UNWRAPPED_OBJECT_CHANGE_TYPE,
    }
}

#[derive(Debug, Clone)]
pub struct CheckpointTransactionBlockResponse {
    pub digest: TransactionDigest,
    /// Transaction input data
    pub transaction: MgoTransactionBlock,
    pub raw_transaction: Vec<u8>,
    pub effects: MgoTransactionBlockEffects,
    pub events: MgoTransactionBlockEvents,
    pub timestamp_ms: u64,
    pub confirmed_local_execution: Option<bool>,
    pub checkpoint: CheckpointSequenceNumber,
}

impl TryFrom<MgoTransactionBlockResponse> for CheckpointTransactionBlockResponse {
    type Error = anyhow::Error;

    fn try_from(response: MgoTransactionBlockResponse) -> Result<Self, Self::Error> {
        let MgoTransactionBlockResponse {
            digest,
            transaction,
            raw_transaction,
            effects,
            events,
            object_changes: _,
            balance_changes: _,
            timestamp_ms,
            confirmed_local_execution,
            checkpoint,
            errors,
            raw_effects: _,
        } = response;

        let transaction = transaction.ok_or_else(|| {
            anyhow::anyhow!(
                "Transaction is None in MgoTransactionBlockFullResponse of digest {:?}.",
                digest
            )
        })?;
        let effects = effects.ok_or_else(|| {
            anyhow::anyhow!(
                "Effects is None in MgoTransactionBlockFullResponse of digest {:?}.",
                digest
            )
        })?;
        let events = events.ok_or_else(|| {
            anyhow::anyhow!(
                "Events is None in MgoTransactionBlockFullResponse of digest {:?}.",
                digest
            )
        })?;
        let timestamp_ms = timestamp_ms.ok_or_else(|| {
            anyhow::anyhow!(
                "TimestampMs is None in MgoTransactionBlockFullResponse of digest {:?}.",
                digest
            )
        })?;
        let checkpoint = checkpoint.ok_or_else(|| {
            anyhow::anyhow!(
                "Checkpoint is None in MgoTransactionBlockFullResponse of digest {:?}.",
                digest
            )
        })?;
        if raw_transaction.is_empty() {
            return Err(anyhow::anyhow!(
                "Unexpected empty RawTransaction in MgoTransactionBlockFullResponse of digest {:?}.",
                digest
            ));
        }
        if !errors.is_empty() {
            return Err(anyhow::anyhow!(
                "Errors in MgoTransactionBlockFullResponse of digest {:?}: {:?}",
                digest,
                errors
            ));
        }

        Ok(CheckpointTransactionBlockResponse {
            digest,
            transaction,
            raw_transaction,
            effects,
            events,
            timestamp_ms,
            confirmed_local_execution,
            checkpoint,
        })
    }
}

pub struct AddressData {
    pub account_address: String,
    pub transaction_digest: String,
    pub timestamp_ms: i64,
}

impl CheckpointTransactionBlockResponse {
    pub fn get_input_objects(&self, epoch: u64) -> Result<Vec<InputObject>, IndexerError> {
        let raw_tx = self.raw_transaction.clone();
        let sender_signed_data: SenderSignedData = bcs::from_bytes(&raw_tx).map_err(|err| {
            IndexerError::SerdeError(format!(
                "Failed converting transaction {:?} from bytes {:?} to SenderSignedData with error: {:?}",
                self.digest.clone(), raw_tx, err
            ))
        })?;
        let input_objects: Vec<InputObject> =
            sender_signed_data
                .transaction_data()
                .input_objects()
                .map_err(|err| {
                    IndexerError::InvalidArgumentError(format!(
                    "Failed getting input objects of transaction {:?} from {:?} with error: {:?}",
                    self.digest.clone(), raw_tx, err
                ))
                })?
                .into_iter()
                .map(|obj_kind| InputObject {
                    id: None,
                    transaction_digest: self.digest.to_string(),
                    checkpoint_sequence_number: self.checkpoint as i64,
                    epoch: epoch as i64,
                    object_id: obj_kind.object_id().to_string(),
                    object_version: obj_kind.version().map(|v| v.value() as i64),
                })
                .collect();
        Ok(input_objects)
    }

    pub fn get_changed_objects(&self, epoch: u64) -> Vec<ChangedObject> {
        let created = self
            .effects
            .created()
            .iter()
            .map(|o| (o, CREATED_OBJECT_CHANGE_TYPE));
        let mutated = self
            .effects
            .mutated()
            .iter()
            .map(|o| (o, MUTATED_OBJECT_CHANGE_TYPE));
        let unwrapped = self
            .effects
            .unwrapped()
            .iter()
            .map(|o| (o, UNWRAPPED_OBJECT_CHANGE_TYPE));
        created
            .chain(mutated)
            .chain(unwrapped)
            .map(|(obj_ref, change_type)| ChangedObject {
                id: None,
                transaction_digest: self.digest.to_string(),
                checkpoint_sequence_number: self.checkpoint as i64,
                epoch: epoch as i64,
                object_id: obj_ref.reference.object_id.to_string(),
                object_change_type: change_type.to_string(),
                object_version: obj_ref.reference.version.value() as i64,
            })
            .collect()
    }

    pub fn get_move_calls(&self, epoch: u64) -> Vec<MoveCall> {
        let tx_kind = self.transaction.data.transaction();
        let sender = self.transaction.data.sender();
        match tx_kind {
            MgoTransactionBlockKind::ProgrammableTransaction(pt) => {
                let move_calls: Vec<MoveCall> = pt
                    .commands
                    .clone()
                    .into_iter()
                    .filter_map(move |command| match command {
                        MgoCommand::MoveCall(m) => Some(MoveCall {
                            id: None,
                            transaction_digest: self.digest.to_string(),
                            checkpoint_sequence_number: self.checkpoint as i64,
                            epoch: epoch as i64,
                            sender: sender.to_string(),
                            move_package: m.package.to_string(),
                            move_module: m.module,
                            move_function: m.function,
                        }),
                        _ => None,
                    })
                    .collect();
                Some(move_calls)
            }
            _ => None,
        }
        .unwrap_or_default()
    }

    pub fn get_recipients(&self, epoch: u64) -> Vec<Recipient> {
        let created = self.effects.created().iter();
        let mutated = self.effects.mutated().iter();
        let unwrapped = self.effects.unwrapped().iter();
        created
            .chain(mutated)
            .chain(unwrapped)
            .filter_map(|obj_ref| match obj_ref.owner {
                Owner::AddressOwner(address) => Some(address.to_string()),
                _ => None,
            })
            .unique()
            .map(|recipient| Recipient {
                id: None,
                transaction_digest: self.digest.to_string(),
                checkpoint_sequence_number: self.checkpoint as i64,
                epoch: epoch as i64,
                sender: self.transaction.data.sender().to_string(),
                recipient,
            })
            .collect()
    }

    pub fn get_from_and_to_addresses(&self) -> Vec<AddressData> {
        let created = self.effects.created().iter();
        let mutated = self.effects.mutated().iter();
        let unwrapped = self.effects.unwrapped().iter();
        let mut addresses = created
            .chain(mutated)
            .chain(unwrapped)
            .filter_map(|obj_ref| match obj_ref.owner {
                Owner::AddressOwner(address) => Some(address.to_string()),
                _ => None,
            })
            .collect::<Vec<String>>();
        addresses.push(self.transaction.data.sender().to_string());
        addresses
            .into_iter()
            .map(|r| AddressData {
                account_address: r,
                transaction_digest: self.digest.to_string(),
                timestamp_ms: self.timestamp_ms as i64,
            })
            .collect::<Vec<AddressData>>()
    }

    pub fn get_from_address(&self) -> AddressData {
        AddressData {
            account_address: self.transaction.data.sender().to_string(),
            transaction_digest: self.digest.to_string(),
            timestamp_ms: self.timestamp_ms as i64,
        }
    }
}

pub struct TemporaryTransactionBlockResponseStore {
    pub digest: TransactionDigest,
    /// Transaction input data
    pub transaction: MgoTransactionBlock,
    pub raw_transaction: Vec<u8>,
    pub effects: MgoTransactionBlockEffects,
    pub events: MgoTransactionBlockEvents,
    pub object_changes: Option<Vec<ObjectChange>>,
    pub balance_changes: Option<Vec<BalanceChange>>,
    pub timestamp_ms: Option<u64>,
    pub confirmed_local_execution: Option<bool>,
    pub checkpoint: Option<CheckpointSequenceNumber>,
}

impl From<CheckpointTransactionBlockResponse> for TemporaryTransactionBlockResponseStore {
    fn from(value: CheckpointTransactionBlockResponse) -> Self {
        let CheckpointTransactionBlockResponse {
            digest,
            transaction,
            raw_transaction,
            effects,
            events,
            timestamp_ms,
            confirmed_local_execution,
            checkpoint,
        } = value;

        TemporaryTransactionBlockResponseStore {
            digest,
            transaction,
            raw_transaction,
            effects,
            events,
            object_changes: None,
            balance_changes: None,
            timestamp_ms: Some(timestamp_ms),
            confirmed_local_execution,
            checkpoint: Some(checkpoint),
        }
    }
}

// MgoTransactionBlockResponseWithOptions is only used on the reading path
pub struct MgoTransactionBlockResponseWithOptions {
    pub response: MgoTransactionBlockResponse,
    pub options: MgoTransactionBlockResponseOptions,
}

impl From<MgoTransactionBlockResponseWithOptions> for MgoTransactionBlockResponse {
    fn from(value: MgoTransactionBlockResponseWithOptions) -> Self {
        let MgoTransactionBlockResponseWithOptions { response, options } = value;

        MgoTransactionBlockResponse {
            digest: response.digest,
            transaction: options.show_input.then_some(response.transaction).flatten(),
            raw_transaction: options
                .show_raw_input
                .then_some(response.raw_transaction)
                .unwrap_or_default(),
            effects: options.show_effects.then_some(response.effects).flatten(),
            events: options.show_events.then_some(response.events).flatten(),
            object_changes: options
                .show_object_changes
                .then_some(response.object_changes)
                .flatten(),
            balance_changes: options
                .show_balance_changes
                .then_some(response.balance_changes)
                .flatten(),
            timestamp_ms: response.timestamp_ms,
            confirmed_local_execution: response.confirmed_local_execution,
            checkpoint: response.checkpoint,
            errors: vec![],
            raw_effects: options
                .show_raw_effects
                .then_some(response.raw_effects)
                .unwrap_or_default(),
        }
    }
}
