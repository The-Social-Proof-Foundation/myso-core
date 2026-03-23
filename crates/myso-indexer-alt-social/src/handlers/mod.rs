// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Social events pipeline: processes myso-social events from checkpoints into social tables.
//!
//! Filters events by MYSO_SOCIAL_PACKAGE_ID, routes by module/event name, and inserts into
//! profiles, social_graph_relationships, social_graph_events, etc.

mod blocking;
mod events;
mod governance;
mod insurance;
mod mydata;
mod platform;
mod poc;
mod post;
mod profile;
mod social_graph;
mod spot;
mod spt;
mod subscription;
mod upgrade;

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Bool, Int2, Nullable, Text};
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use move_core_types::account_address::AccountAddress;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    GovernanceRegistryUpdate, NewAnonymousVote, NewBlockedEvent, NewBlockedProfile, NewComment,
    NewCommunityVote, NewDelegate, NewDelegateRating, NewDelegateVote, NewDeletionEvent,
    NewEcosystemTreasury, NewGovernanceEvent, NewGovernanceRegistry, NewInsuranceConfig,
    NewInsuranceEventLog, NewInsuranceMarketExposure, NewInsurancePolicy, NewInsurancePolicyEvent,
    NewInsuranceUserExposure, NewInsuranceVault, NewInsuranceVaultTransaction, NewModerationEvent,
    NewMyDataAccessLog, NewMyDataConfig, NewMyDataData, NewMyDataPurchase, NewMyDataRegistry,
    NewMyDataRevenue, NewMyDataSubscription, NewNominatedDelegate, NewObjectMigratedEvent,
    NewPlatform, NewPlatformBlockedProfile, NewPlatformEvent, NewPlatformMembership,
    NewPlatformModerator, NewPlatformTokenAirdrop, NewPocAnalysisResult, NewPocBadge,
    NewPocConfiguration, NewPocDispute, NewPocDisputeVote, NewPocRevenueRedirection, NewPost,
    NewPostTransfer, NewProfile, NewProfileBadge, NewProfileEvent, NewProfileOffer,
    NewProfileSaleFee, NewProfileSubscription, NewProfileSubscriptionService, NewPromotedPost,
    NewPromotionBudgetEvent, NewPromotionStatusEvent, NewPromotionView, NewProposal, NewReaction,
    NewReactionCount, NewReport, NewRepost, NewRewardDistribution, NewSocialGraphEvent,
    NewSocialGraphRelationship, NewSocialProofTokensConfig, NewSocialProofTokensEvent, NewSpotBet,
    NewSpotBetWithdrawal, NewSpotConfig, NewSpotEventLog, NewSpotPayout, NewSpotRecord,
    NewSpotRefund, NewSpotResolution, NewSptExchangeConfig, NewSptHolding, NewSptPool,
    NewSptPriceHistory, NewSptReservation, NewSptReservationPool, NewSptRevenue, NewSptTransaction,
    NewSubscriptionEvent, NewSubscriptionRevenue, NewTip, NewUnifiedRevenue, NewUpgradeEvent,
    NewVestingEvent, NewVestingWallet, NewVoteDecryptionFailure, ProfileUpdateSet,
    ProposalUpdateSet, RESERVATION_POOL_STATUS_ACTIVE, RESERVATION_POOL_STATUS_THRESHOLD_MET,
    TOKEN_TYPE_POST,
};
use myso_indexer_alt_social_schema::schema::{
    anonymous_votes, blocked_events, blocked_profiles, comments, community_votes, delegate_ratings,
    delegate_votes, delegates, governance_events, governance_registries, nominated_delegates,
    platform_blocked_profiles, platform_events, platform_memberships, platform_moderators,
    platform_token_airdrops, platforms, poc_analysis_results, poc_badges, poc_configuration,
    poc_dispute_votes, poc_disputes, poc_revenue_redirections, post_config, posts,
    posts_deletion_events, posts_moderation_events, posts_reports, posts_transfers, profile_badges,
    profile_events, profile_offers, profile_sale_fees, profiles, promoted_posts,
    promotion_budget_events, promotion_status_events, promotion_views, proposals, reaction_counts,
    reactions, reposts, reward_distributions, social_graph_events, social_graph_relationships,
    tips, vote_decryption_failures,
};
use myso_indexer_alt_social_schema::schema::{
    ecosystem_treasury, object_migrated_events, social_proof_tokens_config,
    social_proof_tokens_events, spt_exchange_config, spt_holdings, spt_pools, spt_price_history,
    spt_reservation_pools, spt_reservations, spt_revenue, spt_transactions, unified_revenue,
    upgrade_events,
};
use myso_indexer_alt_social_schema::schema::{
    insurance_config, insurance_events, insurance_market_exposures, insurance_policies,
    insurance_policy_events, insurance_user_exposures, insurance_vault_transactions,
    insurance_vaults, mydata_access_logs, mydata_config, mydata_data, mydata_purchases,
    mydata_registry, mydata_revenue, mydata_subscriptions,
};
use myso_indexer_alt_social_schema::schema::{
    profile_subscription_services, profile_subscriptions, subscription_events, subscription_revenue,
};
use myso_indexer_alt_social_schema::schema::{
    spot_bet_withdrawals, spot_bets, spot_config, spot_events, spot_payouts, spot_records,
    spot_refunds, spot_resolutions,
};
use myso_indexer_alt_social_schema::schema::{vesting_events, vesting_wallets};
use myso_types::base_types::ObjectID;
use myso_types::MYSO_SOCIAL_PACKAGE_ID;
use tracing::{debug, info, warn};

fn is_social_package_event(package_id: &ObjectID, type_address: &AccountAddress) -> bool {
    use std::ops::Deref;
    *package_id == MYSO_SOCIAL_PACKAGE_ID || type_address == MYSO_SOCIAL_PACKAGE_ID.deref()
}

pub struct SocialEvents;

