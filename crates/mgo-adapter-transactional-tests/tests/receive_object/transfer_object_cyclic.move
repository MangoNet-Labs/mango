// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//# init --addresses tto=0x0

//# publish
module tto::M1 {
    use mgo::object::{Self, UID};
    use mgo::tx_context::TxContext;
    use mgo::transfer::{Self};

    struct A has key, store {
        id: UID,
    }

    struct B has key, store {
        id: UID,
    }

    public fun start(ctx: &mut TxContext) {
        let a = A { id: object::new(ctx) };
        let a_address = object::id_address(&a);
        let b = B { id: object::new(ctx) };
        let b_address = object::id_address(&b);
        transfer::public_transfer(a, b_address);
        transfer::public_transfer(b, a_address);
    }
}

//# run tto::M1::start
