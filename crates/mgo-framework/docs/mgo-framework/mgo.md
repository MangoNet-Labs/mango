
<a name="0x2_mgo"></a>

# Module `0x2::mgo`

Coin<MGO> is the token used to pay for gas in Mgo.
It has 9 decimals, and the smallest unit (10^-9) is called "mango".


-  [Struct `MGO`](#0x2_mgo_MGO)
-  [Constants](#@Constants_0)
-  [Function `new`](#0x2_mgo_new)
-  [Function `transfer`](#0x2_mgo_transfer)


<pre><code><b>use</b> <a href="dependencies/move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="url.md#0x2_url">0x2::url</a>;
</code></pre>



<a name="0x2_mgo_MGO"></a>

## Struct `MGO`

Name of the coin


<pre><code><b>struct</b> <a href="mgo.md#0x2_mgo_MGO">MGO</a> <b>has</b> drop
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

<a name="@Constants_0"></a>

## Constants


<a name="0x2_mgo_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="mgo.md#0x2_mgo_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="0x2_mgo_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="mgo.md#0x2_mgo_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="0x2_mgo_MANGO_PER_MGO"></a>

The amount of Mango per Mgo token based on the the fact that mango is
10^-9 of a Mgo token


<pre><code><b>const</b> <a href="mgo.md#0x2_mgo_MANGO_PER_MGO">MANGO_PER_MGO</a>: u64 = 1000000000;
</code></pre>



<a name="0x2_mgo_TOTAL_SUPPLY_MANGO"></a>

The total supply of Mgo denominated in Mango (10 Billion * 10^9)


<pre><code><b>const</b> <a href="mgo.md#0x2_mgo_TOTAL_SUPPLY_MANGO">TOTAL_SUPPLY_MANGO</a>: u64 = 10000000000000000000;
</code></pre>



<a name="0x2_mgo_TOTAL_SUPPLY_MGO"></a>

The total supply of Mgo denominated in whole Mgo tokens (10 Billion)


<pre><code><b>const</b> <a href="mgo.md#0x2_mgo_TOTAL_SUPPLY_MGO">TOTAL_SUPPLY_MGO</a>: u64 = 10000000000;
</code></pre>



<a name="0x2_mgo_new"></a>

## Function `new`

Register the <code><a href="mgo.md#0x2_mgo_MGO">MGO</a></code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="mgo.md#0x2_mgo_new">new</a>(ctx: &<b>mut</b> <a href="tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="mgo.md#0x2_mgo_MGO">mgo::MGO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="mgo.md#0x2_mgo_new">new</a>(ctx: &<b>mut</b> TxContext): Balance&lt;<a href="mgo.md#0x2_mgo_MGO">MGO</a>&gt; {
    <b>assert</b>!(<a href="tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="mgo.md#0x2_mgo_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(<a href="tx_context.md#0x2_tx_context_epoch">tx_context::epoch</a>(ctx) == 0, <a href="mgo.md#0x2_mgo_EAlreadyMinted">EAlreadyMinted</a>);

    <b>let</b> (treasury, metadata) = <a href="coin.md#0x2_coin_create_currency">coin::create_currency</a>(
        <a href="mgo.md#0x2_mgo_MGO">MGO</a> {},
        9,
        b"<a href="mgo.md#0x2_mgo_MGO">MGO</a>",
        b"Mgo",
        // TODO: add appropriate description and logo <a href="url.md#0x2_url">url</a>
        b"",
        <a href="dependencies/move-stdlib/option.md#0x1_option_none">option::none</a>(),
        ctx
    );
    <a href="transfer.md#0x2_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> supply = <a href="coin.md#0x2_coin_treasury_into_supply">coin::treasury_into_supply</a>(treasury);
    <b>let</b> total_mgo = <a href="balance.md#0x2_balance_increase_supply">balance::increase_supply</a>(&<b>mut</b> supply, <a href="mgo.md#0x2_mgo_TOTAL_SUPPLY_MANGO">TOTAL_SUPPLY_MANGO</a>);
    <a href="balance.md#0x2_balance_destroy_supply">balance::destroy_supply</a>(supply);
    total_mgo
}
</code></pre>



</details>

<a name="0x2_mgo_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> entry <b>fun</b> <a href="transfer.md#0x2_transfer">transfer</a>(c: <a href="coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="mgo.md#0x2_mgo_MGO">mgo::MGO</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="transfer.md#0x2_transfer">transfer</a>(c: <a href="coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="mgo.md#0x2_mgo_MGO">MGO</a>&gt;, recipient: <b>address</b>) {
    <a href="transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>
