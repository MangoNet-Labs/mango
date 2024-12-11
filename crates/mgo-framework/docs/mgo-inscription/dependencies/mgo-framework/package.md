
<a name="0x2_package"></a>

# Module `0x2::package`



-  [Resource `Publisher`](#0x2_package_Publisher)
-  [Resource `UpgradeCap`](#0x2_package_UpgradeCap)
-  [Struct `UpgradeTicket`](#0x2_package_UpgradeTicket)
-  [Struct `UpgradeReceipt`](#0x2_package_UpgradeReceipt)
-  [Constants](#@Constants_0)
-  [Function `claim`](#0x2_package_claim)
-  [Function `claim_and_keep`](#0x2_package_claim_and_keep)
-  [Function `burn_publisher`](#0x2_package_burn_publisher)
-  [Function `from_package`](#0x2_package_from_package)
-  [Function `from_module`](#0x2_package_from_module)
-  [Function `published_module`](#0x2_package_published_module)
-  [Function `published_package`](#0x2_package_published_package)
-  [Function `upgrade_package`](#0x2_package_upgrade_package)
-  [Function `version`](#0x2_package_version)
-  [Function `upgrade_policy`](#0x2_package_upgrade_policy)
-  [Function `ticket_package`](#0x2_package_ticket_package)
-  [Function `ticket_policy`](#0x2_package_ticket_policy)
-  [Function `receipt_cap`](#0x2_package_receipt_cap)
-  [Function `receipt_package`](#0x2_package_receipt_package)
-  [Function `ticket_digest`](#0x2_package_ticket_digest)
-  [Function `compatible_policy`](#0x2_package_compatible_policy)
-  [Function `additive_policy`](#0x2_package_additive_policy)
-  [Function `dep_only_policy`](#0x2_package_dep_only_policy)
-  [Function `only_additive_upgrades`](#0x2_package_only_additive_upgrades)
-  [Function `only_dep_upgrades`](#0x2_package_only_dep_upgrades)
-  [Function `make_immutable`](#0x2_package_make_immutable)
-  [Function `authorize_upgrade`](#0x2_package_authorize_upgrade)
-  [Function `commit_upgrade`](#0x2_package_commit_upgrade)
-  [Function `restrict`](#0x2_package_restrict)


<pre><code><b>use</b> <a href="../../dependencies/move-stdlib/ascii.md#0x1_ascii">0x1::ascii</a>;
<b>use</b> <a href="../../dependencies/move-stdlib/type_name.md#0x1_type_name">0x1::type_name</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="../../dependencies/mgo-framework/types.md#0x2_types">0x2::types</a>;
</code></pre>



<a name="0x2_package_Publisher"></a>

## Resource `Publisher`



<pre><code><b>struct</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">Publisher</a> <b>has</b> store, key
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
<code><a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>: <a href="../../dependencies/move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
<dt>
<code>module_name: <a href="../../dependencies/move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_package_UpgradeCap"></a>

## Resource `UpgradeCap`



<pre><code><b>struct</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a> <b>has</b> store, key
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
<code><a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>: <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>

</dd>
<dt>
<code>policy: u8</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_package_UpgradeTicket"></a>

## Struct `UpgradeTicket`



<pre><code><b>struct</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeTicket">UpgradeTicket</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>cap: <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>: <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code>policy: u8</code>
</dt>
<dd>

</dd>
<dt>
<code>digest: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="0x2_package_UpgradeReceipt"></a>

## Struct `UpgradeReceipt`



<pre><code><b>struct</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeReceipt">UpgradeReceipt</a>
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>cap: <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
<dt>
<code><a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>: <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a></code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x2_package_ADDITIVE"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_ADDITIVE">ADDITIVE</a>: u8 = 128;
</code></pre>



<a name="0x2_package_COMPATIBLE"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_COMPATIBLE">COMPATIBLE</a>: u8 = 0;
</code></pre>



<a name="0x2_package_DEP_ONLY"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_DEP_ONLY">DEP_ONLY</a>: u8 = 192;
</code></pre>



<a name="0x2_package_EAlreadyAuthorized"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_EAlreadyAuthorized">EAlreadyAuthorized</a>: u64 = 2;
</code></pre>



<a name="0x2_package_ENotAuthorized"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_ENotAuthorized">ENotAuthorized</a>: u64 = 3;
</code></pre>



<a name="0x2_package_ENotOneTimeWitness"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_ENotOneTimeWitness">ENotOneTimeWitness</a>: u64 = 0;
</code></pre>



<a name="0x2_package_ETooPermissive"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_ETooPermissive">ETooPermissive</a>: u64 = 1;
</code></pre>



<a name="0x2_package_EWrongUpgradeCap"></a>



<pre><code><b>const</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_EWrongUpgradeCap">EWrongUpgradeCap</a>: u64 = 4;
</code></pre>



<a name="0x2_package_claim"></a>

## Function `claim`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_claim">claim</a>&lt;OTW: drop&gt;(otw: OTW, ctx: &<b>mut</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">package::Publisher</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_claim">claim</a>&lt;OTW: drop&gt;(otw: OTW, ctx: &<b>mut</b> TxContext): <a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">Publisher</a> {
    <b>assert</b>!(<a href="../../dependencies/mgo-framework/types.md#0x2_types_is_one_time_witness">types::is_one_time_witness</a>(&otw), <a href="../../dependencies/mgo-framework/package.md#0x2_package_ENotOneTimeWitness">ENotOneTimeWitness</a>);

    <b>let</b> type = <a href="../../dependencies/move-stdlib/type_name.md#0x1_type_name_get_with_original_ids">type_name::get_with_original_ids</a>&lt;OTW&gt;();

    <a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">Publisher</a> {
        id: <a href="../../dependencies/mgo-framework/object.md#0x2_object_new">object::new</a>(ctx),
        <a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>: <a href="../../dependencies/move-stdlib/type_name.md#0x1_type_name_get_address">type_name::get_address</a>(&type),
        module_name: <a href="../../dependencies/move-stdlib/type_name.md#0x1_type_name_get_module">type_name::get_module</a>(&type),
    }
}
</code></pre>



</details>

<a name="0x2_package_claim_and_keep"></a>

## Function `claim_and_keep`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_claim_and_keep">claim_and_keep</a>&lt;OTW: drop&gt;(otw: OTW, ctx: &<b>mut</b> <a href="../../dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_claim_and_keep">claim_and_keep</a>&lt;OTW: drop&gt;(otw: OTW, ctx: &<b>mut</b> TxContext) {
    mgo::transfer::public_transfer(<a href="../../dependencies/mgo-framework/package.md#0x2_package_claim">claim</a>(otw, ctx), sender(ctx))
}
</code></pre>



</details>

<a name="0x2_package_burn_publisher"></a>

## Function `burn_publisher`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_burn_publisher">burn_publisher</a>(self: <a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">package::Publisher</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_burn_publisher">burn_publisher</a>(self: <a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">Publisher</a>) {
    <b>let</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">Publisher</a> { id, <a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>: _, module_name: _ } = self;
    <a href="../../dependencies/mgo-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>

<a name="0x2_package_from_package"></a>

## Function `from_package`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_from_package">from_package</a>&lt;T&gt;(self: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">package::Publisher</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_from_package">from_package</a>&lt;T&gt;(self: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">Publisher</a>): bool {
    <b>let</b> type = <a href="../../dependencies/move-stdlib/type_name.md#0x1_type_name_get_with_original_ids">type_name::get_with_original_ids</a>&lt;T&gt;();

    (<a href="../../dependencies/move-stdlib/type_name.md#0x1_type_name_get_address">type_name::get_address</a>(&type) == self.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>)
}
</code></pre>



</details>

<a name="0x2_package_from_module"></a>

## Function `from_module`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_from_module">from_module</a>&lt;T&gt;(self: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">package::Publisher</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_from_module">from_module</a>&lt;T&gt;(self: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">Publisher</a>): bool {
    <b>let</b> type = <a href="../../dependencies/move-stdlib/type_name.md#0x1_type_name_get_with_original_ids">type_name::get_with_original_ids</a>&lt;T&gt;();

    (<a href="../../dependencies/move-stdlib/type_name.md#0x1_type_name_get_address">type_name::get_address</a>(&type) == self.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>)
        && (<a href="../../dependencies/move-stdlib/type_name.md#0x1_type_name_get_module">type_name::get_module</a>(&type) == self.module_name)
}
</code></pre>



</details>

<a name="0x2_package_published_module"></a>

## Function `published_module`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_published_module">published_module</a>(self: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">package::Publisher</a>): &<a href="../../dependencies/move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_published_module">published_module</a>(self: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">Publisher</a>): &String {
    &self.module_name
}
</code></pre>



</details>

<a name="0x2_package_published_package"></a>

## Function `published_package`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_published_package">published_package</a>(self: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">package::Publisher</a>): &<a href="../../dependencies/move-stdlib/ascii.md#0x1_ascii_String">ascii::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_published_package">published_package</a>(self: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_Publisher">Publisher</a>): &String {
    &self.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>
}
</code></pre>



</details>

<a name="0x2_package_upgrade_package"></a>

## Function `upgrade_package`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_upgrade_package">upgrade_package</a>(cap: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>): <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_upgrade_package">upgrade_package</a>(cap: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a>): ID {
    cap.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>
}
</code></pre>



</details>

<a name="0x2_package_version"></a>

## Function `version`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_version">version</a>(cap: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_version">version</a>(cap: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a>): u64 {
    cap.version
}
</code></pre>



</details>

<a name="0x2_package_upgrade_policy"></a>

## Function `upgrade_policy`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_upgrade_policy">upgrade_policy</a>(cap: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_upgrade_policy">upgrade_policy</a>(cap: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a>): u8 {
    cap.policy
}
</code></pre>



</details>

<a name="0x2_package_ticket_package"></a>

## Function `ticket_package`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_ticket_package">ticket_package</a>(ticket: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeTicket">package::UpgradeTicket</a>): <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_ticket_package">ticket_package</a>(ticket: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeTicket">UpgradeTicket</a>): ID {
    ticket.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>
}
</code></pre>



</details>

<a name="0x2_package_ticket_policy"></a>

## Function `ticket_policy`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_ticket_policy">ticket_policy</a>(ticket: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeTicket">package::UpgradeTicket</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_ticket_policy">ticket_policy</a>(ticket: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeTicket">UpgradeTicket</a>): u8 {
    ticket.policy
}
</code></pre>



</details>

<a name="0x2_package_receipt_cap"></a>

## Function `receipt_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_receipt_cap">receipt_cap</a>(receipt: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeReceipt">package::UpgradeReceipt</a>): <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_receipt_cap">receipt_cap</a>(receipt: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeReceipt">UpgradeReceipt</a>): ID {
    receipt.cap
}
</code></pre>



</details>

<a name="0x2_package_receipt_package"></a>

## Function `receipt_package`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_receipt_package">receipt_package</a>(receipt: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeReceipt">package::UpgradeReceipt</a>): <a href="../../dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_receipt_package">receipt_package</a>(receipt: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeReceipt">UpgradeReceipt</a>): ID {
    receipt.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>
}
</code></pre>



</details>

<a name="0x2_package_ticket_digest"></a>

## Function `ticket_digest`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_ticket_digest">ticket_digest</a>(ticket: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeTicket">package::UpgradeTicket</a>): &<a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_ticket_digest">ticket_digest</a>(ticket: &<a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeTicket">UpgradeTicket</a>): &<a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt; {
    &ticket.digest
}
</code></pre>



</details>

<a name="0x2_package_compatible_policy"></a>

## Function `compatible_policy`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_compatible_policy">compatible_policy</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_compatible_policy">compatible_policy</a>(): u8 { <a href="../../dependencies/mgo-framework/package.md#0x2_package_COMPATIBLE">COMPATIBLE</a> }
</code></pre>



</details>

<a name="0x2_package_additive_policy"></a>

## Function `additive_policy`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_additive_policy">additive_policy</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_additive_policy">additive_policy</a>(): u8 { <a href="../../dependencies/mgo-framework/package.md#0x2_package_ADDITIVE">ADDITIVE</a> }
</code></pre>



</details>

<a name="0x2_package_dep_only_policy"></a>

## Function `dep_only_policy`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_dep_only_policy">dep_only_policy</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_dep_only_policy">dep_only_policy</a>(): u8 { <a href="../../dependencies/mgo-framework/package.md#0x2_package_DEP_ONLY">DEP_ONLY</a> }
</code></pre>



</details>

<a name="0x2_package_only_additive_upgrades"></a>

## Function `only_additive_upgrades`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_only_additive_upgrades">only_additive_upgrades</a>(cap: &<b>mut</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_only_additive_upgrades">only_additive_upgrades</a>(cap: &<b>mut</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a>) {
    <a href="../../dependencies/mgo-framework/package.md#0x2_package_restrict">restrict</a>(cap, <a href="../../dependencies/mgo-framework/package.md#0x2_package_ADDITIVE">ADDITIVE</a>)
}
</code></pre>



</details>

<a name="0x2_package_only_dep_upgrades"></a>

## Function `only_dep_upgrades`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_only_dep_upgrades">only_dep_upgrades</a>(cap: &<b>mut</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_only_dep_upgrades">only_dep_upgrades</a>(cap: &<b>mut</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a>) {
    <a href="../../dependencies/mgo-framework/package.md#0x2_package_restrict">restrict</a>(cap, <a href="../../dependencies/mgo-framework/package.md#0x2_package_DEP_ONLY">DEP_ONLY</a>)
}
</code></pre>



</details>

<a name="0x2_package_make_immutable"></a>

## Function `make_immutable`



<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_make_immutable">make_immutable</a>(cap: <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_make_immutable">make_immutable</a>(cap: <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a>) {
    <b>let</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a> { id, <a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>: _, version: _, policy: _ } = cap;
    <a href="../../dependencies/mgo-framework/object.md#0x2_object_delete">object::delete</a>(id);
}
</code></pre>



</details>

<a name="0x2_package_authorize_upgrade"></a>

## Function `authorize_upgrade`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_authorize_upgrade">authorize_upgrade</a>(cap: &<b>mut</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>, policy: u8, digest: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;): <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeTicket">package::UpgradeTicket</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_authorize_upgrade">authorize_upgrade</a>(
    cap: &<b>mut</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a>,
    policy: u8,
    digest: <a href="../../dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;
): <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeTicket">UpgradeTicket</a> {
    <b>let</b> id_zero = <a href="../../dependencies/mgo-framework/object.md#0x2_object_id_from_address">object::id_from_address</a>(@0x0);
    <b>assert</b>!(cap.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a> != id_zero, <a href="../../dependencies/mgo-framework/package.md#0x2_package_EAlreadyAuthorized">EAlreadyAuthorized</a>);
    <b>assert</b>!(policy &gt;= cap.policy, <a href="../../dependencies/mgo-framework/package.md#0x2_package_ETooPermissive">ETooPermissive</a>);

    <b>let</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a> = cap.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>;
    cap.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a> = id_zero;

    <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeTicket">UpgradeTicket</a> {
        cap: <a href="../../dependencies/mgo-framework/object.md#0x2_object_id">object::id</a>(cap),
        <a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>,
        policy,
        digest,
    }
}
</code></pre>



</details>

<a name="0x2_package_commit_upgrade"></a>

## Function `commit_upgrade`



<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_commit_upgrade">commit_upgrade</a>(cap: &<b>mut</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>, receipt: <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeReceipt">package::UpgradeReceipt</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_commit_upgrade">commit_upgrade</a>(
    cap: &<b>mut</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a>,
    receipt: <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeReceipt">UpgradeReceipt</a>,
) {
    <b>let</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeReceipt">UpgradeReceipt</a> { cap: cap_id, <a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a> } = receipt;

    <b>assert</b>!(<a href="../../dependencies/mgo-framework/object.md#0x2_object_id">object::id</a>(cap) == cap_id, <a href="../../dependencies/mgo-framework/package.md#0x2_package_EWrongUpgradeCap">EWrongUpgradeCap</a>);
    <b>assert</b>!(<a href="../../dependencies/mgo-framework/object.md#0x2_object_id_to_address">object::id_to_address</a>(&cap.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>) == @0x0, <a href="../../dependencies/mgo-framework/package.md#0x2_package_ENotAuthorized">ENotAuthorized</a>);

    cap.<a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a> = <a href="../../dependencies/mgo-framework/package.md#0x2_package">package</a>;
    cap.version = cap.version + 1;
}
</code></pre>



</details>

<a name="0x2_package_restrict"></a>

## Function `restrict`



<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_restrict">restrict</a>(cap: &<b>mut</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">package::UpgradeCap</a>, policy: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_restrict">restrict</a>(cap: &<b>mut</b> <a href="../../dependencies/mgo-framework/package.md#0x2_package_UpgradeCap">UpgradeCap</a>, policy: u8) {
    <b>assert</b>!(cap.policy &lt;= policy, <a href="../../dependencies/mgo-framework/package.md#0x2_package_ETooPermissive">ETooPermissive</a>);
    cap.policy = policy;
}
</code></pre>



</details>
