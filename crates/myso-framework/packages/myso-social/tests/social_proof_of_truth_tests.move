// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, unused_assignment, duplicate_alias)]
module social_contracts::social_proof_of_truth_tests {
    use std::{string::{Self, String}, option, vector};

    use myso::test_scenario::{Self, Scenario};
    use myso::tx_context;
    use myso::object;
    use myso::coin::{Self, Coin};
    use myso::balance;
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

    // Test addresses
    const ADMIN: address = @0xA0;
    const CREATOR: address = @0xC1;
    const USER1: address = @0x01;
    const USER2: address = @0x02;
    const TEST_PLATFORM_ID: address = @0x01; // Use USER1's address as test platform ID

    const SCALING: u64 = 1000000000; // 1e9

    // --- Helpers ---
    fun setup_env(): Scenario {
        let mut scen = test_scenario::begin(ADMIN);

        // Init core modules used by SPoT flow
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

        // Mint funds
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            transfer_to(USER1, 10_000 * SCALING, test_scenario::ctx(&mut scen));
            transfer_to(USER2, 10_000 * SCALING, test_scenario::ctx(&mut scen));
            transfer_to(CREATOR, 10_000 * SCALING, test_scenario::ctx(&mut scen));
        };

        // Create a platform owned by USER1 (simplified)
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut preg = test_scenario::take_shared<PlatformRegistry>(&scen);
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            platform::create_platform(
                &mut preg,
                &platform_config,
                string::utf8(b"SPoT Test Platform"),
                string::utf8(b"Tag"),
                string::utf8(b"Desc"),
                string::utf8(b"https://logo"),
                string::utf8(b"https://tos"),
                string::utf8(b"https://pp"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example")],
                string::utf8(b"Social Network"), // primary_category
                option::none(), // secondary_category
                3,
                string::utf8(b"2024-01-01"),
                false,
                option::none(), option::none(), option::none(), option::none(), option::none(), option::none(), option::none(),
                option::none(), option::none(),
                &clock,
                test_scenario::ctx(&mut scen)
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

    /// Create a simple post without platform/profile constraints (test helper in post module)
    /// Creates post with SPoT enabled for SPoT tests
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

    fun resolve_spot_market(
        scen: &mut Scenario,
        outcome: u8,
        confidence: u64,
        reasoning: String,
    ) {
        test_scenario::next_tx(scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(scen);
            let claim = test_scenario::take_shared<spot::SpotClaim>(scen);
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(scen);
            let post_ref = test_scenario::take_shared<Post>(scen);
            let mut platform = test_scenario::take_shared<Platform>(scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(scen);
            let clock = test_scenario::take_shared<Clock>(scen);
            let mut evidence_urls = vector::empty<String>();
            vector::push_back(&mut evidence_urls, string::utf8(b"https://example.com/evidence1"));
            spot::oracle_resolve(
                &oracle_admin_cap,
                &cfg,
                &mut spot_registry,
                &claim,
                &mut rec,
                &post_ref,
                &mut platform,
                &treasury,
                outcome,
                confidence,
                reasoning,
                evidence_urls,
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_shared(platform);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(claim);
            test_scenario::return_shared(spot_registry);
        };
    }

    // --- Tests ---

    #[test]
    fun test_spot_bootstrap_and_update_config() {
        let mut scen = setup_env();

        // Update SPoT config to enable immediate resolution and set low fee for tests
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let spot_gov_id = spot::spot_governance_registry_id(&cfg);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::update_spot_config(
                &admin_cap,
                &mut cfg,
                true, // enable
                7000, // confidence_threshold
                0,    // resolution_window_ms (immediate)
                0,    // max_resolution_window_ms (immediate)
                0,    // payout_delay_ms
                25,   // platform_fee_bps
                25,   // ecosystem_fee_bps
                0,    // creator_fee_bps
                0,    // creator_claim_window_ms
                10000, // expired_creator_ecosystem_bps
                2,    // min_betting_options
                10,   // max_betting_options
                1,    // min_reasoning_length
                1000, // max_reasoning_length
                10,   // max_evidence_urls
                ADMIN, // oracle_address
                0,    // max_single_bet
                10000, // max_bets_per_record
                spot_gov_id,
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_spot_bet_and_resolve_yes() {
        let mut scen = setup_env();

        // Configure SPoT for instant resolve
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let spot_gov_id = spot::spot_governance_registry_id(&cfg);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 0, 0, 0, 0, 5000, 5000, 0, 0, 10000, 2, 10, 1, 1000, 10, ADMIN, 0, 10000, spot_gov_id, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };

        // Create post
        test_scenario::next_tx(&mut scen, CREATOR);
        let post_id_addr = {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let ctx = test_scenario::ctx(&mut scen);
            let id = create_test_post(CREATOR, &clock, ctx);
            test_scenario::return_shared(clock);
            id
        };

        // Create SPoT record with betting options
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let mut p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg, 
                &mut spot_registry,
                &mut p, 
                betting_options,
                option::none(), // resolution_window_epochs - immediate resolution
                option::some(0), // max_resolution_window_ms
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(clock);
        };

        // User1 places bet on option 0 (Yes)
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut spot_rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let pay = coin::mint_for_testing<MYSO>(1000 * SCALING, test_scenario::ctx(&mut scen));
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            
            spot::place_spot_bet(
                &spot_cfg,
                &mut spot_rec,
                &post_ref,
                pay,
                0, // option_id 0 = "Yes"
                1000 * SCALING,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            // Assertions on record via getters
            assert!(spot::get_option_escrow(&spot_rec, 0) == 1000 * SCALING, 1);
            assert!(spot::get_bets_len(&spot_rec) == 1, 2);

            test_scenario::return_shared(spot_rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        resolve_spot_market(
            &mut scen,
            0,
            9000,
            string::utf8(b"Test reasoning: High confidence resolution"),
        );

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            assert!(spot::get_status(&rec) == 3, 3); // STATUS_RESOLVED
            test_scenario::return_shared(rec);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_spot_dao_required_and_finalize_draw() {
        let mut scen = setup_env();

        // Lower confidence threshold to require DAO
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let spot_gov_id = spot::spot_governance_registry_id(&cfg);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 9000, 0, 0, 0, 5000, 5000, 0, 0, 10000, 2, 10, 1, 1000, 10, ADMIN, 0, 10000, spot_gov_id, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };

        // Fast-test SPoT governance parameters (low quorum / short voting window)
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let gov_admin = test_scenario::take_from_sender<GovernanceAdminCap>(&scen);
            let mut spot_registry = take_spot_governance_registry(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
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
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_to_sender(&scen, gov_admin);
        };

        // Create post and record
        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            create_test_post(CREATOR, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let mut p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg, 
                &mut spot_registry,
                &mut p, 
                betting_options,
                option::none(), // resolution_window_epochs - immediate resolution
                option::some(0), // max_resolution_window_ms
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(clock);
        };

        // Place bet with USER1 on option 1 (No)
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let pay = coin::mint_for_testing<MYSO>(500 * SCALING, test_scenario::ctx(&mut scen));
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            
            spot::place_spot_bet(&spot_cfg, &mut rec, &post_ref, pay, 1, 500 * SCALING, &clock, test_scenario::ctx(&mut scen)); // option_id 1 = "No"

            // Check state updated via getters
            assert!(spot::get_option_escrow(&rec, 1) == 500 * SCALING, 1);
            assert!(spot::get_bets_len(&rec) == 1, 2);

            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        // Oracle says confidence is too low → DAO_REQUIRED
        resolve_spot_market(&mut scen, 0, 1000, string::utf8(b"Test reasoning: Low confidence, requires DAO"));

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            assert!(spot::get_status(&rec) == 2, 3);
            test_scenario::return_shared(rec);
        };

        // Submit governance proposal to ratify DRAW, then implement after approval
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut spot_registry = take_spot_governance_registry(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut payment = coin::mint_for_testing<MYSO>(10_000 * SCALING, test_scenario::ctx(&mut scen));
            spot::submit_spot_resolution_proposal_to_governance(
                &cfg,
                &mut spot_registry,
                &mut rec,
                &post_ref,
                string::utf8(b"Resolve SPoT as draw"),
                string::utf8(b"Community consensus: draw outcome"),
                spot::outcome_draw(),
                option::none(),
                &mut payment,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            if (coin::value(&payment) > 0) {
                myso::transfer::public_transfer(payment, ADMIN);
            } else {
                coin::destroy_zero(payment);
            };
            test_scenario::return_shared(clock);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(cfg);
        };

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
                true,
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
            let mut gov_registry = take_spot_governance_registry(&scen);
            let mut claim_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let claim = test_scenario::take_shared<spot::SpotClaim>(&scen);
            let mut proposal = test_scenario::take_shared<Proposal>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let mut platform = test_scenario::take_shared<Platform>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let mut clock = test_scenario::take_shared<Clock>(&scen);
            clock::increment_for_testing(&mut clock, 2);
            spot::finalize_spot_governance_proposal(
                &cfg,
                &mut gov_registry,
                &mut proposal,
                &mut rec,
                &post_ref,
                &treasury,
                &clock,
                test_scenario::ctx(&mut scen),
            );
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
                string::utf8(b"DAO consensus: Draw outcome after governance approval"),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(platform);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(clock);
            assert!(spot::get_status(&rec) == 3, 4); // RESOLVED
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

    #[test]
    fun test_spot_refund_unresolved() {
        let mut scen = setup_env();

        // Set max window = 0 for immediate refunds
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let spot_gov_id = spot::spot_governance_registry_id(&cfg);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 7000, 0, 0, 0, 5000, 5000, 0, 0, 10000, 2, 10, 1, 1000, 10, ADMIN, 0, 10000, spot_gov_id, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };

        // Create post + record
        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            create_test_post(CREATOR, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let mut p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg, 
                &mut spot_registry,
                &mut p, 
                betting_options,
                option::none(), // resolution_window_epochs - immediate resolution
                option::some(0), // max_resolution_window_ms
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(clock);
        };

        // Place a bet on option 0 (Yes)
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let pay = coin::mint_for_testing<MYSO>(250 * SCALING, test_scenario::ctx(&mut scen));
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            
            spot::place_spot_bet(&spot_cfg, &mut rec, &post_ref, pay, 0, 250 * SCALING, &clock, test_scenario::ctx(&mut scen)); // option_id 0 = "Yes"

            assert!(spot::get_option_escrow(&rec, 0) == 250 * SCALING, 1);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        // Immediately allow refund_unresolved (max window already 0) - now requires oracle admin cap
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::refund_unresolved(&oracle_admin_cap, &cfg, &mut spot_registry, &mut rec, &post_ref, &clock, test_scenario::ctx(&mut scen));
            assert!(spot::get_status(&rec) == 4, 2); // REFUNDABLE
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::social_proof_of_truth::EDuplicateOption)]
    fun test_spot_duplicate_options_rejected() {
        let mut scen = setup_env();

        // Configure SPoT
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let spot_gov_id = spot::spot_governance_registry_id(&cfg);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 7000, 0, 0, 0, 5000, 5000, 0, 0, 10000, 2, 10, 1, 1000, 10, ADMIN, 0, 10000, spot_gov_id, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };

        // Create post
        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            create_test_post(CREATOR, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
        };

        // Try to create record with duplicate options - should fail
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let mut p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"Yes")); // Duplicate!
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg, 
                &mut spot_registry,
                &mut p, 
                betting_options,
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = social_contracts::social_proof_of_truth::EWithdrawalNotAllowed)]
    fun test_spot_withdrawal_not_allowed_dao_required() {
        let mut scen = setup_env();

        // Configure SPoT with high confidence threshold to force DAO_REQUIRED
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let spot_gov_id = spot::spot_governance_registry_id(&cfg);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 9000, 0, 0, 0, 5000, 5000, 0, 0, 10000, 2, 10, 1, 1000, 10, ADMIN, 0, 10000, spot_gov_id, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };

        // Create post and record
        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            create_test_post(CREATOR, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let mut p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg, 
                &mut spot_registry,
                &mut p, 
                betting_options,
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(clock);
        };

        // Place bet
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let pay = coin::mint_for_testing<MYSO>(500 * SCALING, test_scenario::ctx(&mut scen));
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            
            spot::place_spot_bet(&spot_cfg, &mut rec, &post_ref, pay, 0, 500 * SCALING, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        resolve_spot_market(&mut scen, 0, 1000, string::utf8(b"Low confidence resolution"));

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            assert!(spot::get_status(&rec) == 2, 1);
            test_scenario::return_shared(rec);
        };

        // Try to withdraw when status is DAO_REQUIRED - should fail
        test_scenario::next_tx(&mut scen, USER1);
        {

            let clock = test_scenario::take_shared<Clock>(&scen);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let mut platform = test_scenario::take_shared<Platform>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let claim = test_scenario::take_shared<spot::SpotClaim>(&scen);
            spot::withdraw_spot_bet(
                &spot_cfg,
                &claim,
                &mut rec,
                &post_ref,
                &post_ref,
                &mut platform,
                &treasury,
                0,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(claim);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);

            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_spot_max_bets_per_record_zero_is_unlimited() {
        let mut scen = setup_env();

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let spot_gov_id = spot::spot_governance_registry_id(&cfg);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::update_spot_config(
                &admin_cap,
                &mut cfg,
                true,
                7000,
                0,
                0,
                0,
                0,
                5000,
                0,
                0,
                10000,
                2,
                10,
                1,
                5000,
                5,
                ADMIN,
                0,
                0,
                spot_gov_id,
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            create_test_post(CREATOR, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let mut p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg,
                &mut spot_registry,
                &mut p,
                betting_options,
                option::none(),
                option::some(0),
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut spot_rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let coin1 = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            spot::place_spot_bet(
                &spot_cfg,
                &mut spot_rec,
                &post_ref,
                coin1,
                0,
                SCALING,
                &clock,
                test_scenario::ctx(&mut scen)
            );
            assert!(spot::get_bets_len(&spot_rec) == 1, 1);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(spot_rec);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = spot::ETooManyBets)]
    fun test_spot_bet_limit_enforcement() {
        let mut scen = setup_env();

        // Configure SPoT with low bet limit for testing
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let spot_gov_id = spot::spot_governance_registry_id(&cfg);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 7000, 0, 0, 0, 5000, 5000, 0, 0, 10000, 2, 10, 1, 1000, 10, ADMIN, 0, 3, spot_gov_id, &clock, test_scenario::ctx(&mut scen)); // max_bets_per_record = 3
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };

        // Create post
        test_scenario::next_tx(&mut scen, CREATOR);
        let post_id_addr = {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let ctx = test_scenario::ctx(&mut scen);
            let id = create_test_post(CREATOR, &clock, ctx);
            test_scenario::return_shared(clock);
            id
        };

        // Create SPoT record
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(&scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let mut p = test_scenario::take_shared<Post>(&scen);
            let mut betting_options = vector::empty<String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg,
                &mut spot_registry,
                &mut p,
                betting_options,
                option::none(),
                option::some(0),
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_to_sender(&scen, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(clock);
        };

        // Place 3 bets (at limit)
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut spot_rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let coin1 = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            spot::place_spot_bet(&spot_cfg, &mut spot_rec, &post_ref, coin1, 0, SCALING, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(spot_rec);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, USER2);
        {
            let mut spot_rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let coin2 = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            spot::place_spot_bet(&spot_cfg, &mut spot_rec, &post_ref, coin2, 0, SCALING, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(spot_rec);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let mut spot_rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let coin3 = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            spot::place_spot_bet(&spot_cfg, &mut spot_rec, &post_ref, coin3, 0, SCALING, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(spot_rec);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        // Try to place 4th bet - should fail with ETooManyBets
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut spot_rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let coin4 = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            spot::place_spot_bet(&spot_cfg, &mut spot_rec, &post_ref, coin4, 0, SCALING, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(spot_rec);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }
}
