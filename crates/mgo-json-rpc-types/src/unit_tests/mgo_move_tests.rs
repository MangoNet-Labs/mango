// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use mgo_enum_compat_util::*;

use crate::{MgoMoveStruct, MgoMoveValue};

#[test]
fn enforce_order_test() {
    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "mgo_move_struct.yaml"]);
    check_enum_compat_order::<MgoMoveStruct>(path);

    let mut path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "staged", "mgo_move_value.yaml"]);
    check_enum_compat_order::<MgoMoveValue>(path);
}
