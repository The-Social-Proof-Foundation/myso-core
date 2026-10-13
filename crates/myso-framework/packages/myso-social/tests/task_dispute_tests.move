// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// A task in front of a jury, end to end: the task fixture (a buyer, a seller and a platform from
/// `agent_task_tests`) and a roster of jurors (from `jury_tests`) in one scenario. Money amounts:
/// a 10 MYSO escrow, a 0.3 MYSO case fee each side (3%), jurors capped at a third of one fee.
#[test_only]
#[allow(duplicate_alias, unused_use, unused_const, unused_variable)]
module social_contracts::task_dispute_tests {
    use std::option::{Self, Option};
    use std::vector;

    use myso::clock::{Self, Clock};
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::object::{Self, ID};
    use myso::test_scenario::{Self as ts, Scenario};

    use social_contracts::agent_task::{Self, TaskConfig, TaskEscrow};
    use social_contracts::agent_task_tests::{Self as tt};
    use social_contracts::jury::{Self, JuryCase, JuryConfig, JurorRoster};
    use social_contracts::jury_tests::{Self as jt};
    use social_contracts::memory::{Self, MemoryAccount, MemoryConfig, SubAgent};
    use social_contracts::platform::Platform;

    const ESCROW: u64 = 10_000_000_000;
    const FEE: u64 = 300_000_000;
    const SELLER_NET: u64 = 9_750_000_000;
    const SELLER_HALF_NET: u64 = 4_875_000_000;

    fun setup(sc: &mut Scenario, jurors: u64): (tt::Fixture, jt::Fixture) {
        setup_with(sc, jurors, 3)
    }

    /// `min_roster` 0 keeps the default of twenty.
    fun setup_with(sc: &mut Scenario, jurors: u64, min_roster: u64): (tt::Fixture, jt::Fixture) {
        let tfx = tt::setup(sc);
        jt::bootstrap_jury(sc);
        let jfx = jt::create_jurors(sc, jurors, memory::cap_jury());
        jt::fill_roster(sc, &jfx, jurors);
        if (min_roster > 0) {
            jt::set_min_roster(sc, min_roster);
        };
        (tfx, jfx)
    }

    /// The case the last transaction shared. A transaction's effects exist only once the next one
    /// starts, so this ends the open one first.
    fun case_of(sc: &mut Scenario): ID {
        ts::next_tx(sc, tt::admin());
        ts::most_recent_id_shared<JuryCase>().destroy_some()
    }

    // ============================================================
    // Actions
    // ============================================================

