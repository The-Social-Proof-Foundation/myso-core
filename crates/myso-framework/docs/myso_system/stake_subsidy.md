---
title: Module `myso_system::stake_subsidy`
---



-  [Struct `StakeSubsidy`](#myso_system_stake_subsidy_StakeSubsidy)
-  [Constants](#@Constants_0)
-  [Function `create`](#myso_system_stake_subsidy_create)
-  [Function `advance_epoch`](#myso_system_stake_subsidy_advance_epoch)
-  [Function `current_epoch_subsidy_amount`](#myso_system_stake_subsidy_current_epoch_subsidy_amount)
-  [Function `calculate_effective_apy`](#myso_system_stake_subsidy_calculate_effective_apy)
-  [Function `calculate_epoch_subsidy_amount`](#myso_system_stake_subsidy_calculate_epoch_subsidy_amount)
-  [Function `get_distribution_counter`](#myso_system_stake_subsidy_get_distribution_counter)


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



<a name="myso_system_stake_subsidy_StakeSubsidy"></a>

## Struct `StakeSubsidy`



<pre><code><b>public</b> <b>struct</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
 Balance of MYSO set aside for stake subsidies that will be drawn down over time.
</dd>
<dt>
<code>distribution_counter: u64</code>
</dt>
<dd>
 Count of the number of times stake subsidies have been distributed.
</dd>
<dt>
<code>current_apy_bps: u64</code>
</dt>
<dd>
 The current APY (in basis points) used to compute stake subsidies.
 This amount decays and decreases over time.
</dd>
<dt>
<code>stake_subsidy_period_length: u64</code>
</dt>
<dd>
 Number of distributions to occur before the APY decays.
</dd>
<dt>
<code>stake_subsidy_decrease_rate: u16</code>
</dt>
<dd>
 The rate at which the APY decays at the end of each
 period. Expressed in basis points.
</dd>
<dt>
<code>max_apy_bps: u64</code>
</dt>
<dd>
 Maximum APY cap (in basis points). Effective APY will never exceed this.
</dd>
<dt>
<code>min_apy_bps: u64</code>
</dt>
<dd>
 Minimum APY floor (in basis points). Effective APY will never go below this.
</dd>
<dt>
<code>intended_duration_years: u64</code>
</dt>
<dd>
 Target duration for subsidy pool in years (e.g., 10).
 Used to calculate stake-aware APY reduction to ensure pool sustainability.
</dd>
<dt>
<code>extra_fields: <a href="../myso/bag.md#myso_bag_Bag">myso::bag::Bag</a></code>
</dt>
<dd>
 Any extra fields that's not defined statically.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_system_stake_subsidy_ESubsidyDecreaseRateTooLarge"></a>



<pre><code><b>const</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>: u64 = 0;
</code></pre>



<a name="myso_system_stake_subsidy_ESubsidyInitialApyTooLarge"></a>



<pre><code><b>const</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_ESubsidyInitialApyTooLarge">ESubsidyInitialApyTooLarge</a>: u64 = 1;
</code></pre>



<a name="myso_system_stake_subsidy_ESubsidyMinApyGreaterThanMax"></a>



<pre><code><b>const</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_ESubsidyMinApyGreaterThanMax">ESubsidyMinApyGreaterThanMax</a>: u64 = 2;
</code></pre>



<a name="myso_system_stake_subsidy_ESubsidyMaxApyTooLarge"></a>



<pre><code><b>const</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_ESubsidyMaxApyTooLarge">ESubsidyMaxApyTooLarge</a>: u64 = 3;
</code></pre>



<a name="myso_system_stake_subsidy_ESubsidyIntendedDurationZero"></a>



<pre><code><b>const</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_ESubsidyIntendedDurationZero">ESubsidyIntendedDurationZero</a>: u64 = 4;
</code></pre>



<a name="myso_system_stake_subsidy_BASIS_POINT_DENOMINATOR"></a>



<pre><code><b>const</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>: u128 = 10000;
</code></pre>



<a name="myso_system_stake_subsidy_YEAR_IN_MS"></a>



<pre><code><b>const</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_YEAR_IN_MS">YEAR_IN_MS</a>: u64 = 31536000000;
</code></pre>



<a name="myso_system_stake_subsidy_create"></a>

## Function `create`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_create">create</a>(balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, initial_apy_bps: u64, stake_subsidy_period_length: u64, stake_subsidy_decrease_rate: u16, max_apy_bps: u64, min_apy_bps: u64, intended_duration_years: u64, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">myso_system::stake_subsidy::StakeSubsidy</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_create">create</a>(
    balance: Balance&lt;MYSO&gt;,
    initial_apy_bps: u64,
    stake_subsidy_period_length: u64,
    stake_subsidy_decrease_rate: u16,
    max_apy_bps: u64,
    min_apy_bps: u64,
    intended_duration_years: u64,
    ctx: &<b>mut</b> TxContext,
): <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a> {
    // Rate can't be higher than 100%.
    <b>assert</b>!(
        stake_subsidy_decrease_rate &lt;= <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a> <b>as</b> u16,
        <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_ESubsidyDecreaseRateTooLarge">ESubsidyDecreaseRateTooLarge</a>,
    );
    <b>assert</b>!(
        initial_apy_bps &lt;= <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a> <b>as</b> u64,
        <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_ESubsidyInitialApyTooLarge">ESubsidyInitialApyTooLarge</a>,
    );
    // Max APY can't be higher than 100%.
    <b>assert</b>!(
        max_apy_bps &lt;= <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a> <b>as</b> u64,
        <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_ESubsidyMaxApyTooLarge">ESubsidyMaxApyTooLarge</a>,
    );
    // Min APY must be less than or equal to max APY.
    <b>assert</b>!(
        min_apy_bps &lt;= max_apy_bps,
        <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_ESubsidyMinApyGreaterThanMax">ESubsidyMinApyGreaterThanMax</a>,
    );
    // Intended duration must be greater than zero.
    <b>assert</b>!(
        intended_duration_years &gt; 0,
        <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_ESubsidyIntendedDurationZero">ESubsidyIntendedDurationZero</a>,
    );
    <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a> {
        balance,
        distribution_counter: 0,
        current_apy_bps: initial_apy_bps,
        stake_subsidy_period_length,
        stake_subsidy_decrease_rate,
        max_apy_bps,
        min_apy_bps,
        intended_duration_years,
        extra_fields: bag::new(ctx),
    }
}
</code></pre>



</details>

<a name="myso_system_stake_subsidy_advance_epoch"></a>

## Function `advance_epoch`

Advance the epoch counter and draw down the subsidy for the epoch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_advance_epoch">advance_epoch</a>(self: &<b>mut</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">myso_system::stake_subsidy::StakeSubsidy</a>, total_staked_mist: u64, epoch_duration_ms: u64): <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_advance_epoch">advance_epoch</a>(
    self: &<b>mut</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>,
    total_staked_mist: u64,
    epoch_duration_ms: u64,
): Balance&lt;MYSO&gt; {
    <b>let</b> epoch_subsidy_amount = <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_calculate_epoch_subsidy_amount">calculate_epoch_subsidy_amount</a>(
        self,
        total_staked_mist,
        epoch_duration_ms,
    );
    // Take the minimum of the reward amount and the remaining balance in
    // order to ensure we don't overdraft the remaining stake subsidy
    // balance
    <b>let</b> to_withdraw = epoch_subsidy_amount.min(self.balance.value());
    // Drawn down the subsidy <b>for</b> this epoch.
    <b>let</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy">stake_subsidy</a> = self.balance.split(to_withdraw);
    self.distribution_counter = self.distribution_counter + 1;
    // Decrease the subsidy amount only when the current period ends.
    <b>if</b> (self.distribution_counter % self.stake_subsidy_period_length == 0) {
        <b>let</b> decrease_amount = self.current_apy_bps <b>as</b> u128
            * (self.stake_subsidy_decrease_rate <b>as</b> u128) / <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
        self.current_apy_bps = self.current_apy_bps - (decrease_amount <b>as</b> u64)
    };
    <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy">stake_subsidy</a>
}
</code></pre>



</details>

<a name="myso_system_stake_subsidy_current_epoch_subsidy_amount"></a>

## Function `current_epoch_subsidy_amount`

Returns the amount of stake subsidy to be added at the end of the current epoch.


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_current_epoch_subsidy_amount">current_epoch_subsidy_amount</a>(self: &<a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">myso_system::stake_subsidy::StakeSubsidy</a>, total_staked_mist: u64, epoch_duration_ms: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_current_epoch_subsidy_amount">current_epoch_subsidy_amount</a>(
    self: &<a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>,
    total_staked_mist: u64,
    epoch_duration_ms: u64,
): u64 {
    <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_calculate_epoch_subsidy_amount">calculate_epoch_subsidy_amount</a>(
        self,
        total_staked_mist,
        epoch_duration_ms,
    ).min(self.balance.value())
}
</code></pre>



</details>

<a name="myso_system_stake_subsidy_calculate_effective_apy"></a>

## Function `calculate_effective_apy`

Calculate the effective APY considering stake-aware constraints and caps.
This ensures the subsidy pool lasts the intended duration while respecting min/max bounds.


<pre><code><b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_calculate_effective_apy">calculate_effective_apy</a>(self: &<a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">myso_system::stake_subsidy::StakeSubsidy</a>, total_staked_mist: u64, epoch_duration_ms: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_calculate_effective_apy">calculate_effective_apy</a>(
    self: &<a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>,
    total_staked_mist: u64,
    epoch_duration_ms: u64,
): u64 {
    // Start with the decayed target APY
    <b>let</b> target_apy_bps = self.current_apy_bps;
    // If no stake or zero epoch duration, <b>return</b> 0
    <b>if</b> (total_staked_mist == 0 || epoch_duration_ms == 0) {
        <b>return</b> 0
    };
    // Calculate projected yearly consumption at current stake and target APY
    <b>let</b> projected_yearly_consumption = (total_staked_mist <b>as</b> u128)
        * (target_apy_bps <b>as</b> u128)
        / <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
    // If projected consumption is zero, <b>return</b> 0
    <b>if</b> (projected_yearly_consumption == 0) {
        <b>return</b> 0
    };
    // Calculate remaining years at current rate
    <b>let</b> remaining_balance = self.balance.value() <b>as</b> u128;
    <b>let</b> remaining_years_scaled = <b>if</b> (projected_yearly_consumption &gt; 0 && remaining_balance &gt; 0) {
        <b>let</b> numerator = remaining_balance * <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
        numerator / projected_yearly_consumption
    } <b>else</b> {
        0
    };
    <b>let</b> intended_duration_years_scaled = (self.intended_duration_years <b>as</b> u128) * <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
    // Calculate effective APY with stake-aware reduction
    <b>let</b> effective_apy_bps = <b>if</b> (remaining_years_scaled &gt; 0 && remaining_years_scaled &lt; intended_duration_years_scaled) {
        <b>let</b> scaled_apy = (target_apy_bps <b>as</b> u128) * remaining_years_scaled;
        scaled_apy / intended_duration_years_scaled
    } <b>else</b> <b>if</b> (remaining_years_scaled &gt;= intended_duration_years_scaled) {
        target_apy_bps <b>as</b> u128
    } <b>else</b> {
        self.min_apy_bps <b>as</b> u128
    };
    // Apply min/max caps
    <b>let</b> capped_apy = <b>if</b> (effective_apy_bps &lt; self.min_apy_bps <b>as</b> u128) {
        self.min_apy_bps <b>as</b> u128
    } <b>else</b> <b>if</b> (effective_apy_bps &gt; self.max_apy_bps <b>as</b> u128) {
        self.max_apy_bps <b>as</b> u128
    } <b>else</b> {
        effective_apy_bps
    };
    capped_apy <b>as</b> u64
}
</code></pre>



</details>

<a name="myso_system_stake_subsidy_calculate_epoch_subsidy_amount"></a>

## Function `calculate_epoch_subsidy_amount`

Calculate the epoch subsidy amount using stake-aware APY calculation.


<pre><code><b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_calculate_epoch_subsidy_amount">calculate_epoch_subsidy_amount</a>(self: &<a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">myso_system::stake_subsidy::StakeSubsidy</a>, total_staked_mist: u64, epoch_duration_ms: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_calculate_epoch_subsidy_amount">calculate_epoch_subsidy_amount</a>(
    self: &<a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>,
    total_staked_mist: u64,
    epoch_duration_ms: u64,
): u64 {
    <b>if</b> (total_staked_mist == 0 || epoch_duration_ms == 0) {
        <b>return</b> 0
    };
    <b>let</b> effective_apy_bps = <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_calculate_effective_apy">calculate_effective_apy</a>(self, total_staked_mist, epoch_duration_ms);
    <b>let</b> epochs_per_year = (<a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_YEAR_IN_MS">YEAR_IN_MS</a> / epoch_duration_ms).max(1);
    <b>let</b> yearly_rewards = (total_staked_mist <b>as</b> u128)
        * (effective_apy_bps <b>as</b> u128)
        / <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_BASIS_POINT_DENOMINATOR">BASIS_POINT_DENOMINATOR</a>;
    <b>let</b> per_epoch_rewards = yearly_rewards / (epochs_per_year <b>as</b> u128);
    per_epoch_rewards <b>as</b> u64
}
</code></pre>



</details>

<a name="myso_system_stake_subsidy_get_distribution_counter"></a>

## Function `get_distribution_counter`

Returns the number of distributions that have occurred.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_get_distribution_counter">get_distribution_counter</a>(self: &<a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">myso_system::stake_subsidy::StakeSubsidy</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_get_distribution_counter">get_distribution_counter</a>(self: &<a href="../myso_system/stake_subsidy.md#myso_system_stake_subsidy_StakeSubsidy">StakeSubsidy</a>): u64 {
    self.distribution_counter
}
</code></pre>



</details>
