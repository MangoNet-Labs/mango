// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
/// Dummy policy which showcases all of the methods.
module mgo::dummy_policy {
    use mgo::coin::Coin;
    use mgo::mgo::MGO;
    use mgo::transfer_policy::{
        Self as policy,
        TransferPolicy,
        TransferPolicyCap,
        TransferRequest
    };

    struct Rule has drop {}
    struct Config has store, drop {}

    public fun set<T>(
        policy: &mut TransferPolicy<T>,
        cap: &TransferPolicyCap<T>
    ) {
        policy::add_rule(Rule {}, policy, cap, Config {})
    }

    public fun pay<T>(
        policy: &mut TransferPolicy<T>,
        request: &mut TransferRequest<T>,
        payment: Coin<MGO>
    ) {
        policy::add_to_balance(Rule {}, policy, payment);
        policy::add_receipt(Rule {}, request);
    }
}
