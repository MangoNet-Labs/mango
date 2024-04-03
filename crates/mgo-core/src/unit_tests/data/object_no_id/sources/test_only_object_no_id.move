// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

module object_no_id::test_only_object_no_id {
    #[test_only]
    struct NotObject has key {f: u64}

    #[test]
    fun bad_share() {
        mgo::transfer::share_object(NotObject{f: 42});
    }
}
