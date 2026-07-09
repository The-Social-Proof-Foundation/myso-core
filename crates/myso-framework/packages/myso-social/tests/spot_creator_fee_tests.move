// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, unused_assignment, duplicate_alias)]
module social_contracts::spot_creator_fee_tests {
    use std::{string::{Self, String}, option, vector};

    use myso::test_scenario::{Self, Scenario};
    use myso::tx_context;
    use myso::coin::{Self, Coin};
    use myso::object;
    use myso::myso::MYSO;
    use myso::clock::{Self, Clock};

    use social_contracts::social_proof_of_truth as spot;
    use social_contracts::social_proof_tokens as spt;
    use social_contracts::post::{Self, Post};
    use social_contracts::platform::{Self, Platform, PlatformRegistry, PlatformConfig};
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::profile::{Self, EcosystemTreasury};
    use social_contracts::governance;

    const ADMIN: address = @0xA0;
    const CREATOR: address = @0xC1;
    const REFERRER2: address = @0x02;
    const USER1: address = @0x01;
    const TEST_PLATFORM_ID: address = @0x01;
    const SCALING: u64 = 1_000_000_000;

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
            let gov_ids = governance::bootstrap_init(&clock, test_scenario::ctx(&mut scen));
            spot::test_init(&clock, gov_ids.spot_governance_registry_id(), test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            transfer_to(USER1, 20_000 * SCALING, test_scenario::ctx(&mut scen));
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut preg = test_scenario::take_shared<PlatformRegistry>(&scen);
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            platform::create_platform(
                &mut preg,
                &platform_config,
                string::utf8(b"Creator Fee Test Platform"),
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
                option::none(), option::none(), option::none(), option::none(),
                option::none(), option::none(), option::none(),
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

    fun configure_creator_fees(
        scen: &mut Scenario,
        creator_fee_bps: u64,
        creator_claim_window_ms: u64,
        expired_creator_ecosystem_bps: u64,
    ) {
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
                0,
                0,
                0,
                0,
                0,
                0,
                creator_fee_bps,
                creator_claim_window_ms,
                expired_creator_ecosystem_bps,
                2,
                10,
                1,
                1000,
                10,
                ADMIN,
                0,
                10000,
                spot_gov_id,
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_to_sender(scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };
    }

    fun create_primary_market(scen: &mut Scenario): address {
        let primary_post_id = {
            test_scenario::next_tx(scen, CREATOR);
            let clock = test_scenario::take_shared<Clock>(scen);
            let id = create_test_post(CREATOR, &clock, test_scenario::ctx(scen));
            test_scenario::return_shared(clock);
            id
        };

        test_scenario::next_tx(scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(scen);
            let mut p = test_scenario::take_shared_by_id<Post>(scen, object::id_from_address(primary_post_id));
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

        primary_post_id
    }

    fun link_second_post(scen: &mut Scenario): address {
        let secondary_post_id = {
            test_scenario::next_tx(scen, REFERRER2);
            let clock = test_scenario::take_shared<Clock>(scen);
            let id = create_test_post(REFERRER2, &clock, test_scenario::ctx(scen));
            test_scenario::return_shared(clock);
            id
        };

        test_scenario::next_tx(scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(scen);
            let mut claim = test_scenario::take_shared<spot::SpotClaim>(scen);
            let mut p = test_scenario::take_shared_by_id<Post>(scen, object::id_from_address(secondary_post_id));
            let clock = test_scenario::take_shared<Clock>(scen);
            spot::link_post_to_spot_claim(
                &oracle_admin_cap,
                &mut spot_registry,
                &mut claim,
                &mut p,
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_to_sender(scen, oracle_admin_cap);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(p);
            test_scenario::return_shared(claim);
            test_scenario::return_shared(spot_registry);
        };

        secondary_post_id
    }

    fun place_bet_for_post(
        scen: &mut Scenario,
        bettor: address,
        post_id: address,
        option_id: u8,
        amount: u64,
    ) {
        test_scenario::next_tx(scen, bettor);
        {
            let registry = test_scenario::take_shared<spot::SpotClaimRegistry>(scen);
            let mut market = test_scenario::take_shared<spot::SpotMarket>(scen);
            let post_ref = test_scenario::take_shared_by_id<Post>(scen, object::id_from_address(post_id));
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(scen);
            let clock = test_scenario::take_shared<Clock>(scen);
            let pay = coin::mint_for_testing<MYSO>(amount, test_scenario::ctx(scen));
            spot::place_spot_bet_for_post(
                &spot_cfg,
                &registry,
                &mut market,
                &post_ref,
                pay,
                option_id,
                amount,
                option::none(),
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(market);
            test_scenario::return_shared(registry);
        };
    }

    fun resolve_yes(scen: &mut Scenario, primary_post_id: address) {
        test_scenario::next_tx(scen, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(scen);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(scen);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(scen);
            let claim = test_scenario::take_shared<spot::SpotClaim>(scen);
            let mut market = test_scenario::take_shared<spot::SpotMarket>(scen);
            let post_ref = test_scenario::take_shared_by_id<Post>(scen, object::id_from_address(primary_post_id));
            let mut platform = test_scenario::take_shared<Platform>(scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(scen);
            let clock = test_scenario::take_shared<Clock>(scen);
            let mut evidence_urls = vector::empty<String>();
            vector::push_back(&mut evidence_urls, string::utf8(b"https://example.com/evidence"));
            spot::oracle_resolve(
                &oracle_admin_cap,
                &cfg,
                &mut spot_registry,
                &claim,
                &mut market,
                &post_ref,
                &mut platform,
                &treasury,
                0,
                9000,
                string::utf8(b"Resolved yes"),
                evidence_urls,
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(market);
            test_scenario::return_shared(claim);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(cfg);
            test_scenario::return_to_sender(scen, oracle_admin_cap);
        };
    }

    #[test]
    fun test_per_referrer_creator_fees() {
        let mut scen = setup_env();
        configure_creator_fees(&mut scen, 1000, 30 * 86_400_000, 10000);
        let primary_post_id = create_primary_market(&mut scen);
        let secondary_post_id = link_second_post(&mut scen);

        place_bet_for_post(&mut scen, USER1, primary_post_id, 0, 1000 * SCALING);
        place_bet_for_post(&mut scen, USER1, secondary_post_id, 1, 2000 * SCALING);
        resolve_yes(&mut scen, primary_post_id);

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut market = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::claim_creator_payout(&cfg, &mut market, 0, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
            test_scenario::return_shared(market);
            test_scenario::return_shared(cfg);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let payout = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            assert!(coin::value(&payout) == 100 * SCALING, 1);
            coin::burn_for_testing(payout);
        };

        test_scenario::next_tx(&mut scen, REFERRER2);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut market = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::claim_creator_payout(&cfg, &mut market, 1, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
            test_scenario::return_shared(market);
            test_scenario::return_shared(cfg);
        };

        test_scenario::next_tx(&mut scen, REFERRER2);
        {
            let payout = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            assert!(coin::value(&payout) == 200 * SCALING, 2);
            coin::burn_for_testing(payout);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_lazy_claim_creator_payout() {
        let mut scen = setup_env();
        configure_creator_fees(&mut scen, 500, 30 * 86_400_000, 10000);
        let primary_post_id = create_primary_market(&mut scen);

        place_bet_for_post(&mut scen, USER1, primary_post_id, 0, 2000 * SCALING);
        resolve_yes(&mut scen, primary_post_id);

        test_scenario::next_tx(&mut scen, USER1);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut market = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared_by_id<Post>(&scen, object::id_from_address(primary_post_id));
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::claim_payout(&cfg, &mut market, &post_ref, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(market);
            test_scenario::return_shared(cfg);
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let winner_coin = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            assert!(coin::value(&winner_coin) > 0, 3);
            coin::burn_for_testing(winner_coin);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut market = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::claim_creator_payout(&cfg, &mut market, 0, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
            test_scenario::return_shared(market);
            test_scenario::return_shared(cfg);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let creator_coin = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            assert!(coin::value(&creator_coin) == 100 * SCALING, 4);
            coin::burn_for_testing(creator_coin);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_reclaim_expired_creator_rewards() {
        let mut scen = setup_env();
        configure_creator_fees(&mut scen, 500, 1000, 10000);
        let primary_post_id = create_primary_market(&mut scen);

        place_bet_for_post(&mut scen, USER1, primary_post_id, 0, 2000 * SCALING);
        resolve_yes(&mut scen, primary_post_id);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut market = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let mut platform = test_scenario::take_shared<Platform>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let mut clock = test_scenario::take_shared<Clock>(&scen);
            clock::increment_for_testing(&mut clock, 2000);
            spot::reclaim_expired_creator_rewards(
                &cfg,
                &mut market,
                &mut platform,
                &treasury,
                0,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(market);
            test_scenario::return_shared(cfg);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_shared_claim_multiple_posts() {
        let mut scen = setup_env();
        configure_creator_fees(&mut scen, 0, 0, 10000);
        let primary_post_id = create_primary_market(&mut scen);
        let secondary_post_id = link_second_post(&mut scen);

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let primary = test_scenario::take_shared_by_id<Post>(&scen, object::id_from_address(primary_post_id));
            let primary_spot = *option::borrow(post::get_spot_id(&primary));
            let primary_claim = post::get_spot_claim_id(&primary);
            test_scenario::return_shared(primary);
            test_scenario::next_tx(&mut scen, REFERRER2);
            let secondary = test_scenario::take_shared_by_id<Post>(&scen, object::id_from_address(secondary_post_id));
            let secondary_spot = *option::borrow(post::get_spot_id(&secondary));
            let secondary_claim = post::get_spot_claim_id(&secondary);
            assert!(primary_spot == secondary_spot, 1);
            assert!(option::is_some(&primary_claim), 2);
            assert!(primary_claim == secondary_claim, 3);
            test_scenario::return_shared(secondary);
        };

        test_scenario::end(scen);
    }

    #[test, expected_failure(abort_code = 31, location = social_contracts::social_proof_of_truth)]
    fun test_settlement_creator_only_can_claim() {
        let mut scen = setup_env();
        configure_creator_fees(&mut scen, 1000, 30 * 86_400_000, 10000);
        let primary_post_id = create_primary_market(&mut scen);

        place_bet_for_post(&mut scen, USER1, primary_post_id, 0, 1000 * SCALING);
        resolve_yes(&mut scen, primary_post_id);

        test_scenario::next_tx(&mut scen, REFERRER2);
        {
            let cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut market = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            assert!(spot::get_pending_creator_payout_creator(&market, 0) == CREATOR, 1);
            spot::claim_creator_payout(&cfg, &mut market, 0, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
            test_scenario::return_shared(market);
            test_scenario::return_shared(cfg);
        };

        test_scenario::end(scen);
    }

    #[test, expected_failure(abort_code = 29, location = social_contracts::social_proof_of_truth)]
    fun test_router_rejects_unlinked_post() {
        let mut scen = setup_env();
        configure_creator_fees(&mut scen, 0, 0, 10000);
        let _primary_post_id = create_primary_market(&mut scen);

        let unlinked_post_id = {
            test_scenario::next_tx(&mut scen, REFERRER2);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let id = create_test_post(REFERRER2, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(clock);
            id
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let registry = test_scenario::take_shared<spot::SpotClaimRegistry>(&scen);
            let mut market = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let unlinked_post = test_scenario::take_shared_by_id<Post>(&scen, object::id_from_address(unlinked_post_id));
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let pay = coin::mint_for_testing<MYSO>(100 * SCALING, test_scenario::ctx(&mut scen));
            spot::place_spot_bet_for_post(
                &spot_cfg,
                &registry,
                &mut market,
                &unlinked_post,
                pay,
                0,
                100 * SCALING,
                option::none(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(unlinked_post);
            test_scenario::return_shared(market);
            test_scenario::return_shared(registry);
        };

        test_scenario::end(scen);
    }
}
