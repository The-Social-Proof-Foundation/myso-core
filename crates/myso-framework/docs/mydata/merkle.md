---
title: Module `mydata::merkle`
---

Reusable Merkle proof verification for claim settlement.
Tree construction happens offchain; onchain only verifies proofs.
Uses Blake2b-256 for hashing, aligned with accumulator_settlement pattern.


-  [Constants](#@Constants_0)
-  [Function `verify_proof`](#mydata_merkle_verify_proof)
-  [Function `hash_pair`](#mydata_merkle_hash_pair)
-  [Function `leaf_hash`](#mydata_merkle_leaf_hash)
-  [Function `leaf_hash_with_platform`](#mydata_merkle_leaf_hash_with_platform)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="@Constants_0"></a>

## Constants


<a name="mydata_merkle_HASH_LEN"></a>



<pre><code><b>const</b> <a href="../mydata/merkle.md#mydata_merkle_HASH_LEN">HASH_LEN</a>: u64 = 32;
</code></pre>



<a name="mydata_merkle_verify_proof"></a>

## Function `verify_proof`

Verify that a leaf is included in a Merkle tree with the given root.
@param leaf - Hash of the leaf (e.g. from leaf_hash)
@param proof - Sibling hashes from leaf level up to root (excluding root)
@param leaf_index - Index of the leaf in the tree (0 = leftmost)
@param root - Expected Merkle root


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/merkle.md#mydata_merkle_verify_proof">verify_proof</a>(leaf: vector&lt;u8&gt;, proof: &vector&lt;vector&lt;u8&gt;&gt;, leaf_index: u64, root: vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/merkle.md#mydata_merkle_verify_proof">verify_proof</a>(
    leaf: vector&lt;u8&gt;,
    proof: &vector&lt;vector&lt;u8&gt;&gt;,
    leaf_index: u64,
    root: vector&lt;u8&gt;,
): bool {
    <b>assert</b>!(vector::length(&leaf) == <a href="../mydata/merkle.md#mydata_merkle_HASH_LEN">HASH_LEN</a>, 0);
    <b>assert</b>!(vector::length(&root) == <a href="../mydata/merkle.md#mydata_merkle_HASH_LEN">HASH_LEN</a>, 1);
    <b>let</b> <b>mut</b> current = leaf;
    <b>let</b> <b>mut</b> idx = leaf_index;
    <b>let</b> len = vector::length(proof);
    <b>let</b> <b>mut</b> i = 0u64;
    <b>while</b> (i &lt; len) {
        <b>let</b> sibling = vector::borrow(proof, i);
        <b>assert</b>!(vector::length(sibling) == <a href="../mydata/merkle.md#mydata_merkle_HASH_LEN">HASH_LEN</a>, 2);
        current = <b>if</b> (idx % 2 == 0) {
            <a href="../mydata/merkle.md#mydata_merkle_hash_pair">hash_pair</a>(&current, sibling)
        } <b>else</b> {
            <a href="../mydata/merkle.md#mydata_merkle_hash_pair">hash_pair</a>(sibling, &current)
        };
        idx = idx / 2;
        i = i + 1;
    };
    current == root
}
</code></pre>



</details>

<a name="mydata_merkle_hash_pair"></a>

## Function `hash_pair`

Hash two 32-byte values for Merkle tree internal node.
Order matters: hash(left || right).


<pre><code><b>fun</b> <a href="../mydata/merkle.md#mydata_merkle_hash_pair">hash_pair</a>(left: &vector&lt;u8&gt;, right: &vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/merkle.md#mydata_merkle_hash_pair">hash_pair</a>(left: &vector&lt;u8&gt;, right: &vector&lt;u8&gt;): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> concat = *left;
    vector::append(&<b>mut</b> concat, *right);
    hash::blake2b256(&concat)
}
</code></pre>



</details>

<a name="mydata_merkle_leaf_hash"></a>

## Function `leaf_hash`

Construct leaf hash for claim verification.
Leaf = blake2b256(address || amount || snapshot_id).


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/merkle.md#mydata_merkle_leaf_hash">leaf_hash</a>(addr: <b>address</b>, amount: u64, snapshot_id: vector&lt;u8&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/merkle.md#mydata_merkle_leaf_hash">leaf_hash</a>(
    addr: <b>address</b>,
    amount: u64,
    snapshot_id: vector&lt;u8&gt;,
): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> data = bcs::to_bytes(&addr);
    vector::append(&<b>mut</b> data, bcs::to_bytes(&amount));
    vector::append(&<b>mut</b> data, snapshot_id);
    hash::blake2b256(&data)
}
</code></pre>



</details>

<a name="mydata_merkle_leaf_hash_with_platform"></a>

## Function `leaf_hash_with_platform`

Construct a MyData marketplace leaf that also commits to the fee-routing platform.
<code>platform_id = none</code> identifies the non-platform settlement path.


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/merkle.md#mydata_merkle_leaf_hash_with_platform">leaf_hash_with_platform</a>(addr: <b>address</b>, amount: u64, snapshot_id: vector&lt;u8&gt;, platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/merkle.md#mydata_merkle_leaf_hash_with_platform">leaf_hash_with_platform</a>(
    addr: <b>address</b>,
    amount: u64,
    snapshot_id: vector&lt;u8&gt;,
    platform_id: Option&lt;<b>address</b>&gt;,
): vector&lt;u8&gt; {
    <b>let</b> <b>mut</b> data = bcs::to_bytes(&addr);
    vector::append(&<b>mut</b> data, bcs::to_bytes(&amount));
    vector::append(&<b>mut</b> data, snapshot_id);
    vector::append(&<b>mut</b> data, bcs::to_bytes(&platform_id));
    hash::blake2b256(&data)
}
</code></pre>



</details>
