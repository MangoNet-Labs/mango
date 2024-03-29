// invalid, otw type packed

module a::m {
    use mgo::tx_context;

    struct M has drop { dummy: bool }

    fun init(_otw: M, _ctx: &mut tx_context::TxContext) {
    }

    fun pack(): M {
        M { dummy: false }
    }
}

module mgo::tx_context {
    struct TxContext has drop {}
}
