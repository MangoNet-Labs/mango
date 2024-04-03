// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use fastcrypto::encoding::Base64;
use futures::stream;
use futures::StreamExt;
use futures_core::Stream;
use jsonrpsee::core::client::Subscription;
use std::collections::BTreeMap;
use std::future;
use std::sync::Arc;
use std::time::Instant;
use mgo_json_rpc_types::DevInspectArgs;
use mgo_json_rpc_types::MgoData;

use crate::error::{Error, MgoRpcResult};
use crate::RpcClient;
use mgo_json_rpc_api::{
    CoinReadApiClient, GovernanceReadApiClient, IndexerApiClient, MoveUtilsClient, ReadApiClient,
    WriteApiClient,
};
use mgo_json_rpc_types::{
    Balance, Checkpoint, CheckpointId, Coin, CoinPage, DelegatedStake, DevInspectResults,
    DryRunTransactionBlockResponse, DynamicFieldPage, EventFilter, EventPage, ObjectsPage,
    ProtocolConfigResponse, MgoCoinMetadata, MgoCommittee, MgoEvent, MgoGetPastObjectRequest,
    MgoMoveNormalizedModule, MgoObjectDataOptions, MgoObjectResponse, MgoObjectResponseQuery,
    MgoPastObjectResponse, MgoTransactionBlockEffects, MgoTransactionBlockResponse,
    MgoTransactionBlockResponseOptions, MgoTransactionBlockResponseQuery, TransactionBlocksPage,
    TransactionFilter,
};
use mgo_json_rpc_types::{CheckpointPage, MgoLoadedChildObjectsResponse};
use mgo_types::balance::Supply;
use mgo_types::base_types::{ObjectID, SequenceNumber, MgoAddress, TransactionDigest};
use mgo_types::dynamic_field::DynamicFieldName;
use mgo_types::event::EventID;
use mgo_types::messages_checkpoint::CheckpointSequenceNumber;
use mgo_types::quorum_driver_types::ExecuteTransactionRequestType;
use mgo_types::mgo_serde::BigInt;
use mgo_types::mgo_system_state::mgo_system_state_summary::MgoSystemStateSummary;
use mgo_types::transaction::{Transaction, TransactionData, TransactionKind};

const WAIT_FOR_LOCAL_EXECUTION_RETRY_COUNT: u8 = 3;

/// The main read API structure with functions for retrieving data about different objects and transactions
#[derive(Debug)]
pub struct ReadApi {
    api: Arc<RpcClient>,
}

