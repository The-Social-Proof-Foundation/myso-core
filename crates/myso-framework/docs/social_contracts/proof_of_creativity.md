---
title: Module `social_contracts::poc_username_beneficiary`
---



-  [Struct `PoCBeneficiaryAdminCap`](#social_contracts_poc_username_beneficiary_PoCBeneficiaryAdminCap)
-  [Struct `CreatorIdentityKey`](#social_contracts_poc_username_beneficiary_CreatorIdentityKey)
-  [Struct `VerificationRequirements`](#social_contracts_poc_username_beneficiary_VerificationRequirements)
-  [Struct `PoCUsernameBeneficiaryDirectory`](#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory)
-  [Struct `PoCUsernameBeneficiaryShard`](#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard)
-  [Struct `PoCUsernameBeneficiary`](#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary)
-  [Struct `UsernameBeneficiaryProvisionedEvent`](#social_contracts_poc_username_beneficiary_UsernameBeneficiaryProvisionedEvent)
-  [Struct `UsernameBeneficiaryClaimedEvent`](#social_contracts_poc_username_beneficiary_UsernameBeneficiaryClaimedEvent)
-  [Struct `UsernameBeneficiaryEndedEvent`](#social_contracts_poc_username_beneficiary_UsernameBeneficiaryEndedEvent)
-  [Struct `UsernameBeneficiaryConflictEvent`](#social_contracts_poc_username_beneficiary_UsernameBeneficiaryConflictEvent)
-  [Struct `CreatorIdentityWalletLinkedEvent`](#social_contracts_poc_username_beneficiary_CreatorIdentityWalletLinkedEvent)
-  [Constants](#@Constants_0)
-  [Function `assert_directory_version`](#social_contracts_poc_username_beneficiary_assert_directory_version)
-  [Function `assert_shard_version`](#social_contracts_poc_username_beneficiary_assert_shard_version)
-  [Function `assert_beneficiary_version`](#social_contracts_poc_username_beneficiary_assert_beneficiary_version)
-  [Function `create_beneficiary_admin_cap`](#social_contracts_poc_username_beneficiary_create_beneficiary_admin_cap)
-  [Function `bootstrap_init_directory`](#social_contracts_poc_username_beneficiary_bootstrap_init_directory)
-  [Function `is_username_beneficiary_active`](#social_contracts_poc_username_beneficiary_is_username_beneficiary_active)
-  [Function `shard_index_for_username`](#social_contracts_poc_username_beneficiary_shard_index_for_username)
-  [Function `assert_shard_matches_username`](#social_contracts_poc_username_beneficiary_assert_shard_matches_username)
-  [Function `identity_beneficiary_address`](#social_contracts_poc_username_beneficiary_identity_beneficiary_address)
-  [Function `canonical_username`](#social_contracts_poc_username_beneficiary_canonical_username)
-  [Function `canonical_x_handle`](#social_contracts_poc_username_beneficiary_canonical_x_handle)
-  [Function `beneficiary_status`](#social_contracts_poc_username_beneficiary_beneficiary_status)
-  [Function `identity_key`](#social_contracts_poc_username_beneficiary_identity_key)
-  [Function `emit_username_conflict`](#social_contracts_poc_username_beneficiary_emit_username_conflict)
-  [Function `create_username_beneficiary`](#social_contracts_poc_username_beneficiary_create_username_beneficiary)
-  [Function `claim_username_beneficiary`](#social_contracts_poc_username_beneficiary_claim_username_beneficiary)
-  [Function `claim_username_beneficiary_vault_balance`](#social_contracts_poc_username_beneficiary_claim_username_beneficiary_vault_balance)
-  [Function `end_username_beneficiary`](#social_contracts_poc_username_beneficiary_end_username_beneficiary)
-  [Function `migrate_poc_username_beneficiary_directory`](#social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary_directory)
-  [Function `migrate_poc_username_beneficiary_shard`](#social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary_shard)
-  [Function `migrate_poc_username_beneficiary`](#social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
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
<b>use</b> <a href="../myso/ed25519.md#myso_ed25519">myso::ed25519</a>;
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
<b>use</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit">social_contracts::ai_credit</a>;
<b>use</b> <a href="../social_contracts/memory.md#social_contracts_memory">social_contracts::memory</a>;
<b>use</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault">social_contracts::poc_vault</a>;
<b>use</b> <a href="../social_contracts/profile.md#social_contracts_profile">social_contracts::profile</a>;
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



<a name="social_contracts_poc_username_beneficiary_PoCBeneficiaryAdminCap"></a>

## Struct `PoCBeneficiaryAdminCap`

Admin capability for username beneficiary provisioning lifecycle.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCBeneficiaryAdminCap">PoCBeneficiaryAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_poc_username_beneficiary_CreatorIdentityKey"></a>

## Struct `CreatorIdentityKey`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityKey">CreatorIdentityKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>source: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>identity_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_poc_username_beneficiary_VerificationRequirements"></a>

## Struct `VerificationRequirements`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_VerificationRequirements">VerificationRequirements</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>required_x_handle: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory"></a>

## Struct `PoCUsernameBeneficiaryDirectory`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">PoCUsernameBeneficiaryDirectory</a> <b>has</b> key
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
<code>shard_ids: vector&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>beneficiary_by_identity: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityKey">social_contracts::poc_username_beneficiary::CreatorIdentityKey</a>, <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>wallet_by_identity: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityKey">social_contracts::poc_username_beneficiary::CreatorIdentityKey</a>, <b>address</b>&gt;</code>
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

<a name="social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard"></a>

## Struct `PoCUsernameBeneficiaryShard`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">PoCUsernameBeneficiaryShard</a> <b>has</b> key
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
<code>shard_index: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>username_to_beneficiary: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>, <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
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

<a name="social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary"></a>

## Struct `PoCUsernameBeneficiary`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">PoCUsernameBeneficiary</a> <b>has</b> key
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
<code>username: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>creator_identity: <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityKey">social_contracts::poc_username_beneficiary::CreatorIdentityKey</a></code>
</dt>
<dd>
</dd>
<dt>
<code>verification: <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_VerificationRequirements">social_contracts::poc_username_beneficiary::VerificationRequirements</a></code>
</dt>
<dd>
</dd>
<dt>
<code>provisioned_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>status: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_profile_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_by: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>ended_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>ended_by: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>end_reason_code: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>oracle_evidence_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>vault_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>beneficiary_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>provisioned_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>join_referral_paid: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>join_referrer: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>join_referral_paid_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
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

<a name="social_contracts_poc_username_beneficiary_UsernameBeneficiaryProvisionedEvent"></a>

## Struct `UsernameBeneficiaryProvisionedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_UsernameBeneficiaryProvisionedEvent">UsernameBeneficiaryProvisionedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>beneficiary_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>username: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>creator_identity_source: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_identity_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>required_x_handle: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>beneficiary_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>vault_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>provisioned_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>provisioned_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_poc_username_beneficiary_UsernameBeneficiaryClaimedEvent"></a>

## Struct `UsernameBeneficiaryClaimedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_UsernameBeneficiaryClaimedEvent">UsernameBeneficiaryClaimedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>beneficiary_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>username: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>claimed_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>wallet: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>oracle_evidence_hash: vector&lt;u8&gt;</code>
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

<a name="social_contracts_poc_username_beneficiary_UsernameBeneficiaryEndedEvent"></a>

## Struct `UsernameBeneficiaryEndedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_UsernameBeneficiaryEndedEvent">UsernameBeneficiaryEndedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>beneficiary_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>username: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>ended_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>end_reason_code: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>swept_mys_amount: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>ended_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_poc_username_beneficiary_UsernameBeneficiaryConflictEvent"></a>

## Struct `UsernameBeneficiaryConflictEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_UsernameBeneficiaryConflictEvent">UsernameBeneficiaryConflictEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>username: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>existing_beneficiary_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>attempted_by: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_poc_username_beneficiary_CreatorIdentityWalletLinkedEvent"></a>

## Struct `CreatorIdentityWalletLinkedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityWalletLinkedEvent">CreatorIdentityWalletLinkedEvent</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>creator_identity_source: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>creator_identity_hash: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>wallet: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>beneficiary_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>linked_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_0"></a>

## Constants


<a name="social_contracts_poc_username_beneficiary_NUM_SHARDS"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_NUM_SHARDS">NUM_SHARDS</a>: u64 = 256;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_STATUS_ACTIVE"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_STATUS_ACTIVE">STATUS_ACTIVE</a>: u8 = 1;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_STATUS_CLAIMED"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_STATUS_CLAIMED">STATUS_CLAIMED</a>: u8 = 2;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_STATUS_ENDED"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_STATUS_ENDED">STATUS_ENDED</a>: u8 = 3;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_IDENTITY_SOURCE_X"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_IDENTITY_SOURCE_X">IDENTITY_SOURCE_X</a>: u8 = 1;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_END_REASON_ADMIN"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_END_REASON_ADMIN">END_REASON_ADMIN</a>: u8 = 1;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EUnauthorized"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EUnauthorized">EUnauthorized</a>: u64 = 1;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EUsernameNotAvailable"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EUsernameNotAvailable">EUsernameNotAvailable</a>: u64 = 2;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EIdentityAlreadyProvisioned"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EIdentityAlreadyProvisioned">EIdentityAlreadyProvisioned</a>: u64 = 3;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EInvalidStatus"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EInvalidStatus">EInvalidStatus</a>: u64 = 4;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EVerificationFailed"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EVerificationFailed">EVerificationFailed</a>: u64 = 5;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EWalletAlreadyLinked"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWalletAlreadyLinked">EWalletAlreadyLinked</a>: u64 = 6;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EWalletNotLinked"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWalletNotLinked">EWalletNotLinked</a>: u64 = 7;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EJoinReferralAlreadyPaid"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EJoinReferralAlreadyPaid">EJoinReferralAlreadyPaid</a>: u64 = 8;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EInvalidIdentitySource"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EInvalidIdentitySource">EInvalidIdentitySource</a>: u64 = 9;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EInvalidUsername"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EInvalidUsername">EInvalidUsername</a>: u64 = 10;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWrongVersion">EWrongVersion</a>: u64 = 11;
</code></pre>



<a name="social_contracts_poc_username_beneficiary_assert_directory_version"></a>

## Function `assert_directory_version`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_directory_version">assert_directory_version</a>(directory: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryDirectory</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_directory_version">assert_directory_version</a>(directory: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">PoCUsernameBeneficiaryDirectory</a>) {
    <b>assert</b>!(directory.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_assert_shard_version"></a>

## Function `assert_shard_version`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_shard_version">assert_shard_version</a>(shard: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryShard</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_shard_version">assert_shard_version</a>(shard: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">PoCUsernameBeneficiaryShard</a>) {
    <b>assert</b>!(shard.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_assert_beneficiary_version"></a>

## Function `assert_beneficiary_version`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_beneficiary_version">assert_beneficiary_version</a>(beneficiary: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiary</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_beneficiary_version">assert_beneficiary_version</a>(beneficiary: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">PoCUsernameBeneficiary</a>) {
    <b>assert</b>!(beneficiary.version == <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(), <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_create_beneficiary_admin_cap"></a>

## Function `create_beneficiary_admin_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_create_beneficiary_admin_cap">create_beneficiary_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCBeneficiaryAdminCap">social_contracts::poc_username_beneficiary::PoCBeneficiaryAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_create_beneficiary_admin_cap">create_beneficiary_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCBeneficiaryAdminCap">PoCBeneficiaryAdminCap</a> {
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCBeneficiaryAdminCap">PoCBeneficiaryAdminCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_bootstrap_init_directory"></a>

## Function `bootstrap_init_directory`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_bootstrap_init_directory">bootstrap_init_directory</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_bootstrap_init_directory">bootstrap_init_directory</a>(ctx: &<b>mut</b> TxContext) {
    <b>let</b> <b>mut</b> shard_ids = vector::empty&lt;ID&gt;();
    <b>let</b> <b>mut</b> i = 0u64;
    <b>while</b> (i &lt; <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_NUM_SHARDS">NUM_SHARDS</a>) {
        <b>let</b> shard = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">PoCUsernameBeneficiaryShard</a> {
            id: object::new(ctx),
            shard_index: i,
            username_to_beneficiary: table::new(ctx),
            version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
        };
        <b>let</b> shard_id = object::id(&shard);
        transfer::share_object(shard);
        vector::push_back(&<b>mut</b> shard_ids, shard_id);
        i = i + 1;
    };
    transfer::share_object(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">PoCUsernameBeneficiaryDirectory</a> {
        id: object::new(ctx),
        shard_ids,
        beneficiary_by_identity: table::new(ctx),
        wallet_by_identity: table::new(ctx),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    });
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_is_username_beneficiary_active"></a>

## Function `is_username_beneficiary_active`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_is_username_beneficiary_active">is_username_beneficiary_active</a>(registry: &<a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, username: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_is_username_beneficiary_active">is_username_beneficiary_active</a>(
    registry: &UsernameRegistry,
    username: &String,
): bool {
    <a href="../social_contracts/profile.md#social_contracts_profile_is_username_beneficiary_locked">profile::is_username_beneficiary_locked</a>(registry, username)
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_shard_index_for_username"></a>

## Function `shard_index_for_username`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_shard_index_for_username">shard_index_for_username</a>(username: &<a href="../std/string.md#std_string_String">std::string::String</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_shard_index_for_username">shard_index_for_username</a>(username: &String): u64 {
    <b>let</b> bytes = string::as_bytes(username);
    <b>let</b> h = myso_hash::blake2b256(bytes);
    <b>let</b> first = *vector::borrow(&h, 0);
    (first <b>as</b> u64) % <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_NUM_SHARDS">NUM_SHARDS</a>
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_assert_shard_matches_username"></a>

## Function `assert_shard_matches_username`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_shard_matches_username">assert_shard_matches_username</a>(shard: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryShard</a>, username: &<a href="../std/string.md#std_string_String">std::string::String</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_shard_matches_username">assert_shard_matches_username</a>(shard: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">PoCUsernameBeneficiaryShard</a>, username: &String) {
    <b>assert</b>!(shard.shard_index == <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_shard_index_for_username">shard_index_for_username</a>(username), <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EUnauthorized">EUnauthorized</a>);
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_identity_beneficiary_address"></a>

## Function `identity_beneficiary_address`



<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_identity_beneficiary_address">identity_beneficiary_address</a>(key: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityKey">social_contracts::poc_username_beneficiary::CreatorIdentityKey</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_identity_beneficiary_address">identity_beneficiary_address</a>(key: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityKey">CreatorIdentityKey</a>): <b>address</b> {
    <b>let</b> <b>mut</b> data = vector::empty&lt;u8&gt;();
    vector::push_back(&<b>mut</b> data, key.source);
    vector::append(&<b>mut</b> data, key.identity_hash);
    object::id_to_address(&object::id_from_bytes(myso_hash::blake2b256(&data)))
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_canonical_username"></a>

## Function `canonical_username`



<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_canonical_username">canonical_username</a>(username: vector&lt;u8&gt;): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_canonical_username">canonical_username</a>(username: vector&lt;u8&gt;): String {
    <a href="../social_contracts/profile.md#social_contracts_profile_canonical_registry_username_from_bytes">profile::canonical_registry_username_from_bytes</a>(username)
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_canonical_x_handle"></a>

## Function `canonical_x_handle`



<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_canonical_x_handle">canonical_x_handle</a>(handle: vector&lt;u8&gt;): <a href="../std/string.md#std_string_String">std::string::String</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_canonical_x_handle">canonical_x_handle</a>(handle: vector&lt;u8&gt;): String {
    <a href="../social_contracts/profile.md#social_contracts_profile_canonical_registry_username_from_bytes">profile::canonical_registry_username_from_bytes</a>(handle)
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_beneficiary_status"></a>

## Function `beneficiary_status`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_beneficiary_status">beneficiary_status</a>(beneficiary: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiary</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_beneficiary_status">beneficiary_status</a>(beneficiary: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">PoCUsernameBeneficiary</a>): u8 {
    beneficiary.status
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_identity_key"></a>

## Function `identity_key`



<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_identity_key">identity_key</a>(source: u8, identity_hash: vector&lt;u8&gt;): <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityKey">social_contracts::poc_username_beneficiary::CreatorIdentityKey</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_identity_key">identity_key</a>(source: u8, identity_hash: vector&lt;u8&gt;): <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityKey">CreatorIdentityKey</a> {
    <b>assert</b>!(source == <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_IDENTITY_SOURCE_X">IDENTITY_SOURCE_X</a>, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EInvalidIdentitySource">EInvalidIdentitySource</a>);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityKey">CreatorIdentityKey</a> { source, identity_hash }
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_emit_username_conflict"></a>

## Function `emit_username_conflict`



<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_emit_username_conflict">emit_username_conflict</a>(username: <a href="../std/string.md#std_string_String">std::string::String</a>, existing_beneficiary_id: <b>address</b>, attempted_by: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_emit_username_conflict">emit_username_conflict</a>(
    username: String,
    existing_beneficiary_id: <b>address</b>,
    attempted_by: <b>address</b>,
) {
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_UsernameBeneficiaryConflictEvent">UsernameBeneficiaryConflictEvent</a> {
        username,
        existing_beneficiary_id,
        attempted_by,
    });
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_create_username_beneficiary"></a>

## Function `create_username_beneficiary`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_create_username_beneficiary">create_username_beneficiary</a>(directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryDirectory</a>, shard: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryShard</a>, vault_directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCVaultDirectory">social_contracts::poc_vault::PoCVaultDirectory</a>, username_registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, username: vector&lt;u8&gt;, identity_source: u8, identity_hash: vector&lt;u8&gt;, required_x_handle: vector&lt;u8&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_create_username_beneficiary">create_username_beneficiary</a>(
    directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">PoCUsernameBeneficiaryDirectory</a>,
    shard: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">PoCUsernameBeneficiaryShard</a>,
    vault_directory: &<b>mut</b> PoCVaultDirectory,
    username_registry: &<b>mut</b> UsernameRegistry,
    username: vector&lt;u8&gt;,
    identity_source: u8,
    identity_hash: vector&lt;u8&gt;,
    required_x_handle: vector&lt;u8&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_directory_version">assert_directory_version</a>(directory);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_shard_version">assert_shard_version</a>(shard);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_assert_vault_directory_version">poc_vault::assert_vault_directory_version</a>(vault_directory);
    <b>let</b> username = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_canonical_username">canonical_username</a>(username);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_shard_matches_username">assert_shard_matches_username</a>(shard, &username);
    <b>let</b> username_len = vector::length(string::as_bytes(&username));
    <b>assert</b>!(username_len &gt;= 2 && username_len &lt;= 50, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EInvalidUsername">EInvalidUsername</a>);
    <b>let</b> required_x_handle = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_canonical_x_handle">canonical_x_handle</a>(required_x_handle);
    <b>let</b> creator_identity = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_identity_key">identity_key</a>(identity_source, identity_hash);
    <b>let</b> provisioned_by = tx_context::sender(ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>assert</b>!(<a href="../social_contracts/profile.md#social_contracts_profile_is_username_available">profile::is_username_available</a>(username_registry, username), <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EUsernameNotAvailable">EUsernameNotAvailable</a>);
    <b>if</b> (table::contains(&shard.username_to_beneficiary, username)) {
        <b>let</b> existing_id = *table::borrow(&shard.username_to_beneficiary, username);
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_emit_username_conflict">emit_username_conflict</a>(
            username,
            object::id_to_address(&existing_id),
            provisioned_by,
        );
        <b>abort</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EUsernameNotAvailable">EUsernameNotAvailable</a>
    };
    <b>if</b> (table::contains(&directory.beneficiary_by_identity, creator_identity)) {
        <b>let</b> existing_id = *table::borrow(&directory.beneficiary_by_identity, creator_identity);
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_emit_username_conflict">emit_username_conflict</a>(
            username,
            object::id_to_address(&existing_id),
            provisioned_by,
        );
        <b>abort</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EIdentityAlreadyProvisioned">EIdentityAlreadyProvisioned</a>
    };
    <a href="../social_contracts/profile.md#social_contracts_profile_lock_username_for_beneficiary">profile::lock_username_for_beneficiary</a>(username_registry, username);
    <b>let</b> beneficiary_address = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_identity_beneficiary_address">identity_beneficiary_address</a>(&creator_identity);
    <b>let</b> vault_id = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_ensure_beneficiary_vault">poc_vault::ensure_beneficiary_vault</a>(vault_directory, beneficiary_address, ctx);
    <b>let</b> beneficiary = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">PoCUsernameBeneficiary</a> {
        id: object::new(ctx),
        username,
        creator_identity,
        verification: <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_VerificationRequirements">VerificationRequirements</a> { required_x_handle },
        provisioned_at: now,
        status: <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_STATUS_ACTIVE">STATUS_ACTIVE</a>,
        claimed_profile_id: option::none(),
        claimed_by: option::none(),
        claimed_at: option::none(),
        ended_at: option::none(),
        ended_by: option::none(),
        end_reason_code: option::none(),
        oracle_evidence_hash: vector::empty(),
        vault_id,
        beneficiary_address,
        provisioned_by,
        join_referral_paid: <b>false</b>,
        join_referrer: option::none(),
        join_referral_paid_at: option::none(),
        version: <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>(),
    };
    <b>let</b> beneficiary_id = object::id(&beneficiary);
    <b>let</b> beneficiary_addr = object::id_to_address(&beneficiary_id);
    <b>let</b> event_x_handle = beneficiary.verification.required_x_handle;
    transfer::share_object(beneficiary);
    table::add(&<b>mut</b> directory.beneficiary_by_identity, creator_identity, beneficiary_id);
    table::add(&<b>mut</b> shard.username_to_beneficiary, username, beneficiary_id);
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_UsernameBeneficiaryProvisionedEvent">UsernameBeneficiaryProvisionedEvent</a> {
        beneficiary_id: beneficiary_addr,
        username,
        creator_identity_source: creator_identity.source,
        creator_identity_hash: creator_identity.identity_hash,
        required_x_handle: event_x_handle,
        beneficiary_address,
        vault_id,
        provisioned_by,
        provisioned_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_claim_username_beneficiary"></a>

## Function `claim_username_beneficiary`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_claim_username_beneficiary">claim_username_beneficiary</a>(directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryDirectory</a>, shard: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryShard</a>, username_registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, profile_config: &<a href="../social_contracts/profile.md#social_contracts_profile_ProfileConfig">social_contracts::profile::ProfileConfig</a>, memory_registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>, ai_credit_config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">social_contracts::ai_credit::AiCreditConfig</a>, beneficiary: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiary</a>, evidence_hash: vector&lt;u8&gt;, attested_x_handle: vector&lt;u8&gt;, display_name: vector&lt;u8&gt;, bio: vector&lt;u8&gt;, profile_picture_url: vector&lt;u8&gt;, cover_photo_url: vector&lt;u8&gt;, wallet: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_claim_username_beneficiary">claim_username_beneficiary</a>(
    directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">PoCUsernameBeneficiaryDirectory</a>,
    shard: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">PoCUsernameBeneficiaryShard</a>,
    username_registry: &<b>mut</b> UsernameRegistry,
    profile_config: &ProfileConfig,
    memory_registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">memory::MemoryRegistry</a>,
    ai_credit_config: &<b>mut</b> <a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_AiCreditConfig">ai_credit::AiCreditConfig</a>,
    beneficiary: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">PoCUsernameBeneficiary</a>,
    evidence_hash: vector&lt;u8&gt;,
    attested_x_handle: vector&lt;u8&gt;,
    display_name: vector&lt;u8&gt;,
    bio: vector&lt;u8&gt;,
    profile_picture_url: vector&lt;u8&gt;,
    cover_photo_url: vector&lt;u8&gt;,
    wallet: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_directory_version">assert_directory_version</a>(directory);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_shard_version">assert_shard_version</a>(shard);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_beneficiary_version">assert_beneficiary_version</a>(beneficiary);
    <b>assert</b>!(beneficiary.status == <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_STATUS_ACTIVE">STATUS_ACTIVE</a>, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EInvalidStatus">EInvalidStatus</a>);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_shard_matches_username">assert_shard_matches_username</a>(shard, &beneficiary.username);
    <b>let</b> attested = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_canonical_x_handle">canonical_x_handle</a>(attested_x_handle);
    <b>assert</b>!(
        attested == beneficiary.verification.required_x_handle,
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EVerificationFailed">EVerificationFailed</a>
    );
    <b>assert</b>!(
        !table::contains(&directory.wallet_by_identity, beneficiary.creator_identity),
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWalletAlreadyLinked">EWalletAlreadyLinked</a>
    );
    <b>let</b> profile_id = <a href="../social_contracts/profile.md#social_contracts_profile_create_profile_from_beneficiary_claim">profile::create_profile_from_beneficiary_claim</a>(
        username_registry,
        profile_config,
        memory_registry,
        ai_credit_config,
        display_name,
        beneficiary.username,
        bio,
        profile_picture_url,
        cover_photo_url,
        wallet,
        clock,
        ctx,
    );
    <b>let</b> now = clock::timestamp_ms(clock);
    beneficiary.status = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_STATUS_CLAIMED">STATUS_CLAIMED</a>;
    beneficiary.claimed_profile_id = option::some(profile_id);
    beneficiary.claimed_by = option::some(tx_context::sender(ctx));
    beneficiary.claimed_at = option::some(now);
    beneficiary.oracle_evidence_hash = evidence_hash;
    table::add(&<b>mut</b> directory.wallet_by_identity, beneficiary.creator_identity, wallet);
    <b>if</b> (table::contains(&shard.username_to_beneficiary, beneficiary.username)) {
        table::remove(&<b>mut</b> shard.username_to_beneficiary, beneficiary.username);
    };
    <a href="../social_contracts/profile.md#social_contracts_profile_unlock_username_for_beneficiary">profile::unlock_username_for_beneficiary</a>(username_registry, beneficiary.username);
    <b>let</b> beneficiary_id = object::id_to_address(&object::id(beneficiary));
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_CreatorIdentityWalletLinkedEvent">CreatorIdentityWalletLinkedEvent</a> {
        creator_identity_source: beneficiary.creator_identity.source,
        creator_identity_hash: beneficiary.creator_identity.identity_hash,
        wallet,
        beneficiary_id,
        linked_at: now,
    });
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_UsernameBeneficiaryClaimedEvent">UsernameBeneficiaryClaimedEvent</a> {
        beneficiary_id,
        username: beneficiary.username,
        profile_id,
        claimed_by: tx_context::sender(ctx),
        wallet,
        oracle_evidence_hash: beneficiary.oracle_evidence_hash,
        claimed_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_claim_username_beneficiary_vault_balance"></a>

## Function `claim_username_beneficiary_vault_balance`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_claim_username_beneficiary_vault_balance">claim_username_beneficiary_vault_balance</a>&lt;T&gt;(treasury_fee_bps: u64, join_referral_bps: u64, directory: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryDirectory</a>, beneficiary: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiary</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, join_referrer: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_claim_username_beneficiary_vault_balance">claim_username_beneficiary_vault_balance</a>&lt;T&gt;(
    treasury_fee_bps: u64,
    join_referral_bps: u64,
    directory: &<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">PoCUsernameBeneficiaryDirectory</a>,
    beneficiary: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">PoCUsernameBeneficiary</a>,
    treasury: &EcosystemTreasury,
    vault: &<b>mut</b> PoCBeneficiaryVault,
    join_referrer: Option&lt;<b>address</b>&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_directory_version">assert_directory_version</a>(directory);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_beneficiary_version">assert_beneficiary_version</a>(beneficiary);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_assert_vault_version">poc_vault::assert_vault_version</a>(vault);
    <b>assert</b>!(beneficiary.status == <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_STATUS_CLAIMED">STATUS_CLAIMED</a>, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EInvalidStatus">EInvalidStatus</a>);
    <b>assert</b>!(
        table::contains(&directory.wallet_by_identity, beneficiary.creator_identity),
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWalletNotLinked">EWalletNotLinked</a>
    );
    <b>let</b> linked_wallet = *table::borrow(
        &directory.wallet_by_identity,
        beneficiary.creator_identity,
    );
    <b>assert</b>!(
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_vault_routes_to_beneficiary">poc_vault::vault_routes_to_beneficiary</a>(vault, beneficiary.beneficiary_address),
        <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EUnauthorized">EUnauthorized</a>
    );
    <b>let</b> apply_join_referral = !beneficiary.join_referral_paid;
    <b>if</b> (apply_join_referral && option::is_some(&join_referrer)) {
        <b>assert</b>!(!beneficiary.join_referral_paid, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EJoinReferralAlreadyPaid">EJoinReferralAlreadyPaid</a>);
    };
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_claim_vault_balance_for_linked_wallet">poc_vault::claim_vault_balance_for_linked_wallet</a>&lt;T&gt;(
        vault,
        linked_wallet,
        treasury,
        treasury_fee_bps,
        join_referral_bps,
        apply_join_referral,
        join_referrer,
        clock,
        ctx,
    );
    <b>if</b> (apply_join_referral && option::is_some(&join_referrer) && join_referral_bps &gt; 0) {
        beneficiary.join_referral_paid = <b>true</b>;
        beneficiary.join_referrer = join_referrer;
        beneficiary.join_referral_paid_at = option::some(clock::timestamp_ms(clock));
    } <b>else</b> <b>if</b> (apply_join_referral) {
        beneficiary.join_referral_paid = <b>true</b>;
        beneficiary.join_referral_paid_at = option::some(clock::timestamp_ms(clock));
    };
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_end_username_beneficiary"></a>

## Function `end_username_beneficiary`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_end_username_beneficiary">end_username_beneficiary</a>(directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryDirectory</a>, shard: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryShard</a>, username_registry: &<b>mut</b> <a href="../social_contracts/profile.md#social_contracts_profile_UsernameRegistry">social_contracts::profile::UsernameRegistry</a>, beneficiary: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiary</a>, vault: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_PoCBeneficiaryVault">social_contracts::poc_vault::PoCBeneficiaryVault</a>, treasury: &<a href="../social_contracts/profile.md#social_contracts_profile_EcosystemTreasury">social_contracts::profile::EcosystemTreasury</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_end_username_beneficiary">end_username_beneficiary</a>(
    directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">PoCUsernameBeneficiaryDirectory</a>,
    shard: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">PoCUsernameBeneficiaryShard</a>,
    username_registry: &<b>mut</b> UsernameRegistry,
    beneficiary: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">PoCUsernameBeneficiary</a>,
    vault: &<b>mut</b> PoCBeneficiaryVault,
    treasury: &EcosystemTreasury,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_directory_version">assert_directory_version</a>(directory);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_shard_version">assert_shard_version</a>(shard);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_beneficiary_version">assert_beneficiary_version</a>(beneficiary);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_assert_vault_version">poc_vault::assert_vault_version</a>(vault);
    <b>assert</b>!(beneficiary.status == <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_STATUS_ACTIVE">STATUS_ACTIVE</a>, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EInvalidStatus">EInvalidStatus</a>);
    <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_assert_shard_matches_username">assert_shard_matches_username</a>(shard, &beneficiary.username);
    <b>let</b> ended_by = tx_context::sender(ctx);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>let</b> reason_code = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_END_REASON_ADMIN">END_REASON_ADMIN</a>;
    <b>let</b> swept_mys_amount = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_vault_sweep_vault_balance_to_treasury">poc_vault::sweep_vault_balance_to_treasury</a>&lt;MYSO&gt;(
        vault,
        treasury,
        ctx,
    );
    beneficiary.status = <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_STATUS_ENDED">STATUS_ENDED</a>;
    beneficiary.ended_at = option::some(now);
    beneficiary.ended_by = option::some(ended_by);
    beneficiary.end_reason_code = option::some(reason_code);
    <b>if</b> (table::contains(&directory.beneficiary_by_identity, beneficiary.creator_identity)) {
        table::remove(&<b>mut</b> directory.beneficiary_by_identity, beneficiary.creator_identity);
    };
    <b>if</b> (table::contains(&shard.username_to_beneficiary, beneficiary.username)) {
        table::remove(&<b>mut</b> shard.username_to_beneficiary, beneficiary.username);
    };
    <a href="../social_contracts/profile.md#social_contracts_profile_unlock_username_for_beneficiary">profile::unlock_username_for_beneficiary</a>(username_registry, beneficiary.username);
    <b>let</b> beneficiary_id = object::id_to_address(&object::id(beneficiary));
    event::emit(<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_UsernameBeneficiaryEndedEvent">UsernameBeneficiaryEndedEvent</a> {
        beneficiary_id,
        username: beneficiary.username,
        ended_by,
        end_reason_code: reason_code,
        swept_mys_amount,
        ended_at: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary_directory"></a>

## Function `migrate_poc_username_beneficiary_directory`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary_directory">migrate_poc_username_beneficiary_directory</a>(directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryDirectory</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary_directory">migrate_poc_username_beneficiary_directory</a>(
    directory: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">PoCUsernameBeneficiaryDirectory</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(directory.version &lt; current_version, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWrongVersion">EWrongVersion</a>);
    <b>let</b> old_version = directory.version;
    directory.version = current_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(directory),
        string::utf8(b"<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryDirectory">PoCUsernameBeneficiaryDirectory</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary_shard"></a>

## Function `migrate_poc_username_beneficiary_shard`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary_shard">migrate_poc_username_beneficiary_shard</a>(shard: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiaryShard</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary_shard">migrate_poc_username_beneficiary_shard</a>(
    shard: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">PoCUsernameBeneficiaryShard</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(shard.version &lt; current_version, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWrongVersion">EWrongVersion</a>);
    <b>let</b> old_version = shard.version;
    shard.version = current_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(shard),
        string::utf8(b"<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiaryShard">PoCUsernameBeneficiaryShard</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>

<a name="social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary"></a>

## Function `migrate_poc_username_beneficiary`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary">migrate_poc_username_beneficiary</a>(beneficiary: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">social_contracts::poc_username_beneficiary::PoCUsernameBeneficiary</a>, _: &<a href="../social_contracts/upgrade.md#social_contracts_upgrade_UpgradeAdminCap">social_contracts::upgrade::UpgradeAdminCap</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_migrate_poc_username_beneficiary">migrate_poc_username_beneficiary</a>(
    beneficiary: &<b>mut</b> <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">PoCUsernameBeneficiary</a>,
    _: &UpgradeAdminCap,
    ctx: &<b>mut</b> TxContext
) {
    <b>let</b> current_version = <a href="../social_contracts/upgrade.md#social_contracts_upgrade_current_version">upgrade::current_version</a>();
    <b>assert</b>!(beneficiary.version &lt; current_version, <a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_EWrongVersion">EWrongVersion</a>);
    <b>let</b> old_version = beneficiary.version;
    beneficiary.version = current_version;
    <a href="../social_contracts/upgrade.md#social_contracts_upgrade_emit_migration_event">upgrade::emit_migration_event</a>(
        object::id(beneficiary),
        string::utf8(b"<a href="../social_contracts/proof_of_creativity.md#social_contracts_poc_username_beneficiary_PoCUsernameBeneficiary">PoCUsernameBeneficiary</a>"),
        old_version,
        tx_context::sender(ctx)
    );
}
</code></pre>



</details>
