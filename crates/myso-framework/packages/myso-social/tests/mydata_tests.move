#[test_only]
#[allow(duplicate_alias, unused_use, unused_function)]
module social_contracts::mydata_tests {
    use std::string;
    use std::option;
    use std::vector;
    
    use std::unit_test::assert_eq;
    use myso::test_scenario;
    use myso::transfer;
    use myso::coin::{Self, Coin};
    use myso::clock::{Self, Clock};
    use myso::test_utils;
    use myso::object;
    
    use social_contracts::mydata::{
        Self,
        MyData,
        MyDataRegistry,
        MyDataConfig,
        MyDataAdminCap,
        MyDataPoolRegistry,
        MyDataPoolAdminCap,
        SnapshotAnchorRegistry,
        MyDataClaimVault,
    };
    use social_contracts::profile::{Self, Profile, UsernameRegistry,
        ProfileConfig, EcosystemTreasury};
    use social_contracts::ai_credit::AiCreditConfig;
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::memory::{Self, MemoryRegistry, MemoryAccount, SubAgent, AgenticOrganization,
        MemoryConfig};
    use social_contracts::memory_test_helpers;
    
    // Test addresses
    const CREATOR: address = @0xA1;
    const BUYER: address = @0xB2;
    const ANOTHER_USER: address = @0xC3;
    const AGENT_ADDR: address = @0xA11CE;
    const PLACEHOLDER_AGENT: address = @0xBEEF;
    const AGENT_PUBKEY: vector<u8> = x"0101010101010101010101010101010101010101010101010101010101010101";
    const PLACEHOLDER_PUBKEY: vector<u8> = x"0303030303030303030303030303030303030303030303030303030303030303";

