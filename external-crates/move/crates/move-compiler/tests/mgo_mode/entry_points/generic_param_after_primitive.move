module a::m {
    use mgo::object;
    struct Obj has key {
        id: object::UID,
    }
    public entry fun foo<T>(_: Obj, _: u64, _: T) {
        abort 0
    }

}

module mgo::object {
    struct UID has store {
        id: address,
    }
}
