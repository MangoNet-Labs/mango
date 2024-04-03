// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

/// Coin<MGO> is the token used to pay for gas in Mgo.
/// It has 9 decimals, and the smallest unit (10^-9) is called "mist".
module mgo::mgo {
    use std::option;
    use mgo::tx_context::{Self, TxContext};
    use mgo::balance::{Self, Balance};
    use mgo::transfer;
    use mgo::coin;

    const EAlreadyMinted: u64 = 0;
    /// Sender is not @0x0 the system address.
    const ENotSystemAddress: u64 = 1;

    #[allow(unused_const)]
    /// The amount of Mist per Mgo token based on the the fact that mist is
    /// 10^-9 of a Mgo token
    const MIST_PER_MGO: u64 = 1_000_000_000;

    #[allow(unused_const)]
    /// The total supply of Mgo denominated in whole Mgo tokens (10 Billion)
    const TOTAL_SUPPLY_MGO: u64 = 10_000_000_000;

    /// The total supply of Mgo denominated in Mist (10 Billion * 10^9)
    const TOTAL_SUPPLY_MIST: u64 = 10_000_000_000_000_000_000;

    /// Name of the coin
    struct MGO has drop {}

    #[allow(unused_function)]
    /// Register the `MGO` Coin to acquire its `Supply`.
    /// This should be called only once during genesis creation.
    fun new(ctx: &mut TxContext): Balance<MGO> {
        assert!(tx_context::sender(ctx) == @0x0, ENotSystemAddress);
        assert!(tx_context::epoch(ctx) == 0, EAlreadyMinted);

        let (treasury, metadata) = coin::create_currency(
            MGO {},
            9,
            b"MGO",
            b"Mgo",
            // TODO: add appropriate description and logo url
            b"",
            option::none(),
            ctx
        );
        transfer::public_freeze_object(metadata);
        let supply = coin::treasury_into_supply(treasury);
        let total_mgo = balance::increase_supply(&mut supply, TOTAL_SUPPLY_MIST);
        balance::destroy_supply(supply);
        total_mgo
    }

    public entry fun transfer(c: coin::Coin<MGO>, recipient: address) {
        transfer::public_transfer(c, recipient)
    }
}
