// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias)]
module social_contracts::media_asset_tests {
    use social_contracts::media_asset::{Self as ma};
    use myso::test_scenario::{Self, Scenario};
    use myso::clock::{Self, Clock};

    const OWNER: address = @0xA11CE;

    #[test]
    fun test_manifest_validation_sums_to_bps_total() {
        let manifest = ma::test_revenue_manifest(vector[
            ma::test_manifest_entry(OWNER, 6000, ma::payout_wallet()),
            ma::test_manifest_entry(@0xB0B, 4000, ma::payout_wallet()),
        ]);
        ma::test_validate_manifest(&manifest);
    }

    #[test]
    #[expected_failure(abort_code = ma::EInvalidManifest, location = social_contracts::media_asset)]
    fun test_manifest_rejects_wrong_total() {
        let manifest = ma::test_revenue_manifest(vector[
            ma::test_manifest_entry(OWNER, 5000, ma::payout_wallet()),
        ]);
        ma::test_validate_manifest(&manifest);
    }

    #[test]
    fun test_default_usage_grants_permit_standard_classes() {
        let mut scen = test_scenario::begin(OWNER);
        test_scenario::next_tx(&mut scen, OWNER);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let asset = ma::test_mint_media_asset(
                OWNER,
                x"deadbeef",
                ma::media_type_image(),
                test_scenario::ctx(&mut scen),
            );
            assert!(ma::rights_permits_usage(&asset, ma::usage_social_post(), &clock));
            assert!(ma::rights_permits_usage(&asset, ma::usage_profile_picture(), &clock));
            assert!(ma::rights_permits_usage(&asset, ma::usage_cover_photo(), &clock));
            assert!(ma::rights_permits_usage(&asset, ma::usage_music_soundtrack(), &clock));
            assert!(ma::usage_allows_paid_exploitation(&asset, ma::usage_social_post(), &clock));
            assert!(
                option::borrow(&ma::usage_compensation_type(&asset, ma::usage_social_post(), &clock))
                    == &ma::compensation_revenue_share()
            );
            clock::share_for_testing(clock);
            ma::test_destroy_media_asset(asset);
        };
        test_scenario::end(scen);
    }

    #[test]
    fun test_music_soundtrack_requires_derivative_rights_in_grant() {
        let mut scen = test_scenario::begin(OWNER);
        test_scenario::next_tx(&mut scen, OWNER);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let mut asset = ma::test_mint_media_asset(
                OWNER,
                x"617564696f",
                ma::media_type_audio(),
                test_scenario::ctx(&mut scen),
            );
            ma::test_set_usage_grants(
                &mut asset,
                vector[
                    ma::test_usage_grant(
                        ma::usage_music_soundtrack(),
                        ma::right_reproduction(),
                        ma::license_non_exclusive(),
                        ma::compensation_none(),
                        0,
                        false,
                        false,
                        true,
                    ),
                ],
            );
            assert!(!ma::rights_permits_usage(&asset, ma::usage_music_soundtrack(), &clock));
            clock::share_for_testing(clock);
            ma::test_destroy_media_asset(asset);
        };
        test_scenario::end(scen);
    }

    #[test]
    fun test_advertisement_requires_commercial_flag() {
        let mut scen = test_scenario::begin(OWNER);
        test_scenario::next_tx(&mut scen, OWNER);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let mut asset = ma::test_mint_media_asset(
                OWNER,
                x"ad",
                ma::media_type_image(),
                test_scenario::ctx(&mut scen),
            );
            ma::test_set_usage_grants(
                &mut asset,
                vector[
                    ma::test_usage_grant(
                        ma::usage_advertisement(),
                        ma::required_rights_for_usage(ma::usage_advertisement()),
                        ma::license_non_exclusive(),
                        ma::compensation_revenue_share(),
                        ma::manifest_bps_total(),
                        false,
                        false,
                        false,
                    ),
                ],
            );
            assert!(!ma::rights_permits_usage(&asset, ma::usage_advertisement(), &clock));
            clock::share_for_testing(clock);
            ma::test_destroy_media_asset(asset);
        };
        test_scenario::end(scen);
    }

    #[test]
    fun test_expired_grant_blocks_usage() {
        let mut scen = test_scenario::begin(OWNER);
        test_scenario::next_tx(&mut scen, OWNER);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let mut asset = ma::test_mint_media_asset(
                OWNER,
                x"65787069726564",
                ma::media_type_image(),
                test_scenario::ctx(&mut scen),
            );
            ma::test_set_usage_grants(
                &mut asset,
                vector[
                    ma::test_usage_grant_with_expiry(
                        ma::usage_social_post(),
                        ma::required_rights_for_usage(ma::usage_social_post()),
                        ma::license_non_exclusive(),
                        ma::compensation_revenue_share(),
                        ma::manifest_bps_total(),
                        false,
                        false,
                        true,
                        0,
                        option::some(0),
                    ),
                ],
            );
            assert!(!ma::rights_permits_usage(&asset, ma::usage_social_post(), &clock));
            clock::share_for_testing(clock);
            ma::test_destroy_media_asset(asset);
        };
        test_scenario::end(scen);
    }

    #[test]
    fun test_claims_resolve_to_rights_interests() {
        let claims = vector[
            ma::test_claim(
                ma::claim_type_authorship(),
                OWNER,
                0,
                ma::claim_oracle_verified(),
            ),
            ma::test_claim(
                ma::claim_type_rights_control(),
                OWNER,
                ma::right_reproduction() | ma::right_public_display(),
                ma::claim_oracle_verified(),
            ),
        ];
        let mut scen = test_scenario::begin(OWNER);
        test_scenario::next_tx(&mut scen, OWNER);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let asset = ma::test_mint_media_asset(
                OWNER,
                x"636c61696d73",
                ma::media_type_image(),
                test_scenario::ctx(&mut scen),
            );
            assert!(ma::test_asset_claim_count(&asset) > 0);
            assert!(ma::test_asset_rights_interest_count(&asset) > 0);
            assert!(ma::test_asset_has_creator(&asset, OWNER));
            let _ = claims;
            ma::test_destroy_media_asset(asset);
            clock::share_for_testing(clock);
        };
        test_scenario::end(scen);
    }
}
