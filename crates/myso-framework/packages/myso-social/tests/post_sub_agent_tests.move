// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias, unused_use)]
module social_contracts::post_sub_agent_tests {
    use std::string;
    use std::option;
    use std::vector;

    use myso::test_scenario;
    use myso::object;
    use myso::clock::{Self, Clock};
    use myso::transfer;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;

    use social_contracts::post::{Self, Post, PostConfig, Comment};
    use social_contracts::profile::{Self, UsernameRegistry,
        ProfileConfig};
    use social_contracts::ai_credit::AiCreditConfig;
    use social_contracts::memory::{Self, MemoryRegistry, MemoryAccount, SubAgent, AgenticOrganization,
        MemoryConfig};
    use social_contracts::memory_test_helpers;
    use social_contracts::platform::{Self, Platform, PlatformRegistry,
        PlatformConfig};
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::mydata::{Self, MyDataRegistry};

    const ADMIN: address = @0xAD;
    const AUTHOR: address = @0x1;
    const AGENT_ADDR: address = @0xA11CE;
    const PLACEHOLDER_AGENT: address = @0xBEEF;
    const AGENT_PUBKEY: vector<u8> = x"0101010101010101010101010101010101010101010101010101010101010101";
    const PLACEHOLDER_PUBKEY: vector<u8> = x"0303030303030303030303030303030303030303030303030303030303030303";
    const WRONG_PLATFORM: address = @0xDEAD;

    fun setup(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(scenario));

            post::init_for_testing(test_scenario::ctx(scenario));
            mydata::test_init(&clock, test_scenario::ctx(scenario));

            platform::test_init(&clock, test_scenario::ctx(scenario));
            block_list::test_init(&clock, test_scenario::ctx(scenario));

