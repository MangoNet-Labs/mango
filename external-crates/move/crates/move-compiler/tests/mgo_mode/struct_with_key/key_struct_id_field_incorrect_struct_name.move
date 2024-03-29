// invalid, objects need UID not ID
module a::m {
    use mgo::object;
    struct S has key {
        id: object::ID
    }
}

module mgo::object {
    struct ID has store {
        id: address,
    }
}
