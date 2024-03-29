// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use mgo_json::MgoJsonValue;
use mgo_json_rpc::MgoRpcModule;
use mgo_json_rpc_api::{TransactionBuilderClient, TransactionBuilderServer};
use mgo_json_rpc_types::{
    RPCTransactionRequestParams, MgoTransactionBlockBuilderMode, MgoTypeTag, TransactionBlockBytes,
};
use mgo_open_rpc::Module;
use mgo_types::base_types::{ObjectID, MgoAddress};
use mgo_types::mgo_serde::BigInt;

pub(crate) struct TransactionBuilderApi {
    fullnode: HttpClient,
}

impl TransactionBuilderApi {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self {
            fullnode: fullnode_client,
        }
    }
}

#[async_trait]
impl TransactionBuilderServer for TransactionBuilderApi {
    async fn transfer_object(
        &self,
        signer: MgoAddress,
        object_id: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
        recipient: MgoAddress,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .transfer_object(signer, object_id, gas, gas_budget, recipient)
            .await
    }

    async fn transfer_mgo(
        &self,
        signer: MgoAddress,
        mgo_object_id: ObjectID,
        gas_budget: BigInt<u64>,
        recipient: MgoAddress,
        amount: Option<BigInt<u64>>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .transfer_mgo(signer, mgo_object_id, gas_budget, recipient, amount)
            .await
    }

    async fn pay(
        &self,
        signer: MgoAddress,
        input_coins: Vec<ObjectID>,
        recipients: Vec<MgoAddress>,
        amounts: Vec<BigInt<u64>>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .pay(signer, input_coins, recipients, amounts, gas, gas_budget)
            .await
    }

    async fn pay_mgo(
        &self,
        signer: MgoAddress,
        input_coins: Vec<ObjectID>,
        recipients: Vec<MgoAddress>,
        amounts: Vec<BigInt<u64>>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .pay_mgo(signer, input_coins, recipients, amounts, gas_budget)
            .await
    }

    async fn pay_all_mgo(
        &self,
        signer: MgoAddress,
        input_coins: Vec<ObjectID>,
        recipient: MgoAddress,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .pay_all_mgo(signer, input_coins, recipient, gas_budget)
            .await
    }

    async fn publish(
        &self,
        sender: MgoAddress,
        compiled_modules: Vec<Base64>,
        dep_ids: Vec<ObjectID>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .publish(sender, compiled_modules, dep_ids, gas, gas_budget)
            .await
    }

    async fn split_coin(
        &self,
        signer: MgoAddress,
        coin_object_id: ObjectID,
        split_amounts: Vec<BigInt<u64>>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .split_coin(signer, coin_object_id, split_amounts, gas, gas_budget)
            .await
    }

    async fn split_coin_equal(
        &self,
        signer: MgoAddress,
        coin_object_id: ObjectID,
        split_count: BigInt<u64>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .split_coin_equal(signer, coin_object_id, split_count, gas, gas_budget)
            .await
    }

    async fn merge_coin(
        &self,
        signer: MgoAddress,
        primary_coin: ObjectID,
        coin_to_merge: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .merge_coin(signer, primary_coin, coin_to_merge, gas, gas_budget)
            .await
    }

    async fn move_call(
        &self,
        signer: MgoAddress,
        package_object_id: ObjectID,
        module: String,
        function: String,
        type_arguments: Vec<MgoTypeTag>,
        rpc_arguments: Vec<MgoJsonValue>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
        tx_builder_mode: Option<MgoTransactionBlockBuilderMode>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .move_call(
                signer,
                package_object_id,
                module,
                function,
                type_arguments,
                rpc_arguments,
                gas,
                gas_budget,
                tx_builder_mode,
            )
            .await
    }

    async fn batch_transaction(
        &self,
        signer: MgoAddress,
        params: Vec<RPCTransactionRequestParams>,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
        tx_builder_mode: Option<MgoTransactionBlockBuilderMode>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .batch_transaction(signer, params, gas, gas_budget, tx_builder_mode)
            .await
    }

    async fn request_add_stake(
        &self,
        signer: MgoAddress,
        coins: Vec<ObjectID>,
        amount: Option<BigInt<u64>>,
        validator: MgoAddress,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .request_add_stake(signer, coins, amount, validator, gas, gas_budget)
            .await
    }

    async fn request_withdraw_stake(
        &self,
        signer: MgoAddress,
        staked_mgo: ObjectID,
        gas: Option<ObjectID>,
        gas_budget: BigInt<u64>,
    ) -> RpcResult<TransactionBlockBytes> {
        self.fullnode
            .request_withdraw_stake(signer, staked_mgo, gas, gas_budget)
            .await
    }
}

impl MgoRpcModule for TransactionBuilderApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        mgo_json_rpc_api::TransactionBuilderOpenRpc::module_doc()
    }
}
