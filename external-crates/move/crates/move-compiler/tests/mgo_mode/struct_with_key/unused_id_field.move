module a::m {
    struct Obj has key { id: mgo::object::UID }
}

module mgo::object {
    struct UID has store { value: address }
    public fun borrow_address(id: &UID): &address { &id.value }
}
