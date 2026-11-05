// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_const, duplicate_alias, unused_use)]
module social_contracts::platform_tests {
    use std::string;
    use std::option;
    use std::vector;
    
    use myso::test_scenario;
    use myso::object::{Self, ID};
    use myso::transfer;
    use myso::clock::{Self, Clock};
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::event;
    use myso::permissioned_group::PermissionedGroup;
    
    use social_contracts::profile::{Self, Profile, UsernameRegistry,
        ProfileConfig, EcosystemBadgeAdminCap};
    use social_contracts::memory::{Self, MemoryRegistry, MemoryAccount};
    use social_contracts::ai_credit::{Self, AiCreditBalance, AiCreditConfig};
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::social_graph::{Self, SocialGraph};
    use social_contracts::platform::{
        Self, Platform, PlatformRegistry, PlatformPackage, PlatformBlockAdmin, PlatformBadgeAdmin,
        PlatformConfig, PlatformBadgeLedger, EcosystemBadge, EcosystemBadgeRegistry};
    
    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const USER2: address = @0x2;
    const USER3: address = @0x3;
    const PLATFORM_ADMIN: address = @0xF1;
    const PLATFORM_MOD: address = @0xF2;
    const PLATFORM_USER: address = @0xF3;

    fun ascii_string(length: u64): string::String {
        let mut bytes = vector::empty<u8>();
        let mut i = 0;
        while (i < length) {
            vector::push_back(&mut bytes, 97);
            i = i + 1;
        };
        string::utf8(bytes)
    }

    #[test]
    fun test_fixed_platform_url_limits_accept_2048_bytes() {
        let url = ascii_string(2048);
        platform::test_validate_media_preview_url(&url);
        platform::test_validate_badge_urls(&url, &url);
    }

    #[test, expected_failure(abort_code = platform::EInvalidMediaPreviewUrl)]
    fun test_fixed_media_preview_url_limit_rejects_2049_bytes() {
        platform::test_validate_media_preview_url(&ascii_string(2049));
    }

    #[test, expected_failure(abort_code = platform::EBadgeMediaUrlTooLong)]
    fun test_fixed_badge_media_url_limit_rejects_2049_bytes() {
        platform::test_validate_badge_urls(&ascii_string(2049), &ascii_string(1));
    }

    #[test, expected_failure(abort_code = platform::EBadgeIconUrlTooLong)]
    fun test_fixed_badge_icon_url_limit_rejects_2049_bytes() {
        platform::test_validate_badge_urls(&ascii_string(1), &ascii_string(2049));
    }

    #[test]
    fun test_update_platform_config_with_five_values() {
        let mut scenario = test_scenario::begin(ADMIN);
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            platform::test_init(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };

        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let mut config = test_scenario::take_shared<PlatformConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let admin_cap = platform::create_platform_admin_cap(test_scenario::ctx(&mut scenario));
            platform::update_platform_config(
                &admin_cap,
                &mut config,
                2001,
                2049,
                11,
                101,
                501,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            transfer::public_transfer(admin_cap, ADMIN);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    fun create_test_platform_no_moderator(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            platform::test_init(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
        };

        test_scenario::next_tx(scenario, PLATFORM_ADMIN);
        {
            let platform_config = test_scenario::take_shared<PlatformConfig>(scenario);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            platform::create_platform(
                &mut registry,
                &platform_config,
                string::utf8(b"Test Platform"),
                string::utf8(b"A test platform"),
                string::utf8(b"This is a test platform for badge testing"),
                string::utf8(b"https://example.com/logo.png"),
                string::utf8(b"https://example.com/terms"),
                string::utf8(b"https://example.com/privacy"),
                vector[string::utf8(b"web"), string::utf8(b"mobile")],
                vector[string::utf8(b"https://example.com")],
                string::utf8(b"Social Network"),
                option::none(),
                2,
                string::utf8(b"2023-01-01"),
                true,
                option::some(7),
                option::some(30),
                option::some(50_000_000),
                option::some(5),
                option::some(5_000_000),
                option::some(3),
                option::some(15),
                option::none(),
                option::none(),

                option::none(),
                &clock,
                test_scenario::ctx(scenario)
            );

            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
        };

        test_scenario::next_tx(scenario, PLATFORM_ADMIN);
        {
            let platform = test_scenario::take_shared<Platform>(scenario);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let platform_id = object::uid_to_address(platform::id(&platform));
            platform::test_set_approval(&mut registry, platform_id, true);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(registry);
        };
    }

    // Helper function to create a test platform
    fun create_test_platform(scenario: &mut test_scenario::Scenario) {
        // Initialize the platform registry
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            platform::test_init(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
        };

        // Create a new platform in a separate transaction 
        test_scenario::next_tx(scenario, PLATFORM_ADMIN);
        {
            let platform_config = test_scenario::take_shared<PlatformConfig>(scenario);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            
            // Create platform
            platform::create_platform(
                &mut registry,
                &platform_config,
                string::utf8(b"Test Platform"),
                string::utf8(b"A test platform"),
                string::utf8(b"This is a test platform for badge testing"),
                string::utf8(b"https://example.com/logo.png"),
                string::utf8(b"https://example.com/terms"),
                string::utf8(b"https://example.com/privacy"),
                vector[string::utf8(b"web"), string::utf8(b"mobile")],
                vector[string::utf8(b"https://example.com")],
                string::utf8(b"Social Network"), // primary_category
                option::none(), // secondary_category
                2, // STATUS_BETA
                string::utf8(b"2023-01-01"),
                true, // wants_dao_governance
                option::some(7), // delegate_count
                option::some(30), // delegate_term_epochs
                option::some(50_000_000), // proposal_submission_cost
                option::some(5), // max_votes_per_user
                option::some(5_000_000), // quadratic_base_cost
                option::some(3), // voting_period_epochs
                option::some(15), // quorum_votes
                option::none(), // cover_photo
                option::none(), // media_previews
                option::none(), // redirect_uri
                &clock,
                test_scenario::ctx(scenario)
            );
            
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
        test_scenario::return_shared(platform_config);
        };

        // Add moderator to platform and set approval
        test_scenario::next_tx(scenario, PLATFORM_ADMIN);
        {
            let platform = test_scenario::take_shared<Platform>(scenario);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let mut group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(scenario);
            
            platform::add_moderator(
                &platform,
                &mut group,
                PLATFORM_MOD,
                test_scenario::ctx(scenario)
            );
            
            // Set platform as approved in registry
            let platform_id = object::uid_to_address(platform::id(&platform));
            platform::test_set_approval(&mut registry, platform_id, true);
            
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(registry);
        };
    }

    fun init_block_list_and_graph(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            block_list::test_init(&clock, test_scenario::ctx(scenario));

            social_graph::init_for_testing(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);

        };
    }

    // Helper function to create a test profile
    fun create_test_profile(scenario: &mut test_scenario::Scenario, owner: address, username: string::String) {
        // Make sure profile registry is initialized first
        if (!test_scenario::has_most_recent_shared<UsernameRegistry>()) {
            test_scenario::next_tx(scenario, ADMIN);
            {
                let clock = clock::create_for_testing(test_scenario::ctx(scenario));
                profile::init_for_testing(&clock, test_scenario::ctx(scenario));
                clock::share_for_testing(clock);

            };
        };
        
        // Now create the profile
        test_scenario::next_tx(scenario, owner);
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
                string::utf8(b"Test User"),
                username,
                string::utf8(b"This is a test profile for badges"),
                b"https://example.com/avatar.png",
                b"",
                &clock,
                test_scenario::ctx(scenario)
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(profile_config);
        };
    }

    
    
    
    
    
    
