// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Spend-approval enforcement (`require_approval_above_mist`), role-gated approvals,
//! and parent-delegated budget management.

#[test_only]
#[allow(duplicate_alias, unused_use, unused_const)]
module social_contracts::ai_credit_approval_tests {
    use std::option;
    use std::string;

    use myso::test_scenario;
    use myso::object::{Self, ID};
    use myso::clock::{Self, Clock};
    use myso::transfer;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::permissioned_group::{Self, PermissionedGroup};

    use social_contracts::memory::{
        Self,
        MemoryRegistry,
        MemoryAccount,
        SubAgent,
        AgenticOrganization,
        MemorySharePackage,
        MemoryConfig};
    use social_contracts::memory_test_helpers;
    use social_contracts::profile::{Self, Profile, UsernameRegistry,
        ProfileConfig};
    use social_contracts::ai_credit::{Self, AiCreditBalance, AiCreditConfig};

    const ADMIN: address = @0xAD;
    const USER1: address = @0x1;
    const FINANCE_HUMAN: address = @0xF1;
    const AGENT_ADDR: address = @0xA11CE;
    const CHILD_ADDR: address = @0xC41D;
    const AGENT2_ADDR: address = @0xA2;
    const AGENT_PUBKEY: vector<u8> = x"0101010101010101010101010101010101010101010101010101010101010101";
    const CHILD_PUBKEY: vector<u8> = x"0202020202020202020202020202020202020202020202020202020202020202";
    const AGENT2_PUBKEY: vector<u8> = x"0303030303030303030303030303030303030303030303030303030303030303";

    const DEPOSIT_MIST: u64 = 5_000_000_000;
    const THRESHOLD_MIST: u64 = 100_000_000;
    const SPEND_MIST: u64 = 500_000_000;
    const SMALL_SPEND_MIST: u64 = 50_000_000;
    const EXPIRY_MS: u64 = 60_000;

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

