#[test_only]
#[allow(duplicate_alias)]
module social_contracts::mydata_fee_tests {
    use std::unit_test::assert_eq;
    use myso::test_scenario;
    use social_contracts::mydata::{Self, MyDataConfig};

    #[test]
    fun test_default_p2p_fee_breakdown_no_platform() {
        let mut scenario = test_scenario::begin(@0xA);
        let clock = myso::clock::create_for_testing(test_scenario::ctx(&mut scenario));
        mydata::test_init(&clock, test_scenario::ctx(&mut scenario));
        myso::clock::destroy_for_testing(clock);

        test_scenario::next_tx(&mut scenario, @0xA);
        let config = test_scenario::take_shared<MyDataConfig>(&scenario);
        let (platform_fee, ecosystem_fee, creator_amount) =
            mydata::p2p_fee_breakdown_no_platform_for_testing(&config, 10_000);
        assert_eq!(platform_fee, 0);
        assert_eq!(ecosystem_fee, 250);
        assert_eq!(creator_amount, 9_750);
        test_scenario::return_shared(config);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_default_p2p_fee_breakdown_with_platform() {
        let mut scenario = test_scenario::begin(@0xA);
        let clock = myso::clock::create_for_testing(test_scenario::ctx(&mut scenario));
        mydata::test_init(&clock, test_scenario::ctx(&mut scenario));
        myso::clock::destroy_for_testing(clock);

        test_scenario::next_tx(&mut scenario, @0xA);
        let config = test_scenario::take_shared<MyDataConfig>(&scenario);
        let (platform_fee, ecosystem_fee, creator_amount) =
            mydata::p2p_fee_breakdown_with_platform_for_testing(&config, 10_000);
        assert_eq!(platform_fee, 250);
        assert_eq!(ecosystem_fee, 250);
        assert_eq!(creator_amount, 9_500);
        test_scenario::return_shared(config);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_default_mydata_marketplace_fee_breakdown_no_platform() {
        let mut scenario = test_scenario::begin(@0xA);
        let clock = myso::clock::create_for_testing(test_scenario::ctx(&mut scenario));
        mydata::test_init(&clock, test_scenario::ctx(&mut scenario));
        myso::clock::destroy_for_testing(clock);

        test_scenario::next_tx(&mut scenario, @0xA);
        let config = test_scenario::take_shared<MyDataConfig>(&scenario);
        let (platform_fee, ecosystem_fee, net_amount) =
            mydata::mydata_marketplace_fee_breakdown_no_platform_for_testing(&config, 10_000);
        assert_eq!(platform_fee, 0);
        assert_eq!(ecosystem_fee, 250);
        assert_eq!(net_amount, 9_750);
        test_scenario::return_shared(config);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_default_mydata_marketplace_fee_breakdown_with_platform() {
        let mut scenario = test_scenario::begin(@0xA);
        let clock = myso::clock::create_for_testing(test_scenario::ctx(&mut scenario));
        mydata::test_init(&clock, test_scenario::ctx(&mut scenario));
        myso::clock::destroy_for_testing(clock);

        test_scenario::next_tx(&mut scenario, @0xA);
        let config = test_scenario::take_shared<MyDataConfig>(&scenario);
        let (platform_fee, ecosystem_fee, net_amount) =
            mydata::mydata_marketplace_fee_breakdown_with_platform_for_testing(&config, 10_000);
        assert_eq!(platform_fee, 250);
        assert_eq!(ecosystem_fee, 250);
        assert_eq!(net_amount, 9_500);
        test_scenario::return_shared(config);
        test_scenario::end(scenario);
    }
}
