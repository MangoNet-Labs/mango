
<a name="0x4_singlescription"></a>

# Module `0x4::singlescription`



-  [Resource `CopyrightPoolRecord`](#0x4_singlescription_CopyrightPoolRecord)
-  [Resource `CopyrightCap`](#0x4_singlescription_CopyrightCap)
-  [Resource `SingleScriptionRecord`](#0x4_singlescription_SingleScriptionRecord)
-  [Resource `SingleScription`](#0x4_singlescription_SingleScription)
-  [Struct `SINGLESCRIPTION`](#0x4_singlescription_SINGLESCRIPTION)
-  [Struct `NewCopyright`](#0x4_singlescription_NewCopyright)
-  [Struct `MintSingleScription`](#0x4_singlescription_MintSingleScription)
-  [Struct `BurnSingleScription`](#0x4_singlescription_BurnSingleScription)
-  [Constants](#@Constants_0)
-  [Function `init`](#0x4_singlescription_init)
-  [Function `new_copright`](#0x4_singlescription_new_copright)
-  [Function `find_copyright`](#0x4_singlescription_find_copyright)
-  [Function `copyright_record_address_list`](#0x4_singlescription_copyright_record_address_list)
-  [Function `do_mint`](#0x4_singlescription_do_mint)
-  [Function `do_burn`](#0x4_singlescription_do_burn)


<pre><code><b>use</b> <a href="dependencies/move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="dependencies/move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="dependencies/mgo-framework/display.md#0x2_display">0x2::display</a>;
<b>use</b> <a href="dependencies/mgo-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="dependencies/mgo-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="dependencies/mgo-framework/package.md#0x2_package">0x2::package</a>;
<b>use</b> <a href="dependencies/mgo-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
<b>use</b> <a href="dependencies/mgo-framework/vec_set.md#0x2_vec_set">0x2::vec_set</a>;
<b>use</b> <a href="string_util.md#0x4_string_util">0x4::string_util</a>;
<b>use</b> <a href="svg.md#0x4_svg">0x4::svg</a>;
</code></pre>



<a name="0x4_singlescription_CopyrightPoolRecord"></a>

## Resource `CopyrightPoolRecord`



<pre><code><b>struct</b> <a href="singlescription.md#0x4_singlescription_CopyrightPoolRecord">CopyrightPoolRecord</a> <b>has</b> store, key
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
 The Copyright name -> SingleScriptionRecord object id
</dd>
<dt>
<code>cap_record: <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, <a href="dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_singlescription_CopyrightCap"></a>

## Resource `CopyrightCap`



<pre><code><b>struct</b> <a href="singlescription.md#0x4_singlescription_CopyrightCap">CopyrightCap</a> <b>has</b> store, key
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
<code>copyright: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_singlescription_SingleScriptionRecord"></a>

## Resource `SingleScriptionRecord`



<pre><code><b>struct</b> <a href="singlescription.md#0x4_singlescription_SingleScriptionRecord">SingleScriptionRecord</a> <b>has</b> store, key
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
<code>copyright: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>record: <a href="dependencies/mgo-framework/vec_set.md#0x2_vec_set_VecSet">vec_set::VecSet</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_singlescription_SingleScription"></a>

## Resource `SingleScription`



<pre><code><b>struct</b> <a href="singlescription.md#0x4_singlescription_SingleScription">SingleScription</a> <b>has</b> store, key
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
<code>name: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>typ: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>sub_typ: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>copyright: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>content: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>link: <a href="dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_singlescription_SINGLESCRIPTION"></a>

## Struct `SINGLESCRIPTION`



<pre><code><b>struct</b> <a href="singlescription.md#0x4_singlescription_SINGLESCRIPTION">SINGLESCRIPTION</a> <b>has</b> drop
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

<a name="0x4_singlescription_NewCopyright"></a>

## Struct `NewCopyright`



<pre><code><b>struct</b> <a href="singlescription.md#0x4_singlescription_NewCopyright">NewCopyright</a> <b>has</b> <b>copy</b>, drop
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
<code>copyright: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_singlescription_MintSingleScription"></a>

## Struct `MintSingleScription`



<pre><code><b>struct</b> <a href="singlescription.md#0x4_singlescription_MintSingleScription">MintSingleScription</a> <b>has</b> <b>copy</b>, drop
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
<code>name: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>copyright: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x4_singlescription_BurnSingleScription"></a>

## Struct `BurnSingleScription`



<pre><code><b>struct</b> <a href="singlescription.md#0x4_singlescription_BurnSingleScription">BurnSingleScription</a> <b>has</b> <b>copy</b>, drop
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
<code>copyright: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x4_singlescription_ErrorCoprAlreadyExists"></a>



<pre><code><b>const</b> <a href="singlescription.md#0x4_singlescription_ErrorCoprAlreadyExists">ErrorCoprAlreadyExists</a>: u64 = 2;
</code></pre>



<a name="0x4_singlescription_ErrorCoprAlreadyExistsByCap"></a>



<pre><code><b>const</b> <a href="singlescription.md#0x4_singlescription_ErrorCoprAlreadyExistsByCap">ErrorCoprAlreadyExistsByCap</a>: u64 = 1;
</code></pre>



<a name="0x4_singlescription_ErrorInvalidCopyright"></a>



<pre><code><b>const</b> <a href="singlescription.md#0x4_singlescription_ErrorInvalidCopyright">ErrorInvalidCopyright</a>: u64 = 3;
</code></pre>



<a name="0x4_singlescription_init"></a>

## Function `init`



<pre><code><b>fun</b> <a href="singlescription.md#0x4_singlescription_init">init</a>(otw: <a href="singlescription.md#0x4_singlescription_SINGLESCRIPTION">singlescription::SINGLESCRIPTION</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="singlescription.md#0x4_singlescription_init">init</a>(otw: <a href="singlescription.md#0x4_singlescription_SINGLESCRIPTION">SINGLESCRIPTION</a>, ctx: &<b>mut</b> TxContext) {
    <b>let</b> copyright_pool_record = <a href="singlescription.md#0x4_singlescription_CopyrightPoolRecord">CopyrightPoolRecord</a> {
        id: <a href="dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx), record: <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(), cap_record: <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>()
    };
    <a href="dependencies/mgo-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(copyright_pool_record);

    <b>let</b> keys = <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[
        std::string::utf8(b"name"),
        std::string::utf8(b"image_url"),
    ];

    <b>let</b> p = b"mrc-721";
    <b>let</b> na = b"{name}";
    <b>let</b> typ = b"{type}";
    <b>let</b> copr = b"{copyright}";

    <b>let</b> img_metadata = <a href="svg.md#0x4_svg_generate_singlescription_svg">svg::generate_singlescription_svg</a>(p, na, typ, copr);

    <b>let</b> values = <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[
        std::string::utf8(b"{name}"),
        std::string::utf8(img_metadata),
    ];
    <b>let</b> publisher = <a href="dependencies/mgo-framework/package.md#0x2_package_claim">package::claim</a>(otw, ctx);
    <b>let</b> <a href="dependencies/mgo-framework/display.md#0x2_display">display</a> = <a href="dependencies/mgo-framework/display.md#0x2_display_new_with_fields">display::new_with_fields</a>&lt;<a href="singlescription.md#0x4_singlescription_SingleScription">SingleScription</a>&gt;(
        &publisher, keys, values, ctx
    );
    <a href="dependencies/mgo-framework/display.md#0x2_display_update_version">display::update_version</a>(&<b>mut</b> <a href="dependencies/mgo-framework/display.md#0x2_display">display</a>);
    <a href="dependencies/mgo-framework/package.md#0x2_package_burn_publisher">package::burn_publisher</a>(publisher);
    <a href="dependencies/mgo-framework/transfer.md#0x2_transfer_public_share_object">transfer::public_share_object</a>(<a href="dependencies/mgo-framework/display.md#0x2_display">display</a>);
}
</code></pre>



</details>

<a name="0x4_singlescription_new_copright"></a>

## Function `new_copright`



<pre><code><b>public</b> <b>fun</b> <a href="singlescription.md#0x4_singlescription_new_copright">new_copright</a>(copyright_pool_record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_CopyrightPoolRecord">singlescription::CopyrightPoolRecord</a>, copr: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): (<a href="singlescription.md#0x4_singlescription_CopyrightCap">singlescription::CopyrightCap</a>, <a href="singlescription.md#0x4_singlescription_SingleScriptionRecord">singlescription::SingleScriptionRecord</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="singlescription.md#0x4_singlescription_new_copright">new_copright</a>(
    copyright_pool_record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_CopyrightPoolRecord">CopyrightPoolRecord</a>,
    copr: String,
    ctx: &<b>mut</b> TxContext
): (<a href="singlescription.md#0x4_singlescription_CopyrightCap">CopyrightCap</a>, <a href="singlescription.md#0x4_singlescription_SingleScriptionRecord">SingleScriptionRecord</a>) {
    <b>assert</b>!(!<a href="string_util.md#0x4_string_util_is_empty_str">string_util::is_empty_str</a>(&copr), <a href="singlescription.md#0x4_singlescription_ErrorInvalidCopyright">ErrorInvalidCopyright</a>);
    <b>assert</b>!(!<a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&copyright_pool_record.cap_record, &copr), <a href="singlescription.md#0x4_singlescription_ErrorCoprAlreadyExistsByCap">ErrorCoprAlreadyExistsByCap</a>);
    <b>assert</b>!(!<a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&copyright_pool_record.record, &copr), <a href="singlescription.md#0x4_singlescription_ErrorCoprAlreadyExists">ErrorCoprAlreadyExists</a>);

    <b>let</b> copr_uid = <a href="dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx);
    <b>let</b> copr_id = <a href="dependencies/mgo-framework/object.md#0x2_object_uid_to_inner">object::uid_to_inner</a>(&copr_uid);
    <b>let</b> copr_cap = <a href="singlescription.md#0x4_singlescription_CopyrightCap">CopyrightCap</a> {
        id: copr_uid,
        copyright: copr
    };

    <b>let</b> scription_uid = <a href="dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx);
    <b>let</b> scription_id = uid_to_inner(&scription_uid);
    <b>let</b> scription_record = <a href="singlescription.md#0x4_singlescription_SingleScriptionRecord">SingleScriptionRecord</a> {
        id: scription_uid,
        copyright: copr,
        record: <a href="dependencies/mgo-framework/vec_set.md#0x2_vec_set_empty">vec_set::empty</a>(),
    };
    <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> copyright_pool_record.record, copr, scription_id);
    <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> copyright_pool_record.cap_record, copr, copr_id);
    emit(<a href="singlescription.md#0x4_singlescription_NewCopyright">NewCopyright</a> {
        id: copr_id,
        deployer: <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
        copyright: copr,
    });
    (copr_cap, scription_record)
}
</code></pre>



</details>

<a name="0x4_singlescription_find_copyright"></a>

## Function `find_copyright`



<pre><code><b>public</b> <b>fun</b> <a href="singlescription.md#0x4_singlescription_find_copyright">find_copyright</a>(copyright_pool_record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_CopyrightPoolRecord">singlescription::CopyrightPoolRecord</a>, copr: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>): (<a href="dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;, <a href="dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="singlescription.md#0x4_singlescription_find_copyright">find_copyright</a>(
    copyright_pool_record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_CopyrightPoolRecord">CopyrightPoolRecord</a>,
    copr: String
): (Option&lt;<b>address</b>&gt;, Option&lt;<b>address</b>&gt;) {
    <b>let</b> record_addr = <a href="dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>&lt;<b>address</b>&gt;();
    <b>let</b> copr_cap_addr = <a href="dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>&lt;<b>address</b>&gt;();
    <b>if</b> (<a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_contains">vec_map::contains</a>(&copyright_pool_record.record, &copr)) {
        <b>let</b> id_record = <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_try_get">vec_map::try_get</a>(&copyright_pool_record.record, &copr);
        <b>let</b> id_copr_cap = <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_try_get">vec_map::try_get</a>(&copyright_pool_record.cap_record, &copr);
        record_addr = <a href="dependencies/move-stdlib/option.md#0x1_option_some">option::some</a>(id_to_address(<a href="dependencies/move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&id_record)));
        copr_cap_addr = <a href="dependencies/move-stdlib/option.md#0x1_option_some">option::some</a>(id_to_address(<a href="dependencies/move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&id_copr_cap)));
    };
    (record_addr, copr_cap_addr)
}
</code></pre>



</details>

<a name="0x4_singlescription_copyright_record_address_list"></a>

## Function `copyright_record_address_list`



<pre><code><b>public</b> <b>fun</b> <a href="singlescription.md#0x4_singlescription_copyright_record_address_list">copyright_record_address_list</a>(copyright_pool_record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_CopyrightPoolRecord">singlescription::CopyrightPoolRecord</a>, offset: u64, limit: u64): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="singlescription.md#0x4_singlescription_copyright_record_address_list">copyright_record_address_list</a>(
    copyright_pool_record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_CopyrightPoolRecord">CopyrightPoolRecord</a>,
    offset: u64,
    limit: u64
): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt; {
    <b>let</b> vec_addrs = <a href="dependencies/move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;<b>address</b>&gt;();

    <b>let</b> keys = <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_keys">vec_map::keys</a>(&copyright_pool_record.record);
    <b>let</b> len = <a href="dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&keys);
    <b>if</b> (len &gt; offset) {
        <b>let</b> i = offset;
        <b>let</b> end = len;
        <b>if</b> (offset + limit &lt; len) {
            end = offset + limit;
        };
        <b>while</b> (i &lt; end) {
            <b>let</b> key = <a href="dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&keys, i);
            <b>let</b> value = <a href="dependencies/mgo-framework/vec_map.md#0x2_vec_map_try_get">vec_map::try_get</a>(&copyright_pool_record.record, key);
            <b>let</b> addr = id_to_address(<a href="dependencies/move-stdlib/option.md#0x1_option_borrow">option::borrow</a>(&value));
            <a href="dependencies/move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> vec_addrs, addr);
            i = i + 1;
        };
    };

    vec_addrs
}
</code></pre>



</details>

<a name="0x4_singlescription_do_mint"></a>

## Function `do_mint`



<pre><code><b>public</b> <b>fun</b> <a href="singlescription.md#0x4_singlescription_do_mint">do_mint</a>(cap: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_CopyrightCap">singlescription::CopyrightCap</a>, record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_SingleScriptionRecord">singlescription::SingleScriptionRecord</a>, name: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, typ: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, sub_typ: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, content: <a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, link: <a href="dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;<a href="dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>&gt;, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="singlescription.md#0x4_singlescription_SingleScription">singlescription::SingleScription</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="singlescription.md#0x4_singlescription_do_mint">do_mint</a>(
    cap: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_CopyrightCap">CopyrightCap</a>,
    record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_SingleScriptionRecord">SingleScriptionRecord</a>,
    name: String,
    typ: String,
    sub_typ: String,
    content: String,
    link: Option&lt;String&gt;,
    ctx: &<b>mut</b> TxContext
): <a href="singlescription.md#0x4_singlescription_SingleScription">SingleScription</a> {
    <b>assert</b>!(cap.copyright == record.copyright, <a href="singlescription.md#0x4_singlescription_ErrorInvalidCopyright">ErrorInvalidCopyright</a>);
    <b>let</b> scription_uid = <a href="dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx);
    <b>let</b> scription_id = <a href="dependencies/mgo-framework/object.md#0x2_object_uid_to_inner">object::uid_to_inner</a>(&scription_uid);

    <b>let</b> scription = <a href="singlescription.md#0x4_singlescription_SingleScription">SingleScription</a> {
        id: scription_uid,
        name,
        typ,
        sub_typ,
        content,
        copyright: cap.copyright,
        link
    };

    <b>let</b> scription_address = <a href="dependencies/mgo-framework/object.md#0x2_object_id_address">object::id_address</a>(&scription);
    <a href="dependencies/mgo-framework/vec_set.md#0x2_vec_set_insert">vec_set::insert</a>(&<b>mut</b> record.record, scription_address);
    emit(<a href="singlescription.md#0x4_singlescription_MintSingleScription">MintSingleScription</a> {
        id: scription_id,
        sender: <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
        name,
        copyright: cap.copyright
    });
    scription
}
</code></pre>



</details>

<a name="0x4_singlescription_do_burn"></a>

## Function `do_burn`



<pre><code><b>public</b> <b>fun</b> <a href="singlescription.md#0x4_singlescription_do_burn">do_burn</a>(record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_SingleScriptionRecord">singlescription::SingleScriptionRecord</a>, scription: <a href="singlescription.md#0x4_singlescription_SingleScription">singlescription::SingleScription</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="singlescription.md#0x4_singlescription_do_burn">do_burn</a>(
    record: &<b>mut</b> <a href="singlescription.md#0x4_singlescription_SingleScriptionRecord">SingleScriptionRecord</a>,
    scription: <a href="singlescription.md#0x4_singlescription_SingleScription">SingleScription</a>,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(record.copyright == scription.copyright, <a href="singlescription.md#0x4_singlescription_ErrorInvalidCopyright">ErrorInvalidCopyright</a>);

    <b>let</b> <b>address</b> = <a href="dependencies/mgo-framework/object.md#0x2_object_id_address">object::id_address</a>(&scription);
    <b>let</b> <a href="singlescription.md#0x4_singlescription_SingleScription">SingleScription</a> { id: uid, name: _, typ: _, sub_typ: _, copyright, content: _, link: _ } = scription;
    <b>let</b> id = <a href="dependencies/mgo-framework/object.md#0x2_object_uid_to_inner">object::uid_to_inner</a>(&uid);

    <a href="dependencies/mgo-framework/vec_set.md#0x2_vec_set_remove">vec_set::remove</a>(&<b>mut</b> record.record, &<b>address</b>);
    <a href="dependencies/mgo-framework/object.md#0x2_object_delete">object::delete</a>(uid);

    emit({
        <a href="singlescription.md#0x4_singlescription_BurnSingleScription">BurnSingleScription</a> {
            id,
            sender: <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx),
            copyright,
        }
    });
}
</code></pre>



</details>
