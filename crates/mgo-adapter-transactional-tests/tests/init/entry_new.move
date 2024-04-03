// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// init with entry is no longer allowed

//# init --addresses test=0x0

//# publish
module test::m {
    use mgo::tx_context::TxContext;
    entry fun init(_: &mut TxContext) {
    }
}

// TODO double check this error
//# run test::m::init
