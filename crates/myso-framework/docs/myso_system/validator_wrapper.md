---
title: Module `myso_system::validator_wrapper`
---



-  [Struct `ValidatorWrapper`](#myso_system_validator_wrapper_ValidatorWrapper)
-  [Constants](#@Constants_0)
-  [Function `create_v1`](#myso_system_validator_wrapper_create_v1)
-  [Function `load_validator_maybe_upgrade`](#myso_system_validator_wrapper_load_validator_maybe_upgrade)
-  [Function `destroy`](#myso_system_validator_wrapper_destroy)
-  [Function `upgrade_to_latest`](#myso_system_validator_wrapper_upgrade_to_latest)
-  [Function `version`](#myso_system_validator_wrapper_version)


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
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../myso/versioned.md#myso_versioned">myso::versioned</a>;
<b>use</b> <a href="../myso_system/staking_pool.md#myso_system_staking_pool">myso_system::staking_pool</a>;
<b>use</b> <a href="../myso_system/validator.md#myso_system_validator">myso_system::validator</a>;
<b>use</b> <a href="../myso_system/validator_cap.md#myso_system_validator_cap">myso_system::validator_cap</a>;
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



<a name="myso_system_validator_wrapper_ValidatorWrapper"></a>

## Struct `ValidatorWrapper`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>inner: <a href="../myso/versioned.md#myso_versioned_Versioned">myso::versioned::Versioned</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_system_validator_wrapper_EInvalidVersion"></a>



<pre><code><b>const</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_EInvalidVersion">EInvalidVersion</a>: u64 = 0;
</code></pre>



<a name="myso_system_validator_wrapper_create_v1"></a>

## Function `create_v1`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_create_v1">create_v1</a>(<a href="../myso_system/validator.md#myso_system_validator">validator</a>: <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">myso_system::validator_wrapper::ValidatorWrapper</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_create_v1">create_v1</a>(<a href="../myso_system/validator.md#myso_system_validator">validator</a>: Validator, ctx: &<b>mut</b> TxContext): <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> {
    <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> {
        inner: versioned::create(1, <a href="../myso_system/validator.md#myso_system_validator">validator</a>, ctx),
    }
}
</code></pre>



</details>

<a name="myso_system_validator_wrapper_load_validator_maybe_upgrade"></a>

## Function `load_validator_maybe_upgrade`

This function should always return the latest supported version.
If the inner version is old, we upgrade it lazily in-place.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">myso_system::validator_wrapper::ValidatorWrapper</a>): &<b>mut</b> <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_load_validator_maybe_upgrade">load_validator_maybe_upgrade</a>(self: &<b>mut</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): &<b>mut</b> Validator {
    self.<a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>();
    self.inner.load_value_mut()
}
</code></pre>



</details>

<a name="myso_system_validator_wrapper_destroy"></a>

## Function `destroy`

Destroy the wrapper and retrieve the inner validator object.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_destroy">destroy</a>(self: <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">myso_system::validator_wrapper::ValidatorWrapper</a>): <a href="../myso_system/validator.md#myso_system_validator_Validator">myso_system::validator::Validator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_destroy">destroy</a>(self: <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): Validator {
    <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(&self);
    <b>let</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a> { inner } = self;
    inner.<a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_destroy">destroy</a>()
}
</code></pre>



</details>

<a name="myso_system_validator_wrapper_upgrade_to_latest"></a>

## Function `upgrade_to_latest`



<pre><code><b>fun</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">myso_system::validator_wrapper::ValidatorWrapper</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_upgrade_to_latest">upgrade_to_latest</a>(self: &<a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>) {
    <b>let</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_version">version</a> = self.<a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_version">version</a>();
    // TODO: When new versions are added, we need to explicitly upgrade here.
    <b>assert</b>!(<a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_version">version</a> == 1, <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_EInvalidVersion">EInvalidVersion</a>);
}
</code></pre>



</details>

<a name="myso_system_validator_wrapper_version"></a>

## Function `version`



<pre><code><b>fun</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_version">version</a>(self: &<a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">myso_system::validator_wrapper::ValidatorWrapper</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_version">version</a>(self: &<a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_ValidatorWrapper">ValidatorWrapper</a>): u64 {
    self.inner.<a href="../myso_system/validator_wrapper.md#myso_system_validator_wrapper_version">version</a>()
}
</code></pre>



</details>
