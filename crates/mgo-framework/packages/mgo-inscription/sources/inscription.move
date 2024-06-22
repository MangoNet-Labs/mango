module mgo_inscription::inscription {
    use std::option;
    use std::string;
    use std::string::String;
    use mgo::transfer::{public_transfer, public_share_object};
    use mgo::tx_context;
    use mgo::tx_context::TxContext;
    use mgo_inscription::singlescription;
    use mgo_inscription::singlescription::{CopyrightPoolRecord, CopyrightCap, SingleScriptionRecord, SingleScription};
    use mgo_inscription::coinscription::{TickPoolRecord, TickRecord, CoinScription};
    use mgo_inscription::coinscription;

    #[allow(lint(share_owned))]
    public entry fun coinscription_new_tick(tick_pool_record: &mut TickPoolRecord,
                                            tick: String,
                                            total_supply: u64,
                                            burnable: bool,
                                            ctx: &mut TxContext) {
        let tick_record = coinscription::new_tick(
            tick_pool_record,
            tick,
            total_supply,
            burnable,
            ctx
        );
        public_share_object(tick_record)
    }


    public entry fun coinscription_mint(tick_record: &mut TickRecord,
                                        amount: u64,
                                        ctx: &mut TxContext) {
        let coinscription = coinscription::do_mint(tick_record, amount, ctx);
        public_transfer(coinscription, tx_context::sender(ctx))
    }


    public entry fun coinscription_transfer(scription: CoinScription, receipt: address, _: &mut TxContext) {
        public_transfer(scription, receipt)
    }

    public entry fun coinscription_split(scription: &mut CoinScription, amount: u64, ctx: &mut TxContext) {
        let scription = coinscription::do_split(scription, amount, ctx);
        public_transfer(scription, tx_context::sender(ctx));
    }

    public entry fun coinscription_merge(scription1: &mut CoinScription, scription2: CoinScription, _: &mut TxContext) {
        coinscription::do_merge(scription1, scription2)
    }

    public entry fun coinscription_burn(tick_record: &mut TickRecord,
                                        inscription: CoinScription,
                                        ctx: &mut TxContext) {
        coinscription::do_burn(tick_record, inscription, ctx)
    }


    #[allow(lint(share_owned))]
    public entry fun singlescription_new_copyright(copyright_pool_record: &mut CopyrightPoolRecord,
                                                   copr: String,
                                                   ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        let (cap, record) = singlescription::new_copright(copyright_pool_record, copr, ctx);
        public_share_object(record);
        public_transfer(cap, sender);
    }


    public entry fun singlescription_mint(cap: &mut CopyrightCap,
                                          record: &mut SingleScriptionRecord,
                                          name: String,
                                          typ: String,
                                          sub_typ: String,
                                          content: String,
                                          link: String,
                                          ctx: &mut TxContext) {
        let link_opt = option::none<String>();
        if (!string::is_empty(&link)) {
            option::fill(&mut link_opt, link);
        };
        let scription = singlescription::do_mint(cap, record, name, typ, sub_typ, content, link_opt, ctx);
        public_transfer(scription, tx_context::sender(ctx));
    }

    public entry fun singlescription_transfer(scription: SingleScription, receipt: address, _: &mut TxContext) {
        public_transfer(scription, receipt);
    }

    public entry fun singlescription_burn(record: &mut SingleScriptionRecord,
                                          scription: SingleScription,
                                          ctx: &mut TxContext) {
        singlescription::do_burn(record, scription, ctx);
    }
}