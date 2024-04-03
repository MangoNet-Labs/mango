// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

module examples::publish_with_event {
    use std::ascii::{Self, String};

    use mgo::event;
    use mgo::tx_context::TxContext;

    struct PublishEvent has copy, drop {
        foo: String
    }

    fun init(_ctx: &mut TxContext) {
        event::emit(PublishEvent { foo: ascii::string(b"bar") })
    }
}
