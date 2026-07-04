// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias, unused_use, unused_const)]
module social_contracts::memory_org_invitation_tests {
    use std::string;
    use std::option;

    use myso::test_scenario;
    use myso::object::{Self, ID};
    use myso::clock::{Self, Clock};
    use myso::transfer;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::permissioned_group::{Self, PermissionedGroup};

    use social_contracts::memory::{
        Self,
        MemoryRegistry,
        MemoryAccount,
        AgenticOrganization,
        MemorySharePackage,
        OrgMemoryReader,
    };
    use social_contracts::memory_test_helpers;
    use social_contracts::profile::{Self, Profile, UsernameRegistry,
        ProfileConfig};
    use social_contracts::ai_credit::AiCreditConfig;

    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const INVITEE: address = @0x10001;

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
    fun test_create_accept_org_invitation() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::create_org_invitation(
                &memory_account,
                &mut org,
                &mut group,
                INVITEE,
                option::none(),
                memory::org_perm_memory_read(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
        };

        test_scenario::next_tx(&mut scenario, INVITEE);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::accept_org_invitation(
                &memory_account,
                &mut org,
                &mut group,
                INVITEE,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            assert!(
                permissioned_group::has_permission<MemorySharePackage, OrgMemoryReader>(
                    &group,
                    INVITEE,
                ),
                0,
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_create_decline_org_invitation() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let org_id = setup_org_with_group(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::create_org_invitation(
                &memory_account,
                &mut org,
                &mut group,
                INVITEE,
                option::some(string::utf8(b"auditor")),
                0,
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
        };

        test_scenario::next_tx(&mut scenario, INVITEE);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::decline_org_invitation(
                &memory_account,
                &mut org,
                INVITEE,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(org);
        };

        test_scenario::end(scenario);
    }
}
