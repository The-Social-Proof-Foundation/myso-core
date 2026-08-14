// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, duplicate_alias)]
module social_contracts::post_enforcement_tests {
    use std::string;
    use std::option;
    use std::vector;

    use myso::test_scenario;
    use myso::object;
    use myso::clock::{Self, Clock};

    use social_contracts::post::{Self, Post};
    use social_contracts::profile::{Self, UsernameRegistry};
    use social_contracts::platform;
    use social_contracts::block_list;
    use social_contracts::mydata;
    use social_contracts::media_asset::{Self as ma, MediaAsset};

    const AUTHOR: address = @0x1;
    const ORACLE: address = @0x9999;
    const PLATFORM_ID: address = @0x1;
    const TEST_CONTENT: vector<u8> = b"enforcement post";
    const BINDING_ID: u64 = 1;

    fun init_social(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, AUTHOR);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(scenario));
            profile::test_init(&clock, test_scenario::ctx(scenario));
            platform::test_init(&clock, test_scenario::ctx(scenario));
            block_list::test_init(&clock, test_scenario::ctx(scenario));
            mydata::test_init(&clock, test_scenario::ctx(scenario));
            clock::share_for_testing(clock);
            post::test_init(test_scenario::ctx(scenario));
        };

        test_scenario::next_tx(scenario, AUTHOR);
        {
            let clock = test_scenario::take_shared<Clock>(scenario);
            let mut registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            profile::register_username(
                &mut registry,
                string::utf8(b"author"),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };
    }

    fun create_author_post(scenario: &mut test_scenario::Scenario) {
        test_scenario::next_tx(scenario, AUTHOR);
        {
            let clock = test_scenario::take_shared<Clock>(scenario);
            let registry = test_scenario::take_shared<UsernameRegistry>(scenario);
            let mut profile_id_option = profile::lookup_profile_by_owner(&registry, AUTHOR);
            let profile_id = option::extract(&mut profile_id_option);
            post::test_create_post(
                AUTHOR,
                profile_id,
                PLATFORM_ID,
                string::utf8(TEST_CONTENT),
                &clock,
                test_scenario::ctx(scenario),
            );
            test_scenario::return_shared(registry);
            test_scenario::return_shared(clock);
        };
    }

    #[test]
    fun test_record_refresh_and_playback_denial() {
        let mut scen = test_scenario::begin(AUTHOR);
        init_social(&mut scen);
        create_author_post(&mut scen);

        test_scenario::next_tx(&mut scen, AUTHOR);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            let asset = ma::test_mint_media_asset(
                AUTHOR,
                x"66656564",
                ma::media_type_video(),
                test_scenario::ctx(&mut scen),
            );
            ma::test_share_media_asset(asset);
            clock::share_for_testing(clock);
        };

        test_scenario::next_tx(&mut scen, ORACLE);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut post = test_scenario::take_shared<Post>(&scen);
            let asset = test_scenario::take_shared<MediaAsset>(&scen);
            let asset_id = object::id(&asset);
            let binding = post::test_embedded_binding(
                BINDING_ID,
                asset_id,
                ma::usage_social_post(),
            );
            post::record_embedded_bindings(
                ORACLE,
                &mut post,
                vector[binding],
                &clock,
                test_scenario::ctx(&mut scen),
            );
            post::refresh_post_asset_usage_decision(
                ORACLE,
                &mut post,
                &asset,
                BINDING_ID,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            assert!(post::test_decision_playback_permitted(&post, BINDING_ID));
            test_scenario::return_shared(asset);
            test_scenario::return_shared(post);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, AUTHOR);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut post = test_scenario::take_shared<Post>(&scen);
            let asset = test_scenario::take_shared<MediaAsset>(&scen);
            post::deny_container_usage(
                &mut post,
                &asset,
                BINDING_ID,
                post::test_denial_scope_playback(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            assert!(!post::test_decision_playback_permitted(&post, BINDING_ID));
            assert!(post::test_composition_status(&post) == ma::composition_invalid());
            test_scenario::return_shared(asset);
            test_scenario::return_shared(post);
            test_scenario::return_shared(clock);
        };

        test_scenario::next_tx(&mut scen, AUTHOR);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut post = test_scenario::take_shared<Post>(&scen);
            let asset = test_scenario::take_shared<MediaAsset>(&scen);
            post::lift_container_usage_denial(
                &mut post,
                &asset,
                BINDING_ID,
                post::test_denial_scope_playback(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            assert!(post::test_decision_playback_permitted(&post, BINDING_ID));
            test_scenario::return_shared(asset);
            test_scenario::return_shared(post);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_candidate_manifest_submission() {
        let mut scen = test_scenario::begin(AUTHOR);
        init_social(&mut scen);
        create_author_post(&mut scen);

        test_scenario::next_tx(&mut scen, ORACLE);
        {
            let clock = test_scenario::take_shared<Clock>(&scen);
            let mut post = test_scenario::take_shared<Post>(&scen);
            let manifest = ma::test_revenue_manifest(vector[
                ma::test_manifest_entry(AUTHOR, ma::manifest_bps_total(), ma::payout_wallet()),
            ]);
            post::submit_candidate_revenue_manifest(
                ORACLE,
                &mut post,
                manifest,
                1,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(post);
            test_scenario::return_shared(clock);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_manifest_embedded_redirect_cap_clamp() {
        let creator = @0xCAFE;
        let source_id = object::id_from_address(@0xBEEF);
        let entry_creator = ma::test_manifest_entry(creator, 2000, ma::payout_wallet());
        let entry_source = ma::test_manifest_entry_with_source(
            creator,
            8000,
            ma::payout_wallet(),
            source_id,
        );
        let manifest = ma::test_revenue_manifest(vector[entry_creator, entry_source]);
        let clamped = ma::test_clamp_manifest_embedded_redirect_cap(manifest, 5000);
        let entries = ma::test_manifest_entries(&clamped);
        assert!(vector::length(entries) == 2);
        let e0 = vector::borrow(entries, 0);
        let e1 = vector::borrow(entries, 1);
        assert!(ma::test_manifest_entry_share_bps(e0) == 5000);
        assert!(ma::test_manifest_entry_share_bps(e1) == 5000);
        ma::test_validate_manifest(&clamped);
    }
}
