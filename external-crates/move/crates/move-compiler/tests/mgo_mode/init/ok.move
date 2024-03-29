// valid init function
module a::m {
    use mgo::tx_context;
    fun init(_: &mut tx_context::TxContext) {
    }
}

module mgo::tx_context {
    struct TxContext has drop {}
}
