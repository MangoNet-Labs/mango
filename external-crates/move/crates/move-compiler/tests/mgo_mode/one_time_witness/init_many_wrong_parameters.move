module a::m {
    fun init(_: who::X, _: who::Y, _: who::Z) {}
}

module a::beep {
    struct BEEP has drop {}
    fun init(_: Who, _: u64, _: &mut mgo::tx_context::TxContext) {}
}

module mgo::tx_context {
    struct TxContext {}
}