#[derive(Debug, Clone)]
pub enum SocialEventRow {
    Profile(NewProfile),
    ProfileUpdate(ProfileUpdate),
    EcosystemTreasury(NewEcosystemTreasury),
    SocialGraphRelationship(NewSocialGraphRelationship),
    SocialGraphEvent(NewSocialGraphEvent),
    SocialGraphUnfollow {
        follower_address: String,
        following_address: String,
    },
    BlockedEvent(NewBlockedEvent),
    BlockedProfile(NewBlockedProfile),
    BlockedProfileDelete {
        blocker_address: String,
        blocked_address: String,
    },
    ProfileEvent(NewProfileEvent),
    ProfileOffer(NewProfileOffer),
    ProfileOfferStatusUpdate {
        profile_id: String,
        offeror_address: String,
        status: String,
        resolved_at: i64,
        updated_at: i64,
        transaction_id: String,
    },
    ProfileSaleFee(NewProfileSaleFee),
    ProfileBadge(NewProfileBadge),
    ProfileBadgeRevoke {
        profile_id: String,
        badge_id: String,
        revoked_at: i64,
        revoked_by: String,
    },
    GovernanceRegistry(NewGovernanceRegistry),
    GovernanceRegistryUpdate(GovernanceRegistryUpdate),
    NominatedDelegate(NewNominatedDelegate),
    Delegate(NewDelegate),
    Proposal(NewProposal),
    ProposalUpdate {
        proposal_id: String,
        set: ProposalUpdateSet,
        governance_event: Option<(String, serde_json::Value, String)>,
        submitter_filter: Option<String>,
    },
    DelegateRating(NewDelegateRating),
    DelegateVote(NewDelegateVote),
    CommunityVote(NewCommunityVote),
    RewardDistribution(NewRewardDistribution),
    GovernanceEvent(NewGovernanceEvent),
    GovernanceEventFromProposal {
        proposal_id: String,
        event_type: String,
        event_data: serde_json::Value,
        event_id: String,
        anonymous_voting_related: Option<bool>,
    },
    AnonymousVote(NewAnonymousVote),
    VoteDecryptionFailure(NewVoteDecryptionFailure),
    Post(NewPost),
    Comment(NewComment),
    Reaction(NewReaction),
    ReactionCount(NewReactionCount),
    RemoveReaction {
        object_id: String,
        user_address: String,
        reaction_text: String,
        is_post: bool,
    },
    Repost(NewRepost),
    Tip(NewTip),
    ModerationEvent(NewModerationEvent),
    Report(NewReport),
    DeletionEvent(NewDeletionEvent),
    PostCommentCountIncrement {
        post_id: String,
        delta: i64,
    },
    PostCommentCountDecrementByComment {
        comment_id: String,
        owner: String,
    },
    ProfilePostCountIncrement {
        owner_address: String,
    },
    ProfilePostCountDecrement {
        owner_address: String,
    },
    PostRepostCountIncrement {
        original_id: String,
        is_original_post: bool,
    },
    PostTipsReceivedIncrement {
        object_id: String,
        amount: i64,
        is_post: bool,
    },
    PostModerationUpdate {
        object_id: String,
        removed: bool,
        moderated_by: String,
    },
    PostDeletedAtUpdate {
        object_id: String,
        owner: String,
        deleted_at: i64,
    },
    CommentDeletedAtUpdate {
        object_id: String,
        owner: String,
        deleted_at: i64,
    },
    PostContentUpdate {
        object_id: String,
        content: String,
        media_urls: Option<serde_json::Value>,
        mentions: Option<serde_json::Value>,
        metadata_json: Option<serde_json::Value>,
        is_post: bool,
        updated_at: i64,
    },
    PostOwnerUpdate {
        object_id: String,
        new_owner: String,
        is_post: bool,
    },
    PostTransfer(NewPostTransfer),
    PostConfig {
        updated_by: String,
        max_content_length: i64,
        max_media_urls: i64,
        max_mentions: i64,
        max_metadata_size: i64,
        max_description_length: i64,
        max_reaction_length: i64,
        commenter_tip_percentage: i64,
        repost_tip_percentage: i64,
        version: Option<i64>,
        updated_at: i64,
        transaction_id: String,
    },
    PromotedPost {
        post_id: String,
        owner: String,
        profile_id: String,
        payment_per_view: i64,
        total_budget: i64,
        created_at: i64,
        transaction_id: String,
    },
    PromotionView {
        promotion_id: String,
        viewer: String,
        payment_amount: i64,
        view_duration: i64,
        platform_id: String,
        timestamp: i64,
        transaction_id: String,
    },
    PromotionStatusEvent {
        promotion_id: String,
        toggled_by: String,
        new_status: bool,
        timestamp: i64,
        transaction_id: String,
    },
    PromotionBudgetEvent {
        promotion_id: String,
        owner: String,
        withdrawn_amount: i64,
        timestamp: i64,
        transaction_id: String,
    },
    Platform(NewPlatform),
    PlatformUpdate {
        platform_id: String,
        name: String,
        tagline: String,
        description: Option<String>,
        terms_of_service: Option<String>,
        privacy_policy: Option<String>,
        platform_names: Option<serde_json::Value>,
        links: Option<serde_json::Value>,
        status: i16,
        release_date: Option<String>,
        shutdown_date: Option<String>,
        updated_at: chrono::NaiveDateTime,
        primary_category: String,
        secondary_category: Option<String>,
    },
    PlatformApprovalChange {
        platform_id: String,
        is_approved: bool,
        approved_by: String,
        changed_at: chrono::NaiveDateTime,
    },
    PlatformModerator(NewPlatformModerator),
    PlatformModeratorRemove {
        platform_id: String,
        moderator_address: String,
    },
    PlatformBlockedProfile(NewPlatformBlockedProfile),
    PlatformBlockedProfileRemove {
        platform_id: String,
        wallet_address: String,
    },
    PlatformMembership(NewPlatformMembership),
    PlatformMembershipRemove {
        platform_id: String,
        wallet_address: String,
    },
    PlatformTokenAirdrop(NewPlatformTokenAirdrop),
    PlatformEvent(NewPlatformEvent),
    PlatformDeleted {
        platform_id: String,
        deleted_at: chrono::NaiveDateTime,
    },
    PocBadge(NewPocBadge),
    PocAnalysisResult(NewPocAnalysisResult),
    PocRevenueRedirection(NewPocRevenueRedirection),
    PocDispute(NewPocDispute),
    PocDisputeVote(NewPocDisputeVote),
    PocConfiguration(NewPocConfiguration),
    PostPocUpdate {
        post_id: String,
        poc_reasoning: Option<String>,
        poc_evidence_urls: Option<serde_json::Value>,
        poc_similarity_score: Option<i64>,
        poc_media_type: Option<i16>,
        poc_oracle_address: Option<String>,
        poc_analyzed_at: Option<i64>,
    },
    PostRevenueRedirectUpdate {
        post_id: String,
        revenue_redirect_to: String,
        revenue_redirect_percentage: i64,
    },
    PocDisputeResolved {
        dispute_id: String,
        post_id: String,
        resolution: i16,
        winning_side: i16,
        total_winning_stake: i64,
        total_losing_stake: i64,
        resolved_at: i64,
        badge_revoked: bool,
        redirection_removed: bool,
    },
    PocVoteRewardClaimed {
        dispute_id: String,
        voter: String,
        reward_amount: i64,
    },
    MyDataData(NewMyDataData),
    MyDataPurchase(NewMyDataPurchase),
    MyDataSubscription(NewMyDataSubscription),
    MyDataRevenue(NewMyDataRevenue),
    MyDataAccessLog(NewMyDataAccessLog),
    MyDataRegistry(NewMyDataRegistry),
    MyDataRegistryUpdate {
        ip_id: String,
        owner: String,
        unregistered_at: i64,
        transaction_id: String,
    },
    MyDataConfig(NewMyDataConfig),
    MyDataContentUpdate {
        mydata_id: String,
        last_updated: i64,
        transaction_id: String,
    },
    InsuranceConfig(NewInsuranceConfig),
    InsuranceVault(NewInsuranceVault),
    InsuranceVaultTransaction(NewInsuranceVaultTransaction),
    InsuranceVaultBalanceUpdate {
        vault_id: String,
        new_balance: i64,
    },
    InsurancePolicy(NewInsurancePolicy),
    InsurancePolicyEvent(NewInsurancePolicyEvent),
    InsuranceMarketExposure(NewInsuranceMarketExposure),
    InsuranceUserExposure(NewInsuranceUserExposure),
    InsuranceEventLog(NewInsuranceEventLog),
    InsurancePolicyStatusUpdate {
        policy_id: String,
        status: i16,
    },
    InsurancePolicyEventFromPolicy {
        policy_id: String,
        event_type: String,
        refunded_amount: Option<i64>,
        fee_paid: Option<i64>,
        payout: Option<i64>,
        reserve_released: Option<i64>,
        timestamp_ms: i64,
        transaction_id: String,
    },
    NominatedDelegateStatusUpdate {
        address: String,
        registry_type: i16,
        status: i16,
    },
    DelegateVoteCountsUpdate {
        target_address: String,
        registry_type: i16,
        is_active_delegate: bool,
        upvotes: i64,
        downvotes: i64,
    },
    ProposalDelegateVoteIncrement {
        proposal_id: String,
        approve: bool,
    },
    DelegateProposalsReviewedIncrement {
        address: String,
    },
    ProposalCommunityVoteUpdate {
        proposal_id: String,
        votes_for_delta: i64,
        votes_against_delta: i64,
    },
    DelegateSidedProposalUpdate {
        address: String,
        is_winning: bool,
    },
    ProposalOutcomeApplyDelegateSidedUpdates {
        proposal_id: String,
        approvers_win: bool,
    },
    DelegateProposalsSubmittedIncrement {
        address: String,
        registry_type: i16,
    },
    ProposalAnonymousVotersIncrement {
        proposal_id: String,
    },
    SpotBet(NewSpotBet),
    SpotResolution(NewSpotResolution),
    SpotPayout(NewSpotPayout),
    SpotRefund(NewSpotRefund),
    SpotEventLog(NewSpotEventLog),
    SpotConfig(NewSpotConfig),
    SpotBetWithdrawal(NewSpotBetWithdrawal),
    SpotRecordUpsert(NewSpotRecord),
    SpotRecordUpdate {
        post_id: String,
        status: i16,
        outcome: Option<i16>,
        last_resolution_epoch: i64,
    },
    SptPool(NewSptPool),
    SptTransaction(NewSptTransaction),
    SptHolding(NewSptHolding),
    SptPoolSupplyUpdate {
        pool_id: String,
        delta: i64,
    },
    SptPriceHistory(NewSptPriceHistory),
    SptReservationPool(NewSptReservationPool),
    SptReservation {
        associated_id: String,
        reservation: NewSptReservation,
        token_type: i16,
        total_reserved: i64,
        threshold_met: bool,
        created_at: i64,
    },
    SptReservationPoolUpdate {
        pool_id: String,
        associated_id: String,
        total_reserved: i64,
        status: Option<String>,
        required_threshold: Option<i64>,
    },
    SptExchangeConfig(NewSptExchangeConfig),
    SocialProofTokensConfig(NewSocialProofTokensConfig),
    SocialProofTokensEvent(NewSocialProofTokensEvent),
    SptRevenue(NewSptRevenue),
    UnifiedRevenue(NewUnifiedRevenue),
    SptBuySellRevenueData {
        pool_id: String,
        associated_id: String,
        token_type: i16,
        trader: String,
        transaction_type: String,
        creator_fee: i64,
        platform_fee: i64,
        treasury_fee: i64,
        amount: i64,
        myso_amount: i64,
        token_price: i64,
        revenue_time: i64,
        transaction_id: String,
    },
    UpgradeEvent(NewUpgradeEvent),
    ObjectMigratedEvent(NewObjectMigratedEvent),
    VestingWallet(NewVestingWallet),
    VestingEvent(NewVestingEvent),
    VestingWalletClaimUpdate {
        wallet_id: String,
        claimed_amount: i64,
        remaining_balance: i64,
    },
    VestingWalletDelete {
        wallet_id: String,
    },
    ProfileSubscriptionService(NewProfileSubscriptionService),
    ProfileSubscription(NewProfileSubscription),
    SubscriptionEvent(NewSubscriptionEvent),
    SubscriptionRevenue(NewSubscriptionRevenue),
    ProfileSubscriptionServiceSubscriberIncrement {
        service_id: String,
    },
    ProfileSubscriptionUpdate {
        subscription_id: String,
        expires_at: i64,
        renewal_count: i64,
    },
    ProfileSubscriptionCancel {
        subscription_id: String,
    },
    ProfileSubscriptionServiceUpdate {
        service_id: String,
        monthly_fee: i64,
        updated_at: i64,
    },
    ProfileSubscriptionRenewalBalanceUpdate {
        subscription_id: String,
        new_balance: i64,
    },
    ProfileSubscriptionServiceDeactivate {
        service_id: String,
        updated_at: i64,
    },
    ProfileSubscriptionServiceSubscriberDecrementBySubscription {
        subscription_id: String,
    },
    SubscriptionRevenueFromCreated {
        service_id: String,
        subscription_id: String,
        from_address: String,
        amount: i64,
        revenue_type: String,
        payment_time: i64,
        transaction_id: String,
    },
    SubscriptionRevenueFromRefund {
        subscription_id: String,
        subscriber: String,
        refunded_amount: i64,
        transaction_id: String,
    },
    SubscriptionRevenueFromRenewal {
        subscription_id: String,
        subscriber: String,
        new_expires_at: i64,
        renewal_count: i64,
        auto_renewed: bool,
        transaction_id: String,
    },
}

#[derive(Debug, Clone)]
pub struct ProfileUpdate {
    pub profile_id: String,
    pub owner_address: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub cover_photo: Option<String>,
    pub birthdate: Option<String>,
    pub current_location: Option<String>,
    pub raised_location: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub political_view: Option<String>,
    pub religion: Option<String>,
    pub education: Option<String>,
    pub primary_language: Option<String>,
    pub relationship_status: Option<String>,
    pub x_username: Option<String>,
    pub facebook_username: Option<String>,
    pub reddit_username: Option<String>,
    pub github_username: Option<String>,
    pub instagram_username: Option<String>,
    pub linkedin_username: Option<String>,
    pub twitch_username: Option<String>,
    pub min_offer_amount: Option<i64>,
    pub username: Option<String>,
    pub selected_badge_id: Option<Option<String>>,
    pub selected_ecosystem_badge_id: Option<Option<String>>,
    pub paid_messaging_enabled: Option<bool>,
    pub paid_messaging_min_cost: Option<i64>,
    pub reservation_pool_address: Option<Option<String>>,
}

impl FieldCount for SocialEventRow {
    const FIELD_COUNT: usize = 116;
}

/// Routes a parsed event to the appropriate domain handler based on Move module name.
fn route_event(
    module: &str,
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
    epoch: u64,
    timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    match module {
        "governance" => governance::handle_governance_event(event_name, data, event_id),
        "block_list" | "blocking" => blocking::handle_blocking_event(event_name, data, event_id),
        "mydata" | "my_ip" => mydata::handle_mydata_event(event_name, data, event_id),
        "profile" => profile::handle_profile_event(event_name, data, event_id),
        "social_graph" => social_graph::handle_social_graph_event(event_name, data, event_id),
        "platform" => platform::handle_platform_event(event_name, data, event_id),
        "post" | "comment" | "reaction" | "repost" | "tip" => {
            post::handle_post_event(event_name, data, event_id)
        }
        "subscription" | "profile_subscription" => {
            subscription::handle_subscription_event(event_name, data, event_id)
        }
        "insurance" => insurance::handle_insurance_event(event_name, data, event_id, timestamp_ms),
        "poc" | "proof_of_creativity" => poc::handle_poc_event(event_name, data, event_id),
        "social_proof_of_truth" | "spot" => {
            spot::handle_spot_event(event_name, data, event_id, epoch, timestamp_ms)
        }
        "social_proof_tokens" | "spt" => {
            spt::handle_spt_event(event_name, data, event_id, epoch, timestamp_ms)
        }
        "upgrade" => upgrade::handle_upgrade_event(event_name, data, event_id),
        _ => None,
    }
}

#[async_trait]
impl Processor for SocialEvents {
    const NAME: &'static str = "social_events";

    type Value = SocialEventRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let Checkpoint {
            transactions,
            summary,
            object_set: _,
            ..
        } = checkpoint.as_ref();
        let seq = summary.sequence_number;
        let mut values = Vec::new();
        let mut social_event_count = 0u64;
        let mut profile_event_count = 0u64;

