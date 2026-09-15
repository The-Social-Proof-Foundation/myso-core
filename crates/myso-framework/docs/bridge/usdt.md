---
title: Module `bridge::usdt`
---

Bridged USDT type owned by the Bridge package so <code>MyUsdPeg</code> can hold <code>Balance&lt;<a href="../bridge/usdt.md#bridge_usdt_USDT">USDT</a>&gt;</code>.


-  [Struct `USDT`](#bridge_usdt_USDT)
-  [Constants](#@Constants_0)
-  [Function `decimals`](#bridge_usdt_decimals)


<pre><code></code></pre>



<a name="bridge_usdt_USDT"></a>

## Struct `USDT`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/usdt.md#bridge_usdt_USDT">USDT</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_usdt_DECIMAL"></a>



<pre><code><b>const</b> <a href="../bridge/usdt.md#bridge_usdt_DECIMAL">DECIMAL</a>: u8 = 6;
</code></pre>



<a name="bridge_usdt_decimals"></a>

## Function `decimals`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/usdt.md#bridge_usdt_decimals">decimals</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/usdt.md#bridge_usdt_decimals">decimals</a>(): u8 {
    <a href="../bridge/usdt.md#bridge_usdt_DECIMAL">DECIMAL</a>
}
</code></pre>



</details>
