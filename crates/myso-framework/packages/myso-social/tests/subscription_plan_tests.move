// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias)]
module social_contracts::subscription_plan_tests {
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
        TestPlanRef,
    };
    use social_contracts::post::{Self, Post};

    const CREATOR: address = @0xC1;
    const SUBSCRIBER: address = @0x2;
    const MONTHLY_MS: u64 = 2_592_000_000;
    const ANNUAL_MS: u64 = 31_536_000_000;
    const BASIC_PRICE: u64 = 1_000;
    const PREMIUM_PRICE: u64 = 2_000;

    fun init_env(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            block_list::test_init(&clock, test_scenario::ctx(scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(scenario));
            subscription::test_init(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
        };
    }

    #[test]
    fun test_plan_duration_zero_uses_config_default_and_positive_is_preserved() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let config = test_scenario::take_shared<SubscriptionConfig>(&scenario);
            assert_eq!(
                subscription::resolve_plan_duration_ms_for_testing(&config, 0),
                MONTHLY_MS,
            );
            assert_eq!(
                subscription::resolve_plan_duration_ms_for_testing(&config, ANNUAL_MS),
                ANNUAL_MS,
            );
            test_scenario::return_shared(config);
        };

        test_scenario::end(scenario);
    }

    fun share_service_with_two_plans(scenario: &mut test_scenario::Scenario): (ID, ID) {
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
            let monthly_plan = subscription::test_create_plan(
                &mut service,
                b"Monthly Basic",
                BASIC_PRICE,
                MONTHLY_MS,
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            let annual_plan = subscription::test_create_plan(
                &mut service,
                b"Annual Basic",
                BASIC_PRICE * 10,
                ANNUAL_MS,
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(service);
            test_scenario::return_shared(clock);
            (monthly_plan, annual_plan)
        }
    }

    fun subscribe_with_plan(
        scenario: &mut test_scenario::Scenario,
        plan_id: ID,
        price: u64,
    ) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let payment = coin::mint_for_testing<MYSO>(price, test_scenario::ctx(scenario));
            transfer::public_transfer(payment, SUBSCRIBER);
        };
        test_scenario::next_tx(scenario, SUBSCRIBER);
        {
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(scenario);
            let config = test_scenario::take_shared<SubscriptionConfig>(scenario);
            let mut service = test_scenario::take_shared<ProfileSubscriptionService>(scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            let mut payment = test_scenario::take_from_sender<Coin<MYSO>>(scenario);

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
                test_scenario::ctx(scenario),
            );

            transfer::public_transfer(payment, SUBSCRIBER);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(config);
            test_scenario::return_shared(service);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(clock);
        };
    }

    #[test]
    fun test_monthly_and_annual_plans_differ_expiry() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);
        let (monthly_plan, annual_plan) = share_service_with_two_plans(&mut scenario);

        subscribe_with_plan(&mut scenario, monthly_plan, BASIC_PRICE);
        test_scenario::next_tx(&mut scenario, SUBSCRIBER);
        {
            let subscription = test_scenario::take_from_sender<ProfileSubscription>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let now = clock::timestamp_ms(&clock);
            let expires = subscription::subscription_expires_at(&subscription);
            assert_eq!(expires, now + MONTHLY_MS);
            test_scenario::return_to_sender(&scenario, subscription);
            test_scenario::return_shared(clock);
        };

        subscribe_with_plan(&mut scenario, annual_plan, BASIC_PRICE * 10);
        test_scenario::next_tx(&mut scenario, SUBSCRIBER);
        {
            let subscription = test_scenario::take_from_sender<ProfileSubscription>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let now = clock::timestamp_ms(&clock);
            let expires = subscription::subscription_expires_at(&subscription);
            assert_eq!(expires, now + ANNUAL_MS);
            test_scenario::return_to_sender(&scenario, subscription);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    fun share_tiered_service(scenario: &mut test_scenario::Scenario): (ID, ID) {
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
            let basic_plan = subscription::test_create_plan(
                &mut service,
                b"Basic",
                BASIC_PRICE,
                MONTHLY_MS,
                option::some(0),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            let premium_plan = subscription::test_create_plan(
                &mut service,
                b"Premium",
                PREMIUM_PRICE,
                MONTHLY_MS,
                option::some(1),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(service);
            test_scenario::return_shared(clock);
            (basic_plan, premium_plan)
        }
    }

    fun share_premium_post(scenario: &mut test_scenario::Scenario, service_id: ID) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(scenario);
            let _post_addr = post::test_share_profile_subscription_post_with_tier(
                CREATOR,
                service_id,
                option::some(1),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
        };
    }

    #[test]
    fun test_premium_subscriber_views_premium_post() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);
        let (_basic_plan, premium_plan) = share_tiered_service(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let service_id = object::id(&service);
            test_scenario::return_shared(service);
            share_premium_post(&mut scenario, service_id);
        };

        subscribe_with_plan(&mut scenario, premium_plan, PREMIUM_PRICE);

        test_scenario::next_tx(&mut scenario, SUBSCRIBER);
        {
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let post = test_scenario::take_shared<Post>(&scenario);
            let service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let subscription = test_scenario::take_from_sender<ProfileSubscription>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            post::assert_can_view_post(
                &block_list_registry,
                &post,
                &service,
                &subscription,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_to_sender(&scenario, subscription);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(post);
            test_scenario::return_shared(service);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 0, location = social_contracts::post)]
    fun test_basic_subscriber_cannot_view_premium_post() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);
        let (basic_plan, _premium_plan) = share_tiered_service(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let service_id = object::id(&service);
            test_scenario::return_shared(service);
            share_premium_post(&mut scenario, service_id);
        };

        subscribe_with_plan(&mut scenario, basic_plan, BASIC_PRICE);

        test_scenario::next_tx(&mut scenario, SUBSCRIBER);
        {
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let post = test_scenario::take_shared<Post>(&scenario);
            let service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let subscription = test_scenario::take_from_sender<ProfileSubscription>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);

            post::assert_can_view_post(
                &block_list_registry,
                &post,
                &service,
                &subscription,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_to_sender(&scenario, subscription);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(post);
            test_scenario::return_shared(service);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }
}