        test_scenario::next_tx(scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(scenario);
            let payment = coin::mint_for_testing<MYSO>(DEPOSIT_MIST, test_scenario::ctx(scenario));
            ai_credit::deposit(&config, &mut balance, payment, test_scenario::ctx(scenario));
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
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

    fun take_group(
        scenario: &test_scenario::Scenario,
        org: &AgenticOrganization,
    ): PermissionedGroup<MemorySharePackage> {
        test_scenario::take_shared_by_id<PermissionedGroup<MemorySharePackage>>(
            scenario,
            object::id_from_address(memory::org_memory_group_address(org)),
        )
    }

    /// Org + root agent (`AGENT_ADDR`, holds ai-spend + budget-manage + register caps)
    /// with a budget entry: budget = deposit, approval threshold = THRESHOLD_MIST.
    fun setup_org_agent_with_threshold(scenario: &mut test_scenario::Scenario): (ID, ID) {
        test_scenario::next_tx(scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(scenario);
        };

        let org_id;
        test_scenario::next_tx(scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(scenario);
            let mut org = memory_test_helpers::take_created_org(scenario);
            org_id = object::id(&org);
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
                memory::cap_ai_spend()
                    | memory::cap_agent_register()
                    | memory::cap_budget_manage(),
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
            test_scenario::return_shared(memory_config);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
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
                option::some(THRESHOLD_MIST),
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
            (balance_id, org_id)
        }
    }

    /// Settle `amount` for `derived` with the given nonce; aborts bubble up to the test.
    fun settle(
        scenario: &mut test_scenario::Scenario,
        balance_id: ID,
        derived: address,
        amount: u64,
        nonce: u64,
    ) {
        test_scenario::next_tx(scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let agent = take_agent(scenario, &memory_account, derived);
            let clock = test_scenario::take_shared<Clock>(scenario);

            let receipt = ai_credit::test_make_receipt(
                balance_id,
                memory::agent_object_id(&agent),
                (nonce as u128),
                amount,
                ai_credit::usage_inference(),
                clock::timestamp_ms(&clock),
                nonce,
            );
            ai_credit::settle_usage_for_testing(
                &config,
                &mut balance,
                &memory_account,
                &agent,
                receipt,
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };
    }

    fun owner_approve(
        scenario: &mut test_scenario::Scenario,
        derived: address,
        max_amount: u64,
        expires_at: u64,
    ) {
        test_scenario::next_tx(scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            ai_credit::approve_agent_spend(
                &config,
                &mut balance,
                agent_object_id_from_derived(&memory_account, derived),
                max_amount,
                expires_at,
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };
    }

    #[test]
    #[expected_failure(abort_code = 18, location = social_contracts::ai_credit)]
    fun test_over_threshold_without_approval_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        settle(&mut scenario, balance_id, AGENT_ADDR, SPEND_MIST, 1);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_under_threshold_settles_without_approval() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        settle(&mut scenario, balance_id, AGENT_ADDR, SMALL_SPEND_MIST, 1);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            assert!(ai_credit::balance_mist(&balance) == DEPOSIT_MIST - SMALL_SPEND_MIST, 0);
            test_scenario::return_shared(balance);
        };
        test_scenario::end(scenario);
    }

    #[test]
    fun test_approved_spend_settles_and_consumes_allowance() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        owner_approve(&mut scenario, AGENT_ADDR, SPEND_MIST, EXPIRY_MS);
        settle(&mut scenario, balance_id, AGENT_ADDR, SPEND_MIST, 1);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            assert!(ai_credit::balance_mist(&balance) == DEPOSIT_MIST - SPEND_MIST, 0);
            // One-shot: the allowance is consumed by the settlement.
            let approval = ai_credit::spend_approval_for(
                &balance,
                agent_object_id_from_derived(&memory_account, AGENT_ADDR),
            );
            assert!(option::is_none(&approval), 1);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
        };
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 19, location = social_contracts::ai_credit)]
    fun test_expired_approval_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        owner_approve(&mut scenario, AGENT_ADDR, SPEND_MIST, 1_000);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut clock = test_scenario::take_shared<Clock>(&scenario);
            clock::increment_for_testing(&mut clock, 2_000);
            test_scenario::return_shared(clock);
        };

        settle(&mut scenario, balance_id, AGENT_ADDR, SPEND_MIST, 1);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 20, location = social_contracts::ai_credit)]
    fun test_insufficient_approval_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        owner_approve(&mut scenario, AGENT_ADDR, SPEND_MIST - 1, EXPIRY_MS);
        settle(&mut scenario, balance_id, AGENT_ADDR, SPEND_MIST, 1);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 18, location = social_contracts::ai_credit)]
    fun test_revoked_approval_aborts_settlement() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        owner_approve(&mut scenario, AGENT_ADDR, SPEND_MIST, EXPIRY_MS);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            ai_credit::revoke_agent_spend_approval(
                &config,
                &mut balance,
                agent_object_id_from_derived(&memory_account, AGENT_ADDR),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        settle(&mut scenario, balance_id, AGENT_ADDR, SPEND_MIST, 1);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_reapprove_overwrites_allowance() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        owner_approve(&mut scenario, AGENT_ADDR, THRESHOLD_MIST + 1, EXPIRY_MS);
        owner_approve(&mut scenario, AGENT_ADDR, SPEND_MIST, EXPIRY_MS);
        settle(&mut scenario, balance_id, AGENT_ADDR, SPEND_MIST, 1);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            assert!(ai_credit::balance_mist(&balance) == DEPOSIT_MIST - SPEND_MIST, 0);
            test_scenario::return_shared(balance);
        };
        test_scenario::end(scenario);
    }

    #[test]
    fun test_role_approver_flow() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (balance_id, org_id) = setup_org_agent_with_threshold(&mut scenario);

        // Owner creates the org group and assigns the finance role to a second human.
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::ensure_org_memory_group(
                &memory_account,
                &mut org,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let mut group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::assign_org_role(
                &memory_account,
                &mut org,
                &mut group,
                FINANCE_HUMAN,
                string::utf8(b"finance_approver"),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        // The finance approver (not the owner) grants the allowance.
        test_scenario::next_tx(&mut scenario, FINANCE_HUMAN);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            ai_credit::approve_agent_spend_as_approver(
                &config,
                &mut balance,
                &memory_account,
                &org,
                &group,
                &agent,
                SPEND_MIST,
                EXPIRY_MS,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        settle(&mut scenario, balance_id, AGENT_ADDR, SPEND_MIST, 1);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 100, location = social_contracts::memory)]
    fun test_non_approver_role_gate_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (_balance_id, org_id) = setup_org_agent_with_threshold(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let mut org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::ensure_org_memory_group(
                &memory_account,
                &mut org,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(org);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        };

        // FINANCE_HUMAN was never assigned the role — the org gate must abort.
        test_scenario::next_tx(&mut scenario, FINANCE_HUMAN);
        {
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let org = test_scenario::take_shared_by_id<AgenticOrganization>(&scenario, org_id);
            let group = take_group(&scenario, &org);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let agent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            ai_credit::approve_agent_spend_as_approver(
                &config,
                &mut balance,
                &memory_account,
                &org,
                &group,
                &agent,
                SPEND_MIST,
                EXPIRY_MS,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(agent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(group);
            test_scenario::return_shared(org);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    // ==== Parent-delegated budgets ====

    /// Registers a child of `AGENT_ADDR` (which holds CAP_BUDGET_MANAGE) with ai-spend.
    fun register_child_of_agent(scenario: &mut test_scenario::Scenario
    ) {
        test_scenario::next_tx(scenario, AGENT_ADDR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(scenario);
            let parent = take_agent(scenario, &memory_account, AGENT_ADDR);
            let clock = test_scenario::take_shared<Clock>(scenario);
            memory::register_sub_agent_delegated(
                &memory_config,
                &mut memory_account,
                &parent,
                CHILD_PUBKEY,
                CHILD_ADDR,
                string::utf8(b"child"),
                memory::class_delegated_ai(),
                0,
                memory::cap_ai_spend(),
                0,
                1,
                0,
                option::none(),
                option::none(),
                option::none(),
                memory::register_child(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(parent);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };
    }

    #[test]
    fun test_parent_sets_child_budget_and_approves_within_envelope() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        register_child_of_agent(&mut scenario);

        // Parent sets the child's budget (all limits at least as strict as its own).
        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let child = take_agent(&scenario, &memory_account, CHILD_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            ai_credit::set_child_agent_budget(
                &config,
                &memory_config,
                
                                &mut balance,
                &memory_account,
                &parent,
                &child,
                option::some(DEPOSIT_MIST / 2),
                option::none(),
                option::none(),
                option::some(THRESHOLD_MIST / 2),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            // Parent approves a child spend within its own threshold envelope.
            ai_credit::approve_child_agent_spend(
                &config,
                &memory_config,
                
                                &mut balance,
                &memory_account,
                &parent,
                &child,
                THRESHOLD_MIST,
                EXPIRY_MS,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(parent);
            test_scenario::return_shared(child);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        // Child settles an over-its-threshold spend covered by the parent's allowance.
        settle(&mut scenario, balance_id, CHILD_ADDR, THRESHOLD_MIST, 1);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            assert!(ai_credit::balance_mist(&balance) == DEPOSIT_MIST - THRESHOLD_MIST, 0);
            test_scenario::return_shared(balance);
        };
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 24, location = social_contracts::ai_credit)]
    fun test_parent_approval_beyond_envelope_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (_balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        register_child_of_agent(&mut scenario);

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let child = take_agent(&scenario, &memory_account, CHILD_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            // Beyond the parent's own approval threshold — escalates to the owner instead.
            ai_credit::approve_child_agent_spend(
                &config,
                &memory_config,
                
                                &mut balance,
                &memory_account,
                &parent,
                &child,
                THRESHOLD_MIST + 1,
                EXPIRY_MS,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(parent);
            test_scenario::return_shared(child);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 24, location = social_contracts::ai_credit)]
    fun test_child_budget_looser_than_parent_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (_balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        register_child_of_agent(&mut scenario);

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let child = take_agent(&scenario, &memory_account, CHILD_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            // Parent has a lifetime budget, so the child cannot be unlimited.
            ai_credit::set_child_agent_budget(
                &config,
                &memory_config,
                
                                &mut balance,
                &memory_account,
                &parent,
                &child,
                option::none(),
                option::none(),
                option::none(),
                option::some(THRESHOLD_MIST),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(parent);
            test_scenario::return_shared(child);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 23, location = social_contracts::ai_credit)]
    fun test_non_parent_signer_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (_balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        register_child_of_agent(&mut scenario);

        // Owner signing a parent-delegated entry is rejected: the parent must sign.
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let child = take_agent(&scenario, &memory_account, CHILD_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            ai_credit::set_child_agent_budget(
                &config,
                &memory_config,
                
                                &mut balance,
                &memory_account,
                &parent,
                &child,
                option::some(DEPOSIT_MIST / 2),
                option::none(),
                option::none(),
                option::some(THRESHOLD_MIST / 2),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(parent);
            test_scenario::return_shared(child);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 22, location = social_contracts::ai_credit)]
    fun test_not_descendant_aborts() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (_balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);

        // Second org with its own root agent — unrelated to AGENT_ADDR's subtree.
        test_scenario::next_tx(&mut scenario, USER1);
        {
            memory_test_helpers::create_default_org_in_tx(&mut scenario);
        };
        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let mut org2 = memory_test_helpers::take_created_org(&mut scenario);
            let mut memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            memory::register_sub_agent(
                &memory_config,
                &mut memory_account,
                &mut org2,
                AGENT2_PUBKEY,
                AGENT2_ADDR,
                string::utf8(b"other-root"),
                memory::class_delegated_ai(),
                0,
                memory::cap_ai_spend(),
                0,
                3,
                0,
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(org2);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::next_tx(&mut scenario, AGENT_ADDR);
        {
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let config = test_scenario::take_shared<AiCreditConfig>(&scenario);
            let mut balance = test_scenario::take_shared<AiCreditBalance>(&scenario);
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let parent = take_agent(&scenario, &memory_account, AGENT_ADDR);
            let other = take_agent(&scenario, &memory_account, AGENT2_ADDR);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            ai_credit::set_child_agent_budget(
                &config,
                &memory_config,
                
                                &mut balance,
                &memory_account,
                &parent,
                &other,
                option::some(DEPOSIT_MIST / 2),
                option::none(),
                option::none(),
                option::some(THRESHOLD_MIST / 2),
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_shared(parent);
            test_scenario::return_shared(other);
            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(balance);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        test_scenario::return_shared(memory_config);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_is_descendant_agent_helper() {
        let mut scenario = test_scenario::begin(ADMIN);
        init_env(&mut scenario);
        let (_balance_id, _org_id) = setup_org_agent_with_threshold(&mut scenario);
        register_child_of_agent(&mut scenario);

        test_scenario::next_tx(&mut scenario, USER1);
        {
            let memory_account = test_scenario::take_shared<MemoryAccount>(&scenario);
            let memory_config = test_scenario::take_shared<MemoryConfig>(&scenario);
            let parent_id = agent_object_id_from_derived(&memory_account, AGENT_ADDR);
            let child_id = agent_object_id_from_derived(&memory_account, CHILD_ADDR);

            assert!(memory::is_descendant_agent(&memory_config, &memory_account, parent_id, child_id), 0);
            assert!(!memory::is_descendant_agent(&memory_config, &memory_account, child_id, parent_id), 1);
            // Self is not a descendant.
            assert!(!memory::is_descendant_agent(&memory_config, &memory_account, parent_id, parent_id), 2);

            test_scenario::return_shared(memory_account);
            test_scenario::return_shared(memory_config);
        };
        test_scenario::end(scenario);
    }
}
