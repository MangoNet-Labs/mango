module token::meth {
    use std::option;
    use mgo::coin::{Self, TreasuryCap};
    use mgo::transfer;
    use mgo::tx_context::{Self, TxContext};
    use mgo::url;

    struct METH has drop {}

    fun init(witness: METH, ctx: &mut TxContext) {
        let (treasury, metadata) = coin::create_currency(
            witness,
            9,
            b"METH",
            b"METH",
            b"METH Coin",
            option::some(
                url::new_unsafe_from_bytes(b"https://image.devnet.mangonetwork.io/img/token/ethereum-eth-logo.png")
            ),
            ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury, tx_context::sender(ctx))
    }

    public entry fun mint(
        treasury_cap: &mut TreasuryCap<METH>, amount: u64, recipient: address, ctx: &mut TxContext
    ) {
        coin::mint_and_transfer(treasury_cap, amount, recipient, ctx);
    }
}