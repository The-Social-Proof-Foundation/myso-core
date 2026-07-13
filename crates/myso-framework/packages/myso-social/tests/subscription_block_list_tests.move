// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias)]
module social_contracts::subscription_block_list_tests {
    use myso::test_scenario;
    use myso::clock::{Self, Clock};
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::transfer;
    use myso::object;

    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::social_graph;
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
    const PLAN_PRICE: u64 = 1_000;
    const THIRTY_DAYS_MS: u64 = 2_592_000_000;

    fun init_env(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            block_list::test_init(&clock, test_scenario::ctx(scenario));
            social_graph::init_for_testing(&clock, test_scenario::ctx(scenario));
            profile::init_for_testing(&clock, test_scenario::ctx(scenario));
            subscription::test_init(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
        };
    }

    fun share_creator_service(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(scenario);
            subscription::test_share_service_with_plan(
                CREATOR,
                object::id_from_address(CREATOR),
                SUBSCRIBER,
                PLAN_PRICE,
                THIRTY_DAYS_MS,
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(clock);
        };
    }

    fun fund_subscriber(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let payment = coin::mint_for_testing<MYSO>(PLAN_PRICE * 2, test_scenario::ctx(scenario));
            transfer::public_transfer(payment, SUBSCRIBER);
        };
    }

    fun subscribe_as_subscriber(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, SUBSCRIBER);
        {
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(scenario);
            let config = test_scenario::take_shared<SubscriptionConfig>(scenario);
            let mut service = test_scenario::take_shared<ProfileSubscriptionService>(scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);
            let mut payment = test_scenario::take_from_sender<Coin<MYSO>>(scenario);
            let plan_ref = test_scenario::take_from_sender<TestPlanRef>(scenario);
            let plan_id = subscription::test_take_plan_id(plan_ref);

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

    fun block_creator_blocks_subscriber(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let mut block_list_registry = test_scenario::take_shared<BlockListRegistry>(scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(scenario);
            social_graph::block_wallet(
                &mut block_list_registry,
                &mut social_graph,
                SUBSCRIBER,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(social_graph);
        };
    }

    fun block_subscriber_blocks_creator(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, SUBSCRIBER);
        {
            let mut block_list_registry = test_scenario::take_shared<BlockListRegistry>(scenario);
            let mut social_graph = test_scenario::take_shared<social_graph::SocialGraph>(scenario);
            social_graph::block_wallet(
                &mut block_list_registry,
                &mut social_graph,
                CREATOR,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(social_graph);
        };
    }

    fun unblock_creator_unblocks_subscriber(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let mut block_list_registry = test_scenario::take_shared<BlockListRegistry>(scenario);
            social_graph::unblock_wallet(
                &mut block_list_registry,
                SUBSCRIBER,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(block_list_registry);
        };
    }

    #[test]
    #[expected_failure(abort_code = 5, location = social_contracts::block_list)]
    fun test_creator_blocks_subscriber_rejects_subscribe() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);
        share_creator_service(&mut scenario);
        fund_subscriber(&mut scenario);
        block_creator_blocks_subscriber(&mut scenario);
        subscribe_as_subscriber(&mut scenario);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 5, location = social_contracts::block_list)]
    fun test_subscriber_blocks_creator_rejects_subscribe() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);
        share_creator_service(&mut scenario);
        fund_subscriber(&mut scenario);
        block_subscriber_blocks_creator(&mut scenario);
        subscribe_as_subscriber(&mut scenario);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_unblock_restores_subscribe() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);
        share_creator_service(&mut scenario);
        fund_subscriber(&mut scenario);
        block_creator_blocks_subscriber(&mut scenario);
        unblock_creator_unblocks_subscriber(&mut scenario);
        subscribe_as_subscriber(&mut scenario);

        test_scenario::next_tx(&mut scenario, SUBSCRIBER);
        {
            let _subscription = test_scenario::take_from_sender<ProfileSubscription>(&scenario);
            test_scenario::return_to_sender(&scenario, _subscription);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 5, location = social_contracts::block_list)]
    fun test_block_rejects_renew_subscription() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);
        share_creator_service(&mut scenario);
        fund_subscriber(&mut scenario);
        subscribe_as_subscriber(&mut scenario);
        block_creator_blocks_subscriber(&mut scenario);

        test_scenario::next_tx(&mut scenario, SUBSCRIBER);
        {
            let block_list_registry = test_scenario::take_shared<BlockListRegistry>(&scenario);
            let config = test_scenario::take_shared<SubscriptionConfig>(&scenario);
            let service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let mut subscription = test_scenario::take_from_sender<ProfileSubscription>(&scenario);
            let payment = coin::mint_for_testing<MYSO>(PLAN_PRICE, test_scenario::ctx(&mut scenario));

            subscription::renew_subscription(
                &block_list_registry,
                &config,
                &service,
                &mut subscription,
                &treasury,
                payment,
                &clock,
                test_scenario::ctx(&mut scenario),
            );

            test_scenario::return_to_sender(&scenario, subscription);
            test_scenario::return_shared(block_list_registry);
            test_scenario::return_shared(config);
            test_scenario::return_shared(service);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 5, location = social_contracts::block_list)]
    fun test_block_rejects_assert_can_view_post() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_env(&mut scenario);
        share_creator_service(&mut scenario);
        fund_subscriber(&mut scenario);
        subscribe_as_subscriber(&mut scenario);

        test_scenario::next_tx(&mut scenario, CREATOR);
        {
            let service = test_scenario::take_shared<ProfileSubscriptionService>(&scenario);
            let clock = test_scenario::take_shared<Clock>(&scenario);
            let service_id = object::id(&service);
            post::test_share_profile_subscription_post(CREATOR, service_id, &clock, test_scenario::ctx(&mut scenario));
            test_scenario::return_shared(service);
            test_scenario::return_shared(clock);
        };

        block_creator_blocks_subscriber(&mut scenario);

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
