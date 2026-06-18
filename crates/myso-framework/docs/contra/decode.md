---
title: Module `contra::decode`
---

Simple deserialization functions that build the composite crypto types from their
byte-encoded elements in a single Move call.


-  [Constants](#@Constants_0)
-  [Function `g_vector`](#contra_decode_g_vector)
-  [Function `encryption`](#contra_decode_encryption)
-  [Function `encrypted_amount`](#contra_decode_encrypted_amount)
-  [Function `multi_recipient_encryption`](#contra_decode_multi_recipient_encryption)
-  [Function `ddh_proof`](#contra_decode_ddh_proof)
-  [Function `elgamal_proof`](#contra_decode_elgamal_proof)
-  [Function `consistency_proof`](#contra_decode_consistency_proof)
-  [Function `key_consistency_proof`](#contra_decode_key_consistency_proof)
-  [Function `encryption_at`](#contra_decode_encryption_at)
-  [Function `elgamal_proof_at`](#contra_decode_elgamal_proof_at)
-  [Function `g_range`](#contra_decode_g_range)
-  [Function `scalar_range`](#contra_decode_scalar_range)


<pre><code><b>use</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount">contra::encrypted_amount</a>;
<b>use</b> <a href="../contra/nizk.md#contra_nizk">contra::nizk</a>;
<b>use</b> <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal">contra::twisted_elgamal</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/group_ops.md#myso_group_ops">myso::group_ops</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/rangeproofs.md#myso_rangeproofs">myso::rangeproofs</a>;
<b>use</b> <a href="../myso/ristretto255.md#myso_ristretto255">myso::ristretto255</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="contra_decode_KEY_LIMBS"></a>



<pre><code><b>const</b> <a href="../contra/decode.md#contra_decode_KEY_LIMBS">KEY_LIMBS</a>: u64 = 8;
</code></pre>



<a name="contra_decode_g_vector"></a>

## Function `g_vector`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_g_vector">g_vector</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_g_vector">g_vector</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): vector&lt;Element&lt;G&gt;&gt; {
    parts.map!(|b| g_from_bytes(&b))
}
</code></pre>



</details>

<a name="contra_decode_encryption"></a>

## Function `encryption`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_encryption">encryption</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_encryption">encryption</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): Encryption {
    <a href="../contra/decode.md#contra_decode_encryption_at">encryption_at</a>(&parts, 0)
}
</code></pre>



</details>

<a name="contra_decode_encrypted_amount"></a>

## Function `encrypted_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount">encrypted_amount</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount">encrypted_amount</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): EncryptedAmount {
    new_encrypted_amount(
        <a href="../contra/decode.md#contra_decode_encryption_at">encryption_at</a>(&parts, 0),
        <a href="../contra/decode.md#contra_decode_encryption_at">encryption_at</a>(&parts, 2),
        <a href="../contra/decode.md#contra_decode_encryption_at">encryption_at</a>(&parts, 4),
        <a href="../contra/decode.md#contra_decode_encryption_at">encryption_at</a>(&parts, 6),
    )
}
</code></pre>



</details>

<a name="contra_decode_multi_recipient_encryption"></a>

## Function `multi_recipient_encryption`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_multi_recipient_encryption">multi_recipient_encryption</a>(parts: vector&lt;vector&lt;u8&gt;&gt;, m: u64): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">contra::twisted_elgamal::MultiRecipientEncryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_multi_recipient_encryption">multi_recipient_encryption</a>(parts: vector&lt;vector&lt;u8&gt;&gt;, m: u64): MultiRecipientEncryption {
    <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_new_multi_recipient_encryption">twisted_elgamal::new_multi_recipient_encryption</a>(
        g_from_bytes(parts.borrow(0)),
        <a href="../contra/decode.md#contra_decode_g_range">g_range</a>(&parts, 1, m),
    )
}
</code></pre>



</details>

<a name="contra_decode_ddh_proof"></a>

## Function `ddh_proof`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_ddh_proof">ddh_proof</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): <a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_ddh_proof">ddh_proof</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): DdhProof {
    <a href="../contra/nizk.md#contra_nizk_new_ddh_proof">nizk::new_ddh_proof</a>(
        g_from_bytes(parts.borrow(0)),
        g_from_bytes(parts.borrow(1)),
        scalar_from_bytes(parts.borrow(2)),
    )
}
</code></pre>



</details>

<a name="contra_decode_elgamal_proof"></a>

## Function `elgamal_proof`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_elgamal_proof">elgamal_proof</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_elgamal_proof">elgamal_proof</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): ElGamalProof {
    <a href="../contra/decode.md#contra_decode_elgamal_proof_at">elgamal_proof_at</a>(&parts, 0)
}
</code></pre>



</details>

<a name="contra_decode_consistency_proof"></a>

## Function `consistency_proof`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_consistency_proof">consistency_proof</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">contra::encrypted_amount::ConsistencyProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_consistency_proof">consistency_proof</a>(parts: vector&lt;vector&lt;u8&gt;&gt;): ConsistencyProof {
    new_consistency_proof(
        <a href="../contra/decode.md#contra_decode_elgamal_proof_at">elgamal_proof_at</a>(&parts, 0),
        <a href="../contra/decode.md#contra_decode_elgamal_proof_at">elgamal_proof_at</a>(&parts, 4),
        <a href="../contra/decode.md#contra_decode_elgamal_proof_at">elgamal_proof_at</a>(&parts, 8),
        <a href="../contra/decode.md#contra_decode_elgamal_proof_at">elgamal_proof_at</a>(&parts, 12),
    )
}
</code></pre>



</details>

<a name="contra_decode_key_consistency_proof"></a>

## Function `key_consistency_proof`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_key_consistency_proof">key_consistency_proof</a>(parts: vector&lt;vector&lt;u8&gt;&gt;, m: u64): <a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">contra::nizk::KeyConsistencyProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/decode.md#contra_decode_key_consistency_proof">key_consistency_proof</a>(parts: vector&lt;vector&lt;u8&gt;&gt;, m: u64): KeyConsistencyProof {
    <b>let</b> a1_count = <a href="../contra/decode.md#contra_decode_KEY_LIMBS">KEY_LIMBS</a> * m;
    <b>let</b> a2_start = a1_count;
    <b>let</b> a3_idx = a2_start + <a href="../contra/decode.md#contra_decode_KEY_LIMBS">KEY_LIMBS</a>;
    <b>let</b> z1_start = a3_idx + 1;
    <b>let</b> z2_start = z1_start + <a href="../contra/decode.md#contra_decode_KEY_LIMBS">KEY_LIMBS</a>;
    <a href="../contra/nizk.md#contra_nizk_new_key_consistency_proof">nizk::new_key_consistency_proof</a>(
        <a href="../contra/decode.md#contra_decode_g_range">g_range</a>(&parts, 0, a1_count),
        <a href="../contra/decode.md#contra_decode_g_range">g_range</a>(&parts, a2_start, <a href="../contra/decode.md#contra_decode_KEY_LIMBS">KEY_LIMBS</a>),
        g_from_bytes(parts.borrow(a3_idx)),
        <a href="../contra/decode.md#contra_decode_scalar_range">scalar_range</a>(&parts, z1_start, <a href="../contra/decode.md#contra_decode_KEY_LIMBS">KEY_LIMBS</a>),
        <a href="../contra/decode.md#contra_decode_scalar_range">scalar_range</a>(&parts, z2_start, <a href="../contra/decode.md#contra_decode_KEY_LIMBS">KEY_LIMBS</a>),
    )
}
</code></pre>



</details>

<a name="contra_decode_encryption_at"></a>

## Function `encryption_at`



<pre><code><b>fun</b> <a href="../contra/decode.md#contra_decode_encryption_at">encryption_at</a>(parts: &vector&lt;vector&lt;u8&gt;&gt;, off: u64): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/decode.md#contra_decode_encryption_at">encryption_at</a>(parts: &vector&lt;vector&lt;u8&gt;&gt;, off: u64): Encryption {
    <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_new">twisted_elgamal::new</a>(g_from_bytes(parts.borrow(off)), g_from_bytes(parts.borrow(off + 1)))
}
</code></pre>



</details>

<a name="contra_decode_elgamal_proof_at"></a>

## Function `elgamal_proof_at`



<pre><code><b>fun</b> <a href="../contra/decode.md#contra_decode_elgamal_proof_at">elgamal_proof_at</a>(parts: &vector&lt;vector&lt;u8&gt;&gt;, off: u64): <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/decode.md#contra_decode_elgamal_proof_at">elgamal_proof_at</a>(parts: &vector&lt;vector&lt;u8&gt;&gt;, off: u64): ElGamalProof {
    <a href="../contra/nizk.md#contra_nizk_new_elgamal_proof">nizk::new_elgamal_proof</a>(
        g_from_bytes(parts.borrow(off)),
        g_from_bytes(parts.borrow(off + 1)),
        scalar_from_bytes(parts.borrow(off + 2)),
        scalar_from_bytes(parts.borrow(off + 3)),
    )
}
</code></pre>



</details>

<a name="contra_decode_g_range"></a>

## Function `g_range`



<pre><code><b>fun</b> <a href="../contra/decode.md#contra_decode_g_range">g_range</a>(parts: &vector&lt;vector&lt;u8&gt;&gt;, start: u64, count: u64): vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/decode.md#contra_decode_g_range">g_range</a>(parts: &vector&lt;vector&lt;u8&gt;&gt;, start: u64, count: u64): vector&lt;Element&lt;G&gt;&gt; {
    <b>let</b> <b>mut</b> out = vector[];
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; count) {
        out.push_back(g_from_bytes(parts.borrow(start + i)));
        i = i + 1;
    };
    out
}
</code></pre>



</details>

<a name="contra_decode_scalar_range"></a>

## Function `scalar_range`



<pre><code><b>fun</b> <a href="../contra/decode.md#contra_decode_scalar_range">scalar_range</a>(parts: &vector&lt;vector&lt;u8&gt;&gt;, start: u64, count: u64): vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_Scalar">myso::ristretto255::Scalar</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/decode.md#contra_decode_scalar_range">scalar_range</a>(parts: &vector&lt;vector&lt;u8&gt;&gt;, start: u64, count: u64): vector&lt;Element&lt;Scalar&gt;&gt; {
    <b>let</b> <b>mut</b> out = vector[];
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; count) {
        out.push_back(scalar_from_bytes(parts.borrow(start + i)));
        i = i + 1;
    };
    out
}
</code></pre>



</details>
