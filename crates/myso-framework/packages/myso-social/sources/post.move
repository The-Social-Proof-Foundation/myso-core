// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

/// Post module for the MySocial network
/// Handles creation and management of posts and comments
/// Implements features like comments, reposts, and quotes

#[allow(duplicate_alias, unused_use, unused_const, unused_variable, lint(public_entry))]
module social_contracts::post {
    use std::string::{Self, String};
    use std::option::{Self, Option};
    use std::type_name::{Self as type_name, TypeName};
    
    use myso::{
        object::{Self, UID, ID},
        tx_context::{Self, TxContext},
        transfer,
        event,
        table::{Self, Table},
        coin::{Self, Coin},
        balance::{Self, Balance},
        url::{Self, Url},
        clock::{Self, Clock},
        dynamic_field as df,
        permissioned_group::PermissionedGroup,
    };
    use myso::myso::MYSO;
    use social_contracts::subscription::{Self, ProfileSubscriptionService, ProfileSubscription};
    use social_contracts::profile::UsernameRegistry;
    use social_contracts::platform;
    use social_contracts::block_list::{Self, BlockListRegistry};
    use social_contracts::mydata;
    use social_contracts::upgrade::{Self, UpgradeAdminCap};
    use social_contracts::profile::{Self, EcosystemTreasury};
    use social_contracts::poc_vault::{Self as poc_vault, PoCBeneficiaryVault};
    use social_contracts::media_asset::{Self as media_asset, CompositionAnalysis, CompositionBadgeSnapshot, RevenueManifest, ManifestEntry, MediaAsset};
    use social_contracts::memory::{Self, MemoryAccount, MemoryConfig, ActingContext};

    /// Error codes
    const EUnauthorized: u64 = 0;
    const EPostNotFound: u64 = 1;
    const EInvalidTipAmount: u64 = 2;
    const ESelfTipping: u64 = 3;
    const EInvalidParentReference: u64 = 4;
    const EContentTooLarge: u64 = 5;
    const ETooManyMediaUrls: u64 = 6;
    const EInvalidPostType: u64 = 7;
    const EUnauthorizedTransfer: u64 = 8;
    const EReportReasonInvalid: u64 = 9;
    const EReportDescriptionTooLong: u64 = 10;
    const EReactionContentTooLong: u64 = 11;
    const EUserNotJoinedPlatform: u64 = 12;
    const EUserBlockedByPlatform: u64 = 13;
    const EWrongVersion: u64 = 14;
    const EReactionsNotAllowed: u64 = 15;
    const ECommentsNotAllowed: u64 = 16;
    const ERepostsNotAllowed: u64 = 17;
    const EQuotesNotAllowed: u64 = 18;
    const ETipsNotAllowed: u64 = 19;
    const EInvalidConfig: u64 = 20;
    const ENoSubscriptionService: u64 = 21;
    const ENoEncryptedContent: u64 = 22;
    const EPriceMismatch: u64 = 23;
    const EPromotionAmountTooLow: u64 = 24;
    const EPromotionAmountTooHigh: u64 = 25;
    const ENotPromotedPost: u64 = 26;
    const EUserAlreadyViewed: u64 = 27;
    const EInsufficientPromotionFunds: u64 = 28;
    const EPromotionInactive: u64 = 29;
    const EInvalidViewDuration: u64 = 30;
    const EOverflow: u64 = 31;
    const EMyDataNotRegistered: u64 = 32;
    const EMyDataOwnerMismatch: u64 = 33;
    const EDisputeCapReached: u64 = 36;
    const EWrongBeneficiaryVault: u64 = 37;
    const ETipPostRequiresBeneficiaryVault: u64 = 38;
    const EInvalidBatch: u64 = 39;
    const ESptRequiresDedicatedCreate: u64 = 40;
    const EInvalidMediaAsset: u64 = 41;
    const EMonetizationNotEnabled: u64 = 42;
    const EInvalidManifest: u64 = 43;
    const ECompositionNotVerified: u64 = 44;
    const EBindingNotFound: u64 = 95;
    const EInvalidDenialScope: u64 = 96;
    const EBindingAssetMismatch: u64 = 97;
    const EInvalidBinding: u64 = 98;

    /// Constants for size limits
    const MAX_CONTENT_LENGTH: u64 = 5000; // 5000 chars max for content
    const MAX_MEDIA_URLS: u64 = 10; // Max 10 media URLs per post
    const MAX_MENTIONS: u64 = 10; // Max 50 mentions per post
    const MAX_METADATA_SIZE: u64 = 10000; // 10KB max for metadata
    const MAX_DESCRIPTION_LENGTH: u64 = 500; // 500 chars max for report description
    const MAX_REACTION_LENGTH: u64 = 20; // 50 chars max for a reaction
    const COMMENTER_TIP_PERCENTAGE: u64 = 80; // 80% of tip goes to commenter, 20% to post owner
    const REPOST_TIP_PERCENTAGE: u64 = 50; // 50% of tip goes to repost owner, 50% to original post owner
    const MAX_U64: u64 = 18446744073709551615; // Max u64 value for overflow protection
    
    /// Constants for promoted posts
    const MIN_PROMOTION_AMOUNT: u64 = 1000; // Minimum 0.001 MYSO (1000 MIST) per view
    const MAX_PROMOTION_AMOUNT: u64 = 100000000; // Maximum 100 MYSO per view
    const MIN_VIEW_DURATION: u64 = 3000; // Minimum 3 seconds view time in milliseconds
    /// Max promotions confirmed in one `confirm_promoted_post_views` call (gas / object-lock bound)
    const MAX_PROMOTION_VIEW_BATCH: u64 = 50;
    const BPS_DENOM: u64 = 10000; 
    const DEFAULT_PLATFORM_FEE_BPS: u64 = 1000; // Default platform fee on each confirmed promo view gross (10%)
    const DEFAULT_ECOSYSTEM_FEE_BPS: u64 = 1000; // Default ecosystem fee on each confirmed promo view gross (10%)

    /// Valid post types
    const POST_TYPE_STANDARD: vector<u8> = b"standard";
    const POST_TYPE_REPOST: vector<u8> = b"repost";
    const POST_TYPE_QUOTE_REPOST: vector<u8> = b"quote_repost";

    /// Constants for report reason codes
    const REPORT_REASON_SPAM: u8 = 1;
    const REPORT_REASON_OFFENSIVE: u8 = 2;
    const REPORT_REASON_MISINFORMATION: u8 = 3;
    const REPORT_REASON_ILLEGAL: u8 = 4;
    const REPORT_REASON_IMPERSONATION: u8 = 5;
    const REPORT_REASON_HARASSMENT: u8 = 6;
    const REPORT_REASON_OTHER: u8 = 99;

    /// Constants for moderation states
    const MODERATION_APPROVED: u8 = 1;
    const MODERATION_FLAGGED: u8 = 2;

    /// Bitfield constants for permission flags (allow_*)
    const PERMISSION_ALLOW_COMMENTS: u8 = 1;      // bit 0
    const PERMISSION_ALLOW_REACTIONS: u8 = 2;      // bit 1
    const PERMISSION_ALLOW_REPOSTS: u8 = 4;        // bit 2
    const PERMISSION_ALLOW_QUOTES: u8 = 8;          // bit 3
    const PERMISSION_ALLOW_TIPS: u8 = 16;          // bit 4

    /// Event/indexer tags for [`PostAccess`] (1=public, 2=profile_sub, 3=marketplace_one_time).
    const POST_ACCESS_PUBLIC: u8 = 1;
    const POST_ACCESS_PROFILE_SUBSCRIPTION: u8 = 2;
    const POST_ACCESS_MARKETPLACE_ONE_TIME: u8 = 3;

    /// Post content access model (replaces legacy `mydata_id` + subscription gate dynamic field).
    public enum PostAccess has store, copy, drop {
        Public,
        ProfileSubscription { service_id: ID, mydata_id: Option<ID>, min_tier_level: Option<u64> },
        MarketplaceOneTime { mydata_id: ID },
    }

    /// Post object that contains content information
    public struct Post has key {
        id: UID,
        owner: address,
        profile_id: address,
        platform_id: address,
        content: String,
        /// Canonical composition: ordered MediaAsset IDs (max 10).
        media_asset_ids: vector<ID>,
        /// Display/cache URLs — not authoritative for PoC.
        media: Option<vector<Url>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        post_type: String,
        parent_post_id: Option<address>,
        created_at: u64,
        reaction_count: u64,
        comment_count: u64,
        repost_count: u64,
        tips_received: u64,
        removed_from_platform: bool,
        user_reactions: Table<address, String>,
        reaction_counts: Table<String, u64>,
        permissions: u8,
        /// Composition resolution status (display/use).
        composition_status: u8,
        /// Revenue eligibility status (separate from display).
        monetization_status: u8,
        composition_badge: Option<CompositionBadgeSnapshot>,
        access: PostAccess,
        promotion_id: Option<address>,
        enable_spt: bool,
        spt_id: Option<address>,
        version: u64,
    }

    /// Published-action attribution stored as a dynamic field (Post is at the VM field-count limit).
    public struct PostAttribution has store, copy, drop {
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
    }

    public struct CommentAttribution has store, copy, drop {
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
    }

    const POST_ATTRIBUTION_DF_KEY: vector<u8> = b"post_attribution";
    const COMMENT_ATTRIBUTION_DF_KEY: vector<u8> = b"comment_attribution";
    const COMPOSITION_ANALYSIS_DF_KEY: vector<u8> = b"composition_analysis";
    const REVENUE_MANIFEST_DF_KEY: vector<u8> = b"revenue_manifest";
    const COMPOSITION_DISPUTES_DF_KEY: vector<u8> = b"composition_disputes_submitted";
    const SPOT_ANALYSIS_DF_KEY: vector<u8> = b"spot_analysis";
    const EMBEDDED_BINDINGS_DF_KEY: vector<u8> = b"embedded_bindings";
    const USAGE_DECISIONS_DF_KEY: vector<u8> = b"usage_decisions";
    const USAGE_DENIALS_DF_KEY: vector<u8> = b"usage_denials";
    const CANDIDATE_MANIFEST_DF_KEY: vector<u8> = b"candidate_revenue_manifest";

    /// Application enforcement — policy reason codes for usage decision snapshots.
    const REASON_ALLOWED: u8 = 0;
    const REASON_NO_GRANT: u8 = 1;
    const REASON_RESOLVED_POLICY: u8 = 2;
    const REASON_DENIAL: u8 = 3;

    const DENIAL_SCOPE_PLAYBACK: u8 = 1;

    /// Phase 4 — embedded asset binding on a post container.
    public struct EmbeddedAssetBinding has store, copy, drop {
        binding_id: u64,
        source_asset_id: ID,
        usage_class: u8,
        stem: u8,
        media_component: u8,
        evidence_commitment: Option<vector<u8>>,
    }

    public struct UsageDecisionSnapshot has store, copy, drop {
        binding_id: u64,
        policy_playback_permitted: bool,
        playback_permitted: bool,
        policy_reason_code: u8,
        policy_version_at_decision: u64,
    }

    public struct ContainerUsageDenial has store, copy, drop {
        binding_id: u64,
        denial_scope: u8,
    }

    public struct EmbeddedBindingRecordedEvent has copy, drop {
        post_id: address,
        bindings: vector<EmbeddedAssetBinding>,
        timestamp: u64,
    }

    public struct UsageDecisionRefreshedEvent has copy, drop {
        post_id: address,
        binding_id: u64,
        policy_playback_permitted: bool,
        playback_permitted: bool,
        policy_reason_code: u8,
        policy_version_at_decision: u64,
        timestamp: u64,
    }

    public struct ContainerUsageDeniedEvent has copy, drop {
        post_id: address,
        binding_id: u64,
        denial_scope: u8,
        timestamp: u64,
    }

    public struct ContainerUsageDenialLiftedEvent has copy, drop {
        post_id: address,
        binding_id: u64,
        timestamp: u64,
    }

    public struct CandidateManifestSubmittedEvent has copy, drop {
        post_id: address,
        manifest_version: u64,
        entries_json: vector<ManifestEntry>,
        timestamp: u64,
    }

    /// Multi-claim Social Proof of Truth analysis (Post is at the VM field-count limit).

    /// SPoT multi-claim analysis lifecycle status (interpreted by `social_proof_of_truth`).
    const SPOT_STATUS_PENDING: u8 = 0;
    const SPOT_STATUS_COMPLETED: u8 = 1;
    const SPOT_STATUS_COMPLETED_NO_ACTIONABLE: u8 = 2;

    /// Errors surfaced by SPoT analysis mutators.
    const ESpotAnalysisNotPending: u64 = 90;
    const ESpotAnalysisVectorMismatch: u64 = 91;
    const ESpotAnalysisIndexOrder: u64 = 92;
    const ESpotAnalysisOverCap: u64 = 93;
    const ESpotAnalysisDuplicateMarket: u64 = 94;

    /// Per-post multi-claim SPoT analysis, attached to `post.id` via `SPOT_ANALYSIS_DF_KEY`.
    /// Future-claim links are stored as aligned vectors (index/claim/market/policy); past
    /// verdict detail lives off-chain (indexer/oracle) and is committed here only as counts
    /// and manifest hashes.
    public struct PostSpotAnalysis has store, copy, drop {
        status: u8,
        detected_claim_count: u64,
        rejected_claim_count: u64,
        truncated_claim_count: u64,
        future_accepted_count: u64,
        past_verified_count: u64,
        max_claim_per_post_applied: u64,
        claim_indexes: vector<u64>,
        claim_ids: vector<address>,
        market_ids: vector<address>,
        policy_hashes: vector<vector<u8>>,
        claim_manifest_hash: Option<vector<u8>>,
        veracity_manifest_hash: Option<vector<u8>>,
    }

    /// Helper: check if a bit is set in a bitfield
    fun has_flag(value: u8, flag: u8): bool {
        (value & flag) == flag
    }

    /// Helper: set a bit in a bitfield
    #[allow(unused_function)]
    fun set_flag(value: &mut u8, flag: u8) {
        *value = *value | flag
    }

    /// Helper: clear a bit in a bitfield
    #[allow(unused_function)]
    fun clear_flag(value: &mut u8, flag: u8) {
        *value = *value & (255 - flag)
    }

    /// Bitfield for `Post.permissions` / `PostCreatedEvent.permissions` (matches `create_post_internal`).
    fun permissions_bitfield(
        allow_comments: bool,
        allow_reactions: bool,
        allow_reposts: bool,
        allow_quotes: bool,
        allow_tips: bool,
    ): u8 {
        let mut p: u8 = 0;
        if (allow_comments) { p = p | PERMISSION_ALLOW_COMMENTS };
        if (allow_reactions) { p = p | PERMISSION_ALLOW_REACTIONS };
        if (allow_reposts) { p = p | PERMISSION_ALLOW_REPOSTS };
        if (allow_quotes) { p = p | PERMISSION_ALLOW_QUOTES };
        if (allow_tips) { p = p | PERMISSION_ALLOW_TIPS };
        p
    }

    /// Single match site for [`PostAccess`] field extraction.
    fun post_access_fields(access: &PostAccess): (u8, Option<ID>, Option<ID>, Option<u64>) {
        match (access) {
            PostAccess::Public => (POST_ACCESS_PUBLIC, option::none(), option::none(), option::none()),
            PostAccess::ProfileSubscription { service_id, mydata_id, min_tier_level } => {
                (
                    POST_ACCESS_PROFILE_SUBSCRIPTION,
                    option::some(*service_id),
                    *mydata_id,
                    *min_tier_level,
                )
            },
            PostAccess::MarketplaceOneTime { mydata_id } => {
                (POST_ACCESS_MARKETPLACE_ONE_TIME, option::none(), option::some(*mydata_id), option::none())
            },
        }
    }

    public fun requires_profile_subscription(post: &Post): bool {
        let (kind, _, _, _) = post_access_fields(&post.access);
        kind == POST_ACCESS_PROFILE_SUBSCRIPTION
    }

    public fun requires_marketplace_purchase(post: &Post): bool {
        let (kind, _, _, _) = post_access_fields(&post.access);
        kind == POST_ACCESS_MARKETPLACE_ONE_TIME
    }

    public fun linked_mydata(post: &Post): Option<ID> {
        let (_, _, mydata_id, _) = post_access_fields(&post.access);
        mydata_id
    }

    public fun subscription_service(post: &Post): Option<ID> {
        let (_, service_id, _, _) = post_access_fields(&post.access);
        service_id
    }

    public fun subscription_min_tier_level(post: &Post): Option<u64> {
        let (_, _, _, min_tier_level) = post_access_fields(&post.access);
        min_tier_level
    }

    public fun post_access_kind(post: &Post): u8 {
        let (kind, _, _, _) = post_access_fields(&post.access);
        kind
    }

    public(package) fun post_access_from_parts(
        access_kind: u8,
        subscription_service_id: Option<ID>,
        linked_mydata_id: Option<ID>,
        subscription_min_tier_level: Option<u64>,
    ): PostAccess {
        if (access_kind == POST_ACCESS_PUBLIC) {
            PostAccess::Public
        } else if (access_kind == POST_ACCESS_PROFILE_SUBSCRIPTION) {
            assert!(option::is_some(&subscription_service_id), ENoSubscriptionService);
            PostAccess::ProfileSubscription {
                service_id: *option::borrow(&subscription_service_id),
                mydata_id: linked_mydata_id,
                min_tier_level: subscription_min_tier_level,
            }
        } else if (access_kind == POST_ACCESS_MARKETPLACE_ONE_TIME) {
            assert!(option::is_some(&linked_mydata_id), ENoEncryptedContent);
            PostAccess::MarketplaceOneTime {
                mydata_id: *option::borrow(&linked_mydata_id),
            }
        } else {
            abort EUnauthorized
        }
    }

    /// Query: check if comments are allowed
    public fun allow_comments(post: &Post): bool {
        has_flag(post.permissions, PERMISSION_ALLOW_COMMENTS)
    }

    /// Query: check if reactions are allowed
    public fun allow_reactions(post: &Post): bool {
        has_flag(post.permissions, PERMISSION_ALLOW_REACTIONS)
    }

    /// Query: check if reposts are allowed
    public fun allow_reposts(post: &Post): bool {
        has_flag(post.permissions, PERMISSION_ALLOW_REPOSTS)
    }

    /// Query: check if quotes are allowed
    public fun allow_quotes(post: &Post): bool {
        has_flag(post.permissions, PERMISSION_ALLOW_QUOTES)
    }

    /// Query: check if tips are allowed
    public fun allow_tips(post: &Post): bool {
        has_flag(post.permissions, PERMISSION_ALLOW_TIPS)
    }

    /// Query: check if SPT is enabled for this post
    public fun is_spt_enabled(post: &Post): bool {
        post.enable_spt
    }

    public fun composition_status(post: &Post): u8 {
        post.composition_status
    }

    public fun monetization_status(post: &Post): u8 {
        post.monetization_status
    }

    public fun media_asset_ids(post: &Post): &vector<ID> {
        &post.media_asset_ids
    }

    public fun composition_badge(post: &Post): &Option<CompositionBadgeSnapshot> {
        &post.composition_badge
    }

    public fun composition_analysis(post: &Post): Option<CompositionAnalysis> {
        if (df::exists_with_type<vector<u8>, CompositionAnalysis>(&post.id, COMPOSITION_ANALYSIS_DF_KEY)) {
            option::some(*df::borrow(&post.id, COMPOSITION_ANALYSIS_DF_KEY))
        } else {
            option::none()
        }
    }

    public fun revenue_manifest(post: &Post): Option<RevenueManifest> {
        if (df::exists_with_type<vector<u8>, RevenueManifest>(&post.id, REVENUE_MANIFEST_DF_KEY)) {
            option::some(*df::borrow(&post.id, REVENUE_MANIFEST_DF_KEY))
        } else {
            option::none()
        }
    }

    public fun composition_disputes_submitted(post: &Post): u8 {
        if (df::exists_with_type<vector<u8>, u8>(&post.id, COMPOSITION_DISPUTES_DF_KEY)) {
            *df::borrow(&post.id, COMPOSITION_DISPUTES_DF_KEY)
        } else {
            0
        }
    }

    public fun monetization_enabled(post: &Post): bool {
        post.monetization_status == media_asset::monetization_enabled()
    }

    /// Legacy alias used by dispute paths during migration.
    public fun poc_disputes_submitted(post: &Post): u8 {
        composition_disputes_submitted(post)
    }

    /// Returns true when manifest-based revenue routing is active.
    public fun has_composition_monetization(post: &Post): bool {
        monetization_enabled(post) && option::is_some(&revenue_manifest(post))
    }

    public fun tip_post_requires_beneficiary_vault_for_amount(post: &Post, tip_amount: u64): bool {
        if (tip_amount == 0 || !monetization_enabled(post)) {
            return false
        };
        let manifest_opt = revenue_manifest(post);
        if (option::is_none(&manifest_opt)) {
            return false
        };
        let manifest = option::borrow(&manifest_opt);
        let entries = media_asset::manifest_entries(manifest);
        let len = vector::length(entries);
        let mut i = 0;
        while (i < len) {
            let e = vector::borrow(entries, i);
            if (media_asset::manifest_entry_payout_mode(e) == media_asset::payout_escrow()) {
                let slice = (tip_amount * media_asset::manifest_entry_share_bps(e)) / media_asset::manifest_bps_total();
                if (slice > 0) {
                    return true
                };
            };
            i = i + 1;
        };
        false
    }

    public fun actor_address(post: &Post): address {
        post_attribution(post).actor_address
    }

    public fun sub_agent_id(post: &Post): Option<ID> {
        post_attribution(post).sub_agent_id
    }

    public fun action_identity_class(post: &Post): u8 {
        post_attribution(post).action_identity_class
    }

    public fun comment_actor_address(comment: &Comment): address {
        comment_attribution(comment).actor_address
    }

    public fun comment_sub_agent_id(comment: &Comment): Option<ID> {
        comment_attribution(comment).sub_agent_id
    }

    public fun comment_action_identity_class(comment: &Comment): u8 {
        comment_attribution(comment).action_identity_class
    }

    fun post_attribution(post: &Post): PostAttribution {
        *df::borrow<vector<u8>, PostAttribution>(&post.id, POST_ATTRIBUTION_DF_KEY)
    }

    fun attach_post_attribution(
        post: &mut Post,
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
    ) {
        df::add(
            &mut post.id,
            POST_ATTRIBUTION_DF_KEY,
            PostAttribution {
                actor_address,
                sub_agent_id,
                organization_id,
                action_identity_class,
            },
        );
    }

    fun comment_attribution(comment: &Comment): CommentAttribution {
        *df::borrow<vector<u8>, CommentAttribution>(&comment.id, COMMENT_ATTRIBUTION_DF_KEY)
    }

    fun attach_comment_attribution(
        comment: &mut Comment,
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
    ) {
        df::add(
            &mut comment.id,
            COMMENT_ATTRIBUTION_DF_KEY,
            CommentAttribution {
                actor_address,
                sub_agent_id,
                organization_id,
                action_identity_class,
            },
        );
    }

