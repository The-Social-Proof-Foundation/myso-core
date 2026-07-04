// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias, unused_use, unused_const)]
module social_contracts::memory_organization_tests {
    use std::string;
    use std::option;

    use myso::test_scenario;
    use myso::object::{Self, ID};
    use myso::clock::{Self, Clock};
    use myso::transfer;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;

    use social_contracts::memory::{Self, MemoryRegistry, MemoryAccount, SubAgent, AgenticOrganization,
        MemoryConfig};
    use social_contracts::memory_test_helpers;
    use social_contracts::profile::{Self, Profile, UsernameRegistry,
        ProfileConfig};
    use social_contracts::ai_credit::AiCreditConfig;

    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const USER2: address = @0x2;
    const ROOT_AGENT: address = @0xA100;
    const CHILD_AGENT: address = @0xA101;
    const SECOND_ROOT_AGENT: address = @0xA102;
    const ROOT_PUBKEY: vector<u8> = x"0101010101010101010101010101010101010101010101010101010101010101";
    const CHILD_PUBKEY: vector<u8> = x"0202020202020202020202020202020202020202020202020202020202020202";
    const SECOND_ROOT_PUBKEY: vector<u8> = x"0303030303030303030303030303030303030303030303030303030303030303";

    fun init_user2_profile(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, USER2);
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
                string::utf8(b"User Two"),
                string::utf8(b"usertwo"),
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

