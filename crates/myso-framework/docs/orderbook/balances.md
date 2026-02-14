---
title: Module `orderbook::balances`
---

<code><a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a></code> represents the three assets that make up a pool: base, quote, and
myso. Whenever funds are moved, they are moved in the form of <code><a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a></code>.


-  [Struct `Balances`](#orderbook_balances_Balances)
-  [Function `empty`](#orderbook_balances_empty)
-  [Function `new`](#orderbook_balances_new)
-  [Function `reset`](#orderbook_balances_reset)
-  [Function `add_balances`](#orderbook_balances_add_balances)
-  [Function `add_base`](#orderbook_balances_add_base)
-  [Function `add_quote`](#orderbook_balances_add_quote)
-  [Function `add_myso`](#orderbook_balances_add_myso)
-  [Function `base`](#orderbook_balances_base)
-  [Function `quote`](#orderbook_balances_quote)
-  [Function `myso`](#orderbook_balances_myso)
-  [Function `mul`](#orderbook_balances_mul)
-  [Function `non_zero_value`](#orderbook_balances_non_zero_value)


<pre><code><b>use</b> <a href="../orderbook/math.md#orderbook_math">orderbook::math</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="orderbook_balances_Balances"></a>

## Struct `Balances`



<pre><code><b>public</b> <b>struct</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../orderbook/balances.md#orderbook_balances_base">base</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="orderbook_balances_empty"></a>

## Function `empty`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_empty">empty</a>(): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_empty">empty</a>(): <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a> {
    <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a> { <a href="../orderbook/balances.md#orderbook_balances_base">base</a>: 0, <a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>: 0, <a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>: 0 }
}
</code></pre>



</details>

<a name="orderbook_balances_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_new">new</a>(<a href="../orderbook/balances.md#orderbook_balances_base">base</a>: u64, <a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>: u64, <a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>: u64): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_new">new</a>(<a href="../orderbook/balances.md#orderbook_balances_base">base</a>: u64, <a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>: u64, <a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>: u64): <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a> {
    <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a> { <a href="../orderbook/balances.md#orderbook_balances_base">base</a>: <a href="../orderbook/balances.md#orderbook_balances_base">base</a>, <a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>: <a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>, <a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>: <a href="../orderbook/balances.md#orderbook_balances_myso">myso</a> }
}
</code></pre>



</details>

<a name="orderbook_balances_reset"></a>

## Function `reset`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_reset">reset</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>): <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_reset">reset</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>): <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a> {
    <b>let</b> old = *<a href="../orderbook/balances.md#orderbook_balances">balances</a>;
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_base">base</a> = 0;
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a> = 0;
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_myso">myso</a> = 0;
    old
}
</code></pre>



</details>

<a name="orderbook_balances_add_balances"></a>

## Function `add_balances`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_add_balances">add_balances</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, other: <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_add_balances">add_balances</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>, other: <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>) {
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_base">base</a> = <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_base">base</a> + other.<a href="../orderbook/balances.md#orderbook_balances_base">base</a>;
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a> = <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a> + other.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>;
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_myso">myso</a> = <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_myso">myso</a> + other.<a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>;
}
</code></pre>



</details>

<a name="orderbook_balances_add_base"></a>

## Function `add_base`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_add_base">add_base</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_base">base</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_add_base">add_base</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_base">base</a>: u64) {
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_base">base</a> = <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_base">base</a> + <a href="../orderbook/balances.md#orderbook_balances_base">base</a>;
}
</code></pre>



</details>

<a name="orderbook_balances_add_quote"></a>

## Function `add_quote`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_add_quote">add_quote</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_add_quote">add_quote</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>: u64) {
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a> = <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a> + <a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>;
}
</code></pre>



</details>

<a name="orderbook_balances_add_myso"></a>

## Function `add_myso`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_add_myso">add_myso</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_add_myso">add_myso</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>, <a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>: u64) {
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_myso">myso</a> = <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_myso">myso</a> + <a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>;
}
</code></pre>



</details>

<a name="orderbook_balances_base"></a>

## Function `base`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_base">base</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_base">base</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>): u64 {
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_base">base</a>
}
</code></pre>



</details>

<a name="orderbook_balances_quote"></a>

## Function `quote`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>): u64 {
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>
}
</code></pre>



</details>

<a name="orderbook_balances_myso"></a>

## Function `myso`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>): u64 {
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>
}
</code></pre>



</details>

<a name="orderbook_balances_mul"></a>

## Function `mul`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_mul">mul</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>, factor: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_mul">mul</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<b>mut</b> <a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>, factor: u64) {
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_base">base</a> = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_base">base</a>, factor);
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a> = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>, factor);
    <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_myso">myso</a> = <a href="../orderbook/math.md#orderbook_math_mul">math::mul</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>, factor);
}
</code></pre>



</details>

<a name="orderbook_balances_non_zero_value"></a>

## Function `non_zero_value`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_non_zero_value">non_zero_value</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<a href="../orderbook/balances.md#orderbook_balances_Balances">orderbook::balances::Balances</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/balances.md#orderbook_balances_non_zero_value">non_zero_value</a>(<a href="../orderbook/balances.md#orderbook_balances">balances</a>: &<a href="../orderbook/balances.md#orderbook_balances_Balances">Balances</a>): u64 {
    <b>if</b> (<a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_base">base</a> &gt; 0) {
        <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_base">base</a>
    } <b>else</b> <b>if</b> (<a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a> &gt; 0) {
        <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_quote">quote</a>
    } <b>else</b> {
        <a href="../orderbook/balances.md#orderbook_balances">balances</a>.<a href="../orderbook/balances.md#orderbook_balances_myso">myso</a>
    }
}
</code></pre>



</details>
