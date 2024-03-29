// init is unused but does not error because we are in Mgo mode
module a::m {
    fun init(_: &mut mgo::tx_context::TxContext) {}
}

module mgo::tx_context {
    struct TxContext has drop {}
}
