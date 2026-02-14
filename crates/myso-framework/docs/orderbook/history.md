---
title: Module `orderbook::history`
---

History module tracks the volume data for the current epoch and past epochs.
It also tracks past trade params. Past maker fees are used to calculate
fills for old orders. The historic median is used to calculate rebates and
burns.


-  [Struct `Volumes`](#orderbook_history_Volumes)
-  [Struct `History`](#orderbook_history_History)
-  [Struct `EpochData`](#orderbook_history_EpochData)
-  [Constants](#@Constants_0)
-  [Function `empty`](#orderbook_history_empty)
-  [Function `update`](#orderbook_history_update)
-  [Function `reset_volumes`](#orderbook_history_reset_volumes)
-  [Function `calculate_rebate_amount`](#orderbook_history_calculate_rebate_amount)
-  [Function `update_historic_median`](#orderbook_history_update_historic_median)
-  [Function `add_volume`](#orderbook_history_add_volume)
-  [Function `balance_to_burn`](#orderbook_history_balance_to_burn)
-  [Function `reset_balance_to_burn`](#orderbook_history_reset_balance_to_burn)
-  [Function `historic_maker_fee`](#orderbook_history_historic_maker_fee)
-  [Function `add_total_fees_collected`](#orderbook_history_add_total_fees_collected)


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
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../orderbook/balances.md#orderbook_balances">orderbook::balances</a>;
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/math.md#orderbook_math">orderbook::math</a>;
<b>use</b> <a href="../orderbook/trade_params.md#orderbook_trade_params">orderbook::trade_params</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="orderbook_history_Volumes"></a>

## Struct `Volumes`

<code><a href="../orderbook/history.md#orderbook_history_Volumes">Volumes</a></code> represents volume data for a single epoch.
Using flashloans on a whitelisted pool, assuming 1_000_000 * 1_000_000_000
in volume per trade, at 1 trade per millisecond, the total volume can reach
1_000_000 * 1_000_000_000 * 1000 * 60 * 60 * 24 * 365 = 8.64e22 in one
epoch.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/history.md#orderbook_history_Volumes">Volumes</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>total_volume: u128</code>
</dt>
<dd>
</dd>
<dt>
<code>total_staked_volume: u128</code>
</dt>
<dd>
</dd>
<dt>
<code>total_fees_collected: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a></code>
</dt>
<dd>
</dd>
<dt>
<code>historic_median: u128</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_history_History"></a>

## Struct `History`

<code><a href="../orderbook/history.md#orderbook_history_History">History</a></code> represents the volume data for the current epoch and past epochs.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/history.md#orderbook_history_History">History</a> <b>has</b> store
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
<code>epoch_created: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>volumes: <a href="../orderbook/history.md#orderbook_history_Volumes">orderbook::history::Volumes</a></code>
</dt>
<dd>
</dd>
<dt>
<code>historic_volumes: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;u64, <a href="../orderbook/history.md#orderbook_history_Volumes">orderbook::history::Volumes</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_history_EpochData"></a>

## Struct `EpochData`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/history.md#orderbook_history_EpochData">EpochData</a> <b>has</b> <b>copy</b>, drop, store
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
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>total_volume: u128</code>
</dt>
<dd>
</dd>
<dt>
<code>total_staked_volume: u128</code>
</dt>
<dd>
</dd>
<dt>
<code>base_fees_collected: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>quote_fees_collected: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>myso_fees_collected: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>historic_median: u128</code>
</dt>
<dd>
</dd>
<dt>
<code>taker_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>maker_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>stake_required: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="orderbook_history_EHistoricVolumesNotFound"></a>



<pre><code><b>const</b> <a href="../orderbook/history.md#orderbook_history_EHistoricVolumesNotFound">EHistoricVolumesNotFound</a>: u64 = 0;
</code></pre>



<a name="orderbook_history_empty"></a>

## Function `empty`

Create a new <code><a href="../orderbook/history.md#orderbook_history_History">History</a></code> instance. Called once upon pool creation. A single
blank <code><a href="../orderbook/history.md#orderbook_history_Volumes">Volumes</a></code> instance is created and added to the historic_volumes table.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_empty">empty</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>, epoch_created: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_empty">empty</a>(
    <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: TradeParams,
    epoch_created: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../orderbook/history.md#orderbook_history_History">History</a> {
    <b>let</b> volumes = <a href="../orderbook/history.md#orderbook_history_Volumes">Volumes</a> {
        total_volume: 0,
        total_staked_volume: 0,
        total_fees_collected: <a href="../orderbook/balances.md#orderbook_balances_empty">balances::empty</a>(),
        historic_median: 0,
        <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>,
    };
    <b>let</b> <b>mut</b> <a href="../orderbook/history.md#orderbook_history">history</a> = <a href="../orderbook/history.md#orderbook_history_History">History</a> {
        epoch: ctx.epoch(),
        epoch_created,
        volumes,
        historic_volumes: table::new(ctx),
        <a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a>: 0,
    };
    <a href="../orderbook/history.md#orderbook_history">history</a>.historic_volumes.add(ctx.epoch(), volumes);
    <a href="../orderbook/history.md#orderbook_history">history</a>
}
</code></pre>



</details>

<a name="orderbook_history_update"></a>

## Function `update`

Update the epoch if it has changed. If there are accounts with rebates,
add the current epoch's volume data to the historic volumes.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_update">update</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>, <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_update">update</a>(
    self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">History</a>,
    <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: TradeParams,
    pool_id: ID,
    ctx: &TxContext,
) {
    <b>let</b> epoch = ctx.epoch();
    <b>if</b> (self.epoch == epoch) <b>return</b>;
    <b>if</b> (self.historic_volumes.contains(self.epoch)) {
        self.historic_volumes.remove(self.epoch);
    };
    self.<a href="../orderbook/history.md#orderbook_history_update_historic_median">update_historic_median</a>();
    self.historic_volumes.add(self.epoch, self.volumes);
    event::emit(<a href="../orderbook/history.md#orderbook_history_EpochData">EpochData</a> {
        epoch: self.epoch,
        pool_id,
        total_volume: self.volumes.total_volume,
        total_staked_volume: self.volumes.total_staked_volume,
        base_fees_collected: self.volumes.total_fees_collected.base(),
        quote_fees_collected: self.volumes.total_fees_collected.quote(),
        myso_fees_collected: self.volumes.total_fees_collected.myso(),
        historic_median: self.volumes.historic_median,
        taker_fee: <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.taker_fee(),
        maker_fee: <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.maker_fee(),
        stake_required: <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.stake_required(),
    });
    self.epoch = epoch;
    self.<a href="../orderbook/history.md#orderbook_history_reset_volumes">reset_volumes</a>(<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>);
    self.historic_volumes.add(self.epoch, self.volumes);
}
</code></pre>



</details>

<a name="orderbook_history_reset_volumes"></a>

## Function `reset_volumes`

Reset the current epoch's volume data.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_reset_volumes">reset_volumes</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>, <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: <a href="../orderbook/trade_params.md#orderbook_trade_params_TradeParams">orderbook::trade_params::TradeParams</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_reset_volumes">reset_volumes</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">History</a>, <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>: TradeParams) {
    event::emit(self.volumes);
    self.volumes =
        <a href="../orderbook/history.md#orderbook_history_Volumes">Volumes</a> {
            total_volume: 0,
            total_staked_volume: 0,
            total_fees_collected: <a href="../orderbook/balances.md#orderbook_balances_empty">balances::empty</a>(),
            historic_median: 0,
            <a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>,
        };
}
</code></pre>



</details>

<a name="orderbook_history_calculate_rebate_amount"></a>

## Function `calculate_rebate_amount`

Given the epoch's volume data and the account's volume data,
calculate and returns rebate amount, updates the burn amount.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_calculate_rebate_amount">calculate_rebate_amount</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>, prev_epoch: u64, maker_volume: u128, account_stake: u64): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_calculate_rebate_amount">calculate_rebate_amount</a>(
    self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">History</a>,
    prev_epoch: u64,
    maker_volume: u128,
    account_stake: u64,
): Balances {
    <b>assert</b>!(self.historic_volumes.contains(prev_epoch), <a href="../orderbook/history.md#orderbook_history_EHistoricVolumesNotFound">EHistoricVolumesNotFound</a>);
    <b>let</b> volumes = &<b>mut</b> self.historic_volumes[prev_epoch];
    <b>if</b> (volumes.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.stake_required() &gt; account_stake) {
        <b>return</b> <a href="../orderbook/balances.md#orderbook_balances_empty">balances::empty</a>()
    };
    <b>let</b> maker_volume = maker_volume <b>as</b> u128;
    <b>let</b> other_maker_liquidity = volumes.total_volume - maker_volume;
    <b>let</b> maker_rebate_percentage = <b>if</b> (volumes.historic_median &gt; 0) {
        <a href="../orderbook/constants.md#orderbook_constants_float_scaling_u128">constants::float_scaling_u128</a>() - <a href="../orderbook/constants.md#orderbook_constants_float_scaling_u128">constants::float_scaling_u128</a>().min(
            <a href="../orderbook/math.md#orderbook_math_div_u128">math::div_u128</a>(other_maker_liquidity, volumes.historic_median),
        )
    } <b>else</b> {
        0
    };
    <b>let</b> maker_rebate_percentage = maker_rebate_percentage <b>as</b> u64;
    <b>let</b> maker_volume_proportion = <b>if</b> (volumes.total_staked_volume &gt; 0) {
        (<a href="../orderbook/math.md#orderbook_math_div_u128">math::div_u128</a>(maker_volume, volumes.total_staked_volume)) <b>as</b> u64
    } <b>else</b> {
        0
    };
    <b>let</b> <b>mut</b> max_rebates = volumes.total_fees_collected;
    max_rebates.mul(maker_volume_proportion); // Maximum rebates possible
    <b>let</b> <b>mut</b> rebates = max_rebates;
    rebates.mul(maker_rebate_percentage); // Actual rebates
    <b>let</b> maker_burn = max_rebates.myso() - rebates.myso();
    self.<a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a> = self.<a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a> + maker_burn;
    rebates
}
</code></pre>



</details>

<a name="orderbook_history_update_historic_median"></a>

## Function `update_historic_median`

Updates the historic_median for past 28 epochs.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_update_historic_median">update_historic_median</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_update_historic_median">update_historic_median</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">History</a>) {
    <b>let</b> epochs_since_creation = self.epoch - self.epoch_created;
    <b>if</b> (epochs_since_creation &lt; <a href="../orderbook/constants.md#orderbook_constants_phase_out_epochs">constants::phase_out_epochs</a>()) {
        self.volumes.historic_median = <a href="../orderbook/constants.md#orderbook_constants_max_u128">constants::max_u128</a>();
        <b>return</b>
    };
    <b>let</b> <b>mut</b> median_vec = vector&lt;u128&gt;[];
    <b>let</b> <b>mut</b> i = self.epoch - <a href="../orderbook/constants.md#orderbook_constants_phase_out_epochs">constants::phase_out_epochs</a>();
    <b>while</b> (i &lt; self.epoch) {
        <b>if</b> (self.historic_volumes.contains(i)) {
            median_vec.push_back(self.historic_volumes[i].total_volume);
        } <b>else</b> {
            median_vec.push_back(0);
        };
        i = i + 1;
    };
    self.volumes.historic_median = <a href="../orderbook/math.md#orderbook_math_median">math::median</a>(median_vec);
}
</code></pre>



</details>

<a name="orderbook_history_add_volume"></a>

## Function `add_volume`

Add volume to the current epoch's volume data.
Increments the total volume and total staked volume.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_add_volume">add_volume</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>, maker_volume: u64, account_stake: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_add_volume">add_volume</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">History</a>, maker_volume: u64, account_stake: u64) {
    <b>if</b> (maker_volume == 0) <b>return</b>;
    <b>let</b> maker_volume = maker_volume <b>as</b> u128;
    self.volumes.total_volume = self.volumes.total_volume + maker_volume;
    <b>if</b> (account_stake &gt;= self.volumes.<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.stake_required()) {
        self.volumes.total_staked_volume = self.volumes.total_staked_volume + maker_volume;
    };
}
</code></pre>



</details>

<a name="orderbook_history_balance_to_burn"></a>

## Function `balance_to_burn`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a>(self: &<a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a>(self: &<a href="../orderbook/history.md#orderbook_history_History">History</a>): u64 {
    self.<a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a>
}
</code></pre>



</details>

<a name="orderbook_history_reset_balance_to_burn"></a>

## Function `reset_balance_to_burn`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_reset_balance_to_burn">reset_balance_to_burn</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_reset_balance_to_burn">reset_balance_to_burn</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">History</a>): u64 {
    <b>let</b> <a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a> = self.<a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a>;
    self.<a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a> = 0;
    <a href="../orderbook/history.md#orderbook_history_balance_to_burn">balance_to_burn</a>
}
</code></pre>



</details>

<a name="orderbook_history_historic_maker_fee"></a>

## Function `historic_maker_fee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_historic_maker_fee">historic_maker_fee</a>(self: &<a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>, epoch: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_historic_maker_fee">historic_maker_fee</a>(self: &<a href="../orderbook/history.md#orderbook_history_History">History</a>, epoch: u64): u64 {
    <b>assert</b>!(self.historic_volumes.contains(epoch), <a href="../orderbook/history.md#orderbook_history_EHistoricVolumesNotFound">EHistoricVolumesNotFound</a>);
    self.historic_volumes[epoch].<a href="../orderbook/trade_params.md#orderbook_trade_params">trade_params</a>.maker_fee()
}
</code></pre>



</details>

<a name="orderbook_history_add_total_fees_collected"></a>

## Function `add_total_fees_collected`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_add_total_fees_collected">add_total_fees_collected</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">orderbook::history::History</a>, fees: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/history.md#orderbook_history_add_total_fees_collected">add_total_fees_collected</a>(self: &<b>mut</b> <a href="../orderbook/history.md#orderbook_history_History">History</a>, fees: Balances) {
    self.volumes.total_fees_collected.add_balances(fees);
}
</code></pre>



</details>
