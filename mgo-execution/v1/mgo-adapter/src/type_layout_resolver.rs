// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use crate::programmable_transactions::context::load_type_from_struct;
use crate::programmable_transactions::linkage_view::LinkageView;
use move_core_types::account_address::AccountAddress;
use move_core_types::annotated_value as A;
use move_core_types::language_storage::StructTag;
use move_core_types::resolver::ResourceResolver;
use move_vm_runtime::move_vm::MoveVM;
use mgo_types::base_types::ObjectID;
use mgo_types::error::MgoResult;
use mgo_types::execution::TypeLayoutStore;
use mgo_types::storage::{BackingPackageStore, PackageObject};
use mgo_types::{error::MgoError, type_resolver::LayoutResolver};

/// Retrieve a `MoveStructLayout` from a `Type`.
/// Invocation into the `Session` to leverage the `LinkageView` implementation
/// common to the runtime.
pub struct TypeLayoutResolver<'state, 'vm> {
    vm: &'vm MoveVM,
    linkage_view: LinkageView<'state>,
}

/// Implements MgoResolver traits by providing null implementations for module and resource
/// resolution and delegating backing package resolution to the trait object.
struct NullMgoResolver<'state>(Box<dyn TypeLayoutStore + 'state>);

impl<'state, 'vm> TypeLayoutResolver<'state, 'vm> {
    pub fn new(vm: &'vm MoveVM, state_view: Box<dyn TypeLayoutStore + 'state>) -> Self {
        let linkage_view = LinkageView::new(Box::new(NullMgoResolver(state_view)));
        Self { vm, linkage_view }
    }
}

impl<'state, 'vm> LayoutResolver for TypeLayoutResolver<'state, 'vm> {
    fn get_annotated_layout(
        &mut self,
        struct_tag: &StructTag,
    ) -> Result<A::MoveStructLayout, MgoError> {
        let Ok(ty) = load_type_from_struct(self.vm, &mut self.linkage_view, &[], struct_tag) else {
            return Err(MgoError::FailObjectLayout {
                st: format!("{}", struct_tag),
            });
        };
        let layout = self.vm.get_runtime().type_to_fully_annotated_layout(&ty);
        let Ok(A::MoveTypeLayout::Struct(layout)) = layout else {
            return Err(MgoError::FailObjectLayout {
                st: format!("{}", struct_tag),
            });
        };
        Ok(layout)
    }
}

impl<'state> BackingPackageStore for NullMgoResolver<'state> {
    fn get_package_object(&self, package_id: &ObjectID) -> MgoResult<Option<PackageObject>> {
        self.0.get_package_object(package_id)
    }
}

impl<'state> ResourceResolver for NullMgoResolver<'state> {
    type Error = MgoError;

    fn get_resource(
        &self,
        _address: &AccountAddress,
        _typ: &StructTag,
    ) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(None)
    }
}
