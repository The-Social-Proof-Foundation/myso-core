// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Task market and bidirectional reputation tests. The fixture (a platform, a task market, a buyer
/// principal with one agent, and a seller principal with two agents in separate organizations) and
/// the action helpers live in this module so no separate helper file is needed.
#[test_only]
#[allow(duplicate_alias, unused_use, unused_const, unused_variable)]
module social_contracts::agent_task_tests {
    use std::option::{Self, Option};
    use std::string;

    use myso::clock::{Self, Clock};
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::object::{Self, ID};
    use myso::test_scenario::{Self as ts, Scenario};

    use social_contracts::agent_task::{Self, TaskConfig, TaskEscrow, TaskMarketCap};
    use social_contracts::ai_credit::AiCreditConfig;
    use social_contracts::jury::{Self, JuryConfig};
    use social_contracts::memory::{Self, AgenticOrganization, MemoryAccount, MemoryConfig, SubAgent};
    use social_contracts::memory_test_helpers;
    use social_contracts::platform::{Self, Platform, PlatformConfig, PlatformRegistry};
    use social_contracts::profile::{Self, ProfileConfig, UsernameRegistry};

    // ============================================================
    // Fixture and actions
    // ============================================================

    public fun admin(): address { @0xAD }
    public fun buyer(): address { @0xB1 }
    public fun seller(): address { @0x5E }
    public fun buyer_agent_addr(): address { @0xB1A }
    public fun seller_agent_addr(): address { @0x5EA }
    public fun seller_agent2_addr(): address { @0x5EB }

    const BUYER_AGENT_KEY: vector<u8> = x"b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1b1";
    const SELLER_AGENT_KEY: vector<u8> = x"5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e5e";
    const SELLER_AGENT2_KEY: vector<u8> = x"5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f5f";

    public fun one_myso(): u64 { 1_000_000_000 }
    public fun hour(): u64 { 3_600_000 }
    public fun day(): u64 { 86_400_000 }
    public fun scope(): vector<u8> { x"1111111111111111111111111111111111111111111111111111111111111111" }
    public fun deliverable(): vector<u8> { x"2222222222222222222222222222222222222222222222222222222222222222" }

    /// Object ids of everything the tests touch.
    public struct Fixture has copy, drop {
        buyer_account: ID,
        seller_account: ID,
        buyer_agent: ID,
        seller_agent: ID,
        seller_agent2: ID,
        platform: address,
    }

    public fun buyer_account(fx: &Fixture): ID { fx.buyer_account }
    public fun seller_account(fx: &Fixture): ID { fx.seller_account }
    public fun buyer_agent(fx: &Fixture): ID { fx.buyer_agent }
    public fun seller_agent(fx: &Fixture): ID { fx.seller_agent }
    public fun seller_agent2(fx: &Fixture): ID { fx.seller_agent2 }
    public fun platform_address(fx: &Fixture): address { fx.platform }

    // ============================================================
    // Setup
    // ============================================================

    public fun setup(sc: &mut Scenario): Fixture {
        setup_with_caps(sc, memory::cap_task_trade(), option::none())
    }

    /// `buyer_caps` / `buyer_spend_cap` shape the buyer agent for authorization tests.
    public fun setup_with_caps(
        sc: &mut Scenario,
        buyer_caps: u64,
        buyer_spend_cap: Option<u64>,
    ): Fixture {
        ts::next_tx(sc, admin());
        {
            let clock = clock::create_for_testing(ts::ctx(sc));
            profile::init_for_testing(&clock, ts::ctx(sc));
            platform::test_init(&clock, ts::ctx(sc));
            agent_task::test_bootstrap(admin(), &clock, ts::ctx(sc));
            clock::share_for_testing(clock);
        };
        ts::next_tx(sc, admin());
        {
            let cap = agent_task::test_create_task_market_cap(ts::ctx(sc));
            myso::transfer::public_transfer(cap, admin());
        };

        let platform_addr = create_platform(sc);
        create_profile(sc, buyer(), b"Buyer", b"buyerone");
        create_profile(sc, seller(), b"Seller", b"sellerone");

        let buyer_account = account_id_of(sc, buyer());
        let seller_account = account_id_of(sc, seller());
        let buyer_agent = register_agent(
            sc, buyer(), buyer_account, buyer_agent_addr(), BUYER_AGENT_KEY, buyer_caps, buyer_spend_cap,
        );
        let seller_agent = register_agent(
            sc, seller(), seller_account, seller_agent_addr(), SELLER_AGENT_KEY,
            memory::cap_task_trade(), option::none(),
        );
        let seller_agent2 = register_agent(
            sc, seller(), seller_account, seller_agent2_addr(), SELLER_AGENT2_KEY,
            memory::cap_task_trade(), option::none(),
        );
        Fixture {
            buyer_account,
            seller_account,
            buyer_agent,
            seller_agent,
            seller_agent2,
            platform: platform_addr,
        }
    }

    fun create_platform(sc: &mut Scenario): address {
        ts::next_tx(sc, admin());
        {
            let mut registry = ts::take_shared<PlatformRegistry>(sc);
            let clock = ts::take_shared<Clock>(sc);
            let platform_config = ts::take_shared<PlatformConfig>(sc);
            platform::create_platform(
                &mut registry,
                &platform_config,
                string::utf8(b"Task Platform"),
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
                ts::ctx(sc),
            );
            ts::return_shared(clock);
            ts::return_shared(platform_config);
            ts::return_shared(registry);
        };
        ts::next_tx(sc, admin());
        let platform_obj = ts::take_shared<Platform>(sc);
        let platform_addr = object::uid_to_address(platform::id(&platform_obj));
        let mut registry = ts::take_shared<PlatformRegistry>(sc);
        platform::test_set_approval(&mut registry, platform_addr, true);
        ts::return_shared(platform_obj);
        ts::return_shared(registry);
        platform_addr
    }

