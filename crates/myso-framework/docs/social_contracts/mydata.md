---
title: Module `social_contracts::mydata`
---

Universal MyData module for encrypted data monetization (one-time purchase, subscription, owner vault).

**Production decrypt path (client-only):** Plaintext must only exist off-chain. Callers encrypt before
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_create">create</a></code> / dedicated <code>create_and_share_*</code> entry points, then authorized users use the MyData SDK: resolve access (indexer or
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_has_access">has_access</a></code>), request keys via the key server (<code>fetch_key</code>) with policy approval (<code><a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve">mydata_approve</a></code>),
and decrypt locally. Do not rely on Move to produce user content plaintext for marketplace listings.

**On-chain state:** <code>encrypted_data</code> is opaque ciphertext. For BF-HMAC MyData blobs,
<code><a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_EncryptedObject">mydata::bf_hmac_encryption::EncryptedObject</a></code> embeds <code>package_id</code> and <code>id</code>; the <code>encryption_id</code> field
must be the same <code>id</code> bytes used when encrypting so policy and clients stay aligned. For client-only
AES-GCM (or other app-managed schemes), ciphertext does not parse as <code>EncryptedObject</code>; encode the
scheme in <code><a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a></code> (e.g. prefix <code>aes_gcm:</code>) or app metadata so indexers pick the right decrypt path.

