---
title: Module `orderbook::book`
---

The book module contains the <code><a href="../orderbook/book.md#orderbook_book_Book">Book</a></code> struct which represents the order book.
All order book operations are defined in this module.


-  [Struct `Book`](#orderbook_book_Book)
-  [Constants](#@Constants_0)
-  [Function `bids`](#orderbook_book_bids)
-  [Function `asks`](#orderbook_book_asks)
-  [Function `tick_size`](#orderbook_book_tick_size)
-  [Function `lot_size`](#orderbook_book_lot_size)
-  [Function `min_size`](#orderbook_book_min_size)
-  [Function `empty`](#orderbook_book_empty)
-  [Function `create_order`](#orderbook_book_create_order)
-  [Function `get_quantity_out`](#orderbook_book_get_quantity_out)
-  [Function `get_base_quantity_in`](#orderbook_book_get_base_quantity_in)
-  [Function `get_quote_quantity_in`](#orderbook_book_get_quote_quantity_in)
-  [Function `cancel_order`](#orderbook_book_cancel_order)
-  [Function `modify_order`](#orderbook_book_modify_order)
-  [Function `mid_price`](#orderbook_book_mid_price)
-  [Function `get_level2_range_and_ticks`](#orderbook_book_get_level2_range_and_ticks)
-  [Function `check_limit_order_params`](#orderbook_book_check_limit_order_params)
-  [Function `check_market_order_params`](#orderbook_book_check_market_order_params)
-  [Function `get_order`](#orderbook_book_get_order)
-  [Function `set_tick_size`](#orderbook_book_set_tick_size)
-  [Function `set_lot_size`](#orderbook_book_set_lot_size)
-  [Function `set_min_size`](#orderbook_book_set_min_size)
-  [Function `book_side_mut`](#orderbook_book_book_side_mut)
-  [Function `book_side`](#orderbook_book_book_side)
-  [Function `match_against_book`](#orderbook_book_match_against_book)
-  [Function `get_order_id`](#orderbook_book_get_order_id)
-  [Function `inject_limit_order`](#orderbook_book_inject_limit_order)
-  [Function `round_up_to_lot_size`](#orderbook_book_round_up_to_lot_size)
-  [Function `get_quantity_in`](#orderbook_book_get_quantity_in)


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
<b>use</b> <a href="../orderbook/big_vector.md#orderbook_big_vector">orderbook::big_vector</a>;
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/fill.md#orderbook_fill">orderbook::fill</a>;
<b>use</b> <a href="../orderbook/math.md#orderbook_math">orderbook::math</a>;
<b>use</b> <a href="../orderbook/myso_price.md#orderbook_myso_price">orderbook::myso_price</a>;
<b>use</b> <a href="../orderbook/order.md#orderbook_order">orderbook::order</a>;
<b>use</b> <a href="../orderbook/order_info.md#orderbook_order_info">orderbook::order_info</a>;
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



<a name="orderbook_book_Book"></a>

## Struct `Book`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/book.md#orderbook_book_tick_size">tick_size</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/book.md#orderbook_book_bids">bids</a>: <a href="../orderbook/big_vector.md#orderbook_big_vector_BigVector">orderbook::big_vector::BigVector</a>&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/book.md#orderbook_book_asks">asks</a>: <a href="../orderbook/big_vector.md#orderbook_big_vector_BigVector">orderbook::big_vector::BigVector</a>&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>next_bid_order_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>next_ask_order_id: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="orderbook_book_EInvalidAmountIn"></a>



<pre><code><b>const</b> <a href="../orderbook/book.md#orderbook_book_EInvalidAmountIn">EInvalidAmountIn</a>: u64 = 1;
</code></pre>



<a name="orderbook_book_EEmptyOrderbook"></a>



<pre><code><b>const</b> <a href="../orderbook/book.md#orderbook_book_EEmptyOrderbook">EEmptyOrderbook</a>: u64 = 2;
</code></pre>



<a name="orderbook_book_EInvalidPriceRange"></a>



<pre><code><b>const</b> <a href="../orderbook/book.md#orderbook_book_EInvalidPriceRange">EInvalidPriceRange</a>: u64 = 3;
</code></pre>



<a name="orderbook_book_EInvalidTicks"></a>



<pre><code><b>const</b> <a href="../orderbook/book.md#orderbook_book_EInvalidTicks">EInvalidTicks</a>: u64 = 4;
</code></pre>



<a name="orderbook_book_EOrderBelowMinimumSize"></a>



<pre><code><b>const</b> <a href="../orderbook/book.md#orderbook_book_EOrderBelowMinimumSize">EOrderBelowMinimumSize</a>: u64 = 5;
</code></pre>



<a name="orderbook_book_EOrderInvalidLotSize"></a>



<pre><code><b>const</b> <a href="../orderbook/book.md#orderbook_book_EOrderInvalidLotSize">EOrderInvalidLotSize</a>: u64 = 6;
</code></pre>



<a name="orderbook_book_ENewQuantityMustBeLessThanOriginal"></a>



<pre><code><b>const</b> <a href="../orderbook/book.md#orderbook_book_ENewQuantityMustBeLessThanOriginal">ENewQuantityMustBeLessThanOriginal</a>: u64 = 7;
</code></pre>



<a name="orderbook_book_START_BID_ORDER_ID"></a>



<pre><code><b>const</b> <a href="../orderbook/book.md#orderbook_book_START_BID_ORDER_ID">START_BID_ORDER_ID</a>: u64 = 18446744073709551615;
</code></pre>



<a name="orderbook_book_START_ASK_ORDER_ID"></a>



<pre><code><b>const</b> <a href="../orderbook/book.md#orderbook_book_START_ASK_ORDER_ID">START_ASK_ORDER_ID</a>: u64 = 1;
</code></pre>



<a name="orderbook_book_bids"></a>

## Function `bids`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_bids">bids</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>): &<a href="../orderbook/big_vector.md#orderbook_big_vector_BigVector">orderbook::big_vector::BigVector</a>&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_bids">bids</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>): &BigVector&lt;Order&gt; {
    &self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a>
}
</code></pre>



</details>

<a name="orderbook_book_asks"></a>

## Function `asks`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_asks">asks</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>): &<a href="../orderbook/big_vector.md#orderbook_big_vector_BigVector">orderbook::big_vector::BigVector</a>&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_asks">asks</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>): &BigVector&lt;Order&gt; {
    &self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a>
}
</code></pre>



</details>

<a name="orderbook_book_tick_size"></a>

## Function `tick_size`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_tick_size">tick_size</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_tick_size">tick_size</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>): u64 {
    self.<a href="../orderbook/book.md#orderbook_book_tick_size">tick_size</a>
}
</code></pre>



</details>

<a name="orderbook_book_lot_size"></a>

## Function `lot_size`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>): u64 {
    self.<a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>
}
</code></pre>



</details>

<a name="orderbook_book_min_size"></a>

## Function `min_size`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>): u64 {
    self.<a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>
}
</code></pre>



</details>

<a name="orderbook_book_empty"></a>

## Function `empty`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_empty">empty</a>(<a href="../orderbook/book.md#orderbook_book_tick_size">tick_size</a>: u64, <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>: u64, <a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_empty">empty</a>(<a href="../orderbook/book.md#orderbook_book_tick_size">tick_size</a>: u64, <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>: u64, <a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>: u64, ctx: &<b>mut</b> TxContext): <a href="../orderbook/book.md#orderbook_book_Book">Book</a> {
    <a href="../orderbook/book.md#orderbook_book_Book">Book</a> {
        <a href="../orderbook/book.md#orderbook_book_tick_size">tick_size</a>,
        <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>,
        <a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>,
        <a href="../orderbook/book.md#orderbook_book_bids">bids</a>: <a href="../orderbook/big_vector.md#orderbook_big_vector_empty">big_vector::empty</a>(
            <a href="../orderbook/constants.md#orderbook_constants_max_slice_size">constants::max_slice_size</a>(),
            <a href="../orderbook/constants.md#orderbook_constants_max_fan_out">constants::max_fan_out</a>(),
            ctx,
        ),
        <a href="../orderbook/book.md#orderbook_book_asks">asks</a>: <a href="../orderbook/big_vector.md#orderbook_big_vector_empty">big_vector::empty</a>(
            <a href="../orderbook/constants.md#orderbook_constants_max_slice_size">constants::max_slice_size</a>(),
            <a href="../orderbook/constants.md#orderbook_constants_max_fan_out">constants::max_fan_out</a>(),
            ctx,
        ),
        next_bid_order_id: <a href="../orderbook/book.md#orderbook_book_START_BID_ORDER_ID">START_BID_ORDER_ID</a>,
        next_ask_order_id: <a href="../orderbook/book.md#orderbook_book_START_ASK_ORDER_ID">START_ASK_ORDER_ID</a>,
    }
}
</code></pre>



</details>

<a name="orderbook_book_create_order"></a>

## Function `create_order`

Creates a new order.
Order is matched against the book and injected into the book if necessary.
If order is IOC or fully executed, it will not be injected.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_create_order">create_order</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_create_order">create_order</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a>, <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &<b>mut</b> OrderInfo, timestamp: u64) {
    <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.validate_inputs(
        self.<a href="../orderbook/book.md#orderbook_book_tick_size">tick_size</a>,
        self.<a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>,
        self.<a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>,
        timestamp,
    );
    <b>let</b> order_id = <a href="../orderbook/utils.md#orderbook_utils_encode_order_id">utils::encode_order_id</a>(
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.is_bid(),
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.price(),
        self.<a href="../orderbook/book.md#orderbook_book_get_order_id">get_order_id</a>(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.is_bid()),
    );
    <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.set_order_id(order_id);
    self.<a href="../orderbook/book.md#orderbook_book_match_against_book">match_against_book</a>(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>, timestamp);
    <b>if</b> (<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.assert_execution()) <b>return</b>;
    self.<a href="../orderbook/book.md#orderbook_book_inject_limit_order">inject_limit_order</a>(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>);
    <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.set_order_inserted();
    <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.emit_order_placed();
}
</code></pre>



</details>

<a name="orderbook_book_get_quantity_out"></a>

## Function `get_quantity_out`

Given base_quantity and quote_quantity, calculate the base_quantity_out and
quote_quantity_out.
Will return (base_quantity_out, quote_quantity_out, deep_quantity_required)
if base_amount > 0 or quote_amount > 0.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_quantity_out">get_quantity_out</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, base_quantity: u64, quote_quantity: u64, taker_fee: u64, <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>, <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>: u64, pay_with_myso: bool, current_timestamp: u64): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_quantity_out">get_quantity_out</a>(
    self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>,
    base_quantity: u64,
    quote_quantity: u64,
    taker_fee: u64,
    <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>: OrderMySoPrice,
    <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>: u64,
    pay_with_myso: bool,
    current_timestamp: u64,
): (u64, u64, u64) {
    <b>assert</b>!((base_quantity &gt; 0) != (quote_quantity &gt; 0), <a href="../orderbook/book.md#orderbook_book_EInvalidAmountIn">EInvalidAmountIn</a>);
    <b>let</b> is_bid = quote_quantity &gt; 0;
    <b>let</b> input_fee_rate = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(
        <a href="../orderbook/constants.md#orderbook_constants_fee_penalty_multiplier">constants::fee_penalty_multiplier</a>(),
        taker_fee,
    );
    <b>if</b> (base_quantity &gt; 0) {
        <b>let</b> trading_base_quantity = <b>if</b> (pay_with_myso) {
            base_quantity
        } <b>else</b> {
            <a href="../orderbook/math.md#orderbook_math_div">math::div</a>(base_quantity, <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>() + input_fee_rate)
        };
        <b>if</b> (trading_base_quantity &lt; self.<a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>) {
            <b>return</b> (base_quantity, quote_quantity, 0)
        }
    };
    <b>let</b> <b>mut</b> quantity_out = 0;
    <b>let</b> <b>mut</b> quantity_in_left = <b>if</b> (is_bid) quote_quantity <b>else</b> base_quantity;
    <b>let</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a> = <b>if</b> (is_bid) &self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a> <b>else</b> &self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a>;
    <b>let</b> (<b>mut</b> ref, <b>mut</b> offset) = <b>if</b> (is_bid) <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.min_slice() <b>else</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.max_slice();
    <b>let</b> max_fills = <a href="../orderbook/constants.md#orderbook_constants_max_fills">constants::max_fills</a>();
    <b>let</b> <b>mut</b> current_fills = 0;
    <b>while</b> (!ref.is_null() && quantity_in_left &gt; 0 && current_fills &lt; max_fills) {
        <b>let</b> <a href="../orderbook/order.md#orderbook_order">order</a> = slice_borrow(<a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.borrow_slice(ref), offset);
        <b>let</b> cur_price = <a href="../orderbook/order.md#orderbook_order">order</a>.price();
        <b>let</b> cur_quantity = <a href="../orderbook/order.md#orderbook_order">order</a>.quantity() - <a href="../orderbook/order.md#orderbook_order">order</a>.filled_quantity();
        <b>if</b> (current_timestamp &lt;= <a href="../orderbook/order.md#orderbook_order">order</a>.expire_timestamp()) {
            <b>let</b> <b>mut</b> matched_base_quantity;
            <b>let</b> quantity_to_match = <b>if</b> (pay_with_myso) {
                quantity_in_left
            } <b>else</b> {
                <a href="../orderbook/math.md#orderbook_math_div">math::div</a>(
                    quantity_in_left,
                    <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>() + input_fee_rate,
                )
            };
            <b>if</b> (is_bid) {
                matched_base_quantity = <a href="../orderbook/math.md#orderbook_math_div">math::div</a>(quantity_to_match, cur_price).min(cur_quantity);
                matched_base_quantity =
                    matched_base_quantity -
                    matched_base_quantity % <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>;
                quantity_out = quantity_out + matched_base_quantity;
                <b>let</b> matched_quote_quantity = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(
                    matched_base_quantity,
                    cur_price,
                );
                quantity_in_left = quantity_in_left - matched_quote_quantity;
                <b>if</b> (!pay_with_myso) {
                    quantity_in_left =
                        quantity_in_left -
                        <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(matched_quote_quantity, input_fee_rate);
                };
            } <b>else</b> {
                matched_base_quantity = quantity_to_match.min(cur_quantity);
                matched_base_quantity =
                    matched_base_quantity -
                    matched_base_quantity % <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>;
                quantity_out = quantity_out + <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(matched_base_quantity, cur_price);
                quantity_in_left = quantity_in_left - matched_base_quantity;
                <b>if</b> (!pay_with_myso) {
                    quantity_in_left =
                        quantity_in_left -
                        <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(matched_base_quantity, input_fee_rate);
                };
            };
            <b>if</b> (matched_base_quantity == 0) <b>break</b>;
        };
        (ref, offset) = <b>if</b> (is_bid) <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.next_slice(ref, offset)
        <b>else</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.prev_slice(ref, offset);
        current_fills = current_fills + 1;
    };
    <b>let</b> myso_fee = <b>if</b> (!pay_with_myso) {
        0
    } <b>else</b> {
        <b>let</b> fee_quantity = <b>if</b> (is_bid) {
            <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.fee_quantity(
                quantity_out,
                quote_quantity - quantity_in_left,
                is_bid,
            )
        } <b>else</b> {
            <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.fee_quantity(
                base_quantity - quantity_in_left,
                quantity_out,
                is_bid,
            )
        };
        <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(taker_fee, fee_quantity.myso())
    };
    <b>if</b> (is_bid) {
        <b>if</b> (quantity_out &lt; self.<a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>) {
            (base_quantity, quote_quantity, 0)
        } <b>else</b> {
            (quantity_out, quantity_in_left, myso_fee)
        }
    } <b>else</b> {
        (quantity_in_left, quantity_out, myso_fee)
    }
}
</code></pre>



</details>

<a name="orderbook_book_get_base_quantity_in"></a>

## Function `get_base_quantity_in`

Given a target quote_quantity to receive from selling, calculate the minimum base_quantity needed.
This is the inverse of get_quantity_out for ask orders.
Returns (base_quantity_in, actual_quote_quantity_out, deep_quantity_required)
Returns (0, 0, 0) if insufficient liquidity or if result would be below min_size.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_base_quantity_in">get_base_quantity_in</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, target_quote_quantity: u64, taker_fee: u64, <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>, pay_with_myso: bool, current_timestamp: u64): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_base_quantity_in">get_base_quantity_in</a>(
    self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>,
    target_quote_quantity: u64,
    taker_fee: u64,
    <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>: OrderMySoPrice,
    pay_with_myso: bool,
    current_timestamp: u64,
): (u64, u64, u64) {
    self.<a href="../orderbook/book.md#orderbook_book_get_quantity_in">get_quantity_in</a>(
        0, // target_base_quantity = 0, we want quote
        target_quote_quantity,
        taker_fee,
        <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>,
        pay_with_myso,
        current_timestamp,
    )
}
</code></pre>



</details>

<a name="orderbook_book_get_quote_quantity_in"></a>

## Function `get_quote_quantity_in`

Given a target base_quantity to receive from buying, calculate the minimum quote_quantity needed.
This is the inverse of get_quantity_out for bid orders.
Returns (actual_base_quantity_out, quote_quantity_in, deep_quantity_required)
Returns (0, 0, 0) if insufficient liquidity or if result would be below min_size.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_quote_quantity_in">get_quote_quantity_in</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, target_base_quantity: u64, taker_fee: u64, <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>, pay_with_myso: bool, current_timestamp: u64): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_quote_quantity_in">get_quote_quantity_in</a>(
    self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>,
    target_base_quantity: u64,
    taker_fee: u64,
    <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>: OrderMySoPrice,
    pay_with_myso: bool,
    current_timestamp: u64,
): (u64, u64, u64) {
    self.<a href="../orderbook/book.md#orderbook_book_get_quantity_in">get_quantity_in</a>(
        target_base_quantity,
        0, // target_quote_quantity = 0, we want base
        taker_fee,
        <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>,
        pay_with_myso,
        current_timestamp,
    )
}
</code></pre>



</details>

<a name="orderbook_book_cancel_order"></a>

## Function `cancel_order`

Cancels an order given order_id


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_cancel_order">cancel_order</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, order_id: u128): <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_cancel_order">cancel_order</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a>, order_id: u128): Order {
    self.<a href="../orderbook/book.md#orderbook_book_book_side_mut">book_side_mut</a>(order_id).remove(order_id)
}
</code></pre>



</details>

<a name="orderbook_book_modify_order"></a>

## Function `modify_order`

Modifies an order given order_id and new_quantity.
New quantity must be less than the original quantity.
Order must not have already expired.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_modify_order">modify_order</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, order_id: u128, new_quantity: u64, timestamp: u64): (u64, &<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_modify_order">modify_order</a>(
    self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a>,
    order_id: u128,
    new_quantity: u64,
    timestamp: u64,
): (u64, &Order) {
    <b>assert</b>!(new_quantity &gt;= self.<a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>, <a href="../orderbook/book.md#orderbook_book_EOrderBelowMinimumSize">EOrderBelowMinimumSize</a>);
    <b>assert</b>!(new_quantity % self.<a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a> == 0, <a href="../orderbook/book.md#orderbook_book_EOrderInvalidLotSize">EOrderInvalidLotSize</a>);
    <b>let</b> <a href="../orderbook/order.md#orderbook_order">order</a> = self.<a href="../orderbook/book.md#orderbook_book_book_side_mut">book_side_mut</a>(order_id).borrow_mut(order_id);
    <b>assert</b>!(new_quantity &lt; <a href="../orderbook/order.md#orderbook_order">order</a>.quantity(), <a href="../orderbook/book.md#orderbook_book_ENewQuantityMustBeLessThanOriginal">ENewQuantityMustBeLessThanOriginal</a>);
    <b>let</b> cancel_quantity = <a href="../orderbook/order.md#orderbook_order">order</a>.quantity() - new_quantity;
    <a href="../orderbook/order.md#orderbook_order">order</a>.modify(new_quantity, timestamp);
    (cancel_quantity, <a href="../orderbook/order.md#orderbook_order">order</a>)
}
</code></pre>



</details>

<a name="orderbook_book_mid_price"></a>

## Function `mid_price`

Returns the mid price of the order book.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_mid_price">mid_price</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, current_timestamp: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_mid_price">mid_price</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>, current_timestamp: u64): u64 {
    <b>let</b> (<b>mut</b> ask_ref, <b>mut</b> ask_offset) = self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a>.min_slice();
    <b>let</b> (<b>mut</b> bid_ref, <b>mut</b> bid_offset) = self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a>.max_slice();
    <b>let</b> <b>mut</b> best_ask_price = 0;
    <b>let</b> <b>mut</b> best_bid_price = 0;
    <b>while</b> (!ask_ref.is_null()) {
        <b>let</b> best_ask_order = slice_borrow(
            self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a>.borrow_slice(ask_ref),
            ask_offset,
        );
        best_ask_price = best_ask_order.price();
        <b>if</b> (current_timestamp &lt;= best_ask_order.expire_timestamp()) <b>break</b>;
        (ask_ref, ask_offset) = self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a>.next_slice(ask_ref, ask_offset);
    };
    <b>while</b> (!bid_ref.is_null()) {
        <b>let</b> best_bid_order = slice_borrow(
            self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a>.borrow_slice(bid_ref),
            bid_offset,
        );
        best_bid_price = best_bid_order.price();
        <b>if</b> (current_timestamp &lt;= best_bid_order.expire_timestamp()) <b>break</b>;
        (bid_ref, bid_offset) = self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a>.prev_slice(bid_ref, bid_offset);
    };
    <b>assert</b>!(!ask_ref.is_null() && !bid_ref.is_null(), <a href="../orderbook/book.md#orderbook_book_EEmptyOrderbook">EEmptyOrderbook</a>);
    <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(best_ask_price + best_bid_price, <a href="../orderbook/constants.md#orderbook_constants_half">constants::half</a>())
}
</code></pre>



</details>

<a name="orderbook_book_get_level2_range_and_ticks"></a>

## Function `get_level2_range_and_ticks`

Returns the best bids and asks.
The number of ticks is the number of price levels to return.
The price_low and price_high are the range of prices to return.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_level2_range_and_ticks">get_level2_range_and_ticks</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, price_low: u64, price_high: u64, ticks: u64, is_bid: bool, current_timestamp: u64): (vector&lt;u64&gt;, vector&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_level2_range_and_ticks">get_level2_range_and_ticks</a>(
    self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>,
    price_low: u64,
    price_high: u64,
    ticks: u64,
    is_bid: bool,
    current_timestamp: u64,
): (vector&lt;u64&gt;, vector&lt;u64&gt;) {
    <b>assert</b>!(price_low &lt;= price_high, <a href="../orderbook/book.md#orderbook_book_EInvalidPriceRange">EInvalidPriceRange</a>);
    <b>assert</b>!(
        price_low &gt;= <a href="../orderbook/constants.md#orderbook_constants_min_price">constants::min_price</a>() &&
        price_low &lt;= <a href="../orderbook/constants.md#orderbook_constants_max_price">constants::max_price</a>(),
        <a href="../orderbook/book.md#orderbook_book_EInvalidPriceRange">EInvalidPriceRange</a>,
    );
    <b>assert</b>!(
        price_high &gt;= <a href="../orderbook/constants.md#orderbook_constants_min_price">constants::min_price</a>() &&
        price_high &lt;= <a href="../orderbook/constants.md#orderbook_constants_max_price">constants::max_price</a>(),
        <a href="../orderbook/book.md#orderbook_book_EInvalidPriceRange">EInvalidPriceRange</a>,
    );
    <b>assert</b>!(ticks &gt; 0, <a href="../orderbook/book.md#orderbook_book_EInvalidTicks">EInvalidTicks</a>);
    <b>let</b> <b>mut</b> price_vec = vector[];
    <b>let</b> <b>mut</b> quantity_vec = vector[];
    // convert price_low and price_high to keys <b>for</b> searching
    <b>let</b> msb = <b>if</b> (is_bid) {
        0u128
    } <b>else</b> {
        1u128 &lt;&lt; 127
    };
    <b>let</b> key_low = ((price_low <b>as</b> u128) &lt;&lt; 64) + msb;
    <b>let</b> key_high = ((price_high <b>as</b> u128) &lt;&lt; 64) + (((1u128 &lt;&lt; 64) - 1) <b>as</b> u128) + msb;
    <b>let</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a> = <b>if</b> (is_bid) &self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a> <b>else</b> &self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a>;
    <b>let</b> (<b>mut</b> ref, <b>mut</b> offset) = <b>if</b> (is_bid) {
        <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.slice_before(key_high)
    } <b>else</b> {
        <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.slice_following(key_low)
    };
    <b>let</b> <b>mut</b> ticks_left = ticks;
    <b>let</b> <b>mut</b> cur_price = 0;
    <b>let</b> <b>mut</b> cur_quantity = 0;
    <b>while</b> (!ref.is_null() && ticks_left &gt; 0) {
        <b>let</b> <a href="../orderbook/order.md#orderbook_order">order</a> = slice_borrow(<a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.borrow_slice(ref), offset);
        <b>if</b> (current_timestamp &lt;= <a href="../orderbook/order.md#orderbook_order">order</a>.expire_timestamp()) {
            <b>let</b> (_, order_price, _) = <a href="../orderbook/utils.md#orderbook_utils_decode_order_id">utils::decode_order_id</a>(<a href="../orderbook/order.md#orderbook_order">order</a>.order_id());
            <b>if</b> (
                (is_bid && order_price &lt; price_low) || (
                    !is_bid && order_price &gt; price_high,
                )
            ) <b>break</b>;
            <b>if</b> (
                cur_price == 0 && (
                    (is_bid && order_price &lt;= price_high) || (
                        !is_bid && order_price &gt;= price_low,
                    ),
                )
            ) {
                cur_price = order_price
            };
            <b>if</b> (cur_price != 0 && order_price != cur_price) {
                price_vec.push_back(cur_price);
                quantity_vec.push_back(cur_quantity);
                cur_price = order_price;
                cur_quantity = 0;
                ticks_left = ticks_left - 1;
                <b>if</b> (ticks_left == 0) <b>break</b>;
            };
            <b>if</b> (cur_price != 0) {
                cur_quantity = cur_quantity + <a href="../orderbook/order.md#orderbook_order">order</a>.quantity() - <a href="../orderbook/order.md#orderbook_order">order</a>.filled_quantity();
            };
        };
        (ref, offset) = <b>if</b> (is_bid) <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.prev_slice(ref, offset)
        <b>else</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.next_slice(ref, offset);
    };
    <b>if</b> (cur_price != 0 && ticks_left &gt; 0) {
        price_vec.push_back(cur_price);
        quantity_vec.push_back(cur_quantity);
    };
    (price_vec, quantity_vec)
}
</code></pre>



</details>

<a name="orderbook_book_check_limit_order_params"></a>

## Function `check_limit_order_params`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_check_limit_order_params">check_limit_order_params</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, price: u64, quantity: u64, expire_timestamp: u64, timestamp_ms: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_check_limit_order_params">check_limit_order_params</a>(
    self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>,
    price: u64,
    quantity: u64,
    expire_timestamp: u64,
    timestamp_ms: u64,
): bool {
    <b>if</b> (expire_timestamp &lt;= timestamp_ms) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (quantity &lt; self.<a href="../orderbook/book.md#orderbook_book_min_size">min_size</a> || quantity % self.<a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a> != 0) {
        <b>return</b> <b>false</b>
    };
    <b>if</b> (
        price % self.<a href="../orderbook/book.md#orderbook_book_tick_size">tick_size</a> != 0 || price &lt; <a href="../orderbook/constants.md#orderbook_constants_min_price">constants::min_price</a>() || price &gt; <a href="../orderbook/constants.md#orderbook_constants_max_price">constants::max_price</a>()
    ) {
        <b>return</b> <b>false</b>
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="orderbook_book_check_market_order_params"></a>

## Function `check_market_order_params`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_check_market_order_params">check_market_order_params</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, quantity: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_check_market_order_params">check_market_order_params</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>, quantity: u64): bool {
    <b>if</b> (quantity &lt; self.<a href="../orderbook/book.md#orderbook_book_min_size">min_size</a> || quantity % self.<a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a> != 0) {
        <b>return</b> <b>false</b>
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="orderbook_book_get_order"></a>

## Function `get_order`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_order">get_order</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, order_id: u128): <a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_order">get_order</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>, order_id: u128): Order {
    <b>let</b> <a href="../orderbook/order.md#orderbook_order">order</a> = self.<a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>(order_id).borrow(order_id);
    <a href="../orderbook/order.md#orderbook_order">order</a>.copy_order()
}
</code></pre>



</details>

<a name="orderbook_book_set_tick_size"></a>

## Function `set_tick_size`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_set_tick_size">set_tick_size</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, new_tick_size: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_set_tick_size">set_tick_size</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a>, new_tick_size: u64) {
    self.<a href="../orderbook/book.md#orderbook_book_tick_size">tick_size</a> = new_tick_size;
}
</code></pre>



</details>

<a name="orderbook_book_set_lot_size"></a>

## Function `set_lot_size`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_set_lot_size">set_lot_size</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, new_lot_size: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_set_lot_size">set_lot_size</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a>, new_lot_size: u64) {
    self.<a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a> = new_lot_size;
}
</code></pre>



</details>

<a name="orderbook_book_set_min_size"></a>

## Function `set_min_size`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_set_min_size">set_min_size</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, new_min_size: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/book.md#orderbook_book_set_min_size">set_min_size</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a>, new_min_size: u64) {
    self.<a href="../orderbook/book.md#orderbook_book_min_size">min_size</a> = new_min_size;
}
</code></pre>



</details>

<a name="orderbook_book_book_side_mut"></a>

## Function `book_side_mut`



<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_book_side_mut">book_side_mut</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, order_id: u128): &<b>mut</b> <a href="../orderbook/big_vector.md#orderbook_big_vector_BigVector">orderbook::big_vector::BigVector</a>&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_book_side_mut">book_side_mut</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a>, order_id: u128): &<b>mut</b> BigVector&lt;Order&gt; {
    <b>let</b> (is_bid, _, _) = <a href="../orderbook/utils.md#orderbook_utils_decode_order_id">utils::decode_order_id</a>(order_id);
    <b>if</b> (is_bid) {
        &<b>mut</b> self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a>
    } <b>else</b> {
        &<b>mut</b> self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a>
    }
}
</code></pre>



</details>

<a name="orderbook_book_book_side"></a>

## Function `book_side`



<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, order_id: u128): &<a href="../orderbook/big_vector.md#orderbook_big_vector_BigVector">orderbook::big_vector::BigVector</a>&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>, order_id: u128): &BigVector&lt;Order&gt; {
    <b>let</b> (is_bid, _, _) = <a href="../orderbook/utils.md#orderbook_utils_decode_order_id">utils::decode_order_id</a>(order_id);
    <b>if</b> (is_bid) {
        &self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a>
    } <b>else</b> {
        &self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a>
    }
}
</code></pre>



</details>

<a name="orderbook_book_match_against_book"></a>

## Function `match_against_book`

Matches the given order and quantity against the order book.
If is_bid, it will match against asks, otherwise against bids.
Mutates the order and the maker order as necessary.


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_match_against_book">match_against_book</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &<b>mut</b> <a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_match_against_book">match_against_book</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a>, <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &<b>mut</b> OrderInfo, timestamp: u64) {
    <b>let</b> is_bid = <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.is_bid();
    <b>let</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a> = <b>if</b> (is_bid) &<b>mut</b> self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a> <b>else</b> &<b>mut</b> self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a>;
    <b>let</b> (<b>mut</b> ref, <b>mut</b> offset) = <b>if</b> (is_bid) <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.min_slice() <b>else</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.max_slice();
    <b>let</b> max_fills = <a href="../orderbook/constants.md#orderbook_constants_max_fills">constants::max_fills</a>();
    <b>let</b> <b>mut</b> current_fills = 0;
    <b>while</b> (!ref.is_null() &&
        current_fills &lt; max_fills) {
        <b>let</b> maker_order = slice_borrow_mut(
            <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.borrow_slice_mut(ref),
            offset,
        );
        <b>if</b> (!<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.match_maker(maker_order, timestamp)) <b>break</b>;
        (ref, offset) = <b>if</b> (is_bid) <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.next_slice(ref, offset)
        <b>else</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.prev_slice(ref, offset);
        current_fills = current_fills + 1;
    };
    <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.fills_ref().do_ref!(|<a href="../orderbook/fill.md#orderbook_fill">fill</a>| {
        <b>if</b> (<a href="../orderbook/fill.md#orderbook_fill">fill</a>.expired() || <a href="../orderbook/fill.md#orderbook_fill">fill</a>.completed()) {
            <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.remove(<a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_order_id());
        };
    });
    <b>if</b> (current_fills == max_fills) {
        <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.set_fill_limit_reached();
    }
}
</code></pre>



</details>

<a name="orderbook_book_get_order_id"></a>

## Function `get_order_id`



<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_order_id">get_order_id</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, is_bid: bool): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_order_id">get_order_id</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a>, is_bid: bool): u64 {
    <b>if</b> (is_bid) {
        self.next_bid_order_id = self.next_bid_order_id - 1;
        self.next_bid_order_id
    } <b>else</b> {
        self.next_ask_order_id = self.next_ask_order_id + 1;
        self.next_ask_order_id
    }
}
</code></pre>



</details>

<a name="orderbook_book_inject_limit_order"></a>

## Function `inject_limit_order`

Balance accounting happens before this function is called


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_inject_limit_order">inject_limit_order</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &<a href="../orderbook/order_info.md#orderbook_order_info_OrderInfo">orderbook::order_info::OrderInfo</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_inject_limit_order">inject_limit_order</a>(self: &<b>mut</b> <a href="../orderbook/book.md#orderbook_book_Book">Book</a>, <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>: &OrderInfo) {
    <b>let</b> <a href="../orderbook/order.md#orderbook_order">order</a> = <a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.to_order();
    <b>if</b> (<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.is_bid()) {
        self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a>.insert(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.order_id(), <a href="../orderbook/order.md#orderbook_order">order</a>);
    } <b>else</b> {
        self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a>.insert(<a href="../orderbook/order_info.md#orderbook_order_info">order_info</a>.order_id(), <a href="../orderbook/order.md#orderbook_order">order</a>);
    };
}
</code></pre>



</details>

<a name="orderbook_book_round_up_to_lot_size"></a>

## Function `round_up_to_lot_size`

Rounds up a quantity to the nearest lot_size multiple.
Returns the smallest multiple of lot_size that is >= quantity.


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_round_up_to_lot_size">round_up_to_lot_size</a>(quantity: u64, <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_round_up_to_lot_size">round_up_to_lot_size</a>(quantity: u64, <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>: u64): u64 {
    <b>let</b> remainder = quantity % <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>;
    <b>if</b> (remainder == 0) quantity <b>else</b> quantity + <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a> - remainder
}
</code></pre>



</details>

<a name="orderbook_book_get_quantity_in"></a>

## Function `get_quantity_in`

If target_base_quantity > 0: Calculate quote needed to buy that base (bid order)
If target_quote_quantity > 0: Calculate base needed to get that quote (ask order)
Returns (base_result, quote_result, deep_quantity_required)


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_quantity_in">get_quantity_in</a>(self: &<a href="../orderbook/book.md#orderbook_book_Book">orderbook::book::Book</a>, target_base_quantity: u64, target_quote_quantity: u64, taker_fee: u64, <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>: <a href="../orderbook/myso_price.md#orderbook_myso_price_OrderMySoPrice">orderbook::myso_price::OrderMySoPrice</a>, pay_with_myso: bool, current_timestamp: u64): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/book.md#orderbook_book_get_quantity_in">get_quantity_in</a>(
    self: &<a href="../orderbook/book.md#orderbook_book_Book">Book</a>,
    target_base_quantity: u64,
    target_quote_quantity: u64,
    taker_fee: u64,
    <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>: OrderMySoPrice,
    pay_with_myso: bool,
    current_timestamp: u64,
): (u64, u64, u64) {
    <b>assert</b>!((target_base_quantity &gt; 0) != (target_quote_quantity &gt; 0), <a href="../orderbook/book.md#orderbook_book_EInvalidAmountIn">EInvalidAmountIn</a>);
    <b>let</b> is_bid = target_base_quantity &gt; 0;
    <b>let</b> input_fee_rate = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(
        <a href="../orderbook/constants.md#orderbook_constants_fee_penalty_multiplier">constants::fee_penalty_multiplier</a>(),
        taker_fee,
    );
    <b>let</b> <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a> = self.<a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>;
    <b>let</b> <b>mut</b> input_quantity = 0; // This will be quote <b>for</b> bid, base <b>for</b> ask (may include fees)
    <b>let</b> <b>mut</b> output_accumulated = 0; // This will be base <b>for</b> bid, quote <b>for</b> ask
    <b>let</b> <b>mut</b> traded_base = 0; // Raw base traded, used <b>for</b> <a href="../orderbook/book.md#orderbook_book_min_size">min_size</a> checks on <a href="../orderbook/book.md#orderbook_book_asks">asks</a>
    // For bid: traverse <a href="../orderbook/book.md#orderbook_book_asks">asks</a> (we're buying base with quote)
    // For ask: traverse <a href="../orderbook/book.md#orderbook_book_bids">bids</a> (we're selling base <b>for</b> quote)
    <b>let</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a> = <b>if</b> (is_bid) &self.<a href="../orderbook/book.md#orderbook_book_asks">asks</a> <b>else</b> &self.<a href="../orderbook/book.md#orderbook_book_bids">bids</a>;
    <b>let</b> (<b>mut</b> ref, <b>mut</b> offset) = <b>if</b> (is_bid) <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.min_slice() <b>else</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.max_slice();
    <b>let</b> max_fills = <a href="../orderbook/constants.md#orderbook_constants_max_fills">constants::max_fills</a>();
    <b>let</b> <b>mut</b> current_fills = 0;
    <b>let</b> target = <b>if</b> (is_bid) target_base_quantity <b>else</b> target_quote_quantity;
    <b>while</b> (!ref.is_null() && output_accumulated &lt; target && current_fills &lt; max_fills) {
        <b>let</b> <a href="../orderbook/order.md#orderbook_order">order</a> = slice_borrow(<a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.borrow_slice(ref), offset);
        <b>let</b> cur_price = <a href="../orderbook/order.md#orderbook_order">order</a>.price();
        <b>let</b> cur_quantity = <a href="../orderbook/order.md#orderbook_order">order</a>.quantity() - <a href="../orderbook/order.md#orderbook_order">order</a>.filled_quantity();
        <b>if</b> (current_timestamp &lt;= <a href="../orderbook/order.md#orderbook_order">order</a>.expire_timestamp()) {
            <b>let</b> output_needed = target - output_accumulated;
            <b>if</b> (is_bid) {
                // Buying base with quote: find smallest lot-multiple &gt;= output_needed, capped by cur_quantity
                <b>let</b> target_lots = <a href="../orderbook/book.md#orderbook_book_round_up_to_lot_size">round_up_to_lot_size</a>(output_needed, <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>);
                <b>let</b> matched_base = target_lots.min(cur_quantity);
                <b>if</b> (matched_base &gt; 0) {
                    output_accumulated = output_accumulated + matched_base;
                    <b>let</b> matched_quote = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(matched_base, cur_price);
                    // Calculate quote needed including fees
                    <b>if</b> (pay_with_myso) {
                        input_quantity = input_quantity + matched_quote;
                    } <b>else</b> {
                        // Need extra quote to cover fees (fees taken from input)
                        <b>let</b> quote_with_fee = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(
                            matched_quote,
                            <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>() + input_fee_rate,
                        );
                        input_quantity = input_quantity + quote_with_fee;
                    }
                };
                <b>if</b> (matched_base == 0) <b>break</b>;
            } <b>else</b> {
                // Selling base <b>for</b> quote: find smallest lot-multiple of base that yields &gt;= output_needed quote
                <b>let</b> base_for_quote = <a href="../orderbook/math.md#orderbook_math_div_round_up">math::div_round_up</a>(output_needed, cur_price);
                <b>let</b> target_lots = <a href="../orderbook/book.md#orderbook_book_round_up_to_lot_size">round_up_to_lot_size</a>(base_for_quote, <a href="../orderbook/book.md#orderbook_book_lot_size">lot_size</a>);
                <b>let</b> matched_base = target_lots.min(cur_quantity);
                <b>if</b> (matched_base &gt; 0) {
                    traded_base = traded_base + matched_base;
                    <b>let</b> matched_quote = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(matched_base, cur_price);
                    output_accumulated = output_accumulated + matched_quote;
                    // Calculate base needed including fees
                    <b>if</b> (pay_with_myso) {
                        input_quantity = input_quantity + matched_base;
                    } <b>else</b> {
                        // Need extra base to cover fees (fees taken from input)
                        <b>let</b> base_with_fee = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(
                            matched_base,
                            <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>() + input_fee_rate,
                        );
                        input_quantity = input_quantity + base_with_fee;
                    }
                };
                <b>if</b> (matched_base == 0) <b>break</b>;
            }
        };
        (ref, offset) = <b>if</b> (is_bid) <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.next_slice(ref, offset)
        <b>else</b> <a href="../orderbook/book.md#orderbook_book_book_side">book_side</a>.prev_slice(ref, offset);
        current_fills = current_fills + 1;
    };
    // Calculate myso fee <b>if</b> paying with MYSO
    <b>let</b> myso_fee = <b>if</b> (!pay_with_myso) {
        0
    } <b>else</b> {
        <b>let</b> fee_quantity = <b>if</b> (is_bid) {
            <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.fee_quantity(
                output_accumulated,
                input_quantity,
                <b>true</b>, // is_bid
            )
        } <b>else</b> {
            <a href="../orderbook/myso_price.md#orderbook_myso_price">myso_price</a>.fee_quantity(
                input_quantity,
                output_accumulated,
                <b>false</b>, // is_ask
            )
        };
        <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(taker_fee, fee_quantity.myso())
    };
    // Check <b>if</b> we accumulated enough and meets <a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>
    <b>let</b> sufficient = <b>if</b> (is_bid) {
        output_accumulated &gt;= target_base_quantity && output_accumulated &gt;= self.<a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>
    } <b>else</b> {
        output_accumulated &gt;= target_quote_quantity && traded_base &gt;= self.<a href="../orderbook/book.md#orderbook_book_min_size">min_size</a>
    };
    <b>if</b> (!sufficient) {
        (0, 0, 0) // Couldn't satisfy the requirement
    } <b>else</b> {
        <b>if</b> (is_bid) {
            (output_accumulated, input_quantity, myso_fee)
        } <b>else</b> {
            (input_quantity, output_accumulated, myso_fee)
        }
    }
}
</code></pre>



</details>
