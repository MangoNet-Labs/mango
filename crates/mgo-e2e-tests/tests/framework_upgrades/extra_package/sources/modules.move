// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

module mgo_extra::msim_extra_1 {
    use mgo::object::{Self, UID};
    use mgo::transfer;
    use mgo::tx_context::TxContext;

    struct S has key { id: UID }

    fun init(ctx: &mut TxContext) {
        transfer::share_object(S {
            id: object::new(ctx)
        })
    }

    public fun canary(): u64 {
        43
    }
}
