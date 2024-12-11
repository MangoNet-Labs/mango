module mgo_inscription::singlescription {
    use std::option;
    use std::option::Option;
    use std::string::String;
    use std::vector;

    use mgo::display;
    use mgo::display::Display;
    use mgo::event::emit;
    use mgo::object::{Self, ID, UID, uid_to_address, uid_to_inner};
    use mgo::package;
    use mgo::table;
    use mgo::table::Table;
    use mgo::transfer;
    use mgo::tx_context;
    use mgo::tx_context::TxContext;
    use mgo::vec_set;
    use mgo::vec_set::VecSet;

    use mgo_inscription::string_util;
    use mgo_inscription::svg;

    // ======== Errors =========
    const ErrorCoprAlreadyExists: u64 = 1;
    const ErrorInvalidCopyright: u64 = 2;
    const ErrorDisplayInited: u64 = 3;

    struct CopyrightPoolRecord has key, store {
        id: UID,
        /// The Copyright name -> SingleScriptionRecord object id
        record: Table<String, address>,
        display: Display<SingleScription>
    }

    struct SingleScriptionRecord has key, store {
        id: UID,
        copyright: String,
        record: VecSet<address>,
    }

    struct SingleScription has key, store {
        id: UID,
        name: String,
        typ: String,
        sub_typ: String,
        copyright: String,
        content: String,
        link: Option<String>,
    }

    struct SINGLESCRIPTION has drop {}

    // ======== Events =========
    struct NewCopyright has copy, drop {
        id: ID,
        deployer: address,
        copyright: String,
    }

    struct MintSingleScription has copy, drop {
        id: ID,
        sender: address,
        name: String,
        copyright: String,
    }

    struct BurnSingleScription has copy, drop {
        id: ID,
        sender: address,
        copyright: String,
    }


    fun init(otw: SINGLESCRIPTION, ctx: &mut TxContext) {
        let publisher = package::claim(otw, ctx);

        let keys = vector[
            std::string::utf8(b"name"),
            std::string::utf8(b"image_url"),
        ];

        let p = b"mrc-721";
        let na = b"{name}";
        let typ = b"{typ}";
        let copr = b"{copyright}";

        let img_metadata = svg::generate_singlescription_svg(p, na, typ, copr);

        let values = vector[
            std::string::utf8(b"{name}"),
            std::string::utf8(img_metadata),
        ];


        let display = display::new_with_fields<SingleScription>(
            &publisher, keys, values, ctx
        );

        let copyright_pool_record = CopyrightPoolRecord {
            id: object::new(ctx), record: table::new(ctx), display
        };
        transfer::share_object(copyright_pool_record);

        package::burn_publisher(publisher);
    }

    public entry fun init_display(copyright_pool_record: &mut CopyrightPoolRecord, _ctx: &mut TxContext) {
        assert!(display::version(&copyright_pool_record.display) == 0, ErrorDisplayInited);
        display::update_version(&mut copyright_pool_record.display);
    }


    public fun new_copright(
        copyright_pool_record: &mut CopyrightPoolRecord,
        copr: String,
        ctx: &mut TxContext
    ): (SingleScriptionRecord) {
        assert!(!string_util::is_empty_str(&copr), ErrorInvalidCopyright);
        assert!(!table::contains(&copyright_pool_record.record, copr), ErrorCoprAlreadyExists);

        let scription_record = SingleScriptionRecord {
            id: object::new(ctx),
            copyright: copr,
            record: vec_set::empty(),
        };
        table::add(&mut copyright_pool_record.record, copr, uid_to_address(&scription_record.id));
        emit(NewCopyright {
            id: uid_to_inner(&scription_record.id),
            deployer: tx_context::sender(ctx),
            copyright: copr,
        });
        scription_record
    }


    public fun find_copyright(
        copyright_pool_record: &mut CopyrightPoolRecord,
        copr: String
    ): (Option<address>) {
        let record_addr = option::none<address>();
        if (table::contains(&copyright_pool_record.record, copr)) {
            let addr = table::borrow(&copyright_pool_record.record, copr);
            record_addr = option::some(*addr);
        };
        (record_addr)
    }

    public fun copyright_record_address_list(
        copyright_pool_record: &mut CopyrightPoolRecord,
        coprs: vector<String>
    ): vector<address> {
        let vec_addrs = vector::empty<address>();

        let i = 0;
        let end = vector::length(&coprs);
        while (i < end) {
            let key = vector::borrow(&coprs, i);
            let value = table::borrow(&copyright_pool_record.record, *key);
            vector::push_back(&mut vec_addrs, *value);
            i = i + 1;
        };

        vec_addrs
    }

    public fun do_mint(
        record: &mut SingleScriptionRecord,
        name: String,
        typ: String,
        sub_typ: String,
        content: String,
        link: Option<String>,
        ctx: &mut TxContext
    ): SingleScription {
        let scription_uid = object::new(ctx);
        let scription_id = object::uid_to_inner(&scription_uid);

        let scription = SingleScription {
            id: scription_uid,
            name,
            typ,
            sub_typ,
            content,
            copyright: record.copyright,
            link
        };

        let scription_address = object::id_address(&scription);
        vec_set::insert(&mut record.record, scription_address);
        emit(MintSingleScription {
            id: scription_id,
            sender: tx_context::sender(ctx),
            name,
            copyright: record.copyright
        });
        scription
    }


    public fun do_burn(
        record: &mut SingleScriptionRecord,
        scription: SingleScription,
        ctx: &mut TxContext
    ) {
        assert!(record.copyright == scription.copyright, ErrorInvalidCopyright);

        let address = object::id_address(&scription);
        let SingleScription { id: uid, name: _, typ: _, sub_typ: _, copyright, content: _, link: _ } = scription;
        let id = object::uid_to_inner(&uid);

        vec_set::remove(&mut record.record, &address);
        object::delete(uid);

        emit({
            BurnSingleScription {
                id,
                sender: tx_context::sender(ctx),
                copyright,
            }
        });
    }
}