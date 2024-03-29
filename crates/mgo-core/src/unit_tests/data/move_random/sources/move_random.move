// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module examples::move_random {
    use std::vector;
    use mgo::object::{Self, UID};
    use mgo::transfer;
    use mgo::tx_context::TxContext;

    struct Object has key, store {
        id: UID,
        data: vector<u64>,
    }

    // simple infinite loop to go out of gas in computation
    public entry fun loopy() {
        loop { }
    }

    // create an object with a vector of size `size` and transfer to recipient
    public entry fun storage_heavy(size: u64, recipient: address, ctx: &mut TxContext) {
        let data = vector::empty();
        while (size > 0) {
            vector::push_back(&mut data, size);
            size = size - 1;
        };
        transfer::public_transfer(
            Object { id: object::new(ctx), data },
            recipient
        )
    }
}
