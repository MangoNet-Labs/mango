// invalid, one-time witness type candidate used in a different module

module a::n {
    use mgo::mgo;
    use mgo::tx_context;

    fun init(_otw: mgo::MGO, _ctx: &mut tx_context::TxContext) {
    }

}


module mgo::tx_context {
    struct TxContext has drop {}
}

module mgo::mgo {
    struct MGO has drop {}
}
