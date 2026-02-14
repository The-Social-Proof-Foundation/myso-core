---
title: Module `orderbook::myso_price`
---

MYSO price module. This module maintains the conversion rate
between MYSO and the base and quote assets.


-  [Struct `Price`](#orderbook_myso_price_Price)
-  [Struct `PriceAdded`](#orderbook_myso_price_PriceAdded)
-  [Struct `MySoPrice`](#orderbook_myso_price_MySoPrice)
-  [Struct `OrderMySoPrice`](#orderbook_myso_price_OrderMySoPrice)
-  [Constants](#@Constants_0)
-  [Function `asset_is_base`](#orderbook_myso_price_asset_is_base)
-  [Function `myso_per_asset`](#orderbook_myso_price_myso_per_asset)
-  [Function `empty`](#orderbook_myso_price_empty)
-  [Function `new_order_myso_price`](#orderbook_myso_price_new_order_myso_price)
-  [Function `get_order_myso_price`](#orderbook_myso_price_get_order_myso_price)
-  [Function `empty_myso_price`](#orderbook_myso_price_empty_myso_price)
-  [Function `fee_quantity`](#orderbook_myso_price_fee_quantity)
-  [Function `myso_quantity_u128`](#orderbook_myso_price_myso_quantity_u128)
-  [Function `add_price_point`](#orderbook_myso_price_add_price_point)
-  [Function `emit_myso_price_added`](#orderbook_myso_price_emit_myso_price_added)
-  [Function `calculate_order_myso_price`](#orderbook_myso_price_calculate_order_myso_price)
-  [Function `last_insert_timestamp`](#orderbook_myso_price_last_insert_timestamp)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../orderbook/balances.md#orderbook_balances">orderbook::balances</a>;
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/math.md#orderbook_math">orderbook::math</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="orderbook_myso_price_Price"></a>

## Struct `Price`

MYSO price point.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_Price">Price</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conversion_rate: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_myso_price_PriceAdded"></a>

## Struct `PriceAdded`

MYSO price point added event.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_PriceAdded">PriceAdded</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>conversion_rate: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>is_base_conversion: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>reference_pool: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>target_pool: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_myso_price_MySoPrice"></a>

## Struct `MySoPrice`

MYSO price points used for trading fee calculations.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">MySoPrice</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>base_prices: vector&lt;<a href="../orderbook/myso_price.md#orderbook_myso_price_Price">orderbook::myso_price::Price</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>cumulative_base: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quote_prices: vector&lt;<a href="../orderbook/myso_price.md#orderbook_myso_price_Price">orderbook::myso_price::Price</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>cumulative_quote: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_myso_price_OrderMySoPrice"></a>

## Struct `OrderMySoPrice`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">OrderMySoPrice</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="orderbook_myso_price_EDataPointRecentlyAdded"></a>



<pre><code><b>const</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_EDataPointRecentlyAdded">EDataPointRecentlyAdded</a>: u64 = 1;
</code></pre>



<a name="orderbook_myso_price_ENoDataPoints"></a>



<pre><code><b>const</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_ENoDataPoints">ENoDataPoints</a>: u64 = 2;
</code></pre>



<a name="orderbook_myso_price_MIN_DURATION_BETWEEN_DATA_POINTS_MS"></a>



<pre><code><b>const</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_MIN_DURATION_BETWEEN_DATA_POINTS_MS">MIN_DURATION_BETWEEN_DATA_POINTS_MS</a>: u64 = 60000;
</code></pre>



<a name="orderbook_myso_price_MAX_DATA_POINT_AGE_MS"></a>



<pre><code><b>const</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_MAX_DATA_POINT_AGE_MS">MAX_DATA_POINT_AGE_MS</a>: u64 = 86400000;
</code></pre>



<a name="orderbook_myso_price_MAX_DATA_POINTS"></a>



<pre><code><b>const</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_MAX_DATA_POINTS">MAX_DATA_POINTS</a>: u64 = 100;
</code></pre>



<a name="orderbook_myso_price_asset_is_base"></a>

## Function `asset_is_base`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">OrderMySoPrice</a>): bool {
    self.<a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>
}
</code></pre>



</details>

<a name="orderbook_myso_price_myso_per_asset"></a>

## Function `myso_per_asset`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">OrderMySoPrice</a>): u64 {
    self.<a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>
}
</code></pre>



</details>

<a name="orderbook_myso_price_empty"></a>

## Function `empty`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_empty">empty</a>(): <a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">orderbook::myso_price::MySoPrice</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_empty">empty</a>(): <a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">MySoPrice</a> {
    <a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">MySoPrice</a> {
        base_prices: vector[],
        cumulative_base: 0,
        quote_prices: vector[],
        cumulative_quote: 0,
    }
}
</code></pre>



</details>

<a name="orderbook_myso_price_new_order_myso_price"></a>

## Function `new_order_myso_price`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_new_order_myso_price">new_order_myso_price</a>(<a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>: bool, <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>: u64): <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_new_order_myso_price">new_order_myso_price</a>(<a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>: bool, <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>: u64): <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">OrderMySoPrice</a> {
    <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">OrderMySoPrice</a> {
        <a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>,
        <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>,
    }
}
</code></pre>



</details>

<a name="orderbook_myso_price_get_order_myso_price"></a>

## Function `get_order_myso_price`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_get_order_myso_price">get_order_myso_price</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">orderbook::myso_price::MySoPrice</a>, whitelisted: bool): <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_get_order_myso_price">get_order_myso_price</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">MySoPrice</a>, whitelisted: bool): <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">OrderMySoPrice</a> {
    <b>let</b> (<a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>, <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>) = self.<a href="../orderbook/myso_price.md#orderbook_myso_price_calculate_order_myso_price">calculate_order_myso_price</a>(
        whitelisted,
    );
    <a href="../orderbook/myso_price.md#orderbook_myso_price_new_order_myso_price">new_order_myso_price</a>(<a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>, <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>)
}
</code></pre>



</details>

<a name="orderbook_myso_price_empty_myso_price"></a>

## Function `empty_myso_price`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_empty_myso_price">empty_myso_price</a>(_self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">orderbook::myso_price::MySoPrice</a>): <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_empty_myso_price">empty_myso_price</a>(_self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">MySoPrice</a>): <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">OrderMySoPrice</a> {
    <a href="../orderbook/myso_price.md#orderbook_myso_price_new_order_myso_price">new_order_myso_price</a>(<b>false</b>, 0)
}
</code></pre>



</details>

<a name="orderbook_myso_price_fee_quantity"></a>

## Function `fee_quantity`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_fee_quantity">fee_quantity</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>, base_quantity: u64, quote_quantity: u64, is_bid: bool): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_fee_quantity">fee_quantity</a>(
    self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">OrderMySoPrice</a>,
    base_quantity: u64,
    quote_quantity: u64,
    is_bid: bool,
): Balances {
    <b>let</b> myso_quantity = <b>if</b> (self.<a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>) {
        <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(base_quantity, self.<a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>)
    } <b>else</b> {
        <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(quote_quantity, self.<a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>)
    };
    <b>if</b> (self.<a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a> &gt; 0) {
        <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(0, 0, myso_quantity)
    } <b>else</b> <b>if</b> (is_bid) {
        <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(
            0,
            <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(
                quote_quantity,
                <a href="../orderbook/constants.md#orderbook_constants_fee_penalty_multiplier">constants::fee_penalty_multiplier</a>(),
            ),
            0,
        )
    } <b>else</b> {
        <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(
            <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(base_quantity, <a href="../orderbook/constants.md#orderbook_constants_fee_penalty_multiplier">constants::fee_penalty_multiplier</a>()),
            0,
            0,
        )
    }
}
</code></pre>



</details>

<a name="orderbook_myso_price_myso_quantity_u128"></a>

## Function `myso_quantity_u128`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_quantity_u128">myso_quantity_u128</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>, base_quantity: u128, quote_quantity: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_quantity_u128">myso_quantity_u128</a>(
    self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">OrderMySoPrice</a>,
    base_quantity: u128,
    quote_quantity: u128,
): u128 {
    <b>if</b> (self.<a href="../orderbook/myso_price.md#orderbook_myso_price_asset_is_base">asset_is_base</a>) {
        <a href="../orderbook/math.md#orderbook_math_mul_u128">math::mul_u128</a>(base_quantity, self.<a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a> <b>as</b> u128)
    } <b>else</b> {
        <a href="../orderbook/math.md#orderbook_math_mul_u128">math::mul_u128</a>(quote_quantity, self.<a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a> <b>as</b> u128)
    }
}
</code></pre>



</details>

<a name="orderbook_myso_price_add_price_point"></a>

## Function `add_price_point`

Add a price point. If max data points are reached, the oldest data point is removed.
Remove all data points older than MAX_DATA_POINT_AGE_MS.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_add_price_point">add_price_point</a>(self: &<b>mut</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">orderbook::myso_price::MySoPrice</a>, conversion_rate: u64, timestamp: u64, is_base_conversion: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_add_price_point">add_price_point</a>(
    self: &<b>mut</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">MySoPrice</a>,
    conversion_rate: u64,
    timestamp: u64,
    is_base_conversion: bool,
) {
    <b>assert</b>!(
        self.<a href="../orderbook/myso_price.md#orderbook_myso_price_last_insert_timestamp">last_insert_timestamp</a>(is_base_conversion) +
        <a href="../orderbook/myso_price.md#orderbook_myso_price_MIN_DURATION_BETWEEN_DATA_POINTS_MS">MIN_DURATION_BETWEEN_DATA_POINTS_MS</a> &lt;
        timestamp,
        <a href="../orderbook/myso_price.md#orderbook_myso_price_EDataPointRecentlyAdded">EDataPointRecentlyAdded</a>,
    );
    <b>let</b> asset_prices = <b>if</b> (is_base_conversion) {
        &<b>mut</b> self.base_prices
    } <b>else</b> {
        &<b>mut</b> self.quote_prices
    };
    asset_prices.push_back(<a href="../orderbook/myso_price.md#orderbook_myso_price_Price">Price</a> {
        timestamp: timestamp,
        conversion_rate: conversion_rate,
    });
    <b>if</b> (is_base_conversion) {
        self.cumulative_base = self.cumulative_base + conversion_rate;
        <b>while</b> (
            asset_prices.length() == <a href="../orderbook/myso_price.md#orderbook_myso_price_MAX_DATA_POINTS">MAX_DATA_POINTS</a> + 1 ||
            asset_prices[0].timestamp + <a href="../orderbook/myso_price.md#orderbook_myso_price_MAX_DATA_POINT_AGE_MS">MAX_DATA_POINT_AGE_MS</a> &lt; timestamp
        ) {
            self.cumulative_base = self.cumulative_base - asset_prices[0].conversion_rate;
            asset_prices.remove(0);
        }
    } <b>else</b> {
        self.cumulative_quote = self.cumulative_quote + conversion_rate;
        <b>while</b> (
            asset_prices.length() == <a href="../orderbook/myso_price.md#orderbook_myso_price_MAX_DATA_POINTS">MAX_DATA_POINTS</a> + 1 ||
            asset_prices[0].timestamp + <a href="../orderbook/myso_price.md#orderbook_myso_price_MAX_DATA_POINT_AGE_MS">MAX_DATA_POINT_AGE_MS</a> &lt; timestamp
        ) {
            self.cumulative_quote = self.cumulative_quote - asset_prices[0].conversion_rate;
            asset_prices.remove(0);
        }
    };
}
</code></pre>



</details>

<a name="orderbook_myso_price_emit_myso_price_added"></a>

## Function `emit_myso_price_added`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_emit_myso_price_added">emit_myso_price_added</a>(conversion_rate: u64, timestamp: u64, is_base_conversion: bool, reference_pool: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, target_pool: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_emit_myso_price_added">emit_myso_price_added</a>(
    conversion_rate: u64,
    timestamp: u64,
    is_base_conversion: bool,
    reference_pool: ID,
    target_pool: ID,
) {
    event::emit(<a href="../orderbook/myso_price.md#orderbook_myso_price_PriceAdded">PriceAdded</a> {
        conversion_rate,
        timestamp,
        is_base_conversion,
        reference_pool,
        target_pool,
    });
}
</code></pre>



</details>

<a name="orderbook_myso_price_calculate_order_myso_price"></a>

## Function `calculate_order_myso_price`

Returns the conversion rate of MYSO per asset token.
Quote will be used by default, if there are no quote data then base will be used


<pre><code><b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_calculate_order_myso_price">calculate_order_myso_price</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">orderbook::myso_price::MySoPrice</a>, whitelisted: bool): (bool, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_calculate_order_myso_price">calculate_order_myso_price</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">MySoPrice</a>, whitelisted: bool): (bool, u64) {
    <b>if</b> (whitelisted) {
        <b>return</b> (<b>false</b>, 0) // no fees <b>for</b> whitelist
    };
    <b>assert</b>!(
        self.<a href="../orderbook/myso_price.md#orderbook_myso_price_last_insert_timestamp">last_insert_timestamp</a>(<b>true</b>) &gt; 0 ||
        self.<a href="../orderbook/myso_price.md#orderbook_myso_price_last_insert_timestamp">last_insert_timestamp</a>(<b>false</b>) &gt; 0,
        <a href="../orderbook/myso_price.md#orderbook_myso_price_ENoDataPoints">ENoDataPoints</a>,
    );
    <b>let</b> is_base_conversion = self.<a href="../orderbook/myso_price.md#orderbook_myso_price_last_insert_timestamp">last_insert_timestamp</a>(<b>false</b>) == 0;
    <b>let</b> cumulative_asset = <b>if</b> (is_base_conversion) {
        self.cumulative_base
    } <b>else</b> {
        self.cumulative_quote
    };
    <b>let</b> asset_length = <b>if</b> (is_base_conversion) {
        self.base_prices.length()
    } <b>else</b> {
        self.quote_prices.length()
    };
    <b>let</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a> = cumulative_asset / asset_length;
    (is_base_conversion, <a href="../orderbook/myso_price.md#orderbook_myso_price_myso_per_asset">myso_per_asset</a>)
}
</code></pre>



</details>

<a name="orderbook_myso_price_last_insert_timestamp"></a>

## Function `last_insert_timestamp`



<pre><code><b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_last_insert_timestamp">last_insert_timestamp</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">orderbook::myso_price::MySoPrice</a>, is_base_conversion: bool): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/myso_price.md#orderbook_myso_price_last_insert_timestamp">last_insert_timestamp</a>(self: &<a href="../orderbook/myso_price.md#orderbook_myso_price_MySoPrice">MySoPrice</a>, is_base_conversion: bool): u64 {
    <b>let</b> prices = <b>if</b> (is_base_conversion) {
        &self.base_prices
    } <b>else</b> {
        &self.quote_prices
    };
    <b>if</b> (prices.length() &gt; 0) {
        prices[prices.length() - 1].timestamp
    } <b>else</b> {
        0
    }
}
</code></pre>



</details>
