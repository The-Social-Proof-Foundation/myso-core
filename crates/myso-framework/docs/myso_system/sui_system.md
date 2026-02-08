---
title: Module `myso_system::myso_system`
---

MySo System State Type Upgrade Guide
<code><a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a></code> is a thin wrapper around <code>MySoSystemStateInner</code> that provides a versioned interface.
The <code><a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a></code> object has a fixed ID 0x5, and the <code>MySoSystemStateInner</code> object is stored as a dynamic field.
There are a few different ways to upgrade the <code>MySoSystemStateInner</code> type:

The simplest and one that doesn't involve a real upgrade is to just add dynamic fields to the <code>extra_fields</code> field
of <code>MySoSystemStateInner</code> or any of its sub type. This is useful when we are in a rush, or making a small change,
or still experimenting a new field.

To properly upgrade the <code>MySoSystemStateInner</code> type, we need to ship a new framework that does the following:
1. Define a new <code>MySoSystemStateInner</code>type (e.g. <code>MySoSystemStateInnerV2</code>).
2. Define a data migration function that migrates the old <code>MySoSystemStateInner</code> to the new one (i.e. MySoSystemStateInnerV2).
3. Replace all uses of <code>MySoSystemStateInner</code> with <code>MySoSystemStateInnerV2</code> in both myso_system.move and myso_system_state_inner.move,
with the exception of the <code><a href="../myso_system/sui_system_state_inner.md#myso_system_myso_system_state_inner_create">myso_system_state_inner::create</a></code> function, which should always return the genesis type.
4. Inside <code><a href="../myso_system/sui_system.md#myso_system_myso_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a></code> function, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade the inner object. Make sure to also update the version in the wrapper.
A detailed example can be found in myso/tests/framework_upgrades/mock_myso_systems/shallow_upgrade.
Along with the Move change, we also need to update the Rust code to support the new type. This includes:
1. Define a new <code>MySoSystemStateInner</code> struct type that matches the new Move type, and implement the MySoSystemStateTrait.
2. Update the <code><a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a></code> struct to include the new version as a new enum variant.
3. Update the <code>get_myso_system_state</code> function to handle the new version.
To test that the upgrade will be successful, we need to modify <code>myso_system_state_production_upgrade_test</code> test in
protocol_version_tests and trigger a real upgrade using the new framework. We will need to keep this directory as old version,
put the new framework in a new directory, and run the test to exercise the upgrade.

To upgrade Validator type, besides everything above, we also need to:
1. Define a new Validator type (e.g. ValidatorV2).
2. Define a data migration function that migrates the old Validator to the new one (i.e. ValidatorV2).
3. Replace all uses of Validator with ValidatorV2 except the genesis creation function.
4. In validator_wrapper::upgrade_to_latest, check the current version in the wrapper, and if it's not the latest version,
call the data migration function to upgrade it.
In Rust, we also need to add a new case in <code>get_validator_from_table</code>.
Note that it is possible to upgrade MySoSystemStateInner without upgrading Validator, but not the other way around.
And when we only upgrade MySoSystemStateInner, the version of Validator in the wrapper will not be updated, and hence may become
inconsistent with the version of MySoSystemStateInner. This is fine as long as we don't use the Validator version to determine
the MySoSystemStateInner version, or vice versa.


