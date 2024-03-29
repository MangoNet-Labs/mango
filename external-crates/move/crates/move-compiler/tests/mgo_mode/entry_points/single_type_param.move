module a::m {
    use mgo::tx_context;

    public entry fun foo<T>(_: T, _: &mut tx_context::TxContext) {
        abort 0
    }

}

module mgo::tx_context {
    struct TxContext has drop {}
}