    /// Get the SPT pool ID for a post
    public fun get_spt_id(post: &Post): &Option<address> {
        &post.spt_id
    }

    /// Internal function to set SPT pool ID (package visibility only)
    public(package) fun set_spt_id(post: &mut Post, spt_id: address) {
        post.spt_id = option::some(spt_id);
    }

    /// Package helper to flip the SPT opt-in flag (late-enable / create-with-SPT paths).
    public(package) fun set_enable_spt(post: &mut Post, enabled: bool) {
        post.enable_spt = enabled;
    }

    // --- Multi-claim SPoT analysis (dynamic-field attached) ---

    /// SPoT analysis status sentinels for cross-module use.
    public fun spot_status_pending(): u8 { SPOT_STATUS_PENDING }
    public fun spot_status_completed(): u8 { SPOT_STATUS_COMPLETED }
    public fun spot_status_completed_no_actionable(): u8 { SPOT_STATUS_COMPLETED_NO_ACTIONABLE }

    /// Whether an analysis record has been attached to this post yet.
    public fun has_spot_analysis(post: &Post): bool {
        df::exists_with_type<vector<u8>, PostSpotAnalysis>(&post.id, SPOT_ANALYSIS_DF_KEY)
    }

    /// Analysis status; `pending` (0) when no analysis has been attached.
    public fun spot_analysis_status(post: &Post): u8 {
        if (has_spot_analysis(post)) {
            df::borrow<vector<u8>, PostSpotAnalysis>(&post.id, SPOT_ANALYSIS_DF_KEY).status
        } else {
            SPOT_STATUS_PENDING
        }
    }

    /// Future-linked market object ids in claim_index order (empty when none/pending).
    public fun spot_analysis_market_ids(post: &Post): vector<address> {
        if (has_spot_analysis(post)) {
            df::borrow<vector<u8>, PostSpotAnalysis>(&post.id, SPOT_ANALYSIS_DF_KEY).market_ids
        } else {
            vector::empty()
        }
    }

    /// Future-linked claim indexes in ascending order (empty when none/pending).
    public fun spot_analysis_claim_indexes(post: &Post): vector<u64> {
        if (has_spot_analysis(post)) {
            df::borrow<vector<u8>, PostSpotAnalysis>(&post.id, SPOT_ANALYSIS_DF_KEY).claim_indexes
        } else {
            vector::empty()
        }
    }

    /// Future-linked claim object ids in claim_index order (empty when none/pending).
    public fun spot_analysis_claim_ids(post: &Post): vector<address> {
        if (has_spot_analysis(post)) {
            df::borrow<vector<u8>, PostSpotAnalysis>(&post.id, SPOT_ANALYSIS_DF_KEY).claim_ids
        } else {
            vector::empty()
        }
    }

    public fun spot_analysis_future_accepted_count(post: &Post): u64 {
        if (has_spot_analysis(post)) {
            df::borrow<vector<u8>, PostSpotAnalysis>(&post.id, SPOT_ANALYSIS_DF_KEY).future_accepted_count
        } else { 0 }
    }

    /// Ensure a `pending` analysis exists so future links can accumulate before finalize.
    public(package) fun ensure_spot_analysis_pending(post: &mut Post, max_claim_per_post_applied: u64) {
        if (!has_spot_analysis(post)) {
            df::add(&mut post.id, SPOT_ANALYSIS_DF_KEY, PostSpotAnalysis {
                status: SPOT_STATUS_PENDING,
                detected_claim_count: 0,
                rejected_claim_count: 0,
                truncated_claim_count: 0,
                future_accepted_count: 0,
                past_verified_count: 0,
                max_claim_per_post_applied,
                claim_indexes: vector::empty(),
                claim_ids: vector::empty(),
                market_ids: vector::empty(),
                policy_hashes: vector::empty(),
                claim_manifest_hash: option::none(),
                veracity_manifest_hash: option::none(),
            });
        }
    }

    /// Append one future-claim link (must be pending, strictly increasing claim_index, unique market).
    public(package) fun spot_analysis_append_future(
        post: &mut Post,
        claim_index: u64,
        claim_id: address,
        market_id: address,
        policy_hash: vector<u8>,
    ) {
        ensure_spot_analysis_pending(post, 0);
        let a = df::borrow_mut<vector<u8>, PostSpotAnalysis>(&mut post.id, SPOT_ANALYSIS_DF_KEY);
        assert!(a.status == SPOT_STATUS_PENDING, ESpotAnalysisNotPending);
        let n = vector::length(&a.claim_indexes);
        if (n > 0) {
            assert!(claim_index > *vector::borrow(&a.claim_indexes, n - 1), ESpotAnalysisIndexOrder);
        };
        assert!(!vector::contains(&a.market_ids, &market_id), ESpotAnalysisDuplicateMarket);
        vector::push_back(&mut a.claim_indexes, claim_index);
        vector::push_back(&mut a.claim_ids, claim_id);
        vector::push_back(&mut a.market_ids, market_id);
        vector::push_back(&mut a.policy_hashes, policy_hash);
        a.future_accepted_count = a.future_accepted_count + 1;
    }

    /// Finalize analysis: validate future vectors, set terminal status, counts and manifests.
    /// Aborts unless currently pending. Called by the SPoT batch-finalize entry.
    public(package) fun finalize_spot_analysis(
        post: &mut Post,
        status: u8,
        detected_claim_count: u64,
        rejected_claim_count: u64,
        truncated_claim_count: u64,
        past_verified_count: u64,
        max_claim_per_post_applied: u64,
        claim_manifest_hash: Option<vector<u8>>,
        veracity_manifest_hash: Option<vector<u8>>,
    ) {
        ensure_spot_analysis_pending(post, max_claim_per_post_applied);
        let a = df::borrow_mut<vector<u8>, PostSpotAnalysis>(&mut post.id, SPOT_ANALYSIS_DF_KEY);
        assert!(a.status == SPOT_STATUS_PENDING, ESpotAnalysisNotPending);
        let future_len = vector::length(&a.claim_indexes);
        assert!(vector::length(&a.claim_ids) == future_len, ESpotAnalysisVectorMismatch);
        assert!(vector::length(&a.market_ids) == future_len, ESpotAnalysisVectorMismatch);
        assert!(vector::length(&a.policy_hashes) == future_len, ESpotAnalysisVectorMismatch);
        assert!(a.future_accepted_count == future_len, ESpotAnalysisVectorMismatch);
        if (max_claim_per_post_applied > 0) {
            assert!(future_len + past_verified_count <= max_claim_per_post_applied, ESpotAnalysisOverCap);
        };
        a.status = status;
        a.detected_claim_count = detected_claim_count;
        a.rejected_claim_count = rejected_claim_count;
        a.truncated_claim_count = truncated_claim_count;
        a.past_verified_count = past_verified_count;
        a.max_claim_per_post_applied = max_claim_per_post_applied;
        a.claim_manifest_hash = claim_manifest_hash;
        a.veracity_manifest_hash = veracity_manifest_hash;
    }

    /// Comment object for posts, supporting nested comments
    public struct Comment has key {
        id: UID,
        /// The post this comment belongs to
        post_id: address,
        /// Optional parent comment ID for nested comments
        parent_comment_id: Option<address>,
        /// Owner's wallet address (the true owner)
        owner: address,
        /// Commenter's profile ID (reference only, not ownership)
        profile_id: address,
        /// Comment content
        content: String,
        /// Optional media URLs
        media: Option<vector<Url>>,
        /// Optional mentioned users (profile IDs)
        mentions: Option<vector<address>>,
        /// Optional metadata in JSON format
        metadata_json: Option<String>,
        /// Creation timestamp
        created_at: u64,
        /// Total number of reactions
        reaction_count: u64,
        /// Number of nested comments
        comment_count: u64,
        /// Number of reposts
        repost_count: u64,
        /// Total tips received in MYSO (tracking only, not actual balance)
        tips_received: u64,
        /// Whether the comment has been removed from its platform
        removed_from_platform: bool,
        /// Table of user wallet addresses to their reactions (emoji or text)
        user_reactions: Table<address, String>,
        /// Table to count reactions by type
        reaction_counts: Table<String, u64>,
        /// Version for upgrades
        version: u64,
    }

    /// Repost reference
    public struct Repost has key {
        id: UID,
        /// The post/comment being reposted
        original_id: address,
        /// Whether the original is a post (true) or comment (false)
        is_original_post: bool,
        /// Owner's wallet address (the true owner)
        owner: address,
        /// Reposter's profile ID (reference only, not ownership)
        profile_id: address,
        /// Creation timestamp
        created_at: u64,
        /// Version for upgrades
        version: u64,
    }

    /// Promoted post view record
    public struct PromotionView has store, copy, drop {
        viewer: address,
        view_duration: u64,
        view_timestamp: u64,
        platform_id: address,
    }

    /// Promoted post metadata
    public struct PromotionData has key {
        id: UID,
        post_id: address,
        /// Amount of MYSO to pay per view
        payment_per_view: u64,
        /// MYSO balance available for payments
        promotion_budget: Balance<MYSO>,
        /// Table tracking which users have already been paid for viewing
        paid_viewers: Table<address, bool>,
        /// List of all views for analytics
        views: vector<PromotionView>,
        /// Whether the promotion is currently active
        active: bool,
        /// Promotion creation timestamp
        created_at: u64,
    }

    /// Admin capability for post administration
    public struct PostAdminCap has key, store {
        id: UID,
    }

    /// Global post feature configuration
    public struct PostConfig has key {
        id: UID,
        /// Maximum character length for post content
        max_content_length: u64,
        /// Maximum number of media URLs per post
        max_media_urls: u64,
        /// Maximum number of mentions in a post
        max_mentions: u64,
        /// Maximum size for post metadata in bytes
        max_metadata_size: u64,
        /// Maximum length for report descriptions
        max_description_length: u64,
        /// Maximum length for reactions
        max_reaction_length: u64,
        /// Percentage of tip that goes to commenter (remainder to post owner)
        commenter_tip_percentage: u64,
        /// Percentage of tip that goes to reposter (remainder to original post owner)
        repost_tip_percentage: u64,
        /// Minimum payment per view for promoted posts (MIST)
        min_promotion_amount: u64,
        /// Maximum payment per view for promoted posts (MIST)
        max_promotion_amount: u64,
        /// Minimum view duration for a promoted post view to count (ms)
        min_view_duration_ms: u64,
        /// Platform fee bps taken from each confirmed promo view gross
        platform_fee_bps: u64,
        /// Ecosystem fee bps taken from each confirmed promo view gross
        ecosystem_fee_bps: u64,
        /// Version for upgrades
        version: u64,
    }

    /// Event emitted when post parameters are updated
    public struct PostParametersUpdatedEvent has copy, drop {
        /// Who performed the update
        updated_by: address,
        /// When the update occurred
        timestamp: u64,
        /// New max content length value
        max_content_length: u64,
        /// New max media URLs value
        max_media_urls: u64, 
        /// New max mentions value
        max_mentions: u64,
        /// New max metadata size value
        max_metadata_size: u64,
        /// New max description length value
        max_description_length: u64,
        /// New max reaction length value
        max_reaction_length: u64,
        /// New commenter tip percentage value
        commenter_tip_percentage: u64,
        /// New repost tip percentage value
        repost_tip_percentage: u64,
        /// New min promotion amount value (MIST per view)
        min_promotion_amount: u64,
        /// New max promotion amount value (MIST per view)
        max_promotion_amount: u64,
        /// New min view duration value (ms)
        min_view_duration_ms: u64,
        /// New platform fee bps of each promo view gross
        platform_fee_bps: u64,
        /// New ecosystem fee bps of each promo view gross
        ecosystem_fee_bps: u64,
    }

    /// Post created event
    public struct PostCreatedEvent has copy, drop {
        post_id: address,
        owner: address,
        profile_id: address,
        platform_id: address,
        permissions: u8,
        content: String,
        post_type: String,
        parent_post_id: Option<address>,
        mentions: Option<vector<address>>,
        media_asset_ids: vector<ID>,
        media_urls: Option<vector<String>>,
        metadata_json: Option<String>,
        access: PostAccess,
        promotion_id: Option<address>,
        composition_status: u8,
        monetization_status: u8,
        enable_spt: bool,
        spt_id: Option<address>,
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
    }

    /// Comment created event
    #[allow(unused_field)]
    public struct CommentCreatedEvent has copy, drop {
        comment_id: address,
        post_id: address,
        parent_comment_id: Option<address>,
        owner: address,
        profile_id: address,
        content: String,
        mentions: Option<vector<address>>,
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
    }

    /// Repost event
    public struct RepostEvent has copy, drop {
        repost_id: address,
        original_id: address,
        is_original_post: bool,
        owner: address,
        profile_id: address,
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
    }

    /// Repost removal event with principal and delegated-agent attribution.
    public struct RepostRemovedEvent has copy, drop {
        repost_id: address,
        original_id: address,
        owner: address,
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
        removed_at: u64,
    }

    /// Reaction event
    public struct ReactionEvent has copy, drop {
        object_id: address,
        user: address,
        reaction: String,
        is_post: bool,
        principal_owner: address,
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
    }

    /// Remove reaction event
    public struct RemoveReactionEvent has copy, drop {
        object_id: address,
        user: address,
        reaction: String,
        is_post: bool,
        principal_owner: address,
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
    }

    /// Tip event
    public struct TipEvent has copy, drop {
        object_id: address,
        from: address,
        to: address,
        amount: u64,
        coin_type: TypeName,
        is_post: bool,
    }

    /// Post ownership transfer event
    public struct OwnershipTransferEvent has copy, drop {
        object_id: address,
        previous_owner: address,
        new_owner: address,
        is_post: bool,
    }

    /// Post moderation event
    public struct PostModerationEvent has copy, drop {
        post_id: address,
        platform_id: address,
        removed: bool,
        moderated_by: address,
    }

    /// Post updated event
    public struct PostUpdatedEvent has copy, drop {
        post_id: address,
        owner: address,
        profile_id: address,
        content: String,
        metadata_json: Option<String>,
        updated_at: u64,
    }

    /// Comment updated event 
    public struct CommentUpdatedEvent has copy, drop {
        comment_id: address,
        post_id: address,
        owner: address,
        profile_id: address,
        content: String,
        updated_at: u64,
    }

    /// Post reported event
    public struct PostReportedEvent has copy, drop {
        post_id: address,
        reporter: address,
        reason_code: u8,
        description: String,
        reported_at: u64,
    }

    /// Comment reported event
    public struct CommentReportedEvent has copy, drop {
        comment_id: address,
        reporter: address,
        reason_code: u8,
        description: String,
        reported_at: u64,
    }

    /// Post deleted event
    public struct PostDeletedEvent has copy, drop {
        post_id: address,
        owner: address,
        profile_id: address,
        post_type: String,
        deleted_at: u64,
    }
    
    /// Comment deleted event
    public struct CommentDeletedEvent has copy, drop {
        comment_id: address,
        post_id: address,
        owner: address,
        profile_id: address,
        deleted_at: u64,
    }

    /// Event emitted when a promoted post is created
    public struct PromotedPostCreatedEvent has copy, drop {
        post_id: address,
        owner: address,
        profile_id: address,
        payment_per_view: u64,
        total_budget: u64,
        created_at: u64,
    }

    /// One item inside a batch promoted-view confirmation
    public struct PromotedViewConfirmItem has copy, drop {
        post_id: address,
        promotion_id: address,
        /// Gross debit from this promotion's budget (`payment_per_view`)
        payment_amount: u64,
        platform_fee: u64,
        ecosystem_fee: u64,
        /// Net attributed to this view (part of the merged wallet transfer)
        recipient_amount: u64,
        view_duration: u64,
    }

    /// Event emitted when one viewer is paid for N (≥1) promoted views in a single tx
    public struct PromotedPostViewsBatchConfirmedEvent has copy, drop {
        viewer: address,
        platform_id: address,
        timestamp: u64,
        items: vector<PromotedViewConfirmItem>,
        total_payment_amount: u64,
        total_platform_fee: u64,
        total_ecosystem_fee: u64,
        total_recipient_amount: u64,
    }

    /// Event emitted when promotion status is toggled
    public struct PromotionStatusToggledEvent has copy, drop {
        post_id: address,
        toggled_by: address,
        new_status: bool,
        timestamp: u64,
    }

    /// Event emitted when promotion funds are withdrawn
    public struct PromotionFundsWithdrawnEvent has copy, drop {
        post_id: address,
        owner: address,
        withdrawn_amount: u64,
        timestamp: u64,
    }

    /// Simple moderation record for tracking moderation decisions
    public struct ModerationRecord has key {
        id: UID,
        post_id: address,
        platform_id: address,
        moderation_state: u8,
        moderator: Option<address>,
        moderation_timestamp: Option<u64>,
        reason: Option<String>,
    }

    /// Bootstrap initialization function - creates the post configuration
    public(package) fun bootstrap_init(clock: &Clock, ctx: &mut TxContext) {
        let admin = tx_context::sender(ctx);
        let config = PostConfig {
            id: object::new(ctx),
            max_content_length: MAX_CONTENT_LENGTH,
            max_media_urls: MAX_MEDIA_URLS,
            max_mentions: MAX_MENTIONS,
            max_metadata_size: MAX_METADATA_SIZE,
            max_description_length: MAX_DESCRIPTION_LENGTH,
            max_reaction_length: MAX_REACTION_LENGTH,
            commenter_tip_percentage: COMMENTER_TIP_PERCENTAGE,
            repost_tip_percentage: REPOST_TIP_PERCENTAGE,
            min_promotion_amount: MIN_PROMOTION_AMOUNT,
            max_promotion_amount: MAX_PROMOTION_AMOUNT,
            min_view_duration_ms: MIN_VIEW_DURATION,
            platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
            ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
            version: upgrade::current_version(),
        };

        // Emit event so indexer can populate post_config table
        event::emit(PostParametersUpdatedEvent {
            updated_by: admin,
            timestamp: clock::timestamp_ms(clock),
            max_content_length: MAX_CONTENT_LENGTH,
            max_media_urls: MAX_MEDIA_URLS,
            max_mentions: MAX_MENTIONS,
            max_metadata_size: MAX_METADATA_SIZE,
            max_description_length: MAX_DESCRIPTION_LENGTH,
            max_reaction_length: MAX_REACTION_LENGTH,
            commenter_tip_percentage: COMMENTER_TIP_PERCENTAGE,
            repost_tip_percentage: REPOST_TIP_PERCENTAGE,
            min_promotion_amount: MIN_PROMOTION_AMOUNT,
            max_promotion_amount: MAX_PROMOTION_AMOUNT,
            min_view_duration_ms: MIN_VIEW_DURATION,
            platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
            ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
        });

        // Create and share post configuration
        transfer::share_object(config);
    }

    /// Convert Option<vector<Url>> to Option<vector<String>> for events
    fun convert_urls_to_strings(media_option: &Option<vector<Url>>): Option<vector<String>> {
        if (option::is_some(media_option)) {
            let urls = option::borrow(media_option);
            let mut url_strings = vector::empty<String>();
            let len = vector::length(urls);
            let mut i = 0;
            while (i < len) {
                let url = vector::borrow(urls, i);
                let url_string = string::from_ascii(url::inner_url(url));
                vector::push_back(&mut url_strings, url_string);
                i = i + 1;
            };
            option::some(url_strings)
        } else {
            option::none()
        }
    }

    /// Resolve social actor: capability, principal platform join, block list, and approval gate.
    fun resolve_social_actor(
        memory_config: &MemoryConfig,
        registry: &UsernameRegistry,
        platform: &platform::Platform,
        block_list_registry: &BlockListRegistry,
        memory_account: &MemoryAccount,
        required_cap: u64,
        spend_amount: u64,
        clock: &Clock,
        ctx: &TxContext,
    ): ActingContext {
        let platform_id = object::uid_to_address(platform::id(platform));
        let acting = memory::resolve_actor_with_cap(
            memory_config,
            memory_account,
            required_cap,
            option::some(platform_id),
            spend_amount,
            clock,
            ctx,
        );
        memory::assert_direct_execution_allowed(memory_account, required_cap, ctx);

        let principal = memory::acting_principal_owner(&acting);
        assert!(memory::owner(memory_account) == principal, EUnauthorized);

        let profile_id = memory::acting_profile_id(&acting);
        let profile_id_option = profile::lookup_profile_by_owner(registry, principal);
        assert!(option::is_some(&profile_id_option), EUnauthorized);
        assert!(*option::borrow(&profile_id_option) == profile_id, EUnauthorized);

        assert!(platform::has_joined_platform(platform, principal), EUserNotJoinedPlatform);
        assert!(
            !block_list::is_blocked(block_list_registry, platform_id, principal),
            EUserBlockedByPlatform,
        );

        acting
    }

    fun assert_tip_spend_limit(
        memory_account: &MemoryAccount,
        amount: u64,
        ctx: &TxContext,
    ) {
        let caller = tx_context::sender(ctx);
        if (memory::is_registered_agent(memory_account, caller)) {
            memory::assert_action_spend_limit(memory_account, amount, ctx);
        };
    }

    /// Internal function to create a post object (not yet shared).
    fun create_post_internal(
        owner: address,
        profile_id: address,
        platform_id: address,
        content: String,
        media_asset_ids: vector<ID>,
        media_option: Option<vector<Url>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        post_type: String,
        parent_post_id: Option<address>,
        allow_comments: bool,
        allow_reactions: bool,
        allow_reposts: bool,
        allow_quotes: bool,
        allow_tips: bool,
        access: PostAccess,
        promotion_id: Option<address>,
        enable_spt: bool,
        actor_address: address,
        sub_agent_id: Option<ID>,
        organization_id: Option<ID>,
        action_identity_class: u8,
        clock: &Clock,
        ctx: &mut TxContext
    ): Post {
        let asset_len = vector::length(&media_asset_ids);
        assert!(asset_len <= MAX_MEDIA_URLS, ETooManyMediaUrls);

        // Build permissions bitfield
        let mut permissions: u8 = 0;
        if (allow_comments) { permissions = permissions | PERMISSION_ALLOW_COMMENTS };
        if (allow_reactions) { permissions = permissions | PERMISSION_ALLOW_REACTIONS };
        if (allow_reposts) { permissions = permissions | PERMISSION_ALLOW_REPOSTS };
        if (allow_quotes) { permissions = permissions | PERMISSION_ALLOW_QUOTES };
        if (allow_tips) { permissions = permissions | PERMISSION_ALLOW_TIPS };

        let mut post = Post {
            id: object::new(ctx),
            owner,
            profile_id,
            platform_id,
            content,
            media_asset_ids,
            media: media_option,
            mentions,
            metadata_json,
            post_type,
            parent_post_id,
            created_at: clock::timestamp_ms(clock),
            reaction_count: 0,
            comment_count: 0,
            repost_count: 0,
            tips_received: 0,
            removed_from_platform: false,
            user_reactions: table::new(ctx),
            reaction_counts: table::new(ctx),
            permissions,
            composition_status: media_asset::composition_none(),
            monetization_status: media_asset::monetization_none(),
            composition_badge: option::none(),
            access,
            promotion_id,
            enable_spt,
            spt_id: option::none(),
            version: upgrade::current_version(),
        };

        attach_post_attribution(
            &mut post,
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
        );

        post
    }