        for tx in transactions.iter() {
            let tx_digest = tx.transaction.digest().to_string();

            let Some(events) = &tx.events else {
                continue;
            };

            for (event_seq, ev) in events.data.iter().enumerate() {
                let package_matches = is_social_package_event(&ev.package_id, &ev.type_.address);
                if package_matches {
                    social_event_count += 1;
                    if ev.type_.module.as_str() == "profile" {
                        profile_event_count += 1;
                    }
                }
                if !package_matches {
                    if ev.type_.module.as_str() == "profile"
                        || ev.type_.module.as_str() == "governance"
                        || ev.type_.module.as_str() == "social_proof_tokens"
                    {
                        warn!(
                            package_id = %ev.package_id,
                            type_address = %ev.type_.address,
                            module = %ev.type_.module,
                            event_name = %ev.type_.name,
                            "skipping event: package mismatch (expected 0x50c1)"
                        );
                    }
                    continue;
                }

                let module = ev.type_.module.as_str();
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);

                if module == "profile" {
                    debug!(
                        event_name = %event_name,
                        event_id = %event_id,
                        contents_len = ev.contents.len(),
                        is_empty = ev.contents.is_empty(),
                        package_id = %ev.package_id,
                        type_address = %ev.type_.address,
                        "profile event received for processing"
                    );
                }
                if module == "social_proof_tokens" {
                    info!(
                        event_name = %event_name,
                        event_id = %event_id,
                        contents_len = ev.contents.len(),
                        is_empty = ev.contents.is_empty(),
                        "SPT event received for processing"
                    );
                }
                if ev.contents.is_empty() && (module == "profile" || module == "social_proof_tokens") {
                    warn!(
                        module = %module,
                        event_name = %event_name,
                        event_id = %event_id,
                        event_type = %ev.type_.to_canonical_string(true),
                        "skipping event: empty contents (proto/ingestion may not populate event.contents)"
                    );
                    continue;
                }

                // Detailed tracking for ProfileCreatedEvent
                if module == "profile" && event_name == "ProfileCreatedEvent" {
                    debug!(
                        event_id = %event_id,
                        contents_len = ev.contents.len(),
                        first_16_bytes = %hex::encode(&ev.contents[..ev.contents.len().min(16)]),
                        "ProfileCreatedEvent attempting BCS parsing"
                    );
                }

                let event_data = match events::parse_event_contents(
                    module,
                    event_name,
                    &ev.contents,
                ) {
                    Ok(v) => v,
                    Err(parse_err) => {
                        let hex_preview = parse_err.contents_hex_preview(128);
                        warn!(
                            module = %module,
                            event_name = %event_name,
                            event_id = %event_id,
                            event_type = %ev.type_.to_canonical_string(true),
                            contents_len = parse_err.contents.len(),
                            parse_error = %parse_err.error,
                            contents_hex_preview = %hex_preview,
                            "skipping event: failed to parse BCS contents (layout may have changed)"
                        );
                        continue;
                    }
                };

                let epoch = summary.epoch;
                let timestamp_ms = summary.timestamp_ms;
                if let Some(rows) = route_event(
                    module,
                    event_name,
                    &event_data,
                    &event_id,
                    epoch,
                    timestamp_ms,
                ) {
                    if module == "social_proof_tokens" && !rows.is_empty() {
                        info!(
                            event_name = %event_name,
                            event_id = %event_id,
                            rows = rows.len(),
                            "SPT event produced rows"
                        );
                    }
                    values.extend(rows);
                } else {
                    if module == "social_proof_tokens" {
                        info!(
                            module = %module,
                            event_name = %event_name,
                            event_id = %event_id,
                            "skipping SPT event: no handler for this module/event"
                        );
                    } else {
                        debug!(
                            module = %module,
                            event_name = %event_name,
                            event_id = %event_id,
                            "skipping event: no handler for this module/event"
                        );
                    }
                }
            }
        }

        if social_event_count > 0 {
            tracing::info!(
                checkpoint = seq,
                social_events = social_event_count,
                profile_events = profile_event_count,
                rows_produced = values.len(),
                "Social indexer processed checkpoint"
            );
        }

        Ok(values)
    }
}

#[async_trait]
impl Handler for SocialEvents {
    const MIN_EAGER_ROWS: usize = 50;
    const MAX_PENDING_ROWS: usize = 5000;

    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;

