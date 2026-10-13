// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// The AI jury behind each human governance step: the delegate council (referral, tie, deadline) and
/// the community vote (no quorum). Uses an ecosystem registry with three delegates (the founder and
/// two added in the test), a proposal pool of 10_000 and a jury fee of 25% of it, and the juror
/// roster from `jury_tests`.
#[test_only]
#[allow(duplicate_alias, unused_use, unused_const, unused_variable)]
module social_contracts::governance_jury_tests {
    use std::option::{Self, Option};
    use std::string;
    use std::vector;

    use myso::clock::{Self, Clock};
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::object::{Self, ID};
    use myso::test_scenario::{Self as ts, Scenario};

    use social_contracts::governance::{Self, GovernanceAdminCap, GovernanceDAO, Proposal};
    use social_contracts::jury::{Self, JuryCase, JuryConfig};
    use social_contracts::jury_tests::{Self as jt};
    use social_contracts::profile::EcosystemTreasury;

    const DELEGATE_2: address = @0xD2;
    const DELEGATE_3: address = @0xD3;
    const SUBMITTER: address = @0x5B;
    const ANYONE: address = @0xEE;
    const POOL: u64 = 10_000;
    const FEE: u64 = 2_500;

    const STATUS_DELEGATE_REVIEW: u8 = 1;
    const STATUS_COMMUNITY_VOTING: u8 = 2;
    const STATUS_APPROVED: u8 = 3;
    const STATUS_REJECTED: u8 = 4;

    const STATE_REFERRING: u8 = 1;
    const STATE_AWAITING: u8 = 2;
    const STATE_ESCALATED: u8 = 3;
    const STATE_DECIDED: u8 = 4;

    // ============================================================
    // Fixture
    // ============================================================

    /// Jury objects and `jurors` roster members, then governance, three delegates, a submitter with
    /// a profile and one proposal in delegate review. `policy` switches the jury on for both steps.
    fun setup(sc: &mut Scenario, jurors: u64, policy: bool): jt::Fixture {
        let fx = jt::setup(sc, jurors);
        jt::fill_roster(sc, &fx, jurors);
        jt::set_min_roster(sc, 3);
        jt::create_profile(sc, SUBMITTER, b"submitter");

        ts::next_tx(sc, jt::admin());
        {
            let clock = ts::take_shared<Clock>(sc);
            let ctx = ts::ctx(sc);
            let _ids = governance::bootstrap_init(&clock, ctx);
            governance::test_grant_admin_cap(ctx);
            ts::return_shared(clock);
        };
        ts::next_tx(sc, jt::admin());
        {
            let cap = ts::take_from_sender<GovernanceAdminCap>(sc);
            let mut registry = take_registry(sc);
            let clock = ts::take_shared<Clock>(sc);
            governance::update_governance_parameters(
                &mut registry, &cap, 3, 90, 10_000, 5, 0, 1_000, 2, &clock, ts::ctx(sc),
            );
            governance::test_add_delegate(&mut registry, DELEGATE_2);
            governance::test_add_delegate(&mut registry, DELEGATE_3);
            if (policy) {
                governance::set_registry_jury_policy(
                    &cap, &mut registry, true, true, 3 * jt::day(), 5, 5, 2500, 0, 1_000, 2, 5000, 6667, &clock,
                );
            };
            ts::return_shared(clock);
            ts::return_shared(registry);
            ts::return_to_sender(sc, cap);
        };
        submit(sc);
        fx
    }

    /// The ecosystem registry (type 0) among the three bootstrap registries.
    fun take_registry(sc: &Scenario): GovernanceDAO {
        let r0 = ts::take_shared<GovernanceDAO>(sc);
        if (governance::registry_type(&r0) == 0) {
            return r0
        };
        let r1 = ts::take_shared<GovernanceDAO>(sc);
        if (governance::registry_type(&r1) == 0) {
            ts::return_shared(r0);
            return r1
        };
        let r2 = ts::take_shared<GovernanceDAO>(sc);
        if (governance::registry_type(&r2) == 0) {
            ts::return_shared(r0);
            ts::return_shared(r1);
            return r2
        };
        abort 999
    }

