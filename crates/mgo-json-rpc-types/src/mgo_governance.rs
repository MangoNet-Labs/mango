// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use mgo_types::base_types::{AuthorityName, EpochId, ObjectID, MgoAddress};
use mgo_types::committee::{Committee, StakeUnit};
use mgo_types::mgo_serde::BigInt;

/// RPC representation of the [Committee] type.
#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename = "CommitteeInfo")]
pub struct MgoCommittee {
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub epoch: EpochId,
    #[schemars(with = "Vec<(AuthorityName, BigInt<u64>)>")]
    #[serde_as(as = "Vec<(_, BigInt<u64>)>")]
    pub validators: Vec<(AuthorityName, StakeUnit)>,
}

impl From<Committee> for MgoCommittee {
    fn from(committee: Committee) -> Self {
        Self {
            epoch: committee.epoch,
            validators: committee.voting_rights,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct DelegatedStake {
    /// Validator's Address.
    pub validator_address: MgoAddress,
    /// Staking pool object id.
    pub staking_pool: ObjectID,
    pub stakes: Vec<Stake>,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(tag = "status")]
pub enum StakeStatus {
    Pending,
    #[serde(rename_all = "camelCase")]
    Active {
        #[schemars(with = "BigInt<u64>")]
        #[serde_as(as = "BigInt<u64>")]
        estimated_reward: u64,
    },
    Unstaked,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Stake {
    /// ID of the StakedMgo receipt object.
    pub staked_mgo_id: ObjectID,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub stake_request_epoch: EpochId,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub stake_active_epoch: EpochId,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub principal: u64,
    #[serde(flatten)]
    pub status: StakeStatus,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ValidatorApys {
    pub apys: Vec<ValidatorApy>,
    #[schemars(with = "BigInt<u64>")]
    #[serde_as(as = "BigInt<u64>")]
    pub epoch: EpochId,
}

#[serde_as]
#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ValidatorApy {
    pub address: MgoAddress,
    pub apy: f64,
}
