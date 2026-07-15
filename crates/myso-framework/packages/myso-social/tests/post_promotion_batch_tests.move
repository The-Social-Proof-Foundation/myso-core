// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

#[test_only]
#[allow(unused_use, unused_const, duplicate_alias, unused_assignment)]
module social_contracts::post_promotion_batch_tests {
    use std::string;
    use std::option;
    use std::vector;

    use myso::test_scenario::{Self, Scenario};
    use myso::object;
    use myso::coin::{Self, Coin};
    use myso::myso::MYSO;
    use myso::clock::{Self, Clock};
    use myso::permissioned_group::PermissionedGroup;

    use social_contracts::post::{Self, PostConfig, PromotionData};
    use social_contracts::platform::{Self, Platform, PlatformRegistry, PlatformConfig, PlatformPackage};
    use social_contracts::block_list;
    use social_contracts::profile::{Self, EcosystemTreasury};

    const ADMIN: address = @0xA0;
    const CREATOR: address = @0xC1;
    const VIEWER: address = @0xB1;
    const OTHER: address = @0x09;

    const PAYMENT_PER_VIEW: u64 = 10_000;
    const BUDGET: u64 = 100_000;
    const VIEW_DURATION: u64 = 3000;

    fun setup(): Scenario {
        let mut scen = test_scenario::begin(ADMIN);

        test_scenario::next_tx(&mut scen, ADMIN);
        {
            let clock = clock::create_for_testing(test_scenario::ctx(&mut scen));
            block_list::test_init(&clock, test_scenario::ctx(&mut scen));
            platform::test_init(&clock, test_scenario::ctx(&mut scen));
            post::test_init(test_scenario::ctx(&mut scen));
            profile::init_for_testing(&clock, test_scenario::ctx(&mut scen));
            clock::share_for_testing(clock);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let mut preg = test_scenario::take_shared<PlatformRegistry>(&scen);
            let platform_config = test_scenario::take_shared<PlatformConfig>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            platform::create_platform(
                &mut preg,
                &platform_config,
                string::utf8(b"Promo Batch Platform"),
                string::utf8(b"Tag"),
                string::utf8(b"Desc"),
                string::utf8(b"https://logo"),
                string::utf8(b"https://tos"),
                string::utf8(b"https://pp"),
                vector[string::utf8(b"web")],
                vector[string::utf8(b"https://example")],
                string::utf8(b"Social Network"),
                option::none(),
                3,
                string::utf8(b"2024-01-01"),
                false,
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                option::none(),
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(platform_config);
            test_scenario::return_shared(preg);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let platform_obj = test_scenario::take_shared<Platform>(&scen);
            let mut registry = test_scenario::take_shared<PlatformRegistry>(&scen);
            let platform_id = object::uid_to_address(platform::id(&platform_obj));
            platform::test_set_approval(&mut registry, platform_id, true);
            test_scenario::return_shared(platform_obj);
            test_scenario::return_shared(registry);
        };

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let mut platform_obj = test_scenario::take_shared<Platform>(&scen);
            platform::test_join_platform(&mut platform_obj, VIEWER);
            test_scenario::return_shared(platform_obj);
        };

        scen
    }

    fun create_active_promotion(scen: &mut Scenario, content: vector<u8>): address {
        let platform_obj = test_scenario::take_shared<Platform>(scen);
        let platform_id = object::uid_to_address(platform::id(&platform_obj));
        let clock = test_scenario::take_shared<Clock>(scen);
        let budget = coin::mint_for_testing<MYSO>(BUDGET, test_scenario::ctx(scen));
        let (_post_id, promotion_id) = post::create_test_promoted_post(
            CREATOR,
            CREATOR,
            platform_id,
            string::utf8(content),
            PAYMENT_PER_VIEW,
            budget,
            &clock,
            test_scenario::ctx(scen),
        );
        test_scenario::return_shared(clock);
        test_scenario::return_shared(platform_obj);
        promotion_id
    }

