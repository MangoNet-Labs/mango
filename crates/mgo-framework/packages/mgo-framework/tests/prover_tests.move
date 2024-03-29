// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module mgo::prover_tests {
    use mgo::object::UID;

    struct Obj has key, store {
        id: UID
    }

    // ====================================================================
    // Object ownership
    // ====================================================================

    public fun simple_transfer(o: Obj, recipient: address) {
        mgo::transfer::public_transfer(o, recipient);
    }

    public fun simple_share(o: Obj) {
        mgo::transfer::public_share_object(o)
    }

    public fun simple_freeze(o: Obj) {
        mgo::transfer::public_freeze_object(o)
    }

    public fun simple_delete(o: Obj) {
        let Obj { id } = o;
        mgo::object::delete(id);
    }

    // ====================================================================
    // Dynamic fields
    // ====================================================================

    public fun simple_field_add(o: &mut Obj, n1: u64, v1: u8, n2: u8, v2: u64) {
        mgo::dynamic_field::add(&mut o.id, n1, v1);
        mgo::dynamic_field::add(&mut o.id, n2, v2);
    }

    public fun simple_field_remove(o: &mut Obj, n1: u64, n2: u8) {
        mgo::dynamic_field::remove<u64,u8>(&mut o.id, n1);
        mgo::dynamic_field::remove<u8,u64>(&mut o.id, n2);
    }
}
