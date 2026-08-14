---
title: Module `social_contracts::derivative_graph`
---

**Bounded DAG:** Version 1 enforces <code>MAX_ANCESTORS</code> as an intentional protocol constraint.
**Cycle prevention:** Each <code>MediaAsset</code> commits transitive <code><a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">AncestryMetadata</a></code>.


-  [Struct `AncestryMetadata`](#social_contracts_derivative_graph_AncestryMetadata)
-  [Struct `DerivativeRelationship`](#social_contracts_derivative_graph_DerivativeRelationship)
-  [Struct `ResolvedRoyaltyObligation`](#social_contracts_derivative_graph_ResolvedRoyaltyObligation)
-  [Struct `PendingComposeAccumulator`](#social_contracts_derivative_graph_PendingComposeAccumulator)
-  [Constants](#@Constants_0)
-  [Function `max_ancestors`](#social_contracts_derivative_graph_max_ancestors)
-  [Function `relationship_remix`](#social_contracts_derivative_graph_relationship_remix)
-  [Function `relationship_sample`](#social_contracts_derivative_graph_relationship_sample)
-  [Function `relationship_cover`](#social_contracts_derivative_graph_relationship_cover)
-  [Function `relationship_mashup`](#social_contracts_derivative_graph_relationship_mashup)
-  [Function `obligation_kind_edge`](#social_contracts_derivative_graph_obligation_kind_edge)
-  [Function `obligation_kind_inherited`](#social_contracts_derivative_graph_obligation_kind_inherited)
-  [Function `obligation_kind_template`](#social_contracts_derivative_graph_obligation_kind_template)
-  [Function `empty_compose_accumulator`](#social_contracts_derivative_graph_empty_compose_accumulator)
-  [Function `compose_total_edge_share_bps`](#social_contracts_derivative_graph_compose_total_edge_share_bps)
-  [Function `compose_accumulated_obligations`](#social_contracts_derivative_graph_compose_accumulated_obligations)
-  [Function `compose_parent_asset_ids`](#social_contracts_derivative_graph_compose_parent_asset_ids)
-  [Function `compose_license_instance_ids`](#social_contracts_derivative_graph_compose_license_instance_ids)
-  [Function `compose_template_version_ids`](#social_contracts_derivative_graph_compose_template_version_ids)
-  [Function `compose_parent_policy_versions`](#social_contracts_derivative_graph_compose_parent_policy_versions)
-  [Function `compose_effective_rights`](#social_contracts_derivative_graph_compose_effective_rights)
-  [Function `compose_derivatives_allowed`](#social_contracts_derivative_graph_compose_derivatives_allowed)
-  [Function `compose_commercial_allowed`](#social_contracts_derivative_graph_compose_commercial_allowed)
-  [Function `compose_attribution_required`](#social_contracts_derivative_graph_compose_attribution_required)
-  [Function `find_parent_index`](#social_contracts_derivative_graph_find_parent_index)
-  [Function `insert_parent_into_lineage_vectors`](#social_contracts_derivative_graph_insert_parent_into_lineage_vectors)
-  [Function `merge_compose_accumulator`](#social_contracts_derivative_graph_merge_compose_accumulator)
-  [Function `find_edge_by_relationship_id`](#social_contracts_derivative_graph_find_edge_by_relationship_id)
-  [Function `set_child_asset_id_on_edges`](#social_contracts_derivative_graph_set_child_asset_id_on_edges)
-  [Function `empty_ancestry`](#social_contracts_derivative_graph_empty_ancestry)
-  [Function `ancestry_version`](#social_contracts_derivative_graph_ancestry_version)
-  [Function `ancestor_ids`](#social_contracts_derivative_graph_ancestor_ids)
-  [Function `new_derivative_relationship`](#social_contracts_derivative_graph_new_derivative_relationship)
-  [Function `edge_parent_asset_id`](#social_contracts_derivative_graph_edge_parent_asset_id)
-  [Function `edge_relationship_id`](#social_contracts_derivative_graph_edge_relationship_id)
-  [Function `edge_license_instance_id`](#social_contracts_derivative_graph_edge_license_instance_id)
-  [Function `edge_template_version_id`](#social_contracts_derivative_graph_edge_template_version_id)
-  [Function `edge_parent_share_bps`](#social_contracts_derivative_graph_edge_parent_share_bps)
-  [Function `new_resolved_royalty_obligation`](#social_contracts_derivative_graph_new_resolved_royalty_obligation)
-  [Function `obligation_beneficiary_asset_id`](#social_contracts_derivative_graph_obligation_beneficiary_asset_id)
-  [Function `obligation_beneficiary_address`](#social_contracts_derivative_graph_obligation_beneficiary_address)
-  [Function `obligation_share_bps`](#social_contracts_derivative_graph_obligation_share_bps)
-  [Function `obligation_source_relationship_id`](#social_contracts_derivative_graph_obligation_source_relationship_id)
-  [Function `obligation_source_license_instance_id`](#social_contracts_derivative_graph_obligation_source_license_instance_id)
-  [Function `obligation_obligation_kind`](#social_contracts_derivative_graph_obligation_obligation_kind)
-  [Function `id_vector_contains`](#social_contracts_derivative_graph_id_vector_contains)
-  [Function `id_less_than`](#social_contracts_derivative_graph_id_less_than)
-  [Function `sort_ids_ascending`](#social_contracts_derivative_graph_sort_ids_ascending)
-  [Function `push_id_if_absent`](#social_contracts_derivative_graph_push_id_if_absent)
-  [Function `canonical_union`](#social_contracts_derivative_graph_canonical_union)
-  [Function `assert_valid_parent_child_edge`](#social_contracts_derivative_graph_assert_valid_parent_child_edge)
-  [Function `merge_ancestry_for_edge`](#social_contracts_derivative_graph_merge_ancestry_for_edge)
-  [Function `assert_relationship_unique`](#social_contracts_derivative_graph_assert_relationship_unique)


<pre><code><b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_derivative_graph_AncestryMetadata"></a>

## Struct `AncestryMetadata`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">AncestryMetadata</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestor_ids">ancestor_ids</a>: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestry_version">ancestry_version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_derivative_graph_DerivativeRelationship"></a>

## Struct `DerivativeRelationship`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>relationship_id: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>parent_asset_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>child_asset_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>relationship_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>license_instance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>template_version_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>parent_share_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>evidence_commitment: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_derivative_graph_ResolvedRoyaltyObligation"></a>

## Struct `ResolvedRoyaltyObligation`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>beneficiary_asset_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>beneficiary_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>share_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>source_relationship_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>source_license_instance_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>obligation_kind: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_derivative_graph_PendingComposeAccumulator"></a>

## Struct `PendingComposeAccumulator`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>effective_rights: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>derivatives_allowed: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>commercial_allowed: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>attribution_required: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>total_edge_share_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accumulated_obligations: vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>parent_asset_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>license_instance_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>template_version_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>parent_policy_versions: vector&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_derivative_graph_E_SELF_EDGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_SELF_EDGE">E_SELF_EDGE</a>: u64 = 1;
</code></pre>



<a name="social_contracts_derivative_graph_E_CYCLE_DETECTED"></a>



<pre><code><b>const</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_CYCLE_DETECTED">E_CYCLE_DETECTED</a>: u64 = 2;
</code></pre>



<a name="social_contracts_derivative_graph_E_ANCESTOR_LIMIT"></a>



<pre><code><b>const</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_ANCESTOR_LIMIT">E_ANCESTOR_LIMIT</a>: u64 = 3;
</code></pre>



<a name="social_contracts_derivative_graph_E_DUPLICATE_RELATIONSHIP"></a>



<pre><code><b>const</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_DUPLICATE_RELATIONSHIP">E_DUPLICATE_RELATIONSHIP</a>: u64 = 4;
</code></pre>



<a name="social_contracts_derivative_graph_E_ROYALTY_OVERFLOW"></a>



<pre><code><b>const</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_ROYALTY_OVERFLOW">E_ROYALTY_OVERFLOW</a>: u64 = 5;
</code></pre>



<a name="social_contracts_derivative_graph_BPS_TOTAL"></a>



<pre><code><b>const</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_BPS_TOTAL">BPS_TOTAL</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_derivative_graph_max_ancestors"></a>

## Function `max_ancestors`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_max_ancestors">max_ancestors</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_max_ancestors">max_ancestors</a>(): u64 { 64 }
</code></pre>



</details>

<a name="social_contracts_derivative_graph_relationship_remix"></a>

## Function `relationship_remix`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_relationship_remix">relationship_remix</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_relationship_remix">relationship_remix</a>(): u8 { 1 }
</code></pre>



</details>

<a name="social_contracts_derivative_graph_relationship_sample"></a>

## Function `relationship_sample`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_relationship_sample">relationship_sample</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_relationship_sample">relationship_sample</a>(): u8 { 2 }
</code></pre>



</details>

<a name="social_contracts_derivative_graph_relationship_cover"></a>

## Function `relationship_cover`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_relationship_cover">relationship_cover</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_relationship_cover">relationship_cover</a>(): u8 { 3 }
</code></pre>



</details>

<a name="social_contracts_derivative_graph_relationship_mashup"></a>

## Function `relationship_mashup`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_relationship_mashup">relationship_mashup</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_relationship_mashup">relationship_mashup</a>(): u8 { 4 }
</code></pre>



</details>

<a name="social_contracts_derivative_graph_obligation_kind_edge"></a>

## Function `obligation_kind_edge`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_kind_edge">obligation_kind_edge</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_kind_edge">obligation_kind_edge</a>(): u8 { 1 }
</code></pre>



</details>

<a name="social_contracts_derivative_graph_obligation_kind_inherited"></a>

## Function `obligation_kind_inherited`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_kind_inherited">obligation_kind_inherited</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_kind_inherited">obligation_kind_inherited</a>(): u8 { 2 }
</code></pre>



</details>

<a name="social_contracts_derivative_graph_obligation_kind_template"></a>

## Function `obligation_kind_template`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_kind_template">obligation_kind_template</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_kind_template">obligation_kind_template</a>(): u8 { 3 }
</code></pre>



</details>

<a name="social_contracts_derivative_graph_empty_compose_accumulator"></a>

## Function `empty_compose_accumulator`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_empty_compose_accumulator">empty_compose_accumulator</a>(): <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_empty_compose_accumulator">empty_compose_accumulator</a>(): <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a> {
    <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a> {
        effective_rights: <a href="../std/u64.md#std_u64_max_value">std::u64::max_value</a>!(),
        derivatives_allowed: <b>true</b>,
        commercial_allowed: <b>true</b>,
        attribution_required: <b>false</b>,
        total_edge_share_bps: 0,
        accumulated_obligations: vector[],
        parent_asset_ids: vector[],
        license_instance_ids: vector[],
        template_version_ids: vector[],
        parent_policy_versions: vector[],
    }
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_compose_total_edge_share_bps"></a>

## Function `compose_total_edge_share_bps`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_total_edge_share_bps">compose_total_edge_share_bps</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_total_edge_share_bps">compose_total_edge_share_bps</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>): u64 {
    acc.total_edge_share_bps
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_compose_accumulated_obligations"></a>

## Function `compose_accumulated_obligations`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_accumulated_obligations">compose_accumulated_obligations</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>): &vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_accumulated_obligations">compose_accumulated_obligations</a>(
    acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>,
): &vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a>&gt; {
    &acc.accumulated_obligations
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_compose_parent_asset_ids"></a>

## Function `compose_parent_asset_ids`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_parent_asset_ids">compose_parent_asset_ids</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>): &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_parent_asset_ids">compose_parent_asset_ids</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>): &vector&lt;ID&gt; {
    &acc.parent_asset_ids
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_compose_license_instance_ids"></a>

## Function `compose_license_instance_ids`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_license_instance_ids">compose_license_instance_ids</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>): &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_license_instance_ids">compose_license_instance_ids</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>): &vector&lt;ID&gt; {
    &acc.license_instance_ids
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_compose_template_version_ids"></a>

## Function `compose_template_version_ids`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_template_version_ids">compose_template_version_ids</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>): &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_template_version_ids">compose_template_version_ids</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>): &vector&lt;ID&gt; {
    &acc.template_version_ids
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_compose_parent_policy_versions"></a>

## Function `compose_parent_policy_versions`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_parent_policy_versions">compose_parent_policy_versions</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>): &vector&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_parent_policy_versions">compose_parent_policy_versions</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>): &vector&lt;u64&gt; {
    &acc.parent_policy_versions
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_compose_effective_rights"></a>

## Function `compose_effective_rights`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_effective_rights">compose_effective_rights</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_effective_rights">compose_effective_rights</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>): u64 {
    acc.effective_rights
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_compose_derivatives_allowed"></a>

## Function `compose_derivatives_allowed`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_derivatives_allowed">compose_derivatives_allowed</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_derivatives_allowed">compose_derivatives_allowed</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>): bool {
    acc.derivatives_allowed
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_compose_commercial_allowed"></a>

## Function `compose_commercial_allowed`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_commercial_allowed">compose_commercial_allowed</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_commercial_allowed">compose_commercial_allowed</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>): bool {
    acc.commercial_allowed
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_compose_attribution_required"></a>

## Function `compose_attribution_required`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_attribution_required">compose_attribution_required</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_compose_attribution_required">compose_attribution_required</a>(acc: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>): bool {
    acc.attribution_required
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_find_parent_index"></a>

## Function `find_parent_index`



<pre><code><b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_find_parent_index">find_parent_index</a>(parent_asset_ids: &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, parent_id: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_find_parent_index">find_parent_index</a>(parent_asset_ids: &vector&lt;ID&gt;, parent_id: &ID): u64 {
    <b>let</b> len = vector::length(parent_asset_ids);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>if</b> (vector::borrow(parent_asset_ids, i) == parent_id) {
            <b>return</b> i
        };
        i = i + 1;
    };
    len
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_insert_parent_into_lineage_vectors"></a>

## Function `insert_parent_into_lineage_vectors`

Keeps parallel lineage vectors index-aligned when parent_asset_ids is canonically sorted.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_insert_parent_into_lineage_vectors">insert_parent_into_lineage_vectors</a>(parent_asset_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, license_instance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, template_version_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, parent_policy_version: u64, parent_asset_ids: &<b>mut</b> vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, license_instance_ids: &<b>mut</b> vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, template_version_ids: &<b>mut</b> vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, parent_policy_versions: &<b>mut</b> vector&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_insert_parent_into_lineage_vectors">insert_parent_into_lineage_vectors</a>(
    parent_asset_id: ID,
    license_instance_id: ID,
    template_version_id: ID,
    parent_policy_version: u64,
    parent_asset_ids: &<b>mut</b> vector&lt;ID&gt;,
    license_instance_ids: &<b>mut</b> vector&lt;ID&gt;,
    template_version_ids: &<b>mut</b> vector&lt;ID&gt;,
    parent_policy_versions: &<b>mut</b> vector&lt;u64&gt;,
) {
    <b>let</b> insert_at = <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_find_parent_index">find_parent_index</a>(parent_asset_ids, &parent_asset_id);
    vector::insert(parent_asset_ids, parent_asset_id, insert_at);
    vector::insert(license_instance_ids, license_instance_id, insert_at);
    vector::insert(template_version_ids, template_version_id, insert_at);
    vector::insert(parent_policy_versions, parent_policy_version, insert_at);
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_merge_compose_accumulator"></a>

## Function `merge_compose_accumulator`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_merge_compose_accumulator">merge_compose_accumulator</a>(acc: &<b>mut</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">social_contracts::derivative_graph::PendingComposeAccumulator</a>, parent_effective_rights: u64, parent_derivatives_allowed: bool, parent_commercial_allowed: bool, parent_attribution_required: bool, parent_share_bps: u64, parent_obligations: vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>&gt;, edge_obligation: <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>, parent_asset_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, license_instance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, template_version_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, parent_policy_version: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_merge_compose_accumulator">merge_compose_accumulator</a>(
    acc: &<b>mut</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_PendingComposeAccumulator">PendingComposeAccumulator</a>,
    parent_effective_rights: u64,
    parent_derivatives_allowed: bool,
    parent_commercial_allowed: bool,
    parent_attribution_required: bool,
    parent_share_bps: u64,
    parent_obligations: vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a>&gt;,
    edge_obligation: <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a>,
    parent_asset_id: ID,
    license_instance_id: ID,
    template_version_id: ID,
    parent_policy_version: u64,
) {
    acc.effective_rights = acc.effective_rights & parent_effective_rights;
    acc.derivatives_allowed = acc.derivatives_allowed && parent_derivatives_allowed;
    acc.commercial_allowed = acc.commercial_allowed && parent_commercial_allowed;
    acc.attribution_required = acc.attribution_required || parent_attribution_required;
    <b>let</b> new_total = acc.total_edge_share_bps + parent_share_bps;
    <b>assert</b>!(new_total &lt;= <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_BPS_TOTAL">BPS_TOTAL</a>, <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_ROYALTY_OVERFLOW">E_ROYALTY_OVERFLOW</a>);
    acc.total_edge_share_bps = new_total;
    <b>let</b> <b>mut</b> i = 0;
    <b>let</b> parent_len = vector::length(&parent_obligations);
    <b>while</b> (i &lt; parent_len) {
        vector::push_back(&<b>mut</b> acc.accumulated_obligations, *vector::borrow(&parent_obligations, i));
        i = i + 1;
    };
    vector::push_back(&<b>mut</b> acc.accumulated_obligations, edge_obligation);
    <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_insert_parent_into_lineage_vectors">insert_parent_into_lineage_vectors</a>(
        parent_asset_id,
        license_instance_id,
        template_version_id,
        parent_policy_version,
        &<b>mut</b> acc.parent_asset_ids,
        &<b>mut</b> acc.license_instance_ids,
        &<b>mut</b> acc.template_version_ids,
        &<b>mut</b> acc.parent_policy_versions,
    );
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_find_edge_by_relationship_id"></a>

## Function `find_edge_by_relationship_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_find_edge_by_relationship_id">find_edge_by_relationship_id</a>(edges: &vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">social_contracts::derivative_graph::DerivativeRelationship</a>&gt;, relationship_id: u64): &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">social_contracts::derivative_graph::DerivativeRelationship</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_find_edge_by_relationship_id">find_edge_by_relationship_id</a>(
    edges: &vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a>&gt;,
    relationship_id: u64,
): &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a> {
    <b>let</b> len = vector::length(edges);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> edge = vector::borrow(edges, i);
        <b>if</b> (edge.relationship_id == relationship_id) {
            <b>return</b> edge
        };
        i = i + 1;
    };
    <b>abort</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_DUPLICATE_RELATIONSHIP">E_DUPLICATE_RELATIONSHIP</a>
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_set_child_asset_id_on_edges"></a>

## Function `set_child_asset_id_on_edges`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_set_child_asset_id_on_edges">set_child_asset_id_on_edges</a>(edges: &<b>mut</b> vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">social_contracts::derivative_graph::DerivativeRelationship</a>&gt;, child_asset_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_set_child_asset_id_on_edges">set_child_asset_id_on_edges</a>(
    edges: &<b>mut</b> vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a>&gt;,
    child_asset_id: ID,
) {
    <b>let</b> len = vector::length(edges);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> edge = vector::borrow_mut(edges, i);
        edge.child_asset_id = child_asset_id;
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_empty_ancestry"></a>

## Function `empty_ancestry`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_empty_ancestry">empty_ancestry</a>(): <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">social_contracts::derivative_graph::AncestryMetadata</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_empty_ancestry">empty_ancestry</a>(): <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">AncestryMetadata</a> {
    <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">AncestryMetadata</a> {
        <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestor_ids">ancestor_ids</a>: vector[],
        <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestry_version">ancestry_version</a>: 0,
    }
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_ancestry_version"></a>

## Function `ancestry_version`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestry_version">ancestry_version</a>(metadata: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">social_contracts::derivative_graph::AncestryMetadata</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestry_version">ancestry_version</a>(metadata: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">AncestryMetadata</a>): u64 {
    metadata.<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestry_version">ancestry_version</a>
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_ancestor_ids"></a>

## Function `ancestor_ids`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestor_ids">ancestor_ids</a>(metadata: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">social_contracts::derivative_graph::AncestryMetadata</a>): &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestor_ids">ancestor_ids</a>(metadata: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">AncestryMetadata</a>): &vector&lt;ID&gt; {
    &metadata.<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestor_ids">ancestor_ids</a>
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_new_derivative_relationship"></a>

## Function `new_derivative_relationship`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_new_derivative_relationship">new_derivative_relationship</a>(relationship_id: u64, parent_asset_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, child_asset_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, relationship_type: u8, license_instance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, template_version_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, parent_share_bps: u64, evidence_commitment: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, created_at: u64): <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">social_contracts::derivative_graph::DerivativeRelationship</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_new_derivative_relationship">new_derivative_relationship</a>(
    relationship_id: u64,
    parent_asset_id: ID,
    child_asset_id: ID,
    relationship_type: u8,
    license_instance_id: ID,
    template_version_id: ID,
    parent_share_bps: u64,
    evidence_commitment: Option&lt;vector&lt;u8&gt;&gt;,
    created_at: u64,
): <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a> {
    <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a> {
        relationship_id,
        parent_asset_id,
        child_asset_id,
        relationship_type,
        license_instance_id,
        template_version_id,
        parent_share_bps,
        evidence_commitment,
        created_at,
    }
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_edge_parent_asset_id"></a>

## Function `edge_parent_asset_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_edge_parent_asset_id">edge_parent_asset_id</a>(edge: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">social_contracts::derivative_graph::DerivativeRelationship</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_edge_parent_asset_id">edge_parent_asset_id</a>(edge: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a>): ID {
    edge.parent_asset_id
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_edge_relationship_id"></a>

## Function `edge_relationship_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_edge_relationship_id">edge_relationship_id</a>(edge: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">social_contracts::derivative_graph::DerivativeRelationship</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_edge_relationship_id">edge_relationship_id</a>(edge: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a>): u64 {
    edge.relationship_id
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_edge_license_instance_id"></a>

## Function `edge_license_instance_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_edge_license_instance_id">edge_license_instance_id</a>(edge: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">social_contracts::derivative_graph::DerivativeRelationship</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_edge_license_instance_id">edge_license_instance_id</a>(edge: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a>): ID {
    edge.license_instance_id
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_edge_template_version_id"></a>

## Function `edge_template_version_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_edge_template_version_id">edge_template_version_id</a>(edge: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">social_contracts::derivative_graph::DerivativeRelationship</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_edge_template_version_id">edge_template_version_id</a>(edge: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a>): ID {
    edge.template_version_id
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_edge_parent_share_bps"></a>

## Function `edge_parent_share_bps`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_edge_parent_share_bps">edge_parent_share_bps</a>(edge: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">social_contracts::derivative_graph::DerivativeRelationship</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_edge_parent_share_bps">edge_parent_share_bps</a>(edge: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a>): u64 {
    edge.parent_share_bps
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_new_resolved_royalty_obligation"></a>

## Function `new_resolved_royalty_obligation`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_new_resolved_royalty_obligation">new_resolved_royalty_obligation</a>(beneficiary_asset_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, beneficiary_address: <b>address</b>, share_bps: u64, source_relationship_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, source_license_instance_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, obligation_kind: u8): <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_new_resolved_royalty_obligation">new_resolved_royalty_obligation</a>(
    beneficiary_asset_id: Option&lt;ID&gt;,
    beneficiary_address: <b>address</b>,
    share_bps: u64,
    source_relationship_id: Option&lt;u64&gt;,
    source_license_instance_id: Option&lt;ID&gt;,
    obligation_kind: u8,
): <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a> {
    <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a> {
        beneficiary_asset_id,
        beneficiary_address,
        share_bps,
        source_relationship_id,
        source_license_instance_id,
        obligation_kind,
    }
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_obligation_beneficiary_asset_id"></a>

## Function `obligation_beneficiary_asset_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_beneficiary_asset_id">obligation_beneficiary_asset_id</a>(obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_beneficiary_asset_id">obligation_beneficiary_asset_id</a>(
    obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a>,
): Option&lt;ID&gt; {
    obligation.beneficiary_asset_id
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_obligation_beneficiary_address"></a>

## Function `obligation_beneficiary_address`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_beneficiary_address">obligation_beneficiary_address</a>(obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_beneficiary_address">obligation_beneficiary_address</a>(
    obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a>,
): <b>address</b> {
    obligation.beneficiary_address
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_obligation_share_bps"></a>

## Function `obligation_share_bps`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_share_bps">obligation_share_bps</a>(obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_share_bps">obligation_share_bps</a>(obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a>): u64 {
    obligation.share_bps
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_obligation_source_relationship_id"></a>

## Function `obligation_source_relationship_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_source_relationship_id">obligation_source_relationship_id</a>(obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_source_relationship_id">obligation_source_relationship_id</a>(
    obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a>,
): Option&lt;u64&gt; {
    obligation.source_relationship_id
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_obligation_source_license_instance_id"></a>

## Function `obligation_source_license_instance_id`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_source_license_instance_id">obligation_source_license_instance_id</a>(obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_source_license_instance_id">obligation_source_license_instance_id</a>(
    obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a>,
): Option&lt;ID&gt; {
    obligation.source_license_instance_id
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_obligation_obligation_kind"></a>

## Function `obligation_obligation_kind`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_obligation_kind">obligation_obligation_kind</a>(obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">social_contracts::derivative_graph::ResolvedRoyaltyObligation</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_obligation_obligation_kind">obligation_obligation_kind</a>(obligation: &<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ResolvedRoyaltyObligation">ResolvedRoyaltyObligation</a>): u8 {
    obligation.obligation_kind
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_id_vector_contains"></a>

## Function `id_vector_contains`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_id_vector_contains">id_vector_contains</a>(haystack: &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, needle: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_id_vector_contains">id_vector_contains</a>(haystack: &vector&lt;ID&gt;, needle: &ID): bool {
    <b>let</b> len = vector::length(haystack);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> candidate = vector::borrow(haystack, i);
        <b>if</b> (candidate == needle) {
            <b>return</b> <b>true</b>
        };
        i = i + 1;
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_id_less_than"></a>

## Function `id_less_than`



<pre><code><b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_id_less_than">id_less_than</a>(a: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, b: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_id_less_than">id_less_than</a>(a: &ID, b: &ID): bool {
    <b>let</b> a_bytes = object::id_to_bytes(a);
    <b>let</b> b_bytes = object::id_to_bytes(b);
    <b>let</b> a_len = vector::length(&a_bytes);
    <b>let</b> b_len = vector::length(&b_bytes);
    <b>let</b> min_len = <b>if</b> (a_len &lt; b_len) { a_len } <b>else</b> { b_len };
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; min_len) {
        <b>let</b> a_byte = *vector::borrow(&a_bytes, i);
        <b>let</b> b_byte = *vector::borrow(&b_bytes, i);
        <b>if</b> (a_byte &lt; b_byte) {
            <b>return</b> <b>true</b>
        };
        <b>if</b> (a_byte &gt; b_byte) {
            <b>return</b> <b>false</b>
        };
        i = i + 1;
    };
    a_len &lt; b_len
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_sort_ids_ascending"></a>

## Function `sort_ids_ascending`



<pre><code><b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_sort_ids_ascending">sort_ids_ascending</a>(ids: &<b>mut</b> vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_sort_ids_ascending">sort_ids_ascending</a>(ids: &<b>mut</b> vector&lt;ID&gt;) {
    <b>let</b> len = vector::length(ids);
    <b>if</b> (len &lt;= 1) {
        <b>return</b>
    };
    <b>let</b> <b>mut</b> i = 1;
    <b>while</b> (i &lt; len) {
        <b>let</b> key = *vector::borrow(ids, i);
        <b>let</b> <b>mut</b> j = i;
        <b>while</b> (j &gt; 0) {
            <b>let</b> prev = vector::borrow(ids, j - 1);
            <b>if</b> (!<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_id_less_than">id_less_than</a>(&key, prev)) {
                <b>break</b>
            };
            *vector::borrow_mut(ids, j) = *prev;
            j = j - 1;
        };
        *vector::borrow_mut(ids, j) = key;
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_push_id_if_absent"></a>

## Function `push_id_if_absent`



<pre><code><b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_push_id_if_absent">push_id_if_absent</a>(ids: &<b>mut</b> vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_push_id_if_absent">push_id_if_absent</a>(ids: &<b>mut</b> vector&lt;ID&gt;, id: ID) {
    <b>if</b> (!<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_id_vector_contains">id_vector_contains</a>(ids, &id)) {
        vector::push_back(ids, id);
    };
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_canonical_union"></a>

## Function `canonical_union`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_canonical_union">canonical_union</a>(a: &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, b: &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, extra: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;): vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_canonical_union">canonical_union</a>(
    a: &vector&lt;ID&gt;,
    b: &vector&lt;ID&gt;,
    extra: Option&lt;ID&gt;,
): vector&lt;ID&gt; {
    <b>let</b> <b>mut</b> merged = vector[];
    <b>let</b> a_len = vector::length(a);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; a_len) {
        <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_push_id_if_absent">push_id_if_absent</a>(&<b>mut</b> merged, *vector::borrow(a, i));
        i = i + 1;
    };
    <b>let</b> b_len = vector::length(b);
    i = 0;
    <b>while</b> (i &lt; b_len) {
        <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_push_id_if_absent">push_id_if_absent</a>(&<b>mut</b> merged, *vector::borrow(b, i));
        i = i + 1;
    };
    <b>if</b> (option::is_some(&extra)) {
        <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_push_id_if_absent">push_id_if_absent</a>(&<b>mut</b> merged, *option::borrow(&extra));
    };
    <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_sort_ids_ascending">sort_ids_ascending</a>(&<b>mut</b> merged);
    <b>assert</b>!(vector::length(&merged) &lt;= <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_max_ancestors">max_ancestors</a>(), <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_ANCESTOR_LIMIT">E_ANCESTOR_LIMIT</a>);
    merged
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_assert_valid_parent_child_edge"></a>

## Function `assert_valid_parent_child_edge`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_assert_valid_parent_child_edge">assert_valid_parent_child_edge</a>(parent_id: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, child_id: &<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, parent_ancestors: &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_assert_valid_parent_child_edge">assert_valid_parent_child_edge</a>(
    parent_id: &ID,
    child_id: &ID,
    parent_ancestors: &vector&lt;ID&gt;,
) {
    <b>assert</b>!(parent_id != child_id, <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_SELF_EDGE">E_SELF_EDGE</a>);
    <b>assert</b>!(!<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_id_vector_contains">id_vector_contains</a>(parent_ancestors, child_id), <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_CYCLE_DETECTED">E_CYCLE_DETECTED</a>);
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_merge_ancestry_for_edge"></a>

## Function `merge_ancestry_for_edge`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_merge_ancestry_for_edge">merge_ancestry_for_edge</a>(child: &<b>mut</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">social_contracts::derivative_graph::AncestryMetadata</a>, parent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, parent_ancestors: &vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_merge_ancestry_for_edge">merge_ancestry_for_edge</a>(
    child: &<b>mut</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_AncestryMetadata">AncestryMetadata</a>,
    parent_id: ID,
    parent_ancestors: &vector&lt;ID&gt;,
) {
    child.<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestor_ids">ancestor_ids</a> = <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_canonical_union">canonical_union</a>(
        &child.<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestor_ids">ancestor_ids</a>,
        parent_ancestors,
        option::some(parent_id),
    );
    child.<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestry_version">ancestry_version</a> = child.<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_ancestry_version">ancestry_version</a> + 1;
}
</code></pre>



</details>

<a name="social_contracts_derivative_graph_assert_relationship_unique"></a>

## Function `assert_relationship_unique`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_assert_relationship_unique">assert_relationship_unique</a>(existing: &vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">social_contracts::derivative_graph::DerivativeRelationship</a>&gt;, parent_asset_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, child_asset_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, relationship_type: u8, license_instance_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_assert_relationship_unique">assert_relationship_unique</a>(
    existing: &vector&lt;<a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_DerivativeRelationship">DerivativeRelationship</a>&gt;,
    parent_asset_id: ID,
    child_asset_id: ID,
    relationship_type: u8,
    license_instance_id: ID,
) {
    <b>let</b> len = vector::length(existing);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>let</b> edge = vector::borrow(existing, i);
        <b>if</b> (edge.parent_asset_id == parent_asset_id &&
            edge.child_asset_id == child_asset_id &&
            edge.relationship_type == relationship_type &&
            edge.license_instance_id == license_instance_id) {
            <b>abort</b> <a href="../social_contracts/media_asset.md#social_contracts_derivative_graph_E_DUPLICATE_RELATIONSHIP">E_DUPLICATE_RELATIONSHIP</a>
        };
        i = i + 1;
    };
}
</code></pre>



</details>
