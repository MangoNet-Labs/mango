module mango_amm::amm_script {
    use mango_amm::amm_swap::{Pool, PoolLiquidityCoin, AdminCap, AMMFactory};
    use mango_amm::amm_config::{GlobalPauseStatus};
    use mango_amm::amm_router;
    use mgo::coin::{Coin};
    use mgo::tx_context::{TxContext};

    public entry fun init_pool<CoinTypeA, CoinTypeB>(
        amm_factory: &mut AMMFactory,
        ctx: &mut TxContext
    ) {
        amm_router::init_pool<CoinTypeA, CoinTypeB>(
            amm_factory,
            ctx
        );
    }

    public entry fun swap_exact_coinA_for_coinB<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        pause_status: &GlobalPauseStatus,
        coin_a: Coin<CoinTypeA>,
        amount_a_in: u64,
        amount_b_out_min: u64,
        ctx: &mut TxContext
    ) {
        amm_router::swap_exact_coinA_for_coinB(
            pool,
            pause_status,
            coin_a,
            amount_a_in,
            amount_b_out_min,
            ctx
        );
    }

    public entry fun swap_exact_coinB_for_coinA<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        pause_status: &GlobalPauseStatus,
        coin_b: Coin<CoinTypeB>,
        amount_b_in: u64,
        amount_a_out_min: u64,
        ctx: &mut TxContext
    ) {
        amm_router::swap_exact_coinB_for_coinA(
            pool,
            pause_status,
            coin_b,
            amount_b_in,
            amount_a_out_min,
            ctx
        );
    }

    public entry fun swap_coinA_for_exact_coinB<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        pause_status: &GlobalPauseStatus,
        coin_a: Coin<CoinTypeA>,
        amount_a_max: u64,
        amount_b_out: u64,
        ctx: &mut TxContext
    ) {
        amm_router::swap_coinA_for_exact_coinB(
            pool,
            pause_status,
            coin_a,
            amount_a_max,
            amount_b_out,
            ctx
        );
    }

    public entry fun swap_coinB_for_exact_coinA<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        pause_status: &GlobalPauseStatus,
        coin_b: Coin<CoinTypeB>,
        amount_b_max: u64,
        amount_a_out: u64,
        ctx: &mut TxContext
    ) {
        amm_router::swap_coinB_for_exact_coinA(
            pool,
            pause_status,
            coin_b,
            amount_b_max,
            amount_a_out,
            ctx
        );
    }

    public entry fun add_liquidity<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        pause_status: &GlobalPauseStatus,
        coin_a: Coin<CoinTypeA>,
        coin_b: Coin<CoinTypeB>,
        amount_a_desired: u64,
        amount_b_desired: u64,
        amount_a_min: u64,
        amount_b_min: u64,
        ctx: &mut TxContext
    ) {
        amm_router::add_liquidity(
            pool,
            pause_status,
            coin_a,
            coin_b,
            amount_a_desired,
            amount_b_desired,
            amount_a_min,
            amount_b_min,
            ctx);
    }

    public entry fun remove_liquidity<CoinTypeA, CoinTypeB>(
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        pause_status: &GlobalPauseStatus,
        coin_lp: Coin<PoolLiquidityCoin<CoinTypeA, CoinTypeB>>,
        amount_lp: u64,
        amount_a_min: u64,
        amount_b_min: u64,
        ctx: &mut TxContext
    ) {
        amm_router::remove_liquidity(
            pool,
            pause_status,
            coin_lp,
            amount_lp,
            amount_a_min,
            amount_b_min,
            ctx
        );
    }

    public entry fun set_global_pause_status(
        admin_cap: &AdminCap,
        global_pause_status: &mut GlobalPauseStatus,
        status: bool,
        ctx: &mut TxContext) {
        amm_router::set_global_pause_status(
            admin_cap,
            global_pause_status,
            status,
            ctx);
    }

    public entry fun set_fee_config<CoinTypeA, CoinTypeB>(
        admin_cap: &AdminCap,
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        trade_fee_numerator: u64,
        trade_fee_denominator: u64,
        protocol_fee_numerator: u64,
        protocol_fee_denominator: u64,
        ctx: &mut TxContext
    ) {
        amm_router::set_fee_config(
            admin_cap,
            pool,
            trade_fee_numerator,
            trade_fee_denominator,
            protocol_fee_numerator,
            protocol_fee_denominator,
            ctx
        );
    }


    public entry fun set_global_fee_config(
        admin_cap: &AdminCap,
        amm_factory: &mut AMMFactory,
        trade_fee_numerator: u64,
        trade_fee_denominator: u64,
        protocol_fee_numerator: u64,
        protocol_fee_denominator: u64,
        ctx: &mut TxContext
    ) {
        amm_router::set_global_fee_config(
            admin_cap,
            amm_factory,
            trade_fee_numerator,
            trade_fee_denominator,
            protocol_fee_numerator,
            protocol_fee_denominator,
            ctx
        );
    }

    public entry fun claim_fee<CoinTypeA, CoinTypeB>(
        admin_cap: &AdminCap,
        pool: &mut Pool<CoinTypeA, CoinTypeB>,
        ctx: &mut TxContext
    ) {
        amm_router::claim_fee(
            admin_cap,
            pool,
            ctx
        );
    }
}