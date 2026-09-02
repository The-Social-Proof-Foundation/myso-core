// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias)]
module social_contracts::subscription_multi_coin_tests {
    use std::option;
    use std::unit_test::assert_eq;

    use myso::test_scenario;
    use myso::clock::{Self, Clock};
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::transfer;
    use myso::object;

    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::profile::{Self, EcosystemTreasury};
    use social_contracts::subscription::{
        Self,
        SubscriptionConfig,
        ProfileSubscriptionService,
        ProfileSubscription,
    };
    use social_contracts::platform::{Self, Platform, PlatformConfig, PlatformRegistry};

    const CREATOR: address = @0xC1;
    const SUBSCRIBER: address = @0x2;
    const MONTHLY_MS: u64 = 2_592_000_000;
    const MYSO_PRICE: u64 = 1_000;
    const USD_PRICE: u64 = 3_990_000;

    public struct TEST_USD has drop {}

    fun init_env(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            block_list::test_init(&clock, test_scenario::ctx(scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(scenario));
            subscription::test_init(&clock, test_scenario::ctx(scenario));
            platform::test_init(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
        };
    }

    fun create_platform(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let platform_config = test_scenario::take_shared<PlatformConfig>(scenario);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            platform::create_platform(
                &mut registry,
                &platform_config,
                std::string::utf8(b"DripDrop"),
                std::string::utf8(b"tag"),
                std::string::utf8(b"desc"),
                std::string::utf8(b"https://example.com/logo.png"),
                std::string::utf8(b"https://example.com/terms"),
                std::string::utf8(b"https://example.com/privacy"),
                vector[std::string::utf8(b"web")],
                vector[std::string::utf8(b"https://example.com")],
                std::string::utf8(b"Social Network"),
                option::none(),
                2,
                std::string::utf8(b"2026-01-01"),
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
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };
    }

    fun share_service_with_usd_plan(scenario: &mut test_scenario::Scenario): object::ID {
        test_scenario::next_tx(scenario, CREATOR);
        {
            subscription::test_share_empty_service(
                CREATOR,
                object::id_from_address(CREATOR),
                test_scenario::ctx(scenario),
            );
        };
        test_scenario::next_tx(scenario, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(scenario);
            let mut service = test_scenario::take_shared<ProfileSubscriptionService>(scenario);
            let plan_id = subscription::test_create_plan_with_coin<TEST_USD>(
                &mut service,
                b"USD Monthly",
                USD_PRICE,
                MONTHLY_MS,
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(service);
            test_scenario::return_shared(clock);
            plan_id
        }
    }

    #[test]
    fun test_subscribe_and_cancel_usd_plan() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);
        let plan_id = share_service_with_usd_plan(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let payment = coin::mint_for_testing<TEST_USD>(USD_PRICE, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(payment, SUBSCRIBER);
        };

        test_scenario::next_tx(&mut scenario, SUBSCRIBER);
        {
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let config = test_scenario::take_shared<SubscriptionConfig>(&scenario);
            let mut service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut payment = test_scenario::take_from_sender<Coin<TEST_USD>>(&scenario);

            subscription::subscribe_to_profile(
                &block_list_registry,
                &config,
                &mut service,
                plan_id,
                &treasury,
                &mut payment,
                false,
                0,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            coin::destroy_zero(payment);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(config);
            test_scenario::return_shared(service);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scenario, SUBSCRIBER);
        {
            let mut service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let subscription = test_scenario::take_from_sender<ProfileSubscription>(&scenario);
            assert_eq!(subscription::subscription_renewal_balance<TEST_USD>(&subscription), 0);
            subscription::cancel_subscription<TEST_USD>(
                &mut service,
                subscription,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(service);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_usd_subscribe_with_platform_funds_usd_treasury() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);
        create_platform(&mut scenario);
        let plan_id = share_service_with_usd_plan(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let payment = coin::mint_for_testing<TEST_USD>(USD_PRICE, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(payment, SUBSCRIBER);
        };

        test_scenario::next_tx(&mut scenario, SUBSCRIBER);
        {
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let config = test_scenario::take_shared<SubscriptionConfig>(&scenario);
            let mut service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            let mut platform = test_scenario::take_shared<Platform>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut payment = test_scenario::take_from_sender<Coin<TEST_USD>>(&scenario);

            subscription::subscribe_to_profile_with_platform(
                &block_list_registry,
                &config,
                &mut service,
                plan_id,
                &treasury,
                &mut platform,
                &mut payment,
                false,
                0,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            let expected_platform_fee = (USD_PRICE * 250) / 10_000;
            assert_eq!(platform::treasury_balance<TEST_USD>(&platform), expected_platform_fee);
            assert_eq!(platform::treasury_balance<MYSO>(&platform), 0);

            coin::destroy_zero(payment);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(config);
            test_scenario::return_shared(service);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(platform);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    fun test_myso_and_usd_plans_coexist() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            subscription::test_share_empty_service(
                CREATOR,
                object::id_from_address(CREATOR),
                test_scenario::ctx(&mut scenario),
            );
        };

        test_scenario::next_tx(&mut scenario, CREATOR);
        let (myso_plan, usd_plan) = {
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let myso_plan = subscription::test_create_plan(
                &mut service,
                b"MYSO Monthly",
                MYSO_PRICE,
                MONTHLY_MS,
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            let usd_plan = subscription::test_create_plan_with_coin<TEST_USD>(
                &mut service,
                b"USD Monthly",
                USD_PRICE,
                MONTHLY_MS,
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            test_scenario::return_shared(service);
            test_scenario::return_shared(clock);
            (myso_plan, usd_plan)
        };

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let myso_pay = coin::mint_for_testing<MYSO>(MYSO_PRICE, test_scenario::ctx(&mut scenario));
            let usd_pay = coin::mint_for_testing<TEST_USD>(USD_PRICE, test_scenario::ctx(&mut scenario));
            transfer::public_transfer(myso_pay, SUBSCRIBER);
            transfer::public_transfer(usd_pay, SUBSCRIBER);
        };

        test_scenario::next_tx(&mut scenario, SUBSCRIBER);
        {
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let config = test_scenario::take_shared<SubscriptionConfig>(&scenario);
            let mut service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut myso_pay = test_scenario::take_from_sender<Coin<MYSO>>(&scenario);
            let mut usd_pay = test_scenario::take_from_sender<Coin<TEST_USD>>(&scenario);

            subscription::subscribe_to_profile(
                &block_list_registry,
                &config,
                &mut service,
                myso_plan,
                &treasury,
                &mut myso_pay,
                false,
                0,
                &clock,
                test_scenario::ctx(&mut scenario),
            );
            subscription::subscribe_to_profile(
                &block_list_registry,
                &config,
                &mut service,
                usd_plan,
                &treasury,
                &mut usd_pay,
                false,
                0,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            coin::destroy_zero(myso_pay);
            coin::destroy_zero(usd_pay);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(config);
            test_scenario::return_shared(service);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }
}
