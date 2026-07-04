// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias, unused_use, unused_const)]
module social_contracts::memory_org_roles_tests {
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
        MemoryConfig,
        AgenticOrganization,
        MemorySharePackage,
        OrgMemoryReader,
        OrgMemoryWriter,
        OrgBudgetManager,
        OrgSpendApprover,
        OrgDashboardViewer,
        OrgAuditor,
    };
    use social_contracts::memory_test_helpers;
    use social_contracts::profile::{Self, Profile, UsernameRegistry,
        ProfileConfig};
    use social_contracts::ai_credit::AiCreditConfig;

    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const FINANCE_HUMAN: address = @0xF1;
    const STAFF_HUMAN: address = @0x5AFF;

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
            let memory_config = test_scenario::take_shared<MemoryConfig>(scenario);
            let profile_config = test_scenario::take_shared<ProfileConfig>(scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            profile::create_profile(
                &mut registry,
                &profile_config,
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

            test_scenario::return_shared(profile_config);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
        };
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

    #[test]
    fun test_assign_and_revoke_builtin_role() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::assign_org_role(
                &memory_account,
                &mut org,
                &mut group,
                FINANCE_HUMAN,
                string::utf8(b"finance_approver"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            // finance_approver = budget manager + spend approver.
            assert!(memory::has_org_permission<OrgBudgetManager>(&org, &group, FINANCE_HUMAN), 0);
            assert!(memory::has_org_permission<OrgSpendApprover>(&org, &group, FINANCE_HUMAN), 1);
            assert!(!memory::has_org_permission<OrgMemoryReader>(&org, &group, FINANCE_HUMAN), 2);
            let assigned = memory::org_role_assignment_mask(
                &org,
                FINANCE_HUMAN,
                string::utf8(b"finance_approver"),
            );
            assert!(option::is_some(&assigned), 3);
            assert!(*option::borrow(&assigned) == memory::role_mask_finance_approver(), 4);

            memory::revoke_org_role(
                &memory_account,
                &mut org,
                &mut group,
                FINANCE_HUMAN,
                string::utf8(b"finance_approver"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(!memory::has_org_permission<OrgBudgetManager>(&org, &group, FINANCE_HUMAN), 5);
            assert!(!memory::has_org_permission<OrgSpendApprover>(&org, &group, FINANCE_HUMAN), 6);
            let after = memory::org_role_assignment_mask(
                &org,
                FINANCE_HUMAN,
                string::utf8(b"finance_approver"),
            );
            assert!(option::is_none(&after), 7);

            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_role_revoke_preserves_prior_direct_grants() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            // Direct grant of memory read first.
            memory::grant_org_memory_permission(
                &memory_account,
                &org,
                &mut group,
                STAFF_HUMAN,
                memory::org_perm_memory_read(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            // memory_administrator = read + write. Delta must exclude the pre-held read bit.
            memory::assign_org_role(
                &memory_account,
                &mut org,
                &mut group,
                STAFF_HUMAN,
                string::utf8(b"memory_administrator"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            let assigned = memory::org_role_assignment_mask(
                &org,
                STAFF_HUMAN,
                string::utf8(b"memory_administrator"),
            );
            assert!(*option::borrow(&assigned) == memory::org_perm_memory_write(), 0);

            // Revoking the role removes write but keeps the directly-granted read.
            memory::revoke_org_role(
                &memory_account,
                &mut org,
                &mut group,
                STAFF_HUMAN,
                string::utf8(b"memory_administrator"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(memory::has_org_permission<OrgMemoryReader>(&org, &group, STAFF_HUMAN), 1);
            assert!(!memory::has_org_permission<OrgMemoryWriter>(&org, &group, STAFF_HUMAN), 2);

            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_custom_role_define_assign_and_redefine_safety() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::define_custom_org_role(
                &memory_config,
                &memory_account,
                &mut org,
                &group,
                string::utf8(b"observer"),
                memory::org_perm_dashboard_viewer() | memory::org_perm_auditor(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            let mask = memory::org_role_mask(&org, &string::utf8(b"observer"));
            assert!(
                *option::borrow(&mask)
                    == (memory::org_perm_dashboard_viewer() | memory::org_perm_auditor()),
                0,
            );

            memory::assign_org_role(
                &memory_account,
                &mut org,
                &mut group,
                STAFF_HUMAN,
                string::utf8(b"observer"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(memory::has_org_permission<OrgDashboardViewer>(&org, &group, STAFF_HUMAN), 1);
            assert!(memory::has_org_permission<OrgAuditor>(&org, &group, STAFF_HUMAN), 2);

            // Redefine the role to a wider mask — the existing assignment's recorded delta
            // must be unchanged, so revoke removes only what was originally granted.
            memory::define_custom_org_role(
                &memory_config,
                &memory_account,
                &mut org,
                &group,
                string::utf8(b"observer"),
                memory::org_perm_all(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            memory::revoke_org_role(
                &memory_account,
                &mut org,
                &mut group,
                STAFF_HUMAN,
                string::utf8(b"observer"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(!memory::has_org_permission<OrgDashboardViewer>(&org, &group, STAFF_HUMAN), 3);
            assert!(!memory::has_org_permission<OrgAuditor>(&org, &group, STAFF_HUMAN), 4);
            // Member never held memory read (redefined mask was never re-granted).
            assert!(!memory::has_org_permission<OrgMemoryReader>(&org, &group, STAFF_HUMAN), 5);

            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 54, location = social_contracts::memory)]
    fun test_builtin_role_redefine_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::define_custom_org_role(
                &memory_config,
                &memory_account,
                &mut org,
                &group,
                string::utf8(b"admin"),
                memory::org_perm_memory_read(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 49, location = social_contracts::memory)]
    fun test_assign_unknown_role_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::assign_org_role(
                &memory_account,
                &mut org,
                &mut group,
                STAFF_HUMAN,
                string::utf8(b"nonexistent"),
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
    #[expected_failure(abort_code = 50, location = social_contracts::memory)]
    fun test_double_assign_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::assign_org_role(
                &memory_account,
                &mut org,
                &mut group,
                STAFF_HUMAN,
                string::utf8(b"auditor"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            memory::assign_org_role(
                &memory_account,
                &mut org,
                &mut group,
                STAFF_HUMAN,
                string::utf8(b"auditor"),
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