impl ReadApi {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }
    /// Return a paginated response with the objects owned by the given address, or an error upon failure.
    ///
    /// Note that if the address owns more than `QUERY_MAX_RESULT_LIMIT` objects (default is 50),
    /// the pagination is not accurate, because previous page may have been updated when the next page is fetched.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::MgoAddress;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let owned_objects = mgo
    ///         .read_api()
    ///         .get_owned_objects(address, None, None, None)
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_owned_objects(
        &self,
        address: MgoAddress,
        query: Option<MgoObjectResponseQuery>,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> MgoRpcResult<ObjectsPage> {
        Ok(self
            .api
            .http
            .get_owned_objects(address, query, cursor, limit)
            .await?)
    }

    /// Return a paginated response with the dynamic fields owned by the given [ObjectID], or an error upon failure.
    ///
    /// The return type is a list of `DynamicFieldInfo` objects, where the field name is always present,
    /// represented as a Move `Value`.
    ///
    /// If the field is a dynamic field, returns the ID of the Field object (which contains both the name and the value).
    /// If the field is a dynamic object field, it returns the ID of the Object (the value of the field).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::{ObjectID, MgoAddress};
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let owned_objects = mgo
    ///         .read_api()
    ///         .get_owned_objects(address, None, None, None)
    ///         .await?;
    ///     // this code example assumes that there are previous owned objects
    ///     let object = owned_objects.data.get(0).expect(&format!(
    ///         "No owned objects for this address {}",
    ///         address
    ///     ));
    ///     let object_data = object.data.as_ref().expect(&format!(
    ///         "No object data for this MgoObjectResponse {:?}",
    ///         object
    ///     ));
    ///     let object_id = object_data.object_id;
    ///     let dynamic_fields = mgo
    ///         .read_api()
    ///         .get_dynamic_fields(object_id, None, None)
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_dynamic_fields(
        &self,
        object_id: ObjectID,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> MgoRpcResult<DynamicFieldPage> {
        Ok(self
            .api
            .http
            .get_dynamic_fields(object_id, cursor, limit)
            .await?)
    }

    /// Return the dynamic field object information for a specified object.
    pub async fn get_dynamic_field_object(
        &self,
        parent_object_id: ObjectID,
        name: DynamicFieldName,
    ) -> MgoRpcResult<MgoObjectResponse> {
        Ok(self
            .api
            .http
            .get_dynamic_field_object(parent_object_id, name)
            .await?)
    }

    /// Return a parsed past object for the provided [ObjectID] and version, or an error upon failure.
    ///
    /// An object's version increases (though it is not guaranteed that it increases always by 1) when
    /// the object is mutated. A past object can be used to understand how the object changed over time,
    /// i.e. what was the total balance at a specific version.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::{ObjectID, MgoAddress};
    /// use mgo_json_rpc_types::MgoObjectDataOptions;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let owned_objects = mgo
    ///         .read_api()
    ///         .get_owned_objects(address, None, None, None)
    ///         .await?;
    ///     // this code example assumes that there are previous owned objects
    ///     let object = owned_objects.data.get(0).expect(&format!(
    ///         "No owned objects for this address {}",
    ///         address
    ///     ));
    ///     let object_data = object.data.as_ref().expect(&format!(
    ///         "No object data for this MgoObjectResponse {:?}",
    ///         object
    ///     ));
    ///     let object_id = object_data.object_id;
    ///     let version = object_data.version;
    ///     let past_object = mgo
    ///         .read_api()
    ///         .try_get_parsed_past_object(
    ///             object_id,
    ///             version,
    ///             MgoObjectDataOptions {
    ///                 show_type: true,
    ///                 show_owner: true,
    ///                 show_previous_transaction: true,
    ///                 show_display: true,
    ///                 show_content: true,
    ///                 show_bcs: true,
    ///                 show_storage_rebate: true,
    ///             },
    ///         )
    ///         .await?;
    ///     Ok(())
    /// }
    ///```
    pub async fn try_get_parsed_past_object(
        &self,
        object_id: ObjectID,
        version: SequenceNumber,
        options: MgoObjectDataOptions,
    ) -> MgoRpcResult<MgoPastObjectResponse> {
        Ok(self
            .api
            .http
            .try_get_past_object(object_id, version, Some(options))
            .await?)
    }

    /// Return a list of [MgoPastObjectResponse] objects, or an error upon failure.
    ///
    /// See [this function](ReadApi::try_get_parsed_past_object) for more details about past objects.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::{ObjectID, MgoAddress};
    /// use mgo_json_rpc_types::{MgoObjectDataOptions, MgoGetPastObjectRequest};
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let owned_objects = mgo
    ///         .read_api()
    ///         .get_owned_objects(address, None, None, None)
    ///         .await?;
    ///     // this code example assumes that there are previous owned objects
    ///     let object = owned_objects.data.get(0).expect(&format!(
    ///         "No owned objects for this address {}",
    ///         address
    ///     ));
    ///     let object_data = object.data.as_ref().expect(&format!(
    ///         "No object data for this MgoObjectResponse {:?}",
    ///         object
    ///     ));
    ///     let object_id = object_data.object_id;
    ///     let version = object_data.version;
    ///     let past_object = mgo
    ///         .read_api()
    ///         .try_get_parsed_past_object(
    ///             object_id,
    ///             version,
    ///             MgoObjectDataOptions {
    ///                 show_type: true,
    ///                 show_owner: true,
    ///                 show_previous_transaction: true,
    ///                 show_display: true,
    ///                 show_content: true,
    ///                 show_bcs: true,
    ///                 show_storage_rebate: true,
    ///             },
    ///         )
    ///         .await?;
    ///     let past_object = past_object.into_object()?;
    ///     let multi_past_object = mgo
    ///         .read_api()
    ///         .try_multi_get_parsed_past_object(
    ///             vec![MgoGetPastObjectRequest {
    ///                 object_id: past_object.object_id,
    ///                 version: past_object.version,
    ///             }],
    ///             MgoObjectDataOptions {
    ///                 show_type: true,
    ///                 show_owner: true,
    ///                 show_previous_transaction: true,
    ///                 show_display: true,
    ///                 show_content: true,
    ///                 show_bcs: true,
    ///                 show_storage_rebate: true,
    ///             },
    ///         )
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn try_multi_get_parsed_past_object(
        &self,
        past_objects: Vec<MgoGetPastObjectRequest>,
        options: MgoObjectDataOptions,
    ) -> MgoRpcResult<Vec<MgoPastObjectResponse>> {
        Ok(self
            .api
            .http
            .try_multi_get_past_objects(past_objects, Some(options))
            .await?)
    }

    /// Return a [MgoObjectResponse] based on the provided [ObjectID] and [MgoObjectDataOptions], or an error upon failure.
    ///
    /// The [MgoObjectResponse] contains two fields:
    /// 1) `data` for the object's data (see [MgoObjectData](mgo_json_rpc_types::MgoObjectData)),
    /// 2) `error` for the error (if any) (see [MgoObjectResponseError](mgo_types::error::MgoObjectResponseError)).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::MgoAddress;
    /// use mgo_json_rpc_types::MgoObjectDataOptions;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let owned_objects = mgo
    ///         .read_api()
    ///         .get_owned_objects(address, None, None, None)
    ///         .await?;
    ///     // this code example assumes that there are previous owned objects
    ///     let object = owned_objects.data.get(0).expect(&format!(
    ///         "No owned objects for this address {}",
    ///         address
    ///     ));
    ///     let object_data = object.data.as_ref().expect(&format!(
    ///         "No object data for this MgoObjectResponse {:?}",
    ///         object
    ///     ));
    ///     let object_id = object_data.object_id;
    ///     let object = mgo.read_api().get_object_with_options(object_id,
    ///             MgoObjectDataOptions {
    ///                 show_type: true,
    ///                 show_owner: true,
    ///                 show_previous_transaction: true,
    ///                 show_display: true,
    ///                 show_content: true,
    ///                 show_bcs: true,
    ///                 show_storage_rebate: true,
    ///             },
    ///         ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_object_with_options(
        &self,
        object_id: ObjectID,
        options: MgoObjectDataOptions,
    ) -> MgoRpcResult<MgoObjectResponse> {
        Ok(self.api.http.get_object(object_id, Some(options)).await?)
    }

    /// Return a list of [MgoObjectResponse] from the given vector of [ObjectID]s and [MgoObjectDataOptions], or an error upon failure.
    ///
    /// If only one object is needed, use the [get_object_with_options](ReadApi::get_object_with_options) function instead.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::MgoAddress;
    /// use mgo_json_rpc_types::MgoObjectDataOptions;
    /// use std::str::FromStr;
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let owned_objects = mgo
    ///         .read_api()
    ///         .get_owned_objects(address, None, None, None)
    ///         .await?;
    ///     // this code example assumes that there are previous owned objects
    ///     let object = owned_objects.data.get(0).expect(&format!(
    ///         "No owned objects for this address {}",
    ///         address
    ///     ));
    ///     let object_data = object.data.as_ref().expect(&format!(
    ///         "No object data for this MgoObjectResponse {:?}",
    ///         object
    ///     ));
    ///     let object_id = object_data.object_id;
    ///     let object_ids = vec![object_id]; // and other object ids
    ///     let object = mgo.read_api().multi_get_object_with_options(object_ids,
    ///             MgoObjectDataOptions {
    ///                 show_type: true,
    ///                 show_owner: true,
    ///                 show_previous_transaction: true,
    ///                 show_display: true,
    ///                 show_content: true,
    ///                 show_bcs: true,
    ///                 show_storage_rebate: true,
    ///             },
    ///         ).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn multi_get_object_with_options(
        &self,
        object_ids: Vec<ObjectID>,
        options: MgoObjectDataOptions,
    ) -> MgoRpcResult<Vec<MgoObjectResponse>> {
        Ok(self
            .api
            .http
            .multi_get_objects(object_ids, Some(options))
            .await?)
    }

    /// Return An object's bcs content [`Vec<u8>`] based on the provided [ObjectID], or an error upon failure.
    pub async fn get_move_object_bcs(&self, object_id: ObjectID) -> MgoRpcResult<Vec<u8>> {
        let resp = self
            .get_object_with_options(object_id, MgoObjectDataOptions::default().with_bcs())
            .await?
            .into_object()
            .map_err(|e| {
                Error::DataError(format!("Can't get bcs of object {:?}: {:?}", object_id, e))
            })?;
        // unwrap: requested bcs data
        let move_object = resp.bcs.unwrap();
        let raw_move_obj = move_object.try_into_move().ok_or(Error::DataError(format!(
            "Object {:?} is not a MoveObject",
            object_id
        )))?;
        Ok(raw_move_obj.bcs_bytes)
    }

    /// Return the total number of transaction blocks known to server, or an error upon failure.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let total_transaction_blocks = mgo
    ///         .read_api()
    ///         .get_total_transaction_blocks()
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_total_transaction_blocks(&self) -> MgoRpcResult<u64> {
        Ok(*self.api.http.get_total_transaction_blocks().await?)
    }

    /// Return a transaction and its effects in a [MgoTransactionBlockResponse] based on its
    /// [TransactionDigest], or an error upon failure.
    pub async fn get_transaction_with_options(
        &self,
        digest: TransactionDigest,
        options: MgoTransactionBlockResponseOptions,
    ) -> MgoRpcResult<MgoTransactionBlockResponse> {
        Ok(self
            .api
            .http
            .get_transaction_block(digest, Some(options))
            .await?)
    }
    /// Return a list of [MgoTransactionBlockResponse] based on the given vector of [TransactionDigest], or an error upon failure.
    ///
    /// If only one transaction data is needed, use the
    /// [get_transaction_with_options](ReadApi::get_transaction_with_options) function instead.
    pub async fn multi_get_transactions_with_options(
        &self,
        digests: Vec<TransactionDigest>,
        options: MgoTransactionBlockResponseOptions,
    ) -> MgoRpcResult<Vec<MgoTransactionBlockResponse>> {
        Ok(self
            .api
            .http
            .multi_get_transaction_blocks(digests, Some(options))
            .await?)
    }

    /// Return the [MgoCommittee] information for the provided `epoch`, or an error upon failure.
    ///
    /// The [MgoCommittee] contains the validators list and their information (name and stakes).
    ///
    /// The argument `epoch` is either a known epoch id or `None` for the current epoch.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let committee_info = mgo
    ///         .read_api()
    ///         .get_committee_info(None)
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_committee_info(
        &self,
        epoch: Option<BigInt<u64>>,
    ) -> MgoRpcResult<MgoCommittee> {
        Ok(self.api.http.get_committee_info(epoch).await?)
    }

    /// Return a paginated response with all transaction blocks information, or an error upon failure.
    pub async fn query_transaction_blocks(
        &self,
        query: MgoTransactionBlockResponseQuery,
        cursor: Option<TransactionDigest>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> MgoRpcResult<TransactionBlocksPage> {
        Ok(self
            .api
            .http
            .query_transaction_blocks(query, cursor, limit, Some(descending_order))
            .await?)
    }

    /// Return the first four bytes of the chain's genesis checkpoint digest, or an error upon failure.
    pub async fn get_chain_identifier(&self) -> MgoRpcResult<String> {
        Ok(self.api.http.get_chain_identifier().await?)
    }

    /// Return a checkpoint, or an error upon failure.
    ///
    /// A Mgo checkpoint is a sequence of transaction sets that a quorum of validators
    /// agree upon as having been executed within the Mgo system.
    pub async fn get_checkpoint(&self, id: CheckpointId) -> MgoRpcResult<Checkpoint> {
        Ok(self.api.http.get_checkpoint(id).await?)
    }

    /// Return a paginated list of checkpoints, or an error upon failure.
    pub async fn get_checkpoints(
        &self,
        cursor: Option<BigInt<u64>>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> MgoRpcResult<CheckpointPage> {
        Ok(self
            .api
            .http
            .get_checkpoints(cursor, limit, descending_order)
            .await?)
    }

    /// Return the sequence number of the latest checkpoint that has been executed, or an error upon failure.
    pub async fn get_latest_checkpoint_sequence_number(
        &self,
    ) -> MgoRpcResult<CheckpointSequenceNumber> {
        Ok(*self
            .api
            .http
            .get_latest_checkpoint_sequence_number()
            .await?)
    }

    /// Return a stream of [MgoTransactionBlockResponse], or an error upon failure.
    pub fn get_transactions_stream(
        &self,
        query: MgoTransactionBlockResponseQuery,
        cursor: Option<TransactionDigest>,
        descending_order: bool,
    ) -> impl Stream<Item = MgoTransactionBlockResponse> + '_ {
        stream::unfold(
            (vec![], cursor, true, query),
            move |(mut data, cursor, first, query)| async move {
                if let Some(item) = data.pop() {
                    Some((item, (data, cursor, false, query)))
                } else if (cursor.is_none() && first) || cursor.is_some() {
                    let page = self
                        .query_transaction_blocks(
                            query.clone(),
                            cursor,
                            Some(100),
                            descending_order,
                        )
                        .await
                        .ok()?;
                    let mut data = page.data;
                    data.reverse();
                    data.pop()
                        .map(|item| (item, (data, page.next_cursor, false, query)))
                } else {
                    None
                }
            },
        )
    }

    /// Subscribe to a stream of transactions.
    ///
    /// This is only available through WebSockets.
    pub async fn subscribe_transaction(
        &self,
        filter: TransactionFilter,
    ) -> MgoRpcResult<impl Stream<Item = MgoRpcResult<MgoTransactionBlockEffects>>> {
        let Some(c) = &self.api.ws else {
            return Err(Error::Subscription(
                "Subscription only supported by WebSocket client.".to_string(),
            ));
        };
        let subscription: Subscription<MgoTransactionBlockEffects> =
            c.subscribe_transaction(filter).await?;
        Ok(subscription.map(|item| Ok(item?)))
    }

    /// Return a map consisting of the move package name and the normalized module, or an error upon failure.
    pub async fn get_normalized_move_modules_by_package(
        &self,
        package: ObjectID,
    ) -> MgoRpcResult<BTreeMap<String, MgoMoveNormalizedModule>> {
        Ok(self
            .api
            .http
            .get_normalized_move_modules_by_package(package)
            .await?)
    }

    // TODO(devx): we can probably cache this given an epoch
    /// Return the reference gas price, or an error upon failure.
    pub async fn get_reference_gas_price(&self) -> MgoRpcResult<u64> {
        Ok(*self.api.http.get_reference_gas_price().await?)
    }

    /// Dry run a transaction block given the provided transaction data. Returns an error upon failure.
    ///
    /// Simulate running the transaction, including all standard checks, without actually running it.
    /// This is useful for estimating the gas fees of a transaction before executing it.
    /// You can also use it to identify any side-effects of a transaction before you execute it on the network.
    pub async fn dry_run_transaction_block(
        &self,
        tx: TransactionData,
    ) -> MgoRpcResult<DryRunTransactionBlockResponse> {
        Ok(self
            .api
            .http
            .dry_run_transaction_block(Base64::from_bytes(&bcs::to_bytes(&tx)?))
            .await?)
    }

    /// Return the inspection of the transaction block, or an error upon failure.
    ///
    /// Use this function to inspect the current state of the network by running a programmable
    /// transaction block without committing its effects on chain.  Unlike
    /// [dry_run_transaction_block](ReadApi::dry_run_transaction_block),
    /// dev inspect will not validate whether the transaction block
    /// would succeed or fail under normal circumstances, e.g.:
    ///
    /// - Transaction inputs are not checked for ownership (i.e. you can
    ///   construct calls involving objects you do not own).
    /// - Calls are not checked for visibility (you can call private functions on modules)
    /// - Inputs of any type can be constructed and passed in, (including
    ///    Coins and other objects that would usually need to be constructed
    ///    with a move call).
    /// - Function returns do not need to be used, even if they do not have `drop`.
    ///
    /// Dev inspect's output includes a breakdown of results returned by every transaction
    /// in the block, as well as the transaction's effects.
    ///
    /// To run an accurate simulation of a transaction and understand whether
    /// it will successfully validate and run,
    /// use the [dry_run_transaction_block](ReadApi::dry_run_transaction_block) function instead.
    pub async fn dev_inspect_transaction_block(
        &self,
        sender_address: MgoAddress,
        tx: TransactionKind,
        gas_price: Option<BigInt<u64>>,
        epoch: Option<BigInt<u64>>,
        additional_args: Option<DevInspectArgs>,
    ) -> MgoRpcResult<DevInspectResults> {
        Ok(self
            .api
            .http
            .dev_inspect_transaction_block(
                sender_address,
                Base64::from_bytes(&bcs::to_bytes(&tx)?),
                gas_price,
                epoch,
                additional_args,
            )
            .await?)
    }

    /// Return the loaded child objects response for the the provided digest, or an error upon failure.
    ///
    /// Loaded child objects ([MgoLoadedChildObject](mgo_json_rpc_types::MgoLoadedChildObject)) are the non-input objects that the transaction at the digest loaded
    /// in response to dynamic field accesses.
    pub async fn get_loaded_child_objects(
        &self,
        digest: TransactionDigest,
    ) -> MgoRpcResult<MgoLoadedChildObjectsResponse> {
        Ok(self.api.http.get_loaded_child_objects(digest).await?)
    }

    /// Return the protocol config, or an error upon failure.
    pub async fn get_protocol_config(
        &self,
        version: Option<BigInt<u64>>,
    ) -> MgoRpcResult<ProtocolConfigResponse> {
        Ok(self.api.http.get_protocol_config(version).await?)
    }
}

