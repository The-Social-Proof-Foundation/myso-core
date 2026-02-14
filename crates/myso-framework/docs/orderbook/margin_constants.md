---
title: Module `orderbook::margin_constants`
---



-  [Constants](#@Constants_0)
-  [Function `margin_version`](#orderbook_margin_constants_margin_version)
-  [Function `max_risk_ratio`](#orderbook_margin_constants_max_risk_ratio)
-  [Function `default_user_liquidation_reward`](#orderbook_margin_constants_default_user_liquidation_reward)
-  [Function `default_pool_liquidation_reward`](#orderbook_margin_constants_default_pool_liquidation_reward)
-  [Function `min_leverage`](#orderbook_margin_constants_min_leverage)
-  [Function `max_leverage`](#orderbook_margin_constants_max_leverage)
-  [Function `year_ms`](#orderbook_margin_constants_year_ms)
-  [Function `min_min_borrow`](#orderbook_margin_constants_min_min_borrow)
-  [Function `max_margin_managers`](#orderbook_margin_constants_max_margin_managers)
-  [Function `default_referral`](#orderbook_margin_constants_default_referral)
-  [Function `max_protocol_spread`](#orderbook_margin_constants_max_protocol_spread)
-  [Function `min_liquidation_repay`](#orderbook_margin_constants_min_liquidation_repay)
-  [Function `max_conf_bps`](#orderbook_margin_constants_max_conf_bps)
-  [Function `max_ewma_difference_bps`](#orderbook_margin_constants_max_ewma_difference_bps)
-  [Function `max_conditional_orders`](#orderbook_margin_constants_max_conditional_orders)
-  [Function `day_ms`](#orderbook_margin_constants_day_ms)
-  [Function `default_max_price_age_ms`](#orderbook_margin_constants_default_max_price_age_ms)
-  [Function `default_price_tolerance`](#orderbook_margin_constants_default_price_tolerance)
-  [Function `min_price_age_ms`](#orderbook_margin_constants_min_price_age_ms)
-  [Function `max_price_age_ms`](#orderbook_margin_constants_max_price_age_ms)
-  [Function `min_price_tolerance`](#orderbook_margin_constants_min_price_tolerance)
-  [Function `max_price_tolerance`](#orderbook_margin_constants_max_price_tolerance)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="orderbook_margin_constants_MARGIN_VERSION"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MARGIN_VERSION">MARGIN_VERSION</a>: u64 = 3;
</code></pre>



<a name="orderbook_margin_constants_MAX_RISK_RATIO"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_RISK_RATIO">MAX_RISK_RATIO</a>: u64 = 1000000000000;
</code></pre>



<a name="orderbook_margin_constants_DEFAULT_USER_LIQUIDATION_REWARD"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DEFAULT_USER_LIQUIDATION_REWARD">DEFAULT_USER_LIQUIDATION_REWARD</a>: u64 = 10000000;
</code></pre>



<a name="orderbook_margin_constants_DEFAULT_POOL_LIQUIDATION_REWARD"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DEFAULT_POOL_LIQUIDATION_REWARD">DEFAULT_POOL_LIQUIDATION_REWARD</a>: u64 = 40000000;
</code></pre>



<a name="orderbook_margin_constants_MIN_LEVERAGE"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MIN_LEVERAGE">MIN_LEVERAGE</a>: u64 = 1000000000;
</code></pre>



<a name="orderbook_margin_constants_MAX_LEVERAGE"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_LEVERAGE">MAX_LEVERAGE</a>: u64 = 20000000000;
</code></pre>



<a name="orderbook_margin_constants_YEAR_MS"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_YEAR_MS">YEAR_MS</a>: u64 = 31536000000;
</code></pre>



<a name="orderbook_margin_constants_DAY_MS"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DAY_MS">DAY_MS</a>: u64 = 86400000;
</code></pre>



<a name="orderbook_margin_constants_MIN_MIN_BORROW"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MIN_MIN_BORROW">MIN_MIN_BORROW</a>: u64 = 1000;
</code></pre>



<a name="orderbook_margin_constants_MAX_MARGIN_MANAGERS"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_MARGIN_MANAGERS">MAX_MARGIN_MANAGERS</a>: u64 = 100;
</code></pre>



<a name="orderbook_margin_constants_DEFAULT_REFERRAL"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DEFAULT_REFERRAL">DEFAULT_REFERRAL</a>: <b>address</b> = 0x0;
</code></pre>



<a name="orderbook_margin_constants_MAX_PROTOCOL_SPREAD"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_PROTOCOL_SPREAD">MAX_PROTOCOL_SPREAD</a>: u64 = 200000000;
</code></pre>



<a name="orderbook_margin_constants_MIN_LIQUIDATION_REPAY"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MIN_LIQUIDATION_REPAY">MIN_LIQUIDATION_REPAY</a>: u64 = 1000;
</code></pre>



<a name="orderbook_margin_constants_MAX_CONF_BPS"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_CONF_BPS">MAX_CONF_BPS</a>: u64 = 10000;
</code></pre>



<a name="orderbook_margin_constants_MAX_EWMA_DIFFERENCE_BPS"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_EWMA_DIFFERENCE_BPS">MAX_EWMA_DIFFERENCE_BPS</a>: u64 = 10000;
</code></pre>



<a name="orderbook_margin_constants_MAX_CONDITIONAL_ORDERS"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_CONDITIONAL_ORDERS">MAX_CONDITIONAL_ORDERS</a>: u64 = 10;
</code></pre>



<a name="orderbook_margin_constants_DEFAULT_MAX_PRICE_AGE_MS"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DEFAULT_MAX_PRICE_AGE_MS">DEFAULT_MAX_PRICE_AGE_MS</a>: u64 = 300000;
</code></pre>



<a name="orderbook_margin_constants_DEFAULT_PRICE_TOLERANCE"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DEFAULT_PRICE_TOLERANCE">DEFAULT_PRICE_TOLERANCE</a>: u64 = 50000000;
</code></pre>



<a name="orderbook_margin_constants_MIN_PRICE_AGE_MS"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MIN_PRICE_AGE_MS">MIN_PRICE_AGE_MS</a>: u64 = 30000;
</code></pre>



<a name="orderbook_margin_constants_MAX_PRICE_AGE_MS"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_PRICE_AGE_MS">MAX_PRICE_AGE_MS</a>: u64 = 3600000;
</code></pre>



<a name="orderbook_margin_constants_MIN_PRICE_TOLERANCE"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MIN_PRICE_TOLERANCE">MIN_PRICE_TOLERANCE</a>: u64 = 10000000;
</code></pre>



<a name="orderbook_margin_constants_MAX_PRICE_TOLERANCE"></a>



<pre><code><b>const</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_PRICE_TOLERANCE">MAX_PRICE_TOLERANCE</a>: u64 = 500000000;
</code></pre>



<a name="orderbook_margin_constants_margin_version"></a>

## Function `margin_version`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_margin_version">margin_version</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_margin_version">margin_version</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MARGIN_VERSION">MARGIN_VERSION</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_max_risk_ratio"></a>

## Function `max_risk_ratio`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_risk_ratio">max_risk_ratio</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_risk_ratio">max_risk_ratio</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_RISK_RATIO">MAX_RISK_RATIO</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_default_user_liquidation_reward"></a>

## Function `default_user_liquidation_reward`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_default_user_liquidation_reward">default_user_liquidation_reward</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_default_user_liquidation_reward">default_user_liquidation_reward</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DEFAULT_USER_LIQUIDATION_REWARD">DEFAULT_USER_LIQUIDATION_REWARD</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_default_pool_liquidation_reward"></a>

## Function `default_pool_liquidation_reward`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_default_pool_liquidation_reward">default_pool_liquidation_reward</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_default_pool_liquidation_reward">default_pool_liquidation_reward</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DEFAULT_POOL_LIQUIDATION_REWARD">DEFAULT_POOL_LIQUIDATION_REWARD</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_min_leverage"></a>

## Function `min_leverage`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_min_leverage">min_leverage</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_min_leverage">min_leverage</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MIN_LEVERAGE">MIN_LEVERAGE</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_max_leverage"></a>

## Function `max_leverage`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_leverage">max_leverage</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_leverage">max_leverage</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_LEVERAGE">MAX_LEVERAGE</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_year_ms"></a>

## Function `year_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_year_ms">year_ms</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_year_ms">year_ms</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_YEAR_MS">YEAR_MS</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_min_min_borrow"></a>

## Function `min_min_borrow`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_min_min_borrow">min_min_borrow</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_min_min_borrow">min_min_borrow</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MIN_MIN_BORROW">MIN_MIN_BORROW</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_max_margin_managers"></a>

## Function `max_margin_managers`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_margin_managers">max_margin_managers</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_margin_managers">max_margin_managers</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_MARGIN_MANAGERS">MAX_MARGIN_MANAGERS</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_default_referral"></a>

## Function `default_referral`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_default_referral">default_referral</a>(): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_default_referral">default_referral</a>(): ID {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DEFAULT_REFERRAL">DEFAULT_REFERRAL</a>.to_id()
}
</code></pre>



</details>

<a name="orderbook_margin_constants_max_protocol_spread"></a>

## Function `max_protocol_spread`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_protocol_spread">max_protocol_spread</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_protocol_spread">max_protocol_spread</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_PROTOCOL_SPREAD">MAX_PROTOCOL_SPREAD</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_min_liquidation_repay"></a>

## Function `min_liquidation_repay`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_min_liquidation_repay">min_liquidation_repay</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_min_liquidation_repay">min_liquidation_repay</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MIN_LIQUIDATION_REPAY">MIN_LIQUIDATION_REPAY</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_max_conf_bps"></a>

## Function `max_conf_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_conf_bps">max_conf_bps</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_conf_bps">max_conf_bps</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_CONF_BPS">MAX_CONF_BPS</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_max_ewma_difference_bps"></a>

## Function `max_ewma_difference_bps`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_ewma_difference_bps">max_ewma_difference_bps</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_ewma_difference_bps">max_ewma_difference_bps</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_EWMA_DIFFERENCE_BPS">MAX_EWMA_DIFFERENCE_BPS</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_max_conditional_orders"></a>

## Function `max_conditional_orders`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_conditional_orders">max_conditional_orders</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_conditional_orders">max_conditional_orders</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_CONDITIONAL_ORDERS">MAX_CONDITIONAL_ORDERS</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_day_ms"></a>

## Function `day_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_day_ms">day_ms</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_day_ms">day_ms</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DAY_MS">DAY_MS</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_default_max_price_age_ms"></a>

## Function `default_max_price_age_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_default_max_price_age_ms">default_max_price_age_ms</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_default_max_price_age_ms">default_max_price_age_ms</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DEFAULT_MAX_PRICE_AGE_MS">DEFAULT_MAX_PRICE_AGE_MS</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_default_price_tolerance"></a>

## Function `default_price_tolerance`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_default_price_tolerance">default_price_tolerance</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_default_price_tolerance">default_price_tolerance</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_DEFAULT_PRICE_TOLERANCE">DEFAULT_PRICE_TOLERANCE</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_min_price_age_ms"></a>

## Function `min_price_age_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_min_price_age_ms">min_price_age_ms</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_min_price_age_ms">min_price_age_ms</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MIN_PRICE_AGE_MS">MIN_PRICE_AGE_MS</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_max_price_age_ms"></a>

## Function `max_price_age_ms`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_price_age_ms">max_price_age_ms</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_price_age_ms">max_price_age_ms</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_PRICE_AGE_MS">MAX_PRICE_AGE_MS</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_min_price_tolerance"></a>

## Function `min_price_tolerance`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_min_price_tolerance">min_price_tolerance</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_min_price_tolerance">min_price_tolerance</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MIN_PRICE_TOLERANCE">MIN_PRICE_TOLERANCE</a>
}
</code></pre>



</details>

<a name="orderbook_margin_constants_max_price_tolerance"></a>

## Function `max_price_tolerance`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_price_tolerance">max_price_tolerance</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/margin_constants.md#orderbook_margin_constants_max_price_tolerance">max_price_tolerance</a>(): u64 {
    <a href="../orderbook/margin_constants.md#orderbook_margin_constants_MAX_PRICE_TOLERANCE">MAX_PRICE_TOLERANCE</a>
}
</code></pre>



</details>
