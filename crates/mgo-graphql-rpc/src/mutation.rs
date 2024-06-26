// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use crate::types::transaction_block_effects::TransactionBlockEffectsKind;
use crate::{
    error::Error, types::execution_result::ExecutionResult,
    types::transaction_block_effects::TransactionBlockEffects,
};
use async_graphql::*;
use fastcrypto::encoding::Encoding;
use fastcrypto::{encoding::Base64, traits::ToFromBytes};
use mgo_json_rpc_types::MgoTransactionBlockResponseOptions;
use mgo_sdk::MgoClient;
use mgo_types::effects::TransactionEffects as NativeTransactionEffects;
use mgo_types::event::Event as NativeEvent;
use mgo_types::quorum_driver_types::ExecuteTransactionRequestType;
use mgo_types::transaction::SenderSignedData;
use mgo_types::{signature::GenericSignature, transaction::Transaction};
pub struct Mutation;

/// Mutations are used to write to the Mgo network.
#[Object]
impl Mutation {
    /// Execute a transaction, committing its effects on chain.
    ///
    /// `txBytes` is a `TransactionData` struct that has been BCS-encoded
    ///     and then Base64-encoded.
    /// `signatures` are a list of `flag || signature || pubkey` bytes,
    ///     Base64-encoded.
    ///
    /// Waits until the transaction has been finalized on chain to return
    /// its transaction digest.  If the transaction could not be
    /// finalized, returns the errors that prevented it, instead.
    async fn execute_transaction_block(
        &self,
        ctx: &Context<'_>,
        tx_bytes: String,
        signatures: Vec<String>,
    ) -> Result<ExecutionResult> {
        let mgo_sdk_client: &Option<MgoClient> = ctx
            .data()
            .map_err(|_| Error::Internal("Unable to fetch Mgo SDK client".to_string()))
            .extend()?;
        let mgo_sdk_client = mgo_sdk_client
            .as_ref()
            .ok_or_else(|| Error::Internal("Mgo SDK client not initialized".to_string()))
            .extend()?;
        let tx_data = bcs::from_bytes(
            &Base64::decode(&tx_bytes)
                .map_err(|e| {
                    Error::Client(format!(
                        "Unable to deserialize transaction bytes from Base64: {e}"
                    ))
                })
                .extend()?,
        )
        .map_err(|e| {
            Error::Client(format!(
                "Unable to deserialize transaction bytes as BCS: {e}"
            ))
        })
        .extend()?;

        let mut sigs = Vec::new();
        for sig in signatures {
            sigs.push(
                GenericSignature::from_bytes(
                    &Base64::decode(&sig)
                        .map_err(|e| {
                            Error::Client(format!(
                                "Unable to deserialize signature bytes {sig} from Base64: {e}"
                            ))
                        })
                        .extend()?,
                )
                .map_err(|e| Error::Client(format!("Unable to create signature from bytes: {e}")))
                .extend()?,
            );
        }
        let transaction = Transaction::from_generic_sig_data(tx_data, sigs);
        let options = MgoTransactionBlockResponseOptions::new()
            .with_events()
            .with_raw_input()
            .with_raw_effects();

        let result = mgo_sdk_client
            .quorum_driver_api()
            .execute_transaction_block(
                transaction,
                options,
                Some(ExecuteTransactionRequestType::WaitForEffectsCert),
            )
            .await
            // TODO: use proper error type as this could be a client error or internal error
            // depending on the specific error returned
            .map_err(|e| Error::Internal(format!("Unable to execute transaction: {e}")))
            .extend()?;

        let native: NativeTransactionEffects = bcs::from_bytes(&result.raw_effects)
            .map_err(|e| Error::Internal(format!("Unable to deserialize transaction effects: {e}")))
            .extend()?;
        let tx_data: SenderSignedData = bcs::from_bytes(&result.raw_transaction)
            .map_err(|e| Error::Internal(format!("Unable to deserialize transaction data: {e}")))
            .extend()?;

        let events = result
            .events
            .ok_or_else(|| {
                Error::Internal("No events are returned from transaction execution".to_string())
            })?
            .data
            .into_iter()
            .map(|e| NativeEvent {
                package_id: e.package_id,
                transaction_module: e.transaction_module,
                sender: e.sender,
                type_: e.type_,
                contents: e.bcs,
            })
            .collect();

        Ok(ExecutionResult {
            errors: if result.errors.is_empty() {
                None
            } else {
                Some(result.errors)
            },
            effects: TransactionBlockEffects {
                kind: TransactionBlockEffectsKind::Executed {
                    tx_data,
                    native,
                    events,
                },
                // set to u64::MAX, as the executed transaction has not been indexed yet
                checkpoint_viewed_at: u64::MAX,
            },
        })
    }
}
