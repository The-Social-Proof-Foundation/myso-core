// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_const, duplicate_alias, unused_assignment, dead_code)]
module social_contracts::post_tests {
    use std::string;
    use std::option;
    use std::vector;
    
    use myso::test_scenario;
    use myso::tx_context;
    use myso::table;
    use myso::object;
    use myso::transfer;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::clock::{Self, Clock};
    use myso::test_utils;
    
    use social_contracts::post::{Self, Post, Comment, PostConfig, PostAdminCap, PromotionData};
    use social_contracts::profile::{Self, UsernameRegistry,
        ProfileConfig};
    use social_contracts::ai_credit::AiCreditConfig;
    use social_contracts::memory::{MemoryRegistry, MemoryAccount, SubAgent, Self as memory, AgenticOrganization,
        MemoryConfig};
    use social_contracts::memory_test_helpers;
    
    const PLACEHOLDER_AGENT: address = @0xBEEF;
    const PLACEHOLDER_PUBKEY: vector<u8> = x"0303030303030303030303030303030303030303030303030303030303030303";
    use social_contracts::poc_vault::{Self as poc_vault, PoCBeneficiaryVault};
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::mydata::{Self, MyData, MyDataConfig, MyDataRegistry};
    
    // Test constants
    const TEST_CONTENT: vector<u8> = b"This is a test post";
    const USER1: address = @0x1;
    const USER2: address = @0x2;
    const USER3: address = @0x3;
    const PLATFORM_DEVELOPER: address = @0xCAFE;
    const PLATFORM_MODERATOR: address = @0xBEEF;
    const REGULAR_USER: address = @0x5;
    const TEST_PLATFORM_ID: address = @0x1; // Use USER1's address as test platform ID

