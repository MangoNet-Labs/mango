module token::msui {
    use std::option;
    use mgo::coin::{Self, TreasuryCap};
    use mgo::transfer;
    use mgo::tx_context::{Self, TxContext};
    use mgo::url;

    struct MSUI has drop {}

    fun init(witness: MSUI, ctx: &mut TxContext) {
        let (treasury, metadata) = coin::create_currency(
            witness,
            9,
            b"MSUI",
            b"MSUI",
            b"MSUI Coin",
            option::some(
                url::new_unsafe_from_bytes(b"https://image.devnet.mangonetwork.io/img/token/sui-sui-logo.png")
            ),
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury, tx_context::sender(ctx))
    }

    public entry fun mint(
        treasury_cap: &mut TreasuryCap<MSUI>, amount: u64, recipient: address, ctx: &mut TxContext
    ) {
        coin::mint_and_transfer(treasury_cap, amount, recipient, ctx);
    }
}