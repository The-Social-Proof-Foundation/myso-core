---
title: Module `myso::clock`
---

APIs for accessing time from move calls, via the <code><a href="../myso/clock.md#myso_clock_Clock">Clock</a></code>: a unique
shared object that is created at 0x6 during genesis.


-  [Struct `Clock`](#myso_clock_Clock)
-  [Constants](#@Constants_0)
-  [Function `timestamp_ms`](#myso_clock_timestamp_ms)
-  [Function `create`](#myso_clock_create)
-  [Function `consensus_commit_prologue`](#myso_clock_consensus_commit_prologue)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
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



<a name="myso_clock_Clock"></a>

## Struct `Clock`

Singleton shared object that exposes time to Move calls.  This
object is found at address 0x6, and can only be read (accessed
via an immutable reference) by entry functions.

Entry Functions that attempt to accept <code><a href="../myso/clock.md#myso_clock_Clock">Clock</a></code> by mutable
reference or value will fail to verify, and honest validators
will not sign or execute transactions that use <code><a href="../myso/clock.md#myso_clock_Clock">Clock</a></code> as an
input parameter, unless it is passed by immutable reference.


<pre><code><b>public</b> <b>struct</b> <a href="../myso/clock.md#myso_clock_Clock">Clock</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso/clock.md#myso_clock_timestamp_ms">timestamp_ms</a>: u64</code>
</dt>
<dd>
 The clock's timestamp, which is set automatically by a
 system transaction every time consensus commits a
 schedule, or by <code>myso::clock::increment_for_testing</code> during
 testing.
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_clock_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../myso/clock.md#myso_clock_ENotSystemAddress">ENotSystemAddress</a>: u64 = 0;
</code></pre>



<a name="myso_clock_timestamp_ms"></a>

## Function `timestamp_ms`

The <code><a href="../myso/clock.md#myso_clock">clock</a></code>'s current timestamp as a running total of
milliseconds since an arbitrary point in the past.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/clock.md#myso_clock_timestamp_ms">timestamp_ms</a>(<a href="../myso/clock.md#myso_clock">clock</a>: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/clock.md#myso_clock_timestamp_ms">timestamp_ms</a>(<a href="../myso/clock.md#myso_clock">clock</a>: &<a href="../myso/clock.md#myso_clock_Clock">Clock</a>): u64 {
    <a href="../myso/clock.md#myso_clock">clock</a>.<a href="../myso/clock.md#myso_clock_timestamp_ms">timestamp_ms</a>
}
</code></pre>



</details>

<a name="myso_clock_create"></a>

## Function `create`

Create and share the singleton Clock -- this function is
called exactly once, during genesis.


<pre><code><b>fun</b> <a href="../myso/clock.md#myso_clock_create">create</a>(ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso/clock.md#myso_clock_create">create</a>(ctx: &TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../myso/clock.md#myso_clock_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../myso/transfer.md#myso_transfer_share_object">transfer::share_object</a>(<a href="../myso/clock.md#myso_clock_Clock">Clock</a> {
        id: <a href="../myso/object.md#myso_object_clock">object::clock</a>(),
        // Initialised to zero, but set to a real timestamp by a
        // system transaction before it can be witnessed by a <b>move</b>
        // call.
        <a href="../myso/clock.md#myso_clock_timestamp_ms">timestamp_ms</a>: 0,
    })
}
</code></pre>



</details>

<a name="myso_clock_consensus_commit_prologue"></a>

## Function `consensus_commit_prologue`



<pre><code><b>fun</b> <a href="../myso/clock.md#myso_clock_consensus_commit_prologue">consensus_commit_prologue</a>(<a href="../myso/clock.md#myso_clock">clock</a>: &<b>mut</b> <a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, <a href="../myso/clock.md#myso_clock_timestamp_ms">timestamp_ms</a>: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../myso/clock.md#myso_clock_consensus_commit_prologue">consensus_commit_prologue</a>(<a href="../myso/clock.md#myso_clock">clock</a>: &<b>mut</b> <a href="../myso/clock.md#myso_clock_Clock">Clock</a>, <a href="../myso/clock.md#myso_clock_timestamp_ms">timestamp_ms</a>: u64, ctx: &TxContext) {
    // Validator will make a special system call with sender set <b>as</b> 0x0.
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../myso/clock.md#myso_clock_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../myso/clock.md#myso_clock">clock</a>.<a href="../myso/clock.md#myso_clock_timestamp_ms">timestamp_ms</a> = <a href="../myso/clock.md#myso_clock_timestamp_ms">timestamp_ms</a>
}
</code></pre>



</details>
