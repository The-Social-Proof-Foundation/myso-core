---
title: Module `contra::auditors`
---



-  [Struct `Auditors`](#contra_auditors_Auditors)
-  [Struct `VerifiedKeyEncryption`](#contra_auditors_VerifiedKeyEncryption)
-  [Struct `KeyEncryption`](#contra_auditors_KeyEncryption)
-  [Constants](#@Constants_0)
-  [Function `new`](#contra_auditors_new)
-  [Function `update`](#contra_auditors_update)
-  [Function `assert_no_identity_pk`](#contra_auditors_assert_no_identity_pk)
-  [Function `pks`](#contra_auditors_pks)
-  [Function `is_empty`](#contra_auditors_is_empty)
-  [Function `version`](#contra_auditors_version)
-  [Function `recommended_min_version`](#contra_auditors_recommended_min_version)
-  [Function `ciphertext`](#contra_auditors_ciphertext)
-  [Function `key_version`](#contra_auditors_key_version)
-  [Function `is_set`](#contra_auditors_is_set)
-  [Function `new_key_encryption`](#contra_auditors_new_key_encryption)
-  [Function `new_empty_verified_key_encryption`](#contra_auditors_new_empty_verified_key_encryption)
-  [Function `verify_key_encryption`](#contra_auditors_verify_key_encryption)


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



<a name="contra_auditors_Auditors"></a>

## Struct `Auditors`

Holds the set of auditor <code>public_keys</code> registered for a token. Auditors can decrypt the
viewing-key ciphertexts attached to each transfer, giving them read access to transaction
amounts without being able to move funds.

The <code><a href="../contra/auditors.md#contra_auditors_version">version</a></code> number is incremented on every <code><a href="../contra/auditors.md#contra_auditors_update">update</a></code> so that <code><a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a></code> values
stored on user accounts can be checked for staleness. <code><a href="../contra/auditors.md#contra_auditors_recommended_min_version">recommended_min_version</a></code> is the issuer's
advertised minimum <code><a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a>.<a href="../contra/auditors.md#contra_auditors_version">version</a></code>; it is not enforced on chain. Wallets and
other clients should treat any account whose <code><a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a>.<a href="../contra/auditors.md#contra_auditors_version">version</a></code> is below it as
stale and prompt the user to rotate before transferring.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a> <b>has</b> store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../contra/auditors.md#contra_auditors_pks">pks</a>: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../contra/auditors.md#contra_auditors_version">version</a>: u32</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../contra/auditors.md#contra_auditors_recommended_min_version">recommended_min_version</a>: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_auditors_VerifiedKeyEncryption"></a>

## Struct `VerifiedKeyEncryption`

A user's viewing key encrypted to each auditor's public key, stored on their account after
passing a <code>KeyConsistencyProof</code> check. The <code><a href="../contra/auditors.md#contra_auditors_version">version</a></code> records which auditor key set it was
produced against, so callers can compare it against <code><a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a>.<a href="../contra/auditors.md#contra_auditors_recommended_min_version">recommended_min_version</a></code> to
detect encryptions that the issuer considers stale.

An empty <code><a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a></code> means the user's account was registered while the token had no
auditors set.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>: vector&lt;<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">contra::twisted_elgamal::MultiRecipientEncryption</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../contra/auditors.md#contra_auditors_version">version</a>: u32</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="contra_auditors_KeyEncryption"></a>

## Struct `KeyEncryption`

A user's viewing key encrypted to each auditor's public key, bundled with the
proofs needed to register it on-chain:
- <code>proof</code> is the <code>KeyConsistencyProof</code> showing each limb of <code><a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a></code> correctly
encrypts the matching 32-bit limb of the user's private key under every auditor's
public key, and that the limbs sum to <code>sender_public_key</code>'s discrete log.
- <code>range_proof</code> is an aggregate Bulletproof showing every limb's plaintext lies
in <code>[0, 2^32)</code> so that auditors can recover each limb via baby-step giant-step.


<pre><code><b>public</b> <b>struct</b> <a href="../contra/auditors.md#contra_auditors_KeyEncryption">KeyEncryption</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>: vector&lt;<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">contra::twisted_elgamal::MultiRecipientEncryption</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>proof: <a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">contra::nizk::KeyConsistencyProof</a></code>
</dt>
<dd>
</dd>
<dt>
<code>range_proof: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="contra_auditors_BULLETPROOFS_VERSION"></a>

Bulletproof construction version (Bünz et al., 2018).


<pre><code><b>const</b> <a href="../contra/auditors.md#contra_auditors_BULLETPROOFS_VERSION">BULLETPROOFS_VERSION</a>: u8 = 0;
</code></pre>



<a name="contra_auditors_LIMB_BITS"></a>

Bit-length of each private-key limb committed in the viewing-key encryption.


<pre><code><b>const</b> <a href="../contra/auditors.md#contra_auditors_LIMB_BITS">LIMB_BITS</a>: u8 = 32;
</code></pre>



<a name="contra_auditors_EInvalidEncryptedViewingKey"></a>



<pre><code><b>const</b> <a href="../contra/auditors.md#contra_auditors_EInvalidEncryptedViewingKey">EInvalidEncryptedViewingKey</a>: u64 = 0;
</code></pre>



<a name="contra_auditors_EMissingEncryptedViewingKeyArguments"></a>



<pre><code><b>const</b> <a href="../contra/auditors.md#contra_auditors_EMissingEncryptedViewingKeyArguments">EMissingEncryptedViewingKeyArguments</a>: u64 = 1;
</code></pre>



<a name="contra_auditors_ETooManyEncryptedViewingKeyArguments"></a>



<pre><code><b>const</b> <a href="../contra/auditors.md#contra_auditors_ETooManyEncryptedViewingKeyArguments">ETooManyEncryptedViewingKeyArguments</a>: u64 = 2;
</code></pre>



<a name="contra_auditors_EIdentityAuditorPublicKey"></a>



<pre><code><b>const</b> <a href="../contra/auditors.md#contra_auditors_EIdentityAuditorPublicKey">EIdentityAuditorPublicKey</a>: u64 = 3;
</code></pre>



<a name="contra_auditors_new"></a>

## Function `new`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_new">new</a>(<a href="../contra/auditors.md#contra_auditors_pks">pks</a>: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;): <a href="../contra/auditors.md#contra_auditors_Auditors">contra::auditors::Auditors</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_new">new</a>(<a href="../contra/auditors.md#contra_auditors_pks">pks</a>: vector&lt;Element&lt;G&gt;&gt;): <a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a> {
    <a href="../contra/auditors.md#contra_auditors_assert_no_identity_pk">assert_no_identity_pk</a>(&<a href="../contra/auditors.md#contra_auditors_pks">pks</a>);
    <b>let</b> <a href="../contra/auditors.md#contra_auditors">auditors</a> = <a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a> {
        <a href="../contra/auditors.md#contra_auditors_pks">pks</a>,
        <a href="../contra/auditors.md#contra_auditors_version">version</a>: 0,
        <a href="../contra/auditors.md#contra_auditors_recommended_min_version">recommended_min_version</a>: 0,
    };
    <a href="../contra/auditors.md#contra_auditors">auditors</a>
}
</code></pre>



</details>

<a name="contra_auditors_update"></a>

## Function `update`

Rotate the auditor key set. The <code><a href="../contra/auditors.md#contra_auditors_version">version</a></code> is bumped on every call. When
<code>bump_recommended_min</code> is true, <code><a href="../contra/auditors.md#contra_auditors_recommended_min_version">recommended_min_version</a></code> is raised to the new <code><a href="../contra/auditors.md#contra_auditors_version">version</a></code>,
signalling that the issuer would like every user to refresh keys.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_update">update</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<b>mut</b> <a href="../contra/auditors.md#contra_auditors_Auditors">contra::auditors::Auditors</a>, new_pks: vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;, bump_recommended_min: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_update">update</a>(
    <a href="../contra/auditors.md#contra_auditors">auditors</a>: &<b>mut</b> <a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a>,
    new_pks: vector&lt;Element&lt;G&gt;&gt;,
    bump_recommended_min: bool,
) {
    <a href="../contra/auditors.md#contra_auditors_assert_no_identity_pk">assert_no_identity_pk</a>(&new_pks);
    <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_pks">pks</a> = new_pks;
    <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_version">version</a> = <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_version">version</a> + 1;
    <b>if</b> (bump_recommended_min) {
        <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_recommended_min_version">recommended_min_version</a> = <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_version">version</a>;
    };
}
</code></pre>



</details>

<a name="contra_auditors_assert_no_identity_pk"></a>

## Function `assert_no_identity_pk`

Abort with <code><a href="../contra/auditors.md#contra_auditors_EIdentityAuditorPublicKey">EIdentityAuditorPublicKey</a></code> if any entry of <code><a href="../contra/auditors.md#contra_auditors_pks">pks</a></code> is the group identity.


<pre><code><b>fun</b> <a href="../contra/auditors.md#contra_auditors_assert_no_identity_pk">assert_no_identity_pk</a>(<a href="../contra/auditors.md#contra_auditors_pks">pks</a>: &vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/auditors.md#contra_auditors_assert_no_identity_pk">assert_no_identity_pk</a>(<a href="../contra/auditors.md#contra_auditors_pks">pks</a>: &vector&lt;Element&lt;G&gt;&gt;) {
    <b>let</b> identity = g_identity();
    <a href="../contra/auditors.md#contra_auditors_pks">pks</a>.do_ref!(|pk| <b>assert</b>!(*pk != identity, <a href="../contra/auditors.md#contra_auditors_EIdentityAuditorPublicKey">EIdentityAuditorPublicKey</a>));
}
</code></pre>



</details>

<a name="contra_auditors_pks"></a>

## Function `pks`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_pks">pks</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">contra::auditors::Auditors</a>): &vector&lt;<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_pks">pks</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a>): &vector&lt;Element&lt;G&gt;&gt; {
    &<a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_pks">pks</a>
}
</code></pre>



</details>

<a name="contra_auditors_is_empty"></a>

## Function `is_empty`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_is_empty">is_empty</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">contra::auditors::Auditors</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_is_empty">is_empty</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a>): bool {
    <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_pks">pks</a>.<a href="../contra/auditors.md#contra_auditors_is_empty">is_empty</a>()
}
</code></pre>



</details>

<a name="contra_auditors_version"></a>

## Function `version`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_version">version</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">contra::auditors::Auditors</a>): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_version">version</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a>): u32 {
    <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_version">version</a>
}
</code></pre>



</details>

<a name="contra_auditors_recommended_min_version"></a>

## Function `recommended_min_version`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_recommended_min_version">recommended_min_version</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">contra::auditors::Auditors</a>): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_recommended_min_version">recommended_min_version</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a>): u32 {
    <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_recommended_min_version">recommended_min_version</a>
}
</code></pre>



</details>

<a name="contra_auditors_ciphertext"></a>

## Function `ciphertext`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>(verified_key_encryption: &<a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a>): &vector&lt;<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">contra::twisted_elgamal::MultiRecipientEncryption</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>(
    verified_key_encryption: &<a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a>,
): &vector&lt;MultiRecipientEncryption&gt; {
    &verified_key_encryption.<a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>
}
</code></pre>



</details>

<a name="contra_auditors_key_version"></a>

## Function `key_version`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_key_version">key_version</a>(verified_key_encryption: &<a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a>): u32
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_key_version">key_version</a>(verified_key_encryption: &<a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a>): u32 {
    verified_key_encryption.<a href="../contra/auditors.md#contra_auditors_version">version</a>
}
</code></pre>



</details>

<a name="contra_auditors_is_set"></a>

## Function `is_set`

True iff this <code><a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a></code> was produced from a non-empty <code><a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a></code> set.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_is_set">is_set</a>(verified_key_encryption: &<a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_is_set">is_set</a>(verified_key_encryption: &<a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a>): bool {
    !verified_key_encryption.<a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>.<a href="../contra/auditors.md#contra_auditors_is_empty">is_empty</a>()
}
</code></pre>



</details>

<a name="contra_auditors_new_key_encryption"></a>

## Function `new_key_encryption`



<pre><code><b>public</b> <b>fun</b> <a href="../contra/auditors.md#contra_auditors_new_key_encryption">new_key_encryption</a>(<a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>: vector&lt;<a href="../contra/twisted_elgamal.md#contra_twisted_elgamal_MultiRecipientEncryption">contra::twisted_elgamal::MultiRecipientEncryption</a>&gt;, proof: <a href="../contra/nizk.md#contra_nizk_KeyConsistencyProof">contra::nizk::KeyConsistencyProof</a>, range_proof: vector&lt;u8&gt;): <a href="../contra/auditors.md#contra_auditors_KeyEncryption">contra::auditors::KeyEncryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../contra/auditors.md#contra_auditors_new_key_encryption">new_key_encryption</a>(
    <a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>: vector&lt;MultiRecipientEncryption&gt;,
    proof: KeyConsistencyProof,
    range_proof: vector&lt;u8&gt;,
): <a href="../contra/auditors.md#contra_auditors_KeyEncryption">KeyEncryption</a> {
    <a href="../contra/auditors.md#contra_auditors_KeyEncryption">KeyEncryption</a> { <a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>, proof, range_proof }
}
</code></pre>



</details>

<a name="contra_auditors_new_empty_verified_key_encryption"></a>

## Function `new_empty_verified_key_encryption`

Placeholder <code><a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a></code> for accounts registered while the token has no
auditors configured. The <code><a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a></code> is empty.


<pre><code><b>fun</b> <a href="../contra/auditors.md#contra_auditors_new_empty_verified_key_encryption">new_empty_verified_key_encryption</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">contra::auditors::Auditors</a>): <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../contra/auditors.md#contra_auditors_new_empty_verified_key_encryption">new_empty_verified_key_encryption</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a>): <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a> {
    <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a> { <a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>: vector[], <a href="../contra/auditors.md#contra_auditors_version">version</a>: <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_version">version</a> }
}
</code></pre>



</details>

<a name="contra_auditors_verify_key_encryption"></a>

## Function `verify_key_encryption`

Resolve an <code>Option&lt;<a href="../contra/auditors.md#contra_auditors_KeyEncryption">KeyEncryption</a>&gt;</code> against the configured <code><a href="../contra/auditors.md#contra_auditors">auditors</a></code> and produce a
<code><a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a></code>. When auditors are set, a <code><a href="../contra/auditors.md#contra_auditors_KeyEncryption">KeyEncryption</a></code> must be provided; the
sigma proof and the aggregate Bulletproof over the limb commitments are both checked
before returning. When auditors are not set, no <code><a href="../contra/auditors.md#contra_auditors_KeyEncryption">KeyEncryption</a></code> may be provided and an
empty placeholder is returned. Aborts with <code><a href="../contra/auditors.md#contra_auditors_EMissingEncryptedViewingKeyArguments">EMissingEncryptedViewingKeyArguments</a></code> /
<code><a href="../contra/auditors.md#contra_auditors_ETooManyEncryptedViewingKeyArguments">ETooManyEncryptedViewingKeyArguments</a></code> on mismatch.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_verify_key_encryption">verify_key_encryption</a>(<a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">contra::auditors::Auditors</a>, sender_public_key: &<a href="../myso/group_ops.md#myso_group_ops_Element">myso::group_ops::Element</a>&lt;<a href="../myso/ristretto255.md#myso_ristretto255_G">myso::ristretto255::G</a>&gt;, key_encryption: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../contra/auditors.md#contra_auditors_KeyEncryption">contra::auditors::KeyEncryption</a>&gt;, dst: vector&lt;u8&gt;): <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">contra::auditors::VerifiedKeyEncryption</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../contra/auditors.md#contra_auditors_verify_key_encryption">verify_key_encryption</a>(
    <a href="../contra/auditors.md#contra_auditors">auditors</a>: &<a href="../contra/auditors.md#contra_auditors_Auditors">Auditors</a>,
    sender_public_key: &Element&lt;G&gt;,
    key_encryption: Option&lt;<a href="../contra/auditors.md#contra_auditors_KeyEncryption">KeyEncryption</a>&gt;,
    dst: vector&lt;u8&gt;,
): <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a> {
    <b>if</b> (<a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_is_empty">is_empty</a>()) {
        <b>assert</b>!(key_encryption.is_none(), <a href="../contra/auditors.md#contra_auditors_ETooManyEncryptedViewingKeyArguments">ETooManyEncryptedViewingKeyArguments</a>);
        <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_new_empty_verified_key_encryption">new_empty_verified_key_encryption</a>()
    } <b>else</b> {
        <b>assert</b>!(key_encryption.is_some(), <a href="../contra/auditors.md#contra_auditors_EMissingEncryptedViewingKeyArguments">EMissingEncryptedViewingKeyArguments</a>);
        <b>let</b> <a href="../contra/auditors.md#contra_auditors_KeyEncryption">KeyEncryption</a> { <a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>, proof, range_proof } = key_encryption.destroy_some();
        // TODO: <b>use</b> different DSTs <b>for</b> the key consistency and range proofs below.
        <b>assert</b>!(
            proof.verify_key_consistency(
                dst,
                sender_public_key,
                <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_pks">pks</a>(),
                &<a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>,
            ) &&
                rangeproofs::verify_bulletproofs_with_dst_ristretto255(
                    &range_proof,
                    <a href="../contra/auditors.md#contra_auditors_LIMB_BITS">LIMB_BITS</a>,
                    &vector::tabulate!(
                        <a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>.length(),
                        |i| *<a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>[i].multi_recipient_ciphertext(),
                    ),
                    &dst,
                    <a href="../contra/auditors.md#contra_auditors_BULLETPROOFS_VERSION">BULLETPROOFS_VERSION</a>,
                ),
            <a href="../contra/auditors.md#contra_auditors_EInvalidEncryptedViewingKey">EInvalidEncryptedViewingKey</a>,
        );
        <a href="../contra/auditors.md#contra_auditors_VerifiedKeyEncryption">VerifiedKeyEncryption</a> { <a href="../contra/auditors.md#contra_auditors_ciphertext">ciphertext</a>, <a href="../contra/auditors.md#contra_auditors_version">version</a>: <a href="../contra/auditors.md#contra_auditors">auditors</a>.<a href="../contra/auditors.md#contra_auditors_version">version</a> }
    }
}
</code></pre>



</details>
