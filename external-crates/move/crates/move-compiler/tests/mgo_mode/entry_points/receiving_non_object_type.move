// invalid, Receiving type with non-object type param

module a::m {
    use mgo::transfer::Receiving;

    public entry fun no(_: Receiving<u64>) { abort 0 }
}

module mgo::transfer {
    struct Receiving<phantom T: key> has drop {
        id: address
    }
}
