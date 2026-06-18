---
title: Module `contra::nizk`
---



-  [Struct `DdhProof`](#contra_nizk_DdhProof)
-  [Struct `ElGamalProof`](#contra_nizk_ElGamalProof)
-  [Struct `KeyConsistencyProof`](#contra_nizk_KeyConsistencyProof)
-  [Constants](#@Constants_0)
-  [Function `new_ddh_proof`](#contra_nizk_new_ddh_proof)
-  [Function `new_elgamal_proof`](#contra_nizk_new_elgamal_proof)
-  [Function `new_key_consistency_proof`](#contra_nizk_new_key_consistency_proof)
-  [Function `verify_ddh`](#contra_nizk_verify_ddh)
-  [Function `verify_elgamal`](#contra_nizk_verify_elgamal)
-  [Function `verify_key_consistency`](#contra_nizk_verify_key_consistency)
-  [Function `challenge_ddh`](#contra_nizk_challenge_ddh)
-  [Function `challenge_elgamal`](#contra_nizk_challenge_elgamal)
-  [Function `challenge_key_consistency`](#contra_nizk_challenge_key_consistency)
-  [Function `fiat_shamir_challenge`](#contra_nizk_fiat_shamir_challenge)
-  [Function `is_valid_relation`](#contra_nizk_is_valid_relation)


<pre><code><b>use</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal">contra::twisted_elgamal</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/group_ops.md#myso_group_ops">myso::group_ops</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/ristretto255.md#myso_ristretto255">myso::ristretto255</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="contra_nizk_DdhProof"></a>

## Struct `DdhProof`

A non-interactive zero knowledge proof of the following relation:
Prover knows <code>x</code> such that <code>x_g = x * g</code> and <code>x_h = x * h</code>, where <code>g</code> and <code>h</code> are generators of the group.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/nizk.md#contra_nizk_DdhProof">DdhProof</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>a: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>b: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>z: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_nizk_ElGamalProof"></a>

## Struct `ElGamalProof`

A non-interactive zero knowledge proof of the following relation:
Prover knows <code>r</code> and <code>m</code> such that <code>c = r * g + m * h</code> and <code>d = r * pk</code> where <code>(c, d)</code> is a twisted ElGamal ciphertext,
<code>pk</code> is a public key, and <code>g</code> and <code>h</code> are generators of the group.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/nizk.md#contra_nizk_ElGamalProof">ElGamalProof</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>a: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>b: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>z1: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>z2: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_nizk_KeyConsistencyProof"></a>

## Struct `KeyConsistencyProof`

A non-interactive zero knowledge proof of knowledge showing that the eight 32-bit limbs of a
256-bit private key are correctly encrypted to a list of m recipient public keys <code>pk_j</code> using
Twisted ElGamal. The proof shows that the prover knows randomness <code>(r_1, ..., r_8)</code> and key
limbs <code>(u_1, ..., u_8)</code> such that:
- <code>D_ij = r_i * pk_j</code> for all i and j, where <code>D_ij</code> is the decryption handle for the i-th limb
and j-th recipient.
- <code>C_i = r_i * g + u_i * h</code> for all i, where <code>C_i</code> is the ciphertext for the i-th limb.
- <code>(\sum_i u_i * 2^{32i}) * g == sender_public_key</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">KeyConsistencyProof</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>a1: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>a2: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>a3: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>z1: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>z2: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="contra_nizk_KEY_CONSISTENCY_LIMBS"></a>

Number of 32-bit limbs in a <code><a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">KeyConsistencyProof</a></code>. A 256-bit Ristretto255 scalar split into
32-bit chunks gives exactly 8 limbs; this is fixed by the protocol, not negotiable per call.


<pre><code><b>const</b> <a href="../contra/nizk.md#contra_nizk_KEY_CONSISTENCY_LIMBS">KEY_CONSISTENCY_LIMBS</a>: u64 = 8;
</code></pre>



<a name="contra_nizk_EMalformedKeyConsistencyProof"></a>

<code><a href="../contra/nizk.md#contra_nizk_verify_key_consistency">verify_key_consistency</a></code> was called with vectors whose lengths don't match the protocol
constants (<code><a href="../contra/nizk.md#contra_nizk_KEY_CONSISTENCY_LIMBS">KEY_CONSISTENCY_LIMBS</a></code> limbs, <code>recipient_encryption_keys.length()</code> recipients).


<pre><code><b>const</b> <a href="../contra/nizk.md#contra_nizk_EMalformedKeyConsistencyProof">EMalformedKeyConsistencyProof</a>: u64 = 0;
</code></pre>



<a name="contra_nizk_new_ddh_proof"></a>

## Function `new_ddh_proof`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/nizk.md#contra_nizk_new_ddh_proof">new_ddh_proof</a>(a: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, b: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, z: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;): <a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/nizk.md#contra_nizk_new_ddh_proof">new_ddh_proof</a>(a: Element&lt;G&gt;, b: Element&lt;G&gt;, z: Element&lt;Scalar&gt;): <a href="../contra/nizk.md#contra_nizk_DdhProof">DdhProof</a> {
    <a href="../contra/nizk.md#contra_nizk_DdhProof">DdhProof</a> { a, b, z }
}
</code></pre>



</details>

<a name="contra_nizk_new_elgamal_proof"></a>

## Function `new_elgamal_proof`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/nizk.md#contra_nizk_new_elgamal_proof">new_elgamal_proof</a>(a: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, b: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, z1: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;, z2: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;): <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/nizk.md#contra_nizk_new_elgamal_proof">new_elgamal_proof</a>(
    a: Element&lt;G&gt;,
    b: Element&lt;G&gt;,
    z1: Element&lt;Scalar&gt;,
    z2: Element&lt;Scalar&gt;,
): <a href="../contra/nizk.md#contra_nizk_ElGamalProof">ElGamalProof</a> {
    <a href="../contra/nizk.md#contra_nizk_ElGamalProof">ElGamalProof</a> { a, b, z1, z2 }
}
</code></pre>



</details>

<a name="contra_nizk_new_key_consistency_proof"></a>

## Function `new_key_consistency_proof`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/nizk.md#contra_nizk_new_key_consistency_proof">new_key_consistency_proof</a>(a1: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, a2: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, a3: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, z1: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;&gt;, z2: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;&gt;): <a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">contra::nizk::KeyConsistencyProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/nizk.md#contra_nizk_new_key_consistency_proof">new_key_consistency_proof</a>(
    a1: vector&lt;Element&lt;G&gt;&gt;,
    a2: vector&lt;Element&lt;G&gt;&gt;,
    a3: Element&lt;G&gt;,
    z1: vector&lt;Element&lt;Scalar&gt;&gt;,
    z2: vector&lt;Element&lt;Scalar&gt;&gt;,
): <a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">KeyConsistencyProof</a> {
    <a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">KeyConsistencyProof</a> { a1, a2, a3, z1, z2 }
}
</code></pre>



</details>

<a name="contra_nizk_verify_ddh"></a>

## Function `verify_ddh`

Verify a DDH proof that the prover knows <code>x</code> such that <code>x_g = x * g</code> and <code>x_h = x * h</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/nizk.md#contra_nizk_verify_ddh">verify_ddh</a>(proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, dst: vector&lt;u8&gt;, g: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, h: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, x_g: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, x_h: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/nizk.md#contra_nizk_verify_ddh">verify_ddh</a>(
    proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">DdhProof</a>,
    dst: vector&lt;u8&gt;,
    g: &Element&lt;G&gt;,
    h: &Element&lt;G&gt;,
    x_g: &Element&lt;G&gt;,
    x_h: &Element&lt;G&gt;,
): bool {
    // TODO: check <b>for</b> degenerate case where g or h is the identity element.
    <b>let</b> challenge = <a href="../contra/nizk.md#contra_nizk_challenge_ddh">challenge_ddh</a>(dst, g, h, x_g, x_h, &proof.a, &proof.b);
    <a href="../contra/nizk.md#contra_nizk_is_valid_relation">is_valid_relation</a>(
        &proof.a,
        x_g,
        g,
        &proof.z,
        &challenge,
    ) && <a href="../contra/nizk.md#contra_nizk_is_valid_relation">is_valid_relation</a>(
        &proof.b,
        x_h,
        h,
        &proof.z,
        &challenge,
    )
}
</code></pre>



</details>

<a name="contra_nizk_verify_elgamal"></a>

## Function `verify_elgamal`

Verify that the prover knows the message <code>m</code> and randomness <code>r</code> in a twisted ElGamal
encryption <code>(e1 = r*g + m*h, e2 = r*pk)</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/nizk.md#contra_nizk_verify_elgamal">verify_elgamal</a>(proof: &<a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a>, dst: vector&lt;u8&gt;, pk: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, e: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/nizk.md#contra_nizk_verify_elgamal">verify_elgamal</a>(
    proof: &<a href="../contra/nizk.md#contra_nizk_ElGamalProof">ElGamalProof</a>,
    dst: vector&lt;u8&gt;,
    pk: &Element&lt;G&gt;,
    e: &Encryption,
): bool {
    <b>let</b> g = <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">twisted_elgamal::g</a>();
    <b>let</b> h = <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">twisted_elgamal::h</a>();
    // TODO: can skip fixed g, h (left <b>as</b> a defense in depth)
    <b>let</b> e1 = e.ciphertext();
    <b>let</b> e2 = e.decryption_handle();
    <b>let</b> challenge = <a href="../contra/nizk.md#contra_nizk_challenge_elgamal">challenge_elgamal</a>(dst, &g, &h, pk, e1, e2, &proof.a, &proof.b);
    // Equation 1: z1 * pk == ch * e2 + a
    // Equation 2: ch * e1 + b == z1 * g + z2 * h
    <b>return</b> g_mul(&proof.z1, pk) == g_add(&g_mul(&challenge, e2), &proof.a)
    && g_add(&g_mul(&challenge, e1), &proof.b) == g_add(&g_mul(&proof.z1, &g), &g_mul(&proof.z2, &h))
}
</code></pre>



</details>

<a name="contra_nizk_verify_key_consistency"></a>

## Function `verify_key_consistency`

Verify a <code><a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">KeyConsistencyProof</a></code> against the recipient public keys and <code>encryptions[i]</code>, the
i-th-limb <code>MultiRecipientEncryption</code> (one shared <code>ciphertext</code> + one <code>decryption_handle</code> per
recipient).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/nizk.md#contra_nizk_verify_key_consistency">verify_key_consistency</a>(proof: &<a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">contra::nizk::KeyConsistencyProof</a>, dst: vector&lt;u8&gt;, sender_public_key: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, recipient_encryption_keys: &vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, encryptions: &vector&lt;<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">contra::twisted_elgamal::MultiRecipientEncryption</a>&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/nizk.md#contra_nizk_verify_key_consistency">verify_key_consistency</a>(
    proof: &<a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">KeyConsistencyProof</a>,
    dst: vector&lt;u8&gt;,
    sender_public_key: &Element&lt;G&gt;,
    recipient_encryption_keys: &vector&lt;Element&lt;G&gt;&gt;,
    encryptions: &vector&lt;MultiRecipientEncryption&gt;,
): bool {
    <b>let</b> n = <a href="../contra/nizk.md#contra_nizk_KEY_CONSISTENCY_LIMBS">KEY_CONSISTENCY_LIMBS</a>;
    <b>let</b> m = recipient_encryption_keys.length();
    <b>assert</b>!(proof.a1.length() == n * m, <a href="../contra/nizk.md#contra_nizk_EMalformedKeyConsistencyProof">EMalformedKeyConsistencyProof</a>);
    <b>assert</b>!(proof.a2.length() == n, <a href="../contra/nizk.md#contra_nizk_EMalformedKeyConsistencyProof">EMalformedKeyConsistencyProof</a>);
    <b>assert</b>!(proof.z1.length() == n, <a href="../contra/nizk.md#contra_nizk_EMalformedKeyConsistencyProof">EMalformedKeyConsistencyProof</a>);
    <b>assert</b>!(proof.z2.length() == n, <a href="../contra/nizk.md#contra_nizk_EMalformedKeyConsistencyProof">EMalformedKeyConsistencyProof</a>);
    <b>assert</b>!(encryptions.length() == n, <a href="../contra/nizk.md#contra_nizk_EMalformedKeyConsistencyProof">EMalformedKeyConsistencyProof</a>);
    encryptions.do_ref!(
        |e| <b>assert</b>!(
            e.multi_recipient_decryption_handles().length() == m,
            <a href="../contra/nizk.md#contra_nizk_EMalformedKeyConsistencyProof">EMalformedKeyConsistencyProof</a>,
        ),
    );
    <b>let</b> g = <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_g">twisted_elgamal::g</a>();
    <b>let</b> h = <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_h">twisted_elgamal::h</a>();
    // TODO: can skip fixed g, h (left <b>as</b> a defense in depth)
    <b>let</b> c = <a href="../contra/nizk.md#contra_nizk_challenge_key_consistency">challenge_key_consistency</a>(
        dst,
        &g,
        &h,
        sender_public_key,
        recipient_encryption_keys,
        encryptions,
        &proof.a1,
        &proof.a2,
        &proof.a3,
    );
    // Check 1: A1_ij + c * D_ij == z1_i * pk_j <b>for</b> all (i, j)
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> z1i = &proof.z1[i];
        <b>let</b> dhs = encryptions[i].multi_recipient_decryption_handles();
        <b>let</b> <b>mut</b> j = 0;
        <b>while</b> (j &lt; m) {
            <b>let</b> a1ij = &proof.a1[i * m + j];
            <b>let</b> dij = &dhs[j];
            <b>let</b> pkj = &recipient_encryption_keys[j];
            <b>if</b> (g_add(a1ij, &g_mul(&c, dij)) != g_mul(z1i, pkj)) <b>return</b> <b>false</b>;
            j = j + 1;
        };
        i = i + 1;
    };
    // Check 2: A2_i + c * C_i == z1_i * g + z2_i * h <b>for</b> all i
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> z1i = &proof.z1[i];
        <b>let</b> z2i = &proof.z2[i];
        <b>let</b> a2i = &proof.a2[i];
        <b>let</b> ci = encryptions[i].multi_recipient_ciphertext();
        <b>if</b> (g_add(a2i, &g_mul(&c, ci)) != g_add(&g_mul(z1i, &g), &g_mul(z2i, &h))) <b>return</b> <b>false</b>;
        i = i + 1;
    };
    // Check 3: (\sum_i z2_i * 2^{32i}) * g == A3 + c * sender_public_key
    <b>let</b> base = scalar_from_u64(1u64 &lt;&lt; 32);
    <b>let</b> <b>mut</b> exp = scalar_from_u64(1u64);
    <b>let</b> <b>mut</b> z_sum = scalar_from_u64(0u64);
    n.do!(|i| {
        <b>let</b> z2i = &proof.z2[i];
        z_sum = scalar_add(&z_sum, &scalar_mul(z2i, &exp));
        exp = scalar_mul(&exp, &base);
    });
    g_mul(&z_sum, &g) == g_add(&proof.a3, &g_mul(&c, sender_public_key))
}
</code></pre>



</details>

<a name="contra_nizk_challenge_ddh"></a>

## Function `challenge_ddh`



<pre><code><b>fun</b> <a href="../contra/nizk.md#contra_nizk_challenge_ddh">challenge_ddh</a>(dst: vector&lt;u8&gt;, g: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, h: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, x_g: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, x_h: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, a: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, b: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;): <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/nizk.md#contra_nizk_challenge_ddh">challenge_ddh</a>(
    dst: vector&lt;u8&gt;,
    g: &Element&lt;G&gt;,
    h: &Element&lt;G&gt;,
    x_g: &Element&lt;G&gt;,
    x_h: &Element&lt;G&gt;,
    a: &Element&lt;G&gt;,
    b: &Element&lt;G&gt;,
): Element&lt;Scalar&gt; {
    <a href="../contra/nizk.md#contra_nizk_fiat_shamir_challenge">fiat_shamir_challenge</a>(vector[
        dst,
        *g.bytes(),
        *h.bytes(),
        *x_g.bytes(),
        *x_h.bytes(),
        *a.bytes(),
        *b.bytes(),
    ])
}
</code></pre>



</details>

<a name="contra_nizk_challenge_elgamal"></a>

## Function `challenge_elgamal`



<pre><code><b>fun</b> <a href="../contra/nizk.md#contra_nizk_challenge_elgamal">challenge_elgamal</a>(dst: vector&lt;u8&gt;, g: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, h: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, pk: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, e1: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, e2: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, a: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, b: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;): <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/nizk.md#contra_nizk_challenge_elgamal">challenge_elgamal</a>(
    dst: vector&lt;u8&gt;,
    g: &Element&lt;G&gt;,
    h: &Element&lt;G&gt;,
    pk: &Element&lt;G&gt;,
    e1: &Element&lt;G&gt;,
    e2: &Element&lt;G&gt;,
    a: &Element&lt;G&gt;,
    b: &Element&lt;G&gt;,
): Element&lt;Scalar&gt; {
    <a href="../contra/nizk.md#contra_nizk_fiat_shamir_challenge">fiat_shamir_challenge</a>(vector[
        dst,
        *g.bytes(),
        *h.bytes(),
        *pk.bytes(),
        *e1.bytes(),
        *e2.bytes(),
        *a.bytes(),
        *b.bytes(),
    ])
}
</code></pre>



</details>

<a name="contra_nizk_challenge_key_consistency"></a>

## Function `challenge_key_consistency`

Compute the Fiat-Shamir challenge for a <code><a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">KeyConsistencyProof</a></code>. The transcript binds the bases
<code>g, h</code>, the sender public key, the recipient public keys, every per-limb ciphertext with its
decryption handles, and finally the prover commitments <code>(a1, a2, a3)</code>.


<pre><code><b>fun</b> <a href="../contra/nizk.md#contra_nizk_challenge_key_consistency">challenge_key_consistency</a>(dst: vector&lt;u8&gt;, g: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, h: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, sender_public_key: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, recipient_encryption_keys: &vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, encryptions: &vector&lt;<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">contra::twisted_elgamal::MultiRecipientEncryption</a>&gt;, a1: &vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, a2: &vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, a3: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;): <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/nizk.md#contra_nizk_challenge_key_consistency">challenge_key_consistency</a>(
    dst: vector&lt;u8&gt;,
    g: &Element&lt;G&gt;,
    h: &Element&lt;G&gt;,
    sender_public_key: &Element&lt;G&gt;,
    recipient_encryption_keys: &vector&lt;Element&lt;G&gt;&gt;,
    encryptions: &vector&lt;MultiRecipientEncryption&gt;,
    a1: &vector&lt;Element&lt;G&gt;&gt;,
    a2: &vector&lt;Element&lt;G&gt;&gt;,
    a3: &Element&lt;G&gt;,
): Element&lt;Scalar&gt; {
    <b>let</b> <b>mut</b> random_oracle_inputs = vector[dst, *g.bytes(), *h.bytes(), *sender_public_key.bytes()];
    recipient_encryption_keys.do_ref!(|rek| random_oracle_inputs.push_back(*rek.bytes()));
    // For each limb: first the commitment, then its decryption handles.
    encryptions.do_ref!(|e| {
        random_oracle_inputs.push_back(*e.multi_recipient_ciphertext().bytes());
        e
            .multi_recipient_decryption_handles()
            .do_ref!(|dh| random_oracle_inputs.push_back(*dh.bytes()));
    });
    a1.do_ref!(|a1i| random_oracle_inputs.push_back(*a1i.bytes()));
    a2.do_ref!(|a2i| random_oracle_inputs.push_back(*a2i.bytes()));
    random_oracle_inputs.push_back(*a3.bytes());
    <a href="../contra/nizk.md#contra_nizk_fiat_shamir_challenge">fiat_shamir_challenge</a>(random_oracle_inputs)
}
</code></pre>



</details>

<a name="contra_nizk_fiat_shamir_challenge"></a>

## Function `fiat_shamir_challenge`



<pre><code><b>fun</b> <a href="../contra/nizk.md#contra_nizk_fiat_shamir_challenge">fiat_shamir_challenge</a>(random_oracle_inputs: vector&lt;vector&lt;u8&gt;&gt;): <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/nizk.md#contra_nizk_fiat_shamir_challenge">fiat_shamir_challenge</a>(random_oracle_inputs: vector&lt;vector&lt;u8&gt;&gt;): Element&lt;Scalar&gt; {
    <b>let</b> <b>mut</b> hash = <a href="../myso/hash.md#myso_hash_blake2b256">myso::hash::blake2b256</a>(&bcs::to_bytes(&random_oracle_inputs));
    // Clearing the top byte ensures the challenge is below the group order.
    // Fiat-Shamir only requires a large domain.
    *vector::borrow_mut(&<b>mut</b> hash, 31) = 0;
    scalar_from_bytes(&hash)
}
</code></pre>



</details>

<a name="contra_nizk_is_valid_relation"></a>

## Function `is_valid_relation`

Checks the linear relation: <code>e1 + c * e2 == z * e3</code>.


<pre><code><b>fun</b> <a href="../contra/nizk.md#contra_nizk_is_valid_relation">is_valid_relation</a>(e1: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, e2: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, e3: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, z: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;, c: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/nizk.md#contra_nizk_is_valid_relation">is_valid_relation</a>(
    e1: &Element&lt;G&gt;,
    e2: &Element&lt;G&gt;,
    e3: &Element&lt;G&gt;,
    z: &Element&lt;Scalar&gt;,
    c: &Element&lt;Scalar&gt;,
): bool {
    g_add(e1, &g_mul(c, e2)) == g_mul(z, e3)
}
</code></pre>



</details>
