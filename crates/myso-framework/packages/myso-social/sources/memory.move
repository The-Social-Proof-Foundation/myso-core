// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Memory — human root account and hierarchical permissioned sub-agent layer.
///
/// One slim [`MemoryAccount`] per human owner (linked from [`social_contracts::profile::Profile`]).
/// Each sub-agent is a **shared derived object** keyed by `(memory_account_id, derived_address)`.
/// Agents sign as `derived_address` and resolve to the human `principal_owner` for profile,
/// platform join, and MyData.
///
/// **Hierarchy:** `parent_object_id` points up; no child list on parent. Max depth 8.
/// **Registry:** auth mirror [`Table`] on [`MemoryAccount`] keyed by `derived_address` (plus
/// object-id reverse index) so social/MyData PTBs need only [`MemoryAccount`], not ancestor refs.
/// **Lifecycle:** deactivate/revoke operate on explicit agent objects; subtree batches are
/// computed off-chain (indexer/server).
///
/// **`max_action_spend`:** optional per-transaction MYSO (MIST) ceiling for sub-agent signers.
///
/// ## Indexer event payloads
///
/// **SubAgentRegistered / SubAgentUpdated:** `account_id`, `principal_owner`, `profile_id`,
/// `agent_object_id`, `derived_address`, `label`, `identity_class`, `role_tags`, `capabilities`,
/// `delegatable_caps`, `register_scope`, `approval_required_caps`, `max_action_spend`, `platform_scope`,
/// `parent_object_id`, `depth`, `registered_by`, `expires_at`, `active`, `created_at`
///
/// **SubAgentDeactivated / SubAgentRevoked:** identifiers above + `agent_object_id`, `derived_address`
///
/// **SubAgentsClearedOnTransfer:** `account_id`, `principal_owner`, `profile_id`,
/// `previous_owner`, `new_owner`, `revoked_count`
///
/// **Social events** (post module): all include `actor_address`, `sub_agent_id` (agent object id),
/// `action_identity_class` and reactions add `principal_owner`.

