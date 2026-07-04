// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias, unused_use, unused_const)]
module social_contracts::memory_hierarchy_tests {
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
    const PARENT_AGENT: address = @0xA100;
    const CHILD_AGENT: address = @0xA101;
    const PEER_AGENT: address = @0xA102;
    const PARENT_PUBKEY: vector<u8> = x"0101010101010101010101010101010101010101010101010101010101010101";
    const CHILD_PUBKEY: vector<u8> = x"0202020202020202020202020202020202020202020202020202020202020202";
    const PEER_PUBKEY: vector<u8> = x"0303030303030303030303030303030303030303030303030303030303030303";

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
        clock: &Clock,
        ctx: &mut myso::tx_context::TxContext,
    ) {
        memory::register_sub_agent(
            config,
            memory_account,
            org,
            PARENT_PUBKEY,
            PARENT_AGENT,
            string::utf8(b"parent"),
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

    fun register_peer_agent(
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
            PEER_PUBKEY,
            PEER_AGENT,
            string::utf8(b"peer"),
            memory::class_delegated_ai(),
            0,
            memory::cap_post_publish(),
            0,
            2,
            0,
            option::none(),
            option::none(),
            option::none(),
            memory::register_peer(),
            clock,
            ctx,
        );
    }

    fun register_root_from_created_org(
        scenario: &mut test_scenario::Scenario,
    ) {
        let memory_config = test_scenario::take_shared<MemoryConfig>(scenario);
        let mut org = memory_test_helpers::take_created_org(scenario);
        let mut memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
        let clock = test_scenario::take_shared<Clock>(scenario);
        register_root_agent(
            &memory_config,
                &mut memory_account,
            &mut org,
            &clock,
            test_scenario::ctx(scenario),
        );
        test_scenario::return_shared(org);
        test_scenario::return_shared(memory_account);
        test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
    }

    #[test]
    fun test_child_and_peer_registration() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_org_in_tx(
                &mut scenario,
                memory::org_type_company(),
            );
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_root_from_created_org(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, PARENT_AGENT);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, PARENT_AGENT);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            register_child_agent(&memory_config, &mut memory_account, &parent, &clock, test_scenario::ctx(&mut scenario));
            register_peer_agent(&memory_config, &mut memory_account, &parent, &clock, test_scenario::ctx(&mut scenario));

            test_scenario::return_shared(parent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let child = take_agent(&scenario, &memory_account, CHILD_AGENT);
            let peer = take_agent(&scenario, &memory_account, PEER_AGENT);

            assert!(memory::sub_agent_depth(&child) == 2, 1);
            assert!(memory::sub_agent_depth(&peer) == 1, 2);

            test_scenario::return_shared(child);
            test_scenario::return_shared(peer);
            test_scenario::return_shared(memory_account);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_parent_revokes_child() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_org_in_tx(
                &mut scenario,
                memory::org_type_company(),
            );
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_root_from_created_org(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, PARENT_AGENT);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, PARENT_AGENT);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            register_child_agent(&memory_config, &mut memory_account, &parent, &clock, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(parent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, PARENT_AGENT);
        {
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, PARENT_AGENT);
            let child = take_agent(&scenario, &memory_account, CHILD_AGENT);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::revoke_sub_agent(
                &mut memory_account,
                child,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(parent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 25, location = social_contracts::memory)]
    fun test_caps_not_subset_rejected() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_org_in_tx(
                &mut scenario,
                memory::org_type_company(),
            );
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_root_from_created_org(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, PARENT_AGENT);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, PARENT_AGENT);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::register_sub_agent_delegated(
                &memory_config,
                &mut memory_account,
                &parent,
                CHILD_PUBKEY,
                CHILD_AGENT,
                string::utf8(b"child"),
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
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(parent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_deactivate_parent_blocks_child_actor() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_org_in_tx(
                &mut scenario,
                memory::org_type_company(),
            );
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_root_from_created_org(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, PARENT_AGENT);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, PARENT_AGENT);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            register_child_agent(&memory_config, &mut memory_account, &parent, &clock, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(parent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut parent = take_agent(&scenario, &memory_account, PARENT_AGENT);
            let child = take_agent(&scenario, &memory_account, CHILD_AGENT);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::deactivate_sub_agent(
                &mut memory_account,
                &mut parent,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(!memory::sub_agent_active(&parent), 0);

            test_scenario::return_shared(child);
            test_scenario::return_shared(parent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 15, location = social_contracts::memory)]
    fun test_inactive_ancestor_blocks_child_actor() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_org_in_tx(
                &mut scenario,
                memory::org_type_company(),
            );
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_root_from_created_org(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, PARENT_AGENT);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, PARENT_AGENT);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            register_child_agent(&memory_config, &mut memory_account, &parent, &clock, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(parent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut parent = take_agent(&scenario, &memory_account, PARENT_AGENT);
            let child = take_agent(&scenario, &memory_account, CHILD_AGENT);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::deactivate_sub_agent(
                &mut memory_account,
                &mut parent,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(child);
            test_scenario::return_shared(parent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, CHILD_AGENT);
        {
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);

            let _acting = memory::resolve_actor_from_account(
                &memory_config,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }
}
