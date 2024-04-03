// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

// tests cannot call init with programmable transactions

//# init --addresses test=0x0 --accounts A

//# publish
module test::m1 {
    struct A has copy, drop, store {}
    public fun a(): A { A {} }
}

//# programmable
//> 0: test::m1::a();
//> mgo::event::emit<test::m1::A>(Result(0));

//# programmable
//> 0: test::m1::a();
// wrong type annotation doesn't matter
//> mgo::event::emit<bool>(Result(0));

//# programmable
//> 0: test::m1::a();
// function doesn't exist
//> mgo::event::does_not_exist<test::m1::A>(Result(0));
