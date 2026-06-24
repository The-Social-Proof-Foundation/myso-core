---
title: Module `social_contracts::memory`
---

Memory — human root account and hierarchical permissioned sub-agent layer.

One slim [<code><a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a></code>] per human owner (linked from [<code><a href="../social_contracts/profile.md#social_contracts_profile_Profile">social_contracts::profile::Profile</a></code>]).
Each sub-agent is a **shared derived object** keyed by <code>(memory_account_id, derived_address)</code>.
Agents sign as <code>derived_address</code> and resolve to the human <code>principal_owner</code> for profile,
platform join, and MyData.

**Hierarchy:** <code>parent_object_id</code> points up; no child list on parent. Max depth 8.
**Registry:** auth mirror [<code>Table</code>] on [<code><a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a></code>] keyed by <code>derived_address</code> (plus
object-id reverse index) so social/MyData PTBs need only [<code><a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a></code>], not ancestor refs.
**Lifecycle:** deactivate/revoke operate on explicit agent objects; subtree batches are
computed off-chain (indexer/server).

**<code>max_action_spend</code>:** optional per-transaction MYSO (MIST) ceiling for sub-agent signers.


<a name="@Indexer_event_payloads_0"></a>

### Indexer event payloads


**SubAgentRegistered / SubAgentUpdated:** <code>account_id</code>, <code>principal_owner</code>, <code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a></code>,
<code><a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a></code>, <code>derived_address</code>, <code>label</code>, <code>identity_class</code>, <code>role_tags</code>, <code>capabilities</code>,
<code>delegatable_caps</code>, <code>register_scope</code>, <code>approval_required_caps</code>, <code>max_action_spend</code>, <code>platform_scope</code>,
<code>parent_object_id</code>, <code>depth</code>, <code>registered_by</code>, <code>expires_at</code>, <code>active</code>, <code>created_at</code>

**SubAgentDeactivated / SubAgentRevoked:** identifiers above + <code><a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a></code>, <code>derived_address</code>

**SubAgentsClearedOnTransfer:** <code>account_id</code>, <code>principal_owner</code>, <code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a></code>,
<code>previous_owner</code>, <code>new_owner</code>, <code>revoked_count</code>

