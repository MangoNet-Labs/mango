// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module mgo_system::mgo_system {
    use mgo::balance::Balance;
    use mgo::object::UID;
    use mgo::mgo::MGO;
    use mgo::transfer;
    use mgo::tx_context::{Self, TxContext};
    use mgo::dynamic_field;

    use mgo_system::validator::Validator;
    use mgo_system::mgo_system_state_inner::MgoSystemStateInner;
    use mgo_system::mgo_system_state_inner;

    friend mgo_system::genesis;

    struct MgoSystemState has key {
        id: UID,
        version: u64,
    }

    public(friend) fun create(
        id: UID,
        validators: vector<Validator>,
        storage_fund: Balance<MGO>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ) {
        let system_state = mgo_system_state_inner::create(
            validators,
            storage_fund,
            protocol_version,
            epoch_start_timestamp_ms,
            epoch_duration_ms,
            ctx,
        );
        let version = mgo_system_state_inner::genesis_system_state_version();
        let self = MgoSystemState {
            id,
            version,
        };
        dynamic_field::add(&mut self.id, version, system_state);
        transfer::share_object(self);
    }

    fun advance_epoch(
        storage_reward: Balance<MGO>,
        computation_reward: Balance<MGO>,
        wrapper: &mut MgoSystemState,
        _new_epoch: u64,
        _next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64,
        _reward_slashing_rate: u64,
        _epoch_start_timestamp_ms: u64,
        ctx: &mut TxContext,
    ) : Balance<MGO> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x1, 0); // aborts here
        mgo_system_state_inner::advance_epoch(
            self,
            storage_reward,
            computation_reward,
            storage_rebate,
        )
    }

    fun load_system_state_mut(self: &mut MgoSystemState): &mut MgoSystemStateInner {
        let version = self.version;
        dynamic_field::borrow_mut(&mut self.id, version)
    }
}