/// Coin Read API provides the functionality needed to get information from the Mgo network regarding the coins owned by an address.
#[derive(Debug, Clone)]
pub struct CoinReadApi {
    api: Arc<RpcClient>,
}

impl CoinReadApi {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }

    /// Return a paginated response with the coins for the given address, or an error upon failure.
    ///
    /// The coins can be filtered by `coin_type` (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC)
    /// or use `None` for the default `Coin<MGO>`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::MgoAddress;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let coins = mgo
    ///         .coin_read_api()
    ///         .get_coins(address, None, None, None)
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_coins(
        &self,
        owner: MgoAddress,
        coin_type: Option<String>,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> MgoRpcResult<CoinPage> {
        Ok(self
            .api
            .http
            .get_coins(owner, coin_type, cursor, limit)
            .await?)
    }
    /// Return a paginated response with all the coins for the given address, or an error upon failure.
    ///
    /// This function includes all coins. If needed to filter by coin type, use the `get_coins` method instead.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::MgoAddress;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let coins = mgo
    ///         .coin_read_api()
    ///         .get_all_coins(address, None, None)
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_all_coins(
        &self,
        owner: MgoAddress,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> MgoRpcResult<CoinPage> {
        Ok(self.api.http.get_all_coins(owner, cursor, limit).await?)
    }

    /// Return the coins for the given address as a stream.
    ///
    /// The coins can be filtered by `coin_type` (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC)
    /// or use `None` for the default `Coin<MGO>`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::MgoAddress;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let coins = mgo
    ///         .coin_read_api()
    ///         .get_coins_stream(address, None);
    ///     Ok(())
    /// }
    /// ```
    pub fn get_coins_stream(
        &self,
        owner: MgoAddress,
        coin_type: Option<String>,
    ) -> impl Stream<Item = Coin> + '_ {
        stream::unfold(
            (
                vec![],
                /* cursor */ None,
                /* has_next_page */ true,
                coin_type,
            ),
            move |(mut data, cursor, has_next_page, coin_type)| async move {
                if let Some(item) = data.pop() {
                    Some((item, (data, cursor, /* has_next_page */ true, coin_type)))
                } else if has_next_page {
                    let page = self
                        .get_coins(owner, coin_type.clone(), cursor, Some(100))
                        .await
                        .ok()?;
                    let mut data = page.data;
                    data.reverse();
                    data.pop().map(|item| {
                        (
                            item,
                            (data, page.next_cursor, page.has_next_page, coin_type),
                        )
                    })
                } else {
                    None
                }
            },
        )
    }

    /// Return a list of coins for the given address, or an error upon failure.
    ///
    /// Note that the function selects coins to meet or exceed the requested `amount`.
    /// If that it is not possible, it will fail with an insufficient fund error.
    ///
    /// The coins can be filtered by `coin_type` (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC)
    /// or use `None` to use the default `Coin<MGO>`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::MgoAddress;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let coins = mgo
    ///         .coin_read_api()
    ///         .select_coins(address, None, 5, vec![])
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn select_coins(
        &self,
        address: MgoAddress,
        coin_type: Option<String>,
        amount: u128,
        exclude: Vec<ObjectID>,
    ) -> MgoRpcResult<Vec<Coin>> {
        let mut total = 0u128;
        let coins = self
            .get_coins_stream(address, coin_type)
            .filter(|coin: &Coin| future::ready(!exclude.contains(&coin.coin_object_id)))
            .take_while(|coin: &Coin| {
                let ready = future::ready(total < amount);
                total += coin.balance as u128;
                ready
            })
            .collect::<Vec<_>>()
            .await;

        if total < amount {
            return Err(Error::InsufficientFund { address, amount });
        }
        Ok(coins)
    }

    /// Return the balance for the given coin type owned by address, or an error upon failure.
    ///
    /// Note that this function sums up all the balances of all the coins matching
    /// the given coin type. By default, if `coin_type` is set to `None`,
    /// it will use the default `Coin<MGO>`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::MgoAddress;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let balance = mgo
    ///         .coin_read_api()
    ///         .get_balance(address, None)
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_balance(
        &self,
        owner: MgoAddress,
        coin_type: Option<String>,
    ) -> MgoRpcResult<Balance> {
        Ok(self.api.http.get_balance(owner, coin_type).await?)
    }

    /// Return a list of balances for each coin type owned by the given address,
    /// or an error upon failure.
    ///
    /// Note that this function groups the coins by coin type, and sums up all their balances.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::MgoAddress;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let address = MgoAddress::from_str("0x0000....0000")?;
    ///     let all_balances = mgo
    ///         .coin_read_api()
    ///         .get_all_balances(address)
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_all_balances(&self, owner: MgoAddress) -> MgoRpcResult<Vec<Balance>> {
        Ok(self.api.http.get_all_balances(owner).await?)
    }

    /// Return the coin metadata (name, symbol, description, decimals, etc.) for a given coin type,
    /// or an error upon failure.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let coin_metadata = mgo
    ///         .coin_read_api()
    ///         .get_coin_metadata("0x2::mgo::MGO".to_string())
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_coin_metadata(
        &self,
        coin_type: String,
    ) -> MgoRpcResult<Option<MgoCoinMetadata>> {
        Ok(self.api.http.get_coin_metadata(coin_type).await?)
    }

    /// Return the total supply for a given coin type, or an error upon failure.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let total_supply = mgo
    ///         .coin_read_api()
    ///         .get_total_supply("0x2::mgo::MGO".to_string())
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_total_supply(&self, coin_type: String) -> MgoRpcResult<Supply> {
        Ok(self.api.http.get_total_supply(coin_type).await?)
    }
}

