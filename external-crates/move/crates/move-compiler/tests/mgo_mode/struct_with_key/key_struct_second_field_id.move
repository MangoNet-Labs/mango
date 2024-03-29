// invalid, object must have UID as first field not some other field

module a::m {
    use mgo::object;
    struct S has key {
        flag: bool,
        id: object::UID,
    }

    struct R has key {
        flag: bool,
        id: address,
    }
}

module mgo::object {
    struct UID has store {
        id: address,
    }
}
