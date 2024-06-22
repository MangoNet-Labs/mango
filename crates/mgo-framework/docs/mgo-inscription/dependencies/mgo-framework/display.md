
<a name="0x2_display"></a>

# Module `0x2::display`



-  [Resource `Display`](#0x2_display_Display)
-  [Struct `DisplayCreated`](#0x2_display_DisplayCreated)
-  [Struct `VersionUpdated`](#0x2_display_VersionUpdated)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x2_display_new)
-  [Function `new_with_fields`](#0x2_display_new_with_fields)
-  [Function `create_and_keep`](#0x2_display_create_and_keep)
-  [Function `update_version`](#0x2_display_update_version)
-  [Function `add`](#0x2_display_add)
-  [Function `add_multiple`](#0x2_display_add_multiple)
-  [Function `edit`](#0x2_display_edit)
-  [Function `remove`](#0x2_display_remove)
-  [Function `is_authorized`](#0x2_display_is_authorized)
-  [Function `version`](#0x2_display_version)
-  [Function `fields`](#0x2_display_fields)
-  [Function `create_internal`](#0x2_display_create_internal)
-  [Function `add_internal`](#0x2_display_add_internal)


<pre><code><b>use</b> <a href="../../dependencies/move-stdlib/string.md#0x1_string">0x1::string</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/event.md#0x2_event">0x2::event</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package">0x2::package</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/vec_map.md#0x2_vec_map">0x2::vec_map</a>;
</code></pre>



<a name="0x2_display_Display"></a>

## Resource `Display`



<pre><code><b>struct</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T: key&gt; <b>has</b> store, key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../../dependencies/mgo-framework/object.md#0x2_object_UID">object::UID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>fields: <a href="../../dependencies/mgo-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, <a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>version: u16</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_display_DisplayCreated"></a>

## Struct `DisplayCreated`



<pre><code><b>struct</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_DisplayCreated">DisplayCreated</a>&lt;T: key&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_display_VersionUpdated"></a>

## Struct `VersionUpdated`



<pre><code><b>struct</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_VersionUpdated">VersionUpdated</a>&lt;T: key&gt; <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>version: u16</code>
</dt>
<dd>

</dd>
<dt>
<code>fields: <a href="../../dependencies/mgo-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, <a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x2_display_ENotOwner"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_ENotOwner">ENotOwner</a>: u64 = 0;
</code></pre>



<a name="0x2_display_EVecLengthMismatch"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_EVecLengthMismatch">EVecLengthMismatch</a>: u64 = 1;
</code></pre>



<a name="0x2_display_new"></a>

## Function `new`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_new">new</a>&lt;T: key&gt;(pub: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">package::Publisher</a>, ctx: &<b>mut</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_new">new</a>&lt;T: key&gt;(pub: &Publisher, ctx: &<b>mut</b> TxContext): <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt; {
    <b>assert</b>!(<a href="../../dependencies/mgo-framework/display.md#0x2_display_is_authorized">is_authorized</a>&lt;T&gt;(pub), <a href="../../dependencies/mgo-framework/display.md#0x2_display_ENotOwner">ENotOwner</a>);
    <a href="../../dependencies/mgo-framework/display.md#0x2_display_create_internal">create_internal</a>(ctx)
}
</code></pre>



</details>

<a name="0x2_display_new_with_fields"></a>

## Function `new_with_fields`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_new_with_fields">new_with_fields</a>&lt;T: key&gt;(pub: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">package::Publisher</a>, fields: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>&gt;, values: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>&gt;, ctx: &<b>mut</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_new_with_fields">new_with_fields</a>&lt;T: key&gt;(
    pub: &Publisher, fields: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;String&gt;, values: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;String&gt;, ctx: &<b>mut</b> TxContext
): <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt; {
    <b>let</b> len = <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&fields);
    <b>assert</b>!(len == <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&values), <a href="../../dependencies/mgo-framework/display.md#0x2_display_EVecLengthMismatch">EVecLengthMismatch</a>);

    <b>let</b> i = 0;
    <b>let</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a> = <a href="../../dependencies/mgo-framework/display.md#0x2_display_new">new</a>&lt;T&gt;(pub, ctx);
    <b>while</b> (i &lt; len) {
        <a href="../../dependencies/mgo-framework/display.md#0x2_display_add_internal">add_internal</a>(&<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>, *<a href="../../dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&fields, i), *<a href="../../dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&values, i));
        i = i + 1;
    };

    <a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>
}
</code></pre>



</details>

<a name="0x2_display_create_and_keep"></a>

## Function `create_and_keep`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_create_and_keep">create_and_keep</a>&lt;T: key&gt;(pub: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">package::Publisher</a>, ctx: &<b>mut</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_create_and_keep">create_and_keep</a>&lt;T: key&gt;(pub: &Publisher, ctx: &<b>mut</b> TxContext) {
    <a href="../../dependencies/mgo-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="../../dependencies/mgo-framework/display.md#0x2_display_new">new</a>&lt;T&gt;(pub, ctx), sender(ctx))
}
</code></pre>



</details>

<a name="0x2_display_update_version"></a>

## Function `update_version`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_update_version">update_version</a>&lt;T: key&gt;(<a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_update_version">update_version</a>&lt;T: key&gt;(
    <a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt;
) {
    <a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>.version = <a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>.version + 1;
    <a href="../../dependencies/mgo-framework/event.md#0x2_event_emit">event::emit</a>(<a href="../../dependencies/mgo-framework/display.md#0x2_display_VersionUpdated">VersionUpdated</a>&lt;T&gt; {
        version: <a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>.version,
        fields: *&<a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>.fields,
        id: <a href="../../dependencies/mgo-framework/object.md#0x2_object_uid_to_inner">object::uid_to_inner</a>(&<a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>.id),
    })
}
</code></pre>



</details>

<a name="0x2_display_add"></a>

## Function `add`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_add">add</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;, name: <a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, value: <a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_add">add</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt;, name: String, value: String) {
    <a href="../../dependencies/mgo-framework/display.md#0x2_display_add_internal">add_internal</a>(self, name, value)
}
</code></pre>



</details>

<a name="0x2_display_add_multiple"></a>

## Function `add_multiple`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_add_multiple">add_multiple</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;, fields: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>&gt;, values: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_add_multiple">add_multiple</a>&lt;T: key&gt;(
    self: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt;, fields: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;String&gt;, values: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;String&gt;
) {
    <b>let</b> len = <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&fields);
    <b>assert</b>!(len == <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&values), <a href="../../dependencies/mgo-framework/display.md#0x2_display_EVecLengthMismatch">EVecLengthMismatch</a>);

    <b>let</b> i = 0;
    <b>while</b> (i &lt; len) {
        <a href="../../dependencies/mgo-framework/display.md#0x2_display_add_internal">add_internal</a>(self, *<a href="../../dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&fields, i), *<a href="../../dependencies/move-stdlib/vector.md#0x1_vector_borrow">vector::borrow</a>(&values, i));
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="0x2_display_edit"></a>

## Function `edit`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_edit">edit</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;, name: <a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, value: <a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_edit">edit</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt;, name: String, value: String) {
    <b>let</b> (_, _) = <a href="../../dependencies/mgo-framework/vec_map.md#0x2_vec_map_remove">vec_map::remove</a>(&<b>mut</b> self.fields, &name);
    <a href="../../dependencies/mgo-framework/display.md#0x2_display_add_internal">add_internal</a>(self, name, value)
}
</code></pre>



</details>

<a name="0x2_display_remove"></a>

## Function `remove`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_remove">remove</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;, name: <a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code>entry <b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_remove">remove</a>&lt;T: key&gt;(self: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt;, name: String) {
    <a href="../../dependencies/mgo-framework/vec_map.md#0x2_vec_map_remove">vec_map::remove</a>(&<b>mut</b> self.fields, &name);
}
</code></pre>



</details>

<a name="0x2_display_is_authorized"></a>

## Function `is_authorized`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_is_authorized">is_authorized</a>&lt;T: key&gt;(pub: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">package::Publisher</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_is_authorized">is_authorized</a>&lt;T: key&gt;(pub: &Publisher): bool {
    from_package&lt;T&gt;(pub)
}
</code></pre>



</details>

<a name="0x2_display_version"></a>

## Function `version`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_version">version</a>&lt;T: key&gt;(d: &<a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_version">version</a>&lt;T: key&gt;(d: &<a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt;): u16 {
    d.version
}
</code></pre>



</details>

<a name="0x2_display_fields"></a>

## Function `fields`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_fields">fields</a>&lt;T: key&gt;(d: &<a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;): &<a href="../../dependencies/mgo-framework/vec_map.md#0x2_vec_map_VecMap">vec_map::VecMap</a>&lt;<a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, <a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_fields">fields</a>&lt;T: key&gt;(d: &<a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt;): &VecMap&lt;String, String&gt; {
    &d.fields
}
</code></pre>



</details>

<a name="0x2_display_create_internal"></a>

## Function `create_internal`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_create_internal">create_internal</a>&lt;T: key&gt;(ctx: &<b>mut</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_create_internal">create_internal</a>&lt;T: key&gt;(ctx: &<b>mut</b> TxContext): <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt; {
    <b>let</b> uid = <a href="../../dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx);

    <a href="../../dependencies/mgo-framework/event.md#0x2_event_emit">event::emit</a>(<a href="../../dependencies/mgo-framework/display.md#0x2_display_DisplayCreated">DisplayCreated</a>&lt;T&gt; {
        id: <a href="../../dependencies/mgo-framework/object.md#0x2_object_uid_to_inner">object::uid_to_inner</a>(&uid)
    });

    <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a> {
        id: uid,
        fields: <a href="../../dependencies/mgo-framework/vec_map.md#0x2_vec_map_empty">vec_map::empty</a>(),
        version: 0,
    }
}
</code></pre>



</details>

<a name="0x2_display_add_internal"></a>

## Function `add_internal`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_add_internal">add_internal</a>&lt;T: key&gt;(<a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">display::Display</a>&lt;T&gt;, name: <a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>, value: <a href="../../dependencies/move-stdlib/string.md#0x1_string_String">string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_add_internal">add_internal</a>&lt;T: key&gt;(<a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>: &<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display_Display">Display</a>&lt;T&gt;, name: String, value: String) {
    <a href="../../dependencies/mgo-framework/vec_map.md#0x2_vec_map_insert">vec_map::insert</a>(&<b>mut</b> <a href="../../dependencies/mgo-framework/display.md#0x2_display">display</a>.fields, name, value)
}
</code></pre>



</details>
