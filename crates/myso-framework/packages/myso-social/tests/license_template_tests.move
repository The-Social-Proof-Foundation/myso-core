// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias)]
module social_contracts::license_template_tests {
    use social_contracts::license_template::{Self as lt};
    use social_contracts::derivative_graph;
    use myso::test_scenario::{Self, Scenario};
    use myso::clock::{Self, Clock};
    use myso::object::{Self, ID};

    const OWNER: address = @0xA11CE;
    const LICENSEE: address = @0xBEEF;

    fun family_id(): ID {
        object::id_from_address(@0xF001)
    }

    #[test]
    fun test_derive_parent_share_bps_from_template() {
        let mut scen = test_scenario::begin(OWNER);
        test_scenario::next_tx(&mut scen, OWNER);
        {
            let template = lt::test_publish_template(family_id(), 1500, true, test_scenario::ctx(&mut scen));
            assert!(lt::derive_parent_share_bps(&template, derivative_graph::relationship_remix()) == 1500);
            lt::test_destroy_template(template);
        };
        test_scenario::end(scen);
    }

    const E_INSTANCE_NOT_REVOCABLE: u64 = 4;

    #[test]
    #[expected_failure(abort_code = E_INSTANCE_NOT_REVOCABLE, location = social_contracts::license_template)]
    fun test_revoke_blocked_when_template_non_revocable() {
        let mut scen = test_scenario::begin(OWNER);
        test_scenario::next_tx(&mut scen, LICENSEE);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let template = lt::test_publish_template(family_id(), 1000, false, test_scenario::ctx(&mut scen));
            let mut instance = lt::test_accept_instance(
                &template,
                object::id_from_address(@0xA55E7),
                test_scenario::ctx(&mut scen),
            );
            lt::revoke_license_instance(&mut instance, &template, &clock, test_scenario::ctx(&mut scen));
            lt::test_destroy_instance(instance);
            lt::test_destroy_template(template);
            clock::share_for_testing(clock);
        };
        test_scenario::end(scen);
    }
}