    fun emit_post_media_asset_usages(post: &Post, clock: &Clock) {
        let post_id = object::uid_to_address(&post.id);
        let assets = &post.media_asset_ids;
        let len = vector::length(assets);
        let mut i = 0;
        while (i < len) {
            let asset_id = *vector::borrow(assets, i);
            media_asset::emit_media_asset_used(
                post_id,
                media_asset::container_post(),
                asset_id,
                media_asset::usage_social_post(),
                (i as u8),
                clock,
            );
            i = i + 1;
        };
    }

    public(package) fun share_post(post: Post): address {
        let post_id = object::uid_to_address(&post.id);
        transfer::share_object(post);
        post_id
    }

    /// Validates registry ownership when a MyData listing is linked on `access`.
    fun assert_post_access_mydata_binding(
        owner: address,
        access: &PostAccess,
        registry: &mydata::MyDataRegistry,
    ) {
        let linked = linked_mydata_from_access(access);
        if (option::is_none(&linked)) {
            return
        };
        let linked_id = *option::borrow(&linked);
        let ip_id = object::id_to_address(&linked_id);
        let reg_owner = mydata::registry_get_owner(registry, ip_id);
        assert!(option::is_some(&reg_owner), EMyDataNotRegistered);
        assert!(*option::borrow(&reg_owner) == owner, EMyDataOwnerMismatch);
    }

    /// Cross-layer MyData access binding when the listing object is supplied in the PTB.
    fun assert_post_access_mydata_object_binding(
        access: &PostAccess,
        mydata: &mydata::MyData,
    ) {
        let linked = linked_mydata_from_access(access);
        assert!(option::is_some(&linked), ENoEncryptedContent);
        let linked_id = *option::borrow(&linked);
        assert!(object::id(mydata) == linked_id, EMyDataOwnerMismatch);
        assert!(!mydata::requires_marketplace_subscription(mydata), ENoEncryptedContent);

        match (access) {
            PostAccess::Public => abort EUnauthorized,
            PostAccess::ProfileSubscription { .. } => {
                assert!(mydata::requires_profile_subscription_access(mydata), ENoEncryptedContent);
            },
            PostAccess::MarketplaceOneTime { .. } => {
                assert!(mydata::requires_marketplace_purchase(mydata), EPriceMismatch);
            },
        }
    }

    fun linked_mydata_from_access(access: &PostAccess): Option<ID> {
        let (_, _, mydata_id, _) = post_access_fields(access);
        mydata_id
    }

    fun assert_profile_subscription_access_service(
        owner: address,
        access: &PostAccess,
        service: &ProfileSubscriptionService,
    ) {
        match (access) {
            PostAccess::ProfileSubscription { service_id, .. } => {
                assert!(subscription::service_profile_owner(service) == owner, EUnauthorized);
                assert!(subscription::service_is_active(service), ENoSubscriptionService);
                assert!(*service_id == object::id(service), ENoSubscriptionService);
            },
            _ => abort EUnauthorized,
        }
    }

