// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub use balance_changes::*;
pub use object_changes::*;
pub use mgo_checkpoint::*;
pub use mgo_coin::*;
pub use mgo_event::*;
pub use mgo_extended::*;
pub use mgo_governance::*;
pub use mgo_move::*;
pub use mgo_object::*;
pub use mgo_protocol::*;
pub use mgo_transaction::*;
use mgo_types::base_types::ObjectID;
use mgo_types::dynamic_field::DynamicFieldInfo;

#[cfg(test)]
#[path = "unit_tests/rpc_types_tests.rs"]
mod rpc_types_tests;

mod balance_changes;
mod displays;
mod object_changes;
mod mgo_checkpoint;
mod mgo_coin;
mod mgo_event;
mod mgo_extended;
mod mgo_governance;
mod mgo_move;
mod mgo_object;
mod mgo_protocol;
mod mgo_transaction;

pub type DynamicFieldPage = Page<DynamicFieldInfo, ObjectID>;
/// `next_cursor` points to the last item in the page;
/// Reading with `next_cursor` will start from the next item after `next_cursor` if
/// `next_cursor` is `Some`, otherwise it will start from the first item.
#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Page<T, C> {
    pub data: Vec<T>,
    pub next_cursor: Option<C>,
    pub has_next_page: bool,
}

impl<T, C> Page<T, C> {
    pub fn empty() -> Self {
        Self {
            data: vec![],
            next_cursor: None,
            has_next_page: false,
        }
    }
}
