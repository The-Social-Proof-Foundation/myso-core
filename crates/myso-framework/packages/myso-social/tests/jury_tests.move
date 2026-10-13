// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Jury engine tests. The fixture (up to 16 owners, each with a profile, a memory account and one
/// jury-capable agent) and the action helpers live in this module so no separate helper file is
/// needed. Cases are opened through the package-private `open_case` exactly as an integrator would.
#[test_only]
#[allow(duplicate_alias, unused_use, unused_const, unused_variable)]
module social_contracts::jury_tests {
    use std::option::{Self, Option};
    use std::string;
    use std::vector;

    use myso::balance;
    use myso::clock::{Self, Clock};
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::object::{Self, ID};
    use myso::random::{Self, Random};
    use myso::test_scenario::{Self as ts, Scenario};

    use social_contracts::ai_credit::AiCreditConfig;
    use social_contracts::jury::{Self, JuryAdminCap, JuryCase, JuryConfig, JurorRoster};
    use social_contracts::memory::{Self, AgenticOrganization, MemoryAccount, MemoryConfig, SubAgent};
    use social_contracts::memory_test_helpers;
    use social_contracts::platform;
    use social_contracts::profile::{Self, ProfileConfig, UsernameRegistry};

    // ============================================================
    // Fixture
    // ============================================================

    public fun admin(): address { @0xAD }
    public fun hour(): u64 { 3_600_000 }
    public fun day(): u64 { 86_400_000 }

    public fun owners(): vector<address> {
        vector[
            @0xA01, @0xA02, @0xA03, @0xA04, @0xA05, @0xA06, @0xA07, @0xA08,
            @0xA09, @0xA0A, @0xA0B, @0xA0C, @0xA0D, @0xA0E, @0xA0F, @0xA10,
        ]
    }

    public fun agent_addrs(): vector<address> {
        vector[
            @0xB01, @0xB02, @0xB03, @0xB04, @0xB05, @0xB06, @0xB07, @0xB08,
            @0xB09, @0xB0A, @0xB0B, @0xB0C, @0xB0D, @0xB0E, @0xB0F, @0xB10,
        ]
    }

    public fun owner(i: u64): address { *vector::borrow(&owners(), i) }
    public fun agent_addr(i: u64): address { *vector::borrow(&agent_addrs(), i) }

    public fun repeat_byte(b: u8, n: u64): vector<u8> {
        let mut v = vector::empty<u8>();
        let mut j = 0;
        while (j < n) {
            vector::push_back(&mut v, b);
            j = j + 1;
        };
        v
    }

    public fun agent_key(i: u64): vector<u8> { repeat_byte(((i + 16) as u8), 32) }
    public fun digest(b: u8): vector<u8> { repeat_byte(b, 32) }
    public fun salt(b: u8): vector<u8> { repeat_byte(b, 16) }

    public fun username(i: u64): vector<u8> {
        let mut name = b"juror";
        vector::push_back(&mut name, ((97 + i / 26) as u8));
        vector::push_back(&mut name, ((97 + i % 26) as u8));
        name
    }

    public struct Fixture has copy, drop {
        accounts: vector<ID>,
        agents: vector<ID>,
    }

    public fun agent_id(fx: &Fixture, i: u64): ID { *vector::borrow(&fx.agents, i) }
    public fun account_id(fx: &Fixture, i: u64): ID { *vector::borrow(&fx.accounts, i) }

    public fun index_of(fx: &Fixture, id: ID): u64 {
        let mut i = 0;
        while (i < vector::length(&fx.agents)) {
            if (*vector::borrow(&fx.agents, i) == id) {
                return i
            };
            i = i + 1;
        };
        abort 999
    }

    /// Randomness, jury objects, and `n` owners each with one agent holding the jury capability.
    public fun setup(sc: &mut Scenario, n: u64): Fixture {
        setup_with_caps(sc, n, memory::cap_jury())
    }

    public fun setup_with_caps(sc: &mut Scenario, n: u64, caps: u64): Fixture {
        ts::next_tx(sc, admin());
        {
            let clock = clock::create_for_testing(ts::ctx(sc));
            profile::init_for_testing(&clock, ts::ctx(sc));
            platform::test_init(&clock, ts::ctx(sc));
            clock::share_for_testing(clock);
        };
        bootstrap_jury(sc);
        create_jurors(sc, n, caps)
    }

    /// The jury objects, the admin cap and randomness, on top of an existing clock and profile setup.
    public fun bootstrap_jury(sc: &mut Scenario) {
        ts::next_tx(sc, admin());
        {
            let clock = ts::take_shared<Clock>(sc);
            jury::test_bootstrap(&clock, ts::ctx(sc));
            ts::return_shared(clock);
        };
        ts::next_tx(sc, admin());
        {
            let cap = jury::test_create_admin_cap(ts::ctx(sc));
            myso::transfer::public_transfer(cap, admin());
        };
        ts::next_tx(sc, @0x0);
        {
            random::create_for_testing(ts::ctx(sc));
        };
        ts::next_tx(sc, @0x0);
        {
            let mut rnd = ts::take_shared<Random>(sc);
            random::update_randomness_state_for_testing(
                &mut rnd,
                0,
                x"1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F1F",
                ts::ctx(sc),
            );
            ts::return_shared(rnd);
        };
    }

    /// `n` owners, each with a profile, a memory account and one agent holding `caps`.
    public fun create_jurors(sc: &mut Scenario, n: u64, caps: u64): Fixture {
        let mut accounts = vector::empty<ID>();
        let mut agents = vector::empty<ID>();
        let mut i = 0;
        while (i < n) {
            create_profile(sc, owner(i), username(i));
            let account = account_id_of(sc, owner(i));
            let agent = register_agent(sc, owner(i), account, agent_addr(i), agent_key(i), caps);
            vector::push_back(&mut accounts, account);
            vector::push_back(&mut agents, agent);
            i = i + 1;
        };
        Fixture { accounts, agents }
    }

