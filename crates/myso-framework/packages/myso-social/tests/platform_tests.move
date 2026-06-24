// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_const, duplicate_alias, unused_use)]
module social_contracts::platform_tests {
    use std::string;
    use std::option;
    use std::vector;
    
    use myso::test_scenario;
    use myso::object;
    use myso::transfer;
    use myso::clock::{Self, Clock};
    use myso::permissioned_group::PermissionedGroup;
    
    use social_contracts::profile::{Self, Profile, UsernameRegistry};
    use social_contracts::memory::{MemoryRegistry, MemoryAccount};
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::social_graph::{Self, SocialGraph};
    use social_contracts::platform::{
        Self, Platform, PlatformRegistry, PlatformPackage, PlatformBlockAdmin, PlatformBadgeAdmin,
    };
    
    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const USER2: address = @0x2;
    const USER3: address = @0x3;
    const PLATFORM_ADMIN: address = @0xF1;
    const PLATFORM_MOD: address = @0xF2;
    const PLATFORM_USER: address = @0xF3;
    
    fun create_test_platform_no_moderator(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            platform::test_init(test_scenario::ctx(scenario));
        };

        test_scenario::next_tx(scenario, ADMIN);
        {
            let c = clock::create_for_testing(test_scenario::ctx(scenario));
            clock::share_for_testing(c);
        };

        test_scenario::next_tx(scenario, PLATFORM_ADMIN);
        {
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            platform::create_platform(
                &mut registry,
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
                &clock,
                test_scenario::ctx(scenario)
            );

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
            platform::test_init(test_scenario::ctx(scenario));
        };

        test_scenario::next_tx(scenario, ADMIN);
        {
            let c = clock::create_for_testing(test_scenario::ctx(scenario));
            clock::share_for_testing(c);
        };

