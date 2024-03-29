// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::collections::BTreeMap;

use async_trait::async_trait;
use jsonrpsee::core::RpcResult;
use jsonrpsee::RpcModule;

use mgo_json_rpc::error::MgoRpcInputError;
use mgo_json_rpc::MgoRpcModule;
use mgo_json_rpc_api::MoveUtilsServer;
use mgo_json_rpc_types::ObjectValueKind;
use mgo_json_rpc_types::MgoMoveNormalizedType;
use mgo_json_rpc_types::{
    MoveFunctionArgType, MgoMoveNormalizedFunction, MgoMoveNormalizedModule,
    MgoMoveNormalizedStruct,
};
use mgo_open_rpc::Module;
use mgo_types::base_types::ObjectID;
use mgo_types::move_package::normalize_modules;

use crate::indexer_reader::IndexerReader;

pub struct MoveUtilsApiV2 {
    inner: IndexerReader,
}

impl MoveUtilsApiV2 {
    pub fn new(inner: IndexerReader) -> Self {
        Self { inner }
    }
}

#[async_trait]
impl MoveUtilsServer for MoveUtilsApiV2 {
    async fn get_normalized_move_modules_by_package(
        &self,
        package_id: ObjectID,
    ) -> RpcResult<BTreeMap<String, MgoMoveNormalizedModule>> {
        let package = self
            .inner
            .get_package_in_blocking_task(package_id)
            .await
            .map_err(|e| MgoRpcInputError::GenericNotFound(e.to_string()))?
            .ok_or_else(|| {
                MgoRpcInputError::GenericNotFound(format!(
                    "Package object does not exist with ID {package_id}",
                ))
            })?;
        let modules =
                // we are on the read path - it's OK to use VERSION_MAX of the supported Move
                // binary format
                normalize_modules(
                    package.serialized_module_map().values(),
                    /* max_binary_format_version */ move_binary_format::file_format_common::VERSION_MAX,
                    /* no_extraneous_module_bytes */ false,
                )
                .map_err(|e| MgoRpcInputError::GenericInvalid(e.to_string()))?;
        Ok(modules
            .into_iter()
            .map(|(name, module)| (name, module.into()))
            .collect::<BTreeMap<String, MgoMoveNormalizedModule>>())
    }

    async fn get_normalized_move_module(
        &self,
        package: ObjectID,
        module_name: String,
    ) -> RpcResult<MgoMoveNormalizedModule> {
        let mut modules = self.get_normalized_move_modules_by_package(package).await?;
        let module = modules.remove(&module_name).ok_or_else(|| {
            MgoRpcInputError::GenericNotFound(format!(
                "No module was found with name {module_name}",
            ))
        })?;
        Ok(module)
    }

    async fn get_normalized_move_struct(
        &self,
        package: ObjectID,
        module_name: String,
        struct_name: String,
    ) -> RpcResult<MgoMoveNormalizedStruct> {
        let mut module = self
            .get_normalized_move_module(package, module_name)
            .await?;
        module
            .structs
            .remove(&struct_name)
            .ok_or_else(|| {
                MgoRpcInputError::GenericNotFound(format!(
                    "No struct was found with struct name {struct_name}"
                ))
            })
            .map_err(Into::into)
    }

    async fn get_normalized_move_function(
        &self,
        package: ObjectID,
        module_name: String,
        function_name: String,
    ) -> RpcResult<MgoMoveNormalizedFunction> {
        let mut module = self
            .get_normalized_move_module(package, module_name)
            .await?;
        module
            .exposed_functions
            .remove(&function_name)
            .ok_or_else(|| {
                MgoRpcInputError::GenericNotFound(format!(
                    "No function was found with function name {function_name}",
                ))
            })
            .map_err(Into::into)
    }

    async fn get_move_function_arg_types(
        &self,
        package: ObjectID,
        module: String,
        function: String,
    ) -> RpcResult<Vec<MoveFunctionArgType>> {
        let function = self
            .get_normalized_move_function(package, module, function)
            .await?;
        let args = function
            .parameters
            .iter()
            .map(|p| match p {
                MgoMoveNormalizedType::Struct { .. } => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByValue)
                }
                MgoMoveNormalizedType::Vector(_) => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByValue)
                }
                MgoMoveNormalizedType::Reference(_) => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByImmutableReference)
                }
                MgoMoveNormalizedType::MutableReference(_) => {
                    MoveFunctionArgType::Object(ObjectValueKind::ByMutableReference)
                }
                _ => MoveFunctionArgType::Pure,
            })
            .collect::<Vec<MoveFunctionArgType>>();
        Ok(args)
    }
}

impl MgoRpcModule for MoveUtilsApiV2 {
    fn rpc(self) -> RpcModule<Self> {
        self.into_rpc()
    }

    fn rpc_doc_module() -> Module {
        mgo_json_rpc_api::MoveUtilsOpenRpc::module_doc()
    }
}
