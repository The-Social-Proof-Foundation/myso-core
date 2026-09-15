// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias)]
module social_contracts::media_asset_tests {
    use social_contracts::media_asset::{Self as ma};
    use social_contracts::license_template::{Self as lt};
    use myso::test_scenario::{Self, Scenario};
    use myso::clock::{Self, Clock};
    use myso::object;

    const OWNER: address = @0xA11CE;
    const STRANGER: address = @0xBAD;

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

    #[test]
    fun test_pause_blocks_usage_and_unpause_restores_grants() {
        let mut scen = test_scenario::begin(OWNER);
        test_scenario::next_tx(&mut scen, OWNER);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let mut asset = ma::test_mint_media_asset(
                OWNER,
                x"7061757365",
                ma::media_type_video(),
                test_scenario::ctx(&mut scen),
            );
            assert!(ma::rights_permits_usage(&asset, ma::usage_social_post(), &clock));
            assert!(ma::test_authorization_version(&asset) == 1);
            ma::set_media_asset_future_usage_paused(
                &mut asset,
                true,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            assert!(ma::test_future_usage_paused(&asset));
            assert!(!ma::rights_permits_usage(&asset, ma::usage_social_post(), &clock));
            assert!(ma::test_authorization_version(&asset) == 2);
            ma::set_media_asset_future_usage_paused(
                &mut asset,
                false,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            assert!(!ma::test_future_usage_paused(&asset));
            assert!(ma::rights_permits_usage(&asset, ma::usage_social_post(), &clock));
            assert!(ma::test_authorization_version(&asset) == 3);
            clock::share_for_testing(clock);
            ma::test_destroy_media_asset(asset);
        };
        test_scenario::end(scen);
    }

    #[test]
    fun test_licensor_revoke_bumps_authorization_version() {
        let mut scen = test_scenario::begin(OWNER);
        test_scenario::next_tx(&mut scen, OWNER);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let mut asset = ma::test_mint_media_asset(
                OWNER,
                x"7265766f6b65",
                ma::media_type_audio(),
                test_scenario::ctx(&mut scen),
            );
            let template = lt::test_publish_template(
                object::id_from_address(@0xF001),
                1000,
                true,
                test_scenario::ctx(&mut scen),
            );
            ma::attach_media_asset_license(
                &mut asset,
                &template,
                ma::usage_music_soundtrack(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            let after_attach = ma::test_authorization_version(&asset);
            let mut instance = lt::test_accept_instance(
                &template,
                object::id(&asset),
                test_scenario::ctx(&mut scen),
            );
            ma::revoke_license_instance_by_licensor(
                &mut asset,
                &mut instance,
                &template,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            assert!(ma::test_authorization_version(&asset) == after_attach + 1);
            assert!(lt::instance_status(&instance) == lt::license_instance_revoked());
            lt::test_destroy_instance(instance);
            lt::test_destroy_template(template);
            clock::share_for_testing(clock);
            ma::test_destroy_media_asset(asset);
        };
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = 1, location = social_contracts::media_asset)]
    fun test_licensor_revoke_rejects_unauthorized() {
        let mut scen = test_scenario::begin(OWNER);
        test_scenario::next_tx(&mut scen, OWNER);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let mut asset = ma::test_mint_media_asset(
                OWNER,
                x"756e61757468",
                ma::media_type_audio(),
                test_scenario::ctx(&mut scen),
            );
            let template = lt::test_publish_template(
                object::id_from_address(@0xF002),
                1000,
                true,
                test_scenario::ctx(&mut scen),
            );
            ma::attach_media_asset_license(
                &mut asset,
                &template,
                ma::usage_music_soundtrack(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            ma::test_share_media_asset(asset);
            lt::test_share_template(template);
            clock::share_for_testing(clock);
        };
        test_scenario::next_tx(&mut scen, STRANGER);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut asset = test_scenario::take_shared<ma::MediaAsset>(&scen);
            let template = test_scenario::take_shared<lt::LicenseTemplateVersion>(&scen);
            let mut instance = lt::test_accept_instance(
                &template,
                object::id(&asset),
                test_scenario::ctx(&mut scen),
            );
            ma::revoke_license_instance_by_licensor(
                &mut asset,
                &mut instance,
                &template,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            lt::test_destroy_instance(instance);
            test_scenario::return_shared(template);
            test_scenario::return_shared(asset);
            test_scenario::return_shared(clock);
        };
        test_scenario::end(scen);
    }
}
