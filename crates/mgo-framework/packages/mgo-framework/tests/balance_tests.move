// Copyright (c) MangoNet Labs Ltd.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
module mgo::coin_balance_tests {
    use mgo::test_scenario::{Self, ctx};
    use mgo::pay;
    use mgo::coin;
    use mgo::balance;
    use mgo::mgo::MGO;

    #[test]
    fun type_morphing() {
        let scenario = test_scenario::begin(@0x1);
        let test = &mut scenario;

        let balance = balance::zero<MGO>();
        let coin = coin::from_balance(balance, ctx(test));
        let balance = coin::into_balance(coin);

        balance::destroy_zero(balance);

        let coin = coin::mint_for_testing<MGO>(100, ctx(test));
        let balance_mut = coin::balance_mut(&mut coin);
        let sub_balance = balance::split(balance_mut, 50);

        assert!(balance::value(&sub_balance) == 50, 0);
        assert!(coin::value(&coin) == 50, 0);

        let balance = coin::into_balance(coin);
        balance::join(&mut balance, sub_balance);

        assert!(balance::value(&balance) == 100, 0);

        let coin = coin::from_balance(balance, ctx(test));
        pay::keep(coin, ctx(test));
        test_scenario::end(scenario);
    }
}
