// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// tests calling public transfer functions

//# init --addresses test=0x0 --accounts A

//# publish
module test::m1 {
    use mgo::object::{Self, UID};
    use mgo::tx_context::TxContext;

    struct Pub has key, store { id: UID }
    public fun pub(ctx: &mut TxContext): Pub { Pub { id: object::new(ctx) } }

    struct Priv has key { id: UID }
    public fun priv(ctx: &mut TxContext): Priv { Priv { id: object::new(ctx) } }
}

// Has store, but cannot be used with internal variants

//# programmable --sender A --inputs @A
//> 0: test::m1::pub();
//> mgo::transfer::transfer<test::m1::Pub>(Result(0), Input(0));

//# programmable
//> 0: test::m1::pub();
//> mgo::transfer::share_object<test::m1::Pub>(Result(0));

//# programmable
//> 0: test::m1::pub();
//> mgo::transfer::freeze_object<test::m1::Pub>(Result(0));


// Does not have store, cannot be used with internal variants

//# programmable --sender A --inputs @A
//> 0: test::m1::priv();
//> mgo::transfer::transfer<test::m1::Priv>(Result(0), Input(0));

//# programmable
//> 0: test::m1::priv();
//> mgo::transfer::share_object<test::m1::Priv>(Result(0));

//# programmable
//> 0: test::m1::priv();
//> mgo::transfer::freeze_object<test::m1::Priv>(Result(0));


// Does not have store, cannot be used with public variants

//# programmable --sender A --inputs @A
//> 0: test::m1::priv();
//> mgo::transfer::public_transfer<test::m1::Priv>(Result(0), Input(0));

//# programmable
//> 0: test::m1::priv();
//> mgo::transfer::public_share_object<test::m1::Priv>(Result(0));

//# programmable
//> 0: test::m1::priv();
//> mgo::transfer::public_freeze_object<test::m1::Priv>(Result(0));