/// Event API provides the functionality to fetch, query, or subscribe to events on the Mgo network.
#[derive(Clone)]
pub struct EventApi {
    api: Arc<RpcClient>,
}

impl EventApi {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }

    /// Return a stream of events, or an error upon failure.
    ///
    /// Subscription is only possible via WebSockets.
    /// For a list of possible event filters, see [EventFilter].
    ///
    /// # Examples
    ///
    /// ```rust, no_run
    /// use futures::StreamExt;
    /// use std::str::FromStr;
    /// use mgo_json_rpc_types::EventFilter;
    /// use mgo_sdk::MgoClientBuilder;
    /// use mgo_types::base_types::MgoAddress;
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default()
    ///         .ws_url("wss://rpc.mainnet.mangonetwork.io:443")
    ///         .build("https://fullnode.mainnet.mangonetwork.io:443")
    ///         .await?;
    ///     let mut subscribe_all = mgo
    ///         .event_api()
    ///         .subscribe_event(EventFilter::All(vec![]))
    ///         .await?;
    ///     loop {
    ///         println!("{:?}", subscribe_all.next().await);
    ///     }
    ///     Ok(())
    /// }
    /// ```
    pub async fn subscribe_event(
        &self,
        filter: EventFilter,
    ) -> MgoRpcResult<impl Stream<Item = MgoRpcResult<MgoEvent>>> {
        match &self.api.ws {
            Some(c) => {
                let subscription: Subscription<MgoEvent> = c.subscribe_event(filter).await?;
                Ok(subscription.map(|item| Ok(item?)))
            }
            _ => Err(Error::Subscription(
                "Subscription only supported by WebSocket client.".to_string(),
            )),
        }
    }

    /// Return a list of events for the given transaction digest, or an error upon failure.
    pub async fn get_events(&self, digest: TransactionDigest) -> MgoRpcResult<Vec<MgoEvent>> {
        Ok(self.api.http.get_events(digest).await?)
    }

    /// Return a paginated response with events for the given event filter, or an error upon failure.
    ///
    /// The ordering of the events can be set with the `descending_order` argument.
    /// For a list of possible event filters, see [EventFilter].
    pub async fn query_events(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> MgoRpcResult<EventPage> {
        Ok(self
            .api
            .http
            .query_events(query, cursor, limit, Some(descending_order))
            .await?)
    }

    /// Return a stream of events for the given event filter.
    ///
    /// The ordering of the events can be set with the `descending_order` argument.
    /// For a list of possible event filters, see [EventFilter].
    pub fn get_events_stream(
        &self,
        query: EventFilter,
        cursor: Option<EventID>,
        descending_order: bool,
    ) -> impl Stream<Item = MgoEvent> + '_ {
        stream::unfold(
            (vec![], cursor, true, query),
            move |(mut data, cursor, first, query)| async move {
                if let Some(item) = data.pop() {
                    Some((item, (data, cursor, false, query)))
                } else if (cursor.is_none() && first) || cursor.is_some() {
                    let page = self
                        .query_events(query.clone(), cursor, Some(100), descending_order)
                        .await
                        .ok()?;
                    let mut data = page.data;
                    data.reverse();
                    data.pop()
                        .map(|item| (item, (data, page.next_cursor, false, query)))
                } else {
                    None
                }
            },
        )
    }
}

