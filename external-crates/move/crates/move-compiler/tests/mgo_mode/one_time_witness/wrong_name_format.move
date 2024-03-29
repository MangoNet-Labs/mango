// invalid, wrong one-time witness type name format

module a::mod {
    use mgo::tx_context;

    struct Mod has drop { dummy: bool }

    fun init(_mod: Mod, _ctx: &mut tx_context::TxContext) {
    }
}

module mgo::tx_context {
    struct TxContext has drop {}
}
