// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

/// Provides a way to get address length since it's a
/// platform-specific parameter.
module std::address {
    /// Should be converted to a native function.
    /// Current implementation only works for Mgo.
    public fun length(): u64 {
        32
    }
}
