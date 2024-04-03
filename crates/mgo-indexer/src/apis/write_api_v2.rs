// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// TODO remove after the functions are implemented
#![allow(unused_variables)]
#![allow(dead_code)]

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use mgo_json_rpc::MgoRpcModule;
use mgo_json_rpc_api::WriteApiServer;
use mgo_json_rpc_types::{
    DevInspectArgs, DevInspectResults, DryRunTransactionBlockResponse, MgoTransactionBlockResponse,
    MgoTransactionBlockResponseOptions,
};
use mgo_open_rpc::Module;
use mgo_types::base_types::MgoAddress;
use mgo_types::quorum_driver_types::ExecuteTransactionRequestType;
use mgo_types::mgo_serde::BigInt;

pub(crate) struct WriteApiV2 {
    fullnode_client: HttpClient,
}

impl WriteApiV2 {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self { fullnode_client }
    }
}

#[async_trait]
impl WriteApiServer for WriteApiV2 {
    async fn execute_transaction_block(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        options: Option<MgoTransactionBlockResponseOptions>,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<MgoTransactionBlockResponse> {
        unimplemented!()
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender_address: MgoAddress,
        tx_bytes: Base64,
        gas_price: Option<BigInt<u64>>,
        epoch: Option<BigInt<u64>>,
        additional_args: Option<DevInspectArgs>,
    ) -> RpcResult<DevInspectResults> {
        unimplemented!()
    }

    async fn dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> RpcResult<DryRunTransactionBlockResponse> {
        unimplemented!()
    }
}

impl MgoRpcModule for WriteApiV2 {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        mgo_json_rpc_api::WriteApiOpenRpc::module_doc()
    }
}