/// Quorum API that provides functionality to execute a transaction block and submit it to the fullnode(s).
#[derive(Clone)]
pub struct QuorumDriverApi {
    api: Arc<RpcClient>,
}

impl QuorumDriverApi {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }

    /// Execute a transaction with a FullNode client. `request_type`
    /// defaults to `ExecuteTransactionRequestType::WaitForLocalExecution`.
    /// When `ExecuteTransactionRequestType::WaitForLocalExecution` is used,
    /// but returned `confirmed_local_execution` is false, the client will
    /// keep retry for WAIT_FOR_LOCAL_EXECUTION_RETRY_COUNT times. If it
    /// still fails, it will return an error.
    pub async fn execute_transaction_block(
        &self,
        tx: Transaction,
        options: MgoTransactionBlockResponseOptions,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> MgoRpcResult<MgoTransactionBlockResponse> {
        let (tx_bytes, signatures) = tx.to_tx_bytes_and_signatures();
        let request_type = request_type.unwrap_or_else(|| options.default_execution_request_type());
        let mut retry_count = 0;
        let start = Instant::now();
        while retry_count < WAIT_FOR_LOCAL_EXECUTION_RETRY_COUNT {
            let response: MgoTransactionBlockResponse = self
                .api
                .http
                .execute_transaction_block(
                    tx_bytes.clone(),
                    signatures.clone(),
                    Some(options.clone()),
                    Some(request_type.clone()),
                )
                .await?;

            match request_type {
                ExecuteTransactionRequestType::WaitForEffectsCert => {
                    return Ok(response);
                }
                ExecuteTransactionRequestType::WaitForLocalExecution => {
                    if let Some(true) = response.confirmed_local_execution {
                        return Ok(response);
                    } else {
                        // If fullnode executed the cert in the network but did not confirm local
                        // execution, it must have timed out and hence we could retry.
                        retry_count += 1;
                    }
                }
            }
        }
        Err(Error::FailToConfirmTransactionStatus(
            *tx.digest(),
            start.elapsed().as_secs(),
        ))
    }
}

