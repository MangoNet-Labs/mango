// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use move_core_types::{account_address::AccountAddress, ident_str, identifier::IdentStr};

use crate::base_types::SequenceNumber;

use crate::error::MgoResult;
use crate::object::Owner;
use crate::storage::ObjectStore;
use crate::{MGO_FRAMEWORK_ADDRESS, MGO_RANDOMNESS_STATE_OBJECT_ID};

pub const RANDOMNESS_MODULE_NAME: &IdentStr = ident_str!("random");
pub const RANDOMNESS_STATE_STRUCT_NAME: &IdentStr = ident_str!("Random");
pub const RANDOMNESS_STATE_UPDATE_FUNCTION_NAME: &IdentStr = ident_str!("update_randomness_state");
pub const RANDOMNESS_STATE_CREATE_FUNCTION_NAME: &IdentStr = ident_str!("create");
pub const RESOLVED_MGO_RANDOMNESS_STATE: (&AccountAddress, &IdentStr, &IdentStr) = (
    &MGO_FRAMEWORK_ADDRESS,
    RANDOMNESS_MODULE_NAME,
    RANDOMNESS_STATE_STRUCT_NAME,
);

pub fn get_randomness_state_obj_initial_shared_version(
    object_store: &dyn ObjectStore,
) -> MgoResult<Option<SequenceNumber>> {
    Ok(object_store
        .get_object(&MGO_RANDOMNESS_STATE_OBJECT_ID)?
        .map(|obj| match obj.owner {
            Owner::Shared {
                initial_shared_version,
            } => initial_shared_version,
            _ => unreachable!("Randomness state object must be shared"),
        }))
}
