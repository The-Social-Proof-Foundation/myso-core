---
title: Module `myso::pay`
---

This module provides handy functionality for wallets and <code>myso::Coin</code> management.


-  [Constants](#@Constants_0)
-  [Function `keep`](#myso_pay_keep)
-  [Function `split`](#myso_pay_split)
-  [Function `split_vec`](#myso_pay_split_vec)
-  [Function `split_and_transfer`](#myso_pay_split_and_transfer)
-  [Function `divide_and_keep`](#myso_pay_divide_and_keep)
-  [Function `join`](#myso_pay_join)
-  [Function `join_vec`](#myso_pay_join_vec)
-  [Function `join_vec_and_transfer`](#myso_pay_join_vec_and_transfer)


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
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="myso_pay_ENoCoins"></a>

For when empty vector is supplied into join function.


<pre><code><b>const</b> <a href="../myso/pay.md#myso_pay_ENoCoins">ENoCoins</a>: u64 = 0;
</code></pre>



<a name="myso_pay_keep"></a>

## Function `keep`

Transfer <code>c</code> to the sender of the current transaction


<pre><code><b>public</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_keep">keep</a>&lt;T&gt;(c: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_keep">keep</a>&lt;T&gt;(c: Coin&lt;T&gt;, ctx: &TxContext) {
    <a href="../myso/transfer.md#myso_transfer_public_transfer">transfer::public_transfer</a>(c, ctx.sender())
}
</code></pre>



</details>

<a name="myso_pay_split"></a>

## Function `split`

Split <code><a href="../myso/coin.md#myso_coin">coin</a></code> to two coins, one with balance <code>split_amount</code>,
and the remaining balance is left in <code><a href="../myso/coin.md#myso_coin">coin</a></code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_split">split</a>&lt;T&gt;(<a href="../myso/coin.md#myso_coin">coin</a>: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_split">split</a>&lt;T&gt;(<a href="../myso/coin.md#myso_coin">coin</a>: &<b>mut</b> Coin&lt;T&gt;, split_amount: u64, ctx: &<b>mut</b> TxContext) {
    <a href="../myso/pay.md#myso_pay_keep">keep</a>(<a href="../myso/coin.md#myso_coin">coin</a>.<a href="../myso/pay.md#myso_pay_split">split</a>(split_amount, ctx), ctx)
}
</code></pre>



</details>

<a name="myso_pay_split_vec"></a>

## Function `split_vec`

Split coin <code>self</code> into multiple coins, each with balance specified
in <code>split_amounts</code>. Remaining balance is left in <code>self</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_split_vec">split_vec</a>&lt;T&gt;(self: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, split_amounts: vector&lt;u64&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_split_vec">split_vec</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, split_amounts: vector&lt;u64&gt;, ctx: &<b>mut</b> TxContext) {
    split_amounts.do!(|amount| <a href="../myso/pay.md#myso_pay_split">split</a>(self, amount, ctx));
}
</code></pre>



</details>

<a name="myso_pay_split_and_transfer"></a>

## Function `split_and_transfer`

Send <code>amount</code> units of <code>c</code> to <code>recipient</code>
Aborts with <code><a href="../myso/balance.md#myso_balance_ENotEnough">myso::balance::ENotEnough</a></code> if <code>amount</code> is greater than the balance in <code>c</code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_split_and_transfer">split_and_transfer</a>&lt;T&gt;(c: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, amount: u64, recipient: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_split_and_transfer">split_and_transfer</a>&lt;T&gt;(
    c: &<b>mut</b> Coin&lt;T&gt;,
    amount: u64,
    recipient: <b>address</b>,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../myso/transfer.md#myso_transfer_public_transfer">transfer::public_transfer</a>(c.<a href="../myso/pay.md#myso_pay_split">split</a>(amount, ctx), recipient)
}
</code></pre>



</details>

<a name="myso_pay_divide_and_keep"></a>

## Function `divide_and_keep`

Divide coin <code>self</code> into <code>n - 1</code> coins with equal balances. If the balance is
not evenly divisible by <code>n</code>, the remainder is left in <code>self</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_divide_and_keep">divide_and_keep</a>&lt;T&gt;(self: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, n: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_divide_and_keep">divide_and_keep</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, n: u64, ctx: &<b>mut</b> TxContext) {
    self.divide_into_n(n, ctx).destroy!(|<a href="../myso/coin.md#myso_coin">coin</a>| <a href="../myso/transfer.md#myso_transfer_public_transfer">transfer::public_transfer</a>(<a href="../myso/coin.md#myso_coin">coin</a>, ctx.sender()));
}
</code></pre>



</details>

<a name="myso_pay_join"></a>

## Function `join`

Join <code><a href="../myso/coin.md#myso_coin">coin</a></code> into <code>self</code>. Re-exports <code><a href="../myso/coin.md#myso_coin_join">coin::join</a></code> function.
Deprecated: you should call <code><a href="../myso/coin.md#myso_coin">coin</a>.<a href="../myso/pay.md#myso_pay_join">join</a>(other)</code> directly.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, <a href="../myso/coin.md#myso_coin">coin</a>: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_join">join</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, <a href="../myso/coin.md#myso_coin">coin</a>: Coin&lt;T&gt;) {
    self.<a href="../myso/pay.md#myso_pay_join">join</a>(<a href="../myso/coin.md#myso_coin">coin</a>)
}
</code></pre>



</details>

<a name="myso_pay_join_vec"></a>

## Function `join_vec`

Join everything in <code>coins</code> with <code>self</code>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_join_vec">join_vec</a>&lt;T&gt;(self: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, coins: vector&lt;<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_join_vec">join_vec</a>&lt;T&gt;(self: &<b>mut</b> Coin&lt;T&gt;, coins: vector&lt;Coin&lt;T&gt;&gt;) {
    coins.destroy!(|<a href="../myso/coin.md#myso_coin">coin</a>| self.<a href="../myso/pay.md#myso_pay_join">join</a>(<a href="../myso/coin.md#myso_coin">coin</a>));
}
</code></pre>



</details>

<a name="myso_pay_join_vec_and_transfer"></a>

## Function `join_vec_and_transfer`

Join a vector of <code>Coin</code> into a single object and transfer it to <code>receiver</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_join_vec_and_transfer">join_vec_and_transfer</a>&lt;T&gt;(coins: vector&lt;<a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;&gt;, receiver: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/pay.md#myso_pay_join_vec_and_transfer">join_vec_and_transfer</a>&lt;T&gt;(<b>mut</b> coins: vector&lt;Coin&lt;T&gt;&gt;, receiver: <b>address</b>) {
    <b>assert</b>!(coins.length() &gt; 0, <a href="../myso/pay.md#myso_pay_ENoCoins">ENoCoins</a>);
    <b>let</b> <b>mut</b> self = coins.pop_back();
    <a href="../myso/pay.md#myso_pay_join_vec">join_vec</a>(&<b>mut</b> self, coins);
    <a href="../myso/transfer.md#myso_transfer_public_transfer">transfer::public_transfer</a>(self, receiver)
}
</code></pre>



</details>