    fun activate_promotion(scen: &Scenario, promotion_id: address) {
        let mut promotion = test_scenario::take_shared_by_id<PromotionData>(
            scen,
            object::id_from_address(promotion_id),
        );
        post::test_activate_promotion(&mut promotion);
        test_scenario::return_shared(promotion);
    }

    fun confirm_batch(scen: &mut Scenario, promotion_ids: vector<address>, durations: vector<u64>) {
        let mut promotions = vector::empty<PromotionData>();
        let mut i = 0;
        let n = vector::length(&promotion_ids);
        while (i < n) {
            let id = *vector::borrow(&promotion_ids, i);
            let promo = test_scenario::take_shared_by_id<PromotionData>(
                scen,
                object::id_from_address(id),
            );
            vector::push_back(&mut promotions, promo);
            i = i + 1;
        };

        let config = test_scenario::take_shared<PostConfig>(scen);
        let mut platform_obj = test_scenario::take_shared<Platform>(scen);
        let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(scen);
        let treasury = test_scenario::take_shared<EcosystemTreasury>(scen);
        let clock = test_scenario::take_shared<Clock>(scen);

        post::confirm_promoted_post_views(
            promotions,
            durations,
            &config,
            &mut platform_obj,
            &group,
            &treasury,
            VIEWER,
            &clock,
            test_scenario::ctx(scen),
        );

        test_scenario::return_shared(clock);
        test_scenario::return_shared(treasury);
        test_scenario::return_shared(group);
        test_scenario::return_shared(platform_obj);
        test_scenario::return_shared(config);
    }

    #[test]
    fun test_batch_confirm_len_one_fees() {
        let mut scen = setup();
        test_scenario::next_tx(&mut scen, CREATOR);
        let promo_id = create_active_promotion(&mut scen, b"promo one");

        test_scenario::next_tx(&mut scen, CREATOR);
        activate_promotion(&scen, promo_id);

        test_scenario::next_tx(&mut scen, CREATOR);
        confirm_batch(
            &mut scen,
            vector[promo_id],
            vector[VIEW_DURATION],
        );

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let promotion = test_scenario::take_shared_by_id<PromotionData>(
                &scen,
                object::id_from_address(promo_id),
            );
            let (ppv, remaining, _active, views) = post::get_promotion_stats(&promotion);
            assert!(ppv == PAYMENT_PER_VIEW, 0);
            assert!(remaining == BUDGET - PAYMENT_PER_VIEW, 1);
            assert!(views == 1, 2);
            assert!(post::has_user_viewed_promoted_post(&promotion, VIEWER), 3);
            test_scenario::return_shared(promotion);

            let coins = test_scenario::ids_for_address<Coin<MYSO>>(VIEWER);
            assert!(!vector::is_empty(&coins), 4);
        };

        // Viewer net = 80% of gross with default 10%/10% fees
        test_scenario::next_tx(&mut scen, VIEWER);
        {
            let payment = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            assert!(coin::value(&payment) == 8_000, 5);
            test_scenario::return_to_sender(&scen, payment);
        };