    public fun create_profile(sc: &mut Scenario, who: address, username: vector<u8>) {
        ts::next_tx(sc, who);
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
            string::utf8(b"Juror"),
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

    public fun account_id_of(sc: &mut Scenario, who: address): ID {
        ts::next_tx(sc, who);
        let account = memory_test_helpers::take_owner_memory_account(sc);
        let id = object::id(&account);
        ts::return_shared(account);
        id
    }

    public fun register_agent(
        sc: &mut Scenario,
        who: address,
        account_id: ID,
        derived: address,
        key: vector<u8>,
        caps: u64,
    ): ID {
        ts::next_tx(sc, who);
        memory_test_helpers::create_default_org_in_tx(sc);

        ts::next_tx(sc, who);
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
            string::utf8(b"juror-agent"),
            memory::class_delegated_ai(),
            0,
            caps,
            caps,
            3,
            0,
            option::none(),
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

    public fun advance(sc: &mut Scenario, ms: u64) {
        ts::next_tx(sc, admin());
        let mut clock = ts::take_shared<Clock>(sc);
        clock::increment_for_testing(&mut clock, ms);
        ts::return_shared(clock);
    }

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

    // ============================================================
    // Admin and roster actions
    // ============================================================

    public fun add_genesis(sc: &mut Scenario, fx: &Fixture, i: u64) {
        ts::next_tx(sc, admin());
        let cap = ts::take_from_sender<JuryAdminCap>(sc);
        let mut config = ts::take_shared<JuryConfig>(sc);
        let clock = ts::take_shared<Clock>(sc);
        jury::add_genesis_agent(&cap, &mut config, agent_id(fx, i), &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(config);
        ts::return_to_sender(sc, cap);
    }

    public fun join(sc: &mut Scenario, fx: &Fixture, sender: address, i: u64) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<JuryConfig>(sc);
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let account = ts::take_shared_by_id<MemoryAccount>(sc, account_id(fx, i));
        let agent = ts::take_shared_by_id<SubAgent>(sc, agent_id(fx, i));
        let clock = ts::take_shared<Clock>(sc);
        jury::join_roster(&config, &mut roster, &memory_config, &account, &agent, owner(i), 1, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(agent);
        ts::return_shared(account);
        ts::return_shared(memory_config);
        ts::return_shared(roster);
        ts::return_shared(config);
    }

    /// Genesis-allows and joins owners `0..n`.
    public fun fill_roster(sc: &mut Scenario, fx: &Fixture, n: u64) {
        let mut i = 0;
        while (i < n) {
            add_genesis(sc, fx, i);
            join(sc, fx, owner(i), i);
            i = i + 1;
        };
    }

    public fun leave(sc: &mut Scenario, fx: &Fixture, sender: address, i: u64) {
        ts::next_tx(sc, sender);
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let account = ts::take_shared_by_id<MemoryAccount>(sc, account_id(fx, i));
        let clock = ts::take_shared<Clock>(sc);
        jury::leave_roster(&mut roster, &account, agent_id(fx, i), &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(account);
        ts::return_shared(roster);
    }

    public fun set_available(sc: &mut Scenario, fx: &Fixture, i: u64, available: bool) {
        ts::next_tx(sc, owner(i));
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let account = ts::take_shared_by_id<MemoryAccount>(sc, account_id(fx, i));
        jury::set_availability(&mut roster, &account, agent_id(fx, i), available, ts::ctx(sc));
        ts::return_shared(account);
        ts::return_shared(roster);
    }

    public fun refresh(sc: &mut Scenario, fx: &Fixture, i: u64) {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<JuryConfig>(sc);
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let account = ts::take_shared_by_id<MemoryAccount>(sc, account_id(fx, i));
        let agent = ts::take_shared_by_id<SubAgent>(sc, agent_id(fx, i));
        let clock = ts::take_shared<Clock>(sc);
        jury::refresh_roster_entry(&config, &mut roster, &memory_config, &account, &agent, &clock);
        ts::return_shared(clock);
        ts::return_shared(agent);
        ts::return_shared(account);
        ts::return_shared(memory_config);
        ts::return_shared(roster);
        ts::return_shared(config);
    }

    public fun evict(sc: &mut Scenario, fx: &Fixture, i: u64) {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<JuryConfig>(sc);
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let clock = ts::take_shared<Clock>(sc);
        jury::evict_ineligible(&config, &mut roster, agent_id(fx, i), &clock);
        ts::return_shared(clock);
        ts::return_shared(roster);
        ts::return_shared(config);
    }

    public fun eligible(sc: &mut Scenario): u64 {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<JuryConfig>(sc);
        let roster = ts::take_shared<JurorRoster>(sc);
        let clock = ts::take_shared<Clock>(sc);
        let count = jury::eligible_count(&config, &roster, clock::timestamp_ms(&clock));
        ts::return_shared(clock);
        ts::return_shared(roster);
        ts::return_shared(config);
        count
    }

    public fun roster_ready(sc: &mut Scenario) {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<JuryConfig>(sc);
        let roster = ts::take_shared<JurorRoster>(sc);
        let clock = ts::take_shared<Clock>(sc);
        jury::assert_roster_ready(&config, &roster, &clock);
        ts::return_shared(clock);
        ts::return_shared(roster);
        ts::return_shared(config);
    }

    public fun set_min_roster(sc: &mut Scenario, n: u64) {
        ts::next_tx(sc, admin());
        let cap = ts::take_from_sender<JuryAdminCap>(sc);
        let mut config = ts::take_shared<JuryConfig>(sc);
        let clock = ts::take_shared<Clock>(sc);
        jury::set_eligibility(&cap, &mut config, 4000, 2000, 3, 7 * day(), 7 * day(), 3, n, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(config);
        ts::return_to_sender(sc, cap);
    }

    /// Strikes before suspension; everything else keeps its default.
    public fun set_max_strikes(sc: &mut Scenario, strikes: u64) {
        ts::next_tx(sc, admin());
        let cap = ts::take_from_sender<JuryAdminCap>(sc);
        let mut config = ts::take_shared<JuryConfig>(sc);
        let clock = ts::take_shared<Clock>(sc);
        jury::set_duty(&cap, &mut config, 100, 200, 50, 5, 10, 25, strikes, 14 * day(), &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(config);
        ts::return_to_sender(sc, cap);
    }

    public fun set_paused(sc: &mut Scenario, active: bool) {
        ts::next_tx(sc, admin());
        let cap = ts::take_from_sender<JuryAdminCap>(sc);
        let mut config = ts::take_shared<JuryConfig>(sc);
        let clock = ts::take_shared<Clock>(sc);
        jury::set_active(&cap, &mut config, active, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(config);
        ts::return_to_sender(sc, cap);
    }

    public fun set_sunset(sc: &mut Scenario, sunset_ms: u64) {
        ts::next_tx(sc, admin());
        let cap = ts::take_from_sender<JuryAdminCap>(sc);
        let mut config = ts::take_shared<JuryConfig>(sc);
        let clock = ts::take_shared<Clock>(sc);
        jury::set_genesis_sunset(&cap, &mut config, sunset_ms, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(config);
        ts::return_to_sender(sc, cap);
    }

    public fun entry_score(sc: &mut Scenario, fx: &Fixture, i: u64): u64 {
        ts::next_tx(sc, admin());
        let roster = ts::take_shared<JurorRoster>(sc);
        let v = jury::entry_duty_score(&roster, agent_id(fx, i));
        ts::return_shared(roster);
        v
    }

    public fun entry_strikes(sc: &mut Scenario, fx: &Fixture, i: u64): u64 {
        ts::next_tx(sc, admin());
        let roster = ts::take_shared<JurorRoster>(sc);
        let v = jury::entry_strikes(&roster, agent_id(fx, i));
        ts::return_shared(roster);
        v
    }

    public fun entry_status(sc: &mut Scenario, fx: &Fixture, i: u64): u8 {
        ts::next_tx(sc, admin());
        let roster = ts::take_shared<JurorRoster>(sc);
        let v = jury::entry_status(&roster, agent_id(fx, i));
        ts::return_shared(roster);
        v
    }

    public fun entry_active(sc: &mut Scenario, fx: &Fixture, i: u64): u64 {
        ts::next_tx(sc, admin());
        let roster = ts::take_shared<JurorRoster>(sc);
        let v = jury::entry_active_cases(&roster, agent_id(fx, i));
        ts::return_shared(roster);
        v
    }

    public fun roster_size(sc: &mut Scenario): u64 {
        ts::next_tx(sc, admin());
        let roster = ts::take_shared<JurorRoster>(sc);
        let v = jury::roster_count(&roster);
        ts::return_shared(roster);
        v
    }

    // ============================================================
    // Case actions
    // ============================================================

    /// Governance-shaped defaults: one fixed evidence item, no evidence window, fee 1000, no cap.
    public fun open_simple(sc: &mut Scenario, round_sizes: vector<u64>, quorum: u64, appealable: bool): ID {
        open_full(sc, round_sizes, quorum, 5000, appealable, 0, vector[], vector[], 1000, 0)
    }

    public fun open_full(
        sc: &mut Scenario,
        round_sizes: vector<u64>,
        quorum: u64,
        approval_bps: u64,
        appealable: bool,
        evidence_window_ms: u64,
        exclusion_owners: vector<address>,
        exclusion_agents: vector<ID>,
        fee: u64,
        max_juror_fee: u64,
    ): ID {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<JuryConfig>(sc);
        let clock = ts::take_shared<Clock>(sc);
        let mut case = jury::open_case(
            &config,
            jury::subject_governance(),
            object::id_from_address(@0x5B1),
            1,
            2,
            approval_bps,
            quorum,
            round_sizes,
            appealable,
            0,
            evidence_window_ms,
            exclusion_owners,
            vector[],
            exclusion_agents,
            balance::create_for_testing<MYSO>(fee),
            max_juror_fee,
            @0x7EA5,
            &clock,
            ts::ctx(sc),
        );
        jury::add_evidence(&config, &mut case, admin(), 1, digest(1), b"ipfs://proposal", &clock);
        let case_id = jury::share_case(case);
        ts::return_shared(clock);
        ts::return_shared(config);
        case_id
    }

    public fun add_evidence_as(sc: &mut Scenario, case_id: ID, who: address, kind: u8, dig: vector<u8>) {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<JuryConfig>(sc);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let clock = ts::take_shared<Clock>(sc);
        jury::add_evidence(&config, &mut case, who, kind, dig, b"ipfs://more", &clock);
        ts::return_shared(clock);
        ts::return_shared(case);
        ts::return_shared(config);
    }

    public fun do_select(sc: &mut Scenario, case_id: ID) {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<JuryConfig>(sc);
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let rnd = ts::take_shared<Random>(sc);
        let clock = ts::take_shared<Clock>(sc);
        jury::test_select_jury(&config, &mut roster, &mut case, &rnd, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(rnd);
        ts::return_shared(case);
        ts::return_shared(roster);
        ts::return_shared(config);
    }

    public fun do_replace(sc: &mut Scenario, case_id: ID) {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<JuryConfig>(sc);
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let rnd = ts::take_shared<Random>(sc);
        let clock = ts::take_shared<Clock>(sc);
        jury::test_replace_no_shows(&config, &mut roster, &mut case, &rnd, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(rnd);
        ts::return_shared(case);
        ts::return_shared(roster);
        ts::return_shared(config);
    }

    public fun do_accept(sc: &mut Scenario, fx: &Fixture, sender: address, i: u64, case_id: ID) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<JuryConfig>(sc);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let account = ts::take_shared_by_id<MemoryAccount>(sc, account_id(fx, i));
        let agent = ts::take_shared_by_id<SubAgent>(sc, agent_id(fx, i));
        let clock = ts::take_shared<Clock>(sc);
        jury::accept_assignment(&config, &mut case, &memory_config, &account, &agent, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(agent);
        ts::return_shared(account);
        ts::return_shared(memory_config);
        ts::return_shared(case);
        ts::return_shared(config);
    }

    public fun refs_for(outcome: u8): vector<u64> {
        if (outcome == jury::abstain()) { vector[] } else { vector[0] }
    }

    public fun commitment_of(case_id: ID, round: u64, agent: ID, outcome: u8, refs: &vector<u64>, salt_byte: u8): vector<u8> {
        jury::compute_commitment(case_id, round, agent, outcome, &digest(7), refs, &digest(9), &salt(salt_byte))
    }

    public fun do_commit_with(
        sc: &mut Scenario,
        fx: &Fixture,
        sender: address,
        i: u64,
        case_id: ID,
        commitment: vector<u8>,
    ) {
        ts::next_tx(sc, sender);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let account = ts::take_shared_by_id<MemoryAccount>(sc, account_id(fx, i));
        let agent = ts::take_shared_by_id<SubAgent>(sc, agent_id(fx, i));
        let clock = ts::take_shared<Clock>(sc);
        jury::commit_vote(&mut case, &memory_config, &account, &agent, commitment, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(agent);
        ts::return_shared(account);
        ts::return_shared(memory_config);
        ts::return_shared(case);
    }

    public fun do_commit(sc: &mut Scenario, fx: &Fixture, i: u64, case_id: ID, outcome: u8, salt_byte: u8) {
        let round = round_of(sc, case_id);
        let c = commitment_of(case_id, round, agent_id(fx, i), outcome, &refs_for(outcome), salt_byte);
        do_commit_with(sc, fx, agent_addr(i), i, case_id, c);
    }

    public fun do_reveal_with(
        sc: &mut Scenario,
        fx: &Fixture,
        sender: address,
        i: u64,
        case_id: ID,
        outcome: u8,
        refs: vector<u64>,
        salt_byte: u8,
    ) {
        ts::next_tx(sc, sender);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let account = ts::take_shared_by_id<MemoryAccount>(sc, account_id(fx, i));
        let agent = ts::take_shared_by_id<SubAgent>(sc, agent_id(fx, i));
        let clock = ts::take_shared<Clock>(sc);
        jury::reveal_vote(
            &mut case, &memory_config, &account, &agent, outcome, digest(7), refs, digest(9), salt(salt_byte),
            &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(agent);
        ts::return_shared(account);
        ts::return_shared(memory_config);
        ts::return_shared(case);
    }

    public fun do_reveal(sc: &mut Scenario, fx: &Fixture, i: u64, case_id: ID, outcome: u8, salt_byte: u8) {
        do_reveal_with(sc, fx, agent_addr(i), i, case_id, outcome, refs_for(outcome), salt_byte);
    }

    public fun do_finalize_round(sc: &mut Scenario, case_id: ID) {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<JuryConfig>(sc);
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let clock = ts::take_shared<Clock>(sc);
        jury::finalize_round(&config, &mut roster, &mut case, &clock);
        ts::return_shared(clock);
        ts::return_shared(case);
        ts::return_shared(roster);
        ts::return_shared(config);
    }

    public fun do_finalize_case(sc: &mut Scenario, case_id: ID) {
        ts::next_tx(sc, admin());
        let config = ts::take_shared<JuryConfig>(sc);
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let clock = ts::take_shared<Clock>(sc);
        jury::finalize_case(&config, &mut roster, &mut case, &clock);
        ts::return_shared(clock);
        ts::return_shared(case);
        ts::return_shared(roster);
        ts::return_shared(config);
    }

    public fun do_void(sc: &mut Scenario, case_id: ID) {
        ts::next_tx(sc, admin());
        let cap = ts::take_from_sender<JuryAdminCap>(sc);
        let config = ts::take_shared<JuryConfig>(sc);
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let clock = ts::take_shared<Clock>(sc);
        jury::void_case(&cap, &config, &mut roster, &mut case, &clock);
        ts::return_shared(clock);
        ts::return_shared(case);
        ts::return_shared(roster);
        ts::return_shared(config);
        ts::return_to_sender(sc, cap);
    }

    public fun do_appeal(sc: &mut Scenario, case_id: ID) {
        ts::next_tx(sc, admin());
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let clock = ts::take_shared<Clock>(sc);
        jury::appeal(&mut case, @0xFEED, &clock);
        ts::return_shared(clock);
        ts::return_shared(case);
    }

    public fun do_cancel(sc: &mut Scenario, case_id: ID) {
        ts::next_tx(sc, admin());
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let clock = ts::take_shared<Clock>(sc);
        jury::cancel_case(&mut case, &clock);
        ts::return_shared(clock);
        ts::return_shared(case);
    }

    public fun do_claim(sc: &mut Scenario, case_id: ID, id: ID) {
        ts::next_tx(sc, admin());
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        jury::claim_juror_fee(&mut case, id, ts::ctx(sc));
        ts::return_shared(case);
    }

    public fun do_sweep(sc: &mut Scenario, case_id: ID): u64 {
        ts::next_tx(sc, admin());
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let remainder = jury::sweep_remainder(&mut case);
        let value = balance::destroy_for_testing(remainder);
        ts::return_shared(case);
        value
    }

    // Readers.

    public fun state_of(sc: &mut Scenario, case_id: ID): u8 {
        ts::next_tx(sc, admin());
        let case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let v = jury::case_state(&case);
        ts::return_shared(case);
        v
    }

    public fun round_of(sc: &mut Scenario, case_id: ID): u64 {
        ts::next_tx(sc, admin());
        let case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let v = jury::case_round(&case);
        ts::return_shared(case);
        v
    }

    public fun jurors_of(sc: &mut Scenario, case_id: ID): vector<ID> {
        ts::next_tx(sc, admin());
        let case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let v = jury::round_jurors(&case, jury::case_round(&case));
        ts::return_shared(case);
        v
    }

    public fun final_of(sc: &mut Scenario, case_id: ID): Option<u8> {
        ts::next_tx(sc, admin());
        let case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let v = jury::case_final_outcome(&case);
        ts::return_shared(case);
        v
    }

    public fun fail_reason_of(sc: &mut Scenario, case_id: ID): u8 {
        ts::next_tx(sc, admin());
        let case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let v = jury::case_fail_reason(&case);
        ts::return_shared(case);
        v
    }

    public fun slot_status_of(sc: &mut Scenario, case_id: ID, id: ID): u8 {
        ts::next_tx(sc, admin());
        let case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let v = jury::slot_status(&case, id);
        ts::return_shared(case);
        v
    }

    public fun participants_of(sc: &mut Scenario, case_id: ID): u64 {
        ts::next_tx(sc, admin());
        let case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let v = jury::case_participants(&case);
        ts::return_shared(case);
        v
    }

    public fun per_juror_of(sc: &mut Scenario, case_id: ID): u64 {
        ts::next_tx(sc, admin());
        let case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let v = jury::case_per_juror_fee(&case);
        ts::return_shared(case);
        v
    }

    /// Accepts every drafted seat of the current round, each as its own agent.
    public fun accept_all(sc: &mut Scenario, fx: &Fixture, case_id: ID) {
        let jurors = jurors_of(sc, case_id);
        let mut k = 0;
        while (k < vector::length(&jurors)) {
            let i = index_of(fx, *vector::borrow(&jurors, k));
            do_accept(sc, fx, agent_addr(i), i, case_id);
            k = k + 1;
        };
    }

    /// Juror `k` of the current round commits `outcomes[k]` (salt byte `k + 1`).
    public fun commit_all(sc: &mut Scenario, fx: &Fixture, case_id: ID, outcomes: vector<u8>) {
        let jurors = jurors_of(sc, case_id);
        let mut k = 0;
        while (k < vector::length(&outcomes)) {
            let i = index_of(fx, *vector::borrow(&jurors, k));
            do_commit(sc, fx, i, case_id, *vector::borrow(&outcomes, k), ((k + 1) as u8));
            k = k + 1;
        };
    }

    public fun reveal_all(sc: &mut Scenario, fx: &Fixture, case_id: ID, outcomes: vector<u8>) {
        let jurors = jurors_of(sc, case_id);
        let mut k = 0;
        while (k < vector::length(&outcomes)) {
            let i = index_of(fx, *vector::borrow(&jurors, k));
            do_reveal(sc, fx, i, case_id, *vector::borrow(&outcomes, k), ((k + 1) as u8));
            k = k + 1;
        };
    }

    /// Select, accept, commit and reveal a full round with `outcomes`, then finalize it.
    public fun run_round(sc: &mut Scenario, fx: &Fixture, case_id: ID, outcomes: vector<u8>) {
        do_select(sc, case_id);
        accept_all(sc, fx, case_id);
        commit_all(sc, fx, case_id, outcomes);
        reveal_all(sc, fx, case_id, outcomes);
        do_finalize_round(sc, case_id);
    }

    // ============================================================
    // Roster
    // ============================================================

    #[test]
    fun bootstrap_creates_an_active_config_and_an_empty_roster() {
        let mut sc = ts::begin(admin());
        let _fx = setup(&mut sc, 0);
        ts::next_tx(&mut sc, admin());
        let config = ts::take_shared<JuryConfig>(&sc);
        let roster = ts::take_shared<JurorRoster>(&sc);
        assert!(jury::config_active(&config), 0);
        assert!(jury::roster_count(&roster) == 0, 1);
        assert!(jury::min_eligible_roster(&config) == 20, 2);
        assert!(jury::neutral_window_ms(&config) == 72 * hour(), 3);
        assert!(jury::initial_duty_score(&config) == 100, 4);
        ts::return_shared(roster);
        ts::return_shared(config);
        ts::end(sc);
    }

    #[test]
    fun a_genesis_agent_joins_the_roster_without_any_reputation() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 1);
        add_genesis(&mut sc, &fx, 0);
        join(&mut sc, &fx, owner(0), 0);
        assert!(roster_size(&mut sc) == 1, 0);
        assert!(entry_score(&mut sc, &fx, 0) == 100, 1);
        assert!(entry_status(&mut sc, &fx, 0) == jury::roster_active(), 2);
        ts::next_tx(&mut sc, admin());
        let roster = ts::take_shared<JurorRoster>(&sc);
        assert!(jury::on_roster(&roster, agent_id(&fx, 0)), 3);
        assert!(jury::entry_exempt(&roster, agent_id(&fx, 0)), 4);
        assert!(jury::entry_owner(&roster, agent_id(&fx, 0)) == owner(0), 5);
        ts::return_shared(roster);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EIneligible)]
    fun an_agent_without_reputation_or_a_genesis_pass_cannot_join() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 1);
        join(&mut sc, &fx, owner(0), 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EIneligible)]
    fun an_agent_without_the_jury_capability_cannot_join_even_as_genesis() {
        let mut sc = ts::begin(admin());
        let fx = setup_with_caps(&mut sc, 1, memory::cap_task_trade());
        add_genesis(&mut sc, &fx, 0);
        join(&mut sc, &fx, owner(0), 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::memory::ENoAccess)]
    fun only_the_owner_can_put_an_agent_on_the_roster() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 2);
        add_genesis(&mut sc, &fx, 0);
        join(&mut sc, &fx, owner(1), 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EAlreadyOnRoster)]
    fun joining_twice_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 1);
        add_genesis(&mut sc, &fx, 0);
        join(&mut sc, &fx, owner(0), 0);
        join(&mut sc, &fx, owner(0), 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EJuryPaused)]
    fun a_paused_jury_takes_no_new_roster_members() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 1);
        add_genesis(&mut sc, &fx, 0);
        set_paused(&mut sc, false);
        join(&mut sc, &fx, owner(0), 0);
        ts::end(sc);
    }

    #[test]
    fun leaving_keeps_the_roster_index_dense() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 3);
        fill_roster(&mut sc, &fx, 3);
        leave(&mut sc, &fx, owner(0), 0);
        assert!(roster_size(&mut sc) == 2, 0);
        ts::next_tx(&mut sc, admin());
        let roster = ts::take_shared<JurorRoster>(&sc);
        assert!(!jury::on_roster(&roster, agent_id(&fx, 0)), 1);
        assert!(jury::on_roster(&roster, agent_id(&fx, 1)), 2);
        assert!(jury::on_roster(&roster, agent_id(&fx, 2)), 3);
        ts::return_shared(roster);
        // The moved entry is still removable and the freed slot is reusable.
        leave(&mut sc, &fx, owner(2), 2);
        join(&mut sc, &fx, owner(0), 0);
        assert!(roster_size(&mut sc) == 2, 4);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ENotOwner)]
    fun only_the_owner_can_remove_an_agent_from_the_roster() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 2);
        fill_roster(&mut sc, &fx, 1);
        // Owner 1 holds a valid account but not agent 0.
        ts::next_tx(&mut sc, owner(1));
        let mut roster = ts::take_shared<JurorRoster>(&sc);
        let account = ts::take_shared_by_id<MemoryAccount>(&sc, account_id(&fx, 1));
        let clock = ts::take_shared<Clock>(&sc);
        jury::leave_roster(&mut roster, &account, agent_id(&fx, 0), &clock, ts::ctx(&mut sc));
        ts::return_shared(clock);
        ts::return_shared(account);
        ts::return_shared(roster);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EHasActiveCases)]
    fun an_agent_on_an_open_case_cannot_leave() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 4);
        fill_roster(&mut sc, &fx, 4);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        let i = index_of(&fx, *vector::borrow(&jurors, 0));
        leave(&mut sc, &fx, owner(i), i);
        ts::end(sc);
    }

    #[test]
    fun a_paused_agent_is_not_drafted_until_it_resumes() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 3);
        fill_roster(&mut sc, &fx, 3);
        assert!(eligible(&mut sc) == 3, 0);
        set_available(&mut sc, &fx, 1, false);
        assert!(entry_status(&mut sc, &fx, 1) == jury::roster_paused(), 1);
        assert!(eligible(&mut sc) == 2, 2);
        set_available(&mut sc, &fx, 1, true);
        assert!(eligible(&mut sc) == 3, 3);
        ts::end(sc);
    }

    #[test]
    fun lowering_the_minimum_lets_a_small_roster_pass_the_readiness_gate() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 3);
        fill_roster(&mut sc, &fx, 3);
        set_min_roster(&mut sc, 3);
        roster_ready(&mut sc);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ERosterTooSmall)]
    fun the_default_minimum_of_twenty_blocks_a_small_roster() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 3);
        fill_roster(&mut sc, &fx, 3);
        roster_ready(&mut sc);
        ts::end(sc);
    }

    #[test]
    fun the_genesis_exemption_lapses_after_the_sunset() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 2);
        set_sunset(&mut sc, day());
        fill_roster(&mut sc, &fx, 2);
        assert!(eligible(&mut sc) == 2, 0);
        advance(&mut sc, 2 * day());
        refresh(&mut sc, &fx, 0);
        refresh(&mut sc, &fx, 1);
        // Neither has reputation of its own, so the exemption was the only thing letting them in.
        assert!(eligible(&mut sc) == 0, 1);
        ts::end(sc);
    }

    #[test]
    fun a_stale_snapshot_is_skipped_until_the_entry_is_refreshed() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 4);
        fill_roster(&mut sc, &fx, 4);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        advance(&mut sc, 8 * day());
        assert!(eligible(&mut sc) == 0, 0);
        do_select(&mut sc, case_id);
        assert!(vector::length(&jurors_of(&mut sc, case_id)) == 0, 1);
        let mut i = 0;
        while (i < 4) {
            refresh(&mut sc, &fx, i);
            i = i + 1;
        };
        assert!(eligible(&mut sc) == 4, 2);
        // The empty round is topped up by the replacement crank once the accept window ends.
        advance(&mut sc, 13 * hour());
        do_replace(&mut sc, case_id);
        assert!(vector::length(&jurors_of(&mut sc, case_id)) == 3, 3);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ENotIneligible)]
    fun an_eligible_agent_cannot_be_evicted() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 1);
        fill_roster(&mut sc, &fx, 1);
        evict(&mut sc, &fx, 0);
        ts::end(sc);
    }

    // ============================================================
    // Drawing
    // ============================================================

    #[test]
    fun a_case_without_an_evidence_window_waits_for_a_draw() {
        let mut sc = ts::begin(admin());
        let _fx = setup(&mut sc, 0);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        assert!(state_of(&mut sc, case_id) == jury::case_awaiting_draw(), 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EWrongCaseState)]
    fun the_draw_waits_for_the_evidence_window_to_close() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 4);
        fill_roster(&mut sc, &fx, 4);
        let case_id = open_full(&mut sc, vector[3], 2, 5000, false, day(), vector[], vector[], 1000, 0);
        assert!(state_of(&mut sc, case_id) == jury::case_evidence(), 0);
        do_select(&mut sc, case_id);
        ts::end(sc);
    }

    #[test]
    fun the_draw_runs_after_the_evidence_window_closes() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 4);
        fill_roster(&mut sc, &fx, 4);
        let case_id = open_full(&mut sc, vector[3], 2, 5000, false, day(), vector[], vector[], 1000, 0);
        advance(&mut sc, day() + 1);
        do_select(&mut sc, case_id);
        assert!(state_of(&mut sc, case_id) == jury::case_selecting(), 0);
        assert!(vector::length(&jurors_of(&mut sc, case_id)) == 3, 1);
        ts::end(sc);
    }

    #[test]
    fun a_draw_seats_distinct_owners_and_counts_each_seat() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 8);
        fill_roster(&mut sc, &fx, 8);
        let case_id = open_simple(&mut sc, vector[5], 3, false);
        do_select(&mut sc, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        assert!(vector::length(&jurors) == 5, 0);
        let mut seen = vector::empty<address>();
        let mut k = 0;
        while (k < 5) {
            let i = index_of(&fx, *vector::borrow(&jurors, k));
            assert!(!vector::contains(&seen, &owner(i)), 1);
            vector::push_back(&mut seen, owner(i));
            assert!(entry_active(&mut sc, &fx, i) == 1, 2);
            assert!(slot_status_of(&mut sc, case_id, *vector::borrow(&jurors, k)) == jury::slot_drafted(), 3);
            k = k + 1;
        };
        ts::end(sc);
    }

    #[test]
    fun excluded_owners_and_agents_are_never_drawn() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 10);
        fill_roster(&mut sc, &fx, 10);
        let case_id = open_full(
            &mut sc,
            vector[3],
            2,
            5000,
            false,
            0,
            vector[owner(0), owner(1), owner(2)],
            vector[agent_id(&fx, 3), agent_id(&fx, 4)],
            1000,
            0,
        );
        do_select(&mut sc, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        assert!(vector::length(&jurors) == 3, 0);
        let mut k = 0;
        while (k < 3) {
            let i = index_of(&fx, *vector::borrow(&jurors, k));
            assert!(i >= 5, 1);
            k = k + 1;
        };
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EJuryPaused)]
    fun a_paused_jury_does_not_draw() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 4);
        fill_roster(&mut sc, &fx, 4);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        set_paused(&mut sc, false);
        do_select(&mut sc, case_id);
        ts::end(sc);
    }

    // ============================================================
    // Accepting and replacing
    // ============================================================

    #[test]
    fun voting_starts_the_moment_every_seat_is_accepted() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        assert!(state_of(&mut sc, case_id) == jury::case_voting(), 0);
        ts::end(sc);
    }

    #[test]
    fun the_owner_can_accept_for_the_agent() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 4);
        fill_roster(&mut sc, &fx, 4);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        let i = index_of(&fx, *vector::borrow(&jurors, 0));
        do_accept(&mut sc, &fx, owner(i), i, case_id);
        assert!(slot_status_of(&mut sc, case_id, *vector::borrow(&jurors, 0)) == jury::slot_accepted(), 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ENotJuror)]
    fun someone_else_cannot_accept_for_a_juror() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        let i = index_of(&fx, *vector::borrow(&jurors, 0));
        let stranger = if (i == 4) { owner(3) } else { owner(4) };
        do_accept(&mut sc, &fx, stranger, i, case_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ENotDrafted)]
    fun an_agent_that_was_not_drawn_cannot_accept() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        let mut outsider = 0;
        while (vector::contains(&jurors, &agent_id(&fx, outsider))) {
            outsider = outsider + 1;
        };
        do_accept(&mut sc, &fx, agent_addr(outsider), outsider, case_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EWindowClosed)]
    fun a_late_acceptance_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        advance(&mut sc, 13 * hour());
        let jurors = jurors_of(&mut sc, case_id);
        let i = index_of(&fx, *vector::borrow(&jurors, 0));
        do_accept(&mut sc, &fx, agent_addr(i), i, case_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ENotYet)]
    fun no_show_replacement_waits_for_the_accept_window() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        advance(&mut sc, hour());
        do_replace(&mut sc, case_id);
        ts::end(sc);
    }

    #[test]
    fun a_no_show_is_replaced_penalized_and_the_new_seat_can_accept() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 8);
        fill_roster(&mut sc, &fx, 8);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        let drafted = jurors_of(&mut sc, case_id);
        let a = index_of(&fx, *vector::borrow(&drafted, 0));
        let b = index_of(&fx, *vector::borrow(&drafted, 1));
        let absent = *vector::borrow(&drafted, 2);
        let absent_index = index_of(&fx, absent);
        do_accept(&mut sc, &fx, agent_addr(a), a, case_id);
        do_accept(&mut sc, &fx, agent_addr(b), b, case_id);
        advance(&mut sc, 13 * hour());
        do_replace(&mut sc, case_id);

        assert!(slot_status_of(&mut sc, case_id, absent) == jury::slot_replaced(), 0);
        assert!(entry_score(&mut sc, &fx, absent_index) == 90, 1);
        assert!(entry_strikes(&mut sc, &fx, absent_index) == 1, 2);
        assert!(entry_active(&mut sc, &fx, absent_index) == 0, 3);
        let seats = jurors_of(&mut sc, case_id);
        assert!(vector::length(&seats) == 3, 4);
        assert!(!vector::contains(&seats, &absent), 5);
        // The replacement is a different owner, and accepting fills the round.
        let fresh = *vector::borrow(&seats, 2);
        let fresh_index = index_of(&fx, fresh);
        assert!(owner(fresh_index) != owner(absent_index), 6);
        do_accept(&mut sc, &fx, agent_addr(fresh_index), fresh_index, case_id);
        assert!(state_of(&mut sc, case_id) == jury::case_voting(), 7);
        ts::end(sc);
    }

    #[test]
    fun enough_acceptances_after_the_replacements_run_out_start_voting_short_handed() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 3);
        fill_roster(&mut sc, &fx, 3);
        // Quorum 2 of 3: with everyone drafted and two accepting, replacements cannot add anyone.
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        let seats = jurors_of(&mut sc, case_id);
        let a = index_of(&fx, *vector::borrow(&seats, 0));
        let b = index_of(&fx, *vector::borrow(&seats, 1));
        do_accept(&mut sc, &fx, agent_addr(a), a, case_id);
        do_accept(&mut sc, &fx, agent_addr(b), b, case_id);
        let mut calls = 0u64;
        while (calls < 4) {
            advance(&mut sc, 13 * hour());
            if (state_of(&mut sc, case_id) == jury::case_selecting()) {
                do_replace(&mut sc, case_id);
            };
            calls = calls + 1;
        };
        assert!(state_of(&mut sc, case_id) == jury::case_voting(), 0);
        assert!(vector::length(&jurors_of(&mut sc, case_id)) == 2, 1);
        ts::end(sc);
    }

    #[test]
    fun a_jury_that_cannot_be_formed_fails_without_blaming_anyone_who_accepted() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 3);
        fill_roster(&mut sc, &fx, 3);
        let case_id = open_simple(&mut sc, vector[3], 3, false);
        do_select(&mut sc, case_id);
        let seats = jurors_of(&mut sc, case_id);
        let a = index_of(&fx, *vector::borrow(&seats, 0));
        do_accept(&mut sc, &fx, agent_addr(a), a, case_id);
        let mut calls = 0u64;
        while (calls < 5) {
            advance(&mut sc, 13 * hour());
            if (state_of(&mut sc, case_id) == jury::case_selecting()) {
                do_replace(&mut sc, case_id);
            };
            calls = calls + 1;
        };
        assert!(state_of(&mut sc, case_id) == jury::case_failed(), 0);
        assert!(fail_reason_of(&mut sc, case_id) == jury::fail_cannot_form(), 1);
        assert!(option::is_none(&final_of(&mut sc, case_id)), 2);
        // The juror who accepted is released with no penalty; the two who never did lose score.
        assert!(entry_score(&mut sc, &fx, a) == 100, 3);
        assert!(entry_active(&mut sc, &fx, a) == 0, 4);
        assert!(do_sweep(&mut sc, case_id) == 1000, 5);
        ts::end(sc);
    }

    #[test]
    fun a_juror_at_the_strike_limit_is_suspended_for_the_configured_time() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 8);
        fill_roster(&mut sc, &fx, 8);
        set_max_strikes(&mut sc, 1);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        let seats = jurors_of(&mut sc, case_id);
        let absent = *vector::borrow(&seats, 2);
        let absent_index = index_of(&fx, absent);
        let a = index_of(&fx, *vector::borrow(&seats, 0));
        let b = index_of(&fx, *vector::borrow(&seats, 1));
        do_accept(&mut sc, &fx, agent_addr(a), a, case_id);
        do_accept(&mut sc, &fx, agent_addr(b), b, case_id);
        advance(&mut sc, 13 * hour());
        do_replace(&mut sc, case_id);
        assert!(entry_status(&mut sc, &fx, absent_index) == jury::roster_suspended(), 0);
        assert!(entry_strikes(&mut sc, &fx, absent_index) == 0, 1);
        ts::next_tx(&mut sc, admin());
        let roster = ts::take_shared<JurorRoster>(&sc);
        let now_ms = 13 * hour();
        assert!(jury::entry_suspended_until_ms(&roster, agent_id(&fx, absent_index)) >= now_ms + 14 * day(), 2);
        ts::return_shared(roster);
        // A suspended agent is not eligible for the next draw.
        assert!(eligible(&mut sc) == 7, 3);
        ts::end(sc);
    }

    // ============================================================
    // Voting
    // ============================================================

    #[test]
    fun a_jury_decides_by_majority_and_pays_each_participant_an_equal_share() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 8);
        fill_roster(&mut sc, &fx, 8);
        let case_id = open_simple(&mut sc, vector[5], 3, false);
        run_round(&mut sc, &fx, case_id, vector[0, 0, 0, 1, 1]);

        assert!(state_of(&mut sc, case_id) == jury::case_final(), 0);
        assert!(final_of(&mut sc, case_id) == option::some(0), 1);
        assert!(participants_of(&mut sc, case_id) == 5, 2);
        // 1000 split over five jurors, however they voted.
        assert!(per_juror_of(&mut sc, case_id) == 200, 3);

        let jurors = jurors_of(&mut sc, case_id);
        let mut k = 0;
        while (k < 5) {
            let id = *vector::borrow(&jurors, k);
            do_claim(&mut sc, case_id, id);
            assert!(coin_total(&mut sc, owner(index_of(&fx, id))) == 200, 4);
            assert!(entry_score(&mut sc, &fx, index_of(&fx, id)) == 105, 5);
            assert!(entry_active(&mut sc, &fx, index_of(&fx, id)) == 0, 6);
            k = k + 1;
        };
        assert!(do_sweep(&mut sc, case_id) == 0, 7);
        ts::end(sc);
    }

    #[test]
    fun the_minority_is_paid_and_credited_like_the_majority() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        run_round(&mut sc, &fx, case_id, vector[1, 1, 0]);
        let jurors = jurors_of(&mut sc, case_id);
        let dissenter = *vector::borrow(&jurors, 2);
        let i = index_of(&fx, dissenter);
        do_claim(&mut sc, case_id, dissenter);
        assert!(coin_total(&mut sc, owner(i)) == 333, 0);
        assert!(entry_score(&mut sc, &fx, i) == 105, 1);
        assert!(entry_strikes(&mut sc, &fx, i) == 0, 2);
        ts::end(sc);
    }

    #[test]
    fun a_per_juror_cap_returns_the_surplus_to_the_integrator() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_full(&mut sc, vector[3], 2, 5000, false, 0, vector[], vector[], 1000, 100);
        run_round(&mut sc, &fx, case_id, vector[0, 0, 0]);
        assert!(per_juror_of(&mut sc, case_id) == 100, 0);
        assert!(do_sweep(&mut sc, case_id) == 700, 1);
        // The reserved shares stay claimable after the sweep.
        let jurors = jurors_of(&mut sc, case_id);
        do_claim(&mut sc, case_id, *vector::borrow(&jurors, 0));
        assert!(coin_total(&mut sc, owner(index_of(&fx, *vector::borrow(&jurors, 0)))) == 100, 2);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EAlreadyClaimed)]
    fun a_fee_share_can_be_claimed_once() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        run_round(&mut sc, &fx, case_id, vector[0, 0, 0]);
        let jurors = jurors_of(&mut sc, case_id);
        do_claim(&mut sc, case_id, *vector::borrow(&jurors, 0));
        do_claim(&mut sc, case_id, *vector::borrow(&jurors, 0));
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EFeesNotSettled)]
    fun fees_cannot_be_claimed_before_the_case_ends() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        do_claim(&mut sc, case_id, *vector::borrow(&jurors, 0));
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EWrongSlotStatus)]
    fun a_juror_who_never_revealed_cannot_claim() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        commit_all(&mut sc, &fx, case_id, vector[0, 0, 0]);
        reveal_all(&mut sc, &fx, case_id, vector[0, 0]);
        advance(&mut sc, 2 * day());
        do_finalize_round(&mut sc, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        do_claim(&mut sc, case_id, *vector::borrow(&jurors, 2));
        ts::end(sc);
    }

    #[test]
    fun a_juror_who_never_reveals_loses_score_and_takes_a_strike() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        commit_all(&mut sc, &fx, case_id, vector[0, 0, 0]);
        reveal_all(&mut sc, &fx, case_id, vector[0, 0]);
        let jurors = jurors_of(&mut sc, case_id);
        let silent = *vector::borrow(&jurors, 2);
        let silent_index = index_of(&fx, silent);
        advance(&mut sc, 2 * day());
        do_finalize_round(&mut sc, case_id);

        // Two reveals meet the quorum of two and agree, so the case still decides.
        assert!(final_of(&mut sc, case_id) == option::some(0), 0);
        assert!(slot_status_of(&mut sc, case_id, silent) == jury::slot_defaulted(), 1);
        assert!(entry_score(&mut sc, &fx, silent_index) == 75, 2);
        assert!(entry_strikes(&mut sc, &fx, silent_index) == 1, 3);
        assert!(participants_of(&mut sc, case_id) == 2, 4);
        assert!(per_juror_of(&mut sc, case_id) == 500, 5);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EWindowClosed)]
    fun committing_after_the_commit_window_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        advance(&mut sc, 25 * hour());
        commit_all(&mut sc, &fx, case_id, vector[0]);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ENotYet)]
    fun a_vote_cannot_be_opened_while_a_seat_has_not_committed() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        commit_all(&mut sc, &fx, case_id, vector[0, 0]);
        reveal_all(&mut sc, &fx, case_id, vector[0]);
        ts::end(sc);
    }

    #[test]
    fun votes_open_once_the_commit_window_ends_even_with_a_missing_commit() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        commit_all(&mut sc, &fx, case_id, vector[0, 0]);
        advance(&mut sc, 25 * hour());
        reveal_all(&mut sc, &fx, case_id, vector[0, 0]);
        assert!(participants_of(&mut sc, case_id) == 2, 0);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ECommitmentMismatch)]
    fun a_reveal_with_the_wrong_salt_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        commit_all(&mut sc, &fx, case_id, vector[0, 0, 0]);
        let jurors = jurors_of(&mut sc, case_id);
        let i = index_of(&fx, *vector::borrow(&jurors, 0));
        // Committed with salt byte 1; reveal with 9.
        do_reveal_with(&mut sc, &fx, agent_addr(i), i, case_id, 0, vector[0], 9);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ECommitmentMismatch)]
    fun a_reveal_that_changes_the_vote_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        commit_all(&mut sc, &fx, case_id, vector[0, 0, 0]);
        let jurors = jurors_of(&mut sc, case_id);
        let i = index_of(&fx, *vector::borrow(&jurors, 0));
        do_reveal_with(&mut sc, &fx, agent_addr(i), i, case_id, 1, vector[0], 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ECommitmentMismatch)]
    fun a_copied_commitment_cannot_be_revealed_by_the_copier() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        let a = index_of(&fx, *vector::borrow(&jurors, 0));
        let b = index_of(&fx, *vector::borrow(&jurors, 1));
        let c = index_of(&fx, *vector::borrow(&jurors, 2));
        // Juror a commits honestly; b copies a's commitment verbatim; c commits.
        let copied = commitment_of(case_id, 0, agent_id(&fx, a), 0, &vector[0], 1);
        do_commit_with(&mut sc, &fx, agent_addr(a), a, case_id, copy copied);
        do_commit_with(&mut sc, &fx, agent_addr(b), b, case_id, copied);
        do_commit(&mut sc, &fx, c, case_id, 0, 3);
        // b can reveal exactly what a revealed, but the commitment binds a's agent id, not b's.
        do_reveal_with(&mut sc, &fx, agent_addr(b), b, case_id, 0, vector[0], 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EInvalidOutcome)]
    fun an_outcome_outside_the_menu_cannot_be_revealed() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        let i = index_of(&fx, *vector::borrow(&jurors, 0));
        let c = commitment_of(case_id, 0, agent_id(&fx, i), 2, &vector[0], 1);
        do_commit_with(&mut sc, &fx, agent_addr(i), i, case_id, c);
        do_commit(&mut sc, &fx, index_of(&fx, *vector::borrow(&jurors, 1)), case_id, 0, 2);
        do_commit(&mut sc, &fx, index_of(&fx, *vector::borrow(&jurors, 2)), case_id, 0, 3);
        do_reveal_with(&mut sc, &fx, agent_addr(i), i, case_id, 2, vector[0], 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EInvalidEvidenceRef)]
    fun a_vote_for_an_option_must_cite_evidence() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        let i = index_of(&fx, *vector::borrow(&jurors, 0));
        let c = commitment_of(case_id, 0, agent_id(&fx, i), 0, &vector[], 1);
        do_commit_with(&mut sc, &fx, agent_addr(i), i, case_id, c);
        do_commit(&mut sc, &fx, index_of(&fx, *vector::borrow(&jurors, 1)), case_id, 0, 2);
        do_commit(&mut sc, &fx, index_of(&fx, *vector::borrow(&jurors, 2)), case_id, 0, 3);
        do_reveal_with(&mut sc, &fx, agent_addr(i), i, case_id, 0, vector[], 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EInvalidEvidenceRef)]
    fun a_vote_cannot_cite_evidence_that_does_not_exist() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        let i = index_of(&fx, *vector::borrow(&jurors, 0));
        let c = commitment_of(case_id, 0, agent_id(&fx, i), 0, &vector[5], 1);
        do_commit_with(&mut sc, &fx, agent_addr(i), i, case_id, c);
        do_commit(&mut sc, &fx, index_of(&fx, *vector::borrow(&jurors, 1)), case_id, 0, 2);
        do_commit(&mut sc, &fx, index_of(&fx, *vector::borrow(&jurors, 2)), case_id, 0, 3);
        do_reveal_with(&mut sc, &fx, agent_addr(i), i, case_id, 0, vector[5], 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EWrongSlotStatus)]
    fun a_second_commit_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        commit_all(&mut sc, &fx, case_id, vector[0]);
        commit_all(&mut sc, &fx, case_id, vector[1]);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EWrongSlotStatus)]
    fun a_second_reveal_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        commit_all(&mut sc, &fx, case_id, vector[0, 0, 0]);
        reveal_all(&mut sc, &fx, case_id, vector[0]);
        reveal_all(&mut sc, &fx, case_id, vector[0]);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ERoundNotOver)]
    fun a_round_cannot_close_while_votes_are_still_open() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        commit_all(&mut sc, &fx, case_id, vector[0, 0, 0]);
        reveal_all(&mut sc, &fx, case_id, vector[0, 0]);
        do_finalize_round(&mut sc, case_id);
        ts::end(sc);
    }

    // ============================================================
    // Tally: decisive, inconclusive, failed
    // ============================================================

    #[test]
    fun abstentions_count_toward_quorum_but_not_toward_an_option() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 3, false);
        // Two for option 1 and one abstention: quorum of 3 met, 2 of 2 non-abstain agree.
        run_round(&mut sc, &fx, case_id, vector[1, 1, 255]);
        assert!(final_of(&mut sc, case_id) == option::some(1), 0);
        ts::end(sc);
    }

    #[test]
    fun a_tie_on_the_last_round_fails_the_case_and_still_pays_the_participants() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[4], 3, false);
        run_round(&mut sc, &fx, case_id, vector[0, 0, 1, 1]);
        assert!(state_of(&mut sc, case_id) == jury::case_failed(), 0);
        assert!(fail_reason_of(&mut sc, case_id) == jury::fail_cannot_decide(), 1);
        assert!(option::is_none(&final_of(&mut sc, case_id)), 2);
        assert!(participants_of(&mut sc, case_id) == 4, 3);
        assert!(per_juror_of(&mut sc, case_id) == 250, 4);
        ts::end(sc);
    }

    #[test]
    fun too_few_reveals_is_inconclusive() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 3, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        commit_all(&mut sc, &fx, case_id, vector[0, 0, 0]);
        reveal_all(&mut sc, &fx, case_id, vector[0, 0]);
        advance(&mut sc, 2 * day());
        do_finalize_round(&mut sc, case_id);
        assert!(state_of(&mut sc, case_id) == jury::case_failed(), 0);
        assert!(fail_reason_of(&mut sc, case_id) == jury::fail_cannot_decide(), 1);
        ts::end(sc);
    }

    #[test]
    fun a_higher_approval_bar_turns_a_bare_majority_into_no_decision() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_full(&mut sc, vector[3], 2, 6666, false, 0, vector[], vector[], 1000, 0);
        // Two of three is 66.67%, but the bar here is 66.66%: it passes; the next case needs more.
        run_round(&mut sc, &fx, case_id, vector[1, 1, 0]);
        assert!(final_of(&mut sc, case_id) == option::some(1), 0);

        let strict = open_full(&mut sc, vector[3], 2, 8000, false, 0, vector[], vector[], 1000, 0);
        run_round(&mut sc, &fx, strict, vector[1, 1, 0]);
        assert!(state_of(&mut sc, strict) == jury::case_failed(), 1);
        ts::end(sc);
    }

    #[test]
    fun an_inconclusive_round_escalates_to_a_larger_jury_of_fresh_owners() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 12);
        fill_roster(&mut sc, &fx, 12);
        let case_id = open_simple(&mut sc, vector[3, 5], 2, false);
        run_round(&mut sc, &fx, case_id, vector[0, 1, 255]);

        assert!(state_of(&mut sc, case_id) == jury::case_awaiting_draw(), 0);
        assert!(round_of(&mut sc, case_id) == 1, 1);
        let first = {
            ts::next_tx(&mut sc, admin());
            let case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
            let v = jury::round_jurors(&case, 0);
            ts::return_shared(case);
            v
        };

        run_round(&mut sc, &fx, case_id, vector[0, 0, 0, 0, 1]);
        assert!(final_of(&mut sc, case_id) == option::some(0), 2);
        assert!(participants_of(&mut sc, case_id) == 8, 3);
        assert!(per_juror_of(&mut sc, case_id) == 125, 4);
        // Nobody from round one sat again.
        let second = jurors_of(&mut sc, case_id);
        let mut k = 0;
        while (k < vector::length(&second)) {
            assert!(!vector::contains(&first, vector::borrow(&second, k)), 5);
            k = k + 1;
        };
        ts::end(sc);
    }

    // ============================================================
    // Appeals
    // ============================================================

    #[test]
    fun a_decisive_round_can_be_appealed_and_the_larger_jury_overturns_it() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 12);
        fill_roster(&mut sc, &fx, 12);
        let case_id = open_simple(&mut sc, vector[3, 5], 2, true);
        run_round(&mut sc, &fx, case_id, vector[0, 0, 1]);
        assert!(state_of(&mut sc, case_id) == jury::case_tallied(), 0);
        ts::next_tx(&mut sc, admin());
        {
            let case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
            assert!(jury::case_provisional_outcome(&case) == option::some(0), 1);
            assert!(jury::case_appeal_deadline_ms(&case) > 0, 2);
            ts::return_shared(case);
        };

        do_appeal(&mut sc, case_id);
        assert!(state_of(&mut sc, case_id) == jury::case_awaiting_draw(), 3);
        ts::next_tx(&mut sc, admin());
        {
            let case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
            assert!(jury::case_prior_outcome(&case) == option::some(0), 4);
            assert!(jury::case_appeals_used(&case) == 1, 5);
            ts::return_shared(case);
        };

        run_round(&mut sc, &fx, case_id, vector[1, 1, 1, 0, 0]);
        assert!(final_of(&mut sc, case_id) == option::some(1), 6);
        ts::next_tx(&mut sc, admin());
        {
            let case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
            assert!(!jury::case_failed_appeal(&case), 7);
            ts::return_shared(case);
        };
        ts::end(sc);
    }

    #[test]
    fun an_unappealed_verdict_becomes_final_after_the_appeal_window() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 8);
        fill_roster(&mut sc, &fx, 8);
        let case_id = open_simple(&mut sc, vector[3, 5], 2, true);
        run_round(&mut sc, &fx, case_id, vector[1, 1, 1]);
        advance(&mut sc, 49 * hour());
        do_finalize_case(&mut sc, case_id);
        assert!(state_of(&mut sc, case_id) == jury::case_final(), 0);
        assert!(final_of(&mut sc, case_id) == option::some(1), 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ENotYet)]
    fun the_appeal_window_must_end_before_the_verdict_is_final() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 8);
        fill_roster(&mut sc, &fx, 8);
        let case_id = open_simple(&mut sc, vector[3, 5], 2, true);
        run_round(&mut sc, &fx, case_id, vector[1, 1, 1]);
        advance(&mut sc, 47 * hour());
        do_finalize_case(&mut sc, case_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EWindowClosed)]
    fun an_appeal_after_the_window_aborts() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 8);
        fill_roster(&mut sc, &fx, 8);
        let case_id = open_simple(&mut sc, vector[3, 5], 2, true);
        run_round(&mut sc, &fx, case_id, vector[1, 1, 1]);
        advance(&mut sc, 49 * hour());
        do_appeal(&mut sc, case_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EWrongCaseState)]
    fun a_case_that_is_not_appealable_goes_straight_to_final() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 8);
        fill_roster(&mut sc, &fx, 8);
        let case_id = open_simple(&mut sc, vector[3, 5], 2, false);
        run_round(&mut sc, &fx, case_id, vector[1, 1, 1]);
        assert!(state_of(&mut sc, case_id) == jury::case_final(), 0);
        do_appeal(&mut sc, case_id);
        ts::end(sc);
    }

    #[test]
    fun a_failed_appeal_leaves_the_first_verdict_standing() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 12);
        fill_roster(&mut sc, &fx, 12);
        let case_id = open_simple(&mut sc, vector[3, 3], 2, true);
        run_round(&mut sc, &fx, case_id, vector[0, 0, 0]);
        do_appeal(&mut sc, case_id);
        do_select(&mut sc, case_id);
        // Nobody on the appeal jury ever accepts.
        let mut calls = 0u64;
        while (calls < 6) {
            advance(&mut sc, 13 * hour());
            if (state_of(&mut sc, case_id) == jury::case_selecting()) {
                do_replace(&mut sc, case_id);
            };
            calls = calls + 1;
        };
        assert!(state_of(&mut sc, case_id) == jury::case_final(), 0);
        assert!(final_of(&mut sc, case_id) == option::some(0), 1);
        ts::next_tx(&mut sc, admin());
        let case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
        assert!(jury::case_failed_appeal(&case), 2);
        ts::return_shared(case);
        ts::end(sc);
    }

    // ============================================================
    // Evidence
    // ============================================================

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EEvidenceClosed)]
    fun evidence_closes_with_the_evidence_window() {
        let mut sc = ts::begin(admin());
        let _fx = setup(&mut sc, 0);
        let case_id = open_full(&mut sc, vector[3], 2, 5000, false, day(), vector[], vector[], 1000, 0);
        advance(&mut sc, day() + 1);
        add_evidence_as(&mut sc, case_id, @0xCAFE, 2, digest(2));
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EEvidenceClosed)]
    fun evidence_closes_once_the_jury_is_drawn() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 4);
        fill_roster(&mut sc, &fx, 4);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        add_evidence_as(&mut sc, case_id, @0xCAFE, 2, digest(2));
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EInvalidDigest)]
    fun evidence_needs_a_thirty_two_byte_digest() {
        let mut sc = ts::begin(admin());
        let _fx = setup(&mut sc, 0);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        add_evidence_as(&mut sc, case_id, @0xCAFE, 2, repeat_byte(2, 31));
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ETooMuchEvidence)]
    fun one_party_cannot_flood_the_case_with_evidence() {
        let mut sc = ts::begin(admin());
        let _fx = setup(&mut sc, 0);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        let mut i = 0u64;
        while (i < 9) {
            add_evidence_as(&mut sc, case_id, @0xCAFE, 2, digest(2));
            i = i + 1;
        };
        ts::end(sc);
    }

    #[test]
    fun evidence_from_different_parties_is_counted_separately() {
        let mut sc = ts::begin(admin());
        let _fx = setup(&mut sc, 0);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        let mut i = 0u64;
        while (i < 8) {
            add_evidence_as(&mut sc, case_id, @0xCAFE, 2, digest(2));
            add_evidence_as(&mut sc, case_id, @0xD00D, 2, digest(3));
            i = i + 1;
        };
        ts::next_tx(&mut sc, admin());
        let case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
        // Sixteen from two parties plus the one attached at opening.
        assert!(jury::case_evidence_count(&case) == 17, 0);
        ts::return_shared(case);
        ts::end(sc);
    }

    // ============================================================
    // Cancel, void, timeout
    // ============================================================

    #[test]
    fun cancelling_before_a_draw_returns_the_whole_fee() {
        let mut sc = ts::begin(admin());
        let _fx = setup(&mut sc, 0);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_cancel(&mut sc, case_id);
        assert!(state_of(&mut sc, case_id) == jury::case_cancelled(), 0);
        assert!(do_sweep(&mut sc, case_id) == 1000, 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EWrongCaseState)]
    fun a_case_cannot_be_cancelled_once_jurors_are_drawn() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 4);
        fill_roster(&mut sc, &fx, 4);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        do_cancel(&mut sc, case_id);
        ts::end(sc);
    }

    #[test]
    fun voiding_a_case_fails_it_without_choosing_an_outcome_and_frees_the_seats() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 5);
        fill_roster(&mut sc, &fx, 5);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        accept_all(&mut sc, &fx, case_id);
        let jurors = jurors_of(&mut sc, case_id);
        do_void(&mut sc, case_id);
        assert!(state_of(&mut sc, case_id) == jury::case_failed(), 0);
        assert!(fail_reason_of(&mut sc, case_id) == jury::fail_voided(), 1);
        assert!(option::is_none(&final_of(&mut sc, case_id)), 2);
        let i = index_of(&fx, *vector::borrow(&jurors, 0));
        assert!(entry_active(&mut sc, &fx, i) == 0, 3);
        // Jurors who accepted a round that never voted are not penalized.
        assert!(entry_score(&mut sc, &fx, i) == 100, 4);
        ts::end(sc);
    }

    #[test]
    fun a_case_that_outlives_its_maximum_duration_fails_with_a_timeout() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 4);
        fill_roster(&mut sc, &fx, 4);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        advance(&mut sc, 31 * day());
        do_finalize_case(&mut sc, case_id);
        assert!(state_of(&mut sc, case_id) == jury::case_failed(), 0);
        assert!(fail_reason_of(&mut sc, case_id) == jury::fail_timeout(), 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ENotYet)]
    fun a_case_cannot_time_out_early() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 4);
        fill_roster(&mut sc, &fx, 4);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        do_select(&mut sc, case_id);
        advance(&mut sc, 29 * day());
        do_finalize_case(&mut sc, case_id);
        ts::end(sc);
    }

    #[test]
    fun a_finished_case_can_be_marked_executed_once() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        run_round(&mut sc, &fx, case_id, vector[0, 0, 0]);
        ts::next_tx(&mut sc, admin());
        let mut case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
        assert!(!jury::case_executed(&case), 0);
        jury::mark_executed(&mut case);
        assert!(jury::case_executed(&case), 1);
        ts::return_shared(case);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EAlreadyExecuted)]
    fun a_case_cannot_be_executed_twice() {
        let mut sc = ts::begin(admin());
        let fx = setup(&mut sc, 6);
        fill_roster(&mut sc, &fx, 6);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        run_round(&mut sc, &fx, case_id, vector[0, 0, 0]);
        ts::next_tx(&mut sc, admin());
        let mut case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
        jury::mark_executed(&mut case);
        jury::mark_executed(&mut case);
        ts::return_shared(case);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ENotFinished)]
    fun an_unfinished_case_cannot_be_marked_executed() {
        let mut sc = ts::begin(admin());
        let _fx = setup(&mut sc, 0);
        let case_id = open_simple(&mut sc, vector[3], 2, false);
        ts::next_tx(&mut sc, admin());
        let mut case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
        jury::mark_executed(&mut case);
        ts::return_shared(case);
        ts::end(sc);
    }

    // ============================================================
    // Commitment encoding
    // ============================================================

    #[test]
    fun the_commitment_binds_every_field() {
        let case_id = object::id_from_address(@0xC1);
        let agent = object::id_from_address(@0xA1);
        let other = object::id_from_address(@0xA2);
        let base = jury::compute_commitment(case_id, 0, agent, 0, &digest(7), &vector[0], &digest(9), &salt(1));
        assert!(vector::length(&base) == 32, 0);
        assert!(base != jury::compute_commitment(object::id_from_address(@0xC2), 0, agent, 0, &digest(7), &vector[0], &digest(9), &salt(1)), 1);
        assert!(base != jury::compute_commitment(case_id, 1, agent, 0, &digest(7), &vector[0], &digest(9), &salt(1)), 2);
        assert!(base != jury::compute_commitment(case_id, 0, other, 0, &digest(7), &vector[0], &digest(9), &salt(1)), 3);
        assert!(base != jury::compute_commitment(case_id, 0, agent, 1, &digest(7), &vector[0], &digest(9), &salt(1)), 4);
        assert!(base != jury::compute_commitment(case_id, 0, agent, 0, &digest(8), &vector[0], &digest(9), &salt(1)), 5);
        assert!(base != jury::compute_commitment(case_id, 0, agent, 0, &digest(7), &vector[1], &digest(9), &salt(1)), 6);
        assert!(base != jury::compute_commitment(case_id, 0, agent, 0, &digest(7), &vector[0], &digest(8), &salt(1)), 7);
        assert!(base != jury::compute_commitment(case_id, 0, agent, 0, &digest(7), &vector[0], &digest(9), &salt(2)), 8);
        assert!(base == jury::compute_commitment(case_id, 0, agent, 0, &digest(7), &vector[0], &digest(9), &salt(1)), 9);
    }

    /// The juror runner builds this same commitment off chain, byte for byte: BCS of the ids, round,
    /// outcome and refs, the raw digests and salt, hashed with SHA3-256. This vector is computed
    /// independently in `services/jury-runner/src/commitment.test.ts`; if either side changes the
    /// layout, both tests fail.
    #[test]
    fun the_commitment_matches_the_reference_vector_the_runner_uses() {
        let case_id = object::id_from_address(@0xc1);
        let agent = object::id_from_address(@0xa1);
        let commitment = jury::compute_commitment(
            case_id,
            1,
            agent,
            1,
            &digest(7),
            &vector[0u64, 2u64],
            &digest(9),
            &salt(3),
        );
        assert!(commitment == x"718d31370f8c35fa555188fa50d4feb33e2971449d4d3bc29b4ea7d36b534c31", 0);
    }
}
