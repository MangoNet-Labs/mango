// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use mgo_json_rpc::error::MgoRpcInputError;
use mgo_types::error::MgoObjectResponseError;
use mgo_types::object::ObjectRead;

use crate::errors::IndexerError;
use crate::indexer_reader::IndexerReader;
use mgo_json_rpc::MgoRpcModule;
use mgo_json_rpc_api::{ReadApiServer, QUERY_MAX_RESULT_LIMIT};
use mgo_json_rpc_types::{
    Checkpoint, CheckpointId, CheckpointPage, ProtocolConfigResponse, MgoEvent,
    MgoGetPastObjectRequest, MgoObjectDataOptions, MgoObjectResponse, MgoPastObjectResponse,
    MgoTransactionBlockResponse, MgoTransactionBlockResponseOptions,
};
use mgo_open_rpc::Module;
use mgo_protocol_config::{ProtocolConfig, ProtocolVersion};
use mgo_types::base_types::{ObjectID, SequenceNumber};
use mgo_types::digests::{ChainIdentifier, TransactionDigest};
use mgo_types::mgo_serde::BigInt;

use mgo_json_rpc_types::MgoLoadedChildObjectsResponse;

#[derive(Clone)]
pub(crate) struct ReadApiV2 {
    inner: IndexerReader,
}

impl ReadApiV2 {
    pub fn new(inner: IndexerReader) -> Self {
        Self { inner }
    }

    async fn get_checkpoint(&self, id: CheckpointId) -> Result<Checkpoint, IndexerError> {
        match self
            .inner
            .spawn_blocking(move |this| this.get_checkpoint(id))
            .await
        {
            Ok(Some(epoch_info)) => Ok(epoch_info),
            Ok(None) => Err(IndexerError::InvalidArgumentError(format!(
                "Checkpoint {id:?} not found"
            ))),
            Err(e) => Err(e),
        }
    }

    async fn get_latest_checkpoint(&self) -> Result<Checkpoint, IndexerError> {
        self.inner
            .spawn_blocking(|this| this.get_latest_checkpoint())
            .await
    }

    async fn get_chain_identifier(&self) -> RpcResult<ChainIdentifier> {
        let genesis_checkpoint = self.get_checkpoint(CheckpointId::SequenceNumber(0)).await?;
        Ok(ChainIdentifier::from(genesis_checkpoint.digest))
    }
}

#[async_trait]
impl ReadApiServer for ReadApiV2 {
    async fn get_object(
        &self,
        object_id: ObjectID,
        options: Option<MgoObjectDataOptions>,
    ) -> RpcResult<MgoObjectResponse> {
        let options = options.unwrap_or_default();
        let object_read = self
            .inner
            .get_object_read_in_blocking_task(object_id)
            .await?;

        match object_read {
            ObjectRead::NotExists(id) => Ok(MgoObjectResponse::new_with_error(
                MgoObjectResponseError::NotExists { object_id: id },
            )),
            ObjectRead::Exists(object_ref, o, layout) => {
                let mut display_fields = None;
                if options.show_display {
                    match self.inner.get_display_fields(&o, &layout).await {
                        Ok(rendered_fields) => display_fields = Some(rendered_fields),
                        Err(e) => {
                            return Ok(MgoObjectResponse::new(
                                Some((object_ref, o, layout, options, None).try_into()?),
                                Some(MgoObjectResponseError::DisplayError {
                                    error: e.to_string(),
                                }),
                            ));
                        }
                    }
                }
                Ok(MgoObjectResponse::new_with_data(
                    (object_ref, o, layout, options, display_fields).try_into()?,
                ))
            }
            ObjectRead::Deleted((object_id, version, digest)) => Ok(
                MgoObjectResponse::new_with_error(MgoObjectResponseError::Deleted {
                    object_id,
                    version,
                    digest,
                }),
            ),
        }
    }

    // For ease of implementation we just forward to the single object query, although in the
    // future we may want to improve the performance by having a more naitive multi_get
    // functionality
    async fn multi_get_objects(
        &self,
        object_ids: Vec<ObjectID>,
        options: Option<MgoObjectDataOptions>,
    ) -> RpcResult<Vec<MgoObjectResponse>> {
        if object_ids.len() > *QUERY_MAX_RESULT_LIMIT {
            return Err(
                MgoRpcInputError::SizeLimitExceeded(QUERY_MAX_RESULT_LIMIT.to_string()).into(),
            );
        }

        let mut futures = vec![];
        for object_id in object_ids {
            futures.push(self.get_object(object_id, options.clone()));
        }

        futures::future::join_all(futures)
            .await
            .into_iter()
            .collect::<Result<Vec<_>, _>>()
    }

    async fn get_total_transaction_blocks(&self) -> RpcResult<BigInt<u64>> {
        let checkpoint = self.get_latest_checkpoint().await?;
        Ok(BigInt::from(checkpoint.network_total_transactions))
    }

