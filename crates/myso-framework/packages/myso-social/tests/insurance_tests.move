// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, unused_assignment, duplicate_alias)]
module social_contracts::insurance_tests {
    use std::{string, option, vector};

    use myso::test_scenario::{Self, Scenario};
    use myso::tx_context;
    use myso::coin::{Self, Coin};
    use myso::clock::{Self, Clock};
    use myso::transfer;
    use myso::myso::MYSO;
    use myso::object;

    use social_contracts::insurance;
    use social_contracts::social_proof_of_truth as spot;
    use social_contracts::governance;
    use social_contracts::social_proof_tokens as spt;
    use social_contracts::post::{Self, Post};
    use social_contracts::platform::{Self, Platform, PlatformRegistry,
        PlatformConfig};
    use social_contracts::block_list;
    use social_contracts::profile::{Self, EcosystemTreasury};

    const ADMIN: address = @0xA0;
    const CREATOR: address = @0xC1;
    const UNDERWRITER: address = @0xB1;
    const USER1: address = @0x01;
    const TEST_PLATFORM_ID: address = @0x01; // Use USER1's address as test platform ID

    const SCALING: u64 = 1_000_000_000; // 1e9
    const DAY_MS: u64 = 86_400_000;

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
            let admin_cap = test_scenario::take_from_sender<spot::SpotAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let spot_gov_id = spot::spot_governance_registry_id(&cfg);
            let clock = test_scenario::take_shared<Clock>(&scen);
            spot::update_spot_config(&admin_cap, &mut cfg, true, 7000, 0, 0, 0, 100, 5000, 0, 0, 10000, 2, 10, 1, 1000, 10, ADMIN, 0, 10000, 10, spot_gov_id, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            transfer_to(USER1, 10_000 * SCALING, test_scenario::ctx(&mut scen));
            transfer_to(CREATOR, 10_000 * SCALING, test_scenario::ctx(&mut scen));
            transfer_to(UNDERWRITER, 10_000 * SCALING, test_scenario::ctx(&mut scen));
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut preg = test_scenario::take_shared<PlatformRegistry>(&scen);
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            platform::create_platform(
                &mut preg,
                &platform_config,
                string::utf8(b"Insurance Test Platform"),
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
        transfer::public_transfer(c, to);
    }

    fun create_test_post(owner: address, clock: &Clock, ctx: &mut tx_context::TxContext): address {
        post::test_create_post_with_spot(owner, owner, TEST_PLATFORM_ID, string::utf8(b"truth?"), clock, ctx)
    }

    // === Helper Functions ===

    fun setup_env_with_insurance(): Scenario {
        let mut scen = setup_env();
        test_scenario::next_tx(&mut scen, ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scen);
            insurance::init_config(
                1000,
                9000,
                7 * DAY_MS,
                50,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            test_scenario::return_shared(clock);
        };
        scen
    }

    fun setup_vault_with_capital(scenario: &mut Scenario, capital_amount: u64) {
        test_scenario::next_tx(scenario, UNDERWRITER);
        {
            insurance::create_vault(25, 5000, 0, 0, test_scenario::ctx(scenario));
        };
        // Enable the config before depositing capital
        test_scenario::next_tx(scenario, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<insurance::InsuranceAdminCap>(scenario);
            let mut config = test_scenario::take_shared<insurance::InsuranceConfig>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            insurance::set_insurance_enabled(&admin_cap, &mut config, true, &clock, test_scenario::ctx(scenario));
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(scenario, admin_cap);
        };
        test_scenario::next_tx(scenario, UNDERWRITER);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(scenario);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(scenario);
            let deposit = coin::mint_for_testing<MYSO>(capital_amount, test_scenario::ctx(scenario));
            insurance::deposit_capital(&config, &mut vault, deposit, test_scenario::ctx(scenario));
            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
        };
    }

    fun create_open_market_with_bet(scenario: &mut Scenario, bettor: address, bet_amount: u64, option_id: u8) {
        create_post_and_spot_market(scenario);
        place_spot_bet_amt(scenario, bettor, option_id, bet_amount);
    }