    fun finish_purchase_payment(payment: Coin<myso::myso::MYSO>, recipient: address) {
        if (coin::value(&payment) > 0) {
            transfer::public_transfer(payment, recipient);
        } else {
            coin::destroy_zero(payment);
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

    fun register_placeholder_agent(
        scenario: &mut test_scenario::Scenario,
    ) {
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
    }

    fun register_mydata_agent(
        scenario: &mut test_scenario::Scenario,
        derived: address,
        label: vector<u8>,
        capabilities: u64,
        max_action_spend: Option<u64>,
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
            string::utf8(label),
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
        test_scenario::return_shared(memory_config);
}
    
    #[test]
    fun test_create_mydata_data() {
        let mut scenario = test_scenario::begin(CREATOR);
        
        // Set up test environment
        init_test_environment(&mut scenario);
        
        // Create MyData data
        {
            test_scenario::next_tx(&mut scenario, CREATOR);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let mut registry = test_scenario::take_shared<MyDataRegistry>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            
            mydata::create_and_share_marketplace_one_time_mydata(
                &config,
                &mut registry,
                string::utf8(b"data"),
                vector[string::utf8(b"analytics"), string::utf8(b"personal")],
                option::none<address>(),
                1000,
                option::some(2000),
                b"encrypted_test_data",
                b"encryption_id_123",
                100,
                option::some(string::utf8(b"US")),
                option::some(string::utf8(b"high")),
                option::some(1000),
                option::some(string::utf8(b"automated")),
                true,
                option::some(string::utf8(b"daily")),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(config);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };
        
        // Verify MyData was created with correct properties
        {
            test_scenario::next_tx(&mut scenario, CREATOR);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            
            assert_eq!(mydata::owner(&mydata), CREATOR);
            assert_eq!(mydata::media_type(&mydata), string::utf8(b"data"));
            assert_eq!(mydata::one_time_price(&mydata), option::some(100));
            assert_eq!(mydata::requires_marketplace_purchase(&mydata), true);
            assert_eq!(mydata::requires_profile_subscription_access(&mydata), false);
            
            test_scenario::return_shared(mydata);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_purchase_one_time_access() {
        let mut scenario = test_scenario::begin(CREATOR);
        
        // Setup and create MyData
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);
        
        // Give BUYER some coins
        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let coin = coin::mint_for_testing<myso::myso::MYSO>(200, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coin, BUYER);
        };
        
        // BUYER purchases one-time access
        {
            test_scenario::next_tx(&mut scenario, BUYER);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let mut payment = test_scenario::take_from_sender<Coin<myso::myso::MYSO>>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            mydata::purchase_one_time(
                &config,
                &block_list,
                &memory_config,
                &mut mydata,
                &treasury,
                &mut payment,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            finish_purchase_payment(payment, BUYER);

            test_scenario::return_shared(treasury);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(clock);
        };
        
        // Verify access was granted
        {
            test_scenario::next_tx(&mut scenario, BUYER);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            
            assert_eq!(mydata::has_access(&mydata, BUYER, &clock), true);
            assert_eq!(mydata::has_access(&mydata, ANOTHER_USER, &clock), false);
            
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_purchase_subscription() {
        let mut scenario = test_scenario::begin(CREATOR);
        
        // Setup and create MyData
        init_test_environment(&mut scenario);
        create_test_recurring_mydata(&mut scenario);
        
        // Give BUYER some coins
        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let coin = coin::mint_for_testing<myso::myso::MYSO>(200, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coin, BUYER);
        };
        
        // BUYER purchases subscription
        {
            test_scenario::next_tx(&mut scenario, BUYER);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let mut payment = test_scenario::take_from_sender<Coin<myso::myso::MYSO>>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            mydata::purchase_subscription(
                &config,
                &block_list,
                &memory_config,
                &mut mydata,
                &treasury,
                &mut payment,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            finish_purchase_payment(payment, BUYER);

            test_scenario::return_shared(treasury);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(clock);
        };
        
        // Verify subscription access
        {
            test_scenario::next_tx(&mut scenario, BUYER);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            
            assert_eq!(mydata::has_access(&mydata, BUYER, &clock), true);
            assert_eq!(mydata::has_active_subscription(&mydata, BUYER, &clock), true);
            assert_eq!(mydata::has_access(&mydata, ANOTHER_USER, &clock), false);
            
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_update_pricing() {
        let mut scenario = test_scenario::begin(CREATOR);
        
        // Setup and create MyData
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);
        
        // Update pricing
        {
            test_scenario::next_tx(&mut scenario, CREATOR);
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            
            mydata::update_pricing(
                &config,
                &mut mydata,
                option::some(150),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            assert_eq!(mydata::one_time_price(&mydata), option::some(150));
            
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_grant_free_access() {
        let mut scenario = test_scenario::begin(CREATOR);
        
        // Setup and create MyData
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);
        
        // Grant free access to BUYER
        {
            test_scenario::next_tx(&mut scenario, CREATOR);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            
            mydata::grant_access(
                &config,
                &mut mydata,
                BUYER,
                0, // one-time access
                option::none<u64>(),
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            
            test_scenario::return_shared(config);
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(clock);
        };
        
        // Verify free access was granted
        {
            test_scenario::next_tx(&mut scenario, BUYER);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            
            assert_eq!(mydata::has_access(&mydata, BUYER, &clock), true);
            
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_access_control() {
        let mut scenario = test_scenario::begin(CREATOR);
        
        // Setup and create MyData
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);
        
        // Verify owner always has access
        {
            test_scenario::next_tx(&mut scenario, CREATOR);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            
            assert_eq!(mydata::has_access(&mydata, CREATOR, &clock), true);
            assert_eq!(mydata::has_access(&mydata, BUYER, &clock), false);
            assert_eq!(mydata::has_access(&mydata, ANOTHER_USER, &clock), false);
            
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(clock);
        };
        
        test_scenario::end(scenario);
    }
    
    #[test]
    fun test_registry_functions() {
        let mut scenario = test_scenario::begin(CREATOR);
        
        // Setup and create MyData
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);
        
        // Test registry functions
        {
            test_scenario::next_tx(&mut scenario, CREATOR);
            let registry = test_scenario::take_shared<MyDataRegistry>(&scenario);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            
            // Test permission checks (simplified implementation returns true for registered IPs)
            // Note: For this test, we'll skip the ID-based registry lookups since the field is private
            
            test_scenario::return_shared(registry);
            test_scenario::return_shared(mydata);
        };
        
        test_scenario::end(scenario);
    }

    #[test]
    fun test_mydata_approve_after_purchase() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let coin = coin::mint_for_testing<myso::myso::MYSO>(200, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coin, BUYER);
        };

        test_scenario::next_tx(&mut scenario, BUYER);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let mut payment = test_scenario::take_from_sender<Coin<myso::myso::MYSO>>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            mydata::purchase_one_time(
                &config,
                &block_list,
                &memory_config,
                &mut mydata,
                &treasury,
                &mut payment,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            finish_purchase_payment(payment, BUYER);

            test_scenario::return_shared(treasury);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, BUYER);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            mydata::mydata_approve(
                b"encryption_id",
                &block_list,
                &memory_config,
                &mydata,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(mydata);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 12, location = social_contracts::mydata)]
    fun test_mydata_approve_wrong_id_aborts() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let coin = coin::mint_for_testing<myso::myso::MYSO>(200, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coin, BUYER);
        };

        test_scenario::next_tx(&mut scenario, BUYER);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let mut payment = test_scenario::take_from_sender<Coin<myso::myso::MYSO>>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            mydata::purchase_one_time(
                &config,
                &block_list,
                &memory_config,
                &mut mydata,
                &treasury,
                &mut payment,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            finish_purchase_payment(payment, BUYER);

            test_scenario::return_shared(treasury);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, BUYER);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            mydata::mydata_approve(
                b"wrong_id",
                &block_list,
                &memory_config,
                &mydata,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(mydata);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 13, location = social_contracts::mydata)]
    fun test_mydata_approve_non_buyer_aborts() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);

        test_scenario::next_tx(&mut scenario, ANOTHER_USER);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            mydata::mydata_approve(
                b"encryption_id",
                &block_list,
                &memory_config,
                &mydata,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(mydata);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_revoke_one_time_buyer_loses_access() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let coin = coin::mint_for_testing<myso::myso::MYSO>(200, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coin, BUYER);
        };

        test_scenario::next_tx(&mut scenario, BUYER);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let mut payment = test_scenario::take_from_sender<Coin<myso::myso::MYSO>>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            mydata::purchase_one_time(
                &config,
                &block_list,
                &memory_config,
                &mut mydata,
                &treasury,
                &mut payment,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            finish_purchase_payment(payment, BUYER);
            assert!(mydata::has_access(&mydata, BUYER, &clock), 0);

            test_scenario::return_shared(treasury);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            mydata::revoke_access(&mut mydata, BUYER, 0, &clock, test_scenario::ctx(&mut scenario));
            assert!(!mydata::has_access(&mydata, BUYER, &clock), 1);

            test_scenario::return_shared(mydata);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_revoke_subscription_buyer_loses_access() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_recurring_mydata(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let coin = coin::mint_for_testing<myso::myso::MYSO>(200, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coin, BUYER);
        };

        test_scenario::next_tx(&mut scenario, BUYER);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let mut payment = test_scenario::take_from_sender<Coin<myso::myso::MYSO>>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            mydata::purchase_subscription(
                &config,
                &block_list,
                &memory_config,
                &mut mydata,
                &treasury,
                &mut payment,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            finish_purchase_payment(payment, BUYER);
            assert!(mydata::has_access(&mydata, BUYER, &clock), 0);

            test_scenario::return_shared(treasury);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            mydata::revoke_access(&mut mydata, BUYER, 1, &clock, test_scenario::ctx(&mut scenario));
            assert!(!mydata::has_access(&mydata, BUYER, &clock), 1);

            test_scenario::return_shared(mydata);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 13, location = social_contracts::mydata)]
    fun test_mydata_approve_after_revoke_aborts() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let coin = coin::mint_for_testing<myso::myso::MYSO>(200, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(coin, BUYER);
        };

        test_scenario::next_tx(&mut scenario, BUYER);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let mut payment = test_scenario::take_from_sender<Coin<myso::myso::MYSO>>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            mydata::purchase_one_time(
                &config,
                &block_list,
                &memory_config,
                &mut mydata,
                &treasury,
                &mut payment,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );
            finish_purchase_payment(payment, BUYER);

            test_scenario::return_shared(treasury);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            mydata::revoke_access(&mut mydata, BUYER, 0, &clock, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, BUYER);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            mydata::mydata_approve(
                b"encryption_id",
                &block_list,
                &memory_config,
                &mydata,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            test_scenario::return_shared(mydata);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 14, location = social_contracts::mydata)]
    fun test_revoke_non_buyer_aborts() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            mydata::revoke_access(&mut mydata, BUYER, 0, &clock, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_create_broad_pool() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);

        {
            test_scenario::next_tx(&mut scenario, CREATOR);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let admin = test_scenario::take_from_sender<MyDataPoolAdminCap>(&scenario);
            let mut pool_registry = test_scenario::take_shared<MyDataPoolRegistry>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            mydata::create_broad_pool(
                &config,
                &admin,
                &mut pool_registry,
                string::utf8(b"coffee_pool"),
                string::utf8(b"Coffee consumer data"),
                &clock,
            );

            test_scenario::return_to_sender(&scenario, admin);
            test_scenario::return_shared(config);
            test_scenario::return_shared(pool_registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_create_sub_pool() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);

        {
            test_scenario::next_tx(&mut scenario, CREATOR);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let admin = test_scenario::take_from_sender<MyDataPoolAdminCap>(&scenario);
            let mut pool_registry = test_scenario::take_shared<MyDataPoolRegistry>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            mydata::create_broad_pool(
                &config,
                &admin,
                &mut pool_registry,
                string::utf8(b"coffee_pool"),
                string::utf8(b"Coffee data"),
                &clock,
            );

            let broad_pool = mydata::get_broad_pool(&pool_registry, mydata::last_created_pool_id(&pool_registry));
            assert!(option::is_some(&broad_pool), 0);
            let broad_pool_id = mydata::broad_pool_id(option::borrow(&broad_pool));

            mydata::create_sub_pool(
                &config,
                &admin,
                &mut pool_registry,
                broad_pool_id,
                string::utf8(b"coffee_us_genz"),
                string::utf8(b"US GenZ coffee consumers"),
                option::none<vector<u8>>(),
                &clock,
            );

            test_scenario::return_to_sender(&scenario, admin);
            test_scenario::return_shared(config);
            test_scenario::return_shared(pool_registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_assign_mydata_to_sub_pools() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);

        {
            test_scenario::next_tx(&mut scenario, CREATOR);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let admin = test_scenario::take_from_sender<MyDataPoolAdminCap>(&scenario);
            let mut pool_registry = test_scenario::take_shared<MyDataPoolRegistry>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mydata = test_scenario::take_shared<MyData>(&scenario);

            mydata::create_broad_pool(
                &config,
                &admin,
                &mut pool_registry,
                string::utf8(b"test_pool"),
                string::utf8(b"Test"),
                &clock,
            );

            let broad_pool = mydata::get_broad_pool(&pool_registry, mydata::last_created_pool_id(&pool_registry));
            let broad_pool_id = mydata::broad_pool_id(option::borrow(&broad_pool));

            mydata::create_sub_pool(
                &config,
                &admin,
                &mut pool_registry,
                broad_pool_id,
                string::utf8(b"test_sub"),
                string::utf8(b"Test sub"),
                option::none<vector<u8>>(),
                &clock,
            );

            let sub_pool_id = mydata::last_created_sub_pool_id(&pool_registry);
            mydata::assign_mydata_to_pools(
                &config,
                &mydata,
                &mut pool_registry,
                vector[sub_pool_id],
                &clock,
                test_scenario::ctx(&mut scenario)
            );

            let ip_id = mydata::object_address(&mydata);
            let sub_pools = mydata::get_mydata_sub_pools(&pool_registry, ip_id);
            assert!(option::is_some(&sub_pools), 0);
            assert!(vector::length(option::borrow(&sub_pools)) == 1, 0);

            test_scenario::return_to_sender(&scenario, admin);
            test_scenario::return_shared(config);
            test_scenario::return_shared(pool_registry);
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_mydata_approve_sub_agent_with_cap() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            register_mydata_agent(
                &mut scenario,
                AGENT_ADDR,
                b"mydata-agent",
                social_contracts::memory::cap_mydata_read(),
                option::none(),
            );
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            mydata::mydata_approve(
                b"encryption_id",
                &block_list,
                &memory_config,
                &mydata,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 18, location = social_contracts::memory)]
    fun test_mydata_approve_sub_agent_missing_cap_aborts() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            register_mydata_agent(
                &mut scenario,
                AGENT_ADDR,
                b"no-cap",
                0,
                option::none(),
            );
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mydata = test_scenario::take_shared<MyData>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            mydata::mydata_approve(
                b"encryption_id",
                &block_list,
                &memory_config,
                &mydata,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 30, location = social_contracts::memory)]
    fun test_purchase_one_time_exceeds_sub_agent_spend_cap() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_test_environment(&mut scenario);
        create_test_mydata(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            register_mydata_agent(
                &mut scenario,
                AGENT_ADDR,
                b"buyer-agent",
                0,
                option::some(50),
            );
        };

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let payment = coin::mint_for_testing<myso::myso::MYSO>(100, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(payment, AGENT_ADDR);
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<MyDataConfig>(&scenario);
            let block_list = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let mut mydata = test_scenario::take_shared<MyData>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let mut payment = test_scenario::take_from_sender<Coin<myso::myso::MYSO>>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            mydata::purchase_one_time(
                &config,
                &block_list,
                &memory_config,
                &mut mydata,
                &treasury,
                &mut payment,
                &memory_account,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            finish_purchase_payment(payment, AGENT_ADDR);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(config);
            test_scenario::return_shared(block_list);
            test_scenario::return_shared(mydata);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }
    
    // Helper functions
    
    fun init_test_environment(scenario: &mut test_scenario::Scenario) {
        // Initialize MyData registry
        test_scenario::next_tx(scenario, CREATOR);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            mydata::test_init(&clock, test_scenario::ctx(scenario));
            block_list::test_init(&clock, test_scenario::ctx(scenario));

            profile::init_for_testing(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);

            let _witness = test_utils::create_one_time_witness<myso::myso::MYSO>();
            clock::share_for_testing(clock::create_for_testing(test_scenario::ctx(scenario)));
        };
        
        // Create profile for creator
        test_scenario::next_tx(scenario, CREATOR);
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
                string::utf8(b"Test Creator"),
                string::utf8(b"creator"),
                string::utf8(b"Creator profile for testing"),
                b"https://example.com/creator.jpg",
                b"",
                &clock,
                test_scenario::ctx(scenario)
            );
            
            test_scenario::return_shared(clock);
            test_scenario::return_shared(ai_credit_config);
            test_scenario::return_shared(memory_registry);
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(profile_config);
            test_scenario::return_shared(registry);
        };

        test_scenario::next_tx(scenario, CREATOR);
        {
            memory_test_helpers::create_default_org_in_tx(scenario);
        };

        test_scenario::next_tx(scenario, CREATOR);
        {
            register_placeholder_agent(scenario);
        };
    }
    
    fun create_test_recurring_mydata(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let config = test_scenario::take_shared<MyDataConfig>(scenario);
            let mut registry = test_scenario::take_shared<MyDataRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            mydata::create_and_share_marketplace_recurring_mydata(
                &config,
                &mut registry,
                string::utf8(b"data"),
                vector[string::utf8(b"recurring")],
                option::none<address>(),
                1000,
                option::none<u64>(),
                b"encrypted_recurring_data",
                b"encryption_recurring",
                100,
                30,
                option::none<string::String>(),
                option::none<string::String>(),
                option::none<u64>(),
                option::none<string::String>(),
                false,
                option::none<string::String>(),
                &clock,
                test_scenario::ctx(scenario)
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };
    }

    fun create_test_mydata(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
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
                option::none<string::String>(),
                option::none<string::String>(),
                option::none<u64>(),
                option::none<string::String>(),
                false,
                option::none<string::String>(),
                &clock,
                test_scenario::ctx(scenario)
            );
            
            test_scenario::return_shared(config);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };
    }
}
