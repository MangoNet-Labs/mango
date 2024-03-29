
<a name="0x3_mgo_system"></a>

# Module `0x3::mgo_system`

Mgo System State Type Upgrade Guide
<code><a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a></code> is a thin wrapper around <code>MgoSystemStateInner</code> that provides a versioned interface.
The <code><a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a></code> object has a fixed ID 0x5, and the <code>MgoSystemStateInner</code> object is stored as a dynamic field.
There are a few different ways to upgrade the <code>MgoSystemStateInner</code> type:

The simplest and one that doesn't involve a real upgrade is to just add dynamic fields to the <code>extra_fields</code> field
of <code>MgoSystemStateInner</code> or any of its sub type. This is useful when we are in a rush, or making a small change,
or still experimenting a new field.

To properly upgrade the <code>MgoSystemStateInner</code> type, we need to ship a new framework that does the following:
1. Define a new <code>MgoSystemStateInner</code>type (e.g. <code>MgoSystemStateInnerV2</code>).
2. Define a data migration function that migrates the old <code>MgoSystemStateInner</code> to the new one (i.e. MgoSystemStateInnerV2).
3. Replace all uses of <code>MgoSystemStateInner</code> with <code>MgoSystemStateInnerV2</code> in both mgo_system.move and mgo_system_state_inner.move,
with the exception of the <code><a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_create">mgo_system_state_inner::create</a></code> function, which should always return the genesis type.
4. Inside <code>load_inner_maybe_upgrade</code> function, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade the inner object. Make sure to also update the version in the wrapper.
A detailed example can be found in mgo/tests/framework_upgrades/mock_mgo_systems/shallow_upgrade.
Along with the Move change, we also need to update the Rust code to support the new type. This includes:
1. Define a new <code>MgoSystemStateInner</code> struct type that matches the new Move type, and implement the MgoSystemStateTrait.
2. Update the <code><a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a></code> struct to include the new version as a new enum variant.
3. Update the <code>get_mgo_system_state</code> function to handle the new version.
To test that the upgrade will be successful, we need to modify <code>mgo_system_state_production_upgrade_test</code> test in
protocol_version_tests and trigger a real upgrade using the new framework. We will need to keep this directory as old version,
put the new framework in a new directory, and run the test to exercise the upgrade.

To upgrade Validator type, besides everything above, we also need to:
1. Define a new Validator type (e.g. ValidatorV2).
2. Define a data migration function that migrates the old Validator to the new one (i.e. ValidatorV2).
3. Replace all uses of Validator with ValidatorV2 except the genesis creation function.
4. In validator_wrapper::upgrade_to_latest, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade it.
In Rust, we also need to add a new case in <code>get_validator_from_table</code>.
Note that it is possible to upgrade MgoSystemStateInner without upgrading Validator, but not the other way around.
And when we only upgrade MgoSystemStateInner, the version of Validator in the wrapper will not be updated, and hence may become
inconsistent with the version of MgoSystemStateInner. This is fine as long as we don't use the Validator version to determine
the MgoSystemStateInner version, or vice versa.


