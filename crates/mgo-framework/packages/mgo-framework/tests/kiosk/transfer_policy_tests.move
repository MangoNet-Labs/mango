// Copyright (c) MangoNet Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module mgo::malicious_policy {
    use mgo::transfer_policy::{Self as policy, TransferRequest};

    struct Rule has drop {}

    public fun cheat<T>(request: &mut TransferRequest<T>) {
        policy::add_receipt(Rule {}, request);
    }
}

#[test_only]
module mgo::transfer_policy_tests {
    use mgo::transfer_policy::{Self as policy, TransferPolicy, TransferPolicyCap};
    use mgo::tx_context::{Self, TxContext, dummy as ctx};
    use mgo::object::{Self, ID, UID};
    use mgo::dummy_policy;
    use mgo::malicious_policy;
    use mgo::vec_set;
    use mgo::package;
    use mgo::coin;

    struct OTW has drop {}
    struct Asset has key, store { id: UID }

    #[test]
    /// No policy set;
    fun test_default_flow() {
        let ctx = &mut ctx();
        let (policy, cap) = prepare(ctx);

        // time to make a new transfer request
        let request = policy::new_request(fresh_id(ctx), 10_000, fresh_id(ctx));
        policy::confirm_request(&policy, request);

        wrapup(policy, cap, ctx);
    }

    #[test]
    /// Policy set and completed;
    fun test_rule_completed() {
        let ctx = &mut ctx();
        let (policy, cap) = prepare(ctx);

        assert!(vec_set::size(policy::rules(&policy)) == 0, 0);
        // now require everyone to pay any amount
        dummy_policy::set(&mut policy, &cap);

        assert!(vec_set::size(policy::rules(&policy)) == 1, 1);

        let request = policy::new_request(fresh_id(ctx), 10_000, fresh_id(ctx));

        dummy_policy::pay(&mut policy, &mut request, coin::mint_for_testing(10_000, ctx));
        policy::confirm_request(&policy, request);

        let profits = wrapup(policy, cap, ctx);

        assert!(profits == 10_000, 0);
    }

    #[test]
    /// Policy set and completed; rule removed; empty policy works
    fun test_remove_rule_completed() {
        let ctx = &mut ctx();
        let (policy, cap) = prepare(ctx);

        assert!(vec_set::size(policy::rules(&policy)) == 0, 0);

        // now require everyone to pay any amount
        dummy_policy::set(&mut policy, &cap);

        assert!(vec_set::size(policy::rules(&policy)) == 1, 0);

        let request = policy::new_request(fresh_id(ctx), 10_000, fresh_id(ctx));
        dummy_policy::pay(&mut policy, &mut request, coin::mint_for_testing(10_000, ctx));
        policy::confirm_request(&policy, request);

        // remove policy and start over - this time ignore dummy_policy
        policy::remove_rule<Asset, dummy_policy::Rule, dummy_policy::Config>(&mut policy, &cap);
        let request = policy::new_request(fresh_id(ctx), 10_000, fresh_id(ctx));
        policy::confirm_request(&policy, request);

        assert!(vec_set::size(policy::rules(&policy)) == 0, 0);
        assert!(wrapup(policy, cap, ctx) == 10_000, 0);
    }

    #[test]
    #[expected_failure(abort_code = mgo::transfer_policy::EPolicyNotSatisfied)]
    /// Policy set but not satisfied;
    fun test_rule_ignored() {
        let ctx = &mut ctx();
        let (policy, cap) = prepare(ctx);

        // now require everyone to pay any amount
        dummy_policy::set(&mut policy, &cap);

        let request = policy::new_request(fresh_id(ctx), 10_000, fresh_id(ctx));
        policy::confirm_request(&policy, request);

        wrapup(policy, cap, ctx);
    }

    #[test]
    #[expected_failure(abort_code = mgo::transfer_policy::ERuleAlreadySet)]
    /// Attempt to add another policy;
    fun test_rule_exists() {
        let ctx = &mut ctx();
        let (policy, cap) = prepare(ctx);

        // now require everyone to pay any amount
        dummy_policy::set(&mut policy, &cap);
        dummy_policy::set(&mut policy, &cap);

        let request = policy::new_request(fresh_id(ctx), 10_000, fresh_id(ctx));
        policy::confirm_request(&policy, request);

        wrapup(policy, cap, ctx);
    }

    #[test]
    #[expected_failure(abort_code = mgo::transfer_policy::EIllegalRule)]
    /// Attempt to cheat by using another rule approval;
    fun test_rule_swap() {
        let ctx = &mut ctx();
        let (policy, cap) = prepare(ctx);

        // now require everyone to pay any amount
        dummy_policy::set(&mut policy, &cap);
        let request = policy::new_request(fresh_id(ctx), 10_000, fresh_id(ctx));

        // try to add receipt from another rule
        malicious_policy::cheat(&mut request);
        policy::confirm_request(&policy, request);

        wrapup(policy, cap, ctx);
    }

    public fun prepare(ctx: &mut TxContext): (TransferPolicy<Asset>, TransferPolicyCap<Asset>) {
        let publisher = package::test_claim(OTW {}, ctx);
        let (policy, cap) = policy::new<Asset>(&publisher, ctx);
        package::burn_publisher(publisher);
        (policy, cap)
    }

    public fun wrapup(policy: TransferPolicy<Asset>, cap: TransferPolicyCap<Asset>, ctx: &mut TxContext): u64 {
        let profits = policy::destroy_and_withdraw(policy, cap, ctx);
        coin::burn_for_testing(profits)
    }

    public fun fresh_id(ctx: &mut TxContext): ID {
        object::id_from_address(tx_context::fresh_object_address(ctx))
    }
}