    fun submit(sc: &mut Scenario) {
        ts::next_tx(sc, SUBMITTER);
        let mut registry = take_registry(sc);
        let clock = ts::take_shared<Clock>(sc);
        let mut payment = coin::mint_for_testing<MYSO>(POOL, ts::ctx(sc));
        governance::submit_ecosystem_proposal(
            &mut registry,
            string::utf8(b"Raise the cap"),
            string::utf8(b"A proposal worth judging"),
            option::none(),
            option::none(),
            &mut payment,
            &clock,
            ts::ctx(sc),
        );
        coin::burn_for_testing(payment);
        ts::return_shared(clock);
        ts::return_shared(registry);
    }

    // ============================================================
    // Actions
    // ============================================================

    fun council_vote(sc: &mut Scenario, who: address, approve: bool) {
        ts::next_tx(sc, who);
        let mut registry = take_registry(sc);
        let mut proposal = ts::take_shared<Proposal>(sc);
        let treasury = ts::take_shared<EcosystemTreasury>(sc);
        let clock = ts::take_shared<Clock>(sc);
        governance::delegate_vote_on_proposal(
            &mut registry, &mut proposal, &treasury, approve, option::none(), &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(treasury);
        ts::return_shared(proposal);
        ts::return_shared(registry);
    }

    fun refer(sc: &mut Scenario, who: address) {
        ts::next_tx(sc, who);
        let mut registry = take_registry(sc);
        let mut proposal = ts::take_shared<Proposal>(sc);
        let clock = ts::take_shared<Clock>(sc);
        governance::delegate_refer_to_jury(
            &mut registry, &mut proposal, option::some(string::utf8(b"too big for us")), &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(proposal);
        ts::return_shared(registry);
    }

    fun escalate(sc: &mut Scenario, stage: u8) {
        ts::next_tx(sc, ANYONE);
        let mut registry = take_registry(sc);
        let mut proposal = ts::take_shared<Proposal>(sc);
        let config = ts::take_shared<JuryConfig>(sc);
        let treasury = ts::take_shared<EcosystemTreasury>(sc);
        let clock = ts::take_shared<Clock>(sc);
        governance::escalate_to_jury(
            &mut registry, &mut proposal, &config, &treasury, stage, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(treasury);
        ts::return_shared(config);
        ts::return_shared(proposal);
        ts::return_shared(registry);
    }

    fun finalize(sc: &mut Scenario) {
        ts::next_tx(sc, ANYONE);
        let mut registry = take_registry(sc);
        let mut proposal = ts::take_shared<Proposal>(sc);
        let treasury = ts::take_shared<EcosystemTreasury>(sc);
        let clock = ts::take_shared<Clock>(sc);
        governance::finalize_proposal(&mut registry, &mut proposal, &treasury, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(treasury);
        ts::return_shared(proposal);
        ts::return_shared(registry);
    }

    fun apply(sc: &mut Scenario, case_id: ID) {
        ts::next_tx(sc, ANYONE);
        let mut registry = take_registry(sc);
        let mut proposal = ts::take_shared<Proposal>(sc);
        let mut case = ts::take_shared_by_id<JuryCase>(sc, case_id);
        let treasury = ts::take_shared<EcosystemTreasury>(sc);
        let clock = ts::take_shared<Clock>(sc);
        governance::apply_jury_decision(
            &mut registry, &mut proposal, &mut case, &treasury, &clock, ts::ctx(sc),
        );
        ts::return_shared(clock);
        ts::return_shared(treasury);
        ts::return_shared(case);
        ts::return_shared(proposal);
        ts::return_shared(registry);
    }

    fun reject_manually(sc: &mut Scenario, who: address) {
        ts::next_tx(sc, who);
        let mut registry = take_registry(sc);
        let mut proposal = ts::take_shared<Proposal>(sc);
        let treasury = ts::take_shared<EcosystemTreasury>(sc);
        let clock = ts::take_shared<Clock>(sc);
        governance::reject_proposal_manually(&mut registry, &mut proposal, &treasury, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(treasury);
        ts::return_shared(proposal);
        ts::return_shared(registry);
    }

    fun rescind(sc: &mut Scenario) {
        ts::next_tx(sc, SUBMITTER);
        let mut registry = take_registry(sc);
        let mut proposal = ts::take_shared<Proposal>(sc);
        let clock = ts::take_shared<Clock>(sc);
        governance::rescind_proposal(&mut registry, &mut proposal, &clock, ts::ctx(sc));
        ts::return_shared(clock);
        ts::return_shared(proposal);
        ts::return_shared(registry);
    }

    fun case_of(sc: &mut Scenario): ID {
        ts::next_tx(sc, jt::admin());
        ts::most_recent_id_shared<JuryCase>().destroy_some()
    }

    fun status(sc: &mut Scenario): u8 {
        ts::next_tx(sc, jt::admin());
        let proposal = ts::take_shared<Proposal>(sc);
        let s = governance::proposal_status(&proposal);
        ts::return_shared(proposal);
        s
    }

    fun pool(sc: &mut Scenario): u64 {
        ts::next_tx(sc, jt::admin());
        let proposal = ts::take_shared<Proposal>(sc);
        let p = governance::proposal_pool_value(&proposal);
        ts::return_shared(proposal);
        p
    }

    /// The council record state and the community record state as one number (`both(council, community)`).
    fun states(sc: &mut Scenario): u64 {
        ts::next_tx(sc, jt::admin());
        let proposal = ts::take_shared<Proposal>(sc);
        let s = both(governance::jury_council_state(&proposal), governance::jury_community_state(&proposal));
        ts::return_shared(proposal);
        s
    }

    fun both(council: u8, community: u8): u64 { (council as u64) * 10 + (community as u64) }

    /// Both founders approve, which moves the proposal to the community vote.
    fun open_community_vote(sc: &mut Scenario) {
        council_vote(sc, jt::admin(), true);
        council_vote(sc, DELEGATE_2, true);
        assert!(status(sc) == STATUS_COMMUNITY_VOTING, 100);
    }

    /// Ends the (empty) community vote and moves to the jury.
    fun end_vote_without_quorum(sc: &mut Scenario) {
        jt::advance(sc, 2_000);
        finalize(sc);
    }

    // ============================================================
    // Council step
    // ============================================================

    #[test]
    fun a_referral_majority_sends_the_council_step_to_a_jury_that_can_advance_it() {
        let mut sc = ts::begin(jt::admin());
        let fx = setup(&mut sc, 12, true);
        refer(&mut sc, DELEGATE_2);
        assert!(states(&mut sc) == both(STATE_REFERRING, 0), 0);
        refer(&mut sc, DELEGATE_3);

        escalate(&mut sc, 1);
        let case_id = case_of(&mut sc);
        assert!(states(&mut sc) == both(STATE_ESCALATED, 0), 1);
        assert!(pool(&mut sc) == POOL - FEE, 2);
        assert!(status(&mut sc) == STATUS_DELEGATE_REVIEW, 3);

        jt::run_round(&mut sc, &fx, case_id, vector[0, 0, 0, 0, 0]);
        apply(&mut sc, case_id);

        assert!(status(&mut sc) == STATUS_COMMUNITY_VOTING, 4);
        assert!(states(&mut sc) == both(STATE_DECIDED, 0), 5);
        // Referring does not touch the delegates' vote record.
        assert!(pool(&mut sc) == POOL - FEE, 6);
        ts::end(sc);
    }

    #[test]
    fun a_jury_rejection_at_the_council_step_forfeits_the_pool_to_the_treasury() {
        let mut sc = ts::begin(jt::admin());
        let fx = setup(&mut sc, 12, true);
        refer(&mut sc, DELEGATE_2);
        refer(&mut sc, DELEGATE_3);
        escalate(&mut sc, 1);
        let case_id = case_of(&mut sc);
        jt::run_round(&mut sc, &fx, case_id, vector[1, 1, 1, 1, 1]);
        let before = jt::coin_total(&mut sc, jt::admin());
        apply(&mut sc, case_id);

        assert!(status(&mut sc) == STATUS_REJECTED, 0);
        assert!(pool(&mut sc) == 0, 1);
        assert!(jt::coin_total(&mut sc, jt::admin()) == before + (POOL - FEE), 2);
        assert!(jt::coin_total(&mut sc, SUBMITTER) == 0, 3);
        ts::end(sc);
    }

    #[test]
    fun a_failed_jury_at_the_council_step_returns_the_decision_to_the_humans() {
        let mut sc = ts::begin(jt::admin());
        let fx = setup(&mut sc, 12, true);
        refer(&mut sc, DELEGATE_2);
        refer(&mut sc, DELEGATE_3);
        escalate(&mut sc, 1);
        let case_id = case_of(&mut sc);
        jt::do_void(&mut sc, case_id);
        apply(&mut sc, case_id);

        // The council deadlocked into the community vote rather than being rejected.
        assert!(status(&mut sc) == STATUS_COMMUNITY_VOTING, 0);
        assert!(states(&mut sc) == both(STATE_DECIDED, 0), 1);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::ENoEscalationTrigger)]
    fun one_referral_is_not_a_majority() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        refer(&mut sc, DELEGATE_2);
        escalate(&mut sc, 1);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::ENotDelegate)]
    fun only_a_delegate_can_refer() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        refer(&mut sc, ANYONE);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::EAlreadyReferred)]
    fun a_delegate_cannot_refer_twice() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        refer(&mut sc, DELEGATE_2);
        refer(&mut sc, DELEGATE_2);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::EDelegateAlreadyActed)]
    fun a_delegate_who_voted_cannot_refer() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        council_vote(&mut sc, DELEGATE_2, false);
        refer(&mut sc, DELEGATE_2);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::EJuryStageDisabled)]
    fun a_registry_without_a_policy_has_no_jury_to_refer_to() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, false);
        refer(&mut sc, DELEGATE_2);
        ts::end(sc);
    }

    #[test]
    fun a_council_that_has_all_acted_without_a_majority_can_go_to_the_jury() {
        let mut sc = ts::begin(jt::admin());
        let fx = setup(&mut sc, 12, true);
        council_vote(&mut sc, jt::admin(), true);
        council_vote(&mut sc, DELEGATE_2, false);
        refer(&mut sc, DELEGATE_3);

        escalate(&mut sc, 1);
        assert!(states(&mut sc) == both(STATE_ESCALATED, 0), 0);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::ENoEscalationTrigger)]
    fun a_council_that_has_not_finished_voting_cannot_be_escalated_early() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        council_vote(&mut sc, jt::admin(), true);
        council_vote(&mut sc, DELEGATE_2, false);
        escalate(&mut sc, 1);
        ts::end(sc);
    }

    #[test]
    fun the_council_window_running_out_lets_anyone_escalate() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        jt::advance(&mut sc, 3 * jt::day() + 1);
        escalate(&mut sc, 1);
        assert!(states(&mut sc) == both(STATE_ESCALATED, 0), 0);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::ENoEscalationTrigger)]
    fun before_the_window_ends_nothing_allows_escalation() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        jt::advance(&mut sc, jt::day());
        escalate(&mut sc, 1);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::EJuryHasJurisdiction)]
    fun delegates_cannot_vote_once_a_jury_has_the_proposal() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        jt::advance(&mut sc, 3 * jt::day() + 1);
        escalate(&mut sc, 1);
        council_vote(&mut sc, jt::admin(), false);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::EJuryHasJurisdiction)]
    fun a_delegate_cannot_reject_manually_once_a_jury_has_the_proposal() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        jt::advance(&mut sc, 3 * jt::day() + 1);
        escalate(&mut sc, 1);
        reject_manually(&mut sc, jt::admin());
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::EJuryHasJurisdiction)]
    fun the_submitter_cannot_withdraw_the_pool_once_a_jury_has_the_proposal() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        jt::advance(&mut sc, 3 * jt::day() + 1);
        escalate(&mut sc, 1);
        rescind(&mut sc);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::EJuryHasJurisdiction)]
    fun a_proposal_cannot_go_to_a_jury_twice() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        jt::advance(&mut sc, 3 * jt::day() + 1);
        escalate(&mut sc, 1);
        escalate(&mut sc, 1);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::EJuryNotFinished)]
    fun a_decision_cannot_be_applied_before_the_jury_finishes() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        jt::advance(&mut sc, 3 * jt::day() + 1);
        escalate(&mut sc, 1);
        let case_id = case_of(&mut sc);
        apply(&mut sc, case_id);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::ENoJuryRecord)]
    fun a_decision_applies_once() {
        let mut sc = ts::begin(jt::admin());
        let fx = setup(&mut sc, 12, true);
        refer(&mut sc, DELEGATE_2);
        refer(&mut sc, DELEGATE_3);
        escalate(&mut sc, 1);
        let case_id = case_of(&mut sc);
        jt::run_round(&mut sc, &fx, case_id, vector[0, 0, 0, 0, 0]);
        apply(&mut sc, case_id);
        apply(&mut sc, case_id);
        ts::end(sc);
    }

    #[test]
    fun delegates_and_the_submitter_never_sit_on_their_own_jury() {
        let mut sc = ts::begin(jt::admin());
        // Six jurors and a jury of five: with juror 0 seated as a delegate only the other five can be drawn.
        let fx = jt::setup(&mut sc, 6);
        jt::fill_roster(&mut sc, &fx, 6);
        jt::set_min_roster(&mut sc, 3);
        jt::create_profile(&mut sc, SUBMITTER, b"submitter");
        ts::next_tx(&mut sc, jt::admin());
        {
            let clock = ts::take_shared<Clock>(&sc);
            let ctx = ts::ctx(&mut sc);
            let _ids = governance::bootstrap_init(&clock, ctx);
            governance::test_grant_admin_cap(ctx);
            ts::return_shared(clock);
        };
        ts::next_tx(&mut sc, jt::admin());
        {
            let cap = ts::take_from_sender<GovernanceAdminCap>(&sc);
            let mut registry = take_registry(&sc);
            let clock = ts::take_shared<Clock>(&sc);
            governance::update_governance_parameters(
                &mut registry, &cap, 3, 90, 10_000, 5, 0, 1_000, 2, &clock, ts::ctx(&mut sc),
            );
            governance::test_add_delegate(&mut registry, jt::owner(0));
            governance::set_registry_jury_policy(
                &cap, &mut registry, true, true, 3 * jt::day(), 5, 5, 2500, 0, 1_000, 2, 5000, 6667, &clock,
            );
            ts::return_shared(clock);
            ts::return_shared(registry);
            ts::return_to_sender(&sc, cap);
        };
        submit(&mut sc);
        jt::advance(&mut sc, 3 * jt::day() + 1);
        escalate(&mut sc, 1);
        let case_id = case_of(&mut sc);
        jt::do_select(&mut sc, case_id);

        let jurors = jt::jurors_of(&mut sc, case_id);
        assert!(vector::length(&jurors) == 5, 0);
        assert!(!vector::contains(&jurors, &jt::agent_id(&fx, 0)), 1);
        ts::end(sc);
    }

    // ============================================================
    // Community step
    // ============================================================

    #[test]
    fun a_vote_with_no_quorum_waits_for_the_jury_instead_of_rejecting() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        open_community_vote(&mut sc);
        end_vote_without_quorum(&mut sc);

        // Nothing moved: still voting, the pool untouched, the jury waiting.
        assert!(status(&mut sc) == STATUS_COMMUNITY_VOTING, 0);
        assert!(pool(&mut sc) == POOL, 1);
        assert!(states(&mut sc) == both(0, STATE_AWAITING), 2);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::EJuryHasJurisdiction)]
    fun a_vote_waiting_for_the_jury_cannot_be_finalized_again() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        open_community_vote(&mut sc);
        end_vote_without_quorum(&mut sc);
        finalize(&mut sc);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::ENoJuryRecord)]
    fun a_proposal_nobody_asked_the_jury_about_cannot_be_escalated_at_the_community_step() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        open_community_vote(&mut sc);
        jt::advance(&mut sc, 2_000);
        escalate(&mut sc, 2);
        ts::end(sc);
    }

    #[test]
    fun a_jury_can_approve_a_proposal_nobody_voted_on() {
        let mut sc = ts::begin(jt::admin());
        let fx = setup(&mut sc, 12, true);
        open_community_vote(&mut sc);
        end_vote_without_quorum(&mut sc);
        escalate(&mut sc, 2);
        let case_id = case_of(&mut sc);
        assert!(states(&mut sc) == both(0, STATE_ESCALATED), 0);
        assert!(pool(&mut sc) == POOL - FEE, 1);

        // The community step asks for two thirds, so three of five is not enough and four is.
        jt::run_round(&mut sc, &fx, case_id, vector[0, 0, 0, 0, 1]);
        apply(&mut sc, case_id);
        assert!(status(&mut sc) == STATUS_APPROVED, 2);
        assert!(states(&mut sc) == both(0, STATE_DECIDED), 3);
        ts::end(sc);
    }

    #[test]
    fun a_jury_can_reject_and_the_pool_goes_to_the_treasury() {
        let mut sc = ts::begin(jt::admin());
        let fx = setup(&mut sc, 12, true);
        open_community_vote(&mut sc);
        end_vote_without_quorum(&mut sc);
        escalate(&mut sc, 2);
        let case_id = case_of(&mut sc);
        jt::run_round(&mut sc, &fx, case_id, vector[1, 1, 1, 1, 1]);
        let before = jt::coin_total(&mut sc, jt::admin());
        apply(&mut sc, case_id);

        assert!(status(&mut sc) == STATUS_REJECTED, 0);
        assert!(pool(&mut sc) == 0, 1);
        assert!(jt::coin_total(&mut sc, jt::admin()) == before + (POOL - FEE), 2);
        ts::end(sc);
    }

    #[test]
    fun a_jury_that_cannot_decide_rejects_what_no_quorum_always_rejected() {
        let mut sc = ts::begin(jt::admin());
        let fx = setup(&mut sc, 12, true);
        open_community_vote(&mut sc);
        end_vote_without_quorum(&mut sc);
        escalate(&mut sc, 2);
        let case_id = case_of(&mut sc);
        jt::do_void(&mut sc, case_id);
        apply(&mut sc, case_id);
        assert!(status(&mut sc) == STATUS_REJECTED, 0);
        ts::end(sc);
    }

    #[test]
    fun a_paused_jury_cannot_strand_a_finished_vote() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        open_community_vote(&mut sc);
        end_vote_without_quorum(&mut sc);
        jt::set_paused(&mut sc, false);
        let before = jt::coin_total(&mut sc, jt::admin());
        escalate(&mut sc, 2);

        // Rejected at once, with the whole pool forfeited and no case opened.
        assert!(status(&mut sc) == STATUS_REJECTED, 0);
        assert!(states(&mut sc) == both(0, STATE_DECIDED), 1);
        assert!(jt::coin_total(&mut sc, jt::admin()) == before + POOL, 2);
        ts::end(sc);
    }

    #[test]
    fun without_a_policy_a_vote_with_no_quorum_is_rejected_as_before() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, false);
        open_community_vote(&mut sc);
        let before = jt::coin_total(&mut sc, jt::admin());
        end_vote_without_quorum(&mut sc);

        assert!(status(&mut sc) == STATUS_REJECTED, 0);
        assert!(states(&mut sc) == both(0, 0), 1);
        assert!(jt::coin_total(&mut sc, jt::admin()) == before + POOL, 2);
        ts::end(sc);
    }

    // ============================================================
    // Policy
    // ============================================================

    #[test, expected_failure(abort_code = governance::EInvalidJuryPolicy)]
    fun a_jury_that_cannot_be_paid_cannot_be_switched_on() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, false);
        ts::next_tx(&mut sc, jt::admin());
        let cap = ts::take_from_sender<GovernanceAdminCap>(&sc);
        let mut registry = take_registry(&sc);
        let clock = ts::take_shared<Clock>(&sc);
        governance::set_registry_jury_policy(
            &cap, &mut registry, true, true, jt::day(), 5, 5, 0, 0, 1_000, 2, 5000, 6667, &clock,
        );
        ts::return_shared(clock);
        ts::return_shared(registry);
        ts::return_to_sender(&sc, cap);
        ts::end(sc);
    }

    #[test, expected_failure(abort_code = governance::EInvalidJuryPolicy)]
    fun a_jury_smaller_than_its_quorum_is_rejected() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, false);
        ts::next_tx(&mut sc, jt::admin());
        let cap = ts::take_from_sender<GovernanceAdminCap>(&sc);
        let mut registry = take_registry(&sc);
        let clock = ts::take_shared<Clock>(&sc);
        governance::set_registry_jury_policy(
            &cap, &mut registry, true, true, jt::day(), 1, 5, 2500, 0, 1_000, 2, 5000, 6667, &clock,
        );
        ts::return_shared(clock);
        ts::return_shared(registry);
        ts::return_to_sender(&sc, cap);
        ts::end(sc);
    }

    #[test]
    fun a_pool_too_small_to_pay_jurors_keeps_the_old_behaviour() {
        let mut sc = ts::begin(jt::admin());
        let _fx = setup(&mut sc, 12, true);
        ts::next_tx(&mut sc, jt::admin());
        {
            let cap = ts::take_from_sender<GovernanceAdminCap>(&sc);
            let mut registry = take_registry(&sc);
            let clock = ts::take_shared<Clock>(&sc);
            // Raise the pool floor above what the proposal holds.
            governance::set_registry_jury_policy(
                &cap, &mut registry, true, true, 3 * jt::day(), 5, 5, 2500, 0, POOL + 1, 2, 5000, 6667, &clock,
            );
            ts::return_shared(clock);
            ts::return_shared(registry);
            ts::return_to_sender(&sc, cap);
        };
        open_community_vote(&mut sc);
        end_vote_without_quorum(&mut sc);
        assert!(status(&mut sc) == STATUS_REJECTED, 0);
        ts::end(sc);
    }
}