    fun create_post_and_spot_market(scenario: &mut Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(scenario);
            create_test_post(CREATOR, &clock, test_scenario::ctx(scenario));
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(scenario, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(scenario);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(scenario);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(scenario);
            let mut p = test_scenario::take_shared<Post>(scenario);
            let mut betting_options = vector::empty<string::String>();
            vector::push_back(&mut betting_options, string::utf8(b"Yes"));
            vector::push_back(&mut betting_options, string::utf8(b"No"));
            let clock = test_scenario::take_shared<Clock>(scenario);
            spot::create_spot_record_for_post(
                &oracle_admin_cap,
                &cfg,
                &mut spot_registry,
                &mut p,
                betting_options,
                option::none(),
                option::some(0),
                &clock,
                test_scenario::ctx(scenario)
            );
            test_scenario::return_to_sender(scenario, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(p);
            test_scenario::return_shared(spot_registry);
            test_scenario::return_shared(clock);
        };
    }

    fun resolve_spot_market_yes(scenario: &mut Scenario, outcome: u8) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let oracle_admin_cap = test_scenario::take_from_sender<spot::SpotOracleAdminCap>(scenario);
            let cfg = test_scenario::take_shared<spot::SpotConfig>(scenario);
            let mut spot_registry = test_scenario::take_shared<spot::SpotClaimRegistry>(scenario);
            let claim = test_scenario::take_shared<spot::SpotClaim>(scenario);
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(scenario);
            let post_ref = test_scenario::take_shared<Post>(scenario);
            let mut platform = test_scenario::take_shared<Platform>(scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            let mut evidence_urls = vector::empty<string::String>();
            vector::push_back(&mut evidence_urls, string::utf8(b"https://example.com/evidence"));
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
                9000u64,
                string::utf8(b"Test resolution"),
                evidence_urls,
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_to_sender(scenario, oracle_admin_cap);
            test_scenario::return_shared(cfg);
            test_scenario::return_shared(rec);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(claim);
            test_scenario::return_shared(spot_registry);
        };
    }

    fun place_spot_bet_amt(scenario: &mut Scenario, bettor: address, option_id: u8, bet_amount: u64) {
        test_scenario::next_tx(scenario, bettor);
        {
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(scenario);
            let post_ref = test_scenario::take_shared<Post>(scenario);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            let pay = coin::mint_for_testing<MYSO>(bet_amount, test_scenario::ctx(scenario));
            spot::place_spot_bet(
                &spot_cfg,
                &mut rec,
                &post_ref,
                pay,
                option_id,
                bet_amount,
                &clock,
                test_scenario::ctx(scenario)
            );
            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };
    }