    fun init_env(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(scenario));

            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(scenario));
            transfer::public_transfer(coins, USER1);
            let coins2 = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(scenario));
            transfer::public_transfer(coins2, USER2);
        };

        test_scenario::next_tx(scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(scenario);
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

            test_scenario::return_shared(clock);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
        test_scenario::return_shared(profile_config);
        };
    }

    fun agent_object_id_from_derived(
        memory_account: &MemoryAccount,
        derived: address,
    ): ID {
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

    fun register_root_agent(
        config: &MemoryConfig,
        memory_account: &mut MemoryAccount,
        org: &mut AgenticOrganization,
        derived: address,
        pubkey: vector<u8>,
        clock: &Clock,
        ctx: &mut myso::tx_context::TxContext,
    ) {
        memory::register_sub_agent(
            config,
            memory_account,
            org,
            pubkey,
            derived,
            string::utf8(b"root"),
            memory::class_delegated_ai(),
            0,
            memory::cap_agent_register() | memory::cap_agent_revoke(),
            memory::cap_agent_register() | memory::cap_agent_revoke() | memory::cap_post_publish(),
            3,
            0,
            option::none(),
            option::none(),
            option::none(),
            clock,
            ctx,
        );
    }

    fun register_child_agent(
        config: &MemoryConfig,
        memory_account: &mut MemoryAccount,
        parent: &SubAgent,
        clock: &Clock,
        ctx: &mut myso::tx_context::TxContext,
    ) {
        memory::register_sub_agent_delegated(
            config,
            memory_account,
            parent,
            CHILD_PUBKEY,
            CHILD_AGENT,
            string::utf8(b"child"),
            memory::class_delegated_ai(),
            0,
            memory::cap_post_publish(),
            memory::cap_post_publish(),
            1,
            0,
            option::none(),
            option::none(),
            option::none(),
            memory::register_child(),
            clock,
            ctx,
        );
    }

    #[test]
    fun test_create_org_each_valid_type() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
        let max_per_user = memory::max_organizations_per_user(&memory_config);
        let type_count = memory::org_type_count();
        test_scenario::return_shared(memory_config);

        let mut i: u8 = 0;
        while (i < max_per_user) {
            test_scenario::next_tx(&mut scenario, USER1);
            {
                memory_test_helpers::create_org_in_tx(&mut scenario, i);
            };

            test_scenario::next_tx(&mut scenario, USER1);
            {
                let org = memory_test_helpers::take_created_org(&mut scenario);
                assert!(memory::organization_org_type(&org) == i, (i as u64));
                test_scenario::return_shared(org);
            };

            i = i + 1;
        };

        i = max_per_user;
        init_user2_profile(&mut scenario);
        while (i < type_count) {
            test_scenario::next_tx(&mut scenario, USER2);
            {
                memory_test_helpers::create_org_in_tx(&mut scenario, i);
            };

            test_scenario::next_tx(&mut scenario, USER2);
            {
                let org = memory_test_helpers::take_created_org(&mut scenario);
                assert!(memory::organization_org_type(&org) == i, (i as u64));
                test_scenario::return_shared(org);
            };

            i = i + 1;
        };

        test_scenario::end(scenario);
    }

    fun create_org_for_limit_test(
        scenario: &mut test_scenario::Scenario
    ) {
        test_scenario::next_tx(scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            memory::test_force_account_version(&mut memory_account, 4);
            memory::test_create_agentic_organization(
                &memory_config,
                &mut memory_account,
                memory::org_type_other(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };
    }

    #[test]
    #[expected_failure(abort_code = 35, location = social_contracts::memory)]
    fun test_org_limit_exceeded(    ) {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        create_org_for_limit_test(&mut scenario);
        create_org_for_limit_test(&mut scenario);
        create_org_for_limit_test(&mut scenario);
        create_org_for_limit_test(&mut scenario);
        create_org_for_limit_test(&mut scenario);
        create_org_for_limit_test(&mut scenario);
        create_org_for_limit_test(&mut scenario);
        create_org_for_limit_test(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::test_force_account_version(&mut memory_account, 4);
            memory::test_create_agentic_organization(
                &memory_config,
                &mut memory_account,
                memory::org_type_other(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_root_agent_binds_to_org() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut org = memory_test_helpers::take_created_org(&mut scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            register_root_agent(
                &memory_config,
                &mut memory_account,
                &mut org,
                ROOT_AGENT,
                ROOT_PUBKEY,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = test_scenario::take_shared<AgenticOrganization>(&scenario);
            let organization_id = memory::organization_id(&org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);

            let root = take_agent(&scenario, &memory_account, ROOT_AGENT);
            assert!(memory::sub_agent_organization_id(&root) == organization_id, 0);
            assert!(
                option::is_some(&memory::organization_root_agent_id(&org)),
                1,
            );
            assert!(
                *option::borrow(&memory::organization_root_agent_id(&org))
                    == memory::agent_object_id(&root),
                2,
            );

            test_scenario::return_shared(root);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 40, location = social_contracts::memory)]
    fun test_second_root_fails() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut org = memory_test_helpers::take_created_org(&mut scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            register_root_agent(
                &memory_config,
                &mut memory_account,
                &mut org,
                ROOT_AGENT,
                ROOT_PUBKEY,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            register_root_agent(
                &memory_config,
                &mut memory_account,
                &mut org,
                SECOND_ROOT_AGENT,
                SECOND_ROOT_PUBKEY,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_delegated_child_inherits_organization_id() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut org = memory_test_helpers::take_created_org(&mut scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            register_root_agent(
                &memory_config,
                &mut memory_account,
                &mut org,
                ROOT_AGENT,
                ROOT_PUBKEY,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, ROOT_AGENT);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let org = test_scenario::take_shared<AgenticOrganization>(&scenario);
            let parent = take_agent(&scenario, &memory_account, ROOT_AGENT);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            register_child_agent(
                &memory_config,
                &mut memory_account,
                &parent,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(parent);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = test_scenario::take_shared<AgenticOrganization>(&scenario);
            let organization_id = memory::organization_id(&org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let child = take_agent(&scenario, &memory_account, CHILD_AGENT);
            assert!(memory::sub_agent_organization_id(&child) == organization_id, 0);

            test_scenario::return_shared(child);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 36, location = social_contracts::memory)]
    fun test_invalid_org_type_rejected() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::test_create_agentic_organization(
                &memory_config,
                &mut memory_account,
                memory::org_type_count(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    fun repeat_byte(n: u64, byte: u8): vector<u8> {
        let mut v = vector[];
        let mut i = 0;
        while (i < n) {
            vector::push_back(&mut v, byte);
            i = i + 1;
        };
        v
    }

    #[test]
    fun test_create_org_with_name_and_description() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::test_create_agentic_organization(
                &memory_config,
                &mut memory_account,
                memory::org_type_company(),
                option::some(string::utf8(b"Acme Agents")),
                option::some(string::utf8(b"We build autonomous trading agents.")),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = memory_test_helpers::take_created_org(&mut scenario);
            assert!(option::is_some(memory::organization_name(&org)), 0);
            assert!(option::is_some(memory::organization_description(&org)), 1);
            test_scenario::return_shared(org);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_create_org_without_metadata() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::test_create_agentic_organization(
                &memory_config,
                &mut memory_account,
                memory::org_type_other(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = memory_test_helpers::take_created_org(&mut scenario);
            assert!(option::is_none(memory::organization_name(&org)), 0);
            assert!(option::is_none(memory::organization_description(&org)), 1);
            test_scenario::return_shared(org);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_update_org_metadata() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = memory_test_helpers::take_created_org(&mut scenario);
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            memory::update_agentic_organization_metadata(
                &memory_config,
                &memory_account,
                &mut org,
                option::some(string::utf8(b"Renamed Org")),
                option::none(),
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let org = test_scenario::take_shared<AgenticOrganization>(&scenario);
            assert!(option::is_some(memory::organization_name(&org)), 0);
            assert!(option::is_none(memory::organization_description(&org)), 1);
            test_scenario::return_shared(org);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_description_at_max_length() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let desc_bytes = repeat_byte(memory::max_org_description_length(), 97);
            memory::test_create_agentic_organization(
                &memory_config,
                &mut memory_account,
                memory::org_type_other(),
                option::none(),
                option::some(string::utf8(desc_bytes)),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 45, location = social_contracts::memory)]
    fun test_description_over_max_length() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let desc_bytes = repeat_byte(memory::max_org_description_length() + 1, 97);
            memory::test_create_agentic_organization(
                &memory_config,
                &mut memory_account,
                memory::org_type_other(),
                option::none(),
                option::some(string::utf8(desc_bytes)),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }
}
