// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module mgo::id_tests {
    use mgo::object;
    use mgo::tx_context;

    const EIdBytesMismatch: u64 = 0;

    struct Object has key {
        id: object::UID,
    }

    #[test]
    fun test_get_id() {
        let ctx = tx_context::dummy();
        let id = object::new(&mut ctx);
        let obj_id = object::uid_to_inner(&id);
        let obj = Object { id };
        assert!(object::id(&obj) == obj_id, EIdBytesMismatch);
        let Object { id } = obj;
        object::delete(id);
    }
}
