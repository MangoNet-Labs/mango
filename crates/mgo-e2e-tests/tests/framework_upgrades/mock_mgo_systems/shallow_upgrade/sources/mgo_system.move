// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

module mgo_system::mgo_system {
    use mgo::balance::Balance;
    use mgo::object::UID;
    use mgo::mgo::MGO;
    use mgo::transfer;
    use mgo::tx_context::{Self, TxContext};
    use mgo::dynamic_field;

    use mgo_system::validator::Validator;
    use mgo_system::mgo_system_state_inner::{Self, MgoSystemStateInner, MgoSystemStateInnerV2};

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
        new_epoch: u64,
        next_protocol_version: u64,
        storage_rebate: u64,
        _non_refundable_storage_fee: u64,
        _storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
                                         // into storage fund, in basis point.
        _reward_slashing_rate: u64, // how much rewards are slashed to punish a validator, in bps.
        epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
        ctx: &mut TxContext,
    ) : Balance<MGO> {
        let self = load_system_state_mut(wrapper);
        assert!(tx_context::sender(ctx) == @0x0, 0);
        let storage_rebate = mgo_system_state_inner::advance_epoch(
            self,
            new_epoch,
            next_protocol_version,
            storage_reward,
            computation_reward,
            storage_rebate,
            epoch_start_timestamp_ms,
        );

        storage_rebate
    }

    fun load_system_state_mut(self: &mut MgoSystemState): &mut MgoSystemStateInnerV2 {
        load_inner_maybe_upgrade(self)
    }

    fun load_inner_maybe_upgrade(self: &mut MgoSystemState): &mut MgoSystemStateInnerV2 {
        let version = self.version;
        if (version == mgo_system_state_inner::genesis_system_state_version()) {
            let inner: MgoSystemStateInner = dynamic_field::remove(&mut self.id, version);
            let new_inner = mgo_system_state_inner::v1_to_v2(inner);
            version = mgo_system_state_inner::system_state_version(&new_inner);
            dynamic_field::add(&mut self.id, version, new_inner);
            self.version = version;
        };

        let inner: &mut MgoSystemStateInnerV2 = dynamic_field::borrow_mut(&mut self.id, version);
        assert!(mgo_system_state_inner::system_state_version(inner) == version, 0);
        inner
    }
}