    /// Create a new post with interaction permissions.
    fun create_post_entry_body(
        registry: &UsernameRegistry,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        content: String,
        media_asset_ids: vector<ID>,
        mut media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        allow_comments: Option<bool>,
        allow_reactions: Option<bool>,
        allow_reposts: Option<bool>,
        allow_quotes: Option<bool>,
        allow_tips: Option<bool>,
        enable_spt: Option<bool>,
        enable_spot: Option<bool>,
        access: PostAccess,
        mydata_registry: &mydata::MyDataRegistry,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let acting = resolve_social_actor(
            memory_config,
            registry,
            platform,
            block_list_registry,
            memory_account,
            memory::cap_post_publish(),
            0,
            clock,
            ctx,
        );
        let owner = memory::acting_principal_owner(&acting);
        let profile_id = memory::acting_profile_id(&acting);
        let actor_address = memory::acting_actor_address(&acting);
        let sub_agent_id = memory::acting_sub_agent_id(&acting);
        let organization_id = memory::acting_organization_id(&acting);
        let action_identity_class = memory::acting_identity_class(&acting);

        assert_post_access_mydata_binding(owner, &access, mydata_registry);
        
        // Check if platform is approved
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), EUnauthorized);
        
        // Validate content length using config
        assert!(string::length(&content) <= config.max_content_length, EContentTooLarge);
        
        // Validate metadata size if provided
        if (option::is_some(&metadata_json)) {
            let metadata_ref = option::borrow(&metadata_json);
            assert!(string::length(metadata_ref) <= config.max_metadata_size, EContentTooLarge);
        };
        
        // Convert and validate media URLs if provided
        let media_option = if (option::is_some(&media_urls)) {
            let url_strings = option::extract(&mut media_urls);
            
            // Validate media URLs count using config
            assert!(vector::length(&url_strings) <= config.max_media_urls, ETooManyMediaUrls);
            
            // Convert string URLs to Url objects
            let mut urls = vector::empty<Url>();
            let mut i = 0;
            let len = vector::length(&url_strings);
            while (i < len) {
                let url_string = vector::borrow(&url_strings, i);
                let url_bytes = string::as_bytes(url_string);
                vector::push_back(&mut urls, url::new_unsafe_from_bytes(*url_bytes));
                i = i + 1;
            };
            option::some(urls)
        } else {
            option::none<vector<Url>>()
        };
        
        // Validate mentions if provided using config
        if (option::is_some(&mentions)) {
            let mentions_ref = option::borrow(&mentions);
            assert!(vector::length(mentions_ref) <= config.max_mentions, EContentTooLarge);
        };
        
        // Set defaults for optional boolean parameters
        let final_allow_comments = if (option::is_some(&allow_comments)) {
            *option::borrow(&allow_comments)
        } else {
            true // Default to allowing comments
        };
        let final_allow_reactions = if (option::is_some(&allow_reactions)) {
            *option::borrow(&allow_reactions)
        } else {
            true // Default to allowing reactions
        };
        let final_allow_reposts = if (option::is_some(&allow_reposts)) {
            *option::borrow(&allow_reposts)
        } else {
            true // Default to allowing reposts
        };
        let final_allow_quotes = if (option::is_some(&allow_quotes)) {
            *option::borrow(&allow_quotes)
        } else {
            true // Default to allowing quotes
        };
        let final_allow_tips = if (option::is_some(&allow_tips)) {
            *option::borrow(&allow_tips)
        } else {
            true // Default to allowing tips
        };
        
        // Set defaults for feature flags (default to opt-out - users must explicitly opt-in)
        let final_enable_spt = if (option::is_some(&enable_spt)) {
            *option::borrow(&enable_spt)
        } else {
            false // Default to opt-out (user must explicitly opt-in)
        };
        // SPT opt-in at create must go through social_proof_tokens::create_post_with_reservation_pool.
        assert!(!final_enable_spt, ESptRequiresDedicatedCreate);
        // enable_spot retained for entry-signature compatibility; SPoT is always-on.
        let _ = enable_spot;

        // Convert media URLs to strings for event (before moving media_option)
        let media_urls_for_event = convert_urls_to_strings(&media_option);
        let media_asset_ids_for_event = media_asset_ids;

        let post = create_post_internal(
            owner,
            profile_id,
            platform_id,
            content,
            media_asset_ids,
            media_option,
            mentions,
            metadata_json,
            string::utf8(POST_TYPE_STANDARD),
            option::none(),
            final_allow_comments,
            final_allow_reactions,
            final_allow_reposts,
            final_allow_quotes,
            final_allow_tips,
            access,
            option::none(), // promotion_id
            false,
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
            clock,
            ctx
        );

        emit_post_media_asset_usages(&post, clock);
        let post_id = share_post(post);

        let permissions_for_event = permissions_bitfield(
            final_allow_comments,
            final_allow_reactions,
            final_allow_reposts,
            final_allow_quotes,
            final_allow_tips,
        );

        event::emit(PostCreatedEvent {
            post_id,
            owner,
            profile_id,
            platform_id,
            permissions: permissions_for_event,
            content,
            post_type: string::utf8(POST_TYPE_STANDARD),
            parent_post_id: option::none(),
            mentions,
            media_asset_ids: media_asset_ids_for_event,
            media_urls: media_urls_for_event,
            metadata_json,
            access,
            promotion_id: option::none(),
            composition_status: media_asset::composition_none(),
            monetization_status: media_asset::monetization_none(),
            enable_spt: false,
            spt_id: option::none(),
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
        });
    }

    /// Build an unsared standard post with `enable_spt = true` for SPT create-with-reservation.
    /// Caller bootstraps the reservation pool, sets `spt_id`, then `share_and_emit_spt_post`.
    public(package) fun create_post_object_for_spt(
        registry: &UsernameRegistry,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        content: String,
        media_asset_ids: vector<ID>,
        mut media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        allow_comments: Option<bool>,
        allow_reactions: Option<bool>,
        allow_reposts: Option<bool>,
        allow_quotes: Option<bool>,
        allow_tips: Option<bool>,
        access: PostAccess,
        mydata_registry: &mydata::MyDataRegistry,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ): Post {
        let acting = resolve_social_actor(
            memory_config,
            registry,
            platform,
            block_list_registry,
            memory_account,
            memory::cap_post_publish(),
            0,
            clock,
            ctx,
        );
        let owner = memory::acting_principal_owner(&acting);
        let profile_id = memory::acting_profile_id(&acting);
        let actor_address = memory::acting_actor_address(&acting);
        let sub_agent_id = memory::acting_sub_agent_id(&acting);
        let organization_id = memory::acting_organization_id(&acting);
        let action_identity_class = memory::acting_identity_class(&acting);

        assert_post_access_mydata_binding(owner, &access, mydata_registry);

        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), EUnauthorized);
        assert!(string::length(&content) <= config.max_content_length, EContentTooLarge);
        if (option::is_some(&metadata_json)) {
            let metadata_ref = option::borrow(&metadata_json);
            assert!(string::length(metadata_ref) <= config.max_metadata_size, EContentTooLarge);
        };

        let media_option = if (option::is_some(&media_urls)) {
            let url_strings = option::extract(&mut media_urls);
            assert!(vector::length(&url_strings) <= config.max_media_urls, ETooManyMediaUrls);
            let mut urls = vector::empty<Url>();
            let mut i = 0;
            let len = vector::length(&url_strings);
            while (i < len) {
                let url_string = vector::borrow(&url_strings, i);
                let url_bytes = string::as_bytes(url_string);
                vector::push_back(&mut urls, url::new_unsafe_from_bytes(*url_bytes));
                i = i + 1;
            };
            option::some(urls)
        } else {
            option::none<vector<Url>>()
        };

        if (option::is_some(&mentions)) {
            let mentions_ref = option::borrow(&mentions);
            assert!(vector::length(mentions_ref) <= config.max_mentions, EContentTooLarge);
        };

        let final_allow_comments = if (option::is_some(&allow_comments)) {
            *option::borrow(&allow_comments)
        } else {
            true
        };
        let final_allow_reactions = if (option::is_some(&allow_reactions)) {
            *option::borrow(&allow_reactions)
        } else {
            true
        };
        let final_allow_reposts = if (option::is_some(&allow_reposts)) {
            *option::borrow(&allow_reposts)
        } else {
            true
        };
        let final_allow_quotes = if (option::is_some(&allow_quotes)) {
            *option::borrow(&allow_quotes)
        } else {
            true
        };
        let final_allow_tips = if (option::is_some(&allow_tips)) {
            *option::borrow(&allow_tips)
        } else {
            true
        };

        create_post_internal(
            owner,
            profile_id,
            platform_id,
            content,
            media_asset_ids,
            media_option,
            mentions,
            metadata_json,
            string::utf8(POST_TYPE_STANDARD),
            option::none(),
            final_allow_comments,
            final_allow_reactions,
            final_allow_reposts,
            final_allow_quotes,
            final_allow_tips,
            access,
            option::none(),
            true, // enable_spt
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
            clock,
            ctx
        )
    }

    /// Set reservation-pool `spt_id`, share the post, and emit `PostCreatedEvent` with SPT fields filled.
    public(package) fun share_and_emit_spt_post(mut post: Post, spt_pool_id: address, clock: &Clock): address {
        set_enable_spt(&mut post, true);
        set_spt_id(&mut post, spt_pool_id);
        emit_post_media_asset_usages(&post, clock);
        let media_urls_for_event = convert_urls_to_strings(&post.media);
        let media_asset_ids_for_event = post.media_asset_ids;
        let attr = post_attribution(&post);
        let post_id = object::uid_to_address(&post.id);
        event::emit(PostCreatedEvent {
            post_id,
            owner: post.owner,
            profile_id: post.profile_id,
            platform_id: post.platform_id,
            permissions: post.permissions,
            content: post.content,
            post_type: post.post_type,
            parent_post_id: post.parent_post_id,
            mentions: post.mentions,
            media_asset_ids: media_asset_ids_for_event,
            media_urls: media_urls_for_event,
            metadata_json: post.metadata_json,
            access: post.access,
            promotion_id: post.promotion_id,
            composition_status: post.composition_status,
            monetization_status: post.monetization_status,
            enable_spt: true,
            spt_id: option::some(spt_pool_id),
            actor_address: attr.actor_address,
            sub_agent_id: attr.sub_agent_id,
            organization_id: attr.organization_id,
            action_identity_class: attr.action_identity_class,
        });
        transfer::share_object(post);
        post_id
    }

    /// Create a new post; caller supplies access via kind + optional service/mydata ids.
    public entry fun create_post(
        registry: &UsernameRegistry,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        content: String,
        media_asset_ids: vector<ID>,
        media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        allow_comments: Option<bool>,
        allow_reactions: Option<bool>,
        allow_reposts: Option<bool>,
        allow_quotes: Option<bool>,
        allow_tips: Option<bool>,
        enable_spt: Option<bool>,
        enable_spot: Option<bool>,
        access_kind: u8,
        subscription_service_id: Option<ID>,
        linked_mydata_id: Option<ID>,
        subscription_min_tier_level: Option<u64>,
        mydata_registry: &mydata::MyDataRegistry,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let access = post_access_from_parts(
            access_kind,
            subscription_service_id,
            linked_mydata_id,
            subscription_min_tier_level,
        );
        create_post_entry_body(
            registry,
            platform_registry,
            platform,
            block_list_registry,
            config,
            memory_config,
            content,
            media_asset_ids,
            media_urls,
            mentions,
            metadata_json,
            allow_comments,
            allow_reactions,
            allow_reposts,
            allow_quotes,
            allow_tips,
            enable_spt,
            enable_spot,
            access,
            mydata_registry,
            memory_account,
            clock,
            ctx,
        );
    }

    /// Optional object binding for [`create_post`] when a linked MyData listing is in the PTB.
    public entry fun validate_post_mydata_binding(
        access_kind: u8,
        subscription_service_id: Option<ID>,
        linked_mydata_id: Option<ID>,
        subscription_min_tier_level: Option<u64>,
        mydata: &mydata::MyData,
    ) {
        let access = post_access_from_parts(
            access_kind,
            subscription_service_id,
            linked_mydata_id,
            subscription_min_tier_level,
        );
        assert_post_access_mydata_object_binding(&access, mydata);
    }

    /// Create a public post (no subscription or marketplace gate).
    public entry fun create_public_post(
        registry: &UsernameRegistry,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        content: String,
        media_asset_ids: vector<ID>,
        media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        allow_comments: Option<bool>,
        allow_reactions: Option<bool>,
        allow_reposts: Option<bool>,
        allow_quotes: Option<bool>,
        allow_tips: Option<bool>,
        enable_spt: Option<bool>,
        enable_spot: Option<bool>,
        mydata_registry: &mydata::MyDataRegistry,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        create_post_entry_body(
            registry,
            platform_registry,
            platform,
            block_list_registry,
            config,
            memory_config,
            content,
            media_asset_ids,
            media_urls,
            mentions,
            metadata_json,
            allow_comments,
            allow_reactions,
            allow_reposts,
            allow_quotes,
            allow_tips,
            enable_spt,
            enable_spot,
            PostAccess::Public,
            mydata_registry,
            memory_account,
            clock,
            ctx,
        );
    }

    /// Create a profile-subscription-gated post (optional linked MyData for encrypted body).
    public entry fun create_profile_subscription_post(
        registry: &UsernameRegistry,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        content: String,
        media_asset_ids: vector<ID>,
        media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        allow_comments: Option<bool>,
        allow_reactions: Option<bool>,
        allow_reposts: Option<bool>,
        allow_quotes: Option<bool>,
        allow_tips: Option<bool>,
        enable_spt: Option<bool>,
        enable_spot: Option<bool>,
        service: &ProfileSubscriptionService,
        min_tier_level: Option<u64>,
        mydata_id: Option<ID>,
        mydata_registry: &mydata::MyDataRegistry,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let acting = resolve_social_actor(
            memory_config,
            registry,
            platform,
            block_list_registry,
            memory_account,
            memory::cap_post_publish(),
            0,
            clock,
            ctx,
        );
        let owner = memory::acting_principal_owner(&acting);
        let access = PostAccess::ProfileSubscription {
            service_id: object::id(service),
            mydata_id,
            min_tier_level,
        };
        assert_profile_subscription_access_service(owner, &access, service);
        create_post_entry_body(
            registry,
            platform_registry,
            platform,
            block_list_registry,
            config,
            memory_config,
            content,
            media_asset_ids,
            media_urls,
            mentions,
            metadata_json,
            allow_comments,
            allow_reactions,
            allow_reposts,
            allow_quotes,
            allow_tips,
            enable_spt,
            enable_spot,
            access,
            mydata_registry,
            memory_account,
            clock,
            ctx,
        );
    }

    /// Profile-subscription post with linked MyData object binding in the same PTB.
    public entry fun create_profile_subscription_post_with_mydata(
        registry: &UsernameRegistry,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        content: String,
        media_asset_ids: vector<ID>,
        media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        allow_comments: Option<bool>,
        allow_reactions: Option<bool>,
        allow_reposts: Option<bool>,
        allow_quotes: Option<bool>,
        allow_tips: Option<bool>,
        enable_spt: Option<bool>,
        enable_spot: Option<bool>,
        service: &ProfileSubscriptionService,
        min_tier_level: Option<u64>,
        mydata: &mydata::MyData,
        mydata_registry: &mydata::MyDataRegistry,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let acting = resolve_social_actor(
            memory_config,
            registry,
            platform,
            block_list_registry,
            memory_account,
            memory::cap_post_publish(),
            0,
            clock,
            ctx,
        );
        let owner = memory::acting_principal_owner(&acting);
        let access = PostAccess::ProfileSubscription {
            service_id: object::id(service),
            mydata_id: option::some(object::id(mydata)),
            min_tier_level,
        };
        assert_profile_subscription_access_service(owner, &access, service);
        assert_post_access_mydata_binding(owner, &access, mydata_registry);
        assert_post_access_mydata_object_binding(&access, mydata);
        create_post_entry_body(
            registry,
            platform_registry,
            platform,
            block_list_registry,
            config,
            memory_config,
            content,
            media_asset_ids,
            media_urls,
            mentions,
            metadata_json,
            allow_comments,
            allow_reactions,
            allow_reposts,
            allow_quotes,
            allow_tips,
            enable_spt,
            enable_spot,
            access,
            mydata_registry,
            memory_account,
            clock,
            ctx,
        );
    }

    /// Create a marketplace one-time purchase gated post (requires linked MyData listing).
    public entry fun create_marketplace_one_time_post(
        registry: &UsernameRegistry,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        content: String,
        media_asset_ids: vector<ID>,
        media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        allow_comments: Option<bool>,
        allow_reactions: Option<bool>,
        allow_reposts: Option<bool>,
        allow_quotes: Option<bool>,
        allow_tips: Option<bool>,
        enable_spt: Option<bool>,
        enable_spot: Option<bool>,
        mydata: &mydata::MyData,
        mydata_registry: &mydata::MyDataRegistry,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let access = PostAccess::MarketplaceOneTime { mydata_id: object::id(mydata) };
        let acting = resolve_social_actor(
            memory_config,
            registry,
            platform,
            block_list_registry,
            memory_account,
            memory::cap_post_publish(),
            0,
            clock,
            ctx,
        );
        let owner = memory::acting_principal_owner(&acting);
        assert_post_access_mydata_binding(owner, &access, mydata_registry);
        assert_post_access_mydata_object_binding(&access, mydata);
        create_post_entry_body(
            registry,
            platform_registry,
            platform,
            block_list_registry,
            config,
            memory_config,
            content,
            media_asset_ids,
            media_urls,
            mentions,
            metadata_json,
            allow_comments,
            allow_reactions,
            allow_reposts,
            allow_quotes,
            allow_tips,
            enable_spt,
            enable_spot,
            access,
            mydata_registry,
            memory_account,
            clock,
            ctx,
        );
    }

    /// Create a comment on a post or a reply to another comment
    /// Returns the ID of the created comment
    public fun create_comment(
        registry: &UsernameRegistry,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        memory_account: &MemoryAccount,
        parent_post: &mut Post,
        parent_comment_id: Option<address>,
        content: String,
        mut media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ): address {
        let acting = resolve_social_actor(
            memory_config,
            registry,
            platform,
            block_list_registry,
            memory_account,
            memory::cap_comment(),
            0,
            clock,
            ctx,
        );
        let owner = memory::acting_principal_owner(&acting);
        let profile_id = memory::acting_profile_id(&acting);
        let actor_address = memory::acting_actor_address(&acting);
        let sub_agent_id = memory::acting_sub_agent_id(&acting);
        let organization_id = memory::acting_organization_id(&acting);
        let action_identity_class = memory::acting_identity_class(&acting);
        
        // Check if platform is approved
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), EUnauthorized);

        // Check if the actor is blocked by the post creator
        assert!(!block_list::is_blocked(block_list_registry, parent_post.owner, actor_address), EUnauthorized);
        
        // Check if comments are allowed on the parent post
        assert!(allow_comments(parent_post), ECommentsNotAllowed);
        
        // Validate content length using config
        assert!(string::length(&content) <= config.max_content_length, EContentTooLarge);
        
        // Validate metadata size if provided
        if (option::is_some(&metadata_json)) {
            let metadata_ref = option::borrow(&metadata_json);
            assert!(string::length(metadata_ref) <= config.max_metadata_size, EContentTooLarge);
        };
        
        // Convert and validate media URLs if provided
        let media_option = if (option::is_some(&media_urls)) {
            let url_strings = option::extract(&mut media_urls);
            
            // Validate media URLs count using config
            assert!(vector::length(&url_strings) <= config.max_media_urls, ETooManyMediaUrls);
            
            // Convert string URLs to Url objects
            let mut urls = vector::empty<Url>();
            let mut i = 0;
            let len = vector::length(&url_strings);
            while (i < len) {
                let url_string = vector::borrow(&url_strings, i);
                let url_bytes = string::as_bytes(url_string);
                vector::push_back(&mut urls, url::new_unsafe_from_bytes(*url_bytes));
                i = i + 1;
            };
            option::some(urls)
        } else {
            option::none<vector<Url>>()
        };
        
        // Validate mentions if provided using config
        if (option::is_some(&mentions)) {
            let mentions_ref = option::borrow(&mentions);
            assert!(vector::length(mentions_ref) <= config.max_mentions, EContentTooLarge);
        };
        
        // Get parent post ID
        let parent_post_id = object::uid_to_address(&parent_post.id);
        
        // Create a proper Comment object instead of reusing post structure
        let mut comment = Comment {
            id: object::new(ctx),
            post_id: parent_post_id,
            parent_comment_id,
            owner,
            profile_id,
            content,
            media: media_option,
            mentions,
            metadata_json,
            created_at: clock::timestamp_ms(clock),
            reaction_count: 0,
            comment_count: 0,
            repost_count: 0,
            tips_received: 0,
            removed_from_platform: false,
            user_reactions: table::new(ctx),
            reaction_counts: table::new(ctx),
            version: upgrade::current_version(),
        };

        attach_comment_attribution(
            &mut comment,
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
        );
        
        // Get comment ID before sharing
        let comment_id = object::uid_to_address(&comment.id);
        
        // Increment the parent post's comment count with overflow protection
        // Stop incrementing at max but allow commenting to continue
        if (parent_post.comment_count < MAX_U64) {
            assert!(parent_post.comment_count <= MAX_U64 - 1, EOverflow);
            parent_post.comment_count = parent_post.comment_count + 1;
        };
        
        // Emit comment created event
        event::emit(CommentCreatedEvent {
            comment_id,
            post_id: parent_post_id,
            parent_comment_id,
            owner,
            profile_id,
            content,
            mentions,
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
        });
        
        // Share the comment object
        transfer::share_object(comment);
        
        // Return the comment ID to the caller
        comment_id
    }

    /// Create a repost or quote repost depending on provided parameters
    /// If content is provided, it's treated as a quote repost
    /// If content is empty/none, it's treated as a standard repost
    public fun create_repost(
        registry: &UsernameRegistry,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        original_post: &mut Post,
        mut content: Option<String>,
        media_asset_ids: vector<ID>,
        mut media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        allow_comments: Option<bool>,
        allow_reactions: Option<bool>,
        allow_reposts: Option<bool>,
        allow_quotes: Option<bool>,
        allow_tips: Option<bool>,
        enable_spt: Option<bool>,
        enable_spot: Option<bool>,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let acting = resolve_social_actor(
            memory_config,
            registry,
            platform,
            block_list_registry,
            memory_account,
            memory::cap_post_publish(),
            0,
            clock,
            ctx,
        );
        let owner = memory::acting_principal_owner(&acting);
        let profile_id = memory::acting_profile_id(&acting);
        let actor_address = memory::acting_actor_address(&acting);
        let sub_agent_id = memory::acting_sub_agent_id(&acting);
        let organization_id = memory::acting_organization_id(&acting);
        let action_identity_class = memory::acting_identity_class(&acting);
        
        // Check if platform is approved
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), EUnauthorized);
        
        let original_post_id = object::uid_to_address(&original_post.id);
        
        // Determine if this is a quote repost or standard repost
        let is_quote_repost = option::is_some(&content) && string::length(option::borrow(&content)) > 0;
        
        // Check post permissions directly
        if (is_quote_repost) {
            // For quote reposts, check if quoting is allowed
            assert!(allow_quotes(original_post), EQuotesNotAllowed);
        } else {
            // For regular reposts, check if reposting is allowed
            assert!(allow_reposts(original_post), ERepostsNotAllowed);
        };
        
        // Initialize content string
        let content_string = if (is_quote_repost) {
            // Validate content length for quote reposts
            let content_value = option::extract(&mut content);
            // Use config value instead of hardcoded constant
            assert!(string::length(&content_value) <= config.max_content_length, EContentTooLarge);
            content_value
        } else {
            // Empty string for standard reposts
            string::utf8(b"")
        };
        
        // Validate and process media URLs if provided
        let media_option = if (option::is_some(&media_urls)) {
            let url_strings = option::extract(&mut media_urls);
            
            // Validate media URLs count
            assert!(vector::length(&url_strings) <= config.max_media_urls, ETooManyMediaUrls);
            
            // Convert string URLs to Url objects
            let mut urls = vector::empty<Url>();
            let mut i = 0;
            let len = vector::length(&url_strings);
            while (i < len) {
                let url_string = vector::borrow(&url_strings, i);
                let url_bytes = string::as_bytes(url_string);
                vector::push_back(&mut urls, url::new_unsafe_from_bytes(*url_bytes));
                i = i + 1;
            };
            option::some(urls)
        } else {
            option::none<vector<Url>>()
        };
        
        // Validate metadata size if provided
        if (option::is_some(&metadata_json)) {
            let metadata_ref = option::borrow(&metadata_json);
            assert!(string::length(metadata_ref) <= config.max_metadata_size, EContentTooLarge);
        };
        
        // Validate mentions if provided
        if (option::is_some(&mentions)) {
            let mentions_ref = option::borrow(&mentions);
            assert!(vector::length(mentions_ref) <= config.max_mentions, EContentTooLarge);
        };
        
        // Create repost as post with appropriate type
        let post_type = if (is_quote_repost) {
            string::utf8(POST_TYPE_QUOTE_REPOST)
        } else {
            string::utf8(POST_TYPE_REPOST)
        };
        
        // For standard reposts, also create a Repost object
        if (!is_quote_repost) {
            let repost = Repost {
                id: object::new(ctx),
                original_id: original_post_id,
                is_original_post: true,
                owner,
                profile_id,
                created_at: clock::timestamp_ms(clock),
                version: upgrade::current_version(),
            };
            
            // Get repost ID before sharing
            let repost_id = object::uid_to_address(&repost.id);
            
            // Emit repost event before sharing
            event::emit(RepostEvent {
                repost_id,
                original_id: original_post_id,
                is_original_post: true,
                owner,
                profile_id,
                actor_address,
                sub_agent_id,
                organization_id,
                action_identity_class,
            });
            
            // Share repost object
            transfer::share_object(repost);
        };
        
        // Increment original post repost count
        assert!(original_post.repost_count <= MAX_U64 - 1, EOverflow);
        original_post.repost_count = original_post.repost_count + 1;
        
        // Set defaults for optional boolean parameters
        let final_allow_comments = if (option::is_some(&allow_comments)) {
            *option::borrow(&allow_comments)
        } else {
            true // Default to allowing comments
        };
        let final_allow_reactions = if (option::is_some(&allow_reactions)) {
            *option::borrow(&allow_reactions)
        } else {
            true // Default to allowing reactions
        };
        let final_allow_reposts = if (option::is_some(&allow_reposts)) {
            *option::borrow(&allow_reposts)
        } else {
            true // Default to allowing reposts
        };
        let final_allow_quotes = if (option::is_some(&allow_quotes)) {
            *option::borrow(&allow_quotes)
        } else {
            true // Default to allowing quotes
        };
        let final_allow_tips = if (option::is_some(&allow_tips)) {
            *option::borrow(&allow_tips)
        } else {
            true // Default to allowing tips
        };
        
        // Set defaults for feature flags (default to opt-out - users must explicitly opt-in)
        let final_enable_spt = if (option::is_some(&enable_spt)) {
            *option::borrow(&enable_spt)
        } else {
            false // Default to opt-out (user must explicitly opt-in)
        };
        assert!(!final_enable_spt, ESptRequiresDedicatedCreate);
        // enable_spot retained for entry-signature compatibility; SPoT is always-on.
        let _ = enable_spot;

        // Convert media URLs to strings for event (before moving media_option)
        let media_urls_for_event = convert_urls_to_strings(&media_option);
        let media_asset_ids_for_event = media_asset_ids;

        let post = create_post_internal(
            owner,
            profile_id,
            platform_id,
            content_string,
            media_asset_ids,
            media_option,
            mentions,
            metadata_json,
            post_type,
            option::some(original_post_id),
            final_allow_comments,
            final_allow_reactions,
            final_allow_reposts,
            final_allow_quotes,
            final_allow_tips,
            PostAccess::Public,
            option::none(), // promotion_id
            false,
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
            clock,
            ctx
        );
        
        let permissions_for_event = permissions_bitfield(
            final_allow_comments,
            final_allow_reactions,
            final_allow_reposts,
            final_allow_quotes,
            final_allow_tips,
        );

        emit_post_media_asset_usages(&post, clock);
        let post_id = share_post(post);

        event::emit(PostCreatedEvent {
            post_id,
            owner,
            profile_id,
            platform_id,
            permissions: permissions_for_event,
            content: content_string,
            post_type,
            parent_post_id: option::some(original_post_id),
            mentions,
            media_asset_ids: media_asset_ids_for_event,
            media_urls: media_urls_for_event,
            metadata_json,
            access: PostAccess::Public,
            promotion_id: option::none(),
            composition_status: media_asset::composition_none(),
            monetization_status: media_asset::monetization_none(),
            enable_spt: false,
            spt_id: option::none(),
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
        });
    }

    /// Delete a post owned by the caller
    public fun delete_post(
        post: Post,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(sender == post.owner, EUnauthorized);
        let post_id_addr = object::uid_to_address(&post.id);
        
        // Emit event for the post deletion
        event::emit(PostDeletedEvent {
            post_id: post_id_addr,
            owner: post.owner,
            profile_id: post.profile_id,
            post_type: post.post_type,
            deleted_at: clock::timestamp_ms(clock)
        });
        
        // Extract UID to delete the post object
        let Post {
            id,
            owner: _,
            profile_id: _,
            platform_id: _,
            content: _,
            media_asset_ids: _,
            media: _,
            mentions: _,
            metadata_json: _,
            post_type: _,
            parent_post_id: _,
            created_at: _,
            reaction_count: _,
            comment_count: _,
            repost_count: _,
            tips_received: _,
            removed_from_platform: _,
            user_reactions,
            reaction_counts,
            permissions: _,
            composition_status: _,
            monetization_status: _,
            composition_badge: _,
            access: _,
            promotion_id: _,
            enable_spt: _,
            spt_id: _,
            version: _,
        } = post;
        
        // Clean up associated data structures
        table::drop(user_reactions);
        table::drop(reaction_counts);
        
        // Delete the post object
        object::delete(id);
    }
    
    /// Delete a comment owned by the caller
    public fun delete_comment(
        post: &mut Post,
        comment: Comment,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let sender = tx_context::sender(ctx);
        assert!(sender == comment.owner, EUnauthorized);
        
        // Verify the comment belongs to this post
        let comment_post_id = comment.post_id;
        let post_id = object::uid_to_address(&post.id);
        assert!(comment_post_id == post_id, EPostNotFound);
        
        // Decrement the post's comment count
        post.comment_count = post.comment_count - 1;
        
        // Emit event for the comment deletion
        event::emit(CommentDeletedEvent {
            comment_id: object::uid_to_address(&comment.id),
            post_id,
            owner: comment.owner,
            profile_id: comment.profile_id,
            deleted_at: clock::timestamp_ms(clock)
        });
        
        // Extract UID to delete the comment object
        let Comment {
            id,
            post_id: _,
            parent_comment_id: _,
            owner: _,
            profile_id: _,
            content: _,
            media: _,
            mentions: _,
            metadata_json: _,
            created_at: _,
            reaction_count: _,
            comment_count: _,
            repost_count: _,
            tips_received: _,
            removed_from_platform: _,
            user_reactions,
            reaction_counts,
            version: _,
        } = comment;
        
        // Clean up associated data structures
        table::drop(user_reactions);
        table::drop(reaction_counts);
        
        // Delete the comment object
        object::delete(id);
    }

    /// React to a post with a specific reaction (emoji or text)
    /// If the user already has the exact same reaction, it will be removed (toggle behavior)
    public fun react_to_post(
        registry: &UsernameRegistry,
        post: &mut Post,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        memory_account: &MemoryAccount,
        reaction: String,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let acting = resolve_social_actor(
            memory_config,
            registry,
            platform,
            block_list_registry,
            memory_account,
            memory::cap_react(),
            0,
            clock,
            ctx,
        );
        let actor_address = memory::acting_actor_address(&acting);
        let principal_owner = memory::acting_principal_owner(&acting);
        let sub_agent_id = memory::acting_sub_agent_id(&acting);
        let organization_id = memory::acting_organization_id(&acting);
        let action_identity_class = memory::acting_identity_class(&acting);
        
        // Check if platform is approved
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), EUnauthorized);
        
        // Validate reaction length using config
        assert!(string::length(&reaction) <= config.max_reaction_length, EReactionContentTooLong);
        
        // Check if reactions are allowed on this post
        assert!(allow_reactions(post), EReactionsNotAllowed);
        
        // Check if user already reacted to the post
        if (table::contains(&post.user_reactions, actor_address)) {
            // Get the previous reaction
            let previous_reaction = *table::borrow(&post.user_reactions, actor_address);
            
            // If the reaction is the same, remove it (toggle behavior)
            if (reaction == previous_reaction) {
                // Remove user's reaction
                table::remove(&mut post.user_reactions, actor_address);
                
                // Decrease count for this reaction type
                let count = *table::borrow(&post.reaction_counts, reaction);
                if (count <= 1) {
                    table::remove(&mut post.reaction_counts, reaction);
                } else {
                    *table::borrow_mut(&mut post.reaction_counts, reaction) = count - 1;
                };
                
                // Decrement post reaction count with underflow protection
                assert!(post.reaction_count > 0, EOverflow);
                post.reaction_count = post.reaction_count - 1;
                
                // Emit remove reaction event
                event::emit(RemoveReactionEvent {
                    object_id: object::uid_to_address(&post.id),
                    user: actor_address,
                    reaction,
                    is_post: true,
                    principal_owner,
                    actor_address,
                    sub_agent_id,
                    organization_id,
                    action_identity_class,
                });
                
                return
            };
            
            // Different reaction, update existing one
            // Decrease count for previous reaction
            let previous_count = *table::borrow(&post.reaction_counts, previous_reaction);
            if (previous_count <= 1) {
                table::remove(&mut post.reaction_counts, previous_reaction);
            } else {
                *table::borrow_mut(&mut post.reaction_counts, previous_reaction) = previous_count - 1;
            };
            
            // Update user's reaction
            *table::borrow_mut(&mut post.user_reactions, actor_address) = reaction;
        } else {
            // New reaction from this user
            table::add(&mut post.user_reactions, actor_address, reaction);

            // Increment post reaction count
            assert!(post.reaction_count <= MAX_U64 - 1, EOverflow);
            post.reaction_count = post.reaction_count + 1;
        };
        
        // Increment count for the reaction
        if (table::contains(&post.reaction_counts, reaction)) {
            let count = *table::borrow(&post.reaction_counts, reaction);
            assert!(count <= MAX_U64 - 1, EOverflow);
            *table::borrow_mut(&mut post.reaction_counts, reaction) = count + 1;
        } else {
            table::add(&mut post.reaction_counts, reaction, 1);
        };
        
        // Emit reaction event
        event::emit(ReactionEvent {
            object_id: object::uid_to_address(&post.id),
            user: actor_address,
            reaction,
            is_post: true,
            principal_owner,
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
        });
    }

    /// Tip a post creator with coin type `T`. When manifest entries use escrow payout mode for this
    /// `amount`, `beneficiary_vault` must match the beneficiary's shared vault.
    public fun tip_post<T>(
        post: &mut Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        coins: &mut Coin<T>,
        amount: u64,
        min_vault_deposit_amount: u64,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        assert!(amount > 0, EInvalidTipAmount);
        assert_tip_spend_limit(memory_account, amount, ctx);
        let tipper = tx_context::sender(ctx);
        assert!(tipper != post.owner, ESelfTipping);
        assert!(
            string::utf8(POST_TYPE_REPOST) != post.post_type &&
            string::utf8(POST_TYPE_QUOTE_REPOST) != post.post_type,
            EInvalidPostType
        );
        assert!(allow_tips(post), ETipsNotAllowed);
        let post_owner_addr = post.owner;
        let post_oid = object::uid_to_address(&post.id);
        let actual_received = apply_revenue_manifest_coin<T>(
            post,
            beneficiary_vault,
            post_owner_addr,
            amount,
            coins,
            tipper,
            post_oid,
            true,
            min_vault_deposit_amount,
            clock,
            ctx
        );
        assert!(post.tips_received <= MAX_U64 - actual_received, EOverflow);
        post.tips_received = post.tips_received + actual_received;
        if (actual_received > 0) {
            event::emit(TipEvent {
                object_id: post_oid,
                from: tipper,
                to: post_owner_addr,
                amount: actual_received,
                coin_type: type_name::with_defining_ids<T>(),
                is_post: true,
            });
        };
    }

    /// Like [`tip_post`] but without a `PoCBeneficiaryVault` argument. Only for posts where
    /// [`tip_post_requires_beneficiary_vault_for_amount`] is false for this `amount`.
    /// If an escrow deposit is required, aborts with [`ETipPostRequiresBeneficiaryVault`].
    public fun tip_post_simple<T>(
        post: &mut Post,
        coins: &mut Coin<T>,
        amount: u64,
        memory_account: &MemoryAccount,
        ctx: &mut TxContext
    ) {
        assert!(amount > 0, EInvalidTipAmount);
        assert!(
            !tip_post_requires_beneficiary_vault_for_amount(post, amount),
            ETipPostRequiresBeneficiaryVault
        );
        assert_tip_spend_limit(memory_account, amount, ctx);
        let tipper = tx_context::sender(ctx);
        assert!(tipper != post.owner, ESelfTipping);
        assert!(
            string::utf8(POST_TYPE_REPOST) != post.post_type &&
            string::utf8(POST_TYPE_QUOTE_REPOST) != post.post_type,
            EInvalidPostType
        );
        assert!(allow_tips(post), ETipsNotAllowed);
        let post_owner_addr = post.owner;
        let post_oid = object::uid_to_address(&post.id);
        let actual_received = apply_revenue_manifest_coin_without_beneficiary_vault<T>(
            post,
            post_owner_addr,
            amount,
            coins,
            tipper,
            post_oid,
            true,
            ctx
        );
        assert!(post.tips_received <= MAX_U64 - actual_received, EOverflow);
        post.tips_received = post.tips_received + actual_received;
        if (actual_received > 0) {
            event::emit(TipEvent {
                object_id: post_oid,
                from: tipper,
                to: post_owner_addr,
                amount: actual_received,
                coin_type: type_name::with_defining_ids<T>(),
                is_post: true,
            });
        };
    }

    /// Manifest-based revenue split for tips and fees on the creator-attributable pool.
    fun apply_revenue_manifest_coin<T>(
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        intended_recipient: address,
        amount: u64,
        coins: &mut Coin<T>,
        tipper: address,
        object_id: address,
        is_post_event: bool,
        min_vault_deposit_amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ): u64 {
        if (intended_recipient != post.owner) {
            let tip_coins = coin::split(coins, amount, ctx);
            transfer::public_transfer(tip_coins, intended_recipient);
            return amount
        };

        if (!monetization_enabled(post)) {
            let tip_coins = coin::split(coins, amount, ctx);
            transfer::public_transfer(tip_coins, intended_recipient);
            return amount
        };

        let manifest_opt = revenue_manifest(post);
        if (option::is_none(&manifest_opt)) {
            let tip_coins = coin::split(coins, amount, ctx);
            transfer::public_transfer(tip_coins, intended_recipient);
            return amount
        };

        let manifest = option::borrow(&manifest_opt);
        let entries = media_asset::manifest_entries(manifest);
        let len = vector::length(entries);
        let bps_total = media_asset::manifest_bps_total();
        let coin_type = type_name::with_defining_ids<T>();

        let mut tip_coins = coin::split(coins, amount, ctx);
        let mut owner_received = 0u64;
        let mut i = 0;
        while (i < len) {
            let e = vector::borrow(entries, i);
            let slice = (amount * media_asset::manifest_entry_share_bps(e)) / bps_total;
            if (slice > 0) {
                let pay_coins = coin::split(&mut tip_coins, slice, ctx);
                if (media_asset::manifest_entry_payout_mode(e) == media_asset::payout_escrow()) {
                    assert!(poc_vault::beneficiary_address(beneficiary_vault) == media_asset::manifest_entry_beneficiary(e), EWrongBeneficiaryVault);
                    poc_vault::deposit_coin<T>(
                        beneficiary_vault,
                        media_asset::manifest_entry_beneficiary(e),
                        pay_coins,
                        option::some(object_id),
                        min_vault_deposit_amount,
                        clock,
                        ctx
                    );
                } else {
                    transfer::public_transfer(pay_coins, media_asset::manifest_entry_beneficiary(e));
                    event::emit(TipEvent {
                        object_id,
                        from: tipper,
                        to: media_asset::manifest_entry_beneficiary(e),
                        amount: slice,
                        coin_type,
                        is_post: is_post_event,
                    });
                };
                if (media_asset::manifest_entry_beneficiary(e) == intended_recipient) {
                    owner_received = owner_received + slice;
                };
            };
            i = i + 1;
        };

        let remainder = coin::value(&tip_coins);
        if (remainder > 0) {
            transfer::public_transfer(tip_coins, intended_recipient);
            owner_received = owner_received + remainder;
        } else {
            coin::destroy_zero(tip_coins);
        };
        owner_received
    }

    /// Same as [`apply_revenue_manifest_coin`] for tip paths that never deposit into escrow for this `amount`.
    fun apply_revenue_manifest_coin_without_beneficiary_vault<T>(
        post: &Post,
        intended_recipient: address,
        amount: u64,
        coins: &mut Coin<T>,
        tipper: address,
        object_id: address,
        is_post_event: bool,
        ctx: &mut TxContext
    ): u64 {
        if (intended_recipient != post.owner) {
            let tip_coins = coin::split(coins, amount, ctx);
            transfer::public_transfer(tip_coins, intended_recipient);
            return amount
        };

        if (!monetization_enabled(post)) {
            let tip_coins = coin::split(coins, amount, ctx);
            transfer::public_transfer(tip_coins, intended_recipient);
            return amount
        };

        let manifest_opt = revenue_manifest(post);
        if (option::is_none(&manifest_opt)) {
            let tip_coins = coin::split(coins, amount, ctx);
            transfer::public_transfer(tip_coins, intended_recipient);
            return amount
        };

        let manifest = option::borrow(&manifest_opt);
        let entries = media_asset::manifest_entries(manifest);
        let len = vector::length(entries);
        let bps_total = media_asset::manifest_bps_total();
        let coin_type = type_name::with_defining_ids<T>();

        let mut tip_coins = coin::split(coins, amount, ctx);
        let mut owner_received = 0u64;
        let mut i = 0;
        while (i < len) {
            let e = vector::borrow(entries, i);
            let slice = (amount * media_asset::manifest_entry_share_bps(e)) / bps_total;
            if (slice > 0) {
                if (media_asset::manifest_entry_payout_mode(e) == media_asset::payout_escrow()) {
                    abort ETipPostRequiresBeneficiaryVault
                };
                let pay_coins = coin::split(&mut tip_coins, slice, ctx);
                transfer::public_transfer(pay_coins, media_asset::manifest_entry_beneficiary(e));
                event::emit(TipEvent {
                    object_id,
                    from: tipper,
                    to: media_asset::manifest_entry_beneficiary(e),
                    amount: slice,
                    coin_type,
                    is_post: is_post_event,
                });
                if (media_asset::manifest_entry_beneficiary(e) == intended_recipient) {
                    owner_received = owner_received + slice;
                };
            };
            i = i + 1;
        };

        let remainder = coin::value(&tip_coins);
        if (remainder > 0) {
            transfer::public_transfer(tip_coins, intended_recipient);
            owner_received = owner_received + remainder;
        } else {
            coin::destroy_zero(tip_coins);
        };
        owner_received
    }

    // === Phase 4 — Application enforcement (bindings, decisions, denials) ===

    fun embedded_bindings(post: &Post): vector<EmbeddedAssetBinding> {
        if (df::exists_with_type<vector<u8>, vector<EmbeddedAssetBinding>>(&post.id, EMBEDDED_BINDINGS_DF_KEY)) {
            *df::borrow(&post.id, EMBEDDED_BINDINGS_DF_KEY)
        } else {
            vector[]
        }
    }

    fun usage_decisions(post: &Post): vector<UsageDecisionSnapshot> {
        if (df::exists_with_type<vector<u8>, vector<UsageDecisionSnapshot>>(&post.id, USAGE_DECISIONS_DF_KEY)) {
            *df::borrow(&post.id, USAGE_DECISIONS_DF_KEY)
        } else {
            vector[]
        }
    }

    fun usage_denials(post: &Post): vector<ContainerUsageDenial> {
        if (df::exists_with_type<vector<u8>, vector<ContainerUsageDenial>>(&post.id, USAGE_DENIALS_DF_KEY)) {
            *df::borrow(&post.id, USAGE_DENIALS_DF_KEY)
        } else {
            vector[]
        }
    }

    fun set_embedded_bindings(post: &mut Post, bindings: vector<EmbeddedAssetBinding>) {
        if (df::exists_with_type<vector<u8>, vector<EmbeddedAssetBinding>>(&post.id, EMBEDDED_BINDINGS_DF_KEY)) {
            let stored = df::borrow_mut(&mut post.id, EMBEDDED_BINDINGS_DF_KEY);
            *stored = bindings;
        } else {
            df::add(&mut post.id, EMBEDDED_BINDINGS_DF_KEY, bindings);
        };
    }

    fun set_usage_decisions(post: &mut Post, decisions: vector<UsageDecisionSnapshot>) {
        if (df::exists_with_type<vector<u8>, vector<UsageDecisionSnapshot>>(&post.id, USAGE_DECISIONS_DF_KEY)) {
            let stored = df::borrow_mut(&mut post.id, USAGE_DECISIONS_DF_KEY);
            *stored = decisions;
        } else {
            df::add(&mut post.id, USAGE_DECISIONS_DF_KEY, decisions);
        };
    }

    fun set_usage_denials(post: &mut Post, denials: vector<ContainerUsageDenial>) {
        if (df::exists_with_type<vector<u8>, vector<ContainerUsageDenial>>(&post.id, USAGE_DENIALS_DF_KEY)) {
            let stored = df::borrow_mut(&mut post.id, USAGE_DENIALS_DF_KEY);
            *stored = denials;
        } else {
            df::add(&mut post.id, USAGE_DENIALS_DF_KEY, denials);
        };
    }

    fun find_binding_index(bindings: &vector<EmbeddedAssetBinding>, binding_id: u64): Option<u64> {
        let len = vector::length(bindings);
        let mut i = 0;
        while (i < len) {
            if (vector::borrow(bindings, i).binding_id == binding_id) {
                return option::some(i)
            };
            i = i + 1;
        };
        option::none()
    }

    fun borrow_binding(bindings: &vector<EmbeddedAssetBinding>, binding_id: u64): &EmbeddedAssetBinding {
        let idx = option::borrow(&find_binding_index(bindings, binding_id));
        vector::borrow(bindings, *idx)
    }

    fun upsert_binding(bindings: &mut vector<EmbeddedAssetBinding>, binding: EmbeddedAssetBinding) {
        let binding_id = binding.binding_id;
        if (option::is_some(&find_binding_index(bindings, binding_id))) {
            let idx = *option::borrow(&find_binding_index(bindings, binding_id));
            *vector::borrow_mut(bindings, idx) = binding;
        } else {
            vector::push_back(bindings, binding);
        };
    }

    fun upsert_decision(decisions: &mut vector<UsageDecisionSnapshot>, snapshot: UsageDecisionSnapshot) {
        let binding_id = snapshot.binding_id;
        let len = vector::length(decisions);
        let mut i = 0;
        let mut found = false;
        while (i < len) {
            if (vector::borrow(decisions, i).binding_id == binding_id) {
                *vector::borrow_mut(decisions, i) = snapshot;
                found = true;
                break
            };
            i = i + 1;
        };
        if (!found) {
            vector::push_back(decisions, snapshot);
        };
    }

    fun upsert_denial(denials: &mut vector<ContainerUsageDenial>, denial: ContainerUsageDenial) {
        let binding_id = denial.binding_id;
        let len = vector::length(denials);
        let mut i = 0;
        let mut found = false;
        while (i < len) {
            if (vector::borrow(denials, i).binding_id == binding_id) {
                *vector::borrow_mut(denials, i) = denial;
                found = true;
                break
            };
            i = i + 1;
        };
        if (!found) {
            vector::push_back(denials, denial);
        };
    }

    fun remove_denial(denials: &mut vector<ContainerUsageDenial>, binding_id: u64, denial_scope: u8) {
        let mut out = vector[];
        let len = vector::length(denials);
        let mut i = 0;
        while (i < len) {
            let d = vector::borrow(denials, i);
            if (!(d.binding_id == binding_id && d.denial_scope == denial_scope)) {
                vector::push_back(&mut out, *d);
            };
            i = i + 1;
        };
        *denials = out;
    }

    fun apply_denials_to_snapshot(
        policy_playback: bool,
        denials: &vector<ContainerUsageDenial>,
        binding_id: u64,
    ): bool {
        let mut playback = policy_playback;
        let len = vector::length(denials);
        let mut i = 0;
        while (i < len) {
            let d = vector::borrow(denials, i);
            if (d.binding_id == binding_id && d.denial_scope == DENIAL_SCOPE_PLAYBACK) {
                playback = false;
            };
            i = i + 1;
        };
        playback
    }

    fun evaluate_binding_policy(
        asset: &MediaAsset,
        usage_class: u8,
        clock: &Clock,
    ): (bool, u8, u64) {
        let version = if (option::is_some(&media_asset::resolved_policy_version(asset))) {
            *option::borrow(&media_asset::resolved_policy_version(asset))
        } else {
            0
        };
        let grant_playback = media_asset::rights_permits_usage(asset, usage_class, clock);
        if (!grant_playback) {
            return (false, REASON_NO_GRANT, version)
        };
        if (!media_asset::resolved_policy_permits_usage(asset, usage_class)) {
            return (false, REASON_RESOLVED_POLICY, version)
        };
        (true, REASON_ALLOWED, version)
    }

    fun refresh_composition_status_from_decisions(post: &mut Post) {
        let decisions = usage_decisions(post);
        let len = vector::length(&decisions);
        if (len == 0) {
            return
        };
        let mut any_restricted = false;
        let mut all_restricted = true;
        let mut i = 0;
        while (i < len) {
            let d = vector::borrow(&decisions, i);
            let restricted = !d.playback_permitted;
            if (restricted) {
                any_restricted = true;
            } else {
                all_restricted = false;
            };
            i = i + 1;
        };
        if (all_restricted) {
            post.composition_status = media_asset::composition_invalid();
        } else if (any_restricted) {
            post.composition_status = media_asset::composition_partially_restricted();
        };
    }

    fun validate_binding(binding: &EmbeddedAssetBinding) {
        assert!(binding.binding_id > 0, EInvalidBinding);
        media_asset::assert_valid_usage_class(binding.usage_class);
        // Oracle slot indices — must be readable at validation time for event/indexer ABI parity.
        assert!(binding.stem <= 255, EInvalidBinding);
        assert!(binding.media_component <= 255, EInvalidBinding);
        media_asset::assert_valid_optional_evidence_commitment(&binding.evidence_commitment);
    }

    fun write_usage_decision_for_binding(
        post: &mut Post,
        asset: &MediaAsset,
        binding_id: u64,
        clock: &Clock,
        timestamp: u64,
    ): UsageDecisionSnapshot {
        let bindings = embedded_bindings(post);
        assert!(option::is_some(&find_binding_index(&bindings, binding_id)), EBindingNotFound);
        let binding = borrow_binding(&bindings, binding_id);
        assert!(binding.source_asset_id == object::id(asset), EBindingAssetMismatch);
        let (policy_playback, reason_code, policy_version) =
            evaluate_binding_policy(asset, binding.usage_class, clock);
        let denials = usage_denials(post);
        let playback = apply_denials_to_snapshot(policy_playback, &denials, binding_id);
        let reason = if (playback != policy_playback) {
            REASON_DENIAL
        } else {
            reason_code
        };
        let snapshot = UsageDecisionSnapshot {
            binding_id,
            policy_playback_permitted: policy_playback,
            playback_permitted: playback,
            policy_reason_code: reason,
            policy_version_at_decision: policy_version,
        };
        let mut decisions = usage_decisions(post);
        upsert_decision(&mut decisions, snapshot);
        set_usage_decisions(post, decisions);
        refresh_composition_status_from_decisions(post);
        event::emit(UsageDecisionRefreshedEvent {
            post_id: get_id_address(post),
            binding_id,
            policy_playback_permitted: policy_playback,
            playback_permitted: playback,
            policy_reason_code: reason,
            policy_version_at_decision: policy_version,
            timestamp,
        });
        snapshot
    }

    /// Oracle records embedded asset bindings detected during composition analysis.
    public fun record_embedded_bindings(
        oracle: address,
        post: &mut Post,
        bindings: vector<EmbeddedAssetBinding>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == oracle, EUnauthorized);
        let timestamp = clock::timestamp_ms(clock);
        let mut stored = embedded_bindings(post);
        let len = vector::length(&bindings);
        let mut i = 0;
        while (i < len) {
            let b = vector::borrow(&bindings, i);
            validate_binding(b);
            upsert_binding(&mut stored, *b);
            i = i + 1;
        };
        set_embedded_bindings(post, stored);
        event::emit(EmbeddedBindingRecordedEvent {
            post_id: get_id_address(post),
            bindings: embedded_bindings(post),
            timestamp,
        });
    }

    /// Oracle refreshes a usage decision snapshot for one binding (requires live `MediaAsset`).
    public fun refresh_post_asset_usage_decision(
        oracle: address,
        post: &mut Post,
        asset: &MediaAsset,
        binding_id: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == oracle, EUnauthorized);
        let timestamp = clock::timestamp_ms(clock);
        let _ = write_usage_decision_for_binding(post, asset, binding_id, clock, timestamp);
    }

    /// Oracle submits a candidate revenue manifest for indexer materialization.
    public fun submit_candidate_revenue_manifest(
        oracle: address,
        post: &mut Post,
        manifest: RevenueManifest,
        manifest_version: u64,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(tx_context::sender(ctx) == oracle, EUnauthorized);
        media_asset::validate_revenue_manifest(&manifest);
        let timestamp = clock::timestamp_ms(clock);
        let entries = *media_asset::manifest_entries(&manifest);
        if (df::exists_with_type<vector<u8>, RevenueManifest>(&post.id, CANDIDATE_MANIFEST_DF_KEY)) {
            let stored = df::borrow_mut(&mut post.id, CANDIDATE_MANIFEST_DF_KEY);
            *stored = manifest;
        } else {
            df::add(&mut post.id, CANDIDATE_MANIFEST_DF_KEY, manifest);
        };
        event::emit(CandidateManifestSubmittedEvent {
            post_id: get_id_address(post),
            manifest_version,
            entries_json: entries,
            timestamp,
        });
    }

    /// Rights holder denies container-scoped usage on this post for one binding.
    public entry fun deny_container_usage(
        post: &mut Post,
        asset: &MediaAsset,
        binding_id: u64,
        denial_scope: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(denial_scope == DENIAL_SCOPE_PLAYBACK, EInvalidDenialScope);
        let sender = tx_context::sender(ctx);
        assert!(media_asset::can_update_rights(asset, sender), EUnauthorized);
        let bindings = embedded_bindings(post);
        assert!(option::is_some(&find_binding_index(&bindings, binding_id)), EBindingNotFound);
        let binding = borrow_binding(&bindings, binding_id);
        assert!(binding.source_asset_id == object::id(asset), EBindingAssetMismatch);
        let timestamp = clock::timestamp_ms(clock);
        let mut denials = usage_denials(post);
        upsert_denial(&mut denials, ContainerUsageDenial { binding_id, denial_scope });
        set_usage_denials(post, denials);
        let _ = write_usage_decision_for_binding(post, asset, binding_id, clock, timestamp);
        event::emit(ContainerUsageDeniedEvent {
            post_id: get_id_address(post),
            binding_id,
            denial_scope,
            timestamp,
        });
    }

    /// Rights holder lifts a container-scoped denial for one binding and scope.
    public entry fun lift_container_usage_denial(
        post: &mut Post,
        asset: &MediaAsset,
        binding_id: u64,
        denial_scope: u8,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert!(denial_scope == DENIAL_SCOPE_PLAYBACK, EInvalidDenialScope);
        let sender = tx_context::sender(ctx);
        assert!(media_asset::can_update_rights(asset, sender), EUnauthorized);
        let bindings = embedded_bindings(post);
        assert!(option::is_some(&find_binding_index(&bindings, binding_id)), EBindingNotFound);
        let binding = borrow_binding(&bindings, binding_id);
        assert!(binding.source_asset_id == object::id(asset), EBindingAssetMismatch);
        let timestamp = clock::timestamp_ms(clock);
        let mut denials = usage_denials(post);
        remove_denial(&mut denials, binding_id, denial_scope);
        set_usage_denials(post, denials);
        let _ = write_usage_decision_for_binding(post, asset, binding_id, clock, timestamp);
        event::emit(ContainerUsageDenialLiftedEvent {
            post_id: get_id_address(post),
            binding_id,
            timestamp,
        });
    }

    /// Internal function to store composition analysis + optional revenue manifest (oracle-only).
    public(package) fun set_composition_result(
        post: &mut Post,
        composition_status: u8,
        monetization_status: u8,
        badge: CompositionBadgeSnapshot,
        analysis: CompositionAnalysis,
        manifest: Option<RevenueManifest>,
    ) {
        post.composition_status = composition_status;
        post.monetization_status = monetization_status;
        post.composition_badge = option::some(badge);

        if (df::exists_with_type<vector<u8>, CompositionAnalysis>(&post.id, COMPOSITION_ANALYSIS_DF_KEY)) {
            let stored = df::borrow_mut(&mut post.id, COMPOSITION_ANALYSIS_DF_KEY);
            *stored = analysis;
        } else {
            df::add(&mut post.id, COMPOSITION_ANALYSIS_DF_KEY, analysis);
        };

        if (option::is_some(&manifest)) {
            let m = *option::borrow(&manifest);
            media_asset::validate_revenue_manifest(&m);
            if (df::exists_with_type<vector<u8>, RevenueManifest>(&post.id, REVENUE_MANIFEST_DF_KEY)) {
                let stored = df::borrow_mut(&mut post.id, REVENUE_MANIFEST_DF_KEY);
                *stored = m;
            } else {
                df::add(&mut post.id, REVENUE_MANIFEST_DF_KEY, m);
            };
        } else if (df::exists_with_type<vector<u8>, RevenueManifest>(&post.id, REVENUE_MANIFEST_DF_KEY)) {
            df::remove<vector<u8>, RevenueManifest>(&mut post.id, REVENUE_MANIFEST_DF_KEY);
        };
    }

    /// Clears composition routing data on the post after dispute resolution (vault balances are unaffected).
    public(package) fun clear_composition_data(post: &mut Post) {
        post.composition_status = media_asset::composition_none();
        post.monetization_status = media_asset::monetization_none();
        post.composition_badge = option::none();
        if (df::exists_with_type<vector<u8>, CompositionAnalysis>(&post.id, COMPOSITION_ANALYSIS_DF_KEY)) {
            df::remove<vector<u8>, CompositionAnalysis>(&mut post.id, COMPOSITION_ANALYSIS_DF_KEY);
        };
        if (df::exists_with_type<vector<u8>, RevenueManifest>(&post.id, REVENUE_MANIFEST_DF_KEY)) {
            df::remove<vector<u8>, RevenueManifest>(&mut post.id, REVENUE_MANIFEST_DF_KEY);
        };
        if (df::exists_with_type<vector<u8>, vector<EmbeddedAssetBinding>>(&post.id, EMBEDDED_BINDINGS_DF_KEY)) {
            df::remove<vector<u8>, vector<EmbeddedAssetBinding>>(&mut post.id, EMBEDDED_BINDINGS_DF_KEY);
        };
        if (df::exists_with_type<vector<u8>, vector<UsageDecisionSnapshot>>(&post.id, USAGE_DECISIONS_DF_KEY)) {
            df::remove<vector<u8>, vector<UsageDecisionSnapshot>>(&mut post.id, USAGE_DECISIONS_DF_KEY);
        };
        if (df::exists_with_type<vector<u8>, vector<ContainerUsageDenial>>(&post.id, USAGE_DENIALS_DF_KEY)) {
            df::remove<vector<u8>, vector<ContainerUsageDenial>>(&mut post.id, USAGE_DENIALS_DF_KEY);
        };
        if (df::exists_with_type<vector<u8>, RevenueManifest>(&post.id, CANDIDATE_MANIFEST_DF_KEY)) {
            df::remove<vector<u8>, RevenueManifest>(&mut post.id, CANDIDATE_MANIFEST_DF_KEY);
        };
    }

    /// Legacy alias for dispute paths during migration.
    public(package) fun clear_poc_data(post: &mut Post) {
        clear_composition_data(post);
    }

    /// Increment after each successful composition dispute submission.
    public(package) fun increment_composition_disputes_submitted(post: &mut Post, max_disputes: u8) {
        let submitted = composition_disputes_submitted(post);
        assert!(submitted < max_disputes, EDisputeCapReached);
        let next = submitted + 1;
        if (df::exists_with_type<vector<u8>, u8>(&post.id, COMPOSITION_DISPUTES_DF_KEY)) {
            let count = df::borrow_mut(&mut post.id, COMPOSITION_DISPUTES_DF_KEY);
            *count = next;
        } else {
            df::add(&mut post.id, COMPOSITION_DISPUTES_DF_KEY, next);
        };
    }

    /// Legacy alias for dispute paths during migration.
    public(package) fun increment_poc_disputes_submitted(post: &mut Post, max_disputes: u8) {
        increment_composition_disputes_submitted(post, max_disputes);
    }

    /// Deposit manifest-routed fees into the beneficiary vault when the post uses escrow payout mode.
    public(package) fun deposit_coin_to_beneficiary_vault<T>(
        post: &Post,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        fee_coin: Coin<T>,
        min_vault_deposit_amount: u64,
        clock: &Clock,
        ctx: &TxContext
    ) {
        assert!(monetization_enabled(post), EMonetizationNotEnabled);
        let manifest_opt = revenue_manifest(post);
        assert!(option::is_some(&manifest_opt), EInvalidManifest);
        let manifest = option::borrow(&manifest_opt);
        let ben = poc_vault::beneficiary_address(beneficiary_vault);
        let entries = media_asset::manifest_entries(manifest);
        let len = vector::length(entries);
        let mut i = 0;
        let mut found_escrow = false;
        while (i < len) {
            let e = vector::borrow(entries, i);
            if (media_asset::manifest_entry_payout_mode(e) == media_asset::payout_escrow()
                && media_asset::manifest_entry_beneficiary(e) == ben) {
                found_escrow = true;
                break
            };
            i = i + 1;
        };
        assert!(found_escrow, EInvalidManifest);
        poc_vault::deposit_coin<T>(
            beneficiary_vault,
            ben,
            fee_coin,
            option::some(get_id_address(post)),
            min_vault_deposit_amount,
            clock,
            ctx
        );
    }

    /// Tip a repost or quote repost; splits per `PostConfig` between repost owner and original author.
    /// Pass the shared `PoCBeneficiaryVault` for each post when manifest entries use escrow payout mode.
    public fun tip_repost<T>(
        post: &mut Post,
        original_post: &mut Post,
        config: &PostConfig,
        vault_for_post: &mut PoCBeneficiaryVault,
        vault_for_original: &mut PoCBeneficiaryVault,
        coin: &mut Coin<T>,
        amount: u64,
        min_vault_deposit_amount: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let tipper = tx_context::sender(ctx);
        assert!(amount > 0 && coin::value(coin) >= amount, EInvalidTipAmount);
        assert!(tipper != post.owner, ESelfTipping);
        assert!(
            string::utf8(POST_TYPE_REPOST) == post.post_type ||
            string::utf8(POST_TYPE_QUOTE_REPOST) == post.post_type,
            EInvalidPostType
        );
        assert!(option::is_some(&post.parent_post_id), EInvalidParentReference);
        let parent_id = *option::borrow(&post.parent_post_id);
        assert!(parent_id == object::uid_to_address(&original_post.id), EInvalidParentReference);
        assert!(allow_tips(post), ETipsNotAllowed);
        assert!(allow_tips(original_post), ETipsNotAllowed);

        let ct = type_name::with_defining_ids<T>();

        if (post.owner == original_post.owner) {
            let po = post.owner;
            let poid = object::uid_to_address(&post.id);
            let actual_received = apply_revenue_manifest_coin<T>(
                post,
                vault_for_post,
                po,
                amount,
                coin,
                tipper,
                poid,
                true,
                min_vault_deposit_amount,
                clock,
                ctx
            );
            assert!(post.tips_received <= MAX_U64 - actual_received, EOverflow);
            post.tips_received = post.tips_received + actual_received;
            if (actual_received > 0) {
                event::emit(TipEvent {
                    object_id: poid,
                    from: tipper,
                    to: po,
                    amount: actual_received,
                    coin_type: ct,
                    is_post: true,
                });
            };
        } else {
            let repost_owner_amount = (amount * config.repost_tip_percentage) / 100;
            let original_owner_amount = amount - repost_owner_amount;
            let po = post.owner;
            let poid = object::uid_to_address(&post.id);
            let opo = original_post.owner;
            let opoid = object::uid_to_address(&original_post.id);
            let repost_actual_received = apply_revenue_manifest_coin<T>(
                post,
                vault_for_post,
                po,
                repost_owner_amount,
                coin,
                tipper,
                poid,
                true,
                min_vault_deposit_amount,
                clock,
                ctx
            );
            let original_actual_received = apply_revenue_manifest_coin<T>(
                original_post,
                vault_for_original,
                opo,
                original_owner_amount,
                coin,
                tipper,
                opoid,
                true,
                min_vault_deposit_amount,
                clock,
                ctx
            );
            assert!(post.tips_received <= MAX_U64 - repost_actual_received, EOverflow);
            post.tips_received = post.tips_received + repost_actual_received;
            assert!(original_post.tips_received <= MAX_U64 - original_actual_received, EOverflow);
            original_post.tips_received = original_post.tips_received + original_actual_received;
            if (repost_actual_received > 0) {
                event::emit(TipEvent {
                    object_id: poid,
                    from: tipper,
                    to: po,
                    amount: repost_actual_received,
                    coin_type: ct,
                    is_post: true,
                });
            };
            if (original_actual_received > 0) {
                event::emit(TipEvent {
                    object_id: opoid,
                    from: tipper,
                    to: opo,
                    amount: original_actual_received,
                    coin_type: ct,
                    is_post: true,
                });
            };
        }
    }

    /// Tip a comment; split per `PostConfig` with manifest-based routing on the post owner's share.
    public fun tip_comment<T>(
        comment: &mut Comment,
        post: &mut Post,
        config: &PostConfig,
        beneficiary_vault: &mut PoCBeneficiaryVault,
        coin: &mut Coin<T>,
        amount: u64,
        min_vault_deposit_amount: u64,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let tipper = tx_context::sender(ctx);
        assert!(amount > 0 && coin::value(coin) >= amount, EInvalidTipAmount);
        assert_tip_spend_limit(memory_account, amount, ctx);
        assert!(tipper != comment.owner, ESelfTipping);
        assert!(allow_tips(post), ETipsNotAllowed);
        let commenter_amount = (amount * config.commenter_tip_percentage) / 100;
        let post_owner_amount = amount - commenter_amount;
        let commenter_tip = coin::split(coin, commenter_amount, ctx);
        transfer::public_transfer(commenter_tip, comment.owner);
        let po = post.owner;
        let poid = object::uid_to_address(&post.id);
        let ct = type_name::with_defining_ids<T>();
        let post_owner_actual_received = apply_revenue_manifest_coin<T>(
            post,
            beneficiary_vault,
            po,
            post_owner_amount,
            coin,
            tipper,
            poid,
            true,
            min_vault_deposit_amount,
            clock,
            ctx
        );
        assert!(comment.tips_received <= MAX_U64 - commenter_amount, EOverflow);
        comment.tips_received = comment.tips_received + commenter_amount;
        assert!(post.tips_received <= MAX_U64 - post_owner_actual_received, EOverflow);
        post.tips_received = post.tips_received + post_owner_actual_received;
        event::emit(TipEvent {
            object_id: object::uid_to_address(&comment.id),
            from: tipper,
            to: comment.owner,
            amount: commenter_amount,
            coin_type: ct,
            is_post: false,
        });
        if (post_owner_actual_received > 0) {
            event::emit(TipEvent {
                object_id: poid,
                from: tipper,
                to: po,
                amount: post_owner_actual_received,
                coin_type: ct,
                is_post: true,
            });
        };
    }

    /// Like [`tip_comment`] but without a `PoCBeneficiaryVault` / Clock / min vault deposit.
    /// Only for posts where [`tip_post_requires_beneficiary_vault_for_amount`] is false for the
    /// **post-owner slice** of this tip. If an escrow deposit is required, aborts with
    /// [`ETipPostRequiresBeneficiaryVault`].
    public fun tip_comment_simple<T>(
        comment: &mut Comment,
        post: &mut Post,
        config: &PostConfig,
        coin: &mut Coin<T>,
        amount: u64,
        memory_account: &MemoryAccount,
        ctx: &mut TxContext
    ) {
        let tipper = tx_context::sender(ctx);
        assert!(amount > 0 && coin::value(coin) >= amount, EInvalidTipAmount);
        assert_tip_spend_limit(memory_account, amount, ctx);
        assert!(tipper != comment.owner, ESelfTipping);
        assert!(allow_tips(post), ETipsNotAllowed);
        let commenter_amount = (amount * config.commenter_tip_percentage) / 100;
        let post_owner_amount = amount - commenter_amount;
        assert!(
            !tip_post_requires_beneficiary_vault_for_amount(post, post_owner_amount),
            ETipPostRequiresBeneficiaryVault
        );
        let commenter_tip = coin::split(coin, commenter_amount, ctx);
        transfer::public_transfer(commenter_tip, comment.owner);
        let po = post.owner;
        let poid = object::uid_to_address(&post.id);
        let ct = type_name::with_defining_ids<T>();
        let post_owner_actual_received = apply_revenue_manifest_coin_without_beneficiary_vault<T>(
            post,
            po,
            post_owner_amount,
            coin,
            tipper,
            poid,
            true,
            ctx
        );
        assert!(comment.tips_received <= MAX_U64 - commenter_amount, EOverflow);
        comment.tips_received = comment.tips_received + commenter_amount;
        assert!(post.tips_received <= MAX_U64 - post_owner_actual_received, EOverflow);
        post.tips_received = post.tips_received + post_owner_actual_received;
        event::emit(TipEvent {
            object_id: object::uid_to_address(&comment.id),
            from: tipper,
            to: comment.owner,
            amount: commenter_amount,
            coin_type: ct,
            is_post: false,
        });
        if (post_owner_actual_received > 0) {
            event::emit(TipEvent {
                object_id: poid,
                from: tipper,
                to: po,
                amount: post_owner_actual_received,
                coin_type: ct,
                is_post: true,
            });
        };
    }

    /// Transfer post ownership to another user (by post owner)
    public fun transfer_post_ownership(
        post: &mut Post,
        new_owner: address,
        registry: &UsernameRegistry,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(post.version == upgrade::current_version(), EWrongVersion);
        
        let current_owner = tx_context::sender(ctx);
        
        // Verify current owner is authorized
        assert!(current_owner == post.owner, EUnauthorizedTransfer);
        
        // Look up the profile ID for the new owner (for reference, not ownership)
        let mut profile_id_option = social_contracts::profile::lookup_profile_by_owner(registry, new_owner);
        assert!(option::is_some(&profile_id_option), EUnauthorized);
        let new_profile_id = option::extract(&mut profile_id_option);
        
        // Update post ownership
        let previous_owner = post.owner;
        post.owner = new_owner;
        post.profile_id = new_profile_id;
        
        // Emit ownership transfer event
        event::emit(OwnershipTransferEvent {
            object_id: object::uid_to_address(&post.id),
            previous_owner,
            new_owner,
            is_post: true,
        });
    }

    /// Admin function to transfer post ownership (requires PostAdminCap)
    public fun admin_transfer_post_ownership(
        _: &PostAdminCap,
        post: &mut Post,
        new_owner: address,
        registry: &UsernameRegistry,
        _ctx: &mut TxContext
    ) {
        // Admin capability verification is handled by type system
        
        // Check version compatibility
        assert!(post.version == upgrade::current_version(), EWrongVersion);
        
        // Look up the profile ID for the new owner (for reference, not ownership)
        let mut profile_id_option = social_contracts::profile::lookup_profile_by_owner(registry, new_owner);
        assert!(option::is_some(&profile_id_option), EUnauthorized);
        let new_profile_id = option::extract(&mut profile_id_option);
        
        // Update post ownership
        let previous_owner = post.owner;
        post.owner = new_owner;
        post.profile_id = new_profile_id;
        
        // Emit ownership transfer event
        event::emit(OwnershipTransferEvent {
            object_id: object::uid_to_address(&post.id),
            previous_owner,
            new_owner,
            is_post: true,
        });
    }

    /// Moderate a post (remove/restore from platform)
    public fun moderate_post(
        post: &mut Post,
        platform: &platform::Platform,
        group: &PermissionedGroup<platform::PlatformPackage>,
        platform_registry: &platform::PlatformRegistry,
        remove: bool,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(post.version == upgrade::current_version(), EWrongVersion);
        assert!(platform::platform_version(platform) == upgrade::current_version(), EWrongVersion);
        
        let caller = tx_context::sender(ctx);
        assert!(
            platform::has_moderator_permission<platform::PlatformContentModerator>(group, platform, caller),
            EUnauthorized,
        );
        
        // Verify platform is approved
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), EUnauthorized);
        
        // Update post status
        post.removed_from_platform = remove;
        
        // Emit moderation event
        event::emit(PostModerationEvent {
            post_id: object::uid_to_address(&post.id),
            platform_id: object::uid_to_address(platform::id(platform)),
            removed: remove,
            moderated_by: caller,
        });
    }

    /// Moderate a comment (remove/restore from platform)
    public fun moderate_comment(
        comment: &mut Comment,
        platform: &platform::Platform,
        group: &PermissionedGroup<platform::PlatformPackage>,
        platform_registry: &platform::PlatformRegistry,
        remove: bool,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(comment.version == upgrade::current_version(), EWrongVersion);
        assert!(platform::platform_version(platform) == upgrade::current_version(), EWrongVersion);
        
        let caller = tx_context::sender(ctx);
        assert!(
            platform::has_moderator_permission<platform::PlatformContentModerator>(group, platform, caller),
            EUnauthorized,
        );
        
        // Verify platform is approved
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), EUnauthorized);
        
        // Update comment status
        comment.removed_from_platform = remove;
        
        // Emit moderation event
        event::emit(PostModerationEvent {
            post_id: object::uid_to_address(&comment.id),
            platform_id: object::uid_to_address(platform::id(platform)),
            removed: remove,
            moderated_by: caller,
        });
    }

    /// Update an existing post
    public fun update_post(
        post: &mut Post,
        config: &PostConfig,
        content: String,
        mut media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Verify caller is the owner
        let owner = tx_context::sender(ctx);
        assert!(owner == post.owner, EUnauthorized);
        
        // Validate content length using config
        assert!(string::length(&content) <= config.max_content_length, EContentTooLarge);
        
        // Validate and update metadata if provided
        if (option::is_some(&metadata_json)) {
            let metadata_string = option::borrow(& metadata_json);
            assert!(string::length(metadata_string) <= config.max_metadata_size, EContentTooLarge);
            // Clear the current value and set the new one
            post.metadata_json = option::some(*metadata_string);
        };
        
        // Convert and validate media URLs if provided
        if (option::is_some(&media_urls)) {
            let url_strings = option::extract(&mut media_urls);
            
            // Validate media URLs count
            assert!(vector::length(&url_strings) <= config.max_media_urls, ETooManyMediaUrls);
            
            // Convert string URLs to Url objects
            let mut urls = vector::empty<Url>();
            let mut i = 0;
            let len = vector::length(&url_strings);
            while (i < len) {
                let url_string = vector::borrow(&url_strings, i);
                let url_bytes = string::as_bytes(url_string);
                vector::push_back(&mut urls, url::new_unsafe_from_bytes(*url_bytes));
                i = i + 1;
            };
            post.media = option::some(urls);
        };
        
        // Validate mentions if provided using config
        if (option::is_some(&mentions)) {
            let mentions_ref = option::borrow(&mentions);
            assert!(vector::length(mentions_ref) <= config.max_mentions, EContentTooLarge);
            post.mentions = mentions;
        };
        
        // Update post content
        post.content = content;
        
        // Emit post updated event
        event::emit(PostUpdatedEvent {
            post_id: object::uid_to_address(&post.id),
            owner: post.owner,
            profile_id: post.profile_id,
            content: post.content,
            metadata_json: post.metadata_json,
            updated_at: clock::timestamp_ms(clock),
        });
    }

    /// Update an existing comment
    public fun update_comment(
        comment: &mut Comment,
        config: &PostConfig,
        content: String,
        mentions: Option<vector<address>>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Verify caller is the owner
        let owner = tx_context::sender(ctx);
        assert!(owner == comment.owner, EUnauthorized);
        
        // Validate content length using config
        assert!(string::length(&content) <= config.max_content_length, EContentTooLarge);
        
        // Validate mentions if provided using config
        if (option::is_some(&mentions)) {
            let mentions_ref = option::borrow(&mentions);
            assert!(vector::length(mentions_ref) <= config.max_mentions, EContentTooLarge);
            comment.mentions = mentions;
        };
        
        // Update comment content
        comment.content = content;
        
        // Emit comment updated event
        event::emit(CommentUpdatedEvent {
            comment_id: object::uid_to_address(&comment.id),
            post_id: comment.post_id,
            owner: comment.owner,
            profile_id: comment.profile_id,
            content: comment.content,
            updated_at: clock::timestamp_ms(clock),
        });
    }

    /// Report a post
    public fun report_post(
        post: &Post,
        config: &PostConfig,
        reason_code: u8,
        description: String,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Validate reason code
        assert!(
            reason_code == REPORT_REASON_SPAM ||
            reason_code == REPORT_REASON_OFFENSIVE ||
            reason_code == REPORT_REASON_MISINFORMATION ||
            reason_code == REPORT_REASON_ILLEGAL ||
            reason_code == REPORT_REASON_IMPERSONATION ||
            reason_code == REPORT_REASON_HARASSMENT ||
            reason_code == REPORT_REASON_OTHER,
            EReportReasonInvalid
        );
        
        // Validate description length using config
        assert!(string::length(&description) <= config.max_description_length, EReportDescriptionTooLong);
        
        // Get reporter's address
        let reporter = tx_context::sender(ctx);
        
        // Emit post reported event
        event::emit(PostReportedEvent {
            post_id: object::uid_to_address(&post.id),
            reporter,
            reason_code,
            description,
            reported_at: clock::timestamp_ms(clock),
        });
    }

    /// Report a comment
    public fun report_comment(
        comment: &Comment,
        config: &PostConfig,
        reason_code: u8,
        description: String,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Validate reason code
        assert!(
            reason_code == REPORT_REASON_SPAM ||
            reason_code == REPORT_REASON_OFFENSIVE ||
            reason_code == REPORT_REASON_MISINFORMATION ||
            reason_code == REPORT_REASON_ILLEGAL ||
            reason_code == REPORT_REASON_IMPERSONATION ||
            reason_code == REPORT_REASON_HARASSMENT ||
            reason_code == REPORT_REASON_OTHER,
            EReportReasonInvalid
        );
        
        // Validate description length using config
        assert!(string::length(&description) <= config.max_description_length, EReportDescriptionTooLong);
        
        // Get reporter's address
        let reporter = tx_context::sender(ctx);
        
        // Emit comment reported event
        event::emit(CommentReportedEvent {
            comment_id: object::uid_to_address(&comment.id),
            reporter,
            reason_code,
            description,
            reported_at: clock::timestamp_ms(clock),
        });
    }

    /// React to a comment with a specific reaction (emoji or text)
    /// If the user already has the exact same reaction, it will be removed (toggle behavior)
    public fun react_to_comment(
        registry: &UsernameRegistry,
        comment: &mut Comment,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        memory_account: &MemoryAccount,
        reaction: String,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let acting = resolve_social_actor(
            memory_config,
            registry,
            platform,
            block_list_registry,
            memory_account,
            memory::cap_react(),
            0,
            clock,
            ctx,
        );
        let actor_address = memory::acting_actor_address(&acting);
        let principal_owner = memory::acting_principal_owner(&acting);
        let sub_agent_id = memory::acting_sub_agent_id(&acting);
        let organization_id = memory::acting_organization_id(&acting);
        let action_identity_class = memory::acting_identity_class(&acting);
        
        // Check if platform is approved
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), EUnauthorized);
        
        // Validate reaction length using config
        assert!(string::length(&reaction) <= config.max_reaction_length, EReactionContentTooLong);
        
        // Check if user already reacted to the comment
        if (table::contains(&comment.user_reactions, actor_address)) {
            // Get the previous reaction
            let previous_reaction = *table::borrow(&comment.user_reactions, actor_address);
            
            // If the reaction is the same, remove it (toggle behavior)
            if (reaction == previous_reaction) {
                // Remove user's reaction
                table::remove(&mut comment.user_reactions, actor_address);
                
                // Decrease count for this reaction type
                let count = *table::borrow(&comment.reaction_counts, reaction);
                if (count <= 1) {
                    table::remove(&mut comment.reaction_counts, reaction);
                } else {
                    *table::borrow_mut(&mut comment.reaction_counts, reaction) = count - 1;
                };
                
                // Decrement comment reaction count with underflow protection
                assert!(comment.reaction_count > 0, EOverflow);
                comment.reaction_count = comment.reaction_count - 1;
                
                // Emit remove reaction event
                event::emit(RemoveReactionEvent {
                    object_id: object::uid_to_address(&comment.id),
                    user: actor_address,
                    reaction,
                    is_post: false,
                    principal_owner,
                    actor_address,
                    sub_agent_id,
                    organization_id,
                    action_identity_class,
                });
                
                return
            };
            
            // Different reaction, update existing one
            // Decrease count for previous reaction
            let previous_count = *table::borrow(&comment.reaction_counts, previous_reaction);
            if (previous_count <= 1) {
                table::remove(&mut comment.reaction_counts, previous_reaction);
            } else {
                *table::borrow_mut(&mut comment.reaction_counts, previous_reaction) = previous_count - 1;
            };
            
            // Update user's reaction
            *table::borrow_mut(&mut comment.user_reactions, actor_address) = reaction;
        } else {
            // New reaction from this user
            table::add(&mut comment.user_reactions, actor_address, reaction);

            // Increment comment reaction count
            assert!(comment.reaction_count <= MAX_U64 - 1, EOverflow);
            comment.reaction_count = comment.reaction_count + 1;
        };
        
        // Increment count for the reaction
        if (table::contains(&comment.reaction_counts, reaction)) {
            let count = *table::borrow(&comment.reaction_counts, reaction);
            assert!(count <= MAX_U64 - 1, EOverflow);
            *table::borrow_mut(&mut comment.reaction_counts, reaction) = count + 1;
        } else {
            table::add(&mut comment.reaction_counts, reaction, 1);
        };
        
        // Emit reaction event
        event::emit(ReactionEvent {
            object_id: object::uid_to_address(&comment.id),
            user: actor_address,
            reaction,
            is_post: false,
            principal_owner,
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
        });
    }

    /// Edit a post through the delegated-agent authorization model.
    public fun edit_post(
        registry: &UsernameRegistry,
        platform: &platform::Platform,
        block_list_registry: &BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        memory_account: &MemoryAccount,
        post: &mut Post,
        content: String,
        mut media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let acting = resolve_social_actor(
            memory_config, registry, platform, block_list_registry, memory_account,
            memory::cap_post_publish(), 0, clock, ctx,
        );
        assert!(memory::acting_principal_owner(&acting) == post.owner, EUnauthorized);
        assert!(string::length(&content) <= config.max_content_length, EContentTooLarge);
        if (option::is_some(&metadata_json)) {
            let metadata_string = option::borrow(&metadata_json);
            assert!(string::length(metadata_string) <= config.max_metadata_size, EContentTooLarge);
            post.metadata_json = option::some(*metadata_string);
        };
        if (option::is_some(&media_urls)) {
            let url_strings = option::extract(&mut media_urls);
            assert!(vector::length(&url_strings) <= config.max_media_urls, ETooManyMediaUrls);
            let mut urls = vector::empty<Url>();
            let mut i = 0;
            let len = vector::length(&url_strings);
            while (i < len) {
                let url_string = vector::borrow(&url_strings, i);
                vector::push_back(
                    &mut urls,
                    url::new_unsafe_from_bytes(*string::as_bytes(url_string)),
                );
                i = i + 1;
            };
            post.media = option::some(urls);
        };
        if (option::is_some(&mentions)) {
            assert!(vector::length(option::borrow(&mentions)) <= config.max_mentions, EContentTooLarge);
            post.mentions = mentions;
        };
        post.content = content;
        event::emit(PostUpdatedEvent {
            post_id: object::uid_to_address(&post.id),
            owner: post.owner,
            profile_id: post.profile_id,
            content: post.content,
            metadata_json: post.metadata_json,
            updated_at: clock::timestamp_ms(clock),
        });
    }

    /// Edit a comment through the delegated-agent authorization model.
    public fun edit_comment(
        registry: &UsernameRegistry,
        platform: &platform::Platform,
        block_list_registry: &BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        memory_account: &MemoryAccount,
        comment: &mut Comment,
        content: String,
        mentions: Option<vector<address>>,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let acting = resolve_social_actor(
            memory_config, registry, platform, block_list_registry, memory_account,
            memory::cap_comment(), 0, clock, ctx,
        );
        assert!(memory::acting_principal_owner(&acting) == comment.owner, EUnauthorized);
        assert!(string::length(&content) <= config.max_content_length, EContentTooLarge);
        if (option::is_some(&mentions)) {
            assert!(vector::length(option::borrow(&mentions)) <= config.max_mentions, EContentTooLarge);
            comment.mentions = mentions;
        };
        comment.content = content;
        event::emit(CommentUpdatedEvent {
            comment_id: object::uid_to_address(&comment.id),
            post_id: comment.post_id,
            owner: comment.owner,
            profile_id: comment.profile_id,
            content: comment.content,
            updated_at: clock::timestamp_ms(clock),
        });
    }

    /// Remove the caller principal's current post reaction without toggle semantics.
    public fun remove_post_reaction(
        registry: &UsernameRegistry,
        post: &mut Post,
        platform: &platform::Platform,
        block_list_registry: &BlockListRegistry,
        memory_config: &MemoryConfig,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let acting = resolve_social_actor(
            memory_config, registry, platform, block_list_registry, memory_account,
            memory::cap_react(), 0, clock, ctx,
        );
        let actor_address = memory::acting_actor_address(&acting);
        let principal_owner = memory::acting_principal_owner(&acting);
        assert!(table::contains(&post.user_reactions, actor_address), EUnauthorized);
        let reaction = table::remove(&mut post.user_reactions, actor_address);
        let count = *table::borrow(&post.reaction_counts, reaction);
        if (count <= 1) {
            table::remove(&mut post.reaction_counts, reaction);
        } else {
            *table::borrow_mut(&mut post.reaction_counts, reaction) = count - 1;
        };
        assert!(post.reaction_count > 0, EOverflow);
        post.reaction_count = post.reaction_count - 1;
        event::emit(RemoveReactionEvent {
            object_id: object::uid_to_address(&post.id),
            user: actor_address,
            reaction,
            is_post: true,
            principal_owner,
            actor_address,
            sub_agent_id: memory::acting_sub_agent_id(&acting),
            organization_id: memory::acting_organization_id(&acting),
            action_identity_class: memory::acting_identity_class(&acting),
        });
    }

    /// Remove the caller principal's current comment reaction without toggle semantics.
    public fun remove_comment_reaction(
        registry: &UsernameRegistry,
        comment: &mut Comment,
        platform: &platform::Platform,
        block_list_registry: &BlockListRegistry,
        memory_config: &MemoryConfig,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let acting = resolve_social_actor(
            memory_config, registry, platform, block_list_registry, memory_account,
            memory::cap_react(), 0, clock, ctx,
        );
        let actor_address = memory::acting_actor_address(&acting);
        let principal_owner = memory::acting_principal_owner(&acting);
        assert!(table::contains(&comment.user_reactions, actor_address), EUnauthorized);
        let reaction = table::remove(&mut comment.user_reactions, actor_address);
        let count = *table::borrow(&comment.reaction_counts, reaction);
        if (count <= 1) {
            table::remove(&mut comment.reaction_counts, reaction);
        } else {
            *table::borrow_mut(&mut comment.reaction_counts, reaction) = count - 1;
        };
        assert!(comment.reaction_count > 0, EOverflow);
        comment.reaction_count = comment.reaction_count - 1;
        event::emit(RemoveReactionEvent {
            object_id: object::uid_to_address(&comment.id),
            user: actor_address,
            reaction,
            is_post: false,
            principal_owner,
            actor_address,
            sub_agent_id: memory::acting_sub_agent_id(&acting),
            organization_id: memory::acting_organization_id(&acting),
            action_identity_class: memory::acting_identity_class(&acting),
        });
    }

    /// Remove a repost and decrement its original post's aggregate count.
    public fun remove_repost(
        registry: &UsernameRegistry,
        platform: &platform::Platform,
        block_list_registry: &BlockListRegistry,
        memory_config: &MemoryConfig,
        memory_account: &MemoryAccount,
        original_post: &mut Post,
        repost: Repost,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let acting = resolve_social_actor(
            memory_config, registry, platform, block_list_registry, memory_account,
            memory::cap_post_publish(), 0, clock, ctx,
        );
        let principal_owner = memory::acting_principal_owner(&acting);
        assert!(principal_owner == repost.owner, EUnauthorized);
        assert!(repost.is_original_post, EInvalidParentReference);
        assert!(repost.original_id == object::uid_to_address(&original_post.id), EInvalidParentReference);
        assert!(original_post.repost_count > 0, EOverflow);
        original_post.repost_count = original_post.repost_count - 1;
        let repost_id = object::uid_to_address(&repost.id);
        event::emit(RepostRemovedEvent {
            repost_id,
            original_id: repost.original_id,
            owner: principal_owner,
            actor_address: memory::acting_actor_address(&acting),
            sub_agent_id: memory::acting_sub_agent_id(&acting),
            organization_id: memory::acting_organization_id(&acting),
            action_identity_class: memory::acting_identity_class(&acting),
            removed_at: clock::timestamp_ms(clock),
        });
        let Repost { id, original_id: _, is_original_post: _, owner: _, profile_id: _, created_at: _, version: _ } = repost;
        object::delete(id);
    }

    /// Get post content
    public fun get_post_content(post: &Post): String {
        post.content
    }

    /// Get post owner
    public fun get_post_owner(post: &Post): address {
        post.owner
    }

    /// Get post ID
    public fun get_post_id(post: &Post): &UID {
        &post.id
    }

    /// Get post comment count
    public fun get_post_comment_count(post: &Post): u64 {
        post.comment_count
    }

    /// Get comment owner
    public fun get_comment_owner(comment: &Comment): address {
        comment.owner
    }

    /// Get comment post ID
    public fun get_comment_post_id(comment: &Comment): address {
        comment.post_id
    }

    /// Get the ID address of a post
    public fun get_id_address(post: &Post): address {
        object::uid_to_address(&post.id)
    }

    /// Get the reaction count of a post
    public fun get_reaction_count(post: &Post): u64 {
        post.reaction_count
    }

    /// Get the tips received for a post
    public fun get_tips_received(post: &Post): u64 {
        post.tips_received
    }

    /// Get the tips received for a comment (commenter share of tip_comment / tip_comment_simple).
    public fun get_comment_tips_received(comment: &Comment): u64 {
        comment.tips_received
    }

    /// Get the platform ID for a post
    public fun get_platform_id(post: &Post): address {
        post.platform_id
    }

    /// Get the revenue redirect address for a post (legacy compat stub — always none).
    public fun get_revenue_redirect_to(_post: &Post): Option<address> {
        option::none()
    }

    /// Get the revenue redirect percentage for a post (legacy compat stub — always none).
    public fun get_revenue_redirect_percentage(_post: &Post): Option<u64> {
        option::none()
    }

    /// Legacy compat — derived from cached revenue manifest payout modes.
    public fun poc_redirection_kind(post: &Post): u8 {
        let manifest_opt = revenue_manifest(post);
        if (option::is_none(&manifest_opt)) {
            return 0
        };
        let manifest = option::borrow(&manifest_opt);
        if (media_asset::manifest_uses_escrow_redirect(manifest)) {
            2
        } else {
            1
        }
    }

    /// Legacy compat stub for fee-routing callers during migration.
    public fun poc_redirection_none(): u8 { 0 }

    #[test_only]
    public fun test_assert_post_access_mydata_binding(
        owner: address,
        access: PostAccess,
        mydata_registry: &mydata::MyDataRegistry,
    ) {
        assert_post_access_mydata_binding(owner, &access, mydata_registry);
    }

    #[test_only]
    public fun test_access_kind_public(): u8 {
        POST_ACCESS_PUBLIC
    }

    #[test_only]
    public fun test_post_access_public(): PostAccess {
        PostAccess::Public
    }

    #[test_only]
    public fun test_post_access_profile_subscription(
        service_id: ID,
        mydata_id: Option<ID>,
        min_tier_level: Option<u64>,
    ): PostAccess {
        PostAccess::ProfileSubscription { service_id, mydata_id, min_tier_level }
    }

    #[test_only]
    public fun test_post_access_marketplace_one_time(mydata_id: ID): PostAccess {
        PostAccess::MarketplaceOneTime { mydata_id }
    }

    #[test_only]
    public fun test_assert_post_access_mydata_object_binding(
        access: PostAccess,
        mydata: &mydata::MyData,
    ) {
        assert_post_access_mydata_object_binding(&access, mydata);
    }

    /// Test-only initialization function
    #[test_only]
    public fun test_init(ctx: &mut TxContext) {
        // Create and share post configuration for testing
        transfer::share_object(
            PostConfig {
                id: object::new(ctx),
                max_content_length: MAX_CONTENT_LENGTH,
                max_media_urls: MAX_MEDIA_URLS,
                max_mentions: MAX_MENTIONS,
                max_metadata_size: MAX_METADATA_SIZE,
                max_description_length: MAX_DESCRIPTION_LENGTH,
                max_reaction_length: MAX_REACTION_LENGTH,
                commenter_tip_percentage: COMMENTER_TIP_PERCENTAGE,
                repost_tip_percentage: REPOST_TIP_PERCENTAGE,
                min_promotion_amount: MIN_PROMOTION_AMOUNT,
                max_promotion_amount: MAX_PROMOTION_AMOUNT,
                min_view_duration_ms: MIN_VIEW_DURATION,
                platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
                ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
                version: upgrade::current_version(),
            }
        );
        
        // Create and transfer the admin capability for testing
        let admin_cap = PostAdminCap {
            id: object::new(ctx),
        };
        
        transfer::public_transfer(admin_cap, tx_context::sender(ctx));
    }
    
    /// Test-only function to create a post directly for testing
    #[test_only]
    public fun test_create_post(
        owner: address,
        profile_id: address,
        platform_id: address,
        content: String,
        clock: &Clock,
        ctx: &mut TxContext
    ): address {
        share_post(create_post_internal(
            owner,
            profile_id,
            platform_id,
            content,
            vector[],
            option::none(), // No media
            option::none(), // No mentions
            option::none(), // No metadata
            string::utf8(POST_TYPE_STANDARD), // Standard post type
            option::none(), // No parent post
            true, // allow_comments
            true, // allow_reactions
            true, // allow_reposts
            true, // allow_quotes
            true, // allow_tips
            PostAccess::Public,
            option::none(), // promotion_id
            false, // enable_spt - default to opt-out
            owner,
            option::none(),
            option::none(),
            memory::class_human(),
            clock,
            ctx
        ))
    }

    /// Test-only: post with revenue manifest set (for fee routing tests).
    #[test_only]
    public fun test_create_post_with_revenue_manifest(
        owner: address,
        profile_id: address,
        platform_id: address,
        content: String,
        manifest: RevenueManifest,
        clock: &Clock,
        ctx: &mut TxContext
    ): address {
        let mut post = create_post_internal(
            owner,
            profile_id,
            platform_id,
            content,
            vector[],
            option::none(),
            option::none(),
            option::none(),
            string::utf8(POST_TYPE_STANDARD),
            option::none(),
            true,
            true,
            true,
            true,
            true,
            PostAccess::Public,
            option::none(),
            false,
            owner,
            option::none(),
            option::none(),
            memory::class_human(),
            clock,
            ctx
        );
        let badge = media_asset::new_composition_badge_snapshot(
            clock::timestamp_ms(clock),
            owner,
            option::none(),
            option::none(),
            true,
            false,
        );
        let analysis = media_asset::test_composition_analysis(
            clock::timestamp_ms(clock),
            media_asset::usage_social_post(),
        );
        set_composition_result(
            &mut post,
            media_asset::composition_verified(),
            media_asset::monetization_enabled(),
            badge,
            analysis,
            option::some(manifest),
        );
        share_post(post)
    }

    #[test_only]
    public fun test_create_post_with_escrow_manifest(
        owner: address,
        profile_id: address,
        platform_id: address,
        content: String,
        beneficiary: address,
        share_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ): address {
        let manifest = media_asset::test_revenue_manifest(vector[
            media_asset::test_manifest_entry(
                beneficiary,
                share_bps,
                media_asset::payout_escrow(),
            ),
        ]);
        test_create_post_with_revenue_manifest(
            owner,
            profile_id,
            platform_id,
            content,
            manifest,
            clock,
            ctx,
        )
    }

    #[test_only]
    public fun test_create_post_with_wallet_manifest(
        owner: address,
        profile_id: address,
        platform_id: address,
        content: String,
        beneficiary: address,
        share_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ): address {
        let manifest = media_asset::test_revenue_manifest(vector[
            media_asset::test_manifest_entry(
                beneficiary,
                share_bps,
                media_asset::payout_wallet(),
            ),
        ]);
        test_create_post_with_revenue_manifest(
            owner,
            profile_id,
            platform_id,
            content,
            manifest,
            clock,
            ctx,
        )
    }

    /// Legacy test helper: maps redirect percentage (0-100) to a wallet manifest split.
    #[test_only]
    public fun test_create_post_with_revenue_redirect(
        owner: address,
        profile_id: address,
        platform_id: address,
        content: String,
        redirect_to: address,
        redirect_percentage: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ): address {
        let redirect_bps = redirect_percentage * 100;
        let owner_bps = media_asset::manifest_bps_total() - redirect_bps;
        let mut entries = vector[];
        if (redirect_bps > 0) {
            vector::push_back(
                &mut entries,
                media_asset::test_manifest_entry(
                    redirect_to,
                    redirect_bps,
                    media_asset::payout_wallet(),
                ),
            );
        };
        if (owner_bps > 0) {
            vector::push_back(
                &mut entries,
                media_asset::test_manifest_entry(
                    owner,
                    owner_bps,
                    media_asset::payout_wallet(),
                ),
            );
        };
        test_create_post_with_revenue_manifest(
            owner,
            profile_id,
            platform_id,
            content,
            media_asset::test_revenue_manifest(entries),
            clock,
            ctx,
        )
    }

    #[test_only]
    public fun test_create_post_with_escrow_redirect(
        owner: address,
        profile_id: address,
        platform_id: address,
        content: String,
        redirect_to: address,
        redirect_percentage: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ): address {
        let redirect_bps = redirect_percentage * 100;
        let owner_bps = media_asset::manifest_bps_total() - redirect_bps;
        let mut entries = vector[];
        if (redirect_bps > 0) {
            vector::push_back(
                &mut entries,
                media_asset::test_manifest_entry(
                    redirect_to,
                    redirect_bps,
                    media_asset::payout_escrow(),
                ),
            );
        };
        if (owner_bps > 0) {
            vector::push_back(
                &mut entries,
                media_asset::test_manifest_entry(
                    owner,
                    owner_bps,
                    media_asset::payout_wallet(),
                ),
            );
        };
        test_create_post_with_revenue_manifest(
            owner,
            profile_id,
            platform_id,
            content,
            media_asset::test_revenue_manifest(entries),
            clock,
            ctx,
        )
    }

    /// Test helper to create a post with SPoT enabled
    #[test_only]
    public fun test_create_post_with_spot(
        owner: address,
        profile_id: address,
        platform_id: address,
        content: String,
        clock: &Clock,
        ctx: &mut TxContext
    ): address {
        share_post(create_post_internal(
            owner,
            profile_id,
            platform_id,
            content,
            vector[],
            option::none(), // No media
            option::none(), // No mentions
            option::none(), // No metadata
            string::utf8(POST_TYPE_STANDARD), // Standard post type
            option::none(), // No parent post
            true, // allow_comments
            true, // allow_reactions
            true, // allow_reposts
            true, // allow_quotes
            true, // allow_tips
            PostAccess::Public,
            option::none(), // promotion_id
            false, // enable_spt - default to opt-out
            owner,
            option::none(),
            option::none(),
            memory::class_human(),
            clock,
            ctx
        ))
    }
    
    /// Test-only function to create a promoted post directly for testing
    #[test_only]
    public fun create_test_promoted_post(
        owner: address,
        profile_id: address,
        platform_id: address,
        content: String,
        payment_per_view: u64,
        promotion_budget: Coin<MYSO>,
        clock: &Clock,
        ctx: &mut TxContext
    ): (address, address) {
        // Create promotion data
        let mut promotion_data = PromotionData {
            id: object::new(ctx),
            post_id: @0x0, // Will be set after post creation
            payment_per_view,
            promotion_budget: coin::into_balance(promotion_budget),
            paid_viewers: table::new(ctx),
            views: vector::empty(),
            active: false, // Starts inactive
            created_at: clock::timestamp_ms(clock),
        };
        
        let promotion_id = object::uid_to_address(&promotion_data.id);
        
        let media_option = option::none<vector<Url>>();
        let media_urls_for_event = convert_urls_to_strings(&media_option);
        let media_asset_ids_for_event = vector[];
        let post = create_post_internal(
            owner,
            profile_id,
            platform_id,
            content,
            vector[],
            media_option,
            option::none(), // No mentions
            option::none(), // No metadata
            string::utf8(POST_TYPE_STANDARD),
            option::none(), // No parent post
            true, // allow_comments
            true, // allow_reactions
            true, // allow_reposts
            true, // allow_quotes
            true, // allow_tips
            PostAccess::Public,
            option::some(promotion_id), // promotion_id
            false, // enable_spt - default to opt-out
            owner,
            option::none(),
            option::none(),
            memory::class_human(),
            clock,
            ctx
        );

        emit_post_media_asset_usages(&post, clock);
        let post_id = share_post(post);

        event::emit(PostCreatedEvent {
            post_id,
            owner,
            profile_id,
            platform_id,
            permissions: permissions_bitfield(true, true, true, true, true),
            content,
            post_type: string::utf8(POST_TYPE_STANDARD),
            parent_post_id: option::none(),
            mentions: option::none(),
            media_asset_ids: media_asset_ids_for_event,
            media_urls: media_urls_for_event,
            metadata_json: option::none(),
            access: PostAccess::Public,
            promotion_id: option::some(promotion_id),
            composition_status: media_asset::composition_none(),
            monetization_status: media_asset::monetization_none(),
            enable_spt: false,
            spt_id: option::none(),
            actor_address: owner,
            sub_agent_id: option::none(),
            organization_id: option::none(),
            action_identity_class: memory::class_human(),
        });
        
        // Update promotion data with post ID
        promotion_data.post_id = post_id;
        
        // Share promotion data
        transfer::share_object(promotion_data);
        
        (post_id, promotion_id)
    }

    /// Test-only function to get the admin cap ID
    #[test_only]
    public fun test_get_admin_cap(
        ctx: &mut TxContext
    ): address {
        // Create a new admin cap for testing
        let admin_cap = PostAdminCap {
            id: object::new(ctx),
        };
        
        let admin_cap_id = object::uid_to_address(&admin_cap.id);
        
        // Transfer to sender
        transfer::public_transfer(admin_cap, tx_context::sender(ctx));
        
        admin_cap_id
    }
    
    /// Test-only function to create a comment directly for testing
    #[test_only]
    public fun test_create_comment(
        owner: address,
        profile_id: address,
        post_id: address,
        content: String,
        clock: &Clock,
        ctx: &mut TxContext
    ): address {
        // Create a Comment object directly
        let mut comment = Comment {
            id: object::new(ctx),
            post_id,
            parent_comment_id: option::none(),
            owner,
            profile_id,
            content,
            media: option::none(),
            mentions: option::none(),
            metadata_json: option::none(),
            created_at: clock::timestamp_ms(clock),
            reaction_count: 0,
            comment_count: 0,
            repost_count: 0,
            tips_received: 0,
            removed_from_platform: false,
            user_reactions: table::new(ctx),
            reaction_counts: table::new(ctx),
            version: upgrade::current_version(),
        };

        attach_comment_attribution(
            &mut comment,
            owner,
            option::none(),
            option::none(),
            memory::class_human(),
        );
        
        // Get comment ID before sharing
        let comment_id = object::uid_to_address(&comment.id);
        
        // Share the comment
        transfer::share_object(comment);
        
        // Return the comment ID
        comment_id
    }

    // === Versioning Functions ===

    /// Get the version of a post
    public fun version(post: &Post): u64 {
        post.version
    }

    /// Get a mutable reference to the post version (for upgrade module)
    public(package) fun borrow_version_mut(post: &mut Post): &mut u64 {
        &mut post.version
    }

    /// Get the version of a comment
    public fun comment_version(comment: &Comment): u64 {
        comment.version
    }

    /// Get a mutable reference to the comment version (for upgrade module)
    public(package) fun borrow_comment_version_mut(comment: &mut Comment): &mut u64 {
        &mut comment.version
    }

    /// Get the version of a repost
    public fun repost_version(repost: &Repost): u64 {
        repost.version
    }

    /// Get a mutable reference to the repost version (for upgrade module)
    public(package) fun borrow_repost_version_mut(repost: &mut Repost): &mut u64 {
        &mut repost.version
    }

    /// Migration function for Post
    public entry fun migrate_post(
        post: &mut Post,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(post.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = post.version;
        if (old_version < 2) {
            post.composition_status = media_asset::composition_none();
            post.monetization_status = media_asset::monetization_none();
            post.composition_badge = option::none();
        };
        if (old_version < 3) {
            if (!df::exists_with_type<vector<u8>, PostAttribution>(&post.id, POST_ATTRIBUTION_DF_KEY)) {
                let owner = post.owner;
                attach_post_attribution(
                    post,
                    owner,
                    option::none(),
                    option::none(),
                    memory::class_human(),
                );
            };
        };
        post.version = current_version;
        
        // Initialize platform_id for existing posts (set to @0x0 as sentinel for unknown platform)
        // This field was added in a later version - existing posts will have @0x0
        // Platform-specific features may require manual update of platform_id
        // Note: This assumes platform_id field exists. If migrating from version before platform_id was added,
        // the field will be initialized to @0x0 by default.
        
        // Emit event for object migration
        let post_id = object::id(post);
        upgrade::emit_migration_event(
            post_id,
            string::utf8(POST_TYPE_STANDARD),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }

    /// Migration function for Comment
    public entry fun migrate_comment(
        comment: &mut Comment,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(comment.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = comment.version;
        comment.version = current_version;
        
        // Emit event for object migration
        let comment_id = object::id(comment);
        upgrade::emit_migration_event(
            comment_id,
            string::utf8(b"Comment"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }

    /// Migration function for Repost
    public entry fun migrate_repost(
        repost: &mut Repost,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(repost.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = repost.version;
        repost.version = current_version;
        
        // Emit event for object migration
        let repost_id = object::id(repost);
        upgrade::emit_migration_event(
            repost_id,
            string::utf8(b"Repost"),
            old_version,
            tx_context::sender(ctx)
        );
        
        // Any migration logic can be added here for future upgrades
    }

    /// Migration function for PostConfig
    public entry fun migrate_post_config(
        config: &mut PostConfig,
        _: &UpgradeAdminCap,
        ctx: &mut TxContext
    ) {
        let current_version = upgrade::current_version();
        
        // Verify this is an upgrade (new version > current version)
        assert!(config.version < current_version, EWrongVersion);
        
        // Remember old version and update to new version
        let old_version = config.version;
        config.version = current_version;
        // Seed promotion fee bps for configs created before these fields existed
        config.platform_fee_bps = DEFAULT_PLATFORM_FEE_BPS;
        config.ecosystem_fee_bps = DEFAULT_ECOSYSTEM_FEE_BPS;
        
        // Emit event for object migration
        let config_id = object::id(config);
        upgrade::emit_migration_event(
            config_id,
            string::utf8(b"PostConfig"),
            old_version,
            tx_context::sender(ctx)
        );
    }

    /// Update post parameters (admin only)
    public fun update_post_parameters(
        _admin_cap: &PostAdminCap,
        config: &mut PostConfig,
        max_content_length: u64,
        max_media_urls: u64,
        max_mentions: u64,
        max_metadata_size: u64,
        max_description_length: u64,
        max_reaction_length: u64,
        commenter_tip_percentage: u64,
        repost_tip_percentage: u64,
        min_promotion_amount: u64,
        max_promotion_amount: u64,
        min_view_duration_ms: u64,
        platform_fee_bps: u64,
        ecosystem_fee_bps: u64,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Validation
        assert!(commenter_tip_percentage <= 100, EInvalidConfig);
        assert!(repost_tip_percentage <= 100, EInvalidConfig);
        assert!(max_content_length > 0, EInvalidConfig);
        assert!(max_media_urls > 0, EInvalidConfig);
        assert!(max_mentions > 0, EInvalidConfig);
        assert!(min_promotion_amount > 0, EInvalidConfig);
        assert!(max_promotion_amount >= min_promotion_amount, EInvalidConfig);
        assert!(platform_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(ecosystem_fee_bps <= BPS_DENOM, EInvalidConfig);
        assert!(platform_fee_bps + ecosystem_fee_bps <= BPS_DENOM, EInvalidConfig);

        // Update config
        config.max_content_length = max_content_length;
        config.max_media_urls = max_media_urls;
        config.max_mentions = max_mentions;
        config.max_metadata_size = max_metadata_size;
        config.max_description_length = max_description_length;
        config.max_reaction_length = max_reaction_length;
        config.commenter_tip_percentage = commenter_tip_percentage;
        config.repost_tip_percentage = repost_tip_percentage;
        config.min_promotion_amount = min_promotion_amount;
        config.max_promotion_amount = max_promotion_amount;
        config.min_view_duration_ms = min_view_duration_ms;
        config.platform_fee_bps = platform_fee_bps;
        config.ecosystem_fee_bps = ecosystem_fee_bps;

        // Emit update event
        event::emit(PostParametersUpdatedEvent {
            updated_by: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
            max_content_length,
            max_media_urls,
            max_mentions,
            max_metadata_size,
            max_description_length,
            max_reaction_length,
            commenter_tip_percentage,
            repost_tip_percentage,
            min_promotion_amount,
            max_promotion_amount,
            min_view_duration_ms,
            platform_fee_bps,
            ecosystem_fee_bps,
        });
    }

    /// Create a promoted post with MYSO tokens for viewer payments
    public fun create_promoted_post(
        registry: &UsernameRegistry,
        platform_registry: &platform::PlatformRegistry,
        platform: &platform::Platform,
        block_list_registry: &block_list::BlockListRegistry,
        config: &PostConfig,
        memory_config: &MemoryConfig,
        content: String,
        media_asset_ids: vector<ID>,
        mut media_urls: Option<vector<String>>,
        mentions: Option<vector<address>>,
        metadata_json: Option<String>,
        payment_per_view: u64,
        promotion_budget: Coin<MYSO>,
        enable_spt: Option<bool>,
        enable_spot: Option<bool>,
        mydata_registry: &mydata::MyDataRegistry,
        memory_account: &MemoryAccount,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let access = PostAccess::Public;
        let acting = resolve_social_actor(
            memory_config,
            registry,
            platform,
            block_list_registry,
            memory_account,
            memory::cap_post_publish(),
            0,
            clock,
            ctx,
        );
        let owner = memory::acting_principal_owner(&acting);
        let profile_id = memory::acting_profile_id(&acting);
        let actor_address = memory::acting_actor_address(&acting);
        let sub_agent_id = memory::acting_sub_agent_id(&acting);
        let organization_id = memory::acting_organization_id(&acting);
        let action_identity_class = memory::acting_identity_class(&acting);
        
        // Validate promotion parameters
        assert!(payment_per_view >= config.min_promotion_amount, EPromotionAmountTooLow);
        assert!(payment_per_view <= config.max_promotion_amount, EPromotionAmountTooHigh);
        assert!(coin::value(&promotion_budget) >= payment_per_view, EInsufficientPromotionFunds);
        
        assert_post_access_mydata_binding(owner, &access, mydata_registry);
        
        // Check if platform is approved 
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), EUnauthorized);
        
        // Validate content length using config
        assert!(string::length(&content) <= config.max_content_length, EContentTooLarge);
        
        // Validate and convert media URLs if provided
        let media_option = if (option::is_some(&media_urls)) {
            let url_strings = option::extract(&mut media_urls);
            assert!(vector::length(&url_strings) <= config.max_media_urls, ETooManyMediaUrls);
            
            let mut urls = vector::empty<Url>();
            let mut i = 0;
            while (i < vector::length(&url_strings)) {
                let url_string = vector::borrow(&url_strings, i);
                let url_bytes = string::as_bytes(url_string);
                let url = url::new_unsafe_from_bytes(*url_bytes);
                vector::push_back(&mut urls, url);
                i = i + 1;
            };
            option::some(urls)
        } else {
            option::none()
        };
        
        // Validate mentions if provided using config
        if (option::is_some(&mentions)) {
            let mentions_ref = option::borrow(&mentions);
            assert!(vector::length(mentions_ref) <= config.max_mentions, EContentTooLarge);
        };
        
        // Validate metadata if provided using config
        if (option::is_some(&metadata_json)) {
            let metadata_string = option::borrow(&metadata_json);
            assert!(string::length(metadata_string) <= config.max_metadata_size, EContentTooLarge);
        };
        
        // Create promotion data (starts as inactive until platform activates it)
        let mut promotion_data = PromotionData {
            id: object::new(ctx),
            post_id: @0x0, // Will be set after post creation
            payment_per_view,
            promotion_budget: coin::into_balance(promotion_budget),
            paid_viewers: table::new(ctx),
            views: vector::empty(),
            active: false, // Starts inactive until platform approves
            created_at: clock::timestamp_ms(clock),
        };
        
        let promotion_id = object::uid_to_address(&promotion_data.id);
        
        // Set defaults for feature flags (default to opt-out - users must explicitly opt-in)
        let final_enable_spt = if (option::is_some(&enable_spt)) {
            *option::borrow(&enable_spt)
        } else {
            false // Default to opt-out (user must explicitly opt-in)
        };
        assert!(!final_enable_spt, ESptRequiresDedicatedCreate);
        // enable_spot retained for entry-signature compatibility; SPoT is always-on.
        let _ = enable_spot;
        // Convert media URLs to strings for PostCreatedEvent (before moving media_option)
        let media_urls_for_event = convert_urls_to_strings(&media_option);
        let media_asset_ids_for_event = media_asset_ids;

        let post = create_post_internal(
            owner,
            profile_id,
            platform_id,
            content,
            media_asset_ids,
            media_option,
            mentions,
            metadata_json,
            string::utf8(POST_TYPE_STANDARD),
            option::none(),
            true, // allow_comments
            true, // allow_reactions
            true, // allow_reposts
            true, // allow_quotes
            true, // allow_tips
            access,
            option::some(promotion_id),
            false,
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
            clock,
            ctx
        );
        
        // Indexers read PostCreatedEvent to upsert `posts` with promotion_id before PromotedPostCreatedEvent
        emit_post_media_asset_usages(&post, clock);
        let post_id = share_post(post);

        event::emit(PostCreatedEvent {
            post_id,
            owner,
            profile_id,
            platform_id,
            permissions: permissions_bitfield(true, true, true, true, true),
            content,
            post_type: string::utf8(POST_TYPE_STANDARD),
            parent_post_id: option::none(),
            mentions,
            media_asset_ids: media_asset_ids_for_event,
            media_urls: media_urls_for_event,
            metadata_json,
            access,
            promotion_id: option::some(promotion_id),
            composition_status: media_asset::composition_none(),
            monetization_status: media_asset::monetization_none(),
            enable_spt: false,
            spt_id: option::none(),
            actor_address,
            sub_agent_id,
            organization_id,
            action_identity_class,
        });
        
        // Update promotion data with post ID
        promotion_data.post_id = post_id;
        
        // Get budget value before moving the promotion_data
        let total_budget = balance::value(&promotion_data.promotion_budget);
        
        // Share promotion data
        transfer::share_object(promotion_data);
        
        // Emit promoted post creation event
        event::emit(PromotedPostCreatedEvent {
            post_id,
            owner,
            profile_id,
            payment_per_view,
            total_budget,
            created_at: clock::timestamp_ms(clock),
        });
    }

    /// Confirm that one viewer was paid for N (≥1) promoted post views in a single transaction.
    /// Takes promotions by value (MakeMoveVec), merges nets into one transfer, then re-shares each
    /// `PromotionData`. Fees come out of each view's `payment_per_view` gross.
    public fun confirm_promoted_post_views(
        mut promotions: vector<PromotionData>,
        view_durations: vector<u64>,
        config: &PostConfig,
        platform_obj: &mut platform::Platform,
        group: &PermissionedGroup<platform::PlatformPackage>,
        treasury: &EcosystemTreasury,
        viewer_address: address,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        assert!(
            platform::has_moderator_permission<platform::PlatformPromotionAdmin>(group, platform_obj, caller),
            EUnauthorized,
        );

        let n = vector::length(&promotions);
        assert!(n > 0, EInvalidBatch);
        assert!(n == vector::length(&view_durations), EInvalidBatch);
        assert!(n <= MAX_PROMOTION_VIEW_BATCH, EInvalidBatch);

        assert!(platform::has_joined_platform(platform_obj, viewer_address), EUserNotJoinedPlatform);
        assert!(viewer_address != caller, EUnauthorized);

        let platform_id = object::uid_to_address(platform::id(platform_obj));
        let timestamp = clock::timestamp_ms(clock);

        let mut items = vector::empty<PromotedViewConfirmItem>();
        let mut total_payment_amount = 0u64;
        let mut total_platform_fee = 0u64;
        let mut total_ecosystem_fee = 0u64;
        let mut total_recipient_amount = 0u64;
        let mut merged_payment: Option<Coin<MYSO>> = option::none();

        let mut i = 0u64;
        while (i < n) {
            let promotion_data = vector::borrow_mut(&mut promotions, i);
            let view_duration = *vector::borrow(&view_durations, i);

            assert!(promotion_data.active, EPromotionInactive);
            assert!(view_duration >= config.min_view_duration_ms, EInvalidViewDuration);
            assert!(!table::contains(&promotion_data.paid_viewers, viewer_address), EUserAlreadyViewed);
            assert!(
                balance::value(&promotion_data.promotion_budget) >= promotion_data.payment_per_view,
                EInsufficientPromotionFunds,
            );

            let promotion_id = object::uid_to_address(&promotion_data.id);
            let post_id = promotion_data.post_id;

            vector::push_back(&mut promotion_data.views, PromotionView {
                viewer: viewer_address,
                view_duration,
                view_timestamp: timestamp,
                platform_id,
            });
            table::add(&mut promotion_data.paid_viewers, viewer_address, true);

            let gross = promotion_data.payment_per_view;
            let platform_fee = (gross * config.platform_fee_bps) / BPS_DENOM;
            let ecosystem_fee = (gross * config.ecosystem_fee_bps) / BPS_DENOM;
            let recipient_amount = gross - platform_fee - ecosystem_fee;

            assert!(total_payment_amount <= MAX_U64 - gross, EOverflow);
            assert!(total_platform_fee <= MAX_U64 - platform_fee, EOverflow);
            assert!(total_ecosystem_fee <= MAX_U64 - ecosystem_fee, EOverflow);
            assert!(total_recipient_amount <= MAX_U64 - recipient_amount, EOverflow);
            total_payment_amount = total_payment_amount + gross;
            total_platform_fee = total_platform_fee + platform_fee;
            total_ecosystem_fee = total_ecosystem_fee + ecosystem_fee;
            total_recipient_amount = total_recipient_amount + recipient_amount;

            let payment = coin::from_balance(
                balance::split(&mut promotion_data.promotion_budget, gross),
                ctx,
            );
            if (option::is_none(&merged_payment)) {
                option::fill(&mut merged_payment, payment);
            } else {
                coin::join(option::borrow_mut(&mut merged_payment), payment);
            };

            if (balance::value(&promotion_data.promotion_budget) < promotion_data.payment_per_view) {
                promotion_data.active = false;
            };

            vector::push_back(&mut items, PromotedViewConfirmItem {
                post_id,
                promotion_id,
                payment_amount: gross,
                platform_fee,
                ecosystem_fee,
                recipient_amount,
                view_duration,
            });

            i = i + 1;
        };

        let mut payment = option::extract(&mut merged_payment);
        option::destroy_none(merged_payment);

        if (total_ecosystem_fee > 0) {
            let eco_coin = coin::split(&mut payment, total_ecosystem_fee, ctx);
            transfer::public_transfer(eco_coin, profile::get_treasury_address(treasury));
        };

        if (total_platform_fee > 0) {
            let mut platform_coin = coin::split(&mut payment, total_platform_fee, ctx);
            platform::add_to_treasury(platform_obj, &mut platform_coin, total_platform_fee, clock, ctx);
            coin::destroy_zero(platform_coin);
        };

        transfer::public_transfer(payment, viewer_address);

        event::emit(PromotedPostViewsBatchConfirmedEvent {
            viewer: viewer_address,
            platform_id,
            timestamp,
            items,
            total_payment_amount,
            total_platform_fee,
            total_ecosystem_fee,
            total_recipient_amount,
        });

        while (!vector::is_empty(&promotions)) {
            let promotion_data = vector::pop_back(&mut promotions);
            transfer::share_object(promotion_data);
        };
        vector::destroy_empty(promotions);
    }


    /// Toggle promotion status (platform can activate, both platform and owner can deactivate)
    /// Use with activate: false to deactivate promotions
    public fun toggle_promotion_status(
        post: &Post,
        promotion_data: &mut PromotionData,
        platform_obj: &platform::Platform,
        group: &PermissionedGroup<platform::PlatformPackage>,
        activate: bool,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        
        // Verify the post is promoted
        assert!(option::is_some(&post.promotion_id), ENotPromotedPost);
        let post_promotion_id = *option::borrow(&post.promotion_id);
        assert!(post_promotion_id == object::uid_to_address(&promotion_data.id), ENotPromotedPost);
        
        if (activate) {
            assert!(
                platform::has_moderator_permission<platform::PlatformPromotionAdmin>(group, platform_obj, caller),
                EUnauthorized,
            );
        } else {
            let is_platform = platform::is_moderator(group, platform_obj, caller);
            let is_owner = caller == post.owner;
            assert!(is_platform || is_owner, EUnauthorized);
        };
        
        promotion_data.active = activate;
        
        // Emit status change event
        event::emit(PromotionStatusToggledEvent {
            post_id: post_promotion_id,
            toggled_by: caller,
            new_status: activate,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Withdraw all MYSO tokens from promotion (owner only, deactivates promotion)
    #[allow(lint(self_transfer))]
    public fun withdraw_promotion_funds(
        post: &Post,
        promotion_data: &mut PromotionData,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        let caller = tx_context::sender(ctx);
        
        // Verify caller is post owner
        assert!(caller == post.owner, EUnauthorized);
        
        // Verify the post is promoted
        assert!(option::is_some(&post.promotion_id), ENotPromotedPost);
        let post_promotion_id = *option::borrow(&post.promotion_id);
        assert!(post_promotion_id == object::uid_to_address(&promotion_data.id), ENotPromotedPost);
        
        // Get remaining funds
        let remaining_amount = balance::value(&promotion_data.promotion_budget);
        
        // Extract all remaining balance and transfer to owner
        let withdrawn_balance = balance::withdraw_all(&mut promotion_data.promotion_budget);
        let withdrawn_coins = coin::from_balance(withdrawn_balance, ctx);
        transfer::public_transfer(withdrawn_coins, caller);
        
        // Deactivate promotion
        promotion_data.active = false;
        
        // Emit withdrawal event
        event::emit(PromotionFundsWithdrawnEvent {
            post_id: post_promotion_id,
            owner: caller,
            withdrawn_amount: remaining_amount,
            timestamp: clock::timestamp_ms(clock),
        });
    }

    /// Get promotion statistics for a post
    public fun get_promotion_stats(promotion_data: &PromotionData): (u64, u64, bool, u64) {
        (
            promotion_data.payment_per_view,
            balance::value(&promotion_data.promotion_budget),
            promotion_data.active,
            vector::length(&promotion_data.views)
        )
    }

    /// Check if a user has already been paid for viewing a promoted post
    public fun has_user_viewed_promoted_post(promotion_data: &PromotionData, user: address): bool {
        table::contains(&promotion_data.paid_viewers, user)
    }

    /// Get the promotion ID from a post
    public fun get_promotion_id(post: &Post): Option<address> {
        post.promotion_id
    }

    /// Set moderation status for a post (platform devs/mods only)
    public fun set_moderation_status(
        post: &mut Post,
        platform: &platform::Platform,
        group: &PermissionedGroup<platform::PlatformPackage>,
        platform_registry: &platform::PlatformRegistry,
        status: u8, // MODERATION_APPROVED or MODERATION_FLAGGED
        reason: Option<String>,
        clock: &Clock,
        ctx: &mut TxContext
    ) {
        // Check version compatibility
        assert!(post.version == upgrade::current_version(), EWrongVersion);
        assert!(platform::platform_version(platform) == upgrade::current_version(), EWrongVersion);
        
        let caller = tx_context::sender(ctx);
        assert!(
            platform::has_moderator_permission<platform::PlatformContentModerator>(group, platform, caller),
            EUnauthorized,
        );
        
        // Verify platform is approved
        let platform_id = object::uid_to_address(platform::id(platform));
        assert!(platform::is_approved(platform_registry, platform_id), EUnauthorized);
        
        // Validate status
        assert!(status == MODERATION_APPROVED || status == MODERATION_FLAGGED, EUnauthorized);
        
        // Update post status based on moderation decision
        if (status == MODERATION_FLAGGED) {
            post.removed_from_platform = true;
        } else {
            post.removed_from_platform = false;
        };
        
        // Create or update moderation record
        let moderation_record = ModerationRecord {
            id: object::new(ctx),
            post_id: object::uid_to_address(&post.id),
            platform_id: object::uid_to_address(platform::id(platform)),
            moderation_state: status,
            moderator: option::some(caller),
            moderation_timestamp: option::some(clock::timestamp_ms(clock)),
            reason,
        };
        
        transfer::share_object(moderation_record);
        
        // Emit moderation event
        event::emit(PostModerationEvent {
            post_id: object::uid_to_address(&post.id),
            platform_id: object::uid_to_address(platform::id(platform)),
            removed: (status == MODERATION_FLAGGED),
            moderated_by: caller,
        });
    }

    /// Check if content is approved (not flagged)
    public fun is_content_approved(post: &Post): bool {
        !post.removed_from_platform
    }

    public struct PostSubscriptionAccessEvent has copy, drop {
        post_id: ID,
        service_id: ID,
        subscription_id: ID,
        subscriber: address,
        timestamp: u64,
    }

    /// Abort unless caller is post owner or holds a valid profile subscription for the gated service.
    public entry fun assert_can_view_post(
        block_list_registry: &BlockListRegistry,
        post: &Post,
        service: &ProfileSubscriptionService,
        subscription: &ProfileSubscription,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        let sender = tx_context::sender(ctx);
        if (sender == post.owner) {
            return
        };
        block_list::assert_not_blocked(block_list_registry, sender, post.owner);
        assert!(requires_profile_subscription(post), ENoSubscriptionService);
        let service_id = subscription_service(post);
        assert!(option::is_some(&service_id), ENoSubscriptionService);
        assert!(*option::borrow(&service_id) == object::id(service), ENoSubscriptionService);
        let min_tier = subscription_min_tier_level(post);
        let content_platform_id = option::some(post.platform_id);
        assert!(
            subscription::subscription_satisfies_access(
                subscription,
                service,
                sender,
                min_tier,
                content_platform_id,
                clock,
            ),
            EUnauthorized,
        );
    }

    /// Record a subscriber view after access check; emits `PostSubscriptionAccessEvent`.
    public entry fun record_post_subscription_view(
        block_list_registry: &BlockListRegistry,
        post: &Post,
        service: &ProfileSubscriptionService,
        subscription: &ProfileSubscription,
        clock: &Clock,
        ctx: &mut TxContext,
    ) {
        assert_can_view_post(block_list_registry, post, service, subscription, clock, ctx);
        event::emit(PostSubscriptionAccessEvent {
            post_id: object::id(post),
            service_id: object::id(service),
            subscription_id: object::id(subscription),
            subscriber: tx_context::sender(ctx),
            timestamp: clock::timestamp_ms(clock),
        });
    }

    #[test_only]
    public fun set_comment_count_for_testing(post: &mut Post, count: u64) {
        post.comment_count = count;
    }

    #[test_only]
    public fun test_share_profile_subscription_post_with_tier(
        owner: address,
        service_id: ID,
        min_tier_level: Option<u64>,
        clock: &Clock,
        ctx: &mut TxContext,
    ): address {
        share_post(create_post_internal(
            owner,
            owner,
            owner,
            string::utf8(b"premium post"),
            vector[],
            option::none(),
            option::none(),
            option::none(),
            string::utf8(POST_TYPE_STANDARD),
            option::none(),
            true,
            true,
            true,
            true,
            true,
            PostAccess::ProfileSubscription {
                service_id,
                mydata_id: option::none(),
                min_tier_level,
            },
            option::none(),
            false,
            owner,
            option::none(),
            option::none(),
            memory::class_human(),
            clock,
            ctx,
        ))
    }

    #[test_only]
    public fun test_share_profile_subscription_post(
        owner: address,
        service_id: ID,
        clock: &Clock,
        ctx: &mut TxContext,
    ): address {
        share_post(create_post_internal(
            owner,
            owner,
            owner,
            string::utf8(b"premium post"),
            vector[],
            option::none(),
            option::none(),
            option::none(),
            string::utf8(POST_TYPE_STANDARD),
            option::none(),
            true,
            true,
            true,
            true,
            true,
            PostAccess::ProfileSubscription {
                service_id,
                mydata_id: option::none(),
                min_tier_level: option::none(),
            },
            option::none(),
            false,
            owner,
            option::none(),
            option::none(),
            memory::class_human(),
            clock,
            ctx,
        ))
    }
    
    /// Create a PostAdminCap for bootstrap (package visibility only)
    /// This function is only callable by other modules in the same package
    public(package) fun create_post_admin_cap(ctx: &mut TxContext): PostAdminCap {
        PostAdminCap {
            id: object::new(ctx)
        }
    }
    
    #[test_only]
    /// Gross fee split used by confirmed promo views (for unit tests).
    public fun test_promotion_view_fee_amounts(config: &PostConfig, gross: u64): (u64, u64, u64) {
        let platform_fee = (gross * config.platform_fee_bps) / BPS_DENOM;
        let ecosystem_fee = (gross * config.ecosystem_fee_bps) / BPS_DENOM;
        let recipient_amount = gross - platform_fee - ecosystem_fee;
        (platform_fee, ecosystem_fee, recipient_amount)
    }

    #[test_only]
    public fun test_activate_promotion(promotion_data: &mut PromotionData) {
        promotion_data.active = true;
    }

    #[test_only]
    public fun test_platform_fee_bps(config: &PostConfig): u64 {
        config.platform_fee_bps
    }

    #[test_only]
    public fun test_ecosystem_fee_bps(config: &PostConfig): u64 {
        config.ecosystem_fee_bps
    }
    
    #[test_only]
    public fun test_denial_scope_playback(): u8 { DENIAL_SCOPE_PLAYBACK }

    #[test_only]
    public fun test_embedded_binding(
        binding_id: u64,
        source_asset_id: ID,
        usage_class: u8,
    ): EmbeddedAssetBinding {
        EmbeddedAssetBinding {
            binding_id,
            source_asset_id,
            usage_class,
            stem: 0,
            media_component: 0,
            evidence_commitment: option::none(),
        }
    }

    #[test_only]
    public fun test_decision_playback_permitted(post: &Post, binding_id: u64): bool {
        let decisions = usage_decisions(post);
        let len = vector::length(&decisions);
        let mut i = 0;
        while (i < len) {
            let d = vector::borrow(&decisions, i);
            if (d.binding_id == binding_id) {
                return d.playback_permitted
            };
            i = i + 1;
        };
        false
    }

    #[test_only]
    public fun test_composition_status(post: &Post): u8 {
        post.composition_status
    }

    #[test_only]
    /// Initialize the post module for testing
    /// In testing, we create admin caps directly for convenience
    public fun init_for_testing(ctx: &mut TxContext) {
        let sender = tx_context::sender(ctx);
        
        // Create and transfer admin capability to the transaction sender
        transfer::public_transfer(
            PostAdminCap {
                id: object::new(ctx),
            },
            sender
        );
        
        // Create and share post configuration (same as production init)
        transfer::share_object(
            PostConfig {
                id: object::new(ctx),
                max_content_length: MAX_CONTENT_LENGTH,
                max_media_urls: MAX_MEDIA_URLS,
                max_mentions: MAX_MENTIONS,
                max_metadata_size: MAX_METADATA_SIZE,
                max_description_length: MAX_DESCRIPTION_LENGTH,
                max_reaction_length: MAX_REACTION_LENGTH,
                commenter_tip_percentage: COMMENTER_TIP_PERCENTAGE,
                repost_tip_percentage: REPOST_TIP_PERCENTAGE,
                min_promotion_amount: MIN_PROMOTION_AMOUNT,
                max_promotion_amount: MAX_PROMOTION_AMOUNT,
                min_view_duration_ms: MIN_VIEW_DURATION,
                platform_fee_bps: DEFAULT_PLATFORM_FEE_BPS,
                ecosystem_fee_bps: DEFAULT_ECOSYSTEM_FEE_BPS,
                version: upgrade::current_version(),
            }
        );
    }
}
