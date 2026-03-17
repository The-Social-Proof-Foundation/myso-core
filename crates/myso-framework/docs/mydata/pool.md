---
title: Module `mydata::pool`
---

MyData Query Marketplace: Broad Pools, Sub-Pools, Query Snapshots, Claim Vault, Merkle Settlement.
All pool/marketplace logic in one module.


-  [Struct `BroadPool`](#mydata_pool_BroadPool)
-  [Struct `SubPool`](#mydata_pool_SubPool)
-  [Struct `MyDataPoolRegistry`](#mydata_pool_MyDataPoolRegistry)
-  [Struct `MyDataPoolAdminCap`](#mydata_pool_MyDataPoolAdminCap)
-  [Struct `BroadPoolCreatedEvent`](#mydata_pool_BroadPoolCreatedEvent)
-  [Struct `SubPoolCreatedEvent`](#mydata_pool_SubPoolCreatedEvent)
-  [Struct `MyDataAssignedToSubPoolEvent`](#mydata_pool_MyDataAssignedToSubPoolEvent)
-  [Struct `QuerySnapshotAnchor`](#mydata_pool_QuerySnapshotAnchor)
-  [Struct `SnapshotAnchorRegistry`](#mydata_pool_SnapshotAnchorRegistry)
-  [Struct `SnapshotAnchorRecordedEvent`](#mydata_pool_SnapshotAnchorRecordedEvent)
-  [Struct `MyDataClaimVault`](#mydata_pool_MyDataClaimVault)
-  [Struct `MerkleRootPublishedEvent`](#mydata_pool_MerkleRootPublishedEvent)
-  [Struct `ClaimExecutedEvent`](#mydata_pool_ClaimExecutedEvent)
-  [Struct `DistributionRound`](#mydata_pool_DistributionRound)
-  [Struct `DistributionRegistry`](#mydata_pool_DistributionRegistry)
-  [Constants](#@Constants_0)
-  [Function `create_admin_cap`](#mydata_pool_create_admin_cap)
-  [Function `bootstrap_init`](#mydata_pool_bootstrap_init)
-  [Function `gen_pool_id`](#mydata_pool_gen_pool_id)
-  [Function `create_broad_pool`](#mydata_pool_create_broad_pool)
-  [Function `create_sub_pool`](#mydata_pool_create_sub_pool)
-  [Function `assign_mydata_to_sub_pools`](#mydata_pool_assign_mydata_to_sub_pools)
-  [Function `remove_mydata_from_sub_pool`](#mydata_pool_remove_mydata_from_sub_pool)
-  [Function `gen_snapshot_id`](#mydata_pool_gen_snapshot_id)
-  [Function `record_snapshot_anchor`](#mydata_pool_record_snapshot_anchor)
-  [Function `get_snapshot_anchor`](#mydata_pool_get_snapshot_anchor)
-  [Function `publish_merkle_root`](#mydata_pool_publish_merkle_root)
-  [Function `claim`](#mydata_pool_claim)
-  [Function `deposit`](#mydata_pool_deposit)
-  [Function `record_distribution`](#mydata_pool_record_distribution)
-  [Function `get_broad_pool`](#mydata_pool_get_broad_pool)
-  [Function `get_sub_pool`](#mydata_pool_get_sub_pool)
-  [Function `get_mydata_sub_pools`](#mydata_pool_get_mydata_sub_pools)
-  [Function `get_distribution_round`](#mydata_pool_get_distribution_round)
-  [Function `broad_pool_id`](#mydata_pool_broad_pool_id)
-  [Function `sub_pool_id`](#mydata_pool_sub_pool_id)


<pre><code><b>use</b> <a href="../mydata/merkle.md#mydata_merkle">mydata::merkle</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/myso.md#myso_myso">myso::myso</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/internal.md#std_internal">std::internal</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/u128.md#std_u128">std::u128</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="mydata_pool_BroadPool"></a>

## Struct `BroadPool`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_BroadPool">BroadPool</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_SubPool"></a>

## Struct `SubPool`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_SubPool">SubPool</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/pool.md#mydata_pool_broad_pool_id">broad_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>schema_metadata: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_MyDataPoolRegistry"></a>

## Struct `MyDataPoolRegistry`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a> <b>has</b> key
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
<code>broad_pools: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../mydata/pool.md#mydata_pool_BroadPool">mydata::pool::BroadPool</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>sub_pools: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../mydata/pool.md#mydata_pool_SubPool">mydata::pool::SubPool</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>broad_to_sub: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>mydata_to_sub_pools: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>next_broad_pool_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>next_sub_pool_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>last_created_broad_pool_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>last_created_sub_pool_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_MyDataPoolAdminCap"></a>

## Struct `MyDataPoolAdminCap`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">MyDataPoolAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>id: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_BroadPoolCreatedEvent"></a>

## Struct `BroadPoolCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_BroadPoolCreatedEvent">BroadPoolCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
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

<a name="mydata_pool_SubPoolCreatedEvent"></a>

## Struct `SubPoolCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_SubPoolCreatedEvent">SubPoolCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../mydata/pool.md#mydata_pool_broad_pool_id">broad_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
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

<a name="mydata_pool_MyDataAssignedToSubPoolEvent"></a>

## Struct `MyDataAssignedToSubPoolEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_MyDataAssignedToSubPoolEvent">MyDataAssignedToSubPoolEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>ip_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>sub_pool_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>assigned_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_QuerySnapshotAnchor"></a>

## Struct `QuerySnapshotAnchor`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_QuerySnapshotAnchor">QuerySnapshotAnchor</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>buyer_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>source_pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>source_sub_pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>price_paid: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>snapshot_manifest_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>payment_reference: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_SnapshotAnchorRegistry"></a>

## Struct `SnapshotAnchorRegistry`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a> <b>has</b> key
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
<code>anchors: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../mydata/pool.md#mydata_pool_QuerySnapshotAnchor">mydata::pool::QuerySnapshotAnchor</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>next_snapshot_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_SnapshotAnchorRecordedEvent"></a>

## Struct `SnapshotAnchorRecordedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_SnapshotAnchorRecordedEvent">SnapshotAnchorRecordedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>buyer_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>price_paid: u64</code>
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

<a name="mydata_pool_MyDataClaimVault"></a>

## Struct `MyDataClaimVault`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">MyDataClaimVault</a> <b>has</b> key
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
<code>balance: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>merkle_roots: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, vector&lt;u8&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, bool&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_MerkleRootPublishedEvent"></a>

## Struct `MerkleRootPublishedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_MerkleRootPublishedEvent">MerkleRootPublishedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>root_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>published_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_ClaimExecutedEvent"></a>

## Struct `ClaimExecutedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_ClaimExecutedEvent">ClaimExecutedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>claimant: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_DistributionRound"></a>

## Struct `DistributionRound`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_DistributionRound">DistributionRound</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>total_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>contributor_count: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>merkle_root: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>published_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="mydata_pool_DistributionRegistry"></a>

## Struct `DistributionRegistry`



<pre><code><b>public</b> <b>struct</b> <a href="../mydata/pool.md#mydata_pool_DistributionRegistry">DistributionRegistry</a> <b>has</b> key
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
<code>rounds: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../mydata/pool.md#mydata_pool_DistributionRound">mydata::pool::DistributionRound</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="mydata_pool_VERSION"></a>



<pre><code><b>const</b> <a href="../mydata/pool.md#mydata_pool_VERSION">VERSION</a>: u64 = 1;
</code></pre>



<a name="mydata_pool_EInvalidInput"></a>



<pre><code><b>const</b> <a href="../mydata/pool.md#mydata_pool_EInvalidInput">EInvalidInput</a>: u64 = 2;
</code></pre>



<a name="mydata_pool_EPoolNotFound"></a>



<pre><code><b>const</b> <a href="../mydata/pool.md#mydata_pool_EPoolNotFound">EPoolNotFound</a>: u64 = 3;
</code></pre>



<a name="mydata_pool_ESubPoolNotFound"></a>



<pre><code><b>const</b> <a href="../mydata/pool.md#mydata_pool_ESubPoolNotFound">ESubPoolNotFound</a>: u64 = 4;
</code></pre>



<a name="mydata_pool_EInvalidProof"></a>



<pre><code><b>const</b> <a href="../mydata/pool.md#mydata_pool_EInvalidProof">EInvalidProof</a>: u64 = 5;
</code></pre>



<a name="mydata_pool_EAlreadyClaimed"></a>



<pre><code><b>const</b> <a href="../mydata/pool.md#mydata_pool_EAlreadyClaimed">EAlreadyClaimed</a>: u64 = 6;
</code></pre>



<a name="mydata_pool_EMerkleRootNotPublished"></a>



<pre><code><b>const</b> <a href="../mydata/pool.md#mydata_pool_EMerkleRootNotPublished">EMerkleRootNotPublished</a>: u64 = 7;
</code></pre>



<a name="mydata_pool_EInsufficientPayment"></a>



<pre><code><b>const</b> <a href="../mydata/pool.md#mydata_pool_EInsufficientPayment">EInsufficientPayment</a>: u64 = 8;
</code></pre>



<a name="mydata_pool_create_admin_cap"></a>

## Function `create_admin_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_create_admin_cap">create_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">mydata::pool::MyDataPoolAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_create_admin_cap">create_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">MyDataPoolAdminCap</a> {
    <a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">MyDataPoolAdminCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="mydata_pool_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_bootstrap_init">bootstrap_init</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> registry = <a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a> {
        id: object::new(ctx),
        broad_pools: table::new(ctx),
        sub_pools: table::new(ctx),
        broad_to_sub: table::new(ctx),
        mydata_to_sub_pools: table::new(ctx),
        next_broad_pool_nonce: 0,
        next_sub_pool_nonce: 0,
        last_created_broad_pool_id: option::none(),
        last_created_sub_pool_id: option::none(),
        version: <a href="../mydata/pool.md#mydata_pool_VERSION">VERSION</a>,
    };
    transfer::share_object(registry);
    <b>let</b> anchor_registry = <a href="../mydata/pool.md#mydata_pool_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a> {
        id: object::new(ctx),
        anchors: table::new(ctx),
        next_snapshot_nonce: 0,
        version: <a href="../mydata/pool.md#mydata_pool_VERSION">VERSION</a>,
    };
    transfer::share_object(anchor_registry);
    <b>let</b> vault = <a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">MyDataClaimVault</a> {
        id: object::new(ctx),
        balance: balance::zero(),
        merkle_roots: table::new(ctx),
        claimed: table::new(ctx),
        version: <a href="../mydata/pool.md#mydata_pool_VERSION">VERSION</a>,
    };
    transfer::share_object(vault);
    <b>let</b> dist_registry = <a href="../mydata/pool.md#mydata_pool_DistributionRegistry">DistributionRegistry</a> {
        id: object::new(ctx),
        rounds: table::new(ctx),
        version: <a href="../mydata/pool.md#mydata_pool_VERSION">VERSION</a>,
    };
    transfer::share_object(dist_registry);
}
</code></pre>



</details>

<a name="mydata_pool_gen_pool_id"></a>

## Function `gen_pool_id`



<pre><code><b>fun</b> <a href="../mydata/pool.md#mydata_pool_gen_pool_id">gen_pool_id</a>(registry: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">mydata::pool::MyDataPoolRegistry</a>, nonce: u64): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/pool.md#mydata_pool_gen_pool_id">gen_pool_id</a>(registry: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a>, nonce: u64): ID {
    <b>let</b> <b>mut</b> data = bcs::to_bytes(&object::uid_to_address(&registry.id));
    vector::append(&<b>mut</b> data, bcs::to_bytes(&nonce));
    object::id_from_bytes(hash::blake2b256(&data))
}
</code></pre>



</details>

<a name="mydata_pool_create_broad_pool"></a>

## Function `create_broad_pool`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_create_broad_pool">create_broad_pool</a>(_: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">mydata::pool::MyDataPoolAdminCap</a>, registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">mydata::pool::MyDataPoolRegistry</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_create_broad_pool">create_broad_pool</a>(
    _: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    name: String,
    description: String,
    clock: &Clock,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>let</b> nonce = registry.next_broad_pool_nonce;
    registry.next_broad_pool_nonce = nonce + 1;
    <b>let</b> pool_id = <a href="../mydata/pool.md#mydata_pool_gen_pool_id">gen_pool_id</a>(registry, nonce);
    <b>let</b> broad_pool = <a href="../mydata/pool.md#mydata_pool_BroadPool">BroadPool</a> {
        id: pool_id,
        name,
        description,
        created_at: clock::timestamp_ms(clock),
        version: <a href="../mydata/pool.md#mydata_pool_VERSION">VERSION</a>,
    };
    table::add(&<b>mut</b> registry.broad_pools, pool_id, broad_pool);
    table::add(&<b>mut</b> registry.broad_to_sub, pool_id, vector::empty());
    registry.last_created_broad_pool_id = option::some(pool_id);
    event::emit(<a href="../mydata/pool.md#mydata_pool_BroadPoolCreatedEvent">BroadPoolCreatedEvent</a> {
        pool_id,
        name: broad_pool.name,
        created_at: broad_pool.created_at,
    });
}
</code></pre>



</details>

<a name="mydata_pool_create_sub_pool"></a>

## Function `create_sub_pool`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_create_sub_pool">create_sub_pool</a>(_: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">mydata::pool::MyDataPoolAdminCap</a>, registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">mydata::pool::MyDataPoolRegistry</a>, <a href="../mydata/pool.md#mydata_pool_broad_pool_id">broad_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, schema_metadata: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, _ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_create_sub_pool">create_sub_pool</a>(
    _: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    <a href="../mydata/pool.md#mydata_pool_broad_pool_id">broad_pool_id</a>: ID,
    name: String,
    description: String,
    schema_metadata: Option&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
    _ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(table::contains(&registry.broad_pools, <a href="../mydata/pool.md#mydata_pool_broad_pool_id">broad_pool_id</a>), <a href="../mydata/pool.md#mydata_pool_EPoolNotFound">EPoolNotFound</a>);
    <b>let</b> nonce = registry.next_sub_pool_nonce;
    registry.next_sub_pool_nonce = nonce + 1;
    <b>let</b> <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a> = <a href="../mydata/pool.md#mydata_pool_gen_pool_id">gen_pool_id</a>(registry, 0x100000000 | nonce);
    <b>let</b> sub_pool = <a href="../mydata/pool.md#mydata_pool_SubPool">SubPool</a> {
        id: <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>,
        <a href="../mydata/pool.md#mydata_pool_broad_pool_id">broad_pool_id</a>,
        name,
        description,
        schema_metadata,
        created_at: clock::timestamp_ms(clock),
        version: <a href="../mydata/pool.md#mydata_pool_VERSION">VERSION</a>,
    };
    table::add(&<b>mut</b> registry.sub_pools, <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>, sub_pool);
    registry.last_created_sub_pool_id = option::some(<a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>);
    <b>let</b> sub_ids = table::borrow_mut(&<b>mut</b> registry.broad_to_sub, <a href="../mydata/pool.md#mydata_pool_broad_pool_id">broad_pool_id</a>);
    vector::push_back(sub_ids, <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>);
    event::emit(<a href="../mydata/pool.md#mydata_pool_SubPoolCreatedEvent">SubPoolCreatedEvent</a> {
        <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>,
        <a href="../mydata/pool.md#mydata_pool_broad_pool_id">broad_pool_id</a>,
        name: sub_pool.name,
        created_at: sub_pool.created_at,
    });
}
</code></pre>



</details>

<a name="mydata_pool_assign_mydata_to_sub_pools"></a>

## Function `assign_mydata_to_sub_pools`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_assign_mydata_to_sub_pools">assign_mydata_to_sub_pools</a>(registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">mydata::pool::MyDataPoolRegistry</a>, ip_id: <b>address</b>, sub_pool_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_assign_mydata_to_sub_pools">assign_mydata_to_sub_pools</a>(
    registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    ip_id: <b>address</b>,
    sub_pool_ids: vector&lt;ID&gt;,
    clock: &Clock,
) {
    <b>let</b> <b>mut</b> existing = <b>if</b> (table::contains(&registry.mydata_to_sub_pools, ip_id)) {
        *table::borrow(&registry.mydata_to_sub_pools, ip_id)
    } <b>else</b> {
        vector::empty()
    };
    <b>let</b> <b>mut</b> i = 0u64;
    <b>while</b> (i &lt; vector::length(&sub_pool_ids)) {
        <b>let</b> sub_id = *vector::borrow(&sub_pool_ids, i);
        <b>assert</b>!(table::contains(&registry.sub_pools, sub_id), <a href="../mydata/pool.md#mydata_pool_ESubPoolNotFound">ESubPoolNotFound</a>);
        <b>let</b> (<b>has</b>, _) = vector::index_of(&existing, &sub_id);
        <b>if</b> (!<b>has</b>) {
            vector::push_back(&<b>mut</b> existing, sub_id);
        };
        i = i + 1;
    };
    <b>if</b> (table::contains(&registry.mydata_to_sub_pools, ip_id)) {
        *table::borrow_mut(&<b>mut</b> registry.mydata_to_sub_pools, ip_id) = existing;
    } <b>else</b> {
        table::add(&<b>mut</b> registry.mydata_to_sub_pools, ip_id, existing);
    };
    event::emit(<a href="../mydata/pool.md#mydata_pool_MyDataAssignedToSubPoolEvent">MyDataAssignedToSubPoolEvent</a> {
        ip_id,
        sub_pool_ids,
        assigned_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="mydata_pool_remove_mydata_from_sub_pool"></a>

## Function `remove_mydata_from_sub_pool`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_remove_mydata_from_sub_pool">remove_mydata_from_sub_pool</a>(registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">mydata::pool::MyDataPoolRegistry</a>, ip_id: <b>address</b>, <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, _clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_remove_mydata_from_sub_pool">remove_mydata_from_sub_pool</a>(
    registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    ip_id: <b>address</b>,
    <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>: ID,
    _clock: &Clock,
) {
    <b>assert</b>!(table::contains(&registry.mydata_to_sub_pools, ip_id), <a href="../mydata/pool.md#mydata_pool_EInvalidInput">EInvalidInput</a>);
    <b>let</b> sub_ids = table::borrow_mut(&<b>mut</b> registry.mydata_to_sub_pools, ip_id);
    <b>let</b> (found, idx) = vector::index_of(sub_ids, &<a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>);
    <b>assert</b>!(found, <a href="../mydata/pool.md#mydata_pool_EInvalidInput">EInvalidInput</a>);
    vector::remove(sub_ids, idx);
}
</code></pre>



</details>

<a name="mydata_pool_gen_snapshot_id"></a>

## Function `gen_snapshot_id`



<pre><code><b>fun</b> <a href="../mydata/pool.md#mydata_pool_gen_snapshot_id">gen_snapshot_id</a>(registry_id: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, nonce: u64): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../mydata/pool.md#mydata_pool_gen_snapshot_id">gen_snapshot_id</a>(registry_id: &UID, nonce: u64): ID {
    <b>let</b> <b>mut</b> data = bcs::to_bytes(&object::uid_to_address(registry_id));
    vector::append(&<b>mut</b> data, bcs::to_bytes(&nonce));
    object::id_from_bytes(hash::blake2b256(&data))
}
</code></pre>



</details>

<a name="mydata_pool_record_snapshot_anchor"></a>

## Function `record_snapshot_anchor`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_record_snapshot_anchor">record_snapshot_anchor</a>(anchor_registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_SnapshotAnchorRegistry">mydata::pool::SnapshotAnchorRegistry</a>, vault: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">mydata::pool::MyDataClaimVault</a>, pool_registry: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">mydata::pool::MyDataPoolRegistry</a>, source_pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, source_sub_pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, manifest_hash: vector&lt;u8&gt;, payment_reference: vector&lt;u8&gt;, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_record_snapshot_anchor">record_snapshot_anchor</a>(
    anchor_registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a>,
    vault: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">MyDataClaimVault</a>,
    pool_registry: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    source_pool_id: ID,
    source_sub_pool_id: ID,
    manifest_hash: vector&lt;u8&gt;,
    payment_reference: vector&lt;u8&gt;,
    payment: Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(table::contains(&pool_registry.broad_pools, source_pool_id), <a href="../mydata/pool.md#mydata_pool_EPoolNotFound">EPoolNotFound</a>);
    <b>assert</b>!(table::contains(&pool_registry.sub_pools, source_sub_pool_id), <a href="../mydata/pool.md#mydata_pool_ESubPoolNotFound">ESubPoolNotFound</a>);
    <b>let</b> price_paid = coin::value(&payment);
    <b>assert</b>!(price_paid &gt; 0, <a href="../mydata/pool.md#mydata_pool_EInsufficientPayment">EInsufficientPayment</a>);
    <b>let</b> nonce = anchor_registry.next_snapshot_nonce;
    anchor_registry.next_snapshot_nonce = nonce + 1;
    <b>let</b> snapshot_id = <a href="../mydata/pool.md#mydata_pool_gen_snapshot_id">gen_snapshot_id</a>(&anchor_registry.id, nonce);
    <b>let</b> buyer = tx_context::sender(ctx);
    <b>let</b> anchor = <a href="../mydata/pool.md#mydata_pool_QuerySnapshotAnchor">QuerySnapshotAnchor</a> {
        snapshot_id,
        buyer_address: buyer,
        source_pool_id,
        source_sub_pool_id,
        price_paid,
        created_at: clock::timestamp_ms(clock),
        snapshot_manifest_hash: manifest_hash,
        payment_reference,
    };
    table::add(&<b>mut</b> anchor_registry.anchors, snapshot_id, anchor);
    balance::join(&<b>mut</b> vault.balance, coin::into_balance(payment));
    event::emit(<a href="../mydata/pool.md#mydata_pool_SnapshotAnchorRecordedEvent">SnapshotAnchorRecordedEvent</a> {
        snapshot_id,
        buyer_address: buyer,
        price_paid,
        created_at: anchor.created_at,
    });
}
</code></pre>



</details>

<a name="mydata_pool_get_snapshot_anchor"></a>

## Function `get_snapshot_anchor`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_get_snapshot_anchor">get_snapshot_anchor</a>(anchor_registry: &<a href="../mydata/pool.md#mydata_pool_SnapshotAnchorRegistry">mydata::pool::SnapshotAnchorRegistry</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mydata/pool.md#mydata_pool_QuerySnapshotAnchor">mydata::pool::QuerySnapshotAnchor</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_get_snapshot_anchor">get_snapshot_anchor</a>(anchor_registry: &<a href="../mydata/pool.md#mydata_pool_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a>, snapshot_id: ID): Option&lt;<a href="../mydata/pool.md#mydata_pool_QuerySnapshotAnchor">QuerySnapshotAnchor</a>&gt; {
    <b>if</b> (table::contains(&anchor_registry.anchors, snapshot_id)) {
        option::some(*table::borrow(&anchor_registry.anchors, snapshot_id))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="mydata_pool_publish_merkle_root"></a>

## Function `publish_merkle_root`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_publish_merkle_root">publish_merkle_root</a>(_: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">mydata::pool::MyDataPoolAdminCap</a>, vault: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">mydata::pool::MyDataClaimVault</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, root_hash: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_publish_merkle_root">publish_merkle_root</a>(
    _: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    vault: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">MyDataClaimVault</a>,
    snapshot_id: ID,
    root_hash: vector&lt;u8&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(vector::length(&root_hash) == 32, <a href="../mydata/pool.md#mydata_pool_EInvalidInput">EInvalidInput</a>);
    table::add(&<b>mut</b> vault.merkle_roots, snapshot_id, root_hash);
    event::emit(<a href="../mydata/pool.md#mydata_pool_MerkleRootPublishedEvent">MerkleRootPublishedEvent</a> {
        snapshot_id,
        root_hash,
        published_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="mydata_pool_claim"></a>

## Function `claim`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_claim">claim</a>(vault: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">mydata::pool::MyDataClaimVault</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, amount: u64, leaf_index: u64, proof: vector&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_claim">claim</a>(
    vault: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">MyDataClaimVault</a>,
    snapshot_id: ID,
    amount: u64,
    leaf_index: u64,
    proof: vector&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(table::contains(&vault.merkle_roots, snapshot_id), <a href="../mydata/pool.md#mydata_pool_EMerkleRootNotPublished">EMerkleRootNotPublished</a>);
    <b>let</b> claimant = tx_context::sender(ctx);
    <b>let</b> leaf = <a href="../mydata/merkle.md#mydata_merkle_leaf_hash">merkle::leaf_hash</a>(claimant, amount, object::id_to_bytes(&snapshot_id));
    <b>let</b> root = *table::borrow(&vault.merkle_roots, snapshot_id);
    <b>assert</b>!(<a href="../mydata/merkle.md#mydata_merkle_verify_proof">merkle::verify_proof</a>(leaf, &proof, leaf_index, root), <a href="../mydata/pool.md#mydata_pool_EInvalidProof">EInvalidProof</a>);
    <b>if</b> (!table::contains(&vault.claimed, snapshot_id)) {
        table::add(&<b>mut</b> vault.claimed, snapshot_id, table::new(ctx));
    };
    <b>let</b> claimed_table = table::borrow_mut(&<b>mut</b> vault.claimed, snapshot_id);
    <b>assert</b>!(!table::contains(claimed_table, claimant), <a href="../mydata/pool.md#mydata_pool_EAlreadyClaimed">EAlreadyClaimed</a>);
    table::add(claimed_table, claimant, <b>true</b>);
    <b>let</b> payout = balance::split(&<b>mut</b> vault.balance, amount);
    transfer::public_transfer(coin::from_balance(payout, ctx), claimant);
    event::emit(<a href="../mydata/pool.md#mydata_pool_ClaimExecutedEvent">ClaimExecutedEvent</a> {
        snapshot_id,
        claimant,
        amount,
        claimed_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="mydata_pool_deposit"></a>

## Function `deposit`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_deposit">deposit</a>(vault: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">mydata::pool::MyDataClaimVault</a>, _snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_deposit">deposit</a>(vault: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">MyDataClaimVault</a>, _snapshot_id: ID, payment: Coin&lt;MYSO&gt;) {
    balance::join(&<b>mut</b> vault.balance, coin::into_balance(payment));
}
</code></pre>



</details>

<a name="mydata_pool_record_distribution"></a>

## Function `record_distribution`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_record_distribution">record_distribution</a>(_: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">mydata::pool::MyDataPoolAdminCap</a>, dist_registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_DistributionRegistry">mydata::pool::DistributionRegistry</a>, vault: &<a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">mydata::pool::MyDataClaimVault</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, total_amount: u64, contributor_count: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_record_distribution">record_distribution</a>(
    _: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    dist_registry: &<b>mut</b> <a href="../mydata/pool.md#mydata_pool_DistributionRegistry">DistributionRegistry</a>,
    vault: &<a href="../mydata/pool.md#mydata_pool_MyDataClaimVault">MyDataClaimVault</a>,
    snapshot_id: ID,
    total_amount: u64,
    contributor_count: u64,
    clock: &Clock,
) {
    <b>assert</b>!(table::contains(&vault.merkle_roots, snapshot_id), <a href="../mydata/pool.md#mydata_pool_EMerkleRootNotPublished">EMerkleRootNotPublished</a>);
    <b>let</b> root = *table::borrow(&vault.merkle_roots, snapshot_id);
    <b>let</b> round = <a href="../mydata/pool.md#mydata_pool_DistributionRound">DistributionRound</a> {
        snapshot_id,
        total_amount,
        contributor_count,
        merkle_root: root,
        published_at: clock::timestamp_ms(clock),
    };
    table::add(&<b>mut</b> dist_registry.rounds, snapshot_id, round);
}
</code></pre>



</details>

<a name="mydata_pool_get_broad_pool"></a>

## Function `get_broad_pool`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_get_broad_pool">get_broad_pool</a>(registry: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">mydata::pool::MyDataPoolRegistry</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mydata/pool.md#mydata_pool_BroadPool">mydata::pool::BroadPool</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_get_broad_pool">get_broad_pool</a>(registry: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a>, pool_id: ID): Option&lt;<a href="../mydata/pool.md#mydata_pool_BroadPool">BroadPool</a>&gt; {
    <b>if</b> (table::contains(&registry.broad_pools, pool_id)) {
        option::some(*table::borrow(&registry.broad_pools, pool_id))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="mydata_pool_get_sub_pool"></a>

## Function `get_sub_pool`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_get_sub_pool">get_sub_pool</a>(registry: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">mydata::pool::MyDataPoolRegistry</a>, <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mydata/pool.md#mydata_pool_SubPool">mydata::pool::SubPool</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_get_sub_pool">get_sub_pool</a>(registry: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a>, <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>: ID): Option&lt;<a href="../mydata/pool.md#mydata_pool_SubPool">SubPool</a>&gt; {
    <b>if</b> (table::contains(&registry.sub_pools, <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>)) {
        option::some(*table::borrow(&registry.sub_pools, <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="mydata_pool_get_mydata_sub_pools"></a>

## Function `get_mydata_sub_pools`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_get_mydata_sub_pools">get_mydata_sub_pools</a>(registry: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">mydata::pool::MyDataPoolRegistry</a>, ip_id: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_get_mydata_sub_pools">get_mydata_sub_pools</a>(registry: &<a href="../mydata/pool.md#mydata_pool_MyDataPoolRegistry">MyDataPoolRegistry</a>, ip_id: <b>address</b>): Option&lt;vector&lt;ID&gt;&gt; {
    <b>if</b> (table::contains(&registry.mydata_to_sub_pools, ip_id)) {
        option::some(*table::borrow(&registry.mydata_to_sub_pools, ip_id))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="mydata_pool_get_distribution_round"></a>

## Function `get_distribution_round`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_get_distribution_round">get_distribution_round</a>(registry: &<a href="../mydata/pool.md#mydata_pool_DistributionRegistry">mydata::pool::DistributionRegistry</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../mydata/pool.md#mydata_pool_DistributionRound">mydata::pool::DistributionRound</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_get_distribution_round">get_distribution_round</a>(registry: &<a href="../mydata/pool.md#mydata_pool_DistributionRegistry">DistributionRegistry</a>, snapshot_id: ID): Option&lt;<a href="../mydata/pool.md#mydata_pool_DistributionRound">DistributionRound</a>&gt; {
    <b>if</b> (table::contains(&registry.rounds, snapshot_id)) {
        option::some(*table::borrow(&registry.rounds, snapshot_id))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="mydata_pool_broad_pool_id"></a>

## Function `broad_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_broad_pool_id">broad_pool_id</a>(<a href="../mydata/pool.md#mydata_pool">pool</a>: &<a href="../mydata/pool.md#mydata_pool_BroadPool">mydata::pool::BroadPool</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_broad_pool_id">broad_pool_id</a>(<a href="../mydata/pool.md#mydata_pool">pool</a>: &<a href="../mydata/pool.md#mydata_pool_BroadPool">BroadPool</a>): ID { <a href="../mydata/pool.md#mydata_pool">pool</a>.id }
</code></pre>



</details>

<a name="mydata_pool_sub_pool_id"></a>

## Function `sub_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>(<a href="../mydata/pool.md#mydata_pool">pool</a>: &<a href="../mydata/pool.md#mydata_pool_SubPool">mydata::pool::SubPool</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../mydata/pool.md#mydata_pool_sub_pool_id">sub_pool_id</a>(<a href="../mydata/pool.md#mydata_pool">pool</a>: &<a href="../mydata/pool.md#mydata_pool_SubPool">SubPool</a>): ID { <a href="../mydata/pool.md#mydata_pool">pool</a>.id }
</code></pre>



</details>
