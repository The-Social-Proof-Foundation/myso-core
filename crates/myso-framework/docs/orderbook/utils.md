---
title: Module `orderbook::utils`
---

Orderbook utility functions.


-  [Function `pop_until`](#orderbook_utils_pop_until)
-  [Function `pop_n`](#orderbook_utils_pop_n)
-  [Function `encode_order_id`](#orderbook_utils_encode_order_id)
-  [Function `decode_order_id`](#orderbook_utils_decode_order_id)


<pre><code><b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="orderbook_utils_pop_until"></a>

## Function `pop_until`

Pop elements from the back of <code>v</code> until its length equals <code>n</code>,
returning the elements that were popped in the order they
appeared in <code>v</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/utils.md#orderbook_utils_pop_until">pop_until</a>&lt;T&gt;(v: &<b>mut</b> vector&lt;T&gt;, n: u64): vector&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/utils.md#orderbook_utils_pop_until">pop_until</a>&lt;T&gt;(v: &<b>mut</b> vector&lt;T&gt;, n: u64): vector&lt;T&gt; {
    <b>let</b> <b>mut</b> res = vector[];
    <b>while</b> (v.length() &gt; n) {
        res.push_back(v.pop_back());
    };
    res.reverse();
    res
}
</code></pre>



</details>

<a name="orderbook_utils_pop_n"></a>

## Function `pop_n`

Pop <code>n</code> elements from the back of <code>v</code>, returning the elements
that were popped in the order they appeared in <code>v</code>.

Aborts if <code>v</code> has fewer than <code>n</code> elements.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/utils.md#orderbook_utils_pop_n">pop_n</a>&lt;T&gt;(v: &<b>mut</b> vector&lt;T&gt;, n: u64): vector&lt;T&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/utils.md#orderbook_utils_pop_n">pop_n</a>&lt;T&gt;(v: &<b>mut</b> vector&lt;T&gt;, n: u64): vector&lt;T&gt; {
    <b>let</b> <b>mut</b> res = vector[];
    n.do!(|_| res.push_back(v.pop_back()));
    res.reverse();
    res
}
</code></pre>



</details>

<a name="orderbook_utils_encode_order_id"></a>

## Function `encode_order_id`

first bit is 0 for bid, 1 for ask
next 63 bits are price (assertion for price is done in order function)
last 64 bits are order_id


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/utils.md#orderbook_utils_encode_order_id">encode_order_id</a>(is_bid: bool, price: u64, order_id: u64): u128
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/utils.md#orderbook_utils_encode_order_id">encode_order_id</a>(is_bid: bool, price: u64, order_id: u64): u128 {
    <b>if</b> (is_bid) {
        ((price <b>as</b> u128) &lt;&lt; 64) + (order_id <b>as</b> u128)
    } <b>else</b> {
        (1u128 &lt;&lt; 127) + ((price <b>as</b> u128) &lt;&lt; 64) + (order_id <b>as</b> u128)
    }
}
</code></pre>



</details>

<a name="orderbook_utils_decode_order_id"></a>

## Function `decode_order_id`

Decode order_id into (is_bid, price, order_id)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/utils.md#orderbook_utils_decode_order_id">decode_order_id</a>(encoded_order_id: u128): (bool, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../orderbook/utils.md#orderbook_utils_decode_order_id">decode_order_id</a>(encoded_order_id: u128): (bool, u64, u64) {
    <b>let</b> is_bid = (encoded_order_id &gt;&gt; 127) == 0;
    <b>let</b> price = (encoded_order_id &gt;&gt; 64) <b>as</b> u64;
    <b>let</b> price = price & ((1u64 &lt;&lt; 63) - 1);
    <b>let</b> order_id = (encoded_order_id & ((1u128 &lt;&lt; 64) - 1)) <b>as</b> u64;
    (is_bid, price, order_id)
}
</code></pre>



</details>
