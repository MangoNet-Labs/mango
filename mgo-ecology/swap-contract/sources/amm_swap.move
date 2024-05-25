module mango_amm::amm_swap {
    friend mango_amm::amm_router;
    use std::type_name;
    use std::type_name::TypeName;
    use mgo::table;
    use mgo::table::Table;
    use mgo::object::{Self, UID, ID, id_to_address};
    use mgo::coin::{Self, Coin};
    use mgo::balance::{Self, Supply, Balance};
    use mgo::transfer;
    use mgo::event;
    use mgo::math;
    use mgo::tx_context::{Self, TxContext};
    use mango_amm::amm_config::{new_global_pause_status_and_shared};
    use mango_amm::amm_math;
    use mgo::pay;

    const MINIMUM_LIQUIDITY: u64 = 10;

    const ECoinInsufficient: u64 = 0;
    const ESwapoutCalcInvalid: u64 = 1;
    const ELiquidityInsufficientMinted: u64 = 2;
    const ELiquiditySwapBurnCalcInvalid: u64 = 3;
    const EPoolInvalid: u64 = 4;
    const EAMOUNTINCORRECT: u64 = 5;
    const EPOOLEXIST: u64 = 6;

    struct AdminCap has key {
        id: UID,
    }

    struct AMMFactory has key, store {
        id: UID,
        trade_fee_numerator: u64,
        trade_fee_denominator: u64,
        protocol_fee_numerator: u64,
        protocol_fee_denominator: u64,
        pools: Table<TypeName, Table<TypeName, ID>>,
    }

    struct PoolLiquidityCoin<phantom CoinTypeA, phantom CoinTypeB> has drop {}

    struct Pool<phantom CoinTypeA, phantom CoinTypeB> has key, store {
        id: UID,

        coin_a: Balance<CoinTypeA>,
        coin_b: Balance<CoinTypeB>,
        coin_a_admin: Balance<CoinTypeA>,
        coin_b_admin: Balance<CoinTypeB>,

        lp_locked: Balance<PoolLiquidityCoin<CoinTypeA, CoinTypeB>>,
        lp_supply: Supply<PoolLiquidityCoin<CoinTypeA, CoinTypeB>>,

        trade_fee_numerator: u64,
        trade_fee_denominator: u64,
        protocol_fee_numerator: u64,
        protocol_fee_denominator: u64,
    }

    struct InitEvent has copy, drop {
        sender: address,
        global_paulse_status_id: ID
    }

    struct InitPoolEvent has copy, drop {
        sender: address,
        pool_id: ID,
        trade_fee_numerator: u64,
        trade_fee_denominator: u64,
        protocol_fee_numerator: u64,
        protocol_fee_denominator: u64,
    }

    struct LiquidityEvent has copy, drop {
        sender: address,
        pool_id: ID,
        is_add_liquidity: bool,
        liquidity: u64,
        amount_a: u64,
        amount_b: u64,
    }

    struct SwapEvent has copy, drop {
        sender: address,
        pool_id: ID,
        amount_a_in: u64,
        amount_a_out: u64,
        amount_b_in: u64,
        amount_b_out: u64,
    }

    struct SetFeeEvent has copy, drop {
        sender: address,
        pool_id: ID,
        trade_fee_numerator: u64,
        trade_fee_denominator: u64,
        protocol_fee_numerator: u64,
        protocol_fee_denominator: u64,
    }

    struct SetGlobalFeeEvent has copy, drop {
        sender: address,
        trade_fee_numerator: u64,
        trade_fee_denominator: u64,
        protocol_fee_numerator: u64,
        protocol_fee_denominator: u64,
    }

    struct ClaimFeeEvent has copy, drop {
        sender: address,
        pool_id: ID,
        amount_a: u64,
        amount_b: u64,
    }

    struct FlashSwapReceipt<phantom CoinTypeA, phantom CoinTypeB> {
        pool_id: ID,
        a2b: bool,
        pay_amount: u64,
        protocol_fee_amount: u64,
    }

    fun init(ctx: &mut TxContext) {
        transfer::transfer(
            AdminCap {
                id: object::new(ctx)
            },
            tx_context::sender(ctx)
        );

        let id = new_global_pause_status_and_shared(ctx);
        event::emit(InitEvent {
            sender: tx_context::sender(ctx),
            global_paulse_status_id: id
        });

        let amm_factory = AMMFactory {
            id: object::new(ctx),
            trade_fee_numerator: 1,
            trade_fee_denominator: 100,
            protocol_fee_numerator: 1,
            protocol_fee_denominator: 100,
            pools: table::new(ctx)
        };
        transfer::share_object(amm_factory);
    }


    public fun get_pool_id<CoinTypeA, CoinTypeB>(amm_factory: &AMMFactory): (address, bool) {
        let addr = @0x0;
        let a2b = false;
        let a_type = type_name::get<CoinTypeA>();
        let b_type = type_name::get<CoinTypeB>();
        if (table::contains(&amm_factory.pools, a_type)) {
            let table_inner = table::borrow(&amm_factory.pools, a_type);
            if (table::contains(table_inner, b_type)) {
                addr = id_to_address(table::borrow(table_inner, b_type));
                a2b = true;
            };
        };

        if (table::contains(&amm_factory.pools, b_type)) {
            let table_inner = table::borrow(&amm_factory.pools, b_type);
            if (table::contains(table_inner, a_type)) {
                addr = id_to_address(table::borrow(table_inner, a_type));
                a2b = false;
            };
        };

        (addr, a2b)
    }

    #[allow(lint(share_owned))]
    public(friend) fun init_pool<CoinTypeA, CoinTypeB>(
        amm_factory: &mut AMMFactory,
        ctx: &mut TxContext
    ) {
        let (check_pool_id, _) = get_pool_id<CoinTypeA, CoinTypeB>(amm_factory);
        assert!(check_pool_id == @0x0, EPOOLEXIST);

        let trade_fee_numerator = amm_factory.trade_fee_numerator;
        let trade_fee_denominator = amm_factory.trade_fee_denominator;
        let protocol_fee_numerator = amm_factory.protocol_fee_numerator;
        let protocol_fee_denominator = amm_factory.protocol_fee_denominator;
        let pool = make_pool<CoinTypeA, CoinTypeB>(
            trade_fee_numerator,
            trade_fee_denominator,
            protocol_fee_numerator,
            protocol_fee_denominator,
            ctx);
        let pool_id = object::id(&pool);
        transfer::share_object(pool);

        let a_type = type_name::get<CoinTypeA>();
        let b_type = type_name::get<CoinTypeB>();
        if (table::contains(&amm_factory.pools, a_type)) {
            let tab_inner = table::borrow_mut(&mut amm_factory.pools, a_type);
            table::add(tab_inner, b_type, pool_id);
        } else {
            let tab_inner = table::new<TypeName, ID>(ctx);
            table::add(&mut tab_inner, b_type, pool_id);
            table::add(&mut amm_factory.pools, a_type, tab_inner);
        };

        event::emit(InitPoolEvent {
            sender: tx_context::sender(ctx),
            pool_id,
            trade_fee_numerator,
            trade_fee_denominator,
            protocol_fee_numerator,
            protocol_fee_denominator
        });
    }

    fun make_pool<CoinTypeA, CoinTypeB>(
        trade_fee_numerator: u64,
        trade_fee_denominator: u64,
        protocol_fee_numerator: u64,
        protocol_fee_denominator: u64,
        ctx: &mut TxContext
    ): Pool<CoinTypeA, CoinTypeB> {
        let lp_supply = balance::create_supply(PoolLiquidityCoin<CoinTypeA, CoinTypeB> {});

        Pool<CoinTypeA, CoinTypeB> {
            id: object::new(ctx),
            coin_a: balance::zero<CoinTypeA>(),
            coin_b: balance::zero<CoinTypeB>(),
            coin_a_admin: balance::zero<CoinTypeA>(),
            coin_b_admin: balance::zero<CoinTypeB>(),
            lp_locked: balance::zero<PoolLiquidityCoin<CoinTypeA, CoinTypeB>>(),
            lp_supply,
            trade_fee_numerator,
            trade_fee_denominator,
            protocol_fee_numerator,
            protocol_fee_denominator
        }
    }

    public fun get_trade_fee<CoinTypeA, CoinTypeB>(pool: &Pool<CoinTypeA, CoinTypeB>): (u64, u64) {
        (pool.trade_fee_numerator, pool.trade_fee_denominator)
    }

    public fun get_protocol_fee<CoinTypeA, CoinTypeB>(pool: &Pool<CoinTypeA, CoinTypeB>): (u64, u64) {
        (pool.protocol_fee_numerator, pool.protocol_fee_denominator)
    }

    public fun get_reserves<CoinTypeA, CoinTypeB>(pool: &Pool<CoinTypeA, CoinTypeB>): (u64, u64) {
        (balance::value(&pool.coin_a), balance::value(&pool.coin_b))
    }

    public(friend) fun flash_swap_and_emit_event<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        amount_in: u64,
        amount_out: u64,
        a2b: bool,
        _ctx: &mut TxContext
    ): (Balance<CoinTypeA>, Balance<CoinTypeB>, FlashSwapReceipt<CoinTypeA, CoinTypeB>) {
        let (balance_a_swapped, balance_b_swapped, receipt) = flash_swap(pool, amount_in, amount_out, a2b);
        if (a2b) {
            event::emit(SwapEvent {
                sender: tx_context::sender(_ctx),
                pool_id: object::id(pool),
                amount_a_in: amount_in,
                amount_a_out: 0,
                amount_b_in: 0,
                amount_b_out: amount_out,
            });
        } else {
            event::emit(SwapEvent {
                sender: tx_context::sender(_ctx),
                pool_id: object::id(pool),
                amount_a_in: 0,
                amount_a_out: amount_out,
                amount_b_in: amount_in,
                amount_b_out: 0,
            });
        };
        (balance_a_swapped, balance_b_swapped, receipt)
    }

    public fun swap_pay_amount<CoinTypeA, CoinTypeB>(receipt: &FlashSwapReceipt<CoinTypeA, CoinTypeB>): u64 {
        receipt.pay_amount
    }

    public(friend) fun flash_swap<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        amount_in: u64,
        amount_out: u64,
        a2b: bool,
    ): (Balance<CoinTypeA>, Balance<CoinTypeB>, FlashSwapReceipt<CoinTypeA, CoinTypeB>) {
        assert!(amount_in > 0, ECoinInsufficient);
        let (a_reserve, b_reserve) = get_reserves<CoinTypeA, CoinTypeB>(pool);
        let (fee_numerator, fee_denominator) = get_trade_fee<CoinTypeA, CoinTypeB>(pool);

        let balance_a_swapped = balance::zero<CoinTypeA>();
        let balance_b_swapped = balance::zero<CoinTypeB>();
        if (a2b) {
            balance::join(&mut balance_b_swapped, balance::split(&mut pool.coin_b, amount_out));
            let a_reserve_new = a_reserve + amount_in;
            let b_reserve_new = b_reserve - amount_out;

            let (a_adjusted, b_adjusted) = new_reserves_adjusted(
                a_reserve_new,
                b_reserve_new,
                amount_in,
                0,
                fee_numerator,
                fee_denominator);


            assert_lp_value_incr(
                a_reserve,
                b_reserve,
                a_adjusted,
                b_adjusted,
                fee_denominator
            );
        } else {
            balance::join(&mut balance_a_swapped, balance::split(&mut pool.coin_a, amount_out));
            let a_reserve_new = a_reserve - amount_out;
            let b_reserve_new = b_reserve + amount_in;

            let (a_adjusted, b_adjusted) = new_reserves_adjusted(
                a_reserve_new,
                b_reserve_new,
                0,
                amount_in,
                fee_numerator,
                fee_denominator);

            assert_lp_value_incr(
                a_reserve,
                b_reserve,
                a_adjusted,
                b_adjusted,
                fee_denominator
            );
        };

        let (protocol_fee_numberator, protocol_fee_denominator) = calc_swap_protocol_fee_rate(pool);
        let protocol_swap_fee = amm_math::safe_mul_div_u64(
            amount_in,
            protocol_fee_numberator,
            protocol_fee_denominator
        );
        (balance_a_swapped, balance_b_swapped, FlashSwapReceipt<CoinTypeA, CoinTypeB> {
            pool_id: object::id(pool),
            a2b,
            pay_amount: amount_in,
            protocol_fee_amount: protocol_swap_fee
        })
    }

    public(friend) fun repay_flash_swap<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        balance_a: Balance<CoinTypeA>,
        balance_b: Balance<CoinTypeB>,
        receipt: FlashSwapReceipt<CoinTypeA, CoinTypeB>
    ) {
        let FlashSwapReceipt<CoinTypeA, CoinTypeB> {
            pool_id,
            a2b,
            pay_amount,
            protocol_fee_amount
        } = receipt;
        assert!(pool_id == object::id(pool), EPoolInvalid);

        if (a2b) {
            assert!(balance::value(&balance_a) == pay_amount, EAMOUNTINCORRECT);
            let balance_protocol_fee = balance::split(&mut balance_a, protocol_fee_amount);
            balance::join(&mut pool.coin_a, balance_a);
            balance::join(&mut pool.coin_a_admin, balance_protocol_fee);
            balance::destroy_zero(balance_b);
        } else {
            assert!(balance::value(&balance_b) == pay_amount, EAMOUNTINCORRECT);
            let balance_protocol_fee = balance::split(&mut balance_b, protocol_fee_amount);
            balance::join(&mut pool.coin_b, balance_b);
            balance::join(&mut pool.coin_b_admin, balance_protocol_fee);
            balance::destroy_zero(balance_a);
        }
    }

    public(friend) fun swap_and_emit_event<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        balance_a_in: Balance<CoinTypeA>,
        b_out: u64,
        balance_b_in: Balance<CoinTypeB>,
        a_out: u64,
        _ctx: &mut TxContext
    ): (Balance<CoinTypeA>, Balance<CoinTypeB>, Balance<CoinTypeA>, Balance<CoinTypeB>) {
        let balance_a_in_value = balance::value(&balance_a_in);
        let balance_b_in_value = balance::value(&balance_b_in);

        let (balance_a_out, balance_b_out, balance_a_fee, balance_b_fee) = swap(
            pool,
            balance_a_in,
            b_out,
            balance_b_in,
            a_out
        );
        event::emit(SwapEvent {
            sender: tx_context::sender(_ctx),
            pool_id: object::id(pool),
            amount_a_in: balance_a_in_value,
            amount_a_out: balance::value(&balance_a_out),
            amount_b_in: balance_b_in_value,
            amount_b_out: balance::value(&balance_b_out),
        });
        (balance_a_out, balance_b_out, balance_a_fee, balance_b_fee)
    }

    fun swap<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        balance_a_in: Balance<CoinTypeA>,
        b_out: u64,
        balance_b_in: Balance<CoinTypeB>,
        a_out: u64
    ): (Balance<CoinTypeA>, Balance<CoinTypeB>, Balance<CoinTypeA>, Balance<CoinTypeB>) {
        let balance_a_in_value = balance::value(&balance_a_in);
        let balance_b_in_value = balance::value(&balance_b_in);
        assert!(
            balance_a_in_value > 0 || balance_b_in_value > 0,
            ECoinInsufficient
        );

        let (a_reserve, b_reserve) = get_reserves<CoinTypeA, CoinTypeB>(pool);
        balance::join(&mut pool.coin_a, balance_a_in);
        balance::join(&mut pool.coin_b, balance_b_in);


        let balance_a_swapped = balance::split(&mut pool.coin_a, a_out);
        let balance_b_swapped = balance::split(&mut pool.coin_b, b_out);

        {
            let a_reserve_new = balance::value(&pool.coin_a);
            let b_reserve_new = balance::value(&pool.coin_b);
            let (fee_numerator, fee_denominator) = get_trade_fee<CoinTypeA, CoinTypeB>(pool);

            let (a_adjusted, b_adjusted) = new_reserves_adjusted(
                a_reserve_new,
                b_reserve_new,
                balance_a_in_value,
                balance_b_in_value,
                fee_numerator,
                fee_denominator);


            assert_lp_value_incr(
                a_reserve,
                b_reserve,
                a_adjusted,
                b_adjusted,
                fee_denominator
            );
        };

        let (protocol_fee_numberator, protocol_fee_denominator) = calc_swap_protocol_fee_rate(pool);
        let a_swap_fee = balance::split(
            &mut pool.coin_a,
            amm_math::safe_mul_div_u64(balance_a_in_value, protocol_fee_numberator, protocol_fee_denominator)
        );
        let b_swap_fee = balance::split(
            &mut pool.coin_b,
            amm_math::safe_mul_div_u64(balance_b_in_value, protocol_fee_numberator, protocol_fee_denominator)
        );
        (balance_a_swapped, balance_b_swapped, a_swap_fee, b_swap_fee)
    }

    fun calc_swap_protocol_fee_rate<CoinTypeA, CoinTypeB>(pool: &Pool<CoinTypeA, CoinTypeB>): (u64, u64) {
        let (fee_numerator, fee_denominator) = get_trade_fee(pool);
        let (protocol_fee_numerator, protocol_fee_denominator) = get_protocol_fee(pool);
        (amm_math::safe_mul_u64(fee_numerator, protocol_fee_numerator), amm_math::safe_mul_u64(
            fee_denominator,
            protocol_fee_denominator
        ))
    }

    public(friend) fun handle_swap_protocol_fee<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        fee_a: Balance<CoinTypeA>,
        fee_b: Balance<CoinTypeB>
    ) {
        balance::join(&mut pool.coin_a_admin, fee_a);
        balance::join(&mut pool.coin_b_admin, fee_b);
    }

    public(friend) fun set_fee_and_emit_event<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        trade_fee_numerator: u64,
        trade_fee_denominator: u64,
        protocol_fee_numerator: u64,
        protocol_fee_denominator: u64,
        _ctx: &mut TxContext
    ) {
        pool.trade_fee_numerator = trade_fee_numerator;
        pool.trade_fee_denominator = trade_fee_denominator;
        pool.protocol_fee_numerator = protocol_fee_numerator;
        pool.protocol_fee_denominator = protocol_fee_denominator;

        event::emit(SetFeeEvent {
            sender: tx_context::sender(_ctx),
            pool_id: object::id(pool),
            trade_fee_numerator,
            trade_fee_denominator,
            protocol_fee_numerator,
            protocol_fee_denominator
        });
    }


    public(friend) fun set_global_fee_and_emit_event(
        amm_factory: &mut AMMFactory,
        trade_fee_numerator: u64,
        trade_fee_denominator: u64,
        protocol_fee_numerator: u64,
        protocol_fee_denominator: u64,
        _ctx: &mut TxContext
    ) {
        amm_factory.trade_fee_numerator = trade_fee_numerator;
        amm_factory.trade_fee_denominator = trade_fee_denominator;
        amm_factory.protocol_fee_numerator = protocol_fee_numerator;
        amm_factory.protocol_fee_denominator = protocol_fee_denominator;

        event::emit(SetGlobalFeeEvent {
            sender: tx_context::sender(_ctx),
            trade_fee_numerator,
            trade_fee_denominator,
            protocol_fee_numerator,
            protocol_fee_denominator
        });
    }

    public(friend) fun claim_fee<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        ctx: &mut TxContext
    ) {
        let a_fee_value = balance::value(&pool.coin_a_admin);
        let b_fee_value = balance::value(&pool.coin_b_admin);

        assert!(
            a_fee_value > 0 || b_fee_value > 0,
            ECoinInsufficient
        );

        let balance_a_fee = balance::split(&mut pool.coin_a_admin, a_fee_value);
        let balance_b_fee = balance::split(&mut pool.coin_b_admin, b_fee_value);

        pay::keep(coin::from_balance(balance_a_fee, ctx), ctx);
        pay::keep(coin::from_balance(balance_b_fee, ctx), ctx);

        event::emit(ClaimFeeEvent {
            sender: tx_context::sender(ctx),
            pool_id: object::id(pool),
            amount_a: a_fee_value,
            amount_b: b_fee_value,
        });
    }

    public(friend) fun mint_and_emit_event<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        balance_a: Balance<CoinTypeA>,
        balance_b: Balance<CoinTypeB>,
        amount_a_desired: u64,
        amount_b_desired: u64,
        ctx: &mut TxContext
    ): Coin<PoolLiquidityCoin<CoinTypeA, CoinTypeB>> {
        let coin_liquidity = mint(pool, balance_a, balance_b, ctx);
        event::emit(LiquidityEvent {
            sender: tx_context::sender(ctx),
            pool_id: object::id(pool),
            is_add_liquidity: true,
            liquidity: coin::value(&coin_liquidity),
            amount_a: amount_a_desired,
            amount_b: amount_b_desired,
        });
        coin_liquidity
    }

    fun mint<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        balance_a: Balance<CoinTypeA>,
        balance_b: Balance<CoinTypeB>,
        ctx: &mut TxContext
    ): Coin<PoolLiquidityCoin<CoinTypeA, CoinTypeB>> {
        let (reserve_a, reserve_b) = get_reserves(pool);

        let amount_a = balance::value(&balance_a);
        let amonut_b = balance::value(&balance_b);

        let total_supply = balance::supply_value(&pool.lp_supply);
        let liquidity: u64;
        if (total_supply == 0) {
            liquidity = (math::sqrt_u128((amount_a as u128) * (amonut_b as u128)) as u64) - MINIMUM_LIQUIDITY;
            let balance_lp_locked = balance::increase_supply(&mut pool.lp_supply, MINIMUM_LIQUIDITY);
            balance::join(&mut pool.lp_locked, balance_lp_locked);
        } else {
            liquidity = math::min(
                amm_math::safe_mul_div_u64(amount_a, total_supply, reserve_a),
                amm_math::safe_mul_div_u64(amonut_b, total_supply, reserve_b));
        };

        assert!(liquidity > 0, ELiquidityInsufficientMinted);

        balance::join(&mut pool.coin_a, balance_a);
        balance::join(&mut pool.coin_b, balance_b);

        coin::from_balance(
            balance::increase_supply(
                &mut pool.lp_supply,
                liquidity
            ), ctx)
    }

    public(friend) fun burn_and_emit_event<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        to_burn: Balance<PoolLiquidityCoin<CoinTypeA, CoinTypeB>>,
        ctx: &mut TxContext
    ): (Coin<CoinTypeA>, Coin<CoinTypeB>) {
        let to_burn_value = balance::value(&to_burn);
        let (coin_a, coin_b) = burn(pool, to_burn, ctx);

        event::emit(LiquidityEvent {
            sender: tx_context::sender(ctx),
            pool_id: object::id(pool),
            is_add_liquidity: false,
            liquidity: to_burn_value,
            amount_a: coin::value(&coin_a),
            amount_b: coin::value(&coin_b),
        });

        (coin_a, coin_b)
    }

    fun burn<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        to_burn: Balance<PoolLiquidityCoin<CoinTypeA, CoinTypeB>>,
        ctx: &mut TxContext
    ): (Coin<CoinTypeA>, Coin<CoinTypeB>) {
        let to_burn_value = balance::value(&to_burn);

        let (reserve_a, reserve_b) = get_reserves(pool);
        let total_supply = balance::supply_value(&pool.lp_supply);

        let amount_a = amm_math::safe_mul_div_u64(to_burn_value, reserve_a, total_supply);
        let amount_b = amm_math::safe_mul_div_u64(to_burn_value, reserve_b, total_supply);
        assert!(amount_a > 0 && amount_b > 0, ELiquiditySwapBurnCalcInvalid);

        balance::decrease_supply(&mut pool.lp_supply, to_burn);

        let coin_a = coin::from_balance(balance::split(&mut pool.coin_a, amount_a), ctx);
        let coin_b = coin::from_balance(balance::split(&mut pool.coin_b, amount_b), ctx);
        (coin_a, coin_b)
    }

    public fun lp_to_ab<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        lp: u64,
    ): (u64, u64) {
        let lp_value = lp;

        let (reserve_a, reserve_b) = get_reserves(pool);
        let total_supply = balance::supply_value(&pool.lp_supply);

        let amount_a = amm_math::safe_mul_div_u64(lp_value, reserve_a, total_supply);
        let amount_b = amm_math::safe_mul_div_u64(lp_value, reserve_b, total_supply);
        (amount_a,amount_b)
    }

    fun new_reserves_adjusted(
        a_reserve: u64,
        b_reserve: u64,
        a_in_val: u64,
        b_in_val: u64,
        fee_numerator: u64,
        fee_denominator: u64
    ): (u64, u64) {
        let a_adjusted = a_reserve * fee_denominator - a_in_val * fee_numerator;
        let b_adjusted = b_reserve * fee_denominator - b_in_val * fee_numerator;
        (a_adjusted, b_adjusted)
    }

    fun assert_lp_value_incr(
        a_reserve: u64,
        b_reserve: u64,
        a_adjusted: u64,
        b_adjusted: u64,
        fee_denominator: u64
    ) {
        assert!(
            amm_math::safe_compare_mul_u64(
                a_adjusted,
                b_adjusted,
                a_reserve * fee_denominator,
                b_reserve * fee_denominator
            ),
            ESwapoutCalcInvalid);
    }
}