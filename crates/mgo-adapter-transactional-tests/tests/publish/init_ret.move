//# init --addresses Test=0x0
// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//# publish

// initializer not valid due to return value

module Test::M1 {
    use mgo::object::{Self, UID};
    use mgo::tx_context::{Self, TxContext};
    use mgo::transfer;

    struct Object has key, store {
        id: UID,
        value: u64,
    }

    // initializer that should be executed upon publishing this module
    fun init(ctx: &mut TxContext): u64 {
        let value = 42;
        let singleton = Object { id: object::new(ctx), value };
        transfer::public_transfer(singleton, tx_context::sender(ctx));
        value
    }
}
