---
title: Module `social_contracts::platform`
---

Platform module for the MySocial network
Manages social media platforms and their timelines


-  [Struct `PlatformStatus`](#social_contracts_platform_PlatformStatus)
-  [Struct `PlatformAdminCap`](#social_contracts_platform_PlatformAdminCap)
-  [Struct `PlatformPackage`](#social_contracts_platform_PlatformPackage)
-  [Struct `ModeratorsGroupTag`](#social_contracts_platform_ModeratorsGroupTag)
-  [Struct `PlatformBlockAdmin`](#social_contracts_platform_PlatformBlockAdmin)
-  [Struct `PlatformBadgeAdmin`](#social_contracts_platform_PlatformBadgeAdmin)
-  [Struct `PlatformTreasuryAdmin`](#social_contracts_platform_PlatformTreasuryAdmin)
-  [Struct `PlatformContentModerator`](#social_contracts_platform_PlatformContentModerator)
-  [Struct `PlatformPromotionAdmin`](#social_contracts_platform_PlatformPromotionAdmin)
-  [Struct `Platform`](#social_contracts_platform_Platform)
-  [Struct `PlatformRegistry`](#social_contracts_platform_PlatformRegistry)
-  [Struct `PlatformCreatedEvent`](#social_contracts_platform_PlatformCreatedEvent)
-  [Struct `PlatformUpdatedEvent`](#social_contracts_platform_PlatformUpdatedEvent)
-  [Struct `ModeratorPermissionsGrantedEvent`](#social_contracts_platform_ModeratorPermissionsGrantedEvent)
-  [Struct `ModeratorPermissionsRevokedEvent`](#social_contracts_platform_ModeratorPermissionsRevokedEvent)
-  [Struct `ModeratorRemovedEvent`](#social_contracts_platform_ModeratorRemovedEvent)
-  [Struct `PlatformApprovalChangedEvent`](#social_contracts_platform_PlatformApprovalChangedEvent)
-  [Struct `PlatformDeletedEvent`](#social_contracts_platform_PlatformDeletedEvent)
-  [Struct `UserJoinedPlatformEvent`](#social_contracts_platform_UserJoinedPlatformEvent)
-  [Struct `UserLeftPlatformEvent`](#social_contracts_platform_UserLeftPlatformEvent)
-  [Struct `TokenAirdropEvent`](#social_contracts_platform_TokenAirdropEvent)
-  [Struct `TreasuryFundedEvent`](#social_contracts_platform_TreasuryFundedEvent)
-  [Constants](#@Constants_0)
-  [Function `bootstrap_init`](#social_contracts_platform_bootstrap_init)
-  [Function `create_platform`](#social_contracts_platform_create_platform)
-  [Function `update_platform`](#social_contracts_platform_update_platform)
-  [Function `platform_version`](#social_contracts_platform_platform_version)
-  [Function `borrow_platform_version_mut`](#social_contracts_platform_borrow_platform_version_mut)
-  [Function `registry_version`](#social_contracts_platform_registry_version)
-  [Function `borrow_registry_version_mut`](#social_contracts_platform_borrow_registry_version_mut)
-  [Function `add_to_treasury`](#social_contracts_platform_add_to_treasury)
-  [Function `mark_platform_governance_proposal_implemented`](#social_contracts_platform_mark_platform_governance_proposal_implemented)
-  [Function `delegate_vote_on_platform_governance_proposal`](#social_contracts_platform_delegate_vote_on_platform_governance_proposal)
-  [Function `finalize_platform_governance_proposal`](#social_contracts_platform_finalize_platform_governance_proposal)
-  [Function `finalize_platform_governance_proposal_anonymous`](#social_contracts_platform_finalize_platform_governance_proposal_anonymous)
-  [Function `reject_platform_governance_proposal_manually`](#social_contracts_platform_reject_platform_governance_proposal_manually)
-  [Function `grant_moderator_permission`](#social_contracts_platform_grant_moderator_permission)
-  [Function `grant_all_moderator_permissions`](#social_contracts_platform_grant_all_moderator_permissions)
-  [Function `revoke_moderator_permission`](#social_contracts_platform_revoke_moderator_permission)
-  [Function `add_moderator`](#social_contracts_platform_add_moderator)
-  [Function `remove_moderator`](#social_contracts_platform_remove_moderator)
-  [Function `block_wallet`](#social_contracts_platform_block_wallet)
-  [Function `unblock_wallet`](#social_contracts_platform_unblock_wallet)
-  [Function `toggle_platform_approval`](#social_contracts_platform_toggle_platform_approval)
-  [Function `delete_platform`](#social_contracts_platform_delete_platform)
-  [Function `new_status`](#social_contracts_platform_new_status)
-  [Function `status_value`](#social_contracts_platform_status_value)
-  [Function `validate_cover_photo`](#social_contracts_platform_validate_cover_photo)
-  [Function `validate_media_previews`](#social_contracts_platform_validate_media_previews)
-  [Function `is_valid_category`](#social_contracts_platform_is_valid_category)
-  [Function `join_platform`](#social_contracts_platform_join_platform)
-  [Function `leave_platform`](#social_contracts_platform_leave_platform)
-  [Function `is_approved`](#social_contracts_platform_is_approved)
-  [Function `has_joined_platform`](#social_contracts_platform_has_joined_platform)
-  [Function `is_developer`](#social_contracts_platform_is_developer)
-  [Function `has_moderator_permission`](#social_contracts_platform_has_moderator_permission)
-  [Function `is_moderator`](#social_contracts_platform_is_moderator)
-  [Function `is_developer_or_moderator`](#social_contracts_platform_is_developer_or_moderator)
-  [Function `moderators_group_id`](#social_contracts_platform_moderators_group_id)
-  [Function `assert_moderator_permission`](#social_contracts_platform_assert_moderator_permission)
-  [Function `grant_all_extension_permissions`](#social_contracts_platform_grant_all_extension_permissions)
-  [Function `all_extension_permission_names`](#social_contracts_platform_all_extension_permission_names)
-  [Function `permission_name`](#social_contracts_platform_permission_name)
-  [Function `has_any_extension_permission`](#social_contracts_platform_has_any_extension_permission)
-  [Function `emit_permissions_granted`](#social_contracts_platform_emit_permissions_granted)
-  [Function `emit_permissions_revoked`](#social_contracts_platform_emit_permissions_revoked)
-  [Function `maybe_emit_moderator_removed`](#social_contracts_platform_maybe_emit_moderator_removed)
-  [Function `name`](#social_contracts_platform_name)
-  [Function `tagline`](#social_contracts_platform_tagline)
-  [Function `description`](#social_contracts_platform_description)
-  [Function `logo`](#social_contracts_platform_logo)
-  [Function `cover_photo`](#social_contracts_platform_cover_photo)
-  [Function `media_previews`](#social_contracts_platform_media_previews)
-  [Function `developer`](#social_contracts_platform_developer)
-  [Function `terms_of_service`](#social_contracts_platform_terms_of_service)
-  [Function `privacy_policy`](#social_contracts_platform_privacy_policy)
-  [Function `get_platforms`](#social_contracts_platform_get_platforms)
-  [Function `get_links`](#social_contracts_platform_get_links)
-  [Function `primary_category`](#social_contracts_platform_primary_category)
-  [Function `secondary_category`](#social_contracts_platform_secondary_category)
-  [Function `status`](#social_contracts_platform_status)
-  [Function `release_date`](#social_contracts_platform_release_date)
-  [Function `shutdown_date`](#social_contracts_platform_shutdown_date)
-  [Function `created_at`](#social_contracts_platform_created_at)
-  [Function `treasury_balance`](#social_contracts_platform_treasury_balance)
-  [Function `id`](#social_contracts_platform_id)
-  [Function `get_platform_by_name`](#social_contracts_platform_get_platform_by_name)
-  [Function `get_platforms_by_developer`](#social_contracts_platform_get_platforms_by_developer)
-  [Function `wants_dao_governance`](#social_contracts_platform_wants_dao_governance)
-  [Function `governance_registry_id`](#social_contracts_platform_governance_registry_id)
-  [Function `governance_parameters`](#social_contracts_platform_governance_parameters)
-  [Function `update_platform_governance`](#social_contracts_platform_update_platform_governance)
-  [Function `airdrop_from_treasury`](#social_contracts_platform_airdrop_from_treasury)
-  [Function `assign_badge`](#social_contracts_platform_assign_badge)
-  [Function `revoke_badge`](#social_contracts_platform_revoke_badge)
-  [Function `add_moderator_register`](#social_contracts_platform_add_moderator_register)
-  [Function `remove_moderator_unregister`](#social_contracts_platform_remove_moderator_unregister)
-  [Function `create_platform_admin_cap`](#social_contracts_platform_create_platform_admin_cap)
-  [Function `migrate_platform`](#social_contracts_platform_migrate_platform)
-  [Function `migrate_registry`](#social_contracts_platform_migrate_registry)


<pre><code><b>use</b> <a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption">mydata::bf_hmac_encryption</a>;
<b>use</b> <a href="../mydata/gf256.md#mydata_gf256">mydata::gf256</a>;
<b>use</b> <a href="../mydata/hmac256ctr.md#mydata_hmac256ctr">mydata::hmac256ctr</a>;
<b>use</b> <a href="../mydata/kdf.md#mydata_kdf">mydata::kdf</a>;
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
<b>use</b> <a href="../social_contracts/block_list.md#social_contracts_block_list">social_contracts::block_list</a>;
<b>use</b> <a href="../social_contracts/governance.md#social_contracts_governance">social_contracts::governance</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
<b>use</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_contracts::social_graph</a>;
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



<a name="social_contracts_platform_PlatformStatus"></a>

## Struct `PlatformStatus`

Platform status enum


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">PlatformStatus</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformAdminCap"></a>

## Struct `PlatformAdminCap`

Admin capability for Platform system management


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">PlatformAdminCap</a> <b>has</b> key, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformPackage"></a>

## Struct `PlatformPackage`

Package witness for <code>PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;</code>


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_platform_ModeratorsGroupTag"></a>

## Struct `ModeratorsGroupTag`

Derivation key for the per-platform moderators permission group


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_ModeratorsGroupTag">ModeratorsGroupTag</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_platform_PlatformBlockAdmin"></a>

## Struct `PlatformBlockAdmin`

Permission to block/unblock wallets on behalf of the platform


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformBlockAdmin">PlatformBlockAdmin</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_platform_PlatformBadgeAdmin"></a>

## Struct `PlatformBadgeAdmin`

Permission to assign/revoke platform badges


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformBadgeAdmin">PlatformBadgeAdmin</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_platform_PlatformTreasuryAdmin"></a>

## Struct `PlatformTreasuryAdmin`

Permission to airdrop from the platform treasury


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformTreasuryAdmin">PlatformTreasuryAdmin</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_platform_PlatformContentModerator"></a>

## Struct `PlatformContentModerator`

Permission to moderate posts and comments on the platform


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformContentModerator">PlatformContentModerator</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_platform_PlatformPromotionAdmin"></a>

## Struct `PlatformPromotionAdmin`

Permission to manage promoted posts and view payouts


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformPromotionAdmin">PlatformPromotionAdmin</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_platform_Platform"></a>

## Struct `Platform`

Platform object that contains information about a social media platform


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform name
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform tagline
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform description
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform logo URL
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Optional cover photo URL
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;</code>
</dt>
<dd>
 Optional screenshot/video preview URLs
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>: <b>address</b></code>
</dt>
<dd>
 Platform developer address
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform terms of service URL
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform privacy policy URL
</dd>
<dt>
<code>platforms: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Platform names
</dd>
<dt>
<code>links: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Platform URLs
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Primary platform category
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Secondary platform category (optional)
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">social_contracts::platform::PlatformStatus</a></code>
</dt>
<dd>
 Platform status
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
 Platform release date
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
 Platform shutdown date (optional)
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_created_at">created_at</a>: u64</code>
</dt>
<dd>
 Creation timestamp
</dd>
<dt>
<code>treasury: <a href="../myso/balance.md#myso_balance_Balance">myso::balance::Balance</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;</code>
</dt>
<dd>
 Platform-specific MYSO tokens treasury
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>: bool</code>
</dt>
<dd>
 Whether the platform wants DAO governance
</dd>
<dt>
<code>delegate_count: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
 DAO governance configuration parameters (all optional)
</dd>
<dt>
<code>delegate_term_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_submission_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>max_votes_per_user: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>quadratic_base_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_period_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum_votes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
 ID of governance registry if created
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformRegistry"></a>

## Struct `PlatformRegistry`

Platform registry that keeps track of all platforms


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a> <b>has</b> key
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: <a href="../myso/object.md#myso_object_UID">myso::object::UID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>platforms_by_name: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <b>address</b>&gt;</code>
</dt>
<dd>
 Table mapping platform names to platform IDs
</dd>
<dt>
<code>platforms_by_developer: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, vector&lt;<b>address</b>&gt;&gt;</code>
</dt>
<dd>
 Table mapping developer addresses to their platforms
</dd>
<dt>
<code>platform_approvals: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, bool&gt;</code>
</dt>
<dd>
 Table mapping platform IDs to their approval status (admin-controlled)
</dd>
<dt>
<code>version: u64</code>
</dt>
<dd>
 Version for upgrades
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformCreatedEvent"></a>

## Struct `PlatformCreatedEvent`

Platform created event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformCreatedEvent">PlatformCreatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>platforms: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>links: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">social_contracts::platform::PlatformStatus</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>: bool</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_count: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>delegate_term_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>proposal_submission_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>max_votes_per_user: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>quadratic_base_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>voting_period_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>quorum_votes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformUpdatedEvent"></a>

## Struct `PlatformUpdatedEvent`

Platform updated event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformUpdatedEvent">PlatformUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>platforms: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>links: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">social_contracts::platform::PlatformStatus</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>updated_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_ModeratorPermissionsGrantedEvent"></a>

## Struct `ModeratorPermissionsGrantedEvent`

Emitted when one or more moderator permissions are granted


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_ModeratorPermissionsGrantedEvent">ModeratorPermissionsGrantedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>permissions: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>granted_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_ModeratorPermissionsRevokedEvent"></a>

## Struct `ModeratorPermissionsRevokedEvent`

Emitted when one or more moderator permissions are revoked


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_ModeratorPermissionsRevokedEvent">ModeratorPermissionsRevokedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>permissions: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>revoked_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_ModeratorRemovedEvent"></a>

## Struct `ModeratorRemovedEvent`

Emitted when a member is fully removed from the moderators group


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_ModeratorRemovedEvent">ModeratorRemovedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>removed_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformApprovalChangedEvent"></a>

## Struct `PlatformApprovalChangedEvent`

Platform approval status changed event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformApprovalChangedEvent">PlatformApprovalChangedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>approved: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>changed_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_PlatformDeletedEvent"></a>

## Struct `PlatformDeletedEvent`

Platform deleted event


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformDeletedEvent">PlatformDeletedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>deleted_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_platform_UserJoinedPlatformEvent"></a>

## Struct `UserJoinedPlatformEvent`

Event emitted when a user joins a platform


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_UserJoinedPlatformEvent">UserJoinedPlatformEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>wallet_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
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

<a name="social_contracts_platform_UserLeftPlatformEvent"></a>

## Struct `UserLeftPlatformEvent`

Event emitted when a user leaves a platform


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_UserLeftPlatformEvent">UserLeftPlatformEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>wallet_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
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

<a name="social_contracts_platform_TokenAirdropEvent"></a>

## Struct `TokenAirdropEvent`

Event emitted when tokens are airdropped from the platform treasury


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_TokenAirdropEvent">TokenAirdropEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>recipient: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>reason_code: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>executed_by: <b>address</b></code>
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

<a name="social_contracts_platform_TreasuryFundedEvent"></a>

## Struct `TreasuryFundedEvent`

Event emitted when tokens are added to platform treasury


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/platform.md#social_contracts_platform_TreasuryFundedEvent">TreasuryFundedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>platform_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>funded_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_balance: u64</code>
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


<a name="social_contracts_platform_EUnauthorized"></a>

Error codes


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>: u64 = 0;
</code></pre>



<a name="social_contracts_platform_EPlatformAlreadyExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EPlatformAlreadyExists">EPlatformAlreadyExists</a>: u64 = 1;
</code></pre>



<a name="social_contracts_platform_EInvalidTokenAmount"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidTokenAmount">EInvalidTokenAmount</a>: u64 = 2;
</code></pre>



<a name="social_contracts_platform_EAlreadyJoined"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EAlreadyJoined">EAlreadyJoined</a>: u64 = 3;
</code></pre>



<a name="social_contracts_platform_ENotJoined"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_ENotJoined">ENotJoined</a>: u64 = 4;
</code></pre>



<a name="social_contracts_platform_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>: u64 = 5;
</code></pre>



<a name="social_contracts_platform_EInsufficientTreasuryFunds"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInsufficientTreasuryFunds">EInsufficientTreasuryFunds</a>: u64 = 6;
</code></pre>



<a name="social_contracts_platform_EEmptyRecipientsList"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EEmptyRecipientsList">EEmptyRecipientsList</a>: u64 = 7;
</code></pre>



<a name="social_contracts_platform_EInvalidBadgeType"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidBadgeType">EInvalidBadgeType</a>: u64 = 8;
</code></pre>



<a name="social_contracts_platform_EBadgeNameTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeNameTooLong">EBadgeNameTooLong</a>: u64 = 9;
</code></pre>



<a name="social_contracts_platform_EBadgeDescriptionTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeDescriptionTooLong">EBadgeDescriptionTooLong</a>: u64 = 10;
</code></pre>



<a name="social_contracts_platform_EBadgeMediaUrlTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeMediaUrlTooLong">EBadgeMediaUrlTooLong</a>: u64 = 11;
</code></pre>



<a name="social_contracts_platform_EInvalidReasoning"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidReasoning">EInvalidReasoning</a>: u64 = 12;
</code></pre>



<a name="social_contracts_platform_EBadgeIconUrlTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeIconUrlTooLong">EBadgeIconUrlTooLong</a>: u64 = 13;
</code></pre>



<a name="social_contracts_platform_EInvalidCategory"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidCategory">EInvalidCategory</a>: u64 = 14;
</code></pre>



<a name="social_contracts_platform_ECategoriesSame"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_ECategoriesSame">ECategoriesSame</a>: u64 = 15;
</code></pre>



<a name="social_contracts_platform_EPlatformApproved"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EPlatformApproved">EPlatformApproved</a>: u64 = 16;
</code></pre>



<a name="social_contracts_platform_EInvalidCoverPhotoUrl"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidCoverPhotoUrl">EInvalidCoverPhotoUrl</a>: u64 = 17;
</code></pre>



<a name="social_contracts_platform_ETooManyMediaPreviews"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_ETooManyMediaPreviews">ETooManyMediaPreviews</a>: u64 = 18;
</code></pre>



<a name="social_contracts_platform_EInvalidMediaPreviewUrl"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidMediaPreviewUrl">EInvalidMediaPreviewUrl</a>: u64 = 19;
</code></pre>



<a name="social_contracts_platform_MAX_BADGE_NAME_LENGTH"></a>

Maximum lengths for badge fields


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_NAME_LENGTH">MAX_BADGE_NAME_LENGTH</a>: u64 = 100;
</code></pre>



<a name="social_contracts_platform_MAX_BADGE_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_DESCRIPTION_LENGTH">MAX_BADGE_DESCRIPTION_LENGTH</a>: u64 = 500;
</code></pre>



<a name="social_contracts_platform_MAX_BADGE_MEDIA_URL_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_MEDIA_URL_LENGTH">MAX_BADGE_MEDIA_URL_LENGTH</a>: u64 = 2048;
</code></pre>



<a name="social_contracts_platform_MAX_BADGE_ICON_URL_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_ICON_URL_LENGTH">MAX_BADGE_ICON_URL_LENGTH</a>: u64 = 2048;
</code></pre>



<a name="social_contracts_platform_MAX_COVER_PHOTO_URL_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_COVER_PHOTO_URL_LENGTH">MAX_COVER_PHOTO_URL_LENGTH</a>: u64 = 2048;
</code></pre>



<a name="social_contracts_platform_MAX_MEDIA_PREVIEWS"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_MEDIA_PREVIEWS">MAX_MEDIA_PREVIEWS</a>: u64 = 10;
</code></pre>



<a name="social_contracts_platform_MAX_MEDIA_PREVIEW_URL_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_MEDIA_PREVIEW_URL_LENGTH">MAX_MEDIA_PREVIEW_URL_LENGTH</a>: u64 = 2048;
</code></pre>



<a name="social_contracts_platform_MAX_REASONING_LENGTH"></a>

Maximum length for approval reasoning


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_MAX_REASONING_LENGTH">MAX_REASONING_LENGTH</a>: u64 = 2000;
</code></pre>



<a name="social_contracts_platform_CATEGORY_SOCIAL_NETWORK"></a>

Platform category constants


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_SOCIAL_NETWORK">CATEGORY_SOCIAL_NETWORK</a>: vector&lt;u8&gt; = vector[83, 111, 99, 105, 97, 108, 32, 78, 101, 116, 119, 111, 114, 107];
</code></pre>



<a name="social_contracts_platform_CATEGORY_MESSAGING"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_MESSAGING">CATEGORY_MESSAGING</a>: vector&lt;u8&gt; = vector[77, 101, 115, 115, 97, 103, 105, 110, 103];
</code></pre>



<a name="social_contracts_platform_CATEGORY_LONG_FORM_PUBLISHING"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_LONG_FORM_PUBLISHING">CATEGORY_LONG_FORM_PUBLISHING</a>: vector&lt;u8&gt; = vector[76, 111, 110, 103, 32, 70, 111, 114, 109, 32, 80, 117, 98, 108, 105, 115, 104, 105, 110, 103];
</code></pre>



<a name="social_contracts_platform_CATEGORY_COMMUNITY_FORUM"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_COMMUNITY_FORUM">CATEGORY_COMMUNITY_FORUM</a>: vector&lt;u8&gt; = vector[67, 111, 109, 109, 117, 110, 105, 116, 121, 32, 70, 111, 114, 117, 109];
</code></pre>



<a name="social_contracts_platform_CATEGORY_VIDEO_STREAMING"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_VIDEO_STREAMING">CATEGORY_VIDEO_STREAMING</a>: vector&lt;u8&gt; = vector[86, 105, 100, 101, 111, 32, 83, 116, 114, 101, 97, 109, 105, 110, 103];
</code></pre>



<a name="social_contracts_platform_CATEGORY_LIVE_STREAMING"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_LIVE_STREAMING">CATEGORY_LIVE_STREAMING</a>: vector&lt;u8&gt; = vector[76, 105, 118, 101, 32, 83, 116, 114, 101, 97, 109, 105, 110, 103];
</code></pre>



<a name="social_contracts_platform_CATEGORY_AUDIO_STREAMING"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_AUDIO_STREAMING">CATEGORY_AUDIO_STREAMING</a>: vector&lt;u8&gt; = vector[65, 117, 100, 105, 111, 32, 83, 116, 114, 101, 97, 109, 105, 110, 103];
</code></pre>



<a name="social_contracts_platform_CATEGORY_DECENTRALIZED_EXCHANGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_DECENTRALIZED_EXCHANGE">CATEGORY_DECENTRALIZED_EXCHANGE</a>: vector&lt;u8&gt; = vector[68, 101, 99, 101, 110, 116, 114, 97, 108, 105, 122, 101, 100, 32, 69, 120, 99, 104, 97, 110, 103, 101];
</code></pre>



<a name="social_contracts_platform_CATEGORY_PREDICTION_MARKET"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_PREDICTION_MARKET">CATEGORY_PREDICTION_MARKET</a>: vector&lt;u8&gt; = vector[80, 114, 101, 100, 105, 99, 116, 105, 111, 110, 32, 77, 97, 114, 107, 101, 116];
</code></pre>



<a name="social_contracts_platform_CATEGORY_INSURANCE_MARKET"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_INSURANCE_MARKET">CATEGORY_INSURANCE_MARKET</a>: vector&lt;u8&gt; = vector[73, 110, 115, 117, 114, 97, 110, 99, 101, 32, 77, 97, 114, 107, 101, 116];
</code></pre>



<a name="social_contracts_platform_CATEGORY_AGENTIC_MARKET"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_AGENTIC_MARKET">CATEGORY_AGENTIC_MARKET</a>: vector&lt;u8&gt; = vector[65, 103, 101, 110, 116, 105, 99, 32, 77, 97, 114, 107, 101, 116];
</code></pre>



<a name="social_contracts_platform_CATEGORY_YIELD_AND_STAKING"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_YIELD_AND_STAKING">CATEGORY_YIELD_AND_STAKING</a>: vector&lt;u8&gt; = vector[89, 105, 101, 108, 100, 32, 97, 110, 100, 32, 83, 116, 97, 107, 105, 110, 103];
</code></pre>



<a name="social_contracts_platform_CATEGORY_REAL_WORLD_ASSET"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_REAL_WORLD_ASSET">CATEGORY_REAL_WORLD_ASSET</a>: vector&lt;u8&gt; = vector[82, 101, 97, 108, 32, 87, 111, 114, 108, 100, 32, 65, 115, 115, 101, 116];
</code></pre>



<a name="social_contracts_platform_CATEGORY_TICKETING_AND_EVENTS"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_TICKETING_AND_EVENTS">CATEGORY_TICKETING_AND_EVENTS</a>: vector&lt;u8&gt; = vector[84, 105, 99, 107, 101, 116, 105, 110, 103, 32, 97, 110, 100, 32, 69, 118, 101, 110, 116, 115];
</code></pre>



<a name="social_contracts_platform_CATEGORY_IP_LICENSING_AND_ROYALTIES"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_IP_LICENSING_AND_ROYALTIES">CATEGORY_IP_LICENSING_AND_ROYALTIES</a>: vector&lt;u8&gt; = vector[73, 80, 32, 76, 105, 99, 101, 110, 115, 105, 110, 103, 32, 97, 110, 100, 32, 82, 111, 121, 97, 108, 116, 105, 101, 115];
</code></pre>



<a name="social_contracts_platform_CATEGORY_DIGITAL_ASSET_VAULT"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_DIGITAL_ASSET_VAULT">CATEGORY_DIGITAL_ASSET_VAULT</a>: vector&lt;u8&gt; = vector[68, 105, 103, 105, 116, 97, 108, 32, 65, 115, 115, 101, 116, 32, 86, 97, 117, 108, 116];
</code></pre>



<a name="social_contracts_platform_CATEGORY_REPUTATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_REPUTATION">CATEGORY_REPUTATION</a>: vector&lt;u8&gt; = vector[82, 101, 112, 117, 116, 97, 116, 105, 111, 110];
</code></pre>



<a name="social_contracts_platform_CATEGORY_ADVERTISING"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_ADVERTISING">CATEGORY_ADVERTISING</a>: vector&lt;u8&gt; = vector[65, 100, 118, 101, 114, 116, 105, 115, 105, 110, 103];
</code></pre>



<a name="social_contracts_platform_CATEGORY_DATA_MARKETPLACE"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_DATA_MARKETPLACE">CATEGORY_DATA_MARKETPLACE</a>: vector&lt;u8&gt; = vector[68, 97, 116, 97, 32, 77, 97, 114, 107, 101, 116, 112, 108, 97, 99, 101];
</code></pre>



<a name="social_contracts_platform_CATEGORY_ORACLE_AND_DATA_FEEDS"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_ORACLE_AND_DATA_FEEDS">CATEGORY_ORACLE_AND_DATA_FEEDS</a>: vector&lt;u8&gt; = vector[79, 114, 97, 99, 108, 101, 32, 97, 110, 100, 32, 68, 97, 116, 97, 32, 70, 101, 101, 100, 115];
</code></pre>



<a name="social_contracts_platform_CATEGORY_ANALYTICS"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_ANALYTICS">CATEGORY_ANALYTICS</a>: vector&lt;u8&gt; = vector[65, 110, 97, 108, 121, 116, 105, 99, 115];
</code></pre>



<a name="social_contracts_platform_CATEGORY_FILE_STORAGE"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_FILE_STORAGE">CATEGORY_FILE_STORAGE</a>: vector&lt;u8&gt; = vector[70, 105, 108, 101, 32, 83, 116, 111, 114, 97, 103, 101];
</code></pre>



<a name="social_contracts_platform_CATEGORY_PRIVACY"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_PRIVACY">CATEGORY_PRIVACY</a>: vector&lt;u8&gt; = vector[80, 114, 105, 118, 97, 99, 121];
</code></pre>



<a name="social_contracts_platform_CATEGORY_GAMING"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_GAMING">CATEGORY_GAMING</a>: vector&lt;u8&gt; = vector[71, 97, 109, 105, 110, 103];
</code></pre>



<a name="social_contracts_platform_CATEGORY_DEVELOPER_TOOLS"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_DEVELOPER_TOOLS">CATEGORY_DEVELOPER_TOOLS</a>: vector&lt;u8&gt; = vector[68, 101, 118, 101, 108, 111, 112, 101, 114, 32, 84, 111, 111, 108, 115];
</code></pre>



<a name="social_contracts_platform_CATEGORY_HARDWARE"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_HARDWARE">CATEGORY_HARDWARE</a>: vector&lt;u8&gt; = vector[72, 97, 114, 100, 119, 97, 114, 101];
</code></pre>



<a name="social_contracts_platform_CATEGORY_RESEARCH"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_RESEARCH">CATEGORY_RESEARCH</a>: vector&lt;u8&gt; = vector[82, 101, 115, 101, 97, 114, 99, 104];
</code></pre>



<a name="social_contracts_platform_JOINED_WALLETS_FIELD"></a>

Field names for dynamic fields


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_WALLETS_FIELD">JOINED_WALLETS_FIELD</a>: vector&lt;u8&gt; = vector[106, 111, 105, 110, 101, 100, 95, 119, 97, 108, 108, 101, 116, 115];
</code></pre>



<a name="social_contracts_platform_PERM_BLOCK_ADMIN"></a>

Stable permission names indexed off-chain (must match indexer constants)


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_PERM_BLOCK_ADMIN">PERM_BLOCK_ADMIN</a>: vector&lt;u8&gt; = vector[80, 108, 97, 116, 102, 111, 114, 109, 66, 108, 111, 99, 107, 65, 100, 109, 105, 110];
</code></pre>



<a name="social_contracts_platform_PERM_BADGE_ADMIN"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_PERM_BADGE_ADMIN">PERM_BADGE_ADMIN</a>: vector&lt;u8&gt; = vector[80, 108, 97, 116, 102, 111, 114, 109, 66, 97, 100, 103, 101, 65, 100, 109, 105, 110];
</code></pre>



<a name="social_contracts_platform_PERM_TREASURY_ADMIN"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_PERM_TREASURY_ADMIN">PERM_TREASURY_ADMIN</a>: vector&lt;u8&gt; = vector[80, 108, 97, 116, 102, 111, 114, 109, 84, 114, 101, 97, 115, 117, 114, 121, 65, 100, 109, 105, 110];
</code></pre>



<a name="social_contracts_platform_PERM_CONTENT_MODERATOR"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_PERM_CONTENT_MODERATOR">PERM_CONTENT_MODERATOR</a>: vector&lt;u8&gt; = vector[80, 108, 97, 116, 102, 111, 114, 109, 67, 111, 110, 116, 101, 110, 116, 77, 111, 100, 101, 114, 97, 116, 111, 114];
</code></pre>



<a name="social_contracts_platform_PERM_PROMOTION_ADMIN"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_PERM_PROMOTION_ADMIN">PERM_PROMOTION_ADMIN</a>: vector&lt;u8&gt; = vector[80, 108, 97, 116, 102, 111, 114, 109, 80, 114, 111, 109, 111, 116, 105, 111, 110, 65, 100, 109, 105, 110];
</code></pre>



<a name="social_contracts_platform_STATUS_DEVELOPMENT"></a>

Platform status constants


<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_DEVELOPMENT">STATUS_DEVELOPMENT</a>: u8 = 0;
</code></pre>



<a name="social_contracts_platform_STATUS_ALPHA"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_ALPHA">STATUS_ALPHA</a>: u8 = 1;
</code></pre>



<a name="social_contracts_platform_STATUS_BETA"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_BETA">STATUS_BETA</a>: u8 = 2;
</code></pre>



<a name="social_contracts_platform_STATUS_LIVE"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_LIVE">STATUS_LIVE</a>: u8 = 3;
</code></pre>



<a name="social_contracts_platform_STATUS_MAINTENANCE"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_MAINTENANCE">STATUS_MAINTENANCE</a>: u8 = 4;
</code></pre>



<a name="social_contracts_platform_STATUS_SUNSET"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SUNSET">STATUS_SUNSET</a>: u8 = 5;
</code></pre>



<a name="social_contracts_platform_STATUS_SHUTDOWN"></a>



<pre><code><b>const</b> <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SHUTDOWN">STATUS_SHUTDOWN</a>: u8 = 6;
</code></pre>



<a name="social_contracts_platform_bootstrap_init"></a>

## Function `bootstrap_init`

Bootstrap initialization function - creates the platform registry


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_bootstrap_init">bootstrap_init</a>(_clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_bootstrap_init">bootstrap_init</a>(_clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> registry = <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a> {
        <a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: object::new(ctx),
        platforms_by_name: table::new(ctx),
        platforms_by_developer: table::new(ctx),
        platform_approvals: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    transfer::share_object(registry);
}
</code></pre>



</details>

<a name="social_contracts_platform_create_platform"></a>

## Function `create_platform`

Create a new platform and transfer to developer


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_create_platform">create_platform</a>(registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, logo_url: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, platforms: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, links: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: u8, <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>: bool, delegate_count: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, delegate_term_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, proposal_submission_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, max_votes_per_user: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, quadratic_base_cost: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, voting_period_epochs: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, quorum_votes: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_create_platform">create_platform</a>(
    registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: String,
    logo_url: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: String,
    platforms: vector&lt;String&gt;,
    links: vector&lt;String&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>: Option&lt;String&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: u8,
    <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: String,
    <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>: bool,
    delegate_count: Option&lt;u64&gt;,
    delegate_term_epochs: Option&lt;u64&gt;,
    proposal_submission_cost: Option&lt;u64&gt;,
    max_votes_per_user: Option&lt;u64&gt;,
    quadratic_base_cost: Option&lt;u64&gt;,
    voting_period_epochs: Option&lt;u64&gt;,
    quorum_votes: Option&lt;u64&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>: Option&lt;String&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>: Option&lt;vector&lt;String&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(registry.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> platform_id = object::new(ctx);
    <b>let</b> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> = tx_context::sender(ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    <a href="../social_contracts/platform.md#social_contracts_platform_validate_cover_photo">validate_cover_photo</a>(&<a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>);
    <a href="../social_contracts/platform.md#social_contracts_platform_validate_media_previews">validate_media_previews</a>(&<a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>);
    // Check <b>if</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a> is already taken
    <b>assert</b>!(!table::contains(&registry.platforms_by_name, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>), <a href="../social_contracts/platform.md#social_contracts_platform_EPlatformAlreadyExists">EPlatformAlreadyExists</a>);
    // Validate primary category
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_valid_category">is_valid_category</a>(&<a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>), <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidCategory">EInvalidCategory</a>);
    // Validate secondary category <b>if</b> provided
    <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>)) {
        <b>let</b> secondary = option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>);
        <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_valid_category">is_valid_category</a>(secondary), <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidCategory">EInvalidCategory</a>);
        // Ensure primary and secondary categories are different
        <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a> != *secondary, <a href="../social_contracts/platform.md#social_contracts_platform_ECategoriesSame">ECategoriesSame</a>);
    };
    // Validate <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> code is one of the defined constants
    <b>assert</b>!(
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_DEVELOPMENT">STATUS_DEVELOPMENT</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_ALPHA">STATUS_ALPHA</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_BETA">STATUS_BETA</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_LIVE">STATUS_LIVE</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_MAINTENANCE">STATUS_MAINTENANCE</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SUNSET">STATUS_SUNSET</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SHUTDOWN">STATUS_SHUTDOWN</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>
    );
    // If DAO <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> is not wanted, set all <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> parameters to None
    <b>let</b> actual_delegate_count = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) delegate_count <b>else</b> option::none();
    <b>let</b> actual_delegate_term_epochs = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) delegate_term_epochs <b>else</b> option::none();
    <b>let</b> actual_proposal_submission_cost = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) proposal_submission_cost <b>else</b> option::none();
    <b>let</b> actual_max_votes_per_user = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) max_votes_per_user <b>else</b> option::none();
    <b>let</b> actual_quadratic_base_cost = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) quadratic_base_cost <b>else</b> option::none();
    <b>let</b> actual_voting_period_epochs = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) voting_period_epochs <b>else</b> option::none();
    <b>let</b> actual_quorum_votes = <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) quorum_votes <b>else</b> option::none();
    <b>let</b> <b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> = <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a> {
        <a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: platform_id,
        <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>: logo_url,
        <a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>,
        platforms,
        links,
        <a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>(<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>),
        <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>: option::none(),
        <a href="../social_contracts/platform.md#social_contracts_platform_created_at">created_at</a>: now,
        treasury: balance::zero(),
        <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>,
        delegate_count: actual_delegate_count,
        delegate_term_epochs: actual_delegate_term_epochs,
        proposal_submission_cost: actual_proposal_submission_cost,
        max_votes_per_user: actual_max_votes_per_user,
        quadratic_base_cost: actual_quadratic_base_cost,
        voting_period_epochs: actual_voting_period_epochs,
        quorum_votes: actual_quorum_votes,
        <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>: option::none(),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> <b>mut</b> moderators_group = permissioned_group::new_derived&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_ModeratorsGroupTag">ModeratorsGroupTag</a>&gt;(
        <a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>(),
        &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_ModeratorsGroupTag">ModeratorsGroupTag</a>(),
        ctx,
    );
    <b>let</b> <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a> = object::id(&moderators_group);
    <a href="../social_contracts/platform.md#social_contracts_platform_grant_all_extension_permissions">grant_all_extension_permissions</a>(&<b>mut</b> moderators_group, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>, ctx);
    <a href="../social_contracts/platform.md#social_contracts_platform_emit_permissions_granted">emit_permissions_granted</a>(
        &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_all_extension_permission_names">all_extension_permission_names</a>(),
        <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>,
    );
    transfer::public_share_object(moderators_group);
    // Register <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> in registry
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    // Add to platforms by <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>
    table::add(&<b>mut</b> registry.platforms_by_name, *&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>, platform_id);
    // Add to platforms by <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
    <b>if</b> (!table::contains(&registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>)) {
        table::add(&<b>mut</b> registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>, vector::empty&lt;<b>address</b>&gt;());
    };
    <b>let</b> developer_platforms = table::borrow_mut(&<b>mut</b> registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>);
    vector::push_back(developer_platforms, platform_id);
    // Add to <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> approvals (starts <b>as</b> not approved)
    table::add(&<b>mut</b> registry.platform_approvals, platform_id, <b>false</b>);
    // If <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> wants DAO <a href="../social_contracts/governance.md#social_contracts_governance">governance</a>, create <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> registry immediately so it can
    // be reflected in the creation event payload.
    <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>) {
        // Use default values <b>if</b> options are None
        <b>let</b> delegate_count = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_count)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_count)
        } <b>else</b> {
            7 // Default value
        };
        <b>let</b> delegate_term_epochs = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_term_epochs)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_term_epochs)
        } <b>else</b> {
            30 // Default value
        };
        <b>let</b> proposal_submission_cost = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.proposal_submission_cost)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.proposal_submission_cost)
        } <b>else</b> {
            50_000_000 // Default value
        };
        <b>let</b> max_votes_per_user = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.max_votes_per_user)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.max_votes_per_user)
        } <b>else</b> {
            5 // Default value
        };
        <b>let</b> quadratic_base_cost = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quadratic_base_cost)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quadratic_base_cost)
        } <b>else</b> {
            5_000_000 // Default value
        };
        <b>let</b> voting_period_epochs = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.voting_period_epochs)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.voting_period_epochs)
        } <b>else</b> {
            3 // Default value
        };
        <b>let</b> quorum_votes = <b>if</b> (option::is_some(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quorum_votes)) {
            *option::borrow(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quorum_votes)
        } <b>else</b> {
            15 // Default value
        };
        // Create <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> registry <b>for</b> this <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
        <b>let</b> registry_id = <a href="../social_contracts/governance.md#social_contracts_governance_create_platform_governance">governance::create_platform_governance</a>(
            delegate_count,
            delegate_term_epochs,
            proposal_submission_cost,
            max_votes_per_user,
            quadratic_base_cost,
            voting_period_epochs,
            quorum_votes,
            clock,
            ctx
        );
        // Store registry ID in the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a> = option::some(registry_id);
    };
    // Emit <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> created event (after <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> registry creation so DAO fields are populated)
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_PlatformCreatedEvent">PlatformCreatedEvent</a> {
        platform_id,
        <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>,
        platforms: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.platforms,
        links: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.links,
        <a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>,
        delegate_count: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_count,
        delegate_term_epochs: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_term_epochs,
        proposal_submission_cost: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.proposal_submission_cost,
        max_votes_per_user: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.max_votes_per_user,
        quadratic_base_cost: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quadratic_base_cost,
        voting_period_epochs: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.voting_period_epochs,
        quorum_votes: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quorum_votes,
        <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>,
    });
    // Share <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <b>as</b> a shared object (publicly accessible)
    transfer::share_object(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
}
</code></pre>



</details>

<a name="social_contracts_platform_update_platform"></a>

## Function `update_platform`

Update platform information


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_update_platform">update_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, new_name: <a href="../std/string.md#std_string_String">std::string::String</a>, new_tagline: <a href="../std/string.md#std_string_String">std::string::String</a>, new_description: <a href="../std/string.md#std_string_String">std::string::String</a>, new_logo_url: <a href="../std/string.md#std_string_String">std::string::String</a>, new_terms_of_service: <a href="../std/string.md#std_string_String">std::string::String</a>, new_privacy_policy: <a href="../std/string.md#std_string_String">std::string::String</a>, new_platforms: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, new_links: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, new_primary_category: <a href="../std/string.md#std_string_String">std::string::String</a>, new_secondary_category: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>: u8, new_release_date: <a href="../std/string.md#std_string_String">std::string::String</a>, new_shutdown_date: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, new_cover_photo: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, new_media_previews: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_update_platform">update_platform</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    new_name: String,
    new_tagline: String,
    new_description: String,
    new_logo_url: String,
    new_terms_of_service: String,
    new_privacy_policy: String,
    new_platforms: vector&lt;String&gt;,
    new_links: vector&lt;String&gt;,
    new_primary_category: String,
    new_secondary_category: Option&lt;String&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>: u8,
    new_release_date: String,
    new_shutdown_date: Option&lt;String&gt;,
    new_cover_photo: Option&lt;String&gt;,
    new_media_previews: Option&lt;vector&lt;String&gt;&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> now = clock::timestamp_ms(clock);
    <a href="../social_contracts/platform.md#social_contracts_platform_validate_cover_photo">validate_cover_photo</a>(&new_cover_photo);
    <a href="../social_contracts/platform.md#social_contracts_platform_validate_media_previews">validate_media_previews</a>(&new_media_previews);
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == tx_context::sender(ctx), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Validate primary category
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_valid_category">is_valid_category</a>(&new_primary_category), <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidCategory">EInvalidCategory</a>);
    // Validate secondary category <b>if</b> provided
    <b>if</b> (option::is_some(&new_secondary_category)) {
        <b>let</b> secondary = option::borrow(&new_secondary_category);
        <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_valid_category">is_valid_category</a>(secondary), <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidCategory">EInvalidCategory</a>);
        // Ensure primary and secondary categories are different
        <b>assert</b>!(new_primary_category != *secondary, <a href="../social_contracts/platform.md#social_contracts_platform_ECategoriesSame">ECategoriesSame</a>);
    };
    // Update <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> information
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_name">name</a> = new_name;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a> = new_tagline;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_description">description</a> = new_description;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a> = new_logo_url;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a> = new_terms_of_service;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a> = new_privacy_policy;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.platforms = new_platforms;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.links = new_links;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a> = new_primary_category;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a> = new_secondary_category;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> = <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>(<a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>);
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a> = new_release_date;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a> = new_shutdown_date;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a> = new_cover_photo;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a> = new_media_previews;
    // Emit <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> updated event
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_PlatformUpdatedEvent">PlatformUpdatedEvent</a> {
        platform_id: object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>),
        <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>,
        platforms: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.platforms,
        links: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.links,
        <a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>: <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>,
        updated_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_platform_version"></a>

## Function `platform_version`

Get the version of a platform


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_platform_version">platform_version</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_platform_version">platform_version</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): u64 {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version
}
</code></pre>



</details>

<a name="social_contracts_platform_borrow_platform_version_mut"></a>

## Function `borrow_platform_version_mut`

Get a mutable reference to the platform version (only for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_borrow_platform_version_mut">borrow_platform_version_mut</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_borrow_platform_version_mut">borrow_platform_version_mut</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &<b>mut</b> u64 {
    &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version
}
</code></pre>



</details>

<a name="social_contracts_platform_registry_version"></a>

## Function `registry_version`

Get the version of the platform registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_registry_version">registry_version</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_registry_version">registry_version</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>): u64 {
    registry.version
}
</code></pre>



</details>

<a name="social_contracts_platform_borrow_registry_version_mut"></a>

## Function `borrow_registry_version_mut`

Get a mutable reference to the registry version (only for upgrade module)


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>): &<b>mut</b> u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_borrow_registry_version_mut">borrow_registry_version_mut</a>(registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>): &<b>mut</b> u64 {
    &<b>mut</b> registry.version
}
</code></pre>



</details>

<a name="social_contracts_platform_add_to_treasury"></a>

## Function `add_to_treasury`

Add MYSO tokens to platform treasury


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, coin: &<b>mut</b> <a href="../myso/coin.md#myso_coin_Coin">myso::coin::Coin</a>&lt;<a href="../myso/myso.md#myso_myso_MYSO">myso::myso::MYSO</a>&gt;, amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">add_to_treasury</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    coin: &<b>mut</b> Coin&lt;MYSO&gt;,
    amount: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Check amount validity
    <b>assert</b>!(amount &gt; 0 && coin::value(coin) &gt;= amount, <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidTokenAmount">EInvalidTokenAmount</a>);
    // Split coin and add to treasury
    <b>let</b> treasury_coin = coin::split(coin, amount, ctx);
    balance::join(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.treasury, coin::into_balance(treasury_coin));
    // Emit treasury funded event
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>let</b> new_balance = balance::value(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.treasury);
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_TreasuryFundedEvent">TreasuryFundedEvent</a> {
        platform_id,
        amount,
        funded_by: tx_context::sender(ctx),
        new_balance,
        timestamp: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_mark_platform_governance_proposal_implemented"></a>

## Function `mark_platform_governance_proposal_implemented`

Mark a platform-linked governance proposal as implemented; pay the proposer the implementation pool.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_mark_platform_governance_proposal_implemented">mark_platform_governance_proposal_implemented</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_mark_platform_governance_proposal_implemented">mark_platform_governance_proposal_implemented</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">governance::GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">governance::Proposal</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> registry_id = object::id(registry);
    <b>let</b> opt = <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>assert</b>!(option::is_some(opt), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(*option::borrow(opt) == registry_id, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(
        <a href="../social_contracts/governance.md#social_contracts_governance_registry_type">governance::registry_type</a>(registry) == <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_platform_value">governance::proposal_type_platform_value</a>(),
        <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>
    );
    <b>let</b> sent_time = clock::timestamp_ms(clock);
    <b>let</b> submitter = <a href="../social_contracts/governance.md#social_contracts_governance_proposal_submitter">governance::proposal_submitter</a>(proposal);
    <b>let</b> bal = <a href="../social_contracts/governance.md#social_contracts_governance_mark_proposal_implemented_take_pool">governance::mark_proposal_implemented_take_pool</a>(registry, proposal, <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>, clock, ctx);
    <b>let</b> amount = balance::value(&bal);
    <b>if</b> (amount &gt; 0) {
        <b>let</b> c = coin::from_balance(bal, ctx);
        transfer::public_transfer(c, submitter);
    } <b>else</b> {
        balance::destroy_zero(bal);
    };
    <b>let</b> proposal_id = object::id(proposal);
    <a href="../social_contracts/governance.md#social_contracts_governance_emit_implementation_reward_to_submitter_event">governance::emit_implementation_reward_to_submitter_event</a>(
        proposal_id,
        submitter,
        amount,
        <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_platform_value">governance::proposal_type_platform_value</a>(),
        sent_time,
    );
}
</code></pre>



</details>

<a name="social_contracts_platform_delegate_vote_on_platform_governance_proposal"></a>

## Function `delegate_vote_on_platform_governance_proposal`

Platform-linked registry: delegate vote; on council rejection, the pool is forfeited to this platform’s treasury.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_delegate_vote_on_platform_governance_proposal">delegate_vote_on_platform_governance_proposal</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, approve: bool, reason: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_delegate_vote_on_platform_governance_proposal">delegate_vote_on_platform_governance_proposal</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">governance::GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">governance::Proposal</a>,
    approve: bool,
    reason: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> registry_id = object::id(registry);
    <b>let</b> opt = <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>assert</b>!(option::is_some(opt), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(*option::borrow(opt) == registry_id, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">governance::registry_type</a>(registry) == <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_platform_value">governance::proposal_type_platform_value</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> out = <a href="../social_contracts/governance.md#social_contracts_governance_run_delegate_review_vote">governance::run_delegate_review_vote</a>(
        registry,
        proposal,
        approve,
        reason,
        clock,
        ctx
    );
    <b>let</b> platform_addr = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>if</b> (out == <a href="../social_contracts/governance.md#social_contracts_governance_delegate_vote_outcome_reject_value">governance::delegate_vote_outcome_reject_value</a>()) {
        <b>let</b> bal = <a href="../social_contracts/governance.md#social_contracts_governance_complete_delegate_reject_drain_to_platform_governance">governance::complete_delegate_reject_drain_to_platform_governance</a>(
            registry,
            proposal,
            platform_addr,
            current_time,
            ctx
        );
        <b>let</b> amt = balance::value(&bal);
        <b>if</b> (amt &gt; 0) {
            <b>let</b> <b>mut</b> c = coin::from_balance(bal, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> c, amt, clock, ctx);
            coin::destroy_zero(c);
        } <b>else</b> {
            balance::destroy_zero(bal);
        };
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_finalize_platform_governance_proposal"></a>

## Function `finalize_platform_governance_proposal`

After community voting, finalize a platform-governance proposal; on failure, pool to this platform’s treasury.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_finalize_platform_governance_proposal">finalize_platform_governance_proposal</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_finalize_platform_governance_proposal">finalize_platform_governance_proposal</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">governance::GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">governance::Proposal</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> registry_id = object::id(registry);
    <b>let</b> opt = <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>assert</b>!(option::is_some(opt), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(*option::borrow(opt) == registry_id, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">governance::registry_type</a>(registry) == <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_platform_value">governance::proposal_type_platform_value</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>let</b> (outcome, time_ms) = <a href="../social_contracts/governance.md#social_contracts_governance_finalize_community_voting_internals">governance::finalize_community_voting_internals</a>(
        registry,
        proposal,
        clock,
        ctx,
    );
    <b>let</b> platform_addr = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>if</b> (outcome == <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_reject_community_value">governance::community_finalize_voting_reject_community_value</a>()) {
        <b>let</b> bal = <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_after_community_reject_to_platform_treasury">governance::forfeit_proposal_pool_after_community_reject_to_platform_treasury</a>(
            registry,
            proposal,
            platform_addr,
            <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_community_rejected_value">governance::forfeit_reason_community_rejected_value</a>(),
            time_ms,
        );
        <b>let</b> amt = balance::value(&bal);
        <b>if</b> (amt &gt; 0) {
            <b>let</b> <b>mut</b> c = coin::from_balance(bal, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> c, amt, clock, ctx);
            coin::destroy_zero(c);
        } <b>else</b> {
            balance::destroy_zero(bal);
        };
    } <b>else</b> <b>if</b> (outcome == <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_reject_quorum_value">governance::community_finalize_voting_reject_quorum_value</a>()) {
        <b>let</b> bal = <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_after_community_reject_to_platform_treasury">governance::forfeit_proposal_pool_after_community_reject_to_platform_treasury</a>(
            registry,
            proposal,
            platform_addr,
            <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_quorum_not_met_value">governance::forfeit_reason_quorum_not_met_value</a>(),
            time_ms,
        );
        <b>let</b> amt = balance::value(&bal);
        <b>if</b> (amt &gt; 0) {
            <b>let</b> <b>mut</b> c = coin::from_balance(bal, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> c, amt, clock, ctx);
            coin::destroy_zero(c);
        } <b>else</b> {
            balance::destroy_zero(bal);
        };
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_finalize_platform_governance_proposal_anonymous"></a>

## Function `finalize_platform_governance_proposal_anonymous`

Same constraints as [<code><a href="../social_contracts/governance.md#social_contracts_governance_finalize_proposal_anonymous">governance::finalize_proposal_anonymous</a></code>]: non-<code><b>entry</b></code> so decryption key vectors are valid arguments.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_finalize_platform_governance_proposal_anonymous">finalize_platform_governance_proposal_anonymous</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_VerifiedDerivedKey">mydata::bf_hmac_encryption::VerifiedDerivedKey</a>&gt;, public_keys: &vector&lt;<a href="../mydata/bf_hmac_encryption.md#mydata_bf_hmac_encryption_PublicKey">mydata::bf_hmac_encryption::PublicKey</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_finalize_platform_governance_proposal_anonymous">finalize_platform_governance_proposal_anonymous</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">governance::GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">governance::Proposal</a>,
    keys: &vector&lt;VerifiedDerivedKey&gt;,
    public_keys: &vector&lt;PublicKey&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> registry_id = object::id(registry);
    <b>let</b> opt = <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>assert</b>!(option::is_some(opt), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(*option::borrow(opt) == registry_id, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(<a href="../social_contracts/governance.md#social_contracts_governance_registry_type">governance::registry_type</a>(registry) == <a href="../social_contracts/governance.md#social_contracts_governance_proposal_type_platform_value">governance::proposal_type_platform_value</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>let</b> (outcome, time_ms) = <a href="../social_contracts/governance.md#social_contracts_governance_anonymous_tally_votes_and_finalize_community_internals">governance::anonymous_tally_votes_and_finalize_community_internals</a>(
        registry,
        proposal,
        keys,
        public_keys,
        clock,
        ctx,
    );
    <b>let</b> platform_addr = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>if</b> (outcome == <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_reject_community_value">governance::community_finalize_voting_reject_community_value</a>()) {
        <b>let</b> bal = <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_after_community_reject_to_platform_treasury">governance::forfeit_proposal_pool_after_community_reject_to_platform_treasury</a>(
            registry,
            proposal,
            platform_addr,
            <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_community_rejected_value">governance::forfeit_reason_community_rejected_value</a>(),
            time_ms,
        );
        <b>let</b> amt = balance::value(&bal);
        <b>if</b> (amt &gt; 0) {
            <b>let</b> <b>mut</b> c = coin::from_balance(bal, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> c, amt, clock, ctx);
            coin::destroy_zero(c);
        } <b>else</b> {
            balance::destroy_zero(bal);
        };
    } <b>else</b> <b>if</b> (outcome == <a href="../social_contracts/governance.md#social_contracts_governance_community_finalize_voting_reject_quorum_value">governance::community_finalize_voting_reject_quorum_value</a>()) {
        <b>let</b> bal = <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_proposal_pool_after_community_reject_to_platform_treasury">governance::forfeit_proposal_pool_after_community_reject_to_platform_treasury</a>(
            registry,
            proposal,
            platform_addr,
            <a href="../social_contracts/governance.md#social_contracts_governance_forfeit_reason_quorum_not_met_value">governance::forfeit_reason_quorum_not_met_value</a>(),
            time_ms,
        );
        <b>let</b> amt = balance::value(&bal);
        <b>if</b> (amt &gt; 0) {
            <b>let</b> <b>mut</b> c = coin::from_balance(bal, ctx);
            <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> c, amt, clock, ctx);
            coin::destroy_zero(c);
        } <b>else</b> {
            balance::destroy_zero(bal);
        };
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_reject_platform_governance_proposal_manually"></a>

## Function `reject_platform_governance_proposal_manually`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_reject_platform_governance_proposal_manually">reject_platform_governance_proposal_manually</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">social_contracts::governance::Proposal</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_reject_platform_governance_proposal_manually">reject_platform_governance_proposal_manually</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">governance::GovernanceDAO</a>,
    proposal: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_Proposal">governance::Proposal</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> registry_id = object::id(registry);
    <b>let</b> opt = <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>assert</b>!(option::is_some(opt), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(*option::borrow(opt) == registry_id, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>let</b> t = <a href="../social_contracts/governance.md#social_contracts_governance_assert_platform_delegate_manual_reject">governance::assert_platform_delegate_manual_reject</a>(
        registry,
        proposal,
        clock,
        ctx
    );
    <b>let</b> platform_addr = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>let</b> bal = <a href="../social_contracts/governance.md#social_contracts_governance_complete_delegate_reject_drain_to_platform_governance">governance::complete_delegate_reject_drain_to_platform_governance</a>(
        registry,
        proposal,
        platform_addr,
        t,
        ctx
    );
    <b>let</b> amt = balance::value(&bal);
    <b>if</b> (amt &gt; 0) {
        <b>let</b> <b>mut</b> c = coin::from_balance(bal, ctx);
        <a href="../social_contracts/platform.md#social_contracts_platform_add_to_treasury">add_to_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, &<b>mut</b> c, amt, clock, ctx);
        coin::destroy_zero(c);
    } <b>else</b> {
        balance::destroy_zero(bal);
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_grant_moderator_permission"></a>

## Function `grant_moderator_permission`

Grant a single extension permission to a platform moderator


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_grant_moderator_permission">grant_moderator_permission</a>&lt;P: drop&gt;(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, member: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_grant_moderator_permission">grant_moderator_permission</a>&lt;P: drop&gt;(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    member: <b>address</b>,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(
        group.has_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, ExtensionPermissionsAdmin&gt;(caller),
        <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>,
    );
    permissioned_group::grant_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, P&gt;(group, member, ctx);
    <a href="../social_contracts/platform.md#social_contracts_platform_emit_permissions_granted">emit_permissions_granted</a>(
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        object::id(group),
        member,
        vector[<a href="../social_contracts/platform.md#social_contracts_platform_permission_name">permission_name</a>&lt;P&gt;()],
        caller,
    );
}
</code></pre>



</details>

<a name="social_contracts_platform_grant_all_moderator_permissions"></a>

## Function `grant_all_moderator_permissions`

Grant all extension permissions to a member (full platform moderator)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_grant_all_moderator_permissions">grant_all_moderator_permissions</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, member: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_grant_all_moderator_permissions">grant_all_moderator_permissions</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    member: <b>address</b>,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(
        group.has_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, ExtensionPermissionsAdmin&gt;(caller),
        <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>,
    );
    <a href="../social_contracts/platform.md#social_contracts_platform_grant_all_extension_permissions">grant_all_extension_permissions</a>(group, member, ctx);
    <a href="../social_contracts/platform.md#social_contracts_platform_emit_permissions_granted">emit_permissions_granted</a>(
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        object::id(group),
        member,
        <a href="../social_contracts/platform.md#social_contracts_platform_all_extension_permission_names">all_extension_permission_names</a>(),
        caller,
    );
}
</code></pre>



</details>

<a name="social_contracts_platform_revoke_moderator_permission"></a>

## Function `revoke_moderator_permission`

Revoke a single extension permission from a platform moderator


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_revoke_moderator_permission">revoke_moderator_permission</a>&lt;P: drop&gt;(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, member: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_revoke_moderator_permission">revoke_moderator_permission</a>&lt;P: drop&gt;(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    member: <b>address</b>,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(
        group.has_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, ExtensionPermissionsAdmin&gt;(caller),
        <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>,
    );
    <b>assert</b>!(member != <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    permissioned_group::revoke_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, P&gt;(group, member, ctx);
    <a href="../social_contracts/platform.md#social_contracts_platform_emit_permissions_revoked">emit_permissions_revoked</a>(
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>,
        object::id(group),
        member,
        vector[<a href="../social_contracts/platform.md#social_contracts_platform_permission_name">permission_name</a>&lt;P&gt;()],
        caller,
    );
    <a href="../social_contracts/platform.md#social_contracts_platform_maybe_emit_moderator_removed">maybe_emit_moderator_removed</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, group, member, caller);
}
</code></pre>



</details>

<a name="social_contracts_platform_add_moderator"></a>

## Function `add_moderator`

Add a moderator with all extension permissions (developer-only convenience wrapper)


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_moderator">add_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, moderator_address: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_moderator">add_moderator</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    moderator_address: <b>address</b>,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == tx_context::sender(ctx), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <a href="../social_contracts/platform.md#social_contracts_platform_grant_all_moderator_permissions">grant_all_moderator_permissions</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, group, moderator_address, ctx);
}
</code></pre>



</details>

<a name="social_contracts_platform_remove_moderator"></a>

## Function `remove_moderator`

Remove a moderator entirely from the platform moderators group


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_remove_moderator">remove_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, moderator_address: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_remove_moderator">remove_moderator</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    moderator_address: <b>address</b>,
    ctx: &TxContext,
) {
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == caller, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(moderator_address != <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>assert</b>!(group.is_member&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;(moderator_address), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    permissioned_group::remove_member&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;(group, moderator_address, ctx);
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_ModeratorRemovedEvent">ModeratorRemovedEvent</a> {
        platform_id: object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>),
        <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>: object::id(group),
        member: moderator_address,
        removed_by: caller,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_block_wallet"></a>

## Function `block_wallet`

Block a wallet address from the platform
Allows platform developers/moderators to block wallets using the platform address as the blocker
This enables platforms (shared objects) to block user wallets


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_block_wallet">block_wallet</a>(block_list_registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_contracts::social_graph::SocialGraph</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, blocked_wallet_address: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_block_wallet">block_wallet</a>(
    block_list_registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>: &<b>mut</b> <a href="../social_contracts/social_graph.md#social_contracts_social_graph_SocialGraph">social_graph::SocialGraph</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    blocked_wallet_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <a href="../social_contracts/platform.md#social_contracts_platform_assert_moderator_permission">assert_moderator_permission</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformBlockAdmin">PlatformBlockAdmin</a>&gt;(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, group, caller);
    // Get the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <b>address</b> (this will be the blocker <b>address</b>)
    <b>let</b> platform_address = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    // Call <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>'s internal helper function with <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <b>address</b> <b>as</b> blocker
    <a href="../social_contracts/block_list.md#social_contracts_block_list_block_wallet_internal">block_list::block_wallet_internal</a>(
        block_list_registry,
        <a href="../social_contracts/social_graph.md#social_contracts_social_graph">social_graph</a>,
        platform_address,
        blocked_wallet_address
    );
}
</code></pre>



</details>

<a name="social_contracts_platform_unblock_wallet"></a>

## Function `unblock_wallet`

Unblock a wallet address from the platform
Allows platform developers/moderators to unblock wallets using the platform address as the blocker


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_unblock_wallet">unblock_wallet</a>(block_list_registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, blocked_wallet_address: <b>address</b>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_unblock_wallet">unblock_wallet</a>(
    block_list_registry: &<b>mut</b> <a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    blocked_wallet_address: <b>address</b>,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <a href="../social_contracts/platform.md#social_contracts_platform_assert_moderator_permission">assert_moderator_permission</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformBlockAdmin">PlatformBlockAdmin</a>&gt;(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, group, caller);
    // Get the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <b>address</b> (this is the blocker <b>address</b>)
    <b>let</b> platform_address = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    // Call <a href="../social_contracts/block_list.md#social_contracts_block_list">block_list</a>'s internal helper function with <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <b>address</b> <b>as</b> blocker
    <a href="../social_contracts/block_list.md#social_contracts_block_list_unblock_wallet_internal">block_list::unblock_wallet_internal</a>(block_list_registry, platform_address, blocked_wallet_address);
}
</code></pre>



</details>

<a name="social_contracts_platform_toggle_platform_approval"></a>

## Function `toggle_platform_approval`

Toggle platform approval status (requires PlatformAdminCap only)
Optional reasoning can be provided to explain the decision


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_toggle_platform_approval">toggle_platform_approval</a>(registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, platform_id: <b>address</b>, _: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">social_contracts::platform::PlatformAdminCap</a>, reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_toggle_platform_approval">toggle_platform_approval</a>(
    registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    platform_id: <b>address</b>,
    _: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">PlatformAdminCap</a>,
    reasoning: Option&lt;String&gt;,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(registry.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Admin capability verification is handled by type system
    // Verify the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> exists in the registry
    <b>assert</b>!(table::contains(&registry.platform_approvals, platform_id), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Validate reasoning length <b>if</b> provided
    <b>if</b> (option::is_some(&reasoning)) {
        <b>let</b> reasoning_val = option::borrow(&reasoning);
        <b>assert</b>!(string::length(reasoning_val) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_REASONING_LENGTH">MAX_REASONING_LENGTH</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidReasoning">EInvalidReasoning</a>);
    };
    // Get current approval <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> and toggle it
    <b>let</b> current_approval = *table::borrow(&registry.platform_approvals, platform_id);
    <b>let</b> new_approval = !current_approval;
    // Update the approval <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> in the registry
    *table::borrow_mut(&<b>mut</b> registry.platform_approvals, platform_id) = new_approval;
    // Emit approval <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> changed event with reasoning
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_PlatformApprovalChangedEvent">PlatformApprovalChangedEvent</a> {
        platform_id,
        approved: new_approval,
        changed_by: tx_context::sender(ctx),
        reasoning,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_delete_platform"></a>

## Function `delete_platform`

Delete a platform (requires PlatformAdminCap only)
Can only delete platforms that are NOT approved
Optional reasoning can be provided to explain the deletion


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_delete_platform">delete_platform</a>(registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, _: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">social_contracts::platform::PlatformAdminCap</a>, reasoning: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_delete_platform">delete_platform</a>(
    registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    _: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">PlatformAdminCap</a>,
    reasoning: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>assert</b>!(registry.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Admin capability verification is handled by type system
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>let</b> platform_name = <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>let</b> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> = <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    // Verify the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> exists in the registry
    <b>assert</b>!(table::contains(&registry.platform_approvals, platform_id), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Verify <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is NOT approved (can only delete unapproved platforms)
    <b>let</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a> = *table::borrow(&registry.platform_approvals, platform_id);
    <b>assert</b>!(!<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EPlatformApproved">EPlatformApproved</a>);
    // Validate reasoning length <b>if</b> provided
    <b>if</b> (option::is_some(&reasoning)) {
        <b>let</b> reasoning_val = option::borrow(&reasoning);
        <b>assert</b>!(string::length(reasoning_val) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_REASONING_LENGTH">MAX_REASONING_LENGTH</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidReasoning">EInvalidReasoning</a>);
    };
    // Remove from platforms_by_name table
    <b>if</b> (table::contains(&registry.platforms_by_name, platform_name)) {
        table::remove(&<b>mut</b> registry.platforms_by_name, platform_name);
    };
    // Remove from platforms_by_developer table
    <b>if</b> (table::contains(&registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>)) {
        <b>let</b> developer_platforms = table::borrow_mut(&<b>mut</b> registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>);
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(developer_platforms);
        <b>while</b> (i &lt; len) {
            <b>if</b> (*vector::borrow(developer_platforms, i) == platform_id) {
                vector::remove(developer_platforms, i);
                <b>break</b>
            };
            i = i + 1;
        };
        // If <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> <b>has</b> no more platforms, remove the <b>entry</b>
        <b>if</b> (vector::length(developer_platforms) == 0) {
            table::remove(&<b>mut</b> registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>);
        };
    };
    // Remove from platform_approvals table
    table::remove(&<b>mut</b> registry.platform_approvals, platform_id);
    // Emit <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> deleted event
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_PlatformDeletedEvent">PlatformDeletedEvent</a> {
        platform_id,
        <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: platform_name,
        <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>,
        deleted_by: tx_context::sender(ctx),
        timestamp: clock::timestamp_ms(clock),
        reasoning,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_new_status"></a>

## Function `new_status`

Create a new platform status


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>(<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: u8): <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">social_contracts::platform::PlatformStatus</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_new_status">new_status</a>(<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: u8): <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">PlatformStatus</a> {
    // Validate the <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> code is one of the defined constants
    <b>assert</b>!(
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_DEVELOPMENT">STATUS_DEVELOPMENT</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_ALPHA">STATUS_ALPHA</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_BETA">STATUS_BETA</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_LIVE">STATUS_LIVE</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_MAINTENANCE">STATUS_MAINTENANCE</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SUNSET">STATUS_SUNSET</a> ||
        <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> == <a href="../social_contracts/platform.md#social_contracts_platform_STATUS_SHUTDOWN">STATUS_SHUTDOWN</a>,
        <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>
    );
    <a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">PlatformStatus</a> { <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a> }
}
</code></pre>



</details>

<a name="social_contracts_platform_status_value"></a>

## Function `status_value`

Get the status value


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_status_value">status_value</a>(<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">social_contracts::platform::PlatformStatus</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_status_value">status_value</a>(<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformStatus">PlatformStatus</a>): u8 {
    <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>.<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_validate_cover_photo"></a>

## Function `validate_cover_photo`



<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_validate_cover_photo">validate_cover_photo</a>(<a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_validate_cover_photo">validate_cover_photo</a>(<a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>: &Option&lt;String&gt;) {
    <b>if</b> (option::is_some(<a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>)) {
        <b>let</b> url = option::borrow(<a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>);
        <b>assert</b>!(
            string::length(url) &gt; 0 && string::length(url) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_COVER_PHOTO_URL_LENGTH">MAX_COVER_PHOTO_URL_LENGTH</a>,
            <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidCoverPhotoUrl">EInvalidCoverPhotoUrl</a>
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_validate_media_previews"></a>

## Function `validate_media_previews`



<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_validate_media_previews">validate_media_previews</a>(<a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_validate_media_previews">validate_media_previews</a>(<a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>: &Option&lt;vector&lt;String&gt;&gt;) {
    <b>if</b> (option::is_some(<a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>)) {
        <b>let</b> previews = option::borrow(<a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>);
        <b>assert</b>!(vector::length(previews) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_MEDIA_PREVIEWS">MAX_MEDIA_PREVIEWS</a>, <a href="../social_contracts/platform.md#social_contracts_platform_ETooManyMediaPreviews">ETooManyMediaPreviews</a>);
        <b>let</b> <b>mut</b> i = 0;
        <b>let</b> len = vector::length(previews);
        <b>while</b> (i &lt; len) {
            <b>let</b> url = vector::borrow(previews, i);
            <b>assert</b>!(
                string::length(url) &gt; 0 && string::length(url) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_MEDIA_PREVIEW_URL_LENGTH">MAX_MEDIA_PREVIEW_URL_LENGTH</a>,
                <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidMediaPreviewUrl">EInvalidMediaPreviewUrl</a>
            );
            i = i + 1;
        };
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_is_valid_category"></a>

## Function `is_valid_category`

Validate that a category string matches one of the allowed categories


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_valid_category">is_valid_category</a>(category: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_valid_category">is_valid_category</a>(category: &String): bool {
    <b>let</b> category_bytes = string::as_bytes(category);
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_SOCIAL_NETWORK">CATEGORY_SOCIAL_NETWORK</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_MESSAGING">CATEGORY_MESSAGING</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_LONG_FORM_PUBLISHING">CATEGORY_LONG_FORM_PUBLISHING</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_COMMUNITY_FORUM">CATEGORY_COMMUNITY_FORUM</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_VIDEO_STREAMING">CATEGORY_VIDEO_STREAMING</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_LIVE_STREAMING">CATEGORY_LIVE_STREAMING</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_AUDIO_STREAMING">CATEGORY_AUDIO_STREAMING</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_DECENTRALIZED_EXCHANGE">CATEGORY_DECENTRALIZED_EXCHANGE</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_PREDICTION_MARKET">CATEGORY_PREDICTION_MARKET</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_INSURANCE_MARKET">CATEGORY_INSURANCE_MARKET</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_AGENTIC_MARKET">CATEGORY_AGENTIC_MARKET</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_YIELD_AND_STAKING">CATEGORY_YIELD_AND_STAKING</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_REAL_WORLD_ASSET">CATEGORY_REAL_WORLD_ASSET</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_TICKETING_AND_EVENTS">CATEGORY_TICKETING_AND_EVENTS</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_IP_LICENSING_AND_ROYALTIES">CATEGORY_IP_LICENSING_AND_ROYALTIES</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_DIGITAL_ASSET_VAULT">CATEGORY_DIGITAL_ASSET_VAULT</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_REPUTATION">CATEGORY_REPUTATION</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_ADVERTISING">CATEGORY_ADVERTISING</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_DATA_MARKETPLACE">CATEGORY_DATA_MARKETPLACE</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_ORACLE_AND_DATA_FEEDS">CATEGORY_ORACLE_AND_DATA_FEEDS</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_ANALYTICS">CATEGORY_ANALYTICS</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_FILE_STORAGE">CATEGORY_FILE_STORAGE</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_PRIVACY">CATEGORY_PRIVACY</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_GAMING">CATEGORY_GAMING</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_DEVELOPER_TOOLS">CATEGORY_DEVELOPER_TOOLS</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_HARDWARE">CATEGORY_HARDWARE</a> ||
    category_bytes == <a href="../social_contracts/platform.md#social_contracts_platform_CATEGORY_RESEARCH">CATEGORY_RESEARCH</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_join_platform"></a>

## Function `join_platform`

Join a platform - establishes initial connection between wallet and platform
Checks for blocks before allowing the join and verifies platform is approved
Works with wallet addresses only, no profile required


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_join_platform">join_platform</a>(platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">social_contracts::block_list::BlockListRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_join_platform">join_platform</a>(
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    block_list_registry: &<a href="../social_contracts/block_list.md#social_contracts_block_list_BlockListRegistry">block_list::BlockListRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> platform_id = object::id(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    // Check <b>if</b> the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <b>has</b> blocked this wallet <b>address</b>
    <b>let</b> platform_address = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>assert</b>!(!<a href="../social_contracts/block_list.md#social_contracts_block_list_is_blocked">block_list::is_blocked</a>(block_list_registry, platform_address, caller), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Check <b>if</b> the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved by the contract owner (<b>use</b> registry)
    <b>let</b> platform_id_addr = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>(platform_registry, platform_id_addr), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Create joined wallets set <b>if</b> it doesn't exist
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_WALLETS_FIELD">JOINED_WALLETS_FIELD</a>)) {
        <b>let</b> joined_wallets = vec_set::empty&lt;<b>address</b>&gt;();
        dynamic_field::add(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_WALLETS_FIELD">JOINED_WALLETS_FIELD</a>, joined_wallets);
    };
    // Get joined wallets set
    <b>let</b> joined_wallets = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_WALLETS_FIELD">JOINED_WALLETS_FIELD</a>);
    // Check <b>if</b> wallet is already joined to the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>assert</b>!(!vec_set::contains(joined_wallets, &caller), <a href="../social_contracts/platform.md#social_contracts_platform_EAlreadyJoined">EAlreadyJoined</a>);
    // Add wallet to joined wallets
    vec_set::insert(joined_wallets, caller);
    // Emit event
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_UserJoinedPlatformEvent">UserJoinedPlatformEvent</a> {
        wallet_address: caller,
        platform_id,
        timestamp: current_time,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_leave_platform"></a>

## Function `leave_platform`

Leave a platform - removes the connection between wallet and platform


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_leave_platform">leave_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_leave_platform">leave_platform</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> platform_id = object::id(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>let</b> current_time = clock::timestamp_ms(clock);
    // Check <b>if</b> joined wallets set exists
    <b>assert</b>!(dynamic_field::exists_(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_WALLETS_FIELD">JOINED_WALLETS_FIELD</a>), <a href="../social_contracts/platform.md#social_contracts_platform_ENotJoined">ENotJoined</a>);
    // Get joined wallets set
    <b>let</b> joined_wallets = dynamic_field::borrow_mut&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_WALLETS_FIELD">JOINED_WALLETS_FIELD</a>);
    // Check <b>if</b> wallet is a member of the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>assert</b>!(vec_set::contains(joined_wallets, &caller), <a href="../social_contracts/platform.md#social_contracts_platform_ENotJoined">ENotJoined</a>);
    // Remove wallet from joined wallets
    vec_set::remove(joined_wallets, &caller);
    // Emit event
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_UserLeftPlatformEvent">UserLeftPlatformEvent</a> {
        wallet_address: caller,
        platform_id,
        timestamp: current_time,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_is_approved"></a>

## Function `is_approved`

Get platform approval status from registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, platform_id: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>, platform_id: <b>address</b>): bool {
    <b>if</b> (!table::contains(&registry.platform_approvals, platform_id)) {
        <b>return</b> <b>false</b>
    };
    *table::borrow(&registry.platform_approvals, platform_id)
}
</code></pre>



</details>

<a name="social_contracts_platform_has_joined_platform"></a>

## Function `has_joined_platform`

Check if a wallet address has joined a platform


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, wallet_address: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_has_joined_platform">has_joined_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>, wallet_address: <b>address</b>): bool {
    <b>if</b> (!dynamic_field::exists_(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_WALLETS_FIELD">JOINED_WALLETS_FIELD</a>)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> joined_wallets = dynamic_field::borrow&lt;vector&lt;u8&gt;, VecSet&lt;<b>address</b>&gt;&gt;(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>, <a href="../social_contracts/platform.md#social_contracts_platform_JOINED_WALLETS_FIELD">JOINED_WALLETS_FIELD</a>);
    vec_set::contains(joined_wallets, &wallet_address)
}
</code></pre>



</details>

<a name="social_contracts_platform_is_developer"></a>

## Function `is_developer`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_developer">is_developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_developer">is_developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>, addr: <b>address</b>): bool {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == addr
}
</code></pre>



</details>

<a name="social_contracts_platform_has_moderator_permission"></a>

## Function `has_moderator_permission`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_has_moderator_permission">has_moderator_permission</a>&lt;P: drop&gt;(group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_has_moderator_permission">has_moderator_permission</a>&lt;P: drop&gt;(
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    addr: <b>address</b>,
): bool {
    <a href="../social_contracts/platform.md#social_contracts_platform_is_developer">is_developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, addr) ||
        group.has_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, P&gt;(addr)
}
</code></pre>



</details>

<a name="social_contracts_platform_is_moderator"></a>

## Function `is_moderator`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_moderator">is_moderator</a>(group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_moderator">is_moderator</a>(
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    addr: <b>address</b>,
): bool {
    <b>if</b> (<a href="../social_contracts/platform.md#social_contracts_platform_is_developer">is_developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, addr)) {
        <b>return</b> <b>true</b>
    };
    <a href="../social_contracts/platform.md#social_contracts_platform_has_any_extension_permission">has_any_extension_permission</a>(group, addr)
}
</code></pre>



</details>

<a name="social_contracts_platform_is_developer_or_moderator"></a>

## Function `is_developer_or_moderator`

Backward-compatible helper: true if developer or has any extension permission


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">is_developer_or_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_is_developer_or_moderator">is_developer_or_moderator</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    addr: <b>address</b>,
): bool {
    <a href="../social_contracts/platform.md#social_contracts_platform_is_moderator">is_moderator</a>(group, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, addr)
}
</code></pre>



</details>

<a name="social_contracts_platform_moderators_group_id"></a>

## Function `moderators_group_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): ID {
    derived_object::derive_address(object::id(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>), <a href="../social_contracts/platform.md#social_contracts_platform_ModeratorsGroupTag">ModeratorsGroupTag</a>()).to_id()
}
</code></pre>



</details>

<a name="social_contracts_platform_assert_moderator_permission"></a>

## Function `assert_moderator_permission`



<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_assert_moderator_permission">assert_moderator_permission</a>&lt;P: drop&gt;(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_assert_moderator_permission">assert_moderator_permission</a>&lt;P: drop&gt;(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    addr: <b>address</b>,
) {
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_has_moderator_permission">has_moderator_permission</a>&lt;P&gt;(group, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, addr), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
}
</code></pre>



</details>

<a name="social_contracts_platform_grant_all_extension_permissions"></a>

## Function `grant_all_extension_permissions`



<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_grant_all_extension_permissions">grant_all_extension_permissions</a>(group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, member: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_grant_all_extension_permissions">grant_all_extension_permissions</a>(
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    member: <b>address</b>,
    ctx: &TxContext,
) {
    permissioned_group::grant_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_PlatformBlockAdmin">PlatformBlockAdmin</a>&gt;(group, member, ctx);
    permissioned_group::grant_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_PlatformBadgeAdmin">PlatformBadgeAdmin</a>&gt;(group, member, ctx);
    permissioned_group::grant_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_PlatformTreasuryAdmin">PlatformTreasuryAdmin</a>&gt;(group, member, ctx);
    permissioned_group::grant_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_PlatformContentModerator">PlatformContentModerator</a>&gt;(group, member, ctx);
    permissioned_group::grant_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_PlatformPromotionAdmin">PlatformPromotionAdmin</a>&gt;(group, member, ctx);
}
</code></pre>



</details>

<a name="social_contracts_platform_all_extension_permission_names"></a>

## Function `all_extension_permission_names`



<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_all_extension_permission_names">all_extension_permission_names</a>(): vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_all_extension_permission_names">all_extension_permission_names</a>(): vector&lt;String&gt; {
    vector[
        string::utf8(<a href="../social_contracts/platform.md#social_contracts_platform_PERM_BLOCK_ADMIN">PERM_BLOCK_ADMIN</a>),
        string::utf8(<a href="../social_contracts/platform.md#social_contracts_platform_PERM_BADGE_ADMIN">PERM_BADGE_ADMIN</a>),
        string::utf8(<a href="../social_contracts/platform.md#social_contracts_platform_PERM_TREASURY_ADMIN">PERM_TREASURY_ADMIN</a>),
        string::utf8(<a href="../social_contracts/platform.md#social_contracts_platform_PERM_CONTENT_MODERATOR">PERM_CONTENT_MODERATOR</a>),
        string::utf8(<a href="../social_contracts/platform.md#social_contracts_platform_PERM_PROMOTION_ADMIN">PERM_PROMOTION_ADMIN</a>),
    ]
}
</code></pre>



</details>

<a name="social_contracts_platform_permission_name"></a>

## Function `permission_name`



<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_permission_name">permission_name</a>&lt;P: drop&gt;(): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_permission_name">permission_name</a>&lt;P: drop&gt;(): String {
    <b>if</b> (type_name::with_original_ids&lt;P&gt;() == type_name::with_original_ids&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformBlockAdmin">PlatformBlockAdmin</a>&gt;()) {
        string::utf8(<a href="../social_contracts/platform.md#social_contracts_platform_PERM_BLOCK_ADMIN">PERM_BLOCK_ADMIN</a>)
    } <b>else</b> <b>if</b> (type_name::with_original_ids&lt;P&gt;() == type_name::with_original_ids&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformBadgeAdmin">PlatformBadgeAdmin</a>&gt;()) {
        string::utf8(<a href="../social_contracts/platform.md#social_contracts_platform_PERM_BADGE_ADMIN">PERM_BADGE_ADMIN</a>)
    } <b>else</b> <b>if</b> (type_name::with_original_ids&lt;P&gt;() == type_name::with_original_ids&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformTreasuryAdmin">PlatformTreasuryAdmin</a>&gt;()) {
        string::utf8(<a href="../social_contracts/platform.md#social_contracts_platform_PERM_TREASURY_ADMIN">PERM_TREASURY_ADMIN</a>)
    } <b>else</b> <b>if</b> (type_name::with_original_ids&lt;P&gt;() == type_name::with_original_ids&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformContentModerator">PlatformContentModerator</a>&gt;()) {
        string::utf8(<a href="../social_contracts/platform.md#social_contracts_platform_PERM_CONTENT_MODERATOR">PERM_CONTENT_MODERATOR</a>)
    } <b>else</b> <b>if</b> (type_name::with_original_ids&lt;P&gt;() == type_name::with_original_ids&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPromotionAdmin">PlatformPromotionAdmin</a>&gt;()) {
        string::utf8(<a href="../social_contracts/platform.md#social_contracts_platform_PERM_PROMOTION_ADMIN">PERM_PROMOTION_ADMIN</a>)
    } <b>else</b> {
        <b>abort</b> <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>
    }
}
</code></pre>



</details>

<a name="social_contracts_platform_has_any_extension_permission"></a>

## Function `has_any_extension_permission`



<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_has_any_extension_permission">has_any_extension_permission</a>(group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_has_any_extension_permission">has_any_extension_permission</a>(
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    addr: <b>address</b>,
): bool {
    group.has_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_PlatformBlockAdmin">PlatformBlockAdmin</a>&gt;(addr) ||
        group.has_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_PlatformBadgeAdmin">PlatformBadgeAdmin</a>&gt;(addr) ||
        group.has_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_PlatformTreasuryAdmin">PlatformTreasuryAdmin</a>&gt;(addr) ||
        group.has_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_PlatformContentModerator">PlatformContentModerator</a>&gt;(addr) ||
        group.has_permission&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>, <a href="../social_contracts/platform.md#social_contracts_platform_PlatformPromotionAdmin">PlatformPromotionAdmin</a>&gt;(addr)
}
</code></pre>



</details>

<a name="social_contracts_platform_emit_permissions_granted"></a>

## Function `emit_permissions_granted`



<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_emit_permissions_granted">emit_permissions_granted</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, member: <b>address</b>, permissions: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, granted_by: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_emit_permissions_granted">emit_permissions_granted</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>: ID,
    member: <b>address</b>,
    permissions: vector&lt;String&gt;,
    granted_by: <b>address</b>,
) {
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_ModeratorPermissionsGrantedEvent">ModeratorPermissionsGrantedEvent</a> {
        platform_id: object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>),
        <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>,
        member,
        permissions,
        granted_by,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_emit_permissions_revoked"></a>

## Function `emit_permissions_revoked`



<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_emit_permissions_revoked">emit_permissions_revoked</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, member: <b>address</b>, permissions: vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, revoked_by: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_emit_permissions_revoked">emit_permissions_revoked</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>: ID,
    member: <b>address</b>,
    permissions: vector&lt;String&gt;,
    revoked_by: <b>address</b>,
) {
    event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_ModeratorPermissionsRevokedEvent">ModeratorPermissionsRevokedEvent</a> {
        platform_id: object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>),
        <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>,
        member,
        permissions,
        revoked_by,
    });
}
</code></pre>



</details>

<a name="social_contracts_platform_maybe_emit_moderator_removed"></a>

## Function `maybe_emit_moderator_removed`



<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_maybe_emit_moderator_removed">maybe_emit_moderator_removed</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, member: <b>address</b>, removed_by: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_maybe_emit_moderator_removed">maybe_emit_moderator_removed</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    member: <b>address</b>,
    removed_by: <b>address</b>,
) {
    <b>if</b> (!group.is_member&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;(member)) {
        event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_ModeratorRemovedEvent">ModeratorRemovedEvent</a> {
            platform_id: object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>),
            <a href="../social_contracts/platform.md#social_contracts_platform_moderators_group_id">moderators_group_id</a>: object::id(group),
            member,
            removed_by,
        });
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_name"></a>

## Function `name`

Get platform name


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_tagline"></a>

## Function `tagline`

Get platform tagline


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_tagline">tagline</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_description"></a>

## Function `description`

Get platform description


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_description">description</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_logo"></a>

## Function `logo`

Get platform logo URL


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &String {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_logo">logo</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_cover_photo"></a>

## Function `cover_photo`

Get platform cover photo URL


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &Option&lt;String&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_media_previews"></a>

## Function `media_previews`

Get platform media preview URLs


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &Option&lt;vector&lt;String&gt;&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_developer"></a>

## Function `developer`

Get platform developer


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): <b>address</b> {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_terms_of_service"></a>

## Function `terms_of_service`

Get platform terms of service


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_terms_of_service">terms_of_service</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_privacy_policy"></a>

## Function `privacy_policy`

Get platform privacy policy


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_privacy_policy">privacy_policy</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_get_platforms"></a>

## Function `get_platforms`

Get platform platforms


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platforms">get_platforms</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platforms">get_platforms</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &vector&lt;String&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.platforms
}
</code></pre>



</details>

<a name="social_contracts_platform_get_links"></a>

## Function `get_links`

Get platform links


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_links">get_links</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &vector&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_links">get_links</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &vector&lt;String&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.links
}
</code></pre>



</details>

<a name="social_contracts_platform_primary_category"></a>

## Function `primary_category`

Get platform primary category


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_primary_category">primary_category</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_secondary_category"></a>

## Function `secondary_category`

Get platform secondary category


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &Option&lt;String&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_secondary_category">secondary_category</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_status"></a>

## Function `status`

Get platform status


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): u8 {
    <a href="../social_contracts/platform.md#social_contracts_platform_status_value">status_value</a>(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_status">status</a>)
}
</code></pre>



</details>

<a name="social_contracts_platform_release_date"></a>

## Function `release_date`

Get platform release date


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): String {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_release_date">release_date</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_shutdown_date"></a>

## Function `shutdown_date`

Get platform shutdown date


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &Option&lt;String&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_shutdown_date">shutdown_date</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_created_at"></a>

## Function `created_at`

Get platform creation timestamp


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_created_at">created_at</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_created_at">created_at</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): u64 {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_created_at">created_at</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_treasury_balance"></a>

## Function `treasury_balance`

Get platform treasury balance


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_treasury_balance">treasury_balance</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_treasury_balance">treasury_balance</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): u64 {
    balance::value(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.treasury)
}
</code></pre>



</details>

<a name="social_contracts_platform_id"></a>

## Function `id`

Get platform ID


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &UID {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_get_platform_by_name"></a>

## Function `get_platform_by_name`

Get platform by name from registry


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platform_by_name">get_platform_by_name</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: <a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platform_by_name">get_platform_by_name</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>: String): Option&lt;<b>address</b>&gt; {
    <b>if</b> (!table::contains(&registry.platforms_by_name, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>)) {
        <b>return</b> option::none()
    };
    option::some(*table::borrow(&registry.platforms_by_name, <a href="../social_contracts/platform.md#social_contracts_platform_name">name</a>))
}
</code></pre>



</details>

<a name="social_contracts_platform_get_platforms_by_developer"></a>

## Function `get_platforms_by_developer`

Get platforms owned by a developer


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platforms_by_developer">get_platforms_by_developer</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>: <b>address</b>): vector&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_get_platforms_by_developer">get_platforms_by_developer</a>(registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>: <b>address</b>): vector&lt;<b>address</b>&gt; {
    <b>if</b> (!table::contains(&registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>)) {
        <b>return</b> vector::empty()
    };
    *table::borrow(&registry.platforms_by_developer, <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>)
}
</code></pre>



</details>

<a name="social_contracts_platform_wants_dao_governance"></a>

## Function `wants_dao_governance`

Check if platform wants DAO governance


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): bool {
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_wants_dao_governance">wants_dao_governance</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_governance_registry_id"></a>

## Function `governance_registry_id`

Get platform's governance registry ID if available


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): &Option&lt;ID&gt; {
    &<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>
}
</code></pre>



</details>

<a name="social_contracts_platform_governance_parameters"></a>

## Function `governance_parameters`

Get platform's governance parameters


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_governance_parameters">governance_parameters</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>): (<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_governance_parameters">governance_parameters</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>): (Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;, Option&lt;u64&gt;) {
    (
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_count,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.delegate_term_epochs,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.proposal_submission_cost,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.max_votes_per_user,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quadratic_base_cost,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.voting_period_epochs,
        <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.quorum_votes
    )
}
</code></pre>



</details>

<a name="social_contracts_platform_update_platform_governance"></a>

## Function `update_platform_governance`

Update governance parameters for this platform's governance registry
Can only be called by the platform developer


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_update_platform_governance">update_platform_governance</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">social_contracts::governance::GovernanceDAO</a>, delegate_count: u64, delegate_term_epochs: u64, proposal_submission_cost: u64, max_votes_per_user: u64, quadratic_base_cost: u64, voting_period_epochs: u64, quorum_votes: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_update_platform_governance">update_platform_governance</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    registry: &<b>mut</b> <a href="../social_contracts/governance.md#social_contracts_governance_GovernanceDAO">governance::GovernanceDAO</a>,
    delegate_count: u64,
    delegate_term_epochs: u64,
    proposal_submission_cost: u64,
    max_votes_per_user: u64,
    quadratic_base_cost: u64,
    voting_period_epochs: u64,
    quorum_votes: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Verify caller is <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>
    <b>let</b> caller = tx_context::sender(ctx);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>) == caller, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Verify that the <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>'s <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a> matches this registry
    // This ensures the registry actually belongs to this <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>
    <b>let</b> platform_registry_id_opt = <a href="../social_contracts/platform.md#social_contracts_platform_governance_registry_id">governance_registry_id</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <b>assert</b>!(option::is_some(platform_registry_id_opt), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <b>let</b> platform_registry_id = *option::borrow(platform_registry_id_opt);
    <b>let</b> registry_id = object::id(registry);
    <b>assert</b>!(platform_registry_id == registry_id, <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Call <a href="../social_contracts/governance.md#social_contracts_governance">governance</a> function with verified <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> <b>address</b>
    <a href="../social_contracts/governance.md#social_contracts_governance_update_platform_governance_parameters">governance::update_platform_governance_parameters</a>(
        registry,
        <a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>),
        delegate_count,
        delegate_term_epochs,
        proposal_submission_cost,
        max_votes_per_user,
        quadratic_base_cost,
        voting_period_epochs,
        quorum_votes,
        clock,
        ctx
    );
}
</code></pre>



</details>

<a name="social_contracts_platform_airdrop_from_treasury"></a>

## Function `airdrop_from_treasury`

Airdrop tokens to multiple recipients from the platform treasury
Can only be called by platform developer or moderator with treasury permission


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_airdrop_from_treasury">airdrop_from_treasury</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, recipients: vector&lt;<b>address</b>&gt;, amount_per_recipient: u64, reason_code: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_airdrop_from_treasury">airdrop_from_treasury</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    recipients: vector&lt;<b>address</b>&gt;,
    amount_per_recipient: u64,
    reason_code: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <a href="../social_contracts/platform.md#social_contracts_platform_assert_moderator_permission">assert_moderator_permission</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformTreasuryAdmin">PlatformTreasuryAdmin</a>&gt;(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, group, caller);
    // Check that recipients list is not empty
    <b>let</b> recipients_count = vector::length(&recipients);
    <b>assert</b>!(recipients_count &gt; 0, <a href="../social_contracts/platform.md#social_contracts_platform_EEmptyRecipientsList">EEmptyRecipientsList</a>);
    // Calculate total amount needed
    <b>let</b> total_amount = amount_per_recipient * recipients_count;
    // Verify <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury <b>has</b> enough funds
    <b>assert</b>!(balance::value(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.treasury) &gt;= total_amount, <a href="../social_contracts/platform.md#social_contracts_platform_EInsufficientTreasuryFunds">EInsufficientTreasuryFunds</a>);
    // Get current timestamp <b>for</b> events
    <b>let</b> current_time = clock::timestamp_ms(clock);
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    // Send tokens to each recipient
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; recipients_count) {
        <b>let</b> recipient = *vector::borrow(&recipients, i);
        // Create coin from <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> treasury balance
        <b>let</b> airdrop_coin = coin::from_balance(
            balance::split(&<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.treasury, amount_per_recipient),
            ctx
        );
        // Transfer to recipient
        transfer::public_transfer(airdrop_coin, recipient);
        // Emit airdrop event <b>for</b> tracking
        event::emit(<a href="../social_contracts/platform.md#social_contracts_platform_TokenAirdropEvent">TokenAirdropEvent</a> {
            platform_id,
            recipient,
            amount: amount_per_recipient,
            reason_code,
            executed_by: caller,
            timestamp: current_time,
        });
        i = i + 1;
    };
}
</code></pre>



</details>

<a name="social_contracts_platform_assign_badge"></a>

## Function `assign_badge`

Assign a badge to a profile - can only be called by platform admin/moderator
This is the primary entry point for badge assignment


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_assign_badge">assign_badge</a>(platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, badge_name: <a href="../std/string.md#std_string_String">std::string::String</a>, badge_description: <a href="../std/string.md#std_string_String">std::string::String</a>, badge_media_url: <a href="../std/string.md#std_string_String">std::string::String</a>, badge_icon_url: <a href="../std/string.md#std_string_String">std::string::String</a>, badge_type: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_assign_badge">assign_badge</a>(
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">profile::Profile</a>,
    badge_name: String,
    badge_description: String,
    badge_media_url: String,
    badge_icon_url: String,
    badge_type: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <a href="../social_contracts/platform.md#social_contracts_platform_assert_moderator_permission">assert_moderator_permission</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformBadgeAdmin">PlatformBadgeAdmin</a>&gt;(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, group, caller);
    // Verify <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Validate badge type (1-100 <b>as</b> documented)
    <b>assert</b>!(badge_type &gt;= 1 && badge_type &lt;= 100, <a href="../social_contracts/platform.md#social_contracts_platform_EInvalidBadgeType">EInvalidBadgeType</a>);
    // Validate badge field lengths
    <b>assert</b>!(string::length(&badge_name) &gt; 0 && string::length(&badge_name) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_NAME_LENGTH">MAX_BADGE_NAME_LENGTH</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeNameTooLong">EBadgeNameTooLong</a>);
    <b>assert</b>!(string::length(&badge_description) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_DESCRIPTION_LENGTH">MAX_BADGE_DESCRIPTION_LENGTH</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeDescriptionTooLong">EBadgeDescriptionTooLong</a>);
    <b>assert</b>!(string::length(&badge_media_url) &gt; 0 && string::length(&badge_media_url) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_MEDIA_URL_LENGTH">MAX_BADGE_MEDIA_URL_LENGTH</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeMediaUrlTooLong">EBadgeMediaUrlTooLong</a>);
    <b>assert</b>!(string::length(&badge_icon_url) &gt; 0 && string::length(&badge_icon_url) &lt;= <a href="../social_contracts/platform.md#social_contracts_platform_MAX_BADGE_ICON_URL_LENGTH">MAX_BADGE_ICON_URL_LENGTH</a>, <a href="../social_contracts/platform.md#social_contracts_platform_EBadgeIconUrlTooLong">EBadgeIconUrlTooLong</a>);
    // Get current time
    <b>let</b> now = clock::timestamp_ms(clock);
    // Create a unique badge ID by including <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> ID to prevent collisions
    <b>let</b> <b>mut</b> badge_id = string::utf8(b"badge_");
    // Convert <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> ID to hex string and append to ensure uniqueness
    <b>let</b> platform_id_str = <a href="../myso/address.md#myso_address_to_string">myso::address::to_string</a>(platform_id);
    string::append(&<b>mut</b> badge_id, platform_id_str);
    string::append(&<b>mut</b> badge_id, string::utf8(b"_"));
    string::append(&<b>mut</b> badge_id, badge_name);
    // Add the badge directly to the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <a href="../social_contracts/profile.md#social_contracts_profile_add_badge_to_profile">profile::add_badge_to_profile</a>(
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>,
        badge_id,
        badge_name,
        badge_description,
        badge_media_url,
        badge_icon_url,
        platform_id,
        now,
        caller,
        badge_type
    );
}
</code></pre>



</details>

<a name="social_contracts_platform_revoke_badge"></a>

## Function `revoke_badge`

Revoke a badge from a profile - can only be called by platform admin/moderator
This is the primary entry point for badge revocation


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_revoke_badge">revoke_badge</a>(platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a>, badge_id: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_revoke_badge">revoke_badge</a>(
    platform_registry: &<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_Profile">profile::Profile</a>,
    badge_id: String,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    // Check version compatibility
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <a href="../social_contracts/platform.md#social_contracts_platform_assert_moderator_permission">assert_moderator_permission</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformBadgeAdmin">PlatformBadgeAdmin</a>&gt;(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, group, caller);
    // Verify <a href="../social_contracts/platform.md#social_contracts_platform">platform</a> is approved
    <b>let</b> platform_id = object::uid_to_address(&<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>);
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform_is_approved">is_approved</a>(platform_registry, platform_id), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    // Get current time
    <b>let</b> now = clock::timestamp_ms(clock);
    // Remove the badge directly from the <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>
    <a href="../social_contracts/profile.md#social_contracts_profile_remove_badge_from_profile">profile::remove_badge_from_profile</a>(
        <a href="../social_contracts/profile.md#social_contracts_profile">profile</a>,
        &badge_id,
        platform_id,
        caller,
        now
    );
}
</code></pre>



</details>

<a name="social_contracts_platform_add_moderator_register"></a>

## Function `add_moderator_register`

When adding a moderator to a platform, register them with the profile module


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_moderator_register">add_moderator_register</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, moderator_address: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_add_moderator_register">add_moderator_register</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    moderator_address: <b>address</b>,
    ctx: &TxContext
) {
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_developer">developer</a> == tx_context::sender(ctx), <a href="../social_contracts/platform.md#social_contracts_platform_EUnauthorized">EUnauthorized</a>);
    <a href="../social_contracts/platform.md#social_contracts_platform_grant_all_moderator_permissions">grant_all_moderator_permissions</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, group, moderator_address, ctx);
}
</code></pre>



</details>

<a name="social_contracts_platform_remove_moderator_unregister"></a>

## Function `remove_moderator_unregister`

When removing a moderator from a platform


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_remove_moderator_unregister">remove_moderator_unregister</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">social_contracts::platform::PlatformPackage</a>&gt;, moderator_address: <b>address</b>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_remove_moderator_unregister">remove_moderator_unregister</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/platform.md#social_contracts_platform_PlatformPackage">PlatformPackage</a>&gt;,
    moderator_address: <b>address</b>,
    ctx: &TxContext
) {
    <a href="../social_contracts/platform.md#social_contracts_platform_remove_moderator">remove_moderator</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>, group, moderator_address, ctx);
}
</code></pre>



</details>

<a name="social_contracts_platform_create_platform_admin_cap"></a>

## Function `create_platform_admin_cap`

Create a PlatformAdminCap for bootstrap (package visibility only)
This function is only callable by other modules in the same package


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_create_platform_admin_cap">create_platform_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">social_contracts::platform::PlatformAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_create_platform_admin_cap">create_platform_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">PlatformAdminCap</a> {
    <a href="../social_contracts/platform.md#social_contracts_platform_PlatformAdminCap">PlatformAdminCap</a> {
        <a href="../social_contracts/platform.md#social_contracts_platform_id">id</a>: object::new(ctx)
    }
}
</code></pre>



</details>

<a name="social_contracts_platform_migrate_platform"></a>

## Function `migrate_platform`

Migration function for Platform


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_migrate_platform">migrate_platform</a>(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">social_contracts::platform::Platform</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_migrate_platform">migrate_platform</a>(
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version &lt; current_version, <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version;
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_cover_photo">cover_photo</a> = option::none();
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.<a href="../social_contracts/platform.md#social_contracts_platform_media_previews">media_previews</a> = option::none();
    <a href="../social_contracts/platform.md#social_contracts_platform">platform</a>.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> platform_id = object::id(<a href="../social_contracts/platform.md#social_contracts_platform">platform</a>);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        platform_id,
        string::utf8(b"<a href="../social_contracts/platform.md#social_contracts_platform_Platform">Platform</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_platform_migrate_registry"></a>

## Function `migrate_registry`

Migration function for PlatformRegistry


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_migrate_registry">migrate_registry</a>(registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">social_contracts::platform::PlatformRegistry</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/platform.md#social_contracts_platform_migrate_registry">migrate_registry</a>(
    registry: &<b>mut</b> <a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    // Verify this is an <a href="../social_contracts/upgrade.md#social_contracts_upgrade">upgrade</a> (new version &gt; current version)
    <b>assert</b>!(registry.version &lt; current_version, <a href="../social_contracts/platform.md#social_contracts_platform_EWrongVersion">EWrongVersion</a>);
    // Remember old version and update to new version
    <b>let</b> old_version = registry.version;
    registry.version = current_version;
    // Emit event <b>for</b> object migration
    <b>let</b> registry_id = object::id(registry);
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        registry_id,
        string::utf8(b"<a href="../social_contracts/platform.md#social_contracts_platform_PlatformRegistry">PlatformRegistry</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>
