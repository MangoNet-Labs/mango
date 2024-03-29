// invalid, Clock by value

module a::m {
    public entry fun no_clock_val(_: mgo::clock::Clock) {
        abort 0
    }
}

module mgo::clock {
    struct Clock has key {
        id: mgo::object::UID,
    }
}

module mgo::object {
    struct UID has store {
        id: address,
    }
}
