// valid
module a::m {
    use mgo::object;
    struct S has key {
        id: object::UID
    }
}

module mgo::object {
    struct UID has store {
        id: address,
    }
}
