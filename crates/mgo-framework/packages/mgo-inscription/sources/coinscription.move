module mgo_inscription::coinscription {
    use std::option;
    use std::option::Option;
    use std::string::String;
    use std::vector;
    use mgo::display;
    use mgo::event::emit;
    use mgo::object::{Self, ID, UID, id_to_address};
    use mgo::package;
    use mgo::transfer;
    use mgo::tx_context;
    use mgo::tx_context::TxContext;
    use mgo::vec_map;
    use mgo::vec_map::VecMap;
    use mgo_inscription::string_util;
    use mgo_inscription::svg;


    // ======== Errors =========
    const ErrorTickAlreadyExists: u64 = 1;
    const ErrorNotEnoughSupply: u64 = 2;
    const ErrorNotEnoughToMint: u64 = 3;
    const EInvalidAmount: u64 = 4;
    const ErrorNotSameTick: u64 = 5;
    const ErrorCannotBurn: u64 = 6;
    const ErrorNotZero: u64 = 7;
    const ErrorInvalidTick: u64 = 8;

    struct TickPoolRecord has key, store {
        id: UID,
        /// The Tick name -> TickRecord object id
        record: VecMap<String, ID>,
    }

    struct TickRecord has key, store {
        id: UID,
        tick: String,
        total_supply: u64,
        burnable: bool,
        remain: u64,
        current_supply: u64,
    }

    struct CoinScription has key, store {
        id: UID,
        amount: u64,
        tick: String,
    }

    struct COINSCRIPTION has drop {}

    // ======== Events =========
    struct NewTick has copy, drop {
        id: ID,
        deployer: address,
        tick: String,
        total_supply: u64,
        burnable: bool,
    }

    struct MintCoinScription has copy, drop {
        id: ID,
        sender: address,
        tick: String,
        amount: u64,
    }

    struct BurnCoinScription has copy, drop {
        sender: address,
        tick: String,
        amount: u64,
    }


    fun init(otw: COINSCRIPTION, ctx: &mut TxContext) {
        let tick_pool_record = TickPoolRecord { id: object::new(ctx), record: vec_map::empty() };
        transfer::share_object(tick_pool_record);

        let keys = vector[
            std::string::utf8(b"tick"),
            std::string::utf8(b"amount"),
            std::string::utf8(b"image_url"),
        ];

        let p = b"mrc-20";
        let op = b"mint";
        let tick = b"{tick}";
        let amt = b"{amount}";

        let img_metadata = svg::generate_coinscription_svg(p, op, tick, amt);

        let values = vector[
            std::string::utf8(b"{tick}"),
            std::string::utf8(b"{amount}"),
            std::string::utf8(img_metadata),
        ];
        let publisher = package::claim(otw, ctx);
        let display = display::new_with_fields<CoinScription>(
            &publisher, keys, values, ctx
        );
        display::update_version(&mut display);
        package::burn_publisher(publisher);
        transfer::public_share_object(display);
    }

    public fun new_tick(
        tick_pool_record: &mut TickPoolRecord,
        tick: String,
        total_supply: u64,
        burnable: bool,
        ctx: &mut TxContext
    ): TickRecord {
        assert!(string_util::is_tick_valid(&tick), ErrorInvalidTick);
        assert!(!vec_map::contains(&tick_pool_record.record, &tick), ErrorTickAlreadyExists);
        assert!(total_supply > 0, ErrorNotEnoughSupply);

        let tick_uid = object::new(ctx);
        let tick_id = object::uid_to_inner(&tick_uid);
        let tick_record: TickRecord = TickRecord {
            id: tick_uid,
            tick,
            total_supply,
            burnable,
            remain: total_supply,
            current_supply: 0,
        };
        vec_map::insert(&mut tick_pool_record.record, tick, tick_id);
        emit(NewTick {
            id: tick_id,
            deployer: tx_context::sender(ctx),
            tick,
            total_supply,
            burnable
        });
        tick_record
    }

    public fun find_tick(tick_pool_record: &mut TickPoolRecord, tick: String): Option<address> {
        if (vec_map::contains(&tick_pool_record.record, &tick)) {
            let id_tick = vec_map::try_get(&tick_pool_record.record, &tick);
            option::some(id_to_address(option::borrow(&id_tick)))
        } else {
            option::none()
        }
    }

    public fun tick_record_address_list(
        tick_pool_record: &mut TickPoolRecord,
        offset: u64,
        limit: u64
    ): vector<address> {
        let vec_addrs = vector::empty<address>();

        let keys = vec_map::keys(&tick_pool_record.record);
        let len = vector::length(&keys);
        if (len > offset) {
            let i = offset;
            let end = len;
            if (offset + limit < len) {
                end = offset + limit;
            };
            while (i < end) {
                let key = vector::borrow(&keys, i);
                let value = vec_map::try_get(&tick_pool_record.record, key);
                let addr = id_to_address(option::borrow(&value));
                vector::push_back(&mut vec_addrs, addr);
                i = i + 1;
            };
        };

        vec_addrs
    }

    public fun do_mint(
        tick_record: &mut TickRecord,
        amount: u64,
        ctx: &mut TxContext
    ): CoinScription {
        assert!(tick_record.remain > 0, ErrorNotEnoughToMint);
        assert!(tick_record.remain >= amount, ErrorNotEnoughToMint);

        tick_record.remain = tick_record.remain - amount;
        tick_record.current_supply = tick_record.current_supply + amount;

        let tick: String = tick_record.tick;
        let sender = tx_context::sender(ctx);
        let coinscription = CoinScription {
            id: object::new(ctx),
            amount,
            tick,
        };
        emit(MintCoinScription {
            id: object::id(&coinscription),
            sender,
            tick,
            amount,
        });
        coinscription
    }

    // ======= Merge functions ========

    public fun is_mergeable(inscription1: &CoinScription, inscription2: &CoinScription): bool {
        inscription1.tick == inscription2.tick
    }

    public fun do_merge(
        inscription1: &mut CoinScription,
        inscription2: CoinScription,
    ) {
        assert!(inscription1.tick == inscription2.tick, ErrorNotSameTick);
        let CoinScription { id, amount, tick: _ } = inscription2;
        inscription1.amount = inscription1.amount + amount;
        object::delete(id);
    }


    // ======= Split functions ========

    /// Check if the inscription can be split
    public fun is_splitable(inscription: &CoinScription): bool {
        inscription.amount > 1
    }

    /// Split the inscription and return the new inscription
    public fun do_split(
        inscription: &mut CoinScription,
        amount: u64,
        ctx: &mut TxContext
    ): CoinScription {
        assert!(0 < amount && amount < inscription.amount, EInvalidAmount);
        let original_amount = inscription.amount;
        inscription.amount = original_amount - amount;
        let split_movescription = CoinScription {
            id: object::new(ctx),
            amount,
            tick: inscription.tick,
        };
        split_movescription
    }

    #[lint_allow(self_transfer)]
    public entry fun split(
        inscription: &mut CoinScription,
        amount: u64,
        ctx: &mut TxContext
    ) {
        let ins = do_split(inscription, amount, ctx);
        transfer::public_transfer(ins, tx_context::sender(ctx));
    }


    // ========= Destroy and Burn functions =========

    public fun zero(tick_record: &TickRecord, ctx: &mut TxContext): CoinScription {
        CoinScription {
            id: object::new(ctx),
            tick: tick_record.tick,
            amount: 0
        }
    }

    public fun is_zero(self: &CoinScription): bool {
        self.amount == 0
    }

    public fun destroy_zero(self: CoinScription) {
        assert!(self.amount == 0, ErrorNotZero);
        let CoinScription { id, amount: _, tick: _ } = self;
        object::delete(id);
    }


    public entry fun do_burn(
        tick_record: &mut TickRecord,
        inscription: CoinScription,
        ctx: &mut TxContext
    ) {
        assert!(tick_record.tick == inscription.tick, ErrorNotSameTick);
        assert!(tick_record.burnable, ErrorCannotBurn);
        let sender = tx_context::sender(ctx);
        let CoinScription { id: scription_uid, amount, tick } = inscription;
        tick_record.current_supply = tick_record.current_supply - amount;
        object::delete(scription_uid);

        emit({
            BurnCoinScription {
                sender,
                tick,
                amount,
            }
        });
    }
}