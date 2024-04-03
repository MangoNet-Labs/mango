// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

#[allow(unused_field)]
module mgo::object {
    /// A test version of the UID type to allow us to have types with
    /// `key` in these test packages. It has a different structure to
    /// the real UID, but that is not relevant.
    struct UID has store {
        id: address,
    }
}