**Social events** (post module): all include <code>actor_address</code>, <code>sub_agent_id</code> (agent object id),
<code>action_identity_class</code> and reactions add <code>principal_owner</code>.


    -  [Indexer event payloads](#@Indexer_event_payloads_0)
-  [Struct `MemoryRegistry`](#social_contracts_memory_MemoryRegistry)
-  [Struct `SubAgentKey`](#social_contracts_memory_SubAgentKey)
-  [Struct `AgentMemoryVaultKey`](#social_contracts_memory_AgentMemoryVaultKey)
-  [Struct `SubAgentConstraints`](#social_contracts_memory_SubAgentConstraints)
-  [Struct `OrgRegistryEntry`](#social_contracts_memory_OrgRegistryEntry)
-  [Struct `AgenticOrganization`](#social_contracts_memory_AgenticOrganization)
-  [Struct `AgentRegistryEntry`](#social_contracts_memory_AgentRegistryEntry)
-  [Struct `SubAgent`](#social_contracts_memory_SubAgent)
-  [Struct `AgentMemoryVault`](#social_contracts_memory_AgentMemoryVault)
-  [Struct `ActingContext`](#social_contracts_memory_ActingContext)
-  [Struct `MemoryAccount`](#social_contracts_memory_MemoryAccount)
-  [Struct `MemoryAccountCreated`](#social_contracts_memory_MemoryAccountCreated)
-  [Struct `SubAgentRegistered`](#social_contracts_memory_SubAgentRegistered)
-  [Struct `SubAgentUpdated`](#social_contracts_memory_SubAgentUpdated)
-  [Struct `SubAgentDeactivated`](#social_contracts_memory_SubAgentDeactivated)
-  [Struct `SubAgentRevoked`](#social_contracts_memory_SubAgentRevoked)
-  [Struct `SubAgentsClearedOnTransfer`](#social_contracts_memory_SubAgentsClearedOnTransfer)
-  [Struct `MemoryAccountDeactivated`](#social_contracts_memory_MemoryAccountDeactivated)
-  [Struct `MemoryAccountReactivated`](#social_contracts_memory_MemoryAccountReactivated)
-  [Struct `MemoryAccountMigrated`](#social_contracts_memory_MemoryAccountMigrated)
-  [Struct `MemoryRegistryMigrated`](#social_contracts_memory_MemoryRegistryMigrated)
-  [Struct `AgentMemoryVaultCreated`](#social_contracts_memory_AgentMemoryVaultCreated)
-  [Struct `AgenticOrganizationCreated`](#social_contracts_memory_AgenticOrganizationCreated)
-  [Struct `AgenticOrganizationUpdated`](#social_contracts_memory_AgenticOrganizationUpdated)
-  [Struct `AgenticOrganizationCategoryUpdated`](#social_contracts_memory_AgenticOrganizationCategoryUpdated)
-  [Struct `AgenticOrganizationDeactivated`](#social_contracts_memory_AgenticOrganizationDeactivated)
-  [Constants](#@Constants_1)
-  [Function `class_human`](#social_contracts_memory_class_human)
-  [Function `class_delegated_ai`](#social_contracts_memory_class_delegated_ai)
-  [Function `class_organization`](#social_contracts_memory_class_organization)
-  [Function `org_type_company`](#social_contracts_memory_org_type_company)
-  [Function `org_type_startup`](#social_contracts_memory_org_type_startup)
-  [Function `org_type_investment_fund`](#social_contracts_memory_org_type_investment_fund)
-  [Function `org_type_nonprofit`](#social_contracts_memory_org_type_nonprofit)
-  [Function `org_type_research`](#social_contracts_memory_org_type_research)
-  [Function `org_type_government`](#social_contracts_memory_org_type_government)
-  [Function `org_type_media`](#social_contracts_memory_org_type_media)
-  [Function `org_type_stewardship`](#social_contracts_memory_org_type_stewardship)
-  [Function `org_type_brand`](#social_contracts_memory_org_type_brand)
-  [Function `org_type_community`](#social_contracts_memory_org_type_community)
-  [Function `org_type_sports`](#social_contracts_memory_org_type_sports)
-  [Function `org_type_education`](#social_contracts_memory_org_type_education)
-  [Function `org_type_healthcare`](#social_contracts_memory_org_type_healthcare)
-  [Function `org_type_other`](#social_contracts_memory_org_type_other)
-  [Function `org_type_count`](#social_contracts_memory_org_type_count)
-  [Function `max_organizations_per_user`](#social_contracts_memory_max_organizations_per_user)
-  [Function `cap_memory_read`](#social_contracts_memory_cap_memory_read)
-  [Function `cap_memory_write`](#social_contracts_memory_cap_memory_write)
-  [Function `cap_mydata_read`](#social_contracts_memory_cap_mydata_read)
-  [Function `cap_post_publish`](#social_contracts_memory_cap_post_publish)
-  [Function `cap_comment`](#social_contracts_memory_cap_comment)
-  [Function `cap_react`](#social_contracts_memory_cap_react)
-  [Function `cap_message_read`](#social_contracts_memory_cap_message_read)
-  [Function `cap_message_send`](#social_contracts_memory_cap_message_send)
-  [Function `cap_trade_monitor`](#social_contracts_memory_cap_trade_monitor)
-  [Function `cap_trade_execute`](#social_contracts_memory_cap_trade_execute)
-  [Function `cap_agent_register`](#social_contracts_memory_cap_agent_register)
-  [Function `cap_agent_revoke`](#social_contracts_memory_cap_agent_revoke)
-  [Function `cap_agent_update`](#social_contracts_memory_cap_agent_update)
-  [Function `register_child`](#social_contracts_memory_register_child)
-  [Function `register_peer`](#social_contracts_memory_register_peer)
-  [Function `derive_sub_agent_address`](#social_contracts_memory_derive_sub_agent_address)
-  [Function `agent_object_id`](#social_contracts_memory_agent_object_id)
-  [Function `organization_id`](#social_contracts_memory_organization_id)
-  [Function `sub_agent_organization_id`](#social_contracts_memory_sub_agent_organization_id)
-  [Function `organization_org_type`](#social_contracts_memory_organization_org_type)
-  [Function `organization_root_agent_id`](#social_contracts_memory_organization_root_agent_id)
-  [Function `organization_name`](#social_contracts_memory_organization_name)
-  [Function `organization_description`](#social_contracts_memory_organization_description)
-  [Function `bootstrap_init`](#social_contracts_memory_bootstrap_init)
-  [Function `create_account_for_profile`](#social_contracts_memory_create_account_for_profile)
-  [Function `transfer_account_owner_with_profile`](#social_contracts_memory_transfer_account_owner_with_profile)
-  [Function `create_agentic_organization`](#social_contracts_memory_create_agentic_organization)
-  [Function `create_agentic_organization_internal`](#social_contracts_memory_create_agentic_organization_internal)
-  [Function `update_agentic_organization_metadata`](#social_contracts_memory_update_agentic_organization_metadata)
-  [Function `update_agentic_organization_category`](#social_contracts_memory_update_agentic_organization_category)
-  [Function `deactivate_agentic_organization`](#social_contracts_memory_deactivate_agentic_organization)
-  [Function `register_sub_agent`](#social_contracts_memory_register_sub_agent)
-  [Function `register_sub_agent_delegated`](#social_contracts_memory_register_sub_agent_delegated)
-  [Function `update_sub_agent`](#social_contracts_memory_update_sub_agent)
-  [Function `update_sub_agent_label`](#social_contracts_memory_update_sub_agent_label)
-  [Function `deactivate_sub_agent`](#social_contracts_memory_deactivate_sub_agent)
-  [Function `revoke_sub_agent`](#social_contracts_memory_revoke_sub_agent)
-  [Function `emit_sub_agents_cleared_on_transfer`](#social_contracts_memory_emit_sub_agents_cleared_on_transfer)
-  [Function `ensure_agent_memory_vault`](#social_contracts_memory_ensure_agent_memory_vault)
-  [Function `deactivate_account`](#social_contracts_memory_deactivate_account)
-  [Function `reactivate_account`](#social_contracts_memory_reactivate_account)
-  [Function `migrate_account`](#social_contracts_memory_migrate_account)
-  [Function `admin_migrate_account`](#social_contracts_memory_admin_migrate_account)
-  [Function `migrate_registry`](#social_contracts_memory_migrate_registry)
-  [Function `resolve_human_actor`](#social_contracts_memory_resolve_human_actor)
-  [Function `resolve_actor_from_account`](#social_contracts_memory_resolve_actor_from_account)
-  [Function `resolve_actor_with_cap`](#social_contracts_memory_resolve_actor_with_cap)
-  [Function `assert_human_actor_with_cap`](#social_contracts_memory_assert_human_actor_with_cap)
-  [Function `assert_action_spend_limit`](#social_contracts_memory_assert_action_spend_limit)
-  [Function `assert_direct_execution_allowed`](#social_contracts_memory_assert_direct_execution_allowed)
-  [Function `assert_platform_scope_entry`](#social_contracts_memory_assert_platform_scope_entry)
-  [Function `assert_platform_scope`](#social_contracts_memory_assert_platform_scope)
-  [Function `assert_sub_agent_active`](#social_contracts_memory_assert_sub_agent_active)
-  [Function `profile_id`](#social_contracts_memory_profile_id)
-  [Function `owner`](#social_contracts_memory_owner)
-  [Function `sub_agent_derived_address`](#social_contracts_memory_sub_agent_derived_address)
-  [Function `sub_agent_capabilities`](#social_contracts_memory_sub_agent_capabilities)
-  [Function `sub_agent_platform_scope`](#social_contracts_memory_sub_agent_platform_scope)
-  [Function `sub_agent_active`](#social_contracts_memory_sub_agent_active)
-  [Function `sub_agent_depth`](#social_contracts_memory_sub_agent_depth)
-  [Function `sub_agent_parent_object_id`](#social_contracts_memory_sub_agent_parent_object_id)
-  [Function `sub_agent_memory_account_id`](#social_contracts_memory_sub_agent_memory_account_id)
-  [Function `acting_principal_owner`](#social_contracts_memory_acting_principal_owner)
-  [Function `acting_profile_id`](#social_contracts_memory_acting_profile_id)
-  [Function `acting_actor_address`](#social_contracts_memory_acting_actor_address)
-  [Function `acting_sub_agent_id`](#social_contracts_memory_acting_sub_agent_id)
-  [Function `acting_identity_class`](#social_contracts_memory_acting_identity_class)
-  [Function `acting_parent_object_id`](#social_contracts_memory_acting_parent_object_id)
-  [Function `acting_depth`](#social_contracts_memory_acting_depth)
-  [Function `acting_organization_id`](#social_contracts_memory_acting_organization_id)
-  [Function `has_account`](#social_contracts_memory_has_account)
-  [Function `account_id_for_owner`](#social_contracts_memory_account_id_for_owner)
-  [Function `is_registered_agent`](#social_contracts_memory_is_registered_agent)
-  [Function `is_active`](#social_contracts_memory_is_active)
-  [Function `account_version`](#social_contracts_memory_account_version)
-  [Function `registry_version`](#social_contracts_memory_registry_version)
-  [Function `current_contract_version`](#social_contracts_memory_current_contract_version)
-  [Function `approve_key_policy`](#social_contracts_memory_approve_key_policy)
-  [Function `approve_key_write_policy`](#social_contracts_memory_approve_key_write_policy)
-  [Function `owner_key_suffix_bytes`](#social_contracts_memory_owner_key_suffix_bytes)
-  [Function `register_sub_agent_internal`](#social_contracts_memory_register_sub_agent_internal)
-  [Function `register_sub_agent_delegated_internal`](#social_contracts_memory_register_sub_agent_delegated_internal)
-  [Function `finish_register_sub_agent`](#social_contracts_memory_finish_register_sub_agent)
-  [Function `bind_root_agent_to_organization`](#social_contracts_memory_bind_root_agent_to_organization)
-  [Function `registry_entry_from_agent`](#social_contracts_memory_registry_entry_from_agent)
-  [Function `insert_registry_entry`](#social_contracts_memory_insert_registry_entry)
-  [Function `sync_registry_from_agent`](#social_contracts_memory_sync_registry_from_agent)
-  [Function `sync_registry_active`](#social_contracts_memory_sync_registry_active)
-  [Function `remove_registry_entry`](#social_contracts_memory_remove_registry_entry)
-  [Function `assert_registry_entry_active`](#social_contracts_memory_assert_registry_entry_active)
-  [Function `assert_action_spend_limit_from_entry`](#social_contracts_memory_assert_action_spend_limit_from_entry)
-  [Function `assert_ancestor_chain_active_from_table`](#social_contracts_memory_assert_ancestor_chain_active_from_table)
-  [Function `resolve_delegated_registration_placement`](#social_contracts_memory_resolve_delegated_registration_placement)
-  [Function `assert_may_manage`](#social_contracts_memory_assert_may_manage)
-  [Function `assert_registrar_is_ancestor_from_table`](#social_contracts_memory_assert_registrar_is_ancestor_from_table)
-  [Function `assert_update_caps_monotonic`](#social_contracts_memory_assert_update_caps_monotonic)
-  [Function `assert_agent_belongs_to_account`](#social_contracts_memory_assert_agent_belongs_to_account)
-  [Function `destroy_sub_agent`](#social_contracts_memory_destroy_sub_agent)
-  [Function `assert_caps_subset`](#social_contracts_memory_assert_caps_subset)
-  [Function `assert_scope_allowed_for_delegate`](#social_contracts_memory_assert_scope_allowed_for_delegate)
-  [Function `assert_valid_register_scope`](#social_contracts_memory_assert_valid_register_scope)
-  [Function `assert_sub_agent_not_expired`](#social_contracts_memory_assert_sub_agent_not_expired)
-  [Function `assert_valid_identity_class`](#social_contracts_memory_assert_valid_identity_class)
-  [Function `assert_org_name_within_limit`](#social_contracts_memory_assert_org_name_within_limit)
-  [Function `assert_org_description_within_limit`](#social_contracts_memory_assert_org_description_within_limit)
-  [Function `assert_valid_org_type`](#social_contracts_memory_assert_valid_org_type)
-  [Function `assert_organization_belongs_to_account`](#social_contracts_memory_assert_organization_belongs_to_account)
-  [Function `assert_organization_ready_for_root`](#social_contracts_memory_assert_organization_ready_for_root)
-  [Function `has_cap`](#social_contracts_memory_has_cap)
-  [Function `cap_requires_approval`](#social_contracts_memory_cap_requires_approval)
-  [Function `emit_sub_agent_registered`](#social_contracts_memory_emit_sub_agent_registered)
-  [Function `emit_sub_agent_updated`](#social_contracts_memory_emit_sub_agent_updated)
-  [Function `emit_sub_agent_deactivated`](#social_contracts_memory_emit_sub_agent_deactivated)
-  [Function `emit_sub_agent_revoked`](#social_contracts_memory_emit_sub_agent_revoked)
-  [Function `get_version`](#social_contracts_memory_get_version)
-  [Function `set_version`](#social_contracts_memory_set_version)
-  [Function `bump_version`](#social_contracts_memory_bump_version)
-  [Function `assert_object_version`](#social_contracts_memory_assert_object_version)
-  [Function `assert_cap_for_this_package`](#social_contracts_memory_assert_cap_for_this_package)
-  [Function `has_suffix`](#social_contracts_memory_has_suffix)


<pre><code><b>use</b> <a href="../myso/accumulator.md#myso_accumulator">myso::accumulator</a>;
<b>use</b> <a href="../myso/accumulator_settlement.md#myso_accumulator_settlement">myso::accumulator_settlement</a>;
<b>use</b> <a href="../myso/address.md#myso_address">myso::address</a>;
<b>use</b> <a href="../myso/bcs.md#myso_bcs">myso::bcs</a>;
<b>use</b> <a href="../myso/bootstrap_key.md#myso_bootstrap_key">myso::bootstrap_key</a>;
<b>use</b> <a href="../myso/clock.md#myso_clock">myso::clock</a>;
<b>use</b> <a href="../myso/derived_object.md#myso_derived_object">myso::derived_object</a>;
<b>use</b> <a href="../myso/dynamic_field.md#myso_dynamic_field">myso::dynamic_field</a>;
<b>use</b> <a href="../myso/event.md#myso_event">myso::event</a>;
<b>use</b> <a href="../myso/hash.md#myso_hash">myso::hash</a>;
<b>use</b> <a href="../myso/hex.md#myso_hex">myso::hex</a>;
<b>use</b> <a href="../myso/object.md#myso_object">myso::object</a>;
<b>use</b> <a href="../myso/package.md#myso_package">myso::package</a>;
<b>use</b> <a href="../myso/party.md#myso_party">myso::party</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_memory_MemoryRegistry"></a>

## Struct `MemoryRegistry`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a> <b>has</b> key
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
<code>accounts: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_SubAgentKey"></a>

## Struct `SubAgentKey`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentKey">SubAgentKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>derived_address: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_AgentMemoryVaultKey"></a>

## Struct `AgentMemoryVaultKey`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgentMemoryVaultKey">AgentMemoryVaultKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_SubAgentConstraints"></a>

## Struct `SubAgentConstraints`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentConstraints">SubAgentConstraints</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>approval_required_caps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgRegistryEntry"></a>

## Struct `OrgRegistryEntry`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgRegistryEntry">OrgRegistryEntry</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>active: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_AgenticOrganization"></a>

## Struct `AgenticOrganization`

Competitive agentic organization wrapper (one root-agent tree per org).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a> <b>has</b> key, store
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
<code>memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>org_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>root_agent_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>deactivated_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>category_updated_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_AgentRegistryEntry"></a>

## Struct `AgentRegistryEntry`

Auth mirror for on-chain ancestor walks without PTB ancestor inputs.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">AgentRegistryEntry</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>parent_object_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>depth: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>identity_class: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>capabilities: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegatable_caps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>register_scope: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>constraints: <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentConstraints">social_contracts::memory::SubAgentConstraints</a></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_SubAgent"></a>

## Struct `SubAgent`

Shared derived sub-agent object (one per agent).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a> <b>has</b> key, store
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
<code>memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>derived_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>public_key: vector&lt;u8&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>label: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>identity_class: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>role_tags: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>capabilities: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegatable_caps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>register_scope: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>constraints: <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentConstraints">social_contracts::memory::SubAgentConstraints</a></code>
</dt>
<dd>
</dd>
<dt>
<code>platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>parent_object_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>depth: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>registered_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_AgentMemoryVault"></a>

## Struct `AgentMemoryVault`

Lazy per-agent memory blob anchor (derived from [<code><a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a></code>]).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgentMemoryVault">AgentMemoryVault</a> <b>has</b> key
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
<code><a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
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

<a name="social_contracts_memory_ActingContext"></a>

## Struct `ActingContext`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_profile_id: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>actor_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>sub_agent_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>identity_class: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>parent_object_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>depth: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryAccount"></a>

## Struct `MemoryAccount`

Human root plus on-chain agent auth index (shared [<code><a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a></code>] objects remain canonical).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a> <b>has</b> key, store
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
<code><a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>created_at: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>active: bool</code>
</dt>
<dd>
</dd>
<dt>
<code>agents: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<b>address</b>, <a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">social_contracts::memory::AgentRegistryEntry</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>agent_ids: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>organizations: <a href="../myso/table.md#myso_table_Table">myso::table::Table</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgRegistryEntry">social_contracts::memory::OrgRegistryEntry</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>org_count: u8</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryAccountCreated"></a>

## Struct `MemoryAccountCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountCreated">MemoryAccountCreated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_SubAgentRegistered"></a>

## Struct `SubAgentRegistered`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentRegistered">SubAgentRegistered</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>derived_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>label: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>identity_class: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>role_tags: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>capabilities: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegatable_caps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>register_scope: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>approval_required_caps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>parent_object_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>depth: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>registered_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>active: bool</code>
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

<a name="social_contracts_memory_SubAgentUpdated"></a>

## Struct `SubAgentUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentUpdated">SubAgentUpdated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>derived_address: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>label: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>identity_class: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>role_tags: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>capabilities: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>delegatable_caps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>register_scope: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>approval_required_caps: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>parent_object_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>depth: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>registered_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>active: bool</code>
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

<a name="social_contracts_memory_SubAgentDeactivated"></a>

## Struct `SubAgentDeactivated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentDeactivated">SubAgentDeactivated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>derived_address: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_SubAgentRevoked"></a>

## Struct `SubAgentRevoked`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentRevoked">SubAgentRevoked</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>derived_address: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_SubAgentsClearedOnTransfer"></a>

## Struct `SubAgentsClearedOnTransfer`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentsClearedOnTransfer">SubAgentsClearedOnTransfer</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>previous_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>new_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>revoked_count: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryAccountDeactivated"></a>

## Struct `MemoryAccountDeactivated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountDeactivated">MemoryAccountDeactivated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryAccountReactivated"></a>

## Struct `MemoryAccountReactivated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountReactivated">MemoryAccountReactivated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryAccountMigrated"></a>

## Struct `MemoryAccountMigrated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountMigrated">MemoryAccountMigrated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>from: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>to: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_MemoryRegistryMigrated"></a>

## Struct `MemoryRegistryMigrated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistryMigrated">MemoryRegistryMigrated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>registry_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>from: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>to: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_AgentMemoryVaultCreated"></a>

## Struct `AgentMemoryVaultCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgentMemoryVaultCreated">AgentMemoryVaultCreated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>vault_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>memory_account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_AgenticOrganizationCreated"></a>

## Struct `AgenticOrganizationCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganizationCreated">AgenticOrganizationCreated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>account_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>principal_owner: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>org_type: u8</code>
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

<a name="social_contracts_memory_AgenticOrganizationUpdated"></a>

## Struct `AgenticOrganizationUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganizationUpdated">AgenticOrganizationUpdated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_AgenticOrganizationCategoryUpdated"></a>

## Struct `AgenticOrganizationCategoryUpdated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganizationCategoryUpdated">AgenticOrganizationCategoryUpdated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>org_type: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>previous_org_type: u8</code>
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

<a name="social_contracts_memory_AgenticOrganizationDeactivated"></a>

## Struct `AgenticOrganizationDeactivated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganizationDeactivated">AgenticOrganizationDeactivated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code><a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>deactivated_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="@Constants_1"></a>

## Constants


<a name="social_contracts_memory_CLASS_HUMAN"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CLASS_HUMAN">CLASS_HUMAN</a>: u8 = 0;
</code></pre>



<a name="social_contracts_memory_CLASS_DELEGATED_AI"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CLASS_DELEGATED_AI">CLASS_DELEGATED_AI</a>: u8 = 1;
</code></pre>



<a name="social_contracts_memory_CLASS_ORGANIZATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CLASS_ORGANIZATION">CLASS_ORGANIZATION</a>: u8 = 2;
</code></pre>



<a name="social_contracts_memory_REGISTER_CHILD"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_CHILD">REGISTER_CHILD</a>: u8 = 1;
</code></pre>



<a name="social_contracts_memory_REGISTER_PEER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_PEER">REGISTER_PEER</a>: u8 = 2;
</code></pre>



<a name="social_contracts_memory_REGISTER_SCOPE_CHILD"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_SCOPE_CHILD">REGISTER_SCOPE_CHILD</a>: u8 = 1;
</code></pre>



<a name="social_contracts_memory_REGISTER_SCOPE_PEER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_SCOPE_PEER">REGISTER_SCOPE_PEER</a>: u8 = 2;
</code></pre>



<a name="social_contracts_memory_REGISTER_SCOPE_BOTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_SCOPE_BOTH">REGISTER_SCOPE_BOTH</a>: u8 = 3;
</code></pre>



<a name="social_contracts_memory_CAP_MEMORY_READ"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MEMORY_READ">CAP_MEMORY_READ</a>: u64 = 1;
</code></pre>



<a name="social_contracts_memory_CAP_MEMORY_WRITE"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MEMORY_WRITE">CAP_MEMORY_WRITE</a>: u64 = 2;
</code></pre>



<a name="social_contracts_memory_CAP_MYDATA_READ"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MYDATA_READ">CAP_MYDATA_READ</a>: u64 = 4;
</code></pre>



<a name="social_contracts_memory_CAP_POST_PUBLISH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_POST_PUBLISH">CAP_POST_PUBLISH</a>: u64 = 16;
</code></pre>



<a name="social_contracts_memory_CAP_MESSAGE_READ"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MESSAGE_READ">CAP_MESSAGE_READ</a>: u64 = 32;
</code></pre>



<a name="social_contracts_memory_CAP_MESSAGE_SEND"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MESSAGE_SEND">CAP_MESSAGE_SEND</a>: u64 = 64;
</code></pre>



<a name="social_contracts_memory_CAP_TRADE_MONITOR"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_TRADE_MONITOR">CAP_TRADE_MONITOR</a>: u64 = 128;
</code></pre>



<a name="social_contracts_memory_CAP_TRADE_EXECUTE"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_TRADE_EXECUTE">CAP_TRADE_EXECUTE</a>: u64 = 256;
</code></pre>



<a name="social_contracts_memory_CAP_COMMENT"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_COMMENT">CAP_COMMENT</a>: u64 = 512;
</code></pre>



<a name="social_contracts_memory_CAP_REACT"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_REACT">CAP_REACT</a>: u64 = 1024;
</code></pre>



<a name="social_contracts_memory_CAP_AGENT_REVOKE"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_REVOKE">CAP_AGENT_REVOKE</a>: u64 = 2048;
</code></pre>



<a name="social_contracts_memory_CAP_AGENT_UPDATE"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_UPDATE">CAP_AGENT_UPDATE</a>: u64 = 4096;
</code></pre>



<a name="social_contracts_memory_CAP_AGENT_REGISTER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_REGISTER">CAP_AGENT_REGISTER</a>: u64 = 8192;
</code></pre>



<a name="social_contracts_memory_ROLE_EDITOR"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_EDITOR">ROLE_EDITOR</a>: u64 = 1;
</code></pre>



<a name="social_contracts_memory_ROLE_MODERATOR"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MODERATOR">ROLE_MODERATOR</a>: u64 = 2;
</code></pre>



<a name="social_contracts_memory_ROLE_ORG_ADMIN"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_ORG_ADMIN">ROLE_ORG_ADMIN</a>: u64 = 4;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_COMPANY"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_COMPANY">ORG_TYPE_COMPANY</a>: u8 = 0;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_STARTUP"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_STARTUP">ORG_TYPE_STARTUP</a>: u8 = 1;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_INVESTMENT_FUND"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_INVESTMENT_FUND">ORG_TYPE_INVESTMENT_FUND</a>: u8 = 2;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_NONPROFIT"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_NONPROFIT">ORG_TYPE_NONPROFIT</a>: u8 = 3;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_RESEARCH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_RESEARCH">ORG_TYPE_RESEARCH</a>: u8 = 4;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_GOVERNMENT"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_GOVERNMENT">ORG_TYPE_GOVERNMENT</a>: u8 = 5;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_MEDIA"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_MEDIA">ORG_TYPE_MEDIA</a>: u8 = 6;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_STEWARDSHIP"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_STEWARDSHIP">ORG_TYPE_STEWARDSHIP</a>: u8 = 7;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_BRAND"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_BRAND">ORG_TYPE_BRAND</a>: u8 = 8;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_COMMUNITY"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_COMMUNITY">ORG_TYPE_COMMUNITY</a>: u8 = 9;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_SPORTS"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_SPORTS">ORG_TYPE_SPORTS</a>: u8 = 10;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_EDUCATION"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_EDUCATION">ORG_TYPE_EDUCATION</a>: u8 = 11;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_HEALTHCARE"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_HEALTHCARE">ORG_TYPE_HEALTHCARE</a>: u8 = 12;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_OTHER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_OTHER">ORG_TYPE_OTHER</a>: u8 = 13;
</code></pre>



<a name="social_contracts_memory_ORG_TYPE_COUNT"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_COUNT">ORG_TYPE_COUNT</a>: u8 = 14;
</code></pre>



<a name="social_contracts_memory_MAX_ORGANIZATIONS_PER_USER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORGANIZATIONS_PER_USER">MAX_ORGANIZATIONS_PER_USER</a>: u8 = 8;
</code></pre>



<a name="social_contracts_memory_ORG_CATEGORY_UPDATE_COOLDOWN_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_CATEGORY_UPDATE_COOLDOWN_MS">ORG_CATEGORY_UPDATE_COOLDOWN_MS</a>: u64 = 604800000;
</code></pre>



<a name="social_contracts_memory_ESubAgentNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotFound">ESubAgentNotFound</a>: u64 = 1;
</code></pre>



<a name="social_contracts_memory_EAccountAlreadyExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EAccountAlreadyExists">EAccountAlreadyExists</a>: u64 = 3;
</code></pre>



<a name="social_contracts_memory_ENotOwner"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>: u64 = 4;
</code></pre>



<a name="social_contracts_memory_EInvalidPublicKeyLength"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidPublicKeyLength">EInvalidPublicKeyLength</a>: u64 = 5;
</code></pre>



<a name="social_contracts_memory_EAccountDeactivated"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>: u64 = 6;
</code></pre>



<a name="social_contracts_memory_EWrongVersion"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EWrongVersion">EWrongVersion</a>: u64 = 7;
</code></pre>



<a name="social_contracts_memory_ENotUpgradeAuthority"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENotUpgradeAuthority">ENotUpgradeAuthority</a>: u64 = 8;
</code></pre>



<a name="social_contracts_memory_EAlreadyMigrated"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EAlreadyMigrated">EAlreadyMigrated</a>: u64 = 9;
</code></pre>



<a name="social_contracts_memory_ELabelTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ELabelTooLong">ELabelTooLong</a>: u64 = 10;
</code></pre>



<a name="social_contracts_memory_EAccountAlreadyActive"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EAccountAlreadyActive">EAccountAlreadyActive</a>: u64 = 11;
</code></pre>



<a name="social_contracts_memory_ENewOwnerHasMemoryAccount"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENewOwnerHasMemoryAccount">ENewOwnerHasMemoryAccount</a>: u64 = 12;
</code></pre>



<a name="social_contracts_memory_ERegistryAccountMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ERegistryAccountMismatch">ERegistryAccountMismatch</a>: u64 = 13;
</code></pre>



<a name="social_contracts_memory_ESubAgentDuplicateDerivedAddress"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentDuplicateDerivedAddress">ESubAgentDuplicateDerivedAddress</a>: u64 = 14;
</code></pre>



<a name="social_contracts_memory_ESubAgentNotActive"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotActive">ESubAgentNotActive</a>: u64 = 15;
</code></pre>



<a name="social_contracts_memory_ESubAgentExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentExpired">ESubAgentExpired</a>: u64 = 16;
</code></pre>



<a name="social_contracts_memory_ESubAgentWrongPlatformScope"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentWrongPlatformScope">ESubAgentWrongPlatformScope</a>: u64 = 17;
</code></pre>



<a name="social_contracts_memory_ESubAgentMissingCap"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentMissingCap">ESubAgentMissingCap</a>: u64 = 18;
</code></pre>



<a name="social_contracts_memory_ESubAgentApprovalRequired"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentApprovalRequired">ESubAgentApprovalRequired</a>: u64 = 19;
</code></pre>



<a name="social_contracts_memory_ESubAgentNotGlobalScope"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotGlobalScope">ESubAgentNotGlobalScope</a>: u64 = 20;
</code></pre>



<a name="social_contracts_memory_EInvalidIdentityClass"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidIdentityClass">EInvalidIdentityClass</a>: u64 = 21;
</code></pre>



<a name="social_contracts_memory_EInvalidRegisterRelation"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidRegisterRelation">EInvalidRegisterRelation</a>: u64 = 22;
</code></pre>



<a name="social_contracts_memory_EInvalidRegisterScope"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidRegisterScope">EInvalidRegisterScope</a>: u64 = 23;
</code></pre>



<a name="social_contracts_memory_EAgentDepthExceeded"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EAgentDepthExceeded">EAgentDepthExceeded</a>: u64 = 24;
</code></pre>



<a name="social_contracts_memory_ECapsNotSubset"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ECapsNotSubset">ECapsNotSubset</a>: u64 = 25;
</code></pre>



<a name="social_contracts_memory_EScopeWidening"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EScopeWidening">EScopeWidening</a>: u64 = 26;
</code></pre>



<a name="social_contracts_memory_ENotRegistrarAncestor"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENotRegistrarAncestor">ENotRegistrarAncestor</a>: u64 = 27;
</code></pre>



<a name="social_contracts_memory_EInvalidRegistrar"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidRegistrar">EInvalidRegistrar</a>: u64 = 28;
</code></pre>



<a name="social_contracts_memory_ESubAgentInactiveAncestor"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentInactiveAncestor">ESubAgentInactiveAncestor</a>: u64 = 29;
</code></pre>



<a name="social_contracts_memory_ESubAgentSpendExceeded"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentSpendExceeded">ESubAgentSpendExceeded</a>: u64 = 30;
</code></pre>



<a name="social_contracts_memory_ESubAgentAccountMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentAccountMismatch">ESubAgentAccountMismatch</a>: u64 = 31;
</code></pre>



<a name="social_contracts_memory_ESubAgentWrongSigner"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentWrongSigner">ESubAgentWrongSigner</a>: u64 = 32;
</code></pre>



<a name="social_contracts_memory_EInvalidAncestorChain"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidAncestorChain">EInvalidAncestorChain</a>: u64 = 33;
</code></pre>



<a name="social_contracts_memory_ECapEscalation"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ECapEscalation">ECapEscalation</a>: u64 = 34;
</code></pre>



<a name="social_contracts_memory_EOrganizationLimitExceeded"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationLimitExceeded">EOrganizationLimitExceeded</a>: u64 = 35;
</code></pre>



<a name="social_contracts_memory_EInvalidOrgType"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidOrgType">EInvalidOrgType</a>: u64 = 36;
</code></pre>



<a name="social_contracts_memory_EOrganizationNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotFound">EOrganizationNotFound</a>: u64 = 37;
</code></pre>



<a name="social_contracts_memory_EOrganizationNotActive"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>: u64 = 38;
</code></pre>



<a name="social_contracts_memory_EOrganizationAccountMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationAccountMismatch">EOrganizationAccountMismatch</a>: u64 = 39;
</code></pre>



<a name="social_contracts_memory_EOrganizationHasRoot"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationHasRoot">EOrganizationHasRoot</a>: u64 = 40;
</code></pre>



<a name="social_contracts_memory_EOrganizationMissingRoot"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationMissingRoot">EOrganizationMissingRoot</a>: u64 = 41;
</code></pre>



<a name="social_contracts_memory_EOrganizationOrgMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationOrgMismatch">EOrganizationOrgMismatch</a>: u64 = 42;
</code></pre>



<a name="social_contracts_memory_EOrgCategoryUpdateCooldown"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgCategoryUpdateCooldown">EOrgCategoryUpdateCooldown</a>: u64 = 43;
</code></pre>



<a name="social_contracts_memory_ENameTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENameTooLong">ENameTooLong</a>: u64 = 44;
</code></pre>



<a name="social_contracts_memory_EDescriptionTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EDescriptionTooLong">EDescriptionTooLong</a>: u64 = 45;
</code></pre>



<a name="social_contracts_memory_ENoAccess"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENoAccess">ENoAccess</a>: u64 = 100;
</code></pre>



<a name="social_contracts_memory_ED25519_PUBLIC_KEY_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ED25519_PUBLIC_KEY_LENGTH">ED25519_PUBLIC_KEY_LENGTH</a>: u64 = 32;
</code></pre>



<a name="social_contracts_memory_MAX_LABEL_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_MAX_LABEL_LENGTH">MAX_LABEL_LENGTH</a>: u64 = 64;
</code></pre>



<a name="social_contracts_memory_MAX_ORG_NAME_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORG_NAME_LENGTH">MAX_ORG_NAME_LENGTH</a>: u64 = 100;
</code></pre>



<a name="social_contracts_memory_MAX_ORG_DESCRIPTION_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORG_DESCRIPTION_LENGTH">MAX_ORG_DESCRIPTION_LENGTH</a>: u64 = 1200;
</code></pre>



<a name="social_contracts_memory_MAX_AGENT_DEPTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_MAX_AGENT_DEPTH">MAX_AGENT_DEPTH</a>: u8 = 8;
</code></pre>



<a name="social_contracts_memory_VERSION"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>: u64 = 4;
</code></pre>



<a name="social_contracts_memory_VERSION_DF_KEY"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>: vector&lt;u8&gt; = vector[109, 101, 109, 111, 114, 121, 95, 118, 101, 114, 115, 105, 111, 110];
</code></pre>



<a name="social_contracts_memory_class_human"></a>

## Function `class_human`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_class_human">class_human</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_class_human">class_human</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_CLASS_HUMAN">CLASS_HUMAN</a> }
</code></pre>



</details>

<a name="social_contracts_memory_class_delegated_ai"></a>

## Function `class_delegated_ai`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_class_delegated_ai">class_delegated_ai</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_class_delegated_ai">class_delegated_ai</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_CLASS_DELEGATED_AI">CLASS_DELEGATED_AI</a> }
</code></pre>



</details>

<a name="social_contracts_memory_class_organization"></a>

## Function `class_organization`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_class_organization">class_organization</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_class_organization">class_organization</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_CLASS_ORGANIZATION">CLASS_ORGANIZATION</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_company"></a>

## Function `org_type_company`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_company">org_type_company</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_company">org_type_company</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_COMPANY">ORG_TYPE_COMPANY</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_startup"></a>

## Function `org_type_startup`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_startup">org_type_startup</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_startup">org_type_startup</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_STARTUP">ORG_TYPE_STARTUP</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_investment_fund"></a>

## Function `org_type_investment_fund`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_investment_fund">org_type_investment_fund</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_investment_fund">org_type_investment_fund</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_INVESTMENT_FUND">ORG_TYPE_INVESTMENT_FUND</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_nonprofit"></a>

## Function `org_type_nonprofit`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_nonprofit">org_type_nonprofit</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_nonprofit">org_type_nonprofit</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_NONPROFIT">ORG_TYPE_NONPROFIT</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_research"></a>

## Function `org_type_research`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_research">org_type_research</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_research">org_type_research</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_RESEARCH">ORG_TYPE_RESEARCH</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_government"></a>

## Function `org_type_government`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_government">org_type_government</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_government">org_type_government</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_GOVERNMENT">ORG_TYPE_GOVERNMENT</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_media"></a>

## Function `org_type_media`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_media">org_type_media</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_media">org_type_media</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_MEDIA">ORG_TYPE_MEDIA</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_stewardship"></a>

## Function `org_type_stewardship`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_stewardship">org_type_stewardship</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_stewardship">org_type_stewardship</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_STEWARDSHIP">ORG_TYPE_STEWARDSHIP</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_brand"></a>

## Function `org_type_brand`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_brand">org_type_brand</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_brand">org_type_brand</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_BRAND">ORG_TYPE_BRAND</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_community"></a>

## Function `org_type_community`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_community">org_type_community</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_community">org_type_community</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_COMMUNITY">ORG_TYPE_COMMUNITY</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_sports"></a>

## Function `org_type_sports`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_sports">org_type_sports</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_sports">org_type_sports</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_SPORTS">ORG_TYPE_SPORTS</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_education"></a>

## Function `org_type_education`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_education">org_type_education</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_education">org_type_education</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_EDUCATION">ORG_TYPE_EDUCATION</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_healthcare"></a>

## Function `org_type_healthcare`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_healthcare">org_type_healthcare</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_healthcare">org_type_healthcare</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_HEALTHCARE">ORG_TYPE_HEALTHCARE</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_other"></a>

## Function `org_type_other`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_other">org_type_other</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_other">org_type_other</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_OTHER">ORG_TYPE_OTHER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_type_count"></a>

## Function `org_type_count`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_count">org_type_count</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_type_count">org_type_count</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_COUNT">ORG_TYPE_COUNT</a> }
</code></pre>



</details>

<a name="social_contracts_memory_max_organizations_per_user"></a>

## Function `max_organizations_per_user`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORGANIZATIONS_PER_USER">MAX_ORGANIZATIONS_PER_USER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_memory_read"></a>

## Function `cap_memory_read`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_memory_read">cap_memory_read</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_memory_read">cap_memory_read</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MEMORY_READ">CAP_MEMORY_READ</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_memory_write"></a>

## Function `cap_memory_write`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_memory_write">cap_memory_write</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_memory_write">cap_memory_write</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MEMORY_WRITE">CAP_MEMORY_WRITE</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_mydata_read"></a>

## Function `cap_mydata_read`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_mydata_read">cap_mydata_read</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_mydata_read">cap_mydata_read</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MYDATA_READ">CAP_MYDATA_READ</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_post_publish"></a>

## Function `cap_post_publish`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">cap_post_publish</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_post_publish">cap_post_publish</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_POST_PUBLISH">CAP_POST_PUBLISH</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_comment"></a>

## Function `cap_comment`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_comment">cap_comment</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_comment">cap_comment</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_COMMENT">CAP_COMMENT</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_react"></a>

## Function `cap_react`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_react">cap_react</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_react">cap_react</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_REACT">CAP_REACT</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_message_read"></a>

## Function `cap_message_read`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_message_read">cap_message_read</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_message_read">cap_message_read</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MESSAGE_READ">CAP_MESSAGE_READ</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_message_send"></a>

## Function `cap_message_send`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_message_send">cap_message_send</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_message_send">cap_message_send</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MESSAGE_SEND">CAP_MESSAGE_SEND</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_trade_monitor"></a>

## Function `cap_trade_monitor`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_trade_monitor">cap_trade_monitor</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_trade_monitor">cap_trade_monitor</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_TRADE_MONITOR">CAP_TRADE_MONITOR</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_trade_execute"></a>

## Function `cap_trade_execute`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_trade_execute">cap_trade_execute</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_trade_execute">cap_trade_execute</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_TRADE_EXECUTE">CAP_TRADE_EXECUTE</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_agent_register"></a>

## Function `cap_agent_register`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_agent_register">cap_agent_register</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_agent_register">cap_agent_register</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_REGISTER">CAP_AGENT_REGISTER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_agent_revoke"></a>

## Function `cap_agent_revoke`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_agent_revoke">cap_agent_revoke</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_agent_revoke">cap_agent_revoke</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_REVOKE">CAP_AGENT_REVOKE</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_agent_update"></a>

## Function `cap_agent_update`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_agent_update">cap_agent_update</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_agent_update">cap_agent_update</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_UPDATE">CAP_AGENT_UPDATE</a> }
</code></pre>



</details>

<a name="social_contracts_memory_register_child"></a>

## Function `register_child`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_child">register_child</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_child">register_child</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_CHILD">REGISTER_CHILD</a> }
</code></pre>



</details>

<a name="social_contracts_memory_register_peer"></a>

## Function `register_peer`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_peer">register_peer</a>(): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_peer">register_peer</a>(): u8 { <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_PEER">REGISTER_PEER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_derive_sub_agent_address"></a>

## Function `derive_sub_agent_address`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_derive_sub_agent_address">derive_sub_agent_address</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, derived_address: <b>address</b>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_derive_sub_agent_address">derive_sub_agent_address</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, derived_address: <b>address</b>): <b>address</b> {
    derived_object::derive_address(object::id(account), <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentKey">SubAgentKey</a> { derived_address })
}
</code></pre>



</details>

<a name="social_contracts_memory_agent_object_id"></a>

## Function `agent_object_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>): ID {
    object::id(agent)
}
</code></pre>



</details>

<a name="social_contracts_memory_organization_id"></a>

## Function `organization_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>): ID {
    object::id(org)
}
</code></pre>



</details>

<a name="social_contracts_memory_sub_agent_organization_id"></a>

## Function `sub_agent_organization_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_organization_id">sub_agent_organization_id</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_organization_id">sub_agent_organization_id</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>): ID {
    agent.<a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>
}
</code></pre>



</details>

<a name="social_contracts_memory_organization_org_type"></a>

## Function `organization_org_type`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_org_type">organization_org_type</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_org_type">organization_org_type</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>): u8 {
    org.org_type
}
</code></pre>



</details>

<a name="social_contracts_memory_organization_root_agent_id"></a>

## Function `organization_root_agent_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_root_agent_id">organization_root_agent_id</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_root_agent_id">organization_root_agent_id</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>): Option&lt;ID&gt; {
    org.root_agent_id
}
</code></pre>



</details>

<a name="social_contracts_memory_organization_name"></a>

## Function `organization_name`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_name">organization_name</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_name">organization_name</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>): &Option&lt;String&gt; {
    &org.name
}
</code></pre>



</details>

<a name="social_contracts_memory_organization_description"></a>

## Function `organization_description`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_description">organization_description</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>): &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_description">organization_description</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>): &Option&lt;String&gt; {
    &org.description
}
</code></pre>



</details>

<a name="social_contracts_memory_bootstrap_init"></a>

## Function `bootstrap_init`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bootstrap_init">bootstrap_init</a>(_clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bootstrap_init">bootstrap_init</a>(_clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> <b>mut</b> registry = <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a> {
        id: object::new(ctx),
        accounts: table::new(ctx),
    };
    <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(&<b>mut</b> registry.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    transfer::share_object(registry);
}
</code></pre>



</details>

<a name="social_contracts_memory_create_account_for_profile"></a>

## Function `create_account_for_profile`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_account_for_profile">create_account_for_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>, <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_account_for_profile">create_account_for_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>,
    <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): ID {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&registry.id);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>assert</b>!(!table::contains(&registry.accounts, sender), <a href="../social_contracts/memory.md#social_contracts_memory_EAccountAlreadyExists">EAccountAlreadyExists</a>);
    <b>let</b> <b>mut</b> account = <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a> {
        id: object::new(ctx),
        <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: sender,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        created_at: clock::timestamp_ms(clock),
        active: <b>true</b>,
        agents: table::new(ctx),
        agent_ids: table::new(ctx),
        organizations: table::new(ctx),
        org_count: 0,
    };
    <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(&<b>mut</b> account.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    <b>let</b> account_id = object::id(&account);
    table::add(&<b>mut</b> registry.accounts, sender, account_id);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountCreated">MemoryAccountCreated</a> {
        account_id,
        <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: sender,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
    });
    transfer::share_object(account);
    account_id
}
</code></pre>



</details>

<a name="social_contracts_memory_transfer_account_owner_with_profile"></a>

## Function `transfer_account_owner_with_profile`

Profile transfer must revoke all sub-agent objects in the same PTB (via [<code><a href="../social_contracts/memory.md#social_contracts_memory_revoke_sub_agent">revoke_sub_agent</a></code>])
before calling [<code><a href="../social_contracts/profile.md#social_contracts_profile_transfer_profile_with_memory">profile::transfer_profile_with_memory</a></code>].


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_transfer_account_owner_with_profile">transfer_account_owner_with_profile</a>(registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b>, old_owner: <b>address</b>, new_owner: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_transfer_account_owner_with_profile">transfer_account_owner_with_profile</a>(
    registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>,
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: <b>address</b>,
    old_owner: <b>address</b>,
    new_owner: <b>address</b>,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&registry.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> == old_owner, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a> == <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(table::contains(&registry.accounts, old_owner), <a href="../social_contracts/memory.md#social_contracts_memory_ERegistryAccountMismatch">ERegistryAccountMismatch</a>);
    <b>assert</b>!(*table::borrow(&registry.accounts, old_owner) == object::id(account), <a href="../social_contracts/memory.md#social_contracts_memory_ERegistryAccountMismatch">ERegistryAccountMismatch</a>);
    <b>assert</b>!(!table::contains(&registry.accounts, new_owner), <a href="../social_contracts/memory.md#social_contracts_memory_ENewOwnerHasMemoryAccount">ENewOwnerHasMemoryAccount</a>);
    table::remove(&<b>mut</b> registry.accounts, old_owner);
    account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> = new_owner;
    table::add(&<b>mut</b> registry.accounts, new_owner, object::id(account));
}
</code></pre>



</details>

<a name="social_contracts_memory_create_agentic_organization"></a>

## Function `create_agentic_organization`

Human owner creates a competitive agentic organization (max 8 per account).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_agentic_organization">create_agentic_organization</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org_type: u8, name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_agentic_organization">create_agentic_organization</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org_type: u8,
    name: Option&lt;String&gt;,
    description: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>let</b> _ = <a href="../social_contracts/memory.md#social_contracts_memory_create_agentic_organization_internal">create_agentic_organization_internal</a>(account, org_type, name, description, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_memory_create_agentic_organization_internal"></a>

## Function `create_agentic_organization_internal`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_agentic_organization_internal">create_agentic_organization_internal</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org_type: u8, name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_agentic_organization_internal">create_agentic_organization_internal</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org_type: u8,
    name: Option&lt;String&gt;,
    description: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
): ID {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_type">assert_valid_org_type</a>(org_type);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_name_within_limit">assert_org_name_within_limit</a>(&name);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_description_within_limit">assert_org_description_within_limit</a>(&description);
    <b>assert</b>!(account.org_count &lt; <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORGANIZATIONS_PER_USER">MAX_ORGANIZATIONS_PER_USER</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationLimitExceeded">EOrganizationLimitExceeded</a>);
    <b>let</b> org = <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a> {
        id: object::new(ctx),
        memory_account_id: object::id(account),
        principal_owner: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        name,
        description,
        org_type,
        root_agent_id: option::none(),
        active: <b>true</b>,
        created_at: clock::timestamp_ms(clock),
        deactivated_at: option::none(),
        category_updated_at: option::none(),
    };
    <b>let</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a> = object::id(&org);
    table::add(
        &<b>mut</b> account.organizations,
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_OrgRegistryEntry">OrgRegistryEntry</a> { active: <b>true</b> },
    );
    account.org_count = account.org_count + 1;
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganizationCreated">AgenticOrganizationCreated</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>,
        account_id: object::id(account),
        principal_owner: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        name: org.name,
        description: org.description,
        org_type: org.org_type,
        created_at: org.created_at,
    });
    transfer::share_object(org);
    <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>
}
</code></pre>



</details>

<a name="social_contracts_memory_update_agentic_organization_metadata"></a>

## Function `update_agentic_organization_metadata`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_agentic_organization_metadata">update_agentic_organization_metadata</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_agentic_organization_metadata">update_agentic_organization_metadata</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    name: Option&lt;String&gt;,
    description: Option&lt;String&gt;,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(tx_context::sender(ctx) == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_name_within_limit">assert_org_name_within_limit</a>(&name);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_description_within_limit">assert_org_description_within_limit</a>(&description);
    org.name = name;
    org.description = description;
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganizationUpdated">AgenticOrganizationUpdated</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        name: org.name,
        description: org.description,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_update_agentic_organization_category"></a>

## Function `update_agentic_organization_category`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_agentic_organization_category">update_agentic_organization_category</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, org_type: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_agentic_organization_category">update_agentic_organization_category</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    org_type: u8,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(tx_context::sender(ctx) == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_type">assert_valid_org_type</a>(org_type);
    <b>if</b> (option::is_some(&org.category_updated_at)) {
        <b>let</b> last = *option::borrow(&org.category_updated_at);
        <b>assert</b>!(
            clock::timestamp_ms(clock) &gt;= last + <a href="../social_contracts/memory.md#social_contracts_memory_ORG_CATEGORY_UPDATE_COOLDOWN_MS">ORG_CATEGORY_UPDATE_COOLDOWN_MS</a>,
            <a href="../social_contracts/memory.md#social_contracts_memory_EOrgCategoryUpdateCooldown">EOrgCategoryUpdateCooldown</a>,
        );
    };
    <b>let</b> previous_org_type = org.org_type;
    org.org_type = org_type;
    org.category_updated_at = option::some(clock::timestamp_ms(clock));
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganizationCategoryUpdated">AgenticOrganizationCategoryUpdated</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        org_type,
        previous_org_type,
        updated_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_deactivate_agentic_organization"></a>

## Function `deactivate_agentic_organization`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_deactivate_agentic_organization">deactivate_agentic_organization</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_deactivate_agentic_organization">deactivate_agentic_organization</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(tx_context::sender(ctx) == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>if</b> (!org.active) {
        <b>return</b>
    };
    org.active = <b>false</b>;
    org.deactivated_at = option::some(clock::timestamp_ms(clock));
    <b>let</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a> = object::id(org);
    <b>if</b> (table::contains(&account.organizations, <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>)) {
        <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> account.organizations, <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>);
        <b>entry</b>.active = <b>false</b>;
    };
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganizationDeactivated">AgenticOrganizationDeactivated</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>,
        deactivated_at: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_register_sub_agent"></a>

## Function `register_sub_agent`

Human owner registers a root-level sub-agent bound to an organization.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent">register_sub_agent</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, organization: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent">register_sub_agent</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    organization: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    public_key: vector&lt;u8&gt;,
    derived_address: <b>address</b>,
    label: String,
    identity_class: u8,
    role_tags: u64,
    capabilities: u64,
    delegatable_caps: u64,
    register_scope: u8,
    approval_required_caps: u64,
    max_action_spend: Option&lt;u64&gt;,
    platform_scope: Option&lt;<b>address</b>&gt;,
    expires_at: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_ready_for_root">assert_organization_ready_for_root</a>(account, organization);
    <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_internal">register_sub_agent_internal</a>(
        account,
        organization,
        public_key,
        derived_address,
        label,
        identity_class,
        role_tags,
        capabilities,
        delegatable_caps,
        register_scope,
        approval_required_caps,
        max_action_spend,
        platform_scope,
        expires_at,
        clock,
        ctx,
    );
}
</code></pre>



</details>

<a name="social_contracts_memory_register_sub_agent_delegated"></a>

## Function `register_sub_agent_delegated`

Delegated agent registers a child or peer sub-agent.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_delegated">register_sub_agent_delegated</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, register_relation: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_delegated">register_sub_agent_delegated</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    parent_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    public_key: vector&lt;u8&gt;,
    derived_address: <b>address</b>,
    label: String,
    identity_class: u8,
    role_tags: u64,
    capabilities: u64,
    delegatable_caps: u64,
    register_scope: u8,
    approval_required_caps: u64,
    max_action_spend: Option&lt;u64&gt;,
    platform_scope: Option&lt;<b>address</b>&gt;,
    expires_at: Option&lt;u64&gt;,
    register_relation: u8,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == parent_agent.derived_address, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidRegistrar">EInvalidRegistrar</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_delegated_internal">register_sub_agent_delegated_internal</a>(
        account,
        parent_agent,
        public_key,
        derived_address,
        label,
        identity_class,
        role_tags,
        capabilities,
        delegatable_caps,
        register_scope,
        approval_required_caps,
        max_action_spend,
        platform_scope,
        expires_at,
        register_relation,
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_memory_update_sub_agent"></a>

## Function `update_sub_agent`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_sub_agent">update_sub_agent</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_sub_agent">update_sub_agent</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    identity_class: u8,
    role_tags: u64,
    capabilities: u64,
    delegatable_caps: u64,
    register_scope: u8,
    approval_required_caps: u64,
    max_action_spend: Option&lt;u64&gt;,
    platform_scope: Option&lt;<b>address</b>&gt;,
    expires_at: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_agent_belongs_to_account">assert_agent_belongs_to_account</a>(account, agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_identity_class">assert_valid_identity_class</a>(identity_class);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_register_scope">assert_valid_register_scope</a>(register_scope);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_may_manage">assert_may_manage</a>(account, agent, <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_UPDATE">CAP_AGENT_UPDATE</a>, clock, ctx);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_update_caps_monotonic">assert_update_caps_monotonic</a>(account, agent, capabilities, delegatable_caps, platform_scope, ctx);
    agent.identity_class = identity_class;
    agent.role_tags = role_tags;
    agent.capabilities = capabilities;
    agent.delegatable_caps = delegatable_caps;
    agent.register_scope = register_scope;
    agent.constraints = <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentConstraints">SubAgentConstraints</a> {
        approval_required_caps,
        max_action_spend,
    };
    agent.platform_scope = platform_scope;
    agent.expires_at = expires_at;
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_not_expired">assert_sub_agent_not_expired</a>(agent, clock);
    <a href="../social_contracts/memory.md#social_contracts_memory_sync_registry_from_agent">sync_registry_from_agent</a>(account, agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_updated">emit_sub_agent_updated</a>(account, agent);
}
</code></pre>



</details>

<a name="social_contracts_memory_update_sub_agent_label"></a>

## Function `update_sub_agent_label`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_sub_agent_label">update_sub_agent_label</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_sub_agent_label">update_sub_agent_label</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    label: String,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_agent_belongs_to_account">assert_agent_belongs_to_account</a>(account, agent);
    <b>assert</b>!(string::length(&label) &lt;= <a href="../social_contracts/memory.md#social_contracts_memory_MAX_LABEL_LENGTH">MAX_LABEL_LENGTH</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ELabelTooLong">ELabelTooLong</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_may_manage">assert_may_manage</a>(account, agent, <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_UPDATE">CAP_AGENT_UPDATE</a>, clock, ctx);
    agent.label = label;
    <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_updated">emit_sub_agent_updated</a>(account, agent);
}
</code></pre>



</details>

<a name="social_contracts_memory_deactivate_sub_agent"></a>

## Function `deactivate_sub_agent`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_deactivate_sub_agent">deactivate_sub_agent</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_deactivate_sub_agent">deactivate_sub_agent</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_agent_belongs_to_account">assert_agent_belongs_to_account</a>(account, agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_may_manage">assert_may_manage</a>(account, agent, <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_REVOKE">CAP_AGENT_REVOKE</a>, clock, ctx);
    <b>if</b> (!agent.active) {
        <b>return</b>
    };
    agent.active = <b>false</b>;
    <a href="../social_contracts/memory.md#social_contracts_memory_sync_registry_active">sync_registry_active</a>(account, agent.derived_address, <b>false</b>);
    <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_deactivated">emit_sub_agent_deactivated</a>(account, agent);
}
</code></pre>



</details>

<a name="social_contracts_memory_revoke_sub_agent"></a>

## Function `revoke_sub_agent`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_revoke_sub_agent">revoke_sub_agent</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_revoke_sub_agent">revoke_sub_agent</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    agent: <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_agent_belongs_to_account">assert_agent_belongs_to_account</a>(account, &agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_may_manage">assert_may_manage</a>(account, &agent, <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_REVOKE">CAP_AGENT_REVOKE</a>, clock, ctx);
    <b>let</b> derived_address = agent.derived_address;
    <b>let</b> <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a> = object::id(&agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_revoked">emit_sub_agent_revoked</a>(account, &agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_remove_registry_entry">remove_registry_entry</a>(account, derived_address, <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_destroy_sub_agent">destroy_sub_agent</a>(agent);
}
</code></pre>



</details>

<a name="social_contracts_memory_emit_sub_agents_cleared_on_transfer"></a>

## Function `emit_sub_agents_cleared_on_transfer`

Emit bulk-clear audit after the last agent revoke during profile transfer orchestration.


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agents_cleared_on_transfer">emit_sub_agents_cleared_on_transfer</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, previous_owner: <b>address</b>, new_owner: <b>address</b>, revoked_count: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agents_cleared_on_transfer">emit_sub_agents_cleared_on_transfer</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    previous_owner: <b>address</b>,
    new_owner: <b>address</b>,
    revoked_count: u64,
) {
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_SubAgentsClearedOnTransfer">SubAgentsClearedOnTransfer</a> {
        account_id: object::id(account),
        principal_owner: new_owner,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        previous_owner,
        new_owner,
        revoked_count,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_ensure_agent_memory_vault"></a>

## Function `ensure_agent_memory_vault`

Lazy-create per-agent memory vault derived from the sub-agent object.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_ensure_agent_memory_vault">ensure_agent_memory_vault</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, _ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_ensure_agent_memory_vault">ensure_agent_memory_vault</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    clock: &Clock,
    _ctx: &TxContext,
): ID {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_agent_belongs_to_account">assert_agent_belongs_to_account</a>(account, agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">assert_sub_agent_active</a>(agent, clock);
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_AgentMemoryVaultKey">AgentMemoryVaultKey</a> {};
    <b>if</b> (derived_object::exists(&agent.id, key)) {
        <b>let</b> addr = derived_object::derive_address(object::id(agent), key);
        <b>return</b> addr.to_id()
    };
    <b>let</b> vault_uid = derived_object::claim(&<b>mut</b> agent.id, key);
    <b>let</b> vault_id = object::uid_to_inner(&vault_uid);
    <b>let</b> vault = <a href="../social_contracts/memory.md#social_contracts_memory_AgentMemoryVault">AgentMemoryVault</a> {
        id: vault_uid,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: object::id(agent),
        memory_account_id: object::id(account),
        created_at: clock::timestamp_ms(clock),
    };
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_AgentMemoryVaultCreated">AgentMemoryVaultCreated</a> {
        vault_id,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: object::id(agent),
        memory_account_id: object::id(account),
    });
    transfer::share_object(vault);
    vault_id
}
</code></pre>



</details>

<a name="social_contracts_memory_deactivate_account"></a>

## Function `deactivate_account`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_deactivate_account">deactivate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_deactivate_account">deactivate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, ctx: &TxContext) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> == tx_context::sender(ctx), <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    account.active = <b>false</b>;
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountDeactivated">MemoryAccountDeactivated</a> {
        account_id: object::id(account),
        <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_reactivate_account"></a>

## Function `reactivate_account`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_reactivate_account">reactivate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_reactivate_account">reactivate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, ctx: &TxContext) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> == tx_context::sender(ctx), <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>assert</b>!(!account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountAlreadyActive">EAccountAlreadyActive</a>);
    account.active = <b>true</b>;
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountReactivated">MemoryAccountReactivated</a> {
        account_id: object::id(account),
        <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_migrate_account"></a>

## Function `migrate_account`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_migrate_account">migrate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_migrate_account">migrate_account</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, ctx: &<b>mut</b> TxContext) {
    <b>assert</b>!(account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> == tx_context::sender(ctx), <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>let</b> cur = <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(&account.id);
    <b>assert</b>!(cur &lt; <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EAlreadyMigrated">EAlreadyMigrated</a>);
    <b>let</b> _ = ctx;
    <a href="../social_contracts/memory.md#social_contracts_memory_bump_version">bump_version</a>(&<b>mut</b> account.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountMigrated">MemoryAccountMigrated</a> {
        account_id: object::id(account),
        from: cur,
        to: <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_admin_migrate_account"></a>

## Function `admin_migrate_account`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_admin_migrate_account">admin_migrate_account</a>(cap: &<a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_admin_migrate_account">admin_migrate_account</a>(cap: &UpgradeCap, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, ctx: &<b>mut</b> TxContext) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_cap_for_this_package">assert_cap_for_this_package</a>(cap);
    <b>let</b> cur = <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(&account.id);
    <b>assert</b>!(cur &lt; <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EAlreadyMigrated">EAlreadyMigrated</a>);
    <b>let</b> _ = ctx;
    <a href="../social_contracts/memory.md#social_contracts_memory_bump_version">bump_version</a>(&<b>mut</b> account.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccountMigrated">MemoryAccountMigrated</a> {
        account_id: object::id(account),
        from: cur,
        to: <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_migrate_registry"></a>

## Function `migrate_registry`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_migrate_registry">migrate_registry</a>(cap: &<a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>, registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_migrate_registry">migrate_registry</a>(cap: &UpgradeCap, registry: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_cap_for_this_package">assert_cap_for_this_package</a>(cap);
    <b>let</b> cur = <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(&registry.id);
    <b>assert</b>!(cur &lt; <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EAlreadyMigrated">EAlreadyMigrated</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_bump_version">bump_version</a>(&<b>mut</b> registry.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistryMigrated">MemoryRegistryMigrated</a> {
        registry_id: object::id(registry),
        from: cur,
        to: <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_resolve_human_actor"></a>

## Function `resolve_human_actor`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_human_actor">resolve_human_actor</a>(root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_human_actor">resolve_human_actor</a>(root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, ctx: &TxContext): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a> {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&root.id);
    <b>assert</b>!(root.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    <b>assert</b>!(tx_context::sender(ctx) == root.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENoAccess">ENoAccess</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a> {
        principal_owner: root.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        principal_profile_id: root.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        actor_address: root.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        sub_agent_id: option::none(),
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: option::none(),
        identity_class: <a href="../social_contracts/memory.md#social_contracts_memory_CLASS_HUMAN">CLASS_HUMAN</a>,
        parent_object_id: option::none(),
        depth: 0,
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_resolve_actor_from_account"></a>

## Function `resolve_actor_from_account`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_from_account">resolve_actor_from_account</a>(root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_from_account">resolve_actor_from_account</a>(
    root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    clock: &Clock,
    ctx: &TxContext,
): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a> {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&root.id);
    <b>assert</b>!(root.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    <b>let</b> sender = tx_context::sender(ctx);
    <b>if</b> (sender == root.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) {
        <b>return</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_human_actor">resolve_human_actor</a>(root, ctx)
    };
    <b>assert</b>!(table::contains(&root.agents, sender), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotFound">ESubAgentNotFound</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_ancestor_chain_active_from_table">assert_ancestor_chain_active_from_table</a>(root, sender, clock);
    <b>let</b> <b>entry</b> = table::borrow(&root.agents, sender);
    <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a> {
        principal_owner: root.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        principal_profile_id: root.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        actor_address: sender,
        sub_agent_id: option::some(<b>entry</b>.<a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>),
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: option::some(<b>entry</b>.<a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>),
        identity_class: <b>entry</b>.identity_class,
        parent_object_id: <b>entry</b>.parent_object_id,
        depth: <b>entry</b>.depth,
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_resolve_actor_with_cap"></a>

## Function `resolve_actor_with_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">resolve_actor_with_cap</a>(root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, required_cap: u64, action_platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, spend_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">resolve_actor_with_cap</a>(
    root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    required_cap: u64,
    action_platform_id: Option&lt;<b>address</b>&gt;,
    spend_amount: u64,
    clock: &Clock,
    ctx: &TxContext,
): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a> {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>if</b> (sender == root.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) {
        <b>return</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_human_actor">resolve_human_actor</a>(root, ctx)
    };
    <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_from_account">resolve_actor_from_account</a>(root, clock, ctx);
    <b>let</b> <b>entry</b> = table::borrow(&root.agents, sender);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_has_cap">has_cap</a>(<b>entry</b>.capabilities, required_cap), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentMissingCap">ESubAgentMissingCap</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_platform_scope_entry">assert_platform_scope_entry</a>(<b>entry</b>, action_platform_id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_action_spend_limit_from_entry">assert_action_spend_limit_from_entry</a>(root, <b>entry</b>, spend_amount, ctx);
    acting
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_human_actor_with_cap"></a>

## Function `assert_human_actor_with_cap`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_human_actor_with_cap">assert_human_actor_with_cap</a>(root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_human_actor_with_cap">assert_human_actor_with_cap</a>(
    root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    ctx: &TxContext,
): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a> {
    <a href="../social_contracts/memory.md#social_contracts_memory_resolve_human_actor">resolve_human_actor</a>(root, ctx)
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_action_spend_limit"></a>

## Function `assert_action_spend_limit`

Per-transaction MYSO (MIST) spend ceiling for sub-agents. Principal owner is exempt.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_action_spend_limit">assert_action_spend_limit</a>(root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, spend_amount: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_action_spend_limit">assert_action_spend_limit</a>(
    root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    spend_amount: u64,
    ctx: &TxContext,
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>if</b> (caller == root.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) {
        <b>return</b>
    };
    <b>assert</b>!(table::contains(&root.agents, caller), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotFound">ESubAgentNotFound</a>);
    <b>let</b> <b>entry</b> = table::borrow(&root.agents, caller);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_action_spend_limit_from_entry">assert_action_spend_limit_from_entry</a>(root, <b>entry</b>, spend_amount, ctx);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_direct_execution_allowed"></a>

## Function `assert_direct_execution_allowed`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_direct_execution_allowed">assert_direct_execution_allowed</a>(root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, required_cap: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_direct_execution_allowed">assert_direct_execution_allowed</a>(
    root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    required_cap: u64,
    ctx: &TxContext,
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>if</b> (caller == root.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) {
        <b>return</b>
    };
    <b>assert</b>!(table::contains(&root.agents, caller), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotFound">ESubAgentNotFound</a>);
    <b>let</b> <b>entry</b> = table::borrow(&root.agents, caller);
    <b>assert</b>!(!<a href="../social_contracts/memory.md#social_contracts_memory_cap_requires_approval">cap_requires_approval</a>(&<b>entry</b>.constraints, required_cap), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentApprovalRequired">ESubAgentApprovalRequired</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_platform_scope_entry"></a>

## Function `assert_platform_scope_entry`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_platform_scope_entry">assert_platform_scope_entry</a>(<b>entry</b>: &<a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">social_contracts::memory::AgentRegistryEntry</a>, action_platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_platform_scope_entry">assert_platform_scope_entry</a>(
    <b>entry</b>: &<a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">AgentRegistryEntry</a>,
    action_platform_id: Option&lt;<b>address</b>&gt;,
) {
    <b>if</b> (option::is_none(&<b>entry</b>.platform_scope)) {
        <b>return</b>
    };
    <b>let</b> scope = *option::borrow(&<b>entry</b>.platform_scope);
    <b>assert</b>!(option::is_some(&action_platform_id), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentWrongPlatformScope">ESubAgentWrongPlatformScope</a>);
    <b>assert</b>!(*option::borrow(&action_platform_id) == scope, <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentWrongPlatformScope">ESubAgentWrongPlatformScope</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_platform_scope"></a>

## Function `assert_platform_scope`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_platform_scope">assert_platform_scope</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, action_platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_platform_scope">assert_platform_scope</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>, action_platform_id: Option&lt;<b>address</b>&gt;) {
    <b>if</b> (option::is_none(&agent.platform_scope)) {
        <b>return</b>
    };
    <b>let</b> scope = *option::borrow(&agent.platform_scope);
    <b>assert</b>!(option::is_some(&action_platform_id), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentWrongPlatformScope">ESubAgentWrongPlatformScope</a>);
    <b>assert</b>!(*option::borrow(&action_platform_id) == scope, <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentWrongPlatformScope">ESubAgentWrongPlatformScope</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_sub_agent_active"></a>

## Function `assert_sub_agent_active`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">assert_sub_agent_active</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">assert_sub_agent_active</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>, clock: &Clock) {
    <b>assert</b>!(agent.active, <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotActive">ESubAgentNotActive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_not_expired">assert_sub_agent_not_expired</a>(agent, clock);
}
</code></pre>



</details>

<a name="social_contracts_memory_profile_id"></a>

## Function `profile_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>): <b>address</b> { account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a> }
</code></pre>



</details>

<a name="social_contracts_memory_owner"></a>

## Function `owner`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>): <b>address</b> { account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a> }
</code></pre>



</details>

<a name="social_contracts_memory_sub_agent_derived_address"></a>

## Function `sub_agent_derived_address`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_derived_address">sub_agent_derived_address</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_derived_address">sub_agent_derived_address</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>): <b>address</b> { agent.derived_address }
</code></pre>



</details>

<a name="social_contracts_memory_sub_agent_capabilities"></a>

## Function `sub_agent_capabilities`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_capabilities">sub_agent_capabilities</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_capabilities">sub_agent_capabilities</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>): u64 { agent.capabilities }
</code></pre>



</details>

<a name="social_contracts_memory_sub_agent_platform_scope"></a>

## Function `sub_agent_platform_scope`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_platform_scope">sub_agent_platform_scope</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_platform_scope">sub_agent_platform_scope</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>): Option&lt;<b>address</b>&gt; { agent.platform_scope }
</code></pre>



</details>

<a name="social_contracts_memory_sub_agent_active"></a>

## Function `sub_agent_active`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_active">sub_agent_active</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_active">sub_agent_active</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>): bool { agent.active }
</code></pre>



</details>

<a name="social_contracts_memory_sub_agent_depth"></a>

## Function `sub_agent_depth`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_depth">sub_agent_depth</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_depth">sub_agent_depth</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>): u8 { agent.depth }
</code></pre>



</details>

<a name="social_contracts_memory_sub_agent_parent_object_id"></a>

## Function `sub_agent_parent_object_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_parent_object_id">sub_agent_parent_object_id</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_parent_object_id">sub_agent_parent_object_id</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>): Option&lt;ID&gt; { agent.parent_object_id }
</code></pre>



</details>

<a name="social_contracts_memory_sub_agent_memory_account_id"></a>

## Function `sub_agent_memory_account_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_memory_account_id">sub_agent_memory_account_id</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sub_agent_memory_account_id">sub_agent_memory_account_id</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>): ID { agent.memory_account_id }
</code></pre>



</details>

<a name="social_contracts_memory_acting_principal_owner"></a>

## Function `acting_principal_owner`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">acting_principal_owner</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_principal_owner">acting_principal_owner</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a>): <b>address</b> { acting.principal_owner }
</code></pre>



</details>

<a name="social_contracts_memory_acting_profile_id"></a>

## Function `acting_profile_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_profile_id">acting_profile_id</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_profile_id">acting_profile_id</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a>): <b>address</b> { acting.principal_profile_id }
</code></pre>



</details>

<a name="social_contracts_memory_acting_actor_address"></a>

## Function `acting_actor_address`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">acting_actor_address</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_actor_address">acting_actor_address</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a>): <b>address</b> { acting.actor_address }
</code></pre>



</details>

<a name="social_contracts_memory_acting_sub_agent_id"></a>

## Function `acting_sub_agent_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">acting_sub_agent_id</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_sub_agent_id">acting_sub_agent_id</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a>): Option&lt;ID&gt; { acting.sub_agent_id }
</code></pre>



</details>

<a name="social_contracts_memory_acting_identity_class"></a>

## Function `acting_identity_class`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">acting_identity_class</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_identity_class">acting_identity_class</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a>): u8 { acting.identity_class }
</code></pre>



</details>

<a name="social_contracts_memory_acting_parent_object_id"></a>

## Function `acting_parent_object_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_parent_object_id">acting_parent_object_id</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_parent_object_id">acting_parent_object_id</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a>): Option&lt;ID&gt; { acting.parent_object_id }
</code></pre>



</details>

<a name="social_contracts_memory_acting_depth"></a>

## Function `acting_depth`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_depth">acting_depth</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_depth">acting_depth</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a>): u8 { acting.depth }
</code></pre>



</details>

<a name="social_contracts_memory_acting_organization_id"></a>

## Function `acting_organization_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">acting_organization_id</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_acting_organization_id">acting_organization_id</a>(acting: &<a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">ActingContext</a>): Option&lt;ID&gt; { acting.<a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a> }
</code></pre>



</details>

<a name="social_contracts_memory_has_account"></a>

## Function `has_account`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_account">has_account</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_account">has_account</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>, addr: <b>address</b>): bool {
    table::contains(&registry.accounts, addr)
}
</code></pre>



</details>

<a name="social_contracts_memory_account_id_for_owner"></a>

## Function `account_id_for_owner`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_account_id_for_owner">account_id_for_owner</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>, <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_account_id_for_owner">account_id_for_owner</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>, <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>: <b>address</b>): Option&lt;ID&gt; {
    <b>if</b> (table::contains(&registry.accounts, <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>)) {
        option::some(*table::borrow(&registry.accounts, <a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_is_registered_agent"></a>

## Function `is_registered_agent`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_registered_agent">is_registered_agent</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, derived: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_registered_agent">is_registered_agent</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, derived: <b>address</b>): bool {
    table::contains(&account.agents, derived)
}
</code></pre>



</details>

<a name="social_contracts_memory_is_active"></a>

## Function `is_active`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_active">is_active</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_active">is_active</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>): bool { account.active }
</code></pre>



</details>

<a name="social_contracts_memory_account_version"></a>

## Function `account_version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_account_version">account_version</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_account_version">account_version</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(&account.id) }
</code></pre>



</details>

<a name="social_contracts_memory_registry_version"></a>

## Function `registry_version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_registry_version">registry_version</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">social_contracts::memory::MemoryRegistry</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_registry_version">registry_version</a>(registry: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a>): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(&registry.id) }
</code></pre>



</details>

<a name="social_contracts_memory_current_contract_version"></a>

## Function `current_contract_version`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_current_contract_version">current_contract_version</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_current_contract_version">current_contract_version</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a> }
</code></pre>



</details>

<a name="social_contracts_memory_approve_key_policy"></a>

## Function `approve_key_policy`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_key_policy">approve_key_policy</a>(id: vector&lt;u8&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_key_policy">approve_key_policy</a>(
    id: vector&lt;u8&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> owner_bytes = bcs::to_bytes(&account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>);
    <b>if</b> ((caller == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) && <a href="../social_contracts/memory.md#social_contracts_memory_has_suffix">has_suffix</a>(&id, &owner_bytes)) {
        <b>return</b>
    };
    <b>if</b> (table::contains(&account.agents, caller)) {
        <b>let</b> <b>entry</b> = table::borrow(&account.agents, caller);
        <b>assert</b>!(option::is_none(&<b>entry</b>.platform_scope), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotGlobalScope">ESubAgentNotGlobalScope</a>);
    };
    <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">resolve_actor_with_cap</a>(
        account,
        <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MEMORY_READ">CAP_MEMORY_READ</a>,
        option::none(),
        0,
        clock,
        ctx,
    );
    <b>assert</b>!(option::is_some(&acting.sub_agent_id), <a href="../social_contracts/memory.md#social_contracts_memory_ENoAccess">ENoAccess</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_approve_key_write_policy"></a>

## Function `approve_key_write_policy`



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_key_write_policy">approve_key_write_policy</a>(id: vector&lt;u8&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_key_write_policy">approve_key_write_policy</a>(
    id: vector&lt;u8&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> owner_bytes = bcs::to_bytes(&account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>);
    <b>if</b> ((caller == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) && <a href="../social_contracts/memory.md#social_contracts_memory_has_suffix">has_suffix</a>(&id, &owner_bytes)) {
        <b>return</b>
    };
    <b>if</b> (table::contains(&account.agents, caller)) {
        <b>let</b> <b>entry</b> = table::borrow(&account.agents, caller);
        <b>assert</b>!(option::is_none(&<b>entry</b>.platform_scope), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotGlobalScope">ESubAgentNotGlobalScope</a>);
    };
    <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">resolve_actor_with_cap</a>(
        account,
        <a href="../social_contracts/memory.md#social_contracts_memory_CAP_MEMORY_WRITE">CAP_MEMORY_WRITE</a>,
        option::none(),
        0,
        clock,
        ctx,
    );
    <b>assert</b>!(option::is_some(&acting.sub_agent_id), <a href="../social_contracts/memory.md#social_contracts_memory_ENoAccess">ENoAccess</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_owner_key_suffix_bytes"></a>

## Function `owner_key_suffix_bytes`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_owner_key_suffix_bytes">owner_key_suffix_bytes</a>(owner_addr: <b>address</b>): vector&lt;u8&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_owner_key_suffix_bytes">owner_key_suffix_bytes</a>(owner_addr: <b>address</b>): vector&lt;u8&gt; {
    bcs::to_bytes(&owner_addr)
}
</code></pre>



</details>

<a name="social_contracts_memory_register_sub_agent_internal"></a>

## Function `register_sub_agent_internal`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_internal">register_sub_agent_internal</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, organization: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_internal">register_sub_agent_internal</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    organization: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    public_key: vector&lt;u8&gt;,
    derived_address: <b>address</b>,
    label: String,
    identity_class: u8,
    role_tags: u64,
    capabilities: u64,
    delegatable_caps: u64,
    register_scope: u8,
    approval_required_caps: u64,
    max_action_spend: Option&lt;u64&gt;,
    platform_scope: Option&lt;<b>address</b>&gt;,
    expires_at: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>let</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a> = object::id(organization);
    <b>let</b> agent_id = <a href="../social_contracts/memory.md#social_contracts_memory_finish_register_sub_agent">finish_register_sub_agent</a>(
        account,
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>,
        public_key,
        derived_address,
        label,
        identity_class,
        role_tags,
        capabilities,
        delegatable_caps,
        register_scope,
        approval_required_caps,
        max_action_spend,
        platform_scope,
        expires_at,
        1,
        option::none(),
        tx_context::sender(ctx),
        clock,
    );
    <a href="../social_contracts/memory.md#social_contracts_memory_bind_root_agent_to_organization">bind_root_agent_to_organization</a>(account, organization, agent_id);
}
</code></pre>



</details>

<a name="social_contracts_memory_register_sub_agent_delegated_internal"></a>

## Function `register_sub_agent_delegated_internal`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_delegated_internal">register_sub_agent_delegated_internal</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, register_relation: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_delegated_internal">register_sub_agent_delegated_internal</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    parent_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    public_key: vector&lt;u8&gt;,
    derived_address: <b>address</b>,
    label: String,
    identity_class: u8,
    role_tags: u64,
    capabilities: u64,
    delegatable_caps: u64,
    register_scope: u8,
    approval_required_caps: u64,
    max_action_spend: Option&lt;u64&gt;,
    platform_scope: Option&lt;<b>address</b>&gt;,
    expires_at: Option&lt;u64&gt;,
    register_relation: u8,
    clock: &Clock,
) {
    <b>let</b> (depth, parent_object_id, registered_by) = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_delegated_registration_placement">resolve_delegated_registration_placement</a>(
        account,
        parent_agent,
        register_relation,
        capabilities,
        delegatable_caps,
        platform_scope,
        clock,
    );
    <a href="../social_contracts/memory.md#social_contracts_memory_finish_register_sub_agent">finish_register_sub_agent</a>(
        account,
        parent_agent.<a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>,
        public_key,
        derived_address,
        label,
        identity_class,
        role_tags,
        capabilities,
        delegatable_caps,
        register_scope,
        approval_required_caps,
        max_action_spend,
        platform_scope,
        expires_at,
        depth,
        parent_object_id,
        registered_by,
        clock,
    );
}
</code></pre>



</details>

<a name="social_contracts_memory_finish_register_sub_agent"></a>

## Function `finish_register_sub_agent`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_finish_register_sub_agent">finish_register_sub_agent</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, depth: u8, parent_object_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, registered_by: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_finish_register_sub_agent">finish_register_sub_agent</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: ID,
    public_key: vector&lt;u8&gt;,
    derived_address: <b>address</b>,
    label: String,
    identity_class: u8,
    role_tags: u64,
    capabilities: u64,
    delegatable_caps: u64,
    register_scope: u8,
    approval_required_caps: u64,
    max_action_spend: Option&lt;u64&gt;,
    platform_scope: Option&lt;<b>address</b>&gt;,
    expires_at: Option&lt;u64&gt;,
    depth: u8,
    parent_object_id: Option&lt;ID&gt;,
    registered_by: <b>address</b>,
    clock: &Clock,
): ID {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_identity_class">assert_valid_identity_class</a>(identity_class);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_register_scope">assert_valid_register_scope</a>(register_scope);
    <b>assert</b>!(table::contains(&account.organizations, <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>), <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotFound">EOrganizationNotFound</a>);
    <b>assert</b>!(vector::length(&public_key) == <a href="../social_contracts/memory.md#social_contracts_memory_ED25519_PUBLIC_KEY_LENGTH">ED25519_PUBLIC_KEY_LENGTH</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidPublicKeyLength">EInvalidPublicKeyLength</a>);
    <b>assert</b>!(string::length(&label) &lt;= <a href="../social_contracts/memory.md#social_contracts_memory_MAX_LABEL_LENGTH">MAX_LABEL_LENGTH</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ELabelTooLong">ELabelTooLong</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_scope_allowed_for_delegate">assert_scope_allowed_for_delegate</a>(option::none(), platform_scope);
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentKey">SubAgentKey</a> { derived_address };
    <b>assert</b>!(!derived_object::exists(&account.id, key), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentDuplicateDerivedAddress">ESubAgentDuplicateDerivedAddress</a>);
    <b>let</b> constraints = <a href="../social_contracts/memory.md#social_contracts_memory_SubAgentConstraints">SubAgentConstraints</a> {
        approval_required_caps,
        max_action_spend,
    };
    <b>let</b> agent_uid = derived_object::claim(&<b>mut</b> account.id, key);
    <b>let</b> agent = <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a> {
        id: agent_uid,
        memory_account_id: object::id(account),
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>,
        principal_owner: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        derived_address,
        public_key,
        label,
        identity_class,
        role_tags,
        capabilities,
        delegatable_caps,
        register_scope,
        constraints,
        platform_scope,
        parent_object_id,
        depth,
        registered_by,
        created_at: clock::timestamp_ms(clock),
        expires_at,
        active: <b>true</b>,
    };
    <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_registered">emit_sub_agent_registered</a>(account, &agent);
    <a href="../social_contracts/memory.md#social_contracts_memory_insert_registry_entry">insert_registry_entry</a>(account, &agent);
    <b>let</b> agent_id = object::id(&agent);
    transfer::share_object(agent);
    agent_id
}
</code></pre>



</details>

<a name="social_contracts_memory_bind_root_agent_to_organization"></a>

## Function `bind_root_agent_to_organization`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bind_root_agent_to_organization">bind_root_agent_to_organization</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, agent_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bind_root_agent_to_organization">bind_root_agent_to_organization</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    agent_id: ID,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <b>assert</b>!(option::is_none(&org.root_agent_id), <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationHasRoot">EOrganizationHasRoot</a>);
    org.root_agent_id = option::some(agent_id);
}
</code></pre>



</details>

<a name="social_contracts_memory_registry_entry_from_agent"></a>

## Function `registry_entry_from_agent`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_registry_entry_from_agent">registry_entry_from_agent</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>): <a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">social_contracts::memory::AgentRegistryEntry</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_registry_entry_from_agent">registry_entry_from_agent</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>): <a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">AgentRegistryEntry</a> {
    <a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">AgentRegistryEntry</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: object::id(agent),
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: agent.<a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>,
        parent_object_id: agent.parent_object_id,
        depth: agent.depth,
        active: agent.active,
        expires_at: agent.expires_at,
        identity_class: agent.identity_class,
        capabilities: agent.capabilities,
        delegatable_caps: agent.delegatable_caps,
        register_scope: agent.register_scope,
        constraints: agent.constraints,
        platform_scope: agent.platform_scope,
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_insert_registry_entry"></a>

## Function `insert_registry_entry`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_insert_registry_entry">insert_registry_entry</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_insert_registry_entry">insert_registry_entry</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>) {
    <b>let</b> derived_address = agent.derived_address;
    <b>let</b> <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a> = object::id(agent);
    <b>assert</b>!(!table::contains(&account.agents, derived_address), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentDuplicateDerivedAddress">ESubAgentDuplicateDerivedAddress</a>);
    <b>assert</b>!(!table::contains(&account.agent_ids, <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentDuplicateDerivedAddress">ESubAgentDuplicateDerivedAddress</a>);
    table::add(
        &<b>mut</b> account.agents,
        derived_address,
        <a href="../social_contracts/memory.md#social_contracts_memory_registry_entry_from_agent">registry_entry_from_agent</a>(agent),
    );
    table::add(&<b>mut</b> account.agent_ids, <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>, derived_address);
}
</code></pre>



</details>

<a name="social_contracts_memory_sync_registry_from_agent"></a>

## Function `sync_registry_from_agent`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sync_registry_from_agent">sync_registry_from_agent</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sync_registry_from_agent">sync_registry_from_agent</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>) {
    <b>let</b> derived_address = agent.derived_address;
    <b>assert</b>!(table::contains(&account.agents, derived_address), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotFound">ESubAgentNotFound</a>);
    <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> account.agents, derived_address);
    *<b>entry</b> = <a href="../social_contracts/memory.md#social_contracts_memory_registry_entry_from_agent">registry_entry_from_agent</a>(agent);
}
</code></pre>



</details>

<a name="social_contracts_memory_sync_registry_active"></a>

## Function `sync_registry_active`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sync_registry_active">sync_registry_active</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, derived_address: <b>address</b>, active: bool)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_sync_registry_active">sync_registry_active</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, derived_address: <b>address</b>, active: bool) {
    <b>assert</b>!(table::contains(&account.agents, derived_address), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotFound">ESubAgentNotFound</a>);
    <b>let</b> <b>entry</b> = table::borrow_mut(&<b>mut</b> account.agents, derived_address);
    <b>entry</b>.active = active;
}
</code></pre>



</details>

<a name="social_contracts_memory_remove_registry_entry"></a>

## Function `remove_registry_entry`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_remove_registry_entry">remove_registry_entry</a>(account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, derived_address: <b>address</b>, <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_remove_registry_entry">remove_registry_entry</a>(
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    derived_address: <b>address</b>,
    <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: ID,
) {
    <b>if</b> (table::contains(&account.agents, derived_address)) {
        <b>let</b> _entry = table::remove(&<b>mut</b> account.agents, derived_address);
    };
    <b>if</b> (table::contains(&account.agent_ids, <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>)) {
        <b>let</b> _derived = table::remove(&<b>mut</b> account.agent_ids, <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_registry_entry_active"></a>

## Function `assert_registry_entry_active`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_registry_entry_active">assert_registry_entry_active</a>(<b>entry</b>: &<a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">social_contracts::memory::AgentRegistryEntry</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_registry_entry_active">assert_registry_entry_active</a>(<b>entry</b>: &<a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">AgentRegistryEntry</a>, clock: &Clock) {
    <b>assert</b>!(<b>entry</b>.active, <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotActive">ESubAgentNotActive</a>);
    <b>if</b> (option::is_some(&<b>entry</b>.expires_at)) {
        <b>assert</b>!(
            clock::timestamp_ms(clock) &lt;= *option::borrow(&<b>entry</b>.expires_at),
            <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentExpired">ESubAgentExpired</a>,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_action_spend_limit_from_entry"></a>

## Function `assert_action_spend_limit_from_entry`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_action_spend_limit_from_entry">assert_action_spend_limit_from_entry</a>(root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <b>entry</b>: &<a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">social_contracts::memory::AgentRegistryEntry</a>, spend_amount: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_action_spend_limit_from_entry">assert_action_spend_limit_from_entry</a>(
    root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    <b>entry</b>: &<a href="../social_contracts/memory.md#social_contracts_memory_AgentRegistryEntry">AgentRegistryEntry</a>,
    spend_amount: u64,
    ctx: &TxContext,
) {
    <b>let</b> caller = tx_context::sender(ctx);
    <b>if</b> (caller == root.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) {
        <b>return</b>
    };
    <b>if</b> (option::is_none(&<b>entry</b>.constraints.max_action_spend)) {
        <b>return</b>
    };
    <b>let</b> max = *option::borrow(&<b>entry</b>.constraints.max_action_spend);
    <b>assert</b>!(spend_amount &lt;= max, <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentSpendExceeded">ESubAgentSpendExceeded</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_ancestor_chain_active_from_table"></a>

## Function `assert_ancestor_chain_active_from_table`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_ancestor_chain_active_from_table">assert_ancestor_chain_active_from_table</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, derived_address: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_ancestor_chain_active_from_table">assert_ancestor_chain_active_from_table</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    derived_address: <b>address</b>,
    clock: &Clock,
) {
    <b>assert</b>!(table::contains(&account.agents, derived_address), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotFound">ESubAgentNotFound</a>);
    <b>let</b> <b>entry</b> = table::borrow(&account.agents, derived_address);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_registry_entry_active">assert_registry_entry_active</a>(<b>entry</b>, clock);
    <b>let</b> <b>mut</b> current_parent = <b>entry</b>.parent_object_id;
    <b>let</b> <b>mut</b> hops = 0u8;
    <b>while</b> (option::is_some(&current_parent)) {
        hops = hops + 1;
        <b>assert</b>!(hops &lt;= <a href="../social_contracts/memory.md#social_contracts_memory_MAX_AGENT_DEPTH">MAX_AGENT_DEPTH</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidAncestorChain">EInvalidAncestorChain</a>);
        <b>let</b> parent_id = *option::borrow(&current_parent);
        <b>assert</b>!(table::contains(&account.agent_ids, parent_id), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentInactiveAncestor">ESubAgentInactiveAncestor</a>);
        <b>let</b> parent_derived = *table::borrow(&account.agent_ids, parent_id);
        <b>let</b> parent_entry = table::borrow(&account.agents, parent_derived);
        <a href="../social_contracts/memory.md#social_contracts_memory_assert_registry_entry_active">assert_registry_entry_active</a>(parent_entry, clock);
        current_parent = parent_entry.parent_object_id;
    };
}
</code></pre>



</details>

<a name="social_contracts_memory_resolve_delegated_registration_placement"></a>

## Function `resolve_delegated_registration_placement`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_delegated_registration_placement">resolve_delegated_registration_placement</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, register_relation: u8, capabilities: u64, delegatable_caps: u64, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (u8, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_delegated_registration_placement">resolve_delegated_registration_placement</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    parent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    register_relation: u8,
    capabilities: u64,
    delegatable_caps: u64,
    platform_scope: Option&lt;<b>address</b>&gt;,
    clock: &Clock,
): (u8, Option&lt;ID&gt;, <b>address</b>) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_agent_belongs_to_account">assert_agent_belongs_to_account</a>(account, parent);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_active">assert_sub_agent_active</a>(parent, clock);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_has_cap">has_cap</a>(parent.capabilities, <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AGENT_REGISTER">CAP_AGENT_REGISTER</a>), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentMissingCap">ESubAgentMissingCap</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_caps_subset">assert_caps_subset</a>(capabilities, parent.delegatable_caps);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_caps_subset">assert_caps_subset</a>(delegatable_caps, parent.delegatable_caps);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_scope_allowed_for_delegate">assert_scope_allowed_for_delegate</a>(parent.platform_scope, platform_scope);
    <b>let</b> sender = parent.derived_address;
    <b>if</b> (register_relation == <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_CHILD">REGISTER_CHILD</a>) {
        <b>assert</b>!(
            parent.register_scope == <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_SCOPE_CHILD">REGISTER_SCOPE_CHILD</a>
                || parent.register_scope == <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_SCOPE_BOTH">REGISTER_SCOPE_BOTH</a>,
            <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidRegisterRelation">EInvalidRegisterRelation</a>,
        );
        <b>let</b> depth = parent.depth + 1;
        <b>assert</b>!(depth &lt;= <a href="../social_contracts/memory.md#social_contracts_memory_MAX_AGENT_DEPTH">MAX_AGENT_DEPTH</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EAgentDepthExceeded">EAgentDepthExceeded</a>);
        (depth, option::some(object::id(parent)), sender)
    } <b>else</b> <b>if</b> (register_relation == <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_PEER">REGISTER_PEER</a>) {
        <b>assert</b>!(
            parent.register_scope == <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_SCOPE_PEER">REGISTER_SCOPE_PEER</a>
                || parent.register_scope == <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_SCOPE_BOTH">REGISTER_SCOPE_BOTH</a>,
            <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidRegisterRelation">EInvalidRegisterRelation</a>,
        );
        (parent.depth, parent.parent_object_id, sender)
    } <b>else</b> {
        <b>abort</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidRegisterRelation">EInvalidRegisterRelation</a>
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_may_manage"></a>

## Function `assert_may_manage`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_may_manage">assert_may_manage</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, target: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, required_cap: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_may_manage">assert_may_manage</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    target: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    required_cap: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <b>let</b> sender = tx_context::sender(ctx);
    <b>if</b> (sender == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) {
        <b>return</b>
    };
    <b>assert</b>!(table::contains(&account.agents, sender), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotFound">ESubAgentNotFound</a>);
    <b>let</b> registrar_entry = table::borrow(&account.agents, sender);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_registry_entry_active">assert_registry_entry_active</a>(registrar_entry, clock);
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_has_cap">has_cap</a>(registrar_entry.capabilities, required_cap), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentMissingCap">ESubAgentMissingCap</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_registrar_is_ancestor_from_table">assert_registrar_is_ancestor_from_table</a>(account, sender, target);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_registrar_is_ancestor_from_table"></a>

## Function `assert_registrar_is_ancestor_from_table`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_registrar_is_ancestor_from_table">assert_registrar_is_ancestor_from_table</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, registrar_derived: <b>address</b>, target: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_registrar_is_ancestor_from_table">assert_registrar_is_ancestor_from_table</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    registrar_derived: <b>address</b>,
    target: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
) {
    <b>if</b> (registrar_derived == target.derived_address) {
        <b>return</b>
    };
    <b>assert</b>!(table::contains(&account.agents, target.derived_address), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentNotFound">ESubAgentNotFound</a>);
    <b>let</b> <b>mut</b> current_parent = target.parent_object_id;
    <b>while</b> (option::is_some(&current_parent)) {
        <b>let</b> parent_id = *option::borrow(&current_parent);
        <b>assert</b>!(table::contains(&account.agent_ids, parent_id), <a href="../social_contracts/memory.md#social_contracts_memory_ENotRegistrarAncestor">ENotRegistrarAncestor</a>);
        <b>let</b> parent_derived = *table::borrow(&account.agent_ids, parent_id);
        <b>if</b> (parent_derived == registrar_derived) {
            <b>return</b>
        };
        <b>let</b> parent_entry = table::borrow(&account.agents, parent_derived);
        current_parent = parent_entry.parent_object_id;
    };
    <b>abort</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENotRegistrarAncestor">ENotRegistrarAncestor</a>
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_update_caps_monotonic"></a>

## Function `assert_update_caps_monotonic`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_update_caps_monotonic">assert_update_caps_monotonic</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, new_capabilities: u64, new_delegatable_caps: u64, new_platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_update_caps_monotonic">assert_update_caps_monotonic</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    new_capabilities: u64,
    new_delegatable_caps: u64,
    new_platform_scope: Option&lt;<b>address</b>&gt;,
    ctx: &TxContext,
) {
    <b>if</b> (tx_context::sender(ctx) == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) {
        <b>return</b>
    };
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_caps_subset">assert_caps_subset</a>(new_capabilities, agent.capabilities);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_caps_subset">assert_caps_subset</a>(new_delegatable_caps, agent.delegatable_caps);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_scope_allowed_for_delegate">assert_scope_allowed_for_delegate</a>(agent.platform_scope, new_platform_scope);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_agent_belongs_to_account"></a>

## Function `assert_agent_belongs_to_account`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_agent_belongs_to_account">assert_agent_belongs_to_account</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_agent_belongs_to_account">assert_agent_belongs_to_account</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>) {
    <b>assert</b>!(agent.memory_account_id == object::id(account), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentAccountMismatch">ESubAgentAccountMismatch</a>);
    <b>assert</b>!(agent.principal_owner == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentAccountMismatch">ESubAgentAccountMismatch</a>);
    <b>assert</b>!(agent.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a> == account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentAccountMismatch">ESubAgentAccountMismatch</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_destroy_sub_agent"></a>

## Function `destroy_sub_agent`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_destroy_sub_agent">destroy_sub_agent</a>(agent: <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_destroy_sub_agent">destroy_sub_agent</a>(agent: <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>) {
    <b>let</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a> { id, .. } = agent;
    object::delete(id);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_caps_subset"></a>

## Function `assert_caps_subset`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_caps_subset">assert_caps_subset</a>(candidate: u64, allowed: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_caps_subset">assert_caps_subset</a>(candidate: u64, allowed: u64) {
    <b>assert</b>!((candidate & allowed) == candidate, <a href="../social_contracts/memory.md#social_contracts_memory_ECapsNotSubset">ECapsNotSubset</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_scope_allowed_for_delegate"></a>

## Function `assert_scope_allowed_for_delegate`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_scope_allowed_for_delegate">assert_scope_allowed_for_delegate</a>(parent_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, child_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_scope_allowed_for_delegate">assert_scope_allowed_for_delegate</a>(
    parent_scope: Option&lt;<b>address</b>&gt;,
    child_scope: Option&lt;<b>address</b>&gt;,
) {
    <b>if</b> (option::is_none(&parent_scope)) {
        <b>return</b>
    };
    <b>assert</b>!(option::is_some(&child_scope), <a href="../social_contracts/memory.md#social_contracts_memory_EScopeWidening">EScopeWidening</a>);
    <b>assert</b>!(
        *option::borrow(&parent_scope) == *option::borrow(&child_scope),
        <a href="../social_contracts/memory.md#social_contracts_memory_EScopeWidening">EScopeWidening</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_valid_register_scope"></a>

## Function `assert_valid_register_scope`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_register_scope">assert_valid_register_scope</a>(register_scope: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_register_scope">assert_valid_register_scope</a>(register_scope: u8) {
    <b>assert</b>!(
        register_scope == <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_SCOPE_CHILD">REGISTER_SCOPE_CHILD</a>
            || register_scope == <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_SCOPE_PEER">REGISTER_SCOPE_PEER</a>
            || register_scope == <a href="../social_contracts/memory.md#social_contracts_memory_REGISTER_SCOPE_BOTH">REGISTER_SCOPE_BOTH</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidRegisterScope">EInvalidRegisterScope</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_sub_agent_not_expired"></a>

## Function `assert_sub_agent_not_expired`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_not_expired">assert_sub_agent_not_expired</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_sub_agent_not_expired">assert_sub_agent_not_expired</a>(agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>, clock: &Clock) {
    <b>if</b> (option::is_none(&agent.expires_at)) {
        <b>return</b>
    };
    <b>assert</b>!(clock::timestamp_ms(clock) &lt;= *option::borrow(&agent.expires_at), <a href="../social_contracts/memory.md#social_contracts_memory_ESubAgentExpired">ESubAgentExpired</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_valid_identity_class"></a>

## Function `assert_valid_identity_class`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_identity_class">assert_valid_identity_class</a>(identity_class: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_identity_class">assert_valid_identity_class</a>(identity_class: u8) {
    <b>assert</b>!(
        identity_class == <a href="../social_contracts/memory.md#social_contracts_memory_CLASS_HUMAN">CLASS_HUMAN</a>
            || identity_class == <a href="../social_contracts/memory.md#social_contracts_memory_CLASS_DELEGATED_AI">CLASS_DELEGATED_AI</a>
            || identity_class == <a href="../social_contracts/memory.md#social_contracts_memory_CLASS_ORGANIZATION">CLASS_ORGANIZATION</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidIdentityClass">EInvalidIdentityClass</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_org_name_within_limit"></a>

## Function `assert_org_name_within_limit`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_name_within_limit">assert_org_name_within_limit</a>(name: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_name_within_limit">assert_org_name_within_limit</a>(name: &Option&lt;String&gt;) {
    <b>if</b> (option::is_some(name)) {
        <b>assert</b>!(
            string::length(option::borrow(name)) &lt;= <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORG_NAME_LENGTH">MAX_ORG_NAME_LENGTH</a>,
            <a href="../social_contracts/memory.md#social_contracts_memory_ENameTooLong">ENameTooLong</a>,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_org_description_within_limit"></a>

## Function `assert_org_description_within_limit`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_description_within_limit">assert_org_description_within_limit</a>(description: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_description_within_limit">assert_org_description_within_limit</a>(description: &Option&lt;String&gt;) {
    <b>if</b> (option::is_some(description)) {
        <b>assert</b>!(
            string::length(option::borrow(description)) &lt;= <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORG_DESCRIPTION_LENGTH">MAX_ORG_DESCRIPTION_LENGTH</a>,
            <a href="../social_contracts/memory.md#social_contracts_memory_EDescriptionTooLong">EDescriptionTooLong</a>,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_valid_org_type"></a>

## Function `assert_valid_org_type`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_type">assert_valid_org_type</a>(org_type: u8)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_type">assert_valid_org_type</a>(org_type: u8) {
    <b>assert</b>!(org_type &lt; <a href="../social_contracts/memory.md#social_contracts_memory_ORG_TYPE_COUNT">ORG_TYPE_COUNT</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidOrgType">EInvalidOrgType</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_organization_belongs_to_account"></a>

## Function `assert_organization_belongs_to_account`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
) {
    <b>assert</b>!(org.memory_account_id == object::id(account), <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationAccountMismatch">EOrganizationAccountMismatch</a>);
    <b>assert</b>!(org.principal_owner == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationAccountMismatch">EOrganizationAccountMismatch</a>);
    <b>assert</b>!(org.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a> == account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationAccountMismatch">EOrganizationAccountMismatch</a>);
    <b>assert</b>!(
        table::contains(&account.organizations, object::id(org)),
        <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotFound">EOrganizationNotFound</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_organization_ready_for_root"></a>

## Function `assert_organization_ready_for_root`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_ready_for_root">assert_organization_ready_for_root</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_ready_for_root">assert_organization_ready_for_root</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <b>assert</b>!(option::is_none(&org.root_agent_id), <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationHasRoot">EOrganizationHasRoot</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_has_cap"></a>

## Function `has_cap`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_cap">has_cap</a>(capabilities: u64, required_cap: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_cap">has_cap</a>(capabilities: u64, required_cap: u64): bool {
    (capabilities & required_cap) == required_cap
}
</code></pre>



</details>

<a name="social_contracts_memory_cap_requires_approval"></a>

## Function `cap_requires_approval`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_requires_approval">cap_requires_approval</a>(constraints: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgentConstraints">social_contracts::memory::SubAgentConstraints</a>, cap: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_requires_approval">cap_requires_approval</a>(constraints: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgentConstraints">SubAgentConstraints</a>, cap: u64): bool {
    (constraints.approval_required_caps & cap) == cap
}
</code></pre>



</details>

<a name="social_contracts_memory_emit_sub_agent_registered"></a>

## Function `emit_sub_agent_registered`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_registered">emit_sub_agent_registered</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_registered">emit_sub_agent_registered</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>) {
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_SubAgentRegistered">SubAgentRegistered</a> {
        account_id: object::id(account),
        principal_owner: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: agent.<a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: object::id(agent),
        derived_address: agent.derived_address,
        label: agent.label,
        identity_class: agent.identity_class,
        role_tags: agent.role_tags,
        capabilities: agent.capabilities,
        delegatable_caps: agent.delegatable_caps,
        register_scope: agent.register_scope,
        approval_required_caps: agent.constraints.approval_required_caps,
        max_action_spend: agent.constraints.max_action_spend,
        platform_scope: agent.platform_scope,
        parent_object_id: agent.parent_object_id,
        depth: agent.depth,
        registered_by: agent.registered_by,
        expires_at: agent.expires_at,
        active: agent.active,
        created_at: agent.created_at,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_emit_sub_agent_updated"></a>

## Function `emit_sub_agent_updated`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_updated">emit_sub_agent_updated</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_updated">emit_sub_agent_updated</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>) {
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_SubAgentUpdated">SubAgentUpdated</a> {
        account_id: object::id(account),
        principal_owner: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: agent.<a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: object::id(agent),
        derived_address: agent.derived_address,
        label: agent.label,
        identity_class: agent.identity_class,
        role_tags: agent.role_tags,
        capabilities: agent.capabilities,
        delegatable_caps: agent.delegatable_caps,
        register_scope: agent.register_scope,
        approval_required_caps: agent.constraints.approval_required_caps,
        max_action_spend: agent.constraints.max_action_spend,
        platform_scope: agent.platform_scope,
        parent_object_id: agent.parent_object_id,
        depth: agent.depth,
        registered_by: agent.registered_by,
        expires_at: agent.expires_at,
        active: agent.active,
        created_at: agent.created_at,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_emit_sub_agent_deactivated"></a>

## Function `emit_sub_agent_deactivated`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_deactivated">emit_sub_agent_deactivated</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_deactivated">emit_sub_agent_deactivated</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>) {
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_SubAgentDeactivated">SubAgentDeactivated</a> {
        account_id: object::id(account),
        principal_owner: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: object::id(agent),
        derived_address: agent.derived_address,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_emit_sub_agent_revoked"></a>

## Function `emit_sub_agent_revoked`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_revoked">emit_sub_agent_revoked</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_emit_sub_agent_revoked">emit_sub_agent_revoked</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>, agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>) {
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_SubAgentRevoked">SubAgentRevoked</a> {
        account_id: object::id(account),
        principal_owner: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>: account.<a href="../social_contracts/memory.md#social_contracts_memory_profile_id">profile_id</a>,
        <a href="../social_contracts/memory.md#social_contracts_memory_agent_object_id">agent_object_id</a>: object::id(agent),
        derived_address: agent.derived_address,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_get_version"></a>

## Function `get_version`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(uid: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(uid: &UID): u64 {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, u64&gt;(uid, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>)) {
        *df::borrow&lt;vector&lt;u8&gt;, u64&gt;(uid, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>)
    } <b>else</b> {
        1
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_set_version"></a>

## Function `set_version`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, v: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(uid: &<b>mut</b> UID, v: u64) {
    <b>if</b> (df::exists_with_type&lt;vector&lt;u8&gt;, u64&gt;(uid, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>)) {
        <b>let</b> r = df::borrow_mut&lt;vector&lt;u8&gt;, u64&gt;(uid, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>);
        *r = v;
    } <b>else</b> {
        df::add(uid, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION_DF_KEY">VERSION_DF_KEY</a>, v);
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_bump_version"></a>

## Function `bump_version`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bump_version">bump_version</a>(uid: &<b>mut</b> <a href="../myso/object.md#myso_object_UID">myso::object::UID</a>, v: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bump_version">bump_version</a>(uid: &<b>mut</b> UID, v: u64) {
    <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(uid, v)
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_object_version"></a>

## Function `assert_object_version`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(uid: &<a href="../myso/object.md#myso_object_UID">myso::object::UID</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(uid: &UID) {
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_get_version">get_version</a>(uid) == <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EWrongVersion">EWrongVersion</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_cap_for_this_package"></a>

## Function `assert_cap_for_this_package`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_cap_for_this_package">assert_cap_for_this_package</a>(cap: &<a href="../myso/package.md#myso_package_UpgradeCap">myso::package::UpgradeCap</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_cap_for_this_package">assert_cap_for_this_package</a>(cap: &UpgradeCap) {
    <b>let</b> cap_pkg = package::upgrade_package(cap);
    <b>assert</b>!(object::id_to_address(&cap_pkg) == @social_contracts, <a href="../social_contracts/memory.md#social_contracts_memory_ENotUpgradeAuthority">ENotUpgradeAuthority</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_has_suffix"></a>

## Function `has_suffix`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_suffix">has_suffix</a>(data: &vector&lt;u8&gt;, suffix: &vector&lt;u8&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_suffix">has_suffix</a>(data: &vector&lt;u8&gt;, suffix: &vector&lt;u8&gt;): bool {
    <b>let</b> data_len = vector::length(data);
    <b>let</b> suffix_len = vector::length(suffix);
    <b>if</b> (suffix_len &gt; data_len) <b>return</b> <b>false</b>;
    <b>let</b> offset = data_len - suffix_len;
    <b>let</b> <b>mut</b> i = 0;
    <b>while</b> (i &lt; suffix_len) {
        <b>if</b> (*vector::borrow(data, offset + i) != *vector::borrow(suffix, i)) <b>return</b> <b>false</b>;
        i = i + 1;
    };
    <b>true</b>
}
</code></pre>



</details>
