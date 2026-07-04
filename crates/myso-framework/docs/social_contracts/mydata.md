---
title: Module `social_contracts::mydata`
---

Universal MyData module for encrypted data monetization (one-time purchase, subscription, owner vault).

**Production decrypt path (client-only):** Plaintext must only exist off-chain. Callers encrypt before
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_create">create</a></code> / <code><a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share">create_and_share</a></code>, then authorized users use the MyData SDK: resolve access (indexer or
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_has_access">has_access</a></code>), request keys via the key server (<code>fetch_key</code>) with policy approval (<code><a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve">mydata_approve</a></code>),
and decrypt locally. Do not rely on Move to produce user content plaintext for marketplace listings.

**On-chain state:** <code>encrypted_data</code> is opaque ciphertext. For BF-HMAC MyData blobs,
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a></code> embeds <code>package_id</code> and <code>id</code>; the <code>encryption_id</code> field
must be the same <code>id</code> bytes used when encrypting so policy and clients stay aligned. For client-only
AES-GCM (or other app-managed schemes), ciphertext does not parse as <code>EncryptedObject</code>; encode the
scheme in <code><a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a></code> (e.g. prefix <code>aes_gcm:</code>) or app metadata so indexers pick the right decrypt path.

**Revocation:** Owners may call [<code><a href="../social_contracts/mydata.md#social_contracts_mydata_revoke_access">revoke_access</a></code>] to remove a buyer from <code>purchasers</code> / <code>subscribers</code>.
Permissioned key servers re-check [<code><a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve">mydata_approve</a></code>] on every <code>fetch_key</code>, so revoked buyers cannot
obtain new derived keys. Already-fetched keys may still decrypt offline client-side.

**Query marketplace:** Broad pools, snapshot anchors, claim vault, and Merkle settlement live in this
module. Manifest hash and payout trees are operator-defined; the chain records price paid and anchors,
not row-level dataset membership.


-  [Struct `MyData`](#social_contracts_mydata_MyData)
-  [Struct `MyDataAdminCap`](#social_contracts_mydata_MyDataAdminCap)
-  [Struct `MyDataConfig`](#social_contracts_mydata_MyDataConfig)
-  [Struct `MyDataRegistry`](#social_contracts_mydata_MyDataRegistry)
-  [Struct `BroadPool`](#social_contracts_mydata_BroadPool)
-  [Struct `SubPool`](#social_contracts_mydata_SubPool)
-  [Struct `MyDataPoolRegistry`](#social_contracts_mydata_MyDataPoolRegistry)
-  [Struct `MyDataPoolAdminCap`](#social_contracts_mydata_MyDataPoolAdminCap)
-  [Struct `BroadPoolCreatedEvent`](#social_contracts_mydata_BroadPoolCreatedEvent)
-  [Struct `SubPoolCreatedEvent`](#social_contracts_mydata_SubPoolCreatedEvent)
-  [Struct `MyDataAssignedToSubPoolEvent`](#social_contracts_mydata_MyDataAssignedToSubPoolEvent)
-  [Struct `QuerySnapshotAnchor`](#social_contracts_mydata_QuerySnapshotAnchor)
-  [Struct `SnapshotAnchorRegistry`](#social_contracts_mydata_SnapshotAnchorRegistry)
-  [Struct `SnapshotAnchorRecordedEvent`](#social_contracts_mydata_SnapshotAnchorRecordedEvent)
-  [Struct `DistributionRecordedEvent`](#social_contracts_mydata_DistributionRecordedEvent)
-  [Struct `MyDataClaimVault`](#social_contracts_mydata_MyDataClaimVault)
-  [Struct `MerkleRootPublishedEvent`](#social_contracts_mydata_MerkleRootPublishedEvent)
-  [Struct `ClaimExecutedEvent`](#social_contracts_mydata_ClaimExecutedEvent)
-  [Struct `DistributionRound`](#social_contracts_mydata_DistributionRound)
-  [Struct `DistributionRegistry`](#social_contracts_mydata_DistributionRegistry)
-  [Struct `MyDataCreatedEvent`](#social_contracts_mydata_MyDataCreatedEvent)
-  [Struct `PurchaseEvent`](#social_contracts_mydata_PurchaseEvent)
-  [Struct `AccessGrantedEvent`](#social_contracts_mydata_AccessGrantedEvent)
-  [Struct `AccessRevokedEvent`](#social_contracts_mydata_AccessRevokedEvent)
-  [Struct `MyDataRegisteredEvent`](#social_contracts_mydata_MyDataRegisteredEvent)
-  [Struct `MyDataUnregisteredEvent`](#social_contracts_mydata_MyDataUnregisteredEvent)
-  [Struct `MyDataConfigUpdatedEvent`](#social_contracts_mydata_MyDataConfigUpdatedEvent)
-  [Constants](#@Constants_0)
-  [Function `create_mydata_admin_cap`](#social_contracts_mydata_create_mydata_admin_cap)
-  [Function `update_mydata_config`](#social_contracts_mydata_update_mydata_config)
-  [Function `marketplace_enabled`](#social_contracts_mydata_marketplace_enabled)
-  [Function `share_mydata_system_objects`](#social_contracts_mydata_share_mydata_system_objects)
-  [Function `bootstrap_init`](#social_contracts_mydata_bootstrap_init)
-  [Function `create_mydata_pool_admin_cap`](#social_contracts_mydata_create_mydata_pool_admin_cap)
-  [Function `gen_pool_id`](#social_contracts_mydata_gen_pool_id)
-  [Function `create_broad_pool`](#social_contracts_mydata_create_broad_pool)
-  [Function `create_sub_pool`](#social_contracts_mydata_create_sub_pool)
-  [Function `assign_mydata_to_sub_pools`](#social_contracts_mydata_assign_mydata_to_sub_pools)
-  [Function `remove_mydata_from_sub_pool`](#social_contracts_mydata_remove_mydata_from_sub_pool)
-  [Function `gen_snapshot_id`](#social_contracts_mydata_gen_snapshot_id)
-  [Function `record_snapshot_anchor`](#social_contracts_mydata_record_snapshot_anchor)
-  [Function `get_snapshot_anchor`](#social_contracts_mydata_get_snapshot_anchor)
-  [Function `publish_merkle_root`](#social_contracts_mydata_publish_merkle_root)
-  [Function `claim`](#social_contracts_mydata_claim)
-  [Function `deposit`](#social_contracts_mydata_deposit)
-  [Function `record_distribution`](#social_contracts_mydata_record_distribution)
-  [Function `get_broad_pool`](#social_contracts_mydata_get_broad_pool)
-  [Function `get_sub_pool`](#social_contracts_mydata_get_sub_pool)
-  [Function `get_mydata_sub_pools`](#social_contracts_mydata_get_mydata_sub_pools)
-  [Function `get_distribution_round`](#social_contracts_mydata_get_distribution_round)
-  [Function `broad_pool_id`](#social_contracts_mydata_broad_pool_id)
-  [Function `sub_pool_id`](#social_contracts_mydata_sub_pool_id)
-  [Function `create`](#social_contracts_mydata_create)
-  [Function `create_and_share`](#social_contracts_mydata_create_and_share)
-  [Function `purchase_one_time`](#social_contracts_mydata_purchase_one_time)
-  [Function `purchase_subscription`](#social_contracts_mydata_purchase_subscription)
-  [Function `update_pricing`](#social_contracts_mydata_update_pricing)
-  [Function `update_content`](#social_contracts_mydata_update_content)
-  [Function `assign_mydata_to_pools`](#social_contracts_mydata_assign_mydata_to_pools)
-  [Function `remove_mydata_from_sub_pools`](#social_contracts_mydata_remove_mydata_from_sub_pools)
-  [Function `has_access`](#social_contracts_mydata_has_access)
-  [Function `encryption_id_matches`](#social_contracts_mydata_encryption_id_matches)
-  [Function `mydata_approve`](#social_contracts_mydata_mydata_approve)
-  [Function `bytes_equal_u8`](#social_contracts_mydata_bytes_equal_u8)
-  [Function `grant_access`](#social_contracts_mydata_grant_access)
-  [Function `revoke_access`](#social_contracts_mydata_revoke_access)
-  [Function `owner`](#social_contracts_mydata_owner)
-  [Function `object_address`](#social_contracts_mydata_object_address)
-  [Function `listing_id`](#social_contracts_mydata_listing_id)
-  [Function `encryption_identity`](#social_contracts_mydata_encryption_identity)
-  [Function `media_type`](#social_contracts_mydata_media_type)
-  [Function `tags`](#social_contracts_mydata_tags)
-  [Function `platform_id`](#social_contracts_mydata_platform_id)
-  [Function `one_time_price`](#social_contracts_mydata_one_time_price)
-  [Function `subscription_price`](#social_contracts_mydata_subscription_price)
-  [Function `subscription_duration_days`](#social_contracts_mydata_subscription_duration_days)
-  [Function `created_at`](#social_contracts_mydata_created_at)
-  [Function `last_updated`](#social_contracts_mydata_last_updated)
-  [Function `timestamp_start`](#social_contracts_mydata_timestamp_start)
-  [Function `timestamp_end`](#social_contracts_mydata_timestamp_end)
-  [Function `geographic_region`](#social_contracts_mydata_geographic_region)
-  [Function `data_quality`](#social_contracts_mydata_data_quality)
-  [Function `sample_size`](#social_contracts_mydata_sample_size)
-  [Function `collection_method`](#social_contracts_mydata_collection_method)
-  [Function `is_updating`](#social_contracts_mydata_is_updating)
-  [Function `update_frequency`](#social_contracts_mydata_update_frequency)
-  [Function `purchaser_count`](#social_contracts_mydata_purchaser_count)
-  [Function `subscriber_count`](#social_contracts_mydata_subscriber_count)
-  [Function `is_one_time_for_sale`](#social_contracts_mydata_is_one_time_for_sale)
-  [Function `is_subscription_available`](#social_contracts_mydata_is_subscription_available)
-  [Function `has_active_subscription`](#social_contracts_mydata_has_active_subscription)
-  [Function `get_subscription_expiry`](#social_contracts_mydata_get_subscription_expiry)
-  [Function `get_revenue_potential`](#social_contracts_mydata_get_revenue_potential)
-  [Function `has_any_sales`](#social_contracts_mydata_has_any_sales)
-  [Function `registry_get_owner`](#social_contracts_mydata_registry_get_owner)
-  [Function `is_registered`](#social_contracts_mydata_is_registered)
-  [Function `register_in_registry`](#social_contracts_mydata_register_in_registry)
-  [Function `unregister_from_registry`](#social_contracts_mydata_unregister_from_registry)
-  [Function `version`](#social_contracts_mydata_version)
-  [Function `borrow_version_mut`](#social_contracts_mydata_borrow_version_mut)
-  [Function `registry_version`](#social_contracts_mydata_registry_version)
-  [Function `borrow_registry_version_mut`](#social_contracts_mydata_borrow_registry_version_mut)
-  [Function `migrate_mydata`](#social_contracts_mydata_migrate_mydata)
-  [Function `migrate_registry`](#social_contracts_mydata_migrate_registry)
-  [Function `migrate_config`](#social_contracts_mydata_migrate_config)


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
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/myso.md#myso_myso">myso::myso</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/package.md#myso_package">myso::package</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/permissioned_group.md#myso_permissioned_group">myso::permissioned_group</a>;
<b>use</b> <a href="../myso/permissions_table.md#myso_permissions_table">myso::permissions_table</a>;
<b>use</b> <a href="../myso/protocol_config.md#myso_protocol_config">myso::protocol_config</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/unpause_cap.md#myso_unpause_cap">myso::unpause_cap</a>;
<b>use</b> <a href="../myso/url.md#myso_url">myso::url</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
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



<a name="social_contracts_mydata_MyData"></a>

## Struct `MyData`

Universal MyData for encrypted data monetization


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a> <b>has</b> key
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Content metadata; may include scheme prefix for client decrypt (e.g. <code>aes_gcm:image</code> vs plain <code>image</code>).
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64</code>
</dt>
<dd>
 Time and context
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>encrypted_data: vector&lt;u8&gt;</code>
</dt>
<dd>
 Opaque ciphertext (BF-HMAC <code>EncryptedObject</code> and/or app-defined encoding).
</dd>
<dt>
<code>encryption_id: vector&lt;u8&gt;</code>
</dt>
<dd>
 Encryption identity bytes: must match <code>id</code> inside the MyData ciphertext and in <code><a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve">mydata_approve</a></code>.
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 Pricing options - user controlled
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>purchasers: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>
 Access tracking
</dd>
<dt>
<code>subscribers: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Extended metadata for data discovery
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
 Version for future upgrades
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataAdminCap"></a>

## Struct `MyDataAdminCap`

Admin capability for MyData system management


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataAdminCap">MyDataAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_mydata_MyDataConfig"></a>

## Struct `MyDataConfig`

Global configuration for MyData system


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a> <b>has</b> key
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>max_tags: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_subscription_days: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_free_access_grants: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_encryption_id_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataRegistry"></a>

## Struct `MyDataRegistry`

Registry for tracking MyData ownership


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a> <b>has</b> key
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
<code>ip_to_owner: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_BroadPool"></a>

## Struct `BroadPool`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPool">BroadPool</a> <b>has</b> <b>copy</b>, drop, store
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_SubPool"></a>

## Struct `SubPool`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_SubPool">SubPool</a> <b>has</b> <b>copy</b>, drop, store
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataPoolRegistry"></a>

## Struct `MyDataPoolRegistry`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a> <b>has</b> key
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
<code>broad_pools: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPool">social_contracts::mydata::BroadPool</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>sub_pools: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_SubPool">social_contracts::mydata::SubPool</a>&gt;</code>
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataPoolAdminCap"></a>

## Struct `MyDataPoolAdminCap`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_mydata_BroadPoolCreatedEvent"></a>

## Struct `BroadPoolCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPoolCreatedEvent">BroadPoolCreatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_SubPoolCreatedEvent"></a>

## Struct `SubPoolCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_SubPoolCreatedEvent">SubPoolCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataAssignedToSubPoolEvent"></a>

## Struct `MyDataAssignedToSubPoolEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataAssignedToSubPoolEvent">MyDataAssignedToSubPoolEvent</a> <b>has</b> <b>copy</b>, drop
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

<a name="social_contracts_mydata_QuerySnapshotAnchor"></a>

## Struct `QuerySnapshotAnchor`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_QuerySnapshotAnchor">QuerySnapshotAnchor</a> <b>has</b> <b>copy</b>, drop, store
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: u64</code>
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

<a name="social_contracts_mydata_SnapshotAnchorRegistry"></a>

## Struct `SnapshotAnchorRegistry`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a> <b>has</b> key
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
<code>anchors: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_QuerySnapshotAnchor">social_contracts::mydata::QuerySnapshotAnchor</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>next_snapshot_nonce: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_SnapshotAnchorRecordedEvent"></a>

## Struct `SnapshotAnchorRecordedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRecordedEvent">SnapshotAnchorRecordedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: u64</code>
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

<a name="social_contracts_mydata_DistributionRecordedEvent"></a>

## Struct `DistributionRecordedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRecordedEvent">DistributionRecordedEvent</a> <b>has</b> <b>copy</b>, drop
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

<a name="social_contracts_mydata_MyDataClaimVault"></a>

## Struct `MyDataClaimVault`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a> <b>has</b> key
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
<code>snapshot_escrow: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, bool&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MerkleRootPublishedEvent"></a>

## Struct `MerkleRootPublishedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MerkleRootPublishedEvent">MerkleRootPublishedEvent</a> <b>has</b> <b>copy</b>, drop
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

<a name="social_contracts_mydata_ClaimExecutedEvent"></a>

## Struct `ClaimExecutedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ClaimExecutedEvent">ClaimExecutedEvent</a> <b>has</b> <b>copy</b>, drop
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

<a name="social_contracts_mydata_DistributionRound"></a>

## Struct `DistributionRound`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRound">DistributionRound</a> <b>has</b> <b>copy</b>, drop, store
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

<a name="social_contracts_mydata_DistributionRegistry"></a>

## Struct `DistributionRegistry`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">DistributionRegistry</a> <b>has</b> key
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
<code>rounds: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRound">social_contracts::mydata::DistributionRound</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataCreatedEvent"></a>

## Struct `MyDataCreatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataCreatedEvent">MyDataCreatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_PurchaseEvent"></a>

## Struct `PurchaseEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_PurchaseEvent">PurchaseEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>buyer: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>price: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>purchase_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>sub_agent_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>organization_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_AccessGrantedEvent"></a>

## Struct `AccessGrantedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_AccessGrantedEvent">AccessGrantedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>access_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>granted_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_AccessRevokedEvent"></a>

## Struct `AccessRevokedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_AccessRevokedEvent">AccessRevokedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>user: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>access_type: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>revoked_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataRegisteredEvent"></a>

## Struct `MyDataRegisteredEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegisteredEvent">MyDataRegisteredEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>registered_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataUnregisteredEvent"></a>

## Struct `MyDataUnregisteredEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataUnregisteredEvent">MyDataUnregisteredEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>unregistered_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_MyDataConfigUpdatedEvent"></a>

## Struct `MyDataConfigUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfigUpdatedEvent">MyDataConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>updated_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>max_tags: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_subscription_days: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_free_access_grants: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_encryption_id_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_mydata_DEFAULT_MARKETPLACE_ENABLED"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MARKETPLACE_ENABLED">DEFAULT_MARKETPLACE_ENABLED</a>: bool = <b>false</b>;
</code></pre>



<a name="social_contracts_mydata_EUnauthorized"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>: u64 = 1;
</code></pre>



<a name="social_contracts_mydata_ENotForSale"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>: u64 = 2;
</code></pre>



<a name="social_contracts_mydata_EPriceMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPriceMismatch">EPriceMismatch</a>: u64 = 3;
</code></pre>



<a name="social_contracts_mydata_ESelfPurchase"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>: u64 = 4;
</code></pre>



<a name="social_contracts_mydata_EAlreadyPurchased"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EAlreadyPurchased">EAlreadyPurchased</a>: u64 = 5;
</code></pre>



<a name="social_contracts_mydata_EActiveSubscription"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EActiveSubscription">EActiveSubscription</a>: u64 = 6;
</code></pre>



<a name="social_contracts_mydata_EInvalidInput"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>: u64 = 7;
</code></pre>



<a name="social_contracts_mydata_ESubscriptionExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ESubscriptionExpired">ESubscriptionExpired</a>: u64 = 8;
</code></pre>



<a name="social_contracts_mydata_EOverflow"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>: u64 = 9;
</code></pre>



<a name="social_contracts_mydata_EInvalidTimeRange"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidTimeRange">EInvalidTimeRange</a>: u64 = 10;
</code></pre>



<a name="social_contracts_mydata_EDisabled"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>: u64 = 11;
</code></pre>



<a name="social_contracts_mydata_EPolicyIdMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyIdMismatch">EPolicyIdMismatch</a>: u64 = 12;
</code></pre>



<a name="social_contracts_mydata_EPolicyNotEntitled"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyNotEntitled">EPolicyNotEntitled</a>: u64 = 13;
</code></pre>



<a name="social_contracts_mydata_ENoAccessToRevoke"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENoAccessToRevoke">ENoAccessToRevoke</a>: u64 = 14;
</code></pre>



<a name="social_contracts_mydata_MAX_TAGS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_TAGS">MAX_TAGS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_mydata_MAX_SUBSCRIPTION_DAYS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_SUBSCRIPTION_DAYS">MAX_SUBSCRIPTION_DAYS</a>: u64 = 365;
</code></pre>



<a name="social_contracts_mydata_MILLISECONDS_PER_DAY"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a>: u64 = 86400000;
</code></pre>



<a name="social_contracts_mydata_MAX_FREE_ACCESS_GRANTS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_FREE_ACCESS_GRANTS">MAX_FREE_ACCESS_GRANTS</a>: u64 = 100000;
</code></pre>



<a name="social_contracts_mydata_MAX_U64"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>: u64 = 18446744073709551615;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_MAX_ENCRYPTION_ID_BYTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_ENCRYPTION_ID_BYTES">DEFAULT_MAX_ENCRYPTION_ID_BYTES</a>: u64 = 1024;
</code></pre>



<a name="social_contracts_mydata_EPqInvalidInput"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>: u64 = 1;
</code></pre>



<a name="social_contracts_mydata_EPqPoolNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqPoolNotFound">EPqPoolNotFound</a>: u64 = 2;
</code></pre>



<a name="social_contracts_mydata_EPqSubPoolNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSubPoolNotFound">EPqSubPoolNotFound</a>: u64 = 3;
</code></pre>



<a name="social_contracts_mydata_EPqInvalidProof"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidProof">EPqInvalidProof</a>: u64 = 4;
</code></pre>



<a name="social_contracts_mydata_EPqAlreadyClaimed"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqAlreadyClaimed">EPqAlreadyClaimed</a>: u64 = 5;
</code></pre>



<a name="social_contracts_mydata_EPqMerkleRootNotPublished"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqMerkleRootNotPublished">EPqMerkleRootNotPublished</a>: u64 = 6;
</code></pre>



<a name="social_contracts_mydata_EPqInsufficientPayment"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInsufficientPayment">EPqInsufficientPayment</a>: u64 = 7;
</code></pre>



<a name="social_contracts_mydata_EPqAnchorNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqAnchorNotFound">EPqAnchorNotFound</a>: u64 = 8;
</code></pre>



<a name="social_contracts_mydata_EPqEscrowExceeded"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqEscrowExceeded">EPqEscrowExceeded</a>: u64 = 9;
</code></pre>



<a name="social_contracts_mydata_EPqSnapshotEscrowMissing"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSnapshotEscrowMissing">EPqSnapshotEscrowMissing</a>: u64 = 10;
</code></pre>



<a name="social_contracts_mydata_create_mydata_admin_cap"></a>

## Function `create_mydata_admin_cap`

Create a MyDataAdminCap for bootstrap (package visibility only)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_mydata_admin_cap">create_mydata_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataAdminCap">social_contracts::mydata::MyDataAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_mydata_admin_cap">create_mydata_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataAdminCap">MyDataAdminCap</a> {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataAdminCap">MyDataAdminCap</a> {
        id: object::new(ctx)
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_update_mydata_config"></a>

## Function `update_mydata_config`

Update MyData configuration (admin only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_mydata_config">update_mydata_config</a>(_: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataAdminCap">social_contracts::mydata::MyDataAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>: bool, max_tags: u64, max_subscription_days: u64, max_free_access_grants: u64, max_encryption_id_bytes: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_mydata_config">update_mydata_config</a>(
    _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataAdminCap">MyDataAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>: bool,
    max_tags: u64,
    max_subscription_days: u64,
    max_free_access_grants: u64,
    max_encryption_id_bytes: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Validate parameters
    <b>assert</b>!(max_subscription_days &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_tags &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_free_access_grants &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_encryption_id_bytes &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a> = <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>;
    config.max_tags = max_tags;
    config.max_subscription_days = max_subscription_days;
    config.max_free_access_grants = max_free_access_grants;
    config.max_encryption_id_bytes = max_encryption_id_bytes;
    // Emit config updated event
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfigUpdatedEvent">MyDataConfigUpdatedEvent</a> {
        updated_by: tx_context::sender(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>,
        max_tags,
        max_subscription_days,
        max_free_access_grants,
        max_encryption_id_bytes,
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_marketplace_enabled"></a>

## Function `marketplace_enabled`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>): bool {
    config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_share_mydata_system_objects"></a>

## Function `share_mydata_system_objects`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_share_mydata_system_objects">share_mydata_system_objects</a>(clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_share_mydata_system_objects">share_mydata_system_objects</a>(clock: &Clock, ctx: &<b>mut</b> TxContext, <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>: bool) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>let</b> ver = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>let</b> config = <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a> {
        id: object::new(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>,
        max_tags: <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_TAGS">MAX_TAGS</a>,
        max_subscription_days: <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_SUBSCRIPTION_DAYS">MAX_SUBSCRIPTION_DAYS</a>,
        max_free_access_grants: <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_FREE_ACCESS_GRANTS">MAX_FREE_ACCESS_GRANTS</a>,
        max_encryption_id_bytes: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_ENCRYPTION_ID_BYTES">DEFAULT_MAX_ENCRYPTION_ID_BYTES</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: ver,
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfigUpdatedEvent">MyDataConfigUpdatedEvent</a> {
        updated_by: sender,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>,
        max_tags: <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_TAGS">MAX_TAGS</a>,
        max_subscription_days: <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_SUBSCRIPTION_DAYS">MAX_SUBSCRIPTION_DAYS</a>,
        max_free_access_grants: <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_FREE_ACCESS_GRANTS">MAX_FREE_ACCESS_GRANTS</a>,
        max_encryption_id_bytes: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_ENCRYPTION_ID_BYTES">DEFAULT_MAX_ENCRYPTION_ID_BYTES</a>,
        timestamp: clock::timestamp_ms(clock),
    });
    transfer::share_object(config);
    transfer::share_object(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a> {
        id: object::new(ctx),
        ip_to_owner: table::new(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: ver,
    });
    transfer::share_object(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a> {
        id: object::new(ctx),
        broad_pools: table::new(ctx),
        sub_pools: table::new(ctx),
        broad_to_sub: table::new(ctx),
        mydata_to_sub_pools: table::new(ctx),
        next_broad_pool_nonce: 0,
        next_sub_pool_nonce: 0,
        last_created_broad_pool_id: option::none(),
        last_created_sub_pool_id: option::none(),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: ver,
    });
    transfer::share_object(<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a> {
        id: object::new(ctx),
        anchors: table::new(ctx),
        next_snapshot_nonce: 0,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: ver,
    });
    transfer::share_object(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a> {
        id: object::new(ctx),
        balance: balance::zero(),
        merkle_roots: table::new(ctx),
        snapshot_escrow: table::new(ctx),
        claimed: table::new(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: ver,
    });
    transfer::share_object(<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">DistributionRegistry</a> {
        id: object::new(ctx),
        rounds: table::new(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: ver,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap: shared config, ownership registry, and query-marketplace objects (pools, anchors, vault).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_bootstrap_init">bootstrap_init</a>(clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_bootstrap_init">bootstrap_init</a>(clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_share_mydata_system_objects">share_mydata_system_objects</a>(clock, ctx, <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MARKETPLACE_ENABLED">DEFAULT_MARKETPLACE_ENABLED</a>);
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_mydata_pool_admin_cap"></a>

## Function `create_mydata_pool_admin_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_mydata_pool_admin_cap">create_mydata_pool_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_mydata_pool_admin_cap">create_mydata_pool_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a> {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="social_contracts_mydata_gen_pool_id"></a>

## Function `gen_pool_id`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_gen_pool_id">gen_pool_id</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, nonce: u64): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_gen_pool_id">gen_pool_id</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>, nonce: u64): ID {
    <b>let</b> <b>mut</b> data = bcs::to_bytes(&object::uid_to_address(&registry.id));
    vector::append(&<b>mut</b> data, bcs::to_bytes(&nonce));
    object::id_from_bytes(hash::blake2b256(&data))
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_broad_pool"></a>

## Function `create_broad_pool`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_broad_pool">create_broad_pool</a>(_: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_broad_pool">create_broad_pool</a>(
    _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    name: String,
    description: String,
    clock: &Clock,
) {
    <b>let</b> nonce = registry.next_broad_pool_nonce;
    registry.next_broad_pool_nonce = nonce + 1;
    <b>let</b> pool_id = <a href="../social_contracts/mydata.md#social_contracts_mydata_gen_pool_id">gen_pool_id</a>(registry, nonce);
    <b>let</b> broad_pool = <a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPool">BroadPool</a> {
        id: pool_id,
        name,
        description,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: clock::timestamp_ms(clock),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>,
    };
    table::add(&<b>mut</b> registry.broad_pools, pool_id, broad_pool);
    table::add(&<b>mut</b> registry.broad_to_sub, pool_id, vector::empty());
    registry.last_created_broad_pool_id = option::some(pool_id);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPoolCreatedEvent">BroadPoolCreatedEvent</a> {
        pool_id,
        name: broad_pool.name,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: broad_pool.<a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_sub_pool"></a>

## Function `create_sub_pool`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_sub_pool">create_sub_pool</a>(_: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, schema_metadata: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_sub_pool">create_sub_pool</a>(
    _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>: ID,
    name: String,
    description: String,
    schema_metadata: Option&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(table::contains(&registry.broad_pools, <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqPoolNotFound">EPqPoolNotFound</a>);
    <b>let</b> nonce = registry.next_sub_pool_nonce;
    registry.next_sub_pool_nonce = nonce + 1;
    <b>let</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a> = <a href="../social_contracts/mydata.md#social_contracts_mydata_gen_pool_id">gen_pool_id</a>(registry, 0x100000000 | nonce);
    <b>let</b> sub_pool = <a href="../social_contracts/mydata.md#social_contracts_mydata_SubPool">SubPool</a> {
        id: <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>,
        name,
        description,
        schema_metadata,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: clock::timestamp_ms(clock),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>,
    };
    table::add(&<b>mut</b> registry.sub_pools, <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>, sub_pool);
    registry.last_created_sub_pool_id = option::some(<a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>);
    <b>let</b> sub_ids = table::borrow_mut(&<b>mut</b> registry.broad_to_sub, <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>);
    vector::push_back(sub_ids, <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_SubPoolCreatedEvent">SubPoolCreatedEvent</a> {
        <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>,
        name: sub_pool.name,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: sub_pool.<a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_assign_mydata_to_sub_pools"></a>

## Function `assign_mydata_to_sub_pools`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_assign_mydata_to_sub_pools">assign_mydata_to_sub_pools</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, ip_id: <b>address</b>, sub_pool_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_assign_mydata_to_sub_pools">assign_mydata_to_sub_pools</a>(
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
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
        <b>assert</b>!(table::contains(&registry.sub_pools, sub_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSubPoolNotFound">EPqSubPoolNotFound</a>);
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
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataAssignedToSubPoolEvent">MyDataAssignedToSubPoolEvent</a> {
        ip_id,
        sub_pool_ids,
        assigned_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_remove_mydata_from_sub_pool"></a>

## Function `remove_mydata_from_sub_pool`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_remove_mydata_from_sub_pool">remove_mydata_from_sub_pool</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, ip_id: <b>address</b>, <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_remove_mydata_from_sub_pool">remove_mydata_from_sub_pool</a>(
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    ip_id: <b>address</b>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>: ID,
) {
    <b>assert</b>!(table::contains(&registry.mydata_to_sub_pools, ip_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>let</b> sub_ids = table::borrow_mut(&<b>mut</b> registry.mydata_to_sub_pools, ip_id);
    <b>let</b> (found, idx) = vector::index_of(sub_ids, &<a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>);
    <b>assert</b>!(found, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    vector::remove(sub_ids, idx);
}
</code></pre>



</details>

<a name="social_contracts_mydata_gen_snapshot_id"></a>

## Function `gen_snapshot_id`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_gen_snapshot_id">gen_snapshot_id</a>(registry_id: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, nonce: u64): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_gen_snapshot_id">gen_snapshot_id</a>(registry_id: &UID, nonce: u64): ID {
    <b>let</b> <b>mut</b> data = bcs::to_bytes(&object::uid_to_address(registry_id));
    vector::append(&<b>mut</b> data, bcs::to_bytes(&nonce));
    object::id_from_bytes(hash::blake2b256(&data))
}
</code></pre>



</details>

<a name="social_contracts_mydata_record_snapshot_anchor"></a>

## Function `record_snapshot_anchor`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_record_snapshot_anchor">record_snapshot_anchor</a>(anchor_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">social_contracts::mydata::SnapshotAnchorRegistry</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, pool_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, source_pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, source_sub_pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, manifest_hash: vector&lt;u8&gt;, payment_reference: vector&lt;u8&gt;, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_record_snapshot_anchor">record_snapshot_anchor</a>(
    anchor_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a>,
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    pool_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    source_pool_id: ID,
    source_sub_pool_id: ID,
    manifest_hash: vector&lt;u8&gt;,
    payment_reference: vector&lt;u8&gt;,
    payment: Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(table::contains(&pool_registry.broad_pools, source_pool_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqPoolNotFound">EPqPoolNotFound</a>);
    <b>assert</b>!(table::contains(&pool_registry.sub_pools, source_sub_pool_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSubPoolNotFound">EPqSubPoolNotFound</a>);
    <b>let</b> price_paid = coin::value(&payment);
    <b>assert</b>!(price_paid &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInsufficientPayment">EPqInsufficientPayment</a>);
    <b>let</b> nonce = anchor_registry.next_snapshot_nonce;
    anchor_registry.next_snapshot_nonce = nonce + 1;
    <b>let</b> snapshot_id = <a href="../social_contracts/mydata.md#social_contracts_mydata_gen_snapshot_id">gen_snapshot_id</a>(&anchor_registry.id, nonce);
    <b>let</b> buyer = tx_context::sender(ctx);
    <b>let</b> anchor = <a href="../social_contracts/mydata.md#social_contracts_mydata_QuerySnapshotAnchor">QuerySnapshotAnchor</a> {
        snapshot_id,
        buyer_address: buyer,
        source_pool_id,
        source_sub_pool_id,
        price_paid,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: clock::timestamp_ms(clock),
        snapshot_manifest_hash: manifest_hash,
        payment_reference,
    };
    <b>let</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a> = anchor.<a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>;
    <b>let</b> snapshot_manifest_hash_ev = <b>copy</b> anchor.snapshot_manifest_hash;
    <b>let</b> payment_reference_ev = <b>copy</b> anchor.payment_reference;
    table::add(&<b>mut</b> anchor_registry.anchors, snapshot_id, anchor);
    balance::join(&<b>mut</b> vault.balance, coin::into_balance(payment));
    <b>assert</b>!(!table::contains(&vault.snapshot_escrow, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    table::add(&<b>mut</b> vault.snapshot_escrow, snapshot_id, price_paid);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRecordedEvent">SnapshotAnchorRecordedEvent</a> {
        snapshot_id,
        buyer_address: buyer,
        price_paid,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>,
        snapshot_manifest_hash: snapshot_manifest_hash_ev,
        payment_reference: payment_reference_ev,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_get_snapshot_anchor"></a>

## Function `get_snapshot_anchor`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_snapshot_anchor">get_snapshot_anchor</a>(anchor_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">social_contracts::mydata::SnapshotAnchorRegistry</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/mydata.md#social_contracts_mydata_QuerySnapshotAnchor">social_contracts::mydata::QuerySnapshotAnchor</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_snapshot_anchor">get_snapshot_anchor</a>(
    anchor_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a>,
    snapshot_id: ID,
): Option&lt;<a href="../social_contracts/mydata.md#social_contracts_mydata_QuerySnapshotAnchor">QuerySnapshotAnchor</a>&gt; {
    <b>if</b> (table::contains(&anchor_registry.anchors, snapshot_id)) {
        option::some(*table::borrow(&anchor_registry.anchors, snapshot_id))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_publish_merkle_root"></a>

## Function `publish_merkle_root`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_publish_merkle_root">publish_merkle_root</a>(_: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, anchor_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">social_contracts::mydata::SnapshotAnchorRegistry</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, root_hash: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_publish_merkle_root">publish_merkle_root</a>(
    _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    anchor_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a>,
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    snapshot_id: ID,
    root_hash: vector&lt;u8&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(table::contains(&anchor_registry.anchors, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqAnchorNotFound">EPqAnchorNotFound</a>);
    <b>assert</b>!(vector::length(&root_hash) == 32, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>assert</b>!(!table::contains(&vault.merkle_roots, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    table::add(&<b>mut</b> vault.merkle_roots, snapshot_id, root_hash);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MerkleRootPublishedEvent">MerkleRootPublishedEvent</a> {
        snapshot_id,
        root_hash,
        published_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_claim"></a>

## Function `claim`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_claim">claim</a>(vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, amount: u64, leaf_index: u64, proof: vector&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_claim">claim</a>(
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    snapshot_id: ID,
    amount: u64,
    leaf_index: u64,
    proof: vector&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(table::contains(&vault.merkle_roots, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqMerkleRootNotPublished">EPqMerkleRootNotPublished</a>);
    <b>assert</b>!(table::contains(&vault.snapshot_escrow, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSnapshotEscrowMissing">EPqSnapshotEscrowMissing</a>);
    <b>assert</b>!(*table::borrow(&vault.snapshot_escrow, snapshot_id) &gt;= amount, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqEscrowExceeded">EPqEscrowExceeded</a>);
    <b>let</b> claimant = tx_context::sender(ctx);
    <b>let</b> leaf = merkle::leaf_hash(claimant, amount, object::id_to_bytes(&snapshot_id));
    <b>let</b> root = *table::borrow(&vault.merkle_roots, snapshot_id);
    <b>assert</b>!(merkle::verify_proof(leaf, &proof, leaf_index, root), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidProof">EPqInvalidProof</a>);
    <b>if</b> (table::contains(&vault.claimed, snapshot_id)) {
        <b>assert</b>!(!table::contains(table::borrow(&vault.claimed, snapshot_id), claimant), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqAlreadyClaimed">EPqAlreadyClaimed</a>);
    };
    <b>let</b> escrow_remaining = table::borrow_mut(&<b>mut</b> vault.snapshot_escrow, snapshot_id);
    *escrow_remaining = *escrow_remaining - amount;
    <b>if</b> (!table::contains(&vault.claimed, snapshot_id)) {
        table::add(&<b>mut</b> vault.claimed, snapshot_id, table::new(ctx));
    };
    <b>let</b> claimed_table = table::borrow_mut(&<b>mut</b> vault.claimed, snapshot_id);
    table::add(claimed_table, claimant, <b>true</b>);
    <b>let</b> payout = balance::split(&<b>mut</b> vault.balance, amount);
    transfer::public_transfer(coin::from_balance(payout, ctx), claimant);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_ClaimExecutedEvent">ClaimExecutedEvent</a> {
        snapshot_id,
        claimant,
        amount,
        claimed_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_deposit"></a>

## Function `deposit`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_deposit">deposit</a>(_: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, _snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_deposit">deposit</a>(
    _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    _snapshot_id: ID,
    payment: Coin&lt;MYSO&gt;,
) {
    balance::join(&<b>mut</b> vault.balance, coin::into_balance(payment));
}
</code></pre>



</details>

<a name="social_contracts_mydata_record_distribution"></a>

## Function `record_distribution`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_record_distribution">record_distribution</a>(_: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, dist_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">social_contracts::mydata::DistributionRegistry</a>, vault: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, total_amount: u64, contributor_count: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_record_distribution">record_distribution</a>(
    _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    dist_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">DistributionRegistry</a>,
    vault: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    snapshot_id: ID,
    total_amount: u64,
    contributor_count: u64,
    clock: &Clock,
) {
    <b>assert</b>!(table::contains(&vault.merkle_roots, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqMerkleRootNotPublished">EPqMerkleRootNotPublished</a>);
    <b>let</b> root = *table::borrow(&vault.merkle_roots, snapshot_id);
    <b>let</b> published_at = clock::timestamp_ms(clock);
    <b>let</b> round = <a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRound">DistributionRound</a> {
        snapshot_id,
        total_amount,
        contributor_count,
        merkle_root: <b>copy</b> root,
        published_at,
    };
    table::add(&<b>mut</b> dist_registry.rounds, snapshot_id, round);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRecordedEvent">DistributionRecordedEvent</a> {
        snapshot_id,
        total_amount,
        contributor_count,
        merkle_root: root,
        published_at,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_get_broad_pool"></a>

## Function `get_broad_pool`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_broad_pool">get_broad_pool</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPool">social_contracts::mydata::BroadPool</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_broad_pool">get_broad_pool</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>, pool_id: ID): Option&lt;<a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPool">BroadPool</a>&gt; {
    <b>if</b> (table::contains(&registry.broad_pools, pool_id)) {
        option::some(*table::borrow(&registry.broad_pools, pool_id))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_get_sub_pool"></a>

## Function `get_sub_pool`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_sub_pool">get_sub_pool</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/mydata.md#social_contracts_mydata_SubPool">social_contracts::mydata::SubPool</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_sub_pool">get_sub_pool</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>: ID): Option&lt;<a href="../social_contracts/mydata.md#social_contracts_mydata_SubPool">SubPool</a>&gt; {
    <b>if</b> (table::contains(&registry.sub_pools, <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>)) {
        option::some(*table::borrow(&registry.sub_pools, <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_get_mydata_sub_pools"></a>

## Function `get_mydata_sub_pools`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_mydata_sub_pools">get_mydata_sub_pools</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, ip_id: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_mydata_sub_pools">get_mydata_sub_pools</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>, ip_id: <b>address</b>): Option&lt;vector&lt;ID&gt;&gt; {
    <b>if</b> (table::contains(&registry.mydata_to_sub_pools, ip_id)) {
        option::some(*table::borrow(&registry.mydata_to_sub_pools, ip_id))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_get_distribution_round"></a>

## Function `get_distribution_round`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_distribution_round">get_distribution_round</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">social_contracts::mydata::DistributionRegistry</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRound">social_contracts::mydata::DistributionRound</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_distribution_round">get_distribution_round</a>(
    registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">DistributionRegistry</a>,
    snapshot_id: ID,
): Option&lt;<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRound">DistributionRound</a>&gt; {
    <b>if</b> (table::contains(&registry.rounds, snapshot_id)) {
        option::some(*table::borrow(&registry.rounds, snapshot_id))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_broad_pool_id"></a>

## Function `broad_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>(pool: &<a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPool">social_contracts::mydata::BroadPool</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>(pool: &<a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPool">BroadPool</a>): ID { pool.id }
</code></pre>



</details>

<a name="social_contracts_mydata_sub_pool_id"></a>

## Function `sub_pool_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>(pool: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SubPool">social_contracts::mydata::SubPool</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>(pool: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SubPool">SubPool</a>): ID { pool.id }
</code></pre>



</details>

<a name="social_contracts_mydata_create"></a>

## Function `create`

Create new MyData data with proper MyData encryption


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create">create</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, encrypted_data: vector&lt;u8&gt;, encryption_id: vector&lt;u8&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool, <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create">create</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: String,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: Option&lt;<b>address</b>&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: Option&lt;u64&gt;,
    encrypted_data: vector&lt;u8&gt;,  // Pre-encrypted data from client
    encryption_id: vector&lt;u8&gt;,   // <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a> encryption ID
    <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a> {
    // Input validation
    <b>assert</b>!(vector::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>) &lt;= config.max_tags, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(!vector::is_empty(&encryption_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(vector::length(&encryption_id) &lt;= config.max_encryption_id_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    // Validate prices with overflow protection
    <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>)) {
        <b>let</b> price_val = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>);
        <b>assert</b>!(price_val &gt; 0 && price_val &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    };
    <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>)) {
        <b>let</b> price_val = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>);
        <b>assert</b>!(price_val &gt; 0 && price_val &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    };
    // Validate <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> duration with overflow protection
    <b>let</b> sub_duration = <b>if</b> (<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> == 0) { 30 } <b>else</b> { <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> };
    <b>assert</b>!(sub_duration &lt;= config.max_subscription_days, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    // Check <b>for</b> potential overflow in millisecond conversion
    <b>let</b> duration_ms = (sub_duration <b>as</b> u128) * (<a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a> <b>as</b> u128);
    <b>assert</b>!(duration_ms &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
    // Validate time range
    <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>)) {
        <b>let</b> end_time = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>);
        <b>assert</b>!(end_time &gt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidTimeRange">EInvalidTimeRange</a>);
    };
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a> = <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a> {
        id: object::new(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: tx_context::sender(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: current_time,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a>: current_time,
        encrypted_data,
        encryption_id,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: sub_duration,
        purchasers: table::new(ctx),
        subscribers: table::new(ctx),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataCreatedEvent">MyDataCreatedEvent</a> {
        ip_id,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>,
    });
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_and_share"></a>

## Function `create_and_share`

Create and share MyData publicly


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share">create_and_share</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, encrypted_data: vector&lt;u8&gt;, encryption_id: vector&lt;u8&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool, <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share">create_and_share</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: String,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: Option&lt;<b>address</b>&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: Option&lt;u64&gt;,
    encrypted_data: vector&lt;u8&gt;,
    encryption_id: vector&lt;u8&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>);
    <b>let</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a> = <a href="../social_contracts/mydata.md#social_contracts_mydata_create">create</a>(
        config,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>,
        encrypted_data,
        encryption_id,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>,
        clock,
        ctx,
    );
    // Register in the registry
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    table::add(&<b>mut</b> registry.ip_to_owner, ip_id, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    transfer::share_object(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>);
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_one_time"></a>

## Function `purchase_one_time`

Purchase one-time access to MyData data.
Sub-agent buyers must satisfy <code>max_action_spend</code> for <code>price</code> on <code>account</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time">purchase_one_time</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time">purchase_one_time</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    payment: Coin&lt;MYSO&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>);
    // Check <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    // Check <b>if</b> one-time purchase is available
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>);
    <b>let</b> price = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>);
    <b>let</b> <b>mut</b> sub_agent_id = option::none();
    <b>let</b> <b>mut</b> organization_id = option::none();
    <b>if</b> (<a href="../social_contracts/memory.md#social_contracts_memory_is_registered_agent">social_contracts::memory::is_registered_agent</a>(account, buyer)) {
        <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">social_contracts::memory::resolve_actor_with_cap</a>(
            memory_config,
            account,
            0,
            <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
            price,
            clock,
            ctx,
        );
        sub_agent_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">social_contracts::memory::acting_sub_agent_id</a>(&acting);
        organization_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">social_contracts::memory::acting_organization_id</a>(&acting);
    };
    // Check payment amount
    <b>assert</b>!(coin::value(&payment) &gt;= price, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPriceMismatch">EPriceMismatch</a>);
    // Check <b>if</b> buyer already <b>has</b> access
    <b>assert</b>!(!table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, buyer), <a href="../social_contracts/mydata.md#social_contracts_mydata_EAlreadyPurchased">EAlreadyPurchased</a>);
    // Prevent self-purchase
    <b>assert</b>!(buyer != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>);
    <b>let</b> <b>mut</b> payment = payment;
    <b>let</b> to_owner = coin::split(&<b>mut</b> payment, price, ctx);
    transfer::public_transfer(to_owner, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, buyer);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Grant access
    table::add(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, buyer, <b>true</b>);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_PurchaseEvent">PurchaseEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        buyer,
        price,
        purchase_type: string::utf8(b"one_time"),
        timestamp: clock::timestamp_ms(clock),
        sub_agent_id,
        organization_id,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_subscription"></a>

## Function `purchase_subscription`

Purchase subscription access to MyData data.
Sub-agent buyers must satisfy <code>max_action_spend</code> for <code>price</code> on <code>account</code>.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription">purchase_subscription</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription">purchase_subscription</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    payment: Coin&lt;MYSO&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>);
    // Check <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    // Check <b>if</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> is available
    <b>assert</b>!(option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>);
    <b>let</b> price = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>);
    <b>let</b> <b>mut</b> sub_agent_id = option::none();
    <b>let</b> <b>mut</b> organization_id = option::none();
    <b>if</b> (<a href="../social_contracts/memory.md#social_contracts_memory_is_registered_agent">social_contracts::memory::is_registered_agent</a>(account, buyer)) {
        <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">social_contracts::memory::resolve_actor_with_cap</a>(
            memory_config,
            account,
            0,
            <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
            price,
            clock,
            ctx,
        );
        sub_agent_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">social_contracts::memory::acting_sub_agent_id</a>(&acting);
        organization_id = <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">social_contracts::memory::acting_organization_id</a>(&acting);
    };
    // Check payment amount
    <b>assert</b>!(coin::value(&payment) &gt;= price, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPriceMismatch">EPriceMismatch</a>);
    // Prevent self-purchase
    <b>assert</b>!(buyer != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>);
    // Validate <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> duration to prevent overflow
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> &lt;= config.max_subscription_days, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    // Calculate <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> expiry safely with overflow protection
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> duration_ms = (<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> <b>as</b> u128) * (<a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a> <b>as</b> u128);
    <b>let</b> expiry_time = (current_time <b>as</b> u128) + duration_ms;
    // Ensure we don't overflow u64
    <b>assert</b>!(expiry_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
    <b>let</b> expiry_time_u64 = expiry_time <b>as</b> u64;
    <b>let</b> <b>mut</b> payment = payment;
    <b>let</b> to_owner = coin::split(&<b>mut</b> payment, price, ctx);
    transfer::public_transfer(to_owner, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    <b>if</b> (coin::value(&payment) &gt; 0) {
        transfer::public_transfer(payment, buyer);
    } <b>else</b> {
        coin::destroy_zero(payment);
    };
    // Grant/extend <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> access
    <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, buyer)) {
        // Extend existing <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>
        <b>let</b> current_expiry = table::remove(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, buyer);
        <b>let</b> new_expiry = <b>if</b> (current_expiry &gt; current_time) {
            // Add to existing time, but check <b>for</b> overflow
            <b>let</b> extended_time = (current_expiry <b>as</b> u128) + duration_ms;
            <b>assert</b>!(extended_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
            extended_time <b>as</b> u64
        } <b>else</b> {
            expiry_time_u64
        };
        table::add(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, buyer, new_expiry);
    } <b>else</b> {
        // New <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>
        table::add(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, buyer, expiry_time_u64);
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_PurchaseEvent">PurchaseEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        buyer,
        price,
        purchase_type: string::utf8(b"<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>"),
        timestamp: clock::timestamp_ms(clock),
        sub_agent_id,
        organization_id,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_update_pricing"></a>

## Function `update_pricing`

Update pricing (owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_pricing">update_pricing</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, new_one_time_price: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, new_subscription_price: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, new_subscription_duration_days: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_pricing">update_pricing</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    new_one_time_price: Option&lt;u64&gt;,
    new_subscription_price: Option&lt;u64&gt;,
    new_subscription_duration_days: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    // Check <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    // Validate new prices
    <b>if</b> (option::is_some(&new_one_time_price)) {
        <b>let</b> price_val = *option::borrow(&new_one_time_price);
        <b>assert</b>!(price_val &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    };
    <b>if</b> (option::is_some(&new_subscription_price)) {
        <b>let</b> price_val = *option::borrow(&new_subscription_price);
        <b>assert</b>!(price_val &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    };
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a> = new_one_time_price;
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a> = new_subscription_price;
    <b>if</b> (option::is_some(&new_subscription_duration_days)) {
        <b>let</b> duration = *option::borrow(&new_subscription_duration_days);
        <b>if</b> (duration &gt; 0) {
            <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> = duration;
        };
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessGrantedEvent">AccessGrantedEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        user: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        access_type: string::utf8(b"pricing_update"),
        granted_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_update_content"></a>

## Function `update_content`

Update MyData content and metadata (owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_content">update_content</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, new_encrypted_data: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, new_tags: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_content">update_content</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    new_encrypted_data: Option&lt;vector&lt;u8&gt;&gt;,
    new_tags: Option&lt;vector&lt;String&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    // Check <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>if</b> (option::is_some(&new_encrypted_data)) {
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.encrypted_data = *option::borrow(&new_encrypted_data);
    };
    <b>if</b> (option::is_some(&new_tags)) {
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a> = *option::borrow(&new_tags);
    };
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a> = clock::timestamp_ms(clock);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessGrantedEvent">AccessGrantedEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        user: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        access_type: string::utf8(b"content_update"),
        granted_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_assign_mydata_to_pools"></a>

## Function `assign_mydata_to_pools`

Assign MyData to sub-pools (owner only).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_assign_mydata_to_pools">assign_mydata_to_pools</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, pool_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, sub_pool_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_assign_mydata_to_pools">assign_mydata_to_pools</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    pool_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    sub_pool_ids: vector&lt;ID&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_assign_mydata_to_sub_pools">assign_mydata_to_sub_pools</a>(pool_registry, ip_id, sub_pool_ids, clock);
}
</code></pre>



</details>

<a name="social_contracts_mydata_remove_mydata_from_sub_pools"></a>

## Function `remove_mydata_from_sub_pools`

Remove this listing from a sub-pool (owner only).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_remove_mydata_from_sub_pools">remove_mydata_from_sub_pools</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, pool_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_remove_mydata_from_sub_pools">remove_mydata_from_sub_pools</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    pool_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>: ID,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_remove_mydata_from_sub_pool">remove_mydata_from_sub_pool</a>(pool_registry, ip_id, <a href="../social_contracts/mydata.md#social_contracts_mydata_sub_pool_id">sub_pool_id</a>);
}
</code></pre>



</details>

<a name="social_contracts_mydata_has_access"></a>

## Function `has_access`

Check if user has access to MyData data


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_access">has_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, user: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_access">has_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>, user: <b>address</b>, clock: &Clock): bool {
    // Owner always <b>has</b> access
    <b>if</b> (user == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>) <b>return</b> <b>true</b>;
    // Check one-time purchase
    <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, user)) <b>return</b> <b>true</b>;
    // Check active <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>
    <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user)) {
        <b>let</b> expiry = *table::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user);
        <b>let</b> current_time = clock::timestamp_ms(clock);
        <b>return</b> current_time &lt;= expiry
    };
    <b>false</b>
}
</code></pre>



</details>

<a name="social_contracts_mydata_encryption_id_matches"></a>

## Function `encryption_id_matches`

True if <code>candidate</code> matches this listing’s <code>encryption_id</code> (the MyData policy <code>id</code> bytes).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_encryption_id_matches">encryption_id_matches</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, candidate: &vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_encryption_id_matches">encryption_id_matches</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>, candidate: &vector&lt;u8&gt;): bool {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_bytes_equal_u8">bytes_equal_u8</a>(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.encryption_id, candidate)
}
</code></pre>



</details>

<a name="social_contracts_mydata_mydata_approve"></a>

## Function `mydata_approve`

Key-server policy hook for <code>fetch_key</code>: <code>id</code> must match <code>encryption_id</code>, and the sender must have
[<code><a href="../social_contracts/mydata.md#social_contracts_mydata_has_access">has_access</a></code>] (owner, purchaser, or active subscriber), or be a registered sub-agent of the
owner with <code>CAP_MYDATA_READ</code>. Register this package on the key server when using permissioned
mode; <code>EncryptedObject.package_id</code> at encrypt time must match this package.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve">mydata_approve</a>(memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, id: vector&lt;u8&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve">mydata_approve</a>(
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    id: vector&lt;u8&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata_encryption_id_matches">encryption_id_matches</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>, &id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyIdMismatch">EPolicyIdMismatch</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>if</b> (<a href="../social_contracts/mydata.md#social_contracts_mydata_has_access">has_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>, sender, clock)) {
        <b>return</b>
    };
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_owner">social_contracts::memory::owner</a>(account) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyNotEntitled">EPolicyNotEntitled</a>,
    );
    <b>if</b> (!<a href="../social_contracts/memory.md#social_contracts_memory_is_registered_agent">social_contracts::memory::is_registered_agent</a>(account, sender)) {
        <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyNotEntitled">EPolicyNotEntitled</a>
    };
    <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">social_contracts::memory::resolve_actor_with_cap</a>(
        memory_config,
        account,
        <a href="../social_contracts/memory.md#social_contracts_memory_cap_mydata_read">social_contracts::memory::cap_mydata_read</a>(),
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        0,
        clock,
        ctx,
    );
    <b>assert</b>!(
        <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">social_contracts::memory::acting_principal_owner</a>(&acting) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyNotEntitled">EPolicyNotEntitled</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_bytes_equal_u8"></a>

## Function `bytes_equal_u8`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_bytes_equal_u8">bytes_equal_u8</a>(a: &vector&lt;u8&gt;, b: &vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_bytes_equal_u8">bytes_equal_u8</a>(a: &vector&lt;u8&gt;, b: &vector&lt;u8&gt;): bool {
    <b>if</b> (vector::length(a) != vector::length(b)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> len = vector::length(a);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; len) {
        <b>if</b> (*vector::borrow(a, i) != *vector::borrow(b, i)) {
            <b>return</b> <b>false</b>
        };
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>

<a name="social_contracts_mydata_grant_access"></a>

## Function `grant_access`

Grant free access (owner only) - useful for samples or promotions


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_grant_access">grant_access</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, user: <b>address</b>, access_type: u8, subscription_days: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_grant_access">grant_access</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    user: <b>address</b>,
    access_type: u8, // 0 = one-time, 1 = <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>
    subscription_days: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>);
    // Check <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(user != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>); // Owner doesn't need granted access
    // Check max free access grants limit
    <b>let</b> total_grants = table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers) + table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers);
    <b>assert</b>!(total_grants &lt; config.max_free_access_grants, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>if</b> (access_type == 0) {
        // Grant one-time access
        <b>if</b> (!table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, user)) {
            table::add(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, user, <b>true</b>);
        };
    } <b>else</b> {
        // Grant <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a> access
        <b>let</b> duration_days = <b>if</b> (option::is_some(&subscription_days)) {
            <b>let</b> days = *option::borrow(&subscription_days);
            <b>assert</b>!(days &gt; 0 && days &lt;= config.max_subscription_days, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
            days
        } <b>else</b> {
            <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>
        };
        <b>let</b> current_time = clock::timestamp_ms(clock);
        <b>let</b> duration_ms = (duration_days <b>as</b> u128) * (<a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a> <b>as</b> u128);
        <b>let</b> expiry_time = (current_time <b>as</b> u128) + duration_ms;
        // Ensure we don't overflow u64
        <b>assert</b>!(expiry_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
        <b>let</b> expiry_time_u64 = expiry_time <b>as</b> u64;
        <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user)) {
            table::remove(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user);
        };
        table::add(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user, expiry_time_u64);
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessGrantedEvent">AccessGrantedEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        user,
        access_type: <b>if</b> (access_type == 0) { string::utf8(b"one_time") } <b>else</b> { string::utf8(b"<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>") },
        granted_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_revoke_access"></a>

## Function `revoke_access`

Revoke a buyer's access (owner only). Removes the user from <code>purchasers</code> and/or <code>subscribers</code>.
<code>access_type</code>: 0 = one-time, 1 = subscription, 2 = both.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_revoke_access">revoke_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, user: <b>address</b>, access_type: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_revoke_access">revoke_access</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    user: <b>address</b>,
    access_type: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(user != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(access_type &lt;= 2, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> <b>mut</b> revoked_one_time = <b>false</b>;
    <b>let</b> <b>mut</b> revoked_subscription = <b>false</b>;
    <b>if</b> (access_type == 0 || access_type == 2) {
        <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, user)) {
            table::remove(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers, user);
            revoked_one_time = <b>true</b>;
        };
    };
    <b>if</b> (access_type == 1 || access_type == 2) {
        <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user)) {
            table::remove(&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user);
            revoked_subscription = <b>true</b>;
        };
    };
    <b>assert</b>!(revoked_one_time || revoked_subscription, <a href="../social_contracts/mydata.md#social_contracts_mydata_ENoAccessToRevoke">ENoAccessToRevoke</a>);
    <b>let</b> access_type_str = <b>if</b> (revoked_one_time && revoked_subscription) {
        string::utf8(b"all")
    } <b>else</b> <b>if</b> (revoked_one_time) {
        string::utf8(b"one_time")
    } <b>else</b> {
        string::utf8(b"<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>")
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessRevokedEvent">AccessRevokedEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        user,
        access_type: access_type_str,
        revoked_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_owner"></a>

## Function `owner`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): <b>address</b> { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_object_address"></a>

## Function `object_address`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_object_address">object_address</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_object_address">object_address</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): <b>address</b> { object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id) }
</code></pre>



</details>

<a name="social_contracts_mydata_listing_id"></a>

## Function `listing_id`

Listing object address for PTB binding in <code>fetch_key</code> policy transactions.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_listing_id">listing_id</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_listing_id">listing_id</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): <b>address</b> { object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id) }
</code></pre>



</details>

<a name="social_contracts_mydata_encryption_identity"></a>

## Function `encryption_identity`

Encryption identity bytes; must match <code>EncryptedObject.id</code> and the <code>id</code> arg to <code><a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve">mydata_approve</a></code>.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_encryption_identity">encryption_identity</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_encryption_identity">encryption_identity</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): vector&lt;u8&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.encryption_id }
</code></pre>



</details>

<a name="social_contracts_mydata_media_type"></a>

## Function `media_type`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): String { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_tags"></a>

## Function `tags`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): vector&lt;String&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_platform_id"></a>

## Function `platform_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;<b>address</b>&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_one_time_price"></a>

## Function `one_time_price`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_subscription_price"></a>

## Function `subscription_price`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_subscription_duration_days"></a>

## Function `subscription_duration_days`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_created_at"></a>

## Function `created_at`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_last_updated"></a>

## Function `last_updated`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_timestamp_start"></a>

## Function `timestamp_start`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_timestamp_end"></a>

## Function `timestamp_end`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_geographic_region"></a>

## Function `geographic_region`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;String&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_data_quality"></a>

## Function `data_quality`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;String&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_sample_size"></a>

## Function `sample_size`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_collection_method"></a>

## Function `collection_method`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;String&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_is_updating"></a>

## Function `is_updating`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_update_frequency"></a>

## Function `update_frequency`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;String&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a> }
</code></pre>



</details>

<a name="social_contracts_mydata_purchaser_count"></a>

## Function `purchaser_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchaser_count">purchaser_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchaser_count">purchaser_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers) }
</code></pre>



</details>

<a name="social_contracts_mydata_subscriber_count"></a>

## Function `subscriber_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscriber_count">subscriber_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscriber_count">subscriber_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 { table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers) }
</code></pre>



</details>

<a name="social_contracts_mydata_is_one_time_for_sale"></a>

## Function `is_one_time_for_sale`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_one_time_for_sale">is_one_time_for_sale</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_one_time_for_sale">is_one_time_for_sale</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool { option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>) }
</code></pre>



</details>

<a name="social_contracts_mydata_is_subscription_available"></a>

## Function `is_subscription_available`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_subscription_available">is_subscription_available</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_subscription_available">is_subscription_available</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool { option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>) }
</code></pre>



</details>

<a name="social_contracts_mydata_has_active_subscription"></a>

## Function `has_active_subscription`

Check if a user has an active subscription


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_active_subscription">has_active_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, user: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_active_subscription">has_active_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>, user: <b>address</b>, clock: &Clock): bool {
    <b>if</b> (!table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user)) <b>return</b> <b>false</b>;
    <b>let</b> expiry = *table::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    current_time &lt;= expiry
}
</code></pre>



</details>

<a name="social_contracts_mydata_get_subscription_expiry"></a>

## Function `get_subscription_expiry`

Get subscription expiry time for a user


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_subscription_expiry">get_subscription_expiry</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, user: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_subscription_expiry">get_subscription_expiry</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>, user: <b>address</b>): Option&lt;u64&gt; {
    <b>if</b> (table::contains(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user)) {
        option::some(*table::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers, user))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_get_revenue_potential"></a>

## Function `get_revenue_potential`

Get total revenue potential (for analytics) with overflow protection


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_revenue_potential">get_revenue_potential</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_get_revenue_potential">get_revenue_potential</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 {
    <b>let</b> one_time_revenue = <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>)) {
        <b>let</b> price = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>);
        <b>let</b> count = table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers);
        // Use u128 <b>for</b> calculation to detect overflow
        <b>let</b> revenue = (price <b>as</b> u128) * (count <b>as</b> u128);
        <b>if</b> (revenue &gt; (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128)) {
            <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>
        } <b>else</b> {
            revenue <b>as</b> u64
        }
    } <b>else</b> {
        0
    };
    <b>let</b> subscription_revenue = <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>)) {
        <b>let</b> price = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>);
        <b>let</b> count = table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers);
        // Use u128 <b>for</b> calculation to detect overflow
        <b>let</b> revenue = (price <b>as</b> u128) * (count <b>as</b> u128);
        <b>if</b> (revenue &gt; (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128)) {
            <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>
        } <b>else</b> {
            revenue <b>as</b> u64
        }
    } <b>else</b> {
        0
    };
    // Safe addition with overflow protection
    <b>let</b> total_revenue = (one_time_revenue <b>as</b> u128) + (subscription_revenue <b>as</b> u128);
    <b>if</b> (total_revenue &gt; (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128)) {
        <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>
    } <b>else</b> {
        total_revenue <b>as</b> u64
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_has_any_sales"></a>

## Function `has_any_sales`

Check if MyData has any sales (one-time or subscription)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_any_sales">has_any_sales</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_has_any_sales">has_any_sales</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool {
    table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.purchasers) &gt; 0 || table::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.subscribers) &gt; 0
}
</code></pre>



</details>

<a name="social_contracts_mydata_registry_get_owner"></a>

## Function `registry_get_owner`

Get owner of a MyData by ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_registry_get_owner">registry_get_owner</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, ip_id: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_registry_get_owner">registry_get_owner</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>, ip_id: <b>address</b>): Option&lt;<b>address</b>&gt; {
    <b>if</b> (table::contains(&registry.ip_to_owner, ip_id)) {
        option::some(*table::borrow(&registry.ip_to_owner, ip_id))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_is_registered"></a>

## Function `is_registered`

Check if a MyData is registered


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_registered">is_registered</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, ip_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_registered">is_registered</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>, ip_id: <b>address</b>): bool {
    table::contains(&registry.ip_to_owner, ip_id)
}
</code></pre>



</details>

<a name="social_contracts_mydata_register_in_registry"></a>

## Function `register_in_registry`

Register a MyData in the registry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_register_in_registry">register_in_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_register_in_registry">register_in_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    // Check <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    <b>if</b> (!table::contains(&registry.ip_to_owner, ip_id)) {
        table::add(&<b>mut</b> registry.ip_to_owner, ip_id, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
        // Emit registration event
        event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegisteredEvent">MyDataRegisteredEvent</a> {
            ip_id,
            <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
            registered_at: clock::timestamp_ms(clock),
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_mydata_unregister_from_registry"></a>

## Function `unregister_from_registry`

Remove a MyData from the registry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_unregister_from_registry">unregister_from_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, ip_id: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_unregister_from_registry">unregister_from_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    ip_id: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    // Check <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> compatibility
    <b>assert</b>!(registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>if</b> (table::contains(&registry.ip_to_owner, ip_id)) {
        <b>let</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a> = *table::borrow(&registry.ip_to_owner, ip_id);
        <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
        table::remove(&<b>mut</b> registry.ip_to_owner, ip_id);
        // Emit unregistration event
        event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataUnregisteredEvent">MyDataUnregisteredEvent</a> {
            ip_id,
            <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
            unregistered_at: clock::timestamp_ms(clock),
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_mydata_version"></a>

## Function `version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 {
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_borrow_version_mut"></a>

## Function `borrow_version_mut`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_borrow_version_mut">borrow_version_mut</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): &<b>mut</b> u64 {
    &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_registry_version"></a>

## Function `registry_version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_registry_version">registry_version</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_registry_version">registry_version</a>(registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>): u64 {
    registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_borrow_registry_version_mut"></a>

## Function `borrow_registry_version_mut`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>): &<b>mut</b> u64 {
    &<b>mut</b> registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_migrate_mydata"></a>

## Function `migrate_mydata`

Migration function for MyData


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_migrate_mydata">migrate_mydata</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_migrate_mydata">migrate_mydata</a>(
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> &lt; current_version, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> old_version = <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>;
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> = current_version;
    <b>let</b> mydata_id = object::id(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        mydata_id,
        string::utf8(b"<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_migrate_registry"></a>

## Function `migrate_registry`

Migration function for MyDataRegistry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_migrate_registry">migrate_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_migrate_registry">migrate_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> &lt; current_version, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> old_version = registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>;
    registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> = current_version;
    <b>let</b> registry_id = object::id(registry);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        registry_id,
        string::utf8(b"<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_migrate_config"></a>

## Function `migrate_config`

Migration function for MyDataConfig


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_migrate_config">migrate_config</a>(config: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_migrate_config">migrate_config</a>(
    config: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> &gt; current <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>)
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> &lt; current_version, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    // Remember old <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> and update to new <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>
    <b>let</b> old_version = config.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>;
    config.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> config_id = object::id(config);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        config_id,
        string::utf8(b"<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>