/// Governance API provides the staking functionality.
#[derive(Debug, Clone)]
pub struct GovernanceApi {
    api: Arc<RpcClient>,
}

impl GovernanceApi {
    pub(crate) fn new(api: Arc<RpcClient>) -> Self {
        Self { api }
    }

    /// Return a list of [DelegatedStake] objects for the given address, or an error upon failure.
    pub async fn get_stakes(&self, owner: MgoAddress) -> MgoRpcResult<Vec<DelegatedStake>> {
        Ok(self.api.http.get_stakes(owner).await?)
    }

    /// Return the [MgoCommittee] information for the given `epoch`, or an error upon failure.
    ///
    /// The argument `epoch` is the known epoch id or `None` for the current epoch.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use mgo_sdk::MgoClientBuilder;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), anyhow::Error> {
    ///     let mgo = MgoClientBuilder::default().build_localnet().await?;
    ///     let committee_info = mgo
    ///         .governance_api()
    ///         .get_committee_info(None)
    ///         .await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_committee_info(
        &self,
        epoch: Option<BigInt<u64>>,
    ) -> MgoRpcResult<MgoCommittee> {
        Ok(self.api.http.get_committee_info(epoch).await?)
    }

    /// Return the latest MGO system state object on-chain, or an error upon failure.
    ///
    /// Use this method to access system's information, such as the current epoch,
    /// the protocol version, the reference gas price, the total stake, active validators,
    /// and much more. See the [MgoSystemStateSummary] for all the available fields.
    pub async fn get_latest_mgo_system_state(&self) -> MgoRpcResult<MgoSystemStateSummary> {
        Ok(self.api.http.get_latest_mgo_system_state().await?)
    }

    /// Return the reference gas price for the network, or an error upon failure.
    pub async fn get_reference_gas_price(&self) -> MgoRpcResult<u64> {
        Ok(*self.api.http.get_reference_gas_price().await?)
    }
}
