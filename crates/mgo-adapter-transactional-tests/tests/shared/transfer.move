// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// tests that shared objects cannot be transfered in a way that will result
// in their ownership no longer being shared

//# init --addresses t1=0x0 t2=0x0 --shared-object-deletion true

//# publish

module t2::o2 {
    use mgo::object::{Self, UID};
    use mgo::transfer;
    use mgo::tx_context::{Self, TxContext};

    struct Obj2 has key, store {
        id: UID,
    }

    public entry fun create(ctx: &mut TxContext) {
        let o = Obj2 { id: object::new(ctx) };
        transfer::public_share_object(o)
    }

    public entry fun transfer_to_single_owner(o2: Obj2, ctx: &mut TxContext) {
        transfer::transfer(o2, tx_context::sender(ctx))
    }
}

//# run t2::o2::create

//# view-object 2,0

//# run t2::o2::transfer_to_single_owner --args object(2,0)
