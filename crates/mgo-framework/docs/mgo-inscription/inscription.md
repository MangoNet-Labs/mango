
<a name="0x4_inscription"></a>

# Module `0x4::inscription`



-  [Function `coinscription_new_tick`](#0x4_inscription_coinscription_new_tick)
-  [Function `coinscription_mint`](#0x4_inscription_coinscription_mint)
-  [Function `coinscription_transfer`](#0x4_inscription_coinscription_transfer)
-  [Function `coinscription_split`](#0x4_inscription_coinscription_split)
-  [Function `coinscription_merge`](#0x4_inscription_coinscription_merge)
-  [Function `coinscription_burn`](#0x4_inscription_coinscription_burn)
-  [Function `singlescription_new_copyright`](#0x4_inscription_singlescription_new_copyright)
-  [Function `singlescription_mint`](#0x4_inscription_singlescription_mint)
-  [Function `singlescription_transfer`](#0x4_inscription_singlescription_transfer)
-  [Function `singlescription_burn`](#0x4_inscription_singlescription_burn)


<pre><code><b>use</b> <a href="dependencies/move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="dependencies/move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="dependencies/mgo-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="coinscription.md#0x4_coinscription">0x4::coinscription</a>;
<b>use</b> <a href="singlescription.md#0x4_singlescription">0x4::singlescription</a>;
</code></pre>



<a name="0x4_inscription_coinscription_new_tick"></a>

## Function `coinscription_new_tick`



<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_new_tick">coinscription_new_tick</a>(tick_pool_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickPoolRecord">coinscription::TickPoolRecord</a>, tick: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, total_supply: u64, burnable: bool, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_new_tick">coinscription_new_tick</a>(tick_pool_record: &<b>mut</b> TickPoolRecord,
                                        tick: String,
                                        total_supply: u64,
                                        burnable: bool,
                                        ctx: &<b>mut</b> TxContext) {
    <b>let</b> tick_record = <a href="coinscription.md#0x4_coinscription_new_tick">coinscription::new_tick</a>(
        tick_pool_record,
        tick,
        total_supply,
        burnable,
        ctx
    );
    public_share_object(tick_record)
}
</code></pre>



</details>

<a name="0x4_inscription_coinscription_mint"></a>

## Function `coinscription_mint`



<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_mint">coinscription_mint</a>(tick_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickRecord">coinscription::TickRecord</a>, amount: u64, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_mint">coinscription_mint</a>(tick_record: &<b>mut</b> TickRecord,
                                    amount: u64,
                                    ctx: &<b>mut</b> TxContext) {
    <b>let</b> <a href="coinscription.md#0x4_coinscription">coinscription</a> = <a href="coinscription.md#0x4_coinscription_do_mint">coinscription::do_mint</a>(tick_record, amount, ctx);
    public_transfer(<a href="coinscription.md#0x4_coinscription">coinscription</a>, <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx))
}
</code></pre>



</details>

<a name="0x4_inscription_coinscription_transfer"></a>

## Function `coinscription_transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_transfer">coinscription_transfer</a>(scription: <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>, receipt: <b>address</b>, _: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_transfer">coinscription_transfer</a>(scription: CoinScription, receipt: <b>address</b>, _: &<b>mut</b> TxContext) {
    public_transfer(scription, receipt)
}
</code></pre>



</details>

<a name="0x4_inscription_coinscription_split"></a>

## Function `coinscription_split`



<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_split">coinscription_split</a>(scription: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>, amount: u64, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_split">coinscription_split</a>(scription: &<b>mut</b> CoinScription, amount: u64, ctx: &<b>mut</b> TxContext) {
    <b>let</b> scription = <a href="coinscription.md#0x4_coinscription_do_split">coinscription::do_split</a>(scription, amount, ctx);
    public_transfer(scription, <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0x4_inscription_coinscription_merge"></a>

## Function `coinscription_merge`



<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_merge">coinscription_merge</a>(scription1: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>, scription2: <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>, _: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_merge">coinscription_merge</a>(scription1: &<b>mut</b> CoinScription, scription2: CoinScription, _: &<b>mut</b> TxContext) {
    <a href="coinscription.md#0x4_coinscription_do_merge">coinscription::do_merge</a>(scription1, scription2)
}
</code></pre>



</details>

<a name="0x4_inscription_coinscription_burn"></a>

## Function `coinscription_burn`



<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_burn">coinscription_burn</a>(tick_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickRecord">coinscription::TickRecord</a>, <a href="inscription.md#0x4_inscription">inscription</a>: <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_coinscription_burn">coinscription_burn</a>(tick_record: &<b>mut</b> TickRecord,
                                    <a href="inscription.md#0x4_inscription">inscription</a>: CoinScription,
                                    ctx: &<b>mut</b> TxContext) {
    <a href="coinscription.md#0x4_coinscription_do_burn">coinscription::do_burn</a>(tick_record, <a href="inscription.md#0x4_inscription">inscription</a>, ctx)
}
</code></pre>



</details>

<a name="0x4_inscription_singlescription_new_copyright"></a>

## Function `singlescription_new_copyright`



<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_singlescription_new_copyright">singlescription_new_copyright</a>(copyright_pool_record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_CopyrightPoolRecord">singlescription::CopyrightPoolRecord</a>, copr: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_singlescription_new_copyright">singlescription_new_copyright</a>(copyright_pool_record: &<b>mut</b> CopyrightPoolRecord,
                                               copr: String,
                                               ctx: &<b>mut</b> TxContext) {
    <b>let</b> sender = <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>let</b> record = <a href="singlescription.md#0x4_singlescription_new_copright">singlescription::new_copright</a>(copyright_pool_record, copr, ctx);
    public_transfer(record, sender);
}
</code></pre>



</details>

<a name="0x4_inscription_singlescription_mint"></a>

## Function `singlescription_mint`



<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_singlescription_mint">singlescription_mint</a>(record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_SingleScriptionRecord">singlescription::SingleScriptionRecord</a>, name: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, typ: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, sub_typ: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, content: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, link: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_singlescription_mint">singlescription_mint</a>(record: &<b>mut</b> SingleScriptionRecord,
                                      name: String,
                                      typ: String,
                                      sub_typ: String,
                                      content: String,
                                      link: String,
                                      ctx: &<b>mut</b> TxContext) {
    <b>let</b> link_opt = <a href="dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>&lt;String&gt;();
    <b>if</b> (!<a href="dependencies/move-stdlib/string.md#0x1_string_is_empty">string::is_empty</a>(&link)) {
        <a href="dependencies/move-stdlib/option.md#0x1_option_fill">option::fill</a>(&<b>mut</b> link_opt, link);
    };
    <b>let</b> scription = <a href="singlescription.md#0x4_singlescription_do_mint">singlescription::do_mint</a>(record, name, typ, sub_typ, content, link_opt, ctx);
    public_transfer(scription, <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0x4_inscription_singlescription_transfer"></a>

## Function `singlescription_transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_singlescription_transfer">singlescription_transfer</a>(scription: <a href="singlescription.md#0x4_singlescription_SingleScription">singlescription::SingleScription</a>, receipt: <b>address</b>, _: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_singlescription_transfer">singlescription_transfer</a>(scription: SingleScription, receipt: <b>address</b>, _: &<b>mut</b> TxContext) {
    public_transfer(scription, receipt);
}
</code></pre>



</details>

<a name="0x4_inscription_singlescription_burn"></a>

## Function `singlescription_burn`



<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_singlescription_burn">singlescription_burn</a>(record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_SingleScriptionRecord">singlescription::SingleScriptionRecord</a>, scription: <a href="singlescription.md#0x4_singlescription_SingleScription">singlescription::SingleScription</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="inscription.md#0x4_inscription_singlescription_burn">singlescription_burn</a>(record: &<b>mut</b> SingleScriptionRecord,
                                      scription: SingleScription,
                                      ctx: &<b>mut</b> TxContext) {
    <a href="singlescription.md#0x4_singlescription_do_burn">singlescription::do_burn</a>(record, scription, ctx);
}
</code></pre>



</details>
