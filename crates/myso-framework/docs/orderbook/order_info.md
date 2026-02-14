---
title: Module `orderbook::order_info`
---

Order module defines the order struct and its methods.
All order matching happens in this module.


-  [Struct `OrderInfo`](#orderbook_order_info_OrderInfo)
-  [Struct `OrderFilled`](#orderbook_order_info_OrderFilled)
-  [Struct `OrderPlaced`](#orderbook_order_info_OrderPlaced)
-  [Struct `OrderExpired`](#orderbook_order_info_OrderExpired)
-  [Struct `OrderFullyFilled`](#orderbook_order_info_OrderFullyFilled)
-  [Constants](#@Constants_0)
-  [Function `pool_id`](#orderbook_order_info_pool_id)
-  [Function `order_id`](#orderbook_order_info_order_id)
-  [Function `balance_manager_id`](#orderbook_order_info_balance_manager_id)
-  [Function `client_order_id`](#orderbook_order_info_client_order_id)
-  [Function `trader`](#orderbook_order_info_trader)
-  [Function `order_type`](#orderbook_order_info_order_type)
-  [Function `self_matching_option`](#orderbook_order_info_self_matching_option)
-  [Function `price`](#orderbook_order_info_price)
-  [Function `is_bid`](#orderbook_order_info_is_bid)
-  [Function `original_quantity`](#orderbook_order_info_original_quantity)
-  [Function `order_myso_price`](#orderbook_order_info_order_myso_price)
-  [Function `expire_timestamp`](#orderbook_order_info_expire_timestamp)
-  [Function `executed_quantity`](#orderbook_order_info_executed_quantity)
-  [Function `cumulative_quote_quantity`](#orderbook_order_info_cumulative_quote_quantity)
-  [Function `fills`](#orderbook_order_info_fills)
-  [Function `fee_is_myso`](#orderbook_order_info_fee_is_myso)
-  [Function `paid_fees`](#orderbook_order_info_paid_fees)
-  [Function `maker_fees`](#orderbook_order_info_maker_fees)
-  [Function `epoch`](#orderbook_order_info_epoch)
-  [Function `status`](#orderbook_order_info_status)
-  [Function `fill_limit_reached`](#orderbook_order_info_fill_limit_reached)
-  [Function `order_inserted`](#orderbook_order_info_order_inserted)
-  [Function `new`](#orderbook_order_info_new)
-  [Function `market_order`](#orderbook_order_info_market_order)
-  [Function `set_order_id`](#orderbook_order_info_set_order_id)
-  [Function `set_paid_fees`](#orderbook_order_info_set_paid_fees)
-  [Function `add_fill`](#orderbook_order_info_add_fill)
-  [Function `fills_ref`](#orderbook_order_info_fills_ref)
-  [Function `paid_fees_balances`](#orderbook_order_info_paid_fees_balances)
-  [Function `calculate_partial_fill_balances`](#orderbook_order_info_calculate_partial_fill_balances)
-  [Function `to_order`](#orderbook_order_info_to_order)
-  [Function `validate_inputs`](#orderbook_order_info_validate_inputs)
-  [Function `assert_execution`](#orderbook_order_info_assert_execution)
-  [Function `remaining_quantity`](#orderbook_order_info_remaining_quantity)
-  [Function `can_match`](#orderbook_order_info_can_match)
-  [Function `match_maker`](#orderbook_order_info_match_maker)
-  [Function `emit_orders_filled`](#orderbook_order_info_emit_orders_filled)
-  [Function `emit_order_placed`](#orderbook_order_info_emit_order_placed)
-  [Function `emit_order_info`](#orderbook_order_info_emit_order_info)
-  [Function `emit_order_fully_filled_if_filled`](#orderbook_order_info_emit_order_fully_filled_if_filled)
-  [Function `emit_order_fully_filled`](#orderbook_order_info_emit_order_fully_filled)
-  [Function `set_fill_limit_reached`](#orderbook_order_info_set_fill_limit_reached)
-  [Function `set_order_inserted`](#orderbook_order_info_set_order_inserted)
-  [Function `order_filled_from_fill`](#orderbook_order_info_order_filled_from_fill)
-  [Function `order_expired_from_fill`](#orderbook_order_info_order_expired_from_fill)
-  [Function `emit_order_canceled_maker_from_fill`](#orderbook_order_info_emit_order_canceled_maker_from_fill)


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
<b>use</b> <a href="../orderbook/order.md#orderbook_order">orderbook::order</a>;
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



<a name="orderbook_order_info_OrderInfo"></a>

## Struct `OrderInfo`

OrderInfo struct represents all order information.
This objects gets created at the beginning of the order lifecycle and
gets updated until it is completed or placed in the book.
It is returned at the end of the order lifecycle.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_self_matching_option">self_matching_option</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_cumulative_quote_quantity">cumulative_quote_quantity</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>: vector&lt;<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_fee_is_myso">fee_is_myso</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_maker_fees">maker_fees</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_epoch">epoch</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_status">status</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_market_order">market_order</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_fill_limit_reached">fill_limit_reached</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_order_inserted">order_inserted</a>: bool</code>
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

<a name="orderbook_order_info_OrderFilled"></a>

## Struct `OrderFilled`

Emitted when a maker order is filled.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderFilled">OrderFilled</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>maker_order_id: u128</code>
</dt>
<dd>
</dd>
<dt>
<code>taker_order_id: u128</code>
</dt>
<dd>
</dd>
<dt>
<code>maker_client_order_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>taker_client_order_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>taker_is_bid: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>taker_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>taker_fee_is_myso: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>maker_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>maker_fee_is_myso: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>base_quantity: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quote_quantity: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>maker_balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>taker_balance_manager_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
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

<a name="orderbook_order_info_OrderPlaced"></a>

## Struct `OrderPlaced`

Emitted when a maker order is injected into the order book.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderPlaced">OrderPlaced</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>placed_quantity: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>: u64</code>
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

<a name="orderbook_order_info_OrderExpired"></a>

## Struct `OrderExpired`

Emitted when a maker order is expired.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderExpired">OrderExpired</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>: u64</code>
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

<a name="orderbook_order_info_OrderFullyFilled"></a>

## Struct `OrderFullyFilled`

Emitted when an order is fully filled.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderFullyFilled">OrderFullyFilled</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>: bool</code>
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


<a name="orderbook_order_info_EOrderInvalidPrice"></a>



<pre><code><b>const</b> <a href="../orderbook/order_info.md#orderbook_order_info_EOrderInvalidPrice">EOrderInvalidPrice</a>: u64 = 0;
</code></pre>



<a name="orderbook_order_info_EOrderBelowMinimumSize"></a>



<pre><code><b>const</b> <a href="../orderbook/order_info.md#orderbook_order_info_EOrderBelowMinimumSize">EOrderBelowMinimumSize</a>: u64 = 1;
</code></pre>



<a name="orderbook_order_info_EOrderInvalidLotSize"></a>



<pre><code><b>const</b> <a href="../orderbook/order_info.md#orderbook_order_info_EOrderInvalidLotSize">EOrderInvalidLotSize</a>: u64 = 2;
</code></pre>



<a name="orderbook_order_info_EInvalidExpireTimestamp"></a>



<pre><code><b>const</b> <a href="../orderbook/order_info.md#orderbook_order_info_EInvalidExpireTimestamp">EInvalidExpireTimestamp</a>: u64 = 3;
</code></pre>



<a name="orderbook_order_info_EInvalidOrderType"></a>



<pre><code><b>const</b> <a href="../orderbook/order_info.md#orderbook_order_info_EInvalidOrderType">EInvalidOrderType</a>: u64 = 4;
</code></pre>



<a name="orderbook_order_info_EPOSTOrderCrossesOrderbook"></a>



<pre><code><b>const</b> <a href="../orderbook/order_info.md#orderbook_order_info_EPOSTOrderCrossesOrderbook">EPOSTOrderCrossesOrderbook</a>: u64 = 5;
</code></pre>



<a name="orderbook_order_info_EFOKOrderCannotBeFullyFilled"></a>



<pre><code><b>const</b> <a href="../orderbook/order_info.md#orderbook_order_info_EFOKOrderCannotBeFullyFilled">EFOKOrderCannotBeFullyFilled</a>: u64 = 6;
</code></pre>



<a name="orderbook_order_info_EMarketOrderCannotBePostOnly"></a>



<pre><code><b>const</b> <a href="../orderbook/order_info.md#orderbook_order_info_EMarketOrderCannotBePostOnly">EMarketOrderCannotBePostOnly</a>: u64 = 7;
</code></pre>



<a name="orderbook_order_info_ESelfMatchingCancelTaker"></a>



<pre><code><b>const</b> <a href="../orderbook/order_info.md#orderbook_order_info_ESelfMatchingCancelTaker">ESelfMatchingCancelTaker</a>: u64 = 8;
</code></pre>



<a name="orderbook_order_info_pool_id"></a>

## Function `pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): ID {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_order_id"></a>

## Function `order_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u128 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_balance_manager_id"></a>

## Function `balance_manager_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): ID {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_client_order_id"></a>

## Function `client_order_id`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u64 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_trader"></a>

## Function `trader`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): <b>address</b> {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_order_type"></a>

## Function `order_type`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u8 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_self_matching_option"></a>

## Function `self_matching_option`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_self_matching_option">self_matching_option</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_self_matching_option">self_matching_option</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u8 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_self_matching_option">self_matching_option</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_price"></a>

## Function `price`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u64 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_is_bid"></a>

## Function `is_bid`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): bool {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_original_quantity"></a>

## Function `original_quantity`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u64 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_order_myso_price"></a>

## Function `order_myso_price`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): OrderMySoPrice {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_expire_timestamp"></a>

## Function `expire_timestamp`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u64 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_executed_quantity"></a>

## Function `executed_quantity`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u64 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_cumulative_quote_quantity"></a>

## Function `cumulative_quote_quantity`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_cumulative_quote_quantity">cumulative_quote_quantity</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_cumulative_quote_quantity">cumulative_quote_quantity</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u64 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_cumulative_quote_quantity">cumulative_quote_quantity</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_fills"></a>

## Function `fills`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): vector&lt;<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): vector&lt;Fill&gt; {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_fee_is_myso"></a>

## Function `fee_is_myso`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_fee_is_myso">fee_is_myso</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_fee_is_myso">fee_is_myso</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): bool {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_fee_is_myso">fee_is_myso</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_paid_fees"></a>

## Function `paid_fees`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u64 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_maker_fees"></a>

## Function `maker_fees`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_maker_fees">maker_fees</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_maker_fees">maker_fees</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u64 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_maker_fees">maker_fees</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_epoch"></a>

## Function `epoch`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_epoch">epoch</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_epoch">epoch</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u64 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_epoch">epoch</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_status"></a>

## Function `status`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_status">status</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_status">status</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u8 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_status">status</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_fill_limit_reached"></a>

## Function `fill_limit_reached`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_fill_limit_reached">fill_limit_reached</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_fill_limit_reached">fill_limit_reached</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): bool {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_fill_limit_reached">fill_limit_reached</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_order_inserted"></a>

## Function `order_inserted`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_inserted">order_inserted</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_inserted">order_inserted</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): bool {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_order_inserted">order_inserted</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_new">new</a>(<a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>: u64, <a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>: <b>address</b>, <a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a>: u8, <a href="../orderbook/order_info.md#orderbook_order_info_self_matching_option">self_matching_option</a>: u8, <a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>: u64, quantity: u64, <a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>: bool, <a href="../orderbook/order_info.md#orderbook_order_info_fee_is_myso">fee_is_myso</a>: bool, <a href="../orderbook/order_info.md#orderbook_order_info_epoch">epoch</a>: u64, <a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>: u64, <a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>, <a href="../orderbook/order_info.md#orderbook_order_info_market_order">market_order</a>: bool, timestamp: u64): <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_new">new</a>(
    <a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: ID,
    <a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>: ID,
    <a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>: u64,
    <a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>: <b>address</b>,
    <a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a>: u8,
    <a href="../orderbook/order_info.md#orderbook_order_info_self_matching_option">self_matching_option</a>: u8,
    <a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>: u64,
    quantity: u64,
    <a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>: bool,
    <a href="../orderbook/order_info.md#orderbook_order_info_fee_is_myso">fee_is_myso</a>: bool,
    <a href="../orderbook/order_info.md#orderbook_order_info_epoch">epoch</a>: u64,
    <a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>: u64,
    <a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>: OrderMySoPrice,
    <a href="../orderbook/order_info.md#orderbook_order_info_market_order">market_order</a>: bool,
    timestamp: u64,
): <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a> {
    <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a> {
        <a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: 0,
        <a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_self_matching_option">self_matching_option</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>: quantity,
        <a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a>: 0,
        <a href="../orderbook/order_info.md#orderbook_order_info_cumulative_quote_quantity">cumulative_quote_quantity</a>: 0,
        <a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>: vector[],
        <a href="../orderbook/order_info.md#orderbook_order_info_fee_is_myso">fee_is_myso</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_epoch">epoch</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>: 0,
        <a href="../orderbook/order_info.md#orderbook_order_info_maker_fees">maker_fees</a>: 0,
        <a href="../orderbook/order_info.md#orderbook_order_info_status">status</a>: <a href="../orderbook/constants.md#orderbook_constants_live">constants::live</a>(),
        <a href="../orderbook/order_info.md#orderbook_order_info_market_order">market_order</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_fill_limit_reached">fill_limit_reached</a>: <b>false</b>,
        <a href="../orderbook/order_info.md#orderbook_order_info_order_inserted">order_inserted</a>: <b>false</b>,
        timestamp,
    }
}
</code></pre>



</details>

<a name="orderbook_order_info_market_order"></a>

## Function `market_order`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_market_order">market_order</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_market_order">market_order</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): bool {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_market_order">market_order</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_set_order_id"></a>

## Function `set_order_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_set_order_id">set_order_id</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_set_order_id">set_order_id</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>, <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: u128) {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a> = <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>;
}
</code></pre>



</details>

<a name="orderbook_order_info_set_paid_fees"></a>

## Function `set_paid_fees`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_set_paid_fees">set_paid_fees</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, <a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_set_paid_fees">set_paid_fees</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>, <a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>: u64) {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a> = <a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>;
}
</code></pre>



</details>

<a name="orderbook_order_info_add_fill"></a>

## Function `add_fill`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_add_fill">add_fill</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, <a href="../orderbook/fill.md#orderbook_fill">fill</a>: <a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_add_fill">add_fill</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>, <a href="../orderbook/fill.md#orderbook_fill">fill</a>: Fill) {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>.push_back(<a href="../orderbook/fill.md#orderbook_fill">fill</a>);
}
</code></pre>



</details>

<a name="orderbook_order_info_fills_ref"></a>

## Function `fills_ref`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_fills_ref">fills_ref</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): &<b>mut</b> vector&lt;<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_fills_ref">fills_ref</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): &<b>mut</b> vector&lt;Fill&gt; {
    &<b>mut</b> self.<a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_paid_fees_balances"></a>

## Function `paid_fees_balances`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_paid_fees_balances">paid_fees_balances</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_paid_fees_balances">paid_fees_balances</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): Balances {
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_fee_is_myso">fee_is_myso</a>) {
        <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(0, 0, self.<a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>)
    } <b>else</b> <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>) {
        <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(0, self.<a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>, 0)
    } <b>else</b> {
        <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(self.<a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a>, 0, 0)
    }
}
</code></pre>



</details>

<a name="orderbook_order_info_calculate_partial_fill_balances"></a>

## Function `calculate_partial_fill_balances`

Given a partially filled <code><a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a></code>, the taker fee and maker fee, for the user
placing the order, calculate all of the balances that need to be settled and
the balances that are owed. The executed quantity is multiplied by the taker_fee
and the remaining quantity is multiplied by the maker_fee to get the MYSO fee.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_calculate_partial_fill_balances">calculate_partial_fill_balances</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, taker_fee: u64, maker_fee: u64): (<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_calculate_partial_fill_balances">calculate_partial_fill_balances</a>(
    self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>,
    taker_fee: u64,
    maker_fee: u64,
): (Balances, Balances) {
    <b>let</b> <b>mut</b> taker_fee_quantity = self
        .<a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>
        .fee_quantity(
            self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a>,
            self.<a href="../orderbook/order_info.md#orderbook_order_info_cumulative_quote_quantity">cumulative_quote_quantity</a>,
            self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>,
        );
    taker_fee_quantity.mul(taker_fee);
    self.<a href="../orderbook/order_info.md#orderbook_order_info_paid_fees">paid_fees</a> = taker_fee_quantity.non_zero_value();
    <b>let</b> <a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a> = &<b>mut</b> self.<a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> num_fills = <a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>.length();
    <b>while</b> (i &lt; num_fills) {
        <b>let</b> <a href="../orderbook/fill.md#orderbook_fill">fill</a> = &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>[i];
        <b>if</b> (!<a href="../orderbook/fill.md#orderbook_fill">fill</a>.expired()) {
            <b>let</b> base_quantity = <a href="../orderbook/fill.md#orderbook_fill">fill</a>.base_quantity();
            <b>let</b> quote_quantity = <a href="../orderbook/fill.md#orderbook_fill">fill</a>.quote_quantity();
            <b>let</b> <b>mut</b> fill_taker_fee_quantity = self
                .<a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>
                .fee_quantity(
                    base_quantity,
                    quote_quantity,
                    self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>,
                );
            fill_taker_fee_quantity.mul(taker_fee);
            <a href="../orderbook/fill.md#orderbook_fill">fill</a>.set_fill_taker_fee(&fill_taker_fee_quantity);
        };
        i = i + 1;
    };
    <b>let</b> <b>mut</b> settled_balances = <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(0, 0, 0);
    <b>let</b> <b>mut</b> owed_balances = <a href="../orderbook/balances.md#orderbook_balances_new">balances::new</a>(0, 0, 0);
    owed_balances.add_balances(taker_fee_quantity);
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>) {
        settled_balances.add_base(self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a>);
        owed_balances.add_quote(self.<a href="../orderbook/order_info.md#orderbook_order_info_cumulative_quote_quantity">cumulative_quote_quantity</a>);
    } <b>else</b> {
        settled_balances.add_quote(self.<a href="../orderbook/order_info.md#orderbook_order_info_cumulative_quote_quantity">cumulative_quote_quantity</a>);
        owed_balances.add_base(self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a>);
    };
    <b>let</b> <a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a> = self.<a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>();
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_order_inserted">order_inserted</a>()) {
        <b>let</b> <b>mut</b> maker_fee_quantity = self
            .<a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>
            .fee_quantity(
                <a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>,
                <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(<a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>, self.<a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>()),
                self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>,
            );
        maker_fee_quantity.mul(maker_fee);
        self.<a href="../orderbook/order_info.md#orderbook_order_info_maker_fees">maker_fees</a> = maker_fee_quantity.non_zero_value();
        owed_balances.add_balances(maker_fee_quantity);
        <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>) {
            owed_balances.add_quote(
                <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(<a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>, self.<a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>()),
            );
        } <b>else</b> {
            owed_balances.add_base(<a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>);
        };
    };
    (settled_balances, owed_balances)
}
</code></pre>



</details>

<a name="orderbook_order_info_to_order"></a>

## Function `to_order`

<code><a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a></code> is converted to an <code>Order</code> before being injected into the order book.
This is done to save space in the order book. Order contains the minimum
information required to match orders.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_to_order">to_order</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_to_order">to_order</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): Order {
    <a href="../orderbook/order.md#orderbook_order_new">order::new</a>(
        self.<a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a>,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_fee_is_myso">fee_is_myso</a>,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_order_myso_price">order_myso_price</a>,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_epoch">epoch</a>,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_status">status</a>,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>,
    )
}
</code></pre>



</details>

<a name="orderbook_order_info_validate_inputs"></a>

## Function `validate_inputs`

Validates that the initial order created meets the pool requirements.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_validate_inputs">validate_inputs</a>(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, tick_size: u64, min_size: u64, lot_size: u64, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_validate_inputs">validate_inputs</a>(
    <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>,
    tick_size: u64,
    min_size: u64,
    lot_size: u64,
    timestamp: u64,
) {
    <b>assert</b>!(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.<a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a> &gt;= min_size, <a href="../orderbook/order_info.md#orderbook_order_info_EOrderBelowMinimumSize">EOrderBelowMinimumSize</a>);
    <b>assert</b>!(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.<a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a> % lot_size == 0, <a href="../orderbook/order_info.md#orderbook_order_info_EOrderInvalidLotSize">EOrderInvalidLotSize</a>);
    <b>assert</b>!(timestamp &lt;= <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.<a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>, <a href="../orderbook/order_info.md#orderbook_order_info_EInvalidExpireTimestamp">EInvalidExpireTimestamp</a>);
    <b>assert</b>!(
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.<a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a> &gt;= <a href="../orderbook/constants.md#orderbook_constants_no_restriction">constants::no_restriction</a>() &&
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.<a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a> &lt;= <a href="../orderbook/constants.md#orderbook_constants_max_restriction">constants::max_restriction</a>(),
        <a href="../orderbook/order_info.md#orderbook_order_info_EInvalidOrderType">EInvalidOrderType</a>,
    );
    <b>if</b> (<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.<a href="../orderbook/order_info.md#orderbook_order_info_market_order">market_order</a>) {
        <b>assert</b>!(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.<a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a> != <a href="../orderbook/constants.md#orderbook_constants_post_only">constants::post_only</a>(), <a href="../orderbook/order_info.md#orderbook_order_info_EMarketOrderCannotBePostOnly">EMarketOrderCannotBePostOnly</a>);
        <b>return</b>
    };
    <b>assert</b>!(
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.<a href="../orderbook/order_info.md#orderbook_order_info_price">price</a> &gt;= <a href="../orderbook/constants.md#orderbook_constants_min_price">constants::min_price</a>() &&
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.<a href="../orderbook/order_info.md#orderbook_order_info_price">price</a> &lt;= <a href="../orderbook/constants.md#orderbook_constants_max_price">constants::max_price</a>(),
        <a href="../orderbook/order_info.md#orderbook_order_info_EOrderInvalidPrice">EOrderInvalidPrice</a>,
    );
    <b>assert</b>!(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.<a href="../orderbook/order_info.md#orderbook_order_info_price">price</a> % tick_size == 0, <a href="../orderbook/order_info.md#orderbook_order_info_EOrderInvalidPrice">EOrderInvalidPrice</a>);
}
</code></pre>



</details>

<a name="orderbook_order_info_assert_execution"></a>

## Function `assert_execution`

Assert order types after partial fill against the order book.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_assert_execution">assert_execution</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_assert_execution">assert_execution</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): bool {
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a> == <a href="../orderbook/constants.md#orderbook_constants_post_only">constants::post_only</a>()) {
        <b>assert</b>!(self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a> == 0, <a href="../orderbook/order_info.md#orderbook_order_info_EPOSTOrderCrossesOrderbook">EPOSTOrderCrossesOrderbook</a>)
    };
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a> == <a href="../orderbook/constants.md#orderbook_constants_fill_or_kill">constants::fill_or_kill</a>()) {
        <b>assert</b>!(self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a> == self.<a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>, <a href="../orderbook/order_info.md#orderbook_order_info_EFOKOrderCannotBeFullyFilled">EFOKOrderCannotBeFullyFilled</a>)
    };
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_order_type">order_type</a> == <a href="../orderbook/constants.md#orderbook_constants_immediate_or_cancel">constants::immediate_or_cancel</a>()) {
        <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>() &gt; 0) {
            self.<a href="../orderbook/order_info.md#orderbook_order_info_status">status</a> = <a href="../orderbook/constants.md#orderbook_constants_canceled">constants::canceled</a>();
        } <b>else</b> {
            self.<a href="../orderbook/order_info.md#orderbook_order_info_status">status</a> = <a href="../orderbook/constants.md#orderbook_constants_filled">constants::filled</a>();
        };
        <b>return</b> <b>true</b>
    };
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>() == 0) {
        self.<a href="../orderbook/order_info.md#orderbook_order_info_status">status</a> = <a href="../orderbook/constants.md#orderbook_constants_filled">constants::filled</a>();
        <b>return</b> <b>true</b>
    };
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_fill_limit_reached">fill_limit_reached</a>) {
        <b>return</b> <b>true</b>
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="orderbook_order_info_remaining_quantity"></a>

## Function `remaining_quantity`

Returns the remaining quantity for the order.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>): u64 {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a> - self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a>
}
</code></pre>



</details>

<a name="orderbook_order_info_can_match"></a>

## Function `can_match`

Returns true if two opposite orders are overlapping in price.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_can_match">can_match</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, <a href="../orderbook/order.md#orderbook_order">order</a>: &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_can_match">can_match</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>, <a href="../orderbook/order.md#orderbook_order">order</a>: &Order): bool {
    <b>let</b> maker_price = <a href="../orderbook/order.md#orderbook_order">order</a>.<a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>();
    (
        self.<a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a> - self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a> &gt; 0 && (
            self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a> && self.<a href="../orderbook/order_info.md#orderbook_order_info_price">price</a> &gt;= maker_price ||
            !self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a> && self.<a href="../orderbook/order_info.md#orderbook_order_info_price">price</a> &lt;= maker_price,
        ),
    )
}
</code></pre>



</details>

<a name="orderbook_order_info_match_maker"></a>

## Function `match_maker`

Matches an <code><a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a></code> with an <code>Order</code> from the book. Appends a <code>Fill</code> to fills.
If the book order is expired, the <code>Fill</code> will have the expired flag set to true.
Funds for the match or an expired order are returned to the maker as settled.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_match_maker">match_maker</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, maker: &<b>mut</b> <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>, timestamp: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_match_maker">match_maker</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>, maker: &<b>mut</b> Order, timestamp: u64): bool {
    <b>if</b> (!self.<a href="../orderbook/order_info.md#orderbook_order_info_can_match">can_match</a>(maker)) <b>return</b> <b>false</b>;
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_self_matching_option">self_matching_option</a>() == <a href="../orderbook/constants.md#orderbook_constants_cancel_taker">constants::cancel_taker</a>()) {
        <b>assert</b>!(maker.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>() != self.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>(), <a href="../orderbook/order_info.md#orderbook_order_info_ESelfMatchingCancelTaker">ESelfMatchingCancelTaker</a>);
    };
    <b>let</b> expire_maker =
        self.<a href="../orderbook/order_info.md#orderbook_order_info_self_matching_option">self_matching_option</a>() == <a href="../orderbook/constants.md#orderbook_constants_cancel_maker">constants::cancel_maker</a>() &&
        maker.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>() == self.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>();
    <b>let</b> <a href="../orderbook/fill.md#orderbook_fill">fill</a> = maker.generate_fill(
        timestamp,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>(),
        self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>,
        expire_maker,
        self.<a href="../orderbook/order_info.md#orderbook_order_info_fee_is_myso">fee_is_myso</a>,
    );
    self.<a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>.push_back(<a href="../orderbook/fill.md#orderbook_fill">fill</a>);
    <b>if</b> (<a href="../orderbook/fill.md#orderbook_fill">fill</a>.expired()) <b>return</b> <b>true</b>;
    self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a> = self.<a href="../orderbook/order_info.md#orderbook_order_info_executed_quantity">executed_quantity</a> + <a href="../orderbook/fill.md#orderbook_fill">fill</a>.base_quantity();
    self.<a href="../orderbook/order_info.md#orderbook_order_info_cumulative_quote_quantity">cumulative_quote_quantity</a> = self.<a href="../orderbook/order_info.md#orderbook_order_info_cumulative_quote_quantity">cumulative_quote_quantity</a> + <a href="../orderbook/fill.md#orderbook_fill">fill</a>.quote_quantity();
    self.<a href="../orderbook/order_info.md#orderbook_order_info_status">status</a> = <a href="../orderbook/constants.md#orderbook_constants_partially_filled">constants::partially_filled</a>();
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>() == 0) self.<a href="../orderbook/order_info.md#orderbook_order_info_status">status</a> = <a href="../orderbook/constants.md#orderbook_constants_filled">constants::filled</a>();
    <b>true</b>
}
</code></pre>



</details>

<a name="orderbook_order_info_emit_orders_filled"></a>

## Function `emit_orders_filled`

Emit all fills for this order in a vector of <code><a href="../orderbook/order_info.md#orderbook_order_info_OrderFilled">OrderFilled</a></code> events.
To avoid DOS attacks, 100 fills are emitted at a time. Up to 10,000
fills can be emitted in a single call.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_orders_filled">emit_orders_filled</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_orders_filled">emit_orders_filled</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>, timestamp: u64) {
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> num_fills = self.<a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>.length();
    <b>while</b> (i &lt; num_fills) {
        <b>let</b> <a href="../orderbook/fill.md#orderbook_fill">fill</a> = &self.<a href="../orderbook/order_info.md#orderbook_order_info_fills">fills</a>[i];
        <b>if</b> (<a href="../orderbook/fill.md#orderbook_fill">fill</a>.completed()) {
            self.<a href="../orderbook/order_info.md#orderbook_order_info_emit_order_fully_filled">emit_order_fully_filled</a>(
                <a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_order_id(),
                <a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_client_order_id(),
                <a href="../orderbook/fill.md#orderbook_fill">fill</a>.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>(),
                <a href="../orderbook/fill.md#orderbook_fill">fill</a>.original_maker_quantity(),
                !<a href="../orderbook/fill.md#orderbook_fill">fill</a>.taker_is_bid(),
                timestamp,
            );
        };
        <b>if</b> (!<a href="../orderbook/fill.md#orderbook_fill">fill</a>.expired()) {
            event::emit(self.<a href="../orderbook/order_info.md#orderbook_order_info_order_filled_from_fill">order_filled_from_fill</a>(<a href="../orderbook/fill.md#orderbook_fill">fill</a>, timestamp));
        } <b>else</b> {
            <b>let</b> cancel_maker = self.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>() == <a href="../orderbook/fill.md#orderbook_fill">fill</a>.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>();
            <b>if</b> (cancel_maker) {
                self.<a href="../orderbook/order_info.md#orderbook_order_info_emit_order_canceled_maker_from_fill">emit_order_canceled_maker_from_fill</a>(<a href="../orderbook/fill.md#orderbook_fill">fill</a>, timestamp);
            } <b>else</b> {
                event::emit(self.<a href="../orderbook/order_info.md#orderbook_order_info_order_expired_from_fill">order_expired_from_fill</a>(<a href="../orderbook/fill.md#orderbook_fill">fill</a>, timestamp));
            };
        };
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="orderbook_order_info_emit_order_placed"></a>

## Function `emit_order_placed`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_order_placed">emit_order_placed</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_order_placed">emit_order_placed</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>) {
    event::emit(<a href="../orderbook/order_info.md#orderbook_order_info_OrderPlaced">OrderPlaced</a> {
        <a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>,
        placed_quantity: self.<a href="../orderbook/order_info.md#orderbook_order_info_remaining_quantity">remaining_quantity</a>(),
        <a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_expire_timestamp">expire_timestamp</a>,
        timestamp: self.timestamp,
    });
}
</code></pre>



</details>

<a name="orderbook_order_info_emit_order_info"></a>

## Function `emit_order_info`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_order_info">emit_order_info</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_order_info">emit_order_info</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>) {
    event::emit(*self);
}
</code></pre>



</details>

<a name="orderbook_order_info_emit_order_fully_filled_if_filled"></a>

## Function `emit_order_fully_filled_if_filled`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_order_fully_filled_if_filled">emit_order_fully_filled_if_filled</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_order_fully_filled_if_filled">emit_order_fully_filled_if_filled</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>, timestamp: u64) {
    <b>if</b> (self.<a href="../orderbook/order_info.md#orderbook_order_info_status">status</a> == <a href="../orderbook/constants.md#orderbook_constants_filled">constants::filled</a>()) {
        self.<a href="../orderbook/order_info.md#orderbook_order_info_emit_order_fully_filled">emit_order_fully_filled</a>(
            self.<a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>,
            self.<a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>,
            self.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>,
            self.<a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>,
            self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>,
            timestamp,
        );
    }
}
</code></pre>



</details>

<a name="orderbook_order_info_emit_order_fully_filled"></a>

## Function `emit_order_fully_filled`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_order_fully_filled">emit_order_fully_filled</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: u128, <a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>: u64, <a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>: u64, <a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>: bool, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_order_fully_filled">emit_order_fully_filled</a>(
    self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>,
    <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: u128,
    <a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>: u64,
    <a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>: ID,
    <a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>: u64,
    <a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>: bool,
    timestamp: u64,
) {
    event::emit(<a href="../orderbook/order_info.md#orderbook_order_info_OrderFullyFilled">OrderFullyFilled</a> {
        <a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>,
        timestamp,
    })
}
</code></pre>



</details>

<a name="orderbook_order_info_set_fill_limit_reached"></a>

## Function `set_fill_limit_reached`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_set_fill_limit_reached">set_fill_limit_reached</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_set_fill_limit_reached">set_fill_limit_reached</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>) {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_fill_limit_reached">fill_limit_reached</a> = <b>true</b>;
}
</code></pre>



</details>

<a name="orderbook_order_info_set_order_inserted"></a>

## Function `set_order_inserted`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_set_order_inserted">set_order_inserted</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_set_order_inserted">set_order_inserted</a>(self: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>) {
    self.<a href="../orderbook/order_info.md#orderbook_order_info_order_inserted">order_inserted</a> = <b>true</b>;
}
</code></pre>



</details>

<a name="orderbook_order_info_order_filled_from_fill"></a>

## Function `order_filled_from_fill`



<pre><code><b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_filled_from_fill">order_filled_from_fill</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, <a href="../orderbook/fill.md#orderbook_fill">fill</a>: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>, timestamp: u64): <a href="../orderbook/order_info.md#orderbook_order_info_OrderFilled">orderbook::order_info::OrderFilled</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_filled_from_fill">order_filled_from_fill</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>, <a href="../orderbook/fill.md#orderbook_fill">fill</a>: &Fill, timestamp: u64): <a href="../orderbook/order_info.md#orderbook_order_info_OrderFilled">OrderFilled</a> {
    <a href="../orderbook/order_info.md#orderbook_order_info_OrderFilled">OrderFilled</a> {
        <a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>,
        maker_order_id: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_order_id(),
        taker_order_id: self.<a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>,
        maker_client_order_id: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_client_order_id(),
        taker_client_order_id: self.<a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.execution_price(),
        taker_is_bid: self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>,
        taker_fee: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.taker_fee(),
        taker_fee_is_myso: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.taker_fee_is_myso(),
        maker_fee: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_fee(),
        maker_fee_is_myso: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_fee_is_myso(),
        base_quantity: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.base_quantity(),
        quote_quantity: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.quote_quantity(),
        maker_balance_manager_id: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>(),
        taker_balance_manager_id: self.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>,
        timestamp,
    }
}
</code></pre>



</details>

<a name="orderbook_order_info_order_expired_from_fill"></a>

## Function `order_expired_from_fill`



<pre><code><b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_expired_from_fill">order_expired_from_fill</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, <a href="../orderbook/fill.md#orderbook_fill">fill</a>: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>, timestamp: u64): <a href="../orderbook/order_info.md#orderbook_order_info_OrderExpired">orderbook::order_info::OrderExpired</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_order_expired_from_fill">order_expired_from_fill</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>, <a href="../orderbook/fill.md#orderbook_fill">fill</a>: &Fill, timestamp: u64): <a href="../orderbook/order_info.md#orderbook_order_info_OrderExpired">OrderExpired</a> {
    <a href="../orderbook/order_info.md#orderbook_order_info_OrderExpired">OrderExpired</a> {
        <a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>(),
        <a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>,
        <a href="../orderbook/order_info.md#orderbook_order_info_order_id">order_id</a>: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_order_id(),
        <a href="../orderbook/order_info.md#orderbook_order_info_client_order_id">client_order_id</a>: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_client_order_id(),
        <a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>: self.<a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>(),
        <a href="../orderbook/order_info.md#orderbook_order_info_price">price</a>: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.execution_price(),
        <a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>: !self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>(),
        <a href="../orderbook/order_info.md#orderbook_order_info_original_quantity">original_quantity</a>: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.original_maker_quantity(),
        base_asset_quantity_canceled: <a href="../orderbook/fill.md#orderbook_fill">fill</a>.base_quantity(),
        timestamp,
    }
}
</code></pre>



</details>

<a name="orderbook_order_info_emit_order_canceled_maker_from_fill"></a>

## Function `emit_order_canceled_maker_from_fill`



<pre><code><b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_order_canceled_maker_from_fill">emit_order_canceled_maker_from_fill</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, <a href="../orderbook/fill.md#orderbook_fill">fill</a>: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/order_info.md#orderbook_order_info_emit_order_canceled_maker_from_fill">emit_order_canceled_maker_from_fill</a>(self: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">OrderInfo</a>, <a href="../orderbook/fill.md#orderbook_fill">fill</a>: &Fill, timestamp: u64) {
    <a href="../orderbook/order.md#orderbook_order_emit_cancel_maker">order::emit_cancel_maker</a>(
        <a href="../orderbook/fill.md#orderbook_fill">fill</a>.<a href="../orderbook/order_info.md#orderbook_order_info_balance_manager_id">balance_manager_id</a>(),
        self.<a href="../orderbook/order_info.md#orderbook_order_info_pool_id">pool_id</a>,
        <a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_order_id(),
        <a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_client_order_id(),
        self.<a href="../orderbook/order_info.md#orderbook_order_info_trader">trader</a>(),
        <a href="../orderbook/fill.md#orderbook_fill">fill</a>.execution_price(),
        !self.<a href="../orderbook/order_info.md#orderbook_order_info_is_bid">is_bid</a>(),
        <a href="../orderbook/fill.md#orderbook_fill">fill</a>.original_maker_quantity(),
        <a href="../orderbook/fill.md#orderbook_fill">fill</a>.base_quantity(),
        timestamp,
    )
}
</code></pre>



</details>
