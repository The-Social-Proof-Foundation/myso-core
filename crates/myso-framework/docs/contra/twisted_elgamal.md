---
title: Module `contra::twisted_elgamal`
---



-  [Struct `Encryption`](#contra_twisted_elgamal_Encryption)
-  [Struct `MultiRecipientEncryption`](#contra_twisted_elgamal_MultiRecipientEncryption)
-  [Function `new`](#contra_twisted_elgamal_new)
-  [Function `g`](#contra_twisted_elgamal_g)
-  [Function `h`](#contra_twisted_elgamal_h)
-  [Function `ciphertext`](#contra_twisted_elgamal_ciphertext)
-  [Function `decryption_handle`](#contra_twisted_elgamal_decryption_handle)
-  [Function `add`](#contra_twisted_elgamal_add)
-  [Function `sub`](#contra_twisted_elgamal_sub)
-  [Function `add_assign`](#contra_twisted_elgamal_add_assign)
-  [Function `sub_assign`](#contra_twisted_elgamal_sub_assign)
-  [Function `add_assign_u64`](#contra_twisted_elgamal_add_assign_u64)
-  [Function `sub_assign_u64`](#contra_twisted_elgamal_sub_assign_u64)
-  [Function `shift_left`](#contra_twisted_elgamal_shift_left)
-  [Function `encrypt_zero`](#contra_twisted_elgamal_encrypt_zero)
-  [Function `encrypt_trivial`](#contra_twisted_elgamal_encrypt_trivial)
-  [Function `new_multi_recipient_encryption`](#contra_twisted_elgamal_new_multi_recipient_encryption)
-  [Function `multi_recipient_ciphertext`](#contra_twisted_elgamal_multi_recipient_ciphertext)
-  [Function `multi_recipient_decryption_handles`](#contra_twisted_elgamal_multi_recipient_decryption_handles)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/group_ops.md#myso_group_ops">myso::group_ops</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/ristretto255.md#myso_ristretto255">myso::ristretto255</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="contra_twisted_elgamal_Encryption"></a>

## Struct `Encryption`

Twisted ElGamal encryption with message in the exponent, over Ristretto255.

Uses two generators with unknown discrete log relationship:
- <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a></code>: the standard Ristretto255 generator
- <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a></code>: derived via <code>hash_to_curve("fastcrypto-blinding-gen-01")</code>, ensuring no one knows <code>log_g(<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a>)</code>

Encryption of message <code>m</code> with public key <code>pk = x * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a></code> and randomness <code>r</code>:
- ciphertext:        <code>c = r * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a> + m * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a></code>
- decryption handle: <code>d = r * pk</code>

Decryption with secret key <code>x</code>:
- Compute <code>c - d/x = c - r*<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a> = m * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a></code>
- Solve the discrete log <code>m = log_h(m * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a>)</code> via brute force

Homomorphic properties: Encryptions can be added and subtracted component-wise,
yielding an encryption of the sum or difference of the plaintexts.

Values up to at least ~2^32 can be decrypted.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_twisted_elgamal_MultiRecipientEncryption"></a>

## Struct `MultiRecipientEncryption`

A single-ciphertext encryption readable by multiple recipients. Shares one <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a></code>
component across all recipients, with a separate <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a></code> per recipient public key.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">MultiRecipientEncryption</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>decryption_handles: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_twisted_elgamal_new"></a>

## Function `new`

Create a new Twisted ElGamal encryption from a given <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a></code> and <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_new">new</a>(<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_new">new</a>(<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: Element&lt;G&gt;, <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>: Element&lt;G&gt;): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
    <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>,
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>,
    }
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_g"></a>

## Function `g`

The standard Ristretto255 generator <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a></code>, used for randomness blinding in ciphertexts.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a>(): <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a>(): Element&lt;G&gt; {
    ristretto255::g_generator()
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_h"></a>

## Function `h`

The blinding generator <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a></code>, derived via <code>hash_to_curve("fastcrypto-blinding-gen-01")</code>.
The discrete log relationship between <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a></code> and <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a></code> is unknown.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a>(): <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a>(): Element&lt;G&gt; {
    g_from_bytes(
        &x"34ce1477c14558178089500a39c864e0f607b3c1f41ab398400e4a9de6d2c446",
    )
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_ciphertext"></a>

## Function `ciphertext`

Returns the ciphertext of a Twisted ElGamal encryption <code>c = r * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a> + m * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>(e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>): &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>(e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>): &Element&lt;G&gt; {
    &e.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_decryption_handle"></a>

## Function `decryption_handle`

Returns the decryption handle of a Twisted ElGamal encryption <code>d = r * pk</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>(e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>): &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>(e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>): &Element&lt;G&gt; {
    &e.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_add"></a>

## Function `add`

Homomorphically add two Twisted ElGamal encryptions. The result is an encryption of the sum of the plaintexts
in the scalar field.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_add">add</a>(e1: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, e2: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_add">add</a>(e1: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>, e2: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
    <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: g_add(&e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>, &e2.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>),
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>: g_add(&e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>, &e2.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>),
    }
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_sub"></a>

## Function `sub`

Homomorphically subtract two Twisted ElGamal encryptions. The result is an encryption of the difference of the
plaintexts in the scalar field.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_sub">sub</a>(e1: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, e2: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_sub">sub</a>(e1: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>, e2: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
    <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: g_sub(&e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>, &e2.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>),
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>: g_sub(&e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>, &e2.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>),
    }
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_add_assign"></a>

## Function `add_assign`

In-place version of <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_add">add</a></code>: <code>e1</code> becomes the homomorphic sum <code>e1 + e2</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_add_assign">add_assign</a>(e1: &<b>mut</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, e2: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_add_assign">add_assign</a>(e1: &<b>mut</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>, e2: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>) {
    e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a> = g_add(&e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>, &e2.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>);
    e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a> = g_add(&e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>, &e2.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>);
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_sub_assign"></a>

## Function `sub_assign`

In-place version of <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_sub">sub</a></code>: <code>e1</code> becomes the homomorphic difference <code>e1 - e2</code>.
Beware of plaintext-side overflow in the scalar field.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_sub_assign">sub_assign</a>(e1: &<b>mut</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, e2: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_sub_assign">sub_assign</a>(e1: &<b>mut</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>, e2: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>) {
    e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a> = g_sub(&e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>, &e2.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>);
    e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a> = g_sub(&e1.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>, &e2.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>);
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_add_assign_u64"></a>

## Function `add_assign_u64`

Add a known public <code>amount</code> to the ciphertext.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_add_assign_u64">add_assign_u64</a>(e: &<b>mut</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_add_assign_u64">add_assign_u64</a>(e: &<b>mut</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>, amount: u64) {
    <b>if</b> (amount == 0) <b>return</b>;
    e.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a> = g_add(&e.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>, &g_mul(&scalar_from_u64(amount), &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a>()));
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_sub_assign_u64"></a>

## Function `sub_assign_u64`

Subtract a known public <code>amount</code> from the ciphertext.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_sub_assign_u64">sub_assign_u64</a>(e: &<b>mut</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, amount: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_sub_assign_u64">sub_assign_u64</a>(e: &<b>mut</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>, amount: u64) {
    <b>if</b> (amount == 0) <b>return</b>;
    e.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a> = g_sub(&e.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>, &g_mul(&scalar_from_u64(amount), &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a>()));
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_shift_left"></a>

## Function `shift_left`

Return an encryption of the same plaintext as the input but where the plaintext is multiplied by 2^bits.
The result is an encryption of the plaintext in the scalar field.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_shift_left">shift_left</a>(e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, bits: u8): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_shift_left">shift_left</a>(e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a>, bits: u8): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
    <b>let</b> factor = scalar_from_u64(1 &lt;&lt; bits);
    <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: g_mul(&factor, &e.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>),
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>: g_mul(&factor, &e.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>),
    }
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_encrypt_zero"></a>

## Function `encrypt_zero`

Trivial encryption of zero without randomness.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_encrypt_zero">encrypt_zero</a>(): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_encrypt_zero">encrypt_zero</a>(): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
    // TODO: consider changing to (pk, <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a>)
    <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: g_identity(),
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>: g_identity(),
    }
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_encrypt_trivial"></a>

## Function `encrypt_trivial`

Trivial encryption without randomness.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_encrypt_trivial">encrypt_trivial</a>(amount: u64): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_encrypt_trivial">encrypt_trivial</a>(amount: u64): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
    <b>if</b> (amount == 0) {
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_encrypt_zero">encrypt_zero</a>()
    } <b>else</b> {
        // TODO: consider changing to (pk, <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a> + amount*<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a>)
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">Encryption</a> {
            <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: g_mul(&scalar_from_u64(amount <b>as</b> u64), &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a>()),
            <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_decryption_handle">decryption_handle</a>: g_identity(),
        }
    }
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_new_multi_recipient_encryption"></a>

## Function `new_multi_recipient_encryption`

Construct a Twisted ElGamal <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">MultiRecipientEncryption</a></code> consisting of a shared ciphertext <code>c = r * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a> + m * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a></code> and
one decryption handle <code>d_i = r * pk_i</code> per recipient identified by their public key <code>pk_i</code>.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_new_multi_recipient_encryption">new_multi_recipient_encryption</a>(<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, decryption_handles: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">contra::twisted_elgamal::MultiRecipientEncryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_new_multi_recipient_encryption">new_multi_recipient_encryption</a>(
    <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>: Element&lt;G&gt;,
    decryption_handles: vector&lt;Element&lt;G&gt;&gt;,
): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">MultiRecipientEncryption</a> {
    <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">MultiRecipientEncryption</a> {
        <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>,
        decryption_handles,
    }
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_multi_recipient_ciphertext"></a>

## Function `multi_recipient_ciphertext`

Returns the shared ciphertext component <code>c = r * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">g</a> + m * <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">h</a></code> of a Twisted ElGamal <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">MultiRecipientEncryption</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_multi_recipient_ciphertext">multi_recipient_ciphertext</a>(e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">contra::twisted_elgamal::MultiRecipientEncryption</a>): &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_multi_recipient_ciphertext">multi_recipient_ciphertext</a>(e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">MultiRecipientEncryption</a>): &Element&lt;G&gt; {
    &e.<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_ciphertext">ciphertext</a>
}
</code></pre>



</details>

<a name="contra_twisted_elgamal_multi_recipient_decryption_handles"></a>

## Function `multi_recipient_decryption_handles`

Returns the per-recipient decryption handles <code>d_i = r * pk_i</code> for recipient public key <code>pk_i</code> of a
Twisted ElGamal <code><a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">MultiRecipientEncryption</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_multi_recipient_decryption_handles">multi_recipient_decryption_handles</a>(e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">contra::twisted_elgamal::MultiRecipientEncryption</a>): &vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_multi_recipient_decryption_handles">multi_recipient_decryption_handles</a>(e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">MultiRecipientEncryption</a>): &vector&lt;Element&lt;G&gt;&gt; {
    &e.decryption_handles
}
</code></pre>



</details>