            clock::share_for_testing(clock);
        };

        test_scenario::next_tx(scenario, ADMIN);
        {
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let platform_config = test_scenario::take_shared<PlatformConfig>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            platform::create_platform(
                &mut registry,
                &platform_config,
                string::utf8(b"Post Platform"),
                string::utf8(b"tagline"),
                string::utf8(b"desc"),
                string::utf8(b"https://example.com/logo.png"),
                string::utf8(b"https://example.com/tos"),
                string::utf8(b"https://example.com/privacy"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example.com")],
                string::utf8(b"Social Network"),
                option::none(),
                3,
                string::utf8(b"2023-01-01"),
                false,
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),

                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
        };

        test_scenario::next_tx(scenario, ADMIN);
        {
            let platform_obj = test_scenario::take_shared<Platform>(scenario);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let platform_id = object::uid_to_address(platform::id(&platform_obj));
            platform::test_set_approval(&mut registry, platform_id, true);
            test_scenario::return_shared(platform_obj);
            test_scenario::return_shared(registry);
        };

        test_scenario::next_tx(scenario, AUTHOR);
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
                string::utf8(b"Author"),
                string::utf8(b"author"),
                string::utf8(b"bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(profile_config);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
        };

        test_scenario::next_tx(scenario, AUTHOR);
        {
            memory_test_helpers::create_default_org_in_tx(scenario);
        };

        test_scenario::next_tx(scenario, AUTHOR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(scenario);
            let mut org = memory_test_helpers::take_created_org(scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            memory::register_sub_agent(
                &memory_config,
                &mut memory_account,
                &mut org,
                PLACEHOLDER_PUBKEY,
                PLACEHOLDER_AGENT,
                string::utf8(b"placeholder"),
                memory::class_delegated_ai(),
                0,
                0,
                0,
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
        test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(scenario, AUTHOR);
        {
            let mut platform_obj = test_scenario::take_shared<Platform>(scenario);
            platform::test_join_platform(&mut platform_obj, AUTHOR);
            test_scenario::return_shared(platform_obj);
        };
    }

    fun take_agent(
        scenario: &test_scenario::Scenario,
        memory_account: &MemoryAccount,
        derived: address,
    ): SubAgent {
        let agent_id = object::id_from_address(
            memory::derive_sub_agent_address(memory_account, derived),
        );
        test_scenario::take_shared_by_id<SubAgent>(scenario, agent_id)
    }

    fun register_agent(
        scenario: &mut test_scenario::Scenario,
        derived: address,
        capabilities: u64,
        approval_required_caps: u64,
        max_action_spend: Option<u64>,
        platform_scope: Option<address>,
    ) {
        let memory_config = test_scenario::take_shared<MemoryConfig>(scenario);
        let mut org = memory_test_helpers::take_created_org(scenario);
        let mut memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
        let clock = test_scenario::take_shared<Clock>(scenario);
        memory::register_sub_agent(
            &memory_config,
            &mut memory_account,
            &mut org,
            AGENT_PUBKEY,
            derived,
            string::utf8(b"post-agent"),
            memory::class_delegated_ai(),
            0,
            capabilities,
            capabilities,
            3,
            approval_required_caps,
            max_action_spend,
            platform_scope,
            option::none(),
            &clock,
            test_scenario::ctx(scenario),
        );
        test_scenario::return_shared(org);
        test_scenario::return_shared(memory_account);
        test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
    }

    fun publish_post(scenario: &mut test_scenario::Scenario, sender: address) {
        test_scenario::next_tx(scenario, sender);
        {
            let registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let platform_registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let platform = test_scenario::take_shared<Platform>(scenario);
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(scenario);
            let config = test_scenario::take_shared<PostConfig>(scenario);
            let memory_config = test_scenario::take_shared<MemoryConfig>(scenario);
            let mydata_registry = test_scenario::take_shared<MyDataRegistry>(scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            post::create_post(
                &registry,
                &platform_registry,
                &platform,
                &block_list_registry,
                &config,
                &memory_config,
                string::utf8(b"hello sub-agent world"),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                post::test_access_kind_public(),
                option::none(),
                option::none(),
                option::none(),
                &mydata_registry,
                &memory_account,
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(mydata_registry);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(platform_registry);
            test_scenario::return_shared(registry);
        };
    }

    #[test]
    fun test_human_publish_human_attribution() {
        let mut scenario = test_scenario::begin(ADMIN);
        setup(&mut scenario);
        publish_post(&mut scenario, AUTHOR);

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            let post = test_scenario::take_shared<Post>(&scenario);
            assert!(post::actor_address(&post) == AUTHOR, 0);
            assert!(option::is_none(&post::sub_agent_id(&post)), 1);
            assert!(post::action_identity_class(&post) == memory::class_human(), 2);
            test_scenario::return_shared(post);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_agent_direct_publish_agent_attribution() {
        let mut scenario = test_scenario::begin(ADMIN);
        setup(&mut scenario);

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            register_agent(
                &mut scenario,
                AGENT_ADDR,
                memory::cap_post_publish(),
                0,
                option::none(),
                option::none(),
            );
        };

        publish_post(&mut scenario, AGENT_ADDR);

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let post = test_scenario::take_shared<Post>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            assert!(post::actor_address(&post) == AGENT_ADDR, 0);
            assert!(option::is_some(&post::sub_agent_id(&post)), 1);
            assert!(*option::borrow(&post::sub_agent_id(&post)) == memory::agent_object_id(&agent), 2);
            assert!(post::action_identity_class(&post) == memory::class_delegated_ai(), 3);
            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(post);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 18, location = social_contracts::memory)]
    fun test_agent_missing_cap_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        setup(&mut scenario);

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            register_agent(
                &mut scenario,
                AGENT_ADDR,
                0,
                0,
                option::none(),
                option::none(),
            );
        };

        publish_post(&mut scenario, AGENT_ADDR);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 19, location = social_contracts::memory)]
    fun test_approval_gated_agent_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        setup(&mut scenario);

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            register_agent(
                &mut scenario,
                AGENT_ADDR,
                memory::cap_post_publish(),
                memory::cap_post_publish(),
                option::none(),
                option::none(),
            );
        };

        publish_post(&mut scenario, AGENT_ADDR);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 17, location = social_contracts::memory)]
    fun test_wrong_platform_scope_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        setup(&mut scenario);

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            register_agent(
                &mut scenario,
                AGENT_ADDR,
                memory::cap_post_publish(),
                0,
                option::none(),
                option::some(WRONG_PLATFORM),
            );
        };

        publish_post(&mut scenario, AGENT_ADDR);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 12, location = social_contracts::post)]
    fun test_principal_not_joined_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        setup(&mut scenario);

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut platform_obj = test_scenario::take_shared<Platform>(&scenario);
            platform::leave_platform(&mut platform_obj, &clock, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(platform_obj);

            test_scenario::return_shared(clock);
        };

        publish_post(&mut scenario, AUTHOR);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_agent_comment_and_react_attribution() {
        let mut scenario = test_scenario::begin(ADMIN);
        setup(&mut scenario);

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            register_agent(
                &mut scenario,
                AGENT_ADDR,
                memory::cap_post_publish() | memory::cap_comment() | memory::cap_react(),
                0,
                option::none(),
                option::none(),
            );
        };

        publish_post(&mut scenario, AUTHOR);

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let platform_registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let config = test_scenario::take_shared<PostConfig>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut post = test_scenario::take_shared<Post>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            post::create_comment(
                &registry,
                &platform_registry,
                &platform,
                &block_list_registry,
                &config,
                &memory_config,
                &memory_account,
                &mut post,
                option::none(),
                string::utf8(b"agent comment"),
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            post::react_to_post(
                &registry,
                &mut post,
                &platform_registry,
                &platform,
                &block_list_registry,
                &config,
                &memory_config,
                &memory_account,
                string::utf8(b"👍"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(post);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(platform_registry);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let comment = test_scenario::take_shared<Comment>(&scenario);
            assert!(post::comment_actor_address(&comment) == AGENT_ADDR, 0);
            assert!(option::is_some(&post::comment_sub_agent_id(&comment)), 1);
            test_scenario::return_shared(comment);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_agent_edit_and_explicit_remove_reaction() {
        let mut scenario = test_scenario::begin(ADMIN);
        setup(&mut scenario);

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };
        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            register_agent(
                &mut scenario,
                AGENT_ADDR,
                memory::cap_post_publish() | memory::cap_react(),
                0,
                option::none(),
                option::none(),
            );
        };
        publish_post(&mut scenario, AGENT_ADDR);

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let platform_registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let config = test_scenario::take_shared<PostConfig>(&scenario);
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut created_post = test_scenario::take_shared<Post>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            post::edit_post(
                &registry,
                &platform,
                &block_list_registry,
                &config,
                &memory_config,
                &memory_account,
                &mut created_post,
                string::utf8(b"edited by agent"),
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            post::react_to_post(
                &registry,
                &mut created_post,
                &platform_registry,
                &platform,
                &block_list_registry,
                &config,
                &memory_config,
                &memory_account,
                string::utf8(b"like"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            post::remove_post_reaction(
                &registry,
                &mut created_post,
                &platform,
                &block_list_registry,
                &memory_config,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(post::get_post_content(&created_post) == string::utf8(b"edited by agent"), 0);
            assert!(post::get_reaction_count(&created_post) == 0, 1);

            test_scenario::return_shared(clock);
            test_scenario::return_shared(created_post);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(platform_registry);
            test_scenario::return_shared(registry);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 18, location = social_contracts::memory)]
    fun test_agent_missing_comment_cap_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        setup(&mut scenario);

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            register_agent(
                &mut scenario,
                AGENT_ADDR,
                memory::cap_post_publish(),
                0,
                option::none(),
                option::none(),
            );
        };

        publish_post(&mut scenario, AUTHOR);

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let platform_registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let config = test_scenario::take_shared<PostConfig>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut post = test_scenario::take_shared<Post>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            post::create_comment(
                &registry,
                &platform_registry,
                &platform,
                &block_list_registry,
                &config,
                &memory_config,
                &memory_account,
                &mut post,
                option::none(),
                string::utf8(b"nope"),
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(post);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(platform_registry);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 30, location = social_contracts::memory)]
    fun test_agent_tip_exceeds_spend_cap() {
        let mut scenario = test_scenario::begin(ADMIN);
        setup(&mut scenario);
        publish_post(&mut scenario, AUTHOR);

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            register_agent(
                &mut scenario,
                AGENT_ADDR,
                memory::cap_post_publish(),
                0,
                option::some(50),
                option::none(),
            );
        };

        test_scenario::next_tx(&mut scenario, AUTHOR);
        {
            let tip_coin = coin::mint_for_testing<MYSO>(1_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(tip_coin, AGENT_ADDR);
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut post = test_scenario::take_shared<Post>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut tip_coin = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);

            post::tip_post_simple<MYSO>(
                &mut post,
                &mut tip_coin,
                100,
                &memory_account,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(post);
            test_scenario::return_shared(memory_account);
            transfer::public_transfer(tip_coin, AGENT_ADDR);

            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }
}
