// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use jsonrpsee::core::RpcResult;
use jsonrpsee::proc_macros::rpc;

use mgo_json_rpc_types::{DelegatedStake, MgoCommittee, ValidatorApys};
use mgo_open_rpc_macros::open_rpc;
use mgo_types::base_types::{ObjectID, MgoAddress};
use mgo_types::mgo_serde::BigInt;
use mgo_types::mgo_system_state::mgo_system_state_summary::MgoSystemStateSummary;

#[open_rpc(namespace = "mgox", tag = "Governance Read API")]
#[rpc(server, client, namespace = "mgox")]
pub trait GovernanceReadApi {
    /// Return one or more [DelegatedStake]. If a Stake was withdrawn its status will be Unstaked.
    #[method(name = "getStakesByIds")]
    async fn get_stakes_by_ids(
        &self,
        staked_mgo_ids: Vec<ObjectID>,
    ) -> RpcResult<Vec<DelegatedStake>>;

    /// Return all [DelegatedStake].
    #[method(name = "getStakes")]
    async fn get_stakes(&self, owner: MgoAddress) -> RpcResult<Vec<DelegatedStake>>;

    /// Return the committee information for the asked `epoch`.
    #[method(name = "getCommitteeInfo")]
    async fn get_committee_info(
        &self,
        /// The epoch of interest. If None, default to the latest epoch
        epoch: Option<BigInt<u64>>,
    ) -> RpcResult<MgoCommittee>;

    /// Return the latest MGO system state object on-chain.
    #[method(name = "getLatestMgoSystemState")]
    async fn get_latest_mgo_system_state(&self) -> RpcResult<MgoSystemStateSummary>;

    /// Return the reference gas price for the network
    #[method(name = "getReferenceGasPrice")]
    async fn get_reference_gas_price(&self) -> RpcResult<BigInt<u64>>;

    /// Return the validator APY
    #[method(name = "getValidatorsApy")]
    async fn get_validators_apy(&self) -> RpcResult<ValidatorApys>;
}
