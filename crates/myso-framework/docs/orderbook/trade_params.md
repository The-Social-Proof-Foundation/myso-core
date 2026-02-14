---
title: Module `orderbook::trade_params`
---

TradeParams module contains the trade parameters for a trading pair.


-  [Struct `TradeParams`](#orderbook_trade_params_TradeParams)
-  [Function `new`](#orderbook_trade_params_new)
-  [Function `maker_fee`](#orderbook_trade_params_maker_fee)
-  [Function `taker_fee`](#orderbook_trade_params_taker_fee)
-  [Function `taker_fee_for_user`](#orderbook_trade_params_taker_fee_for_user)
-  [Function `stake_required`](#orderbook_trade_params_stake_required)


<pre><code></code></pre>



<a name="orderbook_trade_params_TradeParams"></a>

## Struct `TradeParams`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">TradeParams</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee">taker_fee</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/trade_params.md#orderbook_trade_params_maker_fee">maker_fee</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/trade_params.md#orderbook_trade_params_stake_required">stake_required</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_trade_params_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_new">new</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee">taker_fee</a>: u64, <a href="../orderbook/trade_params.md#orderbook_trade_params_maker_fee">maker_fee</a>: u64, <a href="../orderbook/trade_params.md#orderbook_trade_params_stake_required">stake_required</a>: u64): <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_new">new</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee">taker_fee</a>: u64, <a href="../orderbook/trade_params.md#orderbook_trade_params_maker_fee">maker_fee</a>: u64, <a href="../orderbook/trade_params.md#orderbook_trade_params_stake_required">stake_required</a>: u64): <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">TradeParams</a> {
    <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">TradeParams</a> { <a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee">taker_fee</a>, <a href="../orderbook/trade_params.md#orderbook_trade_params_maker_fee">maker_fee</a>, <a href="../orderbook/trade_params.md#orderbook_trade_params_stake_required">stake_required</a> }
}
</code></pre>



</details>

<a name="orderbook_trade_params_maker_fee"></a>

## Function `maker_fee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_maker_fee">maker_fee</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: &<a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_maker_fee">maker_fee</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: &<a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">TradeParams</a>): u64 {
    <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params_maker_fee">maker_fee</a>
}
</code></pre>



</details>

<a name="orderbook_trade_params_taker_fee"></a>

## Function `taker_fee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee">taker_fee</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: &<a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee">taker_fee</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: &<a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">TradeParams</a>): u64 {
    <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee">taker_fee</a>
}
</code></pre>



</details>

<a name="orderbook_trade_params_taker_fee_for_user"></a>

## Function `taker_fee_for_user`

Returns the taker fee for a user based on the active stake and volume in myso.
Taker fee is halved if user has enough stake and volume.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee_for_user">taker_fee_for_user</a>(self: &<a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>, active_stake: u64, volume_in_deep: u128): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee_for_user">taker_fee_for_user</a>(
    self: &<a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">TradeParams</a>,
    active_stake: u64,
    volume_in_deep: u128,
): u64 {
    <b>if</b> (
        active_stake &gt;= self.<a href="../orderbook/trade_params.md#orderbook_trade_params_stake_required">stake_required</a> &&
        volume_in_deep &gt;= (self.<a href="../orderbook/trade_params.md#orderbook_trade_params_stake_required">stake_required</a> <b>as</b> u128)
    ) {
        self.<a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee">taker_fee</a> / 2
    } <b>else</b> {
        self.<a href="../orderbook/trade_params.md#orderbook_trade_params_taker_fee">taker_fee</a>
    }
}
</code></pre>



</details>

<a name="orderbook_trade_params_stake_required"></a>

## Function `stake_required`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_stake_required">stake_required</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: &<a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/trade_params.md#orderbook_trade_params_stake_required">stake_required</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: &<a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">TradeParams</a>): u64 {
    <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.<a href="../orderbook/trade_params.md#orderbook_trade_params_stake_required">stake_required</a>
}
</code></pre>



</details>
