// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

/// Module providing debug functionality.
module std::debug {
    native public fun print<T>(x: &T);

    native public fun print_stack_trace();
}
