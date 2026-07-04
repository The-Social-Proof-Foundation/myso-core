// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias, unused_use, unused_const)]
module social_contracts::memory_org_governance_tests {
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
        AgenticOrganization,
        MemorySharePackage,
        OrgGovernanceProposer,
        OrgGovernanceVoter,
        MemoryConfig,
    };
    use social_contracts::memory_test_helpers;
    use social_contracts::profile::{Self, Profile, UsernameRegistry,
        ProfileConfig};
    use social_contracts::ai_credit::AiCreditConfig;

    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const GOVERNANCE_MEMBER: address = @0x607;

    fun init_env(sc: &mut test_scenario::Scenario) {
        test_scenario::next_tx(sc, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(sc));
            profile::init_for_testing(&clock, test_scenario::ctx(sc));
            clock::share_for_testing(clock);
        };

        test_scenario::next_tx(sc, USER1);
        {
            let mut registry = test_scenario::take_shared<UsernameRegistry>(sc);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(sc);
            let memory_config = test_scenario::take_shared<MemoryConfig>(sc);

            let profile_config = test_scenario::take_shared<ProfileConfig>(sc);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(sc);
            let clock = test_scenario::take_shared<Clock>(sc);

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
                test_scenario::ctx(sc),
            );
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(profile_config);
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

    fun setup_org_with_group(sc: &mut test_scenario::Scenario): ID {
        test_scenario::next_tx(sc, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(sc);
        };

        test_scenario::next_tx(sc, USER1);
        {
            let mut org = memory_test_helpers::take_created_org(sc);
            let org_id = object::id(&org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(sc);
            let clock = test_scenario::take_shared<Clock>(sc);
            let memory_config = test_scenario::take_shared<MemoryConfig>(sc);
            memory::ensure_org_memory_group(
                &memory_account,
                &mut org,
                &clock,
                test_scenario::ctx(sc),
            );
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_config);
            org_id
        }
    }

    #[test]
    fun test_grant_and_revoke_governance_permissions() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        let governance_mask =
            memory::org_perm_governance_proposer() | memory::org_perm_governance_voter();

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            memory::grant_org_memory_permission(
                &memory_account,
                &org,
                &mut group,
                GOVERNANCE_MEMBER,
                governance_mask,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            assert!(
                permissioned_group::has_permission<MemorySharePackage, OrgGovernanceProposer>(
                    &group,
                    GOVERNANCE_MEMBER,
                ),
                0,
            );
            assert!(
                permissioned_group::has_permission<MemorySharePackage, OrgGovernanceVoter>(
                    &group,
                    GOVERNANCE_MEMBER,
                ),
                1,
            );

            memory::revoke_org_memory_permission(
                &memory_account,
                &org,
                &mut group,
                GOVERNANCE_MEMBER,
                governance_mask,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            assert!(
                !permissioned_group::has_permission<MemorySharePackage, OrgGovernanceProposer>(
                    &group,
                    GOVERNANCE_MEMBER,
                ),
                2,
            );
            assert!(
                !permissioned_group::has_permission<MemorySharePackage, OrgGovernanceVoter>(
                    &group,
                    GOVERNANCE_MEMBER,
                ),
                3,
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 47, location = social_contracts::memory)]
    fun test_custom_role_rejects_unknown_governance_bit() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            memory::define_custom_org_role(
                &memory_config,
                &memory_account,
                &mut org,
                &group,
                string::utf8(b"governance_observer"),
                512,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }
}