#[allow(duplicate_alias, lint(public_entry), unused_const)]
module social_contracts::memory {
    use std::option::{Self, Option};
    use std::string::{Self, String};
    use std::vector;

    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        table::{Self, Table},
        clock::{Self, Clock},
        dynamic_field as df,
        package::{Self, UpgradeCap},
        bcs,
        event,
        derived_object,
    };

    // ============================================================
    // Identity classes
    // ============================================================

    const CLASS_HUMAN: u8 = 0;
    const CLASS_DELEGATED_AI: u8 = 1;
    const CLASS_ORGANIZATION: u8 = 2;

    // ============================================================
    // Register relation (agent-initiated registration)
    // ============================================================

    const REGISTER_CHILD: u8 = 1;
    const REGISTER_PEER: u8 = 2;

    const REGISTER_SCOPE_CHILD: u8 = 1;
    const REGISTER_SCOPE_PEER: u8 = 2;
    const REGISTER_SCOPE_BOTH: u8 = 3;

    // ============================================================
    // Capability bitmap
    // ============================================================

    const CAP_MEMORY_READ: u64 = 1;
    const CAP_MEMORY_WRITE: u64 = 2;
    const CAP_MYDATA_READ: u64 = 4;
    const CAP_POST_PUBLISH: u64 = 16;
    const CAP_MESSAGE_READ: u64 = 32;
    const CAP_MESSAGE_SEND: u64 = 64;
    const CAP_TRADE_MONITOR: u64 = 128;
    const CAP_TRADE_EXECUTE: u64 = 256;
    const CAP_COMMENT: u64 = 512;
    const CAP_REACT: u64 = 1024;
    const CAP_AGENT_REVOKE: u64 = 2048;
    const CAP_AGENT_UPDATE: u64 = 4096;
    const CAP_AGENT_REGISTER: u64 = 8192;
    const CAP_AI_SPEND: u64 = 16384;

    // Role tag bits (policy hooks / indexer display)
    const ROLE_EDITOR: u64 = 1;
    const ROLE_MODERATOR: u64 = 2;
    const ROLE_ORG_ADMIN: u64 = 4;

    // ============================================================
    // Agentic organization categories (exactly 14)
    // ============================================================

    const ORG_TYPE_COMPANY: u8 = 0;
    const ORG_TYPE_STARTUP: u8 = 1;
    const ORG_TYPE_INVESTMENT_FUND: u8 = 2;
    const ORG_TYPE_NONPROFIT: u8 = 3;
    const ORG_TYPE_RESEARCH: u8 = 4;
    const ORG_TYPE_GOVERNMENT: u8 = 5;
    const ORG_TYPE_MEDIA: u8 = 6;
    const ORG_TYPE_STEWARDSHIP: u8 = 7;
    const ORG_TYPE_BRAND: u8 = 8;
    const ORG_TYPE_COMMUNITY: u8 = 9;
    const ORG_TYPE_SPORTS: u8 = 10;
    const ORG_TYPE_EDUCATION: u8 = 11;
    const ORG_TYPE_HEALTHCARE: u8 = 12;
    const ORG_TYPE_OTHER: u8 = 13;
    const ORG_TYPE_COUNT: u8 = 14;

    const MAX_ORGANIZATIONS_PER_USER: u8 = 8;
    const ORG_CATEGORY_UPDATE_COOLDOWN_MS: u64 = 7 * 24 * 60 * 60 * 1000;

    // ============================================================
    // Error codes
    // ============================================================

    const ESubAgentNotFound: u64 = 1;
    const EAccountAlreadyExists: u64 = 3;
    const ENotOwner: u64 = 4;
    const EInvalidPublicKeyLength: u64 = 5;
    const EAccountDeactivated: u64 = 6;
    const EWrongVersion: u64 = 7;
    const ENotUpgradeAuthority: u64 = 8;
    const EAlreadyMigrated: u64 = 9;
    const ELabelTooLong: u64 = 10;
    const EAccountAlreadyActive: u64 = 11;
    const ENewOwnerHasMemoryAccount: u64 = 12;
    const ERegistryAccountMismatch: u64 = 13;
    const ESubAgentDuplicateDerivedAddress: u64 = 14;
    const ESubAgentNotActive: u64 = 15;
    const ESubAgentExpired: u64 = 16;
    const ESubAgentWrongPlatformScope: u64 = 17;
    const ESubAgentMissingCap: u64 = 18;
    const ESubAgentApprovalRequired: u64 = 19;
    const ESubAgentNotGlobalScope: u64 = 20;
    const EInvalidIdentityClass: u64 = 21;
    const EInvalidRegisterRelation: u64 = 22;
    const EInvalidRegisterScope: u64 = 23;
    const EAgentDepthExceeded: u64 = 24;
    const ECapsNotSubset: u64 = 25;
    const EScopeWidening: u64 = 26;
    const ENotRegistrarAncestor: u64 = 27;
    const EInvalidRegistrar: u64 = 28;
    const ESubAgentInactiveAncestor: u64 = 29;
    const ESubAgentSpendExceeded: u64 = 30;
    const ESubAgentAccountMismatch: u64 = 31;
    const ESubAgentWrongSigner: u64 = 32;
    const EInvalidAncestorChain: u64 = 33;
    const ECapEscalation: u64 = 34;
    const EOrganizationLimitExceeded: u64 = 35;
    const EInvalidOrgType: u64 = 36;
    const EOrganizationNotFound: u64 = 37;
    const EOrganizationNotActive: u64 = 38;
    const EOrganizationAccountMismatch: u64 = 39;
    const EOrganizationHasRoot: u64 = 40;
    const EOrganizationMissingRoot: u64 = 41;
    const EOrganizationOrgMismatch: u64 = 42;
    const EOrgCategoryUpdateCooldown: u64 = 43;
    const ENameTooLong: u64 = 44;
    const EDescriptionTooLong: u64 = 45;
    const ENoAccess: u64 = 100;

    const ED25519_PUBLIC_KEY_LENGTH: u64 = 32;
    const MAX_LABEL_LENGTH: u64 = 64;
    const MAX_ORG_NAME_LENGTH: u64 = 100;
    const MAX_ORG_DESCRIPTION_LENGTH: u64 = 1200;
    const MAX_AGENT_DEPTH: u8 = 8;

    const VERSION: u64 = 4;
    const VERSION_DF_KEY: vector<u8> = b"memory_version";

    // ============================================================
    // Structs
    // ============================================================

    public struct MemoryRegistry has key {
        id: UID,
        accounts: Table<address, ID>,
    }

    public struct SubAgentKey has copy, drop, store {
        derived_address: address,
    }

    public struct AgentMemoryVaultKey has copy, drop, store {}

    public struct SubAgentConstraints has store, copy, drop {
        approval_required_caps: u64,
        max_action_spend: Option<u64>,
    }

    public struct OrgRegistryEntry has store, copy, drop {
        active: bool,
    }

    /// Competitive agentic organization wrapper (one root-agent tree per org).
    public struct AgenticOrganization has key, store {
        id: UID,
        memory_account_id: ID,
        principal_owner: address,
        profile_id: address,
        name: Option<String>,
        description: Option<String>,
        org_type: u8,
        root_agent_id: Option<ID>,
        active: bool,
        created_at: u64,
        deactivated_at: Option<u64>,
        category_updated_at: Option<u64>,
    }

    /// Auth mirror for on-chain ancestor walks without PTB ancestor inputs.
    public struct AgentRegistryEntry has store, copy, drop {
        agent_object_id: ID,
        organization_id: ID,
        parent_object_id: Option<ID>,
        depth: u8,
        active: bool,
        expires_at: Option<u64>,
        identity_class: u8,
        capabilities: u64,
        delegatable_caps: u64,
        register_scope: u8,
        constraints: SubAgentConstraints,
        platform_scope: Option<address>,
    }

    /// Shared derived sub-agent object (one per agent).
    public struct SubAgent has key, store {
        id: UID,
        memory_account_id: ID,
        organization_id: ID,
        principal_owner: address,
        profile_id: address,
        derived_address: address,
        public_key: vector<u8>,
        label: String,
        identity_class: u8,
        role_tags: u64,
        capabilities: u64,
        delegatable_caps: u64,
        register_scope: u8,
        constraints: SubAgentConstraints,
        platform_scope: Option<address>,
        parent_object_id: Option<ID>,
        depth: u8,
        registered_by: address,
        created_at: u64,
        expires_at: Option<u64>,
        active: bool,
    }

    /// Lazy per-agent memory blob anchor (derived from [`SubAgent`]).
    public struct AgentMemoryVault has key {
        id: UID,
        agent_object_id: ID,
        memory_account_id: ID,
        created_at: u64,
    }

    public struct ActingContext has copy, drop {
        principal_owner: address,
        principal_profile_id: address,
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        identity_class: u8,
        parent_object_id: Option<ID>,
        depth: u8,
    }

    /// Human root plus on-chain agent auth index (shared [`SubAgent`] objects remain canonical).
    public struct MemoryAccount has key, store {
        id: UID,
        owner: address,
        profile_id: address,
        created_at: u64,
        active: bool,
        agents: Table<address, AgentRegistryEntry>,
        agent_ids: Table<ID, address>,
        organizations: Table<ID, OrgRegistryEntry>,
        org_count: u8,
    }

    // ============================================================
    // Events
    // ============================================================

    public struct MemoryAccountCreated has copy, drop {
        account_id: ID,
        owner: address,
        profile_id: address,
    }

    public struct SubAgentRegistered has copy, drop {
        account_id: ID,
        principal_owner: address,
        profile_id: address,
        organization_id: ID,
        agent_object_id: ID,
        derived_address: address,
        label: String,
        identity_class: u8,
        role_tags: u64,
        capabilities: u64,
        delegatable_caps: u64,
        register_scope: u8,
        approval_required_caps: u64,
        max_action_spend: Option<u64>,
        platform_scope: Option<address>,
        parent_object_id: Option<ID>,
        depth: u8,
        registered_by: address,
        expires_at: Option<u64>,
        active: bool,
        created_at: u64,
    }

    public struct SubAgentUpdated has copy, drop {
        account_id: ID,
        principal_owner: address,
        profile_id: address,
        organization_id: ID,
        agent_object_id: ID,
        derived_address: address,
        label: String,
        identity_class: u8,
        role_tags: u64,
        capabilities: u64,
        delegatable_caps: u64,
        register_scope: u8,
        approval_required_caps: u64,
        max_action_spend: Option<u64>,
        platform_scope: Option<address>,
        parent_object_id: Option<ID>,
        depth: u8,
        registered_by: address,
        expires_at: Option<u64>,
        active: bool,
        created_at: u64,
    }

    public struct SubAgentDeactivated has copy, drop {
        account_id: ID,
        principal_owner: address,
        profile_id: address,
        agent_object_id: ID,
        derived_address: address,
    }

    public struct SubAgentRevoked has copy, drop {
        account_id: ID,
        principal_owner: address,
        profile_id: address,
        agent_object_id: ID,
        derived_address: address,
    }

    public struct SubAgentsClearedOnTransfer has copy, drop {
        account_id: ID,
        principal_owner: address,
        profile_id: address,
        previous_owner: address,
        new_owner: address,
        revoked_count: u64,
    }

    public struct MemoryAccountDeactivated has copy, drop {
        account_id: ID,
        owner: address,
    }

    public struct MemoryAccountReactivated has copy, drop {
        account_id: ID,
        owner: address,
    }

    public struct MemoryAccountMigrated has copy, drop {
        account_id: ID,
        from: u64,
        to: u64,
    }

    public struct MemoryRegistryMigrated has copy, drop {
        registry_id: ID,
        from: u64,
        to: u64,
    }

    public struct AgentMemoryVaultCreated has copy, drop {
        vault_id: ID,
        agent_object_id: ID,
        memory_account_id: ID,
    }

    public struct AgenticOrganizationCreated has copy, drop {
        organization_id: ID,
        account_id: ID,
        principal_owner: address,
        profile_id: address,
        name: Option<String>,
        description: Option<String>,
        org_type: u8,
        created_at: u64,
    }

    public struct AgenticOrganizationUpdated has copy, drop {
        organization_id: ID,
        name: Option<String>,
        description: Option<String>,
    }

    public struct AgenticOrganizationCategoryUpdated has copy, drop {
        organization_id: ID,
        org_type: u8,
        previous_org_type: u8,
        updated_at: u64,
    }

    public struct AgenticOrganizationDeactivated has copy, drop {
        organization_id: ID,
        deactivated_at: u64,
    }

    // ============================================================
    // Public accessors
    // ============================================================

    public fun class_human(): u8 { CLASS_HUMAN }
    public fun class_delegated_ai(): u8 { CLASS_DELEGATED_AI }
    public fun class_organization(): u8 { CLASS_ORGANIZATION }

    public fun org_type_company(): u8 { ORG_TYPE_COMPANY }
    public fun org_type_startup(): u8 { ORG_TYPE_STARTUP }
    public fun org_type_investment_fund(): u8 { ORG_TYPE_INVESTMENT_FUND }
    public fun org_type_nonprofit(): u8 { ORG_TYPE_NONPROFIT }
    public fun org_type_research(): u8 { ORG_TYPE_RESEARCH }
    public fun org_type_government(): u8 { ORG_TYPE_GOVERNMENT }
    public fun org_type_media(): u8 { ORG_TYPE_MEDIA }
    public fun org_type_stewardship(): u8 { ORG_TYPE_STEWARDSHIP }
    public fun org_type_brand(): u8 { ORG_TYPE_BRAND }
    public fun org_type_community(): u8 { ORG_TYPE_COMMUNITY }
    public fun org_type_sports(): u8 { ORG_TYPE_SPORTS }
    public fun org_type_education(): u8 { ORG_TYPE_EDUCATION }
    public fun org_type_healthcare(): u8 { ORG_TYPE_HEALTHCARE }
    public fun org_type_other(): u8 { ORG_TYPE_OTHER }
    public fun org_type_count(): u8 { ORG_TYPE_COUNT }
    public fun max_organizations_per_user(): u8 { MAX_ORGANIZATIONS_PER_USER }

    public fun cap_memory_read(): u64 { CAP_MEMORY_READ }
    public fun cap_memory_write(): u64 { CAP_MEMORY_WRITE }
    public fun cap_mydata_read(): u64 { CAP_MYDATA_READ }
    public fun cap_post_publish(): u64 { CAP_POST_PUBLISH }
    public fun cap_comment(): u64 { CAP_COMMENT }
    public fun cap_react(): u64 { CAP_REACT }
    public fun cap_message_read(): u64 { CAP_MESSAGE_READ }
    public fun cap_message_send(): u64 { CAP_MESSAGE_SEND }
    public fun cap_trade_monitor(): u64 { CAP_TRADE_MONITOR }
    public fun cap_trade_execute(): u64 { CAP_TRADE_EXECUTE }
    public fun cap_agent_register(): u64 { CAP_AGENT_REGISTER }
    public fun cap_agent_revoke(): u64 { CAP_AGENT_REVOKE }
    public fun cap_agent_update(): u64 { CAP_AGENT_UPDATE }
    public fun cap_ai_spend(): u64 { CAP_AI_SPEND }

    public fun register_child(): u8 { REGISTER_CHILD }
    public fun register_peer(): u8 { REGISTER_PEER }

    public fun derive_sub_agent_address(account: &MemoryAccount, derived_address: address): address {
        derived_object::derive_address(object::id(account), SubAgentKey { derived_address })
    }

    public fun agent_object_id(agent: &SubAgent): ID {
        object::id(agent)
    }

    public fun organization_id(org: &AgenticOrganization): ID {
        object::id(org)
    }

    public fun sub_agent_organization_id(agent: &SubAgent): ID {
        agent.organization_id
    }

    public fun organization_org_type(org: &AgenticOrganization): u8 {
        org.org_type
    }

    public fun organization_root_agent_id(org: &AgenticOrganization): Option<ID> {
        org.root_agent_id
    }

    public fun organization_name(org: &AgenticOrganization): &Option<String> {
        &org.name
    }

    public fun organization_description(org: &AgenticOrganization): &Option<String> {
        &org.description
    }

    // ============================================================
    // Bootstrap
    // ============================================================

    public(package) fun bootstrap_init(_clock: &Clock, ctx: &mut TxContext) {
        let mut registry = MemoryRegistry {
            id: object::new(ctx),
            accounts: table::new(ctx),
        };
        set_version(&mut registry.id, VERSION);
        transfer::share_object(registry);
    }

    public(package) fun create_account_for_profile(
        registry: &mut MemoryRegistry,
        profile_id: address,
        clock: &Clock,
        ctx: &mut TxContext,
    ): ID {
        assert_object_version(&registry.id);

        let sender = tx_context::sender(ctx);
        assert!(!table::contains(&registry.accounts, sender), EAccountAlreadyExists);

        let mut account = MemoryAccount {
            id: object::new(ctx),
            owner: sender,
            profile_id,
            created_at: clock::timestamp_ms(clock),
            active: true,
            agents: table::new(ctx),
            agent_ids: table::new(ctx),
            organizations: table::new(ctx),
            org_count: 0,
        };
        set_version(&mut account.id, VERSION);

        let account_id = object::id(&account);
        table::add(&mut registry.accounts, sender, account_id);

        event::emit(MemoryAccountCreated {
            account_id,
            owner: sender,
            profile_id,
        });

        transfer::share_object(account);
        account_id
    }

    /// Profile transfer must revoke all sub-agent objects in the same PTB (via [`revoke_sub_agent`])
    /// before calling [`profile::transfer_profile_with_memory`].
    public(package) fun transfer_account_owner_with_profile(
        registry: &mut MemoryRegistry,
        account: &mut MemoryAccount,
        profile_id: address,
        old_owner: address,
        new_owner: address,
    ) {
        assert_object_version(&registry.id);
        assert_object_version(&account.id);
        assert!(account.owner == old_owner, ENotOwner);
        assert!(account.profile_id == profile_id, ENotOwner);
        assert!(table::contains(&registry.accounts, old_owner), ERegistryAccountMismatch);
        assert!(*table::borrow(&registry.accounts, old_owner) == object::id(account), ERegistryAccountMismatch);
        assert!(!table::contains(&registry.accounts, new_owner), ENewOwnerHasMemoryAccount);

        table::remove(&mut registry.accounts, old_owner);
        account.owner = new_owner;
        table::add(&mut registry.accounts, new_owner, object::id(account));
    }

    // ============================================================
    // Agentic organization lifecycle
    // ============================================================

    /// Human owner creates a competitive agentic organization (max 8 per account).
    public entry fun create_agentic_organization(
        account: &mut MemoryAccount,
        org_type: u8,
        name: Option<String>,
        description: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == account.owner, ENotOwner);
        let _ = create_agentic_organization_internal(account, org_type, name, description, clock, ctx);
    }

    public(package) fun create_agentic_organization_internal(
        account: &mut MemoryAccount,
        org_type: u8,
        name: Option<String>,
        description: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext,
    ): ID {
        assert_object_version(&account.id);
        assert!(account.active, EAccountDeactivated);
        assert_valid_org_type(org_type);
        assert_org_name_within_limit(&name);
        assert_org_description_within_limit(&description);
        assert!(account.org_count < MAX_ORGANIZATIONS_PER_USER, EOrganizationLimitExceeded);

        let org = AgenticOrganization {
            id: object::new(ctx),
            memory_account_id: object::id(account),
            principal_owner: account.owner,
            profile_id: account.profile_id,
            name,
            description,
            org_type,
            root_agent_id: option::none(),
            active: true,
            created_at: clock::timestamp_ms(clock),
            deactivated_at: option::none(),
            category_updated_at: option::none(),
        };
        let organization_id = object::id(&org);
        table::add(
            &mut account.organizations,
            organization_id,
            OrgRegistryEntry { active: true },
        );
        account.org_count = account.org_count + 1;

        event::emit(AgenticOrganizationCreated {
            organization_id,
            account_id: object::id(account),
            principal_owner: account.owner,
            profile_id: account.profile_id,
            name: org.name,
            description: org.description,
            org_type: org.org_type,
            created_at: org.created_at,
        });

        transfer::share_object(org);
        organization_id
    }

    public entry fun update_agentic_organization_metadata(
        account: &MemoryAccount,
        org: &mut AgenticOrganization,
        name: Option<String>,
        description: Option<String>,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);
        assert!(tx_context::sender(ctx) == account.owner, ENotOwner);
        assert_organization_belongs_to_account(account, org);
        assert!(org.active, EOrganizationNotActive);
        assert_org_name_within_limit(&name);
        assert_org_description_within_limit(&description);
        org.name = name;
        org.description = description;
        event::emit(AgenticOrganizationUpdated {
            organization_id: object::id(org),
            name: org.name,
            description: org.description,
        });
    }

    public entry fun update_agentic_organization_category(
        account: &MemoryAccount,
        org: &mut AgenticOrganization,
        org_type: u8,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);
        assert!(tx_context::sender(ctx) == account.owner, ENotOwner);
        assert_organization_belongs_to_account(account, org);
        assert!(org.active, EOrganizationNotActive);
        assert_valid_org_type(org_type);
        if (option::is_some(&org.category_updated_at)) {
            let last = *option::borrow(&org.category_updated_at);
            assert!(
                clock::timestamp_ms(clock) >= last + ORG_CATEGORY_UPDATE_COOLDOWN_MS,
                EOrgCategoryUpdateCooldown,
            );
        };
        let previous_org_type = org.org_type;
        org.org_type = org_type;
        org.category_updated_at = option::some(clock::timestamp_ms(clock));
        event::emit(AgenticOrganizationCategoryUpdated {
            organization_id: object::id(org),
            org_type,
            previous_org_type,
            updated_at: clock::timestamp_ms(clock),
        });
    }

    public entry fun deactivate_agentic_organization(
        account: &mut MemoryAccount,
        org: &mut AgenticOrganization,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);
        assert!(tx_context::sender(ctx) == account.owner, ENotOwner);
        assert_organization_belongs_to_account(account, org);
        if (!org.active) {
            return
        };
        org.active = false;
        org.deactivated_at = option::some(clock::timestamp_ms(clock));
        let organization_id = object::id(org);
        if (table::contains(&account.organizations, organization_id)) {
            let entry = table::borrow_mut(&mut account.organizations, organization_id);
            entry.active = false;
        };
        event::emit(AgenticOrganizationDeactivated {
            organization_id,
            deactivated_at: clock::timestamp_ms(clock),
        });
    }

    // ============================================================
    // Sub-agent lifecycle
    // ============================================================

    /// Human owner registers a root-level sub-agent bound to an organization.
    public entry fun register_sub_agent(
        account: &mut MemoryAccount,
        organization: &mut AgenticOrganization,
        public_key: vector<u8>,
        derived_address: address,
        label: String,
        identity_class: u8,
        role_tags: u64,
        capabilities: u64,
        delegatable_caps: u64,
        register_scope: u8,
        approval_required_caps: u64,
        max_action_spend: Option<u64>,
        platform_scope: Option<address>,
        expires_at: Option<u64>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == account.owner, ENotOwner);
        assert_organization_ready_for_root(account, organization);
        register_sub_agent_internal(
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

    /// Delegated agent registers a child or peer sub-agent.
    public entry fun register_sub_agent_delegated(
        account: &mut MemoryAccount,
        parent_agent: &SubAgent,
        public_key: vector<u8>,
        derived_address: address,
        label: String,
        identity_class: u8,
        role_tags: u64,
        capabilities: u64,
        delegatable_caps: u64,
        register_scope: u8,
        approval_required_caps: u64,
        max_action_spend: Option<u64>,
        platform_scope: Option<address>,
        expires_at: Option<u64>,
        register_relation: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == parent_agent.derived_address, EInvalidRegistrar);
        register_sub_agent_delegated_internal(
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

    public entry fun update_sub_agent(
        account: &mut MemoryAccount,
        agent: &mut SubAgent,
        identity_class: u8,
        role_tags: u64,
        capabilities: u64,
        delegatable_caps: u64,
        register_scope: u8,
        approval_required_caps: u64,
        max_action_spend: Option<u64>,
        platform_scope: Option<address>,
        expires_at: Option<u64>,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);
        assert_agent_belongs_to_account(account, agent);
        assert_valid_identity_class(identity_class);
        assert_valid_register_scope(register_scope);
        assert_may_manage(account, agent, CAP_AGENT_UPDATE, clock, ctx);
        assert_update_caps_monotonic(account, agent, capabilities, delegatable_caps, platform_scope, ctx);

        agent.identity_class = identity_class;
        agent.role_tags = role_tags;
        agent.capabilities = capabilities;
        agent.delegatable_caps = delegatable_caps;
        agent.register_scope = register_scope;
        agent.constraints = SubAgentConstraints {
            approval_required_caps,
            max_action_spend,
        };
        agent.platform_scope = platform_scope;
        agent.expires_at = expires_at;
        assert_sub_agent_not_expired(agent, clock);

        sync_registry_from_agent(account, agent);
        emit_sub_agent_updated(account, agent);
    }

    public entry fun update_sub_agent_label(
        account: &mut MemoryAccount,
        agent: &mut SubAgent,
        label: String,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);
        assert_agent_belongs_to_account(account, agent);
        assert!(string::length(&label) <= MAX_LABEL_LENGTH, ELabelTooLong);
        assert_may_manage(account, agent, CAP_AGENT_UPDATE, clock, ctx);
        agent.label = label;
        emit_sub_agent_updated(account, agent);
    }

    public entry fun deactivate_sub_agent(
        account: &mut MemoryAccount,
        agent: &mut SubAgent,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);
        assert_agent_belongs_to_account(account, agent);
        assert_may_manage(account, agent, CAP_AGENT_REVOKE, clock, ctx);
        if (!agent.active) {
            return
        };
        agent.active = false;
        sync_registry_active(account, agent.derived_address, false);
        emit_sub_agent_deactivated(account, agent);
    }

    public entry fun revoke_sub_agent(
        account: &mut MemoryAccount,
        agent: SubAgent,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);
        assert_agent_belongs_to_account(account, &agent);
        assert_may_manage(account, &agent, CAP_AGENT_REVOKE, clock, ctx);
        let derived_address = agent.derived_address;
        let agent_object_id = object::id(&agent);
        emit_sub_agent_revoked(account, &agent);
        remove_registry_entry(account, derived_address, agent_object_id);
        destroy_sub_agent(agent);
    }

    /// Emit bulk-clear audit after the last agent revoke during profile transfer orchestration.
    public(package) fun emit_sub_agents_cleared_on_transfer(
        account: &MemoryAccount,
        previous_owner: address,
        new_owner: address,
        revoked_count: u64,
    ) {
        event::emit(SubAgentsClearedOnTransfer {
            account_id: object::id(account),
            principal_owner: new_owner,
            profile_id: account.profile_id,
            previous_owner,
            new_owner,
            revoked_count,
        });
    }

    /// Lazy-create per-agent memory vault derived from the sub-agent object.
    public entry fun ensure_agent_memory_vault(
        account: &MemoryAccount,
        agent: &mut SubAgent,
        clock: &Clock,
        _ctx: &TxContext,
    ): ID {
        assert_object_version(&account.id);
        assert_agent_belongs_to_account(account, agent);
        assert_sub_agent_active(agent, clock);

        let key = AgentMemoryVaultKey {};
        if (derived_object::exists(&agent.id, key)) {
            let addr = derived_object::derive_address(object::id(agent), key);
            return addr.to_id()
        };

        let vault_uid = derived_object::claim(&mut agent.id, key);
        let vault_id = object::uid_to_inner(&vault_uid);
        let vault = AgentMemoryVault {
            id: vault_uid,
            agent_object_id: object::id(agent),
            memory_account_id: object::id(account),
            created_at: clock::timestamp_ms(clock),
        };
        event::emit(AgentMemoryVaultCreated {
            vault_id,
            agent_object_id: object::id(agent),
            memory_account_id: object::id(account),
        });
        transfer::share_object(vault);
        vault_id
    }

    // ============================================================
    // Activation
    // ============================================================

    public entry fun deactivate_account(account: &mut MemoryAccount, ctx: &TxContext) {
        assert_object_version(&account.id);
        assert!(account.owner == tx_context::sender(ctx), ENotOwner);
        assert!(account.active, EAccountDeactivated);
        account.active = false;

        event::emit(MemoryAccountDeactivated {
            account_id: object::id(account),
            owner: account.owner,
        });
    }

    public entry fun reactivate_account(account: &mut MemoryAccount, ctx: &TxContext) {
        assert_object_version(&account.id);
        assert!(account.owner == tx_context::sender(ctx), ENotOwner);
        assert!(!account.active, EAccountAlreadyActive);
        account.active = true;

        event::emit(MemoryAccountReactivated {
            account_id: object::id(account),
            owner: account.owner,
        });
    }

    // ============================================================
    // Migration
    // ============================================================

    public entry fun migrate_account(account: &mut MemoryAccount, ctx: &mut TxContext) {
        assert!(account.owner == tx_context::sender(ctx), ENotOwner);
        let cur = get_version(&account.id);
        assert!(cur < VERSION, EAlreadyMigrated);
        let _ = ctx;
        bump_version(&mut account.id, VERSION);

        event::emit(MemoryAccountMigrated {
            account_id: object::id(account),
            from: cur,
            to: VERSION,
        });
    }

    public entry fun admin_migrate_account(cap: &UpgradeCap, account: &mut MemoryAccount, ctx: &mut TxContext) {
        assert_cap_for_this_package(cap);
        let cur = get_version(&account.id);
        assert!(cur < VERSION, EAlreadyMigrated);
        let _ = ctx;
        bump_version(&mut account.id, VERSION);

        event::emit(MemoryAccountMigrated {
            account_id: object::id(account),
            from: cur,
            to: VERSION,
        });
    }

    public entry fun migrate_registry(cap: &UpgradeCap, registry: &mut MemoryRegistry) {
        assert_cap_for_this_package(cap);
        let cur = get_version(&registry.id);
        assert!(cur < VERSION, EAlreadyMigrated);
        bump_version(&mut registry.id, VERSION);

        event::emit(MemoryRegistryMigrated {
            registry_id: object::id(registry),
            from: cur,
            to: VERSION,
        });
    }

    // ============================================================
    // Auth helpers
    // ============================================================

    public fun resolve_human_actor(root: &MemoryAccount, ctx: &TxContext): ActingContext {
        assert_object_version(&root.id);
        assert!(root.active, EAccountDeactivated);
        assert!(tx_context::sender(ctx) == root.owner, ENoAccess);
        ActingContext {
            principal_owner: root.owner,
            principal_profile_id: root.profile_id,
            actor_address: root.owner,
            sub_agent_id: option::none(),
            organization_id: option::none(),
            identity_class: CLASS_HUMAN,
            parent_object_id: option::none(),
            depth: 0,
        }
    }

    public fun resolve_actor_from_account(
        root: &MemoryAccount,
        clock: &Clock,
        ctx: &TxContext,
    ): ActingContext {
        assert_object_version(&root.id);
        assert!(root.active, EAccountDeactivated);
        let sender = tx_context::sender(ctx);
        if (sender == root.owner) {
            return resolve_human_actor(root, ctx)
        };
        assert!(table::contains(&root.agents, sender), ESubAgentNotFound);
        assert_ancestor_chain_active_from_table(root, sender, clock);
        let entry = table::borrow(&root.agents, sender);
        ActingContext {
            principal_owner: root.owner,
            principal_profile_id: root.profile_id,
            actor_address: sender,
            sub_agent_id: option::some(entry.agent_object_id),
            organization_id: option::some(entry.organization_id),
            identity_class: entry.identity_class,
            parent_object_id: entry.parent_object_id,
            depth: entry.depth,
        }
    }

    public fun resolve_actor_with_cap(
        root: &MemoryAccount,
        required_cap: u64,
        action_platform_id: Option<address>,
        spend_amount: u64,
        clock: &Clock,
        ctx: &TxContext,
    ): ActingContext {
        let sender = tx_context::sender(ctx);
        if (sender == root.owner) {
            return resolve_human_actor(root, ctx)
        };
        let acting = resolve_actor_from_account(root, clock, ctx);
        let entry = table::borrow(&root.agents, sender);
        assert!(has_cap(entry.capabilities, required_cap), ESubAgentMissingCap);
        assert_platform_scope_entry(entry, action_platform_id);
        assert_action_spend_limit_from_entry(root, entry, spend_amount, ctx);
        acting
    }

    public fun assert_human_actor_with_cap(
        root: &MemoryAccount,
        ctx: &TxContext,
    ): ActingContext {
        resolve_human_actor(root, ctx)
    }

    /// Per-transaction MYSO (MIST) spend ceiling for sub-agents. Principal owner is exempt.
    public fun assert_action_spend_limit(
        root: &MemoryAccount,
        spend_amount: u64,
        ctx: &TxContext,
    ) {
        let caller = tx_context::sender(ctx);
        if (caller == root.owner) {
            return
        };
        assert!(table::contains(&root.agents, caller), ESubAgentNotFound);
        let entry = table::borrow(&root.agents, caller);
        assert_action_spend_limit_from_entry(root, entry, spend_amount, ctx);
    }

    public fun assert_direct_execution_allowed(
        root: &MemoryAccount,
        required_cap: u64,
        ctx: &TxContext,
    ) {
        let caller = tx_context::sender(ctx);
        if (caller == root.owner) {
            return
        };
        assert!(table::contains(&root.agents, caller), ESubAgentNotFound);
        let entry = table::borrow(&root.agents, caller);
        assert!(!cap_requires_approval(&entry.constraints, required_cap), ESubAgentApprovalRequired);
    }

    public fun assert_platform_scope_entry(
        entry: &AgentRegistryEntry,
        action_platform_id: Option<address>,
    ) {
        if (option::is_none(&entry.platform_scope)) {
            return
        };
        let scope = *option::borrow(&entry.platform_scope);
        assert!(option::is_some(&action_platform_id), ESubAgentWrongPlatformScope);
        assert!(*option::borrow(&action_platform_id) == scope, ESubAgentWrongPlatformScope);
    }

    public fun assert_platform_scope(agent: &SubAgent, action_platform_id: Option<address>) {
        if (option::is_none(&agent.platform_scope)) {
            return
        };
        let scope = *option::borrow(&agent.platform_scope);
        assert!(option::is_some(&action_platform_id), ESubAgentWrongPlatformScope);
        assert!(*option::borrow(&action_platform_id) == scope, ESubAgentWrongPlatformScope);
    }

    public fun assert_sub_agent_active(agent: &SubAgent, clock: &Clock) {
        assert!(agent.active, ESubAgentNotActive);
        assert_sub_agent_not_expired(agent, clock);
    }

    // ============================================================
    // Views
    // ============================================================

    public fun profile_id(account: &MemoryAccount): address { account.profile_id }
    public fun owner(account: &MemoryAccount): address { account.owner }

    public fun sub_agent_derived_address(agent: &SubAgent): address { agent.derived_address }
    public fun sub_agent_capabilities(agent: &SubAgent): u64 { agent.capabilities }
    public fun sub_agent_platform_scope(agent: &SubAgent): Option<address> { agent.platform_scope }
    public fun sub_agent_active(agent: &SubAgent): bool { agent.active }
    public fun sub_agent_depth(agent: &SubAgent): u8 { agent.depth }
    public fun sub_agent_parent_object_id(agent: &SubAgent): Option<ID> { agent.parent_object_id }
    public fun sub_agent_memory_account_id(agent: &SubAgent): ID { agent.memory_account_id }

    public fun acting_principal_owner(acting: &ActingContext): address { acting.principal_owner }
    public fun acting_profile_id(acting: &ActingContext): address { acting.principal_profile_id }
    public fun acting_actor_address(acting: &ActingContext): address { acting.actor_address }
    public fun acting_sub_agent_id(acting: &ActingContext): Option<ID> { acting.sub_agent_id }
    public fun acting_identity_class(acting: &ActingContext): u8 { acting.identity_class }
    public fun acting_parent_object_id(acting: &ActingContext): Option<ID> { acting.parent_object_id }
    public fun acting_depth(acting: &ActingContext): u8 { acting.depth }

    public fun acting_organization_id(acting: &ActingContext): Option<ID> { acting.organization_id }

    public fun has_account(registry: &MemoryRegistry, addr: address): bool {
        table::contains(&registry.accounts, addr)
    }

    public fun account_id_for_owner(registry: &MemoryRegistry, owner: address): Option<ID> {
        if (table::contains(&registry.accounts, owner)) {
            option::some(*table::borrow(&registry.accounts, owner))
        } else {
            option::none()
        }
    }

    public fun is_registered_agent(account: &MemoryAccount, derived: address): bool {
        table::contains(&account.agents, derived)
    }

    public fun is_active(account: &MemoryAccount): bool { account.active }

    public fun account_version(account: &MemoryAccount): u64 { get_version(&account.id) }
    public fun registry_version(registry: &MemoryRegistry): u64 { get_version(&registry.id) }
    public fun current_contract_version(): u64 { VERSION }

    // ============================================================
    // Key-server access policies
    // ============================================================

    public entry fun approve_key_policy(
        id: vector<u8>,
        account: &MemoryAccount,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);
        assert!(account.active, EAccountDeactivated);

        let caller = tx_context::sender(ctx);
        let owner_bytes = bcs::to_bytes(&account.owner);
        if ((caller == account.owner) && has_suffix(&id, &owner_bytes)) {
            return
        };

        if (table::contains(&account.agents, caller)) {
            let entry = table::borrow(&account.agents, caller);
            assert!(option::is_none(&entry.platform_scope), ESubAgentNotGlobalScope);
        };

        let acting = resolve_actor_with_cap(
            account,
            CAP_MEMORY_READ,
            option::none(),
            0,
            clock,
            ctx,
        );
        assert!(option::is_some(&acting.sub_agent_id), ENoAccess);
    }

    public entry fun approve_key_write_policy(
        id: vector<u8>,
        account: &MemoryAccount,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        assert_object_version(&account.id);
        assert!(account.active, EAccountDeactivated);

        let caller = tx_context::sender(ctx);
        let owner_bytes = bcs::to_bytes(&account.owner);
        if ((caller == account.owner) && has_suffix(&id, &owner_bytes)) {
            return
        };

        if (table::contains(&account.agents, caller)) {
            let entry = table::borrow(&account.agents, caller);
            assert!(option::is_none(&entry.platform_scope), ESubAgentNotGlobalScope);
        };

        let acting = resolve_actor_with_cap(
            account,
            CAP_MEMORY_WRITE,
            option::none(),
            0,
            clock,
            ctx,
        );
        assert!(option::is_some(&acting.sub_agent_id), ENoAccess);
    }

    public fun owner_key_suffix_bytes(owner_addr: address): vector<u8> {
        bcs::to_bytes(&owner_addr)
    }

    // ============================================================
    // Internal helpers
    // ============================================================

    fun register_sub_agent_internal(
        account: &mut MemoryAccount,
        organization: &mut AgenticOrganization,
        public_key: vector<u8>,
        derived_address: address,
        label: String,
        identity_class: u8,
        role_tags: u64,
        capabilities: u64,
        delegatable_caps: u64,
        register_scope: u8,
        approval_required_caps: u64,
        max_action_spend: Option<u64>,
        platform_scope: Option<address>,
        expires_at: Option<u64>,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        let organization_id = object::id(organization);
        let agent_id = finish_register_sub_agent(
            account,
            organization_id,
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
        bind_root_agent_to_organization(account, organization, agent_id);
    }

    fun register_sub_agent_delegated_internal(
        account: &mut MemoryAccount,
        parent_agent: &SubAgent,
        public_key: vector<u8>,
        derived_address: address,
        label: String,
        identity_class: u8,
        role_tags: u64,
        capabilities: u64,
        delegatable_caps: u64,
        register_scope: u8,
        approval_required_caps: u64,
        max_action_spend: Option<u64>,
        platform_scope: Option<address>,
        expires_at: Option<u64>,
        register_relation: u8,
        clock: &Clock,
    ) {
        let (depth, parent_object_id, registered_by) = resolve_delegated_registration_placement(
            account,
            parent_agent,
            register_relation,
            capabilities,
            delegatable_caps,
            platform_scope,
            clock,
        );
        finish_register_sub_agent(
            account,
            parent_agent.organization_id,
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

    fun finish_register_sub_agent(
        account: &mut MemoryAccount,
        organization_id: ID,
        public_key: vector<u8>,
        derived_address: address,
        label: String,
        identity_class: u8,
        role_tags: u64,
        capabilities: u64,
        delegatable_caps: u64,
        register_scope: u8,
        approval_required_caps: u64,
        max_action_spend: Option<u64>,
        platform_scope: Option<address>,
        expires_at: Option<u64>,
        depth: u8,
        parent_object_id: Option<ID>,
        registered_by: address,
        clock: &Clock,
    ): ID {
        assert_object_version(&account.id);
        assert!(account.active, EAccountDeactivated);
        assert_valid_identity_class(identity_class);
        assert_valid_register_scope(register_scope);
        assert!(table::contains(&account.organizations, organization_id), EOrganizationNotFound);
        assert!(vector::length(&public_key) == ED25519_PUBLIC_KEY_LENGTH, EInvalidPublicKeyLength);
        assert!(string::length(&label) <= MAX_LABEL_LENGTH, ELabelTooLong);
        assert_scope_allowed_for_delegate(option::none(), platform_scope);

        let key = SubAgentKey { derived_address };
        assert!(!derived_object::exists(&account.id, key), ESubAgentDuplicateDerivedAddress);

        let constraints = SubAgentConstraints {
            approval_required_caps,
            max_action_spend,
        };

        let agent_uid = derived_object::claim(&mut account.id, key);
        let agent = SubAgent {
            id: agent_uid,
            memory_account_id: object::id(account),
            organization_id,
            principal_owner: account.owner,
            profile_id: account.profile_id,
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
            active: true,
        };

        emit_sub_agent_registered(account, &agent);
        insert_registry_entry(account, &agent);
        let agent_id = object::id(&agent);
        transfer::share_object(agent);
        agent_id
    }

    fun bind_root_agent_to_organization(
        account: &MemoryAccount,
        org: &mut AgenticOrganization,
        agent_id: ID,
    ) {
        assert_organization_belongs_to_account(account, org);
        assert!(org.active, EOrganizationNotActive);
        assert!(option::is_none(&org.root_agent_id), EOrganizationHasRoot);
        org.root_agent_id = option::some(agent_id);
    }

    fun registry_entry_from_agent(agent: &SubAgent): AgentRegistryEntry {
        AgentRegistryEntry {
            agent_object_id: object::id(agent),
            organization_id: agent.organization_id,
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

    fun insert_registry_entry(account: &mut MemoryAccount, agent: &SubAgent) {
        let derived_address = agent.derived_address;
        let agent_object_id = object::id(agent);
        assert!(!table::contains(&account.agents, derived_address), ESubAgentDuplicateDerivedAddress);
        assert!(!table::contains(&account.agent_ids, agent_object_id), ESubAgentDuplicateDerivedAddress);
        table::add(
            &mut account.agents,
            derived_address,
            registry_entry_from_agent(agent),
        );
        table::add(&mut account.agent_ids, agent_object_id, derived_address);
    }

    fun sync_registry_from_agent(account: &mut MemoryAccount, agent: &SubAgent) {
        let derived_address = agent.derived_address;
        assert!(table::contains(&account.agents, derived_address), ESubAgentNotFound);
        let entry = table::borrow_mut(&mut account.agents, derived_address);
        *entry = registry_entry_from_agent(agent);
    }

    fun sync_registry_active(account: &mut MemoryAccount, derived_address: address, active: bool) {
        assert!(table::contains(&account.agents, derived_address), ESubAgentNotFound);
        let entry = table::borrow_mut(&mut account.agents, derived_address);
        entry.active = active;
    }

    fun remove_registry_entry(
        account: &mut MemoryAccount,
        derived_address: address,
        agent_object_id: ID,
    ) {
        if (table::contains(&account.agents, derived_address)) {
            let _entry = table::remove(&mut account.agents, derived_address);
        };
        if (table::contains(&account.agent_ids, agent_object_id)) {
            let _derived = table::remove(&mut account.agent_ids, agent_object_id);
        };
    }

    fun assert_registry_entry_active(entry: &AgentRegistryEntry, clock: &Clock) {
        assert!(entry.active, ESubAgentNotActive);
        if (option::is_some(&entry.expires_at)) {
            assert!(
                clock::timestamp_ms(clock) <= *option::borrow(&entry.expires_at),
                ESubAgentExpired,
            );
        };
    }

    fun assert_action_spend_limit_from_entry(
        root: &MemoryAccount,
        entry: &AgentRegistryEntry,
        spend_amount: u64,
        ctx: &TxContext,
    ) {
        let caller = tx_context::sender(ctx);
        if (caller == root.owner) {
            return
        };
        if (option::is_none(&entry.constraints.max_action_spend)) {
            return
        };
        let max = *option::borrow(&entry.constraints.max_action_spend);
        assert!(spend_amount <= max, ESubAgentSpendExceeded);
    }

    fun assert_ancestor_chain_active_from_table(
        account: &MemoryAccount,
        derived_address: address,
        clock: &Clock,
    ) {
        assert!(table::contains(&account.agents, derived_address), ESubAgentNotFound);
        let entry = table::borrow(&account.agents, derived_address);
        assert_registry_entry_active(entry, clock);

        let mut current_parent = entry.parent_object_id;
        let mut hops = 0u8;
        while (option::is_some(&current_parent)) {
            hops = hops + 1;
            assert!(hops <= MAX_AGENT_DEPTH, EInvalidAncestorChain);
            let parent_id = *option::borrow(&current_parent);
            assert!(table::contains(&account.agent_ids, parent_id), ESubAgentInactiveAncestor);
            let parent_derived = *table::borrow(&account.agent_ids, parent_id);
            let parent_entry = table::borrow(&account.agents, parent_derived);
            assert_registry_entry_active(parent_entry, clock);
            current_parent = parent_entry.parent_object_id;
        };
    }

    fun resolve_delegated_registration_placement(
        account: &MemoryAccount,
        parent: &SubAgent,
        register_relation: u8,
        capabilities: u64,
        delegatable_caps: u64,
        platform_scope: Option<address>,
        clock: &Clock,
    ): (u8, Option<ID>, address) {
        assert_agent_belongs_to_account(account, parent);
        assert_sub_agent_active(parent, clock);
        assert!(has_cap(parent.capabilities, CAP_AGENT_REGISTER), ESubAgentMissingCap);
        assert_caps_subset(capabilities, parent.delegatable_caps);
        assert_caps_subset(delegatable_caps, parent.delegatable_caps);
        assert_scope_allowed_for_delegate(parent.platform_scope, platform_scope);

        let sender = parent.derived_address;
        if (register_relation == REGISTER_CHILD) {
            assert!(
                parent.register_scope == REGISTER_SCOPE_CHILD
                    || parent.register_scope == REGISTER_SCOPE_BOTH,
                EInvalidRegisterRelation,
            );
            let depth = parent.depth + 1;
            assert!(depth <= MAX_AGENT_DEPTH, EAgentDepthExceeded);
            (depth, option::some(object::id(parent)), sender)
        } else if (register_relation == REGISTER_PEER) {
            assert!(
                parent.register_scope == REGISTER_SCOPE_PEER
                    || parent.register_scope == REGISTER_SCOPE_BOTH,
                EInvalidRegisterRelation,
            );
            (parent.depth, parent.parent_object_id, sender)
        } else {
            abort EInvalidRegisterRelation
        }
    }

    fun assert_may_manage(
        account: &MemoryAccount,
        target: &SubAgent,
        required_cap: u64,
        clock: &Clock,
        ctx: &TxContext,
    ) {
        let sender = tx_context::sender(ctx);
        if (sender == account.owner) {
            return
        };
        assert!(table::contains(&account.agents, sender), ESubAgentNotFound);
        let registrar_entry = table::borrow(&account.agents, sender);
        assert_registry_entry_active(registrar_entry, clock);
        assert!(has_cap(registrar_entry.capabilities, required_cap), ESubAgentMissingCap);
        assert_registrar_is_ancestor_from_table(account, sender, target);
    }

    fun assert_registrar_is_ancestor_from_table(
        account: &MemoryAccount,
        registrar_derived: address,
        target: &SubAgent,
    ) {
        if (registrar_derived == target.derived_address) {
            return
        };
        assert!(table::contains(&account.agents, target.derived_address), ESubAgentNotFound);
        let mut current_parent = target.parent_object_id;
        while (option::is_some(&current_parent)) {
            let parent_id = *option::borrow(&current_parent);
            assert!(table::contains(&account.agent_ids, parent_id), ENotRegistrarAncestor);
            let parent_derived = *table::borrow(&account.agent_ids, parent_id);
            if (parent_derived == registrar_derived) {
                return
            };
            let parent_entry = table::borrow(&account.agents, parent_derived);
            current_parent = parent_entry.parent_object_id;
        };
        abort ENotRegistrarAncestor
    }

    fun assert_update_caps_monotonic(
        account: &MemoryAccount,
        agent: &SubAgent,
        new_capabilities: u64,
        new_delegatable_caps: u64,
        new_platform_scope: Option<address>,
        ctx: &TxContext,
    ) {
        if (tx_context::sender(ctx) == account.owner) {
            return
        };
        assert_caps_subset(new_capabilities, agent.capabilities);
        assert_caps_subset(new_delegatable_caps, agent.delegatable_caps);
        assert_scope_allowed_for_delegate(agent.platform_scope, new_platform_scope);
    }

    fun assert_agent_belongs_to_account(account: &MemoryAccount, agent: &SubAgent) {
        assert!(agent.memory_account_id == object::id(account), ESubAgentAccountMismatch);
        assert!(agent.principal_owner == account.owner, ESubAgentAccountMismatch);
        assert!(agent.profile_id == account.profile_id, ESubAgentAccountMismatch);
    }

    fun destroy_sub_agent(agent: SubAgent) {
        let SubAgent { id, .. } = agent;
        object::delete(id);
    }

    fun assert_caps_subset(candidate: u64, allowed: u64) {
        assert!((candidate & allowed) == candidate, ECapsNotSubset);
    }

    fun assert_scope_allowed_for_delegate(
        parent_scope: Option<address>,
        child_scope: Option<address>,
    ) {
        if (option::is_none(&parent_scope)) {
            return
        };
        assert!(option::is_some(&child_scope), EScopeWidening);
        assert!(
            *option::borrow(&parent_scope) == *option::borrow(&child_scope),
            EScopeWidening,
        );
    }

    fun assert_valid_register_scope(register_scope: u8) {
        assert!(
            register_scope == REGISTER_SCOPE_CHILD
                || register_scope == REGISTER_SCOPE_PEER
                || register_scope == REGISTER_SCOPE_BOTH,
            EInvalidRegisterScope,
        );
    }

    fun assert_sub_agent_not_expired(agent: &SubAgent, clock: &Clock) {
        if (option::is_none(&agent.expires_at)) {
            return
        };
        assert!(clock::timestamp_ms(clock) <= *option::borrow(&agent.expires_at), ESubAgentExpired);
    }

    fun assert_valid_identity_class(identity_class: u8) {
        assert!(
            identity_class == CLASS_HUMAN
                || identity_class == CLASS_DELEGATED_AI
                || identity_class == CLASS_ORGANIZATION,
            EInvalidIdentityClass,
        );
    }

    fun assert_org_name_within_limit(name: &Option<String>) {
        if (option::is_some(name)) {
            assert!(
                string::length(option::borrow(name)) <= MAX_ORG_NAME_LENGTH,
                ENameTooLong,
            );
        };
    }

    fun assert_org_description_within_limit(description: &Option<String>) {
        if (option::is_some(description)) {
            assert!(
                string::length(option::borrow(description)) <= MAX_ORG_DESCRIPTION_LENGTH,
                EDescriptionTooLong,
            );
        };
    }

    fun assert_valid_org_type(org_type: u8) {
        assert!(org_type < ORG_TYPE_COUNT, EInvalidOrgType);
    }

    fun assert_organization_belongs_to_account(
        account: &MemoryAccount,
        org: &AgenticOrganization,
    ) {
        assert!(org.memory_account_id == object::id(account), EOrganizationAccountMismatch);
        assert!(org.principal_owner == account.owner, EOrganizationAccountMismatch);
        assert!(org.profile_id == account.profile_id, EOrganizationAccountMismatch);
        assert!(
            table::contains(&account.organizations, object::id(org)),
            EOrganizationNotFound,
        );
    }

    fun assert_organization_ready_for_root(
        account: &MemoryAccount,
        org: &AgenticOrganization,
    ) {
        assert_organization_belongs_to_account(account, org);
        assert!(org.active, EOrganizationNotActive);
        assert!(option::is_none(&org.root_agent_id), EOrganizationHasRoot);
    }

    public(package) fun has_cap(capabilities: u64, required_cap: u64): bool {
        (capabilities & required_cap) == required_cap
    }

    fun cap_requires_approval(constraints: &SubAgentConstraints, cap: u64): bool {
        (constraints.approval_required_caps & cap) == cap
    }

    fun emit_sub_agent_registered(account: &MemoryAccount, agent: &SubAgent) {
        event::emit(SubAgentRegistered {
            account_id: object::id(account),
            principal_owner: account.owner,
            profile_id: account.profile_id,
            organization_id: agent.organization_id,
            agent_object_id: object::id(agent),
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

    fun emit_sub_agent_updated(account: &MemoryAccount, agent: &SubAgent) {
        event::emit(SubAgentUpdated {
            account_id: object::id(account),
            principal_owner: account.owner,
            profile_id: account.profile_id,
            organization_id: agent.organization_id,
            agent_object_id: object::id(agent),
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

    fun emit_sub_agent_deactivated(account: &MemoryAccount, agent: &SubAgent) {
        event::emit(SubAgentDeactivated {
            account_id: object::id(account),
            principal_owner: account.owner,
            profile_id: account.profile_id,
            agent_object_id: object::id(agent),
            derived_address: agent.derived_address,
        });
    }

    fun emit_sub_agent_revoked(account: &MemoryAccount, agent: &SubAgent) {
        event::emit(SubAgentRevoked {
            account_id: object::id(account),
            principal_owner: account.owner,
            profile_id: account.profile_id,
            agent_object_id: object::id(agent),
            derived_address: agent.derived_address,
        });
    }

    fun get_version(uid: &UID): u64 {
        if (df::exists_with_type<vector<u8>, u64>(uid, VERSION_DF_KEY)) {
            *df::borrow<vector<u8>, u64>(uid, VERSION_DF_KEY)
        } else {
            1
        }
    }

    fun set_version(uid: &mut UID, v: u64) {
        if (df::exists_with_type<vector<u8>, u64>(uid, VERSION_DF_KEY)) {
            let r = df::borrow_mut<vector<u8>, u64>(uid, VERSION_DF_KEY);
            *r = v;
        } else {
            df::add(uid, VERSION_DF_KEY, v);
        }
    }

    fun bump_version(uid: &mut UID, v: u64) {
        set_version(uid, v)
    }

    fun assert_object_version(uid: &UID) {
        assert!(get_version(uid) == VERSION, EWrongVersion);
    }

    fun assert_cap_for_this_package(cap: &UpgradeCap) {
        let cap_pkg = package::upgrade_package(cap);
        assert!(object::id_to_address(&cap_pkg) == @social_contracts, ENotUpgradeAuthority);
    }

    fun has_suffix(data: &vector<u8>, suffix: &vector<u8>): bool {
        let data_len = vector::length(data);
        let suffix_len = vector::length(suffix);
        if (suffix_len > data_len) return false;
        let offset = data_len - suffix_len;
        let mut i = 0;
        while (i < suffix_len) {
            if (*vector::borrow(data, offset + i) != *vector::borrow(suffix, i)) return false;
            i = i + 1;
        };
        true
    }

    // ============================================================
    // Test helpers
    // ============================================================

    #[test_only]
    public fun test_create_agentic_organization(
        account: &mut MemoryAccount,
        org_type: u8,
        name: Option<String>,
        description: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext,
    ): ID {
        create_agentic_organization_internal(account, org_type, name, description, clock, ctx)
    }

    #[test_only]
    public fun error_name_too_long(): u64 { ENameTooLong }

    #[test_only]
    public fun error_description_too_long(): u64 { EDescriptionTooLong }

    #[test_only]
    public fun max_org_description_length(): u64 { MAX_ORG_DESCRIPTION_LENGTH }

    #[test_only]
    public fun error_organization_limit_exceeded(): u64 { EOrganizationLimitExceeded }

    #[test_only]
    public fun error_organization_has_root(): u64 { EOrganizationHasRoot }

    #[test_only]
    public fun error_invalid_org_type(): u64 { EInvalidOrgType }

    #[test_only]
    public fun test_bootstrap_init(clock: &Clock, ctx: &mut TxContext) {
        bootstrap_init(clock, ctx);
    }

    #[test_only]
    public fun test_make_upgrade_cap(ctx: &mut TxContext): UpgradeCap {
        package::test_publish(object::id_from_address(@social_contracts), ctx)
    }

    #[test_only]
    public fun test_make_foreign_upgrade_cap(ctx: &mut TxContext): UpgradeCap {
        package::test_publish(object::id_from_address(@0xBADBAD), ctx)
    }

    #[test_only]
    public fun test_force_account_version(account: &mut MemoryAccount, v: u64) {
        set_version(&mut account.id, v);
    }

    #[test_only]
    public fun test_force_registry_version(registry: &mut MemoryRegistry, v: u64) {
        set_version(&mut registry.id, v);
    }

    #[test_only]
    public fun test_strip_account_version(account: &mut MemoryAccount) {
        if (df::exists_with_type<vector<u8>, u64>(&account.id, VERSION_DF_KEY)) {
            let _: u64 = df::remove(&mut account.id, VERSION_DF_KEY);
        }
    }

    #[test_only]
    public fun test_strip_registry_version(registry: &mut MemoryRegistry) {
        if (df::exists_with_type<vector<u8>, u64>(&registry.id, VERSION_DF_KEY)) {
            let _: u64 = df::remove(&mut registry.id, VERSION_DF_KEY);
        }
    }

    #[test_only]
    public fun error_sub_agent_inactive_ancestor(): u64 { ESubAgentInactiveAncestor }

    #[test_only]
    public fun error_sub_agent_spend_exceeded(): u64 { ESubAgentSpendExceeded }

    #[test_only]
    public fun error_invalid_ancestor_chain(): u64 { EInvalidAncestorChain }

    #[test_only]
    public fun error_cap_escalation(): u64 { ECapsNotSubset }
}
