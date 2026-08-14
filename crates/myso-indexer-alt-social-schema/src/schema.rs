// @generated automatically by Diesel CLI.

diesel::table! {
    anonymous_votes (id, time) {
        id -> Int4,
        proposal_id -> Text,
        voter_address -> Text,
        encrypted_vote_data -> Bytea,
        submitted_at -> Int8,
        decrypted -> Nullable<Bool>,
        decrypted_at -> Nullable<Int8>,
        decrypted_vote -> Nullable<Int2>,
        decryption_status -> Nullable<Int2>,
        decryption_error -> Nullable<Text>,
        time -> Timestamptz,
        transaction_id -> Text,
        processing_success -> Nullable<Bool>,
        processing_error -> Nullable<Text>,
    }
}

diesel::table! {
    blocked_events (id) {
        id -> Int4,
        event_id -> Nullable<Text>,
        event_type -> Text,
        blocker_address -> Text,
        blocked_address -> Nullable<Text>,
        raw_event_data -> Nullable<Jsonb>,
        processed_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::table! {
    blocked_profiles (id) {
        id -> Int4,
        blocker_address -> Text,
        blocked_address -> Text,
        blocked_profile_id -> Nullable<Text>,
        blocked_username -> Text,
        blocked_display_name -> Nullable<Text>,
        blocked_profile_photo -> Nullable<Text>,
        first_blocked_at -> Timestamp,
        last_blocked_at -> Timestamp,
        total_block_count -> Int4,
    }
}

diesel::table! {
    checkpoint_processing (id, processing_start_time) {
        id -> Int4,
        checkpoint_number -> Int8,
        processing_start_time -> Timestamp,
        processing_end_time -> Nullable<Timestamp>,
        events_processed -> Nullable<Int4>,
        profiles_created -> Nullable<Int4>,
        profiles_updated -> Nullable<Int4>,
        follows_created -> Nullable<Int4>,
        follows_removed -> Nullable<Int4>,
        platform_events -> Nullable<Int4>,
        block_events -> Nullable<Int4>,
        processing_status -> Nullable<Text>,
        processing_duration_ms -> Nullable<Int4>,
        error_message -> Nullable<Text>,
    }
}

diesel::table! {
    comments (id, time) {
        id -> Text,
        comment_id -> Text,
        post_id -> Text,
        parent_comment_id -> Nullable<Text>,
        owner -> Text,
        profile_id -> Text,
        content -> Text,
        media_urls -> Nullable<Jsonb>,
        mentions -> Nullable<Jsonb>,
        metadata_json -> Nullable<Jsonb>,
        created_at -> Int8,
        updated_at -> Nullable<Int8>,
        deleted_at -> Nullable<Int8>,
        reaction_count -> Nullable<Int8>,
        comment_count -> Nullable<Int8>,
        repost_count -> Nullable<Int8>,
        tips_received -> Nullable<Int8>,
        removed_from_platform -> Nullable<Bool>,
        removed_by -> Nullable<Text>,
        contract_version -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
        actor_address -> Nullable<Text>,
        sub_agent_id -> Nullable<Text>,
        action_identity_class -> Nullable<Int2>,
        organization_id -> Nullable<Text>,
    }
}

diesel::table! {
    community_votes (id, time) {
        id -> Int4,
        proposal_id -> Text,
        voter_address -> Text,
        vote_weight -> Int8,
        approve -> Bool,
        vote_time -> Int8,
        vote_cost -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    continuous_aggregate_refresh_status (view_name) {
        view_name -> Text,
        last_manual_refresh -> Nullable<Timestamp>,
        notes -> Nullable<Text>,
    }
}

diesel::table! {
    delegate_ratings (id, time) {
        id -> Int4,
        target_address -> Text,
        voter_address -> Text,
        registry_type -> Int2,
        is_active_delegate -> Bool,
        vote_kind -> Int2,
        rated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        governance_registry_id -> Text,
    }
}

diesel::table! {
    delegate_votes (id, time) {
        id -> Int4,
        proposal_id -> Text,
        delegate_address -> Text,
        approve -> Bool,
        vote_time -> Int8,
        reason -> Nullable<Text>,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    delegates (id, time) {
        id -> Int4,
        address -> Text,
        registry_type -> Int2,
        governance_registry_id -> Text,
        upvotes -> Int8,
        downvotes -> Int8,
        proposals_reviewed -> Int8,
        proposals_submitted -> Int8,
        sided_winning_proposals -> Int8,
        sided_losing_proposals -> Int8,
        term_start -> Int8,
        term_end -> Int8,
        is_active -> Bool,
        created_at -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    ecosystem_treasury (id, time) {
        id -> Int4,
        treasury_address -> Varchar,
        updated_by -> Varchar,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
        version -> Int8,
    }
}

diesel::table! {
    governance_events (id) {
        id -> Int4,
        event_type -> Varchar,
        registry_type -> Int2,
        event_data -> Jsonb,
        event_id -> Varchar,
        created_at -> Timestamptz,
        anonymous_voting_related -> Nullable<Bool>,
        governance_registry_id -> Nullable<Text>,
        proposal_id -> Nullable<Text>,
    }
}

diesel::table! {
    governance_registries (id) {
        id -> Int4,
        registry_type -> Int2,
        delegate_count -> Int8,
        delegate_term_epochs -> Int8,
        proposal_submission_cost -> Int8,
        max_votes_per_user -> Int8,
        quadratic_base_cost -> Int8,
        voting_period_ms -> Int8,
        quorum_votes -> Int8,
        last_delegate_panel_boundary_epoch -> Nullable<Int8>,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        registry_id -> Varchar,
    }
}

diesel::table! {
    insurance_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        min_coverage_bps -> Int8,
        max_coverage_bps -> Int8,
        max_duration_ms -> Int8,
        fee_bps -> Int8,
        version -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        insurance_enabled -> Bool,
        min_spot_total_liquidity -> Int8,
        max_coverage_fraction_of_option_bps -> Int8,
        max_risk_multiplier_bps -> Int8,
        min_premium_amount -> Int8,
        spot_smoothing_per_option -> Int8,
        implied_prob_floor_bps -> Int8,
        odds_floor_1x -> Bool,
        odds_cap_bps -> Int8,
        liq_cap_bps -> Int8,
        liq_ref_amount -> Int8,
        exposure_cap_bps -> Int8,
        exposure_k_bps -> Int8,
        odds_base_bps -> Int8,
    }
}

diesel::table! {
    insurance_events (id) {
        id -> Int4,
        event_type -> Text,
        event_data -> Jsonb,
        event_id -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    insurance_coverage_routes (route_id) {
        route_id -> Text,
        insured -> Text,
        market_id -> Text,
        option_id -> Int2,
        coverage_bps -> Int8,
        duration_ms -> Int8,
        total_covered -> Int8,
        total_premium -> Int8,
        total_reserve -> Int8,
        total_backstop_sweep -> Int8,
        expiry_time_ms -> Int8,
        policy_ids -> Jsonb,
        vault_ids -> Jsonb,
        contract_version -> Int8,
        transaction_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    insurance_route_fills (id) {
        id -> Int8,
        route_id -> Text,
        leg_index -> Int2,
        vault_id -> Text,
        policy_id -> Text,
        covered_amount -> Int8,
        premium_paid -> Int8,
        reserve_locked -> Int8,
        backstop_sweep_amount -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        timestamp_ms -> Int8,
        created_at -> Timestamp,
    }
}

diesel::table! {
    insurance_market_exposures (id, time) {
        id -> Int4,
        vault_id -> Text,
        market_id -> Text,
        option_id -> Int2,
        reserved_amount -> Int8,
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    insurance_policies (policy_id) {
        policy_id -> Text,
        market_id -> Text,
        insured -> Text,
        option_id -> Int2,
        covered_amount -> Int8,
        coverage_bps -> Int8,
        premium_paid -> Int8,
        premium_raw -> Int8,
        implied_probability_bps -> Int8,
        risk_multiplier_bps -> Int8,
        base_premium -> Int8,
        market_total_amount -> Int8,
        option_escrow_amount -> Int8,
        start_time_ms -> Int8,
        expiry_time_ms -> Int8,
        vault_id -> Text,
        status -> Int2,
        route_id -> Nullable<Text>,
        route_leg_index -> Nullable<Int2>,
        backstop_sweep_amount -> Int8,
        contract_version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        transaction_id -> Text,
    }
}

diesel::table! {
    insurance_policy_events (id, time) {
        id -> Int4,
        policy_id -> Text,
        event_type -> Text,
        market_id -> Text,
        insured -> Text,
        option_id -> Int2,
        covered_amount -> Int8,
        coverage_bps -> Int8,
        premium_paid -> Int8,
        reserve_locked -> Int8,
        premium_raw -> Nullable<Int8>,
        implied_probability_bps -> Nullable<Int8>,
        risk_multiplier_bps -> Nullable<Int8>,
        base_premium -> Nullable<Int8>,
        market_total_amount -> Nullable<Int8>,
        option_escrow_amount -> Nullable<Int8>,
        refunded_amount -> Nullable<Int8>,
        fee_paid -> Nullable<Int8>,
        payout -> Nullable<Int8>,
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    insurance_user_exposures (id, time) {
        id -> Int4,
        vault_id -> Text,
        insured -> Text,
        reserved_amount -> Int8,
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    insurance_vault_transactions (id, time) {
        id -> Int4,
        vault_id -> Text,
        transaction_type -> Text,
        amount -> Int8,
        balance_after -> Int8,
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    insurance_vaults (vault_id) {
        vault_id -> Text,
        underwriter -> Text,
        capital_balance -> Int8,
        reserved -> Int8,
        base_rate_bps_per_day -> Int8,
        utilization_multiplier_bps -> Int8,
        max_exposure_per_market -> Int8,
        max_exposure_per_user -> Int8,
        max_exposure_per_option -> Int8,
        enabled -> Bool,
        paused -> Bool,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        transaction_id -> Text,
    }
}

diesel::table! {
    mydata_access_logs (id, time) {
        id -> Int4,
        mydata_id -> Text,
        user_address -> Text,
        access_type -> Text,
        access_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    mydata_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        marketplace_enabled -> Bool,
        max_tags -> Int8,
        max_subscription_days -> Int8,
        max_free_access_grants -> Int8,
        max_encryption_id_bytes -> Int8,
        max_encrypted_data_bytes -> Int8,
        max_tag_bytes -> Int8,
        max_metadata_bytes -> Int8,
        max_payment_reference_bytes -> Int8,
        max_pool_assignments -> Int8,
        max_merkle_proof_depth -> Int8,
        max_paid_access_entries -> Int8,
        default_claim_window_ms -> Int8,
        p2p_platform_fee_bps -> Int8,
        p2p_ecosystem_fee_bps -> Int8,
        mydata_marketplace_platform_fee_bps -> Int8,
        mydata_marketplace_ecosystem_fee_bps -> Int8,
        non_platform_platform_to_creator_bps -> Int8,
        non_platform_platform_to_treasury_bps -> Int8,
        version -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    mydata_data (mydata_id) {
        mydata_id -> Text,
        owner -> Text,
        media_type -> Text,
        tags -> Jsonb,
        platform_id -> Nullable<Text>,
        timestamp_start -> Int8,
        timestamp_end -> Nullable<Int8>,
        created_at -> Int8,
        last_updated -> Int8,
        one_time_price -> Nullable<Int8>,
        subscription_price -> Nullable<Int8>,
        access_configuration_kind -> Nullable<Text>,
        subscription_duration_days -> Int8,
        geographic_region -> Nullable<Text>,
        data_quality -> Nullable<Text>,
        sample_size -> Nullable<Int8>,
        collection_method -> Nullable<Text>,
        is_updating -> Bool,
        update_frequency -> Nullable<Text>,
        version -> Int8,
        encrypted_content_hash -> Nullable<Text>,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    mydata_purchases (id, time) {
        id -> Int4,
        mydata_id -> Text,
        buyer -> Text,
        price -> Int8,
        platform_fee -> Int8,
        ecosystem_fee -> Int8,
        creator_amount -> Int8,
        platform_address -> Nullable<Text>,
        purchase_type -> Text,
        purchase_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        revoked -> Bool,
        revoked_at -> Nullable<Int8>,
        revoked_by -> Nullable<Text>,
        organization_id -> Nullable<Text>,
    }
}

diesel::table! {
    mydata_registry (mydata_id) {
        mydata_id -> Text,
        owner -> Text,
        registered_at -> Int8,
        unregistered_at -> Nullable<Int8>,
        is_active -> Bool,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    mydata_revenue (id, time) {
        id -> Int4,
        mydata_id -> Text,
        from_address -> Text,
        to_address -> Text,
        amount -> Int8,
        platform_fee -> Int8,
        ecosystem_fee -> Int8,
        creator_amount -> Int8,
        platform_address -> Nullable<Text>,
        revenue_type -> Text,
        revenue_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    mydata_subscriptions (id, time) {
        id -> Int4,
        mydata_id -> Text,
        subscriber -> Text,
        subscription_start -> Int8,
        subscription_end -> Int8,
        price -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        revoked -> Bool,
        revoked_at -> Nullable<Int8>,
        revoked_by -> Nullable<Text>,
    }
}

diesel::table! {
    mydata_broad_pools (pool_id) {
        pool_id -> Text,
        name -> Text,
        platform_address -> Nullable<Text>,
        created_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    mydata_sub_pools (sub_pool_id) {
        sub_pool_id -> Text,
        broad_pool_id -> Text,
        name -> Text,
        created_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    mydata_listing_sub_pools (listing_id, sub_pool_id) {
        listing_id -> Text,
        sub_pool_id -> Text,
        assigned_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    mydata_merkle_roots (snapshot_id) {
        snapshot_id -> Text,
        root_hash -> Text,
        published_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    mydata_snapshot_anchors (id, time) {
        id -> Int4,
        snapshot_id -> Text,
        buyer_address -> Text,
        price_paid -> Int8,
        source_pool_id -> Text,
        source_sub_pool_id -> Text,
        platform_address -> Nullable<Text>,
        initial_escrow -> Int8,
        created_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
        manifest_hash -> Nullable<Text>,
        payment_reference -> Nullable<Text>,
    }
}

diesel::table! {
    mydata_distribution_rounds (snapshot_id) {
        snapshot_id -> Text,
        total_amount -> Int8,
        contributor_count -> Int8,
        merkle_root -> Text,
        platform_address -> Nullable<Text>,
        claim_deadline_ms -> Int8,
        published_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    mydata_snapshot_escrow (snapshot_id) {
        snapshot_id -> Text,
        total_funded -> Int8,
        total_claimed -> Int8,
        remaining_amount -> Int8,
        claim_deadline_ms -> Nullable<Int8>,
        reclaimed_at_ms -> Nullable<Int8>,
        status -> Text,
        updated_at_ms -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    mydata_claims (id, time) {
        id -> Int4,
        snapshot_id -> Text,
        claimant -> Text,
        amount -> Int8,
        gross_amount -> Int8,
        platform_fee -> Int8,
        ecosystem_fee -> Int8,
        net_amount -> Int8,
        platform_address -> Nullable<Text>,
        claimed_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    nominated_delegates (id, time) {
        id -> Int4,
        address -> Text,
        registry_type -> Int2,
        upvotes -> Int8,
        downvotes -> Int8,
        scheduled_term_start_epoch -> Int8,
        nomination_time -> Int8,
        status -> Int2,
        time -> Timestamptz,
        transaction_id -> Text,
        governance_registry_id -> Text,
    }
}

diesel::table! {
    object_migrated_events (id) {
        id -> Int4,
        #[max_length = 66]
        object_id -> Varchar,
        #[max_length = 255]
        object_type -> Varchar,
        old_version -> Int8,
        new_version -> Int8,
        #[max_length = 66]
        migrated_by -> Varchar,
        #[max_length = 128]
        event_id -> Varchar,
        #[max_length = 128]
        transaction_id -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    platform_blocked_profiles (id) {
        id -> Int4,
        platform_id -> Text,
        wallet_address -> Text,
        blocked_by -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    platform_events (id, created_at) {
        id -> Int4,
        event_type -> Text,
        platform_id -> Text,
        event_data -> Jsonb,
        event_id -> Nullable<Text>,
        created_at -> Timestamp,
        reasoning -> Nullable<Text>,
    }
}

diesel::table! {
    platform_memberships (id) {
        id -> Int4,
        platform_id -> Text,
        wallet_address -> Text,
        joined_at -> Timestamp,
        left_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    platform_moderator_permissions (id) {
        id -> Int4,
        platform_id -> Text,
        moderator_address -> Text,
        permission_type -> Text,
        granted_by -> Text,
        granted_at -> Timestamp,
        revoked_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    platform_moderators (id) {
        id -> Int4,
        platform_id -> Text,
        moderator_address -> Text,
        added_by -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    platform_treasury_balances (platform_id) {
        platform_id -> Text,
        balance_mist -> Int8,
        last_funded_at -> Nullable<Int8>,
        last_withdrawn_at -> Nullable<Int8>,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    platform_treasury_withdrawals (id) {
        id -> Int4,
        platform_id -> Text,
        recipient -> Text,
        amount -> Int8,
        reason_code -> Int2,
        executed_by -> Text,
        timestamp -> Int8,
        created_at -> Timestamp,
        event_id -> Nullable<Text>,
    }
}

diesel::table! {
    platforms (id) {
        id -> Int4,
        platform_id -> Text,
        name -> Text,
        tagline -> Text,
        description -> Nullable<Text>,
        logo -> Nullable<Text>,
        cover_photo -> Nullable<Text>,
        media_previews -> Nullable<Jsonb>,
        developer_address -> Text,
        moderators_group_id -> Nullable<Text>,
        terms_of_service -> Nullable<Text>,
        privacy_policy -> Nullable<Text>,
        redirect_uri -> Nullable<Text>,
        #[sql_name = "platforms"]
        platform_names -> Nullable<Jsonb>,
        links -> Nullable<Jsonb>,
        status -> Int2,
        release_date -> Nullable<Text>,
        shutdown_date -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        is_approved -> Bool,
        approval_changed_at -> Nullable<Timestamp>,
        approved_by -> Nullable<Text>,
        wants_dao_governance -> Nullable<Bool>,
        governance_registry_id -> Nullable<Text>,
        delegate_count -> Nullable<Int8>,
        delegate_term_epochs -> Nullable<Int8>,
        max_votes_per_user -> Nullable<Int8>,
        proposal_submission_cost -> Nullable<Int8>,
        quadratic_base_cost -> Nullable<Int8>,
        quorum_votes -> Nullable<Int8>,
        voting_period_epochs -> Nullable<Int8>,
        version -> Nullable<Int8>,
        primary_category -> Varchar,
        secondary_category -> Nullable<Varchar>,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    poc_analysis_results (post_id, time) {
        post_id -> Text,
        media_type -> Int2,
        similarity_detected -> Bool,
        highest_similarity_score -> Int8,
        oracle_address -> Text,
        original_creator -> Nullable<Text>,
        analysis_timestamp -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
        reasoning -> Nullable<Text>,
        evidence_urls -> Nullable<Jsonb>,
    }
}

diesel::table! {
    poc_badges (badge_id, time) {
        badge_id -> Text,
        post_id -> Text,
        media_type -> Int2,
        issued_by -> Text,
        issued_at -> Int8,
        revoked -> Nullable<Bool>,
        revoked_at -> Nullable<Int8>,
        transaction_id -> Text,
        time -> Timestamptz,
        beneficiary_address -> Nullable<Text>,
        matched_anchor_id -> Nullable<Text>,
        media_index -> Nullable<Int2>,
    }
}

diesel::table! {
    poc_config (id, time) {
        id -> Int4,
        image_threshold -> Int8,
        video_threshold -> Int8,
        audio_threshold -> Int8,
        revenue_redirect_percentage -> Int8,
        dispute_cost -> Int8,
        min_vote_stake -> Int8,
        max_vote_stake -> Int8,
        voting_duration_ms -> Int8,
        updated_by -> Text,
        updated_at -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
        max_reasoning_length -> Int8,
        max_evidence_urls -> Int8,
        max_votes_per_dispute -> Int8,
        dispute_governance_registry_id -> Nullable<Text>,
        oracle_address -> Nullable<Text>,
        claim_treasury_fee_bps -> Int8,
        max_referral_bps -> Int8,
        video_embedded_audio_redirect_bps -> Int8,
        dispute_quorum_base_stake -> Int8,
        dispute_second_round_fee_multiplier_bps -> Int8,
        dispute_second_round_quorum_multiplier_bps -> Int8,
        username_beneficiary_join_referral_bps -> Int8,
        max_disputes_per_post -> Int2,
        min_vault_deposit_amount -> Int8,
        media_asset_dispute_cost -> Int8,
        max_disputes_per_media_asset -> Int2,
        max_embedded_asset_redirect_bps -> Int8,
        version -> Int8,
    }
}

diesel::table! {
    media_asset_governance_links (media_asset_id, proposal_id, time) {
        media_asset_id -> Text,
        proposal_id -> Text,
        submitter -> Text,
        claims_commitment -> Bytea,
        status -> Int2,
        related_post_id -> Nullable<Text>,
        rights_disputes_submitted -> Int2,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    media_asset_rights_updates (media_asset_id, rights_version, time) {
        media_asset_id -> Text,
        rights_version -> Int8,
        proposal_id -> Nullable<Text>,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    poc_creator_identity_links (creator_identity_source, creator_identity_hash) {
        creator_identity_source -> Int2,
        creator_identity_hash -> Text,
        wallet_address -> Text,
        beneficiary_id -> Text,
        linked_at_ms -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    poc_beneficiary_vaults (vault_id) {
        vault_id -> Text,
        vault_routing_key -> Text,
        updated_at_ms -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    poc_vault_coin_balances (vault_id, coin_type) {
        vault_id -> Text,
        coin_type -> Text,
        balance -> Int8,
        updated_at_ms -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    poc_vault_deposits (id) {
        id -> Int8,
        vault_id -> Text,
        vault_routing_key -> Text,
        amount -> Int8,
        coin_type -> Text,
        source_post_id -> Nullable<Text>,
        occurred_at_ms -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    poc_vault_claims (id) {
        id -> Int8,
        vault_id -> Text,
        vault_routing_key -> Text,
        coin_type -> Text,
        referrer_address -> Nullable<Text>,
        treasury_amount -> Int8,
        referrer_amount -> Int8,
        beneficiary_amount -> Int8,
        occurred_at_ms -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
        claim_kind -> Nullable<Text>,
        gross_amount -> Int8,
    }
}

diesel::table! {
    poc_username_beneficiaries (beneficiary_id) {
        beneficiary_id -> Text,
        username -> Text,
        status -> Int2,
        creator_identity_source -> Int2,
        creator_identity_hash -> Text,
        vault_routing_key -> Text,
        vault_id -> Text,
        required_x_handle -> Text,
        oracle_evidence_hash -> Text,
        provisioned_at_ms -> Int8,
        provisioned_by -> Text,
        claimed_profile_id -> Nullable<Text>,
        claimed_by -> Nullable<Text>,
        claimed_at_ms -> Nullable<Int8>,
        ended_at_ms -> Nullable<Int8>,
        ended_by -> Nullable<Text>,
        end_reason_code -> Nullable<Int2>,
        join_referrer -> Nullable<Text>,
        join_referral_paid -> Bool,
        join_referral_paid_at_ms -> Nullable<Int8>,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    poc_username_beneficiary_events (id, time) {
        id -> Int4,
        event_type -> Text,
        beneficiary_id -> Nullable<Text>,
        username -> Nullable<Text>,
        payload_json -> Jsonb,
        transaction_id -> Text,
        event_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    poc_dispute_votes (dispute_id, voter, time) {
        dispute_id -> Text,
        voter -> Text,
        vote_choice -> Int2,
        stake_amount -> Int8,
        voted_at -> Int8,
        reward_claimed -> Nullable<Bool>,
        reward_amount -> Nullable<Int8>,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    poc_disputes (dispute_id, time) {
        dispute_id -> Text,
        post_id -> Text,
        disputer -> Text,
        dispute_type -> Int2,
        evidence -> Text,
        status -> Int2,
        stake_amount -> Int8,
        voting_start_ms -> Int8,
        voting_end_ms -> Int8,
        resolution -> Nullable<Int2>,
        winning_side -> Nullable<Int2>,
        total_winning_stake -> Nullable<Int8>,
        total_losing_stake -> Nullable<Int8>,
        submitted_at -> Int8,
        resolved_at -> Nullable<Int8>,
        transaction_id -> Text,
        time -> Timestamptz,
        dispute_round -> Int2,
        effective_dispute_fee -> Int8,
        required_total_stake_quorum -> Int8,
        quorum_met -> Nullable<Bool>,
    }
}

diesel::table! {
    poc_revenue_redirections (redirection_id, time) {
        redirection_id -> Text,
        accused_post_id -> Text,
        original_post_id -> Text,
        redirect_percentage -> Int8,
        similarity_score -> Int8,
        created_at -> Int8,
        removed -> Nullable<Bool>,
        removed_at -> Nullable<Int8>,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    post_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        max_content_length -> Int8,
        max_media_urls -> Int8,
        max_mentions -> Int8,
        max_metadata_size -> Int8,
        max_description_length -> Int8,
        max_reaction_length -> Int8,
        commenter_tip_percentage -> Int8,
        repost_tip_percentage -> Int8,
        min_promotion_amount -> Int8,
        max_promotion_amount -> Int8,
        min_view_duration_ms -> Int8,
        platform_fee_bps -> Int8,
        ecosystem_fee_bps -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        version -> Int8,
    }
}

diesel::table! {
    posts (post_id, time) {
        post_id -> Text,
        owner -> Text,
        profile_id -> Text,
        content -> Text,
        media_urls -> Nullable<Jsonb>,
        mentions -> Nullable<Jsonb>,
        metadata_json -> Nullable<Jsonb>,
        post_type -> Text,
        parent_post_id -> Nullable<Text>,
        created_at -> Int8,
        updated_at -> Nullable<Int8>,
        deleted_at -> Nullable<Int8>,
        reaction_count -> Nullable<Int8>,
        comment_count -> Nullable<Int8>,
        repost_count -> Nullable<Int8>,
        tips_received -> Nullable<Int8>,
        total_tip_volume -> Int8,
        removed_from_platform -> Nullable<Bool>,
        removed_by -> Nullable<Text>,
        transaction_id -> Text,
        time -> Timestamptz,
        promotion_id -> Nullable<Text>,
        poc_id -> Nullable<Text>,
        revenue_redirect_to -> Nullable<Text>,
        revenue_redirect_percentage -> Nullable<Int8>,
        requires_subscription -> Nullable<Bool>,
        subscription_service_id -> Nullable<Text>,
        subscription_price -> Nullable<Int8>,
        subscription_min_tier_level -> Nullable<Int8>,
        post_access_kind -> Nullable<Text>,
        encrypted_content_hash -> Nullable<Text>,
        enable_spt -> Bool,
        spt_id -> Nullable<Text>,
        spot_analysis_status -> Int2,
        spot_detected_claim_count -> Int8,
        spot_rejected_claim_count -> Int8,
        spot_truncated_claim_count -> Int8,
        spot_future_accepted_count -> Int8,
        spot_past_verified_count -> Int8,
        spot_max_claim_per_post_applied -> Int8,
        spot_claim_indexes -> Jsonb,
        spot_claim_ids -> Jsonb,
        spot_market_ids -> Jsonb,
        spot_policy_hashes -> Jsonb,
        spot_claim_manifest_hash -> Nullable<Text>,
        spot_veracity_manifest_hash -> Nullable<Text>,
        spot_analysis_tx_digest -> Nullable<Text>,
        spot_analyzed_checkpoint -> Nullable<Int8>,
        poc_reasoning -> Nullable<Text>,
        poc_evidence_urls -> Nullable<Jsonb>,
        poc_similarity_score -> Nullable<Int8>,
        poc_media_type -> Nullable<Int2>,
        poc_oracle_address -> Nullable<Text>,
        poc_analyzed_at -> Nullable<Int8>,
        poc_outcome -> Nullable<Int2>,
        poc_redirection_kind -> Nullable<Int2>,
        poc_disputes_submitted -> Int2,
        mydata_id -> Nullable<Text>,
        revenue_recipient -> Nullable<Text>,
        platform_id -> Nullable<Text>,
        permissions -> Nullable<Int2>,
        sub_agent_id -> Nullable<Text>,
        action_identity_class -> Nullable<Int2>,
        organization_id -> Nullable<Text>,
        contract_version -> Int8,
        composition_status -> Nullable<Int2>,
        monetization_status -> Nullable<Int2>,
        media_asset_ids -> Nullable<Jsonb>,
        embedded_bindings -> Nullable<Jsonb>,
        usage_decisions -> Nullable<Jsonb>,
        usage_denials -> Nullable<Jsonb>,
    }
}

diesel::table! {
    media_assets (media_asset_id, time) {
        media_asset_id -> Text,
        content_commitment -> Bytea,
        media_type -> Int2,
        asset_kind -> Int2,
        originality_status -> Int2,
        provenance_status -> Int2,
        lineage_parent_id -> Nullable<Text>,
        rights_version -> Int8,
        economics_version -> Int8,
        registered_by -> Text,
        registered_at -> Int8,
        verified_at -> Nullable<Int8>,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    fingerprint_observations (fingerprint_commitment, media_asset_id, time) {
        fingerprint_commitment -> Bytea,
        media_asset_id -> Text,
        linked_at -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    media_asset_usages (container_id, asset_id, usage_class, position, time) {
        container_id -> Text,
        container_type -> Int2,
        asset_id -> Text,
        usage_class -> Int2,
        position -> Int2,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    composition_analysis_records (post_id, analyzed_at, time) {
        post_id -> Text,
        analyzed_at -> Int8,
        usage_context -> Int2,
        composition_status -> Int2,
        monetization_status -> Int2,
        analysis_json -> Jsonb,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    revenue_manifests (post_id, manifest_version, time) {
        post_id -> Text,
        manifest_version -> Int8,
        entries_json -> Jsonb,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    media_asset_derivative_edges (child_asset_id, parent_asset_id, relationship_id, time) {
        child_asset_id -> Text,
        parent_asset_id -> Text,
        relationship_id -> Int8,
        relationship_type -> Int2,
        license_instance_id -> Text,
        template_version_id -> Text,
        parent_share_bps -> Int8,
        ancestry_version -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    media_asset_ancestry_snapshots (media_asset_id, ancestry_version, time) {
        media_asset_id -> Text,
        ancestry_version -> Int8,
        ancestor_ids -> Jsonb,
        ancestry_hash -> Nullable<Text>,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    license_template_versions (template_version_id, time) {
        template_version_id -> Text,
        family_id -> Text,
        version -> Int8,
        creator -> Text,
        granted_rights -> Int8,
        allow_derivatives -> Bool,
        attribution_required -> Bool,
        royalty_bps -> Int8,
        derivative_royalty_bps -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    license_instances (license_instance_id, time) {
        license_instance_id -> Text,
        template_version_id -> Text,
        licensor_asset_id -> Text,
        licensee -> Text,
        status -> Int2,
        accepted_at -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    media_asset_resolved_policies (media_asset_id, policy_version, time) {
        media_asset_id -> Text,
        policy_version -> Int8,
        effective_rights -> Int8,
        derivatives_allowed -> Bool,
        attribution_required -> Bool,
        commercial_allowed -> Bool,
        lineage_json -> Jsonb,
        lineage_hash -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    media_asset_resolved_obligations (media_asset_id, policy_version, obligation_index, time) {
        media_asset_id -> Text,
        policy_version -> Int8,
        obligation_index -> Int4,
        beneficiary_asset_id -> Nullable<Text>,
        beneficiary_address -> Text,
        share_bps -> Int8,
        source_relationship_id -> Nullable<Int8>,
        source_license_instance_id -> Nullable<Text>,
        obligation_kind -> Int2,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    post_usage_decision_events (post_id, binding_id, time) {
        post_id -> Text,
        binding_id -> Int8,
        playback_permitted -> Bool,
        payout_permitted -> Bool,
        policy_reason_code -> Int2,
        policy_version -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    detected_asset_relationships (proposal_id, time) {
        proposal_id -> Text,
        accused_pending_id -> Text,
        accused_asset_id -> Nullable<Text>,
        original_asset_id -> Text,
        similarity_bps -> Int8,
        evidence_commitment -> Nullable<Bytea>,
        detected_by -> Text,
        detected_at -> Int8,
        status -> Int2,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    posts_deletion_events (id, time) {
        id -> Int4,
        object_id -> Text,
        owner -> Text,
        profile_id -> Text,
        is_post -> Bool,
        post_type -> Nullable<Text>,
        post_id -> Nullable<Text>,
        deleted_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    posts_moderation_events (id, time) {
        id -> Int4,
        object_id -> Text,
        platform_id -> Text,
        removed -> Bool,
        moderated_by -> Text,
        moderated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    posts_reports (id, time) {
        id -> Int4,
        object_id -> Text,
        is_comment -> Bool,
        reporter -> Text,
        reason_code -> Int2,
        description -> Text,
        reported_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    posts_transfers (id, time) {
        id -> Int4,
        object_id -> Text,
        previous_owner -> Text,
        new_owner -> Text,
        is_post -> Bool,
        transferred_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    profile_badges (id, time) {
        id -> Int4,
        profile_id -> Text,
        badge_id -> Text,
        badge_name -> Text,
        badge_description -> Nullable<Text>,
        badge_media_url -> Nullable<Text>,
        platform_id -> Text,
        assigned_by -> Text,
        assigned_at -> Int8,
        revoked -> Nullable<Bool>,
        revoked_at -> Nullable<Int8>,
        revoked_by -> Nullable<Text>,
        badge_type -> Int2,
        transaction_id -> Text,
        time -> Timestamptz,
        badge_icon_url -> Nullable<Text>,
    }
}

diesel::table! {
    profile_events (id, created_at) {
        id -> Int4,
        event_type -> Text,
        profile_id -> Text,
        event_data -> Jsonb,
        event_id -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    username_listings (id, time) {
        id -> Int4,
        username -> Text,
        seller_address -> Text,
        seller_profile_id -> Text,
        min_price -> Int8,
        status -> Text,
        created_at -> Int8,
        cancelled_at -> Nullable<Int8>,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    username_offers (id, time) {
        id -> Int4,
        username -> Text,
        seller_profile_id -> Text,
        buyer_address -> Text,
        buyer_profile_id -> Text,
        amount -> Int8,
        status -> Text,
        created_at -> Int8,
        updated_at -> Int8,
        resolved_at -> Nullable<Int8>,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    username_sale_fees (id, time) {
        id -> Int4,
        username -> Text,
        seller_address -> Text,
        seller_profile_id -> Text,
        buyer_address -> Text,
        buyer_profile_id -> Text,
        sale_amount -> Int8,
        fee_amount -> Int8,
        fee_recipient_address -> Text,
        timestamp -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    profile_subscription_services (service_id) {
        service_id -> Text,
        profile_owner -> Text,
        profile_id -> Text,
        plan_count -> Int8,
        active -> Bool,
        subscriber_count -> Int8,
        created_at -> Int8,
        updated_at -> Nullable<Int8>,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    profile_subscription_plans (plan_id) {
        plan_id -> Text,
        service_id -> Text,
        title -> Text,
        description -> Nullable<Text>,
        price -> Int8,
        duration_ms -> Int8,
        tier_level -> Nullable<Int8>,
        platform_id -> Nullable<Text>,
        active -> Bool,
        created_at -> Int8,
        updated_at -> Nullable<Int8>,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    profile_subscriptions (subscription_id, time) {
        subscription_id -> Text,
        service_id -> Text,
        plan_id -> Text,
        tier_level -> Nullable<Int8>,
        platform_id -> Nullable<Text>,
        price -> Int8,
        duration_ms -> Int8,
        subscriber -> Text,
        created_at -> Int8,
        expires_at -> Int8,
        auto_renew -> Bool,
        renewal_balance -> Int8,
        renewal_count -> Int8,
        cancelled_at -> Nullable<Int8>,
        time -> Timestamptz,
        transaction_id -> Text,
        processing_success -> Bool,
        processing_error -> Nullable<Text>,
    }
}

diesel::table! {
    ai_credit_balances (balance_id) {
        balance_id -> Text,
        memory_account_id -> Text,
        principal_owner -> Text,
        profile_id -> Text,
        balance_mist -> Int8,
        spent_total_mist -> Int8,
        reserved_mist -> Int8,
        daily_cap_mist -> Nullable<Int8>,
        monthly_cap_mist -> Nullable<Int8>,
        spent_day_mist -> Int8,
        spent_month_mist -> Int8,
        settlement_nonce -> Int8,
        reservation_nonce -> Int8,
        active -> Bool,
        contract_version -> Int8,
        updated_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    ai_credit_agent_budgets (balance_id, agent_object_id) {
        balance_id -> Text,
        agent_object_id -> Text,
        budget_mist -> Nullable<Int8>,
        spent_mist -> Int8,
        reserved_mist -> Int8,
        daily_cap_mist -> Nullable<Int8>,
        monthly_cap_mist -> Nullable<Int8>,
        require_approval_above_mist -> Nullable<Int8>,
        enabled -> Bool,
        updated_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    ai_spend_reservations (balance_id, reservation_nonce) {
        balance_id -> Text,
        reservation_nonce -> Int8,
        agent_object_id -> Text,
        status -> Text,
        max_amount_mist -> Int8,
        captured_mist -> Nullable<Int8>,
        released_mist -> Nullable<Int8>,
        provider_envelope_hash_hex -> Text,
        request_hash_hex -> Text,
        fx_quote_id_hex -> Text,
        myso_usd_e8 -> Int8,
        markup_bps -> Int8,
        provider_cost_usd_micros -> Nullable<Int8>,
        provider_generation_hash_hex -> Nullable<Text>,
        capture_deadline_ms -> Int8,
        hard_expiry_ms -> Int8,
        available_mist -> Int8,
        reserve_event_id -> Text,
        reserve_transaction_id -> Text,
        terminal_event_id -> Nullable<Text>,
        terminal_transaction_id -> Nullable<Text>,
        terminal_at_ms -> Nullable<Int8>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    ai_credit_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        oracle_pubkey_hex -> Text,
        treasury_address -> Text,
        min_deposit_mist -> Int8,
        max_single_settlement_mist -> Int8,
        receipt_ttl_ms -> Int8,
        oracle_markup_bps -> Int8,
        catalog_version -> Nullable<Text>,
        version -> Int8,
        updated_at -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    ai_credit_events (id, time) {
        id -> Int4,
        event_type -> Text,
        balance_id -> Nullable<Text>,
        memory_account_id -> Nullable<Text>,
        principal_owner -> Nullable<Text>,
        profile_id -> Nullable<Text>,
        agent_object_id -> Nullable<Text>,
        amount_mist -> Nullable<Int8>,
        new_balance_mist -> Nullable<Int8>,
        receipt_id -> Nullable<Text>,
        usage_kind -> Nullable<Int2>,
        settlement_nonce -> Nullable<Int8>,
        remaining_mist -> Nullable<Int8>,
        daily_cap_mist -> Nullable<Int8>,
        monthly_cap_mist -> Nullable<Int8>,
        budget_mist -> Nullable<Int8>,
        require_approval_above_mist -> Nullable<Int8>,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    ai_credit_usage_lines (id) {
        id -> Int8,
        receipt_id -> Text,
        balance_id -> Text,
        agent_object_id -> Text,
        usage_kind -> Int2,
        amount_mist -> Int8,
        model_id -> Nullable<Text>,
        tool_id -> Nullable<Text>,
        metadata -> Nullable<Jsonb>,
        settled -> Bool,
        settlement_tx -> Nullable<Text>,
        created_at -> Timestamptz,
        organization_id -> Nullable<Text>,
    }
}

diesel::table! {
    ai_credit_spend_approvals (balance_id, agent_object_id) {
        balance_id -> Text,
        agent_object_id -> Text,
        status -> Text,
        requested_amount_mist -> Nullable<Int8>,
        threshold_mist -> Nullable<Int8>,
        approval_nonce -> Nullable<Int8>,
        max_amount_mist -> Nullable<Int8>,
        expires_at_ms -> Nullable<Int8>,
        approved_by -> Nullable<Text>,
        approved_by_agent_id -> Nullable<Text>,
        organization_id -> Nullable<Text>,
        consumed_amount_mist -> Nullable<Int8>,
        requested_at -> Timestamptz,
        updated_at -> Timestamptz,
        event_id -> Nullable<Text>,
    }
}

diesel::table! {
    org_invitations (organization_id, invitee_address) {
        organization_id -> Text,
        invitee_address -> Text,
        role_name -> Nullable<Text>,
        permissions_mask -> Int8,
        status -> Text,
        invited_by -> Text,
        created_at_ms -> Int8,
        expires_at_ms -> Nullable<Int8>,
        responded_at_ms -> Nullable<Int8>,
        responded_by -> Nullable<Text>,
        granted_mask -> Nullable<Int8>,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    org_memory_permissions (organization_id, member_address, permission_kind) {
        organization_id -> Text,
        member_address -> Text,
        permission_kind -> Int8,
        active -> Bool,
        granted_by -> Text,
        group_id -> Nullable<Text>,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    org_roles (organization_id, role_name) {
        organization_id -> Text,
        role_name -> Text,
        mask -> Int8,
        is_builtin -> Bool,
        defined_by -> Text,
        active -> Bool,
        updated_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    org_role_assignments (organization_id, member_address, role_name) {
        organization_id -> Text,
        member_address -> Text,
        role_name -> Text,
        role_mask -> Int8,
        assigned_mask -> Int8,
        active -> Bool,
        assigned_by -> Text,
        assigned_at_ms -> Int8,
        revoked_at_ms -> Nullable<Int8>,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    audit_log (id, time) {
        id -> Int8,
        time -> Timestamptz,
        source -> Text,
        actor_address -> Text,
        actor_type -> Text,
        action -> Text,
        target_type -> Text,
        target_id -> Text,
        organization_id -> Nullable<Text>,
        account_id -> Nullable<Text>,
        prev_state -> Nullable<Jsonb>,
        new_state -> Nullable<Jsonb>,
        tx_digest -> Nullable<Text>,
        event_id -> Nullable<Text>,
        idempotency_key -> Nullable<Text>,
        metadata -> Nullable<Jsonb>,
    }
}

diesel::table! {
    memory_usage_stats (agent_object_id) {
        agent_object_id -> Text,
        organization_id -> Nullable<Text>,
        account_id -> Nullable<Text>,
        entries -> Int8,
        bytes -> Int8,
        org_shared_entries -> Int8,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    profiles (id) {
        id -> Int4,
        owner_address -> Text,
        username -> Text,
        display_name -> Nullable<Text>,
        bio -> Nullable<Text>,
        profile_photo -> Nullable<Text>,
        website -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        post_count -> Int4,
        cover_photo -> Nullable<Text>,
        profile_id -> Nullable<Text>,
        birthdate -> Nullable<Text>,
        location -> Nullable<Text>,
        x_username -> Nullable<Text>,
        followers_count -> Int4,
        following_count -> Int4,
        subscription_service_id -> Nullable<Text>,
        subscription_enabled -> Nullable<Bool>,
        blocked_count -> Int4,
        social_proof_token_address -> Nullable<Varchar>,
        selected_badge_id -> Nullable<Varchar>,
        reservation_pool_address -> Nullable<Varchar>,
        selected_ecosystem_badge_id -> Nullable<Varchar>,
        search_text -> Nullable<Text>,
        memory_account_id -> Nullable<Text>,
        ai_credit_balance_id -> Nullable<Text>,
        contract_version -> Int8,
    }
}

diesel::table! {
    username_registry (username) {
        username -> Text,
        profile_id -> Text,
        transaction_id -> Text,
    }
}

diesel::table! {
    username_reservations (id) {
        id -> Int4,
        username -> Text,
        reason -> Int2,
        reserved_by -> Text,
        reserved_at -> Int8,
        released_by -> Nullable<Text>,
        released_at -> Nullable<Int8>,
        status -> Text,
        reserve_transaction_id -> Text,
        release_transaction_id -> Nullable<Text>,
        time -> Timestamptz,
    }
}

diesel::table! {
    wallet_messaging_policies (wallet_address) {
        wallet_address -> Text,
        enabled -> Bool,
        min_cost -> Nullable<Int8>,
        updated_at -> Int8,
    }
}

diesel::table! {
    sub_agent_memory_vaults (vault_id) {
        vault_id -> Text,
        agent_object_id -> Text,
        memory_account_id -> Text,
        created_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    sub_agent_organizations (organization_id) {
        organization_id -> Text,
        account_id -> Text,
        principal_owner -> Text,
        profile_id -> Text,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        org_type -> Int2,
        root_agent_id -> Nullable<Text>,
        active -> Bool,
        created_at_ms -> Int8,
        deactivated_at_ms -> Nullable<Int8>,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
        org_memory_group_id -> Nullable<Text>,
    }
}

diesel::table! {
    sub_agent_organization_counterparties (organization_id, counterparty_address) {
        organization_id -> Text,
        counterparty_address -> Text,
        first_interaction_at_ms -> Int8,
        last_interaction_at_ms -> Int8,
        interaction_count -> Int8,
    }
}

diesel::table! {
    sub_agent_organization_events (id, time) {
        id -> Int4,
        event_type -> Text,
        organization_id -> Nullable<Text>,
        account_id -> Nullable<Text>,
        principal_owner -> Nullable<Text>,
        profile_id -> Nullable<Text>,
        name -> Nullable<Text>,
        description -> Nullable<Text>,
        org_type -> Nullable<Int2>,
        previous_org_type -> Nullable<Int2>,
        root_agent_id -> Nullable<Text>,
        agent_object_id -> Nullable<Text>,
        active -> Nullable<Bool>,
        created_at_ms -> Nullable<Int8>,
        deactivated_at_ms -> Nullable<Int8>,
        updated_at_ms -> Nullable<Int8>,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    sub_agent_organization_stats (organization_id) {
        organization_id -> Text,
        total_agents -> Int4,
        active_agents -> Int4,
        max_tree_depth -> Int2,
        total_posts -> Int8,
        total_comments -> Int8,
        total_reactions -> Int8,
        total_reposts -> Int8,
        total_engagement -> Int8,
        total_revenue_myso -> Int8,
        total_outbound_spend_myso -> Int8,
        net_cash_flow_myso -> Int8,
        estimated_assets_under_management_myso -> Int8,
        attribution_coverage_bps -> Int4,
        total_spot_participation -> Int8,
        spot_bets_placed -> Int8,
        spot_bets_resolved -> Int8,
        spot_bets_correct -> Int8,
        spot_accuracy_bps -> Nullable<Int4>,
        originality_posts_analyzed -> Int8,
        originality_score_average_bps -> Nullable<Int4>,
        total_counterparties -> Int8,
        total_actions_executed -> Int8,
        total_transactions -> Int8,
        last_activity_at_ms -> Nullable<Int8>,
        stats_rollup_at -> Nullable<Timestamptz>,
        updated_at -> Timestamptz,
        ai_credit_spent_mist -> Int8,
        ai_credit_usage_events -> Int8,
        memory_entries -> Int8,
        memory_bytes -> Int8,
        org_shared_memory_entries -> Int8,
    }
}

diesel::table! {
    sub_agent_organization_stats_daily (organization_id, snapshot_date, time) {
        organization_id -> Text,
        org_type -> Int2,
        snapshot_date -> Date,
        total_revenue_myso -> Int8,
        net_cash_flow_myso -> Int8,
        total_outbound_spend_myso -> Int8,
        total_counterparties -> Int8,
        active_agents -> Int4,
        total_engagement -> Int8,
        estimated_aum_myso -> Int8,
        total_actions_executed -> Int8,
        growth_score -> Int8,
        spot_accuracy_bps -> Nullable<Int4>,
        attribution_coverage_bps -> Int4,
        time -> Timestamptz,
        ai_credit_spent_mist -> Int8,
        memory_bytes -> Int8,
    }
}

diesel::table! {
    memory_accounts (account_id) {
        account_id -> Text,
        principal_owner -> Text,
        profile_id -> Text,
        active -> Bool,
        contract_version -> Int8,
        created_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    sub_agents (agent_object_id) {
        agent_object_id -> Text,
        derived_address -> Text,
        account_id -> Text,
        label -> Text,
        identity_class -> Int2,
        role_tags -> Int8,
        capabilities -> Int8,
        delegatable_caps -> Int8,
        register_scope -> Int2,
        approval_required_caps -> Int8,
        max_action_spend -> Nullable<Int8>,
        platform_scope -> Nullable<Text>,
        parent_object_id -> Nullable<Text>,
        depth -> Int2,
        registered_by -> Text,
        expires_at_ms -> Nullable<Int8>,
        active -> Bool,
        created_at_ms -> Int8,
        deactivated_at_ms -> Nullable<Int8>,
        revoked_at_ms -> Nullable<Int8>,
        updated_at_ms -> Int8,
        organization_id -> Nullable<Text>,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    sub_agent_events (id, time) {
        id -> Int4,
        event_type -> Text,
        account_id -> Nullable<Text>,
        principal_owner -> Nullable<Text>,
        profile_id -> Nullable<Text>,
        agent_object_id -> Nullable<Text>,
        derived_address -> Nullable<Text>,
        label -> Nullable<Text>,
        identity_class -> Nullable<Int2>,
        role_tags -> Nullable<Int8>,
        capabilities -> Nullable<Int8>,
        delegatable_caps -> Nullable<Int8>,
        register_scope -> Nullable<Int2>,
        approval_required_caps -> Nullable<Int8>,
        max_action_spend -> Nullable<Int8>,
        platform_scope -> Nullable<Text>,
        parent_object_id -> Nullable<Text>,
        depth -> Nullable<Int2>,
        registered_by -> Nullable<Text>,
        expires_at_ms -> Nullable<Int8>,
        active -> Nullable<Bool>,
        created_at_ms -> Nullable<Int8>,
        revoked_count -> Nullable<Int8>,
        previous_owner -> Nullable<Text>,
        new_owner -> Nullable<Text>,
        migration_from_version -> Nullable<Int8>,
        migration_to_version -> Nullable<Int8>,
        registry_id -> Nullable<Text>,
        organization_id -> Nullable<Text>,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    progress_store (id) {
        id -> Int4,
        worker_id -> Text,
        module_name -> Text,
        last_processed_checkpoint -> Int8,
        last_processed_event_id -> Nullable<Text>,
        last_processed_timestamp -> Int8,
        processing_state -> Text,
        error_count -> Int4,
        last_error_message -> Nullable<Text>,
        last_error_at -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    promoted_posts (id, time) {
        id -> Int4,
        promotion_id -> Text,
        post_id -> Text,
        owner -> Text,
        profile_id -> Text,
        payment_per_view -> Int8,
        total_budget -> Int8,
        remaining_budget -> Int8,
        active -> Bool,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    promotion_budget_events (id, time) {
        id -> Int4,
        promotion_id -> Text,
        post_id -> Text,
        event_type -> Text,
        amount -> Int8,
        remaining_budget -> Int8,
        timestamp -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    promotion_status_events (id, time) {
        id -> Int4,
        post_id -> Text,
        promotion_id -> Text,
        event_type -> Text,
        triggered_by -> Text,
        new_status -> Nullable<Bool>,
        amount -> Nullable<Int8>,
        timestamp -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    promotion_views (id, time) {
        id -> Int4,
        post_id -> Text,
        promotion_id -> Text,
        viewer -> Text,
        payment_amount -> Int8,
        platform_fee -> Int8,
        ecosystem_fee -> Int8,
        recipient_amount -> Int8,
        view_duration -> Int8,
        platform_id -> Text,
        timestamp -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    proposals (id, time) {
        id -> Text,
        title -> Text,
        description -> Text,
        proposal_type -> Int2,
        reference_id -> Nullable<Text>,
        metadata_json -> Nullable<Jsonb>,
        submitter -> Text,
        submission_time -> Int8,
        delegate_approval_count -> Int8,
        delegate_rejection_count -> Int8,
        community_votes_for -> Int8,
        community_votes_against -> Int8,
        status -> Int2,
        voting_start_time -> Nullable<Int8>,
        voting_end_time -> Nullable<Int8>,
        reward_pool -> Int8,
        implemented_description -> Nullable<Text>,
        implementation_time -> Nullable<Int8>,
        rescind_time -> Nullable<Int8>,
        time -> Timestamptz,
        transaction_id -> Text,
        anonymous_votes_for -> Nullable<Int8>,
        anonymous_votes_against -> Nullable<Int8>,
        anonymous_voters_count -> Nullable<Int8>,
        pending_anonymous_decryption -> Nullable<Bool>,
        anonymous_decryption_completed_at -> Nullable<Int8>,
        rejection_time -> Nullable<Int8>,
        governance_registry_id -> Text,
    }
}

diesel::table! {
    reaction_counts (id) {
        id -> Int4,
        object_id -> Text,
        reaction_text -> Text,
        count -> Int8,
    }
}

diesel::table! {
    reactions (id, time) {
        id -> Int4,
        object_id -> Text,
        user_address -> Text,
        reaction_text -> Text,
        is_post -> Bool,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        principal_owner -> Nullable<Text>,
        actor_address -> Nullable<Text>,
        sub_agent_id -> Nullable<Text>,
        action_identity_class -> Nullable<Int2>,
        organization_id -> Nullable<Text>,
    }
}

diesel::table! {
    reposts (id, time) {
        id -> Text,
        repost_id -> Text,
        original_id -> Text,
        original_post_id -> Text,
        is_original_post -> Bool,
        owner -> Text,
        profile_id -> Text,
        contract_version -> Int8,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        actor_address -> Nullable<Text>,
        sub_agent_id -> Nullable<Text>,
        action_identity_class -> Nullable<Int2>,
        organization_id -> Nullable<Text>,
    }
}

diesel::table! {
    reward_distributions (id, time) {
        id -> Int4,
        proposal_id -> Text,
        recipient_address -> Text,
        amount -> Int8,
        distribution_time -> Int8,
        distribution_type -> Nullable<Text>,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    social_graph_events (id, created_at) {
        id -> Int4,
        event_type -> Text,
        follower_address -> Text,
        following_address -> Text,
        created_at -> Timestamp,
        raw_event_data -> Nullable<Jsonb>,
        event_id -> Nullable<Text>,
    }
}

diesel::table! {
    social_graph_relationships (id) {
        id -> Int4,
        follower_address -> Text,
        following_address -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    social_proof_of_truth (id, time) {
        id -> Int4,
        event_type -> Text,
        post_id -> Text,
        user_address -> Nullable<Text>,
        escrow_amount -> Nullable<Int8>,
        amm_amount -> Nullable<Int8>,
        amount -> Nullable<Int8>,
        outcome -> Nullable<Int2>,
        total_escrow -> Nullable<Int8>,
        fee_taken -> Nullable<Int8>,
        confidence_bps -> Nullable<Int8>,
        timestamp_epoch -> Int8,
        time -> Timestamptz,
        event_id -> Nullable<Text>,
        transaction_id -> Nullable<Text>,
        raw_event -> Nullable<Jsonb>,
        option_id -> Nullable<Int2>,
    }
}

diesel::table! {
    spt_config (id, time) {
        id -> Int4,
        trading_enabled -> Bool,
        admin_address -> Text,
        reason -> Text,
        updated_by -> Text,
        post_threshold -> Int8,
        profile_threshold -> Int8,
        max_individual_reservation_bps -> Int8,
        total_fee_bps -> Int8,
        creator_fee_bps -> Int8,
        platform_fee_bps -> Int8,
        treasury_fee_bps -> Int8,
        trading_creator_fee_bps -> Int8,
        trading_platform_fee_bps -> Int8,
        trading_treasury_fee_bps -> Int8,
        reservation_creator_fee_bps -> Int8,
        reservation_platform_fee_bps -> Int8,
        reservation_treasury_fee_bps -> Int8,
        max_reservers_per_pool -> Int8,
        base_price -> Int8,
        quadratic_coefficient -> Int8,
        max_hold_percent_bps -> Int8,
        non_platform_platform_to_creator_bps -> Int8,
        non_platform_platform_to_treasury_bps -> Int8,
        version -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    spt_events (id) {
        id -> Int4,
        event_type -> Varchar,
        event_data -> Jsonb,
        event_id -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    spot_bet_withdrawals (id, time) {
        id -> Int4,
        post_id -> Text,
        user_address -> Text,
        option_id -> Int2,
        amount -> Int8,
        fee_taken -> Int8,
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    spot_bets (id, time) {
        id -> Int4,
        post_id -> Text,
        user_address -> Text,
        escrow_amount -> Int8,
        amm_amount -> Int8,
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        option_id -> Int2,
        organization_id -> Nullable<Text>,
        market_object_id -> Nullable<Text>,
        referrer_post_id -> Nullable<Text>,
    }
}

diesel::table! {
    spot_claims (id) {
        id -> Int4,
        claim_object_id -> Text,
        semantic_claim_hash -> Text,
        created_at_ms -> Int8,
        transaction_id -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    spot_creator_earnings_daily (creator_address, day) {
        creator_address -> Text,
        day -> Date,
        amount -> Int8,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    spot_creator_payouts (id) {
        id -> Int4,
        market_object_id -> Text,
        payout_id -> Int8,
        creator_address -> Text,
        referrer_post_id -> Text,
        amount -> Int8,
        expires_at_ms -> Int8,
        status -> Text,
        ecosystem_amount -> Nullable<Int8>,
        platform_amount -> Nullable<Int8>,
        claimed_at_ms -> Nullable<Int8>,
        reclaimed_at_ms -> Nullable<Int8>,
        transaction_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    spot_markets (id) {
        id -> Int4,
        market_object_id -> Text,
        claim_object_id -> Text,
        market_key_hash -> Text,
        primary_post_id -> Text,
        primary_creator -> Nullable<Text>,
        status -> Int2,
        outcome -> Nullable<Int2>,
        betting_options -> Jsonb,
        option_escrow -> Jsonb,
        resolution_window_ms -> Nullable<Int8>,
        max_resolution_window_ms -> Nullable<Int8>,
        resolution_at_ms -> Nullable<Int8>,
        created_at_ms -> Int8,
        last_resolution_at_ms -> Nullable<Int8>,
        resolution_timestamp_ms -> Nullable<Int8>,
        creator_fee_total -> Nullable<Int8>,
        transaction_id -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    spot_post_links (id) {
        id -> Int4,
        post_id -> Text,
        claim_object_id -> Text,
        market_object_id -> Nullable<Text>,
        link_kind -> Text,
        transaction_id -> Text,
        created_at -> Timestamp,
        claim_index -> Int8,
        policy_hash -> Text,
    }
}

diesel::table! {
    spot_post_analyses (post_id) {
        post_id -> Text,
        status -> Int2,
        detected_claim_count -> Int8,
        rejected_claim_count -> Int8,
        truncated_claim_count -> Int8,
        future_accepted_count -> Int8,
        past_verified_count -> Int8,
        max_claim_per_post_applied -> Int8,
        claim_manifest_hash -> Nullable<Text>,
        veracity_manifest_hash -> Nullable<Text>,
        finalize_tx_digest -> Nullable<Text>,
        checkpoint -> Nullable<Int8>,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    spot_claim_verdicts (id) {
        id -> Int4,
        post_id -> Text,
        claim_index -> Int8,
        time_class -> Text,
        verdict -> Int2,
        semantic_claim_hash -> Nullable<Text>,
        policy_hash -> Text,
        evidence_manifest_hash -> Text,
        related_market_object_id -> Nullable<Text>,
        related_claim_object_id -> Nullable<Text>,
        evidence_urls -> Jsonb,
        summary -> Nullable<Text>,
        transaction_id -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    spot_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        truth_enabled -> Bool,
        confidence_threshold_bps -> Int8,
        resolution_window_ms -> Int8,
        max_resolution_window_ms -> Int8,
        oracle_address -> Text,
        max_single_bet -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        payout_delay_ms -> Int8,
        version -> Int8,
        spot_governance_registry_id -> Nullable<Text>,
        min_betting_options -> Int8,
        max_betting_options -> Int8,
        min_reasoning_length -> Int8,
        max_reasoning_length -> Int8,
        max_evidence_urls -> Int8,
        platform_fee_bps -> Int8,
        ecosystem_fee_bps -> Int8,
        creator_fee_bps -> Nullable<Int8>,
        creator_claim_window_ms -> Nullable<Int8>,
        expired_creator_ecosystem_bps -> Nullable<Int8>,
        max_bets_per_record -> Int8,
        max_claim_per_post -> Int8,
    }
}

diesel::table! {
    spot_events (id) {
        id -> Int4,
        event_type -> Text,
        post_id -> Text,
        event_data -> Jsonb,
        event_id -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    spot_payouts (id, time) {
        id -> Int4,
        post_id -> Text,
        user_address -> Text,
        amount -> Int8,
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    spot_records (id) {
        id -> Int4,
        post_id -> Text,
        status -> Int2,
        outcome -> Nullable<Int2>,
        amm_split_bps_used -> Int4,
        created_at_ms -> Int8,
        last_resolution_at_ms -> Nullable<Int8>,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        transaction_id -> Text,
        betting_options -> Jsonb,
        option_escrow -> Jsonb,
        resolution_window_ms -> Nullable<Int8>,
        max_resolution_window_ms -> Nullable<Int8>,
        resolution_at_ms -> Nullable<Int8>,
        record_object_id -> Nullable<Text>,
        active_proposal_id -> Nullable<Text>,
        oracle_proposed_outcome -> Nullable<Int2>,
        proposed_outcome -> Nullable<Int2>,
        dao_escalated_at_ms -> Nullable<Int8>,
        claim_object_id -> Nullable<Text>,
        market_object_id -> Nullable<Text>,
        primary_post_id -> Nullable<Text>,
        market_key_hash -> Nullable<Text>,
        creator_fee_total -> Nullable<Int8>,
    }
}

diesel::table! {
    spot_refunds (id, time) {
        id -> Int4,
        post_id -> Text,
        user_address -> Text,
        amount -> Int8,
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    spot_resolutions (id, time) {
        id -> Int4,
        post_id -> Text,
        outcome -> Int2,
        total_escrow -> Int8,
        fee_taken -> Int8,
        resolved_at_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        reasoning -> Text,
        evidence_urls -> Jsonb,
        claim_object_id -> Nullable<Text>,
        market_object_id -> Nullable<Text>,
        creator_fee_total -> Nullable<Int8>,
    }
}

diesel::table! {
    spt_holdings (id, time) {
        id -> Int4,
        pool_id -> Text,
        holder_address -> Text,
        amount -> Int8,
        acquired_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    spt_pools (id, time) {
        id -> Int4,
        pool_id -> Text,
        token_type -> Int2,
        owner -> Text,
        associated_id -> Text,
        circulating_supply -> Int8,
        base_price -> Int8,
        quadratic_coefficient -> Int8,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    spt_price_history (id, time) {
        id -> Int4,
        pool_id -> Text,
        price -> Int8,
        circulating_supply -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    spt_reservation_pools (id, time) {
        id -> Int4,
        pool_id -> Text,
        associated_id -> Text,
        token_type -> Int2,
        owner -> Text,
        total_reserved -> Int8,
        required_threshold -> Int8,
        status -> Text,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    spt_reservations (id, time) {
        id -> Int4,
        pool_id -> Text,
        reserver_address -> Text,
        amount -> Int8,
        reserved_at -> Int8,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        treasury_fee -> Nullable<Int8>,
        platform_fee -> Nullable<Int8>,
        creator_fee -> Nullable<Int8>,
        fee_amount -> Nullable<Int8>,
        organization_id -> Nullable<Text>,
    }
}

diesel::table! {
    spt_revenue (pool_id, time) {
        pool_id -> Text,
        transaction_type -> Text,
        trader -> Text,
        creator_address -> Text,
        platform_address -> Text,
        treasury_address -> Text,
        creator_fee -> Int8,
        platform_fee -> Int8,
        treasury_fee -> Int8,
        total_fee -> Int8,
        token_amount -> Int8,
        myso_amount -> Int8,
        token_price -> Int8,
        revenue_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    spt_swaps (id, time) {
        id -> Int8,
        transaction_id -> Text,
        trader -> Text,
        source_pool_id -> Text,
        dest_pool_id -> Text,
        sell_amount -> Int8,
        dest_amount -> Int8,
        sell_myso_gross -> Int8,
        buy_myso_gross -> Int8,
        sell_fee_amount -> Int8,
        buy_fee_amount -> Int8,
        sell_creator_fee -> Int8,
        sell_platform_fee -> Int8,
        sell_treasury_fee -> Int8,
        buy_creator_fee -> Int8,
        buy_platform_fee -> Int8,
        buy_treasury_fee -> Int8,
        leftover_myso -> Int8,
        source_new_price -> Int8,
        dest_new_price -> Int8,
        organization_id -> Nullable<Text>,
        created_at -> Int8,
        time -> Timestamptz,
    }
}

diesel::table! {
    spt_transfers (id, time) {
        id -> Int8,
        transaction_id -> Text,
        pool_id -> Text,
        from_address -> Text,
        to_address -> Text,
        amount -> Int8,
        organization_id -> Nullable<Text>,
        created_at -> Int8,
        time -> Timestamptz,
    }
}

diesel::table! {
    spt_transactions (id, time) {
        id -> Int4,
        pool_id -> Text,
        transaction_type -> Text,
        sender -> Text,
        amount -> Int8,
        myso_amount -> Int8,
        fee_amount -> Int8,
        creator_fee -> Int8,
        platform_fee -> Int8,
        treasury_fee -> Int8,
        price -> Int8,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        organization_id -> Nullable<Text>,
        counterparty_pool_id -> Nullable<Text>,
        is_swap_leg -> Bool,
    }
}

diesel::table! {
    subscription_access_logs (subscription_id, time) {
        subscription_id -> Text,
        subscriber -> Text,
        content_type -> Text,
        content_id -> Text,
        access_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        processing_success -> Bool,
        processing_error -> Nullable<Text>,
    }
}

diesel::table! {
    subscription_events (event_type, time) {
        event_type -> Text,
        subscription_id -> Nullable<Text>,
        service_id -> Nullable<Text>,
        subscriber -> Nullable<Text>,
        event_data -> Jsonb,
        event_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        processing_success -> Bool,
        processing_error -> Nullable<Text>,
    }
}

diesel::table! {
    subscription_revenue (service_id, time) {
        service_id -> Text,
        subscription_id -> Nullable<Text>,
        from_address -> Text,
        to_address -> Text,
        amount -> Int8,
        platform_fee -> Int8,
        ecosystem_fee -> Int8,
        creator_amount -> Int8,
        platform_address -> Nullable<Text>,
        revenue_type -> Text,
        payment_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        processing_success -> Bool,
        processing_error -> Nullable<Text>,
    }
}

diesel::table! {
    tips (id, time) {
        id -> Int4,
        tipper -> Text,
        recipient -> Text,
        object_id -> Text,
        amount -> Int8,
        is_post -> Bool,
        coin_type -> Text,
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        organization_id -> Nullable<Text>,
    }
}

diesel::table! {
    unified_revenue (revenue_source, time) {
        revenue_source -> Text,
        revenue_type -> Text,
        creator_address -> Text,
        platform_address -> Nullable<Text>,
        amount -> Int8,
        currency -> Text,
        content_id -> Nullable<Text>,
        content_type -> Nullable<Text>,
        payer_address -> Text,
        recipient_address -> Text,
        revenue_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        organization_id -> Nullable<Text>,
    }
}

diesel::table! {
    upgrade_events (id) {
        id -> Int4,
        #[max_length = 66]
        package_id -> Varchar,
        version -> Int8,
        #[max_length = 128]
        event_id -> Varchar,
        #[max_length = 128]
        transaction_id -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    vesting_events (id, time) {
        id -> Int4,
        wallet_id -> Text,
        event_type -> Text,
        owner_address -> Text,
        amount -> Int8,
        remaining_balance -> Nullable<Int8>,
        start_time -> Nullable<Int8>,
        schedule_end -> Nullable<Int8>,
        pieces -> Nullable<Jsonb>,
        event_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    vesting_wallets (wallet_id) {
        wallet_id -> Text,
        owner_address -> Text,
        total_amount -> Int8,
        start_time -> Int8,
        schedule_end -> Int8,
        pieces -> Jsonb,
        claimed_amount -> Int8,
        remaining_balance -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        transaction_id -> Text,
    }
}

diesel::table! {
    vote_decryption_failures (id, time) {
        id -> Int4,
        proposal_id -> Text,
        voter_address -> Text,
        failure_reason -> Text,
        attempted_at -> Int8,
        encrypted_vote_length -> Nullable<Int4>,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    wallet_social_graph (wallet_address) {
        wallet_address -> Varchar,
        followers_count -> Int4,
        following_count -> Int4,
        blocked_count -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    watermarks (pipeline) {
        pipeline -> Text,
        epoch_hi_inclusive -> Int8,
        checkpoint_hi_inclusive -> Int8,
        tx_hi -> Int8,
        timestamp_ms_hi_inclusive -> Int8,
        reader_lo -> Int8,
        pruner_timestamp -> Timestamp,
        pruner_hi -> Int8,
    }
}

diesel::table! {
    insurance_router_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        paused -> Bool,
        max_route_reserve_market -> Int8,
        max_route_reserve_user -> Int8,
        max_route_reserve_option -> Int8,
        max_vault_concentration_bps -> Int8,
        min_vault_health_factor_bps -> Int8,
        max_route_legs -> Int8,
        version -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    messaging_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        paid_msg_platform_fee_bps -> Int8,
        paid_msg_treasury_fee_bps -> Int8,
        payment_expiration_ms -> Int8,
        min_reply_chars -> Int8,
        max_dedupe_key_bytes -> Int8,
        version -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    paid_message_escrows (id, time) {
        id -> Int4,
        group_id -> Text,
        seq -> Int8,
        payer -> Text,
        recipient -> Text,
        amount -> Int8,
        status -> Text,
        platform_fee -> Nullable<Int8>,
        treasury_fee -> Nullable<Int8>,
        net_amount -> Nullable<Int8>,
        platform_fee_recipient -> Nullable<Text>,
        ecosystem_fee_recipient -> Nullable<Text>,
        reply_char_count -> Nullable<Int8>,
        created_at_ms -> Int8,
        claimed_at_ms -> Nullable<Int8>,
        refunded_at_ms -> Nullable<Int8>,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    message_digests (id, time) {
        id -> Int4,
        group_id -> Text,
        seq -> Int8,
        sender -> Text,
        recipient -> Text,
        content_digest -> Text,
        content_uri -> Text,
        created_at_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    messaging_agent_groups (id, time) {
        id -> Int4,
        group_id -> Text,
        creator_actor -> Text,
        creator_principal -> Text,
        creator_sub_agent_id -> Nullable<Text>,
        creator_identity_class -> Int8,
        organization_id -> Nullable<Text>,
        group_name -> Text,
        group_uuid -> Text,
        created_at_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    memory_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        max_organizations_per_user -> Int2,
        org_category_update_cooldown_ms -> Int8,
        max_agent_depth -> Int2,
        max_label_length -> Int8,
        max_org_name_length -> Int8,
        max_org_description_length -> Int8,
        version -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    platform_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        max_reasoning_length -> Int8,
        max_cover_photo_url_length -> Int8,
        max_media_previews -> Int8,
        max_badge_name_length -> Int8,
        max_badge_description_length -> Int8,
        version -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    profile_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        max_vesting_pieces -> Int8,
        curve_factor_min -> Int8,
        curve_factor_max -> Int8,
        curve_precision -> Int8,
        min_claim_threshold_divisor -> Int8,
        min_username_length -> Int8,
        max_username_length -> Int8,
        username_sale_fee_bps -> Int8,
        version -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    subscription_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        default_billing_period_ms -> Int8,
        max_renewal_months -> Int8,
        platform_fee_bps -> Int8,
        ecosystem_fee_bps -> Int8,
        non_platform_platform_to_creator_bps -> Int8,
        non_platform_platform_to_treasury_bps -> Int8,
        version -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::joinable!(profile_subscription_plans -> profile_subscription_services (service_id));
diesel::joinable!(profile_subscriptions -> profile_subscription_services (service_id));
diesel::joinable!(ai_credit_agent_budgets -> ai_credit_balances (balance_id));
diesel::joinable!(ai_credit_balances -> memory_accounts (memory_account_id));
diesel::joinable!(ai_spend_reservations -> ai_credit_balances (balance_id));
diesel::joinable!(profiles -> ai_credit_balances (ai_credit_balance_id));
diesel::joinable!(sub_agents -> memory_accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    ai_credit_agent_budgets,
    ai_credit_balances,
    ai_credit_config,
    ai_credit_events,
    ai_credit_spend_approvals,
    ai_credit_usage_lines,
    ai_spend_reservations,
    anonymous_votes,
    audit_log,
    memory_usage_stats,
    org_invitations,
    org_memory_permissions,
    org_role_assignments,
    org_roles,
    blocked_events,
    blocked_profiles,
    checkpoint_processing,
    comments,
    community_votes,
    continuous_aggregate_refresh_status,
    delegate_ratings,
    delegate_votes,
    delegates,
    ecosystem_treasury,
    governance_events,
    governance_registries,
    insurance_config,
    insurance_coverage_routes,
    insurance_events,
    insurance_market_exposures,
    insurance_policies,
    insurance_policy_events,
    insurance_route_fills,
    insurance_user_exposures,
    insurance_vault_transactions,
    insurance_vaults,
    sub_agent_memory_vaults,
    sub_agent_organizations,
    memory_accounts,
    sub_agent_organization_counterparties,
    sub_agent_organization_events,
    sub_agent_organization_stats,
    sub_agent_organization_stats_daily,
    mydata_access_logs,
    mydata_config,
    mydata_data,
    mydata_purchases,
    mydata_broad_pools,
    mydata_claims,
    mydata_distribution_rounds,
    mydata_listing_sub_pools,
    mydata_merkle_roots,
    mydata_snapshot_anchors,
    mydata_snapshot_escrow,
    mydata_sub_pools,
    mydata_registry,
    mydata_revenue,
    mydata_subscriptions,
    nominated_delegates,
    object_migrated_events,
    platform_blocked_profiles,
    platform_events,
    platform_memberships,
    platform_moderator_permissions,
    platform_moderators,
    platform_treasury_balances,
    platform_treasury_withdrawals,
    platforms,
    poc_analysis_results,
    poc_badges,
    poc_beneficiary_vaults,
    poc_config,
    poc_creator_identity_links,
    poc_dispute_votes,
    poc_disputes,
    poc_revenue_redirections,
    poc_vault_claims,
    poc_vault_coin_balances,
    poc_vault_deposits,
    poc_username_beneficiaries,
    poc_username_beneficiary_events,
    post_config,
    posts,
    posts_deletion_events,
    posts_moderation_events,
    posts_reports,
    posts_transfers,
    profile_badges,
    profile_events,
    username_listings,
    username_offers,
    username_sale_fees,
    profile_subscription_plans,
    profile_subscription_services,
    profile_subscriptions,
    profiles,
    progress_store,
    promoted_posts,
    promotion_budget_events,
    promotion_status_events,
    promotion_views,
    proposals,
    reaction_counts,
    reactions,
    reposts,
    reward_distributions,
    social_graph_events,
    social_graph_relationships,
    social_proof_of_truth,
    spot_bet_withdrawals,
    spot_bets,
    spot_claims,
    spot_config,
    spot_creator_earnings_daily,
    spot_creator_payouts,
    spot_events,
    spot_markets,
    spot_payouts,
    spot_post_analyses,
    spot_post_links,
    spot_claim_verdicts,
    spot_records,
    spot_refunds,
    spot_resolutions,
    sub_agent_events,
    sub_agents,
    spt_config,
    spt_events,
    spt_holdings,
    spt_pools,
    spt_price_history,
    spt_reservation_pools,
    spt_reservations,
    spt_revenue,
    spt_swaps,
    spt_transfers,
    spt_transactions,
    subscription_access_logs,
    subscription_events,
    subscription_revenue,
    tips,
    unified_revenue,
    upgrade_events,
    username_registry,
    username_reservations,
    vesting_events,
    vesting_wallets,
    vote_decryption_failures,
    wallet_messaging_policies,
    wallet_social_graph,
    watermarks,
    insurance_router_config,
    messaging_config,
    paid_message_escrows,
    message_digests,
    messaging_agent_groups,
    memory_config,
    media_assets,
    media_asset_governance_links,
    media_asset_rights_updates,
    fingerprint_observations,
    media_asset_usages,
    composition_analysis_records,
    revenue_manifests,
    media_asset_derivative_edges,
    media_asset_ancestry_snapshots,
    license_template_versions,
    license_instances,
    media_asset_resolved_policies,
    media_asset_resolved_obligations,
    post_usage_decision_events,
    detected_asset_relationships,
    platform_config,
    profile_config,
    subscription_config,
);
