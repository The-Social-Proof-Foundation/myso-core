---
title: Module `orderbook::ewma`
---

The Exponentially Weighted Moving Average (EWMA) state for Orderbook
This state is used to calculate the smoothed mean and variance of gas prices
and apply a penalty to taker fees based on the Z-score of the current gas price
relative to the smoothed mean and variance.
The state is disabled by default and can be configured with different parameters.


-  [Struct `EWMAState`](#orderbook_ewma_EWMAState)
-  [Struct `EWMAUpdate`](#orderbook_ewma_EWMAUpdate)
-  [Function `init_ewma_state`](#orderbook_ewma_init_ewma_state)
-  [Function `update`](#orderbook_ewma_update)
-  [Function `z_score`](#orderbook_ewma_z_score)
-  [Function `set_alpha`](#orderbook_ewma_set_alpha)
-  [Function `set_z_score_threshold`](#orderbook_ewma_set_z_score_threshold)
-  [Function `set_additional_taker_fee`](#orderbook_ewma_set_additional_taker_fee)
-  [Function `enable`](#orderbook_ewma_enable)
-  [Function `disable`](#orderbook_ewma_disable)
-  [Function `apply_taker_penalty`](#orderbook_ewma_apply_taker_penalty)
-  [Function `mean`](#orderbook_ewma_mean)
-  [Function `variance`](#orderbook_ewma_variance)
-  [Function `alpha`](#orderbook_ewma_alpha)
-  [Function `z_score_threshold`](#orderbook_ewma_z_score_threshold)
-  [Function `additional_taker_fee`](#orderbook_ewma_additional_taker_fee)
-  [Function `enabled`](#orderbook_ewma_enabled)
-  [Function `last_updated_timestamp`](#orderbook_ewma_last_updated_timestamp)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../orderbook/constants.md#orderbook_constants">orderbook::constants</a>;
<b>use</b> <a href="../orderbook/math.md#orderbook_math">orderbook::math</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="orderbook_ewma_EWMAState"></a>

## Struct `EWMAState`

The EWMA state structure
It contains the smoothed mean, variance, alpha, Z-score threshold,
additional taker fee, and whether the state is enabled.


<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/ewma.md#orderbook_ewma_z_score_threshold">z_score_threshold</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/ewma.md#orderbook_ewma_additional_taker_fee">additional_taker_fee</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/ewma.md#orderbook_ewma_last_updated_timestamp">last_updated_timestamp</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/ewma.md#orderbook_ewma_enabled">enabled</a>: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_ewma_EWMAUpdate"></a>

## Struct `EWMAUpdate`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAUpdate">EWMAUpdate</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>gas_price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_ewma_init_ewma_state"></a>

## Function `init_ewma_state`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_init_ewma_state">init_ewma_state</a>(ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_init_ewma_state">init_ewma_state</a>(ctx: &TxContext): <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a> {
    <b>let</b> gas_price = ctx.gas_price() * <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>();
    <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a> {
        <a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>: gas_price,
        <a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a>: 0,
        <a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>: <a href="../orderbook/constants.md#orderbook_constants_default_ewma_alpha">constants::default_ewma_alpha</a>(),
        <a href="../orderbook/ewma.md#orderbook_ewma_z_score_threshold">z_score_threshold</a>: <a href="../orderbook/constants.md#orderbook_constants_default_z_score_threshold">constants::default_z_score_threshold</a>(),
        <a href="../orderbook/ewma.md#orderbook_ewma_additional_taker_fee">additional_taker_fee</a>: <a href="../orderbook/constants.md#orderbook_constants_default_additional_taker_fee">constants::default_additional_taker_fee</a>(),
        <a href="../orderbook/ewma.md#orderbook_ewma_last_updated_timestamp">last_updated_timestamp</a>: 0,
        <a href="../orderbook/ewma.md#orderbook_ewma_enabled">enabled</a>: <b>false</b>,
    }
}
</code></pre>



</details>

<a name="orderbook_ewma_update"></a>

## Function `update`

Updates the EWMA state with the current gas price
It calculates the new mean and variance based on the current gas price
and the previous mean and variance using the EWMA formula.
The alpha parameter controls the weight of the current gas price in the calculation.
The mean and variance are updated in the state.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_update">update</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_update">update</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>, pool_id: ID, clock: &Clock, ctx: &TxContext) {
    <b>let</b> current_timestamp = clock.timestamp_ms();
    <b>if</b> (current_timestamp == self.<a href="../orderbook/ewma.md#orderbook_ewma_last_updated_timestamp">last_updated_timestamp</a>) {
        <b>return</b>
    };
    self.<a href="../orderbook/ewma.md#orderbook_ewma_last_updated_timestamp">last_updated_timestamp</a> = current_timestamp;
    <b>let</b> <a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a> = self.<a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>;
    <b>let</b> one_minute_alpha = <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>() - <a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>;
    <b>let</b> gas_price = ctx.gas_price() * <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>();
    <b>let</b> mean_new = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(<a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>, gas_price) + <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(one_minute_alpha, self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>);
    <b>let</b> diff = <b>if</b> (gas_price &gt; self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>) {
        gas_price - self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>
    } <b>else</b> {
        self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a> - gas_price
    };
    <b>let</b> diff_squared = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(diff, diff);
    <b>let</b> variance_new = <b>if</b> (self.<a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a> == 0) {
        diff_squared
    } <b>else</b> {
        <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(self.<a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a>, one_minute_alpha) + <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(<a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>, diff_squared)
    };
    self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a> = mean_new;
    self.<a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a> = variance_new;
    event::emit(<a href="../orderbook/ewma.md#orderbook_ewma_EWMAUpdate">EWMAUpdate</a> {
        pool_id,
        gas_price,
        <a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>: self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>,
        <a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a>: self.<a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a>,
        timestamp: current_timestamp,
    });
}
</code></pre>



</details>

<a name="orderbook_ewma_z_score"></a>

## Function `z_score`

Returns the Z-score of the current gas price relative to the smoothed mean and variance.
The Z-score is calculated as the difference between the current gas price and the mean,
divided by the standard deviation (square root of variance).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_z_score">z_score</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_z_score">z_score</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>, ctx: &TxContext): u64 {
    <b>if</b> (self.<a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a> == 0) {
        <b>return</b> 0
    };
    <b>let</b> gas_price = ctx.gas_price() * <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>();
    <b>let</b> diff = <b>if</b> (gas_price &gt; self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>) {
        gas_price - self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>
    } <b>else</b> {
        self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a> - gas_price
    };
    <b>let</b> std_dev = <a href="../orderbook/math.md#orderbook_math_sqrt">math::sqrt</a>(self.<a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a>, <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>());
    <b>let</b> z = <a href="../orderbook/math.md#orderbook_math_div">math::div</a>(diff, std_dev);
    z
}
</code></pre>



</details>

<a name="orderbook_ewma_set_alpha"></a>

## Function `set_alpha`

Sets the alpha value for the EWMA state. Admin only.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_set_alpha">set_alpha</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>, <a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_set_alpha">set_alpha</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>, <a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>: u64) {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a> = <a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>;
}
</code></pre>



</details>

<a name="orderbook_ewma_set_z_score_threshold"></a>

## Function `set_z_score_threshold`

Sets the Z-score threshold for the EWMA state. Admin only.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_set_z_score_threshold">set_z_score_threshold</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>, threshold: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_set_z_score_threshold">set_z_score_threshold</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>, threshold: u64) {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_z_score_threshold">z_score_threshold</a> = threshold;
}
</code></pre>



</details>

<a name="orderbook_ewma_set_additional_taker_fee"></a>

## Function `set_additional_taker_fee`

Sets the additional taker fee for the EWMA state. Admin only.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_set_additional_taker_fee">set_additional_taker_fee</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>, fee: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_set_additional_taker_fee">set_additional_taker_fee</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>, fee: u64) {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_additional_taker_fee">additional_taker_fee</a> = fee;
}
</code></pre>



</details>

<a name="orderbook_ewma_enable"></a>

## Function `enable`

Enables the EWMA state. Admin only.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_enable">enable</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_enable">enable</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>) {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_enabled">enabled</a> = <b>true</b>;
}
</code></pre>



</details>

<a name="orderbook_ewma_disable"></a>

## Function `disable`

Disables the EWMA state. Admin only.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_disable">disable</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_disable">disable</a>(self: &<b>mut</b> <a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>) {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_enabled">enabled</a> = <b>false</b>;
}
</code></pre>



</details>

<a name="orderbook_ewma_apply_taker_penalty"></a>

## Function `apply_taker_penalty`

Applies the taker penalty based on the Z-score of the current gas price.
If the gas price is below the mean, the taker fee is not applied.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_apply_taker_penalty">apply_taker_penalty</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>, taker_fee: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_apply_taker_penalty">apply_taker_penalty</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>, taker_fee: u64, ctx: &TxContext): u64 {
    <b>let</b> gas_price = ctx.gas_price() * <a href="../orderbook/constants.md#orderbook_constants_float_scaling">constants::float_scaling</a>();
    <b>if</b> (!self.<a href="../orderbook/ewma.md#orderbook_ewma_enabled">enabled</a> || gas_price &lt; self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>) {
        <b>return</b> taker_fee
    };
    <b>let</b> <a href="../orderbook/ewma.md#orderbook_ewma_z_score">z_score</a> = self.<a href="../orderbook/ewma.md#orderbook_ewma_z_score">z_score</a>(ctx);
    <b>if</b> (<a href="../orderbook/ewma.md#orderbook_ewma_z_score">z_score</a> &gt; self.<a href="../orderbook/ewma.md#orderbook_ewma_z_score_threshold">z_score_threshold</a>) {
        taker_fee + self.<a href="../orderbook/ewma.md#orderbook_ewma_additional_taker_fee">additional_taker_fee</a>
    } <b>else</b> {
        taker_fee
    }
}
</code></pre>



</details>

<a name="orderbook_ewma_mean"></a>

## Function `mean`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>): u64 {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_mean">mean</a>
}
</code></pre>



</details>

<a name="orderbook_ewma_variance"></a>

## Function `variance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>): u64 {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_variance">variance</a>
}
</code></pre>



</details>

<a name="orderbook_ewma_alpha"></a>

## Function `alpha`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>): u64 {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_alpha">alpha</a>
}
</code></pre>



</details>

<a name="orderbook_ewma_z_score_threshold"></a>

## Function `z_score_threshold`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_z_score_threshold">z_score_threshold</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_z_score_threshold">z_score_threshold</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>): u64 {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_z_score_threshold">z_score_threshold</a>
}
</code></pre>



</details>

<a name="orderbook_ewma_additional_taker_fee"></a>

## Function `additional_taker_fee`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_additional_taker_fee">additional_taker_fee</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_additional_taker_fee">additional_taker_fee</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>): u64 {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_additional_taker_fee">additional_taker_fee</a>
}
</code></pre>



</details>

<a name="orderbook_ewma_enabled"></a>

## Function `enabled`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_enabled">enabled</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_enabled">enabled</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>): bool {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_enabled">enabled</a>
}
</code></pre>



</details>

<a name="orderbook_ewma_last_updated_timestamp"></a>

## Function `last_updated_timestamp`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_last_updated_timestamp">last_updated_timestamp</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">orderbook::ewma::EWMAState</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/ewma.md#orderbook_ewma_last_updated_timestamp">last_updated_timestamp</a>(self: &<a href="../orderbook/ewma.md#orderbook_ewma_EWMAState">EWMAState</a>): u64 {
    self.<a href="../orderbook/ewma.md#orderbook_ewma_last_updated_timestamp">last_updated_timestamp</a>
}
</code></pre>



</details>
