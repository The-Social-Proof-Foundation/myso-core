// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias, unused_use, unused_const)]
module social_contracts::memory_org_sharing_tests {
    use std::string;
    use std::option;

    use myso::test_scenario;
    use myso::object::{Self, ID};
    use myso::clock::{Self, Clock};
    use myso::transfer;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::permissioned_group::{Self, PermissionedGroup, ExtensionPermissionsAdmin};

    use social_contracts::memory::{
        Self,
        MemoryRegistry,
        MemoryAccount,
        SubAgent,
        AgenticOrganization,
        MemorySharePackage,
        OrgMemoryReader,
        OrgMemoryWriter,
        OrgSpendApprover,
    };
    use social_contracts::memory_test_helpers;
    use social_contracts::profile::{Self, Profile, UsernameRegistry};
    use social_contracts::ai_credit::AiCreditConfig;

    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const ROOT_AGENT: address = @0xA200;
    const MEMBER_AGENT: address = @0xA201;
    const STAFF_HUMAN: address = @0x5AFF;
    const ROOT_PUBKEY: vector<u8> = x"0101010101010101010101010101010101010101010101010101010101010101";
    const MEMBER_PUBKEY: vector<u8> = x"0202020202020202020202020202020202020202020202020202020202020202";

    fun init_env(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            profile::create_profile(
                &mut registry,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"User One"),
                string::utf8(b"userone"),
                string::utf8(b"bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
        };
    }

    fun agent_object_id_from_derived(memory_account: &MemoryAccount, derived: address): ID {
        object::id_from_address(memory::derive_sub_agent_address(memory_account, derived))
    }

    fun take_agent(
        scenario: &test_scenario::Scenario,
        memory_account: &MemoryAccount,
        derived: address,
    ): SubAgent {
        test_scenario::take_shared_by_id<SubAgent>(
            scenario,
            agent_object_id_from_derived(memory_account, derived),
        )
    }

    fun take_group(
        scenario: &test_scenario::Scenario,
        org: &AgenticOrganization,
    ): PermissionedGroup<MemorySharePackage> {
        test_scenario::take_shared_by_id<PermissionedGroup<MemorySharePackage>>(
            scenario,
            object::id_from_address(memory::org_memory_group_address(org)),
        )
    }

