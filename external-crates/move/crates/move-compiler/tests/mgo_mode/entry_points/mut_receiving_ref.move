// valid, Receiving type by mut ref with object type param

module a::m {
    use mgo::object;
    use mgo::transfer::Receiving;

    struct S has key { id: object::UID }

    public entry fun yes(_: &mut Receiving<S>) { }
}

module mgo::object {
    struct UID has store {
        id: address,
    }
}

module mgo::transfer {
    struct Receiving<phantom T: key> has drop {
        id: address
    }
}
