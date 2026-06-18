---
title: Module `contra::encrypted_amount`
---



-  [Struct `EncryptedAmount`](#contra_encrypted_amount_EncryptedAmount)
-  [Struct `WellFormedEncryptedAmount`](#contra_encrypted_amount_WellFormedEncryptedAmount)
-  [Struct `ConsistencyProof`](#contra_encrypted_amount_ConsistencyProof)
-  [Struct `WellFormedProof`](#contra_encrypted_amount_WellFormedProof)
-  [Constants](#@Constants_0)
-  [Function `new_encrypted_amount`](#contra_encrypted_amount_new_encrypted_amount)
-  [Function `new_consistency_proof`](#contra_encrypted_amount_new_consistency_proof)
-  [Function `new_well_formed_proof`](#contra_encrypted_amount_new_well_formed_proof)
-  [Function `verify`](#contra_encrypted_amount_verify)
-  [Function `into_well_formed`](#contra_encrypted_amount_into_well_formed)
-  [Function `batch_into_well_formed`](#contra_encrypted_amount_batch_into_well_formed)
-  [Function `amount`](#contra_encrypted_amount_amount)
-  [Function `pk`](#contra_encrypted_amount_pk)
-  [Function `limb`](#contra_encrypted_amount_limb)
-  [Function `collapse`](#contra_encrypted_amount_collapse)
-  [Function `collapse_sum`](#contra_encrypted_amount_collapse_sum)
-  [Function `verify_equal`](#contra_encrypted_amount_verify_equal)
-  [Function `sum_commitments`](#contra_encrypted_amount_sum_commitments)
-  [Function `collapse_limbs`](#contra_encrypted_amount_collapse_limbs)
-  [Function `from_value`](#contra_encrypted_amount_from_value)
-  [Function `zero`](#contra_encrypted_amount_zero)
-  [Function `add_assign`](#contra_encrypted_amount_add_assign)
-  [Function `verify_well_formed_range_proofs`](#contra_encrypted_amount_verify_well_formed_range_proofs)
-  [Function `batch_sizes`](#contra_encrypted_amount_batch_sizes)
-  [Function `verify_well_formed_knowledge`](#contra_encrypted_amount_verify_well_formed_knowledge)


<pre><code><b>use</b> <a href="../contra/nizk.md#contra_nizk">contra::nizk</a>;
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



<a name="contra_encrypted_amount_EncryptedAmount"></a>

## Struct `EncryptedAmount`

Encrypted u64 amount stored as four u16 limbs that may overflow to at most u32.
The value is <code>l0 + 2^16 * l1 + 2^32 * l2 + 2^48 * l3</code>.
Overflows are prevented by the higher level protocols.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>l0: <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a></code>
</dt>
<dd>
</dd>
<dt>
<code>l1: <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a></code>
</dt>
<dd>
</dd>
<dt>
<code>l2: <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a></code>
</dt>
<dd>
</dd>
<dt>
<code>l3: <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_encrypted_amount_WellFormedEncryptedAmount"></a>

## Struct `WellFormedEncryptedAmount`

A wrapper around EncryptedAmount that has been verified to have the following properties:
1) The plaintexts for all limbs are at most 2^16.
2) All limbs are valid encryptions with respect to the given public key (in the Proof of Knowledge sense).


<pre><code><b>public</b> <b>struct</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_encrypted_amount_ConsistencyProof"></a>

## Struct `ConsistencyProof`

Per-amount ElGamal consistency: one sigma protocol per u16 limb. The public key isn't stored
here — the verifier supplies it at <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify">verify</a></code> time.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">ConsistencyProof</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>p0: <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a></code>
</dt>
<dd>
</dd>
<dt>
<code>p1: <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a></code>
</dt>
<dd>
</dd>
<dt>
<code>p2: <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a></code>
</dt>
<dd>
</dd>
<dt>
<code>p3: <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_encrypted_amount_WellFormedProof"></a>

## Struct `WellFormedProof`

Well-formedness proof: one Bulletproof per chunk of the canonical partition of
<code>consistency_proofs.length()</code> (see <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_sizes">batch_sizes</a></code>; e.g. N=7 → [4, 2, 1], N=20 → [8, 8, 4]),
plus one <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">ConsistencyProof</a></code> per amount. An empty <code>range_proofs</code> vector skips the range
check entirely — only reachable via the <code>#[test_only]</code> constructor.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">WellFormedProof</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>range_proofs: vector&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>consistency_proofs: vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">contra::encrypted_amount::ConsistencyProof</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="contra_encrypted_amount_BULLETPROOFS_VERSION"></a>

Bulletproof construction version. <code>0</code> is the original Bulletproofs construction
(Bünz et al., 2018), the only version currently supported by
<code><a href="../myso/rangeproofs.md#myso_rangeproofs_verify_bulletproofs_with_dst_ristretto255">myso::rangeproofs::verify_bulletproofs_with_dst_ristretto255</a></code>.


<pre><code><b>const</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_BULLETPROOFS_VERSION">BULLETPROOFS_VERSION</a>: u8 = 0;
</code></pre>



<a name="contra_encrypted_amount_LIMB_BITS"></a>

Bit-length used by the per-limb range check: each limb encrypts a u16, so the proof
must show every committed value lies in <code>[0, 2^16)</code>.


<pre><code><b>const</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_LIMB_BITS">LIMB_BITS</a>: u8 = 16;
</code></pre>



<a name="contra_encrypted_amount_MAX_BATCH_SIZE"></a>

Maximum number of amounts covered by a single Bulletproof chunk.
<code><a href="../myso/rangeproofs.md#myso_rangeproofs_verify_bulletproofs_with_dst_ristretto255">myso::rangeproofs::verify_bulletproofs_with_dst_ristretto255</a></code> caps the aggregated commitment count at
32 for <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_LIMB_BITS">LIMB_BITS</a> = 16</code>, and each amount contributes 4 limb commitments, so a single proof
covers at most <code>32 / 4 = 8</code> amounts.


<pre><code><b>const</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_MAX_BATCH_SIZE">MAX_BATCH_SIZE</a>: u64 = 8;
</code></pre>



<a name="contra_encrypted_amount_EIndexOutOfBounds"></a>



<pre><code><b>const</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EIndexOutOfBounds">EIndexOutOfBounds</a>: u64 = 2;
</code></pre>



<a name="contra_encrypted_amount_EMismatchedBatchLength"></a>



<pre><code><b>const</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EMismatchedBatchLength">EMismatchedBatchLength</a>: u64 = 3;
</code></pre>



<a name="contra_encrypted_amount_EWellFormedProofFailed"></a>



<pre><code><b>const</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EWellFormedProofFailed">EWellFormedProofFailed</a>: u64 = 4;
</code></pre>



<a name="contra_encrypted_amount_ERangeProofRequired"></a>



<pre><code><b>const</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_ERangeProofRequired">ERangeProofRequired</a>: u64 = 5;
</code></pre>



<a name="contra_encrypted_amount_new_encrypted_amount"></a>

## Function `new_encrypted_amount`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_new_encrypted_amount">new_encrypted_amount</a>(l0: <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, l1: <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, l2: <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, l3: <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_new_encrypted_amount">new_encrypted_amount</a>(
    l0: Encryption,
    l1: Encryption,
    l2: Encryption,
    l3: Encryption,
): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a> {
    <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a> { l0, l1, l2, l3 }
}
</code></pre>



</details>

<a name="contra_encrypted_amount_new_consistency_proof"></a>

## Function `new_consistency_proof`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_new_consistency_proof">new_consistency_proof</a>(p0: <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a>, p1: <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a>, p2: <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a>, p3: <a href="../contra/nizk.md#contra_nizk_ElGamalProof">contra::nizk::ElGamalProof</a>): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">contra::encrypted_amount::ConsistencyProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_new_consistency_proof">new_consistency_proof</a>(
    p0: ElGamalProof,
    p1: ElGamalProof,
    p2: ElGamalProof,
    p3: ElGamalProof,
): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">ConsistencyProof</a> {
    <a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">ConsistencyProof</a> { p0, p1, p2, p3 }
}
</code></pre>



</details>

<a name="contra_encrypted_amount_new_well_formed_proof"></a>

## Function `new_well_formed_proof`

Bundle range proofs and consistency proofs into a <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">WellFormedProof</a></code>. Pass one consistency
proof per amount and one range proof per <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_sizes">batch_sizes</a>(consistency_proofs.length())</code> chunk,
where each chunk's range proof covers that chunk's amounts (4 limbs each). Aborts on length
mismatch or empty <code>range_proofs[i]</code>; proofs are not verified here — callers must call
<code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify">verify</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_new_well_formed_proof">new_well_formed_proof</a>(range_proofs: vector&lt;vector&lt;u8&gt;&gt;, consistency_proofs: vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">contra::encrypted_amount::ConsistencyProof</a>&gt;): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_new_well_formed_proof">new_well_formed_proof</a>(
    range_proofs: vector&lt;vector&lt;u8&gt;&gt;,
    consistency_proofs: vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">ConsistencyProof</a>&gt;,
): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">WellFormedProof</a> {
    <b>assert</b>!(
        range_proofs.length() == <a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_sizes">batch_sizes</a>(consistency_proofs.length()).length(),
        <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EMismatchedBatchLength">EMismatchedBatchLength</a>,
    );
    <b>assert</b>!(range_proofs.all!(|rp| !rp.is_empty()), <a href="../contra/encrypted_amount.md#contra_encrypted_amount_ERangeProofRequired">ERangeProofRequired</a>);
    <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">WellFormedProof</a> { range_proofs, consistency_proofs }
}
</code></pre>



</details>

<a name="contra_encrypted_amount_verify"></a>

## Function `verify`

Check <code>proof</code> against <code>amounts</code> under <code>pks</code> and <code>dst</code>: every limb of every amount is u16 and
each amount is a valid ElGamal encryption to its matching <code>pks[i]</code>. Returns <code><b>false</b></code> on any
verification failure; aborts only on length mismatch between <code>amounts</code>, <code>pks</code>, and
<code>proof.consistency_proofs</code>. An empty <code>proof.range_proofs</code> skips the range check
entirely — only reachable via the <code>#[test_only]</code> constructor; <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_new_well_formed_proof">new_well_formed_proof</a></code> rejects
empty input.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify">verify</a>(proof: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>, dst: vector&lt;u8&gt;, amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>&gt;, pks: &vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify">verify</a>(
    proof: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">WellFormedProof</a>,
    dst: vector&lt;u8&gt;,
    amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a>&gt;,
    pks: &vector&lt;Element&lt;G&gt;&gt;,
): bool {
    <b>let</b> n = amounts.length();
    <b>assert</b>!(pks.length() == n, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EMismatchedBatchLength">EMismatchedBatchLength</a>);
    <b>assert</b>!(proof.consistency_proofs.length() == n, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EMismatchedBatchLength">EMismatchedBatchLength</a>);
    <b>assert</b>!(
        proof.range_proofs.is_empty() || proof.range_proofs.length() == <a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_sizes">batch_sizes</a>(n).length(),
        <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EMismatchedBatchLength">EMismatchedBatchLength</a>,
    );
    <a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify_well_formed_range_proofs">verify_well_formed_range_proofs</a>(amounts, &proof.range_proofs, dst)
    && <a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify_well_formed_knowledge">verify_well_formed_knowledge</a>(amounts, &proof.consistency_proofs, pks, dst)
}
</code></pre>



</details>

<a name="contra_encrypted_amount_into_well_formed"></a>

## Function `into_well_formed`

Verify <code>proof</code> (a batch-of-1 <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">WellFormedProof</a></code>) against <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a></code> under <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a></code> and <code>dst</code>, and
wrap into a <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a></code>. Aborts with <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_EWellFormedProofFailed">EWellFormedProofFailed</a></code> on failure.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_into_well_formed">into_well_formed</a>(<a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, dst: vector&lt;u8&gt;, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>: <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_into_well_formed">into_well_formed</a>(
    <a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a>,
    dst: vector&lt;u8&gt;,
    <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>: Element&lt;G&gt;,
    proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">WellFormedProof</a>,
): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a> {
    <b>assert</b>!(proof.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify">verify</a>(dst, &vector[<a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>], &vector[<a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>]), <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EWellFormedProofFailed">EWellFormedProofFailed</a>);
    <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a> { <a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a> }
}
</code></pre>



</details>

<a name="contra_encrypted_amount_batch_into_well_formed"></a>

## Function `batch_into_well_formed`

Verify <code>proof</code> against <code>amounts</code> under <code>pks</code> and <code>dst</code> (one aggregate proof for the whole
batch), and wrap each <code>amounts[i]</code> into a <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a> { <a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>: pks[i] }</code>.
Aborts with <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_EWellFormedProofFailed">EWellFormedProofFailed</a></code> on failure.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_into_well_formed">batch_into_well_formed</a>(amounts: vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>&gt;, dst: vector&lt;u8&gt;, pks: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">contra::encrypted_amount::WellFormedProof</a>): vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_into_well_formed">batch_into_well_formed</a>(
    amounts: vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a>&gt;,
    dst: vector&lt;u8&gt;,
    pks: vector&lt;Element&lt;G&gt;&gt;,
    proof: <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedProof">WellFormedProof</a>,
): vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a>&gt; {
    <b>assert</b>!(proof.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify">verify</a>(dst, &amounts, &pks), <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EWellFormedProofFailed">EWellFormedProofFailed</a>);
    amounts.zip_map!(pks, |<a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>| <a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a> { <a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a> })
}
</code></pre>



</details>

<a name="contra_encrypted_amount_amount"></a>

## Function `amount`

The verified encrypted amount carried by <code>self</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>(self: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>): &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>(self: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a>): &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a> {
    &self.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>
}
</code></pre>



</details>

<a name="contra_encrypted_amount_pk"></a>

## Function `pk`

The public key <code>self.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>()</code> is encrypted under.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>(self: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>): &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>(self: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a>): &Element&lt;G&gt; {
    &self.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>
}
</code></pre>



</details>

<a name="contra_encrypted_amount_limb"></a>

## Function `limb`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_limb">limb</a>(ea: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, i: u64): &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_limb">limb</a>(ea: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a>, i: u64): &Encryption {
    <b>assert</b>!(i &lt; 4, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EIndexOutOfBounds">EIndexOutOfBounds</a>);
    <b>if</b> (i == 0) {
        &ea.l0
    } <b>else</b> <b>if</b> (i == 1) {
        &ea.l1
    } <b>else</b> <b>if</b> (i == 2) {
        &ea.l2
    } <b>else</b> {
        &ea.l3
    }
}
</code></pre>



</details>

<a name="contra_encrypted_amount_collapse"></a>

## Function `collapse`

Return a single encryption of the value this encrypted amount encrypts.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse">collapse</a>(eq: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse">collapse</a>(eq: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a>): Encryption {
    <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_new">twisted_elgamal::new</a>(
        <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse_limbs">collapse_limbs</a>(
            eq.l0.ciphertext(),
            eq.l1.ciphertext(),
            eq.l2.ciphertext(),
            eq.l3.ciphertext(),
        ),
        <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse_limbs">collapse_limbs</a>(
            eq.l0.decryption_handle(),
            eq.l1.decryption_handle(),
            eq.l2.decryption_handle(),
            eq.l3.decryption_handle(),
        ),
    )
}
</code></pre>



</details>

<a name="contra_encrypted_amount_collapse_sum"></a>

## Function `collapse_sum`

Collapse the limb-wise sum of <code>amounts</code> into one <code>Encryption</code>, running the per-limb scalar mults
once for the batch instead of per amount (<code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse">collapse</a></code> is linear).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse_sum">collapse_sum</a>(amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>&gt;): <a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse_sum">collapse_sum</a>(amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a>&gt;): Encryption {
    <b>let</b> <b>mut</b> acc = <a href="../contra/encrypted_amount.md#contra_encrypted_amount_zero">zero</a>();
    amounts.do_ref!(|a| acc.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_add_assign">add_assign</a>(a));
    acc.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse">collapse</a>()
}
</code></pre>



</details>

<a name="contra_encrypted_amount_verify_equal"></a>

## Function `verify_equal`

Verify that <code>ea1</code> and <code>ea2</code> encrypt the same plaintext under <code>ea1.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a></code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify_equal">verify_equal</a>(ea1: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>, ea2: &<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_Encryption">contra::twisted_elgamal::Encryption</a>, proof: &<a href="../contra/nizk.md#contra_nizk_DdhProof">contra::nizk::DdhProof</a>, dst: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify_equal">verify_equal</a>(
    ea1: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a>,
    ea2: &Encryption,
    proof: &DdhProof,
    dst: vector&lt;u8&gt;,
): bool {
    <b>let</b> encryption = ea1.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse">collapse</a>().sub(ea2);
    proof.verify_ddh(
        dst,
        &g(),
        encryption.ciphertext(),
        &ea1.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>,
        encryption.decryption_handle(),
    )
}
</code></pre>



</details>

<a name="contra_encrypted_amount_sum_commitments"></a>

## Function `sum_commitments`

Sum of the collapsed Pedersen commitments of <code>amounts</code> (ciphertext components only).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_sum_commitments">sum_commitments</a>(amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">contra::encrypted_amount::WellFormedEncryptedAmount</a>&gt;): <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_sum_commitments">sum_commitments</a>(amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_WellFormedEncryptedAmount">WellFormedEncryptedAmount</a>&gt;): Element&lt;G&gt; {
    // `<a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse_limbs">collapse_limbs</a>` is linear, so sum the four <a href="../contra/encrypted_amount.md#contra_encrypted_amount_limb">limb</a> positions across all amounts first (cheap
    // point adds) and <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse">collapse</a> once, rather than collapsing each <a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a> (three scalar mults each).
    <b>let</b> <b>mut</b> c0 = g_identity();
    <b>let</b> <b>mut</b> c1 = g_identity();
    <b>let</b> <b>mut</b> c2 = g_identity();
    <b>let</b> <b>mut</b> c3 = g_identity();
    amounts.do_ref!(|wfea| {
        <b>let</b> a = &wfea.<a href="../contra/encrypted_amount.md#contra_encrypted_amount_amount">amount</a>;
        c0 = g_add(&c0, a[0].ciphertext());
        c1 = g_add(&c1, a[1].ciphertext());
        c2 = g_add(&c2, a[2].ciphertext());
        c3 = g_add(&c3, a[3].ciphertext());
    });
    <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse_limbs">collapse_limbs</a>(&c0, &c1, &c2, &c3)
}
</code></pre>



</details>

<a name="contra_encrypted_amount_collapse_limbs"></a>

## Function `collapse_limbs`

Combine four limbs into <code>l0 + 2^16 l1 + 2^32 l2 + 2^48 l3</code>.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse_limbs">collapse_limbs</a>(l0: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, l1: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, l2: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, l3: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;): <a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_collapse_limbs">collapse_limbs</a>(
    l0: &Element&lt;G&gt;,
    l1: &Element&lt;G&gt;,
    l2: &Element&lt;G&gt;,
    l3: &Element&lt;G&gt;,
): Element&lt;G&gt; {
    g_add(
        l0,
        &g_add(
            &g_mul(&scalar_from_u64(1 &lt;&lt; 16), l1),
            &g_add(
                &g_mul(&scalar_from_u64(1 &lt;&lt; 32), l2),
                &g_mul(&scalar_from_u64(1 &lt;&lt; 48), l3),
            ),
        ),
    )
}
</code></pre>



</details>

<a name="contra_encrypted_amount_from_value"></a>

## Function `from_value`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_from_value">from_value</a>(value: u64): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_from_value">from_value</a>(value: u64): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a> {
    <b>let</b> l0 = encrypt_trivial(value & 0xFFFF);
    <b>let</b> l1 = encrypt_trivial((value &gt;&gt; 16) & 0xFFFF);
    <b>let</b> l2 = encrypt_trivial((value &gt;&gt; 32) & 0xFFFF);
    <b>let</b> l3 = encrypt_trivial((value &gt;&gt; 48) & 0xFFFF);
    <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a> { l0, l1, l2, l3 }
}
</code></pre>



</details>

<a name="contra_encrypted_amount_zero"></a>

## Function `zero`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_zero">zero</a>(): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_zero">zero</a>(): <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a> {
    <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a> {
        l0: encrypt_zero(),
        l1: encrypt_zero(),
        l2: encrypt_zero(),
        l3: encrypt_zero(),
    }
}
</code></pre>



</details>

<a name="contra_encrypted_amount_add_assign"></a>

## Function `add_assign`

Limb-wise add <code>b</code> into <code>a</code>. Limbs may exceed u16 after this.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_add_assign">add_assign</a>(a: &<b>mut</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>, b: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_add_assign">add_assign</a>(a: &<b>mut</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a>, b: &<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a>) {
    a.l0 = a[0].add(&b[0]);
    a.l1 = a[1].add(&b[1]);
    a.l2 = a[2].add(&b[2]);
    a.l3 = a[3].add(&b[3]);
}
</code></pre>



</details>

<a name="contra_encrypted_amount_verify_well_formed_range_proofs"></a>

## Function `verify_well_formed_range_proofs`

Verify every limb is in <code>[0, 2^16)</code> via one Bulletproof per chunk of <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_sizes">batch_sizes</a></code>. An empty
<code>range_proofs</code> vector skips the range check entirely (test sentinel).


<pre><code><b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify_well_formed_range_proofs">verify_well_formed_range_proofs</a>(amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>&gt;, range_proofs: &vector&lt;vector&lt;u8&gt;&gt;, dst: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify_well_formed_range_proofs">verify_well_formed_range_proofs</a>(
    amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a>&gt;,
    range_proofs: &vector&lt;vector&lt;u8&gt;&gt;,
    dst: vector&lt;u8&gt;,
): bool {
    // For testing only: no range proofs skips the range check.
    <b>if</b> (range_proofs.is_empty()) <b>return</b> <b>true</b>;
    <b>let</b> sizes = <a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_sizes">batch_sizes</a>(amounts.length());
    <b>let</b> <b>mut</b> offset = 0;
    sizes.zip_map_ref!(range_proofs, |chunk, range_proof| {
        <b>let</b> chunk = *chunk;
        <b>let</b> start = offset;
        offset = offset + chunk;
        rangeproofs::verify_bulletproofs_with_dst_ristretto255(
            range_proof,
            <a href="../contra/encrypted_amount.md#contra_encrypted_amount_LIMB_BITS">LIMB_BITS</a>,
            &vector::tabulate!(4 * chunk, |j| *amounts[start + j / 4][j % 4].ciphertext()),
            &dst,
            <a href="../contra/encrypted_amount.md#contra_encrypted_amount_BULLETPROOFS_VERSION">BULLETPROOFS_VERSION</a>,
        )
    }).all!(|ok| *ok)
}
</code></pre>



</details>

<a name="contra_encrypted_amount_batch_sizes"></a>

## Function `batch_sizes`

Canonical Bulletproof chunking for <code>n</code> amounts: greedily take as many <code><a href="../contra/encrypted_amount.md#contra_encrypted_amount_MAX_BATCH_SIZE">MAX_BATCH_SIZE</a></code> chunks
as fit, then halve the chunk size and repeat until <code>n</code> is exhausted. Examples: n=7 → [4, 2, 1];
n=8 → [8]; n=16 → [8, 8]; n=20 → [8, 8, 4]; n=0 → [].


<pre><code><b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_sizes">batch_sizes</a>(n: u64): vector&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_batch_sizes">batch_sizes</a>(n: u64): vector&lt;u64&gt; {
    <b>let</b> <b>mut</b> sizes = vector[];
    <b>let</b> <b>mut</b> remaining = n;
    <b>let</b> <b>mut</b> chunk = <a href="../contra/encrypted_amount.md#contra_encrypted_amount_MAX_BATCH_SIZE">MAX_BATCH_SIZE</a>;
    <b>while</b> (remaining &gt; 0) {
        <b>while</b> (remaining &gt;= chunk) {
            sizes.push_back(chunk);
            remaining = remaining - chunk;
        };
        chunk = chunk / 2;
    };
    sizes
}
</code></pre>



</details>

<a name="contra_encrypted_amount_verify_well_formed_knowledge"></a>

## Function `verify_well_formed_knowledge`

Verify each limb of each <code>amounts[i]</code> is a valid ElGamal encryption under <code>pks[i]</code>.


<pre><code><b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify_well_formed_knowledge">verify_well_formed_knowledge</a>(amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">contra::encrypted_amount::EncryptedAmount</a>&gt;, proofs: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">contra::encrypted_amount::ConsistencyProof</a>&gt;, pks: &vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, dst: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_verify_well_formed_knowledge">verify_well_formed_knowledge</a>(
    amounts: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_EncryptedAmount">EncryptedAmount</a>&gt;,
    proofs: &vector&lt;<a href="../contra/encrypted_amount.md#contra_encrypted_amount_ConsistencyProof">ConsistencyProof</a>&gt;,
    pks: &vector&lt;Element&lt;G&gt;&gt;,
    dst: vector&lt;u8&gt;,
): bool {
    <b>let</b> n = amounts.length();
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; n) {
        <b>let</b> ea = &amounts[i];
        <b>let</b> proof = &proofs[i];
        <b>let</b> <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a> = &pks[i];
        <b>if</b> (!proof.p0.verify_elgamal(dst, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>, &ea[0])) <b>return</b> <b>false</b>;
        <b>if</b> (!proof.p1.verify_elgamal(dst, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>, &ea[1])) <b>return</b> <b>false</b>;
        <b>if</b> (!proof.p2.verify_elgamal(dst, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>, &ea[2])) <b>return</b> <b>false</b>;
        <b>if</b> (!proof.p3.verify_elgamal(dst, <a href="../contra/encrypted_amount.md#contra_encrypted_amount_pk">pk</a>, &ea[3])) <b>return</b> <b>false</b>;
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>
