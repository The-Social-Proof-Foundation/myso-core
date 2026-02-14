---
title: Module `orderbook::account`
---

Account module manages the account data for each user.


-  [Struct `Account`](#orderbook_account_Account)
-  [Function `open_orders`](#orderbook_account_open_orders)
-  [Function `taker_volume`](#orderbook_account_taker_volume)
-  [Function `maker_volume`](#orderbook_account_maker_volume)
-  [Function `total_volume`](#orderbook_account_total_volume)
-  [Function `active_stake`](#orderbook_account_active_stake)
-  [Function `inactive_stake`](#orderbook_account_inactive_stake)
-  [Function `created_proposal`](#orderbook_account_created_proposal)
-  [Function `voted_proposal`](#orderbook_account_voted_proposal)
-  [Function `settled_balances`](#orderbook_account_settled_balances)
-  [Function `empty`](#orderbook_account_empty)
-  [Function `update`](#orderbook_account_update)
-  [Function `process_maker_fill`](#orderbook_account_process_maker_fill)
-  [Function `add_taker_volume`](#orderbook_account_add_taker_volume)
-  [Function `set_voted_proposal`](#orderbook_account_set_voted_proposal)
-  [Function `set_created_proposal`](#orderbook_account_set_created_proposal)
-  [Function `add_settled_balances`](#orderbook_account_add_settled_balances)
-  [Function `add_owed_balances`](#orderbook_account_add_owed_balances)
-  [Function `settle`](#orderbook_account_settle)
-  [Function `add_rebates`](#orderbook_account_add_rebates)
-  [Function `claim_rebates`](#orderbook_account_claim_rebates)
-  [Function `add_order`](#orderbook_account_add_order)
-  [Function `remove_order`](#orderbook_account_remove_order)
-  [Function `add_stake`](#orderbook_account_add_stake)
-  [Function `remove_stake`](#orderbook_account_remove_stake)


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
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../orderbook/balances.md#orderbook_balances">orderbook::balances</a>;
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/fill.md#orderbook_fill">orderbook::fill</a>;
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



<a name="orderbook_account_Account"></a>

## Struct `Account`

Account data that is updated every epoch.
One Account struct per BalanceManager object.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>epoch: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/account.md#orderbook_account_open_orders">open_orders</a>: <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;u128&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/account.md#orderbook_account_taker_volume">taker_volume</a>: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/account.md#orderbook_account_maker_volume">maker_volume</a>: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/account.md#orderbook_account_created_proposal">created_proposal</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/account.md#orderbook_account_voted_proposal">voted_proposal</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>unclaimed_rebates: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a></code>
</dt>
<dd>
</dd>
<dt>
<code>owed_balances: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_account_open_orders"></a>

## Function `open_orders`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_open_orders">open_orders</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): <a href="../myso/vec_set.md#myso_vec_set_VecSet">myso::vec_set::VecSet</a>&lt;u128&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_open_orders">open_orders</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">Account</a>): VecSet&lt;u128&gt; {
    self.<a href="../orderbook/account.md#orderbook_account_open_orders">open_orders</a>
}
</code></pre>



</details>

<a name="orderbook_account_taker_volume"></a>

## Function `taker_volume`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_taker_volume">taker_volume</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_taker_volume">taker_volume</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">Account</a>): u128 {
    self.<a href="../orderbook/account.md#orderbook_account_taker_volume">taker_volume</a>
}
</code></pre>



</details>

<a name="orderbook_account_maker_volume"></a>

## Function `maker_volume`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_maker_volume">maker_volume</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_maker_volume">maker_volume</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">Account</a>): u128 {
    self.<a href="../orderbook/account.md#orderbook_account_maker_volume">maker_volume</a>
}
</code></pre>



</details>

<a name="orderbook_account_total_volume"></a>

## Function `total_volume`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_total_volume">total_volume</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_total_volume">total_volume</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">Account</a>): u128 {
    self.<a href="../orderbook/account.md#orderbook_account_taker_volume">taker_volume</a> + self.<a href="../orderbook/account.md#orderbook_account_maker_volume">maker_volume</a>
}
</code></pre>



</details>

<a name="orderbook_account_active_stake"></a>

## Function `active_stake`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">Account</a>): u64 {
    self.<a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a>
}
</code></pre>



</details>

<a name="orderbook_account_inactive_stake"></a>

## Function `inactive_stake`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">Account</a>): u64 {
    self.<a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a>
}
</code></pre>



</details>

<a name="orderbook_account_created_proposal"></a>

## Function `created_proposal`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_created_proposal">created_proposal</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_created_proposal">created_proposal</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">Account</a>): bool {
    self.<a href="../orderbook/account.md#orderbook_account_created_proposal">created_proposal</a>
}
</code></pre>



</details>

<a name="orderbook_account_voted_proposal"></a>

## Function `voted_proposal`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_voted_proposal">voted_proposal</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_voted_proposal">voted_proposal</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">Account</a>): Option&lt;ID&gt; {
    self.<a href="../orderbook/account.md#orderbook_account_voted_proposal">voted_proposal</a>
}
</code></pre>



</details>

<a name="orderbook_account_settled_balances"></a>

## Function `settled_balances`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>(self: &<a href="../orderbook/account.md#orderbook_account_Account">Account</a>): Balances {
    self.<a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>
}
</code></pre>



</details>

<a name="orderbook_account_empty"></a>

## Function `empty`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_empty">empty</a>(ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_empty">empty</a>(ctx: &TxContext): <a href="../orderbook/account.md#orderbook_account_Account">Account</a> {
    <a href="../orderbook/account.md#orderbook_account_Account">Account</a> {
        epoch: ctx.epoch(),
        <a href="../orderbook/account.md#orderbook_account_open_orders">open_orders</a>: vec_set::empty(),
        <a href="../orderbook/account.md#orderbook_account_taker_volume">taker_volume</a>: 0,
        <a href="../orderbook/account.md#orderbook_account_maker_volume">maker_volume</a>: 0,
        <a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a>: 0,
        <a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a>: 0,
        <a href="../orderbook/account.md#orderbook_account_created_proposal">created_proposal</a>: <b>false</b>,
        <a href="../orderbook/account.md#orderbook_account_voted_proposal">voted_proposal</a>: option::none(),
        unclaimed_rebates: <a href="../orderbook/balances.md#orderbook_balances_empty">balances::empty</a>(),
        <a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>: <a href="../orderbook/balances.md#orderbook_balances_empty">balances::empty</a>(),
        owed_balances: <a href="../orderbook/balances.md#orderbook_balances_empty">balances::empty</a>(),
    }
}
</code></pre>



</details>

<a name="orderbook_account_update"></a>

## Function `update`

Update the account data for the new epoch.
Returns the previous epoch, maker volume, and active stake.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_update">update</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u64, u128, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_update">update</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, ctx: &TxContext): (u64, u128, u64) {
    <b>if</b> (self.epoch == ctx.epoch()) <b>return</b> (0, 0, 0);
    <b>let</b> prev_epoch = self.epoch;
    <b>let</b> prev_maker_volume = self.<a href="../orderbook/account.md#orderbook_account_maker_volume">maker_volume</a>;
    <b>let</b> prev_active_stake = self.<a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a>;
    self.epoch = ctx.epoch();
    self.<a href="../orderbook/account.md#orderbook_account_maker_volume">maker_volume</a> = 0;
    self.<a href="../orderbook/account.md#orderbook_account_taker_volume">taker_volume</a> = 0;
    self.<a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a> = self.<a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a> + self.<a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a>;
    self.<a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a> = 0;
    self.<a href="../orderbook/account.md#orderbook_account_created_proposal">created_proposal</a> = <b>false</b>;
    self.<a href="../orderbook/account.md#orderbook_account_voted_proposal">voted_proposal</a> = option::none();
    (prev_epoch, prev_maker_volume, prev_active_stake)
}
</code></pre>



</details>

<a name="orderbook_account_process_maker_fill"></a>

## Function `process_maker_fill`

Given a fill, update the account balances and volumes as the maker.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_process_maker_fill">process_maker_fill</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, <a href="../orderbook/fill.md#orderbook_fill">fill</a>: &<a href="../orderbook/fill.md#orderbook_fill_Fill">orderbook::fill::Fill</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_process_maker_fill">process_maker_fill</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, <a href="../orderbook/fill.md#orderbook_fill">fill</a>: &Fill) {
    <b>let</b> <a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a> = <a href="../orderbook/fill.md#orderbook_fill">fill</a>.get_settled_maker_quantities();
    self.<a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>.add_balances(<a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>);
    <b>if</b> (!<a href="../orderbook/fill.md#orderbook_fill">fill</a>.expired()) {
        self.<a href="../orderbook/account.md#orderbook_account_maker_volume">maker_volume</a> = self.<a href="../orderbook/account.md#orderbook_account_maker_volume">maker_volume</a> + (<a href="../orderbook/fill.md#orderbook_fill">fill</a>.base_quantity() <b>as</b> u128);
    };
    <b>if</b> (<a href="../orderbook/fill.md#orderbook_fill">fill</a>.expired() || <a href="../orderbook/fill.md#orderbook_fill">fill</a>.completed()) {
        self.<a href="../orderbook/account.md#orderbook_account_open_orders">open_orders</a>.remove(&<a href="../orderbook/fill.md#orderbook_fill">fill</a>.maker_order_id());
    }
}
</code></pre>



</details>

<a name="orderbook_account_add_taker_volume"></a>

## Function `add_taker_volume`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_taker_volume">add_taker_volume</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, volume: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_taker_volume">add_taker_volume</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, volume: u64) {
    self.<a href="../orderbook/account.md#orderbook_account_taker_volume">taker_volume</a> = self.<a href="../orderbook/account.md#orderbook_account_taker_volume">taker_volume</a> + (volume <b>as</b> u128);
}
</code></pre>



</details>

<a name="orderbook_account_set_voted_proposal"></a>

## Function `set_voted_proposal`

Set the voted proposal for the account and return the
previous proposal.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_set_voted_proposal">set_voted_proposal</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, proposal: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_set_voted_proposal">set_voted_proposal</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, proposal: Option&lt;ID&gt;): Option&lt;ID&gt; {
    <b>let</b> prev_proposal = self.<a href="../orderbook/account.md#orderbook_account_voted_proposal">voted_proposal</a>;
    self.<a href="../orderbook/account.md#orderbook_account_voted_proposal">voted_proposal</a> = proposal;
    prev_proposal
}
</code></pre>



</details>

<a name="orderbook_account_set_created_proposal"></a>

## Function `set_created_proposal`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_set_created_proposal">set_created_proposal</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, created: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_set_created_proposal">set_created_proposal</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, created: bool) {
    self.<a href="../orderbook/account.md#orderbook_account_created_proposal">created_proposal</a> = created;
}
</code></pre>



</details>

<a name="orderbook_account_add_settled_balances"></a>

## Function `add_settled_balances`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_settled_balances">add_settled_balances</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, <a href="../orderbook/balances.md#orderbook_balances">balances</a>: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_settled_balances">add_settled_balances</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, <a href="../orderbook/balances.md#orderbook_balances">balances</a>: Balances) {
    self.<a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>.add_balances(<a href="../orderbook/balances.md#orderbook_balances">balances</a>);
}
</code></pre>



</details>

<a name="orderbook_account_add_owed_balances"></a>

## Function `add_owed_balances`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_owed_balances">add_owed_balances</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, <a href="../orderbook/balances.md#orderbook_balances">balances</a>: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_owed_balances">add_owed_balances</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, <a href="../orderbook/balances.md#orderbook_balances">balances</a>: Balances) {
    self.owed_balances.add_balances(<a href="../orderbook/balances.md#orderbook_balances">balances</a>);
}
</code></pre>



</details>

<a name="orderbook_account_settle"></a>

## Function `settle`

Settle the account balances. Returns the settled and
owed balances by this account. Vault uses these values
to perform any necessary transfers.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_settle">settle</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): (<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_settle">settle</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>): (Balances, Balances) {
    <b>let</b> settled = self.<a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>.reset();
    <b>let</b> owed = self.owed_balances.reset();
    (settled, owed)
}
</code></pre>



</details>

<a name="orderbook_account_add_rebates"></a>

## Function `add_rebates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_rebates">add_rebates</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, rebates: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_rebates">add_rebates</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, rebates: Balances) {
    self.unclaimed_rebates.add_balances(rebates);
}
</code></pre>



</details>

<a name="orderbook_account_claim_rebates"></a>

## Function `claim_rebates`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_claim_rebates">claim_rebates</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_claim_rebates">claim_rebates</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>): Balances {
    <b>let</b> rebate_amount = self.unclaimed_rebates;
    self.<a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>.add_balances(self.unclaimed_rebates);
    self.unclaimed_rebates.reset();
    rebate_amount
}
</code></pre>



</details>

<a name="orderbook_account_add_order"></a>

## Function `add_order`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_order">add_order</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, order_id: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_order">add_order</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, order_id: u128) {
    self.<a href="../orderbook/account.md#orderbook_account_open_orders">open_orders</a>.insert(order_id);
}
</code></pre>



</details>

<a name="orderbook_account_remove_order"></a>

## Function `remove_order`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_remove_order">remove_order</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, order_id: u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_remove_order">remove_order</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, order_id: u128) {
    self.<a href="../orderbook/account.md#orderbook_account_open_orders">open_orders</a>.remove(&order_id)
}
</code></pre>



</details>

<a name="orderbook_account_add_stake"></a>

## Function `add_stake`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_stake">add_stake</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>, stake: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_add_stake">add_stake</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>, stake: u64): (u64, u64) {
    <b>let</b> stake_before = self.<a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a> + self.<a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a>;
    self.<a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a> = self.<a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a> + stake;
    self.owed_balances.add_myso(stake);
    (stake_before, self.<a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a> + self.<a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a>)
}
</code></pre>



</details>

<a name="orderbook_account_remove_stake"></a>

## Function `remove_stake`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_remove_stake">remove_stake</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">orderbook::account::Account</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/account.md#orderbook_account_remove_stake">remove_stake</a>(self: &<b>mut</b> <a href="../orderbook/account.md#orderbook_account_Account">Account</a>) {
    <b>let</b> stake_before = self.<a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a> + self.<a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a>;
    self.<a href="../orderbook/account.md#orderbook_account_active_stake">active_stake</a> = 0;
    self.<a href="../orderbook/account.md#orderbook_account_inactive_stake">inactive_stake</a> = 0;
    self.<a href="../orderbook/account.md#orderbook_account_voted_proposal">voted_proposal</a> = option::none();
    self.<a href="../orderbook/account.md#orderbook_account_settled_balances">settled_balances</a>.add_myso(stake_before);
}
</code></pre>



</details>
