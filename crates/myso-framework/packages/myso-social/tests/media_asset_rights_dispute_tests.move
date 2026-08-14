// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, duplicate_alias)]
module social_contracts::media_asset_rights_dispute_tests {
    use std::{string::{Self, String}, option, vector};

    use myso::test_scenario::{Self, Scenario};
    use myso::tx_context;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::clock::{Self, Clock};

    use social_contracts::proof_of_creativity as poc;
    use social_contracts::media_asset::{Self as ma, MediaAsset};
    use social_contracts::profile::{Self, EcosystemTreasury};
    use social_contracts::governance::{Self, GovernanceDAO, Proposal};

    const ADMIN: address = @0xA0;
    const CHALLENGER: address = @0xC1;
    const NEW_CREATOR: address = @0xD2;
    const SCALING: u64 = 1_000_000_000;

    fun take_poc_governance_registry(scenario: &Scenario): GovernanceDAO {
        let r0 = test_scenario::take_shared<GovernanceDAO>(scenario);
        if (governance::registry_type(&r0) == governance::proposal_type_proof_of_creativity_value()) {
            return r0
        };
        let r1 = test_scenario::take_shared<GovernanceDAO>(scenario);
        if (governance::registry_type(&r1) == governance::proposal_type_proof_of_creativity_value()) {
            test_scenario::return_shared(r0);
            return r1
        };
        let r2 = test_scenario::take_shared<GovernanceDAO>(scenario);
        if (governance::registry_type(&r2) == governance::proposal_type_proof_of_creativity_value()) {
            test_scenario::return_shared(r0);
            test_scenario::return_shared(r1);
            return r2
        };
        test_scenario::return_shared(r0);
        test_scenario::return_shared(r1);
        test_scenario::return_shared(r2);
        abort 999
    }