**Revocation:** Owners may call [<code><a href="../social_contracts/mydata.md#social_contracts_mydata_revoke_access">revoke_access</a></code>] to remove a buyer from marketplace access tables.
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
-  [Struct `SnapshotEscrowFundedEvent`](#social_contracts_mydata_SnapshotEscrowFundedEvent)
-  [Struct `SnapshotEscrowReclaimedEvent`](#social_contracts_mydata_SnapshotEscrowReclaimedEvent)
-  [Struct `MyDataClaimVault`](#social_contracts_mydata_MyDataClaimVault)
-  [Struct `MerkleRootPublishedEvent`](#social_contracts_mydata_MerkleRootPublishedEvent)
-  [Struct `ClaimExecutedEvent`](#social_contracts_mydata_ClaimExecutedEvent)
-  [Struct `DistributionRound`](#social_contracts_mydata_DistributionRound)
-  [Struct `DistributionRegistry`](#social_contracts_mydata_DistributionRegistry)
-  [Struct `MyDataCreatedEvent`](#social_contracts_mydata_MyDataCreatedEvent)
-  [Struct `PurchaseEvent`](#social_contracts_mydata_PurchaseEvent)
-  [Struct `AccessGrantedEvent`](#social_contracts_mydata_AccessGrantedEvent)
-  [Struct `AccessRevokedEvent`](#social_contracts_mydata_AccessRevokedEvent)
-  [Struct `MyDataPricingUpdatedEvent`](#social_contracts_mydata_MyDataPricingUpdatedEvent)
-  [Struct `MyDataContentUpdatedEvent`](#social_contracts_mydata_MyDataContentUpdatedEvent)
-  [Struct `MyDataRegisteredEvent`](#social_contracts_mydata_MyDataRegisteredEvent)
-  [Struct `MyDataUnregisteredEvent`](#social_contracts_mydata_MyDataUnregisteredEvent)
-  [Struct `MyDataConfigUpdatedEvent`](#social_contracts_mydata_MyDataConfigUpdatedEvent)
-  [Enum `AccessConfiguration`](#social_contracts_mydata_AccessConfiguration)
-  [Constants](#@Constants_0)
-  [Function `validate_fee_config`](#social_contracts_mydata_validate_fee_config)
-  [Function `calculate_p2p_fees`](#social_contracts_mydata_calculate_p2p_fees)
-  [Function `calculate_mydata_marketplace_fees`](#social_contracts_mydata_calculate_mydata_marketplace_fees)
-  [Function `route_non_platform_platform_fee`](#social_contracts_mydata_route_non_platform_platform_fee)
-  [Function `distribute_p2p_fees_no_platform`](#social_contracts_mydata_distribute_p2p_fees_no_platform)
-  [Function `distribute_p2p_fees_with_platform`](#social_contracts_mydata_distribute_p2p_fees_with_platform)
-  [Function `assert_platform_matches_listing`](#social_contracts_mydata_assert_platform_matches_listing)
-  [Function `emit_mydata_config_updated`](#social_contracts_mydata_emit_mydata_config_updated)
-  [Function `create_mydata_admin_cap`](#social_contracts_mydata_create_mydata_admin_cap)
-  [Function `update_mydata_config`](#social_contracts_mydata_update_mydata_config)
-  [Function `marketplace_enabled`](#social_contracts_mydata_marketplace_enabled)
-  [Function `share_mydata_system_objects`](#social_contracts_mydata_share_mydata_system_objects)
-  [Function `bootstrap_init`](#social_contracts_mydata_bootstrap_init)
-  [Function `create_mydata_pool_admin_cap`](#social_contracts_mydata_create_mydata_pool_admin_cap)
-  [Function `gen_pool_id`](#social_contracts_mydata_gen_pool_id)
-  [Function `create_broad_pool_internal`](#social_contracts_mydata_create_broad_pool_internal)
-  [Function `create_broad_pool`](#social_contracts_mydata_create_broad_pool)
-  [Function `create_broad_pool_with_platform`](#social_contracts_mydata_create_broad_pool_with_platform)
-  [Function `create_sub_pool`](#social_contracts_mydata_create_sub_pool)
-  [Function `assign_mydata_to_sub_pools`](#social_contracts_mydata_assign_mydata_to_sub_pools)
-  [Function `remove_mydata_from_sub_pool`](#social_contracts_mydata_remove_mydata_from_sub_pool)
-  [Function `gen_snapshot_id`](#social_contracts_mydata_gen_snapshot_id)
-  [Function `record_snapshot_anchor`](#social_contracts_mydata_record_snapshot_anchor)
-  [Function `get_snapshot_anchor`](#social_contracts_mydata_get_snapshot_anchor)
-  [Function `deposit_snapshot_escrow`](#social_contracts_mydata_deposit_snapshot_escrow)
-  [Function `publish_distribution`](#social_contracts_mydata_publish_distribution)
-  [Function `distribute_mydata_marketplace_claim_fees_no_platform`](#social_contracts_mydata_distribute_mydata_marketplace_claim_fees_no_platform)
-  [Function `distribute_mydata_marketplace_claim_fees_with_platform`](#social_contracts_mydata_distribute_mydata_marketplace_claim_fees_with_platform)
-  [Function `claim_internal_no_platform`](#social_contracts_mydata_claim_internal_no_platform)
-  [Function `claim_internal_with_platform`](#social_contracts_mydata_claim_internal_with_platform)
-  [Function `claim`](#social_contracts_mydata_claim)
-  [Function `claim_with_platform`](#social_contracts_mydata_claim_with_platform)
-  [Function `reclaim_expired_snapshot_escrow`](#social_contracts_mydata_reclaim_expired_snapshot_escrow)
-  [Function `get_broad_pool`](#social_contracts_mydata_get_broad_pool)
-  [Function `get_sub_pool`](#social_contracts_mydata_get_sub_pool)
-  [Function `get_mydata_sub_pools`](#social_contracts_mydata_get_mydata_sub_pools)
-  [Function `get_distribution_round`](#social_contracts_mydata_get_distribution_round)
-  [Function `broad_pool_id`](#social_contracts_mydata_broad_pool_id)
-  [Function `sub_pool_id`](#social_contracts_mydata_sub_pool_id)
-  [Function `access_configuration`](#social_contracts_mydata_access_configuration)
-  [Function `requires_profile_subscription_access`](#social_contracts_mydata_requires_profile_subscription_access)
-  [Function `requires_marketplace_purchase`](#social_contracts_mydata_requires_marketplace_purchase)
-  [Function `requires_marketplace_subscription`](#social_contracts_mydata_requires_marketplace_subscription)
-  [Function `linked_one_time_price`](#social_contracts_mydata_linked_one_time_price)
-  [Function `access_configuration_kind`](#social_contracts_mydata_access_configuration_kind)
-  [Function `validate_marketplace_price`](#social_contracts_mydata_validate_marketplace_price)
-  [Function `validate_recurring_duration`](#social_contracts_mydata_validate_recurring_duration)
-  [Function `validate_access_configuration`](#social_contracts_mydata_validate_access_configuration)
-  [Function `validate_optional_metadata`](#social_contracts_mydata_validate_optional_metadata)
-  [Function `validate_tags`](#social_contracts_mydata_validate_tags)
-  [Function `emit_mydata_created_event`](#social_contracts_mydata_emit_mydata_created_event)
-  [Function `create`](#social_contracts_mydata_create)
-  [Function `share_created_mydata`](#social_contracts_mydata_share_created_mydata)
-  [Function `create_and_share_internal`](#social_contracts_mydata_create_and_share_internal)
-  [Function `create_and_share_profile_subscription_mydata`](#social_contracts_mydata_create_and_share_profile_subscription_mydata)
-  [Function `create_and_share_marketplace_one_time_mydata`](#social_contracts_mydata_create_and_share_marketplace_one_time_mydata)
-  [Function `create_and_share_marketplace_recurring_mydata`](#social_contracts_mydata_create_and_share_marketplace_recurring_mydata)
-  [Function `purchase_one_time_no_platform`](#social_contracts_mydata_purchase_one_time_no_platform)
-  [Function `purchase_one_time_with_platform_internal`](#social_contracts_mydata_purchase_one_time_with_platform_internal)
-  [Function `purchase_one_time`](#social_contracts_mydata_purchase_one_time)
-  [Function `purchase_one_time_with_platform`](#social_contracts_mydata_purchase_one_time_with_platform)
-  [Function `purchase_subscription_no_platform`](#social_contracts_mydata_purchase_subscription_no_platform)
-  [Function `purchase_subscription_with_platform_internal`](#social_contracts_mydata_purchase_subscription_with_platform_internal)
-  [Function `purchase_subscription`](#social_contracts_mydata_purchase_subscription)
-  [Function `purchase_subscription_with_platform`](#social_contracts_mydata_purchase_subscription_with_platform)
-  [Function `update_pricing`](#social_contracts_mydata_update_pricing)
-  [Function `update_content`](#social_contracts_mydata_update_content)
-  [Function `assign_mydata_to_pools`](#social_contracts_mydata_assign_mydata_to_pools)
-  [Function `remove_mydata_from_sub_pools`](#social_contracts_mydata_remove_mydata_from_sub_pools)
-  [Function `has_access`](#social_contracts_mydata_has_access)
-  [Function `encryption_id_matches`](#social_contracts_mydata_encryption_id_matches)
-  [Function `mydata_approve`](#social_contracts_mydata_mydata_approve)
-  [Function `mydata_approve_profile_subscription`](#social_contracts_mydata_mydata_approve_profile_subscription)
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


<pre><code><b>use</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption">mydata::bf_hmac_encryption</a>;
<b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr">mydata::hmac256ctr</a>;
<b>use</b> <a href="../mydata/kdf.md#mydata_kdf">mydata::kdf</a>;
<b>use</b> <a href="../mydata/merkle.md#mydata_merkle">mydata::merkle</a>;
<b>use</b> <a href="../mydata/polynomial.md#mydata_polynomial">mydata::polynomial</a>;
<b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bag.md#myso_bag">myso::bag</a>;
<b>use</b> <a href="../myso/balance.md#myso_balance">myso::balance</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bls12381.md#myso_bls12381">myso::bls12381</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
<b>use</b> <a href="../myso/coin.md#myso_coin">myso::coin</a>;
<b>use</b> <a href="../myso/config.md#myso_config">myso::config</a>;
<b>use</b> <a href="../myso/deny_list.md#myso_deny_list">myso::deny_list</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/dynamic_object_field.md#myso_dynamic_object_field">myso::dynamic_object_field</a>;
<b>use</b> <a href="../myso/ed25519.md#myso_ed25519">myso::ed25519</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/funds_accumulator.md#myso_funds_accumulator">myso::funds_accumulator</a>;
<b>use</b> <a href="../myso/group_ops.md#myso_group_ops">myso::group_ops</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/hmac.md#myso_hmac">myso::hmac</a>;
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
<b>use</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit">social_contracts::ai_credit</a>;
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/platform.md#social_contracts_platform">social_contracts::platform</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/subscription.md#social_contracts_subscription">social_contracts::subscription</a>;
<b>use</b> <a href="../social_contracts/upgrade.md#social_contracts_upgrade">social_contracts::upgrade</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/hash.md#std_hash">std::hash</a>;
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
<code>access: <a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">social_contracts::mydata::AccessConfiguration</a></code>
</dt>
<dd>
 Access model (profile subscription gate or marketplace one-time/recurring).
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
<code>max_encrypted_data_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_tag_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_metadata_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_payment_reference_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_pool_assignments: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_merkle_proof_depth: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_paid_access_entries: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>default_claim_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>p2p_platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>p2p_ecosystem_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>mydata_marketplace_platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>mydata_marketplace_ecosystem_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>non_platform_platform_to_creator_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>non_platform_platform_to_treasury_bps: u64</code>
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
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
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_deadline_ms: u64</code>
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

<a name="social_contracts_mydata_SnapshotEscrowFundedEvent"></a>

## Struct `SnapshotEscrowFundedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotEscrowFundedEvent">SnapshotEscrowFundedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>funder: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>total_funded: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>funded_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_mydata_SnapshotEscrowReclaimedEvent"></a>

## Struct `SnapshotEscrowReclaimedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotEscrowReclaimedEvent">SnapshotEscrowReclaimedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reclaimed_at: u64</code>
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
<code>gross_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>net_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>claim_deadline_ms: u64</code>
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration_kind">access_configuration_kind</a>: u8</code>
</dt>
<dd>
 [<code>ACCESS_KIND_*</code>] tag for indexers (1=profile, 2=one_time, 3=recurring).
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
<dt>
<code>platform_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ecosystem_fee: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
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

<a name="social_contracts_mydata_MyDataPricingUpdatedEvent"></a>

## Struct `MyDataPricingUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPricingUpdatedEvent">MyDataPricingUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_by: <b>address</b></code>
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

<a name="social_contracts_mydata_MyDataContentUpdatedEvent"></a>

## Struct `MyDataContentUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataContentUpdatedEvent">MyDataContentUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code>encrypted_data_updated: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>tags_updated: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_by: <b>address</b></code>
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
<code>max_encrypted_data_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_tag_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_metadata_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_payment_reference_bytes: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_pool_assignments: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_merkle_proof_depth: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_paid_access_entries: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>default_claim_window_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>p2p_platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>p2p_ecosystem_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>mydata_marketplace_platform_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>mydata_marketplace_ecosystem_fee_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>non_platform_platform_to_creator_bps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>non_platform_platform_to_treasury_bps: u64</code>
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

<a name="social_contracts_mydata_AccessConfiguration"></a>

## Enum `AccessConfiguration`

Mutually exclusive access model for a MyData listing.


<pre><code><b>public</b> <b>enum</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">AccessConfiguration</a> <b>has</b> store
</code></pre>



<details>
<summary>Variants</summary>


<dl>
<dt>
Variant <code>ProfileSubscription</code>
</dt>
<dd>
 Gated by profile subscription on a linked post (no marketplace pricing).
</dd>
<dt>
Variant <code>MarketplaceOneTime</code>
</dt>
<dd>
 One-time marketplace purchase.
</dd>

<dl>
<dt>
<code>price: u64</code>
</dt>
<dd>
</dd>
</dl>


<dl>
<dt>
<code>purchasers: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>
</dd>
</dl>

<dt>
Variant <code>MarketplaceRecurring</code>
</dt>
<dd>
 Recurring marketplace subscription with fixed duration per purchase.
</dd>

<dl>
<dt>
<code>price: u64</code>
</dt>
<dd>
</dd>
</dl>


<dl>
<dt>
<code>duration_days: u64</code>
</dt>
<dd>
</dd>
</dl>


<dl>
<dt>
<code>subscribers: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, u64&gt;</code>
</dt>
<dd>
</dd>
</dl>

</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_mydata_DEFAULT_MARKETPLACE_ENABLED"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MARKETPLACE_ENABLED">DEFAULT_MARKETPLACE_ENABLED</a>: bool = <b>false</b>;
</code></pre>



<a name="social_contracts_mydata_BPS_DENOM"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_P2P_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_P2P_PLATFORM_FEE_BPS">DEFAULT_P2P_PLATFORM_FEE_BPS</a>: u64 = 250;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_P2P_ECOSYSTEM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_P2P_ECOSYSTEM_FEE_BPS">DEFAULT_P2P_ECOSYSTEM_FEE_BPS</a>: u64 = 250;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS">DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS</a>: u64 = 250;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS">DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS</a>: u64 = 250;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS</a>: u64 = 0;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS</a>: u64 = 10000;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_MAX_ENCRYPTED_DATA_BYTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_ENCRYPTED_DATA_BYTES">DEFAULT_MAX_ENCRYPTED_DATA_BYTES</a>: u64 = 262144;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_MAX_TAG_BYTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_TAG_BYTES">DEFAULT_MAX_TAG_BYTES</a>: u64 = 64;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_MAX_METADATA_BYTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_METADATA_BYTES">DEFAULT_MAX_METADATA_BYTES</a>: u64 = 1024;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_MAX_PAYMENT_REFERENCE_BYTES"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_PAYMENT_REFERENCE_BYTES">DEFAULT_MAX_PAYMENT_REFERENCE_BYTES</a>: u64 = 256;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_MAX_POOL_ASSIGNMENTS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_POOL_ASSIGNMENTS">DEFAULT_MAX_POOL_ASSIGNMENTS</a>: u64 = 32;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_MAX_MERKLE_PROOF_DEPTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_MERKLE_PROOF_DEPTH">DEFAULT_MAX_MERKLE_PROOF_DEPTH</a>: u64 = 64;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_MAX_PAID_ACCESS_ENTRIES"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_PAID_ACCESS_ENTRIES">DEFAULT_MAX_PAID_ACCESS_ENTRIES</a>: u64 = 100000;
</code></pre>



<a name="social_contracts_mydata_DEFAULT_CLAIM_WINDOW_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_CLAIM_WINDOW_MS">DEFAULT_CLAIM_WINDOW_MS</a>: u64 = 2592000000;
</code></pre>



<a name="social_contracts_mydata_ACCESS_KIND_PROFILE"></a>

Event/indexer tags for [<code><a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">AccessConfiguration</a></code>] (not used for on-chain policy).


<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ACCESS_KIND_PROFILE">ACCESS_KIND_PROFILE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_mydata_ACCESS_KIND_ONE_TIME"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ACCESS_KIND_ONE_TIME">ACCESS_KIND_ONE_TIME</a>: u8 = 2;
</code></pre>



<a name="social_contracts_mydata_ACCESS_KIND_RECURRING"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ACCESS_KIND_RECURRING">ACCESS_KIND_RECURRING</a>: u8 = 3;
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



<a name="social_contracts_mydata_EInvalidConfig"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidConfig">EInvalidConfig</a>: u64 = 15;
</code></pre>



<a name="social_contracts_mydata_EPlatformMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPlatformMismatch">EPlatformMismatch</a>: u64 = 16;
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



<a name="social_contracts_mydata_EPqDistributionNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqDistributionNotFound">EPqDistributionNotFound</a>: u64 = 11;
</code></pre>



<a name="social_contracts_mydata_EPqClaimExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqClaimExpired">EPqClaimExpired</a>: u64 = 12;
</code></pre>



<a name="social_contracts_mydata_EPqClaimNotExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqClaimNotExpired">EPqClaimNotExpired</a>: u64 = 13;
</code></pre>



<a name="social_contracts_mydata_EPqPlatformMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqPlatformMismatch">EPqPlatformMismatch</a>: u64 = 14;
</code></pre>



<a name="social_contracts_mydata_EPqDistributionPublished"></a>



<pre><code><b>const</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqDistributionPublished">EPqDistributionPublished</a>: u64 = 15;
</code></pre>



<a name="social_contracts_mydata_validate_fee_config"></a>

## Function `validate_fee_config`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_fee_config">validate_fee_config</a>(p2p_platform_fee_bps: u64, p2p_ecosystem_fee_bps: u64, mydata_marketplace_platform_fee_bps: u64, mydata_marketplace_ecosystem_fee_bps: u64, non_platform_platform_to_creator_bps: u64, non_platform_platform_to_treasury_bps: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_fee_config">validate_fee_config</a>(
    p2p_platform_fee_bps: u64,
    p2p_ecosystem_fee_bps: u64,
    mydata_marketplace_platform_fee_bps: u64,
    mydata_marketplace_ecosystem_fee_bps: u64,
    non_platform_platform_to_creator_bps: u64,
    non_platform_platform_to_treasury_bps: u64,
) {
    <b>assert</b>!(p2p_platform_fee_bps &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(p2p_ecosystem_fee_bps &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(p2p_platform_fee_bps + p2p_ecosystem_fee_bps &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(mydata_marketplace_platform_fee_bps &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(mydata_marketplace_ecosystem_fee_bps &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(mydata_marketplace_platform_fee_bps + mydata_marketplace_ecosystem_fee_bps &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(non_platform_platform_to_creator_bps &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(non_platform_platform_to_treasury_bps &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(
        non_platform_platform_to_creator_bps + non_platform_platform_to_treasury_bps == <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidConfig">EInvalidConfig</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_calculate_p2p_fees"></a>

## Function `calculate_p2p_fees`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_calculate_p2p_fees">calculate_p2p_fees</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, gross: u64): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_calculate_p2p_fees">calculate_p2p_fees</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>, gross: u64): (u64, u64, u64) {
    <b>let</b> platform_fee = (gross * config.p2p_platform_fee_bps) / <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> ecosystem_fee = (gross * config.p2p_ecosystem_fee_bps) / <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> creator_amount = gross - platform_fee - ecosystem_fee;
    (platform_fee, ecosystem_fee, creator_amount)
}
</code></pre>



</details>

<a name="social_contracts_mydata_calculate_mydata_marketplace_fees"></a>

## Function `calculate_mydata_marketplace_fees`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_calculate_mydata_marketplace_fees">calculate_mydata_marketplace_fees</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, gross: u64): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_calculate_mydata_marketplace_fees">calculate_mydata_marketplace_fees</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>, gross: u64): (u64, u64, u64) {
    <b>let</b> platform_fee = (gross * config.mydata_marketplace_platform_fee_bps) / <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> ecosystem_fee = (gross * config.mydata_marketplace_ecosystem_fee_bps) / <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> net_amount = gross - platform_fee - ecosystem_fee;
    (platform_fee, ecosystem_fee, net_amount)
}
</code></pre>



</details>

<a name="social_contracts_mydata_route_non_platform_platform_fee"></a>

## Function `route_non_platform_platform_fee`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_route_non_platform_platform_fee">route_non_platform_platform_fee</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, platform_fee: u64, recipient_amount: u64, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_route_non_platform_platform_fee">route_non_platform_platform_fee</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    treasury: &EcosystemTreasury,
    platform_fee: u64,
    recipient_amount: u64,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext,
): u64 {
    <b>let</b> platform_fee_to_recipient =
        (platform_fee * config.non_platform_platform_to_creator_bps) / <a href="../social_contracts/mydata.md#social_contracts_mydata_BPS_DENOM">BPS_DENOM</a>;
    <b>let</b> platform_fee_to_treasury = platform_fee - platform_fee_to_recipient;
    <b>let</b> recipient_amount = recipient_amount + platform_fee_to_recipient;
    <b>if</b> (platform_fee_to_treasury &gt; 0) {
        <b>let</b> treasury_coin = coin::split(payment, platform_fee_to_treasury, ctx);
        transfer::public_transfer(treasury_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    recipient_amount
}
</code></pre>



</details>

<a name="social_contracts_mydata_distribute_p2p_fees_no_platform"></a>

## Function `distribute_p2p_fees_no_platform`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_p2p_fees_no_platform">distribute_p2p_fees_no_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <b>address</b>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_p2p_fees_no_platform">distribute_p2p_fees_no_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <b>address</b>,
    payment: Coin&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext,
): (u64, u64, u64) {
    <b>let</b> gross = coin::value(&payment);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) = <a href="../social_contracts/mydata.md#social_contracts_mydata_calculate_p2p_fees">calculate_p2p_fees</a>(config, gross);
    <b>let</b> <b>mut</b> payment = payment;
    <b>if</b> (ecosystem_fee &gt; 0) {
        <b>let</b> eco_coin = coin::split(&<b>mut</b> payment, ecosystem_fee, ctx);
        transfer::public_transfer(eco_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    <b>let</b> creator_amount = <b>if</b> (platform_fee &gt; 0) {
        <a href="../social_contracts/mydata.md#social_contracts_mydata_route_non_platform_platform_fee">route_non_platform_platform_fee</a>(
            config,
            treasury,
            platform_fee,
            creator_amount,
            &<b>mut</b> payment,
            ctx,
        )
    } <b>else</b> {
        creator_amount
    };
    transfer::public_transfer(payment, <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    (platform_fee, ecosystem_fee, creator_amount)
}
</code></pre>



</details>

<a name="social_contracts_mydata_distribute_p2p_fees_with_platform"></a>

## Function `distribute_p2p_fees_with_platform`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_p2p_fees_with_platform">distribute_p2p_fees_with_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <b>address</b>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_p2p_fees_with_platform">distribute_p2p_fees_with_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <b>address</b>,
    payment: Coin&lt;MYSO&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): (u64, u64, u64) {
    <b>let</b> gross = coin::value(&payment);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) = <a href="../social_contracts/mydata.md#social_contracts_mydata_calculate_p2p_fees">calculate_p2p_fees</a>(config, gross);
    <b>let</b> <b>mut</b> payment = payment;
    <b>if</b> (ecosystem_fee &gt; 0) {
        <b>let</b> eco_coin = coin::split(&<b>mut</b> payment, ecosystem_fee, ctx);
        transfer::public_transfer(eco_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    <b>if</b> (platform_fee &gt; 0) {
        <b>let</b> <b>mut</b> platform_coin = coin::split(&<b>mut</b> payment, platform_fee, ctx);
        <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_coin, platform_fee, clock, ctx);
        coin::destroy_zero(platform_coin);
    };
    transfer::public_transfer(payment, <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    (platform_fee, ecosystem_fee, creator_amount)
}
</code></pre>



</details>

<a name="social_contracts_mydata_assert_platform_matches_listing"></a>

## Function `assert_platform_matches_listing`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_assert_platform_matches_listing">assert_platform_matches_listing</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_assert_platform_matches_listing">assert_platform_matches_listing</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &Platform) {
    <b>if</b> (option::is_some(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>)) {
        <b>let</b> listing_platform = *option::borrow(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>);
        <b>let</b> provided_platform = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
        <b>assert</b>!(listing_platform == provided_platform, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPlatformMismatch">EPlatformMismatch</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_mydata_emit_mydata_config_updated"></a>

## Function `emit_mydata_config_updated`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_emit_mydata_config_updated">emit_mydata_config_updated</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, updated_by: <b>address</b>, timestamp: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_emit_mydata_config_updated">emit_mydata_config_updated</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>, updated_by: <b>address</b>, timestamp: u64) {
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfigUpdatedEvent">MyDataConfigUpdatedEvent</a> {
        updated_by,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>: config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>,
        max_tags: config.max_tags,
        max_subscription_days: config.max_subscription_days,
        max_free_access_grants: config.max_free_access_grants,
        max_encryption_id_bytes: config.max_encryption_id_bytes,
        max_encrypted_data_bytes: config.max_encrypted_data_bytes,
        max_tag_bytes: config.max_tag_bytes,
        max_metadata_bytes: config.max_metadata_bytes,
        max_payment_reference_bytes: config.max_payment_reference_bytes,
        max_pool_assignments: config.max_pool_assignments,
        max_merkle_proof_depth: config.max_merkle_proof_depth,
        max_paid_access_entries: config.max_paid_access_entries,
        default_claim_window_ms: config.default_claim_window_ms,
        p2p_platform_fee_bps: config.p2p_platform_fee_bps,
        p2p_ecosystem_fee_bps: config.p2p_ecosystem_fee_bps,
        mydata_marketplace_platform_fee_bps: config.mydata_marketplace_platform_fee_bps,
        mydata_marketplace_ecosystem_fee_bps: config.mydata_marketplace_ecosystem_fee_bps,
        non_platform_platform_to_creator_bps: config.non_platform_platform_to_creator_bps,
        non_platform_platform_to_treasury_bps: config.non_platform_platform_to_treasury_bps,
        timestamp,
    });
}
</code></pre>



</details>

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


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_mydata_config">update_mydata_config</a>(_: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataAdminCap">social_contracts::mydata::MyDataAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>: bool, max_tags: u64, max_subscription_days: u64, max_free_access_grants: u64, max_encryption_id_bytes: u64, max_encrypted_data_bytes: u64, max_tag_bytes: u64, max_metadata_bytes: u64, max_payment_reference_bytes: u64, max_pool_assignments: u64, max_merkle_proof_depth: u64, max_paid_access_entries: u64, default_claim_window_ms: u64, p2p_platform_fee_bps: u64, p2p_ecosystem_fee_bps: u64, mydata_marketplace_platform_fee_bps: u64, mydata_marketplace_ecosystem_fee_bps: u64, non_platform_platform_to_creator_bps: u64, non_platform_platform_to_treasury_bps: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
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
    max_encrypted_data_bytes: u64,
    max_tag_bytes: u64,
    max_metadata_bytes: u64,
    max_payment_reference_bytes: u64,
    max_pool_assignments: u64,
    max_merkle_proof_depth: u64,
    max_paid_access_entries: u64,
    default_claim_window_ms: u64,
    p2p_platform_fee_bps: u64,
    p2p_ecosystem_fee_bps: u64,
    mydata_marketplace_platform_fee_bps: u64,
    mydata_marketplace_ecosystem_fee_bps: u64,
    non_platform_platform_to_creator_bps: u64,
    non_platform_platform_to_treasury_bps: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(max_subscription_days &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_tags &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_free_access_grants &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_encryption_id_bytes &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_encrypted_data_bytes &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_tag_bytes &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_metadata_bytes &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_payment_reference_bytes &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_pool_assignments &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_merkle_proof_depth &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(max_paid_access_entries &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(default_claim_window_ms &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_fee_config">validate_fee_config</a>(
        p2p_platform_fee_bps,
        p2p_ecosystem_fee_bps,
        mydata_marketplace_platform_fee_bps,
        mydata_marketplace_ecosystem_fee_bps,
        non_platform_platform_to_creator_bps,
        non_platform_platform_to_treasury_bps,
    );
    config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a> = <a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>;
    config.max_tags = max_tags;
    config.max_subscription_days = max_subscription_days;
    config.max_free_access_grants = max_free_access_grants;
    config.max_encryption_id_bytes = max_encryption_id_bytes;
    config.max_encrypted_data_bytes = max_encrypted_data_bytes;
    config.max_tag_bytes = max_tag_bytes;
    config.max_metadata_bytes = max_metadata_bytes;
    config.max_payment_reference_bytes = max_payment_reference_bytes;
    config.max_pool_assignments = max_pool_assignments;
    config.max_merkle_proof_depth = max_merkle_proof_depth;
    config.max_paid_access_entries = max_paid_access_entries;
    config.default_claim_window_ms = default_claim_window_ms;
    config.p2p_platform_fee_bps = p2p_platform_fee_bps;
    config.p2p_ecosystem_fee_bps = p2p_ecosystem_fee_bps;
    config.mydata_marketplace_platform_fee_bps = mydata_marketplace_platform_fee_bps;
    config.mydata_marketplace_ecosystem_fee_bps = mydata_marketplace_ecosystem_fee_bps;
    config.non_platform_platform_to_creator_bps = non_platform_platform_to_creator_bps;
    config.non_platform_platform_to_treasury_bps = non_platform_platform_to_treasury_bps;
    <a href="../social_contracts/mydata.md#social_contracts_mydata_emit_mydata_config_updated">emit_mydata_config_updated</a>(
        config,
        tx_context::sender(ctx),
        clock::timestamp_ms(clock),
    );
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
        max_encrypted_data_bytes: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_ENCRYPTED_DATA_BYTES">DEFAULT_MAX_ENCRYPTED_DATA_BYTES</a>,
        max_tag_bytes: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_TAG_BYTES">DEFAULT_MAX_TAG_BYTES</a>,
        max_metadata_bytes: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_METADATA_BYTES">DEFAULT_MAX_METADATA_BYTES</a>,
        max_payment_reference_bytes: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_PAYMENT_REFERENCE_BYTES">DEFAULT_MAX_PAYMENT_REFERENCE_BYTES</a>,
        max_pool_assignments: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_POOL_ASSIGNMENTS">DEFAULT_MAX_POOL_ASSIGNMENTS</a>,
        max_merkle_proof_depth: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_MERKLE_PROOF_DEPTH">DEFAULT_MAX_MERKLE_PROOF_DEPTH</a>,
        max_paid_access_entries: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_PAID_ACCESS_ENTRIES">DEFAULT_MAX_PAID_ACCESS_ENTRIES</a>,
        default_claim_window_ms: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_CLAIM_WINDOW_MS">DEFAULT_CLAIM_WINDOW_MS</a>,
        p2p_platform_fee_bps: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_P2P_PLATFORM_FEE_BPS">DEFAULT_P2P_PLATFORM_FEE_BPS</a>,
        p2p_ecosystem_fee_bps: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_P2P_ECOSYSTEM_FEE_BPS">DEFAULT_P2P_ECOSYSTEM_FEE_BPS</a>,
        mydata_marketplace_platform_fee_bps: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS">DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS</a>,
        mydata_marketplace_ecosystem_fee_bps: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS">DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS</a>,
        non_platform_platform_to_creator_bps: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS</a>,
        non_platform_platform_to_treasury_bps: <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: ver,
    };
    <a href="../social_contracts/mydata.md#social_contracts_mydata_emit_mydata_config_updated">emit_mydata_config_updated</a>(&config, sender, clock::timestamp_ms(clock));
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



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_mydata_pool_admin_cap">create_mydata_pool_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_mydata_pool_admin_cap">create_mydata_pool_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a> {
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

<a name="social_contracts_mydata_create_broad_pool_internal"></a>

## Function `create_broad_pool_internal`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_broad_pool_internal">create_broad_pool_internal</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_broad_pool_internal">create_broad_pool_internal</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    name: String,
    description: String,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: Option&lt;<b>address</b>&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(string::length(&name) &gt; 0 && string::length(&name) &lt;= config.max_metadata_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>assert</b>!(string::length(&description) &lt;= config.max_metadata_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>let</b> nonce = registry.next_broad_pool_nonce;
    registry.next_broad_pool_nonce = nonce + 1;
    <b>let</b> pool_id = <a href="../social_contracts/mydata.md#social_contracts_mydata_gen_pool_id">gen_pool_id</a>(registry, nonce);
    <b>let</b> broad_pool = <a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPool">BroadPool</a> {
        id: pool_id,
        name,
        description,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: clock::timestamp_ms(clock),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: registry.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>,
    };
    table::add(&<b>mut</b> registry.broad_pools, pool_id, broad_pool);
    table::add(&<b>mut</b> registry.broad_to_sub, pool_id, vector::empty());
    registry.last_created_broad_pool_id = option::some(pool_id);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_BroadPoolCreatedEvent">BroadPoolCreatedEvent</a> {
        pool_id,
        name: broad_pool.name,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: broad_pool.<a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_broad_pool"></a>

## Function `create_broad_pool`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_broad_pool">create_broad_pool</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, cap: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_broad_pool">create_broad_pool</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    cap: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    name: String,
    description: String,
    clock: &Clock,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_create_broad_pool_internal">create_broad_pool_internal</a>(config, cap, registry, name, description, option::none(), clock);
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_broad_pool_with_platform"></a>

## Function `create_broad_pool_with_platform`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_broad_pool_with_platform">create_broad_pool_with_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, cap: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_broad_pool_with_platform">create_broad_pool_with_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    cap: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &Platform,
    name: String,
    description: String,
    clock: &Clock,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_create_broad_pool_internal">create_broad_pool_internal</a>(
        config,
        cap,
        registry,
        name,
        description,
        option::some(object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>))),
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_sub_pool"></a>

## Function `create_sub_pool`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_sub_pool">create_sub_pool</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, name: <a href="../std/string.md#std_string_String">std::string::String</a>, description: <a href="../std/string.md#std_string_String">std::string::String</a>, schema_metadata: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_sub_pool">create_sub_pool</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>: ID,
    name: String,
    description: String,
    schema_metadata: Option&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(table::contains(&registry.broad_pools, <a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqPoolNotFound">EPqPoolNotFound</a>);
    <b>assert</b>!(string::length(&name) &gt; 0 && string::length(&name) &lt;= config.max_metadata_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>assert</b>!(string::length(&description) &lt;= config.max_metadata_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>if</b> (option::is_some(&schema_metadata)) {
        <b>assert</b>!(vector::length(option::borrow(&schema_metadata)) &lt;= config.max_metadata_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    };
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



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_assign_mydata_to_sub_pools">assign_mydata_to_sub_pools</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, ip_id: <b>address</b>, sub_pool_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_assign_mydata_to_sub_pools">assign_mydata_to_sub_pools</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    ip_id: <b>address</b>,
    sub_pool_ids: vector&lt;ID&gt;,
    clock: &Clock,
) {
    <b>assert</b>!(vector::length(&sub_pool_ids) &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>assert</b>!(vector::length(&sub_pool_ids) &lt;= config.max_pool_assignments, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
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
            <b>assert</b>!(vector::length(&existing) &lt;= config.max_pool_assignments, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
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



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_record_snapshot_anchor">record_snapshot_anchor</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, anchor_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">social_contracts::mydata::SnapshotAnchorRegistry</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, pool_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, source_pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, source_sub_pool_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, manifest_hash: vector&lt;u8&gt;, payment_reference: vector&lt;u8&gt;, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_record_snapshot_anchor">record_snapshot_anchor</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
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
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>);
    <b>assert</b>!(table::contains(&pool_registry.broad_pools, source_pool_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqPoolNotFound">EPqPoolNotFound</a>);
    <b>assert</b>!(table::contains(&pool_registry.sub_pools, source_sub_pool_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSubPoolNotFound">EPqSubPoolNotFound</a>);
    <b>let</b> broad_pool = table::borrow(&pool_registry.broad_pools, source_pool_id);
    <b>let</b> sub_pool = table::borrow(&pool_registry.sub_pools, source_sub_pool_id);
    <b>assert</b>!(sub_pool.<a href="../social_contracts/mydata.md#social_contracts_mydata_broad_pool_id">broad_pool_id</a> == source_pool_id, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSubPoolNotFound">EPqSubPoolNotFound</a>);
    <b>assert</b>!(vector::length(&manifest_hash) == 32, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>assert</b>!(vector::length(&payment_reference) &lt;= config.max_payment_reference_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
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
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: broad_pool.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
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
        source_pool_id,
        source_sub_pool_id,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: broad_pool.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
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

<a name="social_contracts_mydata_deposit_snapshot_escrow"></a>

## Function `deposit_snapshot_escrow`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_deposit_snapshot_escrow">deposit_snapshot_escrow</a>(_: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, anchor_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">social_contracts::mydata::SnapshotAnchorRegistry</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, payment: <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_deposit_snapshot_escrow">deposit_snapshot_escrow</a>(
    _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    anchor_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a>,
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    snapshot_id: ID,
    payment: Coin&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>assert</b>!(table::contains(&anchor_registry.anchors, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqAnchorNotFound">EPqAnchorNotFound</a>);
    <b>assert</b>!(!table::contains(&vault.merkle_roots, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqDistributionPublished">EPqDistributionPublished</a>);
    <b>assert</b>!(table::contains(&vault.snapshot_escrow, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSnapshotEscrowMissing">EPqSnapshotEscrowMissing</a>);
    <b>let</b> amount = coin::value(&payment);
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInsufficientPayment">EPqInsufficientPayment</a>);
    <b>let</b> escrow = table::borrow_mut(&<b>mut</b> vault.snapshot_escrow, snapshot_id);
    <b>assert</b>!(*escrow &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> - amount, <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
    *escrow = *escrow + amount;
    <b>let</b> total_funded = *escrow;
    balance::join(&<b>mut</b> vault.balance, coin::into_balance(payment));
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotEscrowFundedEvent">SnapshotEscrowFundedEvent</a> {
        snapshot_id,
        funder: tx_context::sender(ctx),
        amount,
        total_funded,
        funded_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_publish_distribution"></a>

## Function `publish_distribution`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_publish_distribution">publish_distribution</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">social_contracts::mydata::MyDataPoolAdminCap</a>, anchor_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">social_contracts::mydata::SnapshotAnchorRegistry</a>, dist_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">social_contracts::mydata::DistributionRegistry</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, root_hash: vector&lt;u8&gt;, total_amount: u64, contributor_count: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_publish_distribution">publish_distribution</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    _: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolAdminCap">MyDataPoolAdminCap</a>,
    anchor_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a>,
    dist_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">DistributionRegistry</a>,
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    snapshot_id: ID,
    root_hash: vector&lt;u8&gt;,
    total_amount: u64,
    contributor_count: u64,
    clock: &Clock,
) {
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>);
    <b>assert</b>!(table::contains(&anchor_registry.anchors, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqAnchorNotFound">EPqAnchorNotFound</a>);
    <b>assert</b>!(table::contains(&vault.snapshot_escrow, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSnapshotEscrowMissing">EPqSnapshotEscrowMissing</a>);
    <b>assert</b>!(vector::length(&root_hash) == 32, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>assert</b>!(total_amount &gt; 0 && contributor_count &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>assert</b>!(*table::borrow(&vault.snapshot_escrow, snapshot_id) == total_amount, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqEscrowExceeded">EPqEscrowExceeded</a>);
    <b>assert</b>!(!table::contains(&vault.merkle_roots, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqDistributionPublished">EPqDistributionPublished</a>);
    <b>assert</b>!(!table::contains(&dist_registry.rounds, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqDistributionPublished">EPqDistributionPublished</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> - config.default_claim_window_ms, <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
    <b>let</b> claim_deadline_ms = now + config.default_claim_window_ms;
    <b>let</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a> = table::borrow(&anchor_registry.anchors, snapshot_id).<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>;
    table::add(&<b>mut</b> vault.merkle_roots, snapshot_id, <b>copy</b> root_hash);
    table::add(&<b>mut</b> dist_registry.rounds, snapshot_id, <a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRound">DistributionRound</a> {
        snapshot_id,
        total_amount,
        contributor_count,
        merkle_root: <b>copy</b> root_hash,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        claim_deadline_ms,
        published_at: now,
    });
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MerkleRootPublishedEvent">MerkleRootPublishedEvent</a> {
        snapshot_id,
        root_hash: <b>copy</b> root_hash,
        published_at: now,
    });
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRecordedEvent">DistributionRecordedEvent</a> {
        snapshot_id,
        total_amount,
        contributor_count,
        merkle_root: root_hash,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        claim_deadline_ms,
        published_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_distribute_mydata_marketplace_claim_fees_no_platform"></a>

## Function `distribute_mydata_marketplace_claim_fees_no_platform`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_mydata_marketplace_claim_fees_no_platform">distribute_mydata_marketplace_claim_fees_no_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, claimant: <b>address</b>, gross_amount: u64, vault_balance: &<b>mut</b> <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_mydata_marketplace_claim_fees_no_platform">distribute_mydata_marketplace_claim_fees_no_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    treasury: &EcosystemTreasury,
    claimant: <b>address</b>,
    gross_amount: u64,
    vault_balance: &<b>mut</b> Balance&lt;MYSO&gt;,
    ctx: &<b>mut</b> TxContext,
): (u64, u64, u64) {
    <b>let</b> (platform_fee, ecosystem_fee, <b>mut</b> net_amount) = <a href="../social_contracts/mydata.md#social_contracts_mydata_calculate_mydata_marketplace_fees">calculate_mydata_marketplace_fees</a>(config, gross_amount);
    <b>let</b> <b>mut</b> payout_coin = coin::from_balance(balance::split(vault_balance, gross_amount), ctx);
    <b>if</b> (ecosystem_fee &gt; 0) {
        <b>let</b> eco_coin = coin::split(&<b>mut</b> payout_coin, ecosystem_fee, ctx);
        transfer::public_transfer(eco_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    net_amount = <b>if</b> (platform_fee &gt; 0) {
        <a href="../social_contracts/mydata.md#social_contracts_mydata_route_non_platform_platform_fee">route_non_platform_platform_fee</a>(
            config,
            treasury,
            platform_fee,
            net_amount,
            &<b>mut</b> payout_coin,
            ctx,
        )
    } <b>else</b> {
        net_amount
    };
    transfer::public_transfer(payout_coin, claimant);
    (platform_fee, ecosystem_fee, net_amount)
}
</code></pre>



</details>

<a name="social_contracts_mydata_distribute_mydata_marketplace_claim_fees_with_platform"></a>

## Function `distribute_mydata_marketplace_claim_fees_with_platform`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_mydata_marketplace_claim_fees_with_platform">distribute_mydata_marketplace_claim_fees_with_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, claimant: <b>address</b>, gross_amount: u64, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, vault_balance: &<b>mut</b> <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): (u64, u64, u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_mydata_marketplace_claim_fees_with_platform">distribute_mydata_marketplace_claim_fees_with_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    treasury: &EcosystemTreasury,
    claimant: <b>address</b>,
    gross_amount: u64,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    vault_balance: &<b>mut</b> Balance&lt;MYSO&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): (u64, u64, u64) {
    <b>let</b> (platform_fee, ecosystem_fee, net_amount) = <a href="../social_contracts/mydata.md#social_contracts_mydata_calculate_mydata_marketplace_fees">calculate_mydata_marketplace_fees</a>(config, gross_amount);
    <b>let</b> <b>mut</b> payout_coin = coin::from_balance(balance::split(vault_balance, gross_amount), ctx);
    <b>if</b> (ecosystem_fee &gt; 0) {
        <b>let</b> eco_coin = coin::split(&<b>mut</b> payout_coin, ecosystem_fee, ctx);
        transfer::public_transfer(eco_coin, <a href="../social_contracts/profile.md#social_contracts_profile_get_treasury_address">profile::get_treasury_address</a>(treasury));
    };
    <b>if</b> (platform_fee &gt; 0) {
        <b>let</b> <b>mut</b> platform_coin = coin::split(&<b>mut</b> payout_coin, platform_fee, ctx);
        <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">platform::add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> platform_coin, platform_fee, clock, ctx);
        coin::destroy_zero(platform_coin);
    };
    transfer::public_transfer(payout_coin, claimant);
    (platform_fee, ecosystem_fee, net_amount)
}
</code></pre>



</details>

<a name="social_contracts_mydata_claim_internal_no_platform"></a>

## Function `claim_internal_no_platform`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_claim_internal_no_platform">claim_internal_no_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, dist_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">social_contracts::mydata::DistributionRegistry</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, amount: u64, leaf_index: u64, proof: vector&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_claim_internal_no_platform">claim_internal_no_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    dist_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">DistributionRegistry</a>,
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    treasury: &EcosystemTreasury,
    snapshot_id: ID,
    amount: u64,
    leaf_index: u64,
    proof: vector&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>assert</b>!(vector::length(&proof) &lt;= config.max_merkle_proof_depth, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidProof">EPqInvalidProof</a>);
    <b>assert</b>!(table::contains(&dist_registry.rounds, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqDistributionNotFound">EPqDistributionNotFound</a>);
    <b>let</b> round = table::borrow(&dist_registry.rounds, snapshot_id);
    <b>assert</b>!(option::is_none(&round.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqPlatformMismatch">EPqPlatformMismatch</a>);
    <b>assert</b>!(clock::timestamp_ms(clock) &lt;= round.claim_deadline_ms, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqClaimExpired">EPqClaimExpired</a>);
    <b>assert</b>!(table::contains(&vault.merkle_roots, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqMerkleRootNotPublished">EPqMerkleRootNotPublished</a>);
    <b>assert</b>!(table::contains(&vault.snapshot_escrow, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSnapshotEscrowMissing">EPqSnapshotEscrowMissing</a>);
    <b>assert</b>!(*table::borrow(&vault.snapshot_escrow, snapshot_id) &gt;= amount, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqEscrowExceeded">EPqEscrowExceeded</a>);
    <b>let</b> claimant = tx_context::sender(ctx);
    <b>let</b> leaf = merkle::leaf_hash_with_platform(
        claimant,
        amount,
        object::id_to_bytes(&snapshot_id),
        option::none(),
    );
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
    <b>let</b> (platform_fee, ecosystem_fee, net_amount) = <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_mydata_marketplace_claim_fees_no_platform">distribute_mydata_marketplace_claim_fees_no_platform</a>(
        config,
        treasury,
        claimant,
        amount,
        &<b>mut</b> vault.balance,
        ctx,
    );
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_ClaimExecutedEvent">ClaimExecutedEvent</a> {
        snapshot_id,
        claimant,
        gross_amount: amount,
        platform_fee,
        ecosystem_fee,
        net_amount,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: option::none(),
        claimed_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_claim_internal_with_platform"></a>

## Function `claim_internal_with_platform`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_claim_internal_with_platform">claim_internal_with_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, dist_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">social_contracts::mydata::DistributionRegistry</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, amount: u64, leaf_index: u64, proof: vector&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_claim_internal_with_platform">claim_internal_with_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    dist_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">DistributionRegistry</a>,
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    snapshot_id: ID,
    amount: u64,
    leaf_index: u64,
    proof: vector&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidInput">EPqInvalidInput</a>);
    <b>assert</b>!(vector::length(&proof) &lt;= config.max_merkle_proof_depth, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqInvalidProof">EPqInvalidProof</a>);
    <b>assert</b>!(table::contains(&dist_registry.rounds, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqDistributionNotFound">EPqDistributionNotFound</a>);
    <b>let</b> round = table::borrow(&dist_registry.rounds, snapshot_id);
    <b>assert</b>!(option::is_some(&round.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqPlatformMismatch">EPqPlatformMismatch</a>);
    <b>assert</b>!(clock::timestamp_ms(clock) &lt;= round.claim_deadline_ms, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqClaimExpired">EPqClaimExpired</a>);
    <b>let</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a> = object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>));
    <b>assert</b>!(*option::borrow(&round.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>) == <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqPlatformMismatch">EPqPlatformMismatch</a>);
    <b>assert</b>!(table::contains(&vault.merkle_roots, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqMerkleRootNotPublished">EPqMerkleRootNotPublished</a>);
    <b>assert</b>!(table::contains(&vault.snapshot_escrow, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSnapshotEscrowMissing">EPqSnapshotEscrowMissing</a>);
    <b>assert</b>!(*table::borrow(&vault.snapshot_escrow, snapshot_id) &gt;= amount, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqEscrowExceeded">EPqEscrowExceeded</a>);
    <b>let</b> claimant = tx_context::sender(ctx);
    <b>let</b> leaf = merkle::leaf_hash_with_platform(
        claimant,
        amount,
        object::id_to_bytes(&snapshot_id),
        option::some(<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>),
    );
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
    <b>let</b> (platform_fee, ecosystem_fee, net_amount) = <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_mydata_marketplace_claim_fees_with_platform">distribute_mydata_marketplace_claim_fees_with_platform</a>(
        config,
        treasury,
        claimant,
        amount,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        &<b>mut</b> vault.balance,
        clock,
        ctx,
    );
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_ClaimExecutedEvent">ClaimExecutedEvent</a> {
        snapshot_id,
        claimant,
        gross_amount: amount,
        platform_fee,
        ecosystem_fee,
        net_amount,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: option::some(<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>),
        claimed_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_claim"></a>

## Function `claim`

Claim MyData marketplace pool payout from vault escrow (no platform).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_claim">claim</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, dist_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">social_contracts::mydata::DistributionRegistry</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, amount: u64, leaf_index: u64, proof: vector&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_claim">claim</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    dist_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">DistributionRegistry</a>,
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    treasury: &EcosystemTreasury,
    snapshot_id: ID,
    amount: u64,
    leaf_index: u64,
    proof: vector&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_claim_internal_no_platform">claim_internal_no_platform</a>(
        config,
        dist_registry,
        vault,
        treasury,
        snapshot_id,
        amount,
        leaf_index,
        proof,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_claim_with_platform"></a>

## Function `claim_with_platform`

Claim MyData marketplace pool payout with platform treasury routing.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_claim_with_platform">claim_with_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, dist_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">social_contracts::mydata::DistributionRegistry</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, amount: u64, leaf_index: u64, proof: vector&lt;vector&lt;u8&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_claim_with_platform">claim_with_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    dist_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">DistributionRegistry</a>,
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    snapshot_id: ID,
    amount: u64,
    leaf_index: u64,
    proof: vector&lt;vector&lt;u8&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_claim_internal_with_platform">claim_internal_with_platform</a>(
        config,
        dist_registry,
        vault,
        treasury,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        snapshot_id,
        amount,
        leaf_index,
        proof,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_reclaim_expired_snapshot_escrow"></a>

## Function `reclaim_expired_snapshot_escrow`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_reclaim_expired_snapshot_escrow">reclaim_expired_snapshot_escrow</a>(anchor_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">social_contracts::mydata::SnapshotAnchorRegistry</a>, dist_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">social_contracts::mydata::DistributionRegistry</a>, vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">social_contracts::mydata::MyDataClaimVault</a>, snapshot_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_reclaim_expired_snapshot_escrow">reclaim_expired_snapshot_escrow</a>(
    anchor_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotAnchorRegistry">SnapshotAnchorRegistry</a>,
    dist_registry: &<a href="../social_contracts/mydata.md#social_contracts_mydata_DistributionRegistry">DistributionRegistry</a>,
    vault: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataClaimVault">MyDataClaimVault</a>,
    snapshot_id: ID,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(table::contains(&anchor_registry.anchors, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqAnchorNotFound">EPqAnchorNotFound</a>);
    <b>assert</b>!(table::contains(&dist_registry.rounds, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqDistributionNotFound">EPqDistributionNotFound</a>);
    <b>assert</b>!(table::contains(&vault.snapshot_escrow, snapshot_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqSnapshotEscrowMissing">EPqSnapshotEscrowMissing</a>);
    <b>let</b> anchor = table::borrow(&anchor_registry.anchors, snapshot_id);
    <b>assert</b>!(tx_context::sender(ctx) == anchor.buyer_address, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>let</b> round = table::borrow(&dist_registry.rounds, snapshot_id);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(now &gt; round.claim_deadline_ms, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqClaimNotExpired">EPqClaimNotExpired</a>);
    <b>let</b> remaining = table::borrow_mut(&<b>mut</b> vault.snapshot_escrow, snapshot_id);
    <b>let</b> amount = *remaining;
    <b>assert</b>!(amount &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPqEscrowExceeded">EPqEscrowExceeded</a>);
    *remaining = 0;
    <b>let</b> refund = coin::from_balance(balance::split(&<b>mut</b> vault.balance, amount), ctx);
    transfer::public_transfer(refund, anchor.buyer_address);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_SnapshotEscrowReclaimedEvent">SnapshotEscrowReclaimedEvent</a> {
        snapshot_id,
        buyer_address: anchor.buyer_address,
        amount,
        reclaimed_at: now,
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

<a name="social_contracts_mydata_access_configuration"></a>

## Function `access_configuration`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): &<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">social_contracts::mydata::AccessConfiguration</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): &<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">AccessConfiguration</a> {
    &<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access
}
</code></pre>



</details>

<a name="social_contracts_mydata_requires_profile_subscription_access"></a>

## Function `requires_profile_subscription_access`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_requires_profile_subscription_access">requires_profile_subscription_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_requires_profile_subscription_access">requires_profile_subscription_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool {
    match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::ProfileSubscription =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_requires_marketplace_purchase"></a>

## Function `requires_marketplace_purchase`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_purchase">requires_marketplace_purchase</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_purchase">requires_marketplace_purchase</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool {
    match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::MarketplaceOneTime { .. } =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_requires_marketplace_subscription"></a>

## Function `requires_marketplace_subscription`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_subscription">requires_marketplace_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_subscription">requires_marketplace_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool {
    match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::MarketplaceRecurring { .. } =&gt; <b>true</b>,
        _ =&gt; <b>false</b>,
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_linked_one_time_price"></a>

## Function `linked_one_time_price`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_linked_one_time_price">linked_one_time_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_linked_one_time_price">linked_one_time_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; {
    match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::MarketplaceOneTime { price, .. } =&gt; option::some(*price),
        _ =&gt; option::none(),
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_access_configuration_kind"></a>

## Function `access_configuration_kind`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration_kind">access_configuration_kind</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration_kind">access_configuration_kind</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u8 {
    match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::ProfileSubscription =&gt; <a href="../social_contracts/mydata.md#social_contracts_mydata_ACCESS_KIND_PROFILE">ACCESS_KIND_PROFILE</a>,
        AccessConfiguration::MarketplaceOneTime { .. } =&gt; <a href="../social_contracts/mydata.md#social_contracts_mydata_ACCESS_KIND_ONE_TIME">ACCESS_KIND_ONE_TIME</a>,
        AccessConfiguration::MarketplaceRecurring { .. } =&gt; <a href="../social_contracts/mydata.md#social_contracts_mydata_ACCESS_KIND_RECURRING">ACCESS_KIND_RECURRING</a>,
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_validate_marketplace_price"></a>

## Function `validate_marketplace_price`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_marketplace_price">validate_marketplace_price</a>(price: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_marketplace_price">validate_marketplace_price</a>(price: u64) {
    <b>assert</b>!(price &gt; 0 && price &lt;= <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
}
</code></pre>



</details>

<a name="social_contracts_mydata_validate_recurring_duration"></a>

## Function `validate_recurring_duration`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_recurring_duration">validate_recurring_duration</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, duration_days: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_recurring_duration">validate_recurring_duration</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>, duration_days: u64): u64 {
    <b>let</b> sub_duration = <b>if</b> (duration_days == 0) { 30 } <b>else</b> { duration_days };
    <b>assert</b>!(sub_duration &lt;= config.max_subscription_days, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> duration_ms = (sub_duration <b>as</b> u128) * (<a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a> <b>as</b> u128);
    <b>assert</b>!(duration_ms &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
    sub_duration
}
</code></pre>



</details>

<a name="social_contracts_mydata_validate_access_configuration"></a>

## Function `validate_access_configuration`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_access_configuration">validate_access_configuration</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, access: &<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">social_contracts::mydata::AccessConfiguration</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_access_configuration">validate_access_configuration</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>, access: &<a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">AccessConfiguration</a>) {
    match (access) {
        AccessConfiguration::ProfileSubscription =&gt; {},
        AccessConfiguration::MarketplaceOneTime { price, .. } =&gt; <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_marketplace_price">validate_marketplace_price</a>(*price),
        AccessConfiguration::MarketplaceRecurring { price, duration_days, .. } =&gt; {
            <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_marketplace_price">validate_marketplace_price</a>(*price);
            <b>let</b> _ = <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_recurring_duration">validate_recurring_duration</a>(config, *duration_days);
        },
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_validate_optional_metadata"></a>

## Function `validate_optional_metadata`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_optional_metadata">validate_optional_metadata</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, value: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_optional_metadata">validate_optional_metadata</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>, value: &Option&lt;String&gt;) {
    <b>if</b> (option::is_some(value)) {
        <b>assert</b>!(string::length(option::borrow(value)) &lt;= config.max_metadata_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_mydata_validate_tags"></a>

## Function `validate_tags`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_tags">validate_tags</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: &vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_tags">validate_tags</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: &vector&lt;String&gt;) {
    <b>assert</b>!(vector::length(<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>) &lt;= config.max_tags, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; vector::length(<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>)) {
        <b>assert</b>!(string::length(vector::borrow(<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>, i)) &lt;= config.max_tag_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_mydata_emit_mydata_created_event"></a>

## Function `emit_mydata_created_event`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_emit_mydata_created_event">emit_mydata_created_event</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, ip_id: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_emit_mydata_created_event">emit_mydata_created_event</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>, ip_id: <b>address</b>) {
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataCreatedEvent">MyDataCreatedEvent</a> {
        ip_id,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration_kind">access_configuration_kind</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration_kind">access_configuration_kind</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_created_at">created_at</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_create"></a>

## Function `create`

Create new MyData data with proper MyData encryption


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create">create</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, encrypted_data: vector&lt;u8&gt;, encryption_id: vector&lt;u8&gt;, access: <a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">social_contracts::mydata::AccessConfiguration</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool, <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>
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
    access: <a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">AccessConfiguration</a>,
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
    <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_tags">validate_tags</a>(config, &<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>);
    <b>assert</b>!(string::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>) &gt; 0 && string::length(&<a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>) &lt;= config.max_metadata_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(!vector::is_empty(&encrypted_data), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(vector::length(&encrypted_data) &lt;= config.max_encrypted_data_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(!vector::is_empty(&encryption_id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(vector::length(&encryption_id) &lt;= config.max_encryption_id_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_optional_metadata">validate_optional_metadata</a>(config, &<a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_optional_metadata">validate_optional_metadata</a>(config, &<a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_optional_metadata">validate_optional_metadata</a>(config, &<a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_optional_metadata">validate_optional_metadata</a>(config, &<a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_access_configuration">validate_access_configuration</a>(config, &access);
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
        access,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a>: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_emit_mydata_created_event">emit_mydata_created_event</a>(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>, ip_id);
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>
}
</code></pre>



</details>

<a name="social_contracts_mydata_share_created_mydata"></a>

## Function `share_created_mydata`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_share_created_mydata">share_created_mydata</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_share_created_mydata">share_created_mydata</a>(registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>) {
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    table::add(&<b>mut</b> registry.ip_to_owner, ip_id, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    transfer::share_object(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>);
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_and_share_internal"></a>

## Function `create_and_share_internal`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_internal">create_and_share_internal</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, encrypted_data: vector&lt;u8&gt;, encryption_id: vector&lt;u8&gt;, access: <a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">social_contracts::mydata::AccessConfiguration</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool, <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_internal">create_and_share_internal</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: String,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: Option&lt;<b>address</b>&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: Option&lt;u64&gt;,
    encrypted_data: vector&lt;u8&gt;,
    encryption_id: vector&lt;u8&gt;,
    access: <a href="../social_contracts/mydata.md#social_contracts_mydata_AccessConfiguration">AccessConfiguration</a>,
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
        access,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>,
        clock,
        ctx,
    );
    <a href="../social_contracts/mydata.md#social_contracts_mydata_share_created_mydata">share_created_mydata</a>(registry, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>);
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_and_share_profile_subscription_mydata"></a>

## Function `create_and_share_profile_subscription_mydata`

Create and share profile-subscription-gated MyData (no marketplace pricing).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_profile_subscription_mydata">create_and_share_profile_subscription_mydata</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, encrypted_data: vector&lt;u8&gt;, encryption_id: vector&lt;u8&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool, <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_profile_subscription_mydata">create_and_share_profile_subscription_mydata</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: String,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: Option&lt;<b>address</b>&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: Option&lt;u64&gt;,
    encrypted_data: vector&lt;u8&gt;,
    encryption_id: vector&lt;u8&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_internal">create_and_share_internal</a>(
        config,
        registry,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>,
        encrypted_data,
        encryption_id,
        AccessConfiguration::ProfileSubscription,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_and_share_marketplace_one_time_mydata"></a>

## Function `create_and_share_marketplace_one_time_mydata`

Create and share marketplace one-time purchase MyData.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_marketplace_one_time_mydata">create_and_share_marketplace_one_time_mydata</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, encrypted_data: vector&lt;u8&gt;, encryption_id: vector&lt;u8&gt;, price: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool, <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_marketplace_one_time_mydata">create_and_share_marketplace_one_time_mydata</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: String,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: Option&lt;<b>address</b>&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: Option&lt;u64&gt;,
    encrypted_data: vector&lt;u8&gt;,
    encryption_id: vector&lt;u8&gt;,
    price: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_marketplace_price">validate_marketplace_price</a>(price);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_internal">create_and_share_internal</a>(
        config,
        registry,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>,
        encrypted_data,
        encryption_id,
        AccessConfiguration::MarketplaceOneTime {
            price,
            purchasers: table::new(ctx),
        },
        <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_create_and_share_marketplace_recurring_mydata"></a>

## Function `create_and_share_marketplace_recurring_mydata`

Create and share marketplace recurring subscription MyData.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_marketplace_recurring_mydata">create_and_share_marketplace_recurring_mydata</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">social_contracts::mydata::MyDataRegistry</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, encrypted_data: vector&lt;u8&gt;, encryption_id: vector&lt;u8&gt;, price: u64, duration_days: u64, <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool, <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_marketplace_recurring_mydata">create_and_share_marketplace_recurring_mydata</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataRegistry">MyDataRegistry</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>: String,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>: vector&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: Option&lt;<b>address</b>&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>: Option&lt;u64&gt;,
    encrypted_data: vector&lt;u8&gt;,
    encryption_id: vector&lt;u8&gt;,
    price: u64,
    duration_days: u64,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>: Option&lt;u64&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>: Option&lt;String&gt;,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>: bool,
    <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_marketplace_price">validate_marketplace_price</a>(price);
    <b>let</b> sub_duration = <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_recurring_duration">validate_recurring_duration</a>(config, duration_days);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_create_and_share_internal">create_and_share_internal</a>(
        config,
        registry,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_media_type">media_type</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_start">timestamp_start</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_timestamp_end">timestamp_end</a>,
        encrypted_data,
        encryption_id,
        AccessConfiguration::MarketplaceRecurring {
            price,
            duration_days: sub_duration,
            subscribers: table::new(ctx),
        },
        <a href="../social_contracts/mydata.md#social_contracts_mydata_geographic_region">geographic_region</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_data_quality">data_quality</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_sample_size">sample_size</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_collection_method">collection_method</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_is_updating">is_updating</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_update_frequency">update_frequency</a>,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_one_time_no_platform"></a>

## Function `purchase_one_time_no_platform`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time_no_platform">purchase_one_time_no_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time_no_platform">purchase_one_time_no_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    treasury: &EcosystemTreasury,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    <a href="../social_contracts/block_list.md#social_contracts_block_list_assert_not_blocked">block_list::assert_not_blocked</a>(block_list_registry, buyer, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_purchase">requires_marketplace_purchase</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>);
    <b>let</b> price = match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceOneTime { price, .. } =&gt; *price,
        _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
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
    <b>assert</b>!(coin::value(payment) &gt;= price, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPriceMismatch">EPriceMismatch</a>);
    match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceOneTime { purchasers, .. } =&gt; {
            <b>assert</b>!(!table::contains(purchasers, buyer), <a href="../social_contracts/mydata.md#social_contracts_mydata_EAlreadyPurchased">EAlreadyPurchased</a>);
            <b>assert</b>!(table::length(purchasers) &lt; config.max_paid_access_entries, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
        },
        _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
    <b>assert</b>!(buyer != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>);
    <b>let</b> price_coin = coin::split(payment, price, ctx);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) = <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_p2p_fees_no_platform">distribute_p2p_fees_no_platform</a>(
        config,
        treasury,
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        price_coin,
        ctx,
    );
    match (&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceOneTime { purchasers, .. } =&gt; {
            table::add(purchasers, buyer, <b>true</b>);
        },
        _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_PurchaseEvent">PurchaseEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        buyer,
        price,
        purchase_type: string::utf8(b"one_time"),
        timestamp: clock::timestamp_ms(clock),
        sub_agent_id,
        organization_id,
        platform_fee,
        ecosystem_fee,
        creator_amount,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_one_time_with_platform_internal"></a>

## Function `purchase_one_time_with_platform_internal`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time_with_platform_internal">purchase_one_time_with_platform_internal</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time_with_platform_internal">purchase_one_time_with_platform_internal</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_assert_platform_matches_listing">assert_platform_matches_listing</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    <a href="../social_contracts/block_list.md#social_contracts_block_list_assert_not_blocked">block_list::assert_not_blocked</a>(block_list_registry, buyer, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_purchase">requires_marketplace_purchase</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>);
    <b>let</b> price = match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceOneTime { price, .. } =&gt; *price,
        _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
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
    <b>assert</b>!(coin::value(payment) &gt;= price, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPriceMismatch">EPriceMismatch</a>);
    match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceOneTime { purchasers, .. } =&gt; {
            <b>assert</b>!(!table::contains(purchasers, buyer), <a href="../social_contracts/mydata.md#social_contracts_mydata_EAlreadyPurchased">EAlreadyPurchased</a>);
            <b>assert</b>!(table::length(purchasers) &lt; config.max_paid_access_entries, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
        },
        _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
    <b>assert</b>!(buyer != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>);
    <b>let</b> price_coin = coin::split(payment, price, ctx);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) = <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_p2p_fees_with_platform">distribute_p2p_fees_with_platform</a>(
        config,
        treasury,
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        price_coin,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        clock,
        ctx,
    );
    match (&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceOneTime { purchasers, .. } =&gt; {
            table::add(purchasers, buyer, <b>true</b>);
        },
        _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_PurchaseEvent">PurchaseEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        buyer,
        price,
        purchase_type: string::utf8(b"one_time"),
        timestamp: clock::timestamp_ms(clock),
        sub_agent_id,
        organization_id,
        platform_fee,
        ecosystem_fee,
        creator_amount,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: option::some(object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>))),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_one_time"></a>

## Function `purchase_one_time`

Purchase one-time access to MyData data.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time">purchase_one_time</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time">purchase_one_time</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    treasury: &EcosystemTreasury,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time_no_platform">purchase_one_time_no_platform</a>(
        config,
        block_list_registry,
        memory_config,
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>,
        treasury,
        payment,
        account,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_one_time_with_platform"></a>

## Function `purchase_one_time_with_platform`

Purchase one-time access with platform treasury routing.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time_with_platform">purchase_one_time_with_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time_with_platform">purchase_one_time_with_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_one_time_with_platform_internal">purchase_one_time_with_platform_internal</a>(
        config,
        block_list_registry,
        memory_config,
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>,
        treasury,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        payment,
        account,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_subscription_no_platform"></a>

## Function `purchase_subscription_no_platform`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription_no_platform">purchase_subscription_no_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription_no_platform">purchase_subscription_no_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    treasury: &EcosystemTreasury,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    <a href="../social_contracts/block_list.md#social_contracts_block_list_assert_not_blocked">block_list::assert_not_blocked</a>(block_list_registry, buyer, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_subscription">requires_marketplace_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>);
    <b>let</b> (price, duration_days) = match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceRecurring { price, duration_days, .. } =&gt; (*price, *duration_days),
        _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
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
    <b>assert</b>!(coin::value(payment) &gt;= price, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPriceMismatch">EPriceMismatch</a>);
    <b>assert</b>!(buyer != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>);
    <b>assert</b>!(duration_days &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(duration_days &lt;= config.max_subscription_days, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> duration_ms = (duration_days <b>as</b> u128) * (<a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a> <b>as</b> u128);
    <b>let</b> expiry_time = (current_time <b>as</b> u128) + duration_ms;
    <b>assert</b>!(expiry_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
    <b>let</b> expiry_time_u64 = expiry_time <b>as</b> u64;
    <b>let</b> price_coin = coin::split(payment, price, ctx);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) = <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_p2p_fees_no_platform">distribute_p2p_fees_no_platform</a>(
        config,
        treasury,
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        price_coin,
        ctx,
    );
    match (&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceRecurring { subscribers, .. } =&gt; {
            <b>if</b> (table::contains(subscribers, buyer)) {
                <b>let</b> current_expiry = table::remove(subscribers, buyer);
                <b>let</b> new_expiry = <b>if</b> (current_expiry &gt; current_time) {
                    <b>let</b> extended_time = (current_expiry <b>as</b> u128) + duration_ms;
                    <b>assert</b>!(extended_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
                    extended_time <b>as</b> u64
                } <b>else</b> {
                    expiry_time_u64
                };
                table::add(subscribers, buyer, new_expiry);
            } <b>else</b> {
                <b>assert</b>!(table::length(subscribers) &lt; config.max_paid_access_entries, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
                table::add(subscribers, buyer, expiry_time_u64);
            };
        },
        _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_PurchaseEvent">PurchaseEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        buyer,
        price,
        purchase_type: string::utf8(b"<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>"),
        timestamp: clock::timestamp_ms(clock),
        sub_agent_id,
        organization_id,
        platform_fee,
        ecosystem_fee,
        creator_amount,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_subscription_with_platform_internal"></a>

## Function `purchase_subscription_with_platform_internal`



<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription_with_platform_internal">purchase_subscription_with_platform_internal</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription_with_platform_internal">purchase_subscription_with_platform_internal</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(config.<a href="../social_contracts/mydata.md#social_contracts_mydata_marketplace_enabled">marketplace_enabled</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EDisabled">EDisabled</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_assert_platform_matches_listing">assert_platform_matches_listing</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>let</b> buyer = tx_context::sender(ctx);
    <a href="../social_contracts/block_list.md#social_contracts_block_list_assert_not_blocked">block_list::assert_not_blocked</a>(block_list_registry, buyer, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_subscription">requires_marketplace_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>);
    <b>let</b> (price, duration_days) = match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceRecurring { price, duration_days, .. } =&gt; (*price, *duration_days),
        _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
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
    <b>assert</b>!(coin::value(payment) &gt;= price, <a href="../social_contracts/mydata.md#social_contracts_mydata_EPriceMismatch">EPriceMismatch</a>);
    <b>assert</b>!(buyer != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>);
    <b>assert</b>!(duration_days &gt; 0, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(duration_days &lt;= config.max_subscription_days, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> duration_ms = (duration_days <b>as</b> u128) * (<a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a> <b>as</b> u128);
    <b>let</b> expiry_time = (current_time <b>as</b> u128) + duration_ms;
    <b>assert</b>!(expiry_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
    <b>let</b> expiry_time_u64 = expiry_time <b>as</b> u64;
    <b>let</b> price_coin = coin::split(payment, price, ctx);
    <b>let</b> (platform_fee, ecosystem_fee, creator_amount) = <a href="../social_contracts/mydata.md#social_contracts_mydata_distribute_p2p_fees_with_platform">distribute_p2p_fees_with_platform</a>(
        config,
        treasury,
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>,
        price_coin,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        clock,
        ctx,
    );
    match (&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceRecurring { subscribers, .. } =&gt; {
            <b>if</b> (table::contains(subscribers, buyer)) {
                <b>let</b> current_expiry = table::remove(subscribers, buyer);
                <b>let</b> new_expiry = <b>if</b> (current_expiry &gt; current_time) {
                    <b>let</b> extended_time = (current_expiry <b>as</b> u128) + duration_ms;
                    <b>assert</b>!(extended_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
                    extended_time <b>as</b> u64
                } <b>else</b> {
                    expiry_time_u64
                };
                table::add(subscribers, buyer, new_expiry);
            } <b>else</b> {
                <b>assert</b>!(table::length(subscribers) &lt; config.max_paid_access_entries, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
                table::add(subscribers, buyer, expiry_time_u64);
            };
        },
        _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_PurchaseEvent">PurchaseEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        buyer,
        price,
        purchase_type: string::utf8(b"<a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>"),
        timestamp: clock::timestamp_ms(clock),
        sub_agent_id,
        organization_id,
        platform_fee,
        ecosystem_fee,
        creator_amount,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>: option::some(object::uid_to_address(<a href="../social_contracts/platform.md#social_contracts_platform_id">platform::id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>))),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_subscription"></a>

## Function `purchase_subscription`

Purchase subscription access to MyData data.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription">purchase_subscription</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription">purchase_subscription</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    treasury: &EcosystemTreasury,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription_no_platform">purchase_subscription_no_platform</a>(
        config,
        block_list_registry,
        memory_config,
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>,
        treasury,
        payment,
        account,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_purchase_subscription_with_platform"></a>

## Function `purchase_subscription_with_platform`

Purchase subscription access with platform treasury routing.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription_with_platform">purchase_subscription_with_platform</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, payment: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription_with_platform">purchase_subscription_with_platform</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    treasury: &EcosystemTreasury,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> Platform,
    payment: &<b>mut</b> Coin&lt;MYSO&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/mydata.md#social_contracts_mydata_purchase_subscription_with_platform_internal">purchase_subscription_with_platform_internal</a>(
        config,
        block_list_registry,
        memory_config,
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>,
        treasury,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        payment,
        account,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_mydata_update_pricing"></a>

## Function `update_pricing`

Update pricing (owner only; marketplace listings only).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_pricing">update_pricing</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, new_one_time_price: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, new_subscription_price: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, new_subscription_duration_days: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_pricing">update_pricing</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    new_one_time_price: Option&lt;u64&gt;,
    new_subscription_price: Option&lt;u64&gt;,
    new_subscription_duration_days: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    match (&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceOneTime { price, .. } =&gt; {
            <b>assert</b>!(option::is_some(&new_one_time_price), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
            <b>let</b> price_val = *option::borrow(&new_one_time_price);
            <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_marketplace_price">validate_marketplace_price</a>(price_val);
            *price = price_val;
        },
        AccessConfiguration::MarketplaceRecurring { price, duration_days, .. } =&gt; {
            <b>assert</b>!(option::is_some(&new_subscription_price) || option::is_some(&new_subscription_duration_days), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
            <b>if</b> (option::is_some(&new_subscription_price)) {
                <b>let</b> price_val = *option::borrow(&new_subscription_price);
                <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_marketplace_price">validate_marketplace_price</a>(price_val);
                *price = price_val;
            };
            <b>if</b> (option::is_some(&new_subscription_duration_days)) {
                <b>let</b> duration = *option::borrow(&new_subscription_duration_days);
                *duration_days = <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_recurring_duration">validate_recurring_duration</a>(config, duration);
            };
        },
        AccessConfiguration::ProfileSubscription =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
    };
    <b>let</b> (<a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>) = match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::ProfileSubscription =&gt; (option::none(), option::none(), option::none()),
        AccessConfiguration::MarketplaceOneTime { price, .. } =&gt; (option::some(*price), option::none(), option::none()),
        AccessConfiguration::MarketplaceRecurring { price, duration_days, .. } =&gt; {
            (option::none(), option::some(*price), option::some(*duration_days))
        },
    };
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPricingUpdatedEvent">MyDataPricingUpdatedEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>,
        <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>,
        updated_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_update_content"></a>

## Function `update_content`

Update MyData content and metadata (owner only)


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_content">update_content</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, new_encrypted_data: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, new_encryption_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;u8&gt;&gt;, new_tags: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_update_content">update_content</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    new_encrypted_data: Option&lt;vector&lt;u8&gt;&gt;,
    new_encryption_id: Option&lt;vector&lt;u8&gt;&gt;,
    new_tags: Option&lt;vector&lt;String&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    // Check <a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> compatibility
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_version">version</a> == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>let</b> encrypted_data_updated = option::is_some(&new_encrypted_data);
    <b>let</b> encryption_id_updated = option::is_some(&new_encryption_id);
    <b>let</b> tags_updated = option::is_some(&new_tags);
    <b>assert</b>!(encrypted_data_updated == encryption_id_updated, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>assert</b>!(encrypted_data_updated || tags_updated, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>if</b> (encrypted_data_updated) {
        <b>let</b> data = option::borrow(&new_encrypted_data);
        <b>let</b> encryption_id = option::borrow(&new_encryption_id);
        <b>assert</b>!(!vector::is_empty(data) && vector::length(data) &lt;= config.max_encrypted_data_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
        <b>assert</b>!(!vector::is_empty(encryption_id) && vector::length(encryption_id) &lt;= config.max_encryption_id_bytes, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.encrypted_data = *option::borrow(&new_encrypted_data);
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.encryption_id = *option::borrow(&new_encryption_id);
    };
    <b>if</b> (tags_updated) {
        <a href="../social_contracts/mydata.md#social_contracts_mydata_validate_tags">validate_tags</a>(config, option::borrow(&new_tags));
        <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_tags">tags</a> = *option::borrow(&new_tags);
    };
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_last_updated">last_updated</a> = clock::timestamp_ms(clock);
    event::emit(<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataContentUpdatedEvent">MyDataContentUpdatedEvent</a> {
        ip_id: object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id),
        encrypted_data_updated,
        tags_updated,
        updated_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_mydata_assign_mydata_to_pools"></a>

## Function `assign_mydata_to_pools`

Assign MyData to sub-pools (owner only).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_assign_mydata_to_pools">assign_mydata_to_pools</a>(config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">social_contracts::mydata::MyDataConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, pool_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">social_contracts::mydata::MyDataPoolRegistry</a>, sub_pool_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_assign_mydata_to_pools">assign_mydata_to_pools</a>(
    config: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataConfig">MyDataConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    pool_registry: &<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_MyDataPoolRegistry">MyDataPoolRegistry</a>,
    sub_pool_ids: vector&lt;ID&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_EUnauthorized">EUnauthorized</a>);
    <b>let</b> ip_id = object::uid_to_address(&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.id);
    <a href="../social_contracts/mydata.md#social_contracts_mydata_assign_mydata_to_sub_pools">assign_mydata_to_sub_pools</a>(config, pool_registry, ip_id, sub_pool_ids, clock);
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
    <b>if</b> (user == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>) <b>return</b> <b>true</b>;
    match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::ProfileSubscription =&gt; <b>false</b>,
        AccessConfiguration::MarketplaceOneTime { purchasers, .. } =&gt; {
            table::contains(purchasers, user)
        },
        AccessConfiguration::MarketplaceRecurring { subscribers, .. } =&gt; {
            <b>if</b> (!table::contains(subscribers, user)) <b>return</b> <b>false</b>;
            <b>let</b> expiry = *table::borrow(subscribers, user);
            <b>let</b> current_time = clock::timestamp_ms(clock);
            current_time &lt;= expiry
        },
    }
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

Key-server policy hook for <code>fetch_key</code>: marketplace listings only.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve">mydata_approve</a>(id: vector&lt;u8&gt;, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve">mydata_approve</a>(
    id: vector&lt;u8&gt;,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata_encryption_id_matches">encryption_id_matches</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>, &id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyIdMismatch">EPolicyIdMismatch</a>);
    <b>assert</b>!(
        <a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_purchase">requires_marketplace_purchase</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>) || <a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_subscription">requires_marketplace_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>),
        <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyNotEntitled">EPolicyNotEntitled</a>,
    );
    <b>let</b> sender = tx_context::sender(ctx);
    <b>if</b> (sender != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>) {
        <a href="../social_contracts/block_list.md#social_contracts_block_list_assert_not_blocked">block_list::assert_not_blocked</a>(block_list_registry, sender, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    };
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

<a name="social_contracts_mydata_mydata_approve_profile_subscription"></a>

## Function `mydata_approve_profile_subscription`

Key-server policy hook for profile-subscription-gated MyData linked to a post.
<code>id</code> is first so key-server <code>ValidPtb</code> can extract the encryption identity from arg 0.
<code>post_service_id</code> / <code>post_linked_mydata_id</code> come from the linked post's [<code>PostAccess</code>] fields.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve_profile_subscription">mydata_approve_profile_subscription</a>(id: vector&lt;u8&gt;, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, post_service_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, post_linked_mydata_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, post_min_tier_level: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, service: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscriptionService">social_contracts::subscription::ProfileSubscriptionService</a>, <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &<a href="../social_contracts/subscription.md#social_contracts_subscription_ProfileSubscription">social_contracts::subscription::ProfileSubscription</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_mydata_approve_profile_subscription">mydata_approve_profile_subscription</a>(
    id: vector&lt;u8&gt;,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    post_service_id: ID,
    post_linked_mydata_id: ID,
    post_min_tier_level: Option&lt;u64&gt;,
    memory_config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>,
    <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>,
    service: &ProfileSubscriptionService,
    <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>: &ProfileSubscription,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata_encryption_id_matches">encryption_id_matches</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>, &id), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyIdMismatch">EPolicyIdMismatch</a>);
    <b>assert</b>!(<a href="../social_contracts/mydata.md#social_contracts_mydata_requires_profile_subscription_access">requires_profile_subscription_access</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyNotEntitled">EPolicyNotEntitled</a>);
    <b>assert</b>!(post_linked_mydata_id == object::id(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyNotEntitled">EPolicyNotEntitled</a>);
    <b>assert</b>!(post_service_id == object::id(service), <a href="../social_contracts/mydata.md#social_contracts_mydata_EPolicyNotEntitled">EPolicyNotEntitled</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>if</b> (sender == <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>) {
        <b>return</b>
    };
    <a href="../social_contracts/block_list.md#social_contracts_block_list_assert_not_blocked">block_list::assert_not_blocked</a>(block_list_registry, sender, <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>);
    <b>let</b> content_platform_id = <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_platform_id">platform_id</a>;
    <b>if</b> (<a href="../social_contracts/subscription.md#social_contracts_subscription_subscription_satisfies_access">subscription::subscription_satisfies_access</a>(
        <a href="../social_contracts/subscription.md#social_contracts_subscription">subscription</a>,
        service,
        sender,
        post_min_tier_level,
        content_platform_id,
        clock,
    )) {
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
    <b>assert</b>!(user != <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.<a href="../social_contracts/mydata.md#social_contracts_mydata_owner">owner</a>, <a href="../social_contracts/mydata.md#social_contracts_mydata_ESelfPurchase">ESelfPurchase</a>);
    <b>let</b> total_grants = match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::ProfileSubscription =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
        AccessConfiguration::MarketplaceOneTime { purchasers, .. } =&gt; table::length(purchasers),
        AccessConfiguration::MarketplaceRecurring { subscribers, .. } =&gt; table::length(subscribers),
    };
    <b>assert</b>!(total_grants &lt; config.max_free_access_grants, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
    <b>if</b> (access_type == 0) {
        match (&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
            AccessConfiguration::MarketplaceOneTime { purchasers, .. } =&gt; {
                <b>if</b> (!table::contains(purchasers, user)) {
                    table::add(purchasers, user, <b>true</b>);
                };
            },
            _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
        };
    } <b>else</b> {
        <b>let</b> duration_days = <b>if</b> (option::is_some(&subscription_days)) {
            <b>let</b> days = *option::borrow(&subscription_days);
            <b>assert</b>!(days &gt; 0 && days &lt;= config.max_subscription_days, <a href="../social_contracts/mydata.md#social_contracts_mydata_EInvalidInput">EInvalidInput</a>);
            days
        } <b>else</b> {
            match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
                AccessConfiguration::MarketplaceRecurring { duration_days, .. } =&gt; *duration_days,
                _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
            }
        };
        <b>let</b> current_time = clock::timestamp_ms(clock);
        <b>let</b> duration_ms = (duration_days <b>as</b> u128) * (<a href="../social_contracts/mydata.md#social_contracts_mydata_MILLISECONDS_PER_DAY">MILLISECONDS_PER_DAY</a> <b>as</b> u128);
        <b>let</b> expiry_time = (current_time <b>as</b> u128) + duration_ms;
        <b>assert</b>!(expiry_time &lt;= (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128), <a href="../social_contracts/mydata.md#social_contracts_mydata_EOverflow">EOverflow</a>);
        <b>let</b> expiry_time_u64 = expiry_time <b>as</b> u64;
        match (&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
            AccessConfiguration::MarketplaceRecurring { subscribers, .. } =&gt; {
                <b>if</b> (table::contains(subscribers, user)) {
                    table::remove(subscribers, user);
                };
                table::add(subscribers, user, expiry_time_u64);
            },
            _ =&gt; <b>abort</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_ENotForSale">ENotForSale</a>,
        };
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

Revoke a buyer's access (owner only). Removes the user from marketplace access tables.
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
        match (&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
            AccessConfiguration::MarketplaceOneTime { purchasers, .. } =&gt; {
                <b>if</b> (table::contains(purchasers, user)) {
                    table::remove(purchasers, user);
                    revoked_one_time = <b>true</b>;
                };
            },
            _ =&gt; {},
        };
    };
    <b>if</b> (access_type == 1 || access_type == 2) {
        match (&<b>mut</b> <a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
            AccessConfiguration::MarketplaceRecurring { subscribers, .. } =&gt; {
                <b>if</b> (table::contains(subscribers, user)) {
                    table::remove(subscribers, user);
                    revoked_subscription = <b>true</b>;
                };
            },
            _ =&gt; {},
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


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_one_time_price">one_time_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; { <a href="../social_contracts/mydata.md#social_contracts_mydata_linked_one_time_price">linked_one_time_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>) }
</code></pre>



</details>

<a name="social_contracts_mydata_subscription_price"></a>

## Function `subscription_price`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_price">subscription_price</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): Option&lt;u64&gt; {
    match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::MarketplaceRecurring { price, .. } =&gt; option::some(*price),
        _ =&gt; option::none(),
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_subscription_duration_days"></a>

## Function `subscription_duration_days`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscription_duration_days">subscription_duration_days</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 {
    match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::MarketplaceRecurring { duration_days, .. } =&gt; *duration_days,
        _ =&gt; 0,
    }
}
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


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_purchaser_count">purchaser_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 {
    match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::MarketplaceOneTime { purchasers, .. } =&gt; table::length(purchasers),
        _ =&gt; 0,
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_subscriber_count"></a>

## Function `subscriber_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscriber_count">subscriber_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_subscriber_count">subscriber_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): u64 {
    match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::MarketplaceRecurring { subscribers, .. } =&gt; table::length(subscribers),
        _ =&gt; 0,
    }
}
</code></pre>



</details>

<a name="social_contracts_mydata_is_one_time_for_sale"></a>

## Function `is_one_time_for_sale`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_one_time_for_sale">is_one_time_for_sale</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_one_time_for_sale">is_one_time_for_sale</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool { <a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_purchase">requires_marketplace_purchase</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>) }
</code></pre>



</details>

<a name="social_contracts_mydata_is_subscription_available"></a>

## Function `is_subscription_available`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_subscription_available">is_subscription_available</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">social_contracts::mydata::MyData</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/mydata.md#social_contracts_mydata_is_subscription_available">is_subscription_available</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>: &<a href="../social_contracts/mydata.md#social_contracts_mydata_MyData">MyData</a>): bool { <a href="../social_contracts/mydata.md#social_contracts_mydata_requires_marketplace_subscription">requires_marketplace_subscription</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>) }
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
    match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceRecurring { subscribers, .. } =&gt; {
            <b>if</b> (!table::contains(subscribers, user)) <b>return</b> <b>false</b>;
            <b>let</b> expiry = *table::borrow(subscribers, user);
            <b>let</b> current_time = clock::timestamp_ms(clock);
            current_time &lt;= expiry
        },
        _ =&gt; <b>false</b>,
    }
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
    match (&<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>.access) {
        AccessConfiguration::MarketplaceRecurring { subscribers, .. } =&gt; {
            <b>if</b> (table::contains(subscribers, user)) {
                option::some(*table::borrow(subscribers, user))
            } <b>else</b> {
                option::none()
            }
        },
        _ =&gt; option::none(),
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
    <b>let</b> one_time_revenue = match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::MarketplaceOneTime { price, purchasers, .. } =&gt; {
            <b>let</b> count = table::length(purchasers);
            <b>let</b> revenue = (*price <b>as</b> u128) * (count <b>as</b> u128);
            <b>if</b> (revenue &gt; (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128)) {
                <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>
            } <b>else</b> {
                revenue <b>as</b> u64
            }
        },
        _ =&gt; 0,
    };
    <b>let</b> subscription_revenue = match (<a href="../social_contracts/mydata.md#social_contracts_mydata_access_configuration">access_configuration</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>)) {
        AccessConfiguration::MarketplaceRecurring { price, subscribers, .. } =&gt; {
            <b>let</b> count = table::length(subscribers);
            <b>let</b> revenue = (*price <b>as</b> u128) * (count <b>as</b> u128);
            <b>if</b> (revenue &gt; (<a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a> <b>as</b> u128)) {
                <a href="../social_contracts/mydata.md#social_contracts_mydata_MAX_U64">MAX_U64</a>
            } <b>else</b> {
                revenue <b>as</b> u64
            }
        },
        _ =&gt; 0,
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
    <a href="../social_contracts/mydata.md#social_contracts_mydata_purchaser_count">purchaser_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>) &gt; 0 || <a href="../social_contracts/mydata.md#social_contracts_mydata_subscriber_count">subscriber_count</a>(<a href="../social_contracts/mydata.md#social_contracts_mydata">mydata</a>) &gt; 0
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
    config.max_encrypted_data_bytes = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_ENCRYPTED_DATA_BYTES">DEFAULT_MAX_ENCRYPTED_DATA_BYTES</a>;
    config.max_tag_bytes = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_TAG_BYTES">DEFAULT_MAX_TAG_BYTES</a>;
    config.max_metadata_bytes = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_METADATA_BYTES">DEFAULT_MAX_METADATA_BYTES</a>;
    config.max_payment_reference_bytes = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_PAYMENT_REFERENCE_BYTES">DEFAULT_MAX_PAYMENT_REFERENCE_BYTES</a>;
    config.max_pool_assignments = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_POOL_ASSIGNMENTS">DEFAULT_MAX_POOL_ASSIGNMENTS</a>;
    config.max_merkle_proof_depth = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_MERKLE_PROOF_DEPTH">DEFAULT_MAX_MERKLE_PROOF_DEPTH</a>;
    config.max_paid_access_entries = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MAX_PAID_ACCESS_ENTRIES">DEFAULT_MAX_PAID_ACCESS_ENTRIES</a>;
    config.default_claim_window_ms = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_CLAIM_WINDOW_MS">DEFAULT_CLAIM_WINDOW_MS</a>;
    config.p2p_platform_fee_bps = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_P2P_PLATFORM_FEE_BPS">DEFAULT_P2P_PLATFORM_FEE_BPS</a>;
    config.p2p_ecosystem_fee_bps = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_P2P_ECOSYSTEM_FEE_BPS">DEFAULT_P2P_ECOSYSTEM_FEE_BPS</a>;
    config.mydata_marketplace_platform_fee_bps = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS">DEFAULT_MYDATA_MARKETPLACE_PLATFORM_FEE_BPS</a>;
    config.mydata_marketplace_ecosystem_fee_bps = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS">DEFAULT_MYDATA_MARKETPLACE_ECOSYSTEM_FEE_BPS</a>;
    config.non_platform_platform_to_creator_bps = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_CREATOR_BPS</a>;
    config.non_platform_platform_to_treasury_bps = <a href="../social_contracts/mydata.md#social_contracts_mydata_DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS">DEFAULT_NON_PLATFORM_PLATFORM_TO_TREASURY_BPS</a>;
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
