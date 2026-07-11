// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias)]
module social_contracts::access_hardening_tests {
    use std::string;
    use std::option;

    use myso::test_scenario;
    use myso::clock::{Self, Clock};
    use myso::object;

    use social_contracts::mydata::{Self, MyData, MyDataConfig, MyDataRegistry};
    use social_contracts::post::{Self};

    const CREATOR: address = @0x1;

    fun init_mydata_env(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            mydata::test_init(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
        };
    }

    fun create_profile_subscription_mydata(scenario: &mut test_scenario::Scenario): MyData {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let config = test_scenario::take_shared<MyDataConfig>(scenario);
            let mut registry = test_scenario::take_shared<MyDataRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            mydata::create_and_share_profile_subscription_mydata(
                &config,
                &mut registry,
                string::utf8(b"data"),
                vector[string::utf8(b"premium")],
                option::none<address>(),
                1000,
                option::none<u64>(),
                b"encrypted_profile_data",
                b"encryption_profile",
                option::none<string::String>(),
                option::none<string::String>(),
                option::none<u64>(),
                option::none<string::String>(),
                false,
                option::none<string::String>(),
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(scenario, CREATOR);
        test_scenario::take_shared<MyData>(scenario)
    }

    fun create_marketplace_one_time_mydata(scenario: &mut test_scenario::Scenario): MyData {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let config = test_scenario::take_shared<MyDataConfig>(scenario);
            let mut registry = test_scenario::take_shared<MyDataRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            mydata::create_and_share_marketplace_one_time_mydata(
                &config,
                &mut registry,
                string::utf8(b"data"),
                vector[string::utf8(b"paid")],
                option::none<address>(),
                1000,
                option::none<u64>(),
                b"encrypted_paid_data",
                b"encryption_paid",
                500,
                option::none<string::String>(),
                option::none<string::String>(),
                option::none<u64>(),
                option::none<string::String>(),
                false,
                option::none<string::String>(),
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(scenario, CREATOR);
        test_scenario::take_shared<MyData>(scenario)
    }

    #[test]
    fun test_profile_subscription_access_matches_profile_mydata() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_mydata_env(&mut scenario);
        let mydata = create_profile_subscription_mydata(&mut scenario);

        let access = post::test_post_access_profile_subscription(
            object::id_from_address(@0xABC),
            option::some(object::id(&mydata)),
            option::none(),
        );
        post::test_assert_post_access_mydata_object_binding(access, &mydata);

        mydata::test_destroy(mydata);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 22, location = social_contracts::post)]
    fun test_profile_subscription_post_rejects_marketplace_one_time_mydata() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_mydata_env(&mut scenario);
        let mydata = create_marketplace_one_time_mydata(&mut scenario);

        let access = post::test_post_access_profile_subscription(
            object::id_from_address(@0xABC),
            option::some(object::id(&mydata)),
            option::none(),
        );
        post::test_assert_post_access_mydata_object_binding(access, &mydata);

        mydata::test_destroy(mydata);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 23, location = social_contracts::post)]
    fun test_marketplace_one_time_post_rejects_profile_subscription_mydata() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_mydata_env(&mut scenario);
        let mydata = create_profile_subscription_mydata(&mut scenario);

        let access = post::test_post_access_marketplace_one_time(object::id(&mydata));
        post::test_assert_post_access_mydata_object_binding(access, &mydata);

        mydata::test_destroy(mydata);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_marketplace_one_time_access_matches_one_time_mydata() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_mydata_env(&mut scenario);
        let mydata = create_marketplace_one_time_mydata(&mut scenario);

        let access = post::test_post_access_marketplace_one_time(object::id(&mydata));
        post::test_assert_post_access_mydata_object_binding(access, &mydata);

        mydata::test_destroy(mydata);
        test_scenario::end(scenario);
    }

    #[test]
    #[expected_failure(abort_code = 22, location = social_contracts::post)]
    fun test_marketplace_recurring_mydata_rejected_for_post_link() {
        let mut scenario = test_scenario::begin(CREATOR);
        init_mydata_env(&mut scenario);
        let mydata = create_marketplace_recurring_mydata(&mut scenario);

        let access = post::test_post_access_marketplace_one_time(object::id(&mydata));
        post::test_assert_post_access_mydata_object_binding(access, &mydata);

        mydata::test_destroy(mydata);
        test_scenario::end(scenario);
    }

    fun create_marketplace_recurring_mydata(scenario: &mut test_scenario::Scenario): MyData {
        test_scenario::next_tx(scenario, CREATOR);
        {
            let config = test_scenario::take_shared<MyDataConfig>(scenario);
            let mut registry = test_scenario::take_shared<MyDataRegistry>(scenario);
            let clock = test_scenario::take_shared<Clock>(scenario);

            mydata::create_and_share_marketplace_recurring_mydata(
                &config,
                &mut registry,
                string::utf8(b"data"),
                vector[string::utf8(b"recurring")],
                option::none<address>(),
                1000,
                option::none<u64>(),
                b"encrypted_recurring_data",
                b"encryption_recurring",
                500,
                30,
                option::none<string::String>(),
                option::none<string::String>(),
                option::none<u64>(),
                option::none<string::String>(),
                false,
                option::none<string::String>(),
                &clock,
                test_scenario::ctx(scenario),
            );

            test_scenario::return_shared(config);
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(scenario, CREATOR);
        test_scenario::take_shared<MyData>(scenario)
    }
}