    fun setup_env(): Scenario {
        let mut scen = test_scenario::begin(ADMIN);
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scen));
            let ctx = test_scenario::ctx(&mut scen);
            let gov_ids = governance::bootstrap_init(&clock, ctx);
            governance::test_grant_admin_cap(ctx);
            poc::test_init(&clock, gov_ids.poc_governance_registry_id(), ctx);
            clock::share_for_testing(clock);
        };
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            transfer_to(CHALLENGER, 100_000 * SCALING, test_scenario::ctx(&mut scen));
        };
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let gov_admin = test_scenario::take_from_sender<governance::GovernanceAdminCap>(&scen);
            let mut poc_registry = take_poc_governance_registry(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            governance::update_governance_parameters(
                &mut poc_registry,
                &gov_admin,
                2,
                90,
                1_000,
                5,
                0,
                1,
                1,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(poc_registry);
            test_scenario::return_to_sender(&scen, gov_admin);
        };
        scen
    }

    fun transfer_to(to: address, amount: u64, ctx: &mut tx_context::TxContext) {
        let c = coin::mint_for_testing<MYSO>(amount, ctx);
        myso::transfer::public_transfer(c, to);
    }

    fun share_test_asset(scen: &mut Scenario, owner: address) {
        test_scenario::next_tx(scen, owner);
        {
            let asset = ma::test_mint_media_asset(
                owner,
                x"6173736574303031",
                ma::media_type_image(),
                test_scenario::ctx(scen),
            );
            ma::test_share_media_asset(asset);
        };
    }

    fun oracle_verified_claims(creator: address): vector<ma::Claim> {
        vector[
            ma::test_claim(ma::claim_type_authorship(), creator, 0, ma::claim_oracle_verified()),
            ma::test_claim(
                ma::claim_type_rights_control(),
                creator,
                ma::all_statutory_rights(),
                ma::claim_oracle_verified(),
            ),
            ma::test_claim(ma::claim_type_license_authority(), creator, 0, ma::claim_oracle_verified()),
        ]
    }

    fun default_grants(): vector<ma::UsageGrant> {
        vector[ma::test_usage_grant(
            ma::usage_social_post(),
            ma::right_reproduction() | ma::right_public_display(),
            ma::license_non_exclusive(),
            ma::compensation_revenue_share(),
            5000,
            false,
            true,
            false,
        )]
    }

    fun submit_rights_proposal(scen: &mut Scenario, challenger: address) {
        test_scenario::next_tx(scen, challenger);
        {
            let cfg = test_scenario::take_shared<poc::PoCConfig>(scen);
            let mut gov = take_poc_governance_registry(scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(scen);
            let mut asset = test_scenario::take_shared<MediaAsset>(scen);
            let clock = test_scenario::take_shared<Clock>(scen);
            let claims = oracle_verified_claims(NEW_CREATOR);
            let grants = default_grants();
            let commitment = ma::test_compute_claims_commitment(&claims, &grants);
            let mut payment = coin::mint_for_testing<MYSO>(100_000 * SCALING, test_scenario::ctx(scen));
            poc::submit_media_asset_rights_dispute_proposal(
                &cfg,
                &mut gov,
                &treasury,
                &mut asset,
                string::utf8(b"Rights dispute"),
                string::utf8(b"Challenge ownership"),
                commitment,
                option::none(),
                option::none(),
                option::none(),
                &mut payment,
                &clock,
                test_scenario::ctx(scen),
            );
            if (coin::value(&payment) > 0) {
                myso::transfer::public_transfer(payment, challenger);
            } else {
                coin::destroy_zero(payment);
            };
            assert!(ma::test_has_active_rights_proposal(&asset), 1);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(asset);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(gov);
            test_scenario::return_shared(cfg);
        };
    }

    fun approve_proposal(scen: &mut Scenario) {
        test_scenario::next_tx(scen, ADMIN);
        {
            let mut gov = take_poc_governance_registry(scen);
            let mut proposal = test_scenario::take_shared<Proposal>(scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(scen);
            let clock = test_scenario::take_shared<Clock>(scen);
            governance::delegate_vote_on_proposal(
                &mut gov,
                &mut proposal,
                &treasury,
                true,
                option::some(string::utf8(b"Advance")),
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(gov);
        };
        test_scenario::next_tx(scen, CHALLENGER);
        {
            let mut gov = take_poc_governance_registry(scen);
            let mut proposal = test_scenario::take_shared<Proposal>(scen);
            let clock = test_scenario::take_shared<Clock>(scen);
            let mut payment = coin::mint_for_testing<MYSO>(10_000 * SCALING, test_scenario::ctx(scen));
            governance::community_vote_on_proposal(
                &mut gov,
                &mut proposal,
                1,
                true,
                &mut payment,
                &clock,
                test_scenario::ctx(scen),
            );
            if (coin::value(&payment) > 0) {
                myso::transfer::public_transfer(payment, CHALLENGER);
            } else {
                coin::destroy_zero(payment);
            };
            test_scenario::return_shared(clock);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(gov);
        };
        test_scenario::next_tx(scen, ADMIN);
        {
            let cfg = test_scenario::take_shared<poc::PoCConfig>(scen);
            let mut gov = take_poc_governance_registry(scen);
            let mut proposal = test_scenario::take_shared<Proposal>(scen);
            let mut asset = test_scenario::take_shared<MediaAsset>(scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(scen);
            let mut clock = test_scenario::take_shared<Clock>(scen);
            clock::increment_for_testing(&mut clock, 2);
            poc::finalize_media_asset_rights_governance_proposal(
                &cfg,
                &mut gov,
                &mut proposal,
                &mut asset,
                &treasury,
                &clock,
                test_scenario::ctx(scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(asset);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(gov);
            test_scenario::return_shared(cfg);
        };
    }

    #[test]
    fun test_submit_and_implement_rights_dispute() {
        let mut scen = setup_env();
        share_test_asset(&mut scen, ADMIN);
        submit_rights_proposal(&mut scen, CHALLENGER);
        approve_proposal(&mut scen);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let cfg = test_scenario::take_shared<poc::PoCConfig>(&scen);
            let mut gov = take_poc_governance_registry(&scen);
            let mut proposal = test_scenario::take_shared<Proposal>(&scen);
            let mut asset = test_scenario::take_shared<MediaAsset>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let before_version = ma::test_rights_version(&asset);
            let claims = oracle_verified_claims(NEW_CREATOR);
            let grants = default_grants();
            poc::implement_media_asset_rights_from_governance(
                &cfg,
                &mut gov,
                &mut proposal,
                &mut asset,
                &treasury,
                claims,
                grants,
                string::utf8(b"DAO approved new rights holder"),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            assert!(ma::test_rights_version(&asset) == before_version + 1, 2);
            assert!(!ma::test_has_active_rights_proposal(&asset), 3);
            assert!(ma::test_asset_has_creator(&asset, NEW_CREATOR), 4);
            test_scenario::return_shared(clock);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(asset);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(gov);
            test_scenario::return_shared(cfg);
        };

        test_scenario::end(scen);
    }

    #[test, expected_failure(abort_code = 29, location = social_contracts::proof_of_creativity)]
    fun test_implement_commitment_mismatch_aborts() {
        let mut scen = setup_env();
        share_test_asset(&mut scen, ADMIN);
        submit_rights_proposal(&mut scen, CHALLENGER);
        approve_proposal(&mut scen);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let cfg = test_scenario::take_shared<poc::PoCConfig>(&scen);
            let mut gov = take_poc_governance_registry(&scen);
            let mut proposal = test_scenario::take_shared<Proposal>(&scen);
            let mut asset = test_scenario::take_shared<MediaAsset>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            let bad_claims = oracle_verified_claims(ADMIN);
            let grants = default_grants();
            poc::implement_media_asset_rights_from_governance(
                &cfg,
                &mut gov,
                &mut proposal,
                &mut asset,
                &treasury,
                bad_claims,
                grants,
                string::utf8(b"Mismatch"),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(asset);
            test_scenario::return_shared(proposal);
            test_scenario::return_shared(gov);
            test_scenario::return_shared(cfg);
        };

        test_scenario::end(scen);
    }

    #[test, expected_failure(abort_code = 27, location = social_contracts::media_asset)]
    fun test_duplicate_active_proposal_aborts() {
        let mut scen = setup_env();
        share_test_asset(&mut scen, ADMIN);
        submit_rights_proposal(&mut scen, CHALLENGER);
        submit_rights_proposal(&mut scen, CHALLENGER);
        test_scenario::end(scen);
    }
}
