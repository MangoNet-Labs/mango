// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use mgo_json_rpc::MgoRpcModule;
use mgo_json_rpc_api::{GovernanceReadApiClient, GovernanceReadApiServer};
use mgo_json_rpc_types::MgoCommittee;
use mgo_json_rpc_types::{DelegatedStake, ValidatorApys};
use mgo_open_rpc::Module;
use mgo_types::base_types::{ObjectID, MgoAddress};
use mgo_types::mgo_serde::BigInt;
use mgo_types::mgo_system_state::mgo_system_state_summary::MgoSystemStateSummary;

pub(crate) struct GovernanceReadApi {
    fullnode: HttpClient,
}

impl GovernanceReadApi {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self {
            fullnode: fullnode_client,
        }
    }
}

#[async_trait]
impl GovernanceReadApiServer for GovernanceReadApi {
    async fn get_stakes_by_ids(
        &self,
        staked_mgo_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>> {
        self.fullnode.get_stakes_by_ids(staked_mgo_ids).await
    }
    async fn get_stakes(&self, owner: MgoAddress) -> RpcResult<Vec<DelegatedStake>> {
        self.fullnode.get_stakes(owner).await
    }

    async fn get_committee_info(&self, epoch: Option<BigInt<u64>>) -> RpcResult<MgoCommittee> {
        self.fullnode.get_committee_info(epoch).await
    }

    async fn get_latest_mgo_system_state(&self) -> RpcResult<MgoSystemStateSummary> {
        self.fullnode.get_latest_mgo_system_state().await
    }

    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>> {
        self.fullnode.get_reference_gas_price().await
    }

    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys> {
        self.fullnode.get_validators_apy().await
    }
}

impl MgoRpcModule for GovernanceReadApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        mgo_json_rpc_api::GovernanceReadApiOpenRpc::module_doc()
    }
}