-  [Resource `MgoSystemState`](#0x3_mgo_system_MgoSystemState)
-  [Constants](#@Constants_0)
-  [Function `create`](#0x3_mgo_system_create)
-  [Function `request_add_validator_candidate`](#0x3_mgo_system_request_add_validator_candidate)
-  [Function `request_remove_validator_candidate`](#0x3_mgo_system_request_remove_validator_candidate)
-  [Function `request_add_validator`](#0x3_mgo_system_request_add_validator)
-  [Function `request_remove_validator`](#0x3_mgo_system_request_remove_validator)
-  [Function `request_set_gas_price`](#0x3_mgo_system_request_set_gas_price)
-  [Function `set_candidate_validator_gas_price`](#0x3_mgo_system_set_candidate_validator_gas_price)
-  [Function `request_set_commission_rate`](#0x3_mgo_system_request_set_commission_rate)
-  [Function `set_candidate_validator_commission_rate`](#0x3_mgo_system_set_candidate_validator_commission_rate)
-  [Function `request_add_stake`](#0x3_mgo_system_request_add_stake)
-  [Function `request_add_stake_non_entry`](#0x3_mgo_system_request_add_stake_non_entry)
-  [Function `request_add_stake_mul_coin`](#0x3_mgo_system_request_add_stake_mul_coin)
-  [Function `request_withdraw_stake`](#0x3_mgo_system_request_withdraw_stake)
-  [Function `request_withdraw_stake_non_entry`](#0x3_mgo_system_request_withdraw_stake_non_entry)
-  [Function `report_validator`](#0x3_mgo_system_report_validator)
-  [Function `undo_report_validator`](#0x3_mgo_system_undo_report_validator)
-  [Function `rotate_operation_cap`](#0x3_mgo_system_rotate_operation_cap)
-  [Function `update_validator_name`](#0x3_mgo_system_update_validator_name)
-  [Function `update_validator_description`](#0x3_mgo_system_update_validator_description)
-  [Function `update_validator_image_url`](#0x3_mgo_system_update_validator_image_url)
-  [Function `update_validator_project_url`](#0x3_mgo_system_update_validator_project_url)
-  [Function `update_validator_next_epoch_network_address`](#0x3_mgo_system_update_validator_next_epoch_network_address)
-  [Function `update_candidate_validator_network_address`](#0x3_mgo_system_update_candidate_validator_network_address)
-  [Function `update_validator_next_epoch_p2p_address`](#0x3_mgo_system_update_validator_next_epoch_p2p_address)
-  [Function `update_candidate_validator_p2p_address`](#0x3_mgo_system_update_candidate_validator_p2p_address)
-  [Function `update_validator_next_epoch_primary_address`](#0x3_mgo_system_update_validator_next_epoch_primary_address)
-  [Function `update_candidate_validator_primary_address`](#0x3_mgo_system_update_candidate_validator_primary_address)
-  [Function `update_validator_next_epoch_worker_address`](#0x3_mgo_system_update_validator_next_epoch_worker_address)
-  [Function `update_candidate_validator_worker_address`](#0x3_mgo_system_update_candidate_validator_worker_address)
-  [Function `update_validator_next_epoch_protocol_pubkey`](#0x3_mgo_system_update_validator_next_epoch_protocol_pubkey)
-  [Function `update_candidate_validator_protocol_pubkey`](#0x3_mgo_system_update_candidate_validator_protocol_pubkey)
-  [Function `update_validator_next_epoch_worker_pubkey`](#0x3_mgo_system_update_validator_next_epoch_worker_pubkey)
-  [Function `update_candidate_validator_worker_pubkey`](#0x3_mgo_system_update_candidate_validator_worker_pubkey)
-  [Function `update_validator_next_epoch_network_pubkey`](#0x3_mgo_system_update_validator_next_epoch_network_pubkey)
-  [Function `update_candidate_validator_network_pubkey`](#0x3_mgo_system_update_candidate_validator_network_pubkey)
-  [Function `pool_exchange_rates`](#0x3_mgo_system_pool_exchange_rates)
-  [Function `active_validator_addresses`](#0x3_mgo_system_active_validator_addresses)
-  [Function `advance_epoch`](#0x3_mgo_system_advance_epoch)
-  [Function `load_system_state`](#0x3_mgo_system_load_system_state)
-  [Function `load_system_state_mut`](#0x3_mgo_system_load_system_state_mut)
-  [Function `load_inner_maybe_upgrade`](#0x3_mgo_system_load_inner_maybe_upgrade)


<pre><code><b>use</b> <a href="dependencies/move-stdlib/option.md#0x1_option">0x1::option</a>;
<b>use</b> <a href="dependencies/mgo-framework/balance.md#0x2_balance">0x2::balance</a>;
<b>use</b> <a href="dependencies/mgo-framework/coin.md#0x2_coin">0x2::coin</a>;
<b>use</b> <a href="dependencies/mgo-framework/dynamic_field.md#0x2_dynamic_field">0x2::dynamic_field</a>;
<b>use</b> <a href="dependencies/mgo-framework/mgo.md#0x2_mgo">0x2::mgo</a>;
<b>use</b> <a href="dependencies/mgo-framework/object.md#0x2_object">0x2::object</a>;
<b>use</b> <a href="dependencies/mgo-framework/table.md#0x2_table">0x2::table</a>;
<b>use</b> <a href="dependencies/mgo-framework/transfer.md#0x2_transfer">0x2::transfer</a>;
<b>use</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context">0x2::tx_context</a>;
<b>use</b> <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner">0x3::mgo_system_state_inner</a>;
<b>use</b> <a href="stake_subsidy.md#0x3_stake_subsidy">0x3::stake_subsidy</a>;
<b>use</b> <a href="staking_pool.md#0x3_staking_pool">0x3::staking_pool</a>;
<b>use</b> <a href="validator.md#0x3_validator">0x3::validator</a>;
<b>use</b> <a href="validator_cap.md#0x3_validator_cap">0x3::validator_cap</a>;
</code></pre>



<a name="0x3_mgo_system_MgoSystemState"></a>

## Resource `MgoSystemState`



<pre><code><b>struct</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a> <b>has</b> key
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
<code>version: u64</code>
</dt>
<dd>

</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="0x3_mgo_system_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="mgo_system.md#0x3_mgo_system_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="0x3_mgo_system_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="mgo_system.md#0x3_mgo_system_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 1;
</code></pre>



<a name="0x3_mgo_system_create"></a>

## Function `create`

Create a new MgoSystemState object and make it shared.
This function will be called only once in genesis.


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_create">create</a>(id: <a href="dependencies/mgo-framework/object.md#0x2_object_UID">object::UID</a>, validators: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="validator.md#0x3_validator_Validator">validator::Validator</a>&gt;, <a href="storage_fund.md#0x3_storage_fund">storage_fund</a>: <a href="dependencies/mgo-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="dependencies/mgo-framework/mgo.md#0x2_mgo_MGO">mgo::MGO</a>&gt;, protocol_version: u64, epoch_start_timestamp_ms: u64, parameters: <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_SystemParameters">mgo_system_state_inner::SystemParameters</a>, <a href="stake_subsidy.md#0x3_stake_subsidy">stake_subsidy</a>: <a href="stake_subsidy.md#0x3_stake_subsidy_StakeSubsidy">stake_subsidy::StakeSubsidy</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(<b>friend</b>) <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_create">create</a>(
    id: UID,
    validators: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;Validator&gt;,
    <a href="storage_fund.md#0x3_storage_fund">storage_fund</a>: Balance&lt;MGO&gt;,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    parameters: SystemParameters,
    <a href="stake_subsidy.md#0x3_stake_subsidy">stake_subsidy</a>: StakeSubsidy,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_create">mgo_system_state_inner::create</a>(
        validators,
        <a href="storage_fund.md#0x3_storage_fund">storage_fund</a>,
        protocol_version,
        epoch_start_timestamp_ms,
        parameters,
        <a href="stake_subsidy.md#0x3_stake_subsidy">stake_subsidy</a>,
        ctx,
    );
    <b>let</b> version = <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_genesis_system_state_version">mgo_system_state_inner::genesis_system_state_version</a>();
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a> {
        id,
        version,
    };
    <a href="dependencies/mgo-framework/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id, version, system_state);
    <a href="dependencies/mgo-framework/transfer.md#0x2_transfer_share_object">transfer::share_object</a>(self);
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Can be called by anyone who wishes to become a validator candidate and starts accuring delegated
stakes in their staking pool. Once they have at least <code>MIN_VALIDATOR_JOINING_STAKE</code> amount of stake they
can call <code>request_add_validator</code> to officially become an active validator at the next epoch.
Aborts if the caller is already a pending or active validator, or a validator candidate.
Note: <code>proof_of_possession</code> MUST be a valid signature using mgo_address and protocol_pubkey_bytes.
To produce a valid PoP, run [fn test_proof_of_possession].


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_add_validator_candidate">request_add_validator_candidate</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, pubkey_bytes: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, network_pubkey_bytes: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, worker_pubkey_bytes: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, name: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, description: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, image_url: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, project_url: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, net_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, p2p_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, primary_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, worker_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, gas_price: u64, commission_rate: u64, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_add_validator_candidate">request_add_validator_candidate</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    pubkey_bytes: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    network_pubkey_bytes: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    worker_pubkey_bytes: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    name: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    description: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    image_url: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    project_url: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    net_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    p2p_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    primary_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    worker_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    gas_price: u64,
    commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_request_add_validator_candidate">mgo_system_state_inner::request_add_validator_candidate</a>(
        self,
        pubkey_bytes,
        network_pubkey_bytes,
        worker_pubkey_bytes,
        proof_of_possession,
        name,
        description,
        image_url,
        project_url,
        net_address,
        p2p_address,
        primary_address,
        worker_address,
        gas_price,
        commission_rate,
        ctx,
    )
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by a validator candidate to remove themselves from the candidacy. After this call
their staking pool becomes deactivate.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_request_remove_validator_candidate">mgo_system_state_inner::request_remove_validator_candidate</a>(self, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_add_validator"></a>

## Function `request_add_validator`

Called by a validator candidate to add themselves to the active validator set beginning next epoch.
Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
epoch has already reached the maximum.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_add_validator">request_add_validator</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_add_validator">request_add_validator</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_request_add_validator">mgo_system_state_inner::request_add_validator</a>(self, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_remove_validator"></a>

## Function `request_remove_validator`

A validator can call this function to request a removal in the next epoch.
We use the sender of <code>ctx</code> to look up the validator
(i.e. sender must match the mgo_address in the validator).
At the end of the epoch, the <code><a href="validator.md#0x3_validator">validator</a></code> object will be returned to the mgo_address
of the validator.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_remove_validator">request_remove_validator</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_remove_validator">request_remove_validator</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_request_remove_validator">mgo_system_state_inner::request_remove_validator</a>(self, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_set_gas_price"></a>

## Function `request_set_gas_price`

A validator can call this entry function to submit a new gas price quote, to be
used for the reference gas price calculation at the end of the epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_set_gas_price">request_set_gas_price</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, cap: &<a href="validator_cap.md#0x3_validator_cap_UnverifiedValidatorOperationCap">validator_cap::UnverifiedValidatorOperationCap</a>, new_gas_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_set_gas_price">request_set_gas_price</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    new_gas_price: u64,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_request_set_gas_price">mgo_system_state_inner::request_set_gas_price</a>(self, cap, new_gas_price)
}
</code></pre>



</details>

<a name="0x3_mgo_system_set_candidate_validator_gas_price"></a>

## Function `set_candidate_validator_gas_price`

This entry function is used to set new gas price for candidate validators


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, cap: &<a href="validator_cap.md#0x3_validator_cap_UnverifiedValidatorOperationCap">validator_cap::UnverifiedValidatorOperationCap</a>, new_gas_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    new_gas_price: u64,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_set_candidate_validator_gas_price">mgo_system_state_inner::set_candidate_validator_gas_price</a>(self, cap, new_gas_price)
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

A validator can call this entry function to set a new commission rate, updated at the end of
the epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_set_commission_rate">request_set_commission_rate</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, new_commission_rate: u64, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_set_commission_rate">request_set_commission_rate</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    new_commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_request_set_commission_rate">mgo_system_state_inner::request_set_commission_rate</a>(self, new_commission_rate, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_set_candidate_validator_commission_rate"></a>

## Function `set_candidate_validator_commission_rate`

This entry function is used to set new commission rate for candidate validators


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, new_commission_rate: u64, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    new_commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_set_candidate_validator_commission_rate">mgo_system_state_inner::set_candidate_validator_commission_rate</a>(self, new_commission_rate, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_add_stake"></a>

## Function `request_add_stake`

Add stake to a validator's staking pool.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_add_stake">request_add_stake</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, stake: <a href="dependencies/mgo-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="dependencies/mgo-framework/mgo.md#0x2_mgo_MGO">mgo::MGO</a>&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_add_stake">request_add_stake</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    stake: Coin&lt;MGO&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> staked_mgo = <a href="mgo_system.md#0x3_mgo_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(wrapper, stake, validator_address, ctx);
    <a href="dependencies/mgo-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(staked_mgo, <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_add_stake_non_entry"></a>

## Function `request_add_stake_non_entry`

The non-entry version of <code>request_add_stake</code>, which returns the staked MGO instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, stake: <a href="dependencies/mgo-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="dependencies/mgo-framework/mgo.md#0x2_mgo_MGO">mgo::MGO</a>&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="staking_pool.md#0x3_staking_pool_StakedMgo">staking_pool::StakedMgo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    stake: Coin&lt;MGO&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
): StakedMgo {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_request_add_stake">mgo_system_state_inner::request_add_stake</a>(self, stake, validator_address, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_add_stake_mul_coin"></a>

## Function `request_add_stake_mul_coin`

Add stake to a validator's staking pool using multiple coins.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, stakes: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<a href="dependencies/mgo-framework/coin.md#0x2_coin_Coin">coin::Coin</a>&lt;<a href="dependencies/mgo-framework/mgo.md#0x2_mgo_MGO">mgo::MGO</a>&gt;&gt;, stake_amount: <a href="dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    stakes: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;Coin&lt;MGO&gt;&gt;,
    stake_amount: <a href="dependencies/move-stdlib/option.md#0x1_option_Option">option::Option</a>&lt;u64&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <b>let</b> staked_mgo = <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_request_add_stake_mul_coin">mgo_system_state_inner::request_add_stake_mul_coin</a>(self, stakes, stake_amount, validator_address, ctx);
    <a href="dependencies/mgo-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(staked_mgo, <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Withdraw stake from a validator's staking pool.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_withdraw_stake">request_withdraw_stake</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, staked_mgo: <a href="staking_pool.md#0x3_staking_pool_StakedMgo">staking_pool::StakedMgo</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_withdraw_stake">request_withdraw_stake</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    staked_mgo: StakedMgo,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> withdrawn_stake = <a href="mgo_system.md#0x3_mgo_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(wrapper, staked_mgo, ctx);
    <a href="dependencies/mgo-framework/transfer.md#0x2_transfer_public_transfer">transfer::public_transfer</a>(<a href="dependencies/mgo-framework/coin.md#0x2_coin_from_balance">coin::from_balance</a>(withdrawn_stake, ctx), <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx));
}
</code></pre>



</details>

<a name="0x3_mgo_system_request_withdraw_stake_non_entry"></a>

## Function `request_withdraw_stake_non_entry`

Non-entry version of <code>request_withdraw_stake</code> that returns the withdrawn MGO instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, staked_mgo: <a href="staking_pool.md#0x3_staking_pool_StakedMgo">staking_pool::StakedMgo</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dependencies/mgo-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="dependencies/mgo-framework/mgo.md#0x2_mgo_MGO">mgo::MGO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    staked_mgo: StakedMgo,
    ctx: &<b>mut</b> TxContext,
) : Balance&lt;MGO&gt; {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_request_withdraw_stake">mgo_system_state_inner::request_withdraw_stake</a>(self, staked_mgo, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_report_validator"></a>

## Function `report_validator`

Report a validator as a bad or non-performant actor in the system.
Succeeds if all the following are satisfied:
1. both the reporter in <code>cap</code> and the input <code>reportee_addr</code> are active validators.
2. reporter and reportee not the same address.
3. the cap object is still valid.
This function is idempotent.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_report_validator">report_validator</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, cap: &<a href="validator_cap.md#0x3_validator_cap_UnverifiedValidatorOperationCap">validator_cap::UnverifiedValidatorOperationCap</a>, reportee_addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_report_validator">report_validator</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    reportee_addr: <b>address</b>,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_report_validator">mgo_system_state_inner::report_validator</a>(self, cap, reportee_addr)
}
</code></pre>



</details>

<a name="0x3_mgo_system_undo_report_validator"></a>

## Function `undo_report_validator`

Undo a <code>report_validator</code> action. Aborts if
1. the reportee is not a currently active validator or
2. the sender has not previously reported the <code>reportee_addr</code>, or
3. the cap is not valid


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_undo_report_validator">undo_report_validator</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, cap: &<a href="validator_cap.md#0x3_validator_cap_UnverifiedValidatorOperationCap">validator_cap::UnverifiedValidatorOperationCap</a>, reportee_addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_undo_report_validator">undo_report_validator</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    reportee_addr: <b>address</b>,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_undo_report_validator">mgo_system_state_inner::undo_report_validator</a>(self, cap, reportee_addr)
}
</code></pre>



</details>

<a name="0x3_mgo_system_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>UnverifiedValidatorOperationCap</code>, transfer it to the
validator and registers it. The original object is thus revoked.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_rotate_operation_cap">rotate_operation_cap</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_rotate_operation_cap">mgo_system_state_inner::rotate_operation_cap</a>(self, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_name"></a>

## Function `update_validator_name`

Update a validator's name.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_name">update_validator_name</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, name: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_name">update_validator_name</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    name: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_name">mgo_system_state_inner::update_validator_name</a>(self, name, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_description"></a>

## Function `update_validator_description`

Update a validator's description


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_description">update_validator_description</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, description: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_description">update_validator_description</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    description: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_description">mgo_system_state_inner::update_validator_description</a>(self, description, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_image_url"></a>

## Function `update_validator_image_url`

Update a validator's image url


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_image_url">update_validator_image_url</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, image_url: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_image_url">update_validator_image_url</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    image_url: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_image_url">mgo_system_state_inner::update_validator_image_url</a>(self, image_url, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_project_url"></a>

## Function `update_validator_project_url`

Update a validator's project url


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_project_url">update_validator_project_url</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, project_url: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_project_url">update_validator_project_url</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    project_url: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_project_url">mgo_system_state_inner::update_validator_project_url</a>(self, project_url, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_next_epoch_network_address"></a>

## Function `update_validator_next_epoch_network_address`

Update a validator's network address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, network_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    network_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_next_epoch_network_address">mgo_system_state_inner::update_validator_next_epoch_network_address</a>(self, network_address, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_candidate_validator_network_address"></a>

## Function `update_candidate_validator_network_address`

Update candidate validator's network address.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, network_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    network_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_candidate_validator_network_address">mgo_system_state_inner::update_candidate_validator_network_address</a>(self, network_address, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_next_epoch_p2p_address"></a>

## Function `update_validator_next_epoch_p2p_address`

Update a validator's p2p address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, p2p_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    p2p_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_next_epoch_p2p_address">mgo_system_state_inner::update_validator_next_epoch_p2p_address</a>(self, p2p_address, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_candidate_validator_p2p_address"></a>

## Function `update_candidate_validator_p2p_address`

Update candidate validator's p2p address.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, p2p_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    p2p_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_candidate_validator_p2p_address">mgo_system_state_inner::update_candidate_validator_p2p_address</a>(self, p2p_address, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_next_epoch_primary_address"></a>

## Function `update_validator_next_epoch_primary_address`

Update a validator's narwhal primary address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, primary_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    primary_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_next_epoch_primary_address">mgo_system_state_inner::update_validator_next_epoch_primary_address</a>(self, primary_address, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_candidate_validator_primary_address"></a>

## Function `update_candidate_validator_primary_address`

Update candidate validator's narwhal primary address.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, primary_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    primary_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_candidate_validator_primary_address">mgo_system_state_inner::update_candidate_validator_primary_address</a>(self, primary_address, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_next_epoch_worker_address"></a>

## Function `update_validator_next_epoch_worker_address`

Update a validator's narwhal worker address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, worker_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    worker_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_next_epoch_worker_address">mgo_system_state_inner::update_validator_next_epoch_worker_address</a>(self, worker_address, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_candidate_validator_worker_address"></a>

## Function `update_candidate_validator_worker_address`

Update candidate validator's narwhal worker address.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, worker_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    worker_address: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_candidate_validator_worker_address">mgo_system_state_inner::update_candidate_validator_worker_address</a>(self, worker_address, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_next_epoch_protocol_pubkey"></a>

## Function `update_validator_next_epoch_protocol_pubkey`

Update a validator's public key of protocol key and proof of possession.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, protocol_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    protocol_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_next_epoch_protocol_pubkey">mgo_system_state_inner::update_validator_next_epoch_protocol_pubkey</a>(self, protocol_pubkey, proof_of_possession, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_candidate_validator_protocol_pubkey"></a>

## Function `update_candidate_validator_protocol_pubkey`

Update candidate validator's public key of protocol key and proof of possession.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, protocol_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, proof_of_possession: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    protocol_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    proof_of_possession: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_candidate_validator_protocol_pubkey">mgo_system_state_inner::update_candidate_validator_protocol_pubkey</a>(self, protocol_pubkey, proof_of_possession, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_next_epoch_worker_pubkey"></a>

## Function `update_validator_next_epoch_worker_pubkey`

Update a validator's public key of worker key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, worker_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    worker_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_next_epoch_worker_pubkey">mgo_system_state_inner::update_validator_next_epoch_worker_pubkey</a>(self, worker_pubkey, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_candidate_validator_worker_pubkey"></a>

## Function `update_candidate_validator_worker_pubkey`

Update candidate validator's public key of worker key.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, worker_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    worker_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_candidate_validator_worker_pubkey">mgo_system_state_inner::update_candidate_validator_worker_pubkey</a>(self, worker_pubkey, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_validator_next_epoch_network_pubkey"></a>

## Function `update_validator_next_epoch_network_pubkey`

Update a validator's public key of network key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, network_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    network_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_validator_next_epoch_network_pubkey">mgo_system_state_inner::update_validator_next_epoch_network_pubkey</a>(self, network_pubkey, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_update_candidate_validator_network_pubkey"></a>

## Function `update_candidate_validator_network_pubkey`

Update candidate validator's public key of network key.


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, network_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;, ctx: &<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> entry <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(
    self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    network_pubkey: <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;u8&gt;,
    ctx: &TxContext,
) {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_update_candidate_validator_network_pubkey">mgo_system_state_inner::update_candidate_validator_network_pubkey</a>(self, network_pubkey, ctx)
}
</code></pre>



</details>

<a name="0x3_mgo_system_pool_exchange_rates"></a>

## Function `pool_exchange_rates`

Getter of the pool token exchange rate of a staking pool. Works for both active and inactive pools.


<pre><code><b>public</b> <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_pool_exchange_rates">pool_exchange_rates</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, pool_id: &<a href="dependencies/mgo-framework/object.md#0x2_object_ID">object::ID</a>): &<a href="dependencies/mgo-framework/table.md#0x2_table_Table">table::Table</a>&lt;u64, <a href="staking_pool.md#0x3_staking_pool_PoolTokenExchangeRate">staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_pool_exchange_rates">pool_exchange_rates</a>(
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    pool_id: &ID
): &Table&lt;u64, PoolTokenExchangeRate&gt;  {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_pool_exchange_rates">mgo_system_state_inner::pool_exchange_rates</a>(self, pool_id)
}
</code></pre>



</details>

<a name="0x3_mgo_system_active_validator_addresses"></a>

## Function `active_validator_addresses`

Getter returning addresses of the currently active validators.


<pre><code><b>public</b> <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_active_validator_addresses">active_validator_addresses</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="mgo_system.md#0x3_mgo_system_active_validator_addresses">active_validator_addresses</a>(wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>): <a href="dependencies/move-stdlib/vector.md#0x1_vector">vector</a>&lt;<b>address</b>&gt; {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state">load_system_state</a>(wrapper);
    <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_active_validator_addresses">mgo_system_state_inner::active_validator_addresses</a>(self)
}
</code></pre>



</details>

<a name="0x3_mgo_system_advance_epoch"></a>

## Function `advance_epoch`

This function should be called at the end of an epoch, and advances the system to the next epoch.
It does the following things:
1. Add storage charge to the storage fund.
2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
gas coins.
3. Distribute computation charge to validator stake.
4. Update all validators.


<pre><code><b>fun</b> <a href="mgo_system.md#0x3_mgo_system_advance_epoch">advance_epoch</a>(storage_reward: <a href="dependencies/mgo-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="dependencies/mgo-framework/mgo.md#0x2_mgo_MGO">mgo::MGO</a>&gt;, computation_reward: <a href="dependencies/mgo-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="dependencies/mgo-framework/mgo.md#0x2_mgo_MGO">mgo::MGO</a>&gt;, wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>, new_epoch: u64, next_protocol_version: u64, storage_rebate: u64, non_refundable_storage_fee: u64, storage_fund_reinvest_rate: u64, reward_slashing_rate: u64, epoch_start_timestamp_ms: u64, ctx: &<b>mut</b> <a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_TxContext">tx_context::TxContext</a>): <a href="dependencies/mgo-framework/balance.md#0x2_balance_Balance">balance::Balance</a>&lt;<a href="dependencies/mgo-framework/mgo.md#0x2_mgo_MGO">mgo::MGO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="mgo_system.md#0x3_mgo_system_advance_epoch">advance_epoch</a>(
    storage_reward: Balance&lt;MGO&gt;,
    computation_reward: Balance&lt;MGO&gt;,
    wrapper: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>,
    new_epoch: u64,
    next_protocol_version: u64,
    storage_rebate: u64,
    non_refundable_storage_fee: u64,
    storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
                                     // into storage fund, in basis point.
    reward_slashing_rate: u64, // how much rewards are slashed <b>to</b> punish a <a href="validator.md#0x3_validator">validator</a>, in bps.
    epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
    ctx: &<b>mut</b> TxContext,
) : Balance&lt;MGO&gt; {
    <b>let</b> self = <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(wrapper);
    // Validator will make a special system call <b>with</b> sender set <b>as</b> 0x0.
    <b>assert</b>!(<a href="dependencies/mgo-framework/tx_context.md#0x2_tx_context_sender">tx_context::sender</a>(ctx) == @0x0, <a href="mgo_system.md#0x3_mgo_system_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> storage_rebate = <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_advance_epoch">mgo_system_state_inner::advance_epoch</a>(
        self,
        new_epoch,
        next_protocol_version,
        storage_reward,
        computation_reward,
        storage_rebate,
        non_refundable_storage_fee,
        storage_fund_reinvest_rate,
        reward_slashing_rate,
        epoch_start_timestamp_ms,
        ctx,
    );

    storage_rebate
}
</code></pre>



</details>

<a name="0x3_mgo_system_load_system_state"></a>

## Function `load_system_state`



<pre><code><b>fun</b> <a href="mgo_system.md#0x3_mgo_system_load_system_state">load_system_state</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>): &<a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_MgoSystemStateInnerV2">mgo_system_state_inner::MgoSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="mgo_system.md#0x3_mgo_system_load_system_state">load_system_state</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>): &MgoSystemStateInnerV2 {
    <a href="mgo_system.md#0x3_mgo_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self)
}
</code></pre>



</details>

<a name="0x3_mgo_system_load_system_state_mut"></a>

## Function `load_system_state_mut`



<pre><code><b>fun</b> <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>): &<b>mut</b> <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_MgoSystemStateInnerV2">mgo_system_state_inner::MgoSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="mgo_system.md#0x3_mgo_system_load_system_state_mut">load_system_state_mut</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>): &<b>mut</b> MgoSystemStateInnerV2 {
    <a href="mgo_system.md#0x3_mgo_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self)
}
</code></pre>



</details>

<a name="0x3_mgo_system_load_inner_maybe_upgrade"></a>

## Function `load_inner_maybe_upgrade`



<pre><code><b>fun</b> <a href="mgo_system.md#0x3_mgo_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">mgo_system::MgoSystemState</a>): &<b>mut</b> <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_MgoSystemStateInnerV2">mgo_system_state_inner::MgoSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="mgo_system.md#0x3_mgo_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self: &<b>mut</b> <a href="mgo_system.md#0x3_mgo_system_MgoSystemState">MgoSystemState</a>): &<b>mut</b> MgoSystemStateInnerV2 {
    <b>if</b> (self.version == 1) {
      <b>let</b> v1 = <a href="dependencies/mgo-framework/dynamic_field.md#0x2_dynamic_field_remove">dynamic_field::remove</a>(&<b>mut</b> self.id, self.version);
      <b>let</b> v2 = <a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_v1_to_v2">mgo_system_state_inner::v1_to_v2</a>(v1);
      self.version = 2;
      <a href="dependencies/mgo-framework/dynamic_field.md#0x2_dynamic_field_add">dynamic_field::add</a>(&<b>mut</b> self.id, self.version, v2);
    };

    <b>let</b> inner = <a href="dependencies/mgo-framework/dynamic_field.md#0x2_dynamic_field_borrow_mut">dynamic_field::borrow_mut</a>(&<b>mut</b> self.id, self.version);
    <b>assert</b>!(<a href="mgo_system_state_inner.md#0x3_mgo_system_state_inner_system_state_version">mgo_system_state_inner::system_state_version</a>(inner) == self.version, <a href="mgo_system.md#0x3_mgo_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>
