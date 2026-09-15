---
title: Module `bridge::treasury`
---



-  [Struct `BridgeTreasury`](#bridge_treasury_BridgeTreasury)
-  [Struct `BridgeTokenMetadata`](#bridge_treasury_BridgeTokenMetadata)
-  [Struct `ForeignTokenRegistration`](#bridge_treasury_ForeignTokenRegistration)
-  [Struct `UpdateTokenPriceEvent`](#bridge_treasury_UpdateTokenPriceEvent)
-  [Struct `NewTokenEvent`](#bridge_treasury_NewTokenEvent)
-  [Struct `TokenRegistrationEvent`](#bridge_treasury_TokenRegistrationEvent)
-  [Constants](#@Constants_0)
-  [Function `token_id`](#bridge_treasury_token_id)
-  [Function `type_ids`](#bridge_treasury_type_ids)
-  [Function `type_matches_token_id`](#bridge_treasury_type_matches_token_id)
-  [Function `rail_reserve`](#bridge_treasury_rail_reserve)
-  [Function `decimal_multiplier`](#bridge_treasury_decimal_multiplier)
-  [Function `notional_value`](#bridge_treasury_notional_value)
-  [Function `decimal_multiplier_by_id`](#bridge_treasury_decimal_multiplier_by_id)
-  [Function `notional_value_by_id`](#bridge_treasury_notional_value_by_id)
-  [Function `register_foreign_token`](#bridge_treasury_register_foreign_token)
-  [Function `add_new_token`](#bridge_treasury_add_new_token)
-  [Function `create`](#bridge_treasury_create)
-  [Function `bootstrap_native_myso_once`](#bridge_treasury_bootstrap_native_myso_once)
-  [Function `lock_native_myso`](#bridge_treasury_lock_native_myso)
-  [Function `unlock_native_myso`](#bridge_treasury_unlock_native_myso)
-  [Function `native_myso_locked_amount`](#bridge_treasury_native_myso_locked_amount)
-  [Function `native_bridge_ready`](#bridge_treasury_native_bridge_ready)
-  [Function `burn`](#bridge_treasury_burn)
-  [Function `mint`](#bridge_treasury_mint)
-  [Function `credit_rail`](#bridge_treasury_credit_rail)
-  [Function `debit_rail`](#bridge_treasury_debit_rail)
-  [Function `update_asset_notional_price`](#bridge_treasury_update_asset_notional_price)
-  [Function `get_token_metadata`](#bridge_treasury_get_token_metadata)
-  [Function `find_supported_type`](#bridge_treasury_find_supported_type)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/myso.md#myso_myso">myso::myso</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/object_bag.md#myso_object_bag">myso::object_bag</a>;
<b>use</b> <a href="../myso/package.md#myso_package">myso::package</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
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



<a name="bridge_treasury_BridgeTreasury"></a>

## Struct `BridgeTreasury`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>treasuries: <a href="../myso/object_bag.md#myso_object_bag_ObjectBag">myso::object_bag::ObjectBag</a></code>
</dt>
<dd>
</dd>
<dt>
<code>supported_tokens: <a href="../myso/vec_map.md#myso_vec_map_VecMap">myso::vec_map::VecMap</a>&lt;<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>, <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">bridge::treasury::BridgeTokenMetadata</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>id_token_type_map: <a href="../myso/vec_map.md#myso_vec_map_VecMap">myso::vec_map::VecMap</a>&lt;u8, <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>waiting_room: <a href="../myso/bag.md#myso_bag_Bag">myso::bag::Bag</a></code>
</dt>
<dd>
</dd>
<dt>
<code>native_myso_escrow: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
 Escrow for native MYSO (bootstrap + send_myso_token locks); claims release from here.
</dd>
<dt>
<code>native_bridge_initialized: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>type_to_ids: <a href="../myso/vec_map.md#myso_vec_map_VecMap">myso::vec_map::VecMap</a>&lt;<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>, vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
 Type -> rail ids. Length 1 for 1:1 wrappers; length > 1 for shared rails (myUSD).
</dd>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>: <a href="../myso/vec_map.md#myso_vec_map_VecMap">myso::vec_map::VecMap</a>&lt;u8, u64&gt;</code>
</dt>
<dd>
 Outstanding minted amount per rail id (EVM vault backing).
</dd>
</dl>


</details>

<a name="bridge_treasury_BridgeTokenMetadata"></a>

## Struct `BridgeTokenMetadata`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">BridgeTokenMetadata</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>native_token: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_ForeignTokenRegistration"></a>

## Struct `ForeignTokenRegistration`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>type_name: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>uc: <a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a></code>
</dt>
<dd>
</dd>
<dt>
<code>decimal: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_UpdateTokenPriceEvent"></a>

## Struct `UpdateTokenPriceEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_UpdateTokenPriceEvent">UpdateTokenPriceEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>new_price: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_NewTokenEvent"></a>

## Struct `NewTokenEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_NewTokenEvent">NewTokenEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>type_name: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>native_token: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="bridge_treasury_TokenRegistrationEvent"></a>

## Struct `TokenRegistrationEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/treasury.md#bridge_treasury_TokenRegistrationEvent">TokenRegistrationEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>type_name: <a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a></code>
</dt>
<dd>
</dd>
<dt>
<code>decimal: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>native_token: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_treasury_EUnsupportedTokenType"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>: u64 = 1;
</code></pre>



<a name="bridge_treasury_EInvalidUpgradeCap"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_EInvalidUpgradeCap">EInvalidUpgradeCap</a>: u64 = 2;
</code></pre>



<a name="bridge_treasury_ETokenSupplyNonZero"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_ETokenSupplyNonZero">ETokenSupplyNonZero</a>: u64 = 3;
</code></pre>



<a name="bridge_treasury_EInvalidNotionalValue"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_EInvalidNotionalValue">EInvalidNotionalValue</a>: u64 = 4;
</code></pre>



<a name="bridge_treasury_ENativeBridgeAlreadyInitialized"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_ENativeBridgeAlreadyInitialized">ENativeBridgeAlreadyInitialized</a>: u64 = 5;
</code></pre>



<a name="bridge_treasury_EInvalidBootstrapAmount"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_EInvalidBootstrapAmount">EInvalidBootstrapAmount</a>: u64 = 6;
</code></pre>



<a name="bridge_treasury_ENativeBridgeNotInitialized"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_ENativeBridgeNotInitialized">ENativeBridgeNotInitialized</a>: u64 = 7;
</code></pre>



<a name="bridge_treasury_EInsufficientNativeEscrow"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_EInsufficientNativeEscrow">EInsufficientNativeEscrow</a>: u64 = 8;
</code></pre>



<a name="bridge_treasury_EInsufficientRailReserve"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_EInsufficientRailReserve">EInsufficientRailReserve</a>: u64 = 9;
</code></pre>



<a name="bridge_treasury_ETokenRequiresRailId"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_ETokenRequiresRailId">ETokenRequiresRailId</a>: u64 = 10;
</code></pre>



<a name="bridge_treasury_ETokenIdAlreadyExists"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_ETokenIdAlreadyExists">ETokenIdAlreadyExists</a>: u64 = 11;
</code></pre>



<a name="bridge_treasury_MIST_PER_WHOLE_MYSO"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_MIST_PER_WHOLE_MYSO">MIST_PER_WHOLE_MYSO</a>: u64 = 1000000000;
</code></pre>



<a name="bridge_treasury_BOOTSTRAP_NATIVE_MYSO_MIST"></a>



<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_BOOTSTRAP_NATIVE_MYSO_MIST">BOOTSTRAP_NATIVE_MYSO_MIST</a>: u64 = 50000000000000000;
</code></pre>



<a name="bridge_treasury_NATIVE_MYSO_INITIAL_NOTIONAL_USD"></a>

Initial USD notional for limiter ($1.00 at 8 decimal places), same scale as bridge limiter.


<pre><code><b>const</b> <a href="../bridge/treasury.md#bridge_treasury_NATIVE_MYSO_INITIAL_NOTIONAL_USD">NATIVE_MYSO_INITIAL_NOTIONAL_USD</a>: u64 = 100000000;
</code></pre>



<a name="bridge_treasury_token_id"></a>

## Function `token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): u8 {
    <b>let</b> ids = self.<a href="../bridge/treasury.md#bridge_treasury_type_ids">type_ids</a>&lt;T&gt;();
    <b>assert</b>!(ids.length() == 1, <a href="../bridge/treasury.md#bridge_treasury_ETokenRequiresRailId">ETokenRequiresRailId</a>);
    ids[0]
}
</code></pre>



</details>

<a name="bridge_treasury_type_ids"></a>

## Function `type_ids`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_type_ids">type_ids</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_type_ids">type_ids</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): vector&lt;u8&gt; {
    <b>let</b> coin_type = type_name::with_defining_ids&lt;T&gt;();
    <b>assert</b>!(self.type_to_ids.contains(&coin_type), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    *self.type_to_ids.get(&coin_type)
}
</code></pre>



</details>

<a name="bridge_treasury_type_matches_token_id"></a>

## Function `type_matches_token_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_type_matches_token_id">type_matches_token_id</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_type_matches_token_id">type_matches_token_id</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8): bool {
    <b>if</b> (!self.id_token_type_map.contains(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>)) {
        <b>return</b> <b>false</b>
    };
    self.id_token_type_map[&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>] == type_name::with_defining_ids&lt;T&gt;()
}
</code></pre>



</details>

<a name="bridge_treasury_rail_reserve"></a>

## Function `rail_reserve`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8): u64 {
    <b>if</b> (!self.<a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>.contains(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>)) {
        <b>return</b> 0
    };
    self.<a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>[&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>]
}
</code></pre>



</details>

<a name="bridge_treasury_decimal_multiplier"></a>

## Function `decimal_multiplier`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): u64 {
    <b>let</b> metadata = self.<a href="../bridge/treasury.md#bridge_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;();
    metadata.<a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>
}
</code></pre>



</details>

<a name="bridge_treasury_notional_value"></a>

## Function `notional_value`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): u64 {
    <b>let</b> metadata = self.<a href="../bridge/treasury.md#bridge_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;();
    metadata.<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>
}
</code></pre>



</details>

<a name="bridge_treasury_decimal_multiplier_by_id"></a>

## Function `decimal_multiplier_by_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier_by_id">decimal_multiplier_by_id</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier_by_id">decimal_multiplier_by_id</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8): u64 {
    <b>let</b> type_name = self.id_token_type_map.try_get(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>);
    <b>assert</b>!(type_name.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>let</b> type_name = type_name.destroy_some();
    <b>let</b> metadata = self.supported_tokens.try_get(&type_name);
    <b>assert</b>!(metadata.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    metadata.destroy_some().<a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>
}
</code></pre>



</details>

<a name="bridge_treasury_notional_value_by_id"></a>

## Function `notional_value_by_id`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_notional_value_by_id">notional_value_by_id</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_notional_value_by_id">notional_value_by_id</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8): u64 {
    <b>let</b> type_name = self.id_token_type_map.try_get(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>);
    <b>assert</b>!(type_name.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>let</b> type_name = type_name.destroy_some();
    <b>let</b> metadata = self.supported_tokens.try_get(&type_name);
    <b>assert</b>!(metadata.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    metadata.destroy_some().<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>
}
</code></pre>



</details>

<a name="bridge_treasury_register_foreign_token"></a>

## Function `register_foreign_token`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_register_foreign_token">register_foreign_token</a>&lt;T&gt;(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, tc: <a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;T&gt;, uc: <a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>, metadata: &<a href="../myso/coin.md#myso_coin_CoinMetadata">myso::coin::CoinMetadata</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_register_foreign_token">register_foreign_token</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    tc: TreasuryCap&lt;T&gt;,
    uc: UpgradeCap,
    metadata: &CoinMetadata&lt;T&gt;,
) {
    // Make sure TreasuryCap <b>has</b> not been minted before.
    <b>assert</b>!(coin::total_supply(&tc) == 0, <a href="../bridge/treasury.md#bridge_treasury_ETokenSupplyNonZero">ETokenSupplyNonZero</a>);
    <b>let</b> type_name = type_name::with_defining_ids&lt;T&gt;();
    <b>let</b> address_bytes = hex::decode(ascii::into_bytes(type_name::address_string(&type_name)));
    <b>let</b> coin_address = address::from_bytes(address_bytes);
    // Make sure upgrade cap is <b>for</b> the Coin package
    // FIXME: add test
    <b>assert</b>!(
        object::id_to_address(&package::upgrade_package(&uc)) == coin_address,
        <a href="../bridge/treasury.md#bridge_treasury_EInvalidUpgradeCap">EInvalidUpgradeCap</a>,
    );
    <b>let</b> registration = <a href="../bridge/treasury.md#bridge_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a> {
        type_name,
        uc,
        decimal: coin::get_decimals(metadata),
    };
    self.waiting_room.add(type_name::into_string(type_name), registration);
    self.treasuries.add(type_name, tc);
    event::emit(<a href="../bridge/treasury.md#bridge_treasury_TokenRegistrationEvent">TokenRegistrationEvent</a> {
        type_name,
        decimal: coin::get_decimals(metadata),
        native_token: <b>false</b>,
    });
}
</code></pre>



</details>

<a name="bridge_treasury_add_new_token"></a>

## Function `add_new_token`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_add_new_token">add_new_token</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, token_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8, native_token: bool, <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_add_new_token">add_new_token</a>(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    token_name: String,
    <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8,
    native_token: bool,
    <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>: u64,
) {
    <b>if</b> (native_token) {
        <b>return</b>
    };
    <b>assert</b>!(<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a> &gt; 0, <a href="../bridge/treasury.md#bridge_treasury_EInvalidNotionalValue">EInvalidNotionalValue</a>);
    <b>assert</b>!(!self.id_token_type_map.contains(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>), <a href="../bridge/treasury.md#bridge_treasury_ETokenIdAlreadyExists">ETokenIdAlreadyExists</a>);
    <b>if</b> (self.waiting_room.contains(token_name)) {
        <b>let</b> <a href="../bridge/treasury.md#bridge_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a> {
            type_name,
            uc,
            decimal,
        } = self.waiting_room.remove&lt;String, <a href="../bridge/treasury.md#bridge_treasury_ForeignTokenRegistration">ForeignTokenRegistration</a>&gt;(token_name);
        <b>let</b> <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a> = 10u64.pow(decimal);
        <b>if</b> (!self.supported_tokens.contains(&type_name)) {
            self
                .supported_tokens
                .insert(
                    type_name,
                    <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">BridgeTokenMetadata</a> {
                        id: <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>,
                        <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>,
                        <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>,
                        native_token,
                    },
                );
            self.type_to_ids.insert(type_name, vector[<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>]);
        } <b>else</b> {
            self.type_to_ids.get_mut(&type_name).push_back(<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>);
            <b>let</b> meta = self.supported_tokens.get_mut(&type_name);
            meta.<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a> = <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>;
        };
        self.id_token_type_map.insert(<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>, type_name);
        self.<a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>.insert(<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>, 0);
        transfer::public_freeze_object(uc);
        event::emit(<a href="../bridge/treasury.md#bridge_treasury_NewTokenEvent">NewTokenEvent</a> {
            <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>,
            type_name,
            native_token,
            <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>,
            <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>,
        });
        <b>return</b>
    };
    <b>let</b> type_name = self.<a href="../bridge/treasury.md#bridge_treasury_find_supported_type">find_supported_type</a>(token_name);
    <b>assert</b>!(type_name.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>let</b> type_name = type_name.destroy_some();
    self.type_to_ids.get_mut(&type_name).push_back(<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>);
    self.id_token_type_map.insert(<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>, type_name);
    self.<a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>.insert(<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>, 0);
    <b>let</b> meta = self.supported_tokens.get_mut(&type_name);
    meta.<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a> = <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>;
    event::emit(<a href="../bridge/treasury.md#bridge_treasury_NewTokenEvent">NewTokenEvent</a> {
        <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>,
        type_name,
        native_token,
        <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>: meta.<a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>,
        <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>,
    });
}
</code></pre>



</details>

<a name="bridge_treasury_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_create">create</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_create">create</a>(ctx: &<b>mut</b> TxContext): <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a> {
    <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a> {
        treasuries: object_bag::new(ctx),
        supported_tokens: vec_map::empty(),
        id_token_type_map: vec_map::empty(),
        waiting_room: bag::new(ctx),
        native_myso_escrow: balance::zero(),
        native_bridge_initialized: <b>false</b>,
        type_to_ids: vec_map::empty(),
        <a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>: vec_map::empty(),
    }
}
</code></pre>



</details>

<a name="bridge_treasury_bootstrap_native_myso_once"></a>

## Function `bootstrap_native_myso_once`

One-time bootstrap: lock exactly <code><a href="../bridge/treasury.md#bridge_treasury_BOOTSTRAP_NATIVE_MYSO_MIST">BOOTSTRAP_NATIVE_MYSO_MIST</a></code>, register MYSO as token id 0 for limiter metadata.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_bootstrap_native_myso_once">bootstrap_native_myso_once</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_bootstrap_native_myso_once">bootstrap_native_myso_once</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, coin: Coin&lt;MYSO&gt;) {
    <b>assert</b>!(!self.native_bridge_initialized, <a href="../bridge/treasury.md#bridge_treasury_ENativeBridgeAlreadyInitialized">ENativeBridgeAlreadyInitialized</a>);
    <b>assert</b>!(coin::value(&coin) == <a href="../bridge/treasury.md#bridge_treasury_BOOTSTRAP_NATIVE_MYSO_MIST">BOOTSTRAP_NATIVE_MYSO_MIST</a>, <a href="../bridge/treasury.md#bridge_treasury_EInvalidBootstrapAmount">EInvalidBootstrapAmount</a>);
    balance::join(&<b>mut</b> self.native_myso_escrow, coin::into_balance(coin));
    self.native_bridge_initialized = <b>true</b>;
    <b>let</b> type_m = type_name::with_defining_ids&lt;MYSO&gt;();
    <b>assert</b>!(!self.supported_tokens.contains(&type_m), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>assert</b>!(!self.id_token_type_map.contains(&0), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>let</b> <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a> = <a href="../bridge/treasury.md#bridge_treasury_MIST_PER_WHOLE_MYSO">MIST_PER_WHOLE_MYSO</a>;
    self
        .supported_tokens
        .insert(
            type_m,
            <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">BridgeTokenMetadata</a> {
                id: 0,
                <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>,
                <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>: <a href="../bridge/treasury.md#bridge_treasury_NATIVE_MYSO_INITIAL_NOTIONAL_USD">NATIVE_MYSO_INITIAL_NOTIONAL_USD</a>,
                native_token: <b>true</b>,
            },
        );
    self.id_token_type_map.insert(0, type_m);
    self.type_to_ids.insert(type_m, vector[0]);
    event::emit(<a href="../bridge/treasury.md#bridge_treasury_NewTokenEvent">NewTokenEvent</a> {
        <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: 0,
        type_name: type_m,
        native_token: <b>true</b>,
        <a href="../bridge/treasury.md#bridge_treasury_decimal_multiplier">decimal_multiplier</a>,
        <a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a>: <a href="../bridge/treasury.md#bridge_treasury_NATIVE_MYSO_INITIAL_NOTIONAL_USD">NATIVE_MYSO_INITIAL_NOTIONAL_USD</a>,
    });
}
</code></pre>



</details>

<a name="bridge_treasury_lock_native_myso"></a>

## Function `lock_native_myso`

Lock MYSO into bridge escrow for <code>send_myso_token</code> (not TreasuryCap burn).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_lock_native_myso">lock_native_myso</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_lock_native_myso">lock_native_myso</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, coin: Coin&lt;MYSO&gt;) {
    <b>assert</b>!(self.native_bridge_initialized, <a href="../bridge/treasury.md#bridge_treasury_ENativeBridgeNotInitialized">ENativeBridgeNotInitialized</a>);
    balance::join(&<b>mut</b> self.native_myso_escrow, coin::into_balance(coin));
}
</code></pre>



</details>

<a name="bridge_treasury_unlock_native_myso"></a>

## Function `unlock_native_myso`

Release MYSO from escrow for a completed inbound transfer claim.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_unlock_native_myso">unlock_native_myso</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_unlock_native_myso">unlock_native_myso</a>(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    amount: u64,
    ctx: &<b>mut</b> TxContext,
): Coin&lt;MYSO&gt; {
    <b>assert</b>!(self.native_bridge_initialized, <a href="../bridge/treasury.md#bridge_treasury_ENativeBridgeNotInitialized">ENativeBridgeNotInitialized</a>);
    <b>assert</b>!(balance::value(&self.native_myso_escrow) &gt;= amount, <a href="../bridge/treasury.md#bridge_treasury_EInsufficientNativeEscrow">EInsufficientNativeEscrow</a>);
    <b>let</b> b = balance::split(&<b>mut</b> self.native_myso_escrow, amount);
    coin::from_balance(b, ctx)
}
</code></pre>



</details>

<a name="bridge_treasury_native_myso_locked_amount"></a>

## Function `native_myso_locked_amount`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_native_myso_locked_amount">native_myso_locked_amount</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_native_myso_locked_amount">native_myso_locked_amount</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): u64 {
    balance::value(&self.native_myso_escrow)
}
</code></pre>



</details>

<a name="bridge_treasury_native_bridge_ready"></a>

## Function `native_bridge_ready`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_native_bridge_ready">native_bridge_ready</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_native_bridge_ready">native_bridge_ready</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): bool {
    self.native_bridge_initialized
}
</code></pre>



</details>

<a name="bridge_treasury_burn"></a>

## Function `burn`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_burn">burn</a>&lt;T&gt;(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, token: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_burn">burn</a>&lt;T&gt;(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, token: Coin&lt;T&gt;) {
    <b>let</b> <a href="../bridge/treasury.md#bridge_treasury">treasury</a> = &<b>mut</b> self.treasuries[type_name::with_defining_ids&lt;T&gt;()];
    coin::burn(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>, token);
}
</code></pre>



</details>

<a name="bridge_treasury_mint"></a>

## Function `mint`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_mint">mint</a>&lt;T&gt;(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_mint">mint</a>&lt;T&gt;(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, amount: u64, ctx: &<b>mut</b> TxContext): Coin&lt;T&gt; {
    <b>let</b> <a href="../bridge/treasury.md#bridge_treasury">treasury</a> = &<b>mut</b> self.treasuries[type_name::with_defining_ids&lt;T&gt;()];
    coin::mint(<a href="../bridge/treasury.md#bridge_treasury">treasury</a>, amount, ctx)
}
</code></pre>



</details>

<a name="bridge_treasury_credit_rail"></a>

## Function `credit_rail`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_credit_rail">credit_rail</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_credit_rail">credit_rail</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8, amount: u64) {
    <b>if</b> (!self.<a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>.contains(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>)) {
        self.<a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>.insert(<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>, 0);
    };
    <b>let</b> reserved = self.<a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>.get_mut(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>);
    *reserved = *reserved + amount;
}
</code></pre>



</details>

<a name="bridge_treasury_debit_rail"></a>

## Function `debit_rail`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_debit_rail">debit_rail</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_debit_rail">debit_rail</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8, amount: u64) {
    <b>assert</b>!(self.<a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>.contains(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>), <a href="../bridge/treasury.md#bridge_treasury_EInsufficientRailReserve">EInsufficientRailReserve</a>);
    <b>let</b> reserved = self.<a href="../bridge/treasury.md#bridge_treasury_rail_reserve">rail_reserve</a>.get_mut(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>);
    <b>assert</b>!(*reserved &gt;= amount, <a href="../bridge/treasury.md#bridge_treasury_EInsufficientRailReserve">EInsufficientRailReserve</a>);
    *reserved = *reserved - amount;
}
</code></pre>



</details>

<a name="bridge_treasury_update_asset_notional_price"></a>

## Function `update_asset_notional_price`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_update_asset_notional_price">update_asset_notional_price</a>(self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8, new_usd_price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_update_asset_notional_price">update_asset_notional_price</a>(
    self: &<b>mut</b> <a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>,
    <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>: u8,
    new_usd_price: u64,
) {
    <b>let</b> type_name = self.id_token_type_map.try_get(&<a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>);
    <b>assert</b>!(type_name.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    <b>assert</b>!(new_usd_price &gt; 0, <a href="../bridge/treasury.md#bridge_treasury_EInvalidNotionalValue">EInvalidNotionalValue</a>);
    <b>let</b> type_name = type_name.destroy_some();
    <b>let</b> metadata = self.supported_tokens.get_mut(&type_name);
    metadata.<a href="../bridge/treasury.md#bridge_treasury_notional_value">notional_value</a> = new_usd_price;
    event::emit(<a href="../bridge/treasury.md#bridge_treasury_UpdateTokenPriceEvent">UpdateTokenPriceEvent</a> {
        <a href="../bridge/treasury.md#bridge_treasury_token_id">token_id</a>,
        new_price: new_usd_price,
    })
}
</code></pre>



</details>

<a name="bridge_treasury_get_token_metadata"></a>

## Function `get_token_metadata`



<pre><code><b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>): <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">bridge::treasury::BridgeTokenMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_get_token_metadata">get_token_metadata</a>&lt;T&gt;(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>): <a href="../bridge/treasury.md#bridge_treasury_BridgeTokenMetadata">BridgeTokenMetadata</a> {
    <b>let</b> coin_type = type_name::with_defining_ids&lt;T&gt;();
    <b>let</b> metadata = self.supported_tokens.try_get(&coin_type);
    <b>assert</b>!(metadata.is_some(), <a href="../bridge/treasury.md#bridge_treasury_EUnsupportedTokenType">EUnsupportedTokenType</a>);
    metadata.destroy_some()
}
</code></pre>



</details>

<a name="bridge_treasury_find_supported_type"></a>

## Function `find_supported_type`



<pre><code><b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_find_supported_type">find_supported_type</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">bridge::treasury::BridgeTreasury</a>, token_name: <a href="../std/ascii.md#std_ascii_String">std::ascii::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/type_name.md#std_type_name_TypeName">std::type_name::TypeName</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../bridge/treasury.md#bridge_treasury_find_supported_type">find_supported_type</a>(self: &<a href="../bridge/treasury.md#bridge_treasury_BridgeTreasury">BridgeTreasury</a>, token_name: String): Option&lt;TypeName&gt; {
    <b>let</b> keys = self.supported_tokens.keys();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; keys.length()) {
        <b>let</b> tn = keys[i];
        <b>if</b> (type_name::into_string(tn) == token_name) {
            <b>return</b> option::some(tn)
        };
        i = i + 1;
    };
    option::none()
}
</code></pre>



</details>