    /// Create an org and its memory share group; returns the org id.
    fun setup_org_with_group(scenario: &mut test_scenario::Scenario): ID {
        test_scenario::next_tx(scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(scenario);
        };

        test_scenario::next_tx(scenario, USER1);
        {
            let mut org = memory_test_helpers::take_created_org(scenario);
            let org_id = object::id(&org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            memory::ensure_org_memory_group(
                &memory_account,
                &mut org,
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
            org_id
        }
    }

    fun register_root_agent_for_org(scenario: &mut test_scenario::Scenario, org_id: ID) {
        test_scenario::next_tx(scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(scenario, org_id);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            memory::register_sub_agent(
                &mut memory_account,
                &mut org,
                ROOT_PUBKEY,
                ROOT_AGENT,
                string::utf8(b"root"),
                memory::class_delegated_ai(),
                0,
                memory::cap_memory_read() | memory::cap_memory_write() | memory::cap_agent_register(),
                memory::cap_memory_read() | memory::cap_memory_write(),
                3,
                0,
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };
    }

    fun register_member_agent_via_root(scenario: &mut test_scenario::Scenario, org_id: ID) {
        test_scenario::next_tx(scenario, ROOT_AGENT);
        {
            let _org = test_scenario::take_shared_by_id<AgenticOrganization>(scenario, org_id);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let parent = take_agent(scenario, &memory_account, ROOT_AGENT);
            let clock = test_scenario::take_shared<Clock>(scenario);
            memory::register_sub_agent_delegated(
                &mut memory_account,
                &parent,
                MEMBER_PUBKEY,
                MEMBER_AGENT,
                string::utf8(b"member"),
                memory::class_delegated_ai(),
                0,
                memory::cap_memory_read(),
                0,
                1,
                0,
                option::none(),
                option::none(),
                option::none(),
                memory::register_child(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(_org);
            test_scenario::return_shared(parent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };
    }

    #[test]
    fun test_ensure_org_memory_group_idempotent() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        // Second ensure is a no-op (no abort), and the group is takeable.
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            assert!(memory::org_memory_group_exists(&org), 0);
            memory::ensure_org_memory_group(
                &memory_account,
                &mut org,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let group = take_group(&scenario, &org);
            // Owner got PermissionsAdmin + ExtensionPermissionsAdmin from creation.
            assert!(
                permissioned_group::has_permission<MemorySharePackage, ExtensionPermissionsAdmin>(
                    &group,
                    USER1,
                ),
                1,
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_owner_grants_and_revokes_member_permissions() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);
        register_root_agent_for_org(&mut scenario, org_id);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::grant_org_memory_permission(
                &memory_account,
                &org,
                &mut group,
                ROOT_AGENT,
                memory::org_perm_memory_read() | memory::org_perm_memory_write(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(memory::has_org_permission<OrgMemoryReader>(&org, &group, ROOT_AGENT), 0);
            assert!(memory::has_org_permission<OrgMemoryWriter>(&org, &group, ROOT_AGENT), 1);

            memory::revoke_org_memory_permission(
                &memory_account,
                &org,
                &mut group,
                ROOT_AGENT,
                memory::org_perm_memory_write(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(memory::has_org_permission<OrgMemoryReader>(&org, &group, ROOT_AGENT), 2);
            assert!(!memory::has_org_permission<OrgMemoryWriter>(&org, &group, ROOT_AGENT), 3);

            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_root_agent_delegated_management() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);
        register_root_agent_for_org(&mut scenario, org_id);
        register_member_agent_via_root(&mut scenario, org_id);

        // Owner delegates group management to the root agent once.
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            permissioned_group::grant_permission<MemorySharePackage, ExtensionPermissionsAdmin>(
                &mut group,
                ROOT_AGENT,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
        };

        // Root agent grants org memory read to the member agent — no human tx.
        test_scenario::next_tx(&mut scenario, ROOT_AGENT);
        {
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::grant_org_memory_permission(
                &memory_account,
                &org,
                &mut group,
                MEMBER_AGENT,
                memory::org_perm_memory_read(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(memory::has_org_permission<OrgMemoryReader>(&org, &group, MEMBER_AGENT), 0);

            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 100, location = social_contracts::memory)]
    fun test_grant_without_manager_permission_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);
        register_root_agent_for_org(&mut scenario, org_id);

        // Root agent has no ExtensionPermissionsAdmin — grant must abort.
        test_scenario::next_tx(&mut scenario, ROOT_AGENT);
        {
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::grant_org_memory_permission(
                &memory_account,
                &org,
                &mut group,
                MEMBER_AGENT,
                memory::org_perm_memory_read(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 46, location = social_contracts::memory)]
    fun test_wrong_org_group_rejected() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_a_id = setup_org_with_group(&mut scenario);

        // Second org without a group.
        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org_a = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_a_id);
            let mut group_a = take_group(&scenario, &org_a);
            // org A is held, so a plain take returns org B.
            let org_b = test_scenario::take_shared<AgenticOrganization>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::grant_org_memory_permission(
                &memory_account,
                &org_b,
                &mut group_a,
                STAFF_HUMAN,
                memory::org_perm_memory_read(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(group_a);
            test_scenario::return_shared(org_a);
            test_scenario::return_shared(org_b);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 42, location = social_contracts::memory)]
    fun test_cross_org_agent_grant_rejected() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        // Org A gets the root agent; org B gets the group.
        let org_a_id = setup_org_with_group(&mut scenario);
        register_root_agent_for_org(&mut scenario, org_a_id);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        // Create group for org B.
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org_a = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_a_id);
            let mut org_b = test_scenario::take_shared<AgenticOrganization>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::ensure_org_memory_group(
                &memory_account,
                &mut org_b,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(org_a);
            test_scenario::return_shared(org_b);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        // Granting org B's permission to org A's agent must abort (org mismatch).
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org_a = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_a_id);
            let org_b = test_scenario::take_shared<AgenticOrganization>(&scenario);
            let mut group_b = take_group(&scenario, &org_b);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::grant_org_memory_permission(
                &memory_account,
                &org_b,
                &mut group_b,
                ROOT_AGENT,
                memory::org_perm_memory_read(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(group_b);
            test_scenario::return_shared(org_a);
            test_scenario::return_shared(org_b);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_org_key_policy_owner_and_reader_paths() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);
        register_root_agent_for_org(&mut scenario, org_id);

        // Grant reader to the root agent.
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::grant_org_memory_permission(
                &memory_account,
                &org,
                &mut group,
                ROOT_AGENT,
                memory::org_perm_memory_read(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        // Owner path (own-blob suffix).
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::approve_org_key_policy(
                memory::owner_key_suffix_bytes(USER1),
                &memory_account,
                &org,
                &group,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        // Reader path (org agent holding OrgMemoryReader).
        test_scenario::next_tx(&mut scenario, ROOT_AGENT);
        {
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::approve_org_key_policy(
                b"org-blob-id",
                &memory_account,
                &org,
                &group,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 100, location = social_contracts::memory)]
    fun test_org_key_policy_rejects_non_reader() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);
        register_root_agent_for_org(&mut scenario, org_id);

        // Root agent has no OrgMemoryReader grant — key release must abort.
        test_scenario::next_tx(&mut scenario, ROOT_AGENT);
        {
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::approve_org_key_policy(
                b"org-blob-id",
                &memory_account,
                &org,
                &group,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }
}
