---
title: Module `bridge::myusd_peg`
---

Atomic USDC/USDT ↔ MyUSD peg. The relay never holds either coin.


-  [Struct `MyUsdPeg`](#bridge_myusd_peg_MyUsdPeg)
-  [Struct `ConversionAdminCap`](#bridge_myusd_peg_ConversionAdminCap)
-  [Struct `StableClaimConvertedToMyUsd`](#bridge_myusd_peg_StableClaimConvertedToMyUsd)
-  [Struct `StableRedeemedToRail`](#bridge_myusd_peg_StableRedeemedToRail)
-  [Constants](#@Constants_0)
-  [Function `create`](#bridge_myusd_peg_create)
-  [Function `set_paused`](#bridge_myusd_peg_set_paused)
-  [Function `set_rail_enabled`](#bridge_myusd_peg_set_rail_enabled)
-  [Function `enable_conversion_policy`](#bridge_myusd_peg_enable_conversion_policy)
-  [Function `usdc_reserve`](#bridge_myusd_peg_usdc_reserve)
-  [Function `usdt_reserve`](#bridge_myusd_peg_usdt_reserve)
-  [Function `usdc_issued`](#bridge_myusd_peg_usdc_issued)
-  [Function `usdt_issued`](#bridge_myusd_peg_usdt_issued)
-  [Function `usdc_redeemed`](#bridge_myusd_peg_usdc_redeemed)
-  [Function `usdt_redeemed`](#bridge_myusd_peg_usdt_redeemed)
-  [Function `usdc_enabled`](#bridge_myusd_peg_usdc_enabled)
-  [Function `usdt_enabled`](#bridge_myusd_peg_usdt_enabled)
-  [Function `is_paused`](#bridge_myusd_peg_is_paused)
-  [Function `converted_recipient`](#bridge_myusd_peg_converted_recipient)
-  [Function `converted_input_amount`](#bridge_myusd_peg_converted_input_amount)
-  [Function `converted_myusd_amount`](#bridge_myusd_peg_converted_myusd_amount)
-  [Function `myusd_total_supply`](#bridge_myusd_peg_myusd_total_supply)
-  [Function `claim_stable_into_myusd`](#bridge_myusd_peg_claim_stable_into_myusd)
-  [Function `redeem_myusd_and_send_to_evm`](#bridge_myusd_peg_redeem_myusd_and_send_to_evm)
-  [Function `claim_usdc_into_myusd`](#bridge_myusd_peg_claim_usdc_into_myusd)
-  [Function `claim_usdt_into_myusd`](#bridge_myusd_peg_claim_usdt_into_myusd)
-  [Function `finish_usdc_deposit`](#bridge_myusd_peg_finish_usdc_deposit)
-  [Function `finish_usdt_deposit`](#bridge_myusd_peg_finish_usdt_deposit)
-  [Function `pay_myusd`](#bridge_myusd_peg_pay_myusd)
-  [Function `redeem_myusd_to_rail_usdc`](#bridge_myusd_peg_redeem_myusd_to_rail_usdc)
-  [Function `redeem_myusd_to_rail_usdt`](#bridge_myusd_peg_redeem_myusd_to_rail_usdt)
-  [Function `prepare_redeem`](#bridge_myusd_peg_prepare_redeem)
-  [Function `emit_redeem`](#bridge_myusd_peg_emit_redeem)
-  [Function `assert_can_convert`](#bridge_myusd_peg_assert_can_convert)
-  [Function `rail_enabled`](#bridge_myusd_peg_rail_enabled)
-  [Function `is_usdc`](#bridge_myusd_peg_is_usdc)
-  [Function `is_usdt`](#bridge_myusd_peg_is_usdt)
-  [Function `assert_admin`](#bridge_myusd_peg_assert_admin)


<pre><code><b>use</b> <a href="../bridge/bridge.md#bridge_bridge">bridge::bridge</a>;
<b>use</b> <a href="../bridge/chain_ids.md#bridge_chain_ids">bridge::chain_ids</a>;
<b>use</b> <a href="../bridge/committee.md#bridge_committee">bridge::committee</a>;
<b>use</b> <a href="../bridge/crypto.md#bridge_crypto">bridge::crypto</a>;
<b>use</b> <a href="../bridge/limiter.md#bridge_limiter">bridge::limiter</a>;
<b>use</b> <a href="../bridge/message.md#bridge_message">bridge::message</a>;
<b>use</b> <a href="../bridge/message_types.md#bridge_message_types">bridge::message_types</a>;
<b>use</b> <a href="../bridge/myusd.md#bridge_myusd">bridge::myusd</a>;
<b>use</b> <a href="../bridge/treasury.md#bridge_treasury">bridge::treasury</a>;
<b>use</b> <a href="../bridge/usdc.md#bridge_usdc">bridge::usdc</a>;
<b>use</b> <a href="../bridge/usdt.md#bridge_usdt">bridge::usdt</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/ecdsa_k1.md#myso_ecdsa_k1">myso::ecdsa_k1</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/linked_table.md#myso_linked_table">myso::linked_table</a>;
<b>use</b> <a href="../myso/myso.md#myso_myso">myso::myso</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/object_bag.md#myso_object_bag">myso::object_bag</a>;
<b>use</b> <a href="../myso/package.md#myso_package">myso::package</a>;
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
<b>use</b> <a href="../myso_system/myso_system.md#myso_system_myso_system">myso_system::myso_system</a>;
<b>use</b> <a href="../myso_system/myso_system_state_inner.md#myso_system_myso_system_state_inner">myso_system::myso_system_state_inner</a>;
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



<a name="bridge_myusd_peg_MyUsdPeg"></a>

## Struct `MyUsdPeg`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a> <b>has</b> key
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
<code>myusd_treasury: <a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;<a href="../bridge/myusd.md#bridge_myusd_MYUSD">bridge::myusd::MYUSD</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../bridge/usdc.md#bridge_usdc_USDC">bridge::usdc::USDC</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../bridge/usdt.md#bridge_usdt_USDT">bridge::usdt::USDT</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_enabled">usdc_enabled</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_enabled">usdt_enabled</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_issued">usdc_issued</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_issued">usdt_issued</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_redeemed">usdc_redeemed</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_redeemed">usdt_redeemed</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>paused: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>admin: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_myusd_peg_ConversionAdminCap"></a>

## Struct `ConversionAdminCap`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">ConversionAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_myusd_peg_StableClaimConvertedToMyUsd"></a>

## Struct `StableClaimConvertedToMyUsd`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_StableClaimConvertedToMyUsd">StableClaimConvertedToMyUsd</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>message_key_source_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>message_key_seq: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>source_chain: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>bridge_seq_num: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>rail_token_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>input_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>myusd_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>peg_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_issued">usdc_issued</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_issued">usdt_issued</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_myusd_peg_StableRedeemedToRail"></a>

## Struct `StableRedeemedToRail`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_StableRedeemedToRail">StableRedeemedToRail</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>rail_token_id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>myusd_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>rail_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sender: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>peg_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_myusd_peg_EUnsupportedRail"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EUnsupportedRail">EUnsupportedRail</a>: u64 = 0;
</code></pre>



<a name="bridge_myusd_peg_EZeroAmount"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EZeroAmount">EZeroAmount</a>: u64 = 1;
</code></pre>



<a name="bridge_myusd_peg_ERailDisabled"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_ERailDisabled">ERailDisabled</a>: u64 = 2;
</code></pre>



<a name="bridge_myusd_peg_EPegPaused"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EPegPaused">EPegPaused</a>: u64 = 3;
</code></pre>



<a name="bridge_myusd_peg_EInsufficientReserve"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EInsufficientReserve">EInsufficientReserve</a>: u64 = 4;
</code></pre>



<a name="bridge_myusd_peg_EDecimalMismatch"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EDecimalMismatch">EDecimalMismatch</a>: u64 = 5;
</code></pre>



<a name="bridge_myusd_peg_ENotAdmin"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_ENotAdmin">ENotAdmin</a>: u64 = 6;
</code></pre>



<a name="bridge_myusd_peg_EAlreadyInitialized"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EAlreadyInitialized">EAlreadyInitialized</a>: u64 = 7;
</code></pre>



<a name="bridge_myusd_peg_EInvalidPolicy"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EInvalidPolicy">EInvalidPolicy</a>: u64 = 8;
</code></pre>



<a name="bridge_myusd_peg_TOKEN_ID_USDC"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDC">TOKEN_ID_USDC</a>: u8 = 3;
</code></pre>



<a name="bridge_myusd_peg_TOKEN_ID_USDT"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDT">TOKEN_ID_USDT</a>: u8 = 4;
</code></pre>



<a name="bridge_myusd_peg_STABLE_DECIMALS"></a>



<pre><code><b>const</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_STABLE_DECIMALS">STABLE_DECIMALS</a>: u8 = 6;
</code></pre>



<a name="bridge_myusd_peg_create"></a>

## Function `create`

Create the shared peg. <code>treasury_cap</code> must be the sole MyUSD mint authority.


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_create">create</a>(treasury_cap: <a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;<a href="../bridge/myusd.md#bridge_myusd_MYUSD">bridge::myusd::MYUSD</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">bridge::myusd_peg::ConversionAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_create">create</a>(treasury_cap: TreasuryCap&lt;MYUSD&gt;, ctx: &<b>mut</b> TxContext): <a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">ConversionAdminCap</a> {
    <b>assert</b>!(coin::total_supply(&treasury_cap) == 0, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EAlreadyInitialized">EAlreadyInitialized</a>);
    <b>assert</b>!(<a href="../bridge/myusd.md#bridge_myusd_decimals">myusd::decimals</a>() == <a href="../bridge/myusd_peg.md#bridge_myusd_peg_STABLE_DECIMALS">STABLE_DECIMALS</a>, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EDecimalMismatch">EDecimalMismatch</a>);
    <b>assert</b>!(<a href="../bridge/usdc.md#bridge_usdc_decimals">usdc::decimals</a>() == <a href="../bridge/myusd_peg.md#bridge_myusd_peg_STABLE_DECIMALS">STABLE_DECIMALS</a>, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EDecimalMismatch">EDecimalMismatch</a>);
    <b>assert</b>!(<a href="../bridge/usdt.md#bridge_usdt_decimals">usdt::decimals</a>() == <a href="../bridge/myusd_peg.md#bridge_myusd_peg_STABLE_DECIMALS">STABLE_DECIMALS</a>, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EDecimalMismatch">EDecimalMismatch</a>);
    <b>let</b> admin = tx_context::sender(ctx);
    <b>let</b> peg = <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a> {
        id: object::new(ctx),
        myusd_treasury: treasury_cap,
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>: balance::zero(),
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>: balance::zero(),
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_enabled">usdc_enabled</a>: <b>false</b>,
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_enabled">usdt_enabled</a>: <b>false</b>,
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_issued">usdc_issued</a>: 0,
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_issued">usdt_issued</a>: 0,
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_redeemed">usdc_redeemed</a>: 0,
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_redeemed">usdt_redeemed</a>: 0,
        paused: <b>false</b>,
        admin,
    };
    transfer::share_object(peg);
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">ConversionAdminCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="bridge_myusd_peg_set_paused"></a>

## Function `set_paused`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_set_paused">set_paused</a>(peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, cap: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">bridge::myusd_peg::ConversionAdminCap</a>, paused: bool, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_set_paused">set_paused</a>(peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>, cap: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">ConversionAdminCap</a>, paused: bool, ctx: &TxContext) {
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_assert_admin">assert_admin</a>(peg, cap, ctx);
    peg.paused = paused;
}
</code></pre>



</details>

<a name="bridge_myusd_peg_set_rail_enabled"></a>

## Function `set_rail_enabled`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_set_rail_enabled">set_rail_enabled</a>(peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, cap: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">bridge::myusd_peg::ConversionAdminCap</a>, token_id: u8, enabled: bool, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_set_rail_enabled">set_rail_enabled</a>(
    peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    cap: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">ConversionAdminCap</a>,
    token_id: u8,
    enabled: bool,
    ctx: &TxContext,
) {
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_assert_admin">assert_admin</a>(peg, cap, ctx);
    <b>if</b> (token_id == <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDC">TOKEN_ID_USDC</a>) {
        peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_enabled">usdc_enabled</a> = enabled;
    } <b>else</b> {
        <b>assert</b>!(token_id == <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EUnsupportedRail">EUnsupportedRail</a>);
        peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_enabled">usdt_enabled</a> = enabled;
    };
}
</code></pre>



</details>

<a name="bridge_myusd_peg_enable_conversion_policy"></a>

## Function `enable_conversion_policy`

Enable conversion policy on the bridge at <code>(boundary_source_chain, boundary_seq)</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_enable_conversion_policy">enable_conversion_policy</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, cap: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">bridge::myusd_peg::ConversionAdminCap</a>, token_id: u8, boundary_source_chain: u8, boundary_seq: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_enable_conversion_policy">enable_conversion_policy</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> Bridge,
    peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    cap: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">ConversionAdminCap</a>,
    token_id: u8,
    boundary_source_chain: u8,
    boundary_seq: u64,
    ctx: &TxContext,
) {
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_assert_admin">assert_admin</a>(peg, cap, ctx);
    <b>assert</b>!(token_id == <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDC">TOKEN_ID_USDC</a> || token_id == <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EUnsupportedRail">EUnsupportedRail</a>);
    bridge::set_token_claim_policy(
        <a href="../bridge/bridge.md#bridge_bridge">bridge</a>,
        token_id,
        bridge::claim_policy_convert_to_myusd(),
        boundary_source_chain,
        boundary_seq,
        ctx,
    );
}
</code></pre>



</details>

<a name="bridge_myusd_peg_usdc_reserve"></a>

## Function `usdc_reserve`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>): u64 { balance::value(&peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>) }
</code></pre>



</details>

<a name="bridge_myusd_peg_usdt_reserve"></a>

## Function `usdt_reserve`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>): u64 { balance::value(&peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>) }
</code></pre>



</details>

<a name="bridge_myusd_peg_usdc_issued"></a>

## Function `usdc_issued`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_issued">usdc_issued</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_issued">usdc_issued</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>): u64 { peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_issued">usdc_issued</a> }
</code></pre>



</details>

<a name="bridge_myusd_peg_usdt_issued"></a>

## Function `usdt_issued`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_issued">usdt_issued</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_issued">usdt_issued</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>): u64 { peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_issued">usdt_issued</a> }
</code></pre>



</details>

<a name="bridge_myusd_peg_usdc_redeemed"></a>

## Function `usdc_redeemed`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_redeemed">usdc_redeemed</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_redeemed">usdc_redeemed</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>): u64 { peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_redeemed">usdc_redeemed</a> }
</code></pre>



</details>

<a name="bridge_myusd_peg_usdt_redeemed"></a>

## Function `usdt_redeemed`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_redeemed">usdt_redeemed</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_redeemed">usdt_redeemed</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>): u64 { peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_redeemed">usdt_redeemed</a> }
</code></pre>



</details>

<a name="bridge_myusd_peg_usdc_enabled"></a>

## Function `usdc_enabled`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_enabled">usdc_enabled</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_enabled">usdc_enabled</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>): bool { peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_enabled">usdc_enabled</a> }
</code></pre>



</details>

<a name="bridge_myusd_peg_usdt_enabled"></a>

## Function `usdt_enabled`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_enabled">usdt_enabled</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_enabled">usdt_enabled</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>): bool { peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_enabled">usdt_enabled</a> }
</code></pre>



</details>

<a name="bridge_myusd_peg_is_paused"></a>

## Function `is_paused`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_is_paused">is_paused</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_is_paused">is_paused</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>): bool { peg.paused }
</code></pre>



</details>

<a name="bridge_myusd_peg_converted_recipient"></a>

## Function `converted_recipient`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_converted_recipient">converted_recipient</a>(event: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_StableClaimConvertedToMyUsd">bridge::myusd_peg::StableClaimConvertedToMyUsd</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_converted_recipient">converted_recipient</a>(event: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_StableClaimConvertedToMyUsd">StableClaimConvertedToMyUsd</a>): <b>address</b> {
    event.recipient
}
</code></pre>



</details>

<a name="bridge_myusd_peg_converted_input_amount"></a>

## Function `converted_input_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_converted_input_amount">converted_input_amount</a>(event: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_StableClaimConvertedToMyUsd">bridge::myusd_peg::StableClaimConvertedToMyUsd</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_converted_input_amount">converted_input_amount</a>(event: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_StableClaimConvertedToMyUsd">StableClaimConvertedToMyUsd</a>): u64 {
    event.input_amount
}
</code></pre>



</details>

<a name="bridge_myusd_peg_converted_myusd_amount"></a>

## Function `converted_myusd_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_converted_myusd_amount">converted_myusd_amount</a>(event: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_StableClaimConvertedToMyUsd">bridge::myusd_peg::StableClaimConvertedToMyUsd</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_converted_myusd_amount">converted_myusd_amount</a>(event: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_StableClaimConvertedToMyUsd">StableClaimConvertedToMyUsd</a>): u64 {
    event.myusd_amount
}
</code></pre>



</details>

<a name="bridge_myusd_peg_myusd_total_supply"></a>

## Function `myusd_total_supply`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_myusd_total_supply">myusd_total_supply</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_myusd_total_supply">myusd_total_supply</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>): u64 {
    coin::total_supply(&peg.myusd_treasury)
}
</code></pre>



</details>

<a name="bridge_myusd_peg_claim_stable_into_myusd"></a>

## Function `claim_stable_into_myusd`

Generic dispatcher. <code>T</code> must be <code>USDC</code> or <code>USDT</code>; each path uses a typed reserve field.


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_claim_stable_into_myusd">claim_stable_into_myusd</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, source_chain: u8, bridge_seq_num: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_claim_stable_into_myusd">claim_stable_into_myusd</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> Bridge,
    peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>if</b> (<a href="../bridge/myusd_peg.md#bridge_myusd_peg_is_usdc">is_usdc</a>&lt;T&gt;()) {
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_claim_usdc_into_myusd">claim_usdc_into_myusd</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>, peg, clock, source_chain, bridge_seq_num, ctx);
    } <b>else</b> {
        <b>assert</b>!(<a href="../bridge/myusd_peg.md#bridge_myusd_peg_is_usdt">is_usdt</a>&lt;T&gt;(), <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EUnsupportedRail">EUnsupportedRail</a>);
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_claim_usdt_into_myusd">claim_usdt_into_myusd</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>, peg, clock, source_chain, bridge_seq_num, ctx);
    };
}
</code></pre>



</details>

<a name="bridge_myusd_peg_redeem_myusd_and_send_to_evm"></a>

## Function `redeem_myusd_and_send_to_evm`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_redeem_myusd_and_send_to_evm">redeem_myusd_and_send_to_evm</a>&lt;T&gt;(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, <a href="../bridge/myusd.md#bridge_myusd">myusd</a>: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../bridge/myusd.md#bridge_myusd_MYUSD">bridge::myusd::MYUSD</a>&gt;, target_chain: u8, target_address: vector&lt;u8&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_redeem_myusd_and_send_to_evm">redeem_myusd_and_send_to_evm</a>&lt;T&gt;(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> Bridge,
    peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    <a href="../bridge/myusd.md#bridge_myusd">myusd</a>: Coin&lt;MYUSD&gt;,
    target_chain: u8,
    target_address: vector&lt;u8&gt;,
    ctx: &<b>mut</b> TxContext,
) {
    <b>if</b> (<a href="../bridge/myusd_peg.md#bridge_myusd_peg_is_usdc">is_usdc</a>&lt;T&gt;()) {
        <b>let</b> coin_t = <a href="../bridge/myusd_peg.md#bridge_myusd_peg_redeem_myusd_to_rail_usdc">redeem_myusd_to_rail_usdc</a>(peg, <a href="../bridge/myusd.md#bridge_myusd">myusd</a>, ctx);
        bridge::send_token(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>, target_chain, target_address, coin_t, ctx);
    } <b>else</b> {
        <b>assert</b>!(<a href="../bridge/myusd_peg.md#bridge_myusd_peg_is_usdt">is_usdt</a>&lt;T&gt;(), <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EUnsupportedRail">EUnsupportedRail</a>);
        <b>let</b> coin_t = <a href="../bridge/myusd_peg.md#bridge_myusd_peg_redeem_myusd_to_rail_usdt">redeem_myusd_to_rail_usdt</a>(peg, <a href="../bridge/myusd.md#bridge_myusd">myusd</a>, ctx);
        bridge::send_token(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>, target_chain, target_address, coin_t, ctx);
    };
}
</code></pre>



</details>

<a name="bridge_myusd_peg_claim_usdc_into_myusd"></a>

## Function `claim_usdc_into_myusd`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_claim_usdc_into_myusd">claim_usdc_into_myusd</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, source_chain: u8, bridge_seq_num: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_claim_usdc_into_myusd">claim_usdc_into_myusd</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> Bridge,
    peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_assert_can_convert">assert_can_convert</a>(peg, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, <a href="../bridge/bridge.md#bridge_bridge">bridge</a>, source_chain, bridge_seq_num);
    <b>let</b> (maybe_token, recipient) = bridge::claim_token_internal&lt;USDC&gt;(
        <a href="../bridge/bridge.md#bridge_bridge">bridge</a>,
        clock,
        source_chain,
        bridge_seq_num,
        ctx,
    );
    <b>if</b> (maybe_token.is_none()) {
        maybe_token.destroy_none();
        <b>return</b>
    };
    <b>let</b> coin_t = maybe_token.destroy_some();
    <b>let</b> amount = <a href="../bridge/myusd_peg.md#bridge_myusd_peg_finish_usdc_deposit">finish_usdc_deposit</a>(peg, coin_t, recipient, source_chain, bridge_seq_num, ctx);
    <b>let</b> _ = amount;
}
</code></pre>



</details>

<a name="bridge_myusd_peg_claim_usdt_into_myusd"></a>

## Function `claim_usdt_into_myusd`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_claim_usdt_into_myusd">claim_usdt_into_myusd</a>(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> <a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, source_chain: u8, bridge_seq_num: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_claim_usdt_into_myusd">claim_usdt_into_myusd</a>(
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<b>mut</b> Bridge,
    peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    clock: &Clock,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_assert_can_convert">assert_can_convert</a>(peg, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, <a href="../bridge/bridge.md#bridge_bridge">bridge</a>, source_chain, bridge_seq_num);
    <b>let</b> (maybe_token, recipient) = bridge::claim_token_internal&lt;USDT&gt;(
        <a href="../bridge/bridge.md#bridge_bridge">bridge</a>,
        clock,
        source_chain,
        bridge_seq_num,
        ctx,
    );
    <b>if</b> (maybe_token.is_none()) {
        maybe_token.destroy_none();
        <b>return</b>
    };
    <b>let</b> coin_t = maybe_token.destroy_some();
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_finish_usdt_deposit">finish_usdt_deposit</a>(peg, coin_t, recipient, source_chain, bridge_seq_num, ctx);
}
</code></pre>



</details>

<a name="bridge_myusd_peg_finish_usdc_deposit"></a>

## Function `finish_usdc_deposit`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_finish_usdc_deposit">finish_usdc_deposit</a>(peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, coin_t: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../bridge/usdc.md#bridge_usdc_USDC">bridge::usdc::USDC</a>&gt;, recipient: <b>address</b>, source_chain: u8, bridge_seq_num: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_finish_usdc_deposit">finish_usdc_deposit</a>(
    peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    coin_t: Coin&lt;USDC&gt;,
    recipient: <b>address</b>,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &<b>mut</b> TxContext,
): u64 {
    <b>let</b> amount = coin::value(&coin_t);
    <b>assert</b>!(amount &gt; 0, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EZeroAmount">EZeroAmount</a>);
    balance::join(&<b>mut</b> peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>, coin::into_balance(coin_t));
    peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_issued">usdc_issued</a> = peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_issued">usdc_issued</a> + amount;
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_pay_myusd">pay_myusd</a>(peg, amount, recipient, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, source_chain, bridge_seq_num, ctx);
    amount
}
</code></pre>



</details>

<a name="bridge_myusd_peg_finish_usdt_deposit"></a>

## Function `finish_usdt_deposit`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_finish_usdt_deposit">finish_usdt_deposit</a>(peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, coin_t: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../bridge/usdt.md#bridge_usdt_USDT">bridge::usdt::USDT</a>&gt;, recipient: <b>address</b>, source_chain: u8, bridge_seq_num: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_finish_usdt_deposit">finish_usdt_deposit</a>(
    peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    coin_t: Coin&lt;USDT&gt;,
    recipient: <b>address</b>,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> amount = coin::value(&coin_t);
    <b>assert</b>!(amount &gt; 0, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EZeroAmount">EZeroAmount</a>);
    balance::join(&<b>mut</b> peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>, coin::into_balance(coin_t));
    peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_issued">usdt_issued</a> = peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_issued">usdt_issued</a> + amount;
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_pay_myusd">pay_myusd</a>(peg, amount, recipient, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, source_chain, bridge_seq_num, ctx);
}
</code></pre>



</details>

<a name="bridge_myusd_peg_pay_myusd"></a>

## Function `pay_myusd`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_pay_myusd">pay_myusd</a>(peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, amount: u64, recipient: <b>address</b>, rail_token_id: u8, source_chain: u8, bridge_seq_num: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_pay_myusd">pay_myusd</a>(
    peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    amount: u64,
    recipient: <b>address</b>,
    rail_token_id: u8,
    source_chain: u8,
    bridge_seq_num: u64,
    ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> <a href="../bridge/myusd.md#bridge_myusd">myusd</a> = coin::mint(&<b>mut</b> peg.myusd_treasury, amount, ctx);
    transfer::public_transfer(<a href="../bridge/myusd.md#bridge_myusd">myusd</a>, recipient);
    event::emit(<a href="../bridge/myusd_peg.md#bridge_myusd_peg_StableClaimConvertedToMyUsd">StableClaimConvertedToMyUsd</a> {
        message_key_source_chain: source_chain,
        message_key_seq: bridge_seq_num,
        source_chain,
        bridge_seq_num,
        rail_token_id,
        input_amount: amount,
        myusd_amount: amount,
        recipient,
        peg_id: object::id(peg),
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>: balance::value(&peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>),
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>: balance::value(&peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>),
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_issued">usdc_issued</a>: peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_issued">usdc_issued</a>,
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_issued">usdt_issued</a>: peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_issued">usdt_issued</a>,
    });
}
</code></pre>



</details>

<a name="bridge_myusd_peg_redeem_myusd_to_rail_usdc"></a>

## Function `redeem_myusd_to_rail_usdc`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_redeem_myusd_to_rail_usdc">redeem_myusd_to_rail_usdc</a>(peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, <a href="../bridge/myusd.md#bridge_myusd">myusd</a>: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../bridge/myusd.md#bridge_myusd_MYUSD">bridge::myusd::MYUSD</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../bridge/usdc.md#bridge_usdc_USDC">bridge::usdc::USDC</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_redeem_myusd_to_rail_usdc">redeem_myusd_to_rail_usdc</a>(
    peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    <a href="../bridge/myusd.md#bridge_myusd">myusd</a>: Coin&lt;MYUSD&gt;,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;USDC&gt; {
    <b>let</b> amount = <a href="../bridge/myusd_peg.md#bridge_myusd_peg_prepare_redeem">prepare_redeem</a>(peg, &<a href="../bridge/myusd.md#bridge_myusd">myusd</a>, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDC">TOKEN_ID_USDC</a>);
    <b>assert</b>!(balance::value(&peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>) &gt;= amount, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EInsufficientReserve">EInsufficientReserve</a>);
    coin::burn(&<b>mut</b> peg.myusd_treasury, <a href="../bridge/myusd.md#bridge_myusd">myusd</a>);
    peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_redeemed">usdc_redeemed</a> = peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_redeemed">usdc_redeemed</a> + amount;
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_emit_redeem">emit_redeem</a>(peg, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDC">TOKEN_ID_USDC</a>, amount, ctx);
    coin::from_balance(balance::split(&<b>mut</b> peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>, amount), ctx)
}
</code></pre>



</details>

<a name="bridge_myusd_peg_redeem_myusd_to_rail_usdt"></a>

## Function `redeem_myusd_to_rail_usdt`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_redeem_myusd_to_rail_usdt">redeem_myusd_to_rail_usdt</a>(peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, <a href="../bridge/myusd.md#bridge_myusd">myusd</a>: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../bridge/myusd.md#bridge_myusd_MYUSD">bridge::myusd::MYUSD</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../bridge/usdt.md#bridge_usdt_USDT">bridge::usdt::USDT</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_redeem_myusd_to_rail_usdt">redeem_myusd_to_rail_usdt</a>(
    peg: &<b>mut</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    <a href="../bridge/myusd.md#bridge_myusd">myusd</a>: Coin&lt;MYUSD&gt;,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;USDT&gt; {
    <b>let</b> amount = <a href="../bridge/myusd_peg.md#bridge_myusd_peg_prepare_redeem">prepare_redeem</a>(peg, &<a href="../bridge/myusd.md#bridge_myusd">myusd</a>, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDT">TOKEN_ID_USDT</a>);
    <b>assert</b>!(balance::value(&peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>) &gt;= amount, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EInsufficientReserve">EInsufficientReserve</a>);
    coin::burn(&<b>mut</b> peg.myusd_treasury, <a href="../bridge/myusd.md#bridge_myusd">myusd</a>);
    peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_redeemed">usdt_redeemed</a> = peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_redeemed">usdt_redeemed</a> + amount;
    <a href="../bridge/myusd_peg.md#bridge_myusd_peg_emit_redeem">emit_redeem</a>(peg, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDT">TOKEN_ID_USDT</a>, amount, ctx);
    coin::from_balance(balance::split(&<b>mut</b> peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>, amount), ctx)
}
</code></pre>



</details>

<a name="bridge_myusd_peg_prepare_redeem"></a>

## Function `prepare_redeem`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_prepare_redeem">prepare_redeem</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, <a href="../bridge/myusd.md#bridge_myusd">myusd</a>: &<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../bridge/myusd.md#bridge_myusd_MYUSD">bridge::myusd::MYUSD</a>&gt;, token_id: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_prepare_redeem">prepare_redeem</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>, <a href="../bridge/myusd.md#bridge_myusd">myusd</a>: &Coin&lt;MYUSD&gt;, token_id: u8): u64 {
    <b>assert</b>!(!peg.paused, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EPegPaused">EPegPaused</a>);
    <b>assert</b>!(<a href="../bridge/myusd_peg.md#bridge_myusd_peg_rail_enabled">rail_enabled</a>(peg, token_id), <a href="../bridge/myusd_peg.md#bridge_myusd_peg_ERailDisabled">ERailDisabled</a>);
    <b>let</b> amount = coin::value(<a href="../bridge/myusd.md#bridge_myusd">myusd</a>);
    <b>assert</b>!(amount &gt; 0, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EZeroAmount">EZeroAmount</a>);
    amount
}
</code></pre>



</details>

<a name="bridge_myusd_peg_emit_redeem"></a>

## Function `emit_redeem`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_emit_redeem">emit_redeem</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, token_id: u8, amount: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_emit_redeem">emit_redeem</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>, token_id: u8, amount: u64, ctx: &TxContext) {
    event::emit(<a href="../bridge/myusd_peg.md#bridge_myusd_peg_StableRedeemedToRail">StableRedeemedToRail</a> {
        rail_token_id: token_id,
        myusd_amount: amount,
        rail_amount: amount,
        sender: tx_context::sender(ctx),
        peg_id: object::id(peg),
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>: balance::value(&peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_reserve">usdc_reserve</a>),
        <a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>: balance::value(&peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_reserve">usdt_reserve</a>),
    });
}
</code></pre>



</details>

<a name="bridge_myusd_peg_assert_can_convert"></a>

## Function `assert_can_convert`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_assert_can_convert">assert_can_convert</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, token_id: u8, <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &<a href="../bridge/bridge.md#bridge_bridge_Bridge">bridge::bridge::Bridge</a>, source_chain: u8, bridge_seq_num: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_assert_can_convert">assert_can_convert</a>(
    peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>,
    token_id: u8,
    <a href="../bridge/bridge.md#bridge_bridge">bridge</a>: &Bridge,
    source_chain: u8,
    bridge_seq_num: u64,
) {
    <b>assert</b>!(!peg.paused, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EPegPaused">EPegPaused</a>);
    <b>assert</b>!(<a href="../bridge/myusd_peg.md#bridge_myusd_peg_rail_enabled">rail_enabled</a>(peg, token_id), <a href="../bridge/myusd_peg.md#bridge_myusd_peg_ERailDisabled">ERailDisabled</a>);
    <b>let</b> policy = bridge::effective_claim_policy(<a href="../bridge/bridge.md#bridge_bridge">bridge</a>, token_id, source_chain, bridge_seq_num);
    <b>assert</b>!(policy == bridge::claim_policy_convert_to_myusd(), <a href="../bridge/myusd_peg.md#bridge_myusd_peg_EInvalidPolicy">EInvalidPolicy</a>);
}
</code></pre>



</details>

<a name="bridge_myusd_peg_rail_enabled"></a>

## Function `rail_enabled`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_rail_enabled">rail_enabled</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, token_id: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_rail_enabled">rail_enabled</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>, token_id: u8): bool {
    <b>if</b> (token_id == <a href="../bridge/myusd_peg.md#bridge_myusd_peg_TOKEN_ID_USDC">TOKEN_ID_USDC</a>) {
        peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdc_enabled">usdc_enabled</a>
    } <b>else</b> {
        peg.<a href="../bridge/myusd_peg.md#bridge_myusd_peg_usdt_enabled">usdt_enabled</a>
    }
}
</code></pre>



</details>

<a name="bridge_myusd_peg_is_usdc"></a>

## Function `is_usdc`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_is_usdc">is_usdc</a>&lt;T&gt;(): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_is_usdc">is_usdc</a>&lt;T&gt;(): bool {
    type_name::with_defining_ids&lt;T&gt;() == type_name::with_defining_ids&lt;USDC&gt;()
}
</code></pre>



</details>

<a name="bridge_myusd_peg_is_usdt"></a>

## Function `is_usdt`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_is_usdt">is_usdt</a>&lt;T&gt;(): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_is_usdt">is_usdt</a>&lt;T&gt;(): bool {
    type_name::with_defining_ids&lt;T&gt;() == type_name::with_defining_ids&lt;USDT&gt;()
}
</code></pre>



</details>

<a name="bridge_myusd_peg_assert_admin"></a>

## Function `assert_admin`



<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_assert_admin">assert_admin</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">bridge::myusd_peg::MyUsdPeg</a>, _cap: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">bridge::myusd_peg::ConversionAdminCap</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/myusd_peg.md#bridge_myusd_peg_assert_admin">assert_admin</a>(peg: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_MyUsdPeg">MyUsdPeg</a>, _cap: &<a href="../bridge/myusd_peg.md#bridge_myusd_peg_ConversionAdminCap">ConversionAdminCap</a>, ctx: &TxContext) {
    <b>assert</b>!(tx_context::sender(ctx) == peg.admin, <a href="../bridge/myusd_peg.md#bridge_myusd_peg_ENotAdmin">ENotAdmin</a>);
}
</code></pre>



</details>
