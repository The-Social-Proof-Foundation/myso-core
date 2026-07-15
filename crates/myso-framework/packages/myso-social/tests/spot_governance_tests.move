// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, unused_assignment, duplicate_alias)]
module social_contracts::spot_governance_tests {
    use std::{string::{Self, String}, option, vector};

    use myso::test_scenario::{Self, Scenario};
    use myso::tx_context;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::clock::{Self, Clock};

    use social_contracts::social_proof_of_truth as spot;
    use social_contracts::social_proof_tokens as spt;
    use social_contracts::post::{Self, Post};
    use social_contracts::platform::{Self, Platform, PlatformRegistry,
        PlatformConfig};
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::profile::{Self, EcosystemTreasury};
    use social_contracts::governance::{Self, GovernanceDAO, Proposal, GovernanceAdminCap};

    const ADMIN: address = @0xA0;
    const CREATOR: address = @0xC1;
    const USER1: address = @0x01;
    const USER2: address = @0x02;
    const TEST_PLATFORM_ID: address = @0x01;
    const SCALING: u64 = 1000000000;

    fun setup_env(): Scenario {
        let mut scen = test_scenario::begin(ADMIN);
        spt::init_for_testing(test_scenario::ctx(&mut scen));
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            block_list::test_init(&clock, test_scenario::ctx(&mut scen));
            platform::test_init(&clock, test_scenario::ctx(&mut scen));
            post::test_init(test_scenario::ctx(&mut scen));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scen));
            clock::share_for_testing(clock);
        };
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let ctx = test_scenario::ctx(&mut scen);
            let gov_ids = governance::bootstrap_init(&clock, ctx);
            governance::test_grant_admin_cap(ctx);
            spot::test_init(&clock, gov_ids.spot_governance_registry_id(), ctx);
            test_scenario::return_shared(clock);
        };
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            transfer_to(USER1, 10_000 * SCALING, test_scenario::ctx(&mut scen));
            transfer_to(USER2, 10_000 * SCALING, test_scenario::ctx(&mut scen));
            transfer_to(CREATOR, 10_000 * SCALING, test_scenario::ctx(&mut scen));
        };
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut preg = test_scenario::take_shared<PlatformRegistry>(&scen);
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            platform::create_platform(
                &mut preg,
                &platform_config,
                string::utf8(b"SPoT Gov Test Platform"),
                string::utf8(b"Tag"),
                string::utf8(b"Desc"),
                string::utf8(b"https://logo"),
                string::utf8(b"https://tos"),
                string::utf8(b"https://pp"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example")],
                string::utf8(b"Social Network"),
                option::none(),
                3,
                string::utf8(b"2024-01-01"),
                false,
                option::none(), option::none(), option::none(), option::none(), option::none(), option::none(), option::none(),
                option::none(), option::none(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(preg);
        };
        scen
    }

    fun transfer_to(to: address, amount: u64, ctx: &mut tx_context::TxContext) {
        let c = coin::mint_for_testing<MYSO>(amount, ctx);
        myso::transfer::public_transfer(c, to);
    }

    fun create_test_post(owner: address, clock: &Clock, ctx: &mut tx_context::TxContext): address {
        post::test_create_post_with_spot(owner, owner, TEST_PLATFORM_ID, string::utf8(b"truth?"), clock, ctx)
    }

    fun take_spot_governance_registry(scenario: &Scenario): GovernanceDAO {
        let r0 = test_scenario::take_shared<GovernanceDAO>(scenario);
        if (governance::registry_type(&r0) == governance::proposal_type_spot_value()) {
            return r0
        };
        let r1 = test_scenario::take_shared<GovernanceDAO>(scenario);
        if (governance::registry_type(&r1) == governance::proposal_type_spot_value()) {
            test_scenario::return_shared(r0);
            return r1
        };
        let r2 = test_scenario::take_shared<GovernanceDAO>(scenario);
        if (governance::registry_type(&r2) == governance::proposal_type_spot_value()) {
            test_scenario::return_shared(r0);
            test_scenario::return_shared(r1);
            return r2
        };
        test_scenario::return_shared(r0);
        test_scenario::return_shared(r1);
        test_scenario::return_shared(r2);
        abort 999
    }

    fun tune_spot_governance_for_tests(scen: &mut Scenario) {
        test_scenario::next_tx(scen, ADMIN);
        {
            let gov_admin = test_scenario::take_from_sender<GovernanceAdminCap>(scen);
            let mut spot_registry = take_spot_governance_registry(scen);
            let clock = test_scenario::take_shared<Clock>(scen);
            governance::update_governance_parameters(
                &mut spot_registry,
                &gov_admin,
                3,
                90,
                1_000,
                5,
                0,
                1,
                1,
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_to_sender(scen, gov_admin);
        };
    }

    fun oracle_resolve_low_confidence(scen: &mut Scenario) {
        test_scenario::next_tx(scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(scen);
            let mut claim_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(scen);
            let claim = test_scenario::take_shared<spot::SpotClaim>(scen);
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(scen);
            let post_ref = test_scenario::take_shared<Post>(scen);
            let mut evidence_urls = vector::empty<String>();
            vector::push_back(&mut evidence_urls, string::utf8(b"https://example.com/evidence"));
            let mut platform = test_scenario::take_shared<Platform>(scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(scen);
            let clock = test_scenario::take_shared<Clock>(scen);
            spot::oracle_resolve(
                &oracle_admin_cap,
                &cfg,
                &mut claim_registry,
                &claim,
                &mut rec,
                &post_ref,
                &mut platform,
                &treasury,
                0,
                1000u64,
                string::utf8(b"Low confidence"),
                evidence_urls,
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_shared(platform);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(clock);
            assert!(spot::get_status(&rec) == 2, 1);
            test_scenario::return_to_sender(scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(claim);
            test_scenario::return_shared(claim_registry);
        };
    }

    fun create_spot_market_and_escalate(scen: &mut Scenario) {
        test_scenario::next_tx(scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(scen);
            let spot_gov_id = spot::spot_governance_registry_id(&cfg);
            let clock = test_scenario::take_shared<Clock>(scen);
            spot::update_spot_config(
                &admin_cap,
                &mut cfg,
                true,
                9000,
                0,
                0,
                0,
                2500,
                2500,
                100,
                2592000000,
                10000,
                2,
                10,
                1,
                1000,
                10,
                ADMIN,
                0,
                10000,
                10,
                spot_gov_id,
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_to_sender(scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(scen, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(scen);
            create_test_post(CREATOR, &clock, test_scenario::ctx(scen));
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(scen);
            let mut p = test_scenario::take_shared<Post>(scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            let clock = test_scenario::take_shared<Clock>(scen);
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg,
                &mut spot_registry,
                &mut p,
                betting_options,
                option::none(),
                option::some(0),
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_to_sender(scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(scen, USER1);
        {
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(scen);
            let post_ref = test_scenario::take_shared<Post>(scen);
            let pay = coin::mint_for_testing<MYSO>(500 * SCALING, test_scenario::ctx(scen));
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(scen);
            let clock = test_scenario::take_shared<Clock>(scen);
            spot::place_spot_bet(&spot_cfg, &mut rec, &post_ref, pay, 0, 500 * SCALING, &clock, test_scenario::ctx(scen));
            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        oracle_resolve_low_confidence(scen);
    }

    fun submit_draw_proposal(scen: &mut Scenario) {
        test_scenario::next_tx(scen, ADMIN);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(scen);
            let mut spot_registry = take_spot_governance_registry(scen);
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(scen);
            let post_ref = test_scenario::take_shared<Post>(scen);
            let clock = test_scenario::take_shared<Clock>(scen);
            let mut payment = coin::mint_for_testing<MYSO>(10_000 * SCALING, test_scenario::ctx(scen));
            spot::submit_spot_resolution_proposal_to_governance(
                &cfg,
                &mut spot_registry,
                &mut rec,
                &post_ref,
                string::utf8(b"Resolve as draw"),
                string::utf8(b"Draw proposal"),
                spot::outcome_draw(),
                option::none(),
                &mut payment,
                &clock,
                test_scenario::ctx(scen),
            );
            if (coin::value(&payment) > 0) {
                myso::transfer::public_transfer(payment, ADMIN);
            } else {
                coin::destroy_zero(payment);
            };
            assert!(option::is_some(spot::active_proposal_id(&rec)), 2);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(cfg);
        };
    }

    #[test]
    fun test_spot_proposal_reject_clears_link() {
        let mut scen = setup_env();
        tune_spot_governance_for_tests(&mut scen);
        create_spot_market_and_escalate(&mut scen);
        submit_draw_proposal(&mut scen);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let mut spot_registry = take_spot_governance_registry(&scen);
            let mut proposal = test_scenario::take_shared<Proposal>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            governance::delegate_vote_on_proposal(
                &mut spot_registry,
                &mut proposal,
                &treasury,
                true,
                option::some(string::utf8(b"Advance to community")),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(spot_registry);
        };

        test_scenario::next_tx(&mut scen, USER2);
        {
            let mut spot_registry = take_spot_governance_registry(&scen);
            let mut proposal = test_scenario::take_shared<Proposal>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut payment = coin::mint_for_testing<MYSO>(1 * SCALING, test_scenario::ctx(&mut scen));
            governance::community_vote_on_proposal(
                &mut spot_registry,
                &mut proposal,
                1,
                false,
                &mut payment,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            if (coin::value(&payment) > 0) {
                myso::transfer::public_transfer(payment, USER2);
            } else {
                coin::destroy_zero(payment);
            };
            test_scenario::return_shared(clock);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(spot_registry);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut spot_registry = take_spot_governance_registry(&scen);
            let mut proposal = test_scenario::take_shared<Proposal>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let mut clock = test_scenario::take_shared<Clock>(&scen);
            clock::increment_for_testing(&mut clock, 2);
            spot::finalize_spot_governance_proposal(
                &cfg,
                &mut spot_registry,
                &mut proposal,
                &mut rec,
                &post_ref,
                &treasury,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            assert!(option::is_none(spot::active_proposal_id(&rec)), 3);
            assert!(spot::get_status(&rec) == 2, 4);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(cfg);
        };

        submit_draw_proposal(&mut scen);
        test_scenario::end(scen);
    }

    #[test, expected_failure(abort_code = 22, location = social_contracts::social_proof_of_truth)]
    fun test_bets_blocked_during_dao_required() {
        let mut scen = setup_env();
        tune_spot_governance_for_tests(&mut scen);
        create_spot_market_and_escalate(&mut scen);

        test_scenario::next_tx(&mut scen, USER2);
        {
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let pay = coin::mint_for_testing<MYSO>(100 * SCALING, test_scenario::ctx(&mut scen));
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::place_spot_bet(&spot_cfg, &mut rec, &post_ref, pay, 1, 100 * SCALING, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test, expected_failure(abort_code = 24, location = social_contracts::social_proof_of_truth)]
    fun test_implement_without_approved_proposal_aborts() {
        let mut scen = setup_env();
        tune_spot_governance_for_tests(&mut scen);
        create_spot_market_and_escalate(&mut scen);
        submit_draw_proposal(&mut scen);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut gov_registry = take_spot_governance_registry(&scen);
            let mut claim_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let claim = test_scenario::take_shared<spot::SpotClaim>(&scen);
            let mut proposal = test_scenario::take_shared<Proposal>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let mut platform = test_scenario::take_shared<Platform>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::implement_spot_resolution_from_governance(
                &cfg,
                &mut gov_registry,
                &mut proposal,
                &mut claim_registry,
                &claim,
                &mut rec,
                &post_ref,
                &mut platform,
                &treasury,
                string::utf8(b"Should fail without approval"),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(platform);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(claim);
            test_scenario::return_shared(claim_registry);
            test_scenario::return_shared(gov_registry);
            test_scenario::return_shared(cfg);
        };

        test_scenario::end(scen);
    }

    #[test, expected_failure(abort_code = 18, location = social_contracts::social_proof_of_truth)]
    fun test_duplicate_active_proposal_aborts() {
        let mut scen = setup_env();
        tune_spot_governance_for_tests(&mut scen);
        create_spot_market_and_escalate(&mut scen);
        submit_draw_proposal(&mut scen);
        submit_draw_proposal(&mut scen);
        test_scenario::end(scen);
    }
}
