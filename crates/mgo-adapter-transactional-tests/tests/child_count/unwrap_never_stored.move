// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests deleting a wrapped object that has never been in storage

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use mgo::tx_context::{Self, TxContext};

    struct S has key, store {
        id: mgo::object::UID,
    }

    struct R has key {
        id: mgo::object::UID,
        s: S,
    }

    public entry fun create(ctx: &mut TxContext) {
        let parent = mgo::object::new(ctx);
        let child = S { id: mgo::object::new(ctx) };
        mgo::transfer::transfer(R { id: parent, s: child }, tx_context::sender(ctx))
    }

    public entry fun delete(r: R) {
        let R { id, s } = r;
        mgo::object::delete(id);
        let S { id } = s;
        mgo::object::delete(id);
    }
}

//
// Test sharing
//

//# run test::m::create --sender A

//# run test::m::delete --args object(2,0) --sender A
