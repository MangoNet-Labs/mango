// invalid, wrong type of the init function's first param

module a::m {
    use mgo::tx_context;

    struct M has drop { dummy: bool }
    struct N has drop { dummy: bool }

    fun init(_otw: N, _ctx: &mut tx_context::TxContext) {
    }
}

module mgo::tx_context {
    struct TxContext has drop {}
}
