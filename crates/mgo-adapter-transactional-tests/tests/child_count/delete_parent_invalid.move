// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests that the parent cannot be deleted while it has children

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use mgo::tx_context::{Self, TxContext};
    use mgo::dynamic_object_field as ofield;

    struct S has key, store {
        id: mgo::object::UID,
    }

    public entry fun mint(ctx: &mut TxContext) {
        let id = mgo::object::new(ctx);
        mgo::transfer::public_transfer(S { id }, tx_context::sender(ctx))
    }

    public entry fun add(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child = S { id: mgo::object::new(ctx) };
        ofield::add(&mut parent.id, idx, child);
    }

    public entry fun delete(s: S) {
        let S { id } = s;
        mgo::object::delete(id)
    }
}

//# run test::m::mint --sender A

//# run test::m::add --sender A --args object(2,0) 0

//# view-object 2,0

//# run test::m::delete --sender A --args object(2,0)