        // Create a new platform in a separate transaction 
        test_scenario::next_tx(scenario, PLATFORM_ADMIN);
        {
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            
            // Create platform
            platform::create_platform(
                &mut registry,
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
                &clock,
                test_scenario::ctx(scenario)
            );
            
            test_scenario::return_shared(clock);
            test_scenario::return_shared(registry);
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
            let clock = test_scenario::take_shared<Clock>(scenario);

            profile::create_profile(
                &mut registry,
                &mut memory_registry,
                string::utf8(b"Test User"),
                username,
                string::utf8(b"This is a test profile for badges"),
                b"https://example.com/avatar.png",
                b"",
                &clock,
                test_scenario::ctx(scenario)
            );

            test_scenario::return_shared(clock);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
        };
    }
    
    #[test]
    fun test_platform_admin_assigns_badge() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize the clock first
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            // Create test clock
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };
        
        // Create platform and profile
        create_test_platform(&mut scenario);
        create_test_profile(&mut scenario, PLATFORM_USER, string::utf8(b"platform_user"));
        
        // Platform admin assigns a badge to the user's profile
        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut user_profile = test_scenario::take_from_address<Profile>(&scenario, PLATFORM_USER);
            
            let badge_name = string::utf8(b"VIP");
            
            // Assign badge
            platform::assign_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                badge_name,
                string::utf8(b"Very Important Person"),
                string::utf8(b"https://example.com/vip_badge.png"),
                string::utf8(b"https://example.com/vip_badge_icon.png"),
                10,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Verify badge was assigned
            let badges = profile::get_profile_badges(&user_profile);
            assert!(vector::length(&badges) == 1, 1);
            
            // Verify badge count
            assert!(profile::badge_count(&user_profile) == 1, 2);
            
            // Verify badge details - get badge data and extract badge_id
            let badge_data = *vector::borrow(&badges, 0);
            let badge_id = profile::badge_data_id(&badge_data);
            assert!(profile::has_badge(&user_profile, &badge_id), 3);
            
            let badge_opt = profile::get_badge(&user_profile, &badge_id);
            assert!(option::is_some(&badge_opt), 4);
            
            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_to_address(PLATFORM_USER, user_profile);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_platform_mod_assigns_and_revokes_badge() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize the clock first
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            // Create test clock
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };
        
        // Create platform and profile
        create_test_platform(&mut scenario);
        create_test_profile(&mut scenario, PLATFORM_USER, string::utf8(b"platform_user2"));
        
        // Platform moderator assigns a badge to the user's profile
        test_scenario::next_tx(&mut scenario, PLATFORM_MOD);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut user_profile = test_scenario::take_from_address<Profile>(&scenario, PLATFORM_USER);
            
            let badge_name = string::utf8(b"Contributor");
            
            // Assign badge
            platform::assign_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                badge_name,
                string::utf8(b"Active Contributor"),
                string::utf8(b"https://example.com/contributor_badge.png"),
                string::utf8(b"https://example.com/contributor_badge_icon.png"),
                5,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Verify badge was assigned and get the actual badge ID
            assert!(profile::badge_count(&user_profile) == 1, 1);
            let badges = profile::get_profile_badges(&user_profile);
            let badge_data = *vector::borrow(&badges, 0);
            let badge_id = profile::badge_data_id(&badge_data);
            assert!(profile::has_badge(&user_profile, &badge_id), 2);
            
            // Now revoke the badge
            platform::revoke_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                badge_id,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Verify badge was revoked
            assert!(profile::badge_count(&user_profile) == 0, 3);
            assert!(!profile::has_badge(&user_profile, &badge_id), 4);
            
            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_to_address(PLATFORM_USER, user_profile);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_multiple_badges_from_platform() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize the clock first
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            // Create test clock
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };
        
        // Create platform and profile
        create_test_platform(&mut scenario);
        create_test_profile(&mut scenario, PLATFORM_USER, string::utf8(b"platform_user3"));
        
        // Platform admin assigns multiple badges to the user's profile
        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut user_profile = test_scenario::take_from_address<Profile>(&scenario, PLATFORM_USER);
            
            // Create the platform ID for later comparisons
            let platform_id = object::uid_to_address(platform::id(&platform));
            
            // Assign first badge
            platform::assign_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                string::utf8(b"VIP"),
                string::utf8(b"Very Important Person"),
                string::utf8(b"https://example.com/vip_badge.png"),
                string::utf8(b"https://example.com/vip_badge_icon.png"),
                10,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Assign second badge
            platform::assign_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                string::utf8(b"Moderator"),
                string::utf8(b"Community Moderator"),
                string::utf8(b"https://example.com/mod_badge.png"),
                string::utf8(b"https://example.com/mod_badge_icon.png"),
                20,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Verify total badge count
            assert!(profile::badge_count(&user_profile) == 2, 1);
            
            // Verify all badges are from the same platform
            let platform_badges = profile::get_platform_badges(&user_profile, platform_id);
            assert!(vector::length(&platform_badges) == 2, 2);
            
            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_to_address(PLATFORM_USER, user_profile);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    #[expected_failure(abort_code = platform::EUnauthorized)]
    fun test_unauthorized_badge_assignment() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize the clock first
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            // Create test clock
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };
        
        // Create platform and profile
        create_test_platform(&mut scenario);
        create_test_profile(&mut scenario, PLATFORM_USER, string::utf8(b"platform_user4"));
        
        // Normal user attempts to assign a badge (should fail)
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut user_profile = test_scenario::take_from_address<Profile>(&scenario, PLATFORM_USER);
            
            // This should fail with EUnauthorized since USER1 is neither platform admin nor moderator
            platform::assign_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                string::utf8(b"Fake"),
                string::utf8(b"Fake Badge"),
                string::utf8(b"https://example.com/fake.png"),
                string::utf8(b"https://example.com/fake_icon.png"),
                1,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_to_address(PLATFORM_USER, user_profile);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    #[expected_failure(abort_code = profile::EBadgeAlreadyExists)]
    fun test_duplicate_badge_prevention() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize the clock first
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            // Create test clock
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };
        
        // Create platform and profile
        create_test_platform(&mut scenario);
        create_test_profile(&mut scenario, PLATFORM_USER, string::utf8(b"platform_user5"));
        
        // Platform admin assigns a badge, then tries to assign the same badge again
        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut user_profile = test_scenario::take_from_address<Profile>(&scenario, PLATFORM_USER);
            
            let badge_name = string::utf8(b"VIP");
            
            // Assign badge first time - should succeed
            platform::assign_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                badge_name,
                string::utf8(b"Very Important Person"),
                string::utf8(b"https://example.com/vip_badge.png"),
                string::utf8(b"https://example.com/vip_badge_icon.png"),
                10,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Try to assign the same badge again - should fail
            platform::assign_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                badge_name,
                string::utf8(b"Very Important Person"),
                string::utf8(b"https://example.com/vip_badge.png"),
                string::utf8(b"https://example.com/vip_badge_icon.png"),
                10,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Not reached due to expected failure
            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_to_address(PLATFORM_USER, user_profile);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    #[expected_failure(abort_code = profile::EBadgeNotFound)]
    fun test_revoke_nonexistent_badge() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize the clock first
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            // Create test clock
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };
        
        // Create platform and profile
        create_test_platform(&mut scenario);
        create_test_profile(&mut scenario, PLATFORM_USER, string::utf8(b"platform_user6"));
        
        // Platform admin tries to revoke a nonexistent badge
        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut user_profile = test_scenario::take_from_address<Profile>(&scenario, PLATFORM_USER);
            
            // Try to revoke a nonexistent badge - should fail
            // Note: badge ID format changed to include platform ID bytes, but we're testing with a fake ID
            platform::revoke_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                string::utf8(b"badge_NonexistentBadge"),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Not reached due to expected failure
            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_to_address(PLATFORM_USER, user_profile);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_badges_persist_through_profile_transfer() {
        let mut scenario = test_scenario::begin(ADMIN);
        
        // Initialize the clock first
        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            // Create test clock
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };
        
        // Create platform and profile
        create_test_platform(&mut scenario);
        create_test_profile(&mut scenario, PLATFORM_USER, string::utf8(b"platform_user7"));
        
        // Platform admin assigns a badge to the user's profile
        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut user_profile = test_scenario::take_from_address<Profile>(&scenario, PLATFORM_USER);
            
            // Assign badge
            platform::assign_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                string::utf8(b"VIP"),
                string::utf8(b"Very Important Person"),
                string::utf8(b"https://example.com/vip_badge.png"),
                string::utf8(b"https://example.com/vip_badge_icon.png"),
                10,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Verify badge was assigned
            assert!(profile::badge_count(&user_profile) == 1, 1);
            
            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_to_address(PLATFORM_USER, user_profile);

            test_scenario::return_shared(clock);
        };
        
        // PLATFORM_USER transfers profile to USER2
        test_scenario::next_tx(&mut scenario, PLATFORM_USER);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
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
                0,
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // Verify USER2 received the profile with badge intact
        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            // Verify badge is still on the profile after transfer
            assert!(profile::badge_count(&profile) == 1, 2);
            // Badge ID format changed - get the actual badge ID from the badges
            let badges = profile::get_profile_badges(&profile);
            let badge_data = *vector::borrow(&badges, 0);
            let badge_id = profile::badge_data_id(&badge_data);
            assert!(profile::has_badge(&profile, &badge_id), 3);
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        test_scenario::end(scenario);
    }

    #[test]
    fun test_create_platform_with_media_fields() {
        let mut scenario = test_scenario::begin(ADMIN);
        create_test_platform(&mut scenario);

        test_scenario::next_tx(&mut scenario, PLATFORM_ADMIN);
        {
            let mut registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            platform::create_platform(
                &mut registry,
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
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
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
            platform::update_platform(
                &mut platform,
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
            let mut previews = vector::empty<string::String>();
            let mut i = 0u64;
            while (i < 11) {
                vector::push_back(&mut previews, string::utf8(b"https://example.com/p.png"));
                i = i + 1;
            };

            platform::create_platform(
                &mut registry,
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
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
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

            platform::block_wallet(
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

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut user_profile = test_scenario::take_from_address<Profile>(&scenario, PLATFORM_USER);

            platform::assign_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                string::utf8(b"ModBadge"),
                string::utf8(b"Moderator badge"),
                string::utf8(b"https://example.com/mod.png"),
                string::utf8(b"https://example.com/mod_icon.png"),
                1,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(profile::badge_count(&user_profile) == 1, 0);

            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_to_address(PLATFORM_USER, user_profile);

            test_scenario::return_shared(clock);
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

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<PlatformRegistry>(&scenario);
            let platform = test_scenario::take_shared<Platform>(&scenario);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scenario);
            let mut user_profile = test_scenario::take_from_address<Profile>(&scenario, PLATFORM_USER);

            platform::assign_badge(
                &registry,
                &platform,
                &group,
                &mut user_profile,
                string::utf8(b"BadgeOnly"),
                string::utf8(b"Badge only mod"),
                string::utf8(b"https://example.com/b.png"),
                string::utf8(b"https://example.com/b_icon.png"),
                1,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(profile::badge_count(&user_profile) == 1, 0);

            test_scenario::return_shared(registry);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform);
            test_scenario::return_to_address(PLATFORM_USER, user_profile);

            test_scenario::return_shared(clock);
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

            platform::block_wallet(
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

            platform::block_wallet(
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

            platform::block_wallet(
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
} 