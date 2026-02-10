---
title: Module `mydata::time`
---



-  [Constants](#@Constants_0)
-  [Function `check_staleness`](#mydata_time_check_staleness)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="mydata_time_EStaleFullnode"></a>



<pre><code><b>const</b> <a href="../mydata/staleness.md#mydata_time_EStaleFullnode">EStaleFullnode</a>: u64 = 93492;
</code></pre>



<a name="mydata_time_check_staleness"></a>

## Function `check_staleness`

Check that the state of the chain is not stale: Abort if the on-chain time is more than <code>allowed_staleness_in_ms</code> behind <code>now</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/staleness.md#mydata_time_check_staleness">check_staleness</a>(now: u64, allowed_staleness_in_ms: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/staleness.md#mydata_time_check_staleness">check_staleness</a>(now: u64, allowed_staleness_in_ms: u64, clock: &clock::Clock) {
    // If the clock timestamp is more recent, the check passes
    <b>let</b> timestamp = clock.timestamp_ms();
    <b>if</b> (now &lt; timestamp) {
        <b>return</b>
    };
    <b>assert</b>!(now - timestamp &lt;= allowed_staleness_in_ms, <a href="../mydata/staleness.md#mydata_time_EStaleFullnode">EStaleFullnode</a>);
}
</code></pre>



</details>
