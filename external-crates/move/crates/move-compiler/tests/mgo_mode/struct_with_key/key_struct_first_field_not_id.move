// invalid, first field of an ojbect must be mgo::object::UID
module a::m {
    struct S has key {
        flag: bool
    }
}
