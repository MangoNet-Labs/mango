
<a name="0x4_coinscription"></a>

# Module `0x4::coinscription`



-  [Resource `TickPoolRecord`](#0x4_coinscription_TickPoolRecord)
-  [Resource `TickRecord`](#0x4_coinscription_TickRecord)
-  [Resource `CoinScription`](#0x4_coinscription_CoinScription)
-  [Struct `COINSCRIPTION`](#0x4_coinscription_COINSCRIPTION)
-  [Struct `NewTick`](#0x4_coinscription_NewTick)
-  [Struct `MintCoinScription`](#0x4_coinscription_MintCoinScription)
-  [Struct `BurnCoinScription`](#0x4_coinscription_BurnCoinScription)
-  [Constants](#@Constants_0)
-  [Function `init`](#0x4_coinscription_init)
-  [Function `new_tick`](#0x4_coinscription_new_tick)
-  [Function `find_tick`](#0x4_coinscription_find_tick)
-  [Function `tick_record_address_list`](#0x4_coinscription_tick_record_address_list)
-  [Function `do_mint`](#0x4_coinscription_do_mint)
-  [Function `is_mergeable`](#0x4_coinscription_is_mergeable)
-  [Function `do_merge`](#0x4_coinscription_do_merge)
-  [Function `is_splitable`](#0x4_coinscription_is_splitable)
-  [Function `do_split`](#0x4_coinscription_do_split)
-  [Function `split`](#0x4_coinscription_split)
-  [Function `zero`](#0x4_coinscription_zero)
-  [Function `is_zero`](#0x4_coinscription_is_zero)
-  [Function `destroy_zero`](#0x4_coinscription_destroy_zero)
-  [Function `do_burn`](#0x4_coinscription_do_burn)


<pre><code><b>use</b> <a href="dependencies/move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="dependencies/move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="dependencies/mgo-framework/display.md#0x2_display">0x2::display</a>;
<b>use</b> <a href="dependencies/mgo-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="dependencies/mgo-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="dependencies/mgo-framework/package.md#0x2_package">0x2::package</a>;
<b>use</b> <a href="dependencies/mgo-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="string_util.md#0x4_string_util">0x4::string_util</a>;
<b>use</b> <a href="svg.md#0x4_svg">0x4::svg</a>;
</code></pre>



<a name="0x4_coinscription_TickPoolRecord"></a>

## Resource `TickPoolRecord`



<pre><code><b>struct</b> <a href="coinscription.md#0x4_coinscription_TickPoolRecord">TickPoolRecord</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="dependencies/mgo-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>record: <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, <a href="dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>
 The Tick name -> TickRecord object id
</dd>
</dl>


</details>

<a name="0x4_coinscription_TickRecord"></a>

## Resource `TickRecord`



<pre><code><b>struct</b> <a href="coinscription.md#0x4_coinscription_TickRecord">TickRecord</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="dependencies/mgo-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>tick: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>total_supply: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>burnable: bool</code>
</dt>
<dd>

</dd>
<dt>
<code>remain: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>current_supply: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_coinscription_CoinScription"></a>

## Resource `CoinScription`



<pre><code><b>struct</b> <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a> <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="dependencies/mgo-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>tick: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_coinscription_COINSCRIPTION"></a>

## Struct `COINSCRIPTION`



<pre><code><b>struct</b> <a href="coinscription.md#0x4_coinscription_COINSCRIPTION">COINSCRIPTION</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>dummy_field: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_coinscription_NewTick"></a>

## Struct `NewTick`



<pre><code><b>struct</b> <a href="coinscription.md#0x4_coinscription_NewTick">NewTick</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>deployer: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>tick: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>total_supply: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>burnable: bool</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_coinscription_MintCoinScription"></a>

## Struct `MintCoinScription`



<pre><code><b>struct</b> <a href="coinscription.md#0x4_coinscription_MintCoinScription">MintCoinScription</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>tick: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_coinscription_BurnCoinScription"></a>

## Struct `BurnCoinScription`



<pre><code><b>struct</b> <a href="coinscription.md#0x4_coinscription_BurnCoinScription">BurnCoinScription</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>

</dd>
<dt>
<code>tick: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x4_coinscription_EInvalidAmount"></a>



<pre><code><b>const</b> <a href="coinscription.md#0x4_coinscription_EInvalidAmount">EInvalidAmount</a>: u64 = 4;
</code></pre>



<a name="0x4_coinscription_ErrorCannotBurn"></a>



<pre><code><b>const</b> <a href="coinscription.md#0x4_coinscription_ErrorCannotBurn">ErrorCannotBurn</a>: u64 = 6;
</code></pre>



<a name="0x4_coinscription_ErrorInvalidTick"></a>



<pre><code><b>const</b> <a href="coinscription.md#0x4_coinscription_ErrorInvalidTick">ErrorInvalidTick</a>: u64 = 8;
</code></pre>



<a name="0x4_coinscription_ErrorNotEnoughSupply"></a>



<pre><code><b>const</b> <a href="coinscription.md#0x4_coinscription_ErrorNotEnoughSupply">ErrorNotEnoughSupply</a>: u64 = 2;
</code></pre>



<a name="0x4_coinscription_ErrorNotEnoughToMint"></a>



<pre><code><b>const</b> <a href="coinscription.md#0x4_coinscription_ErrorNotEnoughToMint">ErrorNotEnoughToMint</a>: u64 = 3;
</code></pre>



<a name="0x4_coinscription_ErrorNotSameTick"></a>



<pre><code><b>const</b> <a href="coinscription.md#0x4_coinscription_ErrorNotSameTick">ErrorNotSameTick</a>: u64 = 5;
</code></pre>



<a name="0x4_coinscription_ErrorNotZero"></a>



<pre><code><b>const</b> <a href="coinscription.md#0x4_coinscription_ErrorNotZero">ErrorNotZero</a>: u64 = 7;
</code></pre>



<a name="0x4_coinscription_ErrorTickAlreadyExists"></a>



<pre><code><b>const</b> <a href="coinscription.md#0x4_coinscription_ErrorTickAlreadyExists">ErrorTickAlreadyExists</a>: u64 = 1;
</code></pre>



<a name="0x4_coinscription_init"></a>

## Function `init`



<pre><code><b>fun</b> <a href="coinscription.md#0x4_coinscription_init">init</a>(otw: <a href="coinscription.md#0x4_coinscription_COINSCRIPTION">coinscription::COINSCRIPTION</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="coinscription.md#0x4_coinscription_init">init</a>(otw: <a href="coinscription.md#0x4_coinscription_COINSCRIPTION">COINSCRIPTION</a>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> tick_pool_record = <a href="coinscription.md#0x4_coinscription_TickPoolRecord">TickPoolRecord</a> { id: <a href="dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx), record: <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>() };
    <a href="dependencies/mgo-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(tick_pool_record);

    <b>let</b> keys = <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[
        std::string::utf8(b"tick"),
        std::string::utf8(b"amount"),
        std::string::utf8(b"image_url"),
    ];

    <b>let</b> p = b"mrc-20";
    <b>let</b> op = b"mint";
    <b>let</b> tick = b"{tick}";
    <b>let</b> amt = b"{amount}";

    <b>let</b> img_metadata = <a href="svg.md#0x4_svg_generate_coinscription_svg">svg::generate_coinscription_svg</a>(p, op, tick, amt);

    <b>let</b> values = <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[
        std::string::utf8(b"{tick}"),
        std::string::utf8(b"{amount}"),
        std::string::utf8(img_metadata),
    ];
    <b>let</b> publisher = <a href="dependencies/mgo-framework/package.md#0x2_package_claim">package::claim</a>(otw, ctx);
    <b>let</b> <a href="dependencies/mgo-framework/display.md#0x2_display">display</a> = <a href="dependencies/mgo-framework/display.md#0x2_display_new_with_fields">display::new_with_fields</a>&lt;<a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>&gt;(
        &publisher, keys, values, ctx
    );
    <a href="dependencies/mgo-framework/display.md#0x2_display_update_version">display::update_version</a>(&<b>mut</b> <a href="dependencies/mgo-framework/display.md#0x2_display">display</a>);
    <a href="dependencies/mgo-framework/package.md#0x2_package_burn_publisher">package::burn_publisher</a>(publisher);
    <a href="dependencies/mgo-framework/transfer.md#0x2_transfer_public_share_object">transfer::public_share_object</a>(<a href="dependencies/mgo-framework/display.md#0x2_display">display</a>);
}
</code></pre>



</details>

<a name="0x4_coinscription_new_tick"></a>

## Function `new_tick`



<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_new_tick">new_tick</a>(tick_pool_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickPoolRecord">coinscription::TickPoolRecord</a>, tick: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, total_supply: u64, burnable: bool, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="coinscription.md#0x4_coinscription_TickRecord">coinscription::TickRecord</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_new_tick">new_tick</a>(
    tick_pool_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickPoolRecord">TickPoolRecord</a>,
    tick: String,
    total_supply: u64,
    burnable: bool,
    ctx: &<b>mut</b> TxContext
): <a href="coinscription.md#0x4_coinscription_TickRecord">TickRecord</a> {
    <b>assert</b>!(<a href="string_util.md#0x4_string_util_is_tick_valid">string_util::is_tick_valid</a>(&tick), <a href="coinscription.md#0x4_coinscription_ErrorInvalidTick">ErrorInvalidTick</a>);
    <b>assert</b>!(!<a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&tick_pool_record.record, &tick), <a href="coinscription.md#0x4_coinscription_ErrorTickAlreadyExists">ErrorTickAlreadyExists</a>);
    <b>assert</b>!(total_supply &gt; 0, <a href="coinscription.md#0x4_coinscription_ErrorNotEnoughSupply">ErrorNotEnoughSupply</a>);

    <b>let</b> tick_uid = <a href="dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx);
    <b>let</b> tick_id = <a href="dependencies/mgo-framework/object.md#0x2_object_uid_to_inner">object::uid_to_inner</a>(&tick_uid);
    <b>let</b> tick_record: <a href="coinscription.md#0x4_coinscription_TickRecord">TickRecord</a> = <a href="coinscription.md#0x4_coinscription_TickRecord">TickRecord</a> {
        id: tick_uid,
        tick,
        total_supply,
        burnable,
        remain: total_supply,
        current_supply: 0,
    };
    <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> tick_pool_record.record, tick, tick_id);
    emit(<a href="coinscription.md#0x4_coinscription_NewTick">NewTick</a> {
        id: tick_id,
        deployer: <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
        tick,
        total_supply,
        burnable
    });
    tick_record
}
</code></pre>



</details>

<a name="0x4_coinscription_find_tick"></a>

## Function `find_tick`



<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_find_tick">find_tick</a>(tick_pool_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickPoolRecord">coinscription::TickPoolRecord</a>, tick: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>): <a href="dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_find_tick">find_tick</a>(tick_pool_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickPoolRecord">TickPoolRecord</a>, tick: String): Option&lt;<b>address</b>&gt; {
    <b>if</b> (<a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&tick_pool_record.record, &tick)) {
        <b>let</b> id_tick = <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_try_get">vec_map::try_get</a>(&tick_pool_record.record, &tick);
        <a href="dependencies/move-stdlib/option.md#0x1_option_some">option::some</a>(id_to_address(<a href="dependencies/move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&id_tick)))
    } <b>else</b> {
        <a href="dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>()
    }
}
</code></pre>



</details>

<a name="0x4_coinscription_tick_record_address_list"></a>

## Function `tick_record_address_list`



<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_tick_record_address_list">tick_record_address_list</a>(tick_pool_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickPoolRecord">coinscription::TickPoolRecord</a>, offset: u64, limit: u64): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_tick_record_address_list">tick_record_address_list</a>(
    tick_pool_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickPoolRecord">TickPoolRecord</a>,
    offset: u64,
    limit: u64
): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt; {
    <b>let</b> vec_addrs = <a href="dependencies/move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;<b>address</b>&gt;();

    <b>let</b> keys = <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_keys">vec_map::keys</a>(&tick_pool_record.record);
    <b>let</b> len = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&keys);
    <b>if</b> (len &gt; offset) {
        <b>let</b> i = offset;
        <b>let</b> end = len;
        <b>if</b> (offset + limit &lt; len) {
            end = offset + limit;
        };
        <b>while</b> (i &lt; end) {
            <b>let</b> key = <a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&keys, i);
            <b>let</b> value = <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_try_get">vec_map::try_get</a>(&tick_pool_record.record, key);
            <b>let</b> addr = id_to_address(<a href="dependencies/move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&value));
            <a href="dependencies/move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> vec_addrs, addr);
            i = i + 1;
        };
    };

    vec_addrs
}
</code></pre>



</details>

<a name="0x4_coinscription_do_mint"></a>

## Function `do_mint`



<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_do_mint">do_mint</a>(tick_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickRecord">coinscription::TickRecord</a>, amount: u64, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_do_mint">do_mint</a>(
    tick_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickRecord">TickRecord</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext
): <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a> {
    <b>assert</b>!(tick_record.remain &gt; 0, <a href="coinscription.md#0x4_coinscription_ErrorNotEnoughToMint">ErrorNotEnoughToMint</a>);
    <b>assert</b>!(tick_record.remain &gt;= amount, <a href="coinscription.md#0x4_coinscription_ErrorNotEnoughToMint">ErrorNotEnoughToMint</a>);

    tick_record.remain = tick_record.remain - amount;
    tick_record.current_supply = tick_record.current_supply + amount;

    <b>let</b> tick: String = tick_record.tick;
    <b>let</b> sender = <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>let</b> <a href="coinscription.md#0x4_coinscription">coinscription</a> = <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a> {
        id: <a href="dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx),
        amount,
        tick,
    };
    emit(<a href="coinscription.md#0x4_coinscription_MintCoinScription">MintCoinScription</a> {
        id: <a href="dependencies/mgo-framework/object.md#0x2_object_id">object::id</a>(&<a href="coinscription.md#0x4_coinscription">coinscription</a>),
        sender,
        tick,
        amount,
    });
    <a href="coinscription.md#0x4_coinscription">coinscription</a>
}
</code></pre>



</details>

<a name="0x4_coinscription_is_mergeable"></a>

## Function `is_mergeable`



<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_is_mergeable">is_mergeable</a>(inscription1: &<a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>, inscription2: &<a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_is_mergeable">is_mergeable</a>(inscription1: &<a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>, inscription2: &<a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>): bool {
    inscription1.tick == inscription2.tick
}
</code></pre>



</details>

<a name="0x4_coinscription_do_merge"></a>

## Function `do_merge`



<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_do_merge">do_merge</a>(inscription1: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>, inscription2: <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_do_merge">do_merge</a>(
    inscription1: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>,
    inscription2: <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>,
) {
    <b>assert</b>!(inscription1.tick == inscription2.tick, <a href="coinscription.md#0x4_coinscription_ErrorNotSameTick">ErrorNotSameTick</a>);
    <b>let</b> <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a> { id, amount, tick: _ } = inscription2;
    inscription1.amount = inscription1.amount + amount;
    <a href="dependencies/mgo-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>

<a name="0x4_coinscription_is_splitable"></a>

## Function `is_splitable`

Check if the inscription can be split


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_is_splitable">is_splitable</a>(<a href="inscription.md#0x4_inscription">inscription</a>: &<a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_is_splitable">is_splitable</a>(<a href="inscription.md#0x4_inscription">inscription</a>: &<a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>): bool {
    <a href="inscription.md#0x4_inscription">inscription</a>.amount &gt; 1
}
</code></pre>



</details>

<a name="0x4_coinscription_do_split"></a>

## Function `do_split`

Split the inscription and return the new inscription


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_do_split">do_split</a>(<a href="inscription.md#0x4_inscription">inscription</a>: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>, amount: u64, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_do_split">do_split</a>(
    <a href="inscription.md#0x4_inscription">inscription</a>: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext
): <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a> {
    <b>assert</b>!(0 &lt; amount && amount &lt; <a href="inscription.md#0x4_inscription">inscription</a>.amount, <a href="coinscription.md#0x4_coinscription_EInvalidAmount">EInvalidAmount</a>);
    <b>let</b> original_amount = <a href="inscription.md#0x4_inscription">inscription</a>.amount;
    <a href="inscription.md#0x4_inscription">inscription</a>.amount = original_amount - amount;
    <b>let</b> split_movescription = <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a> {
        id: <a href="dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx),
        amount,
        tick: <a href="inscription.md#0x4_inscription">inscription</a>.tick,
    };
    split_movescription
}
</code></pre>



</details>

<a name="0x4_coinscription_split"></a>

## Function `split`



<pre><code><b>public</b> entry <b>fun</b> <a href="coinscription.md#0x4_coinscription_split">split</a>(<a href="inscription.md#0x4_inscription">inscription</a>: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>, amount: u64, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="coinscription.md#0x4_coinscription_split">split</a>(
    <a href="inscription.md#0x4_inscription">inscription</a>: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> ins = <a href="coinscription.md#0x4_coinscription_do_split">do_split</a>(<a href="inscription.md#0x4_inscription">inscription</a>, amount, ctx);
    <a href="dependencies/mgo-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(ins, <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0x4_coinscription_zero"></a>

## Function `zero`



<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_zero">zero</a>(tick_record: &<a href="coinscription.md#0x4_coinscription_TickRecord">coinscription::TickRecord</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_zero">zero</a>(tick_record: &<a href="coinscription.md#0x4_coinscription_TickRecord">TickRecord</a>, ctx: &<b>mut</b> TxContext): <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a> {
    <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a> {
        id: <a href="dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx),
        tick: tick_record.tick,
        amount: 0
    }
}
</code></pre>



</details>

<a name="0x4_coinscription_is_zero"></a>

## Function `is_zero`



<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_is_zero">is_zero</a>(self: &<a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_is_zero">is_zero</a>(self: &<a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>): bool {
    self.amount == 0
}
</code></pre>



</details>

<a name="0x4_coinscription_destroy_zero"></a>

## Function `destroy_zero`



<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_destroy_zero">destroy_zero</a>(self: <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="coinscription.md#0x4_coinscription_destroy_zero">destroy_zero</a>(self: <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>) {
    <b>assert</b>!(self.amount == 0, <a href="coinscription.md#0x4_coinscription_ErrorNotZero">ErrorNotZero</a>);
    <b>let</b> <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a> { id, amount: _, tick: _ } = self;
    <a href="dependencies/mgo-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>

<a name="0x4_coinscription_do_burn"></a>

## Function `do_burn`



<pre><code><b>public</b> entry <b>fun</b> <a href="coinscription.md#0x4_coinscription_do_burn">do_burn</a>(tick_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickRecord">coinscription::TickRecord</a>, <a href="inscription.md#0x4_inscription">inscription</a>: <a href="coinscription.md#0x4_coinscription_CoinScription">coinscription::CoinScription</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="coinscription.md#0x4_coinscription_do_burn">do_burn</a>(
    tick_record: &<b>mut</b> <a href="coinscription.md#0x4_coinscription_TickRecord">TickRecord</a>,
    <a href="inscription.md#0x4_inscription">inscription</a>: <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(tick_record.tick == <a href="inscription.md#0x4_inscription">inscription</a>.tick, <a href="coinscription.md#0x4_coinscription_ErrorNotSameTick">ErrorNotSameTick</a>);
    <b>assert</b>!(tick_record.burnable, <a href="coinscription.md#0x4_coinscription_ErrorCannotBurn">ErrorCannotBurn</a>);
    <b>let</b> sender = <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx);
    <b>let</b> <a href="coinscription.md#0x4_coinscription_CoinScription">CoinScription</a> { id: scription_uid, amount, tick } = <a href="inscription.md#0x4_inscription">inscription</a>;
    tick_record.current_supply = tick_record.current_supply - amount;
    <a href="dependencies/mgo-framework/object.md#0x2_object_delete">object::delete</a>(scription_uid);

    emit({
        <a href="coinscription.md#0x4_coinscription_BurnCoinScription">BurnCoinScription</a> {
            sender,
            tick,
            amount,
        }
    });
}
</code></pre>



</details>
