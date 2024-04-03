// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

pub const TEST_DIR: &str = "tests";
use mgo_transactional_test_runner::run_test;

datatest_stable::harness!(run_test, TEST_DIR, r".*\.(mvir|move)$");
