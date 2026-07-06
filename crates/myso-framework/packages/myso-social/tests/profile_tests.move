// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_const, duplicate_alias, unused_use)]
module social_contracts::profile_tests {
    use std::string::{Self, String};
    use std::option;
    
    use myso::test_scenario;
    use myso::object;
    use social_contracts::memory::{MemoryRegistry, MemoryAccount};
    use social_contracts::ai_credit::AiCreditConfig;
    use social_contracts::profile::{
        Self,
        Profile,
        UsernameRegistry,
        EcosystemTreasury,
        VestingWallet,
        EcosystemBadgeAdminCap,
        UsernameAdminCap,
        ProfileConfig,
        UsernameMarketplace,
    };
    use myso::url;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::clock::{Self, Clock};
    use myso::event;
    use myso::transfer;
    
    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const USER2: address = @0x2;
    const USER3: address = @0x3;
    
    #[test]
    fun test_create_profile() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            // Initialize the UsernameRegistry
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            
            // Mint coins for test
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };
        
        // Create a profile
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            
            // Create profile
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"User One"),
                string::utf8(b"testname"),
                string::utf8(b"This is my bio"),
                b"https://example.com/image.png",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };
        
        // Check profile properties in the next transaction
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            // Check profile properties
            let display_name_opt = profile::display_name(&profile);
            assert!(option::is_some(&display_name_opt), 0);
            assert!(option::borrow(&display_name_opt) == &string::utf8(b"User One"), 0);
            assert!(profile::bio(&profile) == string::utf8(b"This is my bio"), 0);
            assert!(profile::owner(&profile) == USER1, 0);
            
            test_scenario::return_to_sender(&scenario, profile);
        };
        
        test_scenario::end(scenario);
    }

    #[test]
    fun test_registry_username_stores_ascii_lowercase() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"Display"),
                string::utf8(b"MiXeDcAsE"),
                string::utf8(b"bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let profile_id = object::uid_to_address(profile::id(&profile));
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let opt = profile::lookup_profile_by_username(&registry, string::utf8(b"mixedcase"));
            assert!(option::is_some(&opt), 0);
            assert!(*option::borrow(&opt) == profile_id, 0);
            test_scenario::return_shared(registry);
            test_scenario::return_to_sender(&scenario, profile);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let opt = profile::lookup_profile_by_username(&registry, string::utf8(b"mixedcase"));
            assert!(option::is_some(&opt), 0);
            test_scenario::return_shared(registry);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_username_accepts_dot_underscore_digit() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"Dot User"),
                string::utf8(b"user.name_1"),
                string::utf8(b"bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let profile_id = object::uid_to_address(profile::id(&profile));
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let opt = profile::lookup_profile_by_username(&registry, string::utf8(b"user.name_1"));
            assert!(option::is_some(&opt), 0);
            assert!(*option::borrow(&opt) == profile_id, 0);
            test_scenario::return_shared(registry);
            test_scenario::return_to_sender(&scenario, profile);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::EInvalidUsername, location = social_contracts::profile)]
    fun test_username_rejects_at_sign() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"Bad"),
                string::utf8(b"user@name"),
                string::utf8(b""),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::EInvalidUsername, location = social_contracts::profile)]
    fun test_username_rejects_hyphen_and_slash() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"Bad"),
                string::utf8(b"user-name/1"),
                string::utf8(b""),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_admin_revoke_username() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            let cap = profile::create_username_admin_cap(test_scenario::ctx(&mut scenario));
            transfer::public_transfer(cap, ADMIN);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"User One"),
                string::utf8(b"revokeme"),
                string::utf8(b"bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let cap = test_scenario::take_from_sender<UsernameAdminCap>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            profile::admin_revoke_username(
                &cap,
                &mut registry,
                string::utf8(b"revokeme"),
                1,
                test_scenario::ctx(&mut scenario),
            );
            assert!(profile::is_username_available(&registry, string::utf8(b"revokeme")), 0);
            test_scenario::return_shared(registry);
            test_scenario::return_to_sender(&scenario, cap);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::EUsernameLocked, location = social_contracts::profile)]
    fun test_admin_revoke_marketplace_locked_username_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            let cap = profile::create_username_admin_cap(test_scenario::ctx(&mut scenario));
            transfer::public_transfer(cap, ADMIN);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"Seller"),
                string::utf8(b"lockme"),
                string::utf8(b""),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_username_listing(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"lockme"),
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let cap = test_scenario::take_from_sender<UsernameAdminCap>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            profile::admin_revoke_username(
                &cap,
                &mut registry,
                string::utf8(b"lockme"),
                1,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(registry);
            test_scenario::return_to_sender(&scenario, cap);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_admin_reassign_username() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            let cap = profile::create_username_admin_cap(test_scenario::ctx(&mut scenario));
            transfer::public_transfer(cap, ADMIN);
            clock::share_for_testing(clock);
            let c1 = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            let c2 = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(c1, USER1);
            transfer::public_transfer(c2, USER2);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"User One"),
                string::utf8(b"handoff"),
                string::utf8(b"bio1"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"User Two"),
                string::utf8(b"user2"),
                string::utf8(b"bio2"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let cap = test_scenario::take_from_sender<UsernameAdminCap>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile2 = test_scenario::take_from_address<Profile>(&scenario, USER2);
            let profile2_id = object::uid_to_address(profile::id(&profile2));
            profile::admin_reassign_username(
                &cap,
                &mut registry,
                string::utf8(b"handoff"),
                profile2_id,
                2,
                test_scenario::ctx(&mut scenario),
            );
            let opt = profile::lookup_profile_by_username(&registry, string::utf8(b"handoff"));
            assert!(option::is_some(&opt), 0);
            assert!(*option::borrow(&opt) == profile2_id, 1);
            test_scenario::return_to_address(USER2, profile2);
            test_scenario::return_shared(registry);
            test_scenario::return_to_sender(&scenario, cap);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::EUsernameProfileMismatch, location = social_contracts::profile)]
    fun test_admin_reassign_same_profile_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            let cap = profile::create_username_admin_cap(test_scenario::ctx(&mut scenario));
            transfer::public_transfer(cap, ADMIN);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"User One"),
                string::utf8(b"samename"),
                string::utf8(b"bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let cap = test_scenario::take_from_sender<UsernameAdminCap>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile1 = test_scenario::take_from_address<Profile>(&scenario, USER1);
            let profile1_id = object::uid_to_address(profile::id(&profile1));
            profile::admin_reassign_username(
                &cap,
                &mut registry,
                string::utf8(b"samename"),
                profile1_id,
                0,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_to_address(USER1, profile1);
            test_scenario::return_shared(registry);
            test_scenario::return_to_sender(&scenario, cap);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::EUsernameNotAvailable, location = social_contracts::profile)]
    fun test_registry_username_duplicate_ascii_case_conflicts() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            let c1 = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            let c2 = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(c1, USER1);
            transfer::public_transfer(c2, USER2);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"U1"),
                string::utf8(b"MiXeDcAsE"),
                string::utf8(b""),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"U2"),
                string::utf8(b"MIXEDCASE"),
                string::utf8(b""),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_update_profile() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            // Initialize the UsernameRegistry
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            
            // Mint coins for test
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };
        
        // Create a profile
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            
            // Create profile
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"Original Name"),
                string::utf8(b"username"),
                string::utf8(b"Original bio"),
                b"https://example.com/image.png",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };
        
        // Update the profile in the next transaction
        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut profile = test_scenario::take_from_sender<Profile>(&scenario);
            
            profile::update_profile(
                &mut profile,
                string::utf8(b"Updated Name"),
                string::utf8(b"Updated bio"),
                b"https://example.com/new_image.png",
                b"https://example.com/new_cover.png",
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            // Check updated properties
            let display_name_opt = profile::display_name(&profile);
            assert!(option::is_some(&display_name_opt), 0);
            assert!(option::borrow(&display_name_opt) == &string::utf8(b"Updated Name"), 0);
            assert!(profile::bio(&profile) == string::utf8(b"Updated bio"), 0);
            
            test_scenario::return_to_sender(&scenario, profile);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }

    #[test]
    fun test_update_profile_website_birthdate_location() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"Name"),
                string::utf8(b"userloc"),
                string::utf8(b"bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut profile = test_scenario::take_from_sender<Profile>(&scenario);

            profile::update_profile(
                &mut profile,
                string::utf8(b""),
                string::utf8(b""),
                b"",
                b"",
                option::some(string::utf8(b"https://example.com")),
                option::some(string::utf8(b"1990-01-01")),
                option::some(string::utf8(b"New York")),
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            let website = profile::website(&profile);
            assert!(option::is_some(&website), 0);
            assert!(option::borrow(&website) == &string::utf8(b"https://example.com"), 1);

            let birthdate = profile::birthdate(&profile);
            assert!(option::is_some(&birthdate), 2);
            assert!(option::borrow(&birthdate) == &string::utf8(b"1990-01-01"), 3);

            let location = profile::location(&profile);
            assert!(option::is_some(&location), 4);
            assert!(option::borrow(&location) == &string::utf8(b"New York"), 5);

            profile::update_profile(
                &mut profile,
                string::utf8(b""),
                string::utf8(b""),
                b"",
                b"",
                option::some(string::utf8(b"")),
                option::some(string::utf8(b"")),
                option::some(string::utf8(b"")),
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            let website = profile::website(&profile);
            let birthdate = profile::birthdate(&profile);
            let location = profile::location(&profile);
            assert!(option::is_none(&website), 6);
            assert!(option::is_none(&birthdate), 7);
            assert!(option::is_none(&location), 8);

            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }
    
    #[test]
    #[expected_failure(abort_code = profile::EUnauthorized, location = social_contracts::profile)]
    fun test_unauthorized_update() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            // Initialize the UsernameRegistry
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            
            // Mint coins for test
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };
        
        // Create a profile
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            
            // Create profile
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"User One"),
                string::utf8(b"myusername"),
                string::utf8(b"This is my bio"),
                b"https://example.com/image.png",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(registry);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };
        
        // User2 tries to update User1's profile
        test_scenario::next_tx(&mut scenario, USER2);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut profile = test_scenario::take_from_address<Profile>(&scenario, USER1);
            
            // This should fail with EUnauthorized
            profile::update_profile(
                &mut profile,
                string::utf8(b"Hacked Name"),
                string::utf8(b"Hacked bio"),
                b"https://example.com/hacked.png",
                b"https://example.com/hacked_cover.png",
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_to_address(USER1, profile);

            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }

    #[test]
    fun test_admin_set_profile_x_username() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
            let cap = profile::create_ecosystem_badge_admin_cap(test_scenario::ctx(&mut scenario));
            transfer::public_transfer(cap, ADMIN);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"User One"),
                string::utf8(b"xuser"),
                string::utf8(b"bio"),
                b"https://example.com/p.png",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let cap = test_scenario::take_from_sender<EcosystemBadgeAdminCap>(&scenario);
            let mut profile = test_scenario::take_from_address<Profile>(&scenario, USER1);
            profile::admin_set_profile_x_username(
                &cap,
                &mut profile,
                option::some(string::utf8(b"verified_handle")),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_to_sender(&scenario, cap);
            test_scenario::return_to_address(USER1, profile);

            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let x = profile::x_username(&profile);
            assert!(option::is_some(x), 0);
            assert!(option::borrow(x) == &string::utf8(b"verified_handle"), 1);
            test_scenario::return_to_sender(&scenario, profile);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_admin_clears_profile_x_username() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
            let cap = profile::create_ecosystem_badge_admin_cap(test_scenario::ctx(&mut scenario));
            transfer::public_transfer(cap, ADMIN);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"User One"),
                string::utf8(b"xuser2"),
                string::utf8(b"bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let cap = test_scenario::take_from_sender<EcosystemBadgeAdminCap>(&scenario);
            let mut profile = test_scenario::take_from_address<Profile>(&scenario, USER1);
            profile::admin_set_profile_x_username(
                &cap,
                &mut profile,
                option::some(string::utf8(b"temp")),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_to_sender(&scenario, cap);
            test_scenario::return_to_address(USER1, profile);

            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let cap = test_scenario::take_from_sender<EcosystemBadgeAdminCap>(&scenario);
            let mut profile = test_scenario::take_from_address<Profile>(&scenario, USER1);
            profile::admin_set_profile_x_username(
                &cap,
                &mut profile,
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_to_sender(&scenario, cap);
            test_scenario::return_to_address(USER1, profile);

            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            assert!(option::is_none(profile::x_username(&profile)), 0);
            test_scenario::return_to_sender(&scenario, profile);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_update_profile_does_not_change_x_username() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));

            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
            let cap = profile::create_ecosystem_badge_admin_cap(test_scenario::ctx(&mut scenario));
            transfer::public_transfer(cap, ADMIN);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut memory_registry = test_scenario::take_shared<MemoryRegistry>(&scenario);
            let mut ai_credit_config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            profile::create_profile(
                &mut registry,
                &profile_config,
                &mut memory_registry,
                &mut ai_credit_config,
                string::utf8(b"User One"),
                string::utf8(b"xuser3"),
                string::utf8(b"original bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let cap = test_scenario::take_from_sender<EcosystemBadgeAdminCap>(&scenario);
            let mut profile = test_scenario::take_from_address<Profile>(&scenario, USER1);
            profile::admin_set_profile_x_username(
                &cap,
                &mut profile,
                option::some(string::utf8(b"admin_set")),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_to_sender(&scenario, cap);
            test_scenario::return_to_address(USER1, profile);

            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut profile = test_scenario::take_from_sender<Profile>(&scenario);
            profile::update_profile(
                &mut profile,
                string::utf8(b"User One"),
                string::utf8(b"new bio from owner"),
                b"",
                b"",
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            let x = profile::x_username(&profile);
            assert!(option::is_some(x), 0);
            assert!(option::borrow(x) == &string::utf8(b"admin_set"), 1);
            assert!(profile::bio(&profile) == string::utf8(b"new bio from owner"), 2);
            test_scenario::return_to_sender(&scenario, profile);

            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    // === Username Marketplace Tests ===

    fun setup_marketplace_seller(
        scenario: &mut test_scenario::Scenario,
        seller: address,
        username: String,
    ) {
        test_scenario::next_tx(scenario, seller);
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
                string::utf8(b"Seller"),
                username,
                string::utf8(b"seller bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(profile_config);
        };
    }

    fun setup_marketplace_buyer(
        scenario: &mut test_scenario::Scenario,
        buyer: address,
        username: String,
    ) {
        test_scenario::next_tx(scenario, buyer);
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
                string::utf8(b"Buyer"),
                username,
                string::utf8(b"buyer bio"),
                b"",
                b"",
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(profile_config);
        };
    }

    #[test]
    fun test_create_username_offer() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER1,
            );
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
            );
        };

        setup_marketplace_seller(&mut scenario, USER1, string::utf8(b"user1"));
        setup_marketplace_buyer(&mut scenario, USER2, string::utf8(b"user2"));

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_listing(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"user1"),
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            assert!(profile::is_username_listed(&marketplace, string::utf8(b"user1")), 0);

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_offer(
                &mut marketplace,
                &registry,
                string::utf8(b"user1"),
                &mut coins,
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            assert!(profile::has_username_offer_from(&marketplace, string::utf8(b"user1"), USER2), 1);
            assert!(profile::has_username_offers(&marketplace, string::utf8(b"user1")), 2);

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_accept_username_offer() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER1,
            );
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
            );
        };

        setup_marketplace_seller(&mut scenario, USER1, string::utf8(b"user1"));
        setup_marketplace_buyer(&mut scenario, USER2, string::utf8(b"user2"));

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_listing(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"user1"),
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_offer(
                &mut marketplace,
                &registry,
                string::utf8(b"user1"),
                &mut coins,
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);

            let seller_profile_id = profile::get_id_address(&profile);
            let seller_memory_id = *option::borrow(profile::linked_memory_account_id(&profile));
            let buyer_profile_id = *option::borrow(&profile::lookup_profile_by_owner(&registry, USER2));

            let events_before = event::num_events();
            profile::accept_username_offer(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"user1"),
                USER2,
                string::utf8(b"user1_legacy"),
                &profile_config,
                &treasury,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            // Revoke buyer prior, marketplace unlock release, sale settled, offer accepted,
            // sale fee — no replacement UsernameClaimedEvent (would be 6 with claim_username).
            assert!(event::num_events() - events_before == 5, 11);

            let listed_owner = *option::borrow(&profile::lookup_profile_by_username(
                &registry,
                string::utf8(b"user1"),
            ));
            let replacement_owner = *option::borrow(&profile::lookup_profile_by_username(
                &registry,
                string::utf8(b"user1_legacy"),
            ));
            assert!(listed_owner == buyer_profile_id, 1);
            assert!(replacement_owner == seller_profile_id, 2);
            assert!(
                *option::borrow(&profile::lookup_profile_by_owner(&registry, USER1)) == seller_profile_id,
                3,
            );
            assert!(
                *option::borrow(&profile::lookup_profile_by_owner(&registry, USER2)) == buyer_profile_id,
                4,
            );
            assert!(profile::owner(&profile) == USER1, 5);
            assert!(*option::borrow(profile::linked_memory_account_id(&profile)) == seller_memory_id, 6);

            // 1:1 invariant: buyer's prior username `user2` is freed (one username per wallet),
            // and the marketplace reservation on `user1` is released post-settlement.
            assert!(option::is_none(&profile::lookup_profile_by_username(&registry, string::utf8(b"user2"))), 8);
            assert!(!profile::is_username_locked(&registry, string::utf8(b"user1")), 9);
            assert!(profile::is_username_available(&registry, string::utf8(b"user2")), 10);

            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(profile_config);
            test_scenario::return_shared(marketplace);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let buyer_profile = test_scenario::take_from_sender<Profile>(&scenario);
            assert!(profile::owner(&buyer_profile) == USER2, 7);
            test_scenario::return_to_sender(&scenario, buyer_profile);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_reject_username_offer() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER1,
            );
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
            );
        };

        setup_marketplace_seller(&mut scenario, USER1, string::utf8(b"user1"));
        setup_marketplace_buyer(&mut scenario, USER2, string::utf8(b"user2"));

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_listing(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"user1"),
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_offer(
                &mut marketplace,
                &registry,
                string::utf8(b"user1"),
                &mut coins,
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::reject_or_revoke_username_offer(
                &mut marketplace,
                &profile,
                string::utf8(b"user1"),
                USER2,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            assert!(!profile::has_username_offers(&marketplace, string::utf8(b"user1")), 1);
            assert!(profile::owner(&profile) == USER1, 2);

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            test_scenario::return_to_sender(&scenario, coins);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_revoke_username_offer() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER1,
            );
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
            );
        };

        setup_marketplace_seller(&mut scenario, USER1, string::utf8(b"user1"));
        setup_marketplace_buyer(&mut scenario, USER2, string::utf8(b"user2"));

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_listing(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"user1"),
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_offer(
                &mut marketplace,
                &registry,
                string::utf8(b"user1"),
                &mut coins,
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let profile = test_scenario::take_from_address<Profile>(&scenario, USER1);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::reject_or_revoke_username_offer(
                &mut marketplace,
                &profile,
                string::utf8(b"user1"),
                USER2,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            assert!(!profile::has_username_offers(&marketplace, string::utf8(b"user1")), 1);

            test_scenario::return_shared(clock);
            test_scenario::return_to_address(USER1, profile);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::ECannotOfferOwnProfile, location = social_contracts::profile)]
    fun test_cannot_offer_own_username() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER1,
            );
        };

        setup_marketplace_seller(&mut scenario, USER1, string::utf8(b"user1"));

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_listing(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"user1"),
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_offer(
                &mut marketplace,
                &registry,
                string::utf8(b"user1"),
                &mut coins,
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::EOfferDoesNotExist, location = social_contracts::profile)]
    fun test_accept_nonexistent_username_offer() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER1,
            );
        };

        setup_marketplace_seller(&mut scenario, USER1, string::utf8(b"user1"));

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_listing(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"user1"),
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);

            profile::accept_username_offer(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"user1"),
                USER2,
                string::utf8(b"user1_legacy"),
                &profile_config,
                &treasury,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(treasury);
            test_scenario::return_shared(profile_config);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(marketplace);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::EUnauthorizedOfferAction, location = social_contracts::profile)]
    fun test_unauthorized_username_offer_rejection() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER1,
            );
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
            );
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER3,
            );
        };

        setup_marketplace_seller(&mut scenario, USER1, string::utf8(b"user1"));
        setup_marketplace_buyer(&mut scenario, USER2, string::utf8(b"user2"));

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_listing(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"user1"),
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_offer(
                &mut marketplace,
                &registry,
                string::utf8(b"user1"),
                &mut coins,
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER3);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let profile = test_scenario::take_from_address<Profile>(&scenario, USER1);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::reject_or_revoke_username_offer(
                &mut marketplace,
                &profile,
                string::utf8(b"user1"),
                USER2,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_address(USER1, profile);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::EOfferBelowMinimum, location = social_contracts::profile)]
    fun test_username_offer_below_minimum() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER1,
            );
            transfer::public_transfer(
                coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
            );
        };

        setup_marketplace_seller(&mut scenario, USER1, string::utf8(b"user1"));
        setup_marketplace_buyer(&mut scenario, USER2, string::utf8(b"user2"));

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let profile = test_scenario::take_from_sender<Profile>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_listing(
                &mut marketplace,
                &mut registry,
                &profile,
                string::utf8(b"user1"),
                10_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, profile);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let mut marketplace = test_scenario::take_shared<UsernameMarketplace>(&scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            profile::create_username_offer(
                &mut marketplace,
                &registry,
                string::utf8(b"user1"),
                &mut coins,
                5_000_000_000,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(marketplace);
        };

        test_scenario::end(scenario);
    }
    // === Vesting Tests ===

    fun linear_piece_vectors(duration: u64): (
        vector<u8>,
        vector<u64>,
        vector<u64>,
        vector<u64>,
        vector<u64>,
    ) {
        (
            vector[1u8],
            vector[0u64],
            vector[duration],
            vector[10_000u64],
            vector[1000u64],
        )
    }

    fun vest_myso_linear(
        profile_config: &ProfileConfig,
        coin: Coin<MYSO>,
        recipient: address,
        start_time: u64,
        duration: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let (kinds, time_offsets, durations, amount_bps, curve_factors) =
            linear_piece_vectors(duration);
        profile::vest_myso(
            profile_config,
            coin,
            recipient,
            start_time,
            kinds,
            time_offsets,
            durations,
            amount_bps,
            curve_factors,
            clock,
            ctx,
        );
    }

    #[test]
    fun test_vest_myso_basic() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            let vest_amount = 10_000_000_000;
            let start_time = 2000;
            let duration = 10000;

            vest_myso_linear(
                &profile_config,
                coin::split(&mut coins, vest_amount, test_scenario::ctx(&mut scenario)),
                USER2,
                start_time,
                duration,
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            assert!(profile::vesting_owner(&vesting_wallet) == USER2, 1);
            assert!(profile::vesting_total_amount(&vesting_wallet) == 10_000_000_000, 2);
            assert!(profile::vesting_start_time(&vesting_wallet) == 2000, 3);
            assert!(profile::vesting_schedule_end(&vesting_wallet) == 12000, 4);
            assert!(profile::vesting_claimed_amount(&vesting_wallet) == 0, 5);
            assert!(profile::vesting_balance(&vesting_wallet) == 10_000_000_000, 6);
            assert!(profile::vesting_piece_count(&vesting_wallet) == 1, 7);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_claim_before_vesting_starts() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            vest_myso_linear(
                &profile_config,
                coin::split(&mut coins, 10_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
                5000,
                10000,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            assert!(profile::claimable(&profile_config, &vesting_wallet, &clock) == 0, 1);
            profile::claim_vested_tokens(&profile_config, &mut vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            assert!(profile::vesting_claimed_amount(&vesting_wallet) == 0, 2);
            assert!(profile::vesting_balance(&vesting_wallet) == 10_000_000_000, 3);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_claim_during_vesting_period() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            vest_myso_linear(
                &profile_config,
                coin::split(&mut coins, 10_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
                2000,
                10000,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            let mut vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            clock::set_for_testing(&mut clock, 7000);
            assert!(profile::claimable(&profile_config, &vesting_wallet, &clock) == 5_000_000_000, 1);
            profile::claim_vested_tokens(&profile_config, &mut vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            assert!(profile::vesting_claimed_amount(&vesting_wallet) == 5_000_000_000, 2);
            assert!(profile::vesting_balance(&vesting_wallet) == 5_000_000_000, 3);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let claimed_coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            assert!(coin::value(&claimed_coins) == 5_000_000_000, 4);
            test_scenario::return_to_sender(&scenario, claimed_coins);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_claim_after_vesting_complete() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            vest_myso_linear(
                &profile_config,
                coin::split(&mut coins, 10_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
                2000,
                10000,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            let mut vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            clock::set_for_testing(&mut clock, 15000);
            assert!(profile::claimable(&profile_config, &vesting_wallet, &clock) == 10_000_000_000, 1);
            profile::claim_vested_tokens(&profile_config, &mut vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            assert!(profile::vesting_claimed_amount(&vesting_wallet) == 10_000_000_000, 2);
            assert!(profile::vesting_balance(&vesting_wallet) == 0, 3);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let claimed_coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            assert!(coin::value(&claimed_coins) == 10_000_000_000, 4);
            test_scenario::return_to_sender(&scenario, claimed_coins);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_multiple_claims() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            vest_myso_linear(
                &profile_config,
                coin::split(&mut coins, 12_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
                2000,
                12000,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            let mut vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            clock::set_for_testing(&mut clock, 5000);
            assert!(profile::claimable(&profile_config, &vesting_wallet, &clock) == 3_000_000_000, 1);
            profile::claim_vested_tokens(&profile_config, &mut vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            assert!(profile::vesting_claimed_amount(&vesting_wallet) == 3_000_000_000, 2);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            let mut vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            clock::set_for_testing(&mut clock, 11000);
            assert!(profile::claimable(&profile_config, &vesting_wallet, &clock) == 6_000_000_000, 3);
            profile::claim_vested_tokens(&profile_config, &mut vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            assert!(profile::vesting_claimed_amount(&vesting_wallet) == 9_000_000_000, 4);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            let mut vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            clock::set_for_testing(&mut clock, 20000);
            assert!(profile::claimable(&profile_config, &vesting_wallet, &clock) == 3_000_000_000, 5);
            profile::claim_vested_tokens(&profile_config, &mut vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            assert!(profile::vesting_claimed_amount(&vesting_wallet) == 12_000_000_000, 6);
            assert!(profile::vesting_balance(&vesting_wallet) == 0, 7);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_cliff_lump_unlock() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            profile::vest_myso(
                &profile_config,
                coin::split(&mut coins, 10_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
                2000,
                vector[1u8, 0u8],
                vector[0u64, 5000u64],
                vector[10000u64, 0u64],
                vector[7500u64, 2500u64],
                vector[1000u64, 0u64],
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            let mut vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            // Before cliff: continuous portion only, cliff lump not yet unlocked
            clock::set_for_testing(&mut clock, 6999);
            let before_cliff = profile::claimable(&profile_config, &vesting_wallet, &clock);
            assert!(before_cliff > 0, 1);
            assert!(before_cliff < 6_250_000_000, 2);
            // At cliff: +25% lump (2.5B) => 6.25B total vested
            clock::set_for_testing(&mut clock, 7000);
            assert!(profile::claimable(&profile_config, &vesting_wallet, &clock) == 6_250_000_000, 3);
            profile::claim_vested_tokens(&profile_config, &mut vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            assert!(profile::vesting_claimed_amount(&vesting_wallet) == 6_250_000_000, 4);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_claim_threshold_suppresses_dust() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(10_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            vest_myso_linear(
                &profile_config,
                coin::split(&mut coins, 1_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
                2000,
                10000,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            let vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            // 0.05% elapsed => 500 vested; threshold is 1000 (0.1%)
            clock::set_for_testing(&mut clock, 2005);
            assert!(profile::claimable(&profile_config, &vesting_wallet, &clock) == 0, 1);
            // 1% elapsed => 10_000 vested; above threshold
            clock::set_for_testing(&mut clock, 2100);
            assert!(profile::claimable(&profile_config, &vesting_wallet, &clock) == 10_000, 2);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_end_of_schedule_dust_sweep() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(10_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            vest_myso_linear(
                &profile_config,
                coin::split(&mut coins, 1003, test_scenario::ctx(&mut scenario)),
                USER2,
                2000,
                10000,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            let mut vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            clock::set_for_testing(&mut clock, 12000);
            // Mid-schedule tiny accrual would be suppressed; end bypasses threshold
            assert!(profile::claimable(&profile_config, &vesting_wallet, &clock) == 1003, 1);
            profile::claim_vested_tokens(&profile_config, &mut vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            assert!(profile::vesting_balance(&vesting_wallet) == 0, 2);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::ENotVestingWalletOwner, location = social_contracts::profile)]
    fun test_unauthorized_claim() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            vest_myso_linear(
                &profile_config,
                coin::split(&mut coins, 10_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
                2000,
                10000,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER3);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut vesting_wallet = test_scenario::take_from_address<VestingWallet>(&scenario, USER2);
            profile::claim_vested_tokens(&profile_config, &mut vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(clock);
            test_scenario::return_to_address(USER2, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::EInvalidStartTime, location = social_contracts::profile)]
    fun test_invalid_start_time() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 5000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            vest_myso_linear(
                &profile_config,
                coin::split(&mut coins, 10_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
                3000,
                10000,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = profile::ETooManyPieces, location = social_contracts::profile)]
    fun test_too_many_pieces_rejected() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            let mut kinds = vector::empty<u8>();
            let mut time_offsets = vector::empty<u64>();
            let mut durations = vector::empty<u64>();
            let mut amount_bps = vector::empty<u64>();
            let mut curve_factors = vector::empty<u64>();
            let mut i = 0u64;
            while (i < 12) {
                vector::push_back(&mut kinds, 0u8);
                vector::push_back(&mut time_offsets, i * 100);
                vector::push_back(&mut durations, 0);
                vector::push_back(&mut amount_bps, 833);
                vector::push_back(&mut curve_factors, 0);
                i = i + 1;
            };
            profile::vest_myso(
                &profile_config,
                coin::split(&mut coins, 10_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
                2000,
                kinds,
                time_offsets,
                durations,
                amount_bps,
                curve_factors,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(profile_config);
            test_scenario::return_to_sender(&scenario, coins);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_delete_empty_vesting_wallet() {
        let mut scenario = test_scenario::begin(ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scenario));
            clock::share_for_testing(clock);

            let mut clock = clock::create_for_testing(test_scenario::ctx(&mut scenario));
            clock::set_for_testing(&mut clock, 1000);
            clock::share_for_testing(clock);
            let coins = coin::mint_for_testing<MYSO>(20_000_000_000, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coins, USER1);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut coins = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            vest_myso_linear(
                &profile_config,
                coin::split(&mut coins, 10_000_000_000, test_scenario::ctx(&mut scenario)),
                USER2,
                2000,
                10000,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, coins);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let profile_config = test_scenario::take_shared<ProfileConfig>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            let mut vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            clock::set_for_testing(&mut clock, 15000);
            profile::claim_vested_tokens(&profile_config, &mut vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            assert!(profile::vesting_balance(&vesting_wallet) == 0, 1);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scenario, vesting_wallet);
        test_scenario::return_shared(profile_config);
        };

        test_scenario::next_tx(&mut scenario, USER2);
        {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let vesting_wallet = test_scenario::take_from_sender<VestingWallet>(&scenario);
            profile::delete_vesting_wallet(vesting_wallet, &clock, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }
}