    async fn get_transaction_block(
        &self,
        digest: TransactionDigest,
        options: Option<MgoTransactionBlockResponseOptions>,
    ) -> RpcResult<MgoTransactionBlockResponse> {
        let mut txn = self
            .multi_get_transaction_blocks(vec![digest], options)
            .await?;

        let txn = txn.pop().ok_or_else(|| {
            IndexerError::InvalidArgumentError(format!("Transaction {digest} not found"))
        })?;

        Ok(txn)
    }

    async fn multi_get_transaction_blocks(
        &self,
        digests: Vec<TransactionDigest>,
        options: Option<MgoTransactionBlockResponseOptions>,
    ) -> RpcResult<Vec<MgoTransactionBlockResponse>> {
        let num_digests = digests.len();
        if num_digests > *QUERY_MAX_RESULT_LIMIT {
            Err(MgoRpcInputError::SizeLimitExceeded(
                QUERY_MAX_RESULT_LIMIT.to_string(),
            ))?
        }

        let options = options.unwrap_or_default();
        let txns = self
            .inner
            .multi_get_transaction_block_response_in_blocking_task(digests, options)
            .await?;

        Ok(txns)
    }

    async fn try_get_past_object(
        &self,
        _object_id: ObjectID,
        _version: SequenceNumber,
        _options: Option<MgoObjectDataOptions>,
    ) -> RpcResult<MgoPastObjectResponse> {
        Err(jsonrpsee::types::error::CallError::Custom(
            jsonrpsee::types::error::ErrorCode::MethodNotFound.into(),
        )
        .into())
    }

    async fn try_multi_get_past_objects(
        &self,
        _past_objects: Vec<MgoGetPastObjectRequest>,
        _options: Option<MgoObjectDataOptions>,
    ) -> RpcResult<Vec<MgoPastObjectResponse>> {
        Err(jsonrpsee::types::error::CallError::Custom(
            jsonrpsee::types::error::ErrorCode::MethodNotFound.into(),
        )
        .into())
    }

    async fn get_latest_checkpoint_sequence_number(&self) -> RpcResult<BigInt<u64>> {
        let checkpoint = self.get_latest_checkpoint().await?;
        Ok(BigInt::from(checkpoint.sequence_number))
    }

    async fn get_checkpoint(&self, id: CheckpointId) -> RpcResult<Checkpoint> {
        self.get_checkpoint(id).await.map_err(Into::into)
    }

    async fn get_checkpoints(
        &self,
        cursor: Option<BigInt<u64>>,
        limit: Option<usize>,
        descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        let cursor = cursor.map(BigInt::into_inner);
        let limit = mgo_json_rpc_api::validate_limit(
            limit,
            mgo_json_rpc_api::QUERY_MAX_RESULT_LIMIT_CHECKPOINTS,
        )
        .map_err(MgoRpcInputError::from)?;

        let mut checkpoints = self
            .inner
            .spawn_blocking(move |this| this.get_checkpoints(cursor, limit + 1, descending_order))
            .await?;

        let has_next_page = checkpoints.len() > limit;
        checkpoints.truncate(limit);

        let next_cursor = checkpoints.last().map(|d| d.sequence_number.into());

        Ok(CheckpointPage {
            data: checkpoints,
            next_cursor,
            has_next_page,
        })
    }

    async fn get_checkpoints_deprecated_limit(
        &self,
        cursor: Option<BigInt<u64>>,
        limit: Option<BigInt<u64>>,
        descending_order: bool,
    ) -> RpcResult<CheckpointPage> {
        self.get_checkpoints(
            cursor,
            limit.map(|l| l.into_inner() as usize),
            descending_order,
        )
        .await
    }

    async fn get_events(&self, transaction_digest: TransactionDigest) -> RpcResult<Vec<MgoEvent>> {
        self.inner
            .get_transaction_events_in_blocking_task(transaction_digest)
            .await
            .map_err(Into::into)
    }

    async fn get_loaded_child_objects(
        &self,
        _digest: TransactionDigest,
    ) -> RpcResult<MgoLoadedChildObjectsResponse> {
        Err(jsonrpsee::types::error::CallError::Custom(
            jsonrpsee::types::error::ErrorCode::MethodNotFound.into(),
        )
        .into())
    }

    async fn get_protocol_config(
        &self,
        version: Option<BigInt<u64>>,
    ) -> RpcResult<ProtocolConfigResponse> {
        let chain = self.get_chain_identifier().await?.chain();
        let version = if let Some(version) = version {
            (*version).into()
        } else {
            let latest_epoch = self
                .inner
                .spawn_blocking(|this| this.get_latest_epoch_info_from_db())
                .await?;
            (latest_epoch.protocol_version as u64).into()
        };

        ProtocolConfig::get_for_version_if_supported(version, chain)
            .ok_or(MgoRpcInputError::ProtocolVersionUnsupported(
                ProtocolVersion::MIN.as_u64(),
                ProtocolVersion::MAX.as_u64(),
            ))
            .map_err(Into::into)
            .map(ProtocolConfigResponse::from)
    }

    async fn get_chain_identifier(&self) -> RpcResult<String> {
        self.get_chain_identifier().await.map(|id| id.to_string())
    }
}

impl MgoRpcModule for ReadApiV2 {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        mgo_json_rpc_api::ReadApiOpenRpc::module_doc()
    }
}
