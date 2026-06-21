// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias, unused_use, unused_const)]
module social_contracts::memory_tests {
    use std::string;
    use std::option;
    use std::vector;

    use myso::test_scenario;
    use myso::object::{Self, ID};
    use myso::clock::{Self, Clock};
    use myso::transfer;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;

    use social_contracts::memory::{Self, MemoryRegistry, MemoryAccount, SubAgent, AgenticOrganization};
    use social_contracts::memory_test_helpers;
    use social_contracts::profile::{Self, Profile, UsernameRegistry};

    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const USER2: address = @0x2;
    const AGENT_ADDR: address = @0xA11CE;
    const AGENT2_ADDR: address = @0xA12CE;
    const AGENT_PUBKEY: vector<u8> = x"0101010101010101010101010101010101010101010101010101010101010101";
    const AGENT2_PUBKEY: vector<u8> = x"0202020202020202020202020202020202020202020202020202020202020202";
    const WRONG_PLATFORM: address = @0xDEAD;

    fun init_env(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            profile::init_for_testing(test_scenario::ctx(scenario));
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            profile::create_profile(
                &mut registry,
                &mut memory_registry,
                string::utf8(b"User One"),
                string::utf8(b"userone"),
                string::utf8(b"bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(clock);
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

    fun register_test_agent(
        scenario: &mut test_scenario::Scenario,
        derived: address,
        pubkey: vector<u8>,
        capabilities: u64,
        approval_required_caps: u64,
        platform_scope: Option<address>,
    ) {
        let mut org = memory_test_helpers::take_created_org(scenario);
        let mut memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
        let clock = test_scenario::take_shared<Clock>(scenario);
        memory::register_sub_agent(
            &mut memory_account,
            &mut org,
            pubkey,
            derived,
            string::utf8(b"agent"),
            memory::class_delegated_ai(),
            0,
            capabilities,
            capabilities,
            3,
            approval_required_caps,
            option::none(),
            platform_scope,
            option::none(),
            &clock,
            test_scenario::ctx(scenario),
        );
        test_scenario::return_shared(org);
        test_scenario::return_shared(memory_account);
        test_scenario::return_shared(clock);
    }

    fun register_test_agent_with_spend(
        scenario: &mut test_scenario::Scenario,
        derived: address,
        pubkey: vector<u8>,
        capabilities: u64,
        max_action_spend: Option<u64>,
    ) {
        let mut org = memory_test_helpers::take_created_org(scenario);
        let mut memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
        let clock = test_scenario::take_shared<Clock>(scenario);
        memory::register_sub_agent(
            &mut memory_account,
            &mut org,
            pubkey,
            derived,
            string::utf8(b"spend-agent"),
            memory::class_delegated_ai(),
            0,
            capabilities,
            capabilities,
            3,
            0,
            max_action_spend,
            option::none(),
            option::none(),
            &clock,
            test_scenario::ctx(scenario),
        );
        test_scenario::return_shared(org);
        test_scenario::return_shared(memory_account);
        test_scenario::return_shared(clock);
    }

    #[test]
    fun test_register_and_update_sub_agent() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_test_agent(
                &mut scenario,
                AGENT_ADDR,
                AGENT_PUBKEY,
                memory::cap_memory_read(),
                0,
                option::none(),
            );
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_test_agent(
                &mut scenario,
                AGENT2_ADDR,
                AGENT2_PUBKEY,
                0,
                0,
                option::none(),
            );
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::update_sub_agent_label(
                &mut memory_account,
                &mut agent,
                string::utf8(b"updated"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            memory::deactivate_sub_agent(
                &mut memory_account,
                &mut agent,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(!memory::sub_agent_active(&agent), 1);

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::revoke_sub_agent(
                &mut memory_account,
                agent,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 14, location = social_contracts::memory)]
    fun test_duplicate_derived_address_rejected() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = memory_test_helpers::take_created_org(&mut scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::register_sub_agent(
                &mut memory_account,
                &mut org,
                AGENT_PUBKEY,
                AGENT_ADDR,
                string::utf8(b"agent"),
                memory::class_delegated_ai(),
                0,
                memory::cap_memory_read(),
                memory::cap_memory_read(),
                3,
                0,
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = memory_test_helpers::take_created_org(&mut scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::register_sub_agent(
                &mut memory_account,
                &mut org,
                AGENT2_PUBKEY,
                AGENT_ADDR,
                string::utf8(b"agent2"),
                memory::class_delegated_ai(),
                0,
                memory::cap_memory_read(),
                memory::cap_memory_read(),
                3,
                0,
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_transfer_clears_sub_agents() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_test_agent(
                &mut scenario,
                AGENT_ADDR,
                AGENT_PUBKEY,
                memory::cap_memory_read(),
                0,
                option::none(),
            );
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_test_agent(
                &mut scenario,
                AGENT2_ADDR,
                AGENT2_PUBKEY,
                0,
                0,
                option::none(),
            );
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            memory::revoke_sub_agent(
                &mut memory_account,
                agent,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);

            profile::transfer_profile_with_memory(
                &mut registry,
                &mut memory_registry,
                &mut memory_account,
                profile,
                USER2,
                1,
                test_scenario::ctx(&mut scenario),
            );

            assert!(memory::owner(&memory_account) == USER2, 1);

            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_approve_key_policy_owner_and_global_agent() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_test_agent(
                &mut scenario,
                AGENT_ADDR,
                AGENT_PUBKEY,
                memory::cap_memory_read(),
                0,
                option::none(),
            );
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let owner_suffix = memory::owner_key_suffix_bytes(USER1);
            let mut id = b"prefix";
            vector::append(&mut id, owner_suffix);

            memory::approve_key_policy(
                id,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let owner_suffix = memory::owner_key_suffix_bytes(USER1);
            let mut id = b"prefix";
            vector::append(&mut id, owner_suffix);

            memory::approve_key_policy(
                id,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 20, location = social_contracts::memory)]
    fun test_approve_key_policy_rejects_platform_scoped_agent() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_test_agent(
                &mut scenario,
                AGENT_ADDR,
                AGENT_PUBKEY,
                memory::cap_memory_read(),
                0,
                option::some(WRONG_PLATFORM),
            );
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let owner_suffix = memory::owner_key_suffix_bytes(USER1);
            let mut id = b"prefix";
            vector::append(&mut id, owner_suffix);

            memory::approve_key_policy(
                id,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 19, location = social_contracts::memory)]
    fun test_approval_required_blocks_direct_execution() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_test_agent(
                &mut scenario,
                AGENT_ADDR,
                AGENT_PUBKEY,
                memory::cap_post_publish(),
                memory::cap_post_publish(),
                option::none(),
            );
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);

            memory::assert_direct_execution_allowed(
                &memory_account,
                memory::cap_post_publish(),
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_action_spend_within_limit() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_test_agent_with_spend(
                &mut scenario,
                AGENT_ADDR,
                AGENT_PUBKEY,
                memory::cap_post_publish(),
                option::some(1_000),
            );
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);

            memory::assert_action_spend_limit(
                &memory_account,
                1_000,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 30, location = social_contracts::memory)]
    fun test_action_spend_exceeded() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            register_test_agent_with_spend(
                &mut scenario,
                AGENT_ADDR,
                AGENT_PUBKEY,
                memory::cap_post_publish(),
                option::some(100),
            );
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);

            memory::assert_action_spend_limit(
                &memory_account,
                101,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
        };

        test_scenario::end(scenario);
    }
}
