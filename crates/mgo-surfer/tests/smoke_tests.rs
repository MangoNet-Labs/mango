// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;
use std::time::Duration;
use mgo_macros::sim_test;
use mgo_surfer::default_surf_strategy::DefaultSurfStrategy;

#[sim_test]
async fn smoke_test() {
    // This test makes sure that the mgo surfer runs.

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.extend(["tests", "move_building_blocks"]);
    let results = mgo_surfer::run::<DefaultSurfStrategy>(Duration::from_secs(30), vec![path]).await;
    assert!(results.num_successful_transactions > 0);
    assert!(!results.unique_move_functions_called.is_empty());
}
