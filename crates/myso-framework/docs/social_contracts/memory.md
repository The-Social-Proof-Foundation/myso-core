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

**Social events** (post module): all include <code>actor_address</code>, <code>sub_agent_id</code> (agent object id),
<code>action_identity_class</code> and reactions add <code>principal_owner</code>.


    -  [Indexer event payloads](#@Indexer_event_payloads_0)
-  [Struct `MemoryAdminCap`](#social_contracts_memory_MemoryAdminCap)
-  [Struct `MemoryConfig`](#social_contracts_memory_MemoryConfig)
-  [Struct `MemoryConfigUpdatedEvent`](#social_contracts_memory_MemoryConfigUpdatedEvent)
-  [Struct `MemoryRegistry`](#social_contracts_memory_MemoryRegistry)
-  [Struct `SubAgentKey`](#social_contracts_memory_SubAgentKey)
-  [Struct `AgentMemoryVaultKey`](#social_contracts_memory_AgentMemoryVaultKey)
-  [Struct `MemorySharePackage`](#social_contracts_memory_MemorySharePackage)
-  [Struct `OrgMemoryGroupTag`](#social_contracts_memory_OrgMemoryGroupTag)
-  [Struct `OrgMemoryReader`](#social_contracts_memory_OrgMemoryReader)
-  [Struct `OrgMemoryWriter`](#social_contracts_memory_OrgMemoryWriter)
-  [Struct `OrgAgentManager`](#social_contracts_memory_OrgAgentManager)
-  [Struct `OrgBudgetManager`](#social_contracts_memory_OrgBudgetManager)
-  [Struct `OrgSpendApprover`](#social_contracts_memory_OrgSpendApprover)
-  [Struct `OrgDashboardViewer`](#social_contracts_memory_OrgDashboardViewer)
-  [Struct `OrgAuditor`](#social_contracts_memory_OrgAuditor)
-  [Struct `OrgGovernanceProposer`](#social_contracts_memory_OrgGovernanceProposer)
-  [Struct `OrgGovernanceVoter`](#social_contracts_memory_OrgGovernanceVoter)
-  [Struct `OrgInvitationKey`](#social_contracts_memory_OrgInvitationKey)
-  [Struct `OrgInvitation`](#social_contracts_memory_OrgInvitation)
-  [Struct `OrgCustomRoleKey`](#social_contracts_memory_OrgCustomRoleKey)
-  [Struct `OrgRoleAssignmentKey`](#social_contracts_memory_OrgRoleAssignmentKey)
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
-  [Struct `MemoryAccountDeactivated`](#social_contracts_memory_MemoryAccountDeactivated)
-  [Struct `MemoryAccountReactivated`](#social_contracts_memory_MemoryAccountReactivated)
-  [Struct `MemoryAccountMigrated`](#social_contracts_memory_MemoryAccountMigrated)
-  [Struct `MemoryRegistryMigrated`](#social_contracts_memory_MemoryRegistryMigrated)
-  [Struct `AgentMemoryVaultCreated`](#social_contracts_memory_AgentMemoryVaultCreated)
-  [Struct `AgenticOrganizationCreated`](#social_contracts_memory_AgenticOrganizationCreated)
-  [Struct `AgenticOrganizationUpdated`](#social_contracts_memory_AgenticOrganizationUpdated)
-  [Struct `AgenticOrganizationCategoryUpdated`](#social_contracts_memory_AgenticOrganizationCategoryUpdated)
-  [Struct `AgenticOrganizationDeactivated`](#social_contracts_memory_AgenticOrganizationDeactivated)
-  [Struct `OrgMemoryGroupCreated`](#social_contracts_memory_OrgMemoryGroupCreated)
-  [Struct `OrgMemoryPermissionGranted`](#social_contracts_memory_OrgMemoryPermissionGranted)
-  [Struct `OrgMemoryPermissionRevoked`](#social_contracts_memory_OrgMemoryPermissionRevoked)
-  [Struct `OrgRoleDefined`](#social_contracts_memory_OrgRoleDefined)
-  [Struct `OrgRoleAssigned`](#social_contracts_memory_OrgRoleAssigned)
-  [Struct `OrgRoleRevoked`](#social_contracts_memory_OrgRoleRevoked)
-  [Struct `OrgInvitationCreated`](#social_contracts_memory_OrgInvitationCreated)
-  [Struct `OrgInvitationAccepted`](#social_contracts_memory_OrgInvitationAccepted)
-  [Struct `OrgInvitationDeclined`](#social_contracts_memory_OrgInvitationDeclined)
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
-  [Function `cap_ai_spend`](#social_contracts_memory_cap_ai_spend)
-  [Function `cap_budget_manage`](#social_contracts_memory_cap_budget_manage)
-  [Function `org_perm_memory_read`](#social_contracts_memory_org_perm_memory_read)
-  [Function `org_perm_memory_write`](#social_contracts_memory_org_perm_memory_write)
-  [Function `org_perm_agent_manager`](#social_contracts_memory_org_perm_agent_manager)
-  [Function `org_perm_budget_manager`](#social_contracts_memory_org_perm_budget_manager)
-  [Function `org_perm_spend_approver`](#social_contracts_memory_org_perm_spend_approver)
-  [Function `org_perm_dashboard_viewer`](#social_contracts_memory_org_perm_dashboard_viewer)
-  [Function `org_perm_auditor`](#social_contracts_memory_org_perm_auditor)
-  [Function `org_perm_all`](#social_contracts_memory_org_perm_all)
-  [Function `org_perm_governance_proposer`](#social_contracts_memory_org_perm_governance_proposer)
-  [Function `org_perm_governance_voter`](#social_contracts_memory_org_perm_governance_voter)
-  [Function `org_governance_perm_all`](#social_contracts_memory_org_governance_perm_all)
-  [Function `role_mask_owner`](#social_contracts_memory_role_mask_owner)
-  [Function `role_mask_admin`](#social_contracts_memory_role_mask_admin)
-  [Function `role_mask_agent_manager`](#social_contracts_memory_role_mask_agent_manager)
-  [Function `role_mask_finance_approver`](#social_contracts_memory_role_mask_finance_approver)
-  [Function `role_mask_memory_administrator`](#social_contracts_memory_role_mask_memory_administrator)
-  [Function `role_mask_auditor`](#social_contracts_memory_role_mask_auditor)
-  [Function `register_child`](#social_contracts_memory_register_child)
-  [Function `register_peer`](#social_contracts_memory_register_peer)
-  [Function `derive_sub_agent_address`](#social_contracts_memory_derive_sub_agent_address)
-  [Function `agent_object_id`](#social_contracts_memory_agent_object_id)
-  [Function `organization_id`](#social_contracts_memory_organization_id)
-  [Function `organization_memory_account_id`](#social_contracts_memory_organization_memory_account_id)
-  [Function `organization_active`](#social_contracts_memory_organization_active)
-  [Function `sub_agent_organization_id`](#social_contracts_memory_sub_agent_organization_id)
-  [Function `organization_org_type`](#social_contracts_memory_organization_org_type)
-  [Function `organization_root_agent_id`](#social_contracts_memory_organization_root_agent_id)
-  [Function `organization_name`](#social_contracts_memory_organization_name)
-  [Function `organization_description`](#social_contracts_memory_organization_description)
-  [Function `bootstrap_init`](#social_contracts_memory_bootstrap_init)
-  [Function `create_memory_admin_cap`](#social_contracts_memory_create_memory_admin_cap)
-  [Function `update_memory_config`](#social_contracts_memory_update_memory_config)
-  [Function `create_account_for_profile`](#social_contracts_memory_create_account_for_profile)
-  [Function `create_agentic_organization`](#social_contracts_memory_create_agentic_organization)
-  [Function `create_agentic_organization_internal`](#social_contracts_memory_create_agentic_organization_internal)
-  [Function `update_agentic_organization_metadata`](#social_contracts_memory_update_agentic_organization_metadata)
-  [Function `update_agentic_organization_category`](#social_contracts_memory_update_agentic_organization_category)
-  [Function `deactivate_agentic_organization`](#social_contracts_memory_deactivate_agentic_organization)
-  [Function `ensure_org_memory_group`](#social_contracts_memory_ensure_org_memory_group)
-  [Function `org_memory_group_address`](#social_contracts_memory_org_memory_group_address)
-  [Function `org_memory_group_exists`](#social_contracts_memory_org_memory_group_exists)
-  [Function `assert_org_permission`](#social_contracts_memory_assert_org_permission)
-  [Function `has_org_permission`](#social_contracts_memory_has_org_permission)
-  [Function `grant_org_memory_permission`](#social_contracts_memory_grant_org_memory_permission)
-  [Function `revoke_org_memory_permission`](#social_contracts_memory_revoke_org_memory_permission)
-  [Function `define_custom_org_role`](#social_contracts_memory_define_custom_org_role)
-  [Function `assign_org_role`](#social_contracts_memory_assign_org_role)
-  [Function `revoke_org_role`](#social_contracts_memory_revoke_org_role)
-  [Function `org_role_assignment_mask`](#social_contracts_memory_org_role_assignment_mask)
-  [Function `create_org_invitation`](#social_contracts_memory_create_org_invitation)
-  [Function `accept_org_invitation`](#social_contracts_memory_accept_org_invitation)
-  [Function `decline_org_invitation`](#social_contracts_memory_decline_org_invitation)
-  [Function `org_role_mask`](#social_contracts_memory_org_role_mask)
-  [Function `approve_org_key_policy`](#social_contracts_memory_approve_org_key_policy)
-  [Function `register_sub_agent`](#social_contracts_memory_register_sub_agent)
-  [Function `register_sub_agent_delegated`](#social_contracts_memory_register_sub_agent_delegated)
-  [Function `update_sub_agent`](#social_contracts_memory_update_sub_agent)
-  [Function `update_sub_agent_label`](#social_contracts_memory_update_sub_agent_label)
-  [Function `deactivate_sub_agent`](#social_contracts_memory_deactivate_sub_agent)
-  [Function `revoke_sub_agent`](#social_contracts_memory_revoke_sub_agent)
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
-  [Function `is_descendant_agent`](#social_contracts_memory_is_descendant_agent)
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
-  [Function `is_org_group`](#social_contracts_memory_is_org_group)
-  [Function `assert_org_group`](#social_contracts_memory_assert_org_group)
-  [Function `assert_org_permission_manager`](#social_contracts_memory_assert_org_permission_manager)
-  [Function `assert_valid_org_permission_mask`](#social_contracts_memory_assert_valid_org_permission_mask)
-  [Function `assert_valid_org_ops_permission_mask`](#social_contracts_memory_assert_valid_org_ops_permission_mask)
-  [Function `assert_valid_org_governance_permission_mask`](#social_contracts_memory_assert_valid_org_governance_permission_mask)
-  [Function `assert_member_grantable`](#social_contracts_memory_assert_member_grantable)
-  [Function `grant_org_permissions_from_mask_via_org`](#social_contracts_memory_grant_org_permissions_from_mask_via_org)
-  [Function `ensure_org_group_delegate_admin`](#social_contracts_memory_ensure_org_group_delegate_admin)
-  [Function `grant_org_permissions_from_mask`](#social_contracts_memory_grant_org_permissions_from_mask)
-  [Function `revoke_org_permissions_from_mask`](#social_contracts_memory_revoke_org_permissions_from_mask)
-  [Function `is_builtin_role_name`](#social_contracts_memory_is_builtin_role_name)
-  [Function `builtin_role_mask`](#social_contracts_memory_builtin_role_mask)
-  [Function `resolve_role_mask`](#social_contracts_memory_resolve_role_mask)
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
<b>use</b> <a href="../myso/permissioned_group.md#myso_permissioned_group">myso::permissioned_group</a>;
<b>use</b> <a href="../myso/permissions_table.md#myso_permissions_table">myso::permissions_table</a>;
<b>use</b> <a href="../myso/table.md#myso_table">myso::table</a>;
<b>use</b> <a href="../myso/transfer.md#myso_transfer">myso::transfer</a>;
<b>use</b> <a href="../myso/tx_context.md#myso_tx_context">myso::tx_context</a>;
<b>use</b> <a href="../myso/types.md#myso_types">myso::types</a>;
<b>use</b> <a href="../myso/unpause_cap.md#myso_unpause_cap">myso::unpause_cap</a>;
<b>use</b> <a href="../myso/vec_map.md#myso_vec_map">myso::vec_map</a>;
<b>use</b> <a href="../myso/vec_set.md#myso_vec_set">myso::vec_set</a>;
<b>use</b> <a href="../std/address.md#std_address">std::address</a>;
<b>use</b> <a href="../std/ascii.md#std_ascii">std::ascii</a>;
<b>use</b> <a href="../std/bcs.md#std_bcs">std::bcs</a>;
<b>use</b> <a href="../std/option.md#std_option">std::option</a>;
<b>use</b> <a href="../std/string.md#std_string">std::string</a>;
<b>use</b> <a href="../std/type_name.md#std_type_name">std::type_name</a>;
<b>use</b> <a href="../std/vector.md#std_vector">std::vector</a>;
</code></pre>



<a name="social_contracts_memory_MemoryAdminCap"></a>

## Struct `MemoryAdminCap`

Admin capability for memory configuration


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAdminCap">MemoryAdminCap</a> <b>has</b> key, store
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

<a name="social_contracts_memory_MemoryConfig"></a>

## Struct `MemoryConfig`

Global memory feature configuration


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a> <b>has</b> key
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
<code><a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>org_category_update_cooldown_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_agent_depth: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>max_label_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_org_name_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_org_description_length: u64</code>
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

<a name="social_contracts_memory_MemoryConfigUpdatedEvent"></a>

## Struct `MemoryConfigUpdatedEvent`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfigUpdatedEvent">MemoryConfigUpdatedEvent</a> <b>has</b> <b>copy</b>, drop
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
<code><a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>org_category_update_cooldown_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_agent_depth: u8</code>
</dt>
<dd>
</dd>
<dt>
<code>max_label_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_org_name_length: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>max_org_description_length: u64</code>
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

<a name="social_contracts_memory_MemorySharePackage"></a>

## Struct `MemorySharePackage`

Package witness for <code>PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;</code> (org memory share groups).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgMemoryGroupTag"></a>

## Struct `OrgMemoryGroupTag`

Derivation key for the per-organization memory share group (derived from the org UID).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryGroupTag">OrgMemoryGroupTag</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgMemoryReader"></a>

## Struct `OrgMemoryReader`

Permission to read org-visible shared memory (relayer recall scope + MYDATA key release).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryReader">OrgMemoryReader</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgMemoryWriter"></a>

## Struct `OrgMemoryWriter`

Permission to write org-visible shared memory.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryWriter">OrgMemoryWriter</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgAgentManager"></a>

## Struct `OrgAgentManager`

Permission to manage the org's agent fleet (dashboard + future scheduler surfaces).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgAgentManager">OrgAgentManager</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgBudgetManager"></a>

## Struct `OrgBudgetManager`

Permission to manage AI-credit budgets for the org's agents (<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_set_agent_budget_as_manager">ai_credit::set_agent_budget_as_manager</a></code>).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgBudgetManager">OrgBudgetManager</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgSpendApprover"></a>

## Struct `OrgSpendApprover`

Permission to approve over-threshold AI-credit spends (<code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit_approve_agent_spend_as_approver">ai_credit::approve_agent_spend_as_approver</a></code>).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgSpendApprover">OrgSpendApprover</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgDashboardViewer"></a>

## Struct `OrgDashboardViewer`

Permission to view org dashboards (recorded on-chain; server-side read gating is a later phase).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgDashboardViewer">OrgDashboardViewer</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgAuditor"></a>

## Struct `OrgAuditor`

Permission to read org audit logs (recorded on-chain; server-side read gating is a later phase).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgAuditor">OrgAuditor</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgGovernanceProposer"></a>

## Struct `OrgGovernanceProposer`

Permission to create governance proposals for the organization.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceProposer">OrgGovernanceProposer</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgGovernanceVoter"></a>

## Struct `OrgGovernanceVoter`

Permission to vote on governance proposals for the organization.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceVoter">OrgGovernanceVoter</a> <b>has</b> drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
</dl>


</details>

<a name="social_contracts_memory_OrgInvitationKey"></a>

## Struct `OrgInvitationKey`

Dynamic-field key on the org UID for a pending invitation to <code>invitee</code>.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationKey">OrgInvitationKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>invitee: <b>address</b></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgInvitation"></a>

## Struct `OrgInvitation`

Pending org membership invitation stored on the org UID.


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitation">OrgInvitation</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>invitee: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>role_name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>permissions_mask: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>invited_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>created_at_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at_ms: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgCustomRoleKey"></a>

## Struct `OrgCustomRoleKey`

Dynamic-field key on the org UID for a custom role definition (<code>name -&gt; mask</code>).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgCustomRoleKey">OrgCustomRoleKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgRoleAssignmentKey"></a>

## Struct `OrgRoleAssignmentKey`

Dynamic-field key on the org UID recording a role assignment's exact granted delta,
so revocation removes precisely what the assignment added (immune to role redefinition
and to overlap with direct grants or other roles).


<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssignmentKey">OrgRoleAssignmentKey</a> <b>has</b> <b>copy</b>, drop, store
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>role_name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
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

<a name="social_contracts_memory_OrgMemoryGroupCreated"></a>

## Struct `OrgMemoryGroupCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryGroupCreated">OrgMemoryGroupCreated</a> <b>has</b> <b>copy</b>, drop
</code></pre>



<details>
<summary>Fields</summary>


<dl>
<dt>
<code>group_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
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
<code>created_at: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgMemoryPermissionGranted"></a>

## Struct `OrgMemoryPermissionGranted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryPermissionGranted">OrgMemoryPermissionGranted</a> <b>has</b> <b>copy</b>, drop
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
<code>group_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>permissions_mask: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>granted_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgMemoryPermissionRevoked"></a>

## Struct `OrgMemoryPermissionRevoked`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryPermissionRevoked">OrgMemoryPermissionRevoked</a> <b>has</b> <b>copy</b>, drop
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
<code>group_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>permissions_mask: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>revoked_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgRoleDefined"></a>

## Struct `OrgRoleDefined`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleDefined">OrgRoleDefined</a> <b>has</b> <b>copy</b>, drop
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
<code>role_name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>mask: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>previous_mask: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>defined_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgRoleAssigned"></a>

## Struct `OrgRoleAssigned`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssigned">OrgRoleAssigned</a> <b>has</b> <b>copy</b>, drop
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
<code>group_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>role_name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>mask: u64</code>
</dt>
<dd>
 Full mask of the role at assignment time.
</dd>
<dt>
<code>granted_mask: u64</code>
</dt>
<dd>
 Delta actually granted (excludes permissions the member already held).
</dd>
<dt>
<code>assigned_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgRoleRevoked"></a>

## Struct `OrgRoleRevoked`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleRevoked">OrgRoleRevoked</a> <b>has</b> <b>copy</b>, drop
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
<code>group_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>member: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>role_name: <a href="../std/string.md#std_string_String">std::string::String</a></code>
</dt>
<dd>
</dd>
<dt>
<code>revoked_mask: u64</code>
</dt>
<dd>
 Delta actually revoked (the assignment's recorded granted_mask).
</dd>
<dt>
<code>revoked_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgInvitationCreated"></a>

## Struct `OrgInvitationCreated`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationCreated">OrgInvitationCreated</a> <b>has</b> <b>copy</b>, drop
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
<code>invitee: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>role_name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>permissions_mask: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>invited_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>expires_at_ms: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgInvitationAccepted"></a>

## Struct `OrgInvitationAccepted`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationAccepted">OrgInvitationAccepted</a> <b>has</b> <b>copy</b>, drop
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
<code>group_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a></code>
</dt>
<dd>
</dd>
<dt>
<code>invitee: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>role_name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;</code>
</dt>
<dd>
</dd>
<dt>
<code>permissions_mask: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>granted_mask: u64</code>
</dt>
<dd>
</dd>
<dt>
<code>accepted_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
</dt>
<dd>
</dd>
</dl>


</details>

<a name="social_contracts_memory_OrgInvitationDeclined"></a>

## Struct `OrgInvitationDeclined`



<pre><code><b>public</b> <b>struct</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationDeclined">OrgInvitationDeclined</a> <b>has</b> <b>copy</b>, drop
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
<code>invitee: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>declined_by: <b>address</b></code>
</dt>
<dd>
</dd>
<dt>
<code>timestamp_ms: u64</code>
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



<a name="social_contracts_memory_CAP_AI_SPEND"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AI_SPEND">CAP_AI_SPEND</a>: u64 = 16384;
</code></pre>



<a name="social_contracts_memory_CAP_BUDGET_MANAGE"></a>

Parent agents holding this capability may manage AI-credit budgets and
spend allowances for descendants in their subtree (see <code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit">ai_credit</a></code>).


<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_CAP_BUDGET_MANAGE">CAP_BUDGET_MANAGE</a>: u64 = 32768;
</code></pre>



<a name="social_contracts_memory_ORG_PERM_MEMORY_READ"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_READ">ORG_PERM_MEMORY_READ</a>: u64 = 1;
</code></pre>



<a name="social_contracts_memory_ORG_PERM_MEMORY_WRITE"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_WRITE">ORG_PERM_MEMORY_WRITE</a>: u64 = 2;
</code></pre>



<a name="social_contracts_memory_ORG_PERM_AGENT_MANAGER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AGENT_MANAGER">ORG_PERM_AGENT_MANAGER</a>: u64 = 4;
</code></pre>



<a name="social_contracts_memory_ORG_PERM_BUDGET_MANAGER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_BUDGET_MANAGER">ORG_PERM_BUDGET_MANAGER</a>: u64 = 8;
</code></pre>



<a name="social_contracts_memory_ORG_PERM_SPEND_APPROVER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_SPEND_APPROVER">ORG_PERM_SPEND_APPROVER</a>: u64 = 16;
</code></pre>



<a name="social_contracts_memory_ORG_PERM_DASHBOARD_VIEWER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_DASHBOARD_VIEWER">ORG_PERM_DASHBOARD_VIEWER</a>: u64 = 32;
</code></pre>



<a name="social_contracts_memory_ORG_PERM_AUDITOR"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AUDITOR">ORG_PERM_AUDITOR</a>: u64 = 64;
</code></pre>



<a name="social_contracts_memory_ORG_PERM_ALL"></a>

Operational permission bits (memory, agents, budgets, dashboards, audit).


<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_ALL">ORG_PERM_ALL</a>: u64 = 127;
</code></pre>



<a name="social_contracts_memory_ORG_GOVERNANCE_PROPOSER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PROPOSER">ORG_GOVERNANCE_PROPOSER</a>: u64 = 128;
</code></pre>



<a name="social_contracts_memory_ORG_GOVERNANCE_VOTER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_VOTER">ORG_GOVERNANCE_VOTER</a>: u64 = 256;
</code></pre>



<a name="social_contracts_memory_ORG_GOVERNANCE_PERM_ALL"></a>

Governance permission bits (proposer + voter).


<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PERM_ALL">ORG_GOVERNANCE_PERM_ALL</a>: u64 = 384;
</code></pre>



<a name="social_contracts_memory_ROLE_MASK_OWNER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_OWNER">ROLE_MASK_OWNER</a>: u64 = 127;
</code></pre>



<a name="social_contracts_memory_ROLE_MASK_ADMIN"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_ADMIN">ROLE_MASK_ADMIN</a>: u64 = 111;
</code></pre>



<a name="social_contracts_memory_ROLE_MASK_AGENT_MANAGER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_AGENT_MANAGER">ROLE_MASK_AGENT_MANAGER</a>: u64 = 36;
</code></pre>



<a name="social_contracts_memory_ROLE_MASK_FINANCE_APPROVER"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_FINANCE_APPROVER">ROLE_MASK_FINANCE_APPROVER</a>: u64 = 24;
</code></pre>



<a name="social_contracts_memory_ROLE_MASK_MEMORY_ADMINISTRATOR"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_MEMORY_ADMINISTRATOR">ROLE_MASK_MEMORY_ADMINISTRATOR</a>: u64 = 3;
</code></pre>



<a name="social_contracts_memory_ROLE_MASK_AUDITOR"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_AUDITOR">ROLE_MASK_AUDITOR</a>: u64 = 96;
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



<a name="social_contracts_memory_EOrgGroupMismatch"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgGroupMismatch">EOrgGroupMismatch</a>: u64 = 46;
</code></pre>



<a name="social_contracts_memory_EInvalidOrgPermission"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidOrgPermission">EInvalidOrgPermission</a>: u64 = 47;
</code></pre>



<a name="social_contracts_memory_EOrgRoleNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleNotFound">EOrgRoleNotFound</a>: u64 = 49;
</code></pre>



<a name="social_contracts_memory_EOrgRoleAlreadyAssigned"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleAlreadyAssigned">EOrgRoleAlreadyAssigned</a>: u64 = 50;
</code></pre>



<a name="social_contracts_memory_EOrgRoleNotAssigned"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleNotAssigned">EOrgRoleNotAssigned</a>: u64 = 51;
</code></pre>



<a name="social_contracts_memory_EOrgRoleNameTooLong"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleNameTooLong">EOrgRoleNameTooLong</a>: u64 = 52;
</code></pre>



<a name="social_contracts_memory_EOrgRoleMaskEmpty"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleMaskEmpty">EOrgRoleMaskEmpty</a>: u64 = 53;
</code></pre>



<a name="social_contracts_memory_EOrgRoleBuiltinRedefine"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleBuiltinRedefine">EOrgRoleBuiltinRedefine</a>: u64 = 54;
</code></pre>



<a name="social_contracts_memory_ENotDescendantAgent"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENotDescendantAgent">ENotDescendantAgent</a>: u64 = 55;
</code></pre>



<a name="social_contracts_memory_EOrgInvitationExists"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationExists">EOrgInvitationExists</a>: u64 = 56;
</code></pre>



<a name="social_contracts_memory_EOrgInvitationNotFound"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationNotFound">EOrgInvitationNotFound</a>: u64 = 57;
</code></pre>



<a name="social_contracts_memory_EOrgInvitationNotInvitee"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationNotInvitee">EOrgInvitationNotInvitee</a>: u64 = 58;
</code></pre>



<a name="social_contracts_memory_EOrgInvitationExpired"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationExpired">EOrgInvitationExpired</a>: u64 = 59;
</code></pre>



<a name="social_contracts_memory_EOrgInvitationEmpty"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationEmpty">EOrgInvitationEmpty</a>: u64 = 60;
</code></pre>



<a name="social_contracts_memory_ENoAccess"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ENoAccess">ENoAccess</a>: u64 = 100;
</code></pre>



<a name="social_contracts_memory_EInvalidConfig"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidConfig">EInvalidConfig</a>: u64 = 101;
</code></pre>



<a name="social_contracts_memory_ED25519_PUBLIC_KEY_LENGTH"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ED25519_PUBLIC_KEY_LENGTH">ED25519_PUBLIC_KEY_LENGTH</a>: u64 = 32;
</code></pre>



<a name="social_contracts_memory_MAX_ORGANIZATIONS_PER_USER"></a>

Default bootstrap values for MemoryConfig


<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORGANIZATIONS_PER_USER">MAX_ORGANIZATIONS_PER_USER</a>: u8 = 8;
</code></pre>



<a name="social_contracts_memory_ORG_CATEGORY_UPDATE_COOLDOWN_MS"></a>



<pre><code><b>const</b> <a href="../social_contracts/memory.md#social_contracts_memory_ORG_CATEGORY_UPDATE_COOLDOWN_MS">ORG_CATEGORY_UPDATE_COOLDOWN_MS</a>: u64 = 604800000;
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



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>): u8
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>): u8 { config.<a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a> }
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

<a name="social_contracts_memory_cap_ai_spend"></a>

## Function `cap_ai_spend`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_ai_spend">cap_ai_spend</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_ai_spend">cap_ai_spend</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_AI_SPEND">CAP_AI_SPEND</a> }
</code></pre>



</details>

<a name="social_contracts_memory_cap_budget_manage"></a>

## Function `cap_budget_manage`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_budget_manage">cap_budget_manage</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_cap_budget_manage">cap_budget_manage</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_CAP_BUDGET_MANAGE">CAP_BUDGET_MANAGE</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_perm_memory_read"></a>

## Function `org_perm_memory_read`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_memory_read">org_perm_memory_read</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_memory_read">org_perm_memory_read</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_READ">ORG_PERM_MEMORY_READ</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_perm_memory_write"></a>

## Function `org_perm_memory_write`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_memory_write">org_perm_memory_write</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_memory_write">org_perm_memory_write</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_WRITE">ORG_PERM_MEMORY_WRITE</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_perm_agent_manager"></a>

## Function `org_perm_agent_manager`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_agent_manager">org_perm_agent_manager</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_agent_manager">org_perm_agent_manager</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AGENT_MANAGER">ORG_PERM_AGENT_MANAGER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_perm_budget_manager"></a>

## Function `org_perm_budget_manager`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_budget_manager">org_perm_budget_manager</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_budget_manager">org_perm_budget_manager</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_BUDGET_MANAGER">ORG_PERM_BUDGET_MANAGER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_perm_spend_approver"></a>

## Function `org_perm_spend_approver`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_spend_approver">org_perm_spend_approver</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_spend_approver">org_perm_spend_approver</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_SPEND_APPROVER">ORG_PERM_SPEND_APPROVER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_perm_dashboard_viewer"></a>

## Function `org_perm_dashboard_viewer`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_dashboard_viewer">org_perm_dashboard_viewer</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_dashboard_viewer">org_perm_dashboard_viewer</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_DASHBOARD_VIEWER">ORG_PERM_DASHBOARD_VIEWER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_perm_auditor"></a>

## Function `org_perm_auditor`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_auditor">org_perm_auditor</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_auditor">org_perm_auditor</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AUDITOR">ORG_PERM_AUDITOR</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_perm_all"></a>

## Function `org_perm_all`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_all">org_perm_all</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_all">org_perm_all</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_ALL">ORG_PERM_ALL</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_perm_governance_proposer"></a>

## Function `org_perm_governance_proposer`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_governance_proposer">org_perm_governance_proposer</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_governance_proposer">org_perm_governance_proposer</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PROPOSER">ORG_GOVERNANCE_PROPOSER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_perm_governance_voter"></a>

## Function `org_perm_governance_voter`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_governance_voter">org_perm_governance_voter</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_perm_governance_voter">org_perm_governance_voter</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_VOTER">ORG_GOVERNANCE_VOTER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_org_governance_perm_all"></a>

## Function `org_governance_perm_all`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_governance_perm_all">org_governance_perm_all</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_governance_perm_all">org_governance_perm_all</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PERM_ALL">ORG_GOVERNANCE_PERM_ALL</a> }
</code></pre>



</details>

<a name="social_contracts_memory_role_mask_owner"></a>

## Function `role_mask_owner`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_owner">role_mask_owner</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_owner">role_mask_owner</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_OWNER">ROLE_MASK_OWNER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_role_mask_admin"></a>

## Function `role_mask_admin`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_admin">role_mask_admin</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_admin">role_mask_admin</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_ADMIN">ROLE_MASK_ADMIN</a> }
</code></pre>



</details>

<a name="social_contracts_memory_role_mask_agent_manager"></a>

## Function `role_mask_agent_manager`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_agent_manager">role_mask_agent_manager</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_agent_manager">role_mask_agent_manager</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_AGENT_MANAGER">ROLE_MASK_AGENT_MANAGER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_role_mask_finance_approver"></a>

## Function `role_mask_finance_approver`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_finance_approver">role_mask_finance_approver</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_finance_approver">role_mask_finance_approver</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_FINANCE_APPROVER">ROLE_MASK_FINANCE_APPROVER</a> }
</code></pre>



</details>

<a name="social_contracts_memory_role_mask_memory_administrator"></a>

## Function `role_mask_memory_administrator`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_memory_administrator">role_mask_memory_administrator</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_memory_administrator">role_mask_memory_administrator</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_MEMORY_ADMINISTRATOR">ROLE_MASK_MEMORY_ADMINISTRATOR</a> }
</code></pre>



</details>

<a name="social_contracts_memory_role_mask_auditor"></a>

## Function `role_mask_auditor`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_auditor">role_mask_auditor</a>(): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_role_mask_auditor">role_mask_auditor</a>(): u64 { <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_AUDITOR">ROLE_MASK_AUDITOR</a> }
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

<a name="social_contracts_memory_organization_memory_account_id"></a>

## Function `organization_memory_account_id`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_memory_account_id">organization_memory_account_id</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_memory_account_id">organization_memory_account_id</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>): ID {
    org.memory_account_id
}
</code></pre>



</details>

<a name="social_contracts_memory_organization_active"></a>

## Function `organization_active`



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_active">organization_active</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_organization_active">organization_active</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>): bool {
    org.active
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



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bootstrap_init">bootstrap_init</a>(clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_bootstrap_init">bootstrap_init</a>(clock: &Clock, ctx: &<b>mut</b> TxContext) {
    <b>let</b> admin = tx_context::sender(ctx);
    <b>let</b> config = <a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a> {
        id: object::new(ctx),
        <a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>: <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORGANIZATIONS_PER_USER">MAX_ORGANIZATIONS_PER_USER</a>,
        org_category_update_cooldown_ms: <a href="../social_contracts/memory.md#social_contracts_memory_ORG_CATEGORY_UPDATE_COOLDOWN_MS">ORG_CATEGORY_UPDATE_COOLDOWN_MS</a>,
        max_agent_depth: <a href="../social_contracts/memory.md#social_contracts_memory_MAX_AGENT_DEPTH">MAX_AGENT_DEPTH</a>,
        max_label_length: <a href="../social_contracts/memory.md#social_contracts_memory_MAX_LABEL_LENGTH">MAX_LABEL_LENGTH</a>,
        max_org_name_length: <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORG_NAME_LENGTH">MAX_ORG_NAME_LENGTH</a>,
        max_org_description_length: <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORG_DESCRIPTION_LENGTH">MAX_ORG_DESCRIPTION_LENGTH</a>,
        version: <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>,
    };
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfigUpdatedEvent">MemoryConfigUpdatedEvent</a> {
        updated_by: admin,
        <a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>: <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORGANIZATIONS_PER_USER">MAX_ORGANIZATIONS_PER_USER</a>,
        org_category_update_cooldown_ms: <a href="../social_contracts/memory.md#social_contracts_memory_ORG_CATEGORY_UPDATE_COOLDOWN_MS">ORG_CATEGORY_UPDATE_COOLDOWN_MS</a>,
        max_agent_depth: <a href="../social_contracts/memory.md#social_contracts_memory_MAX_AGENT_DEPTH">MAX_AGENT_DEPTH</a>,
        max_label_length: <a href="../social_contracts/memory.md#social_contracts_memory_MAX_LABEL_LENGTH">MAX_LABEL_LENGTH</a>,
        max_org_name_length: <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORG_NAME_LENGTH">MAX_ORG_NAME_LENGTH</a>,
        max_org_description_length: <a href="../social_contracts/memory.md#social_contracts_memory_MAX_ORG_DESCRIPTION_LENGTH">MAX_ORG_DESCRIPTION_LENGTH</a>,
        timestamp: clock::timestamp_ms(clock),
    });
    transfer::share_object(config);
    <b>let</b> <b>mut</b> registry = <a href="../social_contracts/memory.md#social_contracts_memory_MemoryRegistry">MemoryRegistry</a> {
        id: object::new(ctx),
        accounts: table::new(ctx),
    };
    <a href="../social_contracts/memory.md#social_contracts_memory_set_version">set_version</a>(&<b>mut</b> registry.id, <a href="../social_contracts/memory.md#social_contracts_memory_VERSION">VERSION</a>);
    transfer::share_object(registry);
}
</code></pre>



</details>

<a name="social_contracts_memory_create_memory_admin_cap"></a>

## Function `create_memory_admin_cap`

Create a [<code><a href="../social_contracts/memory.md#social_contracts_memory_MemoryAdminCap">MemoryAdminCap</a></code>] for bootstrap (package visibility only).


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_memory_admin_cap">create_memory_admin_cap</a>(ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAdminCap">social_contracts::memory::MemoryAdminCap</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_memory_admin_cap">create_memory_admin_cap</a>(ctx: &<b>mut</b> TxContext): <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAdminCap">MemoryAdminCap</a> {
    <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAdminCap">MemoryAdminCap</a> { id: object::new(ctx) }
}
</code></pre>



</details>

<a name="social_contracts_memory_update_memory_config"></a>

## Function `update_memory_config`

Update global memory configuration (admin only).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_memory_config">update_memory_config</a>(_: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAdminCap">social_contracts::memory::MemoryAdminCap</a>, config: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, <a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>: u8, org_category_update_cooldown_ms: u64, max_agent_depth: u8, max_label_length: u64, max_org_name_length: u64, max_org_description_length: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_memory_config">update_memory_config</a>(
    _: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAdminCap">MemoryAdminCap</a>,
    config: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
    <a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>: u8,
    org_category_update_cooldown_ms: u64,
    max_agent_depth: u8,
    max_label_length: u64,
    max_org_name_length: u64,
    max_org_description_length: u64,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext
) {
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a> &gt; 0, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_agent_depth &gt; 0, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_label_length &gt; 0, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_org_name_length &gt; 0, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidConfig">EInvalidConfig</a>);
    <b>assert</b>!(max_org_description_length &gt; 0, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidConfig">EInvalidConfig</a>);
    config.<a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a> = <a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>;
    config.org_category_update_cooldown_ms = org_category_update_cooldown_ms;
    config.max_agent_depth = max_agent_depth;
    config.max_label_length = max_label_length;
    config.max_org_name_length = max_org_name_length;
    config.max_org_description_length = max_org_description_length;
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfigUpdatedEvent">MemoryConfigUpdatedEvent</a> {
        updated_by: tx_context::sender(ctx),
        <a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>,
        org_category_update_cooldown_ms,
        max_agent_depth,
        max_label_length,
        max_org_name_length,
        max_org_description_length,
        timestamp: clock::timestamp_ms(clock),
    });
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

<a name="social_contracts_memory_create_agentic_organization"></a>

## Function `create_agentic_organization`

Human owner creates a competitive agentic organization (max 8 per account).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_agentic_organization">create_agentic_organization</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org_type: u8, name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_agentic_organization">create_agentic_organization</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org_type: u8,
    name: Option&lt;String&gt;,
    description: Option&lt;String&gt;,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <b>assert</b>!(tx_context::sender(ctx) == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <b>let</b> _ = <a href="../social_contracts/memory.md#social_contracts_memory_create_agentic_organization_internal">create_agentic_organization_internal</a>(config, account, org_type, name, description, clock, ctx);
}
</code></pre>



</details>

<a name="social_contracts_memory_create_agentic_organization_internal"></a>

## Function `create_agentic_organization_internal`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_agentic_organization_internal">create_agentic_organization_internal</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org_type: u8, name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_agentic_organization_internal">create_agentic_organization_internal</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_name_within_limit">assert_org_name_within_limit</a>(config, &name);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_description_within_limit">assert_org_description_within_limit</a>(config, &description);
    <b>assert</b>!(account.org_count &lt; config.<a href="../social_contracts/memory.md#social_contracts_memory_max_organizations_per_user">max_organizations_per_user</a>, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationLimitExceeded">EOrganizationLimitExceeded</a>);
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



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_agentic_organization_metadata">update_agentic_organization_metadata</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, description: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_agentic_organization_metadata">update_agentic_organization_metadata</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_name_within_limit">assert_org_name_within_limit</a>(config, &name);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_description_within_limit">assert_org_description_within_limit</a>(config, &description);
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



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_agentic_organization_category">update_agentic_organization_category</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, org_type: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_agentic_organization_category">update_agentic_organization_category</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
            clock::timestamp_ms(clock) &gt;= last + config.org_category_update_cooldown_ms,
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

<a name="social_contracts_memory_ensure_org_memory_group"></a>

## Function `ensure_org_memory_group`

Lazy-create the org's memory share group (a <code>PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;</code>
derived from the org UID). Owner-only; idempotent. The owner receives
<code>PermissionsAdmin</code> + <code>ExtensionPermissionsAdmin</code> from group creation, then typically
grants <code>ExtensionPermissionsAdmin</code> to the org root agent's derived address once so the
root agent can manage member permissions without further human transactions.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_ensure_org_memory_group">ensure_org_memory_group</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_ensure_org_memory_group">ensure_org_memory_group</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    clock: &Clock,
    ctx: &<b>mut</b> TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(tx_context::sender(ctx) == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>, <a href="../social_contracts/memory.md#social_contracts_memory_ENotOwner">ENotOwner</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <b>if</b> (derived_object::exists(&org.id, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryGroupTag">OrgMemoryGroupTag</a>())) {
        <b>return</b>
    };
    <b>let</b> group = permissioned_group::new_derived&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryGroupTag">OrgMemoryGroupTag</a>&gt;(
        <a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>(),
        &<b>mut</b> org.id,
        <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryGroupTag">OrgMemoryGroupTag</a>(),
        ctx,
    );
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryGroupCreated">OrgMemoryGroupCreated</a> {
        group_id: object::id(&group),
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        account_id: object::id(account),
        principal_owner: account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>,
        created_at: clock::timestamp_ms(clock),
    });
    transfer::public_share_object(group);
}
</code></pre>



</details>

<a name="social_contracts_memory_org_memory_group_address"></a>

## Function `org_memory_group_address`

Deterministic address of the org's memory share group (whether or not created yet).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_memory_group_address">org_memory_group_address</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>): <b>address</b>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_memory_group_address">org_memory_group_address</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>): <b>address</b> {
    derived_object::derive_address(object::id(org), <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryGroupTag">OrgMemoryGroupTag</a>())
}
</code></pre>



</details>

<a name="social_contracts_memory_org_memory_group_exists"></a>

## Function `org_memory_group_exists`

Whether the org's memory share group has been created.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_memory_group_exists">org_memory_group_exists</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_memory_group_exists">org_memory_group_exists</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>): bool {
    derived_object::exists(&org.id, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryGroupTag">OrgMemoryGroupTag</a>())
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_org_permission"></a>

## Function `assert_org_permission`

Shared authorization helper for org-scoped permissions. Verifies the group is the
org's derived share group, the org is active, and <code>addr</code> holds witness permission <code>P</code>.
Used by this module and <code><a href="../social_contracts/ai_credit.md#social_contracts_ai_credit">ai_credit</a></code> (role-gated budget/approval entries) so permission
checks are never duplicated per call site.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission">assert_org_permission</a>&lt;P: drop&gt;(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, addr: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission">assert_org_permission</a>&lt;P: drop&gt;(
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    addr: <b>address</b>,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(org, group);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <b>assert</b>!(
        permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, P&gt;(group, addr),
        <a href="../social_contracts/memory.md#social_contracts_memory_ENoAccess">ENoAccess</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_memory_has_org_permission"></a>

## Function `has_org_permission`

Non-aborting variant of [<code><a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission">assert_org_permission</a></code>].


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_org_permission">has_org_permission</a>&lt;P: drop&gt;(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, addr: <b>address</b>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_org_permission">has_org_permission</a>&lt;P: drop&gt;(
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    addr: <b>address</b>,
): bool {
    <b>if</b> (!<a href="../social_contracts/memory.md#social_contracts_memory_is_org_group">is_org_group</a>(org, group) || !org.active) {
        <b>return</b> <b>false</b>
    };
    permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, P&gt;(group, addr)
}
</code></pre>



</details>

<a name="social_contracts_memory_grant_org_memory_permission"></a>

## Function `grant_org_memory_permission`

Grant org permissions (mask over the fixed witness set) to a member.
Caller must hold <code>ExtensionPermissionsAdmin</code> on the group (owner from creation, or a
delegated manager such as the org root agent). Granting is idempotent per bit.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_grant_org_memory_permission">grant_org_memory_permission</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, member: <b>address</b>, permissions_mask: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_grant_org_memory_permission">grant_org_memory_permission</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    member: <b>address</b>,
    permissions_mask: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(org, group);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission_manager">assert_org_permission_manager</a>(group, ctx);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_permission_mask">assert_valid_org_permission_mask</a>(permissions_mask);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_member_grantable">assert_member_grantable</a>(account, org, member);
    <b>let</b> _ = <a href="../social_contracts/memory.md#social_contracts_memory_grant_org_permissions_from_mask">grant_org_permissions_from_mask</a>(group, member, permissions_mask, ctx);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryPermissionGranted">OrgMemoryPermissionGranted</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        account_id: object::id(account),
        group_id: object::id(group),
        member,
        permissions_mask,
        granted_by: tx_context::sender(ctx),
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_revoke_org_memory_permission"></a>

## Function `revoke_org_memory_permission`

Revoke org permissions (mask) from a member. Revocation is idempotent per bit.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_revoke_org_memory_permission">revoke_org_memory_permission</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, member: <b>address</b>, permissions_mask: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_revoke_org_memory_permission">revoke_org_memory_permission</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    member: <b>address</b>,
    permissions_mask: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(org, group);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission_manager">assert_org_permission_manager</a>(group, ctx);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_permission_mask">assert_valid_org_permission_mask</a>(permissions_mask);
    <b>let</b> _ = <a href="../social_contracts/memory.md#social_contracts_memory_revoke_org_permissions_from_mask">revoke_org_permissions_from_mask</a>(group, member, permissions_mask, ctx);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryPermissionRevoked">OrgMemoryPermissionRevoked</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        account_id: object::id(account),
        group_id: object::id(group),
        member,
        permissions_mask,
        revoked_by: tx_context::sender(ctx),
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_define_custom_org_role"></a>

## Function `define_custom_org_role`

Define (or redefine) a custom org role as a named mask. Built-in role names are
reserved. Redefinition is safe: assignments record their exact granted delta.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_define_custom_org_role">define_custom_org_role</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, name: <a href="../std/string.md#std_string_String">std::string::String</a>, mask: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_define_custom_org_role">define_custom_org_role</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    name: String,
    mask: u64,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(org, group);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission_manager">assert_org_permission_manager</a>(group, ctx);
    <b>assert</b>!(string::length(&name) &lt;= config.max_label_length, <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleNameTooLong">EOrgRoleNameTooLong</a>);
    <b>assert</b>!(!<a href="../social_contracts/memory.md#social_contracts_memory_is_builtin_role_name">is_builtin_role_name</a>(&name), <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleBuiltinRedefine">EOrgRoleBuiltinRedefine</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_permission_mask">assert_valid_org_permission_mask</a>(mask);
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_OrgCustomRoleKey">OrgCustomRoleKey</a> { name };
    <b>let</b> previous_mask = <b>if</b> (df::exists_with_type&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgCustomRoleKey">OrgCustomRoleKey</a>, u64&gt;(&org.id, key)) {
        <b>let</b> existing = df::borrow_mut&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgCustomRoleKey">OrgCustomRoleKey</a>, u64&gt;(&<b>mut</b> org.id, key);
        <b>let</b> prev = *existing;
        *existing = mask;
        option::some(prev)
    } <b>else</b> {
        df::add(&<b>mut</b> org.id, key, mask);
        option::none()
    };
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleDefined">OrgRoleDefined</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        account_id: object::id(account),
        role_name: name,
        mask,
        previous_mask,
        defined_by: tx_context::sender(ctx),
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_assign_org_role"></a>

## Function `assign_org_role`

Assign a role (built-in or custom) to a member: grants the role's constituent
witnesses and records the exact granted delta for later revocation.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assign_org_role">assign_org_role</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, member: <b>address</b>, role_name: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assign_org_role">assign_org_role</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    member: <b>address</b>,
    role_name: String,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(org, group);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission_manager">assert_org_permission_manager</a>(group, ctx);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_member_grantable">assert_member_grantable</a>(account, org, member);
    <b>let</b> mask = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_role_mask">resolve_role_mask</a>(org, &role_name);
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssignmentKey">OrgRoleAssignmentKey</a> { member, role_name };
    <b>assert</b>!(
        !df::exists_with_type&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssignmentKey">OrgRoleAssignmentKey</a>, u64&gt;(&org.id, key),
        <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleAlreadyAssigned">EOrgRoleAlreadyAssigned</a>,
    );
    <b>let</b> granted_mask = <a href="../social_contracts/memory.md#social_contracts_memory_grant_org_permissions_from_mask">grant_org_permissions_from_mask</a>(group, member, mask, ctx);
    df::add(&<b>mut</b> org.id, key, granted_mask);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssigned">OrgRoleAssigned</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        account_id: object::id(account),
        group_id: object::id(group),
        member,
        role_name,
        mask,
        granted_mask,
        assigned_by: tx_context::sender(ctx),
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_revoke_org_role"></a>

## Function `revoke_org_role`

Revoke a role assignment: removes exactly the delta the assignment granted
(permissions the member held before the assignment are untouched).


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_revoke_org_role">revoke_org_role</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, member: <b>address</b>, role_name: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_revoke_org_role">revoke_org_role</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    member: <b>address</b>,
    role_name: String,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(org, group);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission_manager">assert_org_permission_manager</a>(group, ctx);
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssignmentKey">OrgRoleAssignmentKey</a> { member, role_name };
    <b>assert</b>!(
        df::exists_with_type&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssignmentKey">OrgRoleAssignmentKey</a>, u64&gt;(&org.id, key),
        <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleNotAssigned">EOrgRoleNotAssigned</a>,
    );
    <b>let</b> granted_mask: u64 = df::remove(&<b>mut</b> org.id, key);
    <b>let</b> _ = <a href="../social_contracts/memory.md#social_contracts_memory_revoke_org_permissions_from_mask">revoke_org_permissions_from_mask</a>(group, member, granted_mask, ctx);
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleRevoked">OrgRoleRevoked</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        account_id: object::id(account),
        group_id: object::id(group),
        member,
        role_name,
        revoked_mask: granted_mask,
        revoked_by: tx_context::sender(ctx),
        timestamp_ms: clock::timestamp_ms(clock),
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_org_role_assignment_mask"></a>

## Function `org_role_assignment_mask`

Assigned-delta mask for a role assignment, if present.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_role_assignment_mask">org_role_assignment_mask</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, member: <b>address</b>, role_name: <a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_role_assignment_mask">org_role_assignment_mask</a>(
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    member: <b>address</b>,
    role_name: String,
): Option&lt;u64&gt; {
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssignmentKey">OrgRoleAssignmentKey</a> { member, role_name };
    <b>if</b> (df::exists_with_type&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssignmentKey">OrgRoleAssignmentKey</a>, u64&gt;(&org.id, key)) {
        option::some(*df::borrow&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssignmentKey">OrgRoleAssignmentKey</a>, u64&gt;(&org.id, key))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_create_org_invitation"></a>

## Function `create_org_invitation`

Create a pending invitation for <code>invitee</code>. At least one of <code>role_name</code> or
<code>permissions_mask</code> must be set. The invitee accepts or declines via the
corresponding entry functions.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_org_invitation">create_org_invitation</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, invitee: <b>address</b>, role_name: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;, permissions_mask: u64, expires_at_ms: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_create_org_invitation">create_org_invitation</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    invitee: <b>address</b>,
    role_name: Option&lt;String&gt;,
    permissions_mask: u64,
    expires_at_ms: Option&lt;u64&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(org, group);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission_manager">assert_org_permission_manager</a>(group, ctx);
    <b>assert</b>!(
        option::is_some(&role_name) || permissions_mask != 0,
        <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationEmpty">EOrgInvitationEmpty</a>,
    );
    <b>if</b> (permissions_mask != 0) {
        <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_permission_mask">assert_valid_org_permission_mask</a>(permissions_mask);
    };
    <b>if</b> (option::is_some(&role_name)) {
        <b>let</b> name = option::borrow(&role_name);
        <b>let</b> _ = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_role_mask">resolve_role_mask</a>(org, name);
    };
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationKey">OrgInvitationKey</a> { invitee };
    <b>assert</b>!(
        !df::exists_with_type&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationKey">OrgInvitationKey</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitation">OrgInvitation</a>&gt;(&org.id, key),
        <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationExists">EOrgInvitationExists</a>,
    );
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>if</b> (option::is_some(&expires_at_ms)) {
        <b>assert</b>!(*option::borrow(&expires_at_ms) &gt; now, <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationExpired">EOrgInvitationExpired</a>);
    };
    <a href="../social_contracts/memory.md#social_contracts_memory_ensure_org_group_delegate_admin">ensure_org_group_delegate_admin</a>(org, group, ctx);
    <b>let</b> invited_by = tx_context::sender(ctx);
    <b>let</b> invitation = <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitation">OrgInvitation</a> {
        invitee,
        role_name,
        permissions_mask,
        invited_by,
        created_at_ms: now,
        expires_at_ms,
    };
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationCreated">OrgInvitationCreated</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        account_id: object::id(account),
        invitee,
        role_name: invitation.role_name,
        permissions_mask,
        invited_by,
        timestamp_ms: now,
        expires_at_ms,
    });
    df::add(&<b>mut</b> org.id, key, invitation);
}
</code></pre>



</details>

<a name="social_contracts_memory_accept_org_invitation"></a>

## Function `accept_org_invitation`

Accept a pending invitation: grants the invited role and/or permissions,
then removes the invitation dynamic field.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_accept_org_invitation">accept_org_invitation</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, invitee: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_accept_org_invitation">accept_org_invitation</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    invitee: <b>address</b>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(org, group);
    <b>assert</b>!(tx_context::sender(ctx) == invitee, <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationNotInvitee">EOrgInvitationNotInvitee</a>);
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationKey">OrgInvitationKey</a> { invitee };
    <b>assert</b>!(
        df::exists_with_type&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationKey">OrgInvitationKey</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitation">OrgInvitation</a>&gt;(&org.id, key),
        <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationNotFound">EOrgInvitationNotFound</a>,
    );
    <b>let</b> <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitation">OrgInvitation</a> {
        invitee: _,
        role_name,
        permissions_mask,
        invited_by: _,
        created_at_ms: _,
        expires_at_ms,
    } = df::remove(&<b>mut</b> org.id, key);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>if</b> (option::is_some(&expires_at_ms)) {
        <b>assert</b>!(*option::borrow(&expires_at_ms) &gt;= now, <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationExpired">EOrgInvitationExpired</a>);
    };
    <b>let</b> <b>mut</b> granted_mask = 0u64;
    <b>if</b> (option::is_some(&role_name)) {
        <b>let</b> name = option::borrow(&role_name);
        <b>let</b> mask = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_role_mask">resolve_role_mask</a>(org, name);
        <b>let</b> assignment_key = <a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssignmentKey">OrgRoleAssignmentKey</a> { member: invitee, role_name: *name };
        <b>if</b> (!df::exists_with_type&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgRoleAssignmentKey">OrgRoleAssignmentKey</a>, u64&gt;(&org.id, assignment_key)) {
            <b>let</b> role_granted =
                <a href="../social_contracts/memory.md#social_contracts_memory_grant_org_permissions_from_mask_via_org">grant_org_permissions_from_mask_via_org</a>(org, group, invitee, mask);
            df::add(&<b>mut</b> org.id, assignment_key, role_granted);
            granted_mask = granted_mask | role_granted;
        };
    };
    <b>if</b> (permissions_mask != 0) {
        granted_mask =
            granted_mask
                | <a href="../social_contracts/memory.md#social_contracts_memory_grant_org_permissions_from_mask_via_org">grant_org_permissions_from_mask_via_org</a>(
                    org,
                    group,
                    invitee,
                    permissions_mask,
                );
    };
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationAccepted">OrgInvitationAccepted</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        account_id: object::id(account),
        group_id: object::id(group),
        invitee,
        role_name,
        permissions_mask,
        granted_mask,
        accepted_by: tx_context::sender(ctx),
        timestamp_ms: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_decline_org_invitation"></a>

## Function `decline_org_invitation`

Decline a pending invitation and remove the invitation dynamic field.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_decline_org_invitation">decline_org_invitation</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, invitee: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_decline_org_invitation">decline_org_invitation</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    invitee: <b>address</b>,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(tx_context::sender(ctx) == invitee, <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationNotInvitee">EOrgInvitationNotInvitee</a>);
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationKey">OrgInvitationKey</a> { invitee };
    <b>assert</b>!(
        df::exists_with_type&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationKey">OrgInvitationKey</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitation">OrgInvitation</a>&gt;(&org.id, key),
        <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationNotFound">EOrgInvitationNotFound</a>,
    );
    <b>let</b> invitation: <a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitation">OrgInvitation</a> = df::remove(&<b>mut</b> org.id, key);
    <b>let</b> now = clock::timestamp_ms(clock);
    <b>if</b> (option::is_some(&invitation.expires_at_ms)) {
        <b>assert</b>!(*option::borrow(&invitation.expires_at_ms) &gt;= now, <a href="../social_contracts/memory.md#social_contracts_memory_EOrgInvitationExpired">EOrgInvitationExpired</a>);
    };
    event::emit(<a href="../social_contracts/memory.md#social_contracts_memory_OrgInvitationDeclined">OrgInvitationDeclined</a> {
        <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: object::id(org),
        account_id: object::id(account),
        invitee,
        declined_by: tx_context::sender(ctx),
        timestamp_ms: now,
    });
}
</code></pre>



</details>

<a name="social_contracts_memory_org_role_mask"></a>

## Function `org_role_mask`

Effective mask for a role name (built-in constant or custom definition).


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_role_mask">org_role_mask</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, role_name: &<a href="../std/string.md#std_string_String">std::string::String</a>): <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_org_role_mask">org_role_mask</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>, role_name: &String): Option&lt;u64&gt; {
    <b>if</b> (<a href="../social_contracts/memory.md#social_contracts_memory_is_builtin_role_name">is_builtin_role_name</a>(role_name)) {
        <b>return</b> option::some(<a href="../social_contracts/memory.md#social_contracts_memory_builtin_role_mask">builtin_role_mask</a>(role_name))
    };
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_OrgCustomRoleKey">OrgCustomRoleKey</a> { name: *role_name };
    <b>if</b> (df::exists_with_type&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgCustomRoleKey">OrgCustomRoleKey</a>, u64&gt;(&org.id, key)) {
        option::some(*df::borrow&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgCustomRoleKey">OrgCustomRoleKey</a>, u64&gt;(&org.id, key))
    } <b>else</b> {
        option::none()
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_approve_org_key_policy"></a>

## Function `approve_org_key_policy`

Approve MYDATA key release for org-shared memory: the account owner (own-blob suffix)
or any holder of <code><a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryReader">OrgMemoryReader</a></code> on the org's share group. Registered sub-agents
must additionally have an active ancestor chain.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_org_key_policy">approve_org_key_policy</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, id: vector&lt;u8&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_org_key_policy">approve_org_key_policy</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
    id: vector&lt;u8&gt;,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <b>assert</b>!(account.active, <a href="../social_contracts/memory.md#social_contracts_memory_EAccountDeactivated">EAccountDeactivated</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_organization_belongs_to_account">assert_organization_belongs_to_account</a>(account, org);
    <b>assert</b>!(org.active, <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationNotActive">EOrganizationNotActive</a>);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(org, group);
    <b>let</b> caller = tx_context::sender(ctx);
    <b>let</b> owner_bytes = bcs::to_bytes(&account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>);
    <b>if</b> ((caller == account.<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>) && <a href="../social_contracts/memory.md#social_contracts_memory_has_suffix">has_suffix</a>(&id, &owner_bytes)) {
        <b>return</b>
    };
    <b>if</b> (table::contains(&account.agents, caller)) {
        <a href="../social_contracts/memory.md#social_contracts_memory_assert_ancestor_chain_active_from_table">assert_ancestor_chain_active_from_table</a>(config, account, caller, clock);
    };
    <b>assert</b>!(
        permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryReader">OrgMemoryReader</a>&gt;(group, caller),
        <a href="../social_contracts/memory.md#social_contracts_memory_ENoAccess">ENoAccess</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_memory_register_sub_agent"></a>

## Function `register_sub_agent`

Human owner registers a root-level sub-agent bound to an organization.


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent">register_sub_agent</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, organization: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent">register_sub_agent</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
        config,
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


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_delegated">register_sub_agent_delegated</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, register_relation: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<b>mut</b> <a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_delegated">register_sub_agent_delegated</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
        config,
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



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_sub_agent_label">update_sub_agent_label</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_update_sub_agent_label">update_sub_agent_label</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
    account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    agent: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">SubAgent</a>,
    label: String,
    clock: &Clock,
    ctx: &TxContext,
) {
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_object_version">assert_object_version</a>(&account.id);
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_agent_belongs_to_account">assert_agent_belongs_to_account</a>(account, agent);
    <b>assert</b>!(string::length(&label) &lt;= config.max_label_length, <a href="../social_contracts/memory.md#social_contracts_memory_ELabelTooLong">ELabelTooLong</a>);
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



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_from_account">resolve_actor_from_account</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_from_account">resolve_actor_from_account</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
    <a href="../social_contracts/memory.md#social_contracts_memory_assert_ancestor_chain_active_from_table">assert_ancestor_chain_active_from_table</a>(config, root, sender, clock);
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



<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">resolve_actor_with_cap</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, root: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, required_cap: u64, action_platform_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, spend_amount: u64, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): <a href="../social_contracts/memory.md#social_contracts_memory_ActingContext">social_contracts::memory::ActingContext</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_with_cap">resolve_actor_with_cap</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
    <b>let</b> acting = <a href="../social_contracts/memory.md#social_contracts_memory_resolve_actor_from_account">resolve_actor_from_account</a>(config, root, clock, ctx);
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

<a name="social_contracts_memory_is_descendant_agent"></a>

## Function `is_descendant_agent`

True when <code>descendant_id</code> sits strictly below <code>ancestor_id</code> in the agent tree
(walks the registry mirror; bounded by MAX_AGENT_DEPTH). Self is not a descendant.


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_descendant_agent">is_descendant_agent</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, ancestor_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, descendant_id: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_descendant_agent">is_descendant_agent</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    ancestor_id: ID,
    descendant_id: ID,
): bool {
    <b>if</b> (!table::contains(&account.agent_ids, descendant_id)) {
        <b>return</b> <b>false</b>
    };
    <b>let</b> derived = *table::borrow(&account.agent_ids, descendant_id);
    <b>let</b> <b>entry</b> = table::borrow(&account.agents, derived);
    <b>let</b> <b>mut</b> current_parent = <b>entry</b>.parent_object_id;
    <b>let</b> <b>mut</b> hops = 0u8;
    <b>while</b> (option::is_some(&current_parent)) {
        hops = hops + 1;
        <b>if</b> (hops &gt; config.max_agent_depth) {
            <b>return</b> <b>false</b>
        };
        <b>let</b> parent_id = *option::borrow(&current_parent);
        <b>if</b> (parent_id == ancestor_id) {
            <b>return</b> <b>true</b>
        };
        <b>if</b> (!table::contains(&account.agent_ids, parent_id)) {
            <b>return</b> <b>false</b>
        };
        <b>let</b> parent_derived = *table::borrow(&account.agent_ids, parent_id);
        <b>let</b> parent_entry = table::borrow(&account.agents, parent_derived);
        current_parent = parent_entry.parent_object_id;
    };
    <b>false</b>
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



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_key_policy">approve_key_policy</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, id: vector&lt;u8&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_key_policy">approve_key_policy</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
        config,
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



<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_key_write_policy">approve_key_write_policy</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, id: vector&lt;u8&gt;, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b> <b>entry</b> <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_approve_key_write_policy">approve_key_write_policy</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
        config,
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



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_internal">register_sub_agent_internal</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, organization: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_internal">register_sub_agent_internal</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
        config,
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



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_delegated_internal">register_sub_agent_delegated_internal</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent_agent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, register_relation: u8, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_register_sub_agent_delegated_internal">register_sub_agent_delegated_internal</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
        config,
        account,
        parent_agent,
        register_relation,
        capabilities,
        delegatable_caps,
        platform_scope,
        clock,
    );
    <a href="../social_contracts/memory.md#social_contracts_memory_finish_register_sub_agent">finish_register_sub_agent</a>(
        config,
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



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_finish_register_sub_agent">finish_register_sub_agent</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<b>mut</b> <a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, <a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a>: <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>, public_key: vector&lt;u8&gt;, derived_address: <b>address</b>, label: <a href="../std/string.md#std_string_String">std::string::String</a>, identity_class: u8, role_tags: u64, capabilities: u64, delegatable_caps: u64, register_scope: u8, approval_required_caps: u64, max_action_spend: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, expires_at: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;u64&gt;, depth: u8, parent_object_id: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, registered_by: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): <a href="../myso/object.md#myso_object_ID">myso::object::ID</a>
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_finish_register_sub_agent">finish_register_sub_agent</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
    <b>assert</b>!(string::length(&label) &lt;= config.max_label_length, <a href="../social_contracts/memory.md#social_contracts_memory_ELabelTooLong">ELabelTooLong</a>);
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



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_ancestor_chain_active_from_table">assert_ancestor_chain_active_from_table</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, derived_address: <b>address</b>, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_ancestor_chain_active_from_table">assert_ancestor_chain_active_from_table</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
        <b>assert</b>!(hops &lt;= config.max_agent_depth, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidAncestorChain">EInvalidAncestorChain</a>);
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



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_delegated_registration_placement">resolve_delegated_registration_placement</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, parent: &<a href="../social_contracts/memory.md#social_contracts_memory_SubAgent">social_contracts::memory::SubAgent</a>, register_relation: u8, capabilities: u64, delegatable_caps: u64, platform_scope: <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<b>address</b>&gt;, clock: &<a href="../myso/clock.md#myso_clock_Clock">myso::clock::Clock</a>): (u8, <a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../myso/object.md#myso_object_ID">myso::object::ID</a>&gt;, <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_delegated_registration_placement">resolve_delegated_registration_placement</a>(
    config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>,
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
        <b>assert</b>!(depth &lt;= config.max_agent_depth, <a href="../social_contracts/memory.md#social_contracts_memory_EAgentDepthExceeded">EAgentDepthExceeded</a>);
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



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_name_within_limit">assert_org_name_within_limit</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, name: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_name_within_limit">assert_org_name_within_limit</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>, name: &Option&lt;String&gt;) {
    <b>if</b> (option::is_some(name)) {
        <b>assert</b>!(
            string::length(option::borrow(name)) &lt;= config.max_org_name_length,
            <a href="../social_contracts/memory.md#social_contracts_memory_ENameTooLong">ENameTooLong</a>,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_org_description_within_limit"></a>

## Function `assert_org_description_within_limit`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_description_within_limit">assert_org_description_within_limit</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">social_contracts::memory::MemoryConfig</a>, description: &<a href="../std/option.md#std_option_Option">std::option::Option</a>&lt;<a href="../std/string.md#std_string_String">std::string::String</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_description_within_limit">assert_org_description_within_limit</a>(config: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryConfig">MemoryConfig</a>, description: &Option&lt;String&gt;) {
    <b>if</b> (option::is_some(description)) {
        <b>assert</b>!(
            string::length(option::borrow(description)) &lt;= config.max_org_description_length,
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

<a name="social_contracts_memory_is_org_group"></a>

## Function `is_org_group`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_org_group">is_org_group</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_org_group">is_org_group</a>(
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
): bool {
    object::id_to_address(&object::id(group)) == <a href="../social_contracts/memory.md#social_contracts_memory_org_memory_group_address">org_memory_group_address</a>(org)
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_org_group"></a>

## Function `assert_org_group`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_group">assert_org_group</a>(
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
) {
    <b>assert</b>!(<a href="../social_contracts/memory.md#social_contracts_memory_is_org_group">is_org_group</a>(org, group), <a href="../social_contracts/memory.md#social_contracts_memory_EOrgGroupMismatch">EOrgGroupMismatch</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_org_permission_manager"></a>

## Function `assert_org_permission_manager`

The framework re-checks manager permission per grant/revoke; this explicit check exists
so wrappers fail fast (and so no-op masks cannot bypass authorization entirely).


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission_manager">assert_org_permission_manager</a>(group: &<a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_org_permission_manager">assert_org_permission_manager</a>(
    group: &PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    ctx: &TxContext,
) {
    <b>assert</b>!(
        permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, ExtensionPermissionsAdmin&gt;(
            group,
            tx_context::sender(ctx),
        ),
        <a href="../social_contracts/memory.md#social_contracts_memory_ENoAccess">ENoAccess</a>,
    );
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_valid_org_permission_mask"></a>

## Function `assert_valid_org_permission_mask`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_permission_mask">assert_valid_org_permission_mask</a>(mask: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_permission_mask">assert_valid_org_permission_mask</a>(mask: u64) {
    <b>assert</b>!(mask != 0, <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleMaskEmpty">EOrgRoleMaskEmpty</a>);
    <b>let</b> ops = mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_ALL">ORG_PERM_ALL</a>;
    <b>let</b> gov = mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PERM_ALL">ORG_GOVERNANCE_PERM_ALL</a>;
    <b>assert</b>!(ops | gov == mask, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidOrgPermission">EInvalidOrgPermission</a>);
    <b>if</b> (ops != 0) {
        <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_ops_permission_mask">assert_valid_org_ops_permission_mask</a>(ops);
    };
    <b>if</b> (gov != 0) {
        <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_governance_permission_mask">assert_valid_org_governance_permission_mask</a>(gov);
    };
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_valid_org_ops_permission_mask"></a>

## Function `assert_valid_org_ops_permission_mask`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_ops_permission_mask">assert_valid_org_ops_permission_mask</a>(mask: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_ops_permission_mask">assert_valid_org_ops_permission_mask</a>(mask: u64) {
    <b>assert</b>!(mask != 0, <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleMaskEmpty">EOrgRoleMaskEmpty</a>);
    <b>assert</b>!((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_ALL">ORG_PERM_ALL</a>) == mask, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidOrgPermission">EInvalidOrgPermission</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_valid_org_governance_permission_mask"></a>

## Function `assert_valid_org_governance_permission_mask`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_governance_permission_mask">assert_valid_org_governance_permission_mask</a>(mask: u64)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_valid_org_governance_permission_mask">assert_valid_org_governance_permission_mask</a>(mask: u64) {
    <b>assert</b>!(mask != 0, <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleMaskEmpty">EOrgRoleMaskEmpty</a>);
    <b>assert</b>!((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PERM_ALL">ORG_GOVERNANCE_PERM_ALL</a>) == mask, <a href="../social_contracts/memory.md#social_contracts_memory_EInvalidOrgPermission">EInvalidOrgPermission</a>);
}
</code></pre>



</details>

<a name="social_contracts_memory_assert_member_grantable"></a>

## Function `assert_member_grantable`

Grantable members: human addresses (org staff, not in the agents table) or registered
sub-agents belonging to this org. Cross-org agent grants are rejected because the
relayer scopes org recall by the agent's own organization.


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_member_grantable">assert_member_grantable</a>(account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">social_contracts::memory::MemoryAccount</a>, org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, member: <b>address</b>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_assert_member_grantable">assert_member_grantable</a>(
    account: &<a href="../social_contracts/memory.md#social_contracts_memory_MemoryAccount">MemoryAccount</a>,
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    member: <b>address</b>,
) {
    <b>if</b> (table::contains(&account.agents, member)) {
        <b>let</b> <b>entry</b> = table::borrow(&account.agents, member);
        <b>assert</b>!(<b>entry</b>.<a href="../social_contracts/memory.md#social_contracts_memory_organization_id">organization_id</a> == object::id(org), <a href="../social_contracts/memory.md#social_contracts_memory_EOrganizationOrgMismatch">EOrganizationOrgMismatch</a>);
    };
}
</code></pre>



</details>

<a name="social_contracts_memory_grant_org_permissions_from_mask_via_org"></a>

## Function `grant_org_permissions_from_mask_via_org`

Grant each witness in <code>mask</code> via the org object's delegate admin on the group.
Used when the transaction sender is the invitee (accept path) rather than a manager.


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_grant_org_permissions_from_mask_via_org">grant_org_permissions_from_mask_via_org</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, member: <b>address</b>, mask: u64): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_grant_org_permissions_from_mask_via_org">grant_org_permissions_from_mask_via_org</a>(
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    member: <b>address</b>,
    mask: u64,
): u64 {
    <b>let</b> <b>mut</b> granted = 0u64;
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_READ">ORG_PERM_MEMORY_READ</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryReader">OrgMemoryReader</a>&gt;(group, member)) {
        permissioned_group::object_grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryReader">OrgMemoryReader</a>&gt;(
            group,
            &org.id,
            member,
        );
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_READ">ORG_PERM_MEMORY_READ</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_WRITE">ORG_PERM_MEMORY_WRITE</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryWriter">OrgMemoryWriter</a>&gt;(group, member)) {
        permissioned_group::object_grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryWriter">OrgMemoryWriter</a>&gt;(
            group,
            &org.id,
            member,
        );
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_WRITE">ORG_PERM_MEMORY_WRITE</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AGENT_MANAGER">ORG_PERM_AGENT_MANAGER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAgentManager">OrgAgentManager</a>&gt;(group, member)) {
        permissioned_group::object_grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAgentManager">OrgAgentManager</a>&gt;(
            group,
            &org.id,
            member,
        );
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AGENT_MANAGER">ORG_PERM_AGENT_MANAGER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_BUDGET_MANAGER">ORG_PERM_BUDGET_MANAGER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgBudgetManager">OrgBudgetManager</a>&gt;(group, member)) {
        permissioned_group::object_grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgBudgetManager">OrgBudgetManager</a>&gt;(
            group,
            &org.id,
            member,
        );
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_BUDGET_MANAGER">ORG_PERM_BUDGET_MANAGER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_SPEND_APPROVER">ORG_PERM_SPEND_APPROVER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgSpendApprover">OrgSpendApprover</a>&gt;(group, member)) {
        permissioned_group::object_grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgSpendApprover">OrgSpendApprover</a>&gt;(
            group,
            &org.id,
            member,
        );
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_SPEND_APPROVER">ORG_PERM_SPEND_APPROVER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_DASHBOARD_VIEWER">ORG_PERM_DASHBOARD_VIEWER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgDashboardViewer">OrgDashboardViewer</a>&gt;(group, member)) {
        permissioned_group::object_grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgDashboardViewer">OrgDashboardViewer</a>&gt;(
            group,
            &org.id,
            member,
        );
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_DASHBOARD_VIEWER">ORG_PERM_DASHBOARD_VIEWER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AUDITOR">ORG_PERM_AUDITOR</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAuditor">OrgAuditor</a>&gt;(group, member)) {
        permissioned_group::object_grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAuditor">OrgAuditor</a>&gt;(
            group,
            &org.id,
            member,
        );
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AUDITOR">ORG_PERM_AUDITOR</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PROPOSER">ORG_GOVERNANCE_PROPOSER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceProposer">OrgGovernanceProposer</a>&gt;(group, member)) {
        permissioned_group::object_grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceProposer">OrgGovernanceProposer</a>&gt;(
            group,
            &org.id,
            member,
        );
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PROPOSER">ORG_GOVERNANCE_PROPOSER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_VOTER">ORG_GOVERNANCE_VOTER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceVoter">OrgGovernanceVoter</a>&gt;(group, member)) {
        permissioned_group::object_grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceVoter">OrgGovernanceVoter</a>&gt;(
            group,
            &org.id,
            member,
        );
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_VOTER">ORG_GOVERNANCE_VOTER</a>;
    };
    granted
}
</code></pre>



</details>

<a name="social_contracts_memory_ensure_org_group_delegate_admin"></a>

## Function `ensure_org_group_delegate_admin`

One-time bootstrap: grant the org shared object delegate admin on its memory group so
invitation accept can fulfill grants without the invitee holding manager permission.


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_ensure_org_group_delegate_admin">ensure_org_group_delegate_admin</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>)
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_ensure_org_group_delegate_admin">ensure_org_group_delegate_admin</a>(
    org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>,
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    ctx: &TxContext,
) {
    <b>let</b> org_addr = object::id_to_address(&object::id(org));
    <b>if</b> (!permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, ExtensionPermissionsAdmin&gt;(
        group,
        org_addr,
    )) {
        permissioned_group::grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, ExtensionPermissionsAdmin&gt;(
            group,
            org_addr,
            ctx,
        );
    };
}
</code></pre>



</details>

<a name="social_contracts_memory_grant_org_permissions_from_mask"></a>

## Function `grant_org_permissions_from_mask`

Grant each witness in <code>mask</code> the member does not already hold. Returns the delta
actually granted. Static per-bit branches (Move cannot grant by runtime TypeName).


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_grant_org_permissions_from_mask">grant_org_permissions_from_mask</a>(group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, member: <b>address</b>, mask: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_grant_org_permissions_from_mask">grant_org_permissions_from_mask</a>(
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    member: <b>address</b>,
    mask: u64,
    ctx: &TxContext,
): u64 {
    <b>let</b> <b>mut</b> granted = 0u64;
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_READ">ORG_PERM_MEMORY_READ</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryReader">OrgMemoryReader</a>&gt;(group, member)) {
        permissioned_group::grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryReader">OrgMemoryReader</a>&gt;(group, member, ctx);
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_READ">ORG_PERM_MEMORY_READ</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_WRITE">ORG_PERM_MEMORY_WRITE</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryWriter">OrgMemoryWriter</a>&gt;(group, member)) {
        permissioned_group::grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryWriter">OrgMemoryWriter</a>&gt;(group, member, ctx);
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_WRITE">ORG_PERM_MEMORY_WRITE</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AGENT_MANAGER">ORG_PERM_AGENT_MANAGER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAgentManager">OrgAgentManager</a>&gt;(group, member)) {
        permissioned_group::grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAgentManager">OrgAgentManager</a>&gt;(group, member, ctx);
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AGENT_MANAGER">ORG_PERM_AGENT_MANAGER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_BUDGET_MANAGER">ORG_PERM_BUDGET_MANAGER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgBudgetManager">OrgBudgetManager</a>&gt;(group, member)) {
        permissioned_group::grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgBudgetManager">OrgBudgetManager</a>&gt;(group, member, ctx);
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_BUDGET_MANAGER">ORG_PERM_BUDGET_MANAGER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_SPEND_APPROVER">ORG_PERM_SPEND_APPROVER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgSpendApprover">OrgSpendApprover</a>&gt;(group, member)) {
        permissioned_group::grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgSpendApprover">OrgSpendApprover</a>&gt;(group, member, ctx);
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_SPEND_APPROVER">ORG_PERM_SPEND_APPROVER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_DASHBOARD_VIEWER">ORG_PERM_DASHBOARD_VIEWER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgDashboardViewer">OrgDashboardViewer</a>&gt;(group, member)) {
        permissioned_group::grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgDashboardViewer">OrgDashboardViewer</a>&gt;(group, member, ctx);
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_DASHBOARD_VIEWER">ORG_PERM_DASHBOARD_VIEWER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AUDITOR">ORG_PERM_AUDITOR</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAuditor">OrgAuditor</a>&gt;(group, member)) {
        permissioned_group::grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAuditor">OrgAuditor</a>&gt;(group, member, ctx);
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AUDITOR">ORG_PERM_AUDITOR</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PROPOSER">ORG_GOVERNANCE_PROPOSER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceProposer">OrgGovernanceProposer</a>&gt;(group, member)) {
        permissioned_group::grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceProposer">OrgGovernanceProposer</a>&gt;(group, member, ctx);
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PROPOSER">ORG_GOVERNANCE_PROPOSER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_VOTER">ORG_GOVERNANCE_VOTER</a>) != 0
        && !permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceVoter">OrgGovernanceVoter</a>&gt;(group, member)) {
        permissioned_group::grant_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceVoter">OrgGovernanceVoter</a>&gt;(group, member, ctx);
        granted = granted | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_VOTER">ORG_GOVERNANCE_VOTER</a>;
    };
    granted
}
</code></pre>



</details>

<a name="social_contracts_memory_revoke_org_permissions_from_mask"></a>

## Function `revoke_org_permissions_from_mask`

Revoke each witness in <code>mask</code> the member currently holds. Returns the delta revoked.


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_revoke_org_permissions_from_mask">revoke_org_permissions_from_mask</a>(group: &<b>mut</b> <a href="../myso/permissioned_group.md#myso_permissioned_group_PermissionedGroup">myso::permissioned_group::PermissionedGroup</a>&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">social_contracts::memory::MemorySharePackage</a>&gt;, member: <b>address</b>, mask: u64, ctx: &<a href="../myso/tx_context.md#myso_tx_context_TxContext">myso::tx_context::TxContext</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_revoke_org_permissions_from_mask">revoke_org_permissions_from_mask</a>(
    group: &<b>mut</b> PermissionedGroup&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>&gt;,
    member: <b>address</b>,
    mask: u64,
    ctx: &TxContext,
): u64 {
    <b>let</b> <b>mut</b> revoked = 0u64;
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_READ">ORG_PERM_MEMORY_READ</a>) != 0
        && permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryReader">OrgMemoryReader</a>&gt;(group, member)) {
        permissioned_group::revoke_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryReader">OrgMemoryReader</a>&gt;(group, member, ctx);
        revoked = revoked | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_READ">ORG_PERM_MEMORY_READ</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_WRITE">ORG_PERM_MEMORY_WRITE</a>) != 0
        && permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryWriter">OrgMemoryWriter</a>&gt;(group, member)) {
        permissioned_group::revoke_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgMemoryWriter">OrgMemoryWriter</a>&gt;(group, member, ctx);
        revoked = revoked | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_MEMORY_WRITE">ORG_PERM_MEMORY_WRITE</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AGENT_MANAGER">ORG_PERM_AGENT_MANAGER</a>) != 0
        && permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAgentManager">OrgAgentManager</a>&gt;(group, member)) {
        permissioned_group::revoke_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAgentManager">OrgAgentManager</a>&gt;(group, member, ctx);
        revoked = revoked | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AGENT_MANAGER">ORG_PERM_AGENT_MANAGER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_BUDGET_MANAGER">ORG_PERM_BUDGET_MANAGER</a>) != 0
        && permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgBudgetManager">OrgBudgetManager</a>&gt;(group, member)) {
        permissioned_group::revoke_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgBudgetManager">OrgBudgetManager</a>&gt;(group, member, ctx);
        revoked = revoked | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_BUDGET_MANAGER">ORG_PERM_BUDGET_MANAGER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_SPEND_APPROVER">ORG_PERM_SPEND_APPROVER</a>) != 0
        && permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgSpendApprover">OrgSpendApprover</a>&gt;(group, member)) {
        permissioned_group::revoke_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgSpendApprover">OrgSpendApprover</a>&gt;(group, member, ctx);
        revoked = revoked | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_SPEND_APPROVER">ORG_PERM_SPEND_APPROVER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_DASHBOARD_VIEWER">ORG_PERM_DASHBOARD_VIEWER</a>) != 0
        && permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgDashboardViewer">OrgDashboardViewer</a>&gt;(group, member)) {
        permissioned_group::revoke_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgDashboardViewer">OrgDashboardViewer</a>&gt;(group, member, ctx);
        revoked = revoked | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_DASHBOARD_VIEWER">ORG_PERM_DASHBOARD_VIEWER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AUDITOR">ORG_PERM_AUDITOR</a>) != 0
        && permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAuditor">OrgAuditor</a>&gt;(group, member)) {
        permissioned_group::revoke_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgAuditor">OrgAuditor</a>&gt;(group, member, ctx);
        revoked = revoked | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_PERM_AUDITOR">ORG_PERM_AUDITOR</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PROPOSER">ORG_GOVERNANCE_PROPOSER</a>) != 0
        && permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceProposer">OrgGovernanceProposer</a>&gt;(group, member)) {
        permissioned_group::revoke_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceProposer">OrgGovernanceProposer</a>&gt;(group, member, ctx);
        revoked = revoked | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_PROPOSER">ORG_GOVERNANCE_PROPOSER</a>;
    };
    <b>if</b> ((mask & <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_VOTER">ORG_GOVERNANCE_VOTER</a>) != 0
        && permissioned_group::has_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceVoter">OrgGovernanceVoter</a>&gt;(group, member)) {
        permissioned_group::revoke_permission&lt;<a href="../social_contracts/memory.md#social_contracts_memory_MemorySharePackage">MemorySharePackage</a>, <a href="../social_contracts/memory.md#social_contracts_memory_OrgGovernanceVoter">OrgGovernanceVoter</a>&gt;(group, member, ctx);
        revoked = revoked | <a href="../social_contracts/memory.md#social_contracts_memory_ORG_GOVERNANCE_VOTER">ORG_GOVERNANCE_VOTER</a>;
    };
    revoked
}
</code></pre>



</details>

<a name="social_contracts_memory_is_builtin_role_name"></a>

## Function `is_builtin_role_name`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_builtin_role_name">is_builtin_role_name</a>(name: &<a href="../std/string.md#std_string_String">std::string::String</a>): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_is_builtin_role_name">is_builtin_role_name</a>(name: &String): bool {
    <b>let</b> bytes = *string::as_bytes(name);
    bytes == b"<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>"
        || bytes == b"admin"
        || bytes == b"agent_manager"
        || bytes == b"finance_approver"
        || bytes == b"memory_administrator"
        || bytes == b"auditor"
}
</code></pre>



</details>

<a name="social_contracts_memory_builtin_role_mask"></a>

## Function `builtin_role_mask`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_builtin_role_mask">builtin_role_mask</a>(name: &<a href="../std/string.md#std_string_String">std::string::String</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_builtin_role_mask">builtin_role_mask</a>(name: &String): u64 {
    <b>let</b> bytes = *string::as_bytes(name);
    <b>if</b> (bytes == b"<a href="../social_contracts/memory.md#social_contracts_memory_owner">owner</a>") {
        <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_OWNER">ROLE_MASK_OWNER</a>
    } <b>else</b> <b>if</b> (bytes == b"admin") {
        <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_ADMIN">ROLE_MASK_ADMIN</a>
    } <b>else</b> <b>if</b> (bytes == b"agent_manager") {
        <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_AGENT_MANAGER">ROLE_MASK_AGENT_MANAGER</a>
    } <b>else</b> <b>if</b> (bytes == b"finance_approver") {
        <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_FINANCE_APPROVER">ROLE_MASK_FINANCE_APPROVER</a>
    } <b>else</b> <b>if</b> (bytes == b"memory_administrator") {
        <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_MEMORY_ADMINISTRATOR">ROLE_MASK_MEMORY_ADMINISTRATOR</a>
    } <b>else</b> <b>if</b> (bytes == b"auditor") {
        <a href="../social_contracts/memory.md#social_contracts_memory_ROLE_MASK_AUDITOR">ROLE_MASK_AUDITOR</a>
    } <b>else</b> {
        <b>abort</b> <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleNotFound">EOrgRoleNotFound</a>
    }
}
</code></pre>



</details>

<a name="social_contracts_memory_resolve_role_mask"></a>

## Function `resolve_role_mask`



<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_role_mask">resolve_role_mask</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">social_contracts::memory::AgenticOrganization</a>, role_name: &<a href="../std/string.md#std_string_String">std::string::String</a>): u64
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_resolve_role_mask">resolve_role_mask</a>(org: &<a href="../social_contracts/memory.md#social_contracts_memory_AgenticOrganization">AgenticOrganization</a>, role_name: &String): u64 {
    <b>if</b> (<a href="../social_contracts/memory.md#social_contracts_memory_is_builtin_role_name">is_builtin_role_name</a>(role_name)) {
        <b>return</b> <a href="../social_contracts/memory.md#social_contracts_memory_builtin_role_mask">builtin_role_mask</a>(role_name)
    };
    <b>let</b> key = <a href="../social_contracts/memory.md#social_contracts_memory_OrgCustomRoleKey">OrgCustomRoleKey</a> { name: *role_name };
    <b>assert</b>!(
        df::exists_with_type&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgCustomRoleKey">OrgCustomRoleKey</a>, u64&gt;(&org.id, key),
        <a href="../social_contracts/memory.md#social_contracts_memory_EOrgRoleNotFound">EOrgRoleNotFound</a>,
    );
    *df::borrow&lt;<a href="../social_contracts/memory.md#social_contracts_memory_OrgCustomRoleKey">OrgCustomRoleKey</a>, u64&gt;(&org.id, key)
}
</code></pre>



</details>

<a name="social_contracts_memory_has_cap"></a>

## Function `has_cap`



<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_cap">has_cap</a>(capabilities: u64, required_cap: u64): bool
</code></pre>



<details>
<summary>Implementation</summary>


<pre><code><b>public</b>(package) <b>fun</b> <a href="../social_contracts/memory.md#social_contracts_memory_has_cap">has_cap</a>(capabilities: u64, required_cap: u64): bool {
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
