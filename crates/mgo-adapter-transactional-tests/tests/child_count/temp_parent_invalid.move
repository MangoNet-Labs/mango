// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// DEPRECATED child count no longer tracked
// tests the invalid creation and deletion of a parent object

//# init --addresses test=0x0 --accounts A B

//# publish

module test::m {
    use mgo::tx_context::TxContext;

    struct S has key, store {
        id: mgo::object::UID,
    }

    public entry fun t(ctx: &mut TxContext) {
        let parent = mgo::object::new(ctx);
        let child = S { id: mgo::object::new(ctx) };
        mgo::dynamic_object_field::add(&mut parent, 0, child);
        mgo::object::delete(parent);
    }
}

//# run test::m::t --sender A
