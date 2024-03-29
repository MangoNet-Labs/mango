// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

module mgo_system::mgo_system_state_inner {
    use mgo::balance::{Self, Balance};
    use mgo::mgo::MGO;
    use mgo::tx_context::TxContext;
    use mgo::bag::{Self, Bag};
    use mgo::table::{Self, Table};
    use mgo::object::ID;

    use mgo_system::validator::Validator;
    use mgo_system::validator_wrapper::ValidatorWrapper;

    friend mgo_system::mgo_system;

    const SYSTEM_STATE_VERSION_V1: u64 = 18446744073709551605;  // u64::MAX - 10
    const SYSTEM_STATE_VERSION_V2: u64 = 18446744073709551606;  // u64::MAX - 9

    struct SystemParameters has store {
        epoch_duration_ms: u64,
        extra_fields: Bag,
    }

    struct ValidatorSet has store {
        active_validators: vector<Validator>,
        inactive_validators: Table<ID, ValidatorWrapper>,
        extra_fields: Bag,
    }

    struct MgoSystemStateInner has store {
        epoch: u64,
        protocol_version: u64,
        system_state_version: u64,
        validators: ValidatorSet,
        storage_fund: Balance<MGO>,
        parameters: SystemParameters,
        reference_gas_price: u64,
        safe_mode: bool,
        epoch_start_timestamp_ms: u64,
        extra_fields: Bag,
    }

    struct MgoSystemStateInnerV2 has store {
        new_dummy_field: u64,
        epoch: u64,
        protocol_version: u64,
        system_state_version: u64,
        validators: ValidatorSet,
        storage_fund: Balance<MGO>,
        parameters: SystemParameters,
        reference_gas_price: u64,
        safe_mode: bool,
        epoch_start_timestamp_ms: u64,
        extra_fields: Bag,
    }

    public(friend) fun create(
        validators: vector<Validator>,
        storage_fund: Balance<MGO>,
        protocol_version: u64,
        epoch_start_timestamp_ms: u64,
        epoch_duration_ms: u64,
        ctx: &mut TxContext,
    ): MgoSystemStateInner {
        let validators = new_validator_set(validators, ctx);
        let system_state = MgoSystemStateInner {
            epoch: 0,
            protocol_version,
            system_state_version: genesis_system_state_version(),
            validators,
            storage_fund,
            parameters: SystemParameters {
                epoch_duration_ms,
                extra_fields: bag::new(ctx),
            },
            reference_gas_price: 1,
            safe_mode: false,
            epoch_start_timestamp_ms,
            extra_fields: bag::new(ctx),
        };
        system_state
    }

    public(friend) fun advance_epoch(
        self: &mut MgoSystemStateInnerV2,
        new_epoch: u64,
        next_protocol_version: u64,
        storage_reward: Balance<MGO>,
        computation_reward: Balance<MGO>,
        storage_rebate_amount: u64,
        epoch_start_timestamp_ms: u64,
    ) : Balance<MGO> {
        self.epoch_start_timestamp_ms = epoch_start_timestamp_ms;
        self.epoch = self.epoch + 1;
        assert!(new_epoch == self.epoch, 0);
        self.safe_mode = false;
        self.protocol_version = next_protocol_version;

        balance::join(&mut self.storage_fund, computation_reward);
        balance::join(&mut self.storage_fund, storage_reward);
        let storage_rebate = balance::split(&mut self.storage_fund, storage_rebate_amount);
        storage_rebate
    }

    public(friend) fun protocol_version(self: &MgoSystemStateInnerV2): u64 { self.protocol_version }
    public(friend) fun system_state_version(self: &MgoSystemStateInnerV2): u64 { self.system_state_version }
    public(friend) fun genesis_system_state_version(): u64 {
        SYSTEM_STATE_VERSION_V1
    }

    fun new_validator_set(init_active_validators: vector<Validator>, ctx: &mut TxContext): ValidatorSet {
        ValidatorSet {
            active_validators: init_active_validators,
            inactive_validators: table::new(ctx),
            extra_fields: bag::new(ctx),
        }
    }

    public(friend) fun v1_to_v2(v1: MgoSystemStateInner): MgoSystemStateInnerV2 {
        let MgoSystemStateInner {
            epoch,
            protocol_version,
            system_state_version: old_system_state_version,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            safe_mode,
            epoch_start_timestamp_ms,
            extra_fields,
        } = v1;
        assert!(old_system_state_version == SYSTEM_STATE_VERSION_V1, 0);
        MgoSystemStateInnerV2 {
            new_dummy_field: 100,
            epoch,
            protocol_version,
            system_state_version: SYSTEM_STATE_VERSION_V2,
            validators,
            storage_fund,
            parameters,
            reference_gas_price,
            safe_mode,
            epoch_start_timestamp_ms,
            extra_fields,
        }
    }
}