    #[test]
    fun test_create_platform_with_media_fields() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let mut registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            platform::create_platform(
                &mut registry,
                &platform_config,
                string::utf8(b"Media Platform"),
                string::utf8(b"Tagline"),
                string::utf8(b"Description"),
                string::utf8(b"https://example.com/logo.png"),
                string::utf8(b"https://example.com/terms"),
                string::utf8(b"https://example.com/privacy"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example.com")],
                string::utf8(b"Social Network"),
                option::none(),
                3,
                string::utf8(b"2024-01-01"),
                false,
                option::none(), option::none(), option::none(), option::none(),
                option::none(), option::none(), option::none(),
                option::some(string::utf8(b"https://example.com/cover.png")),
                option::some(vector[
                    string::utf8(b"https://example.com/preview1.png"),
                    string::utf8(b"https://example.com/preview2.mp4"),
                ]),

                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(registry);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let cover = platform::cover_photo(&platform);
            assert!(option::is_some(cover), 0);
            assert!(
                *option::borrow(cover) == string::utf8(b"https://example.com/cover.png"),
                1
            );
            let previews = platform::media_previews(&platform);
            assert!(option::is_some(previews), 2);
            assert!(vector::length(option::borrow(previews)) == 2, 3);
            test_scenario::return_shared(platform);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_update_platform_media_and_logo() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scenario);
            platform::update_platform(
                &mut platform,
                &platform_config,
                string::utf8(b"Test Platform"),
                string::utf8(b"A test platform"),
                string::utf8(b"This is a test platform for badge testing"),
                string::utf8(b"https://example.com/new-logo.png"),
                string::utf8(b"https://example.com/terms"),
                string::utf8(b"https://example.com/privacy"),
                vector[string::utf8(b"web"), string::utf8(b"mobile")],
                vector[string::utf8(b"https://example.com")],
                string::utf8(b"Social Network"),
                option::none(),
                2,
                string::utf8(b"2023-01-01"),
                option::none(),
                option::some(string::utf8(b"https://example.com/new-cover.png")),
                option::some(vector[string::utf8(b"https://example.com/shot.png")]),

                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(
                *platform::logo(&platform) == string::utf8(b"https://example.com/new-logo.png"),
                0
            );
            let cover = platform::cover_photo(&platform);
            assert!(option::is_some(cover), 1);
            let previews = platform::media_previews(&platform);
            assert!(option::is_some(previews), 2);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_create_platform_with_redirect_uri() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let mut registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            platform::create_platform(
                &mut registry,
                &platform_config,
                string::utf8(b"Redirect Platform"),
                string::utf8(b"Tagline"),
                string::utf8(b"Description"),
                string::utf8(b"https://example.com/logo.png"),
                string::utf8(b"https://example.com/terms"),
                string::utf8(b"https://example.com/privacy"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example.com")],
                string::utf8(b"Social Network"),
                option::none(),
                3,
                string::utf8(b"2024-01-01"),
                false,
                option::none(), option::none(), option::none(), option::none(),
                option::none(), option::none(), option::none(),
                option::none(),
                option::none(),
                option::some(string::utf8(b"https://app.example.com/auth/callback")),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(registry);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let platform = test_scenario::take_shared<Platform>(&scenario);
            assert!(
                platform::name(&platform) == string::utf8(b"Redirect Platform"),
                0
            );
            let redirect = platform::redirect_uri(&platform);
            assert!(option::is_some(redirect), 1);
            assert!(
                *option::borrow(redirect) == string::utf8(b"https://app.example.com/auth/callback"),
                2
            );
            test_scenario::return_shared(platform);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_update_platform_redirect_uri() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scenario);
            platform::update_platform(
                &mut platform,
                &platform_config,
                string::utf8(b"Test Platform"),
                string::utf8(b"A test platform"),
                string::utf8(b"This is a test platform for badge testing"),
                string::utf8(b"https://example.com/logo.png"),
                string::utf8(b"https://example.com/terms"),
                string::utf8(b"https://example.com/privacy"),
                vector[string::utf8(b"web"), string::utf8(b"mobile")],
                vector[string::utf8(b"https://example.com")],
                string::utf8(b"Social Network"),
                option::none(),
                2,
                string::utf8(b"2023-01-01"),
                option::none(),
                option::none(),
                option::none(),
                option::some(string::utf8(b"https://dripdrop.social/auth/callback")),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            let redirect = platform::redirect_uri(&platform);
            assert!(option::is_some(redirect), 0);
            assert!(
                *option::borrow(redirect) == string::utf8(b"https://dripdrop.social/auth/callback"),
                1
            );
            test_scenario::return_shared(platform);
            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = platform::ETooManyMediaPreviews)]
    fun test_create_platform_rejects_too_many_media_previews() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let mut registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scenario);
            let mut previews = vector::empty<string::String>();
            let mut i = 0u64;
            while (i < 11) {
                vector::push_back(&mut previews, string::utf8(b"https://example.com/p.png"));
                i = i + 1;
            };

            platform::create_platform(
                &mut registry,
                &platform_config,
                string::utf8(b"Too Many Previews"),
                string::utf8(b"Tag"),
                string::utf8(b"Desc"),
                string::utf8(b"https://example.com/logo.png"),
                string::utf8(b"https://example.com/terms"),
                string::utf8(b"https://example.com/privacy"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example.com")],
                string::utf8(b"Social Network"),
                option::none(),
                3,
                string::utf8(b"2024-01-01"),
                false,
                option::none(), option::none(), option::none(), option::none(),
                option::none(), option::none(), option::none(),
                option::none(),
                option::some(previews),

                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(registry);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_full_moderator_can_block_and_badge() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);
        init_block_list_and_graph(&mut scenario);
        create_test_profile(&mut scenario, PLATFORM_USER, string::utf8(b"full_mod_user"));

        test_scenario::next_tx(&mut scenario, PLATFORM_MOD);
        {
            let mut block_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);

            social_graph::block_platform_wallet(
                &mut block_registry,
                &mut social_graph,
                &mut platform,
                &group,
                PLATFORM_USER,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(social_graph);
            test_scenario::return_shared(block_registry);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_MOD);
        {

let platform_config = test_scenario::take_shared<PlatformConfig>(&scenario);

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut ledger = test_scenario::take_shared<PlatformBadgeLedger>(&scenario);

            platform::assign_shared_badge(
                &registry,
                &platform_config,
                &platform,
                &group,
                &mut ledger,
                PLATFORM_USER,
                string::utf8(b"ModBadge"),
                string::utf8(b"Moderator badge"),
                string::utf8(b"https://example.com/mod.png"),
                string::utf8(b"https://example.com/mod_icon.png"),
                1,
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            let platform_id = object::uid_to_address(platform::id(&platform));
            let mut badge_id = string::utf8(b"badge_");
            string::append(&mut badge_id, myso::address::to_string(platform_id));
            string::append(&mut badge_id, string::utf8(b"_ModBadge"));
            assert!(platform::has_active_platform_badge(&ledger, PLATFORM_USER, &badge_id, &clock), 0);

            test_scenario::return_shared(ledger);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);

            test_scenario::return_shared(clock);
        test_scenario::return_shared(platform_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_badge_only_moderator() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform_no_moderator(&mut scenario);
        init_block_list_and_graph(&mut scenario);
        create_test_profile(&mut scenario, PLATFORM_USER, string::utf8(b"badge_only_user"));

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let mut group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            platform::grant_moderator_permission<PlatformBadgeAdmin>(
                &platform,
                &mut group,
                PLATFORM_MOD,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_MOD);
        {

let platform_config = test_scenario::take_shared<PlatformConfig>(&scenario);

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut ledger = test_scenario::take_shared<PlatformBadgeLedger>(&scenario);

            platform::assign_shared_badge(
                &registry,
                &platform_config,
                &platform,
                &group,
                &mut ledger,
                PLATFORM_USER,
                string::utf8(b"BadgeOnly"),
                string::utf8(b"Moderator badge"),
                string::utf8(b"https://example.com/mod.png"),
                string::utf8(b"https://example.com/mod_icon.png"),
                1,
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            let platform_id = object::uid_to_address(platform::id(&platform));
            let mut badge_id = string::utf8(b"badge_");
            string::append(&mut badge_id, myso::address::to_string(platform_id));
            string::append(&mut badge_id, string::utf8(b"_BadgeOnly"));
            assert!(platform::has_active_platform_badge(&ledger, PLATFORM_USER, &badge_id, &clock), 0);

            test_scenario::return_shared(ledger);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);

            test_scenario::return_shared(clock);
        test_scenario::return_shared(platform_config);
        };

        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = platform::EUnauthorized)]
    fun test_badge_only_moderator_cannot_block() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform_no_moderator(&mut scenario);
        init_block_list_and_graph(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let mut group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            platform::grant_moderator_permission<PlatformBadgeAdmin>(
                &platform,
                &mut group,
                PLATFORM_MOD,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_MOD);
        {
            let mut block_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);

            social_graph::block_platform_wallet(
                &mut block_registry,
                &mut social_graph,
                &mut platform,
                &group,
                PLATFORM_USER,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(social_graph);
            test_scenario::return_shared(block_registry);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_block_only_moderator_can_block() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform_no_moderator(&mut scenario);
        init_block_list_and_graph(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let mut group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            platform::grant_moderator_permission<PlatformBlockAdmin>(
                &platform,
                &mut group,
                PLATFORM_MOD,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_MOD);
        {
            let mut block_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);

            social_graph::block_platform_wallet(
                &mut block_registry,
                &mut social_graph,
                &mut platform,
                &group,
                PLATFORM_USER,
                test_scenario::ctx(&mut scenario),
            );

            let platform_address = object::uid_to_address(platform::id(&platform));
            assert!(block_list::is_blocked(&block_registry, platform_address, PLATFORM_USER), 0);

            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(social_graph);
            test_scenario::return_shared(block_registry);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_revoke_last_permission_removes_moderator_status() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform_no_moderator(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let mut group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            platform::grant_moderator_permission<PlatformBadgeAdmin>(
                &platform,
                &mut group,
                PLATFORM_MOD,
                test_scenario::ctx(&mut scenario),
            );
            assert!(platform::is_moderator(&group, &platform, PLATFORM_MOD), 0);

            platform::revoke_moderator_permission<PlatformBadgeAdmin>(
                &platform,
                &mut group,
                PLATFORM_MOD,
                test_scenario::ctx(&mut scenario),
            );
            assert!(!platform::is_moderator(&group, &platform, PLATFORM_MOD), 1);

            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
        };

        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = platform::EUnauthorized)]
    fun test_non_developer_cannot_grant_permissions() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_MOD);
        {
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let mut group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            platform::grant_moderator_permission<PlatformBlockAdmin>(
                &platform,
                &mut group,
                USER3,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_developer_bypass_without_explicit_extension_grants() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform_no_moderator(&mut scenario);
        init_block_list_and_graph(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let mut block_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut social_graph = test_scenario::take_shared<SocialGraph>(&scenario);
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);

            social_graph::block_platform_wallet(
                &mut block_registry,
                &mut social_graph,
                &mut platform,
                &group,
                PLATFORM_USER,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(social_graph);
            test_scenario::return_shared(block_registry);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_platform_treasury_fund_and_withdraw() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut funding_coin = coin::mint_for_testing<MYSO>(500_000_000, test_scenario::ctx(&mut scenario));

            platform::test_fund_platform_treasury(
                &mut platform,
                &mut funding_coin,
                500_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(platform::treasury_balance<MYSO>(&platform) == 500_000_000, 0);

            coin::destroy_zero(funding_coin);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(platform);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let recipients = vector[USER1];
            platform::withdraw_from_platform_treasury<MYSO>(
                &mut platform,
                &group,
                recipients,
                100_000_000,
                1,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(platform::treasury_balance<MYSO>(&platform) == 400_000_000, 1);

            test_scenario::return_shared(clock);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let withdrawn = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            assert!(coin::value(&withdrawn) == 100_000_000, 2);
            coin::burn_for_testing(withdrawn);
        };

        test_scenario::end(scenario);
    }

    #[test, expected_failure(abort_code = platform::EUnauthorized)]
    fun test_platform_treasury_withdraw_requires_permission() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform_no_moderator(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut funding_coin = coin::mint_for_testing<MYSO>(100_000_000, test_scenario::ctx(&mut scenario));
            platform::test_fund_platform_treasury(
                &mut platform,
                &mut funding_coin,
                100_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            coin::destroy_zero(funding_coin);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(platform);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let recipients = vector[USER2];
            platform::withdraw_from_platform_treasury<MYSO>(
                &mut platform,
                &group,
                recipients,
                10_000_000,
                1,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
        };

        test_scenario::end(scenario);
    }

    public struct TEST_USD has drop {}

    #[test]
    fun test_platform_treasury_holds_multiple_coin_types() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut myso_coin = coin::mint_for_testing<MYSO>(100, test_scenario::ctx(&mut scenario));
            let mut usd_coin = coin::mint_for_testing<TEST_USD>(50, test_scenario::ctx(&mut scenario));

            platform::test_fund_platform_treasury(
                &mut platform,
                &mut myso_coin,
                100,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            platform::test_fund_platform_treasury(
                &mut platform,
                &mut usd_coin,
                50,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            assert!(platform::treasury_balance<MYSO>(&platform) == 100, 0);
            assert!(platform::treasury_balance<TEST_USD>(&platform) == 50, 1);
            assert!(platform::treasury_has_balance<MYSO>(&platform), 2);
            assert!(platform::treasury_has_balance<TEST_USD>(&platform), 3);

            coin::destroy_zero(myso_coin);
            coin::destroy_zero(usd_coin);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(platform);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_assign_shared_badge_without_profile() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut ledger = test_scenario::take_shared<PlatformBadgeLedger>(&scenario);

            platform::assign_shared_badge(
                &registry,
                &platform_config,
                &platform,
                &group,
                &mut ledger,
                PLATFORM_USER,
                string::utf8(b"Premium+"),
                string::utf8(b"Premium membership"),
                string::utf8(b"https://example.com/premium.png"),
                string::utf8(b"https://example.com/premium_icon.png"),
                10,
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            assert!(platform::badge_ledger_id(&platform) == object::id(&ledger), 0);

            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(group);
            test_scenario::return_shared(ledger);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_USER);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let mut ledger = test_scenario::take_shared<PlatformBadgeLedger>(&scenario);
            let platform_id = object::uid_to_address(platform::id(&platform));
            let mut badge_id = string::utf8(b"badge_");
            string::append(&mut badge_id, myso::address::to_string(platform_id));
            string::append(&mut badge_id, string::utf8(b"_Premium+"));

            assert!(platform::has_active_platform_badge(&ledger, PLATFORM_USER, &badge_id, &clock), 1);

            platform::select_shared_badge(
                &mut ledger,
                badge_id,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            assert!(option::is_some(&platform::selected_platform_badge_id(&ledger, PLATFORM_USER)), 2);

            test_scenario::return_shared(clock);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(ledger);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut ledger = test_scenario::take_shared<PlatformBadgeLedger>(&scenario);
            let platform_id = object::uid_to_address(platform::id(&platform));
            let mut badge_id = string::utf8(b"badge_");
            string::append(&mut badge_id, myso::address::to_string(platform_id));
            string::append(&mut badge_id, string::utf8(b"_Premium+"));

            platform::extend_shared_badge(
                &registry,
                &platform,
                &group,
                &mut ledger,
                PLATFORM_USER,
                badge_id,
                option::some(clock::timestamp_ms(&clock) + 1_000),
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(group);
            test_scenario::return_shared(ledger);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut ledger = test_scenario::take_shared<PlatformBadgeLedger>(&scenario);
            let platform_id = object::uid_to_address(platform::id(&platform));
            let mut badge_id = string::utf8(b"badge_");
            string::append(&mut badge_id, myso::address::to_string(platform_id));
            string::append(&mut badge_id, string::utf8(b"_Premium+"));

            platform::revoke_shared_badge(
                &registry,
                &platform,
                &group,
                &mut ledger,
                PLATFORM_USER,
                badge_id,
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            let mut check_id = string::utf8(b"badge_");
            string::append(&mut check_id, myso::address::to_string(platform_id));
            string::append(&mut check_id, string::utf8(b"_Premium+"));
            assert!(!platform::has_active_platform_badge(&ledger, PLATFORM_USER, &check_id, &clock), 3);

            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(group);
            test_scenario::return_shared(ledger);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_select_ecosystem_badge_on_ledger() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);
        create_test_profile(&mut scenario, PLATFORM_USER, string::utf8(b"eco_user"));

        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            platform::init_ecosystem_badge_registry_for_testing(test_scenario::ctx(&mut scenario));
        };

        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut badge_registry = test_scenario::take_shared<EcosystemBadgeRegistry>(&scenario);
            let cap = profile::create_ecosystem_badge_admin_cap(test_scenario::ctx(&mut scenario));
            platform::mint_ecosystem_badge(
                &cap,
                &mut badge_registry,
                PLATFORM_USER,
                string::utf8(b"Verified"),
                string::utf8(b"Ecosystem verified"),
                string::utf8(b"https://example.com/eco.png"),
                string::utf8(b"https://example.com/eco_icon.png"),
                5,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            transfer::public_transfer(cap, ADMIN);
            test_scenario::return_shared(badge_registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_USER);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let badge = test_scenario::take_from_sender<EcosystemBadge>(&scenario);
            let mut ledger = test_scenario::take_shared<PlatformBadgeLedger>(&scenario);

            platform::select_ecosystem_badge(
                &mut ledger,
                &badge,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            let selected = platform::selected_ecosystem_badge_id(&ledger, PLATFORM_USER);
            assert!(selected == option::some(string::utf8(b"ecosystem_badge_Verified")), 1);

            test_scenario::return_to_sender(&scenario, badge);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(ledger);
        };

        test_scenario::end(scenario);
    }

    fun create_named_platform(scenario: &mut test_scenario::Scenario, name: vector<u8>): ID {
        test_scenario::next_tx(scenario, PLATFORM_ADMIN);
        {
            let platform_config = test_scenario::take_shared<PlatformConfig>(scenario);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            platform::create_platform(
                &mut registry,
                &platform_config,
                string::utf8(name),
                string::utf8(b"A test platform"),
                string::utf8(b"This is a test platform for badge testing"),
                string::utf8(b"https://example.com/logo.png"),
                string::utf8(b"https://example.com/terms"),
                string::utf8(b"https://example.com/privacy"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example.com")],
                string::utf8(b"Social Network"),
                option::none(),
                2,
                string::utf8(b"2023-01-01"),
                true,
                option::some(7),
                option::some(30),
                option::some(50_000_000),
                option::some(5),
                option::some(5_000_000),
                option::some(3),
                option::some(15),
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(platform_config);
        };
        test_scenario::next_tx(scenario, PLATFORM_ADMIN);
        let platform_id = test_scenario::most_recent_id_shared<Platform>().destroy_some();
        {
            let platform = test_scenario::take_shared_by_id<Platform>(scenario, platform_id);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            platform::test_set_approval(&mut registry, object::id_to_address(&platform_id), true);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(registry);
        };
        platform_id
    }

    #[test]
    fun test_leave_platforms_emits_left_event_and_keeps_developer() {
        let mut scenario = test_scenario::begin(ADMIN);
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            platform::test_init(&clock, test_scenario::ctx(&mut scenario));
            block_list::test_init(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };

        let first_id = create_named_platform(&mut scenario, b"First Platform");
        let second_id = create_named_platform(&mut scenario, b"Second Platform");

        test_scenario::next_tx(&mut scenario, PLATFORM_USER);
        {
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let block_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut first = test_scenario::take_shared_by_id<Platform>(&scenario, first_id);
            let mut second = test_scenario::take_shared_by_id<Platform>(&scenario, second_id);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            platform::join_platform(
                &registry,
                &block_registry,
                &mut first,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            platform::join_platform(
                &registry,
                &block_registry,
                &mut second,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(second);
            test_scenario::return_shared(first);
            test_scenario::return_shared(block_registry);
            test_scenario::return_shared(registry);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_USER);
        {
            let first = test_scenario::take_shared_by_id<Platform>(&scenario, first_id);
            let second = test_scenario::take_shared_by_id<Platform>(&scenario, second_id);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            platform::leave_platforms(
                vector[first, second],
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            let left = event::events_by_type<platform::UserLeftPlatformEvent>();
            assert!(vector::length(&left) == 2, 0);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let first = test_scenario::take_shared_by_id<Platform>(&scenario, first_id);
            let second = test_scenario::take_shared_by_id<Platform>(&scenario, second_id);
            assert!(!platform::has_joined_platform(&first, PLATFORM_USER), 0);
            assert!(!platform::has_joined_platform(&second, PLATFORM_USER), 1);
            assert!(platform::developer(&first) == PLATFORM_ADMIN, 2);
            assert!(platform::developer(&second) == PLATFORM_ADMIN, 3);
            test_scenario::return_shared(second);
            test_scenario::return_shared(first);
        };

        test_scenario::end(scenario);
    }

    // =====================================================================
    // Soulbound ecosystem badges + owned-profile owner rotation
    // =====================================================================

    fun eb_setup(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(scenario));
            platform::init_ecosystem_badge_registry_for_testing(test_scenario::ctx(scenario));
            let cap = profile::create_ecosystem_badge_admin_cap(test_scenario::ctx(scenario));
            transfer::public_transfer(cap, ADMIN);
            clock::share_for_testing(clock);
        };
    }

    fun eb_create_profile_for(scenario: &mut test_scenario::Scenario, owner: address, username: vector<u8>) {
        test_scenario::next_tx(scenario, owner);
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
                string::utf8(b"User"),
                string::utf8(username),
                string::utf8(b"bio"),
                vector::empty(),
                vector::empty(),
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

    fun eb_mint_badge(scenario: &mut test_scenario::Scenario, recipient: address, name: vector<u8>) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let cap = test_scenario::take_from_sender<EcosystemBadgeAdminCap>(scenario);
            let mut registry = test_scenario::take_shared<EcosystemBadgeRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            platform::mint_ecosystem_badge(
                &cap,
                &mut registry,
                recipient,
                string::utf8(name),
                string::utf8(b"desc"),
                string::utf8(b"https://example.com/m.png"),
                string::utf8(b"https://example.com/i.png"),
                2,
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
            test_scenario::return_to_sender(scenario, cap);
        };
    }

    fun eb_mint_verified_x(scenario: &mut test_scenario::Scenario, recipient: address, handle: vector<u8>) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let cap = test_scenario::take_from_sender<EcosystemBadgeAdminCap>(scenario);
            let mut registry = test_scenario::take_shared<EcosystemBadgeRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            platform::mint_verified_x_badge(
                &cap,
                &mut registry,
                recipient,
                string::utf8(handle),
                string::utf8(b"verified_x_account"),
                string::utf8(b"Verified X account"),
                string::utf8(b"https://example.com/x.png"),
                string::utf8(b"https://example.com/x_icon.png"),
                1,
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
            test_scenario::return_to_sender(scenario, cap);
        };
    }

    fun eb_burn_from(scenario: &mut test_scenario::Scenario, holder: address) {
        test_scenario::next_tx(scenario, holder);
        {
            let mut registry = test_scenario::take_shared<EcosystemBadgeRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            let badge = test_scenario::take_from_sender<EcosystemBadge>(scenario);
            platform::burn_ecosystem_badge(&mut registry, badge, &clock, test_scenario::ctx(scenario));
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
        };
    }

    #[test]
    fun test_eco_badge_mint_to_wallet_without_profile() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_mint_badge(&mut scen, USER1, b"early_adopter");

        test_scenario::next_tx(&mut scen, USER1);
        {
            let badge = test_scenario::take_from_sender<EcosystemBadge>(&scen);
            let registry = test_scenario::take_shared<EcosystemBadgeRegistry>(&scen);
            assert!(platform::ecosystem_badge_id(&badge) == string::utf8(b"ecosystem_badge_early_adopter"), 0);
            assert!(platform::ecosystem_badge_name(&badge) == string::utf8(b"early_adopter"), 1);
            assert!(platform::ecosystem_badge_issued_by(&badge) == ADMIN, 2);
            assert!(option::is_none(&platform::ecosystem_badge_subject(&badge)), 3);
            assert!(platform::has_ecosystem_badge(&registry, USER1, string::utf8(b"ecosystem_badge_early_adopter")), 4);
            assert!(!platform::has_ecosystem_badge(&registry, USER2, string::utf8(b"ecosystem_badge_early_adopter")), 5);
            test_scenario::return_to_sender(&scen, badge);
            test_scenario::return_shared(registry);
        };
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = platform::EEcosystemBadgeAlreadyIssued, location = social_contracts::platform)]
    fun test_eco_badge_duplicate_mint_aborts() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_mint_badge(&mut scen, USER1, b"ambassador");
        eb_mint_badge(&mut scen, USER1, b"ambassador");
        test_scenario::end(scen);
    }

    #[test]
    fun test_eco_badge_same_badge_for_different_wallets() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_mint_badge(&mut scen, USER1, b"ambassador");
        eb_mint_badge(&mut scen, USER2, b"ambassador");
        test_scenario::next_tx(&mut scen, USER2);
        {
            assert!(test_scenario::has_most_recent_for_sender<EcosystemBadge>(&scen), 0);
        };
        test_scenario::end(scen);
    }

    #[test]
    fun test_eco_badge_verified_x_binds_canonical_handle() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_mint_verified_x(&mut scen, USER1, b"@Alice_X");

        test_scenario::next_tx(&mut scen, USER1);
        {
            let badge = test_scenario::take_from_sender<EcosystemBadge>(&scen);
            let registry = test_scenario::take_shared<EcosystemBadgeRegistry>(&scen);
            assert!(platform::ecosystem_badge_subject(&badge) == option::some(string::utf8(b"alice_x")), 0);
            assert!(platform::x_handle_of(&registry, USER1) == option::some(string::utf8(b"alice_x")), 1);
            assert!(platform::wallet_of_x_handle(&registry, string::utf8(b"ALICE_X")) == option::some(USER1), 2);
            test_scenario::return_to_sender(&scen, badge);
            test_scenario::return_shared(registry);
        };
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = platform::EXHandleTaken, location = social_contracts::platform)]
    fun test_eco_badge_x_handle_taken_by_other_wallet_aborts() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_mint_verified_x(&mut scen, USER1, b"alice");
        eb_mint_verified_x(&mut scen, USER2, b"Alice");
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = platform::EWalletHasXHandle, location = social_contracts::platform)]
    fun test_eco_badge_wallet_cannot_verify_second_handle() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_mint_verified_x(&mut scen, USER1, b"alice");
        eb_mint_verified_x(&mut scen, USER1, b"bob");
        test_scenario::end(scen);
    }

    #[test]
    fun test_eco_badge_burn_frees_badge_and_handle_for_reissue() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_mint_verified_x(&mut scen, USER1, b"alice");
        eb_burn_from(&mut scen, USER1);

        test_scenario::next_tx(&mut scen, USER1);
        {
            let registry = test_scenario::take_shared<EcosystemBadgeRegistry>(&scen);
            assert!(!platform::has_ecosystem_badge(&registry, USER1, string::utf8(b"ecosystem_badge_verified_x_account")), 0);
            assert!(option::is_none(&platform::wallet_of_x_handle(&registry, string::utf8(b"alice"))), 1);
            assert!(!test_scenario::has_most_recent_for_sender<EcosystemBadge>(&scen), 2);
            test_scenario::return_shared(registry);
        };

        // The handle can now be verified by another wallet, and USER1 can re-link.
        eb_mint_verified_x(&mut scen, USER2, b"alice");
        eb_mint_verified_x(&mut scen, USER1, b"newhandle");
        test_scenario::end(scen);
    }

    #[test]
    fun test_eco_badge_select_on_profile() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_create_profile_for(&mut scen, USER1, b"user_one");
        eb_mint_verified_x(&mut scen, USER1, b"alice");

        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut p = test_scenario::take_from_sender<Profile>(&scen);
            let badge = test_scenario::take_from_sender<EcosystemBadge>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            platform::select_ecosystem_badge_on_profile(&mut p, &badge, &clock, test_scenario::ctx(&mut scen));
            assert!(
                profile::get_selected_ecosystem_badge_id(&p)
                    == option::some(string::utf8(b"ecosystem_badge_verified_x_account")),
                0,
            );
            profile::clear_selected_ecosystem_badge(&mut p, &clock, test_scenario::ctx(&mut scen));
            assert!(option::is_none(&profile::get_selected_ecosystem_badge_id(&p)), 1);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scen, badge);
            test_scenario::return_to_sender(&scen, p);
        };
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = profile::EUnauthorized, location = social_contracts::profile)]
    fun test_eco_badge_select_on_someone_elses_profile_aborts() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_create_profile_for(&mut scen, USER1, b"user_one");
        eb_mint_badge(&mut scen, USER2, b"ambassador");

        test_scenario::next_tx(&mut scen, USER2);
        {
            let mut p = test_scenario::take_from_address<Profile>(&scen, USER1);
            let badge = test_scenario::take_from_sender<EcosystemBadge>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            platform::select_ecosystem_badge_on_profile(&mut p, &badge, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scen, badge);
            test_scenario::return_to_address(USER1, p);
        };
        test_scenario::end(scen);
    }

    // === Profile owner rotation (owned profile handed over by value) ===

    fun eb_propose_rotation(scenario: &mut test_scenario::Scenario, from: address, to: address) {
        test_scenario::next_tx(scenario, from);
        {
            let registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let memory_registry = test_scenario::take_shared<MemoryRegistry>(scenario);
            let mut account = test_scenario::take_shared<MemoryAccount>(scenario);
            let p = test_scenario::take_from_sender<Profile>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            profile::propose_owner_rotation(
                &registry, &memory_registry, &mut account, &p, to,
                vector::empty(), vector::empty(), &clock, test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(scenario, p);
            test_scenario::return_shared(account);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
        };
    }

    fun eb_accept_rotation(scenario: &mut test_scenario::Scenario, recipient: address) {
        test_scenario::next_tx(scenario, recipient);
        {
            let mut account = test_scenario::take_shared<MemoryAccount>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            memory::accept_owner_rotation(&mut account, &clock, test_scenario::ctx(scenario));
            test_scenario::return_shared(clock);
            test_scenario::return_shared(account);
        };
    }

    fun eb_complete_rotation(scenario: &mut test_scenario::Scenario, from: address) {
        test_scenario::next_tx(scenario, from);
        {
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(scenario);
            let mut account = test_scenario::take_shared<MemoryAccount>(scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(scenario);
            let p = test_scenario::take_from_sender<Profile>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            profile::complete_owner_rotation(
                &mut registry, &mut memory_registry, &mut account, &mut balance, p,
                &clock, test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(account);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
        };
    }

    #[test]
    fun test_eco_badge_owner_rotation_moves_profile_and_all_ownership_atomically() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_create_profile_for(&mut scen, USER1, b"user_one");
        eb_mint_badge(&mut scen, USER1, b"ambassador");

        // Display the badge so we can check the selection is cleared on rotation.
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut p = test_scenario::take_from_sender<Profile>(&scen);
            let badge = test_scenario::take_from_sender<EcosystemBadge>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            platform::select_ecosystem_badge_on_profile(&mut p, &badge, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scen, badge);
            test_scenario::return_to_sender(&scen, p);
        };

        eb_propose_rotation(&mut scen, USER1, USER2);
        eb_accept_rotation(&mut scen, USER2);
        eb_complete_rotation(&mut scen, USER1);

        test_scenario::next_tx(&mut scen, USER2);
        {
            let p = test_scenario::take_from_sender<Profile>(&scen);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scen);
            let account = test_scenario::take_shared<MemoryAccount>(&scen);
            let balance = test_scenario::take_shared<AiCreditBalance>(&scen);
            let pid = object::uid_to_address(profile::id(&p));

            assert!(profile::owner(&p) == USER2, 0);
            assert!(profile::lookup_profile_by_owner(&registry, USER2) == option::some(pid), 1);
            assert!(option::is_none(&profile::lookup_profile_by_owner(&registry, USER1)), 2);
            assert!(memory::owner(&account) == USER2, 3);
            assert!(ai_credit::balance_principal_owner(&balance) == USER2, 4);
            assert!(option::is_none(&profile::get_selected_ecosystem_badge_id(&p)), 5);
            assert!(profile::lookup_profile_by_username(&registry, string::utf8(b"user_one")) == option::some(pid), 6);

            test_scenario::return_shared(balance);
            test_scenario::return_shared(account);
            test_scenario::return_shared(registry);
            test_scenario::return_to_sender(&scen, p);
        };

        // Soulbound: the badge stayed with the old wallet.
        test_scenario::next_tx(&mut scen, USER1);
        {
            assert!(test_scenario::has_most_recent_for_sender<EcosystemBadge>(&scen), 7);
            assert!(!test_scenario::has_most_recent_for_sender<Profile>(&scen), 8);
        };
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = memory::EOwnerRotationNotAccepted, location = social_contracts::memory)]
    fun test_eco_badge_owner_rotation_requires_recipient_acceptance() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_create_profile_for(&mut scen, USER1, b"user_one");
        eb_propose_rotation(&mut scen, USER1, USER2);
        eb_complete_rotation(&mut scen, USER1);
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = memory::EOwnerRotationWrongWallet, location = social_contracts::memory)]
    fun test_eco_badge_owner_rotation_accept_by_wrong_wallet_aborts() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_create_profile_for(&mut scen, USER1, b"user_one");
        eb_propose_rotation(&mut scen, USER1, USER2);
        eb_accept_rotation(&mut scen, ADMIN);
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = memory::EOwnerRotationNotPending, location = social_contracts::memory)]
    fun test_eco_badge_cancelled_owner_rotation_cannot_complete() {
        let mut scen = test_scenario::begin(ADMIN);
        eb_setup(&mut scen);
        eb_create_profile_for(&mut scen, USER1, b"user_one");
        eb_propose_rotation(&mut scen, USER1, USER2);
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut account = test_scenario::take_shared<MemoryAccount>(&scen);
            memory::cancel_owner_rotation(&mut account, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(account);
        };
        eb_complete_rotation(&mut scen, USER1);
        test_scenario::end(scen);
    }
}
