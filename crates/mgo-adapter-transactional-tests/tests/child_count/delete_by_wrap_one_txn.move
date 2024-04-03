// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests invalid wrapping of a parent object with children, in a single transaction

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use mgo::tx_context::{Self, TxContext};
    use mgo::dynamic_object_field as ofield;

    struct S has key, store {
        id: mgo::object::UID,
    }

    struct R has key {
        id: mgo::object::UID,
        s: S,
    }

    public entry fun test_wrap(ctx: &mut TxContext) {
        let id = mgo::object::new(ctx);
        let child = S { id: mgo::object::new(ctx) };
        ofield::add(&mut id, 0, child);
        let parent = S { id };
        let r = R { id: mgo::object::new(ctx), s: parent };
        mgo::transfer::transfer(r, tx_context::sender(ctx))
    }
}

//# run test::m::test_wrap --sender A
