// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

module move_test_code::shared_objects_version {
    use mgo::object::{Self, UID};
    use mgo::transfer;
    use mgo::tx_context::{Self, TxContext};

    struct Counter has key {
        id: UID,
        value: u64,
    }

    public entry fun create_counter(ctx: &mut TxContext) {
        transfer::transfer(
            Counter {
                id: object::new(ctx),
                value: 0,
            },
            tx_context::sender(ctx),
        )
    }

    public entry fun create_shared_counter(ctx: &mut TxContext) {
        transfer::share_object(Counter {
            id: object::new(ctx),
            value: 0,
        })
    }

    public entry fun share_counter(counter: Counter) {
        transfer::share_object(counter)
    }

    public entry fun increment_counter(counter: &mut Counter) {
        counter.value = counter.value + 1
    }
}
