// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests various ways of "removing" a child decrements the count

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use mgo::tx_context::{Self, TxContext};
    use mgo::dynamic_object_field as ofield;

    struct S has key, store {
        id: mgo::object::UID,
    }

    struct R has key, store {
        id: mgo::object::UID,
        s: S,
    }

    public entry fun mint(ctx: &mut TxContext) {
        let id = mgo::object::new(ctx);
        mgo::transfer::public_transfer(S { id }, tx_context::sender(ctx))
    }

    public entry fun add(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child = S { id: mgo::object::new(ctx) };
        ofield::add(&mut parent.id, idx, child);
    }

    public entry fun remove(parent: &mut S, idx: u64) {
        let S { id } = ofield::remove(&mut parent.id, idx);
        mgo::object::delete(id)
    }

    public entry fun remove_and_add(parent: &mut S, idx: u64) {
        let child: S = ofield::remove(&mut parent.id, idx);
        ofield::add(&mut parent.id, idx, child)
    }

    public entry fun remove_and_wrap(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child: S = ofield::remove(&mut parent.id, idx);
        ofield::add(&mut parent.id, idx, R { id: mgo::object::new(ctx), s: child })
    }
}

//
// Test remove
//

//# run test::m::mint --sender A

//# view-object 2,0

//# run test::m::add --sender A --args object(2,0) 1

//# run test::m::remove --sender A --args object(2,0) 1

//# view-object 2,0

//
// Test remove and add
//

//# run test::m::mint --sender A

//# view-object 7,0

//# run test::m::add --sender A --args object(7,0) 1

//# run test::m::remove_and_add --sender A --args object(7,0) 1

//# view-object 7,0

//
// Test remove and wrap
//

//# run test::m::mint --sender A

//# view-object 12,0

//# run test::m::add --sender A --args object(12,0) 1

//# run test::m::remove_and_wrap --sender A --args object(12,0) 1

//# view-object 12,0
