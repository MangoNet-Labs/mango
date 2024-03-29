// valid, Clock by immutable reference

module a::m {
    public entry fun yes_clock_ref(_: &mgo::clock::Clock) {
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
