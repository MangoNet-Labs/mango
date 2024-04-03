// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

//# init --addresses Test=0x0

//# publish --upgradeable
module Test::M1 {
    use mgo::tx_context::TxContext;
    fun init(_ctx: &mut TxContext) { }
}

//# view-object 1,1
