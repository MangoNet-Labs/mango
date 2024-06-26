// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

module move_test_code::regulated_coin {
    use std::option;
    use mgo::coin;
    use mgo::object::UID;
    use mgo::transfer;
    use mgo::tx_context;
    use mgo::tx_context::TxContext;

    struct REGULATED_COIN has drop {}

    struct Wallet has key {
        id: UID,
    }

    fun init(otw: REGULATED_COIN, ctx: &mut TxContext) {
        let (treasury_cap, deny_cap, metadata) = coin::create_regulated_currency(
            otw,
            9,
            b"RC",
            b"REGULATED_COIN",
            b"A new regulated coin",
            option::none(),
            ctx
        );
        transfer::public_transfer(deny_cap, tx_context::sender(ctx));
        transfer::public_freeze_object(treasury_cap);
        transfer::public_freeze_object(metadata);
    }
}