-  [Struct `MySoSystemState`](#myso_system_myso_system_MySoSystemState)
-  [Struct `AccumulatorStorageCostKey`](#myso_system_myso_system_AccumulatorStorageCostKey)
-  [Constants](#@Constants_0)
-  [Function `create`](#myso_system_myso_system_create)
-  [Function `request_add_validator_candidate`](#myso_system_myso_system_request_add_validator_candidate)
-  [Function `request_remove_validator_candidate`](#myso_system_myso_system_request_remove_validator_candidate)
-  [Function `request_add_validator`](#myso_system_myso_system_request_add_validator)
-  [Function `request_remove_validator`](#myso_system_myso_system_request_remove_validator)
-  [Function `request_set_gas_price`](#myso_system_myso_system_request_set_gas_price)
-  [Function `set_candidate_validator_gas_price`](#myso_system_myso_system_set_candidate_validator_gas_price)
-  [Function `request_set_commission_rate`](#myso_system_myso_system_request_set_commission_rate)
-  [Function `set_candidate_validator_commission_rate`](#myso_system_myso_system_set_candidate_validator_commission_rate)
-  [Function `request_add_stake`](#myso_system_myso_system_request_add_stake)
-  [Function `request_add_stake_non_entry`](#myso_system_myso_system_request_add_stake_non_entry)
-  [Function `request_add_stake_mul_coin`](#myso_system_myso_system_request_add_stake_mul_coin)
-  [Function `request_withdraw_stake`](#myso_system_myso_system_request_withdraw_stake)
-  [Function `convert_to_fungible_staked_myso`](#myso_system_myso_system_convert_to_fungible_staked_myso)
-  [Function `redeem_fungible_staked_myso`](#myso_system_myso_system_redeem_fungible_staked_myso)
-  [Function `request_withdraw_stake_non_entry`](#myso_system_myso_system_request_withdraw_stake_non_entry)
-  [Function `report_validator`](#myso_system_myso_system_report_validator)
-  [Function `undo_report_validator`](#myso_system_myso_system_undo_report_validator)
-  [Function `rotate_operation_cap`](#myso_system_myso_system_rotate_operation_cap)
-  [Function `update_validator_name`](#myso_system_myso_system_update_validator_name)
-  [Function `update_validator_description`](#myso_system_myso_system_update_validator_description)
-  [Function `update_validator_image_url`](#myso_system_myso_system_update_validator_image_url)
-  [Function `update_validator_project_url`](#myso_system_myso_system_update_validator_project_url)
-  [Function `update_validator_next_epoch_network_address`](#myso_system_myso_system_update_validator_next_epoch_network_address)
-  [Function `update_candidate_validator_network_address`](#myso_system_myso_system_update_candidate_validator_network_address)
-  [Function `update_validator_next_epoch_p2p_address`](#myso_system_myso_system_update_validator_next_epoch_p2p_address)
-  [Function `update_candidate_validator_p2p_address`](#myso_system_myso_system_update_candidate_validator_p2p_address)
-  [Function `update_validator_next_epoch_primary_address`](#myso_system_myso_system_update_validator_next_epoch_primary_address)
-  [Function `update_candidate_validator_primary_address`](#myso_system_myso_system_update_candidate_validator_primary_address)
-  [Function `update_validator_next_epoch_worker_address`](#myso_system_myso_system_update_validator_next_epoch_worker_address)
-  [Function `update_candidate_validator_worker_address`](#myso_system_myso_system_update_candidate_validator_worker_address)
-  [Function `update_validator_next_epoch_protocol_pubkey`](#myso_system_myso_system_update_validator_next_epoch_protocol_pubkey)
-  [Function `update_candidate_validator_protocol_pubkey`](#myso_system_myso_system_update_candidate_validator_protocol_pubkey)
-  [Function `update_validator_next_epoch_worker_pubkey`](#myso_system_myso_system_update_validator_next_epoch_worker_pubkey)
-  [Function `update_candidate_validator_worker_pubkey`](#myso_system_myso_system_update_candidate_validator_worker_pubkey)
-  [Function `update_validator_next_epoch_network_pubkey`](#myso_system_myso_system_update_validator_next_epoch_network_pubkey)
-  [Function `update_candidate_validator_network_pubkey`](#myso_system_myso_system_update_candidate_validator_network_pubkey)
-  [Function `validator_address_by_pool_id`](#myso_system_myso_system_validator_address_by_pool_id)
-  [Function `pool_exchange_rates`](#myso_system_myso_system_pool_exchange_rates)
-  [Function `active_validator_addresses`](#myso_system_myso_system_active_validator_addresses)
-  [Function `active_validator_addresses_ref`](#myso_system_myso_system_active_validator_addresses_ref)
-  [Function `active_validator_voting_powers`](#myso_system_myso_system_active_validator_voting_powers)
-  [Function `calculate_rewards`](#myso_system_myso_system_calculate_rewards)
-  [Function `advance_epoch`](#myso_system_myso_system_advance_epoch)
-  [Function `load_system_state`](#myso_system_myso_system_load_system_state)
-  [Function `load_system_state_mut`](#myso_system_myso_system_load_system_state_mut)
-  [Function `load_system_state_ref`](#myso_system_myso_system_load_system_state_ref)
-  [Function `load_inner_maybe_upgrade`](#myso_system_myso_system_load_inner_maybe_upgrade)
-  [Function `validator_voting_powers`](#myso_system_myso_system_validator_voting_powers)
-  [Function `store_execution_time_estimates`](#myso_system_myso_system_store_execution_time_estimates)
-  [Function `store_execution_time_estimates_v2`](#myso_system_myso_system_store_execution_time_estimates_v2)
-  [Function `get_accumulator_storage_fund_amount`](#myso_system_myso_system_get_accumulator_storage_fund_amount)
-  [Function `write_accumulator_storage_cost`](#myso_system_myso_system_write_accumulator_storage_cost)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/sui.md#myso_myso">myso::myso</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/priority_queue.md#myso_priority_queue">myso::priority_queue</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/table_vec.md#myso_table_vec">myso::table_vec</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../myso/versioned.md#myso_versioned">myso::versioned</a>;
<b>use</b> <a href="../myso_system/sui_system_state_inner.md#myso_system_myso_system_state_inner">myso_system::myso_system_state_inner</a>;
<b>use</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy">myso_system::stake_subsidy</a>;
<b>use</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool">myso_system::staking_pool</a>;
<b>use</b> <a href="../myso_system/storage_fund.md#myso_system_storage_fund">myso_system::storage_fund</a>;
<b>use</b> <a href="../myso_system/validator.md#myso_system_validator">myso_system::validator</a>;
<b>use</b> <a href="../myso_system/validator_cap.md#myso_system_validator_cap">myso_system::validator_cap</a>;
<b>use</b> <a href="../myso_system/validator_set.md#myso_system_validator_set">myso_system::validator_set</a>;
<b>use</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper">myso_system::validator_wrapper</a>;
<b>use</b> <a href="../myso_system/voting_power.md#myso_system_voting_power">myso_system::voting_power</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="myso_system_myso_system_MySoSystemState"></a>

## Struct `MySoSystemState`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
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

<a name="myso_system_myso_system_AccumulatorStorageCostKey"></a>

## Struct `AccumulatorStorageCostKey`

Key for storing the storage cost for accumulator objects, computed at end of epoch.


<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_AccumulatorStorageCostKey">AccumulatorStorageCostKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_system_myso_system_ENotSystemAddress"></a>



<pre><code><b>const</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="myso_system_myso_system_EWrongInnerVersion"></a>



<pre><code><b>const</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_EWrongInnerVersion">EWrongInnerVersion</a>: u64 = 1;
</code></pre>



<a name="myso_system_myso_system_create"></a>

## Function `create`

Create a new MySoSystemState object and make it shared.
This function will be called only once in genesis.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_create">create</a>(id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, validators: vector&lt;<a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>&gt;, <a href="../myso_system/storage_fund.md#myso_system_storage_fund">storage_fund</a>: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, protocol_version: u64, epoch_start_timestamp_ms: u64, parameters: <a href="../myso_system/sui_system_state_inner.md#myso_system_myso_system_state_inner_SystemParameters">myso_system::myso_system_state_inner::SystemParameters</a>, <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy">stake_subsidy</a>: <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">myso_system::stake_subsidy::StakeSubsidy</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_create">create</a>(
    id: UID,
    validators: vector&lt;Validator&gt;,
    <a href="../myso_system/storage_fund.md#myso_system_storage_fund">storage_fund</a>: Balance&lt;MYSO&gt;,
    protocol_version: u64,
    epoch_start_timestamp_ms: u64,
    parameters: SystemParameters,
    <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy">stake_subsidy</a>: StakeSubsidy,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> system_state = <a href="../myso_system/sui_system_state_inner.md#myso_system_myso_system_state_inner_create">myso_system_state_inner::create</a>(
        validators,
        <a href="../myso_system/storage_fund.md#myso_system_storage_fund">storage_fund</a>,
        protocol_version,
        epoch_start_timestamp_ms,
        parameters,
        <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy">stake_subsidy</a>,
        ctx,
    );
    <b>let</b> version = <a href="../myso_system/sui_system_state_inner.md#myso_system_myso_system_state_inner_genesis_system_state_version">myso_system_state_inner::genesis_system_state_version</a>();
    <b>let</b> <b>mut</b> self = <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a> {
        id,
        version,
    };
    dynamic_field::add(&<b>mut</b> self.id, version, system_state);
    transfer::share_object(self);
}
</code></pre>



</details>

<a name="myso_system_myso_system_request_add_validator_candidate"></a>

## Function `request_add_validator_candidate`

Can be called by anyone who wishes to become a validator candidate and starts accruing delegated
stakes in their staking pool. Once they have at least <code>MIN_VALIDATOR_JOINING_STAKE</code> amount of stake they
can call <code><a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_validator">request_add_validator</a></code> to officially become an active validator at the next epoch.
Aborts if the caller is already a pending or active validator, or a validator candidate.
Note: <code>proof_of_possession</code> MUST be a valid signature using myso_address and protocol_pubkey_bytes.
To produce a valid PoP, run [fn test_proof_of_possession].


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_validator_candidate">request_add_validator_candidate</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, pubkey_bytes: vector&lt;u8&gt;, network_pubkey_bytes: vector&lt;u8&gt;, worker_pubkey_bytes: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, name: vector&lt;u8&gt;, description: vector&lt;u8&gt;, image_url: vector&lt;u8&gt;, project_url: vector&lt;u8&gt;, net_address: vector&lt;u8&gt;, p2p_address: vector&lt;u8&gt;, primary_address: vector&lt;u8&gt;, worker_address: vector&lt;u8&gt;, gas_price: u64, commission_rate: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_validator_candidate">request_add_validator_candidate</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    pubkey_bytes: vector&lt;u8&gt;,
    network_pubkey_bytes: vector&lt;u8&gt;,
    worker_pubkey_bytes: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    name: vector&lt;u8&gt;,
    description: vector&lt;u8&gt;,
    image_url: vector&lt;u8&gt;,
    project_url: vector&lt;u8&gt;,
    net_address: vector&lt;u8&gt;,
    p2p_address: vector&lt;u8&gt;,
    primary_address: vector&lt;u8&gt;,
    worker_address: vector&lt;u8&gt;,
    gas_price: u64,
    commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    wrapper
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>()
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_validator_candidate">request_add_validator_candidate</a>(
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

<a name="myso_system_myso_system_request_remove_validator_candidate"></a>

## Function `request_remove_validator_candidate`

Called by a validator candidate to remove themselves from the candidacy. After this call
their staking pool becomes deactivate.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    ctx: &<b>mut</b> TxContext,
) {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_request_remove_validator_candidate">request_remove_validator_candidate</a>(ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_request_add_validator"></a>

## Function `request_add_validator`

Called by a validator candidate to add themselves to the active validator set beginning next epoch.
Aborts if the validator is a duplicate with one of the pending or active validators, or if the amount of
stake the validator has doesn't meet the min threshold, or if the number of new validators for the next
epoch has already reached the maximum.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_validator">request_add_validator</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_validator">request_add_validator</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>, ctx: &<b>mut</b> TxContext) {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_validator">request_add_validator</a>(ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_request_remove_validator"></a>

## Function `request_remove_validator`

A validator can call this function to request a removal in the next epoch.
We use the sender of <code>ctx</code> to look up the validator
(i.e. sender must match the myso_address in the validator).
At the end of the epoch, the <code><a href="../myso_system/validator.md#myso_system_validator">validator</a></code> object will be returned to the myso_address
of the validator.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_remove_validator">request_remove_validator</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_remove_validator">request_remove_validator</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>, ctx: &<b>mut</b> TxContext) {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_request_remove_validator">request_remove_validator</a>(ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_request_set_gas_price"></a>

## Function `request_set_gas_price`

A validator can call this entry function to submit a new gas price quote, to be
used for the reference gas price calculation at the end of the epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_set_gas_price">request_set_gas_price</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, cap: &<a href="../myso_system/validator_cap.md#myso_system_validator_cap_UnverifiedValidatorOperationCap">myso_system::validator_cap::UnverifiedValidatorOperationCap</a>, new_gas_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_set_gas_price">request_set_gas_price</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    new_gas_price: u64,
) {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_request_set_gas_price">request_set_gas_price</a>(cap, new_gas_price)
}
</code></pre>



</details>

<a name="myso_system_myso_system_set_candidate_validator_gas_price"></a>

## Function `set_candidate_validator_gas_price`

This entry function is used to set new gas price for candidate validators


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, cap: &<a href="../myso_system/validator_cap.md#myso_system_validator_cap_UnverifiedValidatorOperationCap">myso_system::validator_cap::UnverifiedValidatorOperationCap</a>, new_gas_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    new_gas_price: u64,
) {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_set_candidate_validator_gas_price">set_candidate_validator_gas_price</a>(cap, new_gas_price)
}
</code></pre>



</details>

<a name="myso_system_myso_system_request_set_commission_rate"></a>

## Function `request_set_commission_rate`

A validator can call this entry function to set a new commission rate, updated at the end of
the epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_set_commission_rate">request_set_commission_rate</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, new_commission_rate: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_set_commission_rate">request_set_commission_rate</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    new_commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_request_set_commission_rate">request_set_commission_rate</a>(new_commission_rate, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_set_candidate_validator_commission_rate"></a>

## Function `set_candidate_validator_commission_rate`

This entry function is used to set new commission rate for candidate validators


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, new_commission_rate: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    new_commission_rate: u64,
    ctx: &<b>mut</b> TxContext,
) {
    wrapper
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>()
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_set_candidate_validator_commission_rate">set_candidate_validator_commission_rate</a>(new_commission_rate, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_request_add_stake"></a>

## Function `request_add_stake`

Add stake to a validator's staking pool.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_stake">request_add_stake</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, stake: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_stake">request_add_stake</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    stake: Coin&lt;MYSO&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> staked_myso = <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(wrapper, stake, validator_address, ctx);
    transfer::public_transfer(staked_myso, ctx.sender());
}
</code></pre>



</details>

<a name="myso_system_myso_system_request_add_stake_non_entry"></a>

## Function `request_add_stake_non_entry`

The non-entry version of <code><a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_stake">request_add_stake</a></code>, which returns the staked MYSO instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, stake: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_stake_non_entry">request_add_stake_non_entry</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    stake: Coin&lt;MYSO&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
): StakedMySo {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_stake">request_add_stake</a>(stake, validator_address, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_request_add_stake_mul_coin"></a>

## Function `request_add_stake_mul_coin`

Add stake to a validator's staking pool using multiple coins.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, stakes: vector&lt;<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;&gt;, stake_amount: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, validator_address: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    stakes: vector&lt;Coin&lt;MYSO&gt;&gt;,
    stake_amount: option::Option&lt;u64&gt;,
    validator_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> staked_myso = wrapper
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>()
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_request_add_stake_mul_coin">request_add_stake_mul_coin</a>(stakes, stake_amount, validator_address, ctx);
    transfer::public_transfer(staked_myso, ctx.sender());
}
</code></pre>



</details>

<a name="myso_system_myso_system_request_withdraw_stake"></a>

## Function `request_withdraw_stake`

Withdraw stake from a validator's staking pool.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_withdraw_stake">request_withdraw_stake</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_withdraw_stake">request_withdraw_stake</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    staked_myso: StakedMySo,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> withdrawn_stake = wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(staked_myso, ctx);
    transfer::public_transfer(withdrawn_stake.into_coin(ctx), ctx.sender());
}
</code></pre>



</details>

<a name="myso_system_myso_system_convert_to_fungible_staked_myso"></a>

## Function `convert_to_fungible_staked_myso`

Convert StakedMySo into a FungibleStakedMySo object.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_convert_to_fungible_staked_myso">convert_to_fungible_staked_myso</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_convert_to_fungible_staked_myso">convert_to_fungible_staked_myso</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    staked_myso: StakedMySo,
    ctx: &<b>mut</b> TxContext,
): FungibleStakedMySo {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_convert_to_fungible_staked_myso">convert_to_fungible_staked_myso</a>(staked_myso, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_redeem_fungible_staked_myso"></a>

## Function `redeem_fungible_staked_myso`

Convert FungibleStakedMySo into a StakedMySo object.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_redeem_fungible_staked_myso">redeem_fungible_staked_myso</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, fungible_staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_FungibleStakedMySo">myso_system::staking_pool::FungibleStakedMySo</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_redeem_fungible_staked_myso">redeem_fungible_staked_myso</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    fungible_staked_myso: FungibleStakedMySo,
    ctx: &TxContext,
): Balance&lt;MYSO&gt; {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_redeem_fungible_staked_myso">redeem_fungible_staked_myso</a>(fungible_staked_myso, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_request_withdraw_stake_non_entry"></a>

## Function `request_withdraw_stake_non_entry`

Non-entry version of <code><a href="../myso_system/sui_system.md#myso_system_myso_system_request_withdraw_stake">request_withdraw_stake</a></code> that returns the withdrawn MYSO instead of transferring it to the sender.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, staked_myso: <a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_request_withdraw_stake_non_entry">request_withdraw_stake_non_entry</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    staked_myso: StakedMySo,
    ctx: &<b>mut</b> TxContext,
): Balance&lt;MYSO&gt; {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_request_withdraw_stake">request_withdraw_stake</a>(staked_myso, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_report_validator"></a>

## Function `report_validator`

Report a validator as a bad or non-performant actor in the system.
Succeeds if all the following are satisfied:
1. both the reporter in <code>cap</code> and the input <code>reportee_addr</code> are active validators.
2. reporter and reportee not the same address.
3. the cap object is still valid.
This function is idempotent.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_report_validator">report_validator</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, cap: &<a href="../myso_system/validator_cap.md#myso_system_validator_cap_UnverifiedValidatorOperationCap">myso_system::validator_cap::UnverifiedValidatorOperationCap</a>, reportee_addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_report_validator">report_validator</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    reportee_addr: <b>address</b>,
) {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_report_validator">report_validator</a>(cap, reportee_addr)
}
</code></pre>



</details>

<a name="myso_system_myso_system_undo_report_validator"></a>

## Function `undo_report_validator`

Undo a <code><a href="../myso_system/sui_system.md#myso_system_myso_system_report_validator">report_validator</a></code> action. Aborts if
1. the reportee is not a currently active validator or
2. the sender has not previously reported the <code>reportee_addr</code>, or
3. the cap is not valid


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_undo_report_validator">undo_report_validator</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, cap: &<a href="../myso_system/validator_cap.md#myso_system_validator_cap_UnverifiedValidatorOperationCap">myso_system::validator_cap::UnverifiedValidatorOperationCap</a>, reportee_addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_undo_report_validator">undo_report_validator</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    cap: &UnverifiedValidatorOperationCap,
    reportee_addr: <b>address</b>,
) {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_undo_report_validator">undo_report_validator</a>(cap, reportee_addr)
}
</code></pre>



</details>

<a name="myso_system_myso_system_rotate_operation_cap"></a>

## Function `rotate_operation_cap`

Create a new <code>UnverifiedValidatorOperationCap</code>, transfer it to the
validator and registers it. The original object is thus revoked.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_rotate_operation_cap">rotate_operation_cap</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>, ctx: &<b>mut</b> TxContext) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_rotate_operation_cap">rotate_operation_cap</a>(ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_name"></a>

## Function `update_validator_name`

Update a validator's name.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_name">update_validator_name</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, name: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_name">update_validator_name</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    name: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_name">update_validator_name</a>(name, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_description"></a>

## Function `update_validator_description`

Update a validator's description


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_description">update_validator_description</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, description: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_description">update_validator_description</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    description: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_description">update_validator_description</a>(description, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_image_url"></a>

## Function `update_validator_image_url`

Update a validator's image url


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_image_url">update_validator_image_url</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, image_url: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_image_url">update_validator_image_url</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    image_url: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_image_url">update_validator_image_url</a>(image_url, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_project_url"></a>

## Function `update_validator_project_url`

Update a validator's project url


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_project_url">update_validator_project_url</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, project_url: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_project_url">update_validator_project_url</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    project_url: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_project_url">update_validator_project_url</a>(project_url, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_next_epoch_network_address"></a>

## Function `update_validator_next_epoch_network_address`

Update a validator's network address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, network_address: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    network_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_network_address">update_validator_next_epoch_network_address</a>(network_address, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_candidate_validator_network_address"></a>

## Function `update_candidate_validator_network_address`

Update candidate validator's network address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, network_address: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    network_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_network_address">update_candidate_validator_network_address</a>(network_address, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_next_epoch_p2p_address"></a>

## Function `update_validator_next_epoch_p2p_address`

Update a validator's p2p address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, p2p_address: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    p2p_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_p2p_address">update_validator_next_epoch_p2p_address</a>(p2p_address, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_candidate_validator_p2p_address"></a>

## Function `update_candidate_validator_p2p_address`

Update candidate validator's p2p address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, p2p_address: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    p2p_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_p2p_address">update_candidate_validator_p2p_address</a>(p2p_address, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_next_epoch_primary_address"></a>

## Function `update_validator_next_epoch_primary_address`

Update a validator's narwhal primary address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, primary_address: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    primary_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_primary_address">update_validator_next_epoch_primary_address</a>(primary_address, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_candidate_validator_primary_address"></a>

## Function `update_candidate_validator_primary_address`

Update candidate validator's narwhal primary address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, primary_address: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    primary_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_primary_address">update_candidate_validator_primary_address</a>(primary_address, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_next_epoch_worker_address"></a>

## Function `update_validator_next_epoch_worker_address`

Update a validator's narwhal worker address.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, worker_address: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    worker_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_worker_address">update_validator_next_epoch_worker_address</a>(worker_address, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_candidate_validator_worker_address"></a>

## Function `update_candidate_validator_worker_address`

Update candidate validator's narwhal worker address.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, worker_address: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    worker_address: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_worker_address">update_candidate_validator_worker_address</a>(worker_address, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_next_epoch_protocol_pubkey"></a>

## Function `update_validator_next_epoch_protocol_pubkey`

Update a validator's public key of protocol key and proof of possession.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, protocol_pubkey: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>()
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_protocol_pubkey">update_validator_next_epoch_protocol_pubkey</a>(protocol_pubkey, proof_of_possession, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_candidate_validator_protocol_pubkey"></a>

## Function `update_candidate_validator_protocol_pubkey`

Update candidate validator's public key of protocol key and proof of possession.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, protocol_pubkey: vector&lt;u8&gt;, proof_of_possession: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    protocol_pubkey: vector&lt;u8&gt;,
    proof_of_possession: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>()
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_protocol_pubkey">update_candidate_validator_protocol_pubkey</a>(protocol_pubkey, proof_of_possession, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_next_epoch_worker_pubkey"></a>

## Function `update_validator_next_epoch_worker_pubkey`

Update a validator's public key of worker key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, worker_pubkey: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    worker_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_worker_pubkey">update_validator_next_epoch_worker_pubkey</a>(worker_pubkey, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_candidate_validator_worker_pubkey"></a>

## Function `update_candidate_validator_worker_pubkey`

Update candidate validator's public key of worker key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, worker_pubkey: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    worker_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_worker_pubkey">update_candidate_validator_worker_pubkey</a>(worker_pubkey, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_validator_next_epoch_network_pubkey"></a>

## Function `update_validator_next_epoch_network_pubkey`

Update a validator's public key of network key.
The change will only take effects starting from the next epoch.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, network_pubkey: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    network_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_validator_next_epoch_network_pubkey">update_validator_next_epoch_network_pubkey</a>(network_pubkey, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_update_candidate_validator_network_pubkey"></a>

## Function `update_candidate_validator_network_pubkey`

Update candidate validator's public key of network key.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, network_pubkey: vector&lt;u8&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    network_pubkey: vector&lt;u8&gt;,
    ctx: &TxContext,
) {
    self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_update_candidate_validator_network_pubkey">update_candidate_validator_network_pubkey</a>(network_pubkey, ctx)
}
</code></pre>



</details>

<a name="myso_system_myso_system_validator_address_by_pool_id"></a>

## Function `validator_address_by_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_validator_address_by_pool_id">validator_address_by_pool_id</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, pool_id: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_validator_address_by_pool_id">validator_address_by_pool_id</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>, pool_id: &ID): <b>address</b> {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_validator_address_by_pool_id">validator_address_by_pool_id</a>(pool_id)
}
</code></pre>



</details>

<a name="myso_system_myso_system_pool_exchange_rates"></a>

## Function `pool_exchange_rates`

Getter of the pool token exchange rate of a staking pool. Works for both active and inactive pools.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_pool_exchange_rates">pool_exchange_rates</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, pool_id: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): &<a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u64, <a href="../myso_system/staking_pool.md#myso_system_staking_pool_PoolTokenExchangeRate">myso_system::staking_pool::PoolTokenExchangeRate</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_pool_exchange_rates">pool_exchange_rates</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    pool_id: &ID,
): &Table&lt;u64, PoolTokenExchangeRate&gt; {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_pool_exchange_rates">pool_exchange_rates</a>(pool_id)
}
</code></pre>



</details>

<a name="myso_system_myso_system_active_validator_addresses"></a>

## Function `active_validator_addresses`

Getter returning addresses of the currently active validators.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_active_validator_addresses">active_validator_addresses</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_active_validator_addresses">active_validator_addresses</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>): vector&lt;<b>address</b>&gt; {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_active_validator_addresses">active_validator_addresses</a>()
}
</code></pre>



</details>

<a name="myso_system_myso_system_active_validator_addresses_ref"></a>

## Function `active_validator_addresses_ref`

Getter returning addresses of the currently active validators by reference.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_active_validator_addresses_ref">active_validator_addresses_ref</a>(wrapper: &<a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_active_validator_addresses_ref">active_validator_addresses_ref</a>(wrapper: &<a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>): vector&lt;<b>address</b>&gt; {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_ref">load_system_state_ref</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_active_validator_addresses">active_validator_addresses</a>()
}
</code></pre>



</details>

<a name="myso_system_myso_system_active_validator_voting_powers"></a>

## Function `active_validator_voting_powers`

Getter returns the voting power of the active validators, values are voting power in the scale of 10000.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_active_validator_voting_powers">active_validator_voting_powers</a>(wrapper: &<a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>): <a href="../myso/vec_map.md#myso_vec_map_VecMap">myso::vec_map::VecMap</a>&lt;<b>address</b>, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_active_validator_voting_powers">active_validator_voting_powers</a>(wrapper: &<a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>): VecMap&lt;<b>address</b>, u64&gt; {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_ref">load_system_state_ref</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_active_validator_voting_powers">active_validator_voting_powers</a>()
}
</code></pre>



</details>

<a name="myso_system_myso_system_calculate_rewards"></a>

## Function `calculate_rewards`

Calculate the rewards for a given staked MYSO object.
Used in the package, and can be dev-inspected.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_calculate_rewards">calculate_rewards</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, staked_myso: &<a href="../myso_system/staking_pool.md#myso_system_staking_pool_StakedMySo">myso_system::staking_pool::StakedMySo</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_calculate_rewards">calculate_rewards</a>(
    self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    staked_myso: &StakedMySo,
    ctx: &TxContext,
): u64 {
    <b>let</b> system_state = self.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>();
    system_state
        .validators_mut()
        .validator_by_pool_id(&staked_myso.pool_id())
        .get_staking_pool_ref()
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_calculate_rewards">calculate_rewards</a>(staked_myso, ctx.epoch())
}
</code></pre>



</details>

<a name="myso_system_myso_system_advance_epoch"></a>

## Function `advance_epoch`

This function should be called at the end of an epoch, and advances the system to the next epoch.
It does the following things:
1. Add storage charge to the storage fund.
2. Burn the storage rebates from the storage fund. These are already refunded to transaction sender's
gas coins.
3. Distribute computation charge to validator stake.
4. Update all validators.


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_advance_epoch">advance_epoch</a>(storage_reward: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, computation_reward: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, new_epoch: u64, next_protocol_version: u64, storage_rebate: u64, non_refundable_storage_fee: u64, storage_fund_reinvest_rate: u64, reward_slashing_rate: u64, epoch_start_timestamp_ms: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_advance_epoch">advance_epoch</a>(
    storage_reward: Balance&lt;MYSO&gt;,
    computation_reward: Balance&lt;MYSO&gt;,
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    new_epoch: u64,
    next_protocol_version: u64,
    storage_rebate: u64,
    non_refundable_storage_fee: u64,
    storage_fund_reinvest_rate: u64, // share of storage fund's rewards that's reinvested
    // into storage fund, in basis point.
    reward_slashing_rate: u64, // how much rewards are slashed to punish a <a href="../myso_system/validator.md#myso_system_validator">validator</a>, in bps.
    epoch_start_timestamp_ms: u64, // Timestamp of the epoch start
    ctx: &<b>mut</b> TxContext,
): Balance&lt;MYSO&gt; {
    // Validator will make a special system call with sender set <b>as</b> 0x0.
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../myso_system/sui_system.md#myso_system_myso_system_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> accumulator_storage_fund_amount = <a href="../myso_system/sui_system.md#myso_system_myso_system_get_accumulator_storage_fund_amount">get_accumulator_storage_fund_amount</a>(wrapper);
    <b>let</b> storage_rebate = wrapper
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>()
        .<a href="../myso_system/sui_system.md#myso_system_myso_system_advance_epoch">advance_epoch</a>(
            new_epoch,
            next_protocol_version,
            storage_reward,
            computation_reward,
            storage_rebate,
            non_refundable_storage_fee,
            storage_fund_reinvest_rate,
            reward_slashing_rate,
            epoch_start_timestamp_ms,
            accumulator_storage_fund_amount,
            ctx,
        );
    storage_rebate
}
</code></pre>



</details>

<a name="myso_system_myso_system_load_system_state"></a>

## Function `load_system_state`



<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state">load_system_state</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>): &<a href="../myso_system/sui_system_state_inner.md#myso_system_myso_system_state_inner_MySoSystemStateInnerV2">myso_system::myso_system_state_inner::MySoSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state">load_system_state</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>): &MySoSystemStateInnerV2 {
    <a href="../myso_system/sui_system.md#myso_system_myso_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self)
}
</code></pre>



</details>

<a name="myso_system_myso_system_load_system_state_mut"></a>

## Function `load_system_state_mut`



<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>): &<b>mut</b> <a href="../myso_system/sui_system_state_inner.md#myso_system_myso_system_state_inner_MySoSystemStateInnerV2">myso_system::myso_system_state_inner::MySoSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>): &<b>mut</b> MySoSystemStateInnerV2 {
    <a href="../myso_system/sui_system.md#myso_system_myso_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self)
}
</code></pre>



</details>

<a name="myso_system_myso_system_load_system_state_ref"></a>

## Function `load_system_state_ref`



<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_ref">load_system_state_ref</a>(self: &<a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>): &<a href="../myso_system/sui_system_state_inner.md#myso_system_myso_system_state_inner_MySoSystemStateInnerV2">myso_system::myso_system_state_inner::MySoSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_ref">load_system_state_ref</a>(self: &<a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>): &MySoSystemStateInnerV2 {
    <b>let</b> inner: &MySoSystemStateInnerV2 = dynamic_field::borrow(
        &self.id,
        self.version,
    );
    <b>assert</b>!(inner.system_state_version() == self.version, <a href="../myso_system/sui_system.md#myso_system_myso_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="myso_system_myso_system_load_inner_maybe_upgrade"></a>

## Function `load_inner_maybe_upgrade`



<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>): &<b>mut</b> <a href="../myso_system/sui_system_state_inner.md#myso_system_myso_system_state_inner_MySoSystemStateInnerV2">myso_system::myso_system_state_inner::MySoSystemStateInnerV2</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_load_inner_maybe_upgrade">load_inner_maybe_upgrade</a>(self: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>): &<b>mut</b> MySoSystemStateInnerV2 {
    <b>if</b> (self.version == 1) {
        <b>let</b> v1: MySoSystemStateInner = dynamic_field::remove(&<b>mut</b> self.id, self.version);
        <b>let</b> v2 = v1.v1_to_v2();
        self.version = 2;
        dynamic_field::add(&<b>mut</b> self.id, self.version, v2);
    };
    <b>let</b> inner: &<b>mut</b> MySoSystemStateInnerV2 = dynamic_field::borrow_mut(
        &<b>mut</b> self.id,
        self.version,
    );
    <b>assert</b>!(inner.system_state_version() == self.version, <a href="../myso_system/sui_system.md#myso_system_myso_system_EWrongInnerVersion">EWrongInnerVersion</a>);
    inner
}
</code></pre>



</details>

<a name="myso_system_myso_system_validator_voting_powers"></a>

## Function `validator_voting_powers`

Returns the voting power of the active validators, values are voting power in the scale of 10000.


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_validator_voting_powers">validator_voting_powers</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>): <a href="../myso/vec_map.md#myso_vec_map_VecMap">myso::vec_map::VecMap</a>&lt;<b>address</b>, u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_validator_voting_powers">validator_voting_powers</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>): VecMap&lt;<b>address</b>, u64&gt; {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state">load_system_state</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_active_validator_voting_powers">active_validator_voting_powers</a>()
}
</code></pre>



</details>

<a name="myso_system_myso_system_store_execution_time_estimates"></a>

## Function `store_execution_time_estimates`

Saves the given execution time estimate blob to the MySoSystemState object, for system use
at the start of the next epoch.


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_store_execution_time_estimates">store_execution_time_estimates</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, estimates_bytes: vector&lt;u8&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_store_execution_time_estimates">store_execution_time_estimates</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>, estimates_bytes: vector&lt;u8&gt;) {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_store_execution_time_estimates">store_execution_time_estimates</a>(estimates_bytes)
}
</code></pre>



</details>

<a name="myso_system_myso_system_store_execution_time_estimates_v2"></a>

## Function `store_execution_time_estimates_v2`

Saves the given execution time estimate chunks to the MySoSystemState object, for system use
at the start of the next epoch.


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_store_execution_time_estimates_v2">store_execution_time_estimates_v2</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, estimate_chunks: vector&lt;vector&lt;u8&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_store_execution_time_estimates_v2">store_execution_time_estimates_v2</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    estimate_chunks: vector&lt;vector&lt;u8&gt;&gt;,
) {
    wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().<a href="../myso_system/sui_system.md#myso_system_myso_system_store_execution_time_estimates_v2">store_execution_time_estimates_v2</a>(estimate_chunks)
}
</code></pre>



</details>

<a name="myso_system_myso_system_get_accumulator_storage_fund_amount"></a>

## Function `get_accumulator_storage_fund_amount`

Returns the storage fund amount for accumulator objects stored in extra_fields.
Returns 0 if no value has been stored.


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_get_accumulator_storage_fund_amount">get_accumulator_storage_fund_amount</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_get_accumulator_storage_fund_amount">get_accumulator_storage_fund_amount</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>): u64 {
    <b>let</b> extra_fields = wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state">load_system_state</a>().extra_fields();
    <b>let</b> key = <a href="../myso_system/sui_system.md#myso_system_myso_system_AccumulatorStorageCostKey">AccumulatorStorageCostKey</a>();
    <b>if</b> (extra_fields.contains(key)) {
        *extra_fields.borrow(key)
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>

<a name="myso_system_myso_system_write_accumulator_storage_cost"></a>

## Function `write_accumulator_storage_cost`

Stores the computed storage cost for accumulator objects.
This is called by an end-of-epoch transaction to record the storage cost
that will be used by advance_epoch.


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_write_accumulator_storage_cost">write_accumulator_storage_cost</a>(wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">myso_system::myso_system::MySoSystemState</a>, storage_cost: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_write_accumulator_storage_cost">write_accumulator_storage_cost</a>(
    wrapper: &<b>mut</b> <a href="../myso_system/sui_system.md#myso_system_myso_system_MySoSystemState">MySoSystemState</a>,
    storage_cost: u64,
    ctx: &TxContext,
) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../myso_system/sui_system.md#myso_system_myso_system_ENotSystemAddress">ENotSystemAddress</a>);
    <b>let</b> extra_fields = wrapper.<a href="../myso_system/sui_system.md#myso_system_myso_system_load_system_state_mut">load_system_state_mut</a>().extra_fields_mut();
    <b>let</b> key = <a href="../myso_system/sui_system.md#myso_system_myso_system_AccumulatorStorageCostKey">AccumulatorStorageCostKey</a>();
    <b>if</b> (extra_fields.contains(key)) {
        <b>let</b> existing: &<b>mut</b> u64 = extra_fields.borrow_mut(key);
        *existing = storage_cost;
    } <b>else</b> {
        extra_fields.add(key, storage_cost);
    };
}
</code></pre>



</details>
