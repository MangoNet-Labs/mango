module examples::my_coin {
    use mgo::coin::{Self, TreasuryCap};
    use mgo::tx_context;
    use mgo::tx_context::TxContext;
    use mgo::transfer;
    use std::option;
    
    struct MY_COIN has drop {}

    fun init(witness: MY_COIN, ctx: &mut TxContext) {
        let (treasury, metadata) = coin::create_currency(witness, 6, b"MY_COIN", b"", b"", option::none(), ctx);
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury, tx_context::sender(ctx))
    }

    public fun mint(
        treasury_cap: &mut TreasuryCap<MY_COIN>, 
        amount: u64, 
        recipient: address, 
        ctx: &mut TxContext,
    ) {
        let coin = coin::mint(treasury_cap, amount, ctx);
        transfer::public_transfer(coin, recipient)
    }
}