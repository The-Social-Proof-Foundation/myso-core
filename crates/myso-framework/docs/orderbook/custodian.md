---
title: Module `orderbook::custodian`
---



-  [Struct `Account`](#orderbook_custodian_Account)
-  [Struct `AccountCap`](#orderbook_custodian_AccountCap)
-  [Struct `Custodian`](#orderbook_custodian_Custodian)
-  [Function `mint_account_cap`](#orderbook_custodian_mint_account_cap)
-  [Function `account_balance`](#orderbook_custodian_account_balance)
-  [Function `new`](#orderbook_custodian_new)
-  [Function `withdraw_asset`](#orderbook_custodian_withdraw_asset)
-  [Function `increase_user_available_balance`](#orderbook_custodian_increase_user_available_balance)
-  [Function `decrease_user_available_balance`](#orderbook_custodian_decrease_user_available_balance)
-  [Function `increase_user_locked_balance`](#orderbook_custodian_increase_user_locked_balance)
-  [Function `decrease_user_locked_balance`](#orderbook_custodian_decrease_user_locked_balance)
-  [Function `lock_balance`](#orderbook_custodian_lock_balance)
-  [Function `unlock_balance`](#orderbook_custodian_unlock_balance)
-  [Function `account_available_balance`](#orderbook_custodian_account_available_balance)
-  [Function `account_locked_balance`](#orderbook_custodian_account_locked_balance)
-  [Function `borrow_mut_account_balance`](#orderbook_custodian_borrow_mut_account_balance)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
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



<a name="orderbook_custodian_Account"></a>

## Struct `Account`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/custodian.md#orderbook_custodian_Account">Account</a>&lt;<b>phantom</b> T&gt; <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>available_balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>locked_balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;T&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_custodian_AccountCap"></a>

## Struct `AccountCap`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">AccountCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_custodian_Custodian"></a>

## Struct `Custodian`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;<b>phantom</b> T&gt; <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>account_balances: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../orderbook/custodian.md#orderbook_custodian_Account">orderbook::custodian::Account</a>&lt;T&gt;&gt;</code>
</dt>
<dd>
 Map from an AccountCap object ID to an Account object
</dd>
</dl>


</details>

<a name="orderbook_custodian_mint_account_cap"></a>

## Function `mint_account_cap`

Create an <code><a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">AccountCap</a></code> that can be used across all DeepBook pool


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_mint_account_cap">mint_account_cap</a>(_ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">orderbook::custodian::AccountCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_mint_account_cap">mint_account_cap</a>(_ctx: &<b>mut</b> TxContext): <a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">AccountCap</a> {
    <b>abort</b> 1337
}
</code></pre>



</details>

<a name="orderbook_custodian_account_balance"></a>

## Function `account_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_account_balance">account_balance</a>&lt;Asset&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;Asset&gt;, user: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_account_balance">account_balance</a>&lt;Asset&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;Asset&gt;,
    user: ID
): (u64, u64) {
    // <b>if</b> <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a> account is not created yet, directly <b>return</b> (0, 0) rather than <b>abort</b>
    <b>if</b> (!table::contains(&<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>.account_balances, user)) {
        <b>return</b> (0, 0)
    };
    <b>let</b> account_balances = table::borrow(&<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>.account_balances, user);
    <b>let</b> avail_balance = balance::value(&account_balances.available_balance);
    <b>let</b> locked_balance = balance::value(&account_balances.locked_balance);
    (avail_balance, locked_balance)
}
</code></pre>



</details>

<a name="orderbook_custodian_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_new">new</a>&lt;T&gt;(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_new">new</a>&lt;T&gt;(ctx: &<b>mut</b> TxContext): <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt; {
    <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt; {
        id: object::new(ctx),
        account_balances: table::new(ctx),
    }
}
</code></pre>



</details>

<a name="orderbook_custodian_withdraw_asset"></a>

## Function `withdraw_asset`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_withdraw_asset">withdraw_asset</a>&lt;Asset&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;Asset&gt;, quantity: u64, account_cap: &<a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">orderbook::custodian::AccountCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;Asset&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_withdraw_asset">withdraw_asset</a>&lt;Asset&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;Asset&gt;,
    quantity: u64,
    account_cap: &<a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">AccountCap</a>,
    ctx: &<b>mut</b> TxContext
): Coin&lt;Asset&gt; {
    coin::from_balance(<a href="../orderbook/custodian.md#orderbook_custodian_decrease_user_available_balance">decrease_user_available_balance</a>&lt;Asset&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>, account_cap, quantity), ctx)
}
</code></pre>



</details>

<a name="orderbook_custodian_increase_user_available_balance"></a>

## Function `increase_user_available_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_increase_user_available_balance">increase_user_available_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;T&gt;, user: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, quantity: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_increase_user_available_balance">increase_user_available_balance</a>&lt;T&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt;,
    user: ID,
    quantity: Balance&lt;T&gt;,
) {
    <b>let</b> account = <a href="../orderbook/custodian.md#orderbook_custodian_borrow_mut_account_balance">borrow_mut_account_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>, user);
    balance::join(&<b>mut</b> account.available_balance, quantity);
}
</code></pre>



</details>

<a name="orderbook_custodian_decrease_user_available_balance"></a>

## Function `decrease_user_available_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_decrease_user_available_balance">decrease_user_available_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;T&gt;, account_cap: &<a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">orderbook::custodian::AccountCap</a>, quantity: u64): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_decrease_user_available_balance">decrease_user_available_balance</a>&lt;T&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt;,
    account_cap: &<a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">AccountCap</a>,
    quantity: u64,
): Balance&lt;T&gt; {
    <b>let</b> account = <a href="../orderbook/custodian.md#orderbook_custodian_borrow_mut_account_balance">borrow_mut_account_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>, object::uid_to_inner(&account_cap.id));
    balance::split(&<b>mut</b> account.available_balance, quantity)
}
</code></pre>



</details>

<a name="orderbook_custodian_increase_user_locked_balance"></a>

## Function `increase_user_locked_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_increase_user_locked_balance">increase_user_locked_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;T&gt;, account_cap: &<a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">orderbook::custodian::AccountCap</a>, quantity: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;T&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_increase_user_locked_balance">increase_user_locked_balance</a>&lt;T&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt;,
    account_cap: &<a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">AccountCap</a>,
    quantity: Balance&lt;T&gt;,
) {
    <b>let</b> account = <a href="../orderbook/custodian.md#orderbook_custodian_borrow_mut_account_balance">borrow_mut_account_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>, object::uid_to_inner(&account_cap.id));
    balance::join(&<b>mut</b> account.locked_balance, quantity);
}
</code></pre>



</details>

<a name="orderbook_custodian_decrease_user_locked_balance"></a>

## Function `decrease_user_locked_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_decrease_user_locked_balance">decrease_user_locked_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;T&gt;, user: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, quantity: u64): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_decrease_user_locked_balance">decrease_user_locked_balance</a>&lt;T&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt;,
    user: ID,
    quantity: u64,
): Balance&lt;T&gt; {
    <b>let</b> account = <a href="../orderbook/custodian.md#orderbook_custodian_borrow_mut_account_balance">borrow_mut_account_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>, user);
    split(&<b>mut</b> account.locked_balance, quantity)
}
</code></pre>



</details>

<a name="orderbook_custodian_lock_balance"></a>

## Function `lock_balance`

Move <code>quantity</code> from the unlocked balance of <code>user</code> to the locked balance of <code>user</code>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_lock_balance">lock_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;T&gt;, account_cap: &<a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">orderbook::custodian::AccountCap</a>, quantity: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_lock_balance">lock_balance</a>&lt;T&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt;,
    account_cap: &<a href="../orderbook/custodian.md#orderbook_custodian_AccountCap">AccountCap</a>,
    quantity: u64,
) {
    <b>let</b> to_lock = <a href="../orderbook/custodian.md#orderbook_custodian_decrease_user_available_balance">decrease_user_available_balance</a>(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>, account_cap, quantity);
    <a href="../orderbook/custodian.md#orderbook_custodian_increase_user_locked_balance">increase_user_locked_balance</a>(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>, account_cap, to_lock);
}
</code></pre>



</details>

<a name="orderbook_custodian_unlock_balance"></a>

## Function `unlock_balance`

Move <code>quantity</code> from the locked balance of <code>user</code> to the unlocked balance of <code>user</code>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_unlock_balance">unlock_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;T&gt;, user: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, quantity: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_unlock_balance">unlock_balance</a>&lt;T&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt;,
    user: ID,
    quantity: u64,
) {
    <b>let</b> locked_balance = <a href="../orderbook/custodian.md#orderbook_custodian_decrease_user_locked_balance">decrease_user_locked_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>, user, quantity);
    <a href="../orderbook/custodian.md#orderbook_custodian_increase_user_available_balance">increase_user_available_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>, user, locked_balance)
}
</code></pre>



</details>

<a name="orderbook_custodian_account_available_balance"></a>

## Function `account_available_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_account_available_balance">account_available_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;T&gt;, user: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_account_available_balance">account_available_balance</a>&lt;T&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt;,
    user: ID,
): u64 {
    balance::value(&table::borrow(&<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>.account_balances, user).available_balance)
}
</code></pre>



