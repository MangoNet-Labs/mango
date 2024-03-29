// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests invalid deletion of an object that has children

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

    public entry fun mint(ctx: &mut TxContext) {
        let s = S { id: mgo::object::new(ctx) };
        mgo::transfer::transfer(s, tx_context::sender(ctx))
    }

    public entry fun add(parent: &mut S, idx: u64, ctx: &mut TxContext) {
        let child = S { id: mgo::object::new(ctx) };
        ofield::add(&mut parent.id, idx, child);
    }

    public entry fun wrap(s: S, ctx: &mut TxContext) {
        let r = R { id: mgo::object::new(ctx), s };
        mgo::transfer::transfer(r, tx_context::sender(ctx))
    }

    public entry fun delete(r: R) {
        let R { id, s } = r;
        mgo::object::delete(id);
        let S { id } = s;
        mgo::object::delete(id);
    }
}

//# run test::m::mint --sender A

//# run test::m::add --sender A --args object(2,0) 0

//# run test::m::wrap --sender A --args object(2,0)