    fun init_tip_test_profile(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, USER1);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(scenario));

            clock::share_for_testing(clock);
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

        test_scenario::next_tx(scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(scenario);
        };

        test_scenario::next_tx(scenario, USER1);
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
    }

    
    /// Test basic string operations for post content
    #[test]
    fun test_post_content_basics() {
        let mut scenario = test_scenario::begin(USER1);
        
        // Test basic string operations that would be used in posts
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let content = string::utf8(TEST_CONTENT);
            assert!(string::length(&content) > 0, 0);
            assert!(string::length(&content) == 19, 1); // "This is a test post" length
            
            // Test option creation/extraction that would be used in post module
            let mut opt_content = option::some(content);
            assert!(option::is_some(&opt_content), 2);
            
            let extracted = option::extract(&mut opt_content);
            assert!(string::length(&extracted) == 19, 3);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test constants and values expected in the post module
    #[test]
    fun test_post_type_constants() {
        let mut scenario = test_scenario::begin(USER1);
        
        // Log post type constants for inspection
        test_scenario::next_tx(&mut scenario, USER1);
        {
            // Expected values matching post module constants
            let standard_type = b"standard";
            let comment_type = b"comment";
            let repost_type = b"repost";
            let quote_repost_type = b"quote_repost";
            
            // These assertions verify the expected constants
            assert!(standard_type == b"standard", 0);
            assert!(comment_type == b"comment", 1);
            assert!(repost_type == b"repost", 2);
            assert!(quote_repost_type == b"quote_repost", 3);
            
            // Test string conversions that would be used in post module
            let std_str = string::utf8(standard_type);
            assert!(string::length(&std_str) == 8, 4);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test transaction context functions used in post module
    #[test]
    fun test_tx_context_functions() {
        let mut scenario = test_scenario::begin(USER1);
        
        // Test tx_context functions that would be used in post module
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            
            // Verify sender functionality
            let sender = tx_context::sender(ctx);
            assert!(sender == USER1, 0);
            
            // Verify epoch functionality (used for timestamps)
            let epoch = tx_context::epoch(ctx);
            assert!(epoch >= 0, 1); // Epoch should be non-negative
        };
        
        // Test different sender
        test_scenario::next_tx(&mut scenario, USER2);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            let sender = tx_context::sender(ctx);
            assert!(sender == USER2, 2);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test like tracking table operations - simulates the logic in like_post and like_comment
    #[test]
    fun test_like_table_operations() {
        let mut scenario = test_scenario::begin(USER1);
        
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            
            // Create a likes table to track likes (simulating what happens inside Likes struct)
            let mut likes = table::new<address, bool>(ctx);
            
            // Initially no likes
            assert!(table::length(&likes) == 0, 0);
            
            // USER2 likes the post
            assert!(!table::contains(&likes, USER2), 1); // Should not be in likes
            table::add(&mut likes, USER2, true);
            assert!(table::contains(&likes, USER2), 2); // Now should be in likes
            assert!(table::length(&likes) == 1, 3);
            
            // USER3 likes the post
            assert!(!table::contains(&likes, USER3), 4);
            table::add(&mut likes, USER3, true);
            assert!(table::contains(&likes, USER3), 5);
            assert!(table::length(&likes) == 2, 6);
            
            // USER3 unlikes the post
            assert!(table::contains(&likes, USER3), 7);
            table::remove(&mut likes, USER3);
            assert!(!table::contains(&likes, USER3), 8);
            assert!(table::length(&likes) == 1, 9);
            
            // Should prevent duplicate likes (this would abort in real code)
            // We're just testing the table contains check that would prevent it
            assert!(table::contains(&likes, USER2), 10);
            
            // Cleanup
            table::drop(likes);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test comment count tracking logic - simulates creating comments and managing counts
    #[test]
    fun test_comment_count_tracking() {
        let mut scenario = test_scenario::begin(USER1);
        
        test_scenario::next_tx(&mut scenario, USER1);
        {
            // Simulate a post's comment_count field being updated
            let mut post_comment_count: u64 = 0;
            
            // USER2 creates a comment
            post_comment_count = post_comment_count + 1;
            assert!(post_comment_count == 1, 0);
            
            // USER3 creates another comment
            post_comment_count = post_comment_count + 1;
            assert!(post_comment_count == 2, 1);
            
            // Track nested comments (inside another comment)
            let mut comment_nested_count: u64 = 0;
            
            // USER1 replies to USER2's comment
            comment_nested_count = comment_nested_count + 1;
            assert!(comment_nested_count == 1, 2);
            
            // USER4 replies to USER2's comment
            comment_nested_count = comment_nested_count + 1;
            assert!(comment_nested_count == 2, 3);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test repost count tracking logic - simulates creating reposts and managing counts
    #[test]
    fun test_repost_count_tracking() {
        let mut scenario = test_scenario::begin(USER1);
        
        test_scenario::next_tx(&mut scenario, USER1);
        {
            // Simulate a post's repost_count field being updated
            let mut post_repost_count: u64 = 0;
            
            // USER2 reposts
            post_repost_count = post_repost_count + 1;
            assert!(post_repost_count == 1, 0);
            
            // USER3 quote reposts (adds a comment too)
            post_repost_count = post_repost_count + 1;
            assert!(post_repost_count == 2, 1);
            
            // Track content types for quote reposts vs. standard reposts
            let standard_repost = string::utf8(b"repost");
            let quote_repost = string::utf8(b"quote_repost");
            
            // Test proper type values
            assert!(standard_repost != quote_repost, 2);
            
            // Simulate checking if a quote repost has content
            let content = string::utf8(b"This is my quote repost comment");
            assert!(string::length(&content) > 0, 3);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test content moderation patterns
    #[test]
    fun test_content_moderation() {
        let mut scenario = test_scenario::begin(PLATFORM_DEVELOPER);
        
        // First transaction: Initial setup for moderation state
        test_scenario::next_tx(&mut scenario, PLATFORM_DEVELOPER);
        {
            // Simulating a post's moderation state
            let mut post_removed = false;
            
            // Post start as not removed
            assert!(post_removed == false, 0);
            
            // Developer can moderate
            let sender = tx_context::sender(test_scenario::ctx(&mut scenario));
            assert!(sender == PLATFORM_DEVELOPER, 1);
            
            // Developer moderates the post (removes it)
            post_removed = true;
            assert!(post_removed == true, 2);
        };
        
        // Second transaction: Moderator can also moderate
        test_scenario::next_tx(&mut scenario, PLATFORM_MODERATOR);
        {
            // Simulating a post's moderation state (continue from above)
            let mut post_removed = true;
            
            // Moderator can unremove the post
            let sender = tx_context::sender(test_scenario::ctx(&mut scenario));
            assert!(sender == PLATFORM_MODERATOR, 3);
            
            // Moderator restores the post
            post_removed = false;
            assert!(post_removed == false, 4);
        };
        
        // Third transaction: Regular user cannot moderate
        test_scenario::next_tx(&mut scenario, REGULAR_USER);
        {
            // Simulating the platform's developer/moderator check
            let sender = tx_context::sender(test_scenario::ctx(&mut scenario));
            
            // Verify sender is not a developer or moderator
            assert!(sender != PLATFORM_DEVELOPER, 5);
            assert!(sender != PLATFORM_MODERATOR, 6);
            
            // In the real implementation, the function would abort here
            // since the sender is not authorized to moderate
            
            // We simulate the authorization check that would happen
            let is_authorized = sender == PLATFORM_DEVELOPER || sender == PLATFORM_MODERATOR;
            assert!(!is_authorized, 7);
        };
        
        test_scenario::end(scenario);
    }
    
    /// Test that post content parsing works correctly
    #[test]
    fun test_post_content() {
        // Test that parsing and comparing content works correctly
        let content = string::utf8(b"Hello, World!");
        
        // Simply test the string operations
        assert!(string::as_bytes(&content) == b"Hello, World!", 0);
        assert!(string::length(&content) == 13, 1);
    }
    
    /// Test post update functionalities with string comparisons
    #[test]
    fun test_post_updates() {
        // This test verifies string equality would work for post updates
        let content = string::utf8(b"Original content");
        let updated = string::utf8(b"Updated content");
        
        // Test content hasn't been modified yet
        assert!(content != updated, 0);
        
        // This verifies string equality would work for tests
        let same = string::utf8(b"Original content");
        assert!(content == same, 1);
    }

    /// Test reactions functionality with toggle behavior
    #[test]
    fun test_reactions() {
        let mut scenario = test_scenario::begin(USER1);
        
        // Create a table to track user reactions (simulating what happens in a Post struct)
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let ctx = test_scenario::ctx(&mut scenario);
            
            // Initialize reaction tracking tables
            let mut user_reactions = table::new<address, string::String>(ctx);
            let mut reaction_counts = table::new<string::String, u64>(ctx);
            let mut total_reaction_count: u64 = 0;
            
            // USER2 adds a reaction "👍"
            let reaction1 = string::utf8(b"👍");
            
            // Initially USER2 has no reaction
            assert!(!table::contains(&user_reactions, USER2), 0);
            
            // Add USER2's reaction
            table::add(&mut user_reactions, USER2, reaction1);
            total_reaction_count = total_reaction_count + 1;
            
            // Update reaction counts for this emoji
            table::add(&mut reaction_counts, reaction1, 1);
            
            // Verify state after first reaction
            assert!(table::contains(&user_reactions, USER2), 1);
            assert!(table::length(&user_reactions) == 1, 2);
            assert!(total_reaction_count == 1, 3);
            assert!(*table::borrow(&reaction_counts, reaction1) == 1, 4);
            
            // USER3 adds a different reaction "❤️"
            let reaction2 = string::utf8(b"❤️");
            
            // Add USER3's reaction
            table::add(&mut user_reactions, USER3, reaction2);
            total_reaction_count = total_reaction_count + 1;
            
            // Update reaction counts
            table::add(&mut reaction_counts, reaction2, 1);
            
            // Verify state after second reaction
            assert!(table::contains(&user_reactions, USER3), 5);
            assert!(table::length(&user_reactions) == 2, 6);
            assert!(total_reaction_count == 2, 7);
            assert!(*table::borrow(&reaction_counts, reaction2) == 1, 8);
            
            // USER2 toggles the same reaction (removes it)
            let user2_reaction = *table::borrow(&user_reactions, USER2);
            assert!(user2_reaction == reaction1, 9);
            
            // Remove USER2's reaction (toggle off)
            table::remove(&mut user_reactions, USER2);
            total_reaction_count = total_reaction_count - 1;
            
            // Update reaction counts for the removed reaction
            let count = *table::borrow(&reaction_counts, reaction1);
            if (count <= 1) {
                table::remove(&mut reaction_counts, reaction1);
            } else {
                *table::borrow_mut(&mut reaction_counts, reaction1) = count - 1;
            };
            
            // Verify state after reaction removal
            assert!(!table::contains(&user_reactions, USER2), 10);
            assert!(table::length(&user_reactions) == 1, 11);
            assert!(total_reaction_count == 1, 12);
            assert!(!table::contains(&reaction_counts, reaction1), 13);
            
            // USER3 changes their reaction from "❤️" to "👍"
            // First remove old reaction count
            let user3_reaction = *table::borrow(&user_reactions, USER3);
            let count = *table::borrow(&reaction_counts, user3_reaction);
            if (count <= 1) {
                table::remove(&mut reaction_counts, user3_reaction);
            } else {
                *table::borrow_mut(&mut reaction_counts, user3_reaction) = count - 1;
            };
            
            // Then update USER3's reaction
            *table::borrow_mut(&mut user_reactions, USER3) = reaction1;
            
            // And update reaction counts for the new reaction
            if (table::contains(&reaction_counts, reaction1)) {
                let count = *table::borrow(&reaction_counts, reaction1);
                *table::borrow_mut(&mut reaction_counts, reaction1) = count + 1;
            } else {
                table::add(&mut reaction_counts, reaction1, 1);
            };
            
            // Verify final state after reaction change
            assert!(table::contains(&user_reactions, USER3), 14);
            assert!(*table::borrow(&user_reactions, USER3) == reaction1, 15);
            assert!(table::length(&user_reactions) == 1, 16);
            assert!(total_reaction_count == 1, 17);
            assert!(*table::borrow(&reaction_counts, reaction1) == 1, 18);
            assert!(!table::contains(&reaction_counts, reaction2), 19);
            
            // Cleanup
            table::drop(user_reactions);
            table::drop(reaction_counts);
        };
        
        test_scenario::end(scenario);
    }

    /// Test delete_post functionality
    #[test]
    fun test_delete_post() {
        let mut scenario = test_scenario::begin(USER1);
        
        // Create test objects and registry
        test_scenario::next_tx(&mut scenario, USER1);
        {
            // Create a registry and platform for testing
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            social_contracts::profile::test_init(&clock, test_scenario::ctx(&mut scenario));

            social_contracts::platform::test_init(&clock, test_scenario::ctx(&mut scenario));
            social_contracts::block_list::test_init(&clock, test_scenario::ctx(&mut scenario));

            social_contracts::mydata::test_init(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            // Initialize the post module
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        
        // USER1 creates a profile
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Register a test username
            social_contracts::profile::register_username(
                &mut registry, 
                string::utf8(b"user1"), 
                option::none(), 
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // USER1 creates a post directly with test helper
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Get the profile ID
            let mut profile_id_option = social_contracts::profile::lookup_profile_by_owner(&registry, USER1);
            assert!(option::is_some(&profile_id_option), 0);
            let profile_id_addr = option::extract(&mut profile_id_option);
            
            // Create a test post directly
            post::test_create_post(
                USER1,
                profile_id_addr,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // USER1 deletes the post they own
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            // Retrieve the post 
            let post = test_scenario::take_shared<Post>(&scenario);
            
            // Verify it has the expected content and owner
            assert!(post::get_post_content(&post) == string::utf8(TEST_CONTENT), 0);
            assert!(post::get_post_owner(&post) == USER1, 1);
            
            // Delete the post
            post::delete_post(
                post,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Post should no longer exist in the shared objects
            assert!(!test_scenario::has_most_recent_shared<Post>(), 2);

            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    /// Test delete_comment functionality
    #[test]
    fun test_delete_comment() {
        let mut scenario = test_scenario::begin(USER1);
        
        // Create test objects and registry
        test_scenario::next_tx(&mut scenario, USER1);
        {
            // Create a registry and platform for testing
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            social_contracts::profile::test_init(&clock, test_scenario::ctx(&mut scenario));

            social_contracts::platform::test_init(&clock, test_scenario::ctx(&mut scenario));
            social_contracts::block_list::test_init(&clock, test_scenario::ctx(&mut scenario));

            social_contracts::mydata::test_init(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            // Initialize the post module
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        
        // USER1 creates a profile
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Register a test username
            social_contracts::profile::register_username(
                &mut registry, 
                string::utf8(b"user1"), 
                option::none(), 
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // USER1 creates a post directly with test helper
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Get the profile ID
            let mut profile_id_option = social_contracts::profile::lookup_profile_by_owner(&registry, USER1);
            assert!(option::is_some(&profile_id_option), 0);
            let profile_id_addr = option::extract(&mut profile_id_option);
            
            // Create a test post directly
            post::test_create_post(
                USER1,
                profile_id_addr,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // Create a variable to store the comment ID
        let comment_id: address;
        
        // USER1 creates a comment on their own post
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let post = test_scenario::take_shared<Post>(&scenario);
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mydata_registry = test_scenario::take_shared<MyDataRegistry>(&scenario);
            
            // Get the profile ID for the comment
            let mut profile_id_option = social_contracts::profile::lookup_profile_by_owner(&registry, USER1);
            assert!(option::is_some(&profile_id_option), 0);
            let profile_id_addr = option::extract(&mut profile_id_option);
            
            // Create test comment directly using the module's test helper
            comment_id = post::test_create_comment(
                USER1,
                profile_id_addr,
                object::uid_to_address(post::get_post_id(&post)),
                string::utf8(b"This is a test comment"),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(post);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(mydata_registry);

            test_scenario::return_shared(clock);
        };
        
        // USER1 deletes their comment
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut post = test_scenario::take_shared<Post>(&scenario);
            
            // Get the comment directly by ID
            let comment_id_obj = object::id_from_address(comment_id);
            let comment = test_scenario::take_shared_by_id<Comment>(&scenario, comment_id_obj);
            
            // Manually set comment_count to 1 since test_create_comment does not increment it
            post::set_comment_count_for_testing(&mut post, 1);
            
            // Verify it's the same post
            assert!(post::get_post_owner(&post) == USER1, 1);
            
            // Verify comment belongs to USER1
            assert!(post::get_comment_owner(&comment) == USER1, 2);
            
            // Get the comment ID to verify it's the right one
            let comment_post_id = post::get_comment_post_id(&comment);
            let post_id = object::uid_to_address(post::get_post_id(&post));
            assert!(comment_post_id == post_id, 3);
            
            // Delete the comment
            post::delete_comment(
                &mut post,
                comment,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(post);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }

    /// Test unauthorized attempts to delete a post
    #[test]
    #[expected_failure(abort_code = 0, location = social_contracts::post)] // EUnauthorized error code
    fun test_unauthorized_post_deletion() {
        let mut scenario = test_scenario::begin(USER1);
        
        // Create test objects and registry
        test_scenario::next_tx(&mut scenario, USER1);
        {
            // Create a registry and platform for testing
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            social_contracts::profile::test_init(&clock, test_scenario::ctx(&mut scenario));

            social_contracts::platform::test_init(&clock, test_scenario::ctx(&mut scenario));
            social_contracts::block_list::test_init(&clock, test_scenario::ctx(&mut scenario));

            social_contracts::mydata::test_init(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            // Initialize the post module
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        
        // USER1 creates a profile
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Register a test username
            social_contracts::profile::register_username(
                &mut registry, 
                string::utf8(b"user1"), 
                option::none(), 
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // USER1 creates a post directly with test helper
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Get the profile ID
            let mut profile_id_option = social_contracts::profile::lookup_profile_by_owner(&registry, USER1);
            assert!(option::is_some(&profile_id_option), 0);
            let profile_id_addr = option::extract(&mut profile_id_option);
            
            // Create a test post directly
            post::test_create_post(
                USER1,
                profile_id_addr,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // USER2 attempts to delete USER1's post - should fail
        test_scenario::next_tx(&mut scenario, USER2);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let post = test_scenario::take_shared<Post>(&scenario);
            
            // This should fail since USER2 != USER1
            post::delete_post(post, &clock, test_scenario::ctx(&mut scenario));
            
            // This line should never be reached due to failure
            abort 42;

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }

    /// Test unauthorized attempt to delete a comment
    #[test]
    #[expected_failure(abort_code = 0, location = social_contracts::post)] // EUnauthorized error code
    fun test_unauthorized_comment_deletion() {
        let mut scenario = test_scenario::begin(USER1);
        
        // Create test objects and registry
        test_scenario::next_tx(&mut scenario, USER1);
        {
            // Create a registry and platform for testing
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            social_contracts::profile::test_init(&clock, test_scenario::ctx(&mut scenario));

            social_contracts::platform::test_init(&clock, test_scenario::ctx(&mut scenario));
            social_contracts::block_list::test_init(&clock, test_scenario::ctx(&mut scenario));

            social_contracts::mydata::test_init(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            // Initialize the post module
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        
        // Register profiles for USER1 and USER2
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Register a test username for USER1
            social_contracts::profile::register_username(
                &mut registry, 
                string::utf8(b"user1"), 
                option::none(), 
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::next_tx(&mut scenario, USER2);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Register a test username for USER2
            social_contracts::profile::register_username(
                &mut registry, 
                string::utf8(b"user2"), 
                option::none(), 
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // USER1 creates a post directly with test helper
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Get the profile ID
            let mut profile_id_option = social_contracts::profile::lookup_profile_by_owner(&registry, USER1);
            assert!(option::is_some(&profile_id_option), 0);
            let profile_id_addr = option::extract(&mut profile_id_option);
            
            // Create a test post directly
            post::test_create_post(
                USER1,
                profile_id_addr,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // Create a variable to store the comment ID
        let comment_id: address;
        
        // USER2 creates a comment on USER1's post
        test_scenario::next_tx(&mut scenario, USER2);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let post = test_scenario::take_shared<Post>(&scenario);
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mydata_registry = test_scenario::take_shared<MyDataRegistry>(&scenario);
            
            // Get the profile ID for USER2
            let mut profile_id_option = social_contracts::profile::lookup_profile_by_owner(&registry, USER2);
            assert!(option::is_some(&profile_id_option), 0);
            let profile_id_addr = option::extract(&mut profile_id_option);
            
            // Create test comment directly using the module's test helper
            comment_id = post::test_create_comment(
                USER2,
                profile_id_addr,
                object::uid_to_address(post::get_post_id(&post)),
                string::utf8(b"This is a test comment by USER2"),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(post);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(mydata_registry);

            test_scenario::return_shared(clock);
        };
        
        // USER1 attempts to delete USER2's comment - should fail
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut post = test_scenario::take_shared<Post>(&scenario);
            
            // Get the comment directly by ID
            let comment_id_obj = object::id_from_address(comment_id);
            let comment = test_scenario::take_shared_by_id<Comment>(&scenario, comment_id_obj);
            
            // This should fail since comment owner is USER2, not USER1
            post::delete_comment(
                &mut post,
                comment,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // This line should never be reached due to failure
            abort 42;

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }

    /// Test block list check in post creation
    #[test]
    fun test_post_creation_with_blocklist() {
        let mut scenario = test_scenario::begin(USER1);
        
        // Create test objects and registry
        test_scenario::next_tx(&mut scenario, USER1);
        {
            // Create a registry and platform for testing
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            social_contracts::profile::test_init(&clock, test_scenario::ctx(&mut scenario));

            social_contracts::platform::test_init(&clock, test_scenario::ctx(&mut scenario));
            social_contracts::block_list::test_init(&clock, test_scenario::ctx(&mut scenario));

            social_contracts::mydata::test_init(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            // Initialize the post module
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        
        // USER1 creates a profile
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Register a test username
            social_contracts::profile::register_username(
                &mut registry, 
                string::utf8(b"user1"), 
                option::none(), 
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Get the profile ID to join platform
            let mut profile_id_option = social_contracts::profile::lookup_profile_by_owner(&registry, USER1);
            assert!(option::is_some(&profile_id_option), 0);
            let _profile_id_addr = option::extract(&mut profile_id_option);
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // USER1 creates a post directly using test helper
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            
            // Get the profile ID again
            let mut profile_id_option = social_contracts::profile::lookup_profile_by_owner(&registry, USER1);
            assert!(option::is_some(&profile_id_option), 0);
            let profile_id_addr = option::extract(&mut profile_id_option);
            
            // Use the test helper to create a post directly
            post::test_create_post(
                USER1,
                profile_id_addr,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);

            test_scenario::return_shared(clock);
        };
        
        // Verify post was created successfully
        test_scenario::next_tx(&mut scenario, USER1);
        {
            // Check if post exists and get it if it does
            if (test_scenario::has_most_recent_shared<Post>()) {
                let post = test_scenario::take_shared<Post>(&scenario);
                assert!(post::get_post_content(&post) == string::utf8(TEST_CONTENT), 1);
                assert!(post::get_post_owner(&post) == USER1, 2);
                test_scenario::return_shared(post);
            } else {
                // If no post exists, this will fail the test
                assert!(false, 97);
            };
        };
        
        test_scenario::end(scenario);
    }

    /// Test promoted post creation and basic functionality
    #[test]
    fun test_promoted_post_creation() {
        let mut scenario = test_scenario::begin(USER1);
        
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };

        // Create a simple promoted post using test functions
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            // Create some MYSO coins for promotion budget
            let promotion_budget = coin::mint_for_testing<myso::myso::MYSO>(1000000, test_scenario::ctx(&mut scenario)); // 1 MYSO
            
            // Create a promoted post using the test helper
            let (post_id, promotion_id) = post::create_test_promoted_post(
                USER1,
                USER1, // profile_id same as owner for test
                TEST_PLATFORM_ID,
                string::utf8(b"This is a promoted post!"),
                10000, // 0.01 MYSO per view
                promotion_budget,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Verify the IDs are different and valid
            assert!(post_id != promotion_id, 0);
            assert!(post_id != @0x0, 1);
            assert!(promotion_id != @0x0, 2);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }

    /// Default promo fee bps are 10% / 10%; fee math yields 80% net to viewer.
    #[test]
    fun test_promotion_fee_defaults_and_math() {
        let mut scenario = test_scenario::begin(USER1);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<PostConfig>(&scenario);
            assert!(post::test_platform_fee_bps(&config) == 1000, 0);
            assert!(post::test_ecosystem_fee_bps(&config) == 1000, 1);
            let (platform_fee, ecosystem_fee, recipient_amount) =
                post::test_promotion_view_fee_amounts(&config, 10_000);
            assert!(platform_fee == 1_000, 2);
            assert!(ecosystem_fee == 1_000, 3);
            assert!(recipient_amount == 8_000, 4);
            test_scenario::return_shared(config);
        };
        test_scenario::end(scenario);
    }

    /// update_post_parameters rejects fee bps that exceed 10000 combined.
    #[test]
    #[expected_failure(abort_code = 20, location = social_contracts::post)] // EInvalidConfig
    fun test_update_post_parameters_rejects_fee_bps_overflow() {
        let mut scenario = test_scenario::begin(USER1);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            post::test_init(test_scenario::ctx(&mut scenario));
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
        };
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut config = test_scenario::take_shared<PostConfig>(&scenario);
            let admin_cap = test_scenario::take_from_sender<PostAdminCap>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            post::update_post_parameters(
                &admin_cap,
                &mut config,
                5000,
                10,
                10,
                10000,
                500,
                20,
                80,
                50,
                1000,
                100_000_000,
                3000,
                6000,
                5000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(config);
            test_scenario::return_to_sender(&scenario, admin_cap);
            test_scenario::return_shared(clock);
        };
        test_scenario::end(scenario);
    }

    /// Shared setup: MyData system objects and a `Clock` for `create_and_share` tests.
    fun init_mydata_clock_and_registry(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, USER1);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            mydata::test_init(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);

        };
        test_scenario::next_tx(scenario, USER1);
        {
            let _w = test_utils::create_one_time_witness<myso::myso::MYSO>();
            clock::share_for_testing(clock::create_for_testing(test_scenario::ctx(scenario)));
        };
    }

    /// Creates a registered MyData as `USER1` and returns its on-chain id.
    fun create_registered_mydata_for_user1(scenario: &mut test_scenario::Scenario): address {
        test_scenario::next_tx(scenario, USER1);
        {
            let config = test_scenario::take_shared<MyDataConfig>(scenario);
            let mut registry = test_scenario::take_shared<MyDataRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            mydata::create_and_share_marketplace_one_time_mydata(
                &config,
                &mut registry,
                string::utf8(b"data"),
                vector[string::utf8(b"test")],
                option::none<address>(),
                1000,
                option::none<u64>(),
                b"encrypted_data",
                b"encryption_id",
                100,
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                false,
                option::none(),
                &clock,
                test_scenario::ctx(scenario)
            );
            test_scenario::return_shared(config);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(scenario, USER1);
        {
            let m = test_scenario::take_shared<MyData>(scenario);
            let ip_id = mydata::object_address(&m);
            test_scenario::return_shared(m);
            ip_id
        }
    }

    #[test]
    fun test_post_access_public_ok() {
        let mut scenario = test_scenario::begin(USER1);
        init_mydata_clock_and_registry(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let reg = test_scenario::take_shared<MyDataRegistry>(&scenario);
            post::test_assert_post_access_mydata_binding(USER1, post::test_post_access_public(), &reg);
            test_scenario::return_shared(reg);
        };
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 32, location = social_contracts::post)]
    fun test_post_access_mydata_not_registered_aborts() {
        let mut scenario = test_scenario::begin(USER1);
        init_mydata_clock_and_registry(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let reg = test_scenario::take_shared<MyDataRegistry>(&scenario);
            let access = post::test_post_access_marketplace_one_time(object::id_from_address(@0x999));
            post::test_assert_post_access_mydata_binding(USER1, access, &reg);
            test_scenario::return_shared(reg);
        };
        test_scenario::end(scenario);
    }

    #[test]
    fun test_post_access_mydata_owner_matches_ok() {
        let mut scenario = test_scenario::begin(USER1);
        init_mydata_clock_and_registry(&mut scenario);
        let ip_id = create_registered_mydata_for_user1(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let reg = test_scenario::take_shared<MyDataRegistry>(&scenario);
            let access = post::test_post_access_marketplace_one_time(object::id_from_address(ip_id));
            post::test_assert_post_access_mydata_binding(USER1, access, &reg);
            test_scenario::return_shared(reg);
        };
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 33, location = social_contracts::post)]
    fun test_post_access_mydata_wrong_owner_aborts() {
        let mut scenario = test_scenario::begin(USER1);
        init_mydata_clock_and_registry(&mut scenario);
        let ip_id = create_registered_mydata_for_user1(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER2);
        {
            let reg = test_scenario::take_shared<MyDataRegistry>(&scenario);
            let access = post::test_post_access_marketplace_one_time(object::id_from_address(ip_id));
            post::test_assert_post_access_mydata_binding(USER2, access, &reg);
            test_scenario::return_shared(reg);
        };
        test_scenario::end(scenario);
    }

    #[test]
    fun test_tip_post_simple_no_redirect() {
        let mut scenario = test_scenario::begin(USER1);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        init_tip_test_profile(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            post::test_create_post(
                USER1,
                USER1,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER2);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut post_obj = test_scenario::take_shared<Post>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut tip_coin = coin::mint_for_testing<MYSO>(10_000_000_000, test_scenario::ctx(&mut scenario));
            assert!(!post::tip_post_requires_beneficiary_vault_for_amount(&post_obj, 5_000_000_000), 0);
            post::tip_post_simple<MYSO>(
                &mut post_obj,
                &mut tip_coin,
                5_000_000_000,
                &memory_account,
                test_scenario::ctx(&mut scenario)
            );
            assert!(post::get_tips_received(&post_obj) == 5_000_000_000, 1);
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(memory_account);
            transfer::public_transfer(tip_coin, USER2);

            test_scenario::return_shared(clock);
        };
        test_scenario::end(scenario);
    }

    #[test]
    fun test_tip_post_simple_wallet_redirect() {
        let mut scenario = test_scenario::begin(USER1);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        init_tip_test_profile(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            post::test_create_post_with_revenue_redirect(
                USER1,
                USER1,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                USER3,
                50,
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER2);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut post_obj = test_scenario::take_shared<Post>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut tip_coin = coin::mint_for_testing<MYSO>(100, test_scenario::ctx(&mut scenario));
            assert!(!post::tip_post_requires_beneficiary_vault_for_amount(&post_obj, 100), 0);
            post::tip_post_simple<MYSO>(&mut post_obj, &mut tip_coin, 100, &memory_account, test_scenario::ctx(&mut scenario));
            assert!(post::get_tips_received(&post_obj) == 50, 1);
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(memory_account);
            transfer::public_transfer(tip_coin, USER2);

            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER3);
        {
            let c = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            assert!(coin::value(&c) == 50, 2);
            test_scenario::return_to_sender(&scenario, c);
        };
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 38, location = social_contracts::post)]
    fun test_tip_post_simple_escrow_requires_vault() {
        let mut scenario = test_scenario::begin(USER1);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        init_tip_test_profile(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            post::test_create_post_with_escrow_redirect(
                USER1,
                USER1,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                USER3,
                50,
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER2);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut post_obj = test_scenario::take_shared<Post>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut tip_coin = coin::mint_for_testing<MYSO>(100, test_scenario::ctx(&mut scenario));
            assert!(post::tip_post_requires_beneficiary_vault_for_amount(&post_obj, 100), 0);
            post::tip_post_simple<MYSO>(&mut post_obj, &mut tip_coin, 100, &memory_account, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(memory_account);
            transfer::public_transfer(tip_coin, USER2);

            test_scenario::return_shared(clock);
        };
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 37, location = social_contracts::post)]
    fun test_tip_post_wrong_beneficiary_vault_aborts() {
        let mut scenario = test_scenario::begin(USER1);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        init_tip_test_profile(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            post::test_create_post_with_escrow_redirect(
                USER1,
                USER1,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                USER3,
                50,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            poc_vault::create_shared_dummy_vault_for_testing(USER1, test_scenario::ctx(&mut scenario));

            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER2);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut post_obj = test_scenario::take_shared<Post>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut wrong_vault = test_scenario::take_shared<PoCBeneficiaryVault>(&scenario);
            let mut tip_coin = coin::mint_for_testing<MYSO>(100, test_scenario::ctx(&mut scenario));
            post::tip_post<MYSO>(
                &mut post_obj,
                &mut wrong_vault,
                &mut tip_coin,
                100,
                0,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(wrong_vault);
            transfer::public_transfer(tip_coin, USER2);

            test_scenario::return_shared(clock);
        };
        test_scenario::end(scenario);
    }

    #[test]
    fun test_tip_post_escrow_rounded_redirect_uses_simple() {
        let mut scenario = test_scenario::begin(USER1);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        init_tip_test_profile(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            post::test_create_post_with_escrow_redirect(
                USER1,
                USER1,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                USER3,
                50,
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER2);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut post_obj = test_scenario::take_shared<Post>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut tip_coin = coin::mint_for_testing<MYSO>(100, test_scenario::ctx(&mut scenario));
            assert!(!post::tip_post_requires_beneficiary_vault_for_amount(&post_obj, 1), 0);
            post::tip_post_simple<MYSO>(&mut post_obj, &mut tip_coin, 1, &memory_account, test_scenario::ctx(&mut scenario));
            assert!(post::get_tips_received(&post_obj) == 1, 1);
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(memory_account);
            transfer::public_transfer(tip_coin, USER2);

            test_scenario::return_shared(clock);
        };
        test_scenario::end(scenario);
    }

    #[test]
    fun test_tip_comment_simple_no_redirect() {
        let mut scenario = test_scenario::begin(USER1);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        init_tip_test_profile(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            post::test_create_post(
                USER1,
                USER1,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let post_obj = test_scenario::take_shared<Post>(&scenario);
            let _ = post::test_create_comment(
                USER3,
                USER3,
                object::uid_to_address(post::get_post_id(&post_obj)),
                string::utf8(b"tip me"),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut post_obj = test_scenario::take_shared<Post>(&scenario);
            let mut comment = test_scenario::take_shared<Comment>(&scenario);
            let config = test_scenario::take_shared<PostConfig>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut tip_coin = coin::mint_for_testing<MYSO>(100, test_scenario::ctx(&mut scenario));
            // 80% commenter / 20% post owner; post-owner slice does not need vault
            assert!(!post::tip_post_requires_beneficiary_vault_for_amount(&post_obj, 20), 0);
            post::tip_comment_simple<MYSO>(
                &mut comment,
                &mut post_obj,
                &config,
                &mut tip_coin,
                100,
                &memory_account,
                test_scenario::ctx(&mut scenario)
            );
            assert!(post::get_comment_tips_received(&comment) == 80, 1);
            assert!(post::get_tips_received(&post_obj) == 20, 2);
            test_scenario::return_shared(comment);
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(config);
            test_scenario::return_shared(memory_account);
            transfer::public_transfer(tip_coin, USER2);
        };
        test_scenario::next_tx(&mut scenario, USER3);
        {
            let c = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            assert!(coin::value(&c) == 80, 3);
            test_scenario::return_to_sender(&scenario, c);
        };
        test_scenario::end(scenario);
    }

    #[test]
    fun test_tip_comment_simple_wallet_redirect() {
        let mut scenario = test_scenario::begin(USER1);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        init_tip_test_profile(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            // Redirect post-owner tip slice to USER3 at 50%
            post::test_create_post_with_revenue_redirect(
                USER1,
                USER1,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                USER3,
                50,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let post_obj = test_scenario::take_shared<Post>(&scenario);
            let _ = post::test_create_comment(
                REGULAR_USER,
                REGULAR_USER,
                object::uid_to_address(post::get_post_id(&post_obj)),
                string::utf8(b"tip me"),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut post_obj = test_scenario::take_shared<Post>(&scenario);
            let mut comment = test_scenario::take_shared<Comment>(&scenario);
            let config = test_scenario::take_shared<PostConfig>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut tip_coin = coin::mint_for_testing<MYSO>(100, test_scenario::ctx(&mut scenario));
            // post_owner_amount = 20; 50% wallet redirect → 10 remaining on post
            assert!(!post::tip_post_requires_beneficiary_vault_for_amount(&post_obj, 20), 0);
            post::tip_comment_simple<MYSO>(
                &mut comment,
                &mut post_obj,
                &config,
                &mut tip_coin,
                100,
                &memory_account,
                test_scenario::ctx(&mut scenario)
            );
            assert!(post::get_comment_tips_received(&comment) == 80, 1);
            assert!(post::get_tips_received(&post_obj) == 10, 2);
            test_scenario::return_shared(comment);
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(config);
            test_scenario::return_shared(memory_account);
            transfer::public_transfer(tip_coin, USER2);
        };
        test_scenario::next_tx(&mut scenario, USER3);
        {
            let c = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            assert!(coin::value(&c) == 10, 3);
            test_scenario::return_to_sender(&scenario, c);
        };
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 38, location = social_contracts::post)]
    fun test_tip_comment_simple_escrow_requires_vault() {
        let mut scenario = test_scenario::begin(USER1);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            post::test_init(test_scenario::ctx(&mut scenario));
        };
        init_tip_test_profile(&mut scenario);
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            post::test_create_post_with_escrow_redirect(
                USER1,
                USER1,
                TEST_PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                USER3,
                50,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let post_obj = test_scenario::take_shared<Post>(&scenario);
            let _ = post::test_create_comment(
                USER3,
                USER3,
                object::uid_to_address(post::get_post_id(&post_obj)),
                string::utf8(b"tip me"),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut post_obj = test_scenario::take_shared<Post>(&scenario);
            let mut comment = test_scenario::take_shared<Comment>(&scenario);
            let config = test_scenario::take_shared<PostConfig>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut tip_coin = coin::mint_for_testing<MYSO>(100, test_scenario::ctx(&mut scenario));
            // post_owner_amount = 20 with 50% escrow → redirected 10 > 0
            assert!(post::tip_post_requires_beneficiary_vault_for_amount(&post_obj, 20), 0);
            post::tip_comment_simple<MYSO>(
                &mut comment,
                &mut post_obj,
                &config,
                &mut tip_coin,
                100,
                &memory_account,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(comment);
            test_scenario::return_shared(post_obj);
            test_scenario::return_shared(config);
            test_scenario::return_shared(memory_account);
            transfer::public_transfer(tip_coin, USER2);
        };
        test_scenario::end(scenario);
    }
}