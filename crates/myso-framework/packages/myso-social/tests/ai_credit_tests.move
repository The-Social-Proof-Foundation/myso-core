// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(duplicate_alias, unused_use, unused_const)]
module social_contracts::ai_credit_tests {
    use std::option;
    use std::string;
    use std::vector;

    use myso::test_scenario;
    use myso::object::{Self, ID};
    use myso::clock::{Self, Clock};
    use myso::transfer;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;

    use social_contracts::memory::{Self, MemoryAccount, MemoryRegistry, SubAgent,
        MemoryConfig};
    use social_contracts::memory_test_helpers;
    use social_contracts::profile::{Self, Profile, UsernameRegistry,
        ProfileConfig};
    use social_contracts::ai_credit::{Self, AiCreditBalance, AiCreditConfig};

    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const AGENT_ADDR: address = @0xA11CE;
    const AGENT_PUBKEY: vector<u8> = x"0101010101010101010101010101010101010101010101010101010101010101";
    const ORACLE_PK: vector<u8> = x"cc62332e34bb2d5cd69f60efbb2a36cb916c7eb458301ea36636c4dbb012bd88";
    const DEPOSIT_MIST: u64 = 5_000_000_000;
    const SPEND_MIST: u64 = 500_000_000;

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

    fun agent_object_id_from_derived(memory_account: &MemoryAccount, derived: address): ID {
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

    fun setup_balance_and_agent(scenario: &mut test_scenario::Scenario): ID {
        test_scenario::next_tx(scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(scenario);
            let payment = coin::mint_for_testing<MYSO>(DEPOSIT_MIST, test_scenario::ctx(scenario));
            ai_credit::deposit(&config, &mut balance, payment, test_scenario::ctx(scenario));
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
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
                AGENT_PUBKEY,
                AGENT_ADDR,
                string::utf8(b"ai-agent"),
                memory::class_delegated_ai(),
                0,
                memory::cap_ai_spend(),
                memory::cap_ai_spend(),
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

        test_scenario::next_tx(scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let agent = take_agent(scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(scenario);

            ai_credit::set_agent_budget(
                &config,
                &mut balance,
                &agent,
                option::some(DEPOSIT_MIST),
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(scenario, USER1);
        {
            let balance = test_scenario::take_shared<AiCreditBalance>(scenario);
            let balance_id = object::id(&balance);
            test_scenario::return_shared(balance);
            balance_id
        }
    }

    #[test]
    fun test_deposit_and_settle_usage() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let balance_id = setup_balance_and_agent(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            let receipt = ai_credit::test_make_receipt(
                balance_id,
                memory::agent_object_id(&agent),
                1,
                SPEND_MIST,
                ai_credit::usage_inference(),
                clock::timestamp_ms(&clock),
                1,
            );

            ai_credit::settle_usage_for_testing(
                &config,
                &mut balance,
                &memory_account,
                &agent,
                receipt,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            assert!(ai_credit::balance_mist(&balance) == DEPOSIT_MIST - SPEND_MIST, 1);
            assert!(ai_credit::credits_from_mist(ai_credit::balance_mist(&balance)) == 4, 2);

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 8, location = social_contracts::ai_credit)]
    fun test_invalid_nonce_rejected() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let balance_id = setup_balance_and_agent(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            let receipt = ai_credit::test_make_receipt(
                balance_id,
                memory::agent_object_id(&agent),
                99,
                SPEND_MIST,
                ai_credit::usage_inference(),
                clock::timestamp_ms(&clock),
                2,
            );

            ai_credit::settle_usage_for_testing(
                &config,
                &mut balance,
                &memory_account,
                &agent,
                receipt,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
  #[expected_failure(abort_code = 4, location = social_contracts::ai_credit)]
    fun test_insufficient_balance_rejected() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let balance_id = setup_balance_and_agent(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            ai_credit::set_agent_budget(
                &config,
                &mut balance,
                &agent,
                option::some(DEPOSIT_MIST + 2_000_000_000),
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            let receipt = ai_credit::test_make_receipt(
                balance_id,
                memory::agent_object_id(&agent),
                1,
                DEPOSIT_MIST + 1,
                ai_credit::usage_inference(),
                clock::timestamp_ms(&clock),
                1,
            );

            ai_credit::settle_usage_for_testing(
                &config,
                &mut balance,
                &memory_account,
                &agent,
                receipt,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_withdraw_unused_balance() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            ai_credit::withdraw(&config, &mut balance, 1_000_000_000, test_scenario::ctx(&mut scenario));
            assert!(ai_credit::balance_mist(&balance) == DEPOSIT_MIST - 1_000_000_000, 1);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_make_usage_receipt_from_objects_ids() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let balance_id = setup_balance_and_agent(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            let receipt = ai_credit::make_usage_receipt_from_objects(
                &balance,
                &agent,
                42,
                SPEND_MIST,
                ai_credit::usage_inference(),
                clock::timestamp_ms(&clock),
                1,
            );

            let expected = ai_credit::test_make_receipt(
                balance_id,
                memory::agent_object_id(&agent),
                42,
                SPEND_MIST,
                ai_credit::usage_inference(),
                clock::timestamp_ms(&clock),
                1,
            );

            assert!(receipt == expected, 1);

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 6, location = social_contracts::ai_credit)]
    fun test_settle_signed_usage_rejects_bad_signature() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            let mut bad_sig = vector::empty<u8>();
            let mut i = 0u64;
            while (i < 64) {
                vector::push_back(&mut bad_sig, 0);
                i = i + 1;
            };

            ai_credit::settle_signed_usage(
                &config,
                &mut balance,
                &memory_account,
                &agent,
                1,
                SPEND_MIST,
                ai_credit::usage_inference(),
                clock::timestamp_ms(&clock),
                1,
                bad_sig,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }
}
