---
title: Module `orderbook::order`
---

Order module defines the order struct and its methods.
All order matching happens in this module.


-  [Struct `Order`](#orderbook_order_Order)
-  [Struct `OrderCanceled`](#orderbook_order_OrderCanceled)
-  [Struct `OrderModified`](#orderbook_order_OrderModified)
-  [Constants](#@Constants_0)
-  [Function `balance_manager_id`](#orderbook_order_balance_manager_id)
-  [Function `order_id`](#orderbook_order_order_id)
-  [Function `client_order_id`](#orderbook_order_client_order_id)
-  [Function `quantity`](#orderbook_order_quantity)
-  [Function `filled_quantity`](#orderbook_order_filled_quantity)
-  [Function `fee_is_deep`](#orderbook_order_fee_is_deep)
-  [Function `order_myso_price`](#orderbook_order_order_myso_price)
-  [Function `epoch`](#orderbook_order_epoch)
-  [Function `status`](#orderbook_order_status)
-  [Function `expire_timestamp`](#orderbook_order_expire_timestamp)
-  [Function `price`](#orderbook_order_price)
-  [Function `new`](#orderbook_order_new)
-  [Function `generate_fill`](#orderbook_order_generate_fill)
-  [Function `modify`](#orderbook_order_modify)
-  [Function `calculate_cancel_refund`](#orderbook_order_calculate_cancel_refund)
-  [Function `locked_balance`](#orderbook_order_locked_balance)
-  [Function `emit_order_canceled`](#orderbook_order_emit_order_canceled)
-  [Function `emit_order_modified`](#orderbook_order_emit_order_modified)
-  [Function `emit_cancel_maker`](#orderbook_order_emit_cancel_maker)
-  [Function `copy_order`](#orderbook_order_copy_order)
-  [Function `set_canceled`](#orderbook_order_set_canceled)
-  [Function `is_bid`](#orderbook_order_is_bid)


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
<b>use</b> <a href="../orderbook/fill.md#orderbook_fill">orderbook::fill</a>;
<b>use</b> <a href="../orderbook/math.md#orderbook_math">orderbook::math</a>;
<b>use</b> <a href="../orderbook/myso_price.md#orderbook_myso_price">orderbook::myso_price</a>;
<b>use</b> <a href="../orderbook/utils.md#orderbook_utils">orderbook::utils</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/u64.md#std_u64">std::u64</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="orderbook_order_Order"></a>

## Struct `Order`

Order struct represents the order in the order book. It is optimized for space.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/order.md#orderbook_order_Order">Order</a> <b>has</b> drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_fee_is_deep">fee_is_deep</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_status">status</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_order_OrderCanceled"></a>

## Struct `OrderCanceled`

Emitted when a maker order is canceled.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/order.md#orderbook_order_OrderCanceled">OrderCanceled</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>trader: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_price">price</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>original_quantity: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>base_asset_quantity_canceled: u64</code>
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

<a name="orderbook_order_OrderModified"></a>

## Struct `OrderModified`

Emitted when a maker order is modified.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/order.md#orderbook_order_OrderModified">OrderModified</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>trader: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_price">price</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>previous_quantity: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>new_quantity: u64</code>
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

<a name="@Constants_0"></a>

## Constants


<a name="orderbook_order_EInvalidNewQuantity"></a>



<pre><code><b>const</b> <a href="../orderbook/order.md#orderbook_order_EInvalidNewQuantity">EInvalidNewQuantity</a>: u64 = 0;
</code></pre>



<a name="orderbook_order_EOrderExpired"></a>



<pre><code><b>const</b> <a href="../orderbook/order.md#orderbook_order_EOrderExpired">EOrderExpired</a>: u64 = 1;
</code></pre>



<a name="orderbook_order_balance_manager_id"></a>

## Function `balance_manager_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): ID {
    self.<a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>
}
</code></pre>



</details>

<a name="orderbook_order_order_id"></a>

## Function `order_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): u128 {
    self.<a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>
}
</code></pre>



</details>

<a name="orderbook_order_client_order_id"></a>

## Function `client_order_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): u64 {
    self.<a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>
}
</code></pre>



</details>

<a name="orderbook_order_quantity"></a>

## Function `quantity`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): u64 {
    self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>
}
</code></pre>



</details>

<a name="orderbook_order_filled_quantity"></a>

## Function `filled_quantity`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): u64 {
    self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>
}
</code></pre>



</details>

<a name="orderbook_order_fee_is_deep"></a>

## Function `fee_is_deep`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_fee_is_deep">fee_is_deep</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_fee_is_deep">fee_is_deep</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): bool {
    self.<a href="../orderbook/order.md#orderbook_order_fee_is_deep">fee_is_deep</a>
}
</code></pre>



</details>

<a name="orderbook_order_order_myso_price"></a>

## Function `order_myso_price`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): &<a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): &OrderMySoPrice {
    &self.<a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>
}
</code></pre>



</details>

<a name="orderbook_order_epoch"></a>

## Function `epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_epoch">epoch</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_epoch">epoch</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): u64 {
    self.<a href="../orderbook/order.md#orderbook_order_epoch">epoch</a>
}
</code></pre>



</details>

<a name="orderbook_order_status"></a>

## Function `status`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_status">status</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_status">status</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): u8 {
    self.<a href="../orderbook/order.md#orderbook_order_status">status</a>
}
</code></pre>



</details>

<a name="orderbook_order_expire_timestamp"></a>

## Function `expire_timestamp`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): u64 {
    self.<a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a>
}
</code></pre>



</details>

<a name="orderbook_order_price"></a>

## Function `price`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_price">price</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order.md#orderbook_order_price">price</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): u64 {
    <b>let</b> (_, <a href="../orderbook/order.md#orderbook_order_price">price</a>, _) = <a href="../orderbook/utils.md#orderbook_utils_decode_order_id">utils::decode_order_id</a>(self.<a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>);
    <a href="../orderbook/order.md#orderbook_order_price">price</a>
}
</code></pre>



</details>

<a name="orderbook_order_new"></a>

## Function `new`

initialize the order struct.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_new">new</a>(<a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>: u128, <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>: u64, <a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>: u64, <a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>: u64, <a href="../orderbook/order.md#orderbook_order_fee_is_deep">fee_is_deep</a>: bool, <a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>, <a href="../orderbook/order.md#orderbook_order_epoch">epoch</a>: u64, <a href="../orderbook/order.md#orderbook_order_status">status</a>: u8, <a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a>: u64): <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_new">new</a>(
    <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>: u128,
    <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>: ID,
    <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>: u64,
    <a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>: u64,
    <a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>: u64,
    <a href="../orderbook/order.md#orderbook_order_fee_is_deep">fee_is_deep</a>: bool,
    <a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>: OrderMySoPrice,
    <a href="../orderbook/order.md#orderbook_order_epoch">epoch</a>: u64,
    <a href="../orderbook/order.md#orderbook_order_status">status</a>: u8,
    <a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a>: u64,
): <a href="../orderbook/order.md#orderbook_order_Order">Order</a> {
    <a href="../orderbook/order.md#orderbook_order_Order">Order</a> {
        <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>,
        <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>,
        <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>,
        <a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>,
        <a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>,
        <a href="../orderbook/order.md#orderbook_order_fee_is_deep">fee_is_deep</a>,
        <a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>,
        <a href="../orderbook/order.md#orderbook_order_epoch">epoch</a>,
        <a href="../orderbook/order.md#orderbook_order_status">status</a>,
        <a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a>,
    }
}
</code></pre>



</details>

<a name="orderbook_order_generate_fill"></a>

## Function `generate_fill`

Generate a fill for the resting order given the timestamp,
quantity and whether the order is a bid.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_generate_fill">generate_fill</a>(self: &<b>mut</b> <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>, timestamp: u64, <a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>: u64, <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>: bool, expire_maker: bool, taker_fee_is_deep: bool): <a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_generate_fill">generate_fill</a>(
    self: &<b>mut</b> <a href="../orderbook/order.md#orderbook_order_Order">Order</a>,
    timestamp: u64,
    <a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>: u64,
    <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>: bool,
    expire_maker: bool,
    taker_fee_is_deep: bool,
): Fill {
    <b>let</b> remaining_quantity = self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a> - self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>;
    <b>let</b> <b>mut</b> base_quantity = remaining_quantity.min(<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>);
    <b>let</b> <b>mut</b> quote_quantity = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(base_quantity, self.<a href="../orderbook/order.md#orderbook_order_price">price</a>());
    <b>let</b> <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a> = self.<a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>;
    <b>let</b> <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a> = self.<a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>;
    <b>let</b> expired = timestamp &gt; self.<a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a> || expire_maker;
    <b>if</b> (expired) {
        self.<a href="../orderbook/order.md#orderbook_order_status">status</a> = <a href="../orderbook/constants.md#orderbook_constants_expired">constants::expired</a>();
        base_quantity = remaining_quantity;
        quote_quantity = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(base_quantity, self.<a href="../orderbook/order.md#orderbook_order_price">price</a>());
    } <b>else</b> {
        self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a> = self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a> + base_quantity;
        self.<a href="../orderbook/order.md#orderbook_order_status">status</a> = <b>if</b> (self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a> == self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>) <a href="../orderbook/constants.md#orderbook_constants_filled">constants::filled</a>()
        <b>else</b> <a href="../orderbook/constants.md#orderbook_constants_partially_filled">constants::partially_filled</a>();
    };
    <a href="../orderbook/fill.md#orderbook_fill_new">fill::new</a>(
        <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>,
        self.<a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>,
        self.<a href="../orderbook/order.md#orderbook_order_price">price</a>(),
        <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>,
        expired,
        self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a> == self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>,
        self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>,
        base_quantity,
        quote_quantity,
        <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>,
        self.<a href="../orderbook/order.md#orderbook_order_epoch">epoch</a>,
        self.<a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>,
        taker_fee_is_deep,
        self.<a href="../orderbook/order.md#orderbook_order_fee_is_deep">fee_is_deep</a>,
    )
}
</code></pre>



</details>

<a name="orderbook_order_modify"></a>

## Function `modify`

Modify the order with a new quantity. The new quantity must be greater
than the filled quantity and less than the original quantity. The
timestamp must be less than the expire timestamp.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_modify">modify</a>(self: &<b>mut</b> <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>, new_quantity: u64, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_modify">modify</a>(self: &<b>mut</b> <a href="../orderbook/order.md#orderbook_order_Order">Order</a>, new_quantity: u64, timestamp: u64) {
    <b>assert</b>!(
        new_quantity &gt; self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a> &&
        new_quantity &lt; self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>,
        <a href="../orderbook/order.md#orderbook_order_EInvalidNewQuantity">EInvalidNewQuantity</a>,
    );
    <b>assert</b>!(timestamp &lt;= self.<a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a>, <a href="../orderbook/order.md#orderbook_order_EOrderExpired">EOrderExpired</a>);
    self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a> = new_quantity;
}
</code></pre>



</details>

<a name="orderbook_order_calculate_cancel_refund"></a>

## Function `calculate_cancel_refund`

Calculate the refund for a canceled order. The refund is any
unfilled quantity and the maker fee. If the cancel quantity is
not provided, the remaining quantity is used. Cancel quantity is
provided when modifying an order, so that the refund can be calculated
based on the quantity that's reduced.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_calculate_cancel_refund">calculate_cancel_refund</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>, maker_fee: u64, cancel_quantity: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_calculate_cancel_refund">calculate_cancel_refund</a>(
    self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>,
    maker_fee: u64,
    cancel_quantity: Option&lt;u64&gt;,
): Balances {
    <b>let</b> cancel_quantity = cancel_quantity.get_with_default(
        self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a> - self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>,
    );
    <b>let</b> <b>mut</b> fee_quantity = self
        .<a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>
        .fee_quantity(
            cancel_quantity,
            <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(cancel_quantity, self.<a href="../orderbook/order.md#orderbook_order_price">price</a>()),
            self.<a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>(),
        );
    fee_quantity.mul(maker_fee);
    <b>let</b> <b>mut</b> base_out = 0;
    <b>let</b> <b>mut</b> quote_out = 0;
    <b>if</b> (self.<a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>()) {
        quote_out = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(cancel_quantity, self.<a href="../orderbook/order.md#orderbook_order_price">price</a>());
    } <b>else</b> {
        base_out = cancel_quantity;
    };
    <b>let</b> <b>mut</b> refund = <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(base_out, quote_out, 0);
    refund.add_balances(fee_quantity);
    refund
}
</code></pre>



</details>

<a name="orderbook_order_locked_balance"></a>

## Function `locked_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_locked_balance">locked_balance</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>, maker_fee: u64): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_locked_balance">locked_balance</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>, maker_fee: u64): Balances {
    <b>let</b> (<a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>, order_price, _) = <a href="../orderbook/utils.md#orderbook_utils_decode_order_id">utils::decode_order_id</a>(self.<a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>());
    <b>let</b> <b>mut</b> base_quantity = 0;
    <b>let</b> <b>mut</b> quote_quantity = 0;
    <b>let</b> remaining_base_quantity = self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>() - self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>();
    <b>let</b> remaining_quote_quantity = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(
        remaining_base_quantity,
        order_price,
    );
    <b>if</b> (<a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>) {
        quote_quantity = quote_quantity + remaining_quote_quantity;
    } <b>else</b> {
        base_quantity = base_quantity + remaining_base_quantity;
    };
    <b>let</b> <b>mut</b> fee_quantity = self
        .<a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>()
        .fee_quantity(
            remaining_base_quantity,
            remaining_quote_quantity,
            <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>,
        );
    fee_quantity.mul(maker_fee);
    <b>let</b> <b>mut</b> <a href="../orderbook/order.md#orderbook_order_locked_balance">locked_balance</a> = <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(base_quantity, quote_quantity, 0);
    <a href="../orderbook/order.md#orderbook_order_locked_balance">locked_balance</a>.add_balances(fee_quantity);
    <a href="../orderbook/order.md#orderbook_order_locked_balance">locked_balance</a>
}
</code></pre>



</details>

<a name="orderbook_order_emit_order_canceled"></a>

## Function `emit_order_canceled`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_emit_order_canceled">emit_order_canceled</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, trader: <b>address</b>, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_emit_order_canceled">emit_order_canceled</a>(
    self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>,
    pool_id: ID,
    trader: <b>address</b>,
    timestamp: u64,
) {
    <b>let</b> <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a> = self.<a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>();
    <b>let</b> <a href="../orderbook/order.md#orderbook_order_price">price</a> = self.<a href="../orderbook/order.md#orderbook_order_price">price</a>();
    <b>let</b> remaining_quantity = self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a> - self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>;
    event::emit(<a href="../orderbook/order.md#orderbook_order_OrderCanceled">OrderCanceled</a> {
        pool_id,
        <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>: self.<a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>,
        <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>: self.<a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>,
        <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>: self.<a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>,
        <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>,
        trader,
        original_quantity: self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>,
        base_asset_quantity_canceled: remaining_quantity,
        timestamp,
        <a href="../orderbook/order.md#orderbook_order_price">price</a>,
    });
}
</code></pre>



</details>

<a name="orderbook_order_emit_order_modified"></a>

## Function `emit_order_modified`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_emit_order_modified">emit_order_modified</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, previous_quantity: u64, trader: <b>address</b>, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_emit_order_modified">emit_order_modified</a>(
    self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>,
    pool_id: ID,
    previous_quantity: u64,
    trader: <b>address</b>,
    timestamp: u64,
) {
    <b>let</b> <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a> = self.<a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>();
    <b>let</b> <a href="../orderbook/order.md#orderbook_order_price">price</a> = self.<a href="../orderbook/order.md#orderbook_order_price">price</a>();
    event::emit(<a href="../orderbook/order.md#orderbook_order_OrderModified">OrderModified</a> {
        <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>: self.<a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>,
        pool_id,
        <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>: self.<a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>,
        <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>: self.<a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>,
        trader,
        <a href="../orderbook/order.md#orderbook_order_price">price</a>,
        <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>,
        previous_quantity,
        <a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>: self.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>,
        new_quantity: self.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>,
        timestamp,
    });
}
</code></pre>



</details>

<a name="orderbook_order_emit_cancel_maker"></a>

## Function `emit_cancel_maker`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_emit_cancel_maker">emit_cancel_maker</a>(<a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>: u128, <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>: u64, trader: <b>address</b>, <a href="../orderbook/order.md#orderbook_order_price">price</a>: u64, <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>: bool, original_quantity: u64, base_asset_quantity_canceled: u64, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_emit_cancel_maker">emit_cancel_maker</a>(
    <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>: ID,
    pool_id: ID,
    <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>: u128,
    <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>: u64,
    trader: <b>address</b>,
    <a href="../orderbook/order.md#orderbook_order_price">price</a>: u64,
    <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>: bool,
    original_quantity: u64,
    base_asset_quantity_canceled: u64,
    timestamp: u64,
) {
    event::emit(<a href="../orderbook/order.md#orderbook_order_OrderCanceled">OrderCanceled</a> {
        <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>,
        pool_id,
        <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>,
        <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>,
        trader,
        <a href="../orderbook/order.md#orderbook_order_price">price</a>,
        <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>,
        original_quantity,
        base_asset_quantity_canceled,
        timestamp,
    });
}
</code></pre>



</details>

<a name="orderbook_order_copy_order"></a>

## Function `copy_order`

Copy the order struct.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_copy_order">copy_order</a>(<a href="../orderbook/order.md#orderbook_order">order</a>: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_copy_order">copy_order</a>(<a href="../orderbook/order.md#orderbook_order">order</a>: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): <a href="../orderbook/order.md#orderbook_order_Order">Order</a> {
    <a href="../orderbook/order.md#orderbook_order_Order">Order</a> {
        <a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>: <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>,
        <a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>: <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order.md#orderbook_order_balance_manager_id">balance_manager_id</a>,
        <a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>: <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order.md#orderbook_order_client_order_id">client_order_id</a>,
        <a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>: <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order.md#orderbook_order_quantity">quantity</a>,
        <a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>: <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order.md#orderbook_order_filled_quantity">filled_quantity</a>,
        <a href="../orderbook/order.md#orderbook_order_fee_is_deep">fee_is_deep</a>: <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order.md#orderbook_order_fee_is_deep">fee_is_deep</a>,
        <a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>: <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order.md#orderbook_order_order_myso_price">order_myso_price</a>,
        <a href="../orderbook/order.md#orderbook_order_epoch">epoch</a>: <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order.md#orderbook_order_epoch">epoch</a>,
        <a href="../orderbook/order.md#orderbook_order_status">status</a>: <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order.md#orderbook_order_status">status</a>,
        <a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a>: <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order.md#orderbook_order_expire_timestamp">expire_timestamp</a>,
    }
}
</code></pre>



</details>

<a name="orderbook_order_set_canceled"></a>

## Function `set_canceled`

Update the order status to canceled.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_set_canceled">set_canceled</a>(self: &<b>mut</b> <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_set_canceled">set_canceled</a>(self: &<b>mut</b> <a href="../orderbook/order.md#orderbook_order_Order">Order</a>) {
    self.<a href="../orderbook/order.md#orderbook_order_status">status</a> = <a href="../orderbook/constants.md#orderbook_constants_canceled">constants::canceled</a>();
}
</code></pre>



</details>

<a name="orderbook_order_is_bid"></a>

## Function `is_bid`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>(self: &<a href="../orderbook/order.md#orderbook_order_Order">Order</a>): bool {
    <b>let</b> (<a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>, _, _) = <a href="../orderbook/utils.md#orderbook_utils_decode_order_id">utils::decode_order_id</a>(self.<a href="../orderbook/order.md#orderbook_order_order_id">order_id</a>);
    <a href="../orderbook/order.md#orderbook_order_is_bid">is_bid</a>
}
</code></pre>



</details>
