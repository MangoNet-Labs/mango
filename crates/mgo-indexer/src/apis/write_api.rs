// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use fastcrypto::encoding::Base64;
use jsonrpsee::core::RpcResult;
use jsonrpsee::http_client::HttpClient;
use jsonrpsee::RpcModule;

use mgo_json_rpc::MgoRpcModule;
use mgo_json_rpc_api::{WriteApiClient, WriteApiServer};
use mgo_json_rpc_types::{
    DevInspectArgs, DevInspectResults, DryRunTransactionBlockResponse, MgoTransactionBlockResponse,
    MgoTransactionBlockResponseOptions,
};
use mgo_open_rpc::Module;
use mgo_types::base_types::MgoAddress;
use mgo_types::quorum_driver_types::ExecuteTransactionRequestType;
use mgo_types::mgo_serde::BigInt;

use crate::types::MgoTransactionBlockResponseWithOptions;

pub(crate) struct WriteApi {
    fullnode: HttpClient,
}

impl WriteApi {
    pub fn new(fullnode_client: HttpClient) -> Self {
        Self {
            fullnode: fullnode_client,
        }
    }
}

#[async_trait]
impl WriteApiServer for WriteApi {
    async fn execute_transaction_block(
        &self,
        tx_bytes: Base64,
        signatures: Vec<Base64>,
        options: Option<MgoTransactionBlockResponseOptions>,
        request_type: Option<ExecuteTransactionRequestType>,
    ) -> RpcResult<MgoTransactionBlockResponse> {
        let fast_path_options = MgoTransactionBlockResponseOptions::full_content();
        let mgo_transaction_response = self
            .fullnode
            .execute_transaction_block(tx_bytes, signatures, Some(fast_path_options), request_type)
            .await?;

        Ok(MgoTransactionBlockResponseWithOptions {
            response: mgo_transaction_response,
            options: options.unwrap_or_default(),
        }
        .into())
    }

    async fn dev_inspect_transaction_block(
        &self,
        sender_address: MgoAddress,
        tx_bytes: Base64,
        gas_price: Option<BigInt<u64>>,
        epoch: Option<BigInt<u64>>,
        additional_args: Option<DevInspectArgs>,
    ) -> RpcResult<DevInspectResults> {
        self.fullnode
            .dev_inspect_transaction_block(
                sender_address,
                tx_bytes,
                gas_price,
                epoch,
                additional_args,
            )
            .await
    }

    async fn dry_run_transaction_block(
        &self,
        tx_bytes: Base64,
    ) -> RpcResult<DryRunTransactionBlockResponse> {
        self.fullnode.dry_run_transaction_block(tx_bytes).await
    }
}

impl MgoRpcModule for WriteApi {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        mgo_json_rpc_api::WriteApiOpenRpc::module_doc()
    }
}