        for row in values {
            match row {
                SocialEventRow::Profile(profile) => {
                    total += diesel::insert_into(profiles::table)
                        .values(profile)
                        .on_conflict(profiles::owner_address)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileUpdate(up) => {
                    let now = chrono::Utc::now().naive_utc();
                    let set = ProfileUpdateSet {
                        updated_at: now,
                        display_name: up.display_name.clone().map(Some),
                        bio: up.bio.clone().map(Some),
                        profile_photo: up.profile_photo.clone().map(Some),
                        cover_photo: up.cover_photo.clone().map(Some),
                        birthdate: up.birthdate.clone().map(Some),
                        current_location: up.current_location.clone().map(Some),
                        raised_location: up.raised_location.clone().map(Some),
                        phone: up.phone.clone().map(Some),
                        email: up.email.clone().map(Some),
                        gender: up.gender.clone().map(Some),
                        political_view: up.political_view.clone().map(Some),
                        religion: up.religion.clone().map(Some),
                        education: up.education.clone().map(Some),
                        primary_language: up.primary_language.clone().map(Some),
                        relationship_status: up.relationship_status.clone().map(Some),
                        x_username: up.x_username.clone().map(Some),
                        facebook_username: up.facebook_username.clone().map(Some),
                        reddit_username: up.reddit_username.clone().map(Some),
                        github_username: up.github_username.clone().map(Some),
                        instagram_username: up.instagram_username.clone().map(Some),
                        linkedin_username: up.linkedin_username.clone().map(Some),
                        twitch_username: up.twitch_username.clone().map(Some),
                        min_offer_amount: up.min_offer_amount.map(Some),
                        username: up.username.clone(),
                        selected_badge_id: up.selected_badge_id.clone(),
                        selected_ecosystem_badge_id: up.selected_ecosystem_badge_id.clone(),
                        paid_messaging_enabled: up.paid_messaging_enabled,
                        paid_messaging_min_cost: up.paid_messaging_min_cost.map(Some),
                        reservation_pool_address: up.reservation_pool_address.clone(),
                    };
                    let filter = profiles::profile_id
                        .eq(&up.profile_id)
                        .or(profiles::owner_address.eq(&up.owner_address));
                    total += diesel::update(profiles::table)
                        .filter(filter)
                        .set(set)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::BlockedEvent(ev) => {
                    total += diesel::insert_into(blocked_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::BlockedProfile(bp) => {
                    let last_blocked_at = bp.last_blocked_at;
                    let blocked_profile_id = bp.blocked_profile_id.clone();
                    let blocked_username = bp.blocked_username.clone();
                    let blocked_display_name = bp.blocked_display_name.clone();
                    let blocked_profile_photo = bp.blocked_profile_photo.clone();
                    total += diesel::insert_into(blocked_profiles::table)
                        .values(bp)
                        .on_conflict((
                            blocked_profiles::blocker_address,
                            blocked_profiles::blocked_address,
                        ))
                        .do_update()
                        .set((
                            blocked_profiles::blocked_profile_id.eq(blocked_profile_id),
                            blocked_profiles::blocked_username.eq(blocked_username),
                            blocked_profiles::blocked_display_name.eq(blocked_display_name),
                            blocked_profiles::blocked_profile_photo.eq(blocked_profile_photo),
                            blocked_profiles::last_blocked_at.eq(last_blocked_at),
                            blocked_profiles::total_block_count
                                .eq(blocked_profiles::total_block_count + 1),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::BlockedProfileDelete {
                    blocker_address,
                    blocked_address,
                } => {
                    total += diesel::delete(blocked_profiles::table)
                        .filter(blocked_profiles::blocker_address.eq(blocker_address))
                        .filter(blocked_profiles::blocked_address.eq(blocked_address))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileEvent(ev) => {
                    total += diesel::insert_into(profile_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileOffer(offer) => {
                    total += diesel::insert_into(profile_offers::table)
                        .values(offer)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileOfferStatusUpdate {
                    profile_id,
                    offeror_address,
                    status,
                    resolved_at,
                    updated_at,
                    transaction_id,
                } => {
                    let _ = diesel::update(profile_offers::table)
                        .filter(profile_offers::profile_id.eq(profile_id))
                        .filter(profile_offers::offeror_address.eq(offeror_address))
                        .filter(profile_offers::status.eq("pending"))
                        .set((
                            profile_offers::status.eq(status),
                            profile_offers::resolved_at.eq(Some(*resolved_at)),
                            profile_offers::updated_at.eq(*updated_at),
                            profile_offers::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::ProfileSaleFee(fee) => {
                    total += diesel::insert_into(profile_sale_fees::table)
                        .values(fee)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::EcosystemTreasury(c) => {
                    let latest: Option<(i32, chrono::NaiveDateTime)> = ecosystem_treasury::table
                        .order(ecosystem_treasury::time.desc())
                        .select((ecosystem_treasury::id, ecosystem_treasury::time))
                        .first(conn)
                        .await
                        .ok();
                    if let Some((id, time)) = latest {
                        total += diesel::update(ecosystem_treasury::table)
                            .filter(ecosystem_treasury::id.eq(id))
                            .filter(ecosystem_treasury::time.eq(time))
                            .set((
                                ecosystem_treasury::treasury_address.eq(&c.treasury_address),
                                ecosystem_treasury::updated_by.eq(&c.updated_by),
                                ecosystem_treasury::timestamp_ms.eq(c.timestamp_ms),
                                ecosystem_treasury::transaction_id.eq(&c.transaction_id),
                            ))
                            .execute(conn)
                            .await?;
                    } else {
                        total += diesel::insert_into(ecosystem_treasury::table)
                            .values(c)
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::ProfileBadge(badge) => {
                    total += diesel::insert_into(profile_badges::table)
                        .values(badge)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileBadgeRevoke {
                    profile_id,
                    badge_id,
                    revoked_at,
                    revoked_by,
                } => {
                    total += diesel::update(profile_badges::table)
                        .filter(profile_badges::profile_id.eq(profile_id))
                        .filter(profile_badges::badge_id.eq(badge_id))
                        .filter(profile_badges::revoked.eq(false))
                        .set((
                            profile_badges::revoked.eq(true),
                            profile_badges::revoked_at.eq(Some(*revoked_at)),
                            profile_badges::revoked_by.eq(Some(revoked_by.clone())),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SocialGraphRelationship(rel) => {
                    total += diesel::insert_into(social_graph_relationships::table)
                        .values(rel)
                        .on_conflict((
                            social_graph_relationships::follower_address,
                            social_graph_relationships::following_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SocialGraphEvent(ev) => {
                    total += diesel::insert_into(social_graph_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SocialGraphUnfollow {
                    follower_address,
                    following_address,
                } => {
                    total += diesel::delete(social_graph_relationships::table)
                        .filter(social_graph_relationships::follower_address.eq(follower_address))
                        .filter(social_graph_relationships::following_address.eq(following_address))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::GovernanceRegistry(reg) => {
                    let registry_id = reg.registry_id.clone();
                    let exists = governance_registries::table
                        .filter(governance_registries::registry_id.eq(&registry_id))
                        .count()
                        .get_result::<i64>(conn)
                        .await
                        .unwrap_or(0)
                        > 0;
                    if !exists {
                        let delegate_count = reg.delegate_count;
                        let delegate_term_epochs = reg.delegate_term_epochs;
                        let proposal_submission_cost = reg.proposal_submission_cost;
                        let max_votes_per_user = reg.max_votes_per_user;
                        let quadratic_base_cost = reg.quadratic_base_cost;
                        let voting_period_ms = reg.voting_period_ms;
                        let quorum_votes = reg.quorum_votes;
                        let updated_at = reg.updated_at;
                        total += diesel::insert_into(governance_registries::table)
                            .values(reg)
                            .on_conflict(governance_registries::registry_type)
                            .do_update()
                            .set((
                                governance_registries::registry_id.eq(registry_id),
                                governance_registries::delegate_count.eq(delegate_count),
                                governance_registries::delegate_term_epochs
                                    .eq(delegate_term_epochs),
                                governance_registries::proposal_submission_cost
                                    .eq(proposal_submission_cost),
                                governance_registries::max_votes_per_user.eq(max_votes_per_user),
                                governance_registries::quadratic_base_cost.eq(quadratic_base_cost),
                                governance_registries::voting_period_ms.eq(voting_period_ms),
                                governance_registries::quorum_votes.eq(quorum_votes),
                                governance_registries::updated_at.eq(updated_at),
                            ))
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::GovernanceRegistryUpdate(up) => {
                    total += diesel::update(governance_registries::table)
                        .filter(governance_registries::registry_type.eq(up.registry_type))
                        .set((
                            governance_registries::delegate_count.eq(up.delegate_count),
                            governance_registries::delegate_term_epochs.eq(up.delegate_term_epochs),
                            governance_registries::proposal_submission_cost
                                .eq(up.proposal_submission_cost),
                            governance_registries::max_votes_per_user.eq(up.max_votes_per_user),
                            governance_registries::quadratic_base_cost.eq(up.quadratic_base_cost),
                            governance_registries::voting_period_ms.eq(up.voting_period_ms),
                            governance_registries::quorum_votes.eq(up.quorum_votes),
                            governance_registries::updated_at.eq(up.updated_at),
                            governance_registries::transaction_id.eq(up.transaction_id.clone()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::NominatedDelegate(n) => {
                    total += diesel::insert_into(nominated_delegates::table)
                        .values(n)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Delegate(d) => {
                    total += diesel::insert_into(delegates::table)
                        .values(d)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Proposal(p) => {
                    total += diesel::insert_into(proposals::table)
                        .values(p)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProposalUpdate {
                    proposal_id,
                    set,
                    governance_event,
                    submitter_filter,
                } => {
                    total += if let Some(ref s) = submitter_filter {
                        diesel::update(proposals::table)
                            .filter(proposals::id.eq(proposal_id))
                            .filter(proposals::submitter.eq(s))
                            .set(set)
                            .execute(conn)
                            .await?
                    } else {
                        diesel::update(proposals::table)
                            .filter(proposals::id.eq(proposal_id))
                            .set(set)
                            .execute(conn)
                            .await?
                    };
                    if let Some((event_type, event_data, event_id)) = governance_event {
                        let proposal_type: Option<i16> = proposals::table
                            .filter(proposals::id.eq(&proposal_id))
                            .select(proposals::proposal_type)
                            .limit(1)
                            .load::<i16>(conn)
                            .await
                            .ok()
                            .and_then(|v| v.into_iter().next());
                        if let Some(registry_type) = proposal_type {
                            let gov_ev = NewGovernanceEvent {
                                event_type: event_type.clone(),
                                registry_type,
                                event_data: event_data.clone(),
                                event_id: event_id.clone(),
                                created_at: chrono::Utc::now(),
                                anonymous_voting_related: None,
                            };
                            total += diesel::insert_into(governance_events::table)
                                .values(&gov_ev)
                                .execute(conn)
                                .await?;
                        }
                    }
                }
                SocialEventRow::DelegateRating(r) => {
                    total += diesel::insert_into(delegate_ratings::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::DelegateVote(v) => {
                    total += diesel::insert_into(delegate_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::CommunityVote(v) => {
                    total += diesel::insert_into(community_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::RewardDistribution(r) => {
                    total += diesel::insert_into(reward_distributions::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::GovernanceEvent(ev) => {
                    total += diesel::insert_into(governance_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::GovernanceEventFromProposal {
                    proposal_id,
                    event_type,
                    event_data,
                    event_id,
                    anonymous_voting_related,
                } => {
                    let proposal_type: Option<i16> = proposals::table
                        .filter(proposals::id.eq(proposal_id))
                        .select(proposals::proposal_type)
                        .limit(1)
                        .load::<i16>(conn)
                        .await
                        .ok()
                        .and_then(|v| v.into_iter().next());
                    if let Some(registry_type) = proposal_type {
                        let gov_ev = NewGovernanceEvent {
                            event_type: event_type.clone(),
                            registry_type,
                            event_data: event_data.clone(),
                            event_id: event_id.clone(),
                            created_at: chrono::Utc::now(),
                            anonymous_voting_related: *anonymous_voting_related,
                        };
                        total += diesel::insert_into(governance_events::table)
                            .values(&gov_ev)
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::AnonymousVote(v) => {
                    total += diesel::insert_into(anonymous_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::VoteDecryptionFailure(f) => {
                    total += diesel::insert_into(vote_decryption_failures::table)
                        .values(f)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::NominatedDelegateStatusUpdate {
                    address,
                    registry_type,
                    status,
                } => {
                    let upd = diesel::sql_query(
                        "UPDATE nominated_delegates SET status = $1 WHERE address = $2 AND registry_type = $3 AND time = (SELECT max(time) FROM nominated_delegates WHERE address = $2 AND registry_type = $3)",
                    )
                    .bind::<Int2, _>(*status)
                    .bind::<Text, _>(address)
                    .bind::<Int2, _>(*registry_type);
                    total += upd.execute(conn).await?;
                }
                SocialEventRow::DelegateVoteCountsUpdate {
                    target_address,
                    registry_type,
                    is_active_delegate,
                    upvotes,
                    downvotes,
                } => {
                    if *is_active_delegate {
                        let upd = diesel::sql_query(
                            "UPDATE delegates SET upvotes = $1, downvotes = $2 WHERE address = $3 AND registry_type = $4 AND time = (SELECT max(time) FROM delegates WHERE address = $3 AND registry_type = $4)",
                        )
                        .bind::<BigInt, _>(*upvotes)
                        .bind::<BigInt, _>(*downvotes)
                        .bind::<Text, _>(target_address)
                        .bind::<Int2, _>(*registry_type);
                        total += upd.execute(conn).await?;
                    } else {
                        let upd = diesel::sql_query(
                            "UPDATE nominated_delegates SET upvotes = $1, downvotes = $2 WHERE address = $3 AND registry_type = $4 AND time = (SELECT max(time) FROM nominated_delegates WHERE address = $3 AND registry_type = $4)",
                        )
                        .bind::<BigInt, _>(*upvotes)
                        .bind::<BigInt, _>(*downvotes)
                        .bind::<Text, _>(target_address)
                        .bind::<Int2, _>(*registry_type);
                        total += upd.execute(conn).await?;
                    }
                }
                SocialEventRow::ProposalDelegateVoteIncrement {
                    proposal_id,
                    approve,
                } => {
                    let sql = if *approve {
                        "UPDATE proposals SET delegate_approval_count = delegate_approval_count + 1 WHERE id = $1 AND time = (SELECT max(time) FROM proposals WHERE id = $1)"
                    } else {
                        "UPDATE proposals SET delegate_rejection_count = delegate_rejection_count + 1 WHERE id = $1 AND time = (SELECT max(time) FROM proposals WHERE id = $1)"
                    };
                    total += diesel::sql_query(sql)
                        .bind::<Text, _>(proposal_id)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::DelegateProposalsReviewedIncrement { address } => {
                    let upd = diesel::sql_query(
                        "UPDATE delegates SET proposals_reviewed = proposals_reviewed + 1 WHERE address = $1 AND is_active = true",
                    )
                    .bind::<Text, _>(address);
                    total += upd.execute(conn).await?;
                }
                SocialEventRow::ProposalCommunityVoteUpdate {
                    proposal_id,
                    votes_for_delta,
                    votes_against_delta,
                } => {
                    let upd = diesel::sql_query(
                        "UPDATE proposals SET community_votes_for = community_votes_for + $1, community_votes_against = community_votes_against + $2 WHERE id = $3 AND time = (SELECT max(time) FROM proposals WHERE id = $3)",
                    )
                    .bind::<BigInt, _>(*votes_for_delta)
                    .bind::<BigInt, _>(*votes_against_delta)
                    .bind::<Text, _>(proposal_id);
                    total += upd.execute(conn).await?;
                }
                SocialEventRow::DelegateSidedProposalUpdate {
                    address,
                    is_winning,
                } => {
                    let sql = if *is_winning {
                        "UPDATE delegates SET sided_winning_proposals = sided_winning_proposals + 1 WHERE address = $1"
                    } else {
                        "UPDATE delegates SET sided_losing_proposals = sided_losing_proposals + 1 WHERE address = $1"
                    };
                    total += diesel::sql_query(sql)
                        .bind::<Text, _>(address)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProposalOutcomeApplyDelegateSidedUpdates {
                    proposal_id,
                    approvers_win,
                } => {
                    let subq = "SELECT DISTINCT ON (delegate_address) delegate_address, approve FROM delegate_votes WHERE proposal_id = $1 ORDER BY delegate_address, time DESC";
                    let win_sql = format!(
                        "UPDATE delegates d SET sided_winning_proposals = sided_winning_proposals + 1 FROM ({}) dv WHERE d.address = dv.delegate_address AND dv.approve = $2",
                        subq
                    );
                    let lose_sql = format!(
                        "UPDATE delegates d SET sided_losing_proposals = sided_losing_proposals + 1 FROM ({}) dv WHERE d.address = dv.delegate_address AND dv.approve = $2",
                        subq
                    );
                    total += diesel::sql_query(&win_sql)
                        .bind::<Text, _>(proposal_id)
                        .bind::<Bool, _>(*approvers_win)
                        .execute(conn)
                        .await?;
                    total += diesel::sql_query(&lose_sql)
                        .bind::<Text, _>(proposal_id)
                        .bind::<Bool, _>(!*approvers_win)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::DelegateProposalsSubmittedIncrement {
                    address,
                    registry_type,
                } => {
                    let upd = diesel::sql_query(
                        "UPDATE delegates SET proposals_submitted = proposals_submitted + 1 WHERE address = $1 AND registry_type = $2 AND is_active = true",
                    )
                    .bind::<Text, _>(address)
                    .bind::<Int2, _>(*registry_type);
                    total += upd.execute(conn).await?;
                }
                SocialEventRow::ProposalAnonymousVotersIncrement { proposal_id } => {
                    let upd = diesel::sql_query(
                        "UPDATE proposals SET anonymous_voters_count = COALESCE(anonymous_voters_count, 0) + 1 WHERE id = $1 AND time = (SELECT max(time) FROM proposals WHERE id = $1)",
                    )
                    .bind::<Text, _>(proposal_id);
                    total += upd.execute(conn).await?;
                }
                SocialEventRow::Post(p) => {
                    total += diesel::insert_into(posts::table)
                        .values(p)
                        .on_conflict((posts::id, posts::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Comment(c) => {
                    total += diesel::insert_into(comments::table)
                        .values(c)
                        .on_conflict((comments::id, comments::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Reaction(r) => {
                    total += diesel::insert_into(reactions::table)
                        .values(r)
                        .on_conflict_do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ReactionCount(rc) => {
                    total += diesel::insert_into(reaction_counts::table)
                        .values(rc)
                        .on_conflict((reaction_counts::object_id, reaction_counts::reaction_text))
                        .do_update()
                        .set(reaction_counts::count.eq(reaction_counts::count + 1))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::RemoveReaction {
                    object_id,
                    user_address,
                    reaction_text,
                    is_post: _,
                } => {
                    let _ = diesel::delete(reactions::table)
                        .filter(reactions::object_id.eq(object_id))
                        .filter(reactions::user_address.eq(user_address))
                        .filter(reactions::reaction_text.eq(reaction_text))
                        .execute(conn)
                        .await;
                    let _ = diesel::update(reaction_counts::table)
                        .filter(reaction_counts::object_id.eq(object_id))
                        .filter(reaction_counts::reaction_text.eq(reaction_text))
                        .set(reaction_counts::count.eq(reaction_counts::count - 1))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::Repost(r) => {
                    total += diesel::insert_into(reposts::table)
                        .values(r)
                        .on_conflict(reposts::id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Tip(t) => {
                    total += diesel::insert_into(tips::table)
                        .values(t)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ModerationEvent(m) => {
                    total += diesel::insert_into(posts_moderation_events::table)
                        .values(m)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Report(r) => {
                    total += diesel::insert_into(posts_reports::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::DeletionEvent(d) => {
                    total += diesel::insert_into(posts_deletion_events::table)
                        .values(d)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PostCommentCountIncrement { post_id, delta } => {
                    let _ = diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set(posts::comment_count.eq(posts::comment_count + delta))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::PostCommentCountDecrementByComment { comment_id, owner } => {
                    use diesel::sql_query;
                    use diesel::sql_types::Text;
                    let _ = sql_query(
                        "UPDATE posts SET comment_count = comment_count - 1 WHERE post_id = (SELECT post_id FROM comments WHERE comment_id = $1 AND owner = $2 LIMIT 1)",
                    )
                    .bind::<Text, _>(comment_id)
                    .bind::<Text, _>(owner)
                    .execute(conn)
                    .await;
                }
                SocialEventRow::ProfilePostCountIncrement { owner_address } => {
                    let _ = diesel::update(profiles::table)
                        .filter(profiles::owner_address.eq(owner_address))
                        .set(profiles::post_count.eq(profiles::post_count + 1))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::ProfilePostCountDecrement { owner_address } => {
                    let _ = diesel::update(profiles::table)
                        .filter(profiles::owner_address.eq(owner_address))
                        .set(profiles::post_count.eq(profiles::post_count - 1))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::PostRepostCountIncrement {
                    original_id,
                    is_original_post,
                } => {
                    if *is_original_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(original_id))
                            .set(posts::repost_count.eq(posts::repost_count + 1))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(original_id))
                            .set(comments::repost_count.eq(comments::repost_count + 1))
                            .execute(conn)
                            .await;
                    }
                }
                SocialEventRow::PostTipsReceivedIncrement {
                    object_id,
                    amount,
                    is_post,
                } => {
                    if *is_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(object_id))
                            .set(posts::tips_received.eq(posts::tips_received + amount))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set(comments::tips_received.eq(comments::tips_received + amount))
                            .execute(conn)
                            .await;
                    }
                }
                SocialEventRow::PostModerationUpdate {
                    object_id,
                    removed,
                    moderated_by,
                } => {
                    let post_updated = diesel::update(posts::table)
                        .filter(posts::post_id.eq(object_id))
                        .set((
                            posts::removed_from_platform.eq(*removed),
                            posts::removed_by.eq(Some(moderated_by.clone())),
                        ))
                        .execute(conn)
                        .await
                        .unwrap_or(0);
                    if post_updated == 0 {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set((
                                comments::removed_from_platform.eq(*removed),
                                comments::removed_by.eq(Some(moderated_by.clone())),
                            ))
                            .execute(conn)
                            .await;
                    }
                }
                SocialEventRow::PostDeletedAtUpdate {
                    object_id,
                    owner,
                    deleted_at,
                } => {
                    let _ = diesel::update(posts::table)
                        .filter(posts::post_id.eq(object_id))
                        .filter(posts::owner.eq(owner))
                        .set(posts::deleted_at.eq(Some(*deleted_at)))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::CommentDeletedAtUpdate {
                    object_id,
                    owner,
                    deleted_at,
                } => {
                    let _ = diesel::update(comments::table)
                        .filter(comments::comment_id.eq(object_id))
                        .filter(comments::owner.eq(owner))
                        .set(comments::deleted_at.eq(Some(*deleted_at)))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::PostContentUpdate {
                    object_id,
                    content,
                    media_urls,
                    mentions,
                    metadata_json,
                    is_post,
                    updated_at,
                } => {
                    if *is_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(object_id))
                            .set((
                                posts::content.eq(content),
                                posts::media_urls.eq(media_urls),
                                posts::mentions.eq(mentions),
                                posts::metadata_json.eq(metadata_json),
                                posts::updated_at.eq(Some(*updated_at)),
                            ))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set((
                                comments::content.eq(content),
                                comments::media_urls.eq(media_urls),
                                comments::mentions.eq(mentions),
                                comments::metadata_json.eq(metadata_json),
                                comments::updated_at.eq(Some(*updated_at)),
                            ))
                            .execute(conn)
                            .await;
                    }
                }
                SocialEventRow::PostOwnerUpdate {
                    object_id,
                    new_owner,
                    is_post,
                } => {
                    if *is_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(object_id))
                            .set(posts::owner.eq(new_owner))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set(comments::owner.eq(new_owner))
                            .execute(conn)
                            .await;
                    }
                }
                SocialEventRow::PostTransfer(transfer) => {
                    total += diesel::insert_into(posts_transfers::table)
                        .values(transfer)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PostConfig {
                    updated_by,
                    max_content_length,
                    max_media_urls,
                    max_mentions,
                    max_metadata_size,
                    max_description_length,
                    max_reaction_length,
                    commenter_tip_percentage,
                    repost_tip_percentage,
                    version,
                    updated_at,
                    transaction_id,
                } => {
                    use diesel::sql_query;
                    use diesel::sql_types::{BigInt, Text};
                    let version_val = version.unwrap_or(-1);
                    if version_val >= 0 {
                        let _ = diesel::insert_into(post_config::table)
                            .values((
                                post_config::updated_by.eq(updated_by),
                                post_config::max_content_length.eq(max_content_length),
                                post_config::max_media_urls.eq(max_media_urls),
                                post_config::max_mentions.eq(max_mentions),
                                post_config::max_metadata_size.eq(max_metadata_size),
                                post_config::max_description_length.eq(max_description_length),
                                post_config::max_reaction_length.eq(max_reaction_length),
                                post_config::commenter_tip_percentage.eq(commenter_tip_percentage),
                                post_config::repost_tip_percentage.eq(repost_tip_percentage),
                                post_config::version.eq(version_val),
                                post_config::updated_at.eq(updated_at),
                                post_config::transaction_id.eq(transaction_id),
                            ))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = sql_query(
                            r#"INSERT INTO post_config (updated_by, max_content_length, max_media_urls, max_mentions, max_metadata_size, max_description_length, max_reaction_length, commenter_tip_percentage, repost_tip_percentage, version, updated_at, transaction_id)
                               SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, COALESCE((SELECT MAX(version) FROM post_config), 0) + 1, $10, $11"#,
                        )
                        .bind::<Text, _>(updated_by)
                        .bind::<BigInt, _>(max_content_length)
                        .bind::<BigInt, _>(max_media_urls)
                        .bind::<BigInt, _>(max_mentions)
                        .bind::<BigInt, _>(max_metadata_size)
                        .bind::<BigInt, _>(max_description_length)
                        .bind::<BigInt, _>(max_reaction_length)
                        .bind::<BigInt, _>(commenter_tip_percentage)
                        .bind::<BigInt, _>(repost_tip_percentage)
                        .bind::<BigInt, _>(updated_at)
                        .bind::<Text, _>(transaction_id)
                        .execute(conn)
                        .await;
                    }
                }
                SocialEventRow::PromotedPost {
                    post_id,
                    owner,
                    profile_id,
                    payment_per_view,
                    total_budget,
                    created_at,
                    transaction_id,
                } => {
                    let promotion_id_opt: Option<String> = posts::table
                        .filter(posts::post_id.eq(&post_id))
                        .order(posts::time.desc())
                        .select(posts::promotion_id)
                        .first::<Option<String>>(conn)
                        .await
                        .ok()
                        .flatten();
                    if let Some(promotion_id) = promotion_id_opt {
                        let time = chrono::DateTime::from_timestamp(created_at / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotedPost {
                            promotion_id,
                            post_id: post_id.clone(),
                            owner: owner.clone(),
                            profile_id: profile_id.clone(),
                            payment_per_view: *payment_per_view,
                            total_budget: *total_budget,
                            remaining_budget: *total_budget,
                            active: false,
                            created_at: *created_at,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promoted_posts::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::PromotionView {
                    promotion_id,
                    viewer,
                    payment_amount,
                    view_duration,
                    platform_id,
                    timestamp,
                    transaction_id,
                } => {
                    let post_id_opt: Option<String> = promoted_posts::table
                        .filter(promoted_posts::promotion_id.eq(promotion_id))
                        .order(promoted_posts::time.desc())
                        .select(promoted_posts::post_id)
                        .first::<String>(conn)
                        .await
                        .ok();
                    if let Some(post_id) = post_id_opt {
                        let time = chrono::DateTime::from_timestamp(*timestamp / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotionView {
                            post_id,
                            promotion_id: promotion_id.clone(),
                            viewer: viewer.clone(),
                            payment_amount: *payment_amount,
                            view_duration: *view_duration,
                            platform_id: platform_id.clone(),
                            timestamp: *timestamp,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promotion_views::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::PromotionStatusEvent {
                    promotion_id,
                    toggled_by,
                    new_status,
                    timestamp,
                    transaction_id,
                } => {
                    let post_id_opt: Option<String> = promoted_posts::table
                        .filter(promoted_posts::promotion_id.eq(promotion_id))
                        .order(promoted_posts::time.desc())
                        .select(promoted_posts::post_id)
                        .first::<String>(conn)
                        .await
                        .ok();
                    if let Some(post_id) = post_id_opt {
                        let time = chrono::DateTime::from_timestamp(*timestamp / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotionStatusEvent {
                            post_id,
                            promotion_id: promotion_id.clone(),
                            event_type: "status_toggled".to_string(),
                            triggered_by: toggled_by.clone(),
                            new_status: Some(*new_status),
                            amount: None,
                            timestamp: *timestamp,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promotion_status_events::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                        total += diesel::update(promoted_posts::table)
                            .filter(promoted_posts::promotion_id.eq(promotion_id))
                            .set(promoted_posts::active.eq(*new_status))
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::PromotionBudgetEvent {
                    promotion_id,
                    owner: _,
                    withdrawn_amount,
                    timestamp,
                    transaction_id,
                } => {
                    let post_id_opt: Option<String> = promoted_posts::table
                        .filter(promoted_posts::promotion_id.eq(promotion_id))
                        .order(promoted_posts::time.desc())
                        .select(promoted_posts::post_id)
                        .first::<String>(conn)
                        .await
                        .ok();
                    if let Some(post_id) = post_id_opt {
                        let time = chrono::DateTime::from_timestamp(*timestamp / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotionBudgetEvent {
                            promotion_id: promotion_id.clone(),
                            post_id,
                            event_type: "withdrawal".to_string(),
                            amount: *withdrawn_amount,
                            remaining_budget: 0,
                            timestamp: *timestamp,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promotion_budget_events::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                        total += diesel::update(promoted_posts::table)
                            .filter(promoted_posts::promotion_id.eq(promotion_id))
                            .set((
                                promoted_posts::remaining_budget.eq(0),
                                promoted_posts::active.eq(false),
                            ))
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::Platform(p) => {
                    total += diesel::insert_into(platforms::table)
                        .values(p)
                        .on_conflict(platforms::platform_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformUpdate {
                    platform_id,
                    name,
                    tagline,
                    description,
                    terms_of_service,
                    privacy_policy,
                    platform_names,
                    links,
                    status,
                    release_date,
                    shutdown_date,
                    updated_at,
                    primary_category,
                    secondary_category,
                } => {
                    total += diesel::update(platforms::table)
                        .filter(platforms::platform_id.eq(platform_id))
                        .set((
                            platforms::name.eq(name),
                            platforms::tagline.eq(tagline),
                            platforms::description.eq(description),
                            platforms::terms_of_service.eq(terms_of_service),
                            platforms::privacy_policy.eq(privacy_policy),
                            platforms::platform_names.eq(platform_names),
                            platforms::links.eq(links),
                            platforms::status.eq(status),
                            platforms::release_date.eq(release_date),
                            platforms::shutdown_date.eq(shutdown_date),
                            platforms::updated_at.eq(updated_at),
                            platforms::primary_category.eq(primary_category),
                            platforms::secondary_category.eq(secondary_category),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformApprovalChange {
                    platform_id,
                    is_approved,
                    approved_by,
                    changed_at,
                } => {
                    total += diesel::update(platforms::table)
                        .filter(platforms::platform_id.eq(platform_id))
                        .set((
                            platforms::is_approved.eq(is_approved),
                            platforms::approval_changed_at.eq(Some(changed_at)),
                            platforms::approved_by.eq(Some(approved_by)),
                            platforms::updated_at.eq(changed_at),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformModerator(m) => {
                    total += diesel::insert_into(platform_moderators::table)
                        .values(m)
                        .on_conflict((
                            platform_moderators::platform_id,
                            platform_moderators::moderator_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformModeratorRemove {
                    platform_id,
                    moderator_address,
                } => {
                    let _ = diesel::delete(platform_moderators::table)
                        .filter(platform_moderators::platform_id.eq(platform_id))
                        .filter(platform_moderators::moderator_address.eq(moderator_address))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::PlatformBlockedProfile(b) => {
                    total += diesel::insert_into(platform_blocked_profiles::table)
                        .values(b)
                        .on_conflict((
                            platform_blocked_profiles::platform_id,
                            platform_blocked_profiles::wallet_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformBlockedProfileRemove {
                    platform_id,
                    wallet_address,
                } => {
                    let _ = diesel::delete(platform_blocked_profiles::table)
                        .filter(platform_blocked_profiles::platform_id.eq(platform_id))
                        .filter(platform_blocked_profiles::wallet_address.eq(wallet_address))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::PlatformMembership(m) => {
                    total += diesel::insert_into(platform_memberships::table)
                        .values(m)
                        .on_conflict((
                            platform_memberships::platform_id,
                            platform_memberships::wallet_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformMembershipRemove {
                    platform_id,
                    wallet_address,
                } => {
                    let _ = diesel::delete(platform_memberships::table)
                        .filter(platform_memberships::platform_id.eq(platform_id))
                        .filter(platform_memberships::wallet_address.eq(wallet_address))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::PlatformTokenAirdrop(a) => {
                    total += diesel::insert_into(platform_token_airdrops::table)
                        .values(a)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformEvent(e) => {
                    total += diesel::insert_into(platform_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformDeleted {
                    platform_id,
                    deleted_at,
                } => {
                    total += diesel::update(platforms::table)
                        .filter(platforms::platform_id.eq(platform_id))
                        .set((
                            platforms::deleted_at.eq(Some(deleted_at)),
                            platforms::updated_at.eq(deleted_at),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PocBadge(badge) => {
                    total += diesel::insert_into(poc_badges::table)
                        .values(badge)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PocAnalysisResult(r) => {
                    total += diesel::insert_into(poc_analysis_results::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PocRevenueRedirection(r) => {
                    total += diesel::insert_into(poc_revenue_redirections::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PocDispute(d) => {
                    total += diesel::insert_into(poc_disputes::table)
                        .values(d)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PocDisputeVote(v) => {
                    total += diesel::insert_into(poc_dispute_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PocConfiguration(c) => {
                    total += diesel::insert_into(poc_configuration::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PostPocUpdate {
                    post_id,
                    poc_reasoning,
                    poc_evidence_urls,
                    poc_similarity_score,
                    poc_media_type,
                    poc_oracle_address,
                    poc_analyzed_at,
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set((
                            posts::poc_reasoning.eq(poc_reasoning),
                            posts::poc_evidence_urls.eq(poc_evidence_urls),
                            posts::poc_similarity_score.eq(poc_similarity_score),
                            posts::poc_media_type.eq(poc_media_type),
                            posts::poc_oracle_address.eq(poc_oracle_address),
                            posts::poc_analyzed_at.eq(poc_analyzed_at),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PostRevenueRedirectUpdate {
                    post_id,
                    revenue_redirect_to,
                    revenue_redirect_percentage,
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set((
                            posts::revenue_redirect_to.eq(Some(revenue_redirect_to)),
                            posts::revenue_redirect_percentage
                                .eq(Some(revenue_redirect_percentage)),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PocDisputeResolved {
                    dispute_id,
                    post_id,
                    resolution,
                    winning_side,
                    total_winning_stake,
                    total_losing_stake,
                    resolved_at,
                    badge_revoked,
                    redirection_removed,
                } => {
                    let update_sql = "UPDATE poc_disputes SET status = $1, resolution = $2, winning_side = $3, total_winning_stake = $4, total_losing_stake = $5, resolved_at = $6 \
                        WHERE dispute_id = $7 AND time = (SELECT time FROM poc_disputes WHERE dispute_id = $7 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<Int2, _>(resolution)
                        .bind::<Nullable<Int2>, _>(Some(resolution))
                        .bind::<Nullable<Int2>, _>(Some(winning_side))
                        .bind::<Nullable<BigInt>, _>(Some(total_winning_stake))
                        .bind::<Nullable<BigInt>, _>(Some(total_losing_stake))
                        .bind::<Nullable<BigInt>, _>(Some(resolved_at))
                        .bind::<Text, _>(dispute_id)
                        .execute(conn)
                        .await?;

                    if *badge_revoked {
                        let revoke_sql = "UPDATE poc_badges SET revoked = TRUE, revoked_at = $1 \
                            WHERE post_id = $2 AND time = (SELECT time FROM poc_badges WHERE post_id = $2 ORDER BY time DESC LIMIT 1)";
                        total += diesel::sql_query(revoke_sql)
                            .bind::<Nullable<BigInt>, _>(Some(resolved_at))
                            .bind::<Text, _>(post_id)
                            .execute(conn)
                            .await?;
                    }

                    if *redirection_removed {
                        let remove_sql = "UPDATE poc_revenue_redirections SET removed = TRUE, removed_at = $1 \
                            WHERE accused_post_id = $2 AND time = (SELECT time FROM poc_revenue_redirections WHERE accused_post_id = $2 ORDER BY time DESC LIMIT 1)";
                        total += diesel::sql_query(remove_sql)
                            .bind::<Nullable<BigInt>, _>(Some(resolved_at))
                            .bind::<Text, _>(post_id)
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::PocVoteRewardClaimed {
                    dispute_id,
                    voter,
                    reward_amount,
                } => {
                    let update_sql = "UPDATE poc_dispute_votes SET reward_claimed = $1, reward_amount = $2 \
                        WHERE dispute_id = $3 AND voter = $4 AND time = (SELECT time FROM poc_dispute_votes WHERE dispute_id = $3 AND voter = $4 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<Bool, _>(true)
                        .bind::<Nullable<BigInt>, _>(Some(*reward_amount))
                        .bind::<Text, _>(dispute_id)
                        .bind::<Text, _>(voter)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::MyDataData(d) => {
                    let owner = d.owner.clone();
                    let media_type = d.media_type.clone();
                    let tags = d.tags.clone();
                    let platform_id = d.platform_id.clone();
                    let one_time_price = d.one_time_price;
                    let subscription_price = d.subscription_price;
                    let last_updated = d.last_updated;
                    let transaction_id = d.transaction_id.clone();
                    total += diesel::insert_into(mydata_data::table)
                        .values(d)
                        .on_conflict(mydata_data::mydata_id)
                        .do_update()
                        .set((
                            mydata_data::owner.eq(owner),
                            mydata_data::media_type.eq(media_type),
                            mydata_data::tags.eq(tags),
                            mydata_data::platform_id.eq(platform_id),
                            mydata_data::one_time_price.eq(one_time_price),
                            mydata_data::subscription_price.eq(subscription_price),
                            mydata_data::last_updated.eq(last_updated),
                            mydata_data::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::MyDataPurchase(p) => {
                    total += diesel::insert_into(mydata_purchases::table)
                        .values(p)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::MyDataSubscription(s) => {
                    total += diesel::insert_into(mydata_subscriptions::table)
                        .values(s)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::MyDataRevenue(r) => {
                    let mut to_address = r.to_address.clone();
                    if to_address.is_empty() {
                        to_address = mydata_data::table
                            .filter(mydata_data::mydata_id.eq(&r.mydata_id))
                            .select(mydata_data::owner)
                            .first::<String>(conn)
                            .await
                            .unwrap_or_else(|_| "unknown".to_string());
                    }
                    let row = NewMyDataRevenue {
                        mydata_id: r.mydata_id.clone(),
                        from_address: r.from_address.clone(),
                        to_address,
                        amount: r.amount,
                        revenue_type: r.revenue_type.clone(),
                        revenue_time: r.revenue_time,
                        transaction_id: r.transaction_id.clone(),
                    };
                    total += diesel::insert_into(mydata_revenue::table)
                        .values(&row)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::MyDataAccessLog(a) => {
                    total += diesel::insert_into(mydata_access_logs::table)
                        .values(a)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::MyDataRegistry(reg) => {
                    let owner = reg.owner.clone();
                    let registered_at = reg.registered_at;
                    let transaction_id = reg.transaction_id.clone();
                    total += diesel::insert_into(mydata_registry::table)
                        .values(reg)
                        .on_conflict(mydata_registry::ip_id)
                        .do_update()
                        .set((
                            mydata_registry::owner.eq(owner),
                            mydata_registry::registered_at.eq(registered_at),
                            mydata_registry::unregistered_at.eq(None::<i64>),
                            mydata_registry::is_active.eq(true),
                            mydata_registry::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::MyDataRegistryUpdate {
                    ip_id,
                    owner,
                    unregistered_at,
                    transaction_id,
                } => {
                    total += diesel::update(mydata_registry::table)
                        .filter(mydata_registry::ip_id.eq(ip_id))
                        .filter(mydata_registry::owner.eq(owner))
                        .filter(mydata_registry::is_active.eq(true))
                        .set((
                            mydata_registry::unregistered_at.eq(Some(*unregistered_at)),
                            mydata_registry::is_active.eq(false),
                            mydata_registry::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::MyDataConfig(c) => {
                    total += diesel::insert_into(mydata_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::MyDataContentUpdate {
                    mydata_id,
                    last_updated,
                    transaction_id,
                } => {
                    total += diesel::update(mydata_data::table)
                        .filter(mydata_data::mydata_id.eq(mydata_id))
                        .set((
                            mydata_data::last_updated.eq(*last_updated),
                            mydata_data::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsuranceConfig(c) => {
                    total += diesel::insert_into(insurance_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsuranceVault(v) => {
                    total += diesel::insert_into(insurance_vaults::table)
                        .values(v)
                        .on_conflict(insurance_vaults::vault_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsuranceVaultTransaction(t) => {
                    total += diesel::insert_into(insurance_vault_transactions::table)
                        .values(t)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsuranceVaultBalanceUpdate {
                    vault_id,
                    new_balance,
                } => {
                    let now = chrono::Utc::now().naive_utc();
                    total += diesel::update(insurance_vaults::table)
                        .filter(insurance_vaults::vault_id.eq(vault_id))
                        .set((
                            insurance_vaults::capital_balance.eq(*new_balance),
                            insurance_vaults::updated_at.eq(now),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsurancePolicy(p) => {
                    total += diesel::insert_into(insurance_policies::table)
                        .values(p)
                        .on_conflict(insurance_policies::policy_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsurancePolicyEvent(pe) => {
                    total += diesel::insert_into(insurance_policy_events::table)
                        .values(pe)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsuranceMarketExposure(me) => {
                    total += diesel::insert_into(insurance_market_exposures::table)
                        .values(me)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsuranceUserExposure(ue) => {
                    total += diesel::insert_into(insurance_user_exposures::table)
                        .values(ue)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsuranceEventLog(log) => {
                    total += diesel::insert_into(insurance_events::table)
                        .values(log)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsurancePolicyStatusUpdate { policy_id, status } => {
                    let now = chrono::Utc::now().naive_utc();
                    total += diesel::update(insurance_policies::table)
                        .filter(insurance_policies::policy_id.eq(policy_id))
                        .set((
                            insurance_policies::status.eq(*status),
                            insurance_policies::updated_at.eq(now),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::InsurancePolicyEventFromPolicy {
                    policy_id,
                    event_type,
                    refunded_amount,
                    fee_paid,
                    payout,
                    reserve_released,
                    timestamp_ms,
                    transaction_id,
                } => {
                    #[derive(QueryableByName)]
                    struct PolicyRow {
                        #[diesel(sql_type = Text)]
                        market_id: String,
                        #[diesel(sql_type = Int2)]
                        option_id: i16,
                        #[diesel(sql_type = BigInt)]
                        covered_amount: i64,
                        #[diesel(sql_type = BigInt)]
                        coverage_bps: i64,
                        #[diesel(sql_type = BigInt)]
                        premium_paid: i64,
                        #[diesel(sql_type = Text)]
                        insured: String,
                    }
                    let policy_row: Option<PolicyRow> = diesel::sql_query(
                        "SELECT market_id, option_id, covered_amount, coverage_bps, premium_paid, insured FROM insurance_policies WHERE policy_id = $1",
                    )
                    .bind::<Text, _>(policy_id)
                    .get_result(conn)
                    .await
                    .ok();
                    if let Some(row) = policy_row {
                        let reserve_locked = reserve_released.unwrap_or_else(|| {
                            ((row.covered_amount as i128 * row.coverage_bps as i128) / 10000i128)
                                as i64
                        });
                        let policy_event = NewInsurancePolicyEvent {
                            policy_id: policy_id.clone(),
                            event_type: event_type.clone(),
                            market_id: row.market_id,
                            insured: row.insured,
                            option_id: row.option_id,
                            covered_amount: row.covered_amount,
                            coverage_bps: row.coverage_bps,
                            premium_paid: row.premium_paid,
                            reserve_locked,
                            refunded_amount: *refunded_amount,
                            fee_paid: *fee_paid,
                            payout: *payout,
                            timestamp_ms: *timestamp_ms,
                            time: chrono::Utc::now(),
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(insurance_policy_events::table)
                            .values(&policy_event)
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::SpotBet(bet) => {
                    total += diesel::insert_into(spot_bets::table)
                        .values(bet)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SpotResolution(r) => {
                    total += diesel::insert_into(spot_resolutions::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SpotPayout(p) => {
                    total += diesel::insert_into(spot_payouts::table)
                        .values(p)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SpotRefund(r) => {
                    total += diesel::insert_into(spot_refunds::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SpotEventLog(log) => {
                    total += diesel::insert_into(spot_events::table)
                        .values(log)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SpotConfig(c) => {
                    total += diesel::insert_into(spot_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SpotBetWithdrawal(w) => {
                    total += diesel::insert_into(spot_bet_withdrawals::table)
                        .values(w)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SpotRecordUpsert(record) => {
                    let betting_options = record
                        .betting_options
                        .clone()
                        .unwrap_or_else(|| serde_json::json!([]));
                    let resolution_window_epochs = record.resolution_window_epochs;
                    let max_resolution_window_epochs = record.max_resolution_window_epochs;
                    total += diesel::insert_into(spot_records::table)
                        .values(record)
                        .on_conflict(spot_records::post_id)
                        .do_update()
                        .set((
                            spot_records::betting_options.eq(betting_options),
                            spot_records::resolution_window_epochs.eq(resolution_window_epochs),
                            spot_records::max_resolution_window_epochs
                                .eq(max_resolution_window_epochs),
                            spot_records::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SpotRecordUpdate {
                    post_id,
                    status,
                    outcome,
                    last_resolution_epoch,
                } => {
                    total += diesel::update(spot_records::table)
                        .filter(spot_records::post_id.eq(post_id))
                        .set((
                            spot_records::status.eq(*status),
                            spot_records::outcome.eq(*outcome),
                            spot_records::last_resolution_epoch.eq(Some(*last_resolution_epoch)),
                            spot_records::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SptPool(p) => {
                    total += diesel::insert_into(spt_pools::table)
                        .values(p)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SptTransaction(t) => {
                    total += diesel::insert_into(spt_transactions::table)
                        .values(t)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SptHolding(h) => {
                    total += diesel::insert_into(spt_holdings::table)
                        .values(h)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SptPoolSupplyUpdate { pool_id, delta } => {
                    let update_sql =
                        "UPDATE spt_pools SET circulating_supply = circulating_supply + $1 \
                         WHERE pool_id = $2 AND time = (SELECT time FROM spt_pools WHERE pool_id = $2 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(*delta)
                        .bind::<Text, _>(pool_id)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SptPriceHistory(ph) => {
                    total += diesel::insert_into(spt_price_history::table)
                        .values(ph)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SptReservationPool(rp) => {
                    total += diesel::insert_into(spt_reservation_pools::table)
                        .values(rp)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SptReservation {
                    associated_id,
                    reservation,
                    token_type,
                    total_reserved,
                    threshold_met,
                    created_at,
                } => {
                    #[derive(QueryableByName)]
                    struct PoolIdRow {
                        #[diesel(sql_type = Text)]
                        pool_id: String,
                    }
                    let pool_id_row: Option<PoolIdRow> = diesel::sql_query(
                        "SELECT pool_id FROM spt_reservation_pools WHERE associated_id = $1 ORDER BY time DESC LIMIT 1",
                    )
                    .bind::<Text, _>(associated_id)
                    .get_result(conn)
                    .await
                    .optional()?;
                    let pool_id = if let Some(ref row) = pool_id_row {
                        row.pool_id.clone()
                    } else {
                        let synthetic_pool_id = format!("reservation_pool_{}", associated_id);
                        #[derive(QueryableByName)]
                        struct OwnerRow {
                            #[diesel(sql_type = Text)]
                            owner: String,
                        }
                        let owner = if *token_type == TOKEN_TYPE_POST {
                            diesel::sql_query(
                                "SELECT owner FROM posts WHERE post_id = $1 ORDER BY time DESC LIMIT 1",
                            )
                            .bind::<Text, _>(associated_id)
                            .get_result::<OwnerRow>(conn)
                            .await
                            .ok()
                            .map(|r| r.owner)
                        } else {
                            diesel::sql_query(
                                "SELECT owner_address FROM profiles WHERE profile_id = $1 OR owner_address = $1 LIMIT 1",
                            )
                            .bind::<Text, _>(associated_id)
                            .get_result::<OwnerRow>(conn)
                            .await
                            .ok()
                            .map(|r| r.owner)
                        }
                        .unwrap_or_else(|| reservation.reserver_address.clone());
                        let status = if *threshold_met {
                            RESERVATION_POOL_STATUS_THRESHOLD_MET.to_string()
                        } else {
                            RESERVATION_POOL_STATUS_ACTIVE.to_string()
                        };
                        let synthetic_pool = NewSptReservationPool {
                            pool_id: synthetic_pool_id.clone(),
                            associated_id: associated_id.clone(),
                            token_type: *token_type,
                            owner: owner.clone(),
                            total_reserved: *total_reserved,
                            required_threshold: *total_reserved,
                            status,
                            created_at: *created_at,
                            time: reservation.time,
                            transaction_id: reservation.transaction_id.clone(),
                        };
                        total += diesel::insert_into(spt_reservation_pools::table)
                            .values(&synthetic_pool)
                            .execute(conn)
                            .await?;
                        info!(
                            associated_id = %associated_id,
                            pool_id = %synthetic_pool_id,
                            "created synthetic SptReservationPool (no canonical pool found)"
                        );
                        synthetic_pool_id
                    };
                    let mut r = reservation.clone();
                    r.pool_id = pool_id.clone();
                    total += diesel::insert_into(spt_reservations::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                    info!(
                        associated_id = %associated_id,
                        pool_id = %pool_id,
                        reserver = %reservation.reserver_address,
                        amount = %reservation.amount,
                        "SptReservation inserted"
                    );
                }
                SocialEventRow::SptReservationPoolUpdate {
                    pool_id: _pool_id,
                    associated_id,
                    total_reserved,
                    status,
                    required_threshold,
                } => {
                    let update_sql =
                        "UPDATE spt_reservation_pools SET total_reserved = $1, \
                         status = COALESCE($2, status), \
                         required_threshold = COALESCE($4, required_threshold) \
                         WHERE associated_id = $3 AND time = (SELECT time FROM spt_reservation_pools WHERE associated_id = $3 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(*total_reserved)
                        .bind::<Nullable<Text>, _>(status.as_deref())
                        .bind::<Text, _>(associated_id)
                        .bind::<Nullable<BigInt>, _>(*required_threshold)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SptExchangeConfig(c) => {
                    let sync_reservation_pool_thresholds =
                        c.profile_threshold > 0 && c.post_threshold > 0;
                    let profile_threshold = c.profile_threshold;
                    let post_threshold = c.post_threshold;
                    let latest: Option<(i32, chrono::NaiveDateTime)> = spt_exchange_config::table
                        .order(spt_exchange_config::time.desc())
                        .select((spt_exchange_config::id, spt_exchange_config::time))
                        .first(conn)
                        .await
                        .ok();
                    if let Some((id, time)) = latest {
                        total += diesel::update(spt_exchange_config::table)
                            .filter(spt_exchange_config::id.eq(id))
                            .filter(spt_exchange_config::time.eq(time))
                            .set((
                                spt_exchange_config::updated_by.eq(&c.updated_by),
                                spt_exchange_config::post_threshold.eq(c.post_threshold),
                                spt_exchange_config::profile_threshold.eq(c.profile_threshold),
                                spt_exchange_config::max_individual_reservation_bps
                                    .eq(c.max_individual_reservation_bps),
                                spt_exchange_config::total_fee_bps.eq(c.total_fee_bps),
                                spt_exchange_config::creator_fee_bps.eq(c.creator_fee_bps),
                                spt_exchange_config::platform_fee_bps.eq(c.platform_fee_bps),
                                spt_exchange_config::treasury_fee_bps.eq(c.treasury_fee_bps),
                                spt_exchange_config::trading_creator_fee_bps
                                    .eq(c.trading_creator_fee_bps),
                                spt_exchange_config::trading_platform_fee_bps
                                    .eq(c.trading_platform_fee_bps),
                                spt_exchange_config::trading_treasury_fee_bps
                                    .eq(c.trading_treasury_fee_bps),
                                spt_exchange_config::reservation_creator_fee_bps
                                    .eq(c.reservation_creator_fee_bps),
                                spt_exchange_config::reservation_platform_fee_bps
                                    .eq(c.reservation_platform_fee_bps),
                                spt_exchange_config::reservation_treasury_fee_bps
                                    .eq(c.reservation_treasury_fee_bps),
                                spt_exchange_config::max_reservers_per_pool
                                    .eq(c.max_reservers_per_pool),
                                spt_exchange_config::base_price.eq(c.base_price),
                                spt_exchange_config::quadratic_coefficient
                                    .eq(c.quadratic_coefficient),
                                spt_exchange_config::max_hold_percent_bps
                                    .eq(c.max_hold_percent_bps),
                                spt_exchange_config::trading_enabled.eq(c.trading_enabled),
                                spt_exchange_config::updated_at.eq(c.updated_at),
                                spt_exchange_config::transaction_id.eq(&c.transaction_id),
                            ))
                            .execute(conn)
                            .await?;
                    } else {
                        total += diesel::insert_into(spt_exchange_config::table)
                            .values(c)
                            .execute(conn)
                            .await?;
                    }
                    if sync_reservation_pool_thresholds {
                        let sync_sql = r#"
                            UPDATE spt_reservation_pools sp
                            SET required_threshold = CASE sp.token_type
                                WHEN 1 THEN $1
                                WHEN 2 THEN $2
                                ELSE sp.required_threshold
                            END
                            FROM (
                                SELECT DISTINCT ON (pool_id) pool_id, time
                                FROM spt_reservation_pools
                                ORDER BY pool_id, time DESC
                            ) AS latest
                            WHERE sp.pool_id = latest.pool_id AND sp.time = latest.time
                        "#;
                        total += diesel::sql_query(sync_sql)
                            .bind::<BigInt, _>(profile_threshold)
                            .bind::<BigInt, _>(post_threshold)
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::SocialProofTokensConfig(c) => {
                    use diesel::dsl::max;
                    let max_id: Option<i32> = social_proof_tokens_config::table
                        .select(max(social_proof_tokens_config::id))
                        .get_result(conn)
                        .await
                        .ok()
                        .flatten();
                    if let Some(id) = max_id {
                        total += diesel::update(social_proof_tokens_config::table)
                            .filter(social_proof_tokens_config::id.eq(id))
                            .set((
                                social_proof_tokens_config::trading_enabled.eq(c.trading_enabled),
                                social_proof_tokens_config::admin_address.eq(&c.admin_address),
                                social_proof_tokens_config::reason.eq(&c.reason),
                                social_proof_tokens_config::timestamp_ms.eq(c.timestamp_ms),
                                social_proof_tokens_config::updated_at.eq(c.updated_at),
                                social_proof_tokens_config::transaction_id.eq(&c.transaction_id),
                            ))
                            .execute(conn)
                            .await?;
                    } else {
                        total += diesel::insert_into(social_proof_tokens_config::table)
                            .values(c)
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::SocialProofTokensEvent(e) => {
                    total += diesel::insert_into(social_proof_tokens_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SptRevenue(r) => {
                    total += diesel::insert_into(spt_revenue::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::UnifiedRevenue(r) => {
                    total += diesel::insert_into(unified_revenue::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SptBuySellRevenueData {
                    pool_id,
                    trader,
                    transaction_type,
                    creator_fee,
                    platform_fee,
                    treasury_fee,
                    amount,
                    myso_amount,
                    token_price,
                    revenue_time,
                    transaction_id,
                    ..
                } => {
                    use myso_indexer_alt_social_schema::models::{
                        NewSptRevenue, NewUnifiedRevenue, REVENUE_TYPE_SPT_CREATOR_FEE,
                        REVENUE_TYPE_SPT_PLATFORM_FEE, REVENUE_TYPE_SPT_TREASURY_FEE,
                    };

                    let pool_row: Option<(String, String, i16)> = spt_pools::table
                        .filter(spt_pools::pool_id.eq(pool_id))
                        .order(spt_pools::time.desc())
                        .select((
                            spt_pools::owner,
                            spt_pools::associated_id,
                            spt_pools::token_type,
                        ))
                        .first::<(String, String, i16)>(conn)
                        .await
                        .ok();

                    let (creator_address, platform_address, treasury_address): (
                        String,
                        String,
                        String,
                    ) = if let Some((owner, _associated_id, _token_type)) = pool_row {
                        let treasury = ecosystem_treasury::table
                            .order(ecosystem_treasury::time.desc())
                            .select(ecosystem_treasury::treasury_address)
                            .first::<String>(conn)
                            .await
                            .ok()
                            .unwrap_or_default();
                        (owner, String::new(), treasury)
                    } else {
                        (String::new(), String::new(), String::new())
                    };

                    if *creator_fee != 0 || *platform_fee != 0 || *treasury_fee != 0 {
                        let spt_rev = if transaction_type == "SELL" {
                            NewSptRevenue::from_sell_event(
                                pool_id.clone(),
                                trader.clone(),
                                creator_address.clone(),
                                platform_address.clone(),
                                treasury_address.clone(),
                                *creator_fee,
                                *platform_fee,
                                *treasury_fee,
                                *amount,
                                *myso_amount,
                                *token_price,
                                *revenue_time,
                                transaction_id.clone(),
                            )
                        } else {
                            NewSptRevenue::from_buy_event(
                                pool_id.clone(),
                                trader.clone(),
                                creator_address.clone(),
                                platform_address.clone(),
                                treasury_address.clone(),
                                *creator_fee,
                                *platform_fee,
                                *treasury_fee,
                                *amount,
                                *myso_amount,
                                *token_price,
                                *revenue_time,
                                transaction_id.clone(),
                            )
                        };
                        total += diesel::insert_into(spt_revenue::table)
                            .values(&spt_rev)
                            .execute(conn)
                            .await?;

                        if *creator_fee > 0 {
                            total += diesel::insert_into(unified_revenue::table)
                                .values(NewUnifiedRevenue::from_spt(
                                    REVENUE_TYPE_SPT_CREATOR_FEE.to_string(),
                                    creator_address.clone(),
                                    Some(platform_address.clone()),
                                    *creator_fee,
                                    pool_id.clone(),
                                    trader.clone(),
                                    creator_address.clone(),
                                    *revenue_time,
                                    transaction_id.clone(),
                                ))
                                .execute(conn)
                                .await?;
                        }
                        if *platform_fee > 0 {
                            total += diesel::insert_into(unified_revenue::table)
                                .values(NewUnifiedRevenue::from_spt(
                                    REVENUE_TYPE_SPT_PLATFORM_FEE.to_string(),
                                    creator_address.clone(),
                                    Some(platform_address.clone()),
                                    *platform_fee,
                                    pool_id.clone(),
                                    trader.clone(),
                                    platform_address.clone(),
                                    *revenue_time,
                                    transaction_id.clone(),
                                ))
                                .execute(conn)
                                .await?;
                        }
                        if *treasury_fee > 0 {
                            total += diesel::insert_into(unified_revenue::table)
                                .values(NewUnifiedRevenue::from_spt(
                                    REVENUE_TYPE_SPT_TREASURY_FEE.to_string(),
                                    creator_address.clone(),
                                    None,
                                    *treasury_fee,
                                    pool_id.clone(),
                                    trader.clone(),
                                    treasury_address.clone(),
                                    *revenue_time,
                                    transaction_id.clone(),
                                ))
                                .execute(conn)
                                .await?;
                        }
                    }
                }
                SocialEventRow::ProfileSubscriptionService(s) => {
                    let profile_id = profiles::table
                        .filter(profiles::owner_address.eq(&s.profile_owner))
                        .select(profiles::profile_id)
                        .first::<Option<String>>(conn)
                        .await
                        .ok()
                        .flatten()
                        .unwrap_or_else(|| s.profile_owner.clone());
                    let service = NewProfileSubscriptionService {
                        profile_id,
                        ..s.clone()
                    };
                    total += diesel::insert_into(profile_subscription_services::table)
                        .values(&service)
                        .on_conflict(profile_subscription_services::service_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileSubscription(s) => {
                    total += diesel::insert_into(profile_subscriptions::table)
                        .values(s)
                        .on_conflict((
                            profile_subscriptions::subscription_id,
                            profile_subscriptions::time,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SubscriptionEvent(ev) => {
                    total += diesel::insert_into(subscription_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SubscriptionRevenue(r) => {
                    total += diesel::insert_into(subscription_revenue::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileSubscriptionServiceSubscriberIncrement { service_id } => {
                    let _ = diesel::update(profile_subscription_services::table)
                        .filter(profile_subscription_services::service_id.eq(service_id))
                        .set(
                            profile_subscription_services::subscriber_count
                                .eq(profile_subscription_services::subscriber_count + 1),
                        )
                        .execute(conn)
                        .await;
                }
                SocialEventRow::ProfileSubscriptionServiceSubscriberDecrementBySubscription {
                    subscription_id,
                } => {
                    let service_id: Option<String> = profile_subscriptions::table
                        .filter(profile_subscriptions::subscription_id.eq(subscription_id))
                        .order(profile_subscriptions::time.desc())
                        .select(profile_subscriptions::service_id)
                        .first(conn)
                        .await
                        .ok();
                    if let Some(sid) = service_id {
                        let _ = diesel::update(profile_subscription_services::table)
                            .filter(profile_subscription_services::service_id.eq(&sid))
                            .set(
                                profile_subscription_services::subscriber_count
                                    .eq(profile_subscription_services::subscriber_count - 1),
                            )
                            .execute(conn)
                            .await;
                    }
                }
                SocialEventRow::ProfileSubscriptionUpdate {
                    subscription_id,
                    expires_at,
                    renewal_count,
                } => {
                    let update_sql = "UPDATE profile_subscriptions SET expires_at = $1, renewal_count = $2 \
                        WHERE subscription_id = $3 AND time = (SELECT time FROM profile_subscriptions WHERE subscription_id = $3 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(expires_at)
                        .bind::<BigInt, _>(renewal_count)
                        .bind::<Text, _>(subscription_id)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileSubscriptionCancel { subscription_id } => {
                    let now = chrono::Utc::now().timestamp_millis();
                    let update_sql = "UPDATE profile_subscriptions SET cancelled_at = $1 \
                        WHERE subscription_id = $2 AND time = (SELECT time FROM profile_subscriptions WHERE subscription_id = $2 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<Nullable<BigInt>, _>(Some(now))
                        .bind::<Text, _>(subscription_id)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileSubscriptionServiceUpdate {
                    service_id,
                    monthly_fee,
                    updated_at,
                } => {
                    total += diesel::update(profile_subscription_services::table)
                        .filter(profile_subscription_services::service_id.eq(service_id))
                        .set((
                            profile_subscription_services::monthly_fee.eq(monthly_fee),
                            profile_subscription_services::updated_at.eq(Some(updated_at)),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileSubscriptionRenewalBalanceUpdate {
                    subscription_id,
                    new_balance,
                } => {
                    let update_sql = "UPDATE profile_subscriptions SET renewal_balance = $1 \
                        WHERE subscription_id = $2 AND time = (SELECT time FROM profile_subscriptions WHERE subscription_id = $2 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(new_balance)
                        .bind::<Text, _>(subscription_id)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileSubscriptionServiceDeactivate {
                    service_id,
                    updated_at,
                } => {
                    total += diesel::update(profile_subscription_services::table)
                        .filter(profile_subscription_services::service_id.eq(service_id))
                        .set((
                            profile_subscription_services::active.eq(false),
                            profile_subscription_services::updated_at.eq(Some(updated_at)),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SubscriptionRevenueFromCreated {
                    service_id,
                    subscription_id,
                    from_address,
                    amount,
                    revenue_type,
                    payment_time,
                    transaction_id,
                } => {
                    let profile_owner: Option<String> = profile_subscription_services::table
                        .filter(profile_subscription_services::service_id.eq(service_id))
                        .select(profile_subscription_services::profile_owner)
                        .first(conn)
                        .await
                        .ok();
                    if let Some(to_address) = profile_owner {
                        let revenue = NewSubscriptionRevenue {
                            service_id: service_id.clone(),
                            subscription_id: Some(subscription_id.clone()),
                            from_address: from_address.clone(),
                            to_address,
                            amount: *amount,
                            revenue_type: revenue_type.clone(),
                            payment_time: *payment_time,
                            time: chrono::Utc::now(),
                            transaction_id: transaction_id.clone(),
                            processing_success: true,
                            processing_error: None,
                        };
                        total += diesel::insert_into(subscription_revenue::table)
                            .values(&revenue)
                            .execute(conn)
                            .await?;
                    }
                }
                SocialEventRow::SubscriptionRevenueFromRefund {
                    subscription_id,
                    subscriber,
                    refunded_amount,
                    transaction_id,
                } => {
                    let sub_row: Option<(String, String)> = profile_subscriptions::table
                        .filter(profile_subscriptions::subscription_id.eq(subscription_id))
                        .order(profile_subscriptions::time.desc())
                        .select((
                            profile_subscriptions::service_id,
                            profile_subscriptions::subscriber,
                        ))
                        .first(conn)
                        .await
                        .ok();
                    if let Some((service_id, _)) = sub_row {
                        let profile_owner: Option<String> = profile_subscription_services::table
                            .filter(profile_subscription_services::service_id.eq(&service_id))
                            .select(profile_subscription_services::profile_owner)
                            .first(conn)
                            .await
                            .ok();
                        if let Some(profile_owner) = profile_owner {
                            let revenue = NewSubscriptionRevenue {
                                service_id,
                                subscription_id: Some(subscription_id.clone()),
                                from_address: profile_owner,
                                to_address: subscriber.clone(),
                                amount: -(*refunded_amount),
                                revenue_type: "refund".to_string(),
                                payment_time: chrono::Utc::now().timestamp_millis(),
                                time: chrono::Utc::now(),
                                transaction_id: transaction_id.clone(),
                                processing_success: true,
                                processing_error: None,
                            };
                            total += diesel::insert_into(subscription_revenue::table)
                                .values(&revenue)
                                .execute(conn)
                                .await?;
                        }
                    }
                }
                SocialEventRow::SubscriptionRevenueFromRenewal {
                    subscription_id,
                    subscriber,
                    new_expires_at,
                    renewal_count: _,
                    auto_renewed,
                    transaction_id,
                } => {
                    let sub_row: Option<(String, i64)> = profile_subscriptions::table
                        .filter(profile_subscriptions::subscription_id.eq(subscription_id))
                        .order(profile_subscriptions::time.desc())
                        .select((
                            profile_subscriptions::service_id,
                            profile_subscriptions::renewal_balance,
                        ))
                        .first(conn)
                        .await
                        .ok();
                    if let Some((service_id, _)) = sub_row {
                        let profile_owner: Option<String> = profile_subscription_services::table
                            .filter(profile_subscription_services::service_id.eq(&service_id))
                            .select(profile_subscription_services::profile_owner)
                            .first(conn)
                            .await
                            .ok();
                        let monthly_fee: Option<i64> = profile_subscription_services::table
                            .filter(profile_subscription_services::service_id.eq(&service_id))
                            .select(profile_subscription_services::monthly_fee)
                            .first(conn)
                            .await
                            .ok();
                        if let (Some(to_address), Some(amount)) = (profile_owner, monthly_fee) {
                            let revenue_type = if *auto_renewed {
                                "auto_renewal"
                            } else {
                                "renewal"
                            };
                            let payment_time = *new_expires_at - (30 * 24 * 60 * 60 * 1000);
                            let revenue = NewSubscriptionRevenue {
                                service_id,
                                subscription_id: Some(subscription_id.clone()),
                                from_address: subscriber.clone(),
                                to_address,
                                amount,
                                revenue_type: revenue_type.to_string(),
                                payment_time,
                                time: chrono::Utc::now(),
                                transaction_id: transaction_id.clone(),
                                processing_success: true,
                                processing_error: None,
                            };
                            total += diesel::insert_into(subscription_revenue::table)
                                .values(&revenue)
                                .execute(conn)
                                .await?;
                        }
                    }
                }
                SocialEventRow::UpgradeEvent(ev) => {
                    total += diesel::insert_into(upgrade_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ObjectMigratedEvent(ev) => {
                    total += diesel::insert_into(object_migrated_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::VestingWallet(w) => {
                    total += diesel::insert_into(vesting_wallets::table)
                        .values(w)
                        .on_conflict(vesting_wallets::wallet_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::VestingEvent(ev) => {
                    total += diesel::insert_into(vesting_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::VestingWalletClaimUpdate {
                    wallet_id,
                    claimed_amount,
                    remaining_balance,
                } => {
                    let now = chrono::Utc::now().naive_utc();
                    total += diesel::update(vesting_wallets::table)
                        .filter(vesting_wallets::wallet_id.eq(wallet_id))
                        .set((
                            vesting_wallets::claimed_amount
                                .eq(vesting_wallets::claimed_amount + claimed_amount),
                            vesting_wallets::remaining_balance.eq(remaining_balance),
                            vesting_wallets::updated_at.eq(now),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::VestingWalletDelete { wallet_id } => {
                    total += diesel::delete(vesting_wallets::table)
                        .filter(vesting_wallets::wallet_id.eq(wallet_id))
                        .execute(conn)
                        .await?;
                }
            }
        }

        Ok(total)
    }
}
