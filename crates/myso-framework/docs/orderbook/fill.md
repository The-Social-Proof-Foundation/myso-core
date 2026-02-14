---
title: Module `orderbook::fill`
---

<code><a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a></code> struct represents the results of a match between two orders.


-  [Struct `Fill`](#orderbook_fill_Fill)
-  [Function `maker_order_id`](#orderbook_fill_maker_order_id)
-  [Function `maker_client_order_id`](#orderbook_fill_maker_client_order_id)
-  [Function `execution_price`](#orderbook_fill_execution_price)
-  [Function `balance_manager_id`](#orderbook_fill_balance_manager_id)
-  [Function `expired`](#orderbook_fill_expired)
-  [Function `completed`](#orderbook_fill_completed)
-  [Function `original_maker_quantity`](#orderbook_fill_original_maker_quantity)
-  [Function `base_quantity`](#orderbook_fill_base_quantity)
-  [Function `taker_is_bid`](#orderbook_fill_taker_is_bid)
-  [Function `quote_quantity`](#orderbook_fill_quote_quantity)
-  [Function `maker_epoch`](#orderbook_fill_maker_epoch)
-  [Function `maker_myso_price`](#orderbook_fill_maker_myso_price)
-  [Function `taker_fee`](#orderbook_fill_taker_fee)
-  [Function `taker_fee_is_myso`](#orderbook_fill_taker_fee_is_myso)
-  [Function `maker_fee`](#orderbook_fill_maker_fee)
-  [Function `maker_fee_is_myso`](#orderbook_fill_maker_fee_is_myso)
-  [Function `new`](#orderbook_fill_new)
-  [Function `get_settled_maker_quantities`](#orderbook_fill_get_settled_maker_quantities)
-  [Function `set_fill_maker_fee`](#orderbook_fill_set_fill_maker_fee)
-  [Function `set_fill_taker_fee`](#orderbook_fill_set_fill_taker_fee)


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
<b>use</b> <a href="../orderbook/myso_price.md#orderbook_myso_price">orderbook::myso_price</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="orderbook_fill_Fill"></a>

## Struct `Fill`

Fill struct represents the results of a match between two orders.
It is used to update the state.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_maker_order_id">maker_order_id</a>: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_maker_client_order_id">maker_client_order_id</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_execution_price">execution_price</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_expired">expired</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_completed">completed</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_original_maker_quantity">original_maker_quantity</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_base_quantity">base_quantity</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_quote_quantity">quote_quantity</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_taker_is_bid">taker_is_bid</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_maker_epoch">maker_epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_maker_myso_price">maker_myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_taker_fee">taker_fee</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_taker_fee_is_myso">taker_fee_is_myso</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_maker_fee">maker_fee</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/fill.md#orderbook_fill_maker_fee_is_myso">maker_fee_is_myso</a>: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_fill_maker_order_id"></a>

## Function `maker_order_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_order_id">maker_order_id</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_order_id">maker_order_id</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): u128 {
    self.<a href="../orderbook/fill.md#orderbook_fill_maker_order_id">maker_order_id</a>
}
</code></pre>



</details>

<a name="orderbook_fill_maker_client_order_id"></a>

## Function `maker_client_order_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_client_order_id">maker_client_order_id</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_client_order_id">maker_client_order_id</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): u64 {
    self.<a href="../orderbook/fill.md#orderbook_fill_maker_client_order_id">maker_client_order_id</a>
}
</code></pre>



</details>

<a name="orderbook_fill_execution_price"></a>

## Function `execution_price`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_execution_price">execution_price</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_execution_price">execution_price</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): u64 {
    self.<a href="../orderbook/fill.md#orderbook_fill_execution_price">execution_price</a>
}
</code></pre>



</details>

<a name="orderbook_fill_balance_manager_id"></a>

## Function `balance_manager_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_balance_manager_id">balance_manager_id</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_balance_manager_id">balance_manager_id</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): ID {
    self.<a href="../orderbook/fill.md#orderbook_fill_balance_manager_id">balance_manager_id</a>
}
</code></pre>



</details>

<a name="orderbook_fill_expired"></a>

## Function `expired`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_expired">expired</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_expired">expired</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): bool {
    self.<a href="../orderbook/fill.md#orderbook_fill_expired">expired</a>
}
</code></pre>



</details>

<a name="orderbook_fill_completed"></a>

## Function `completed`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_completed">completed</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_completed">completed</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): bool {
    self.<a href="../orderbook/fill.md#orderbook_fill_completed">completed</a>
}
</code></pre>



</details>

<a name="orderbook_fill_original_maker_quantity"></a>

## Function `original_maker_quantity`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_original_maker_quantity">original_maker_quantity</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_original_maker_quantity">original_maker_quantity</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): u64 {
    self.<a href="../orderbook/fill.md#orderbook_fill_original_maker_quantity">original_maker_quantity</a>
}
</code></pre>



</details>

<a name="orderbook_fill_base_quantity"></a>

## Function `base_quantity`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_base_quantity">base_quantity</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_base_quantity">base_quantity</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): u64 {
    self.<a href="../orderbook/fill.md#orderbook_fill_base_quantity">base_quantity</a>
}
</code></pre>



</details>

<a name="orderbook_fill_taker_is_bid"></a>

## Function `taker_is_bid`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_taker_is_bid">taker_is_bid</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_taker_is_bid">taker_is_bid</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): bool {
    self.<a href="../orderbook/fill.md#orderbook_fill_taker_is_bid">taker_is_bid</a>
}
</code></pre>



</details>

<a name="orderbook_fill_quote_quantity"></a>

## Function `quote_quantity`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_quote_quantity">quote_quantity</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_quote_quantity">quote_quantity</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): u64 {
    self.<a href="../orderbook/fill.md#orderbook_fill_quote_quantity">quote_quantity</a>
}
</code></pre>



</details>

<a name="orderbook_fill_maker_epoch"></a>

## Function `maker_epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_epoch">maker_epoch</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_epoch">maker_epoch</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): u64 {
    self.<a href="../orderbook/fill.md#orderbook_fill_maker_epoch">maker_epoch</a>
}
</code></pre>



</details>

<a name="orderbook_fill_maker_myso_price"></a>

## Function `maker_myso_price`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_myso_price">maker_myso_price</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_myso_price">maker_myso_price</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): OrderMySoPrice {
    self.<a href="../orderbook/fill.md#orderbook_fill_maker_myso_price">maker_myso_price</a>
}
</code></pre>



</details>

<a name="orderbook_fill_taker_fee"></a>

## Function `taker_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_taker_fee">taker_fee</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_taker_fee">taker_fee</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): u64 {
    self.<a href="../orderbook/fill.md#orderbook_fill_taker_fee">taker_fee</a>
}
</code></pre>



</details>

<a name="orderbook_fill_taker_fee_is_myso"></a>

## Function `taker_fee_is_myso`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_taker_fee_is_myso">taker_fee_is_myso</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_taker_fee_is_myso">taker_fee_is_myso</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): bool {
    self.<a href="../orderbook/fill.md#orderbook_fill_taker_fee_is_myso">taker_fee_is_myso</a>
}
</code></pre>



</details>

<a name="orderbook_fill_maker_fee"></a>

## Function `maker_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_fee">maker_fee</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_fee">maker_fee</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): u64 {
    self.<a href="../orderbook/fill.md#orderbook_fill_maker_fee">maker_fee</a>
}
</code></pre>



</details>

<a name="orderbook_fill_maker_fee_is_myso"></a>

## Function `maker_fee_is_myso`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_fee_is_myso">maker_fee_is_myso</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_maker_fee_is_myso">maker_fee_is_myso</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): bool {
    self.<a href="../orderbook/fill.md#orderbook_fill_maker_fee_is_myso">maker_fee_is_myso</a>
}
</code></pre>



</details>

<a name="orderbook_fill_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_new">new</a>(<a href="../orderbook/fill.md#orderbook_fill_maker_order_id">maker_order_id</a>: u128, <a href="../orderbook/fill.md#orderbook_fill_maker_client_order_id">maker_client_order_id</a>: u64, <a href="../orderbook/fill.md#orderbook_fill_execution_price">execution_price</a>: u64, <a href="../orderbook/fill.md#orderbook_fill_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../orderbook/fill.md#orderbook_fill_expired">expired</a>: bool, <a href="../orderbook/fill.md#orderbook_fill_completed">completed</a>: bool, <a href="../orderbook/fill.md#orderbook_fill_original_maker_quantity">original_maker_quantity</a>: u64, <a href="../orderbook/fill.md#orderbook_fill_base_quantity">base_quantity</a>: u64, <a href="../orderbook/fill.md#orderbook_fill_quote_quantity">quote_quantity</a>: u64, <a href="../orderbook/fill.md#orderbook_fill_taker_is_bid">taker_is_bid</a>: bool, <a href="../orderbook/fill.md#orderbook_fill_maker_epoch">maker_epoch</a>: u64, <a href="../orderbook/fill.md#orderbook_fill_maker_myso_price">maker_myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>, <a href="../orderbook/fill.md#orderbook_fill_taker_fee_is_myso">taker_fee_is_myso</a>: bool, <a href="../orderbook/fill.md#orderbook_fill_maker_fee_is_myso">maker_fee_is_myso</a>: bool): <a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_new">new</a>(
    <a href="../orderbook/fill.md#orderbook_fill_maker_order_id">maker_order_id</a>: u128,
    <a href="../orderbook/fill.md#orderbook_fill_maker_client_order_id">maker_client_order_id</a>: u64,
    <a href="../orderbook/fill.md#orderbook_fill_execution_price">execution_price</a>: u64,
    <a href="../orderbook/fill.md#orderbook_fill_balance_manager_id">balance_manager_id</a>: ID,
    <a href="../orderbook/fill.md#orderbook_fill_expired">expired</a>: bool,
    <a href="../orderbook/fill.md#orderbook_fill_completed">completed</a>: bool,
    <a href="../orderbook/fill.md#orderbook_fill_original_maker_quantity">original_maker_quantity</a>: u64,
    <a href="../orderbook/fill.md#orderbook_fill_base_quantity">base_quantity</a>: u64,
    <a href="../orderbook/fill.md#orderbook_fill_quote_quantity">quote_quantity</a>: u64,
    <a href="../orderbook/fill.md#orderbook_fill_taker_is_bid">taker_is_bid</a>: bool,
    <a href="../orderbook/fill.md#orderbook_fill_maker_epoch">maker_epoch</a>: u64,
    <a href="../orderbook/fill.md#orderbook_fill_maker_myso_price">maker_myso_price</a>: OrderMySoPrice,
    <a href="../orderbook/fill.md#orderbook_fill_taker_fee_is_myso">taker_fee_is_myso</a>: bool,
    <a href="../orderbook/fill.md#orderbook_fill_maker_fee_is_myso">maker_fee_is_myso</a>: bool,
): <a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a> {
    <a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a> {
        <a href="../orderbook/fill.md#orderbook_fill_maker_order_id">maker_order_id</a>,
        <a href="../orderbook/fill.md#orderbook_fill_maker_client_order_id">maker_client_order_id</a>,
        <a href="../orderbook/fill.md#orderbook_fill_execution_price">execution_price</a>,
        <a href="../orderbook/fill.md#orderbook_fill_balance_manager_id">balance_manager_id</a>,
        <a href="../orderbook/fill.md#orderbook_fill_expired">expired</a>,
        <a href="../orderbook/fill.md#orderbook_fill_completed">completed</a>,
        <a href="../orderbook/fill.md#orderbook_fill_original_maker_quantity">original_maker_quantity</a>,
        <a href="../orderbook/fill.md#orderbook_fill_base_quantity">base_quantity</a>,
        <a href="../orderbook/fill.md#orderbook_fill_quote_quantity">quote_quantity</a>,
        <a href="../orderbook/fill.md#orderbook_fill_taker_is_bid">taker_is_bid</a>,
        <a href="../orderbook/fill.md#orderbook_fill_maker_epoch">maker_epoch</a>,
        <a href="../orderbook/fill.md#orderbook_fill_maker_myso_price">maker_myso_price</a>,
        <a href="../orderbook/fill.md#orderbook_fill_taker_fee">taker_fee</a>: 0,
        <a href="../orderbook/fill.md#orderbook_fill_taker_fee_is_myso">taker_fee_is_myso</a>,
        <a href="../orderbook/fill.md#orderbook_fill_maker_fee">maker_fee</a>: 0,
        <a href="../orderbook/fill.md#orderbook_fill_maker_fee_is_myso">maker_fee_is_myso</a>,
    }
}
</code></pre>



</details>

<a name="orderbook_fill_get_settled_maker_quantities"></a>

## Function `get_settled_maker_quantities`

Calculate the quantities to settle for the maker.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_get_settled_maker_quantities">get_settled_maker_quantities</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_get_settled_maker_quantities">get_settled_maker_quantities</a>(self: &<a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>): Balances {
    <b>let</b> (base, quote) = <b>if</b> (self.<a href="../orderbook/fill.md#orderbook_fill_expired">expired</a>) {
        <b>if</b> (self.<a href="../orderbook/fill.md#orderbook_fill_taker_is_bid">taker_is_bid</a>) {
            (self.<a href="../orderbook/fill.md#orderbook_fill_base_quantity">base_quantity</a>, 0)
        } <b>else</b> {
            (0, self.<a href="../orderbook/fill.md#orderbook_fill_quote_quantity">quote_quantity</a>)
        }
    } <b>else</b> {
        <b>if</b> (self.<a href="../orderbook/fill.md#orderbook_fill_taker_is_bid">taker_is_bid</a>) {
            (0, self.<a href="../orderbook/fill.md#orderbook_fill_quote_quantity">quote_quantity</a>)
        } <b>else</b> {
            (self.<a href="../orderbook/fill.md#orderbook_fill_base_quantity">base_quantity</a>, 0)
        }
    };
    <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(base, quote, 0)
}
</code></pre>



</details>

<a name="orderbook_fill_set_fill_maker_fee"></a>

## Function `set_fill_maker_fee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_set_fill_maker_fee">set_fill_maker_fee</a>(self: &<b>mut</b> <a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>, fee: &<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_set_fill_maker_fee">set_fill_maker_fee</a>(self: &<b>mut</b> <a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>, fee: &Balances) {
    <b>if</b> (fee.myso() &gt; 0) {
        self.<a href="../orderbook/fill.md#orderbook_fill_maker_fee_is_myso">maker_fee_is_myso</a> = <b>true</b>;
    } <b>else</b> {
        self.<a href="../orderbook/fill.md#orderbook_fill_maker_fee_is_myso">maker_fee_is_myso</a> = <b>false</b>;
    };
    self.<a href="../orderbook/fill.md#orderbook_fill_maker_fee">maker_fee</a> = fee.non_zero_value();
}
</code></pre>



</details>

<a name="orderbook_fill_set_fill_taker_fee"></a>

## Function `set_fill_taker_fee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_set_fill_taker_fee">set_fill_taker_fee</a>(self: &<b>mut</b> <a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>, fee: &<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/fill.md#orderbook_fill_set_fill_taker_fee">set_fill_taker_fee</a>(self: &<b>mut</b> <a href="../orderbook/fill.md#orderbook_fill_Fill">Fill</a>, fee: &Balances) {
    <b>if</b> (fee.myso() &gt; 0) {
        self.<a href="../orderbook/fill.md#orderbook_fill_taker_fee_is_myso">taker_fee_is_myso</a> = <b>true</b>;
    } <b>else</b> {
        self.<a href="../orderbook/fill.md#orderbook_fill_taker_fee_is_myso">taker_fee_is_myso</a> = <b>false</b>;
    };
    self.<a href="../orderbook/fill.md#orderbook_fill_taker_fee">taker_fee</a> = fee.non_zero_value();
}
</code></pre>



</details>
