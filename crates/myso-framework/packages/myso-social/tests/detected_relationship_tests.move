// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias)]
module social_contracts::detected_relationship_tests {
    use std::option;

    use myso::test_scenario;
    use myso::object;
    use myso::transfer;
    use myso::clock::{Self, Clock};

    use social_contracts::proof_of_creativity as poc;
    use social_contracts::governance;
    use social_contracts::media_asset::{Self as ma, MediaAsset, PendingDerivativeAsset};
    use social_contracts::proof_of_creativity::DetectedAssetRelationship;

    const ADMIN: address = @0xAD;
    const CREATOR: address = @0xBEEF;
    const ORIGINAL_OWNER: address = @0xCAFE;

    fun init_poc(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            let gov_ids = governance::bootstrap_init(&clock, test_scenario::ctx(scenario));
            poc::test_init(&clock, gov_ids.poc_governance_registry_id(), test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
        };
    }

    #[test]
    fun test_propose_accept_detected_relationship() {
        let mut scen = test_scenario::begin(ADMIN);
        init_poc(&mut scen);

        test_scenario::next_tx(&mut scen, ORIGINAL_OWNER);
        {
            let asset = ma::test_mint_media_asset(
                ORIGINAL_OWNER,
                x"6f726967",
                ma::media_type_audio(),
                test_scenario::ctx(&mut scen),
            );
            ma::test_share_media_asset(asset);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let pending = ma::test_mint_pending_derivative(
                CREATOR,
                x"70656e64",
                ma::media_type_audio(),
                test_scenario::ctx(&mut scen),
            );
            transfer::public_transfer(pending, CREATOR);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let config = test_scenario::take_shared<poc::PoCConfig>(&scen);
            let pending = test_scenario::take_from_address<PendingDerivativeAsset>(&scen, CREATOR);
            let original = test_scenario::take_shared<MediaAsset>(&scen);
            poc::propose_detected_relationship(
                &config,
                &pending,
                &original,
                8500,
                option::some(x"65766964"),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            transfer::public_transfer(pending, CREATOR);
            test_scenario::return_shared(original);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let config = test_scenario::take_shared<poc::PoCConfig>(&scen);
            let mut proposal = test_scenario::take_shared<DetectedAssetRelationship>(&scen);
            let pending = test_scenario::take_from_address<PendingDerivativeAsset>(&scen, CREATOR);
            let original = test_scenario::take_shared<MediaAsset>(&scen);
            assert!(
                poc::test_proposal_status(&proposal) == poc::test_detected_status_proposed()
            );
            poc::accept_detected_relationship(
                &config,
                &mut proposal,
                &pending,
                &original,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            assert!(
                poc::test_proposal_status(&proposal) == poc::test_detected_status_accepted()
            );
            transfer::public_transfer(pending, CREATOR);
            test_scenario::return_shared(original);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_reject_detected_relationship() {
        let mut scen = test_scenario::begin(ADMIN);
        init_poc(&mut scen);

        test_scenario::next_tx(&mut scen, ORIGINAL_OWNER);
        {
            let asset = ma::test_mint_media_asset(
                ORIGINAL_OWNER,
                x"6f726731",
                ma::media_type_image(),
                test_scenario::ctx(&mut scen),
            );
            ma::test_share_media_asset(asset);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let pending = ma::test_mint_pending_derivative(
                CREATOR,
                x"70656e6432",
                ma::media_type_image(),
                test_scenario::ctx(&mut scen),
            );
            transfer::public_transfer(pending, CREATOR);
        };

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let config = test_scenario::take_shared<poc::PoCConfig>(&scen);
            let pending = test_scenario::take_from_address<PendingDerivativeAsset>(&scen, CREATOR);
            let original = test_scenario::take_shared<MediaAsset>(&scen);
            poc::propose_detected_relationship(
                &config,
                &pending,
                &original,
                9000,
                option::none(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            transfer::public_transfer(pending, CREATOR);
            test_scenario::return_shared(original);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, ORIGINAL_OWNER);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let config = test_scenario::take_shared<poc::PoCConfig>(&scen);
            let mut proposal = test_scenario::take_shared<DetectedAssetRelationship>(&scen);
            let pending = test_scenario::take_from_address<PendingDerivativeAsset>(&scen, CREATOR);
            let original = test_scenario::take_shared<MediaAsset>(&scen);
            poc::reject_detected_relationship(
                &config,
                &mut proposal,
                &pending,
                &original,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            assert!(
                poc::test_proposal_status(&proposal) == poc::test_detected_status_rejected()
            );
            transfer::public_transfer(pending, CREATOR);
            test_scenario::return_shared(original);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(config);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }
}
