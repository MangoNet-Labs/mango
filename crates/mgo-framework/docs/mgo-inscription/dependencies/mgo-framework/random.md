
<a name="0x2_random"></a>

# Module `0x2::random`



-  [Resource `Random`](#0x2_random_Random)
-  [Struct `RandomInner`](#0x2_random_RandomInner)
-  [Struct `RandomGenerator`](#0x2_random_RandomGenerator)
-  [Constants](#@Constants_0)
-  [Function `create`](#0x2_random_create)
-  [Function `load_inner_mut`](#0x2_random_load_inner_mut)
-  [Function `load_inner`](#0x2_random_load_inner)
-  [Function `update_randomness_state`](#0x2_random_update_randomness_state)
-  [Function `new_generator`](#0x2_random_new_generator)
-  [Function `derive_next_block`](#0x2_random_derive_next_block)
-  [Function `fill_buffer`](#0x2_random_fill_buffer)
-  [Function `generate_bytes`](#0x2_random_generate_bytes)
-  [Function `u256_from_bytes`](#0x2_random_u256_from_bytes)
-  [Function `generate_u256`](#0x2_random_generate_u256)
-  [Function `generate_u128`](#0x2_random_generate_u128)
-  [Function `generate_u64`](#0x2_random_generate_u64)
-  [Function `generate_u32`](#0x2_random_generate_u32)
-  [Function `generate_u16`](#0x2_random_generate_u16)
-  [Function `generate_u8`](#0x2_random_generate_u8)
-  [Function `generate_bool`](#0x2_random_generate_bool)
-  [Function `u128_in_range`](#0x2_random_u128_in_range)
-  [Function `generate_u128_in_range`](#0x2_random_generate_u128_in_range)
-  [Function `generate_u64_in_range`](#0x2_random_generate_u64_in_range)
-  [Function `generate_u32_in_range`](#0x2_random_generate_u32_in_range)
-  [Function `generate_u16_in_range`](#0x2_random_generate_u16_in_range)
-  [Function `generate_u8_in_range`](#0x2_random_generate_u8_in_range)
-  [Function `shuffle`](#0x2_random_shuffle)


<pre><code><b>use</b> <a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs">0x1::bcs</a>;
<b>use</b> <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">0x1::vector</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/address.md#0x2_address">0x2::address</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/hmac.md#0x2_hmac">0x2::hmac</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/versioned.md#0x2_versioned">0x2::versioned</a>;
</code></pre>



<a name="0x2_random_Random"></a>

## Resource `Random`



<pre><code><b>struct</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_Random">Random</a> <b>has</b> key
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
<code>inner: <a href="../../dependencies/mgo-framework/versioned.md#0x2_versioned_Versioned">versioned::Versioned</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_random_RandomInner"></a>

## Struct `RandomInner`



<pre><code><b>struct</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomInner">RandomInner</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>version: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>epoch: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>randomness_round: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>random_bytes: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_random_RandomGenerator"></a>

## Struct `RandomGenerator`



<pre><code><b>struct</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>seed: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
<dt>
<code>counter: u16</code>
</dt>
<dd>

</dd>
<dt>
<code>buffer: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x2_random_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="0x2_random_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 1;
</code></pre>



<a name="0x2_random_CURRENT_VERSION"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_CURRENT_VERSION">CURRENT_VERSION</a>: u64 = 1;
</code></pre>



<a name="0x2_random_EInvalidLength"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_EInvalidLength">EInvalidLength</a>: u64 = 4;
</code></pre>



<a name="0x2_random_EInvalidRandomnessUpdate"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_EInvalidRandomnessUpdate">EInvalidRandomnessUpdate</a>: u64 = 2;
</code></pre>



<a name="0x2_random_EInvalidRange"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_EInvalidRange">EInvalidRange</a>: u64 = 3;
</code></pre>



<a name="0x2_random_RAND_OUTPUT_LEN"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RAND_OUTPUT_LEN">RAND_OUTPUT_LEN</a>: u16 = 32;
</code></pre>



<a name="0x2_random_U16_MAX"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_U16_MAX">U16_MAX</a>: u64 = 65535;
</code></pre>



<a name="0x2_random_create"></a>

## Function `create`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_create">create</a>(ctx: &<b>mut</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_create">create</a>(ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(<a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="../../dependencies/mgo-framework/random.md#0x2_random_ENotSystemAddress">ENotSystemAddress</a>);

    <b>let</b> version = <a href="../../dependencies/mgo-framework/random.md#0x2_random_CURRENT_VERSION">CURRENT_VERSION</a>;

    <b>let</b> inner = <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomInner">RandomInner</a> {
        version,
        epoch: <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx),
        randomness_round: 0,
        random_bytes: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[],
    };

    <b>let</b> self = <a href="../../dependencies/mgo-framework/random.md#0x2_random_Random">Random</a> {
        id: <a href="../../dependencies/mgo-framework/object.md#0x2_object_randomness_state">object::randomness_state</a>(),
        inner: <a href="../../dependencies/mgo-framework/versioned.md#0x2_versioned_create">versioned::create</a>(version, inner, ctx),
    };
    <a href="../../dependencies/mgo-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="0x2_random_load_inner_mut"></a>

## Function `load_inner_mut`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_load_inner_mut">load_inner_mut</a>(self: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_Random">random::Random</a>): &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomInner">random::RandomInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_load_inner_mut">load_inner_mut</a>(self: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_Random">Random</a>): &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomInner">RandomInner</a> {
    <b>let</b> version = <a href="../../dependencies/mgo-framework/versioned.md#0x2_versioned_version">versioned::version</a>(&self.inner);

    // Replace this <b>with</b> a lazy <b>update</b> function when we add a new version of the inner <a href="../../dependencies/mgo-framework/object.md#0x2_object">object</a>.
    <b>assert</b>!(version == <a href="../../dependencies/mgo-framework/random.md#0x2_random_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../../dependencies/mgo-framework/random.md#0x2_random_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomInner">RandomInner</a> = <a href="../../dependencies/mgo-framework/versioned.md#0x2_versioned_load_value_mut">versioned::load_value_mut</a>(&<b>mut</b> self.inner);
    <b>assert</b>!(inner.version == version, <a href="../../dependencies/mgo-framework/random.md#0x2_random_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="0x2_random_load_inner"></a>

## Function `load_inner`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_load_inner">load_inner</a>(self: &<a href="../../dependencies/mgo-framework/random.md#0x2_random_Random">random::Random</a>): &<a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomInner">random::RandomInner</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_load_inner">load_inner</a>(self: &<a href="../../dependencies/mgo-framework/random.md#0x2_random_Random">Random</a>): &<a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomInner">RandomInner</a> {
    <b>let</b> version = <a href="../../dependencies/mgo-framework/versioned.md#0x2_versioned_version">versioned::version</a>(&self.inner);

    // Replace this <b>with</b> a lazy <b>update</b> function when we add a new version of the inner <a href="../../dependencies/mgo-framework/object.md#0x2_object">object</a>.
    <b>assert</b>!(version == <a href="../../dependencies/mgo-framework/random.md#0x2_random_CURRENT_VERSION">CURRENT_VERSION</a>, <a href="../../dependencies/mgo-framework/random.md#0x2_random_EWrongInnerVersion">EWrongInnerVersion</a>);
    <b>let</b> inner: &<a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomInner">RandomInner</a> = <a href="../../dependencies/mgo-framework/versioned.md#0x2_versioned_load_value">versioned::load_value</a>(&self.inner);
    <b>assert</b>!(inner.version == version, <a href="../../dependencies/mgo-framework/random.md#0x2_random_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="0x2_random_update_randomness_state"></a>

## Function `update_randomness_state`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_update_randomness_state">update_randomness_state</a>(self: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_Random">random::Random</a>, new_round: u64, new_bytes: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_update_randomness_state">update_randomness_state</a>(
    self: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_Random">Random</a>,
    new_round: u64,
    new_bytes: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    // Validator will make a special system call <b>with</b> sender set <b>as</b> 0x0.
    <b>assert</b>!(<a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="../../dependencies/mgo-framework/random.md#0x2_random_ENotSystemAddress">ENotSystemAddress</a>);

    // Randomness should only be incremented.
    <b>let</b> epoch = <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx);
    <b>let</b> inner = <a href="../../dependencies/mgo-framework/random.md#0x2_random_load_inner_mut">load_inner_mut</a>(self);
    <b>if</b> (inner.randomness_round == 0 && inner.epoch == 0 && <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_is_empty">vector::is_empty</a>(&inner.random_bytes)) {
        // First <b>update</b> should be for round zero.
        <b>assert</b>!(new_round == 0, <a href="../../dependencies/mgo-framework/random.md#0x2_random_EInvalidRandomnessUpdate">EInvalidRandomnessUpdate</a>);
    } <b>else</b> {
        // Subsequent updates should either increase epoch or increment randomness_round.
        // Note that epoch may increase by more than 1 <b>if</b> an epoch is completed without
        // randomness ever being generated in that epoch.
        <b>assert</b>!(
            (epoch &gt; inner.epoch && new_round == 0) ||
                (new_round == inner.randomness_round + 1),
            <a href="../../dependencies/mgo-framework/random.md#0x2_random_EInvalidRandomnessUpdate">EInvalidRandomnessUpdate</a>,
        );
    };

    inner.epoch = <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx);
    inner.randomness_round = new_round;
    inner.random_bytes = new_bytes;
}
</code></pre>



</details>

<a name="0x2_random_new_generator"></a>

## Function `new_generator`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_new_generator">new_generator</a>(r: &<a href="../../dependencies/mgo-framework/random.md#0x2_random_Random">random::Random</a>, ctx: &<b>mut</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_new_generator">new_generator</a>(r: &<a href="../../dependencies/mgo-framework/random.md#0x2_random_Random">Random</a>, ctx: &<b>mut</b> TxContext): <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a> {
    <b>let</b> inner = <a href="../../dependencies/mgo-framework/random.md#0x2_random_load_inner">load_inner</a>(r);
    <b>let</b> seed = hmac_sha3_256(
        &inner.random_bytes,
        &address::to_bytes(<a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_fresh_object_address">tx_context::fresh_object_address</a>(ctx)),
    );
    <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a> { seed, counter: 0, buffer: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>[] }
}
</code></pre>



</details>

<a name="0x2_random_derive_next_block"></a>

## Function `derive_next_block`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_derive_next_block">derive_next_block</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_derive_next_block">derive_next_block</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    g.counter = g.counter + 1;
    hmac_sha3_256(&g.seed, &<a href="../../dependencies/move-stdlib/bcs.md#0x1_bcs_to_bytes">bcs::to_bytes</a>(&g.counter))
}
</code></pre>



</details>

<a name="0x2_random_fill_buffer"></a>

## Function `fill_buffer`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_fill_buffer">fill_buffer</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_fill_buffer">fill_buffer</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>) {
    <b>let</b> next_block = <a href="../../dependencies/mgo-framework/random.md#0x2_random_derive_next_block">derive_next_block</a>(g);
    <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> g.buffer, next_block);
}
</code></pre>



</details>

<a name="0x2_random_generate_bytes"></a>

## Function `generate_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_bytes">generate_bytes</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>, num_of_bytes: u16): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_bytes">generate_bytes</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>, num_of_bytes: u16): <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    <b>let</b> result = <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_empty">vector::empty</a>&lt;u8&gt;();
    // Append <a href="../../dependencies/mgo-framework/random.md#0x2_random_RAND_OUTPUT_LEN">RAND_OUTPUT_LEN</a> size buffers directly without going through the generator's buffer.
    <b>let</b> num_of_blocks = num_of_bytes / <a href="../../dependencies/mgo-framework/random.md#0x2_random_RAND_OUTPUT_LEN">RAND_OUTPUT_LEN</a>;
    <b>while</b> (num_of_blocks &gt; 0) {
        <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_append">vector::append</a>(&<b>mut</b> result, <a href="../../dependencies/mgo-framework/random.md#0x2_random_derive_next_block">derive_next_block</a>(g));
        num_of_blocks = num_of_blocks - 1;
    };
    // Fill the generator's buffer <b>if</b> needed.
    <b>let</b> num_of_bytes = (num_of_bytes <b>as</b> u64);
    <b>if</b> (<a href="../../dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&g.buffer) &lt; (num_of_bytes - <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&result))) {
        <a href="../../dependencies/mgo-framework/random.md#0x2_random_fill_buffer">fill_buffer</a>(g);
    };
    // Take remaining bytes from the generator's buffer.
    <b>while</b> (<a href="../../dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&result) &lt; num_of_bytes) {
        <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_push_back">vector::push_back</a>(&<b>mut</b> result, <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> g.buffer));
    };
    result
}
</code></pre>



</details>

<a name="0x2_random_u256_from_bytes"></a>

## Function `u256_from_bytes`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_u256_from_bytes">u256_from_bytes</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>, num_of_bytes: u8): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_u256_from_bytes">u256_from_bytes</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>, num_of_bytes: u8): u256 {
    <b>if</b> (<a href="../../dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(&g.buffer) &lt; (num_of_bytes <b>as</b> u64)) {
        <a href="../../dependencies/mgo-framework/random.md#0x2_random_fill_buffer">fill_buffer</a>(g);
    };
    <b>let</b> result: u256 = 0;
    <b>let</b> i = 0;
    <b>while</b> (i &lt; num_of_bytes) {
        <b>let</b> byte = <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_pop_back">vector::pop_back</a>(&<b>mut</b> g.buffer);
        result = (result &lt;&lt; 8) + (byte <b>as</b> u256);
        i = i + 1;
    };
    result
}
</code></pre>



</details>

<a name="0x2_random_generate_u256"></a>

## Function `generate_u256`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u256">generate_u256</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>): u256
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u256">generate_u256</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>): u256 {
    <a href="../../dependencies/mgo-framework/random.md#0x2_random_u256_from_bytes">u256_from_bytes</a>(g, 32)
}
</code></pre>



</details>

<a name="0x2_random_generate_u128"></a>

## Function `generate_u128`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u128">generate_u128</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u128">generate_u128</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>): u128 {
    (<a href="../../dependencies/mgo-framework/random.md#0x2_random_u256_from_bytes">u256_from_bytes</a>(g, 16) <b>as</b> u128)
}
</code></pre>



</details>

<a name="0x2_random_generate_u64"></a>

## Function `generate_u64`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u64">generate_u64</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u64">generate_u64</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>): u64 {
    (<a href="../../dependencies/mgo-framework/random.md#0x2_random_u256_from_bytes">u256_from_bytes</a>(g, 8) <b>as</b> u64)
}
</code></pre>



</details>

<a name="0x2_random_generate_u32"></a>

## Function `generate_u32`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u32">generate_u32</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u32">generate_u32</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>): u32 {
    (<a href="../../dependencies/mgo-framework/random.md#0x2_random_u256_from_bytes">u256_from_bytes</a>(g, 4) <b>as</b> u32)
}
</code></pre>



</details>

<a name="0x2_random_generate_u16"></a>

## Function `generate_u16`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u16">generate_u16</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u16">generate_u16</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>): u16 {
    (<a href="../../dependencies/mgo-framework/random.md#0x2_random_u256_from_bytes">u256_from_bytes</a>(g, 2) <b>as</b> u16)
}
</code></pre>



</details>

<a name="0x2_random_generate_u8"></a>

## Function `generate_u8`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u8">generate_u8</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u8">generate_u8</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>): u8 {
    (<a href="../../dependencies/mgo-framework/random.md#0x2_random_u256_from_bytes">u256_from_bytes</a>(g, 1) <b>as</b> u8)
}
</code></pre>



</details>

<a name="0x2_random_generate_bool"></a>

## Function `generate_bool`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_bool">generate_bool</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_bool">generate_bool</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>): bool {
    (<a href="../../dependencies/mgo-framework/random.md#0x2_random_u256_from_bytes">u256_from_bytes</a>(g, 1) & 1) == 1
}
</code></pre>



</details>

<a name="0x2_random_u128_in_range"></a>

## Function `u128_in_range`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_u128_in_range">u128_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>, <b>min</b>: u128, max: u128, num_of_bytes: u8): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_u128_in_range">u128_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>, <b>min</b>: u128, max: u128, num_of_bytes: u8): u128 {
    <b>assert</b>!(<b>min</b> &lt;= max, <a href="../../dependencies/mgo-framework/random.md#0x2_random_EInvalidRange">EInvalidRange</a>);
    <b>if</b> (<b>min</b> == max) {
        <b>return</b> <b>min</b>
    };
    // Pick a <a href="../../dependencies/mgo-framework/random.md#0x2_random">random</a> number in [0, max - <b>min</b>] by generating a <a href="../../dependencies/mgo-framework/random.md#0x2_random">random</a> number that is larger than max-<b>min</b>, and taking
    // the modulo of the <a href="../../dependencies/mgo-framework/random.md#0x2_random">random</a> number by the range size. Then add the <b>min</b> <b>to</b> the result <b>to</b> get a number in
    // [<b>min</b>, max].
    <b>let</b> range_size = ((max - <b>min</b>) <b>as</b> u256) + 1;
    <b>let</b> rand = <a href="../../dependencies/mgo-framework/random.md#0x2_random_u256_from_bytes">u256_from_bytes</a>(g, num_of_bytes);
    <b>min</b> + (rand % range_size <b>as</b> u128)
}
</code></pre>



</details>

<a name="0x2_random_generate_u128_in_range"></a>

## Function `generate_u128_in_range`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u128_in_range">generate_u128_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>, <b>min</b>: u128, max: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u128_in_range">generate_u128_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>, <b>min</b>: u128, max: u128): u128 {
    <a href="../../dependencies/mgo-framework/random.md#0x2_random_u128_in_range">u128_in_range</a>(g, <b>min</b>, max, 24)
}
</code></pre>



</details>

<a name="0x2_random_generate_u64_in_range"></a>

## Function `generate_u64_in_range`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u64_in_range">generate_u64_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>, <b>min</b>: u64, max: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u64_in_range">generate_u64_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>, <b>min</b>: u64, max: u64): u64 {
    (<a href="../../dependencies/mgo-framework/random.md#0x2_random_u128_in_range">u128_in_range</a>(g, (<b>min</b> <b>as</b> u128), (max <b>as</b> u128), 16) <b>as</b> u64)
}
</code></pre>



</details>

<a name="0x2_random_generate_u32_in_range"></a>

## Function `generate_u32_in_range`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u32_in_range">generate_u32_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>, <b>min</b>: u32, max: u32): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u32_in_range">generate_u32_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>, <b>min</b>: u32, max: u32): u32 {
    (<a href="../../dependencies/mgo-framework/random.md#0x2_random_u128_in_range">u128_in_range</a>(g, (<b>min</b> <b>as</b> u128), (max <b>as</b> u128), 12) <b>as</b> u32)
}
</code></pre>



</details>

<a name="0x2_random_generate_u16_in_range"></a>

## Function `generate_u16_in_range`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u16_in_range">generate_u16_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>, <b>min</b>: u16, max: u16): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u16_in_range">generate_u16_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>, <b>min</b>: u16, max: u16): u16 {
    (<a href="../../dependencies/mgo-framework/random.md#0x2_random_u128_in_range">u128_in_range</a>(g, (<b>min</b> <b>as</b> u128), (max <b>as</b> u128), 10) <b>as</b> u16)
}
</code></pre>



</details>

<a name="0x2_random_generate_u8_in_range"></a>

## Function `generate_u8_in_range`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u8_in_range">generate_u8_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>, <b>min</b>: u8, max: u8): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u8_in_range">generate_u8_in_range</a>(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>, <b>min</b>: u8, max: u8): u8 {
    (<a href="../../dependencies/mgo-framework/random.md#0x2_random_u128_in_range">u128_in_range</a>(g, (<b>min</b> <b>as</b> u128), (max <b>as</b> u128), 9) <b>as</b> u8)
}
</code></pre>



</details>

<a name="0x2_random_shuffle"></a>

## Function `shuffle`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_shuffle">shuffle</a>&lt;T&gt;(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">random::RandomGenerator</a>, v: &<b>mut</b> <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_shuffle">shuffle</a>&lt;T&gt;(g: &<b>mut</b> <a href="../../dependencies/mgo-framework/random.md#0x2_random_RandomGenerator">RandomGenerator</a>, v: &<b>mut</b> <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;T&gt;) {
    <b>let</b> n = <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_length">vector::length</a>(v);
    <b>if</b> (n == 0) {
        <b>return</b>
    };
    <b>assert</b>!(n &lt;= <a href="../../dependencies/mgo-framework/random.md#0x2_random_U16_MAX">U16_MAX</a>, <a href="../../dependencies/mgo-framework/random.md#0x2_random_EInvalidLength">EInvalidLength</a>);
    <b>let</b> n = (n <b>as</b> u16);
    <b>let</b> i: u16 = 0;
    <b>let</b> end = n - 1;
    <b>while</b> (i &lt; end) {
        <b>let</b> j = <a href="../../dependencies/mgo-framework/random.md#0x2_random_generate_u16_in_range">generate_u16_in_range</a>(g, i, end);
        <a href="../../dependencies/move-stdlib/vector.md#0x1_vector_swap">vector::swap</a>(v, (i <b>as</b> u64), (j <b>as</b> u64));
        i = i + 1;
    };
}
</code></pre>



</details>
