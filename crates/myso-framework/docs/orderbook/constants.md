---
title: Module `orderbook::constants`
---



-  [Constants](#@Constants_0)
-  [Function `current_version`](#orderbook_constants_current_version)
-  [Function `pool_creation_fee`](#orderbook_constants_pool_creation_fee)
-  [Function `float_scaling`](#orderbook_constants_float_scaling)
-  [Function `float_scaling_u128`](#orderbook_constants_float_scaling_u128)
-  [Function `max_u64`](#orderbook_constants_max_u64)
-  [Function `max_u128`](#orderbook_constants_max_u128)
-  [Function `no_restriction`](#orderbook_constants_no_restriction)
-  [Function `immediate_or_cancel`](#orderbook_constants_immediate_or_cancel)
-  [Function `fill_or_kill`](#orderbook_constants_fill_or_kill)
-  [Function `post_only`](#orderbook_constants_post_only)
-  [Function `max_restriction`](#orderbook_constants_max_restriction)
-  [Function `live`](#orderbook_constants_live)
-  [Function `partially_filled`](#orderbook_constants_partially_filled)
-  [Function `filled`](#orderbook_constants_filled)
-  [Function `canceled`](#orderbook_constants_canceled)
-  [Function `expired`](#orderbook_constants_expired)
-  [Function `self_matching_allowed`](#orderbook_constants_self_matching_allowed)
-  [Function `cancel_taker`](#orderbook_constants_cancel_taker)
-  [Function `cancel_maker`](#orderbook_constants_cancel_maker)
-  [Function `min_price`](#orderbook_constants_min_price)
-  [Function `max_price`](#orderbook_constants_max_price)
-  [Function `phase_out_epochs`](#orderbook_constants_phase_out_epochs)
-  [Function `default_stake_required`](#orderbook_constants_default_stake_required)
-  [Function `half`](#orderbook_constants_half)
-  [Function `fee_is_myso`](#orderbook_constants_fee_is_myso)
-  [Function `myso_unit`](#orderbook_constants_myso_unit)
-  [Function `max_fills`](#orderbook_constants_max_fills)
-  [Function `max_open_orders`](#orderbook_constants_max_open_orders)
-  [Function `max_slice_size`](#orderbook_constants_max_slice_size)
-  [Function `max_fan_out`](#orderbook_constants_max_fan_out)
-  [Function `fee_penalty_multiplier`](#orderbook_constants_fee_penalty_multiplier)
-  [Function `default_ewma_alpha`](#orderbook_constants_default_ewma_alpha)
-  [Function `default_z_score_threshold`](#orderbook_constants_default_z_score_threshold)
-  [Function `default_additional_taker_fee`](#orderbook_constants_default_additional_taker_fee)
-  [Function `max_ewma_alpha`](#orderbook_constants_max_ewma_alpha)
-  [Function `max_z_score_threshold`](#orderbook_constants_max_z_score_threshold)
-  [Function `max_additional_taker_fee`](#orderbook_constants_max_additional_taker_fee)
-  [Function `ewma_df_key`](#orderbook_constants_ewma_df_key)
-  [Function `referral_max_multiplier`](#orderbook_constants_referral_max_multiplier)
-  [Function `referral_multiplier`](#orderbook_constants_referral_multiplier)
-  [Function `max_balance_managers`](#orderbook_constants_max_balance_managers)
-  [Function `referral_df_key`](#orderbook_constants_referral_df_key)


<pre><code></code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="orderbook_constants_CURRENT_VERSION"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_CURRENT_VERSION">CURRENT_VERSION</a>: u64 = 6;
</code></pre>



<a name="orderbook_constants_POOL_CREATION_FEE"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_POOL_CREATION_FEE">POOL_CREATION_FEE</a>: u64 = 500000000000;
</code></pre>



<a name="orderbook_constants_FLOAT_SCALING"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_FLOAT_SCALING">FLOAT_SCALING</a>: u64 = 1000000000;
</code></pre>



<a name="orderbook_constants_FLOAT_SCALING_U128"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_FLOAT_SCALING_U128">FLOAT_SCALING_U128</a>: u128 = 1000000000;
</code></pre>



<a name="orderbook_constants_MAX_U64"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="orderbook_constants_MAX_U128"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_U128">MAX_U128</a>: u128 = 340282366920938463463374607431768211455;
</code></pre>



<a name="orderbook_constants_MIN_PRICE"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MIN_PRICE">MIN_PRICE</a>: u64 = 1;
</code></pre>



<a name="orderbook_constants_MAX_PRICE"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_PRICE">MAX_PRICE</a>: u64 = 9223372036854775807;
</code></pre>



<a name="orderbook_constants_DEFAULT_STAKE_REQUIRED"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_DEFAULT_STAKE_REQUIRED">DEFAULT_STAKE_REQUIRED</a>: u64 = 100000000000;
</code></pre>



<a name="orderbook_constants_HALF"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_HALF">HALF</a>: u64 = 500000000;
</code></pre>



<a name="orderbook_constants_MYSO_UNIT"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MYSO_UNIT">MYSO_UNIT</a>: u64 = 1000000000;
</code></pre>



<a name="orderbook_constants_FEE_PENALTY_MULTIPLIER"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_FEE_PENALTY_MULTIPLIER">FEE_PENALTY_MULTIPLIER</a>: u64 = 1250000000;
</code></pre>



<a name="orderbook_constants_EWMA_DF_KEY"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_EWMA_DF_KEY">EWMA_DF_KEY</a>: vector&lt;u8&gt; = vector[101, 119, 109, 97];
</code></pre>



<a name="orderbook_constants_REFERRAL_MAX_MULTIPLIER"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_REFERRAL_MAX_MULTIPLIER">REFERRAL_MAX_MULTIPLIER</a>: u64 = 2000000000;
</code></pre>



<a name="orderbook_constants_REFERRAL_MULTIPLIER"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_REFERRAL_MULTIPLIER">REFERRAL_MULTIPLIER</a>: u64 = 100000000;
</code></pre>



<a name="orderbook_constants_MAX_BALANCE_MANAGERS"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_BALANCE_MANAGERS">MAX_BALANCE_MANAGERS</a>: u64 = 100;
</code></pre>



<a name="orderbook_constants_DEFAULT_EWMA_ALPHA"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_DEFAULT_EWMA_ALPHA">DEFAULT_EWMA_ALPHA</a>: u64 = 10000000;
</code></pre>



<a name="orderbook_constants_MAX_EWMA_ALPHA"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_EWMA_ALPHA">MAX_EWMA_ALPHA</a>: u64 = 100000000;
</code></pre>



<a name="orderbook_constants_DEFAULT_Z_SCORE_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_DEFAULT_Z_SCORE_THRESHOLD">DEFAULT_Z_SCORE_THRESHOLD</a>: u64 = 3000000000;
</code></pre>



<a name="orderbook_constants_MAX_Z_SCORE_THRESHOLD"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_Z_SCORE_THRESHOLD">MAX_Z_SCORE_THRESHOLD</a>: u64 = 10000000000;
</code></pre>



<a name="orderbook_constants_DEFAULT_ADDITIONAL_TAKER_FEE"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_DEFAULT_ADDITIONAL_TAKER_FEE">DEFAULT_ADDITIONAL_TAKER_FEE</a>: u64 = 1000000;
</code></pre>



<a name="orderbook_constants_MAX_ADDITIONAL_TAKER_FEE"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_ADDITIONAL_TAKER_FEE">MAX_ADDITIONAL_TAKER_FEE</a>: u64 = 2000000;
</code></pre>



<a name="orderbook_constants_NO_RESTRICTION"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_NO_RESTRICTION">NO_RESTRICTION</a>: u8 = 0;
</code></pre>



<a name="orderbook_constants_IMMEDIATE_OR_CANCEL"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_IMMEDIATE_OR_CANCEL">IMMEDIATE_OR_CANCEL</a>: u8 = 1;
</code></pre>



<a name="orderbook_constants_FILL_OR_KILL"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_FILL_OR_KILL">FILL_OR_KILL</a>: u8 = 2;
</code></pre>



<a name="orderbook_constants_POST_ONLY"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_POST_ONLY">POST_ONLY</a>: u8 = 3;
</code></pre>



<a name="orderbook_constants_MAX_RESTRICTION"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_RESTRICTION">MAX_RESTRICTION</a>: u8 = 3;
</code></pre>



<a name="orderbook_constants_SELF_MATCHING_ALLOWED"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_SELF_MATCHING_ALLOWED">SELF_MATCHING_ALLOWED</a>: u8 = 0;
</code></pre>



<a name="orderbook_constants_CANCEL_TAKER"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_CANCEL_TAKER">CANCEL_TAKER</a>: u8 = 1;
</code></pre>



<a name="orderbook_constants_CANCEL_MAKER"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_CANCEL_MAKER">CANCEL_MAKER</a>: u8 = 2;
</code></pre>



<a name="orderbook_constants_LIVE"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_LIVE">LIVE</a>: u8 = 0;
</code></pre>



<a name="orderbook_constants_PARTIALLY_FILLED"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_PARTIALLY_FILLED">PARTIALLY_FILLED</a>: u8 = 1;
</code></pre>



<a name="orderbook_constants_FILLED"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_FILLED">FILLED</a>: u8 = 2;
</code></pre>



<a name="orderbook_constants_CANCELED"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_CANCELED">CANCELED</a>: u8 = 3;
</code></pre>



<a name="orderbook_constants_EXPIRED"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_EXPIRED">EXPIRED</a>: u8 = 4;
</code></pre>



<a name="orderbook_constants_MAX_FILLS"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_FILLS">MAX_FILLS</a>: u64 = 100;
</code></pre>



<a name="orderbook_constants_MAX_OPEN_ORDERS"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_OPEN_ORDERS">MAX_OPEN_ORDERS</a>: u64 = 100;
</code></pre>



<a name="orderbook_constants_MAX_SLICE_SIZE"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_SLICE_SIZE">MAX_SLICE_SIZE</a>: u64 = 64;
</code></pre>



<a name="orderbook_constants_MAX_FAN_OUT"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_MAX_FAN_OUT">MAX_FAN_OUT</a>: u64 = 64;
</code></pre>



<a name="orderbook_constants_PHASE_OUT_EPOCHS"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_PHASE_OUT_EPOCHS">PHASE_OUT_EPOCHS</a>: u64 = 28;
</code></pre>



<a name="orderbook_constants_FEE_IS_MYSO"></a>



<pre><code><b>const</b> <a href="../orderbook/constants.md#orderbook_constants_FEE_IS_MYSO">FEE_IS_MYSO</a>: bool = <b>true</b>;
</code></pre>



<a name="orderbook_constants_current_version"></a>

## Function `current_version`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_current_version">current_version</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_current_version">current_version</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_CURRENT_VERSION">CURRENT_VERSION</a>
}
</code></pre>



</details>

<a name="orderbook_constants_pool_creation_fee"></a>

## Function `pool_creation_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_pool_creation_fee">pool_creation_fee</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_pool_creation_fee">pool_creation_fee</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_POOL_CREATION_FEE">POOL_CREATION_FEE</a>
}
</code></pre>



</details>

<a name="orderbook_constants_float_scaling"></a>

## Function `float_scaling`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_float_scaling">float_scaling</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_float_scaling">float_scaling</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_FLOAT_SCALING">FLOAT_SCALING</a>
}
</code></pre>



</details>

<a name="orderbook_constants_float_scaling_u128"></a>

## Function `float_scaling_u128`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_float_scaling_u128">float_scaling_u128</a>(): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_float_scaling_u128">float_scaling_u128</a>(): u128 {
    <a href="../orderbook/constants.md#orderbook_constants_FLOAT_SCALING_U128">FLOAT_SCALING_U128</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_u64"></a>

## Function `max_u64`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_u64">max_u64</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_u64">max_u64</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_U64">MAX_U64</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_u128"></a>

## Function `max_u128`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_u128">max_u128</a>(): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_u128">max_u128</a>(): u128 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_U128">MAX_U128</a>
}
</code></pre>



</details>

<a name="orderbook_constants_no_restriction"></a>

## Function `no_restriction`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_no_restriction">no_restriction</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_no_restriction">no_restriction</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_NO_RESTRICTION">NO_RESTRICTION</a>
}
</code></pre>



</details>

<a name="orderbook_constants_immediate_or_cancel"></a>

## Function `immediate_or_cancel`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_immediate_or_cancel">immediate_or_cancel</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_immediate_or_cancel">immediate_or_cancel</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_IMMEDIATE_OR_CANCEL">IMMEDIATE_OR_CANCEL</a>
}
</code></pre>



</details>

<a name="orderbook_constants_fill_or_kill"></a>

## Function `fill_or_kill`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_fill_or_kill">fill_or_kill</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_fill_or_kill">fill_or_kill</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_FILL_OR_KILL">FILL_OR_KILL</a>
}
</code></pre>



</details>

<a name="orderbook_constants_post_only"></a>

## Function `post_only`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_post_only">post_only</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_post_only">post_only</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_POST_ONLY">POST_ONLY</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_restriction"></a>

## Function `max_restriction`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_restriction">max_restriction</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_restriction">max_restriction</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_RESTRICTION">MAX_RESTRICTION</a>
}
</code></pre>



</details>

<a name="orderbook_constants_live"></a>

## Function `live`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_live">live</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_live">live</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_LIVE">LIVE</a>
}
</code></pre>



</details>

<a name="orderbook_constants_partially_filled"></a>

## Function `partially_filled`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_partially_filled">partially_filled</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_partially_filled">partially_filled</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_PARTIALLY_FILLED">PARTIALLY_FILLED</a>
}
</code></pre>



</details>

<a name="orderbook_constants_filled"></a>

## Function `filled`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_filled">filled</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_filled">filled</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_FILLED">FILLED</a>
}
</code></pre>



</details>

<a name="orderbook_constants_canceled"></a>

## Function `canceled`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_canceled">canceled</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_canceled">canceled</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_CANCELED">CANCELED</a>
}
</code></pre>



</details>

<a name="orderbook_constants_expired"></a>

## Function `expired`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_expired">expired</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_expired">expired</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_EXPIRED">EXPIRED</a>
}
</code></pre>



</details>

<a name="orderbook_constants_self_matching_allowed"></a>

## Function `self_matching_allowed`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_self_matching_allowed">self_matching_allowed</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_self_matching_allowed">self_matching_allowed</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_SELF_MATCHING_ALLOWED">SELF_MATCHING_ALLOWED</a>
}
</code></pre>



</details>

<a name="orderbook_constants_cancel_taker"></a>

## Function `cancel_taker`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_cancel_taker">cancel_taker</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_cancel_taker">cancel_taker</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_CANCEL_TAKER">CANCEL_TAKER</a>
}
</code></pre>



</details>

<a name="orderbook_constants_cancel_maker"></a>

## Function `cancel_maker`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_cancel_maker">cancel_maker</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_cancel_maker">cancel_maker</a>(): u8 {
    <a href="../orderbook/constants.md#orderbook_constants_CANCEL_MAKER">CANCEL_MAKER</a>
}
</code></pre>



</details>

<a name="orderbook_constants_min_price"></a>

## Function `min_price`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_min_price">min_price</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_min_price">min_price</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MIN_PRICE">MIN_PRICE</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_price"></a>

## Function `max_price`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_price">max_price</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_price">max_price</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_PRICE">MAX_PRICE</a>
}
</code></pre>



</details>

<a name="orderbook_constants_phase_out_epochs"></a>

## Function `phase_out_epochs`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_phase_out_epochs">phase_out_epochs</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_phase_out_epochs">phase_out_epochs</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_PHASE_OUT_EPOCHS">PHASE_OUT_EPOCHS</a>
}
</code></pre>



</details>

<a name="orderbook_constants_default_stake_required"></a>

## Function `default_stake_required`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_default_stake_required">default_stake_required</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_default_stake_required">default_stake_required</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_DEFAULT_STAKE_REQUIRED">DEFAULT_STAKE_REQUIRED</a>
}
</code></pre>



</details>

<a name="orderbook_constants_half"></a>

## Function `half`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_half">half</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_half">half</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_HALF">HALF</a>
}
</code></pre>



</details>

<a name="orderbook_constants_fee_is_myso"></a>

## Function `fee_is_myso`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_fee_is_myso">fee_is_myso</a>(): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_fee_is_myso">fee_is_myso</a>(): bool {
    <a href="../orderbook/constants.md#orderbook_constants_FEE_IS_MYSO">FEE_IS_MYSO</a>
}
</code></pre>



</details>

<a name="orderbook_constants_myso_unit"></a>

## Function `myso_unit`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_myso_unit">myso_unit</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_myso_unit">myso_unit</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MYSO_UNIT">MYSO_UNIT</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_fills"></a>

## Function `max_fills`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_fills">max_fills</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_fills">max_fills</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_FILLS">MAX_FILLS</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_open_orders"></a>

## Function `max_open_orders`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_open_orders">max_open_orders</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_open_orders">max_open_orders</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_OPEN_ORDERS">MAX_OPEN_ORDERS</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_slice_size"></a>

## Function `max_slice_size`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_slice_size">max_slice_size</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_slice_size">max_slice_size</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_SLICE_SIZE">MAX_SLICE_SIZE</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_fan_out"></a>

## Function `max_fan_out`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_fan_out">max_fan_out</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_fan_out">max_fan_out</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_FAN_OUT">MAX_FAN_OUT</a>
}
</code></pre>



</details>

<a name="orderbook_constants_fee_penalty_multiplier"></a>

## Function `fee_penalty_multiplier`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_fee_penalty_multiplier">fee_penalty_multiplier</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_fee_penalty_multiplier">fee_penalty_multiplier</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_FEE_PENALTY_MULTIPLIER">FEE_PENALTY_MULTIPLIER</a>
}
</code></pre>



</details>

<a name="orderbook_constants_default_ewma_alpha"></a>

## Function `default_ewma_alpha`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_default_ewma_alpha">default_ewma_alpha</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_default_ewma_alpha">default_ewma_alpha</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_DEFAULT_EWMA_ALPHA">DEFAULT_EWMA_ALPHA</a>
}
</code></pre>



</details>

<a name="orderbook_constants_default_z_score_threshold"></a>

## Function `default_z_score_threshold`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_default_z_score_threshold">default_z_score_threshold</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_default_z_score_threshold">default_z_score_threshold</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_DEFAULT_Z_SCORE_THRESHOLD">DEFAULT_Z_SCORE_THRESHOLD</a>
}
</code></pre>



</details>

<a name="orderbook_constants_default_additional_taker_fee"></a>

## Function `default_additional_taker_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_default_additional_taker_fee">default_additional_taker_fee</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_default_additional_taker_fee">default_additional_taker_fee</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_DEFAULT_ADDITIONAL_TAKER_FEE">DEFAULT_ADDITIONAL_TAKER_FEE</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_ewma_alpha"></a>

## Function `max_ewma_alpha`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_ewma_alpha">max_ewma_alpha</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_ewma_alpha">max_ewma_alpha</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_EWMA_ALPHA">MAX_EWMA_ALPHA</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_z_score_threshold"></a>

## Function `max_z_score_threshold`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_z_score_threshold">max_z_score_threshold</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_z_score_threshold">max_z_score_threshold</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_Z_SCORE_THRESHOLD">MAX_Z_SCORE_THRESHOLD</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_additional_taker_fee"></a>

## Function `max_additional_taker_fee`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_additional_taker_fee">max_additional_taker_fee</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_additional_taker_fee">max_additional_taker_fee</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_ADDITIONAL_TAKER_FEE">MAX_ADDITIONAL_TAKER_FEE</a>
}
</code></pre>



</details>

<a name="orderbook_constants_ewma_df_key"></a>

## Function `ewma_df_key`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_ewma_df_key">ewma_df_key</a>(): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_ewma_df_key">ewma_df_key</a>(): vector&lt;u8&gt; {
    <a href="../orderbook/constants.md#orderbook_constants_EWMA_DF_KEY">EWMA_DF_KEY</a>
}
</code></pre>



</details>

<a name="orderbook_constants_referral_max_multiplier"></a>

## Function `referral_max_multiplier`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_referral_max_multiplier">referral_max_multiplier</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_referral_max_multiplier">referral_max_multiplier</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_REFERRAL_MAX_MULTIPLIER">REFERRAL_MAX_MULTIPLIER</a>
}
</code></pre>



</details>

<a name="orderbook_constants_referral_multiplier"></a>

## Function `referral_multiplier`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_referral_multiplier">referral_multiplier</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_referral_multiplier">referral_multiplier</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_REFERRAL_MULTIPLIER">REFERRAL_MULTIPLIER</a>
}
</code></pre>



</details>

<a name="orderbook_constants_max_balance_managers"></a>

## Function `max_balance_managers`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_balance_managers">max_balance_managers</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_max_balance_managers">max_balance_managers</a>(): u64 {
    <a href="../orderbook/constants.md#orderbook_constants_MAX_BALANCE_MANAGERS">MAX_BALANCE_MANAGERS</a>
}
</code></pre>



</details>

<a name="orderbook_constants_referral_df_key"></a>

## Function `referral_df_key`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_referral_df_key">referral_df_key</a>(): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/constants.md#orderbook_constants_referral_df_key">referral_df_key</a>(): vector&lt;u8&gt; {
    <b>abort</b>
}
</code></pre>



</details>
