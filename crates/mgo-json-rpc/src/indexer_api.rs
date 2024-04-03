// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use anyhow::bail;
use async_trait::async_trait;
use futures::Stream;
use jsonrpsee::{
    core::{error::SubscriptionClosed, RpcResult},
    types::SubscriptionResult,
    RpcModule, SubscriptionSink,
};
use move_bytecode_utils::layout::TypeLayoutBuilder;
use move_core_types::language_storage::TypeTag;
use mango_metrics::spawn_monitored_task;
use serde::Serialize;
use std::str::FromStr;
use std::sync::Arc;
use mgo_core::authority::AuthorityState;
use mgo_json::MgoJsonValue;
use mgo_json_rpc_api::{
    cap_page_limit, validate_limit, IndexerApiOpenRpc, IndexerApiServer, JsonRpcMetrics,
    ReadApiServer, QUERY_MAX_RESULT_LIMIT,
};
use mgo_json_rpc_types::{
    DynamicFieldPage, EventFilter, EventPage, ObjectsPage, Page, MgoObjectDataOptions,
    MgoObjectResponse, MgoObjectResponseQuery, MgoTransactionBlockResponse,
    MgoTransactionBlockResponseQuery, TransactionBlocksPage, TransactionFilter,
};
use mgo_open_rpc::Module;
use mgo_storage::key_value_store::TransactionKeyValueStore;
use mgo_types::{
    base_types::{ObjectID, MgoAddress},
    digests::TransactionDigest,
    dynamic_field::{DynamicFieldName, Field},
    error::MgoObjectResponseError,
    event::EventID,
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tracing::{debug, instrument, warn};

use crate::{
    authority_state::StateRead,
    error::{Error, MgoRpcInputError},
    name_service::{Domain, NameRecord, NameServiceConfig},
    with_tracing, MgoRpcModule,
};

pub fn spawn_subscription<S, T>(
    mut sink: SubscriptionSink,
    rx: S,
    permit: Option<OwnedSemaphorePermit>,
) where
    S: Stream<Item = T> + Unpin + Send + 'static,
    T: Serialize,
{
    spawn_monitored_task!(async move {
        let _permit = permit;
        match sink.pipe_from_stream(rx).await {
            SubscriptionClosed::Success => {
                debug!("Subscription completed.");
                sink.close(SubscriptionClosed::Success);
            }
            SubscriptionClosed::RemotePeerAborted => {
                debug!("Subscription aborted by remote peer.");
                sink.close(SubscriptionClosed::RemotePeerAborted);
            }
            SubscriptionClosed::Failed(err) => {
                debug!("Subscription failed: {err:?}");
                sink.close(err);
            }
        };
    });
}
const DEFAULT_MAX_SUBSCRIPTIONS: usize = 100;

pub struct IndexerApi<R> {
    state: Arc<dyn StateRead>,
    read_api: R,
    transaction_kv_store: Arc<TransactionKeyValueStore>,
    name_service_config: NameServiceConfig,
    pub metrics: Arc<JsonRpcMetrics>,
    subscription_semaphore: Arc<Semaphore>,
}

impl<R: ReadApiServer> IndexerApi<R> {
    pub fn new(
        state: Arc<AuthorityState>,
        read_api: R,
        transaction_kv_store: Arc<TransactionKeyValueStore>,
        name_service_config: NameServiceConfig,
        metrics: Arc<JsonRpcMetrics>,
        max_subscriptions: Option<usize>,
    ) -> Self {
        let max_subscriptions = max_subscriptions.unwrap_or(DEFAULT_MAX_SUBSCRIPTIONS);
        Self {
            state,
            transaction_kv_store,
            read_api,
            name_service_config,
            metrics,
            subscription_semaphore: Arc::new(Semaphore::new(max_subscriptions)),
        }
    }

    fn extract_values_from_dynamic_field_name(
        &self,
        name: DynamicFieldName,
    ) -> Result<(TypeTag, Vec<u8>), MgoRpcInputError> {
        let DynamicFieldName {
            type_: name_type,
            value,
        } = name;
        let epoch_store = self.state.load_epoch_store_one_call_per_task();
        let layout = TypeLayoutBuilder::build_with_types(&name_type, epoch_store.module_cache())?;
        let mgo_json_value = MgoJsonValue::new(value)?;
        let name_bcs_value = mgo_json_value.to_bcs_bytes(&layout)?;
        Ok((name_type, name_bcs_value))
    }

    fn acquire_subscribe_permit(&self) -> anyhow::Result<OwnedSemaphorePermit> {
        match self.subscription_semaphore.clone().try_acquire_owned() {
            Ok(p) => Ok(p),
            Err(_) => bail!("Resources exhausted"),
        }
    }
}

#[async_trait]
impl<R: ReadApiServer> IndexerApiServer for IndexerApi<R> {
    #[instrument(skip(self))]
    async fn get_owned_objects(
        &self,
        address: MgoAddress,
        query: Option<MgoObjectResponseQuery>,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<ObjectsPage> {
        with_tracing!(async move {
            let limit =
                validate_limit(limit, *QUERY_MAX_RESULT_LIMIT).map_err(MgoRpcInputError::from)?;
            self.metrics.get_owned_objects_limit.report(limit as u64);
            let MgoObjectResponseQuery { filter, options } = query.unwrap_or_default();
            let options = options.unwrap_or_default();
            let mut objects = self
                .state
                .get_owner_objects_with_limit(address, cursor, limit + 1, filter)
                .map_err(Error::from)?;

            // objects here are of size (limit + 1), where the last one is the cursor for the next page
            let has_next_page = objects.len() > limit;
            objects.truncate(limit);
            let next_cursor = objects
                .last()
                .cloned()
                .map_or(cursor, |o_info| Some(o_info.object_id));

            let data = match options.is_not_in_object_info() {
                true => {
                    let object_ids = objects.iter().map(|obj| obj.object_id).collect();
                    self.read_api
                        .multi_get_objects(object_ids, Some(options))
                        .await?
                }
                false => objects
                    .into_iter()
                    .map(|o_info| MgoObjectResponse::try_from((o_info, options.clone())))
                    .collect::<Result<Vec<MgoObjectResponse>, _>>()?,
            };

            self.metrics
                .get_owned_objects_result_size
                .report(data.len() as u64);
            self.metrics
                .get_owned_objects_result_size_total
                .inc_by(data.len() as u64);
            Ok(Page {
                data,
                next_cursor,
                has_next_page,
            })
        })
    }

    #[instrument(skip(self))]
    async fn query_transaction_blocks(
        &self,
        query: MgoTransactionBlockResponseQuery,
        // If `Some`, the query will start from the next item after the specified cursor
        cursor: Option<TransactionDigest>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> RpcResult<TransactionBlocksPage> {
        with_tracing!(async move {
            let limit = cap_page_limit(limit);
            self.metrics.query_tx_blocks_limit.report(limit as u64);
            let descending = descending_order.unwrap_or_default();
            let opts = query.options.unwrap_or_default();

            // Retrieve 1 extra item for next cursor
            let mut digests = self
                .state
                .get_transactions(
                    &self.transaction_kv_store,
                    query.filter,
                    cursor,
                    Some(limit + 1),
                    descending,
                )
                .await
                .map_err(Error::from)?;

            // extract next cursor
            let has_next_page = digests.len() > limit;
            digests.truncate(limit);
            let next_cursor = digests.last().cloned().map_or(cursor, Some);

            let data: Vec<MgoTransactionBlockResponse> = if opts.only_digest() {
                digests
                    .into_iter()
                    .map(MgoTransactionBlockResponse::new)
                    .collect()
            } else {
                self.read_api
                    .multi_get_transaction_blocks(digests, Some(opts))
                    .await?
            };

            self.metrics
                .query_tx_blocks_result_size
                .report(data.len() as u64);
            self.metrics
                .query_tx_blocks_result_size_total
                .inc_by(data.len() as u64);
            Ok(Page {
                data,
                next_cursor,
                has_next_page,
            })
        })
    }
    #[instrument(skip(self))]
    async fn query_events(
        &self,
        query: EventFilter,
        // exclusive cursor if `Some`, otherwise start from the beginning
        cursor: Option<EventID>,
        limit: Option<usize>,
        descending_order: Option<bool>,
    ) -> RpcResult<EventPage> {
        with_tracing!(async move {
            let descending = descending_order.unwrap_or_default();
            let limit = cap_page_limit(limit);
            self.metrics.query_events_limit.report(limit as u64);
            // Retrieve 1 extra item for next cursor
            let mut data = self
                .state
                .query_events(
                    &self.transaction_kv_store,
                    query,
                    cursor,
                    limit + 1,
                    descending,
                )
                .await
                .map_err(Error::from)?;
            let has_next_page = data.len() > limit;
            data.truncate(limit);
            let next_cursor = data.last().map_or(cursor, |e| Some(e.id));
            self.metrics
                .query_events_result_size
                .report(data.len() as u64);
            self.metrics
                .query_events_result_size_total
                .inc_by(data.len() as u64);
            Ok(EventPage {
                data,
                next_cursor,
                has_next_page,
            })
        })
    }

    #[instrument(skip(self))]
    fn subscribe_event(&self, sink: SubscriptionSink, filter: EventFilter) -> SubscriptionResult {
        let permit = self.acquire_subscribe_permit()?;
        spawn_subscription(
            sink,
            self.state
                .get_subscription_handler()
                .subscribe_events(filter),
            Some(permit),
        );
        Ok(())
    }

    fn subscribe_transaction(
        &self,
        sink: SubscriptionSink,
        filter: TransactionFilter,
    ) -> SubscriptionResult {
        let permit = self.acquire_subscribe_permit()?;
        spawn_subscription(
            sink,
            self.state
                .get_subscription_handler()
                .subscribe_transactions(filter),
            Some(permit),
        );
        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_dynamic_fields(
        &self,
        parent_object_id: ObjectID,
        // If `Some`, the query will start from the next item after the specified cursor
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<DynamicFieldPage> {
        with_tracing!(async move {
            let limit = cap_page_limit(limit);
            self.metrics.get_dynamic_fields_limit.report(limit as u64);
            let mut data = self
                .state
                .get_dynamic_fields(parent_object_id, cursor, limit + 1)
                .map_err(Error::from)?;
            let has_next_page = data.len() > limit;
            data.truncate(limit);
            let next_cursor = data.last().cloned().map_or(cursor, |c| Some(c.0));
            self.metrics
                .get_dynamic_fields_result_size
                .report(data.len() as u64);
            self.metrics
                .get_dynamic_fields_result_size_total
                .inc_by(data.len() as u64);
            Ok(DynamicFieldPage {
                data: data.into_iter().map(|(_, w)| w).collect(),
                next_cursor,
                has_next_page,
            })
        })
    }

    #[instrument(skip(self))]
    async fn get_dynamic_field_object(
        &self,
        parent_object_id: ObjectID,
        name: DynamicFieldName,
    ) -> RpcResult<MgoObjectResponse> {
        with_tracing!(async move {
            let (name_type, name_bcs_value) = self.extract_values_from_dynamic_field_name(name)?;

            let id = self
                .state
                .get_dynamic_field_object_id(parent_object_id, name_type, &name_bcs_value)
                .map_err(Error::from)?;
            // TODO(chris): add options to `get_dynamic_field_object` API as well
            if let Some(id) = id {
                self.read_api
                    .get_object(id, Some(MgoObjectDataOptions::full_content()))
                    .await
                    .map_err(Error::from)
            } else {
                Ok(MgoObjectResponse::new_with_error(
                    MgoObjectResponseError::DynamicFieldNotFound { parent_object_id },
                ))
            }
        })
    }

    #[instrument(skip(self))]
    async fn resolve_name_service_address(&self, name: String) -> RpcResult<Option<MgoAddress>> {
        with_tracing!(async move {
            let domain = Domain::from_str(&name).map_err(|e| {
                Error::UnexpectedError(format!(
                    "Failed to parse NameService Domain with error: {:?}",
                    e
                ))
            })?;

            let record_id = self.name_service_config.record_field_id(&domain);

            let field_record_object = match self.state.get_object(&record_id).await? {
                Some(o) => o,
                None => return Ok(None),
            };

            let record = field_record_object
                .to_rust::<Field<Domain, NameRecord>>()
                .ok_or_else(|| Error::UnexpectedError(format!("Malformed Object {record_id}")))?
                .value;

            Ok(record.target_address)
        })
    }

    #[instrument(skip(self))]
    async fn resolve_name_service_names(
        &self,
        address: MgoAddress,
        _cursor: Option<ObjectID>,
        _limit: Option<usize>,
    ) -> RpcResult<Page<String, ObjectID>> {
        with_tracing!(async move {
            let reverse_record_id = self
                .name_service_config
                .reverse_record_field_id(address.as_ref());

            let field_reverse_record_object =
                match self.state.get_object(&reverse_record_id).await? {
                    Some(o) => o,
                    None => {
                        return Ok(Page {
                            data: vec![],
                            next_cursor: None,
                            has_next_page: false,
                        })
                    }
                };

            let domain = field_reverse_record_object
                .to_rust::<Field<MgoAddress, Domain>>()
                .ok_or_else(|| {
                    Error::UnexpectedError(format!("Malformed Object {reverse_record_id}"))
                })?
                .value;

            Ok(Page {
                data: vec![domain.to_string()],
                next_cursor: None,
                has_next_page: false,
            })
        })
    }
}

impl<R: ReadApiServer> MgoRpcModule for IndexerApi<R> {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        IndexerApiOpenRpc::module_doc()
    }
}
