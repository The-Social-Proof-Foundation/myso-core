---
title: Module `myso::myso`
---

Coin<MYSO> is the token used to pay for gas in MySo.
It has 9 decimals, and the smallest unit (10^-9) is called "mist".


-  [Struct `MYSO`](#myso_myso_MYSO)
-  [Constants](#@Constants_0)
-  [Function `new`](#myso_myso_new)
-  [Function `transfer`](#myso_myso_transfer)


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



<a name="myso_myso_MYSO"></a>

## Struct `MYSO`

Name of the coin


<pre><code><b>public</b> <b>struct</b> <a href="../myso/sui.md#myso_myso_MYSO">MYSO</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_myso_EAlreadyMinted"></a>



<pre><code><b>const</b> <a href="../myso/sui.md#myso_myso_EAlreadyMinted">EAlreadyMinted</a>: u64 = 0;
</code></pre>



<a name="myso_myso_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../myso/sui.md#myso_myso_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="myso_myso_MIST_PER_MYSO"></a>

The amount of Mist per MySo token based on the fact that mist is
10^-9 of a MySo token


<pre><code><b>const</b> <a href="../myso/sui.md#myso_myso_MIST_PER_MYSO">MIST_PER_MYSO</a>: u64 = 1000000000;
</code></pre>



<a name="myso_myso_TOTAL_SUPPLY_MYSO"></a>

The total supply of MySo denominated in whole MySo tokens (10 Billion)


<pre><code><b>const</b> <a href="../myso/sui.md#myso_myso_TOTAL_SUPPLY_MYSO">TOTAL_SUPPLY_MYSO</a>: u64 = 10000000000;
</code></pre>



<a name="myso_myso_TOTAL_SUPPLY_MIST"></a>

The total supply of MySo denominated in Mist (10 Billion * 10^9)


<pre><code><b>const</b> <a href="../myso/sui.md#myso_myso_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>: u64 = 10000000000000000000;
</code></pre>



<a name="myso_myso_new"></a>

## Function `new`

Register the <code><a href="../myso/sui.md#myso_myso_MYSO">MYSO</a></code> Coin to acquire its <code>Supply</code>.
This should be called only once during genesis creation.


<pre><code><b>fun</b> <a href="../myso/sui.md#myso_myso_new">new</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso/sui.md#myso_myso_new">new</a>(ctx: &<b>mut</b> TxContext): Balance&lt;<a href="../myso/sui.md#myso_myso_MYSO">MYSO</a>&gt; {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../myso/sui.md#myso_myso_ENotSystemAddress">ENotSystemAddress</a>);
    <b>assert</b>!(ctx.epoch() == 0, <a href="../myso/sui.md#myso_myso_EAlreadyMinted">EAlreadyMinted</a>);
    <b>let</b> (treasury, metadata) = <a href="../myso/coin.md#myso_coin_create_currency">coin::create_currency</a>(
        <a href="../myso/sui.md#myso_myso_MYSO">MYSO</a> {},
        9,
        b"<a href="../myso/sui.md#myso_myso_MYSO">MYSO</a>",
        b"MySo",
        // TODO: add appropriate description and logo <a href="../myso/url.md#myso_url">url</a>
        b"",
        option::none(),
        ctx,
    );
    <a href="../myso/transfer.md#myso_transfer_public_freeze_object">transfer::public_freeze_object</a>(metadata);
    <b>let</b> <b>mut</b> supply = treasury.treasury_into_supply();
    <b>let</b> total_myso = supply.increase_supply(<a href="../myso/sui.md#myso_myso_TOTAL_SUPPLY_MIST">TOTAL_SUPPLY_MIST</a>);
    supply.destroy_supply();
    total_myso
}
</code></pre>



</details>

<a name="myso_myso_transfer"></a>

## Function `transfer`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/transfer.md#myso_transfer">transfer</a>(c: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, recipient: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../myso/transfer.md#myso_transfer">transfer</a>(c: <a href="../myso/coin.md#myso_coin_Coin">coin::Coin</a>&lt;<a href="../myso/sui.md#myso_myso_MYSO">MYSO</a>&gt;, recipient: <b>address</b>) {
    <a href="../myso/transfer.md#myso_transfer_public_transfer">transfer::public_transfer</a>(c, recipient)
}
</code></pre>



</details>
