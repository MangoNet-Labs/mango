// invalid, object cannot have drop since UID does not have drop

module a::m {
    use mgo::object;
    struct S has key, drop {
        id: object::UID,
        flag: bool
    }
}

module mgo::object {
    struct UID has store {
        id: address,
    }
}