    fun open_dispute_as(sc: &mut Scenario, tfx: &tt::Fixture, sender: address, task_id: ID, pay: u64) {
        ts::next_tx(sc, sender);
        let config = ts::take_shared<TaskConfig>(sc);
        let jury_config = ts::take_shared<JuryConfig>(sc);
        let roster = ts::take_shared<JurorRoster>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, tt::buyer_account(tfx));
        let seller_account = ts::take_shared_by_id<MemoryAccount>(sc, tt::seller_account(tfx));
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, tt::seller_agent(tfx));
        let clock = ts::take_shared<Clock>(sc);
        let fee = coin::mint_for_testing<MYSO>(pay, ts::ctx(sc));
        agent_task::open_dispute(
            &config, &jury_config, &roster, &mut task, &memory_config, &buyer_account,
            &seller_account, &seller_agent, fee, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(seller_account);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(task);
        ts::return_shared(roster);
        ts::return_shared(jury_config);
        ts::return_shared(config);
    }

    fun respond_as(sc: &mut Scenario, tfx: &tt::Fixture, sender: address, task_id: ID, case_id: ID, pay: u64) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, tt::buyer_account(tfx));
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, tt::seller_agent(tfx));
        let clock = ts::take_shared<Clock>(sc);
        let fee = coin::mint_for_testing<MYSO>(pay, ts::ctx(sc));
        agent_task::respond_dispute(&mut task, &mut case, &memory_config, &buyer_account, &seller_agent, fee, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(case);
        ts::return_shared(task);
    }

    fun evidence_as(sc: &mut Scenario, tfx: &tt::Fixture, sender: address, task_id: ID, case_id: ID, digest: vector<u8>) {
        ts::next_tx(sc, sender);
        let jury_config = ts::take_shared<JuryConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, tt::buyer_account(tfx));
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, tt::seller_agent(tfx));
        let clock = ts::take_shared<Clock>(sc);
        agent_task::submit_dispute_evidence(
            &jury_config, &mut task, &mut case, &memory_config, &buyer_account, &seller_agent,
            digest, b"ipfs://party-evidence", &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(case);
        ts::return_shared(task);
        ts::return_shared(jury_config);
    }

    fun appeal_as(sc: &mut Scenario, tfx: &tt::Fixture, sender: address, task_id: ID, case_id: ID, pay: u64) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, tt::buyer_account(tfx));
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, tt::seller_agent(tfx));
        let clock = ts::take_shared<Clock>(sc);
        let fee = coin::mint_for_testing<MYSO>(pay, ts::ctx(sc));
        agent_task::appeal_dispute(&mut task, &mut case, &memory_config, &buyer_account, &seller_agent, fee, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(case);
        ts::return_shared(task);
    }

    fun cancel_as(sc: &mut Scenario, tfx: &tt::Fixture, sender: address, task_id: ID, case_id: ID) {
        ts::next_tx(sc, sender);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let memory_config = ts::take_shared<MemoryConfig>(sc);
        let buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, tt::buyer_account(tfx));
        let seller_agent = ts::take_shared_by_id<SubAgent>(sc, tt::seller_agent(tfx));
        let clock = ts::take_shared<Clock>(sc);
        agent_task::cancel_dispute_before_draw(&mut task, &mut case, &memory_config, &buyer_account, &seller_agent, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(seller_agent);
        ts::return_shared(buyer_account);
        ts::return_shared(memory_config);
        ts::return_shared(case);
        ts::return_shared(task);
    }

    fun verdict_as(sc: &mut Scenario, tfx: &tt::Fixture, task_id: ID, case_id: ID) {
        ts::next_tx(sc, tt::admin());
        let config = ts::take_shared<TaskConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let mut buyer_account = ts::take_shared_by_id<MemoryAccount>(sc, tt::buyer_account(tfx));
        let mut seller_account = ts::take_shared_by_id<MemoryAccount>(sc, tt::seller_account(tfx));
        let mut platform_obj = ts::take_shared_by_id<Platform>(sc, object::id_from_address(tt::platform_address(tfx)));
        let clock = ts::take_shared<Clock>(sc);
        agent_task::execute_verdict(
            &config, &mut task, &mut case, &mut buyer_account, &mut seller_account,
            &mut platform_obj, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(platform_obj);
        ts::return_shared(seller_account);
        ts::return_shared(buyer_account);
        ts::return_shared(case);
        ts::return_shared(task);
        ts::return_shared(config);
    }

    fun start_neutral_as(sc: &mut Scenario, task_id: ID, case_id: ID) {
        ts::next_tx(sc, tt::admin());
        let jury_config = ts::take_shared<JuryConfig>(sc);
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::start_neutral_resolution(&jury_config, &mut task, &mut case, &clock);
        ts::return_shared(clock);
        ts::return_shared(case);
        ts::return_shared(task);
        ts::return_shared(jury_config);
    }

    fun close_case_as(sc: &mut Scenario, task_id: ID, case_id: ID) {
        ts::next_tx(sc, tt::admin());
        let jury_config = ts::take_shared<JuryConfig>(sc);
        let mut roster = ts::take_shared<JurorRoster>(sc);
        let task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::close_case_after_resolution(&jury_config, &mut roster, &task, &mut case, &clock);
        ts::return_shared(clock);
        ts::return_shared(case);
        ts::return_shared(task);
        ts::return_shared(roster);
        ts::return_shared(jury_config);
    }

    fun settle_fees_as(sc: &mut Scenario, task_id: ID, case_id: ID) {
        ts::next_tx(sc, tt::admin());
        let mut task = ts::take_shared_by_id<TaskEscrow>(sc, task_id);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let clock = ts::take_shared<Clock>(sc);
        agent_task::settle_dispute_fees(&mut task, &mut case, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(case);
        ts::return_shared(task);
    }

    /// Delivered, disputed by the buyer, and the seller matched the fee.
    fun contested(sc: &mut Scenario, tfx: &tt::Fixture): (ID, ID) {
        let task_id = tt::delivered_task(sc, tfx, ESCROW);
        open_dispute_as(sc, tfx, tt::buyer(), task_id, FEE);
        let case_id = case_of(sc);
        respond_as(sc, tfx, tt::seller(), task_id, case_id, FEE);
        (task_id, case_id)
    }

    /// Past the evidence window, a full round of three, and the appeal window left to lapse.
    fun decide_unappealed(sc: &mut Scenario, jfx: &jt::Fixture, case_id: ID, outcomes: vector<u8>) {
        tt::advance(sc, 3 * tt::day());
        jt::run_round(sc, jfx, case_id, outcomes);
        tt::advance(sc, 49 * tt::hour());
        jt::do_finalize_case(sc, case_id);
    }

    fun claim_all(sc: &mut Scenario, jfx: &jt::Fixture, case_id: ID, expect_each: u64) {
        let jurors = jt::jurors_of(sc, case_id);
        let mut k = 0;
        while (k < vector::length(&jurors)) {
            let id = *vector::borrow(&jurors, k);
            jt::do_claim(sc, case_id, id);
            assert!(tt::coin_total(sc, jt::owner(jt::index_of(jfx, id))) == expect_each, 900);
            k = k + 1;
        };
    }

    // ============================================================
    // Opening
    // ============================================================

    #[test]
    fun opening_a_dispute_freezes_the_task_attaches_the_facts_and_returns_change() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, jfx) = setup(&mut sc, 6);
        let task_id = tt::delivered_task(&mut sc, &tfx, ESCROW);
        open_dispute_as(&mut sc, &tfx, tt::buyer(), task_id, 1_000_000_000);
        let case_id = case_of(&mut sc);

        let task = tt::take_task(&mut sc, task_id);
        assert!(agent_task::status(&task) == agent_task::status_disputed(), 0);
        assert!(agent_task::dispute_fee_each_mist(&task) == FEE, 1);
        assert!(agent_task::dispute_buyer_paid_mist(&task) == FEE, 2);
        assert!(agent_task::dispute_seller_paid_mist(&task) == 0, 3);
        assert!(agent_task::dispute_case_id(&task) == option::some(case_id), 4);
        assert!(agent_task::escrowed_mist(&task) == ESCROW, 5);
        ts::return_shared(task);

        ts::next_tx(&mut sc, tt::admin());
        let case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
        // Terms, scope and deliverable digests, attached by the contract rather than a party.
        assert!(jury::case_evidence_count(&case) == 3, 6);
        assert!(jury::case_fee_pool_mist(&case) == FEE, 7);
        assert!(jury::case_state(&case) == jury::case_evidence(), 8);
        assert!(jury::case_subject_id(&case) == task_id, 9);
        assert!(jury::case_subject_kind(&case) == jury::subject_task(), 11);
        ts::return_shared(case);
        // The rest of what the buyer put in came back as change.
        assert!(tt::coin_total(&mut sc, tt::buyer()) == 700_000_000, 10);
        ts::end(sc);
    }

    #[test]
    fun the_case_fee_is_a_bounded_share_of_the_escrow() {
        assert!(agent_task::dispute_fee_mist(1) == 100_000_000, 0);
        assert!(agent_task::dispute_fee_mist(ESCROW) == FEE, 1);
        assert!(agent_task::dispute_fee_mist(10_000_000_000_000) == 50_000_000_000, 2);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ERosterTooSmall)]
    fun a_dispute_cannot_open_while_the_roster_is_too_thin() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup_with(&mut sc, 6, 0);
        let task_id = tt::delivered_task(&mut sc, &tfx, ESCROW);
        open_dispute_as(&mut sc, &tfx, tt::buyer(), task_id, FEE);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInsufficientPayment)]
    fun the_opener_must_cover_the_case_fee() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let task_id = tt::delivered_task(&mut sc, &tfx, ESCROW);
        open_dispute_as(&mut sc, &tfx, tt::buyer(), task_id, FEE - 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun only_a_delivered_task_can_go_to_a_jury() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        tt::offer(&mut sc, &tfx, tt::buyer(), ESCROW);
        let task_id = tt::latest_task();
        tt::accept_ok(&mut sc, &tfx, task_id);
        open_dispute_as(&mut sc, &tfx, tt::buyer(), task_id, FEE);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EDisputeWindowClosed)]
    fun a_dispute_cannot_open_once_the_seller_could_already_be_paid() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let task_id = tt::delivered_task(&mut sc, &tfx, ESCROW);
        tt::advance(&mut sc, 4 * tt::day() + 1);
        open_dispute_as(&mut sc, &tfx, tt::buyer(), task_id, FEE);
        ts::end(sc);
    }

    #[test]
    fun the_seller_can_open_a_dispute_too() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let task_id = tt::delivered_task(&mut sc, &tfx, ESCROW);
        open_dispute_as(&mut sc, &tfx, tt::seller(), task_id, FEE);
        let task = tt::take_task(&mut sc, task_id);
        assert!(agent_task::dispute_seller_paid_mist(&task) == FEE, 0);
        assert!(agent_task::dispute_buyer_paid_mist(&task) == 0, 1);
        ts::return_shared(task);
        ts::end(sc);
    }

    // ============================================================
    // Matching the fee, evidence
    // ============================================================

    #[test]
    fun the_other_side_matches_the_fee_before_the_draw() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        let task = tt::take_task(&mut sc, task_id);
        assert!(agent_task::dispute_seller_paid_mist(&task) == FEE, 0);
        ts::return_shared(task);
        ts::next_tx(&mut sc, tt::admin());
        let case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
        assert!(jury::case_fee_pool_mist(&case) == 2 * FEE, 1);
        ts::return_shared(case);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENotCounterparty)]
    fun the_opener_cannot_match_its_own_fee() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let task_id = tt::delivered_task(&mut sc, &tfx, ESCROW);
        open_dispute_as(&mut sc, &tfx, tt::buyer(), task_id, FEE);
        let case_id = case_of(&mut sc);
        respond_as(&mut sc, &tfx, tt::buyer(), task_id, case_id, FEE);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EAlreadyPaid)]
    fun the_fee_can_be_matched_only_once() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        respond_as(&mut sc, &tfx, tt::seller(), task_id, case_id, FEE);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ECaseDrawn)]
    fun the_fee_cannot_be_matched_once_jurors_are_drawn() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let task_id = tt::delivered_task(&mut sc, &tfx, ESCROW);
        open_dispute_as(&mut sc, &tfx, tt::buyer(), task_id, FEE);
        let case_id = case_of(&mut sc);
        tt::advance(&mut sc, 3 * tt::day());
        jt::do_select(&mut sc, case_id);
        respond_as(&mut sc, &tfx, tt::seller(), task_id, case_id, FEE);
        ts::end(sc);
    }

    #[test]
    fun a_party_adds_evidence_and_each_side_has_its_own_cap() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        evidence_as(&mut sc, &tfx, tt::seller(), task_id, case_id, jt::digest(5));
        evidence_as(&mut sc, &tfx, tt::buyer(), task_id, case_id, jt::digest(6));
        ts::next_tx(&mut sc, tt::admin());
        let case = ts::take_shared_by_id<JuryCase>(&sc, case_id);
        assert!(jury::case_evidence_count(&case) == 5, 0);
        ts::return_shared(case);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::ETooMuchEvidence)]
    fun one_party_cannot_flood_the_case() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        let mut i = 0u64;
        while (i < 9) {
            evidence_as(&mut sc, &tfx, tt::seller(), task_id, case_id, jt::digest(5));
            i = i + 1;
        };
        ts::end(sc);
    }

    // ============================================================
    // Verdicts and the fee rule
    // ============================================================

    #[test]
    fun a_verdict_for_the_seller_pays_the_seller_and_the_loser_bears_the_fee() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        decide_unappealed(&mut sc, &jfx, case_id, vector[0, 0, 0]);
        verdict_as(&mut sc, &tfx, task_id, case_id);

        let task = tt::take_task(&mut sc, task_id);
        assert!(agent_task::status(&task) == agent_task::status_settled(), 0);
        assert!(agent_task::settlement_kind(&task) == 3, 1);
        ts::return_shared(task);
        assert!(tt::coin_total(&mut sc, tt::seller()) == SELLER_NET, 2);

        settle_fees_as(&mut sc, task_id, case_id);
        // Jurors took 3 x 0.1 MYSO; the other 0.3 MYSO of the 0.6 pool is the winner's own fee back.
        assert!(tt::coin_total(&mut sc, tt::seller()) == SELLER_NET + FEE, 3);
        assert!(tt::coin_total(&mut sc, tt::buyer()) == 0, 4);
        claim_all(&mut sc, &jfx, case_id, 100_000_000);
        ts::end(sc);
    }

    #[test]
    fun a_verdict_for_the_buyer_refunds_the_escrow_and_the_fee() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        decide_unappealed(&mut sc, &jfx, case_id, vector[1, 1, 1]);
        verdict_as(&mut sc, &tfx, task_id, case_id);
        assert!(tt::status_of(&mut sc, task_id) == agent_task::status_refunded(), 0);
        settle_fees_as(&mut sc, task_id, case_id);
        assert!(tt::coin_total(&mut sc, tt::buyer()) == ESCROW + FEE, 1);
        assert!(tt::coin_total(&mut sc, tt::seller()) == 0, 2);
        ts::end(sc);
    }

    #[test]
    fun a_split_verdict_pays_each_side_half_and_shares_the_juror_cost() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        decide_unappealed(&mut sc, &jfx, case_id, vector[2, 2, 2]);
        verdict_as(&mut sc, &tfx, task_id, case_id);
        let task = tt::take_task(&mut sc, task_id);
        assert!(agent_task::settlement_kind(&task) == 5, 0);
        assert!(agent_task::review_deadline_ms(&task) == 0, 1);
        ts::return_shared(task);
        settle_fees_as(&mut sc, task_id, case_id);
        // No loser: the 0.3 MYSO left over after the jurors comes back 0.15 each.
        assert!(tt::coin_total(&mut sc, tt::buyer()) == ESCROW / 2 + 150_000_000, 2);
        assert!(tt::coin_total(&mut sc, tt::seller()) == SELLER_HALF_NET + 150_000_000, 3);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENoVerdict)]
    fun a_verdict_cannot_run_before_the_case_is_final() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        verdict_as(&mut sc, &tfx, task_id, case_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun a_verdict_runs_once() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        decide_unappealed(&mut sc, &jfx, case_id, vector[0, 0, 0]);
        verdict_as(&mut sc, &tfx, task_id, case_id);
        verdict_as(&mut sc, &tfx, task_id, case_id);
        ts::end(sc);
    }

    // ============================================================
    // Appeals
    // ============================================================

    #[test]
    fun the_losing_side_appeals_to_a_larger_jury_that_overturns_and_the_loser_pays() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, jfx) = setup(&mut sc, 12);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        tt::advance(&mut sc, 3 * tt::day());
        jt::run_round(&mut sc, &jfx, case_id, vector[0, 0, 0]);
        assert!(jt::state_of(&mut sc, case_id) == jury::case_tallied(), 0);

        // The buyer lost, so the buyer appeals, for twice the case fee.
        appeal_as(&mut sc, &tfx, tt::buyer(), task_id, case_id, 2 * FEE);
        assert!(jt::state_of(&mut sc, case_id) == jury::case_awaiting_draw(), 1);
        jt::run_round(&mut sc, &jfx, case_id, vector[1, 1, 1, 1, 1]);
        assert!(jt::state_of(&mut sc, case_id) == jury::case_final(), 2);

        verdict_as(&mut sc, &tfx, task_id, case_id);
        assert!(tt::status_of(&mut sc, task_id) == agent_task::status_refunded(), 3);
        settle_fees_as(&mut sc, task_id, case_id);
        // Pool 1.2 MYSO, eight jurors at 0.1 = 0.8; the remaining 0.4 is the winning buyer's fees back.
        assert!(tt::coin_total(&mut sc, tt::buyer()) == ESCROW + 400_000_000, 4);
        assert!(tt::coin_total(&mut sc, tt::seller()) == 0, 5);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENotAppellant)]
    fun the_winning_side_cannot_appeal() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, jfx) = setup(&mut sc, 12);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        tt::advance(&mut sc, 3 * tt::day());
        jt::run_round(&mut sc, &jfx, case_id, vector[0, 0, 0]);
        appeal_as(&mut sc, &tfx, tt::seller(), task_id, case_id, 2 * FEE);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EInsufficientPayment)]
    fun an_appeal_costs_twice_the_case_fee() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, jfx) = setup(&mut sc, 12);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        tt::advance(&mut sc, 3 * tt::day());
        jt::run_round(&mut sc, &jfx, case_id, vector[0, 0, 0]);
        appeal_as(&mut sc, &tfx, tt::buyer(), task_id, case_id, FEE);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENotTallied)]
    fun there_is_nothing_to_appeal_before_a_verdict() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 12);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        appeal_as(&mut sc, &tfx, tt::buyer(), task_id, case_id, 2 * FEE);
        ts::end(sc);
    }

    // ============================================================
    // A jury that fails: nobody wins by default
    // ============================================================

    #[test]
    fun a_failed_jury_ends_in_a_neutral_split_and_the_juror_cost_is_shared() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, jfx) = setup(&mut sc, 3);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        tt::advance(&mut sc, 3 * tt::day());
        jt::do_select(&mut sc, case_id);
        // Nobody accepts. With three owners there is no one left to replace them.
        let mut calls = 0u64;
        while (calls < 5) {
            tt::advance(&mut sc, 13 * tt::hour());
            if (jt::state_of(&mut sc, case_id) == jury::case_selecting()) {
                jt::do_replace(&mut sc, case_id);
            };
            calls = calls + 1;
        };
        assert!(jt::state_of(&mut sc, case_id) == jury::case_failed(), 0);

        start_neutral_as(&mut sc, task_id, case_id);
        assert!(tt::status_of(&mut sc, task_id) == agent_task::status_disputed(), 1);
        tt::advance(&mut sc, 73 * tt::hour());
        tt::execute_neutral(&mut sc, &tfx, @0xCAFE, task_id);

        let task = tt::take_task(&mut sc, task_id);
        assert!(agent_task::settlement_kind(&task) == 7, 2);
        ts::return_shared(task);
        settle_fees_as(&mut sc, task_id, case_id);
        // Nobody sat, so both fees come back in full.
        assert!(tt::coin_total(&mut sc, tt::buyer()) == ESCROW / 2 + FEE, 3);
        assert!(tt::coin_total(&mut sc, tt::seller()) == SELLER_HALF_NET + FEE, 4);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ECaseNotFailed)]
    fun the_settlement_window_needs_a_failed_jury() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        start_neutral_as(&mut sc, task_id, case_id);
        ts::end(sc);
    }

    // ============================================================
    // Cancel and agreement
    // ============================================================

    #[test]
    fun the_opener_can_withdraw_before_the_draw_and_gets_the_fee_back() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let task_id = tt::delivered_task(&mut sc, &tfx, ESCROW);
        open_dispute_as(&mut sc, &tfx, tt::buyer(), task_id, FEE);
        let case_id = case_of(&mut sc);
        cancel_as(&mut sc, &tfx, tt::buyer(), task_id, case_id);
        assert!(tt::status_of(&mut sc, task_id) == agent_task::status_delivered(), 0);
        settle_fees_as(&mut sc, task_id, case_id);
        assert!(tt::coin_total(&mut sc, tt::buyer()) == FEE, 1);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ENotInitiator)]
    fun only_the_opener_can_withdraw() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        cancel_as(&mut sc, &tfx, tt::seller(), task_id, case_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::jury::EWrongCaseState)]
    fun a_dispute_cannot_be_withdrawn_once_jurors_are_drawn() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        tt::advance(&mut sc, 3 * tt::day());
        jt::do_select(&mut sc, case_id);
        cancel_as(&mut sc, &tfx, tt::buyer(), task_id, case_id);
        ts::end(sc);
    }

    #[test]
    fun parties_who_agree_mid_dispute_close_the_case_and_split_the_fees_by_what_they_paid() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        tt::propose_settlement(&mut sc, &tfx, tt::buyer(), task_id, 1);
        tt::accept_settlement(&mut sc, &tfx, tt::seller(), task_id);
        assert!(tt::status_of(&mut sc, task_id) == agent_task::status_refunded(), 0);

        close_case_as(&mut sc, task_id, case_id);
        settle_fees_as(&mut sc, task_id, case_id);
        // No jury ever sat and neither side is a "loser": each gets its own fee back.
        assert!(tt::coin_total(&mut sc, tt::buyer()) == ESCROW + FEE, 1);
        assert!(tt::coin_total(&mut sc, tt::seller()) == FEE, 2);
        ts::end(sc);
    }

    // ============================================================
    // Fee settlement guards
    // ============================================================

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EWrongStatus)]
    fun fees_wait_until_the_dispute_is_over() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        settle_fees_as(&mut sc, task_id, case_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::ECaseNotFinished)]
    fun fees_wait_until_the_jury_case_is_finished() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, _jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        tt::propose_settlement(&mut sc, &tfx, tt::buyer(), task_id, 1);
        tt::accept_settlement(&mut sc, &tfx, tt::seller(), task_id);
        settle_fees_as(&mut sc, task_id, case_id);
        ts::end(sc);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::agent_task::EFeesSettled)]
    fun fees_settle_once() {
        let mut sc = ts::begin(tt::admin());
        let (tfx, jfx) = setup(&mut sc, 6);
        let (task_id, case_id) = contested(&mut sc, &tfx);
        decide_unappealed(&mut sc, &jfx, case_id, vector[0, 0, 0]);
        verdict_as(&mut sc, &tfx, task_id, case_id);
        settle_fees_as(&mut sc, task_id, case_id);
        settle_fees_as(&mut sc, task_id, case_id);
        ts::end(sc);
    }
}
