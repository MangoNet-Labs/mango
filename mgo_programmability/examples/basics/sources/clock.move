// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/// This example demonstrates reading a clock object.
/// Current time is emitted as an event in the get_time transaction
module basics::clock {
    use mgo::clock::{Self, Clock};
    use mgo::event;
    use mgo::tx_context::TxContext;

    struct TimeEvent has copy, drop {
        timestamp_ms: u64,
    }

    /// Emit event with current time.
    public entry fun get_time(clock: &Clock, _ctx: &mut TxContext) {
        event::emit(TimeEvent { timestamp_ms: clock::timestamp_ms(clock) });
    }
}
