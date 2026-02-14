---
title: Module `orderbook::order_query`
---

This module defines the OrderPage struct and its methods to iterate over orders in a pool.


-  [Struct `OrderPage`](#orderbook_order_query_OrderPage)
-  [Function `iter_orders`](#orderbook_order_query_iter_orders)
-  [Function `orders`](#orderbook_order_query_orders)
-  [Function `has_next_page`](#orderbook_order_query_has_next_page)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
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
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/myso.md#myso_myso">myso::myso</a>;
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
<b>use</b> <a href="../orderbook/account.md#orderbook_account">orderbook::account</a>;
<b>use</b> <a href="../orderbook/balance_manager.md#orderbook_balance_manager">orderbook::balance_manager</a>;
<b>use</b> <a href="../orderbook/balances.md#orderbook_balances">orderbook::balances</a>;
<b>use</b> <a href="../orderbook/big_vector.md#orderbook_big_vector">orderbook::big_vector</a>;
<b>use</b> <a href="../orderbook/book.md#orderbook_book">orderbook::book</a>;
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/ewma.md#orderbook_ewma">orderbook::ewma</a>;
<b>use</b> <a href="../orderbook/fill.md#orderbook_fill">orderbook::fill</a>;
<b>use</b> <a href="../orderbook/governance.md#orderbook_governance">orderbook::governance</a>;
<b>use</b> <a href="../orderbook/history.md#orderbook_history">orderbook::history</a>;
<b>use</b> <a href="../orderbook/math.md#orderbook_math">orderbook::math</a>;
<b>use</b> <a href="../orderbook/myso_price.md#orderbook_myso_price">orderbook::myso_price</a>;
<b>use</b> <a href="../orderbook/order.md#orderbook_order">orderbook::order</a>;
<b>use</b> <a href="../orderbook/order_info.md#orderbook_order_info">orderbook::order_info</a>;
<b>use</b> <a href="../orderbook/pool.md#orderbook_pool">orderbook::pool</a>;
<b>use</b> <a href="../orderbook/registry.md#orderbook_registry">orderbook::registry</a>;
<b>use</b> <a href="../orderbook/state.md#orderbook_state">orderbook::state</a>;
<b>use</b> <a href="../orderbook/trade_params.md#orderbook_trade_params">orderbook::trade_params</a>;
<b>use</b> <a href="../orderbook/utils.md#orderbook_utils">orderbook::utils</a>;
<b>use</b> <a href="../orderbook/vault.md#orderbook_vault">orderbook::vault</a>;
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



<a name="orderbook_order_query_OrderPage"></a>

## Struct `OrderPage`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/order_query.md#orderbook_order_query_OrderPage">OrderPage</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/order_query.md#orderbook_order_query_orders">orders</a>: vector&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/order_query.md#orderbook_order_query_has_next_page">has_next_page</a>: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_order_query_iter_orders"></a>

## Function `iter_orders`

Bid minimum order id has 0 for its first bit, 0 for next 63 bits for price, and 1 for next 64 bits for order id.
Ask minimum order id has 1 for its first bit, 0 for next 63 bits for price, and 0 for next 64 bits for order id.
Bids are iterated from high to low order id, and asks are iterated from low to high order id.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_query.md#orderbook_order_query_iter_orders">iter_orders</a>&lt;BaseAsset, QuoteAsset&gt;(self: &<a href="../orderbook/pool.md#orderbook_pool_Pool">orderbook::pool::Pool</a>&lt;BaseAsset, QuoteAsset&gt;, start_order_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u128&gt;, end_order_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u128&gt;, min_expire_timestamp: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, limit: u64, bids: bool): <a href="../orderbook/order_query.md#orderbook_order_query_OrderPage">orderbook::order_query::OrderPage</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_query.md#orderbook_order_query_iter_orders">iter_orders</a>&lt;BaseAsset, QuoteAsset&gt;(
    self: &Pool&lt;BaseAsset, QuoteAsset&gt;,
    start_order_id: Option&lt;u128&gt;,
    end_order_id: Option&lt;u128&gt;,
    min_expire_timestamp: Option&lt;u64&gt;,
    limit: u64,
    bids: bool,
): <a href="../orderbook/order_query.md#orderbook_order_query_OrderPage">OrderPage</a> {
    <b>let</b> self = self.load_inner();
    <b>let</b> bid_min_order_id = 0;
    <b>let</b> bid_max_order_id = 1u128 &lt;&lt; 127;
    <b>let</b> ask_min_order_id = 1u128 &lt;&lt; 127;
    <b>let</b> ask_max_order_id = <a href="../orderbook/constants.md#orderbook_constants_max_u128">constants::max_u128</a>();
    <b>let</b> start = start_order_id.get_with_default({
        <b>if</b> (bids) bid_max_order_id <b>else</b> ask_min_order_id
    });
    <b>let</b> end = end_order_id.get_with_default({
        <b>if</b> (bids) bid_min_order_id <b>else</b> ask_max_order_id
    });
    <b>let</b> min_expire = min_expire_timestamp.get_with_default(0);
    <b>let</b> side = <b>if</b> (bids) self.bids() <b>else</b> self.asks();
    <b>let</b> <b>mut</b> <a href="../orderbook/order_query.md#orderbook_order_query_orders">orders</a> = vector[];
    <b>let</b> (<b>mut</b> ref, <b>mut</b> offset) = <b>if</b> (bids) {
        side.slice_before(start)
    } <b>else</b> {
        side.slice_following(start)
    };
    <b>while</b> (!ref.is_null() && <a href="../orderbook/order_query.md#orderbook_order_query_orders">orders</a>.length() &lt; limit) {
        <b>let</b> <a href="../orderbook/order.md#orderbook_order">order</a> = slice_borrow(side.borrow_slice(ref), offset);
        <b>if</b> (bids && <a href="../orderbook/order.md#orderbook_order">order</a>.order_id() &lt; end) <b>break</b>;
        <b>if</b> (!bids && <a href="../orderbook/order.md#orderbook_order">order</a>.order_id() &gt; end) <b>break</b>;
        <b>if</b> (<a href="../orderbook/order.md#orderbook_order">order</a>.expire_timestamp() &gt;= min_expire) {
            <a href="../orderbook/order_query.md#orderbook_order_query_orders">orders</a>.push_back(<a href="../orderbook/order.md#orderbook_order">order</a>.copy_order());
        };
        (ref, offset) = <b>if</b> (bids) side.prev_slice(ref, offset) <b>else</b> side.next_slice(ref, offset);
    };
    <a href="../orderbook/order_query.md#orderbook_order_query_OrderPage">OrderPage</a> {
        <a href="../orderbook/order_query.md#orderbook_order_query_orders">orders</a>: <a href="../orderbook/order_query.md#orderbook_order_query_orders">orders</a>,
        <a href="../orderbook/order_query.md#orderbook_order_query_has_next_page">has_next_page</a>: !ref.is_null(),
    }
}
</code></pre>



</details>

<a name="orderbook_order_query_orders"></a>

## Function `orders`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_query.md#orderbook_order_query_orders">orders</a>(self: &<a href="../orderbook/order_query.md#orderbook_order_query_OrderPage">orderbook::order_query::OrderPage</a>): &vector&lt;<a href="../orderbook/order.md#orderbook_order_Order">orderbook::order::Order</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_query.md#orderbook_order_query_orders">orders</a>(self: &<a href="../orderbook/order_query.md#orderbook_order_query_OrderPage">OrderPage</a>): &vector&lt;Order&gt; {
    &self.<a href="../orderbook/order_query.md#orderbook_order_query_orders">orders</a>
}
</code></pre>



</details>

<a name="orderbook_order_query_has_next_page"></a>

## Function `has_next_page`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_query.md#orderbook_order_query_has_next_page">has_next_page</a>(self: &<a href="../orderbook/order_query.md#orderbook_order_query_OrderPage">orderbook::order_query::OrderPage</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/order_query.md#orderbook_order_query_has_next_page">has_next_page</a>(self: &<a href="../orderbook/order_query.md#orderbook_order_query_OrderPage">OrderPage</a>): bool {
    self.<a href="../orderbook/order_query.md#orderbook_order_query_has_next_page">has_next_page</a>
}
</code></pre>



</details>