        test_scenario::end(scen);
    }

    #[test]
    fun test_batch_confirm_len_two_fee_totals() {
        let mut scen = setup();
        test_scenario::next_tx(&mut scen, CREATOR);
        let promo_a = create_active_promotion(&mut scen, b"promo a");
        test_scenario::next_tx(&mut scen, CREATOR);
        let promo_b = create_active_promotion(&mut scen, b"promo b");

        test_scenario::next_tx(&mut scen, CREATOR);
        activate_promotion(&scen, promo_a);
        activate_promotion(&scen, promo_b);

        test_scenario::next_tx(&mut scen, CREATOR);
        confirm_batch(
            &mut scen,
            vector[promo_a, promo_b],
            vector[VIEW_DURATION, VIEW_DURATION + 100],
        );

        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let promotion_a = test_scenario::take_shared_by_id<PromotionData>(
                &scen,
                object::id_from_address(promo_a),
            );
            let promotion_b = test_scenario::take_shared_by_id<PromotionData>(
                &scen,
                object::id_from_address(promo_b),
            );
            assert!(post::has_user_viewed_promoted_post(&promotion_a, VIEWER), 0);
            assert!(post::has_user_viewed_promoted_post(&promotion_b, VIEWER), 1);
            let (_ppv_a, rem_a, _, views_a) = post::get_promotion_stats(&promotion_a);
            let (_ppv_b, rem_b, _, views_b) = post::get_promotion_stats(&promotion_b);
            assert!(rem_a == BUDGET - PAYMENT_PER_VIEW, 2);
            assert!(rem_b == BUDGET - PAYMENT_PER_VIEW, 3);
            assert!(views_a == 1, 4);
            assert!(views_b == 1, 5);
            test_scenario::return_shared(promotion_a);
            test_scenario::return_shared(promotion_b);
        };

        test_scenario::next_tx(&mut scen, VIEWER);
        {
            let payment = test_scenario::take_from_sender<Coin<MYSO>>(&scen);
            // 2 * 8000 net
            assert!(coin::value(&payment) == 16_000, 6);
            test_scenario::return_to_sender(&scen, payment);
        };

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = 39, location = social_contracts::post)] // EInvalidBatch
    fun test_batch_confirm_empty_aborts() {
        let mut scen = setup();
        test_scenario::next_tx(&mut scen, CREATOR);
        {
            let config = test_scenario::take_shared<PostConfig>(&scen);
            let mut platform_obj = test_scenario::take_shared<Platform>(&scen);
            let group = test_scenario::take_shared<PermissionedGroup<PlatformPackage>>(&scen);
            let treasury = test_scenario::take_shared<EcosystemTreasury>(&scen);
            let clock = test_scenario::take_shared<Clock>(&scen);
            post::confirm_promoted_post_views(
                vector::empty(),
                vector::empty(),
                &config,
                &mut platform_obj,
                &group,
                &treasury,
                VIEWER,
                &clock,
                test_scenario::ctx(&mut scen),
            );
            test_scenario::return_shared(clock);
            test_scenario::return_shared(treasury);
            test_scenario::return_shared(group);
            test_scenario::return_shared(platform_obj);
            test_scenario::return_shared(config);
        };
        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = 39, location = social_contracts::post)] // EInvalidBatch
    fun test_batch_confirm_length_mismatch_aborts() {
        let mut scen = setup();
        test_scenario::next_tx(&mut scen, CREATOR);
        let promo_id = create_active_promotion(&mut scen, b"promo mismatch");

        test_scenario::next_tx(&mut scen, CREATOR);
        activate_promotion(&scen, promo_id);

        test_scenario::next_tx(&mut scen, CREATOR);
        confirm_batch(
            &mut scen,
            vector[promo_id],
            vector[VIEW_DURATION, VIEW_DURATION],
        );

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = 0, location = social_contracts::post)] // EUnauthorized
    fun test_batch_confirm_unauthorized_aborts() {
        let mut scen = setup();
        test_scenario::next_tx(&mut scen, CREATOR);
        let promo_id = create_active_promotion(&mut scen, b"promo unauth");

        test_scenario::next_tx(&mut scen, CREATOR);
        activate_promotion(&scen, promo_id);

        test_scenario::next_tx(&mut scen, OTHER);
        confirm_batch(
            &mut scen,
            vector[promo_id],
            vector[VIEW_DURATION],
        );

        test_scenario::end(scen);
    }

    #[test]
    #[expected_failure(abort_code = 27, location = social_contracts::post)] // EUserAlreadyViewed
    fun test_batch_confirm_double_view_across_txs_aborts() {
        let mut scen = setup();
        test_scenario::next_tx(&mut scen, CREATOR);
        let promo_id = create_active_promotion(&mut scen, b"promo twice");

        test_scenario::next_tx(&mut scen, CREATOR);
        activate_promotion(&scen, promo_id);

        test_scenario::next_tx(&mut scen, CREATOR);
        confirm_batch(&mut scen, vector[promo_id], vector[VIEW_DURATION]);

        test_scenario::next_tx(&mut scen, CREATOR);
        confirm_batch(&mut scen, vector[promo_id], vector[VIEW_DURATION]);

        test_scenario::end(scen);
    }
}
