// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

//# init --addresses tto=0x0 --accounts A

//# publish
module tto::M1 {
    use mgo::object::{Self, UID};
    use mgo::tx_context::TxContext;
    use mgo::transfer::{Self, Receiving};
    use mgo::dynamic_object_field as dof;

    const KEY: u64 = 0;

    struct A has key, store {
        id: UID,
        value: u64,
    }

    public fun start(ctx: &mut TxContext) {
        let a = A { id: object::new(ctx), value: 0 };
        let a_address = object::id_address(&a);
        let b = A { id: object::new(ctx), value: 0 };
        dof::add(&mut b.id, KEY, A { id: object::new(ctx), value: 0 });
        transfer::public_share_object(a);
        transfer::public_transfer(b, a_address);
    }

    public entry fun receive(parent: &mut A, x: Receiving<A>) {
        let b = transfer::receive(&mut parent.id, x);
        dof::add(&mut parent.id, KEY, b);
        let _: &A = dof::borrow(&parent.id, KEY);
        let x: &mut A = dof::borrow_mut(&mut parent.id, KEY);
        x.value = 100;
    }
}

//# run tto::M1::start

//# view-object 2,0

//# view-object 2,1

//# view-object 2,2

//# view-object 2,3

//# run tto::M1::receive --args object(2,1) receiving(2,3)

//# view-object 2,0

//# view-object 2,1

//# view-object 2,2

//# view-object 2,3
