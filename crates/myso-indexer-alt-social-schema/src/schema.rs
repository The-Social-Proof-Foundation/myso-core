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
        transaction_id -> Text,
        time -> Timestamptz,
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
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Varchar,
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
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        enable_flag -> Bool,
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
        start_time_ms -> Int8,
        expiry_time_ms -> Int8,
        vault_id -> Text,
        status -> Int2,
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
        enable_flag -> Bool,
        max_tags -> Int8,
        max_subscription_days -> Int8,
        max_free_access_grants -> Int8,
        timestamp_ms -> Int8,
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
        subscription_duration_days -> Int8,
        geographic_region -> Nullable<Text>,
        data_quality -> Nullable<Text>,
        sample_size -> Nullable<Int8>,
        collection_method -> Nullable<Text>,
        is_updating -> Bool,
        update_frequency -> Nullable<Text>,
        version -> Int8,
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
        purchase_type -> Text,
        purchase_time -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
    }
}

diesel::table! {
    mydata_registry (ip_id) {
        ip_id -> Text,
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
    }
}

diesel::table! {
    mydata_query_broad_pools (pool_id) {
        pool_id -> Text,
        name -> Text,
        created_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    mydata_query_sub_pools (sub_pool_id) {
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
    mydata_query_listing_sub_pools (listing_id, sub_pool_id) {
        listing_id -> Text,
        sub_pool_id -> Text,
        assigned_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    mydata_query_merkle_roots (snapshot_id) {
        snapshot_id -> Text,
        root_hash -> Text,
        published_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    mydata_query_snapshot_anchors (id, time) {
        id -> Int4,
        snapshot_id -> Text,
        buyer_address -> Text,
        price_paid -> Int8,
        created_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
        manifest_hash -> Nullable<Text>,
        payment_reference -> Nullable<Text>,
    }
}

diesel::table! {
    mydata_query_distribution_rounds (snapshot_id) {
        snapshot_id -> Text,
        total_amount -> Int8,
        contributor_count -> Int8,
        merkle_root -> Text,
        published_at_ms -> Int8,
        event_id -> Text,
        transaction_id -> Text,
        time -> Timestamptz,
    }
}

diesel::table! {
    mydata_query_claims (id, time) {
        id -> Int4,
        snapshot_id -> Text,
        claimant -> Text,
        amount -> Int8,
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
    platform_token_airdrops (id) {
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
        developer_address -> Text,
        terms_of_service -> Nullable<Text>,
        privacy_policy -> Nullable<Text>,
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
        treasury -> Nullable<Int8>,
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
    }
}

diesel::table! {
    poc_configuration (id) {
        id -> Int4,
        image_threshold -> Int8,
        video_threshold -> Int8,
        audio_threshold -> Int8,
        revenue_redirect_percentage -> Int8,
        dispute_cost -> Int8,
        dispute_protocol_fee -> Int8,
        min_vote_stake -> Int8,
        max_vote_stake -> Int8,
        voting_duration_epochs -> Int8,
        updated_by -> Text,
        updated_at -> Int8,
        transaction_id -> Text,
        time -> Timestamptz,
        max_reasoning_length -> Int8,
        max_evidence_urls -> Int8,
        max_votes_per_dispute -> Int8,
        oracle_address -> Nullable<Text>,
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
        voting_start_epoch -> Int8,
        voting_end_epoch -> Int8,
        resolution -> Nullable<Int2>,
        winning_side -> Nullable<Int2>,
        total_winning_stake -> Nullable<Int8>,
        total_losing_stake -> Nullable<Int8>,
        submitted_at -> Int8,
        resolved_at -> Nullable<Int8>,
        transaction_id -> Text,
        time -> Timestamptz,
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
        encrypted_content_hash -> Nullable<Text>,
        enable_spt -> Bool,
        enable_poc -> Bool,
        enable_spot -> Bool,
        spot_id -> Nullable<Text>,
        spt_id -> Nullable<Text>,
        poc_reasoning -> Nullable<Text>,
        poc_evidence_urls -> Nullable<Jsonb>,
        poc_similarity_score -> Nullable<Int8>,
        poc_media_type -> Nullable<Int2>,
        poc_oracle_address -> Nullable<Text>,
        poc_analyzed_at -> Nullable<Int8>,
        mydata_id -> Nullable<Text>,
        revenue_recipient -> Nullable<Text>,
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
    profile_offers (id, time) {
        id -> Int4,
        profile_id -> Text,
        offeror_address -> Text,
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
    profile_sale_fees (id, time) {
        id -> Int4,
        profile_id -> Text,
        offeror_address -> Text,
        previous_owner_address -> Text,
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
        monthly_fee -> Int8,
        active -> Bool,
        subscriber_count -> Int8,
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
        min_offer_amount -> Nullable<Int8>,
        cover_photo -> Nullable<Text>,
        profile_id -> Nullable<Text>,
        sensitive_data_updated_at -> Nullable<Timestamp>,
        birthdate -> Nullable<Text>,
        current_location -> Nullable<Text>,
        raised_location -> Nullable<Text>,
        phone -> Nullable<Text>,
        email -> Nullable<Text>,
        gender -> Nullable<Text>,
        political_view -> Nullable<Text>,
        religion -> Nullable<Text>,
        education -> Nullable<Text>,
        primary_language -> Nullable<Text>,
        relationship_status -> Nullable<Text>,
        x_username -> Nullable<Text>,
        followers_count -> Int4,
        following_count -> Int4,
        subscription_service_id -> Nullable<Text>,
        subscription_enabled -> Nullable<Bool>,
        blocked_count -> Int4,
        social_proof_token_address -> Nullable<Varchar>,
        selected_badge_id -> Nullable<Varchar>,
        reservation_pool_address -> Nullable<Varchar>,
        paid_messaging_enabled -> Bool,
        paid_messaging_min_cost -> Nullable<Int8>,
        selected_ecosystem_badge_id -> Nullable<Varchar>,
        search_text -> Nullable<Text>,
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
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
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
    spt_config (id) {
        id -> Int4,
        trading_enabled -> Bool,
        admin_address -> Text,
        reason -> Text,
        timestamp_ms -> Int8,
        updated_at -> Timestamptz,
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
        timestamp_epoch -> Int8,
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
        timestamp_epoch -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        option_id -> Int2,
    }
}

diesel::table! {
    spot_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        enable_flag -> Bool,
        confidence_threshold_bps -> Int8,
        resolution_window_epochs -> Int8,
        max_resolution_window_epochs -> Int8,
        fee_bps -> Int8,
        fee_split_bps_platform -> Int8,
        oracle_address -> Text,
        max_single_bet -> Int8,
        timestamp_ms -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        payout_delay_ms -> Int8,
        version -> Int8,
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
        timestamp_epoch -> Int8,
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
        created_epoch -> Int8,
        last_resolution_epoch -> Nullable<Int8>,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        transaction_id -> Text,
        betting_options -> Jsonb,
        option_escrow -> Jsonb,
        resolution_window_epochs -> Nullable<Int8>,
        max_resolution_window_epochs -> Nullable<Int8>,
    }
}

diesel::table! {
    spot_refunds (id, time) {
        id -> Int4,
        post_id -> Text,
        user_address -> Text,
        amount -> Int8,
        timestamp_epoch -> Int8,
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
        resolved_epoch -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        reasoning -> Text,
        evidence_urls -> Jsonb,
    }
}

diesel::table! {
    spt_exchange_config (id, time) {
        id -> Int4,
        updated_by -> Text,
        post_threshold -> Int8,
        profile_threshold -> Int8,
        max_individual_reservation_bps -> Int8,
        total_fee_bps -> Int8,
        creator_fee_bps -> Int8,
        platform_fee_bps -> Int8,
        treasury_fee_bps -> Int8,
        base_price -> Int8,
        quadratic_coefficient -> Int8,
        max_hold_percent_bps -> Int8,
        updated_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
        trading_enabled -> Bool,
        trading_creator_fee_bps -> Int8,
        trading_platform_fee_bps -> Int8,
        trading_treasury_fee_bps -> Int8,
        reservation_creator_fee_bps -> Int8,
        reservation_platform_fee_bps -> Int8,
        reservation_treasury_fee_bps -> Int8,
        max_reservers_per_pool -> Int8,
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
    }
}

diesel::table! {
    subscription_access_logs (subscription_id, time) {
        subscription_id -> Text,
        subscriber -> Text,
        content_type -> Text,
        content_id -> Text,
        access_time -> Int8,
        seal_id -> Nullable<Text>,
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
        created_at -> Int8,
        time -> Timestamptz,
        transaction_id -> Text,
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
        duration -> Nullable<Int8>,
        curve_factor -> Nullable<Int8>,
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
        duration -> Int8,
        curve_factor -> Int8,
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

diesel::joinable!(profile_subscriptions -> profile_subscription_services (service_id));

diesel::allow_tables_to_appear_in_same_query!(
    anonymous_votes,
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
    insurance_events,
    insurance_market_exposures,
    insurance_policies,
    insurance_policy_events,
    insurance_user_exposures,
    insurance_vault_transactions,
    insurance_vaults,
    mydata_access_logs,
    mydata_config,
    mydata_data,
    mydata_purchases,
    mydata_query_broad_pools,
    mydata_query_claims,
    mydata_query_distribution_rounds,
    mydata_query_listing_sub_pools,
    mydata_query_merkle_roots,
    mydata_query_snapshot_anchors,
    mydata_query_sub_pools,
    mydata_registry,
    mydata_revenue,
    mydata_subscriptions,
    nominated_delegates,
    object_migrated_events,
    platform_blocked_profiles,
    platform_events,
    platform_memberships,
    platform_moderators,
    platform_token_airdrops,
    platforms,
    poc_analysis_results,
    poc_badges,
    poc_configuration,
    poc_dispute_votes,
    poc_disputes,
    poc_revenue_redirections,
    post_config,
    posts,
    posts_deletion_events,
    posts_moderation_events,
    posts_reports,
    posts_transfers,
    profile_badges,
    profile_events,
    profile_offers,
    profile_sale_fees,
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
    spot_config,
    spot_events,
    spot_payouts,
    spot_records,
    spot_refunds,
    spot_resolutions,
    spt_config,
    spt_events,
    spt_exchange_config,
    spt_holdings,
    spt_pools,
    spt_price_history,
    spt_reservation_pools,
    spt_reservations,
    spt_revenue,
    spt_transactions,
    subscription_access_logs,
    subscription_events,
    subscription_revenue,
    tips,
    unified_revenue,
    upgrade_events,
    vesting_events,
    vesting_wallets,
    vote_decryption_failures,
    wallet_social_graph,
    watermarks,
);
