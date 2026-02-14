---
title: Module `orderbook::math`
---



-  [Constants](#@Constants_0)
-  [Function `mul`](#orderbook_math_mul)
-  [Function `mul_u128`](#orderbook_math_mul_u128)
-  [Function `mul_round_up`](#orderbook_math_mul_round_up)
-  [Function `div`](#orderbook_math_div)
-  [Function `div_u128`](#orderbook_math_div_u128)
-  [Function `div_round_up`](#orderbook_math_div_round_up)
-  [Function `median`](#orderbook_math_median)
-  [Function `sqrt`](#orderbook_math_sqrt)
-  [Function `is_power_of_ten`](#orderbook_math_is_power_of_ten)
-  [Function `quick_sort`](#orderbook_math_quick_sort)
-  [Function `mul_internal`](#orderbook_math_mul_internal)
-  [Function `mul_internal_u128`](#orderbook_math_mul_internal_u128)
-  [Function `div_internal`](#orderbook_math_div_internal)
-  [Function `div_internal_u128`](#orderbook_math_div_internal_u128)


<pre><code><b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="orderbook_math_FLOAT_SCALING"></a>

scaling setting for float


<pre><code><b>const</b> <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING">FLOAT_SCALING</a>: u64 = 1000000000;
</code></pre>



<a name="orderbook_math_FLOAT_SCALING_U128"></a>



<pre><code><b>const</b> <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U128">FLOAT_SCALING_U128</a>: u128 = 1000000000;
</code></pre>



<a name="orderbook_math_FLOAT_SCALING_U256"></a>



<pre><code><b>const</b> <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U256">FLOAT_SCALING_U256</a>: u256 = 1000000000;
</code></pre>



<a name="orderbook_math_EInvalidPrecision"></a>

Error codes


<pre><code><b>const</b> <a href="../orderbook/math.md#orderbook_math_EInvalidPrecision">EInvalidPrecision</a>: u64 = 0;
</code></pre>



<a name="orderbook_math_mul"></a>

## Function `mul`

Multiply two floating numbers.
This function will round down the result.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_mul">mul</a>(x: u64, y: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_mul">mul</a>(x: u64, y: u64): u64 {
    <b>let</b> (_, result) = <a href="../orderbook/math.md#orderbook_math_mul_internal">mul_internal</a>(x, y);
    result
}
</code></pre>



</details>

<a name="orderbook_math_mul_u128"></a>

## Function `mul_u128`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_mul_u128">mul_u128</a>(x: u128, y: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_mul_u128">mul_u128</a>(x: u128, y: u128): u128 {
    <b>let</b> (_, result) = <a href="../orderbook/math.md#orderbook_math_mul_internal_u128">mul_internal_u128</a>(x, y);
    result
}
</code></pre>



</details>

<a name="orderbook_math_mul_round_up"></a>

## Function `mul_round_up`

Multiply two floating numbers.
This function will round up the result.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_mul_round_up">mul_round_up</a>(x: u64, y: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_mul_round_up">mul_round_up</a>(x: u64, y: u64): u64 {
    <b>let</b> (is_round_down, result) = <a href="../orderbook/math.md#orderbook_math_mul_internal">mul_internal</a>(x, y);
    result + is_round_down
}
</code></pre>



</details>

<a name="orderbook_math_div"></a>

## Function `div`

Divide two floating numbers.
This function will round down the result.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_div">div</a>(x: u64, y: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_div">div</a>(x: u64, y: u64): u64 {
    <b>let</b> (_, result) = <a href="../orderbook/math.md#orderbook_math_div_internal">div_internal</a>(x, y);
    result
}
</code></pre>



</details>

<a name="orderbook_math_div_u128"></a>

## Function `div_u128`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_div_u128">div_u128</a>(x: u128, y: u128): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_div_u128">div_u128</a>(x: u128, y: u128): u128 {
    <b>let</b> (_, result) = <a href="../orderbook/math.md#orderbook_math_div_internal_u128">div_internal_u128</a>(x, y);
    result
}
</code></pre>



</details>

<a name="orderbook_math_div_round_up"></a>

## Function `div_round_up`

Divide two floating numbers.
This function will round up the result.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_div_round_up">div_round_up</a>(x: u64, y: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_div_round_up">div_round_up</a>(x: u64, y: u64): u64 {
    <b>let</b> (is_round_down, result) = <a href="../orderbook/math.md#orderbook_math_div_internal">div_internal</a>(x, y);
    result + is_round_down
}
</code></pre>



</details>

<a name="orderbook_math_median"></a>

## Function `median`

given a vector of u128, return the median


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_median">median</a>(v: vector&lt;u128&gt;): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_median">median</a>(v: vector&lt;u128&gt;): u128 {
    <b>let</b> n = v.length();
    <b>if</b> (n == 0) {
        <b>return</b> 0
    };
    <b>let</b> sorted_v = <a href="../orderbook/math.md#orderbook_math_quick_sort">quick_sort</a>(v);
    <b>if</b> (n % 2 == 0) {
        <a href="../orderbook/math.md#orderbook_math_mul_u128">mul_u128</a>(
            (sorted_v[n / 2 - 1] + sorted_v[n / 2]),
            <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U128">FLOAT_SCALING_U128</a> / 2,
        )
    } <b>else</b> {
        sorted_v[n / 2]
    }
}
</code></pre>



</details>

<a name="orderbook_math_sqrt"></a>

## Function `sqrt`

Computes the integer square root of a scaled u64 value, assuming the
original value
is scaled by precision. The result will be in the same floating-point
representation.


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_sqrt">sqrt</a>(x: u64, precision: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_sqrt">sqrt</a>(x: u64, precision: u64): u64 {
    <b>assert</b>!(precision &lt;= <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING">FLOAT_SCALING</a>, <a href="../orderbook/math.md#orderbook_math_EInvalidPrecision">EInvalidPrecision</a>);
    <b>let</b> multiplier = (<a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING">FLOAT_SCALING</a> / precision) <b>as</b> u128;
    <b>let</b> scaled_x: u128 = (x <b>as</b> u128) * multiplier * <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U128">FLOAT_SCALING_U128</a>;
    <b>let</b> sqrt_scaled_x: u128 = <a href="../std/u128.md#std_u128_sqrt">std::u128::sqrt</a>(scaled_x);
    (sqrt_scaled_x / multiplier) <b>as</b> u64
}
</code></pre>



</details>

<a name="orderbook_math_is_power_of_ten"></a>

## Function `is_power_of_ten`



<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_is_power_of_ten">is_power_of_ten</a>(n: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../orderbook/math.md#orderbook_math_is_power_of_ten">is_power_of_ten</a>(n: u64): bool {
    <b>let</b> <b>mut</b> num = n;
    <b>if</b> (num &lt; 1) {
        <b>false</b>
    } <b>else</b> {
        <b>while</b> (num % 10 == 0) {
            num = num / 10;
        };
        num == 1
    }
}
</code></pre>



</details>

<a name="orderbook_math_quick_sort"></a>

## Function `quick_sort`



<pre><code><b>fun</b> <a href="../orderbook/math.md#orderbook_math_quick_sort">quick_sort</a>(data: vector&lt;u128&gt;): vector&lt;u128&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/math.md#orderbook_math_quick_sort">quick_sort</a>(data: vector&lt;u128&gt;): vector&lt;u128&gt; {
    <b>if</b> (data.length() &lt;= 1) {
        <b>return</b> data
    };
    <b>let</b> pivot = data[0];
    <b>let</b> <b>mut</b> less = vector&lt;u128&gt;[];
    <b>let</b> <b>mut</b> equal = vector&lt;u128&gt;[];
    <b>let</b> <b>mut</b> greater = vector&lt;u128&gt;[];
    data.do!(|value| {
        <b>if</b> (value &lt; pivot) {
            less.push_back(value);
        } <b>else</b> <b>if</b> (value == pivot) {
            equal.push_back(value);
        } <b>else</b> {
            greater.push_back(value);
        };
    });
    <b>let</b> <b>mut</b> sortedData = vector&lt;u128&gt;[];
    sortedData.append(<a href="../orderbook/math.md#orderbook_math_quick_sort">quick_sort</a>(less));
    sortedData.append(equal);
    sortedData.append(<a href="../orderbook/math.md#orderbook_math_quick_sort">quick_sort</a>(greater));
    sortedData
}
</code></pre>



</details>

<a name="orderbook_math_mul_internal"></a>

## Function `mul_internal`



<pre><code><b>fun</b> <a href="../orderbook/math.md#orderbook_math_mul_internal">mul_internal</a>(x: u64, y: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/math.md#orderbook_math_mul_internal">mul_internal</a>(x: u64, y: u64): (u64, u64) {
    <b>let</b> x = x <b>as</b> u128;
    <b>let</b> y = y <b>as</b> u128;
    <b>let</b> round = <b>if</b> ((x * y) % <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U128">FLOAT_SCALING_U128</a> == 0) 0 <b>else</b> 1;
    (round, (x * y / <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U128">FLOAT_SCALING_U128</a>) <b>as</b> u64)
}
</code></pre>



</details>

<a name="orderbook_math_mul_internal_u128"></a>

## Function `mul_internal_u128`



<pre><code><b>fun</b> <a href="../orderbook/math.md#orderbook_math_mul_internal_u128">mul_internal_u128</a>(x: u128, y: u128): (u128, u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/math.md#orderbook_math_mul_internal_u128">mul_internal_u128</a>(x: u128, y: u128): (u128, u128) {
    <b>let</b> x = x <b>as</b> u256;
    <b>let</b> y = y <b>as</b> u256;
    <b>let</b> round = <b>if</b> ((x * y) % <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U256">FLOAT_SCALING_U256</a> == 0) 0 <b>else</b> 1;
    (round, (x * y / <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U256">FLOAT_SCALING_U256</a>) <b>as</b> u128)
}
</code></pre>



</details>

<a name="orderbook_math_div_internal"></a>

## Function `div_internal`



<pre><code><b>fun</b> <a href="../orderbook/math.md#orderbook_math_div_internal">div_internal</a>(x: u64, y: u64): (u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/math.md#orderbook_math_div_internal">div_internal</a>(x: u64, y: u64): (u64, u64) {
    <b>let</b> x = x <b>as</b> u128;
    <b>let</b> y = y <b>as</b> u128;
    <b>let</b> round = <b>if</b> ((x * <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U128">FLOAT_SCALING_U128</a> % y) == 0) 0 <b>else</b> 1;
    (round, (x * <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U128">FLOAT_SCALING_U128</a> / y) <b>as</b> u64)
}
</code></pre>



</details>

<a name="orderbook_math_div_internal_u128"></a>

## Function `div_internal_u128`



<pre><code><b>fun</b> <a href="../orderbook/math.md#orderbook_math_div_internal_u128">div_internal_u128</a>(x: u128, y: u128): (u128, u128)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../orderbook/math.md#orderbook_math_div_internal_u128">div_internal_u128</a>(x: u128, y: u128): (u128, u128) {
    <b>let</b> x = x <b>as</b> u256;
    <b>let</b> y = y <b>as</b> u256;
    <b>let</b> round = <b>if</b> ((x * <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U256">FLOAT_SCALING_U256</a> % y) == 0) 0 <b>else</b> 1;
    (round, (x * <a href="../orderbook/math.md#orderbook_math_FLOAT_SCALING_U256">FLOAT_SCALING_U256</a> / y) <b>as</b> u128)
}
</code></pre>



</details>
