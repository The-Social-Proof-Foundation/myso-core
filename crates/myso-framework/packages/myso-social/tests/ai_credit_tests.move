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
    use social_contracts::upgrade::{Self, UpgradeAdminCap};

    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const AGENT_ADDR: address = @0xA11CE;
    const AGENT_PUBKEY: vector<u8> = x"0101010101010101010101010101010101010101010101010101010101010101";
    const ORACLE_PK: vector<u8> = x"cc62332e34bb2d5cd69f60efbb2a36cb916c7eb458301ea36636c4dbb012bd88";
    const ENVELOPE_HASH: vector<u8> = x"1111111111111111111111111111111111111111111111111111111111111111";
    const REQUEST_HASH: vector<u8> = x"2222222222222222222222222222222222222222222222222222222222222222";
    const GENERATION_HASH: vector<u8> = x"3333333333333333333333333333333333333333333333333333333333333333";
    const FX_QUOTE_ID: vector<u8> = b"test-fx-quote";
    const DEPOSIT_MIST: u64 = 5_000_000_000;
    const SPEND_MIST: u64 = 500_000_000;
    const CAPTURE_MIST: u64 = 200_000_000;

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

    fun reserve_default(
        scenario: &mut test_scenario::Scenario,
        reservation_nonce: u64,
        max_amount_mist: u64,
    ) {
        test_scenario::next_tx(scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let agent = take_agent(scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(scenario);
            let now = clock::timestamp_ms(&clock);

            ai_credit::reserve_spend_for_testing(
                &config,
                &mut balance,
                &memory_account,
                &agent,
                reservation_nonce,
                max_amount_mist,
                ENVELOPE_HASH,
                REQUEST_HASH,
                FX_QUOTE_ID,
                450_000,
                now,
                now + 60_000,
                now + 120_000,
                &clock,
            );

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };
    }

    fun invalid_signature(): vector<u8> {
        let mut signature = vector::empty<u8>();
        let mut i = 0u64;
        while (i < 64) {
            vector::push_back(&mut signature, 0);
            i = i + 1;
        };
        signature
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
            assert!(ai_credit::available_mist(&balance) == DEPOSIT_MIST - SPEND_MIST, 2);

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
    fun test_reservation_locks_balance_and_agent_budget() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent_id = agent_object_id_from_derived(&memory_account, AGENT_ADDR);
            assert!(ai_credit::balance_mist(&balance) == DEPOSIT_MIST, 0);
            assert!(ai_credit::reserved_mist(&balance) == SPEND_MIST, 1);
            assert!(ai_credit::available_mist(&balance) == DEPOSIT_MIST - SPEND_MIST, 2);
            assert!(ai_credit::latest_reservation_nonce(&balance) == 1, 3);
            let mut reservation = ai_credit::reservation_for(&balance, 1);
            assert!(option::is_some(&reservation), 4);
            let reservation = option::extract(&mut reservation);
            assert!(ai_credit::reservation_max_amount_mist(&reservation) == SPEND_MIST, 5);
            assert!(ai_credit::reservation_agent_object_id(&reservation) == agent_id, 6);
            let mut remaining = ai_credit::agent_remaining_mist(&balance, agent_id);
            assert!(option::extract(&mut remaining) == DEPOSIT_MIST - SPEND_MIST, 7);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_capture_charges_actual_and_releases_remainder() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            ai_credit::capture_spend_for_testing(
                &config,
                &mut balance,
                1,
                CAPTURE_MIST,
                900,
                GENERATION_HASH,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            assert!(ai_credit::balance_mist(&balance) == DEPOSIT_MIST - CAPTURE_MIST, 0);
            assert!(ai_credit::reserved_mist(&balance) == 0, 1);
            assert!(ai_credit::available_mist(&balance) == DEPOSIT_MIST - CAPTURE_MIST, 2);
            assert!(option::is_none(&ai_credit::reservation_for(&balance, 1)), 3);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_oracle_cancel_releases_full_reservation() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            ai_credit::cancel_spend_for_testing(&config, &mut balance, 1, &clock);
            assert!(ai_credit::balance_mist(&balance) == DEPOSIT_MIST, 0);
            assert!(ai_credit::reserved_mist(&balance) == 0, 1);
            assert!(ai_credit::available_mist(&balance) == DEPOSIT_MIST, 2);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_permissionless_expiry_releases_reservation() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, @0x999);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            clock::increment_for_testing(&mut clock, 120_000);
            ai_credit::expire_reservation(&config, &mut balance, 1, &clock);
            assert!(ai_credit::reserved_mist(&balance) == 0, 0);
            assert!(ai_credit::available_mist(&balance) == DEPOSIT_MIST, 1);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 4, location = social_contracts::ai_credit)]
    fun test_withdraw_cannot_consume_reserved_funds() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, 4_500_000_000);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            ai_credit::withdraw(
                &config,
                &mut balance,
                1_000_000_000,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 30, location = social_contracts::ai_credit)]
    fun test_reservation_cannot_expire_early() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, @0x999);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            ai_credit::expire_reservation(&config, &mut balance, 1, &clock);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 8, location = social_contracts::ai_credit)]
    fun test_reservation_nonce_cannot_replay_after_cancel() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            ai_credit::cancel_spend_for_testing(&config, &mut balance, 1, &clock);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        reserve_default(&mut scenario, 1, SPEND_MIST);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 9, location = social_contracts::ai_credit)]
    fun test_account_cap_counts_outstanding_reservation() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            ai_credit::set_account_caps(
                &config,
                &mut balance,
                option::some(SPEND_MIST),
                option::some(SPEND_MIST),
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
        };

        reserve_default(&mut scenario, 1, SPEND_MIST);
        reserve_default(&mut scenario, 2, 1);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 9, location = social_contracts::ai_credit)]
    fun test_agent_budget_counts_outstanding_reservation() {
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
            ai_credit::set_agent_budget(
                &config,
                &mut balance,
                &agent,
                option::some(SPEND_MIST),
                option::some(SPEND_MIST),
                option::some(SPEND_MIST),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        reserve_default(&mut scenario, 1, SPEND_MIST);
        reserve_default(&mut scenario, 2, 1);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 9, location = social_contracts::ai_credit)]
    fun test_capture_cannot_exceed_reservation() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            ai_credit::capture_spend_for_testing(
                &config,
                &mut balance,
                1,
                SPEND_MIST + 1,
                900,
                GENERATION_HASH,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 31, location = social_contracts::ai_credit)]
    fun test_signed_cancel_closes_at_capture_deadline() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            clock::increment_for_testing(&mut clock, 60_001);
            ai_credit::cancel_spend_for_testing(&config, &mut balance, 1, &clock);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
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
    fun test_reserve_signed_spend_rejects_bad_signature() {
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
            let now = clock::timestamp_ms(&clock);
            ai_credit::reserve_signed_spend(
                &config,
                &mut balance,
                &memory_account,
                &agent,
                1,
                SPEND_MIST,
                ENVELOPE_HASH,
                REQUEST_HASH,
                FX_QUOTE_ID,
                450_000,
                ai_credit::oracle_markup_bps(&config),
                now,
                now + 60_000,
                now + 120_000,
                invalid_signature(),
                &clock,
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
    #[expected_failure(abort_code = 6, location = social_contracts::ai_credit)]
    fun test_capture_reserved_spend_rejects_bad_signature() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            ai_credit::capture_reserved_spend(
                &config,
                &mut balance,
                1,
                CAPTURE_MIST,
                900,
                GENERATION_HASH,
                clock::timestamp_ms(&clock),
                invalid_signature(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 29, location = social_contracts::ai_credit)]
    fun test_capture_rejected_after_hard_expiry() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            clock::increment_for_testing(&mut clock, 120_000);
            ai_credit::capture_spend_for_testing(
                &config,
                &mut balance,
                1,
                CAPTURE_MIST,
                900,
                GENERATION_HASH,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 28, location = social_contracts::ai_credit)]
    fun test_capture_cannot_replay() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        setup_balance_and_agent(&mut scenario);
        reserve_default(&mut scenario, 1, SPEND_MIST);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            ai_credit::capture_spend_for_testing(
                &config,
                &mut balance,
                1,
                CAPTURE_MIST,
                900,
                GENERATION_HASH,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            ai_credit::capture_spend_for_testing(
                &config,
                &mut balance,
                1,
                CAPTURE_MIST,
                900,
                GENERATION_HASH,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
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

    // === Upgrade / migration tests ===

    fun init_env_with_upgrade_cap(scenario: &mut test_scenario::Scenario) {
        init_env(scenario);
        test_scenario::next_tx(scenario, ADMIN);
        {
            upgrade::init_for_testing(test_scenario::ctx(scenario));
        };
    }

    #[test]
    fun test_ai_credit_genesis_versions() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            assert!(ai_credit::config_version(&config) == upgrade::current_version(), 1);
            assert!(ai_credit::balance_version(&balance) == upgrade::current_version(), 2);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 2, location = social_contracts::ai_credit)]
    fun test_ai_credit_wrong_version_aborts_deposit() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            ai_credit::test_force_balance_version(
                &mut balance,
                upgrade::current_version() + 1,
            );
            let payment = coin::mint_for_testing<MYSO>(DEPOSIT_MIST, test_scenario::ctx(&mut scenario));
            ai_credit::deposit(&config, &mut balance, payment, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_ai_credit_migrate_config_and_balance_then_ops_work() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env_with_upgrade_cap(&mut scenario);

        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<UpgradeAdminCap>(&scenario);
            let mut config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let stale = upgrade::test_stale_version_for_migration();
            ai_credit::test_force_config_version(&mut config, stale);
            ai_credit::test_force_balance_version(&mut balance, stale);

            if (upgrade::test_migration_available()) {
                ai_credit::migrate_config(&mut config, &admin_cap, test_scenario::ctx(&mut scenario));
                ai_credit::migrate_balance(&mut balance, &admin_cap, test_scenario::ctx(&mut scenario));
            } else {
                ai_credit::test_migrate_config(&mut config, &admin_cap, test_scenario::ctx(&mut scenario));
                ai_credit::test_migrate_balance(&mut balance, &admin_cap, test_scenario::ctx(&mut scenario));
            };

            assert!(ai_credit::config_version(&config) == upgrade::current_version(), 1);
            assert!(ai_credit::balance_version(&balance) == upgrade::current_version(), 2);

            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_to_sender(&scenario, admin_cap);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let payment = coin::mint_for_testing<MYSO>(DEPOSIT_MIST, test_scenario::ctx(&mut scenario));
            ai_credit::deposit(&config, &mut balance, payment, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 2, location = social_contracts::ai_credit)]
    fun test_ai_credit_migrate_config_idempotent() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env_with_upgrade_cap(&mut scenario);

        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<UpgradeAdminCap>(&scenario);
            let mut config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let stale = upgrade::test_stale_version_for_migration();
            ai_credit::test_force_config_version(&mut config, stale);
            if (upgrade::test_migration_available()) {
                ai_credit::migrate_config(&mut config, &admin_cap, test_scenario::ctx(&mut scenario));
            } else {
                ai_credit::test_migrate_config(&mut config, &admin_cap, test_scenario::ctx(&mut scenario));
            };
            ai_credit::migrate_config(&mut config, &admin_cap, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(config);
            test_scenario::return_to_sender(&scenario, admin_cap);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 2, location = social_contracts::ai_credit)]
    fun test_ai_credit_migrate_balance_idempotent() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env_with_upgrade_cap(&mut scenario);

        test_scenario::next_tx(&mut scenario, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<UpgradeAdminCap>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let stale = upgrade::test_stale_version_for_migration();
            ai_credit::test_force_balance_version(&mut balance, stale);
            if (upgrade::test_migration_available()) {
                ai_credit::migrate_balance(&mut balance, &admin_cap, test_scenario::ctx(&mut scenario));
            } else {
                ai_credit::test_migrate_balance(&mut balance, &admin_cap, test_scenario::ctx(&mut scenario));
            };
            ai_credit::migrate_balance(&mut balance, &admin_cap, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(balance);
            test_scenario::return_to_sender(&scenario, admin_cap);
        };

        test_scenario::end(scenario);
    }
}
