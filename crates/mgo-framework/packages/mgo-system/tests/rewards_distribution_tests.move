// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module mgo_system::rewards_distribution_tests {
    use mgo::test_scenario::{Self, Scenario};
    use mgo_system::mgo_system::{Self, MgoSystemState};

    use mgo_system::validator_cap::UnverifiedValidatorOperationCap;
    use mgo_system::governance_test_utils::{
        Self,
        advance_epoch,
        advance_epoch_with_reward_amounts,
        advance_epoch_with_reward_amounts_and_slashing_rates,
        assert_validator_total_stake_amounts,
        assert_validator_non_self_stake_amounts,
        assert_validator_self_stake_amounts,
        create_validator_for_testing,
        create_mgo_system_state_for_testing,
        stake_with,
        total_mgo_balance, unstake
    };
    use mgo::test_utils::assert_eq;
    use std::vector;
    use mgo::address;

    const VALIDATOR_ADDR_1: address = @0x1;
    const VALIDATOR_ADDR_2: address = @0x2;
    const VALIDATOR_ADDR_3: address = @0x3;
    const VALIDATOR_ADDR_4: address = @0x4;

    const STAKER_ADDR_1: address = @0x42;
    const STAKER_ADDR_2: address = @0x43;
    const STAKER_ADDR_3: address = @0x44;
    const STAKER_ADDR_4: address = @0x45;

    const MIST_PER_MGO: u64 = 1_000_000_000;

    #[test]
    fun test_validator_rewards() {
        set_up_mgo_system_state();
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        // need to advance epoch so validator's staking starts counting
        governance_test_utils::advance_epoch(scenario);

        advance_epoch_with_reward_amounts(0, 100, scenario);
        assert_validator_total_stake_amounts(
            validator_addrs(),
            vector[125 * MIST_PER_MGO, 225 * MIST_PER_MGO, 325 * MIST_PER_MGO, 425 * MIST_PER_MGO],
            scenario
        );

        stake_with(VALIDATOR_ADDR_2, VALIDATOR_ADDR_2, 720, scenario);

        advance_epoch(scenario);
        advance_epoch_with_reward_amounts(0, 100, scenario);
        // Even though validator 2 has a lot more stake now, it should not get more rewards because
        // the voting power is capped at 10%.
        assert_validator_total_stake_amounts(
            validator_addrs(),
            vector[150 * MIST_PER_MGO, 970 * MIST_PER_MGO, 350 * MIST_PER_MGO, 450 * MIST_PER_MGO],
            scenario
        );
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_stake_subsidy() {
        set_up_mgo_system_state_with_big_amounts();
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        // need to advance epoch so validator's staking starts counting
        governance_test_utils::advance_epoch(scenario);

        advance_epoch_with_reward_amounts(0, 100, scenario);
        assert_validator_total_stake_amounts(validator_addrs(), vector[100_000_025 * MIST_PER_MGO, 200_000_025 * MIST_PER_MGO, 300_000_025 * MIST_PER_MGO, 400_000_025 * MIST_PER_MGO], scenario);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_stake_rewards() {
        set_up_mgo_system_state();
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 200, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_2, 100, scenario);
        governance_test_utils::advance_epoch(scenario);

        assert_validator_total_stake_amounts(validator_addrs(), vector[300 * MIST_PER_MGO, 300 * MIST_PER_MGO, 300 * MIST_PER_MGO, 400 * MIST_PER_MGO], scenario);
        assert_validator_self_stake_amounts(validator_addrs(), vector[100 * MIST_PER_MGO, 200 * MIST_PER_MGO, 300 * MIST_PER_MGO, 400 * MIST_PER_MGO], scenario);

        // Each pool gets 30 MGO.
        advance_epoch_with_reward_amounts(0, 120, scenario);
        assert_validator_self_stake_amounts(validator_addrs(), vector[110 * MIST_PER_MGO, 220 * MIST_PER_MGO, 330 * MIST_PER_MGO, 430 * MIST_PER_MGO], scenario);
        unstake(STAKER_ADDR_1, 0, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_1, 600, scenario);
        // Each pool gets 30 MGO.
        advance_epoch_with_reward_amounts(0, 120, scenario);
        // staker 1 receives only 20 MGO of rewards, not 40 since we are using pre-epoch exchange rate.
        assert_eq(total_mgo_balance(STAKER_ADDR_1, scenario), 220 * MIST_PER_MGO);
        assert_validator_self_stake_amounts(validator_addrs(), vector[140 * MIST_PER_MGO, 240 * MIST_PER_MGO, 360 * MIST_PER_MGO, 460 * MIST_PER_MGO], scenario);
        unstake(STAKER_ADDR_2, 0, scenario);
        assert_eq(total_mgo_balance(STAKER_ADDR_2, scenario), 120 * MIST_PER_MGO); // 20 MGO of rewards received

        advance_epoch_with_reward_amounts(0, 40, scenario);

        unstake(STAKER_ADDR_2, 0, scenario); // unstake 600 principal MGO
        // additional 600 MGO of principal and 46 MGO of rewards withdrawn to Coin<MGO>
        // For this stake, the staking exchange rate is 100 : 140 and the unstaking
        // exchange rate is 528 : 750 -ish so the total mgo withdraw will be:
        // (600 * 100 / 140) * 750 / 528 = ~608. Together with the 120 MGO we already have,
        // that would be about 728 MGO.
        // TODO: Come up with better numbers and clean it up!
        assert_eq(total_mgo_balance(STAKER_ADDR_2, scenario), 728108108107);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_stake_tiny_rewards() {
        set_up_mgo_system_state_with_big_amounts();
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        // stake a large amount
        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 200000000, scenario);

        governance_test_utils::advance_epoch(scenario);

        advance_epoch_with_reward_amounts(0, 150000, scenario);

        // stake a small amount
        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 10, scenario);
        advance_epoch_with_reward_amounts(0, 130, scenario);

        // unstake the stakes
        unstake(STAKER_ADDR_1, 1, scenario);

        // and advance epoch should succeed
        advance_epoch_with_reward_amounts(0, 150, scenario);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_validator_commission() {
        set_up_mgo_system_state();
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 100, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_2, 100, scenario);
        governance_test_utils::advance_epoch(scenario);
        // V1: 200, V2: 300, V3: 300, V4: 400

        set_commission_rate_and_advance_epoch(VALIDATOR_ADDR_2, 2000, scenario); // 50% commission
        advance_epoch_with_reward_amounts(0, 120, scenario);
        // V1: 230, V2: 330, V3: 330, V4: 430
        // 2 MGO, or 20 % of staker_2's rewards, goes to validator_2
        assert_validator_non_self_stake_amounts(validator_addrs(), vector[115 * MIST_PER_MGO, 108 * MIST_PER_MGO, 0, 0], scenario);
        assert_validator_self_stake_amounts(validator_addrs(), vector[115 * MIST_PER_MGO, 222 * MIST_PER_MGO, 330 * MIST_PER_MGO, 430 * MIST_PER_MGO], scenario);

        set_commission_rate_and_advance_epoch(VALIDATOR_ADDR_1, 1000, scenario); // 10% commission

        advance_epoch_with_reward_amounts(0, 240, scenario);
        assert_validator_total_stake_amounts(validator_addrs(), vector[290 * MIST_PER_MGO, 390 * MIST_PER_MGO, 390 * MIST_PER_MGO, 490 * MIST_PER_MGO], scenario);

        // Staker 1 rewards in the recent distribution is 0.9 x 30 = 27 MGO
        // Validator 1 rewards in the recent distribution is 60 - 27 = 33 MGO

        // Staker 2 amounts for 0.8 * 60 * (108 / 330) + 108 = 123.709 MGO
        // Validator 2 amounts for 390 - 123.709 = 266.291 MGO
        assert_validator_non_self_stake_amounts(validator_addrs(), vector[142 * MIST_PER_MGO, 123709090909, 0, 0], scenario);
        assert_validator_self_stake_amounts(validator_addrs(), vector[148 * MIST_PER_MGO, 266290909091, 390 * MIST_PER_MGO, 490 * MIST_PER_MGO], scenario);

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_rewards_slashing() {
        set_up_mgo_system_state();
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        advance_epoch(scenario);

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 100, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_2, 100, scenario);

        advance_epoch(scenario);

        // validator_2 is reported by 3 other validators, so 75% of total stake.
        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_4, VALIDATOR_ADDR_2, scenario);

        // validator_1 is reported by only 1 other validator, which is 25% of total stake.
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_1, scenario);

        // 3600 MGO of total rewards, 50% threshold and 10% reward slashing.
        // So validator_2 is the only one whose rewards should get slashed.
        advance_epoch_with_reward_amounts_and_slashing_rates(
            0, 3600, 1000, scenario
        );

        // Without reward slashing, the validator's stakes should be [100+450, 200+600, 300+900, 400+900]
        // after the last epoch advancement.
        // Since 60 MGO, or 10% of validator_2's rewards (600) are slashed, she only has 800 - 60 = 740 now.
        // There are in total 90 MGO of rewards slashed (60 from the validator, and 30 from her staker)
        // so the unslashed validators each get their share of additional rewards, which is 30.
        assert_validator_self_stake_amounts(validator_addrs(), vector[565 * MIST_PER_MGO, 740 * MIST_PER_MGO, 1230 * MIST_PER_MGO, 1330 * MIST_PER_MGO], scenario);

        // Unstake so we can check the stake rewards as well.
        unstake(STAKER_ADDR_1, 0, scenario);
        unstake(STAKER_ADDR_2, 0, scenario);

        // Same analysis as above. Delegator 1 has 3 additional MGO, and 10% of staker 2's rewards are slashed.
        assert!(total_mgo_balance(STAKER_ADDR_1, scenario) == 565 * MIST_PER_MGO, 0);
        assert!(total_mgo_balance(STAKER_ADDR_2, scenario) == 370 * MIST_PER_MGO, 0);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_entire_rewards_slashing() {
        set_up_mgo_system_state();
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        advance_epoch(scenario);

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 100, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_2, 100, scenario);

        advance_epoch(scenario);

        // validator_2 is reported by 3 other validators, so 75% of total stake.
        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_4, VALIDATOR_ADDR_2, scenario);


        // 3600 MGO of total rewards, 100% reward slashing.
        // So validator_2 is the only one whose rewards should get slashed.
        advance_epoch_with_reward_amounts_and_slashing_rates(
            0, 3600, 10_000, scenario
        );

        // Without reward slashing, the validator's stakes should be [100+450, 200+600, 300+900, 400+900]
        // after the last epoch advancement.
        // The entire rewards of validator 2's staking pool are slashed, which is 900 MGO.
        // so the unslashed validators each get their share of additional rewards, which is 300.
        assert_validator_self_stake_amounts(validator_addrs(), vector[(550 + 150) * MIST_PER_MGO, 200 * MIST_PER_MGO, (1200 + 300) * MIST_PER_MGO, (1300 + 300) * MIST_PER_MGO], scenario);

        // Unstake so we can check the stake rewards as well.
        unstake(STAKER_ADDR_1, 0, scenario);
        unstake(STAKER_ADDR_2, 0, scenario);

        // Same analysis as above. Staker 1 has 150 additional MGO, and since all of staker 2's rewards are slashed she only gets back her principal.
        assert!(total_mgo_balance(STAKER_ADDR_1, scenario) == (550 + 150) * MIST_PER_MGO, 0);
        assert!(total_mgo_balance(STAKER_ADDR_2, scenario) == 100 * MIST_PER_MGO, 0);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_rewards_slashing_with_storage_fund() {
        set_up_mgo_system_state();
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        // Put 300 MGO into the storage fund.
        advance_epoch_with_reward_amounts(300, 0, scenario);

        // Add a few stakes.
        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_3, 100, scenario);
        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_4, 100, scenario);
        advance_epoch(scenario);

        // validator_4 is reported by 3 other validators, so 75% of total stake.
        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_4, scenario);
        report_validator(VALIDATOR_ADDR_2, VALIDATOR_ADDR_4, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_4, scenario);

        // 1000 MGO of storage rewards, 1500 MGO of computation rewards, 50% slashing threshold
        // and 20% slashing rate
        advance_epoch_with_reward_amounts_and_slashing_rates(
            1000, 1500, 2000, scenario
        );

        // Each unslashed validator staking pool gets 300 MGO of computation rewards + 75 MGO of storage fund rewards +
        // 20 MGO (1/3) of validator 4's slashed computation reward and 5 MGO (1/3) of validator 4's slashed
        // storage fund reward, so in total it gets 400 MGO of rewards.
        // Validator 3 has a delegator with her so she gets 320 * 3/4 + 75 + 5 = 320 MGO of rewards.
        // Validator 4's should get 300 * 4/5 * (1 - 20%) = 192 in computation rewards and 75 * (1 - 20%) = 60 in storage rewards.
        assert_validator_self_stake_amounts(validator_addrs(), vector[500 * MIST_PER_MGO, 600 * MIST_PER_MGO, 620 * MIST_PER_MGO, 652 * MIST_PER_MGO], scenario);

        // Unstake so we can check the stake rewards as well.
        unstake(STAKER_ADDR_1, 0, scenario);
        unstake(STAKER_ADDR_2, 0, scenario);

        // Staker 1 gets 320 * 1/4 = 80 MGO of rewards.
        assert_eq(total_mgo_balance(STAKER_ADDR_1, scenario), (100 + 80) * MIST_PER_MGO);
        // Staker 2 gets 300 * 1/5 * (1 - 20%) = 48 MGO of rewards.
        assert_eq(total_mgo_balance(STAKER_ADDR_2, scenario), (100 + 48) * MIST_PER_MGO);

        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_everyone_slashed() {
        // This test is to make sure that if everyone is slashed, our protocol works as expected without aborting
        // and all rewards go to the storage fund.
        set_up_mgo_system_state();
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_4, scenario);
        report_validator(VALIDATOR_ADDR_2, VALIDATOR_ADDR_4, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_4, scenario);
        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_3, scenario);
        report_validator(VALIDATOR_ADDR_2, VALIDATOR_ADDR_3, scenario);
        report_validator(VALIDATOR_ADDR_4, VALIDATOR_ADDR_3, scenario);
        report_validator(VALIDATOR_ADDR_1, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_4, VALIDATOR_ADDR_2, scenario);
        report_validator(VALIDATOR_ADDR_2, VALIDATOR_ADDR_1, scenario);
        report_validator(VALIDATOR_ADDR_3, VALIDATOR_ADDR_1, scenario);
        report_validator(VALIDATOR_ADDR_4, VALIDATOR_ADDR_1, scenario);

        advance_epoch_with_reward_amounts_and_slashing_rates(
            1000, 3000, 10_000, scenario
        );

        // All validators should have 0 rewards added so their stake stays the same.
        assert_validator_self_stake_amounts(validator_addrs(), vector[100 * MIST_PER_MGO, 200 * MIST_PER_MGO, 300 * MIST_PER_MGO, 400 * MIST_PER_MGO], scenario);

        test_scenario::next_tx(scenario, @0x0);
        // Storage fund balance should increase by 4000 MGO.
        let system_state = test_scenario::take_shared<MgoSystemState>(scenario);
        assert_eq(mgo_system::get_storage_fund_total_balance(&mut system_state), 4000 * MIST_PER_MGO);

        // The entire 1000 MGO of storage rewards should go to the object rebate portion of the storage fund.
        assert_eq(mgo_system::get_storage_fund_object_rebates(&mut system_state), 1000 * MIST_PER_MGO);

        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_mul_rewards_withdraws_at_same_epoch() {
        set_up_mgo_system_state();
        let scenario_val = test_scenario::begin(VALIDATOR_ADDR_1);
        let scenario = &mut scenario_val;

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 220, scenario);

        advance_epoch_with_reward_amounts(0, 40, scenario);

        stake_with(STAKER_ADDR_2, VALIDATOR_ADDR_1, 480, scenario);

        // Staker 1 gets 2/3 * 1/4 * 120 = 20 MGO here.
        advance_epoch_with_reward_amounts(0, 120, scenario);

        stake_with(STAKER_ADDR_1, VALIDATOR_ADDR_1, 130, scenario);
        stake_with(STAKER_ADDR_3, VALIDATOR_ADDR_1, 390, scenario);

        // Staker 1 gets 20 MGO here and staker 2 gets 40 MGO here.
        advance_epoch_with_reward_amounts(0, 280, scenario);
        stake_with(STAKER_ADDR_3, VALIDATOR_ADDR_1, 280, scenario);
        stake_with(STAKER_ADDR_4, VALIDATOR_ADDR_1, 1400, scenario);

        // Staker 1 gets 30 MGO, staker 2 gets 40 MGO and staker 3 gets 30 MGO.
        advance_epoch_with_reward_amounts(0, 440, scenario);

        test_scenario::next_tx(scenario, @0x0);
        let system_state = test_scenario::take_shared<MgoSystemState>(scenario);
        // Check that we have the right amount of MGO in the staking pool.
        assert_eq(mgo_system::validator_stake_amount(&mut system_state, VALIDATOR_ADDR_1), 140 * 23 * MIST_PER_MGO);
        test_scenario::return_shared(system_state);

        // Withdraw all stakes at once.
        unstake(STAKER_ADDR_1, 0, scenario);
        unstake(STAKER_ADDR_1, 0, scenario);
        unstake(STAKER_ADDR_2, 0, scenario);
        unstake(STAKER_ADDR_3, 0, scenario);
        unstake(STAKER_ADDR_3, 0, scenario);
        unstake(STAKER_ADDR_4, 0, scenario);

        // staker 1's first stake was active for 3 epochs so got 20 * 3 = 60 MGO of rewards
        // and her second stake was active for only one epoch and got 10 MGO of rewards.
        assert_eq(total_mgo_balance(STAKER_ADDR_1, scenario), (220 + 130 + 20 * 3 + 10) * MIST_PER_MGO);
        // staker 2's stake was active for 2 epochs so got 40 * 2 = 80 MGO of rewards
        assert_eq(total_mgo_balance(STAKER_ADDR_2, scenario), (480 + 40 * 2) * MIST_PER_MGO);
        // staker 3's first stake was active for 1 epoch and got 30 MGO of rewards
        // and her second stake didn't get any rewards.
        assert_eq(total_mgo_balance(STAKER_ADDR_3, scenario), (390 + 280 + 30) * MIST_PER_MGO);
        // staker 4 joined and left in an epoch where no rewards were earned so she got no rewards.
        assert_eq(total_mgo_balance(STAKER_ADDR_4, scenario), 1400 * MIST_PER_MGO);

        advance_epoch_with_reward_amounts(0, 0, scenario);

        test_scenario::next_tx(scenario, @0x0);
        let system_state = test_scenario::take_shared<MgoSystemState>(scenario);
        // Since all the stakes are gone the pool is empty except for the validator's original stake.
        assert_eq(mgo_system::validator_stake_amount(&mut system_state, VALIDATOR_ADDR_1), 140 * MIST_PER_MGO);
        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }

    #[test]
    fun test_uncapped_rewards() {
        let scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;

        let ctx = test_scenario::ctx(scenario);
        let validators = vector[];

        let num_validators = 20;
        let i = 0;
        // Create a set of 20 validators, each with 481 + i * 2 MGO of stake.
        // The stake total sums up to be 481 + 483 + ... + 517 + 519 = 1000 MGO.
        while (i < num_validators) {
            let validator = create_validator_for_testing(address::from_u256((i as u256)), (481 + i * 2), ctx);
            vector::push_back(&mut validators, validator);
            i = i + 1;
        };

        create_mgo_system_state_for_testing(validators, 0, 0, ctx);
        // Each validator's stake gets doubled.
        advance_epoch_with_reward_amounts(0, 10000, scenario);

        let i = 0;
        test_scenario::next_tx(scenario, @0x0);
        // Check that each validator has the correct amount of MGO in their stake pool.
        let system_state = test_scenario::take_shared<MgoSystemState>(scenario);
        while (i < num_validators) {
            let addr = address::from_u256((i as u256));
            assert_eq(mgo_system::validator_stake_amount(&mut system_state, addr), (962 + i * 4) * MIST_PER_MGO);
            i = i + 1;
        };
        test_scenario::return_shared(system_state);
        test_scenario::end(scenario_val);
    }

    fun set_up_mgo_system_state() {
        let scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        let validators = vector[
            create_validator_for_testing(VALIDATOR_ADDR_1, 100, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_2, 200, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_3, 300, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_4, 400, ctx),
        ];
        create_mgo_system_state_for_testing(validators, 1000, 0, ctx);
        test_scenario::end(scenario_val);
    }

    fun set_up_mgo_system_state_with_big_amounts() {
        let scenario_val = test_scenario::begin(@0x0);
        let scenario = &mut scenario_val;
        let ctx = test_scenario::ctx(scenario);

        let validators = vector[
            create_validator_for_testing(VALIDATOR_ADDR_1, 100000000, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_2, 200000000, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_3, 300000000, ctx),
            create_validator_for_testing(VALIDATOR_ADDR_4, 400000000, ctx),
        ];
        create_mgo_system_state_for_testing(validators, 1000000000, 0, ctx);
        test_scenario::end(scenario_val);
    }

    fun validator_addrs() : vector<address> {
        vector[VALIDATOR_ADDR_1, VALIDATOR_ADDR_2, VALIDATOR_ADDR_3, VALIDATOR_ADDR_4]
    }

    fun set_commission_rate_and_advance_epoch(addr: address, commission_rate: u64, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, addr);
        let system_state = test_scenario::take_shared<MgoSystemState>(scenario);
        let ctx = test_scenario::ctx(scenario);
        mgo_system::request_set_commission_rate(&mut system_state, commission_rate, ctx);
        test_scenario::return_shared(system_state);
        governance_test_utils::advance_epoch(scenario);
    }

    fun report_validator(reporter: address, reportee: address, scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, reporter);
        let system_state = test_scenario::take_shared<MgoSystemState>(scenario);
        let cap = test_scenario::take_from_sender<UnverifiedValidatorOperationCap>(scenario);
        mgo_system::report_validator(&mut system_state, &cap, reportee);
        test_scenario::return_to_sender(scenario, cap);
        test_scenario::return_shared(system_state);
    }
}
