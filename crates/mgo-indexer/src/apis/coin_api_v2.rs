// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use crate::indexer_reader::IndexerReader;
use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;
use mgo_json_rpc::coin_api::{parse_to_struct_tag, parse_to_type_tag};
use mgo_json_rpc::MgoRpcModule;
use mgo_json_rpc_api::{cap_page_limit, CoinReadApiServer};
use mgo_json_rpc_types::{Balance, CoinPage, Page, MgoCoinMetadata};
use mgo_open_rpc::Module;
use mgo_types::balance::Supply;
use mgo_types::base_types::{ObjectID, MgoAddress};
use mgo_types::gas_coin::{GAS, TOTAL_SUPPLY_MANGO};

pub(crate) struct CoinReadApiV2 {
    inner: IndexerReader,
}

impl CoinReadApiV2 {
    pub fn new(inner: IndexerReader) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl CoinReadApiServer for CoinReadApiV2 {
    async fn get_coins(
        &self,
        owner: MgoAddress,
        coin_type: Option<String>,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<CoinPage> {
        let limit = cap_page_limit(limit);
        if limit == 0 {
            return Ok(CoinPage::empty());
        }

        // Normalize coin type tag and default to Gas
        let coin_type =
            parse_to_type_tag(coin_type)?.to_canonical_string(/* with_prefix */ true);

        let cursor = match cursor {
            Some(c) => c,
            // If cursor is not specified, we need to start from the beginning of the coin type, which is the minimal possible ObjectID.
            None => ObjectID::ZERO,
        };
        let mut results = self
            .inner
            .get_owned_coins_in_blocking_task(owner, Some(coin_type), cursor, limit + 1)
            .await?;

        let has_next_page = results.len() > limit;
        results.truncate(limit);
        let next_cursor = results.last().map(|o| o.coin_object_id);
        Ok(Page {
            data: results,
            next_cursor,
            has_next_page,
        })
    }

    async fn get_all_coins(
        &self,
        owner: MgoAddress,
        cursor: Option<ObjectID>,
        limit: Option<usize>,
    ) -> RpcResult<CoinPage> {
        let limit = cap_page_limit(limit);
        if limit == 0 {
            return Ok(CoinPage::empty());
        }

        let cursor = match cursor {
            Some(c) => c,
            // If cursor is not specified, we need to start from the beginning of the coin type, which is the minimal possible ObjectID.
            None => ObjectID::ZERO,
        };
        let mut results = self
            .inner
            .get_owned_coins_in_blocking_task(owner, None, cursor, limit + 1)
            .await?;

        let has_next_page = results.len() > limit;
        results.truncate(limit);
        let next_cursor = results.last().map(|o| o.coin_object_id);
        Ok(Page {
            data: results,
            next_cursor,
            has_next_page,
        })
    }

    async fn get_balance(
        &self,
        owner: MgoAddress,
        coin_type: Option<String>,
    ) -> RpcResult<Balance> {
        // Normalize coin type tag and default to Gas
        let coin_type =
            parse_to_type_tag(coin_type)?.to_canonical_string(/* with_prefix */ true);

        let mut results = self
            .inner
            .get_coin_balances_in_blocking_task(owner, Some(coin_type.clone()))
            .await?;
        if results.is_empty() {
            return Ok(Balance::zero(coin_type));
        }
        Ok(results.swap_remove(0))
    }

    async fn get_all_balances(&self, owner: MgoAddress) -> RpcResult<Vec<Balance>> {
        self.inner
            .get_coin_balances_in_blocking_task(owner, None)
            .await
            .map_err(Into::into)
    }

    async fn get_coin_metadata(&self, coin_type: String) -> RpcResult<Option<MgoCoinMetadata>> {
        let coin_struct = parse_to_struct_tag(&coin_type)?;
        self.inner
            .get_coin_metadata_in_blocking_task(coin_struct)
            .await
            .map_err(Into::into)
    }

    async fn get_total_supply(&self, coin_type: String) -> RpcResult<Supply> {
        let coin_struct = parse_to_struct_tag(&coin_type)?;
        if GAS::is_gas(&coin_struct) {
            Ok(Supply {
                value: TOTAL_SUPPLY_MANGO,
            })
        } else {
            self.inner
                .get_total_supply_in_blocking_task(coin_struct)
                .await
                .map_err(Into::into)
        }
    }
}

impl MgoRpcModule for CoinReadApiV2 {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        mgo_json_rpc_api::CoinReadApiOpenRpc::module_doc()
    }
}
