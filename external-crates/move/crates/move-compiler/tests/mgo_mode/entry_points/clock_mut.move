// invalid, Clock by mutable reference

module a::m {
    public entry fun no_clock_mut(_: &mut mgo::clock::Clock) {
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
