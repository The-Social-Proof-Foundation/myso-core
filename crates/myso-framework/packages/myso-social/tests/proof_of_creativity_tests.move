// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_variable, unused_assignment, duplicate_alias)]
module social_contracts::proof_of_creativity_tests {
    use social_contracts::proof_of_creativity as poc;
    use social_contracts::governance;

    use myso::test_scenario::{Self, Scenario};
    use myso::clock::{Self, Clock};

    // Test addresses
    const ADMIN: address = @0xA0;

    const MEDIA_IMAGE: u8 = 1;
    const MEDIA_VIDEO: u8 = 2;

    #[test]
    fun test_poc_bootstrap_and_update_config() {
        let mut scen = test_scenario::begin(ADMIN);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let ctx = test_scenario::ctx(&mut scen);
            let gov_ids = governance::bootstrap_init(&clock, ctx);
            poc::test_init(&clock, gov_ids.poc_governance_registry_id(), ctx);
            clock::share_for_testing(clock);
        };

        // Update PoC config including max_votes_per_dispute
        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let admin_cap = test_scenario::take_from_sender<poc::PoCAdminCap>(&scen);
            let mut cfg = test_scenario::take_shared<poc::PoCConfig>(&scen);
            poc::update_poc_config(
                &admin_cap,
                &mut cfg,
                ADMIN, // oracle_address
                95,    // image_threshold
                95,    // video_threshold
                95,    // audio_threshold
                100,   // revenue_redirect_percentage
                6_000_000_000, // dispute_cost
                1_000_000_000, // min_vote_stake
                100_000_000_000, // max_vote_stake
                7 * 24 * 60 * 60 * 1000, // voting_duration_ms
                5000,  // max_reasoning_length
                10,    // max_evidence_urls
                5000,  // max_votes_per_dispute
                100,   // claim_treasury_fee_bps (1%)
                500,   // max_referral_bps (5%)
                3000,  // video_embedded_audio_redirect_bps (30% ceiling)
                0,     // dispute_quorum_base_stake (disabled)
                10000, // dispute_second_round_fee_multiplier_bps (1x)
                10000, // dispute_second_round_quorum_multiplier_bps (1x)
                500,   // username_beneficiary_join_referral_bps (5%)
                3,     // max_disputes_per_post
                1,     // min_vault_deposit_amount
                10_000_000_000, // media_asset_dispute_cost
                2,     // max_disputes_per_media_asset
                5000,  // max_embedded_asset_redirect_bps
                &clock,
                test_scenario::ctx(&mut scen)
            );
            test_scenario::return_shared(clock);
            test_scenario::return_to_sender(&scen, admin_cap);
            test_scenario::return_shared(cfg);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_second_round_fee_multiplier_math() {
        assert!(poc::test_mul_div_u64_loose(5_000_000_000, 15000, 10000) == 7_500_000_000);
        assert!(poc::test_mul_div_u64_loose(100, 10000, 10000) == 100);
    }

    #[test]
    fun test_bps_to_redirect_percent_rounds() {
        assert!(poc::test_bps_to_redirect_percent(0) == 0);
        assert!(poc::test_bps_to_redirect_percent(50) == 1);
        assert!(poc::test_bps_to_redirect_percent(3000) == 30);
        assert!(poc::test_bps_to_redirect_percent(10000) == 100);
    }

    #[test]
    fun test_similarity_redirect_embed_audio_ceiling_vs_full_video() {
        let threshold = 95u64;
        let score = 100u64;
        let embed_ceiling = poc::test_bps_to_redirect_percent(3000);
        let full_ceiling = 100u64;
        let embed_redirect = poc::test_similarity_redirect_percentage(threshold, score, embed_ceiling);
        let full_redirect = poc::test_similarity_redirect_percentage(threshold, score, full_ceiling);
        assert!(embed_redirect == 30);
        assert!(full_redirect == 100);
    }

    #[test]
    fun test_similarity_same_score_video_vs_audio_threshold() {
        let threshold_video = 90u64;
        let threshold_audio = 95u64;
        let score = 97u64;
        let ceiling = 100u64;
        let video_redirect = poc::test_similarity_redirect_percentage(threshold_video, score, ceiling);
        let audio_redirect = poc::test_similarity_redirect_percentage(threshold_audio, score, ceiling);
        assert!(video_redirect == 70);
        assert!(audio_redirect == 40);
    }

    #[test]
    fun test_assert_embed_audio_flag_ok_for_video() {
        poc::test_assert_embed_audio_derivative_media_type(true, MEDIA_VIDEO);
        poc::test_assert_embed_audio_derivative_media_type(false, MEDIA_IMAGE);
    }

    #[test]
    #[expected_failure(abort_code = poc::EInvalidMediaType, location = social_contracts::proof_of_creativity)]
    fun test_assert_embed_audio_flag_requires_video() {
        poc::test_assert_embed_audio_derivative_media_type(true, MEDIA_IMAGE);
    }

    #[test]
    fun test_self_match_clears_original_creator() {
        let owner = @0xA11CE;
        let cleared = poc::test_clear_self_match_original_creator(owner, option::some(owner));
        assert!(option::is_none(&cleared));
        let other = @0xB0B;
        let kept = poc::test_clear_self_match_original_creator(owner, option::some(other));
        assert!(option::is_some(&kept));
        assert!(*option::borrow(&kept) == other);
    }

    #[test]
    fun test_self_match_skips_derivative_redirect() {
        let owner = @0xA11CE;
        assert!(!poc::test_would_apply_derivative_redirect(owner, option::some(owner), 100, 95));
        assert!(poc::test_would_apply_derivative_redirect(owner, option::some(@0xB0B), 100, 95));
    }
}
