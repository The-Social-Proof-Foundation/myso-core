#[test_only]
#[allow(duplicate_alias)]
module social_contracts::subscription_fee_tests {
    use std::unit_test::assert_eq;
    use myso::test_scenario;
    use social_contracts::subscription::{Self, SubscriptionConfig};

    #[test]
    fun test_default_fee_breakdown() {
        let mut scenario = test_scenario::begin(@0xA);
        let config = subscription::create_config_for_testing(test_scenario::ctx(&mut scenario));

        let (platform_fee, ecosystem_fee, creator_amount) =
            subscription::fee_breakdown_for_testing(&config, 10_000);
        assert_eq!(platform_fee, 250);
        assert_eq!(ecosystem_fee, 250);
        assert_eq!(creator_amount, 9_500);

        let SubscriptionConfig { id, .. } = config;
        myso::object::delete(id);
        test_scenario::end(scenario);
    }

    #[test]
    fun test_zero_gross_fee_breakdown() {
        let mut scenario = test_scenario::begin(@0xA);
        let config = subscription::create_config_for_testing(test_scenario::ctx(&mut scenario));

        let (platform_fee, ecosystem_fee, creator_amount) =
            subscription::fee_breakdown_for_testing(&config, 0);
        assert_eq!(platform_fee, 0);
        assert_eq!(ecosystem_fee, 0);
        assert_eq!(creator_amount, 0);

        let SubscriptionConfig { id, .. } = config;
        myso::object::delete(id);
        test_scenario::end(scenario);
    }
}
