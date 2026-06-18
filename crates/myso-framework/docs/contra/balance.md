---
title: Module `contra::balance`
---

Confidential value: <code><a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;</code> (a single encrypted amount with a count of merged
u16-bounded values that bounds limb growth), plus the linear coin types <code><a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;</code> and
<code><a href="../contra/balance.md#contra_balance_EncryptedCoin">EncryptedCoin</a>&lt;T&gt;</code> that move value in and out.


-  [Struct `EncryptedBalance`](#contra_balance_EncryptedBalance)
-  [Struct `PublicCoin`](#contra_balance_PublicCoin)
-  [Struct `EncryptedCoin`](#contra_balance_EncryptedCoin)
-  [Constants](#@Constants_0)
-  [Function `zero`](#contra_balance_zero)
-  [Function `wrap`](#contra_balance_wrap)
-  [Function `unwrap`](#contra_balance_unwrap)
-  [Function `join`](#contra_balance_join)
-  [Function `take`](#contra_balance_take)
-  [Function `value`](#contra_balance_value)
-  [Function `amount`](#contra_balance_amount)
-  [Function `new`](#contra_balance_new)
-  [Function `empty`](#contra_balance_empty)
-  [Function `upper_bound`](#contra_balance_upper_bound)
-  [Function `max_upper_bound`](#contra_balance_max_upper_bound)
-  [Function `max_upper_bound_minus_1`](#contra_balance_max_upper_bound_minus_1)
-  [Function `is_empty`](#contra_balance_is_empty)
-  [Function `collapse`](#contra_balance_collapse)
-  [Function `merge_into`](#contra_balance_merge_into)
-  [Function `merge_encrypted`](#contra_balance_merge_encrypted)
-  [Function `merge_public`](#contra_balance_merge_public)
-  [Function `try_split_to_public`](#contra_balance_try_split_to_public)
-  [Function `try_split_batch`](#contra_balance_try_split_batch)
-  [Function `try_update`](#contra_balance_try_update)
-  [Function `try_set_public_key`](#contra_balance_try_set_public_key)
-  [Function `overwrite_unchecked`](#contra_balance_overwrite_unchecked)
-  [Function `clear_unchecked`](#contra_balance_clear_unchecked)
-  [Function `set_zero_unchecked`](#contra_balance_set_zero_unchecked)
-  [Function `overwrite`](#contra_balance_overwrite)
-  [Function `set_empty`](#contra_balance_set_empty)


<pre><code><b>use</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount">contra::encrypted_amount</a>;
<b>use</b> <a href="../contra/nizk.md#contra_nizk">contra::nizk</a>;
<b>use</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal">contra::twisted_elgamal</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
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
<b>use</b> <a href="../myso/group_ops.md#myso_group_ops">myso::group_ops</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/rangeproofs.md#myso_rangeproofs">myso::rangeproofs</a>;
<b>use</b> <a href="../myso/ristretto255.md#myso_ristretto255">myso::ristretto255</a>;
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



<a name="contra_balance_EncryptedBalance"></a>

## Struct `EncryptedBalance`

A single confidential amount: an <code>EncryptedAmount</code> plus the count of u16-bounded values that
have been folded into it.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;<b>phantom</b> T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../contra/balance.md#contra_balance_amount">amount</a>: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a>: u16</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_balance_PublicCoin"></a>

## Struct `PublicCoin`

Linear wrapper around a publicly-known <code>u64</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;<b>phantom</b> T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../contra/balance.md#contra_balance_value">value</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_balance_EncryptedCoin"></a>

## Struct `EncryptedCoin`

Linear wrapper around a verified encrypted amount.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/balance.md#contra_balance_EncryptedCoin">EncryptedCoin</a>&lt;<b>phantom</b> T&gt;
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../contra/balance.md#contra_balance_amount">amount</a>: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="contra_balance_EConsistencyProofFailed"></a>

<code><a href="../contra/balance.md#contra_balance_try_split_batch">try_split_batch</a></code>: consistency proof failed.


<pre><code><b>const</b> <a href="../contra/balance.md#contra_balance_EConsistencyProofFailed">EConsistencyProofFailed</a>: u64 = 0;
</code></pre>



<a name="contra_balance_EMismatchedTransferTotal"></a>

<code><a href="../contra/balance.md#contra_balance_try_split_batch">try_split_batch</a></code>: sender amounts don't sum to receiver amounts.


<pre><code><b>const</b> <a href="../contra/balance.md#contra_balance_EMismatchedTransferTotal">EMismatchedTransferTotal</a>: u64 = 1;
</code></pre>



<a name="contra_balance_EMismatchedBatchLength"></a>

<code><a href="../contra/balance.md#contra_balance_try_split_batch">try_split_batch</a></code>: sender and receiver vectors have different length.


<pre><code><b>const</b> <a href="../contra/balance.md#contra_balance_EMismatchedBatchLength">EMismatchedBatchLength</a>: u64 = 2;
</code></pre>



<a name="contra_balance_EInvalidPublicKey"></a>

A value carried into the balance was encrypted under a different key.


<pre><code><b>const</b> <a href="../contra/balance.md#contra_balance_EInvalidPublicKey">EInvalidPublicKey</a>: u64 = 3;
</code></pre>



<a name="contra_balance_zero"></a>

## Function `zero`

A <code><a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a></code> of zero value.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_zero">zero</a>&lt;T&gt;(): <a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_zero">zero</a>&lt;T&gt;(): <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt; {
    <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt; { <a href="../contra/balance.md#contra_balance_value">value</a>: 0 }
}
</code></pre>



</details>

<a name="contra_balance_wrap"></a>

## Function `wrap`

Wrap a <code>Coin&lt;T&gt;</code> into a <code><a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;</code>, sending the coin's funds to <code>pool</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_wrap">wrap</a>&lt;T&gt;(coin: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;, pool: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): <a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_wrap">wrap</a>&lt;T&gt;(coin: Coin&lt;T&gt;, pool: &UID): <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt; {
    <b>let</b> <a href="../contra/balance.md#contra_balance_value">value</a> = coin.<a href="../contra/balance.md#contra_balance_value">value</a>();
    send_funds(coin, pool.to_address());
    <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt; { <a href="../contra/balance.md#contra_balance_value">value</a> }
}
</code></pre>



</details>

<a name="contra_balance_unwrap"></a>

## Function `unwrap`

Unwrap a <code><a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;</code> back into a <code>Coin&lt;T&gt;</code>, withdrawing matching funds from <code>pool</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_unwrap">unwrap</a>&lt;T&gt;(coin: <a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;, pool: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_unwrap">unwrap</a>&lt;T&gt;(coin: <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;, pool: &<b>mut</b> UID, ctx: &<b>mut</b> TxContext): Coin&lt;T&gt; {
    <b>let</b> <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a> { <a href="../contra/balance.md#contra_balance_value">value</a> } = coin;
    redeem_funds(withdraw_funds_from_object&lt;T&gt;(pool, <a href="../contra/balance.md#contra_balance_value">value</a>), ctx)
}
</code></pre>



</details>

<a name="contra_balance_join"></a>

## Function `join`

Merge <code>other</code> into <code>self</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;, other: <a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_join">join</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;, other: <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;) {
    <b>let</b> <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a> { <a href="../contra/balance.md#contra_balance_value">value</a> } = other;
    self.<a href="../contra/balance.md#contra_balance_value">value</a> = self.<a href="../contra/balance.md#contra_balance_value">value</a> + <a href="../contra/balance.md#contra_balance_value">value</a>;
}
</code></pre>



</details>

<a name="contra_balance_take"></a>

## Function `take`

Move the value out of <code>self</code>, leaving it at zero.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_take">take</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;): <a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_take">take</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;): <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt; {
    <b>let</b> <a href="../contra/balance.md#contra_balance_value">value</a> = self.<a href="../contra/balance.md#contra_balance_value">value</a>;
    self.<a href="../contra/balance.md#contra_balance_value">value</a> = 0;
    <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt; { <a href="../contra/balance.md#contra_balance_value">value</a> }
}
</code></pre>



</details>

<a name="contra_balance_value"></a>

## Function `value`

The public value carried by <code>self</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_value">value</a>&lt;T&gt;(self: &<a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_value">value</a>&lt;T&gt;(self: &<a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;): u64 {
    self.<a href="../contra/balance.md#contra_balance_value">value</a>
}
</code></pre>



</details>

<a name="contra_balance_amount"></a>

## Function `amount`

The verified encrypted amount carried by <code>coin</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_amount">amount</a>&lt;T&gt;(coin: &<a href="../contra/balance.md#contra_balance_EncryptedCoin">contra::balance::EncryptedCoin</a>&lt;T&gt;): &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_amount">amount</a>&lt;T&gt;(coin: &<a href="../contra/balance.md#contra_balance_EncryptedCoin">EncryptedCoin</a>&lt;T&gt;): &WellFormedEncryptedAmount {
    &coin.<a href="../contra/balance.md#contra_balance_amount">amount</a>
}
</code></pre>



</details>

<a name="contra_balance_new"></a>

## Function `new`

An <code><a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a></code> encrypting zero with <code><a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> = 1</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_new">new</a>&lt;T&gt;(): <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_new">new</a>&lt;T&gt;(): <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt; {
    <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a> { <a href="../contra/balance.md#contra_balance_amount">amount</a>: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_zero">encrypted_amount::zero</a>(), <a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a>: 1 }
}
</code></pre>



</details>

<a name="contra_balance_empty"></a>

## Function `empty`

An <code><a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a></code> encrypting zero with <code><a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> = 0</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_empty">empty</a>&lt;T&gt;(): <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_empty">empty</a>&lt;T&gt;(): <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt; {
    <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a> { <a href="../contra/balance.md#contra_balance_amount">amount</a>: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_zero">encrypted_amount::zero</a>(), <a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a>: 0 }
}
</code></pre>



</details>

<a name="contra_balance_upper_bound"></a>

## Function `upper_bound`

The number of u16-bounded values folded into <code>self</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a>&lt;T&gt;(self: &<a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a>&lt;T&gt;(self: &<a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;): u16 {
    self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a>
}
</code></pre>



</details>

<a name="contra_balance_max_upper_bound"></a>

## Function `max_upper_bound`

The largest <code><a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a></code> that keeps a limb within the decryption window: each limb is
then bounded by <code>0xFFFF * 0xFFFF &lt; 2^32</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_max_upper_bound">max_upper_bound</a>(): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_max_upper_bound">max_upper_bound</a>(): u16 {
    0xFFFF
}
</code></pre>



</details>

<a name="contra_balance_max_upper_bound_minus_1"></a>

## Function `max_upper_bound_minus_1`

<code><a href="../contra/balance.md#contra_balance_max_upper_bound">max_upper_bound</a>() - 1</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_max_upper_bound_minus_1">max_upper_bound_minus_1</a>(): u16
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_max_upper_bound_minus_1">max_upper_bound_minus_1</a>(): u16 {
    0xFFFE
}
</code></pre>



</details>

<a name="contra_balance_is_empty"></a>

## Function `is_empty`

Whether <code>self</code> is in its post-construction state (nothing merged in).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_is_empty">is_empty</a>&lt;T&gt;(self: &<a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_is_empty">is_empty</a>&lt;T&gt;(self: &<a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;): bool {
    self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> == 0
}
</code></pre>



</details>

<a name="contra_balance_collapse"></a>

## Function `collapse`

Collapsed (single-<code>Encryption</code>) view of <code>self</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_collapse">collapse</a>&lt;T&gt;(self: &<a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_collapse">collapse</a>&lt;T&gt;(self: &<a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;): Encryption {
    self.<a href="../contra/balance.md#contra_balance_amount">amount</a>.<a href="../contra/balance.md#contra_balance_collapse">collapse</a>()
}
</code></pre>



</details>

<a name="contra_balance_merge_into"></a>

## Function `merge_into`

Fold <code>other</code> into <code>self</code>, leaving <code>other</code> at zero. Caller is responsible for ensuring both
sides are encrypted under the same key, and for any protocol-level cap on <code><a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_merge_into">merge_into</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;, other: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_merge_into">merge_into</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;, other: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;) {
    self.<a href="../contra/balance.md#contra_balance_amount">amount</a>.add_assign(&other.<a href="../contra/balance.md#contra_balance_amount">amount</a>);
    self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> = self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> + other.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a>;
    other.<a href="../contra/balance.md#contra_balance_set_empty">set_empty</a>();
}
</code></pre>



</details>

<a name="contra_balance_merge_encrypted"></a>

## Function `merge_encrypted`

Fold an <code><a href="../contra/balance.md#contra_balance_EncryptedCoin">EncryptedCoin</a></code> into <code>self</code>. Aborts if the coin's pk doesn't match the caller-supplied
<code>pk</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_merge_encrypted">merge_encrypted</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;, pk: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, coin: <a href="../contra/balance.md#contra_balance_EncryptedCoin">contra::balance::EncryptedCoin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_merge_encrypted">merge_encrypted</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;,
    pk: &Element&lt;G&gt;,
    coin: <a href="../contra/balance.md#contra_balance_EncryptedCoin">EncryptedCoin</a>&lt;T&gt;,
) {
    <b>let</b> <a href="../contra/balance.md#contra_balance_EncryptedCoin">EncryptedCoin</a> { <a href="../contra/balance.md#contra_balance_amount">amount</a> } = coin;
    <b>assert</b>!(<a href="../contra/balance.md#contra_balance_amount">amount</a>.pk() == pk, <a href="../contra/balance.md#contra_balance_EInvalidPublicKey">EInvalidPublicKey</a>);
    self.<a href="../contra/balance.md#contra_balance_amount">amount</a>.add_assign(<a href="../contra/balance.md#contra_balance_amount">amount</a>.<a href="../contra/balance.md#contra_balance_amount">amount</a>());
    self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> = self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> + 1;
}
</code></pre>



</details>

<a name="contra_balance_merge_public"></a>

## Function `merge_public`

Fold a <code><a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a></code> into <code>self</code>. Zero-valued coins are no-ops (no slot consumed).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_merge_public">merge_public</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;, coin: <a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_merge_public">merge_public</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;, coin: <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;) {
    <b>let</b> <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a> { <a href="../contra/balance.md#contra_balance_value">value</a> } = coin;
    <b>if</b> (<a href="../contra/balance.md#contra_balance_value">value</a> == 0) <b>return</b>;
    self.<a href="../contra/balance.md#contra_balance_amount">amount</a>.add_assign(&from_value(<a href="../contra/balance.md#contra_balance_value">value</a>));
    self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> = self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> + 1;
}
</code></pre>



</details>

<a name="contra_balance_try_split_to_public"></a>

## Function `try_split_to_public`

On a verifying <code>proof</code> that <code>self == new_balance + <a href="../contra/balance.md#contra_balance_value">value</a></code>, lower <code>self</code> to <code>new_balance</code> and
return <code><a href="../contra/balance.md#contra_balance_value">value</a></code> as a <code><a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a></code>; else return <code>none</code>. Aborts if <code>new_balance.pk() != sender_pk</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_try_split_to_public">try_split_to_public</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;, sender_pk: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>, <a href="../contra/balance.md#contra_balance_value">value</a>: u64, proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, dst: vector&lt;u8&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_try_split_to_public">try_split_to_public</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;,
    sender_pk: &Element&lt;G&gt;,
    new_balance: WellFormedEncryptedAmount,
    <a href="../contra/balance.md#contra_balance_value">value</a>: u64,
    proof: &DdhProof,
    dst: vector&lt;u8&gt;,
): Option&lt;<a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;&gt; {
    <b>assert</b>!(new_balance.pk() == sender_pk, <a href="../contra/balance.md#contra_balance_EInvalidPublicKey">EInvalidPublicKey</a>);
    <b>let</b> <b>mut</b> expected = self.<a href="../contra/balance.md#contra_balance_collapse">collapse</a>();
    expected.sub_assign_u64(<a href="../contra/balance.md#contra_balance_value">value</a>);
    <b>if</b> (new_balance.verify_equal(&expected, proof, dst)) {
        self.<a href="../contra/balance.md#contra_balance_overwrite">overwrite</a>(&new_balance);
        option::some(<a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt; { <a href="../contra/balance.md#contra_balance_value">value</a> })
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="contra_balance_try_split_batch"></a>

## Function `try_split_batch`

Split receiver-keyed coins off <code>self</code> for a batched transfer. Returns <code>some(coins)</code> on a
verifying balance proof, else <code>none</code>. Aborts if <code>new_balance.pk() != sender_pk</code>, the
sender/receiver vectors have different length, the sender total doesn't match the receiver
total, or the consistency proof fails.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_try_split_batch">try_split_batch</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;, sender_pk: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>, receiver_amounts: vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>&gt;, sender_amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>&gt;, consistency_proof: <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a>, consistency_dst: vector&lt;u8&gt;, balance_proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, balance_dst: vector&lt;u8&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../contra/balance.md#contra_balance_EncryptedCoin">contra::balance::EncryptedCoin</a>&lt;T&gt;&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_try_split_batch">try_split_batch</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;,
    sender_pk: &Element&lt;G&gt;,
    new_balance: WellFormedEncryptedAmount,
    receiver_amounts: vector&lt;WellFormedEncryptedAmount&gt;,
    sender_amounts: &vector&lt;EncryptedAmount&gt;,
    consistency_proof: ElGamalProof,
    consistency_dst: vector&lt;u8&gt;,
    balance_proof: &DdhProof,
    balance_dst: vector&lt;u8&gt;,
): Option&lt;vector&lt;<a href="../contra/balance.md#contra_balance_EncryptedCoin">EncryptedCoin</a>&lt;T&gt;&gt;&gt; {
    <b>assert</b>!(new_balance.pk() == sender_pk, <a href="../contra/balance.md#contra_balance_EInvalidPublicKey">EInvalidPublicKey</a>);
    <b>assert</b>!(sender_amounts.length() == receiver_amounts.length(), <a href="../contra/balance.md#contra_balance_EMismatchedBatchLength">EMismatchedBatchLength</a>);
    <b>let</b> total_sender = <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse_sum">encrypted_amount::collapse_sum</a>(sender_amounts);
    <b>assert</b>!(
        *total_sender.ciphertext() == sum_commitments(&receiver_amounts),
        <a href="../contra/balance.md#contra_balance_EMismatchedTransferTotal">EMismatchedTransferTotal</a>,
    );
    <b>assert</b>!(
        // Check that the total sender is a valid ElGamal encryption under the sender <b>public</b> key.
        consistency_proof.verify_elgamal(
            consistency_dst,
            sender_pk,
            &total_sender,
        ),
        <a href="../contra/balance.md#contra_balance_EConsistencyProofFailed">EConsistencyProofFailed</a>,
    );
    <b>let</b> <b>mut</b> expected = self.<a href="../contra/balance.md#contra_balance_collapse">collapse</a>();
    expected.sub_assign(&total_sender);
    <b>if</b> (new_balance.verify_equal(&expected, balance_proof, balance_dst)) {
        self.<a href="../contra/balance.md#contra_balance_overwrite">overwrite</a>(&new_balance);
        option::some(receiver_amounts.map!(|<a href="../contra/balance.md#contra_balance_amount">amount</a>| <a href="../contra/balance.md#contra_balance_EncryptedCoin">EncryptedCoin</a> { <a href="../contra/balance.md#contra_balance_amount">amount</a> }))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="contra_balance_try_update"></a>

## Function `try_update`

Re-state <code>self</code> as a verified re-encryption of the same value. Returns whether the proof
verified. Aborts if <code>new_balance.pk() != sender_pk</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_try_update">try_update</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;, sender_pk: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, new_balance: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>, proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, dst: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_try_update">try_update</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;,
    sender_pk: &Element&lt;G&gt;,
    new_balance: WellFormedEncryptedAmount,
    proof: &DdhProof,
    dst: vector&lt;u8&gt;,
): bool {
    <b>assert</b>!(new_balance.pk() == sender_pk, <a href="../contra/balance.md#contra_balance_EInvalidPublicKey">EInvalidPublicKey</a>);
    <b>if</b> (new_balance.verify_equal(&self.<a href="../contra/balance.md#contra_balance_collapse">collapse</a>(), proof, dst)) {
        self.<a href="../contra/balance.md#contra_balance_overwrite">overwrite</a>(&new_balance);
        <b>true</b>
    } <b>else</b> {
        <b>false</b>
    }
}
</code></pre>



</details>

<a name="contra_balance_try_set_public_key"></a>

## Function `try_set_public_key`

On a verifying <code>eq_proof</code> that <code>self</code> (under <code>old_pk</code>) and <code>new_balance</code> (under <code>new_pk</code>)
encrypt the same plaintext+blinding, overwrite <code>self</code> with <code>new_balance</code>. Returns whether the
proof verified. Aborts if <code>new_balance.pk() != new_pk</code>. The caller is responsible for updating
its own record of the active key.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_try_set_public_key">try_set_public_key</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;, old_pk: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, new_pk: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, new_balance: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>, eq_proof: <a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, dst: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_try_set_public_key">try_set_public_key</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;,
    old_pk: &Element&lt;G&gt;,
    new_pk: &Element&lt;G&gt;,
    new_balance: &WellFormedEncryptedAmount,
    eq_proof: DdhProof,
    dst: vector&lt;u8&gt;,
): bool {
    <b>assert</b>!(new_balance.pk() == new_pk, <a href="../contra/balance.md#contra_balance_EInvalidPublicKey">EInvalidPublicKey</a>);
    <b>let</b> new_collapse = new_balance.<a href="../contra/balance.md#contra_balance_amount">amount</a>().<a href="../contra/balance.md#contra_balance_collapse">collapse</a>();
    <b>let</b> old_collapse = self.<a href="../contra/balance.md#contra_balance_collapse">collapse</a>();
    <b>if</b> (
        new_collapse.ciphertext() == old_collapse.ciphertext() && eq_proof.verify_ddh(
            dst,
            old_pk,
            old_collapse.decryption_handle(),
            new_pk,
            new_collapse.decryption_handle(),
        )
    ) {
        self.<a href="../contra/balance.md#contra_balance_overwrite">overwrite</a>(new_balance);
        <b>true</b>
    } <b>else</b> {
        <b>false</b>
    }
}
</code></pre>



</details>

<a name="contra_balance_overwrite_unchecked"></a>

## Function `overwrite_unchecked`

Overwrite <code>self</code> with a raw <code>EncryptedAmount</code>. <code><a href="../contra/balance.md#contra_balance_new">new</a></code> is not range-checked; <code><a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a></code> is
set to 1.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_overwrite_unchecked">overwrite_unchecked</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;, _t: &<b>mut</b> <a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;T&gt;, <a href="../contra/balance.md#contra_balance_new">new</a>: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_overwrite_unchecked">overwrite_unchecked</a>&lt;T&gt;(
    self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;,
    _t: &<b>mut</b> TreasuryCap&lt;T&gt;,
    <a href="../contra/balance.md#contra_balance_new">new</a>: EncryptedAmount,
) {
    self.<a href="../contra/balance.md#contra_balance_amount">amount</a> = <a href="../contra/balance.md#contra_balance_new">new</a>;
    self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> = 1;
}
</code></pre>



</details>

<a name="contra_balance_clear_unchecked"></a>

## Function `clear_unchecked`

Reset <code>self</code> to zero without proof.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_clear_unchecked">clear_unchecked</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;, _t: &<b>mut</b> <a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_clear_unchecked">clear_unchecked</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;, _t: &<b>mut</b> TreasuryCap&lt;T&gt;) {
    self.<a href="../contra/balance.md#contra_balance_set_empty">set_empty</a>();
}
</code></pre>



</details>

<a name="contra_balance_set_zero_unchecked"></a>

## Function `set_zero_unchecked`

Reset a <code><a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a></code> to zero without proof.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_set_zero_unchecked">set_zero_unchecked</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_PublicCoin">contra::balance::PublicCoin</a>&lt;T&gt;, _t: &<b>mut</b> <a href="../myso/coin.md#myso_coin_TreasuryCap">myso::coin::TreasuryCap</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/balance.md#contra_balance_set_zero_unchecked">set_zero_unchecked</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_PublicCoin">PublicCoin</a>&lt;T&gt;, _t: &<b>mut</b> TreasuryCap&lt;T&gt;) {
    self.<a href="../contra/balance.md#contra_balance_value">value</a> = 0;
}
</code></pre>



</details>

<a name="contra_balance_overwrite"></a>

## Function `overwrite`

Overwrite <code>self</code> with the verified amount <code><a href="../contra/balance.md#contra_balance_new">new</a></code>.


<pre><code><b>fun</b> <a href="../contra/balance.md#contra_balance_overwrite">overwrite</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;, <a href="../contra/balance.md#contra_balance_new">new</a>: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/balance.md#contra_balance_overwrite">overwrite</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;, <a href="../contra/balance.md#contra_balance_new">new</a>: &WellFormedEncryptedAmount) {
    self.<a href="../contra/balance.md#contra_balance_amount">amount</a> = *<a href="../contra/balance.md#contra_balance_new">new</a>.<a href="../contra/balance.md#contra_balance_amount">amount</a>();
    self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> = 1;
}
</code></pre>



</details>

<a name="contra_balance_set_empty"></a>

## Function `set_empty`

Reset <code>self</code> to the same state <code><a href="../contra/balance.md#contra_balance_empty">empty</a>()</code> returns.


<pre><code><b>fun</b> <a href="../contra/balance.md#contra_balance_set_empty">set_empty</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">contra::balance::EncryptedBalance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/balance.md#contra_balance_set_empty">set_empty</a>&lt;T&gt;(self: &<b>mut</b> <a href="../contra/balance.md#contra_balance_EncryptedBalance">EncryptedBalance</a>&lt;T&gt;) {
    self.<a href="../contra/balance.md#contra_balance_amount">amount</a> = <a href="../contra/encrypted_amount.md#contra_encrypted_amount_zero">encrypted_amount::zero</a>();
    self.<a href="../contra/balance.md#contra_balance_upper_bound">upper_bound</a> = 0;
}
</code></pre>



</details>
