// invalid, struct type has type param

//# publish
module a::m {
    use mgo::tx_context;

    struct M<phantom T> has drop { dummy: bool }

    fun init<T>(_otw: M<T>, _ctx: &mut tx_context::TxContext) {
    }
}

module a::x {
    use mgo::tx_context;

    struct X<phantom T> has drop { dummy: bool }

    fun init(_ctx: &mut tx_context::TxContext) {
    }
}

module mgo::tx_context {
    struct TxContext has drop {}
}
