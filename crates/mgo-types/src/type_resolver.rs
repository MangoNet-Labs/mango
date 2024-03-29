// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use crate::error::{ExecutionError, MgoError};
use move_core_types::{
    annotated_value as A,
    language_storage::{StructTag, TypeTag},
};
use move_vm_types::loaded_data::runtime_types::Type;

pub trait LayoutResolver {
    fn get_annotated_layout(
        &mut self,
        struct_tag: &StructTag,
    ) -> Result<A::MoveStructLayout, MgoError>;
}

pub trait TypeTagResolver {
    fn get_type_tag(&self, type_: &Type) -> Result<TypeTag, ExecutionError>;
}
