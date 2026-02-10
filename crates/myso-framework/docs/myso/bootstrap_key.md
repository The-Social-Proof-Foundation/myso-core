---
title: Module `myso::bootstrap_key`
---

Centralized one-time bootstrap key for framework and platform initialization.
Ensures all admin capabilities can only be created once during initial bootstrap.


-  [Struct `BootstrapKey`](#myso_bootstrap_key_BootstrapKey)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#myso_bootstrap_key_bootstrap_init)
-  [Function `is_used`](#myso_bootstrap_key_is_used)
-  [Function `version`](#myso_bootstrap_key_version)
-  [Function `assert_not_used`](#myso_bootstrap_key_assert_not_used)
-  [Function `finalize_bootstrap`](#myso_bootstrap_key_finalize_bootstrap)


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



<a name="myso_bootstrap_key_BootstrapKey"></a>

## Struct `BootstrapKey`

One-time bootstrap key - protects all admin capability creation


<pre><code><b>public</b> <b>struct</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">BootstrapKey</a> <b>has</b> key
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
<code>used: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../myso/bootstrap_key.md#myso_bootstrap_key_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="myso_bootstrap_key_EAlreadyUsed"></a>

Bootstrap key has already been used


<pre><code><b>const</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_EAlreadyUsed">EAlreadyUsed</a>: u64 = 0;
</code></pre>



<a name="myso_bootstrap_key_ENotSystemAddress"></a>

Sender is not @0x0 the system address.


<pre><code><b>const</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_ENotSystemAddress">ENotSystemAddress</a>: u64 = 1;
</code></pre>



<a name="myso_bootstrap_key_bootstrap_init"></a>

## Function `bootstrap_init`

Creates the shared BootstrapKey on module publication


<pre><code><b>public</b> <b>fun</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(ctx.sender() == @0x0, <a href="../myso/bootstrap_key.md#myso_bootstrap_key_ENotSystemAddress">ENotSystemAddress</a>);
    <a href="../myso/transfer.md#myso_transfer_share_object">transfer::share_object</a>(<a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">BootstrapKey</a> {
        id: <a href="../myso/object.md#myso_object_new">object::new</a>(ctx),
        used: <b>false</b>,
        <a href="../myso/bootstrap_key.md#myso_bootstrap_key_version">version</a>: 1,
    });
}
</code></pre>



</details>

<a name="myso_bootstrap_key_is_used"></a>

## Function `is_used`



<pre><code><b>public</b> <b>fun</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_is_used">is_used</a>(key: &<a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">myso::bootstrap_key::BootstrapKey</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_is_used">is_used</a>(key: &<a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">BootstrapKey</a>): bool {
    key.used
}
</code></pre>



</details>

<a name="myso_bootstrap_key_version"></a>

## Function `version`



<pre><code><b>public</b> <b>fun</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_version">version</a>(key: &<a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">myso::bootstrap_key::BootstrapKey</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_version">version</a>(key: &<a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">BootstrapKey</a>): u64 {
    key.<a href="../myso/bootstrap_key.md#myso_bootstrap_key_version">version</a>
}
</code></pre>



</details>

<a name="myso_bootstrap_key_assert_not_used"></a>

## Function `assert_not_used`

Aborts if the key has already been used


<pre><code><b>public</b> <b>fun</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_assert_not_used">assert_not_used</a>(key: &<a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">myso::bootstrap_key::BootstrapKey</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_assert_not_used">assert_not_used</a>(key: &<a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">BootstrapKey</a>) {
    <b>assert</b>!(!key.used, <a href="../myso/bootstrap_key.md#myso_bootstrap_key_EAlreadyUsed">EAlreadyUsed</a>);
}
</code></pre>



</details>

<a name="myso_bootstrap_key_finalize_bootstrap"></a>

## Function `finalize_bootstrap`

Finalize bootstrap by marking the key as used (irreversible)
This should ONLY be called after all admin capabilities have been created and distributed.
Combines the check and mark in one operation to prevent DOS attacks.


<pre><code><b>public</b> <b>fun</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_finalize_bootstrap">finalize_bootstrap</a>(key: &<b>mut</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">myso::bootstrap_key::BootstrapKey</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_finalize_bootstrap">finalize_bootstrap</a>(key: &<b>mut</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key_BootstrapKey">BootstrapKey</a>) {
    <a href="../myso/bootstrap_key.md#myso_bootstrap_key_assert_not_used">assert_not_used</a>(key);
    key.used = <b>true</b>;
}
</code></pre>



</details>
