---
title: Module `bridge::myusd`
---

Platform MyUSD type owned by the Bridge package so <code>MyUsdPeg</code> can hold <code>TreasuryCap&lt;<a href="../bridge/myusd.md#bridge_myusd_MYUSD">MYUSD</a>&gt;</code>.


-  [Struct `MYUSD`](#bridge_myusd_MYUSD)
-  [Constants](#@Constants_0)
-  [Function `decimals`](#bridge_myusd_decimals)


<pre><code></code></pre>



<a name="bridge_myusd_MYUSD"></a>

## Struct `MYUSD`



<pre><code><b>public</b> <b>struct</b> <a href="../bridge/myusd.md#bridge_myusd_MYUSD">MYUSD</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="bridge_myusd_DECIMAL"></a>



<pre><code><b>const</b> <a href="../bridge/myusd.md#bridge_myusd_DECIMAL">DECIMAL</a>: u8 = 6;
</code></pre>



<a name="bridge_myusd_decimals"></a>

## Function `decimals`



<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd.md#bridge_myusd_decimals">decimals</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../bridge/myusd.md#bridge_myusd_decimals">decimals</a>(): u8 {
    <a href="../bridge/myusd.md#bridge_myusd_DECIMAL">DECIMAL</a>
}
</code></pre>



</details>
