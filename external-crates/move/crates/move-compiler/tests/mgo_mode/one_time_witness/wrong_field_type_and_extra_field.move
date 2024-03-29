module a::beep {
    struct BEEP has drop {
        f0: u64,
        f1: bool,
    }
    fun init(_ctx: &mut mgo::tx_context::TxContext) {
    }
}

module mgo::tx_context {
    struct TxContext has drop {}
}
