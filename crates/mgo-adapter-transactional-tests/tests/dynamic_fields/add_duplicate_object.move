// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// similar to dynamic_object_field_tests but over multiple transactions,
// as this uses a different code path
// test duplicate add

//# init --addresses a=0x0 --accounts A

//# publish
module a::m {

use mgo::dynamic_object_field::add;
use mgo::object;
use mgo::tx_context::{sender, TxContext};

struct Obj has key, store {
    id: object::UID,
}

entry fun t1(ctx: &mut TxContext) {
    let id = object::new(ctx);
    add(&mut id, 0, Obj { id: object::new(ctx) });
    mgo::transfer::public_transfer(Obj { id }, sender(ctx))
}

entry fun t2(obj: &mut Obj, ctx: &mut TxContext) {
    add(&mut obj.id, 0, Obj { id: object::new(ctx) });
}

}

//# run a::m::t1 --sender A

//# run a::m::t2 --sender A --args object(2,0)
