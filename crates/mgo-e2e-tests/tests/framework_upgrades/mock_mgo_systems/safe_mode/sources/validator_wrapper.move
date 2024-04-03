// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

module mgo_system::validator_wrapper {
    use mgo::versioned::Versioned;

    friend mgo_system::mgo_system_state_inner;

    struct ValidatorWrapper has store {
        inner: Versioned
    }
}
