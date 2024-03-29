// invalid, wrong one-time witness type name

module a::m {
    use mgo::tx_context;

    struct OneTimeWitness has drop { dummy: bool }

    fun init(_otw: OneTimeWitness, _ctx: &mut tx_context::TxContext) {
    }
}

module mgo::tx_context {
    struct TxContext has drop {}
}
