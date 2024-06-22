module mgo_inscription::singlescription {
    use std::option;
    use std::option::Option;
    use std::string::String;
    use std::vector;
    use mgo::display;
    use mgo::event::emit;
    use mgo::object::{Self, ID, UID, uid_to_inner, id_to_address};
    use mgo::package;
    use mgo::transfer;
    use mgo::tx_context;
    use mgo::tx_context::TxContext;
    use mgo::vec_map;
    use mgo::vec_map::VecMap;
    use mgo::vec_set;
    use mgo::vec_set::VecSet;
    use mgo_inscription::string_util;
    use mgo_inscription::svg;

    // ======== Errors =========
    const ErrorCoprAlreadyExistsByCap: u64 = 1;
    const ErrorCoprAlreadyExists: u64 = 2;
    const ErrorInvalidCopyright: u64 = 3;

    struct CopyrightPoolRecord has key, store {
        id: UID,
        /// The Copyright name -> SingleScriptionRecord object id
        record: VecMap<String, ID>,
        cap_record: VecMap<String, ID>
    }

    struct CopyrightCap has key, store {
        id: UID,
        copyright: String
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
        let copyright_pool_record = CopyrightPoolRecord {
            id: object::new(ctx), record: vec_map::empty(), cap_record: vec_map::empty()
        };
        transfer::share_object(copyright_pool_record);

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
        let publisher = package::claim(otw, ctx);
        let display = display::new_with_fields<SingleScription>(
            &publisher, keys, values, ctx
        );
        display::update_version(&mut display);
        package::burn_publisher(publisher);
        transfer::public_share_object(display);
    }


    public fun new_copright(
        copyright_pool_record: &mut CopyrightPoolRecord,
        copr: String,
        ctx: &mut TxContext
    ): (CopyrightCap, SingleScriptionRecord) {
        assert!(!string_util::is_empty_str(&copr), ErrorInvalidCopyright);
        assert!(!vec_map::contains(&copyright_pool_record.cap_record, &copr), ErrorCoprAlreadyExistsByCap);
        assert!(!vec_map::contains(&copyright_pool_record.record, &copr), ErrorCoprAlreadyExists);

        let copr_uid = object::new(ctx);
        let copr_id = object::uid_to_inner(&copr_uid);
        let copr_cap = CopyrightCap {
            id: copr_uid,
            copyright: copr
        };

        let scription_uid = object::new(ctx);
        let scription_id = uid_to_inner(&scription_uid);
        let scription_record = SingleScriptionRecord {
            id: scription_uid,
            copyright: copr,
            record: vec_set::empty(),
        };
        vec_map::insert(&mut copyright_pool_record.record, copr, scription_id);
        vec_map::insert(&mut copyright_pool_record.cap_record, copr, copr_id);
        emit(NewCopyright {
            id: copr_id,
            deployer: tx_context::sender(ctx),
            copyright: copr,
        });
        (copr_cap, scription_record)
    }


    public fun find_copyright(
        copyright_pool_record: &mut CopyrightPoolRecord,
        copr: String
    ): (Option<address>, Option<address>) {
        let record_addr = option::none<address>();
        let copr_cap_addr = option::none<address>();
        if (vec_map::contains(&copyright_pool_record.record, &copr)) {
            let id_record = vec_map::try_get(&copyright_pool_record.record, &copr);
            let id_copr_cap = vec_map::try_get(&copyright_pool_record.cap_record, &copr);
            record_addr = option::some(id_to_address(option::borrow(&id_record)));
            copr_cap_addr = option::some(id_to_address(option::borrow(&id_copr_cap)));
        };
        (record_addr, copr_cap_addr)
    }

    public fun copyright_record_address_list(
        copyright_pool_record: &mut CopyrightPoolRecord,
        offset: u64,
        limit: u64
    ): vector<address> {
        let vec_addrs = vector::empty<address>();

        let keys = vec_map::keys(&copyright_pool_record.record);
        let len = vector::length(&keys);
        if (len > offset) {
            let i = offset;
            let end = len;
            if (offset + limit < len) {
                end = offset + limit;
            };
            while (i < end) {
                let key = vector::borrow(&keys, i);
                let value = vec_map::try_get(&copyright_pool_record.record, key);
                let addr = id_to_address(option::borrow(&value));
                vector::push_back(&mut vec_addrs, addr);
                i = i + 1;
            };
        };

        vec_addrs
    }

    public fun do_mint(
        cap: &mut CopyrightCap,
        record: &mut SingleScriptionRecord,
        name: String,
        typ: String,
        sub_typ: String,
        content: String,
        link: Option<String>,
        ctx: &mut TxContext
    ): SingleScription {
        assert!(cap.copyright == record.copyright, ErrorInvalidCopyright);
        let scription_uid = object::new(ctx);
        let scription_id = object::uid_to_inner(&scription_uid);

        let scription = SingleScription {
            id: scription_uid,
            name,
            typ,
            sub_typ,
            content,
            copyright: cap.copyright,
            link
        };

        let scription_address = object::id_address(&scription);
        vec_set::insert(&mut record.record, scription_address);
        emit(MintSingleScription {
            id: scription_id,
            sender: tx_context::sender(ctx),
            name,
            copyright: cap.copyright
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