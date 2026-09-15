---
title: Module `bridge::usdc`
---

Bridged USDC type owned by the Bridge package so <code>MyUsdPeg</code> can hold <code>Balance&lt;<a href="../bridge/usdc.md#bridge_usdc_USDC">USDC</a>&gt;</code>.


-  [Struct `USDC`](#bridge_usdc_USDC)
-  [Constants](#@Constants_0)
-  [Function `decimals`](#bridge_usdc_decimals)


<pre><code></code></pre>



<a name="bridge_usdc_USDC"></a>

## Struct `USDC`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/usdc.md#bridge_usdc_USDC">USDC</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_usdc_DECIMAL"></a>



<pre><code><b>const</b> <a href="../bridge/usdc.md#bridge_usdc_DECIMAL">DECIMAL</a>: u8 = 6;
</code></pre>



<a name="bridge_usdc_decimals"></a>

## Function `decimals`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/usdc.md#bridge_usdc_decimals">decimals</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/usdc.md#bridge_usdc_decimals">decimals</a>(): u8 {
    <a href="../bridge/usdc.md#bridge_usdc_DECIMAL">DECIMAL</a>
}
</code></pre>



</details>
