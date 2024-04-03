// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

/// Mgo types helpers and utilities
module mgo::types {
    // === one-time witness ===

    /// Tests if the argument type is a one-time witness, that is a type with only one instantiation
    /// across the entire code base.
    public native fun is_one_time_witness<T: drop>(_: &T): bool;
}
