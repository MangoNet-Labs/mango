// invalid, a reference to vector of objects

module a::m {
    use mgo::object;

    struct S has key { id: object::UID }

    public entry fun no(_: &vector<S>) {
        abort 0
    }

}

module mgo::object {
    struct UID has store {
        id: address,
    }
}