</details>

<a name="orderbook_custodian_account_locked_balance"></a>

## Function `account_locked_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_account_locked_balance">account_locked_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;T&gt;, user: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_account_locked_balance">account_locked_balance</a>&lt;T&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt;,
    user: ID,
): u64 {
    balance::value(&table::borrow(&<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>.account_balances, user).locked_balance)
}
</code></pre>



</details>

<a name="orderbook_custodian_borrow_mut_account_balance"></a>

## Function `borrow_mut_account_balance`



<pre><code><b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_borrow_mut_account_balance">borrow_mut_account_balance</a>&lt;T&gt;(<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">orderbook::custodian::Custodian</a>&lt;T&gt;, user: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Account">orderbook::custodian::Account</a>&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/custodian.md#orderbook_custodian_borrow_mut_account_balance">borrow_mut_account_balance</a>&lt;T&gt;(
    <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>: &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Custodian">Custodian</a>&lt;T&gt;,
    user: ID,
): &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian_Account">Account</a>&lt;T&gt; {
    <b>if</b> (!table::contains(&<a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>.account_balances, user)) {
        table::add(
            &<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>.account_balances,
            user,
            <a href="../orderbook/custodian.md#orderbook_custodian_Account">Account</a> { available_balance: balance::zero(), locked_balance: balance::zero() }
        );
    };
    table::borrow_mut(&<b>mut</b> <a href="../orderbook/custodian.md#orderbook_custodian">custodian</a>.account_balances, user)
}
</code></pre>



</details>