    fun set_risk_pricing_for_tests(
        scenario: &mut Scenario,
        min_spot_total_liquidity: u64,
        max_coverage_fraction_of_option_bps: u64,
        min_premium_amount: u64,
    ) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<insurance::InsuranceAdminCap>(scenario);
            let mut config = test_scenario::take_shared<insurance::InsuranceConfig>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            insurance::set_risk_pricing_config(
                &admin_cap,
                &mut config,
                min_spot_total_liquidity,
                max_coverage_fraction_of_option_bps,
                500_000,
                min_premium_amount,
                0,
                10,
                true,
                500_000,
                500_000,
                1_000_000_000_000,
                30_000,
                5000,
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(scenario, admin_cap);
        };
    }

    fun insurance_set_min_spot_liquidity(scenario: &mut Scenario, min_liq: u64) {
        set_risk_pricing_for_tests(scenario, min_liq, 10_000, 1);
    }

    fun insurance_set_max_coverage_fraction_only(scenario: &mut Scenario, max_frac: u64) {
        set_risk_pricing_for_tests(scenario, 1, max_frac, 1);
    }

    fun insurance_set_min_premium_only(scenario: &mut Scenario, min_prem: u64) {
        set_risk_pricing_for_tests(scenario, 1, 10_000, min_prem);
    }

    #[test]
    fun test_buy_and_claim_insurance() {
        let mut scen = setup_env();

        test_scenario::next_tx(&mut scen, ADMIN);
        {

            let clock = test_scenario::take_shared<Clock>(&scen);
            insurance::init_config(
                1000,
                9000,
                7 * DAY_MS,
                50,
                &clock,
                test_scenario::ctx(&mut scen)
            );

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
            let mut betting_options = vector::empty<string::String>();
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
            let mut rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let post_ref = test_scenario::take_shared<Post>(&scen);
            let spot_cfg = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let pay = coin::mint_for_testing<MYSO>(1000 * SCALING, test_scenario::ctx(&mut scen));

            spot::place_spot_bet(
                &spot_cfg,
                &mut rec,
                &post_ref,
                pay,
                0,
                1000 * SCALING,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            assert!(spot::get_user_option_amount(&rec, USER1, 0) == 1000 * SCALING, 1);

            test_scenario::return_shared(rec);
            test_scenario::return_shared(spot_cfg);
            test_scenario::return_shared(post_ref);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, UNDERWRITER);
        {
            insurance::create_vault(25, 5000, 0, 0, test_scenario::ctx(&mut scen));
        };

        // Enable the config before depositing capital
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let admin_cap = test_scenario::take_from_sender<insurance::InsuranceAdminCap>(&scen);
            let mut config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            insurance::set_insurance_enabled(&admin_cap, &mut config, true, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scen, admin_cap);
        };

        test_scenario::next_tx(&mut scen, UNDERWRITER);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let deposit = coin::mint_for_testing<MYSO>(5_000 * SCALING, test_scenario::ctx(&mut scen));
            insurance::deposit_capital(&config, &mut vault, deposit, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let router_cfg = test_scenario::take_shared<insurance::InsuranceRouterConfig>(&scen);
            let mut backstop = test_scenario::take_shared<insurance::InsuranceBackstopPool>(&scen);
            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let payment = coin::mint_for_testing<MYSO>(500 * SCALING, test_scenario::ctx(&mut scen));

            insurance::buy_coverage(
                &config,
                &router_cfg,
                &mut backstop,
                &spot_config,
                &mut vault,
                &record,
                0,
                1000 * SCALING,
                8000,
                3 * DAY_MS,
                payment,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(router_cfg);
            test_scenario::return_shared(backstop);
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
        };

        resolve_spot_market_yes(&mut scen, 1);

        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut policy_id_opt = test_scenario::most_recent_id_shared<insurance::CoveragePolicy>();
            assert!(option::is_some(&policy_id_opt), 1);
            let policy_id = option::extract(&mut policy_id_opt);
            let mut policy = test_scenario::take_shared_by_id<insurance::CoveragePolicy>(&scen, policy_id);

            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            insurance::claim(&config, &spot_config, &mut vault, &record, &mut policy, &clock, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(spot_config);

            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(policy);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_cancel_coverage() {
        let mut scen = setup_env_with_insurance();
        create_open_market_with_bet(&mut scen, USER1, 1000 * SCALING, 0);
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        // Buy coverage
        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let router_cfg = test_scenario::take_shared<insurance::InsuranceRouterConfig>(&scen);
            let mut backstop = test_scenario::take_shared<insurance::InsuranceBackstopPool>(&scen);
            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let payment = coin::mint_for_testing<MYSO>(500 * SCALING, test_scenario::ctx(&mut scen));

            insurance::buy_coverage(
                &config,
                &router_cfg,
                &mut backstop,
                &spot_config,
                &mut vault,
                &record,
                0,
                1000 * SCALING,
                8000,
                3 * DAY_MS,
                payment,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(router_cfg);
            test_scenario::return_shared(backstop);
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
        };

        // Advance clock by 1 day (1/3 of policy duration)
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let mut clock = test_scenario::take_shared<Clock>(&scen);
            clock::increment_for_testing(&mut clock, DAY_MS);
            test_scenario::return_shared(clock);
        };

        // Cancel coverage - should get 2/3 refund minus fee
        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut policy_id_opt = test_scenario::most_recent_id_shared<insurance::CoveragePolicy>();
            assert!(option::is_some(&policy_id_opt), 1);
            let policy_id = option::extract(&mut policy_id_opt);
            let mut policy = test_scenario::take_shared_by_id<insurance::CoveragePolicy>(&scen, policy_id);

            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            insurance::cancel_coverage(
                &config,
                &spot_config,
                &treasury,
                &mut vault,
                &record,
                &mut policy,
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(treasury);

            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(policy);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_withdraw_capital() {
        let mut scen = setup_env_with_insurance();
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        // Withdraw some unreserved capital
        test_scenario::next_tx(&mut scen, UNDERWRITER);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            insurance::withdraw_capital(
                &config,
                &mut vault,
                2_000 * SCALING,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_expire_policy() {
        let mut scen = setup_env_with_insurance();
        create_open_market_with_bet(&mut scen, USER1, 1000 * SCALING, 0);
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        // Buy coverage with 1 day duration
        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let router_cfg = test_scenario::take_shared<insurance::InsuranceRouterConfig>(&scen);
            let mut backstop = test_scenario::take_shared<insurance::InsuranceBackstopPool>(&scen);
            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let payment = coin::mint_for_testing<MYSO>(500 * SCALING, test_scenario::ctx(&mut scen));

            insurance::buy_coverage(
                &config,
                &router_cfg,
                &mut backstop,
                &spot_config,
                &mut vault,
                &record,
                0,
                1000 * SCALING,
                8000,
                DAY_MS,
                payment,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(router_cfg);
            test_scenario::return_shared(backstop);
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
        };

        // Advance clock past expiry time
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let mut clock = test_scenario::take_shared<Clock>(&scen);
            clock::increment_for_testing(&mut clock, DAY_MS + 1000); // 1 day + 1 second
            test_scenario::return_shared(clock);
        };

        // Expire the policy
        test_scenario::next_tx(&mut scen, USER1);
        {
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut policy_id_opt = test_scenario::most_recent_id_shared<insurance::CoveragePolicy>();
            assert!(option::is_some(&policy_id_opt), 1);
            let policy_id = option::extract(&mut policy_id_opt);
            let mut policy = test_scenario::take_shared_by_id<insurance::CoveragePolicy>(&scen, policy_id);

            insurance::expire_policy(&mut vault, &mut policy, &clock);

            test_scenario::return_shared(vault);
            test_scenario::return_shared(policy);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_quote_premium() {
        let mut scen = setup_env_with_insurance();
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);

            let premium1 = insurance::quote_premium(&vault, 1000 * SCALING, 8000, DAY_MS);
            assert!(premium1 > 0, 1);

            let premium2 = insurance::quote_premium(&vault, 1000 * SCALING, 8000, 2 * DAY_MS);
            assert!(premium2 > premium1, 2);

            let premium3 = insurance::quote_premium(&vault, 2000 * SCALING, 8000, DAY_MS);
            assert!(premium3 > premium1, 3);

            test_scenario::return_shared(vault);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_total_option_escrow_sums_escrows() {
        let mut scen = setup_env();
        create_post_and_spot_market(&mut scen);
        place_spot_bet_amt(&mut scen, USER1, 0, 777 * SCALING);
        place_spot_bet_amt(&mut scen, USER1, 1, 333 * SCALING);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let rec = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let a0 = spot::get_option_escrow(&rec, 0);
            let a1 = spot::get_option_escrow(&rec, 1);
            assert!(spot::total_option_escrow(&rec) == a0 + a1, 1);
            assert!(spot::num_betting_options(&rec) == 2, 2);
            test_scenario::return_shared(rec);
        };

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = 18, location = social_contracts::insurance)]
    fun test_buy_coverage_aborts_thin_market() {
        let mut scen = setup_env_with_insurance();
        insurance_set_min_spot_liquidity(&mut scen, 5_000 * SCALING);
        create_open_market_with_bet(&mut scen, USER1, 1_000 * SCALING, 0);
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let router_cfg = test_scenario::take_shared<insurance::InsuranceRouterConfig>(&scen);
            let mut backstop = test_scenario::take_shared<insurance::InsuranceBackstopPool>(&scen);
            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let payment = coin::mint_for_testing<MYSO>(500 * SCALING, test_scenario::ctx(&mut scen));

            insurance::buy_coverage(
                &config,
                &router_cfg,
                &mut backstop,
                &spot_config,
                &mut vault,
                &record,
                0,
                1_000 * SCALING,
                8000,
                3 * DAY_MS,
                payment,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(router_cfg);
            test_scenario::return_shared(backstop);
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = 19, location = social_contracts::insurance)]
    fun test_buy_coverage_aborts_pool_fraction_cap() {
        let mut scen = setup_env_with_insurance();
        insurance_set_max_coverage_fraction_only(&mut scen, 4_000);
        create_open_market_with_bet(&mut scen, USER1, 1_000 * SCALING, 0);
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let router_cfg = test_scenario::take_shared<insurance::InsuranceRouterConfig>(&scen);
            let mut backstop = test_scenario::take_shared<insurance::InsuranceBackstopPool>(&scen);
            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let payment = coin::mint_for_testing<MYSO>(500 * SCALING, test_scenario::ctx(&mut scen));

            insurance::buy_coverage(
                &config,
                &router_cfg,
                &mut backstop,
                &spot_config,
                &mut vault,
                &record,
                0,
                1_000 * SCALING,
                8000,
                3 * DAY_MS,
                payment,
                &clock,
                test_scenario::ctx(&mut scen)
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(router_cfg);
            test_scenario::return_shared(backstop);
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_underdog_option_higher_quote_than_favorite_same_market() {
        let mut scen = setup_env_with_insurance();
        create_post_and_spot_market(&mut scen);
        place_spot_bet_amt(&mut scen, USER1, 0, 900 * SCALING);
        place_spot_bet_amt(&mut scen, USER1, 1, 100 * SCALING);
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let q_fav =
                insurance::quote_premium_with_spot_risk(&config, &vault, &record, 0, 50 * SCALING, 8000, DAY_MS);
            let q_dog =
                insurance::quote_premium_with_spot_risk(&config, &vault, &record, 1, 50 * SCALING, 8000, DAY_MS);

            assert!(insurance::premium_quote_premium(&q_dog) > insurance::premium_quote_premium(&q_fav) * 2, 1);
            assert!(insurance::premium_quote_implied_prob_win_bps(&q_fav) > insurance::premium_quote_implied_prob_win_bps(&q_dog), 2);

            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_balanced_market_near_unit_risk_multiplier() {
        let mut scen = setup_env_with_insurance();
        create_post_and_spot_market(&mut scen);
        place_spot_bet_amt(&mut scen, USER1, 0, 500 * SCALING);
        place_spot_bet_amt(&mut scen, USER1, 1, 500 * SCALING);
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let pq =
                insurance::quote_premium_with_spot_risk(&config, &vault, &record, 0, 50 * SCALING, 8000, DAY_MS);
            assert!(insurance::premium_quote_risk_multiplier_bps(&pq) >= 9000 && insurance::premium_quote_risk_multiplier_bps(&pq) <= 12_500, 1);
            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_min_premium_floor_in_quote_and_buy_coverage() {
        let mut scen = setup_env_with_insurance();
        insurance_set_min_premium_only(&mut scen, 10 * SCALING);
        create_open_market_with_bet(&mut scen, USER1, 1_000 * SCALING, 0);
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let pq = insurance::quote_premium_with_spot_risk(
                &config,
                &vault,
                &record,
                0,
                50 * SCALING,
                1000,
                1,
            );
            assert!(insurance::premium_quote_premium(&pq) >= 10 * SCALING, 1);
            assert!(insurance::premium_quote_premium_raw(&pq) < insurance::premium_quote_premium(&pq), 2);
            test_scenario::return_shared(config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let router_cfg = test_scenario::take_shared<insurance::InsuranceRouterConfig>(&scen);
            let mut backstop = test_scenario::take_shared<insurance::InsuranceBackstopPool>(&scen);
            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut vault = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let payment = coin::mint_for_testing<MYSO>(10 * SCALING, test_scenario::ctx(&mut scen));

            insurance::buy_coverage(
                &config,
                &router_cfg,
                &mut backstop,
                &spot_config,
                &mut vault,
                &record,
                0,
                1_000 * SCALING,
                1000,
                1,
                payment,
                &clock,
                test_scenario::ctx(&mut scen),
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(router_cfg);
            test_scenario::return_shared(backstop);
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(vault);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_route_buy_coverage_two_vaults() {
        let mut scen = setup_env_with_insurance();
        create_open_market_with_bet(&mut scen, USER1, 1_000 * SCALING, 0);
        setup_vault_with_capital(&mut scen, 5_000 * SCALING);

        test_scenario::next_tx(&mut scen, USER1);
        {
            insurance::create_vault(25, 5000, 0, 0, test_scenario::ctx(&mut scen));
        };
        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vx = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let deposit = coin::mint_for_testing<MYSO>(5_000 * SCALING, test_scenario::ctx(&mut scen));
            insurance::deposit_capital(&config, &mut vx, deposit, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(config);
            test_scenario::return_shared(vx);
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            insurance::create_vault(25, 5000, 0, 0, test_scenario::ctx(&mut scen));
        };
        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vx = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let deposit = coin::mint_for_testing<MYSO>(5_000 * SCALING, test_scenario::ctx(&mut scen));
            insurance::deposit_capital(&config, &mut vx, deposit, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(config);
            test_scenario::return_shared(vx);
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            insurance::create_vault(25, 5000, 0, 0, test_scenario::ctx(&mut scen));
        };
        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let mut vx = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let deposit = coin::mint_for_testing<MYSO>(5_000 * SCALING, test_scenario::ctx(&mut scen));
            insurance::deposit_capital(&config, &mut vx, deposit, test_scenario::ctx(&mut scen));
            test_scenario::return_shared(config);
            test_scenario::return_shared(vx);
        };

        test_scenario::next_tx(&mut scen, USER1);
        {
            let config = test_scenario::take_shared<insurance::InsuranceConfig>(&scen);
            let router_cfg = test_scenario::take_shared<insurance::InsuranceRouterConfig>(&scen);
            let mut backstop = test_scenario::take_shared<insurance::InsuranceBackstopPool>(&scen);
            let spot_config = test_scenario::take_shared<spot::SpotConfig>(&scen);
            let mut v0 = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let mut v1 = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let mut v2 = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let mut v3 = test_scenario::take_shared<insurance::UnderwriterVault>(&scen);
            let record = test_scenario::take_shared<spot::SpotMarket>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let payment = coin::mint_for_testing<MYSO>(10_000 * SCALING, test_scenario::ctx(&mut scen));
            let deadline_ms = clock::timestamp_ms(&clock) + 3_600_000;

            insurance::route_buy_coverage_4(
                &config,
                &router_cfg,
                &mut backstop,
                &spot_config,
                &record,
                &mut v0,
                &mut v1,
                &mut v2,
                &mut v3,
                0,
                400 * SCALING,
                600 * SCALING,
                0,
                0,
                8000,
                3 * DAY_MS,
                deadline_ms,
                1_000 * SCALING,
                10_000 * SCALING,
                payment,
                &clock,
                test_scenario::ctx(&mut scen),
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(router_cfg);
            test_scenario::return_shared(backstop);
            test_scenario::return_shared(spot_config);
            test_scenario::return_shared(v0);
            test_scenario::return_shared(v1);
            test_scenario::return_shared(v2);
            test_scenario::return_shared(v3);
            test_scenario::return_shared(record);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }
}