    fun create_profile(sc: &mut Scenario, owner: address, display: vector<u8>, username: vector<u8>) {
        ts::next_tx(sc, owner);
        let mut registry = ts::take_shared<UsernameRegistry>(sc);
        let mut memory_registry = ts::take_shared<memory::MemoryRegistry>(sc);
        let mut ai_credit_config = ts::take_shared<AiCreditConfig>(sc);
        let clock = ts::take_shared<Clock>(sc);
        let profile_config = ts::take_shared<ProfileConfig>(sc);
        profile::create_profile(
            &mut registry,
            &profile_config,
            &mut memory_registry,
            &mut ai_credit_config,
            string::utf8(display),
            string::utf8(username),
            string::utf8(b"bio"),
            b"",
            b"",
            &clock,
            ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(profile_config);
        ts::return_shared(ai_credit_config);
        ts::return_shared(memory_registry);
        ts::return_shared(registry);
    }

    fun account_id_of(sc: &mut Scenario, owner: address): ID {
        ts::next_tx(sc, owner);
        let account = memory_test_helpers::take_owner_memory_account(sc);
        let id = object::id(&account);
        ts::return_shared(account);
        id
    }

    /// Creates an org and registers `derived` as its root agent. Returns the agent object id.
    fun register_agent(
        sc: &mut Scenario,
        owner: address,
        account_id: ID,
        derived: address,
        key: vector<u8>,
        caps: u64,
        spend_cap: Option<u64>,
    ): ID {
        ts::next_tx(sc, owner);
        memory_test_helpers::create_default_org_in_tx(sc);

        ts::next_tx(sc, owner);
        let org_id = ts::most_recent_id_shared<AgenticOrganization>().destroy_some();
        let mut org = ts::take_shared_by_id<AgenticOrganization>(sc, org_id);
        let mut account = ts::take_shared_by_id<MemoryAccount>(sc, account_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let clock = ts::take_shared<Clock>(sc);
        memory::register_sub_agent(
            &memory_config,
            &mut account,
            &mut org,
            key,
            derived,
            string::utf8(b"task-agent"),
            memory::class_delegated_ai(),
            0,
            caps,
            caps,
            3,
            0,
            spend_cap,
            option::none(),
            option::none(),
            &clock,
            ts::ctx(sc),
        );
        let agent_id = object::id_from_address(memory::derive_sub_agent_address(&account, derived));
        ts::return_shared(clock);
        ts::return_shared(memory_config);
        ts::return_shared(account);
        ts::return_shared(org);
        agent_id
    }

    // ============================================================
    // Actions. Each starts its own transaction as `sender`.
    // ============================================================

    public fun advance(sc: &mut Scenario, ms: u64) {
        ts::next_tx(sc, admin());
        let mut clock = ts::take_shared<Clock>(sc);
        clock::increment_for_testing(&mut clock, ms);
        ts::return_shared(clock);
    }

    /// Default offer: buyer principal or buyer agent to the first seller agent.
    public fun offer(sc: &mut Scenario, fx: &Fixture, sender: address, amount: u64) {
        offer_custom(
            sc, fx, sender, fx.buyer_account, fx.seller_account, fx.seller_agent,
            amount, scope(), day(), day(), day(), option::none(),
        );
    }

    public fun offer_with_platform(sc: &mut Scenario, fx: &Fixture, sender: address, amount: u64) {
        offer_custom(
            sc, fx, sender, fx.buyer_account, fx.seller_account, fx.seller_agent,
            amount, scope(), day(), day(), day(), option::some(fx.platform),
        );
    }

    public fun offer_custom(
        sc: &mut Scenario,
        fx: &Fixture,
        sender: address,
        buyer_account_id: ID,
        seller_account_id: ID,
        seller_agent_id: ID,
        amount: u64,
        scope_digest: vector<u8>,
        accept_ms: u64,
        delivery_ms: u64,
        settle_ms: u64,
        platform_id: Option<address>,
    ) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        // A principal offering to its own agent names one account twice, and a scenario can only
        // hand an object out once, so that case reuses the single borrow.
        let same_account = buyer_account_id == seller_account_id;
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, buyer_account_id);
        let other_seller_account = if (same_account) {
            option::none()
        } else {
            option::some(ts::take_shared_by_id<MemoryAccount>(sc, seller_account_id))
        };
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, seller_agent_id);
        let clock = ts::take_shared<Clock>(sc);
        let mut payment = coin::mint_for_testing<MYSO>(amount + one_myso(), ts::ctx(sc));
        agent_task::create_task(
            &config,
            &memory_config,
            &buyer_account,
            if (same_account) { &buyer_account } else { other_seller_account.borrow() },
            &seller_agent,
            &mut payment,
            amount,
            scope_digest,
            string::utf8(b"write the report"),
            accept_ms,
            delivery_ms,
            settle_ms,
            option::none(),
            platform_id,
            &clock,
            ts::ctx(sc),
        );
        coin::burn_for_testing(payment);
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        if (other_seller_account.is_some()) {
            ts::return_shared(other_seller_account.destroy_some());
        } else {
            other_seller_account.destroy_none();
        };
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(config);
        // The escrow is shared in this transaction, and `latest_task` can only see objects from
        // completed ones.
        ts::next_tx(sc, sender);
    }

    public fun latest_task(): ID {
        ts::most_recent_id_shared<TaskEscrow>().destroy_some()
    }

    public fun terms_digest(sc: &mut Scenario, task_id: ID): vector<u8> {
        ts::next_tx(sc, admin());
        let task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let digest = agent_task::terms_digest(&task);
        ts::return_shared(task);
        digest
    }

    public fun accept(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID, digest: vector<u8>) {
        accept_as(sc, fx, sender, task_id, fx.seller_agent, digest);
    }

    public fun accept_as(
        sc: &mut Scenario,
        fx: &Fixture,
        sender: address,
        task_id: ID,
        agent_id: ID,
        digest: vector<u8>,
    ) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let seller_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.seller_account);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, agent_id);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::accept_task(
            &config, &mut task, &memory_config, &seller_account, &seller_agent, digest, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(seller_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    /// Accepts with the offer's own digest.
    public fun accept_ok(sc: &mut Scenario, fx: &Fixture, task_id: ID) {
        let digest = terms_digest(sc, task_id);
        accept(sc, fx, seller_agent_addr(), task_id, digest);
    }

    public fun decline(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let seller_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.seller_account);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::decline_task(&mut task, &memory_config, &seller_account, &seller_agent, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(seller_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
    }

    public fun cancel_offer(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::cancel_offer(&mut task, &memory_config, &buyer_account, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
    }

    public fun expire_offer(sc: &mut Scenario, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::expire_offer(&mut task, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(task);
    }

    public fun deliver(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        deliver_with(sc, fx, sender, task_id, deliverable(), b"walrus://deliverable");
    }

    public fun deliver_with(
        sc: &mut Scenario,
        fx: &Fixture,
        sender: address,
        task_id: ID,
        digest: vector<u8>,
        uri: vector<u8>,
    ) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::mark_delivered(
            &config, &mut task, &seller_agent, digest, string::utf8(uri), &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    public fun settle(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let mut buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let mut seller_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.seller_account);
        let mut platform_obj = ts::take_shared_by_id<Platform>(sc, object::id_from_address(fx.platform));
        let clock = ts::take_shared<Clock>(sc);
        agent_task::settle_task(
            &config, &mut task, &memory_config, &mut buyer_account, &mut seller_account,
            &mut platform_obj, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(platform_obj);
        ts::return_shared(seller_account);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    public fun release(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let mut seller_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.seller_account);
        let mut platform_obj = ts::take_shared_by_id<Platform>(sc, object::id_from_address(fx.platform));
        let clock = ts::take_shared<Clock>(sc);
        agent_task::release_task(
            &config, &mut task, &mut buyer_account, &mut seller_account,
            &mut platform_obj, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(platform_obj);
        ts::return_shared(seller_account);
        ts::return_shared(buyer_account);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    public fun refund(sc: &mut Scenario, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::refund_task(&mut task, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(task);
    }

    public fun propose_refund(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::propose_refund(&mut task, &memory_config, &buyer_account, &seller_agent, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
    }

    public fun accept_refund(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::accept_refund(&mut task, &memory_config, &buyer_account, &seller_agent, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
    }

    public fun reject_refund(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::reject_refund(&mut task, &memory_config, &buyer_account, &seller_agent, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
    }

    public fun withdraw_refund(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::withdraw_refund(&mut task, &memory_config, &buyer_account, &seller_agent, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
    }

    public fun deactivate_seller_agent(sc: &mut Scenario, fx: &Fixture) {
        ts::next_tx(sc, seller());
        let mut account = ts::take_shared_by_id<MemoryAccount>(sc, fx.seller_account);
        let mut agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        memory::deactivate_sub_agent(&mut account, &mut agent, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(agent);
        ts::return_shared(account);
    }

    // ---- Reviews ----

    public fun review_seller(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID, stars: u8) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let mut seller_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.seller_account);
        let mut seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::review_seller(
            &config, &mut task, &memory_config, &buyer_account, &mut seller_account,
            &mut seller_agent, stars, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(seller_account);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    public fun review_buyer_agent(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID, stars: u8) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let mut buyer_agent = ts::take_shared_by_id<SubAgent>(sc, fx.buyer_agent);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::review_buyer_agent(
            &config, &mut task, &mut buyer_account, &mut buyer_agent, &seller_agent,
            stars, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    public fun review_buyer_principal(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID, stars: u8) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::review_buyer_principal(
            &config, &mut task, &mut buyer_account, &seller_agent, stars, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    // ---- Reads ----

    /// Sum of every MYSO coin `addr` holds.
    public fun coin_total(sc: &mut Scenario, addr: address): u64 {
        ts::next_tx(sc, addr);
        let ids = ts::ids_for_address<Coin<MYSO>>(addr);
        let mut total = 0;
        let mut i = 0;
        while (i < ids.length()) {
            let c = ts::take_from_address_by_id<Coin<MYSO>>(sc, addr, ids[i]);
            total = total + coin::value(&c);
            ts::return_to_address(addr, c);
            i = i + 1;
        };
        total
    }

    /// Takes the task in a fresh transaction; the caller reads it and returns it with
    /// `ts::return_shared`.
    public fun take_task(sc: &mut Scenario, task_id: ID): TaskEscrow {
        ts::next_tx(sc, admin());
        ts::take_shared_by_id<TaskEscrow>(sc, task_id)
    }

    public fun status_of(sc: &mut Scenario, task_id: ID): u8 {
        let task = take_task(sc, task_id);
        let status = agent_task::status(&task);
        ts::return_shared(task);
        status
    }

    // ------------------------------------------------------------
    // Fixtures
    // ------------------------------------------------------------

    /// Offer, accept and deliver as the default parties. Returns the task id.
    public fun delivered_task(sc: &mut Scenario, fx: &Fixture, amount: u64): ID {
        offer(sc, fx, buyer(), amount);
        let task_id = latest_task();
        accept_ok(sc, fx, task_id);
        deliver(sc, fx, seller_agent_addr(), task_id);
        task_id
    }

    /// Rewrites the config, keeping every other default. Only fees and `active` vary.
    fun update_config(sc: &mut Scenario, platform_fee_bps: u64, ecosystem_fee_bps: u64, active: bool) {
        // The task market pushes its reputation params to the jury, so a jury config must exist.
        if (!ts::has_most_recent_shared<JuryConfig>()) {
            ts::next_tx(sc, admin());
            let clock = ts::take_shared<Clock>(sc);
            jury::test_bootstrap(&clock, ts::ctx(sc));
            ts::return_shared(clock);
        };
        ts::next_tx(sc, admin());
        let cap = ts::take_from_sender<TaskMarketCap>(sc);
        let mut config = ts::take_shared<TaskConfig>(sc);
        let mut jury_config = ts::take_shared<JuryConfig>(sc);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::set_task_config(
            &cap,
            &mut config,
            &mut jury_config,
            platform_fee_bps,
            ecosystem_fee_bps,
            admin(),
            10_000_000,
            300_000,
            day(),
            14 * day(),
            hour(),
            90 * day(),
            hour(),
            60 * day(),
            3 * day(),
            30 * day(),
            128,
            2048,
            active,
            3000,
            50000,
            1_000_000_000,
            500,
            20000,
            2000,
            500,
            16,
            10000,
            5000,
            5000,
            &clock,
            ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(jury_config);
        ts::return_shared(config);
        ts::return_to_sender(sc, cap);
    }

    // ------------------------------------------------------------
    // Happy paths and fees
    // ------------------------------------------------------------

    #[test]
    fun human_buyer_settles_and_platform_share_folds_into_ecosystem_fee() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        assert!(status_of(&mut sc, task_id) == agent_task::status_delivered(), 0);

        settle(&mut sc, &fx, buyer(), task_id);

        assert!(status_of(&mut sc, task_id) == agent_task::status_settled(), 1);
        // 2% platform + 0.5% ecosystem, no platform bound: 2.5% to the ecosystem recipient.
        assert!(coin_total(&mut sc, seller()) == 975_000_000, 2);
        assert!(coin_total(&mut sc, admin()) == 25_000_000, 3);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::settlement_kind(&task) == 1, 4);
        assert!(agent_task::platform_fee_mist(&task) == 0, 5);
        assert!(agent_task::ecosystem_fee_mist(&task) == 25_000_000, 6);
        assert!(agent_task::net_amount_mist(&task) == 975_000_000, 7);
        assert!(agent_task::escrowed_mist(&task) == 0, 8);
        ts::return_shared(task);
        ts::end(sc);
    }

    #[test]
    fun platform_bound_task_funds_platform_treasury() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer_with_platform(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        deliver(&mut sc, &fx, seller_agent_addr(), task_id);
        settle(&mut sc, &fx, buyer(), task_id);

        assert!(coin_total(&mut sc, seller()) == 975_000_000, 0);
        assert!(coin_total(&mut sc, admin()) == 5_000_000, 1);
        ts::next_tx(&mut sc, admin());
        let platform_obj = ts::take_shared_by_id<Platform>(&sc, object::id_from_address(platform_address(&fx)));
        assert!(platform::treasury_balance<MYSO>(&platform_obj) == 20_000_000, 2);
        ts::return_shared(platform_obj);
        ts::end(sc);
    }

    #[test]
    fun buyer_agent_creates_and_principal_settles() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer_agent_addr(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        deliver(&mut sc, &fx, seller_agent_addr(), task_id);
        // The principal may act for its agent's task.
        settle(&mut sc, &fx, buyer(), task_id);
        let task = take_task(&mut sc, task_id);
        assert!(option::contains(&agent_task::buyer_agent_id(&task), &buyer_agent(&fx)), 0);
        ts::return_shared(task);
        ts::end(sc);
    }

    #[test]
    fun fees_and_windows_are_frozen_at_offer_time() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        // A later config change must not touch an accepted task.
        update_config(&mut sc, 1000, 1000, true);
        settle(&mut sc, &fx, buyer(), task_id);
        assert!(coin_total(&mut sc, seller()) == 975_000_000, 0);
        ts::end(sc);
    }

    #[test]
    fun settlement_counts_toward_both_principals_and_snapshots_credibility() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        settle(&mut sc, &fx, buyer(), task_id);

        let task = take_task(&mut sc, task_id);
        // First settlement: both snapshots are the base credibility (2000 bps).
        assert!(agent_task::buyer_credibility_bps(&task) == 2000, 0);
        assert!(agent_task::seller_credibility_bps(&task) == 2000, 1);
        ts::return_shared(task);

        ts::next_tx(&mut sc, admin());
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(&sc, buyer_account(&fx));
        let seller_account = ts::take_shared_by_id<MemoryAccount>(&sc, seller_account(&fx));
        assert!(memory::account_settled_count(&buyer_account) == 1, 2);
        assert!(memory::account_settled_count(&seller_account) == 1, 3);
        assert!(memory::account_settled_volume_mist(&buyer_account) == (one_myso() as u128), 4);
        ts::return_shared(seller_account);
        ts::return_shared(buyer_account);
        ts::end(sc);
    }

    // ------------------------------------------------------------
    // Acceptance, decline, cancel, expiry
    // ------------------------------------------------------------

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ETermsMismatch)]
    fun accept_with_wrong_terms_digest_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept(&mut sc, &fx, seller_agent_addr(), task_id, deliverable());
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EAcceptExpired)]
    fun accept_after_deadline_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        advance(&mut sc, day() + 1);
        accept_ok(&mut sc, &fx, task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENotSeller)]
    fun accept_by_outsider_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        let digest = terms_digest(&mut sc, task_id);
        accept(&mut sc, &fx, buyer(), task_id, digest);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongTaskObject)]
    fun accept_with_another_seller_agent_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        let digest = terms_digest(&mut sc, task_id);
        accept_as(&mut sc, &fx, seller_agent2_addr(), task_id, seller_agent2(&fx), digest);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun accepting_twice_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        accept_ok(&mut sc, &fx, task_id);
        ts::end(sc);
    }

    #[test]
    fun seller_principal_can_accept_and_deliver_needs_the_agent() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        let digest = terms_digest(&mut sc, task_id);
        accept(&mut sc, &fx, seller(), task_id, digest);
        assert!(status_of(&mut sc, task_id) == agent_task::status_accepted(), 0);
        ts::end(sc);
    }

    #[test]
    fun cancel_offer_returns_the_escrow() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        cancel_offer(&mut sc, &fx, buyer(), task_id);
        assert!(status_of(&mut sc, task_id) == agent_task::status_cancelled(), 0);
        assert!(coin_total(&mut sc, buyer()) == one_myso(), 1);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::close_kind(&task) == 0, 2);
        ts::return_shared(task);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun cancel_after_acceptance_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        cancel_offer(&mut sc, &fx, buyer(), task_id);
        ts::end(sc);
    }

    #[test]
    fun seller_principal_and_agent_can_decline() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let first = latest_task();
        decline(&mut sc, &fx, seller(), first);
        offer(&mut sc, &fx, buyer(), one_myso());
        let second = latest_task();
        decline(&mut sc, &fx, seller_agent_addr(), second);
        assert!(status_of(&mut sc, first) == agent_task::status_cancelled(), 0);
        assert!(status_of(&mut sc, second) == agent_task::status_cancelled(), 1);
        assert!(coin_total(&mut sc, buyer()) == 2 * one_myso(), 2);
        ts::end(sc);
    }

    #[test]
    fun a_deactivated_seller_can_still_decline_but_not_accept() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        deactivate_seller_agent(&mut sc, &fx);
        // The principal can decline an offer made before deactivation.
        decline(&mut sc, &fx, seller(), task_id);
        assert!(status_of(&mut sc, task_id) == agent_task::status_cancelled(), 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::ESubAgentNotActive)]
    fun a_deactivated_seller_cannot_accept() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        deactivate_seller_agent(&mut sc, &fx);
        let digest = terms_digest(&mut sc, task_id);
        accept(&mut sc, &fx, seller(), task_id, digest);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::ESubAgentNotActive)]
    fun a_deactivated_seller_cannot_be_offered_new_work() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        deactivate_seller_agent(&mut sc, &fx);
        offer(&mut sc, &fx, buyer(), one_myso());
        ts::end(sc);
    }

    #[test]
    fun anyone_can_expire_an_unanswered_offer() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        advance(&mut sc, day() + 1);
        expire_offer(&mut sc, @0xCAFE, task_id);
        assert!(status_of(&mut sc, task_id) == agent_task::status_cancelled(), 0);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::close_kind(&task) == 2, 1);
        ts::return_shared(task);
        assert!(coin_total(&mut sc, buyer()) == one_myso(), 2);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ETooEarly)]
    fun expiring_before_the_deadline_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        expire_offer(&mut sc, @0xCAFE, task_id);
        ts::end(sc);
    }

    // ------------------------------------------------------------
    // Delivery
    // ------------------------------------------------------------

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun delivering_an_unaccepted_offer_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        deliver(&mut sc, &fx, seller_agent_addr(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENotSeller)]
    fun only_the_seller_agent_delivers() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        deliver(&mut sc, &fx, buyer(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EDeliveryDeadlinePassed)]
    fun delivering_after_the_deadline_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        advance(&mut sc, day() + 1);
        deliver(&mut sc, &fx, seller_agent_addr(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidDigest)]
    fun delivering_a_short_digest_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        deliver_with(&mut sc, &fx, seller_agent_addr(), task_id, x"2222", b"walrus://x");
        ts::end(sc);
    }

    #[test]
    fun a_seller_deactivated_after_acceptance_can_still_deliver_and_be_paid() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        deactivate_seller_agent(&mut sc, &fx);
        deliver(&mut sc, &fx, seller_agent_addr(), task_id);
        settle(&mut sc, &fx, buyer(), task_id);
        assert!(coin_total(&mut sc, seller()) == 975_000_000, 0);
        ts::end(sc);
    }

    // ------------------------------------------------------------
    // Settlement authorization and races
    // ------------------------------------------------------------

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::ESubAgentNotFound)]
    fun the_seller_cannot_settle() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        settle(&mut sc, &fx, seller(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun settling_twice_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        settle(&mut sc, &fx, buyer(), task_id);
        settle(&mut sc, &fx, buyer(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun settling_an_undelivered_task_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        settle(&mut sc, &fx, buyer(), task_id);
        ts::end(sc);
    }

    #[test]
    fun a_late_buyer_can_still_settle_until_a_release_lands() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        // Past the settle deadline but inside the grace period.
        advance(&mut sc, day() + hour());
        settle(&mut sc, &fx, buyer(), task_id);
        assert!(status_of(&mut sc, task_id) == agent_task::status_settled(), 0);
        ts::end(sc);
    }

    // ------------------------------------------------------------
    // Release (automatic seller payment)
    // ------------------------------------------------------------

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ETooEarly)]
    fun release_before_the_grace_period_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        // Settle window (1 day) elapsed, grace (3 days) has not.
        advance(&mut sc, day() + 1);
        release(&mut sc, &fx, @0xCAFE, task_id);
        ts::end(sc);
    }

    #[test]
    fun anyone_releases_to_the_seller_after_the_grace_period() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        advance(&mut sc, 4 * day() + 1);
        release(&mut sc, &fx, @0xCAFE, task_id);

        let task = take_task(&mut sc, task_id);
        assert!(agent_task::status(&task) == agent_task::status_settled(), 0);
        assert!(agent_task::settlement_kind(&task) == 2, 1);
        ts::return_shared(task);
        assert!(coin_total(&mut sc, seller()) == 975_000_000, 2);
        ts::end(sc);
    }

    #[test]
    fun a_pending_refund_proposal_does_not_delay_the_release() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, buyer(), task_id);
        advance(&mut sc, 4 * day() + 1);
        release(&mut sc, &fx, @0xCAFE, task_id);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::status(&task) == agent_task::status_settled(), 0);
        assert!(agent_task::refund_proposer(&task) == 0, 1);
        ts::return_shared(task);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun settle_after_release_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        advance(&mut sc, 4 * day() + 1);
        release(&mut sc, &fx, @0xCAFE, task_id);
        settle(&mut sc, &fx, buyer(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun release_of_an_undelivered_task_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        advance(&mut sc, 10 * day());
        release(&mut sc, &fx, @0xCAFE, task_id);
        ts::end(sc);
    }

    // ------------------------------------------------------------
    // Refunds: delivery missed, and mutual after delivery
    // ------------------------------------------------------------

    #[test]
    fun anyone_refunds_an_accepted_task_after_a_missed_delivery() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        advance(&mut sc, day() + 1);
        refund(&mut sc, @0xCAFE, task_id);
        assert!(status_of(&mut sc, task_id) == agent_task::status_refunded(), 0);
        assert!(coin_total(&mut sc, buyer()) == one_myso(), 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ETooEarly)]
    fun refund_before_the_delivery_deadline_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        refund(&mut sc, @0xCAFE, task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun refund_task_aborts_on_an_unaccepted_offer() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        advance(&mut sc, 10 * day());
        refund(&mut sc, @0xCAFE, task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun refund_task_cannot_refund_delivered_work() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        advance(&mut sc, 10 * day());
        // No unilateral path exists, from the buyer or from anyone else.
        refund(&mut sc, buyer(), task_id);
        ts::end(sc);
    }

    #[test]
    fun seller_proposes_and_buyer_accepts_a_full_refund() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, seller_agent_addr(), task_id);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::refund_proposer(&task) == 2, 0);
        ts::return_shared(task);
        accept_refund(&mut sc, &fx, buyer(), task_id);
        assert!(status_of(&mut sc, task_id) == agent_task::status_refunded(), 1);
        assert!(coin_total(&mut sc, buyer()) == one_myso(), 2);
        assert!(coin_total(&mut sc, seller()) == 0, 3);
        ts::end(sc);
    }

    #[test]
    fun buyer_proposes_and_seller_accepts_a_full_refund() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, buyer(), task_id);
        accept_refund(&mut sc, &fx, seller(), task_id);
        assert!(status_of(&mut sc, task_id) == agent_task::status_refunded(), 0);
        assert!(coin_total(&mut sc, buyer()) == one_myso(), 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENotCounterparty)]
    fun a_proposer_cannot_accept_its_own_refund() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, buyer(), task_id);
        accept_refund(&mut sc, &fx, buyer(), task_id);
        ts::end(sc);
    }


    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EProposalPending)]
    fun the_counterparty_cannot_override_a_pending_proposal() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, buyer(), task_id);
        propose_refund(&mut sc, &fx, seller(), task_id);
        ts::end(sc);
    }

    #[test]
    fun rejecting_or_withdrawing_clears_the_proposal() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, buyer(), task_id);
        reject_refund(&mut sc, &fx, seller(), task_id);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::refund_proposer(&task) == 0, 0);
        ts::return_shared(task);

        // The rejected side may now propose, and the proposer may withdraw.
        propose_refund(&mut sc, &fx, seller(), task_id);
        withdraw_refund(&mut sc, &fx, seller(), task_id);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::refund_proposer(&task) == 0, 1);
        assert!(agent_task::status(&task) == agent_task::status_delivered(), 2);
        ts::return_shared(task);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENotProposer)]
    fun only_the_proposer_can_withdraw() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, buyer(), task_id);
        withdraw_refund(&mut sc, &fx, seller(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENoProposal)]
    fun accepting_without_a_proposal_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        accept_refund(&mut sc, &fx, seller(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun a_settled_task_cannot_be_refunded() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, seller(), task_id);
        settle(&mut sc, &fx, buyer(), task_id);
        accept_refund(&mut sc, &fx, buyer(), task_id);
        ts::end(sc);
    }

    // ------------------------------------------------------------
    // Offer validation and authorization
    // ------------------------------------------------------------

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ETaskSelfDealing)]
    fun a_principal_cannot_offer_to_its_own_agent() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer_custom(
            &mut sc, &fx, buyer(), buyer_account(&fx), buyer_account(&fx), buyer_agent(&fx),
            one_myso(), scope(), day(), day(), day(), option::none(),
        );
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EEscrowTooSmall)]
    fun dust_offers_are_rejected() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidDigest)]
    fun a_bad_scope_digest_is_rejected() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer_custom(
            &mut sc, &fx, buyer(), buyer_account(&fx), seller_account(&fx), seller_agent(&fx),
            one_myso(), x"11", day(), day(), day(), option::none(),
        );
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidWindow)]
    fun windows_outside_the_configured_bounds_are_rejected() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer_custom(
            &mut sc, &fx, buyer(), buyer_account(&fx), seller_account(&fx), seller_agent(&fx),
            one_myso(), scope(), 1, day(), day(), option::none(),
        );
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::ESubAgentMissingCap)]
    fun a_buyer_agent_without_the_task_cap_cannot_offer() {
        let mut sc = ts::begin(admin());
        let fx = setup_with_caps(&mut sc, memory::cap_memory_read(), option::none());
        offer(&mut sc, &fx, buyer_agent_addr(), one_myso());
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::ESubAgentSpendExceeded)]
    fun a_buyer_agent_cannot_exceed_its_action_spend_cap() {
        let mut sc = ts::begin(admin());
        let fx = setup_with_caps(&mut sc, memory::cap_task_trade(), option::some(one_myso() - 1));
        offer(&mut sc, &fx, buyer_agent_addr(), one_myso());
        ts::end(sc);
    }

    #[test]
    fun a_buyer_agent_within_its_spend_cap_can_offer() {
        let mut sc = ts::begin(admin());
        let fx = setup_with_caps(&mut sc, memory::cap_task_trade(), option::some(one_myso()));
        offer(&mut sc, &fx, buyer_agent_addr(), one_myso());
        assert!(status_of(&mut sc, latest_task()) == agent_task::status_offered(), 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EMarketPaused)]
    fun a_paused_market_blocks_new_offers() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        update_config(&mut sc, 200, 50, false);
        offer(&mut sc, &fx, buyer(), one_myso());
        ts::end(sc);
    }

    #[test]
    fun a_paused_market_still_lets_existing_tasks_finish() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        update_config(&mut sc, 200, 50, false);
        settle(&mut sc, &fx, buyer(), task_id);
        assert!(status_of(&mut sc, task_id) == agent_task::status_settled(), 0);
        ts::end(sc);
    }

    // ------------------------------------------------------------
    // Configuration bounds
    // ------------------------------------------------------------

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidTaskConfig)]
    fun fees_above_one_hundred_percent_are_rejected() {
        let mut sc = ts::begin(admin());
        let _fx = setup(&mut sc);
        update_config(&mut sc, 9000, 1001, true);
        ts::end(sc);
    }

    #[test]
    fun a_valid_config_update_is_applied() {
        let mut sc = ts::begin(admin());
        let _fx = setup(&mut sc);
        update_config(&mut sc, 300, 100, true);
        ts::next_tx(&mut sc, admin());
        let config = ts::take_shared<TaskConfig>(&sc);
        assert!(agent_task::config_active(&config), 0);
        ts::return_shared(config);
        ts::end(sc);
    }

    // ============================================================
    // Reputation: reviews, weighting, Bayesian scores
    // ============================================================

    fun params(sc: &mut Scenario): memory::ReputationParams {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<TaskConfig>(sc);
        let p = agent_task::config_reputation(&config);
        ts::return_shared(config);
        p
    }

    fun agent_seller_agg(sc: &mut Scenario, agent_id: ID): memory::ReputationAggregate {
        ts::next_tx(sc, admin());
        let agent = ts::take_shared_by_id<SubAgent>(sc, agent_id);
        let agg = memory::sub_agent_seller_reputation(&agent);
        ts::return_shared(agent);
        agg
    }

    fun agent_buyer_agg(sc: &mut Scenario, agent_id: ID): memory::ReputationAggregate {
        ts::next_tx(sc, admin());
        let agent = ts::take_shared_by_id<SubAgent>(sc, agent_id);
        let agg = memory::sub_agent_buyer_reputation(&agent);
        ts::return_shared(agent);
        agg
    }

    fun human_buyer_agg(sc: &mut Scenario, account_id: ID): memory::ReputationAggregate {
        ts::next_tx(sc, admin());
        let account = ts::take_shared_by_id<MemoryAccount>(sc, account_id);
        let agg = memory::account_human_buyer_reputation(&account);
        ts::return_shared(account);
        agg
    }

    /// Offer, accept, deliver and settle by the default human buyer; returns the task id.
    fun settled_task(sc: &mut Scenario, fx: &Fixture): ID {
        let task_id = delivered_task(sc, fx, one_myso());
        settle(sc, fx, buyer(), task_id);
        task_id
    }

    #[test]
    fun buyer_review_of_the_seller_agent_matches_the_hand_computed_bayesian_score() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        review_seller(&mut sc, &fx, buyer(), task_id, 5);

        let p = params(&mut sc);
        let agg = agent_seller_agg(&mut sc, seller_agent(&fx));
        assert!(memory::aggregate_review_count(&agg) == 1, 0);
        assert!(memory::aggregate_histogram(&agg) == vector[0, 0, 0, 0, 1], 1);
        // w = value 10000 * credibility 2000 / 10000 = 2000; S = 2000 * 5000.
        assert!(memory::aggregate_weight_sum_bps(&agg) == 2000, 2);
        assert!(memory::aggregate_weighted_star_sum(&agg) == 10_000_000, 3);
        // (50000 * 3000 + 10_000_000) / (50000 + 2000) = 3076.
        assert!(memory::aggregate_score_milli(&agg, &p) == 3076, 4);
        assert!(memory::aggregate_cached_score_milli(&agg) == 3076, 5);
        // 2000 * 10000 / 52000 = 384.
        assert!(memory::aggregate_confidence_bps(&agg, &p) == 384, 6);

        let task = take_task(&mut sc, task_id);
        assert!(agent_task::buyer_review_stars(&task) == 5, 7);
        assert!(agent_task::seller_review_stars(&task) == 0, 8);
        ts::return_shared(task);
        ts::end(sc);
    }

    #[test]
    fun seller_review_of_a_human_buyer_updates_only_the_account_aggregate() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        review_buyer_principal(&mut sc, &fx, seller_agent_addr(), task_id, 4);

        let p = params(&mut sc);
        let human = human_buyer_agg(&mut sc, buyer_account(&fx));
        assert!(memory::aggregate_review_count(&human) == 1, 0);
        // (150_000_000 + 2000 * 4000) / 52000 = 3038.
        assert!(memory::aggregate_score_milli(&human, &p) == 3038, 1);
        let agent_as_buyer = agent_buyer_agg(&mut sc, buyer_agent(&fx));
        assert!(memory::aggregate_review_count(&agent_as_buyer) == 0, 2);
        let seller_agg = agent_seller_agg(&mut sc, seller_agent(&fx));
        assert!(memory::aggregate_review_count(&seller_agg) == 0, 3);
        ts::end(sc);
    }

    #[test]
    fun seller_review_of_a_buyer_agent_updates_that_agents_buyer_role_only() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer_agent_addr(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        deliver(&mut sc, &fx, seller_agent_addr(), task_id);
        settle(&mut sc, &fx, buyer_agent_addr(), task_id);
        review_buyer_agent(&mut sc, &fx, seller_agent_addr(), task_id, 2);

        let p = params(&mut sc);
        let agg = agent_buyer_agg(&mut sc, buyer_agent(&fx));
        assert!(memory::aggregate_review_count(&agg) == 1, 0);
        assert!(memory::aggregate_histogram(&agg) == vector[0, 1, 0, 0, 0], 1);
        // (150_000_000 + 2000 * 2000) / 52000 = 2961.
        assert!(memory::aggregate_score_milli(&agg, &p) == 2961, 2);
        let human = human_buyer_agg(&mut sc, buyer_account(&fx));
        assert!(memory::aggregate_review_count(&human) == 0, 3);
        ts::end(sc);
    }

    #[test]
    fun both_directions_review_the_same_receipt_in_either_order() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        review_buyer_principal(&mut sc, &fx, seller_agent_addr(), task_id, 3);
        review_seller(&mut sc, &fx, buyer(), task_id, 4);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::buyer_review_stars(&task) == 4, 0);
        assert!(agent_task::seller_review_stars(&task) == 3, 1);
        ts::return_shared(task);
        ts::end(sc);
    }

    #[test]
    fun an_auto_released_receipt_can_be_reviewed_by_both_parties() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        advance(&mut sc, 4 * day() + 1);
        release(&mut sc, &fx, @0xCAFE, task_id);
        review_seller(&mut sc, &fx, buyer(), task_id, 1);
        review_buyer_principal(&mut sc, &fx, seller(), task_id, 5);
        ts::end(sc);
    }

    #[test]
    fun a_seller_agent_does_not_inherit_a_sibling_agents_reputation() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        review_seller(&mut sc, &fx, buyer(), task_id, 5);

        let p = params(&mut sc);
        let sibling = agent_seller_agg(&mut sc, seller_agent2(&fx));
        assert!(memory::aggregate_review_count(&sibling) == 0, 0);
        // No reviews: exactly the prior, with no confidence.
        assert!(memory::aggregate_score_milli(&sibling, &p) == 3000, 1);
        assert!(memory::aggregate_confidence_bps(&sibling, &p) == 0, 2);
        ts::end(sc);
    }

    #[test]
    fun a_repeat_pair_review_applies_a_decayed_weight_even_with_a_sibling_agent() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let first = settled_task(&mut sc, &fx);
        review_seller(&mut sc, &fx, buyer(), first, 5);
        let second = settled_task(&mut sc, &fx);
        review_seller(&mut sc, &fx, buyer(), second, 5);

        let p = params(&mut sc);
        let agg = agent_seller_agg(&mut sc, seller_agent(&fx));
        assert!(memory::aggregate_review_count(&agg) == 2, 0);
        // Second settlement: credibility 2000 + 500 = 2500, so base 2500; one prior review
        // from this buyer principal halves it to 1250.
        assert!(memory::aggregate_weight_sum_bps(&agg) == 2000 + 1250, 1);
        assert!(memory::aggregate_weighted_star_sum(&agg) == 10_000_000 + 6_250_000, 2);
        // (150_000_000 + 16_250_000) / 53_250 = 3122.
        assert!(memory::aggregate_score_milli(&agg, &p) == 3122, 3);

        ts::next_tx(&mut sc, admin());
        let seller_acc = ts::take_shared_by_id<MemoryAccount>(&sc, seller_account(&fx));
        assert!(memory::pair_review_count(&seller_acc, buyer(), memory::role_buyer()) == 2, 4);
        // The opposite direction has its own counter.
        assert!(memory::pair_review_count(&seller_acc, buyer(), memory::role_seller()) == 0, 5);
        ts::return_shared(seller_acc);
        ts::end(sc);
    }

    #[test]
    fun the_overall_score_of_a_new_subject_is_the_prior_and_blends_role_evidence() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        let p = params(&mut sc);

        let empty = agent_seller_agg(&mut sc, seller_agent(&fx));
        let (score, confidence) = memory::overall_reputation(&empty, &empty, &p);
        assert!(score == 3000 && confidence == 0, 0);

        review_seller(&mut sc, &fx, buyer(), task_id, 5);
        let seller_agg = agent_seller_agg(&mut sc, seller_agent(&fx));
        let buyer_side = agent_buyer_agg(&mut sc, seller_agent(&fx));
        let (overall, overall_confidence) = memory::overall_reputation(&seller_agg, &buyer_side, &p);
        // (50000*3000*10000 + 5000*10_000_000) / (50000*10000 + 5000*2000) = 3039.
        assert!(overall == 3039, 1);
        // 5000 * 2000 * 10000 / 510_000_000 = 196.
        assert!(overall_confidence == 196, 2);
        // A role with no reviews contributes nothing, so the overall is below the per-role score
        // (which carries the full weight) and above the prior.
        assert!(overall > 3000 && overall < memory::aggregate_score_milli(&seller_agg, &p), 3);
        ts::end(sc);
    }

    // ---- Review gating ----

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun reviewing_before_settlement_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        review_seller(&mut sc, &fx, buyer(), task_id, 5);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun a_refunded_task_cannot_be_reviewed() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, buyer(), task_id);
        accept_refund(&mut sc, &fx, seller(), task_id);
        review_seller(&mut sc, &fx, buyer(), task_id, 5);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidStars)]
    fun zero_stars_are_rejected() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        review_seller(&mut sc, &fx, buyer(), task_id, 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidStars)]
    fun six_stars_are_rejected() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        review_buyer_principal(&mut sc, &fx, seller_agent_addr(), task_id, 6);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EAlreadyReviewed)]
    fun a_direction_can_only_be_reviewed_once() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        review_seller(&mut sc, &fx, buyer(), task_id, 5);
        review_seller(&mut sc, &fx, buyer(), task_id, 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EReviewWindowClosed)]
    fun reviews_after_the_window_are_rejected() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        advance(&mut sc, 30 * day() + 1);
        review_seller(&mut sc, &fx, buyer(), task_id, 5);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::ESubAgentNotFound)]
    fun the_seller_cannot_review_itself() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        review_seller(&mut sc, &fx, seller(), task_id, 5);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENotSeller)]
    fun the_buyer_cannot_review_as_the_seller() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        review_buyer_principal(&mut sc, &fx, buyer(), task_id, 5);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongReviewTarget)]
    fun the_agent_entrypoint_rejects_a_direct_buyer_task() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        review_buyer_agent(&mut sc, &fx, seller_agent_addr(), task_id, 5);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongReviewTarget)]
    fun the_principal_entrypoint_rejects_an_agent_bought_task() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer_agent_addr(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        deliver(&mut sc, &fx, seller_agent_addr(), task_id);
        settle(&mut sc, &fx, buyer(), task_id);
        review_buyer_principal(&mut sc, &fx, seller_agent_addr(), task_id, 5);
        ts::end(sc);
    }

    #[test]
    fun a_seller_deactivated_after_settlement_still_accrues_reputation() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = settled_task(&mut sc, &fx);
        deactivate_seller_agent(&mut sc, &fx);
        review_seller(&mut sc, &fx, buyer(), task_id, 5);
        let agg = agent_seller_agg(&mut sc, seller_agent(&fx));
        assert!(memory::aggregate_review_count(&agg) == 1, 0);
        ts::end(sc);
    }

    // ---- Pure reputation math ----

    fun default_params(): memory::ReputationParams {
        memory::new_reputation_params(3000, 50000, 1_000_000_000, 500, 20000, 2000, 500, 16, 10000, 5000, 5000)
    }

    #[test]
    fun value_weight_floors_dust_and_caps_whales() {
        let p = default_params();
        assert!(memory::value_weight_bps(1, &p) == 500, 0);
        assert!(memory::value_weight_bps(500_000_000, &p) == 5000, 1);
        assert!(memory::value_weight_bps(1_000_000_000, &p) == 10000, 2);
        assert!(memory::value_weight_bps(100_000_000_000, &p) == 20000, 3);
    }

    #[test]
    fun credibility_ramps_with_settled_history_and_caps() {
        let p = default_params();
        assert!(memory::credibility_bps_for(0, &p) == 2000, 0);
        assert!(memory::credibility_bps_for(1, &p) == 2500, 1);
        assert!(memory::credibility_bps_for(16, &p) == 10000, 2);
        assert!(memory::credibility_bps_for(1_000_000, &p) == 10000, 3);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::EInvalidReputationParams)]
    fun a_zero_prior_weight_is_rejected() {
        memory::new_reputation_params(3000, 0, 1_000_000_000, 500, 20000, 2000, 500, 16, 10000, 5000, 5000);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::EInvalidReputationParams)]
    fun a_prior_mean_outside_the_star_scale_is_rejected() {
        memory::new_reputation_params(5001, 50000, 1_000_000_000, 500, 20000, 2000, 500, 16, 10000, 5000, 5000);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::EInvalidReputationParams)]
    fun an_inverted_weight_range_is_rejected() {
        memory::new_reputation_params(3000, 50000, 1_000_000_000, 20000, 500, 2000, 500, 16, 10000, 5000, 5000);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::EInvalidReputationParams)]
    fun role_weights_must_sum_to_one() {
        memory::new_reputation_params(3000, 50000, 1_000_000_000, 500, 20000, 2000, 500, 16, 10000, 6000, 5000);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::EInvalidReputationParams)]
    fun a_zero_value_reference_is_rejected() {
        memory::new_reputation_params(3000, 50000, 0, 500, 20000, 2000, 500, 16, 10000, 5000, 5000);
    }

    // ---- Checked arithmetic ----

    #[test]
    fun checked_helpers_compute_exact_values() {
        assert!(memory::checked_add_u64(2, 3) == 5, 0);
        assert!(memory::checked_mul_u64(1_000_000, 1_000_000) == 1_000_000_000_000, 1);
        assert!(memory::checked_bps_of(1_000_000_000, 250) == 25_000_000, 2);
        assert!(memory::checked_bps_of(0, 250) == 0, 3);
        assert!(memory::checked_div_u128(10, 3) == 3, 4);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::EArithmeticOverflow)]
    fun u128_addition_overflow_aborts_with_a_named_code() {
        memory::checked_add_u128(340282366920938463463374607431768211455, 1);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::EArithmeticOverflow)]
    fun u128_multiplication_overflow_aborts_with_a_named_code() {
        memory::checked_mul_u128(340282366920938463463374607431768211455, 2);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::EArithmeticOverflow)]
    fun narrowing_past_u64_aborts() {
        memory::checked_narrow_u64(18446744073709551616);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::EArithmeticOverflow)]
    fun u64_addition_overflow_aborts() {
        memory::checked_add_u64(18446744073709551615, 1);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::EDivideByZero)]
    fun division_by_zero_aborts_with_a_named_code() {
        memory::checked_div_u128(1, 0);
    }

    // ------------------------------------------------------------
    // Disputes: helpers
    // ------------------------------------------------------------

    /// `enter_dispute` is package-private (the jury integration calls it), so the tests drive it
    /// directly. Roles: 1 buyer, 2 seller.
    public fun open_dispute_for(sc: &mut Scenario, task_id: ID, role: u8) {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::enter_dispute(&config, &mut task, object::id_from_address(@0xCA5E), role, 0, 0, &clock);
        ts::return_shared(clock);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    public fun cancel_dispute_for(sc: &mut Scenario, task_id: ID) {
        ts::next_tx(sc, admin());
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::cancel_dispute(&mut task, &clock);
        ts::return_shared(clock);
        ts::return_shared(task);
    }

    public fun begin_neutral_for(sc: &mut Scenario, task_id: ID, window_ms: u64) {
        ts::next_tx(sc, admin());
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::begin_neutral_window(&mut task, window_ms, &clock);
        ts::return_shared(clock);
        ts::return_shared(task);
    }

    public fun execute_neutral(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let mut seller_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.seller_account);
        let mut platform_obj = ts::take_shared_by_id<Platform>(sc, object::id_from_address(fx.platform));
        let clock = ts::take_shared<Clock>(sc);
        agent_task::execute_neutral_resolution(
            &config, &mut task, &mut buyer_account, &mut seller_account,
            &mut platform_obj, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(platform_obj);
        ts::return_shared(seller_account);
        ts::return_shared(buyer_account);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    public fun propose_settlement(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID, outcome: u8) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::propose_dispute_settlement(
            &mut task, &memory_config, &buyer_account, &seller_agent, outcome, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
    }

    public fun accept_settlement(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let mut buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let mut seller_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.seller_account);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let mut platform_obj = ts::take_shared_by_id<Platform>(sc, object::id_from_address(fx.platform));
        let clock = ts::take_shared<Clock>(sc);
        agent_task::accept_dispute_settlement(
            &config, &mut task, &memory_config, &mut buyer_account, &mut seller_account,
            &seller_agent, &mut platform_obj, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(platform_obj);
        ts::return_shared(seller_agent);
        ts::return_shared(seller_account);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    public fun clear_settlement(sc: &mut Scenario, fx: &Fixture, sender: address, task_id: ID) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, fx.seller_agent);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::clear_dispute_settlement(
            &mut task, &memory_config, &buyer_account, &seller_agent, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
    }

    /// What the jury integration will call with a verdict (or the agreement/neutral paths do
    /// themselves); package-private, so driven directly here.
    public fun apply_outcome(sc: &mut Scenario, fx: &Fixture, task_id: ID, outcome: u8, source: u8) {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.buyer_account);
        let mut seller_account = ts::take_shared_by_id<MemoryAccount>(sc, fx.seller_account);
        let mut platform_obj = ts::take_shared_by_id<Platform>(sc, object::id_from_address(fx.platform));
        let clock = ts::take_shared<Clock>(sc);
        agent_task::apply_dispute_outcome(
            &config, &mut task, &mut buyer_account, &mut seller_account,
            &mut platform_obj, outcome, source, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(platform_obj);
        ts::return_shared(seller_account);
        ts::return_shared(buyer_account);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    /// Delivered and then frozen for a jury, opened by the buyer.
    public fun disputed_task(sc: &mut Scenario, fx: &Fixture, amount: u64): ID {
        let task_id = delivered_task(sc, fx, amount);
        open_dispute_for(sc, task_id, 1);
        task_id
    }

    // ------------------------------------------------------------
    // Disputes: entering, freezing, cancelling
    // ------------------------------------------------------------

    #[test]
    fun a_delivered_task_enters_a_dispute_and_reports_it() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        let task = take_task(&mut sc, task_id);
        assert!(!agent_task::has_dispute(&task), 0);
        assert!(agent_task::dispute_state(&task) == 0, 1);
        ts::return_shared(task);

        open_dispute_for(&mut sc, task_id, 1);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::status(&task) == agent_task::status_disputed(), 2);
        assert!(agent_task::has_dispute(&task), 3);
        assert!(agent_task::dispute_state(&task) == 1, 4);
        assert!(agent_task::dispute_case_id(&task) == option::some(object::id_from_address(@0xCA5E)), 5);
        assert!(agent_task::escrowed_mist(&task) == one_myso(), 6);
        ts::return_shared(task);
        ts::end(sc);
    }

    #[test]
    fun a_pending_refund_proposal_is_voided_when_a_jury_takes_the_task() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, buyer(), task_id);
        open_dispute_for(&mut sc, task_id, 1);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::refund_proposer(&task) == 0, 0);
        ts::return_shared(task);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun a_task_must_be_delivered_to_be_disputed() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        open_dispute_for(&mut sc, task_id, 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidRole)]
    fun a_dispute_needs_a_buyer_or_seller_opener() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        open_dispute_for(&mut sc, task_id, 3);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EDisputeWindowClosed)]
    fun a_dispute_cannot_open_once_the_task_is_releasable() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        advance(&mut sc, 4 * day() + 1);
        open_dispute_for(&mut sc, task_id, 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun a_disputed_task_cannot_be_settled() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        settle(&mut sc, &fx, buyer(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun a_disputed_task_cannot_be_released_however_long_it_waits() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        advance(&mut sc, 30 * day());
        release(&mut sc, &fx, @0xCAFE, task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun a_disputed_task_cannot_be_refunded_by_the_delivery_path() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        advance(&mut sc, 30 * day());
        refund(&mut sc, @0xCAFE, task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun a_disputed_task_takes_no_new_refund_proposals() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        propose_refund(&mut sc, &fx, buyer(), task_id);
        ts::end(sc);
    }

    #[test]
    fun cancelling_returns_the_task_to_delivered_and_rebases_the_release_clock() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        // Settle window 1 day + grace 3 days: originally releasable after day 4.
        advance(&mut sc, 3 * day());
        open_dispute_for(&mut sc, task_id, 2);
        cancel_dispute_for(&mut sc, task_id);

        let task = take_task(&mut sc, task_id);
        assert!(agent_task::status(&task) == agent_task::status_delivered(), 0);
        assert!(agent_task::dispute_state(&task) == 2, 1);
        ts::return_shared(task);

        // Re-based to the cancel time: releasable only 3 days after it, i.e. after day 6.
        advance(&mut sc, 3 * day() + 1);
        release(&mut sc, &fx, @0xCAFE, task_id);
        assert!(status_of(&mut sc, task_id) == agent_task::status_settled(), 2);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ETooEarly)]
    fun a_cancelled_dispute_still_delays_the_release_by_a_full_grace_period() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        advance(&mut sc, 3 * day());
        open_dispute_for(&mut sc, task_id, 2);
        cancel_dispute_for(&mut sc, task_id);
        // Day 4 + 1ms: the original release time, but the clock was re-based.
        advance(&mut sc, day() + 1);
        release(&mut sc, &fx, @0xCAFE, task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EDisputeExists)]
    fun a_task_can_only_be_disputed_once() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        open_dispute_for(&mut sc, task_id, 1);
        cancel_dispute_for(&mut sc, task_id);
        open_dispute_for(&mut sc, task_id, 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun only_a_disputed_task_can_be_cancelled_back() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        cancel_dispute_for(&mut sc, task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EDisputeNotOpen)]
    fun a_dispute_cannot_be_cancelled_after_the_jury_failed() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        begin_neutral_for(&mut sc, task_id, 3 * day());
        cancel_dispute_for(&mut sc, task_id);
        ts::end(sc);
    }

    // ------------------------------------------------------------
    // Disputes: mutual agreement
    // ------------------------------------------------------------

    #[test]
    fun an_agreed_refund_returns_the_whole_escrow_to_the_buyer() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        propose_settlement(&mut sc, &fx, seller_agent_addr(), task_id, 1);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::dispute_agreement_proposer(&task) == 2, 0);
        assert!(agent_task::dispute_agreement_outcome(&task) == 1, 1);
        ts::return_shared(task);
        accept_settlement(&mut sc, &fx, buyer(), task_id);

        assert!(status_of(&mut sc, task_id) == agent_task::status_refunded(), 2);
        assert!(coin_total(&mut sc, buyer()) == one_myso(), 3);
        assert!(coin_total(&mut sc, seller()) == 0, 4);
        let task = take_task(&mut sc, task_id);
        assert!(agent_task::dispute_state(&task) == 3, 5);
        ts::return_shared(task);
        ts::end(sc);
    }

    #[test]
    fun an_agreed_payment_settles_to_the_seller_and_opens_reviews() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        propose_settlement(&mut sc, &fx, buyer(), task_id, 0);
        accept_settlement(&mut sc, &fx, seller(), task_id);

        let task = take_task(&mut sc, task_id);
        assert!(agent_task::status(&task) == agent_task::status_settled(), 0);
        assert!(agent_task::settlement_kind(&task) == 4, 1);
        assert!(agent_task::review_deadline_ms(&task) > 0, 2);
        ts::return_shared(task);
        assert!(coin_total(&mut sc, seller()) == 975_000_000, 3);
        assert!(coin_total(&mut sc, buyer()) == 0, 4);
        ts::end(sc);
    }

    #[test]
    fun an_agreed_split_pays_each_side_half_and_charges_fees_on_the_seller_half_only() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        propose_settlement(&mut sc, &fx, seller_agent_addr(), task_id, 2);
        accept_settlement(&mut sc, &fx, buyer(), task_id);

        let task = take_task(&mut sc, task_id);
        assert!(agent_task::status(&task) == agent_task::status_settled(), 0);
        assert!(agent_task::settlement_kind(&task) == 6, 1);
        assert!(agent_task::review_deadline_ms(&task) == 0, 2);
        ts::return_shared(task);
        // Buyer half in full; 2.5% (no platform: folded into the ecosystem fee) of the seller half.
        assert!(coin_total(&mut sc, buyer()) == 500_000_000, 3);
        assert!(coin_total(&mut sc, seller()) == 487_500_000, 4);
        assert!(coin_total(&mut sc, admin()) == 12_500_000, 5);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENotCounterparty)]
    fun a_proposer_cannot_accept_its_own_dispute_settlement() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        propose_settlement(&mut sc, &fx, buyer(), task_id, 1);
        accept_settlement(&mut sc, &fx, buyer(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENoProposal)]
    fun accepting_without_a_dispute_settlement_proposal_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        accept_settlement(&mut sc, &fx, seller(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidOutcome)]
    fun a_proposal_outside_the_menu_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        propose_settlement(&mut sc, &fx, buyer(), task_id, 3);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EProposalPending)]
    fun a_party_cannot_override_the_counterpartys_pending_proposal() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        propose_settlement(&mut sc, &fx, buyer(), task_id, 1);
        propose_settlement(&mut sc, &fx, seller(), task_id, 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENoProposal)]
    fun a_cleared_dispute_settlement_cannot_be_accepted() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        propose_settlement(&mut sc, &fx, buyer(), task_id, 1);
        clear_settlement(&mut sc, &fx, seller(), task_id);
        accept_settlement(&mut sc, &fx, seller(), task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun a_dispute_settlement_needs_a_disputed_task() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        propose_settlement(&mut sc, &fx, buyer(), task_id, 1);
        ts::end(sc);
    }

    // ------------------------------------------------------------
    // Disputes: failed jury and the neutral split
    // ------------------------------------------------------------

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENeutralWindowNotOpen)]
    fun the_neutral_split_needs_a_failed_jury() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        advance(&mut sc, 30 * day());
        execute_neutral(&mut sc, &fx, @0xCAFE, task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ETooEarly)]
    fun the_neutral_split_waits_for_the_settlement_window() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        begin_neutral_for(&mut sc, task_id, 3 * day());
        advance(&mut sc, hour());
        execute_neutral(&mut sc, &fx, @0xCAFE, task_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENeutralWindowOpen)]
    fun the_settlement_window_cannot_be_opened_twice() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        begin_neutral_for(&mut sc, task_id, 3 * day());
        begin_neutral_for(&mut sc, task_id, 3 * day());
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidWindow)]
    fun a_zero_settlement_window_is_rejected() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        begin_neutral_for(&mut sc, task_id, 0);
        ts::end(sc);
    }

    #[test]
    fun a_failed_jury_ends_in_a_neutral_split_that_favours_neither_side() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        begin_neutral_for(&mut sc, task_id, 3 * day());
        // Still frozen inside the window.
        assert!(status_of(&mut sc, task_id) == agent_task::status_disputed(), 0);
        advance(&mut sc, 3 * day() + 1);
        execute_neutral(&mut sc, &fx, @0xCAFE, task_id);

        let task = take_task(&mut sc, task_id);
        assert!(agent_task::status(&task) == agent_task::status_settled(), 1);
        assert!(agent_task::settlement_kind(&task) == 7, 2);
        assert!(agent_task::escrowed_mist(&task) == 0, 3);
        // A split earns neither side a credibility credit and opens no reviews.
        assert!(agent_task::buyer_credibility_bps(&task) == 0, 4);
        assert!(agent_task::seller_credibility_bps(&task) == 0, 5);
        assert!(agent_task::review_deadline_ms(&task) == 0, 6);
        ts::return_shared(task);

        let buyer_total = coin_total(&mut sc, buyer());
        let seller_total = coin_total(&mut sc, seller());
        let ecosystem_total = coin_total(&mut sc, admin());
        assert!(buyer_total == 500_000_000, 7);
        assert!(seller_total == 487_500_000, 8);
        assert!(buyer_total + seller_total + ecosystem_total == one_myso(), 9);
        // The seller never receives the full release amount.
        assert!(seller_total < 975_000_000, 10);
        ts::end(sc);
    }

    #[test]
    fun an_odd_escrow_conserves_every_unit_in_the_neutral_split() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let amount = 10_000_001;
        let task_id = disputed_task(&mut sc, &fx, amount);
        begin_neutral_for(&mut sc, task_id, day());
        advance(&mut sc, day() + 1);
        execute_neutral(&mut sc, &fx, @0xCAFE, task_id);

        let buyer_total = coin_total(&mut sc, buyer());
        let seller_total = coin_total(&mut sc, seller());
        let ecosystem_total = coin_total(&mut sc, admin());
        assert!(buyer_total == amount / 2, 0);
        assert!(buyer_total + seller_total + ecosystem_total == amount, 1);
        ts::end(sc);
    }

    #[test]
    fun a_platform_task_pays_the_platform_fee_on_the_seller_half_only() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        offer_with_platform(&mut sc, &fx, buyer(), one_myso());
        let task_id = latest_task();
        accept_ok(&mut sc, &fx, task_id);
        deliver(&mut sc, &fx, seller_agent_addr(), task_id);
        open_dispute_for(&mut sc, task_id, 2);
        begin_neutral_for(&mut sc, task_id, day());
        advance(&mut sc, day() + 1);
        execute_neutral(&mut sc, &fx, @0xCAFE, task_id);

        let task = take_task(&mut sc, task_id);
        assert!(agent_task::platform_fee_mist(&task) == 10_000_000, 0);
        assert!(agent_task::ecosystem_fee_mist(&task) == 2_500_000, 1);
        assert!(agent_task::net_amount_mist(&task) == 487_500_000, 2);
        ts::return_shared(task);
        assert!(coin_total(&mut sc, buyer()) == 500_000_000, 3);
        assert!(coin_total(&mut sc, seller()) == 487_500_000, 4);
        assert!(coin_total(&mut sc, admin()) == 2_500_000, 5);
        ts::next_tx(&mut sc, admin());
        let platform_obj = ts::take_shared_by_id<Platform>(&sc, object::id_from_address(platform_address(&fx)));
        assert!(platform::treasury_balance<MYSO>(&platform_obj) == 10_000_000, 6);
        ts::return_shared(platform_obj);
        ts::end(sc);
    }

    #[test]
    fun parties_can_still_agree_inside_the_settlement_window() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        begin_neutral_for(&mut sc, task_id, 3 * day());
        advance(&mut sc, hour());
        propose_settlement(&mut sc, &fx, buyer(), task_id, 1);
        accept_settlement(&mut sc, &fx, seller(), task_id);
        assert!(status_of(&mut sc, task_id) == agent_task::status_refunded(), 0);
        assert!(coin_total(&mut sc, buyer()) == one_myso(), 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun the_neutral_split_cannot_run_after_the_parties_agreed() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        begin_neutral_for(&mut sc, task_id, 3 * day());
        propose_settlement(&mut sc, &fx, buyer(), task_id, 1);
        accept_settlement(&mut sc, &fx, seller(), task_id);
        advance(&mut sc, 3 * day() + 1);
        execute_neutral(&mut sc, &fx, @0xCAFE, task_id);
        ts::end(sc);
    }

    // ------------------------------------------------------------
    // Disputes: verdicts (the one place a dispute pays out)
    // ------------------------------------------------------------

    #[test]
    fun a_verdict_to_pay_the_seller_settles_like_a_normal_payment() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        apply_outcome(&mut sc, &fx, task_id, agent_task::outcome_pay_seller(), agent_task::source_verdict());

        let task = take_task(&mut sc, task_id);
        assert!(agent_task::status(&task) == agent_task::status_settled(), 0);
        assert!(agent_task::settlement_kind(&task) == 3, 1);
        assert!(agent_task::review_deadline_ms(&task) > 0, 2);
        ts::return_shared(task);
        assert!(coin_total(&mut sc, seller()) == 975_000_000, 3);
        ts::end(sc);
    }

    #[test]
    fun a_verdict_to_refund_returns_the_whole_escrow() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        apply_outcome(&mut sc, &fx, task_id, agent_task::outcome_refund_buyer(), agent_task::source_verdict());
        assert!(status_of(&mut sc, task_id) == agent_task::status_refunded(), 0);
        assert!(coin_total(&mut sc, buyer()) == one_myso(), 1);
        assert!(coin_total(&mut sc, seller()) == 0, 2);
        ts::end(sc);
    }

    #[test]
    fun a_verdict_split_pays_halves_with_no_credibility_and_no_reviews() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        apply_outcome(&mut sc, &fx, task_id, agent_task::outcome_split_50_50(), agent_task::source_verdict());

        let task = take_task(&mut sc, task_id);
        assert!(agent_task::settlement_kind(&task) == 5, 0);
        assert!(agent_task::review_deadline_ms(&task) == 0, 1);
        assert!(agent_task::buyer_credibility_bps(&task) == 0, 2);
        ts::return_shared(task);
        assert!(coin_total(&mut sc, buyer()) == 500_000_000, 3);
        assert!(coin_total(&mut sc, seller()) == 487_500_000, 4);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidOutcome)]
    fun an_outcome_outside_the_menu_cannot_move_the_escrow() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        apply_outcome(&mut sc, &fx, task_id, 3, agent_task::source_verdict());
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidOutcome)]
    fun the_neutral_source_can_only_split() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        apply_outcome(&mut sc, &fx, task_id, agent_task::outcome_pay_seller(), agent_task::source_neutral());
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInvalidOutcome)]
    fun an_unknown_source_is_rejected() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        apply_outcome(&mut sc, &fx, task_id, agent_task::outcome_refund_buyer(), 9);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun an_outcome_applies_exactly_once() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = disputed_task(&mut sc, &fx, one_myso());
        apply_outcome(&mut sc, &fx, task_id, agent_task::outcome_refund_buyer(), agent_task::source_verdict());
        apply_outcome(&mut sc, &fx, task_id, agent_task::outcome_pay_seller(), agent_task::source_verdict());
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun an_outcome_needs_a_disputed_task() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc);
        let task_id = delivered_task(&mut sc, &fx, one_myso());
        apply_outcome(&mut sc, &fx, task_id, agent_task::outcome_pay_seller(), agent_task::source_verdict());
        ts::end(sc);
    }
}
