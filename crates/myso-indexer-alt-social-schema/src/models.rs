// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    anonymous_votes, blocked_events, blocked_profiles, comments, community_votes, delegate_ratings,
    delegate_votes, delegates, governance_events, governance_registries, nominated_delegates,
    platform_blocked_profiles, platform_events, platform_memberships, platform_moderators,
    platform_token_airdrops, platforms, poc_analysis_results, poc_badges, poc_configuration,
    poc_dispute_votes, poc_disputes, poc_revenue_redirections, posts, posts_deletion_events,
    posts_moderation_events, posts_reports, profile_badges, profile_events, profiles, proposals,
    reaction_counts, reactions, reposts, reward_distributions, social_graph_events,
    social_graph_relationships, tips, vote_decryption_failures,
};
use crate::schema::{
    insurance_config, insurance_events, insurance_market_exposures, insurance_policies,
    insurance_policy_events, insurance_user_exposures, insurance_vault_transactions,
    insurance_vaults,
};
use crate::schema::{
    mydata_access_logs, mydata_config, mydata_data, mydata_purchases, mydata_registry,
    mydata_revenue, mydata_subscriptions,
};
use crate::schema::{object_migrated_events, upgrade_events};
use crate::schema::{
    profile_subscription_services, profile_subscriptions, subscription_events, subscription_revenue,
};
use crate::schema::{
    social_proof_tokens_config, social_proof_tokens_events, spt_exchange_config, spt_holdings,
    spt_pools, spt_price_history, spt_reservation_pools, spt_reservations, spt_revenue,
    spt_transactions, unified_revenue,
};
use crate::schema::{
    spot_bet_withdrawals, spot_bets, spot_config, spot_events, spot_payouts, spot_records,
    spot_refunds, spot_resolutions,
};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = profiles)]
pub struct Profile {
    pub id: i32,
    pub owner_address: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub website: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub cover_photo: Option<String>,
    pub profile_id: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub blocked_count: i32,
    pub post_count: i32,
    pub min_offer_amount: Option<i64>,
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
    pub social_proof_token_address: Option<String>,
    pub reservation_pool_address: Option<String>,
    pub selected_badge_id: Option<String>,
    pub selected_ecosystem_badge_id: Option<String>,
    pub paid_messaging_enabled: bool,
    pub paid_messaging_min_cost: Option<i64>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profiles)]
pub struct NewProfile {
    pub owner_address: String,
    pub username: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub website: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub cover_photo: Option<String>,
    pub profile_id: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub blocked_count: i32,
    pub post_count: i32,
    pub min_offer_amount: Option<i64>,
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
    pub social_proof_token_address: Option<String>,
    pub reservation_pool_address: Option<String>,
    pub selected_badge_id: Option<String>,
    pub selected_ecosystem_badge_id: Option<String>,
    pub paid_messaging_enabled: bool,
    pub paid_messaging_min_cost: Option<i64>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_graph_relationships)]
pub struct NewSocialGraphRelationship {
    pub follower_address: String,
    pub following_address: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_graph_events)]
pub struct NewSocialGraphEvent {
    pub event_type: String,
    pub follower_address: String,
    pub following_address: String,
    pub created_at: NaiveDateTime,
    pub event_id: Option<String>,
    pub raw_event_data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = blocked_events)]
pub struct NewBlockedEvent {
    pub event_id: Option<String>,
    pub event_type: String,
    pub blocker_address: String,
    pub blocked_address: Option<String>,
    pub raw_event_data: Option<serde_json::Value>,
    pub processed_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = blocked_profiles)]
pub struct NewBlockedProfile {
    pub blocker_address: String,
    pub blocked_address: String,
    pub blocked_profile_id: Option<String>,
    pub blocked_username: String,
    pub blocked_display_name: Option<String>,
    pub blocked_profile_photo: Option<String>,
    pub first_blocked_at: NaiveDateTime,
    pub last_blocked_at: NaiveDateTime,
    pub total_block_count: i32,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_badges)]
pub struct NewProfileBadge {
    pub profile_id: String,
    pub badge_id: String,
    pub badge_name: String,
    pub badge_description: Option<String>,
    pub badge_media_url: Option<String>,
    pub badge_icon_url: Option<String>,
    pub platform_id: String,
    pub assigned_by: String,
    pub assigned_at: i64,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub revoked_by: Option<String>,
    pub badge_type: i16,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_events)]
pub struct NewProfileEvent {
    pub event_type: String,
    pub profile_id: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = profiles)]
pub struct ProfileUpdateSet {
    pub updated_at: NaiveDateTime,
    pub display_name: Option<Option<String>>,
    pub bio: Option<Option<String>>,
    pub profile_photo: Option<Option<String>>,
    pub cover_photo: Option<Option<String>>,
    pub birthdate: Option<Option<String>>,
    pub current_location: Option<Option<String>>,
    pub raised_location: Option<Option<String>>,
    pub phone: Option<Option<String>>,
    pub email: Option<Option<String>>,
    pub gender: Option<Option<String>>,
    pub political_view: Option<Option<String>>,
    pub religion: Option<Option<String>>,
    pub education: Option<Option<String>>,
    pub primary_language: Option<Option<String>>,
    pub relationship_status: Option<Option<String>>,
    pub x_username: Option<Option<String>>,
    pub facebook_username: Option<Option<String>>,
    pub reddit_username: Option<Option<String>>,
    pub github_username: Option<Option<String>>,
    pub instagram_username: Option<Option<String>>,
    pub linkedin_username: Option<Option<String>>,
    pub twitch_username: Option<Option<String>>,
    pub min_offer_amount: Option<Option<i64>>,
    pub username: Option<String>,
    pub selected_badge_id: Option<Option<String>>,
    pub selected_ecosystem_badge_id: Option<Option<String>>,
    pub paid_messaging_enabled: Option<bool>,
    pub paid_messaging_min_cost: Option<Option<i64>>,
}

// =============================================================================
// GOVERNANCE MODELS
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = governance_registries)]
pub struct NewGovernanceRegistry {
    pub registry_type: i16,
    pub registry_id: String,
    pub delegate_count: i64,
    pub delegate_term_epochs: i64,
    pub proposal_submission_cost: i64,
    pub min_on_chain_age_days: i64,
    pub max_votes_per_user: i64,
    pub quadratic_base_cost: i64,
    pub voting_period_ms: i64,
    pub quorum_votes: i64,
    pub updated_at: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = nominated_delegates)]
pub struct NewNominatedDelegate {
    pub address: String,
    pub registry_type: i16,
    pub upvotes: i64,
    pub downvotes: i64,
    pub scheduled_term_start_epoch: i64,
    pub nomination_time: i64,
    pub status: i16,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegates)]
pub struct NewDelegate {
    pub address: String,
    pub registry_type: i16,
    pub upvotes: i64,
    pub downvotes: i64,
    pub proposals_reviewed: i64,
    pub proposals_submitted: i64,
    pub sided_winning_proposals: i64,
    pub sided_losing_proposals: i64,
    pub term_start: i64,
    pub term_end: i64,
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = proposals)]
pub struct NewProposal {
    pub id: String,
    pub title: String,
    pub description: String,
    pub proposal_type: i16,
    pub reference_id: Option<String>,
    pub metadata_json: Option<serde_json::Value>,
    pub submitter: String,
    pub submission_time: i64,
    pub delegate_approval_count: i64,
    pub delegate_rejection_count: i64,
    pub community_votes_for: i64,
    pub community_votes_against: i64,
    pub status: i16,
    pub voting_start_time: Option<i64>,
    pub voting_end_time: Option<i64>,
    pub reward_pool: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegate_ratings)]
pub struct NewDelegateRating {
    pub target_address: String,
    pub voter_address: String,
    pub registry_type: i16,
    pub is_active_delegate: bool,
    pub upvote: bool,
    pub rated_at: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = delegate_votes)]
pub struct NewDelegateVote {
    pub proposal_id: String,
    pub delegate_address: String,
    pub approve: bool,
    pub vote_time: i64,
    pub reason: Option<String>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = community_votes)]
pub struct NewCommunityVote {
    pub proposal_id: String,
    pub voter_address: String,
    pub vote_weight: i64,
    pub approve: bool,
    pub vote_time: i64,
    pub vote_cost: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = reward_distributions)]
pub struct NewRewardDistribution {
    pub proposal_id: String,
    pub recipient_address: String,
    pub amount: i64,
    pub distribution_time: i64,
    pub distribution_type: Option<String>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = governance_events)]
pub struct NewGovernanceEvent {
    pub event_type: String,
    pub registry_type: i16,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub anonymous_voting_related: Option<bool>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = anonymous_votes)]
pub struct NewAnonymousVote {
    pub proposal_id: String,
    pub voter_address: String,
    pub encrypted_vote_data: Vec<u8>,
    pub submitted_at: i64,
    pub decryption_status: i16,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = vote_decryption_failures)]
pub struct NewVoteDecryptionFailure {
    pub proposal_id: String,
    pub voter_address: String,
    pub failure_reason: String,
    pub attempted_at: i64,
    pub encrypted_vote_length: Option<i32>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, AsChangeset)]
#[diesel(table_name = proposals)]
pub struct ProposalUpdateSet {
    pub status: Option<i16>,
    pub voting_start_time: Option<i64>,
    pub voting_end_time: Option<i64>,
    pub reward_pool: Option<i64>,
    pub community_votes_for: Option<i64>,
    pub community_votes_against: Option<i64>,
    pub rescind_time: Option<i64>,
    pub implementation_time: Option<i64>,
    pub implemented_description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GovernanceRegistryUpdate {
    pub registry_type: i16,
    pub delegate_count: i64,
    pub delegate_term_epochs: i64,
    pub proposal_submission_cost: i64,
    pub max_votes_per_user: i64,
    pub quadratic_base_cost: i64,
    pub voting_period_ms: i64,
    pub quorum_votes: i64,
    pub updated_at: i64,
    pub transaction_id: String,
}

// =============================================================================
// POST MODELS
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub id: String,
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub content: String,
    pub media_urls: Option<serde_json::Value>,
    pub mentions: Option<serde_json::Value>,
    pub metadata_json: Option<serde_json::Value>,
    pub post_type: String,
    pub parent_post_id: Option<String>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub deleted_at: Option<i64>,
    pub reaction_count: i64,
    pub comment_count: i64,
    pub repost_count: i64,
    pub tips_received: i64,
    pub removed_from_platform: bool,
    pub removed_by: Option<String>,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
    pub mydata_id: Option<String>,
    pub revenue_recipient: Option<String>,
    pub poc_id: Option<String>,
    pub poc_reasoning: Option<String>,
    pub poc_evidence_urls: Option<serde_json::Value>,
    pub poc_similarity_score: Option<i64>,
    pub poc_media_type: Option<i16>,
    pub poc_oracle_address: Option<String>,
    pub poc_analyzed_at: Option<i64>,
    pub revenue_redirect_to: Option<String>,
    pub revenue_redirect_percentage: Option<i64>,
    pub requires_subscription: Option<bool>,
    pub subscription_service_id: Option<String>,
    pub subscription_price: Option<i64>,
    pub encrypted_content_hash: Option<String>,
    pub promotion_id: Option<String>,
    pub enable_spt: bool,
    pub enable_poc: bool,
    pub enable_spot: bool,
    pub spot_id: Option<String>,
    pub spt_id: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = comments)]
pub struct NewComment {
    pub id: String,
    pub comment_id: String,
    pub post_id: String,
    pub parent_comment_id: Option<String>,
    pub owner: String,
    pub profile_id: String,
    pub content: String,
    pub media_urls: Option<serde_json::Value>,
    pub mentions: Option<serde_json::Value>,
    pub metadata_json: Option<serde_json::Value>,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub deleted_at: Option<i64>,
    pub reaction_count: i64,
    pub comment_count: i64,
    pub repost_count: i64,
    pub tips_received: i64,
    pub removed_from_platform: bool,
    pub removed_by: Option<String>,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = reactions)]
pub struct NewReaction {
    pub object_id: String,
    pub user_address: String,
    pub reaction_text: String,
    pub is_post: bool,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = reaction_counts)]
pub struct NewReactionCount {
    pub object_id: String,
    pub reaction_text: String,
    pub count: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = reposts)]
pub struct NewRepost {
    pub id: String,
    pub repost_id: String,
    pub original_id: String,
    pub original_post_id: String,
    pub is_original_post: bool,
    pub owner: String,
    pub profile_id: String,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = tips)]
pub struct NewTip {
    pub tipper: String,
    pub recipient: String,
    pub object_id: String,
    pub amount: i64,
    pub is_post: bool,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts_moderation_events)]
pub struct NewModerationEvent {
    pub object_id: String,
    pub platform_id: String,
    pub removed: bool,
    pub moderated_by: String,
    pub moderated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts_reports)]
pub struct NewReport {
    pub object_id: String,
    pub is_comment: bool,
    pub reporter: String,
    pub reason_code: i16,
    pub description: String,
    pub reported_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts_deletion_events)]
pub struct NewDeletionEvent {
    pub object_id: String,
    pub owner: String,
    pub profile_id: String,
    pub is_post: bool,
    pub post_type: Option<String>,
    pub post_id: Option<String>,
    pub deleted_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

// =============================================================================
// PLATFORM MODELS
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platforms)]
pub struct NewPlatform {
    pub platform_id: String,
    pub name: String,
    pub tagline: String,
    pub description: Option<String>,
    pub logo: Option<String>,
    pub developer_address: String,
    pub terms_of_service: Option<String>,
    pub privacy_policy: Option<String>,
    pub platform_names: Option<serde_json::Value>,
    pub links: Option<serde_json::Value>,
    pub status: i16,
    pub release_date: Option<String>,
    pub shutdown_date: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_approved: bool,
    pub approval_changed_at: Option<NaiveDateTime>,
    pub approved_by: Option<String>,
    pub wants_dao_governance: Option<bool>,
    pub governance_registry_id: Option<String>,
    pub delegate_count: Option<i64>,
    pub delegate_term_epochs: Option<i64>,
    pub max_votes_per_user: Option<i64>,
    pub min_on_chain_age_days: Option<i64>,
    pub proposal_submission_cost: Option<i64>,
    pub quadratic_base_cost: Option<i64>,
    pub quorum_votes: Option<i64>,
    pub voting_period_epochs: Option<i64>,
    pub treasury: Option<i64>,
    pub version: Option<i64>,
    pub primary_category: String,
    pub secondary_category: Option<String>,
    pub deleted_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_moderators)]
pub struct NewPlatformModerator {
    pub platform_id: String,
    pub moderator_address: String,
    pub added_by: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_blocked_profiles)]
pub struct NewPlatformBlockedProfile {
    pub platform_id: String,
    pub wallet_address: String,
    pub blocked_by: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_events)]
pub struct NewPlatformEvent {
    pub event_type: String,
    pub platform_id: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub reasoning: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_memberships)]
pub struct NewPlatformMembership {
    pub platform_id: String,
    pub wallet_address: String,
    pub joined_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = platform_token_airdrops)]
pub struct NewPlatformTokenAirdrop {
    pub platform_id: String,
    pub recipient: String,
    pub amount: i64,
    pub reason_code: i16,
    pub executed_by: String,
    pub timestamp: i64,
    pub created_at: NaiveDateTime,
    pub event_id: Option<String>,
}

// =============================================================================
// PROOF OF CREATIVITY (POC) MODELS
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_badges)]
pub struct NewPocBadge {
    pub badge_id: String,
    pub post_id: String,
    pub media_type: i16,
    pub issued_by: String,
    pub issued_at: i64,
    pub revoked: bool,
    pub revoked_at: Option<i64>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_revenue_redirections)]
pub struct NewPocRevenueRedirection {
    pub redirection_id: String,
    pub accused_post_id: String,
    pub original_post_id: String,
    pub redirect_percentage: i64,
    pub similarity_score: i64,
    pub created_at: i64,
    pub removed: bool,
    pub removed_at: Option<i64>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_analysis_results)]
pub struct NewPocAnalysisResult {
    pub post_id: String,
    pub media_type: i16,
    pub similarity_detected: bool,
    pub highest_similarity_score: i64,
    pub oracle_address: String,
    pub original_creator: Option<String>,
    pub analysis_timestamp: i64,
    pub transaction_id: String,
    pub reasoning: Option<String>,
    pub evidence_urls: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_disputes)]
pub struct NewPocDispute {
    pub dispute_id: String,
    pub post_id: String,
    pub disputer: String,
    pub dispute_type: i16,
    pub evidence: String,
    pub status: i16,
    pub stake_amount: i64,
    pub voting_start_epoch: i64,
    pub voting_end_epoch: i64,
    pub resolution: Option<i16>,
    pub winning_side: Option<i16>,
    pub total_winning_stake: Option<i64>,
    pub total_losing_stake: Option<i64>,
    pub submitted_at: i64,
    pub resolved_at: Option<i64>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_dispute_votes)]
pub struct NewPocDisputeVote {
    pub dispute_id: String,
    pub voter: String,
    pub vote_choice: i16,
    pub stake_amount: i64,
    pub voted_at: i64,
    pub reward_claimed: bool,
    pub reward_amount: Option<i64>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_configuration)]
pub struct NewPocConfiguration {
    pub image_threshold: i64,
    pub video_threshold: i64,
    pub audio_threshold: i64,
    pub revenue_redirect_percentage: i64,
    pub dispute_cost: i64,
    pub dispute_protocol_fee: i64,
    pub min_vote_stake: i64,
    pub max_vote_stake: i64,
    pub voting_duration_epochs: i64,
    pub max_reasoning_length: i64,
    pub max_evidence_urls: i64,
    pub max_votes_per_dispute: i64,
    pub oracle_address: Option<String>,
    pub updated_by: String,
    pub updated_at: i64,
    pub transaction_id: String,
}

// =============================================================================
// MYDATA MARKETPLACE MODELS
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_data)]
pub struct NewMyDataData {
    pub mydata_id: String,
    pub owner: String,
    pub media_type: String,
    pub tags: serde_json::Value,
    pub platform_id: Option<String>,
    pub timestamp_start: i64,
    pub timestamp_end: Option<i64>,
    pub created_at: i64,
    pub last_updated: i64,
    pub one_time_price: Option<i64>,
    pub subscription_price: Option<i64>,
    pub subscription_duration_days: i64,
    pub geographic_region: Option<String>,
    pub data_quality: Option<String>,
    pub sample_size: Option<i64>,
    pub collection_method: Option<String>,
    pub is_updating: bool,
    pub update_frequency: Option<String>,
    pub version: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_purchases)]
pub struct NewMyDataPurchase {
    pub mydata_id: String,
    pub buyer: String,
    pub price: i64,
    pub purchase_type: String,
    pub purchase_time: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_subscriptions)]
pub struct NewMyDataSubscription {
    pub mydata_id: String,
    pub subscriber: String,
    pub subscription_start: i64,
    pub subscription_end: i64,
    pub price: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_revenue)]
pub struct NewMyDataRevenue {
    pub mydata_id: String,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub revenue_time: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_access_logs)]
pub struct NewMyDataAccessLog {
    pub mydata_id: String,
    pub user_address: String,
    pub access_type: String,
    pub access_time: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_registry)]
pub struct NewMyDataRegistry {
    pub ip_id: String,
    pub owner: String,
    pub registered_at: i64,
    pub unregistered_at: Option<i64>,
    pub is_active: bool,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = mydata_config)]
pub struct NewMyDataConfig {
    pub updated_by: String,
    pub enable_flag: bool,
    pub max_tags: i64,
    pub max_subscription_days: i64,
    pub max_free_access_grants: i64,
    pub timestamp_ms: i64,
    pub transaction_id: String,
}

// =============================================================================
// INSURANCE MODELS
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_config)]
pub struct NewInsuranceConfig {
    pub updated_by: String,
    pub enable_flag: bool,
    pub min_coverage_bps: i64,
    pub max_coverage_bps: i64,
    pub max_duration_ms: i64,
    pub fee_bps: i64,
    pub version: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_vaults)]
pub struct NewInsuranceVault {
    pub vault_id: String,
    pub underwriter: String,
    pub capital_balance: i64,
    pub reserved: i64,
    pub base_rate_bps_per_day: i64,
    pub utilization_multiplier_bps: i64,
    pub max_exposure_per_market: i64,
    pub max_exposure_per_user: i64,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_policies)]
pub struct NewInsurancePolicy {
    pub policy_id: String,
    pub market_id: String,
    pub insured: String,
    pub option_id: i16,
    pub covered_amount: i64,
    pub coverage_bps: i64,
    pub premium_paid: i64,
    pub start_time_ms: i64,
    pub expiry_time_ms: i64,
    pub vault_id: String,
    pub status: i16,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_events)]
pub struct NewInsuranceEventLog {
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_vault_transactions)]
pub struct NewInsuranceVaultTransaction {
    pub vault_id: String,
    pub transaction_type: String,
    pub amount: i64,
    pub balance_after: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_policy_events)]
pub struct NewInsurancePolicyEvent {
    pub policy_id: String,
    pub event_type: String,
    pub market_id: String,
    pub insured: String,
    pub option_id: i16,
    pub covered_amount: i64,
    pub coverage_bps: i64,
    pub premium_paid: i64,
    pub reserve_locked: i64,
    pub refunded_amount: Option<i64>,
    pub fee_paid: Option<i64>,
    pub payout: Option<i64>,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_market_exposures)]
pub struct NewInsuranceMarketExposure {
    pub vault_id: String,
    pub market_id: String,
    pub option_id: i16,
    pub reserved_amount: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = insurance_user_exposures)]
pub struct NewInsuranceUserExposure {
    pub vault_id: String,
    pub insured: String,
    pub reserved_amount: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

// =============================================================================
// SOCIAL PROOF OF TRUTH (SPoT) MODELS
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_records)]
pub struct NewSpotRecord {
    pub post_id: String,
    pub status: i16,
    pub outcome: Option<i16>,
    pub amm_split_bps_used: i32,
    pub betting_options: Option<serde_json::Value>,
    pub option_escrow: Option<serde_json::Value>,
    pub resolution_window_epochs: Option<i64>,
    pub max_resolution_window_epochs: Option<i64>,
    pub created_epoch: i64,
    pub last_resolution_epoch: Option<i64>,
    pub version: i64,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_bets)]
pub struct NewSpotBet {
    pub post_id: String,
    pub user_address: String,
    pub option_id: i16,
    pub escrow_amount: i64,
    pub amm_amount: i64,
    pub timestamp_epoch: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_payouts)]
pub struct NewSpotPayout {
    pub post_id: String,
    pub user_address: String,
    pub amount: i64,
    pub timestamp_epoch: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_refunds)]
pub struct NewSpotRefund {
    pub post_id: String,
    pub user_address: String,
    pub amount: i64,
    pub timestamp_epoch: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_resolutions)]
pub struct NewSpotResolution {
    pub post_id: String,
    pub outcome: i16,
    pub total_escrow: i64,
    pub fee_taken: i64,
    pub resolved_epoch: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub reasoning: String,
    pub evidence_urls: serde_json::Value,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_events)]
pub struct NewSpotEventLog {
    pub event_type: String,
    pub post_id: String,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_config)]
pub struct NewSpotConfig {
    pub updated_by: String,
    pub enable_flag: bool,
    pub confidence_threshold_bps: i64,
    pub resolution_window_epochs: i64,
    pub max_resolution_window_epochs: i64,
    pub payout_delay_ms: i64,
    pub fee_bps: i64,
    pub fee_split_bps_platform: i64,
    pub oracle_address: String,
    pub max_single_bet: i64,
    pub version: i64,
    pub timestamp_ms: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spot_bet_withdrawals)]
pub struct NewSpotBetWithdrawal {
    pub post_id: String,
    pub user_address: String,
    pub option_id: i16,
    pub amount: i64,
    pub fee_taken: i64,
    pub timestamp_epoch: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

// =============================================================================
// SOCIAL PROOF TOKEN (SPT) CONSTANTS
// =============================================================================

pub const TOKEN_TYPE_PROFILE: i16 = 1;
pub const TOKEN_TYPE_POST: i16 = 2;
pub const TRANSACTION_TYPE_BUY: &str = "BUY";
pub const TRANSACTION_TYPE_SELL: &str = "SELL";
pub const RESERVATION_POOL_STATUS_ACTIVE: &str = "active";
pub const RESERVATION_POOL_STATUS_THRESHOLD_MET: &str = "threshold_met";
pub const REVENUE_SOURCE_SPT: &str = "spt";
pub const REVENUE_TYPE_SPT_CREATOR_FEE: &str = "creator_fee";
pub const REVENUE_TYPE_SPT_PLATFORM_FEE: &str = "platform_fee";
pub const REVENUE_TYPE_SPT_TREASURY_FEE: &str = "treasury_fee";
pub const CURRENCY_MYSO: &str = "MYSO";
pub const CONTENT_TYPE_TOKEN: &str = "token";

// =============================================================================
// SOCIAL PROOF TOKEN (SPT) MODELS
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_pools)]
pub struct NewSptPool {
    pub pool_id: String,
    pub token_type: i16,
    pub owner: String,
    pub associated_id: String,
    pub symbol: String,
    pub name: String,
    pub circulating_supply: i64,
    pub base_price: i64,
    pub quadratic_coefficient: i64,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_holdings)]
pub struct NewSptHolding {
    pub pool_id: String,
    pub holder_address: String,
    pub amount: i64,
    pub acquired_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_transactions)]
pub struct NewSptTransaction {
    pub pool_id: String,
    pub transaction_type: String,
    pub sender: String,
    pub amount: i64,
    pub myso_amount: i64,
    pub fee_amount: i64,
    pub creator_fee: i64,
    pub platform_fee: i64,
    pub treasury_fee: i64,
    pub price: i64,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_reservation_pools)]
pub struct NewSptReservationPool {
    pub pool_id: String,
    pub associated_id: String,
    pub token_type: i16,
    pub owner: String,
    pub total_reserved: i64,
    pub required_threshold: i64,
    pub status: String,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_reservations)]
pub struct NewSptReservation {
    pub pool_id: String,
    pub reserver_address: String,
    pub amount: i64,
    pub reserved_at: i64,
    pub fee_amount: Option<i64>,
    pub creator_fee: Option<i64>,
    pub platform_fee: Option<i64>,
    pub treasury_fee: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_exchange_config)]
pub struct NewSptExchangeConfig {
    pub updated_by: String,
    pub post_threshold: i64,
    pub profile_threshold: i64,
    pub max_individual_reservation_bps: i64,
    pub total_fee_bps: i64,
    pub creator_fee_bps: i64,
    pub platform_fee_bps: i64,
    pub treasury_fee_bps: i64,
    pub trading_creator_fee_bps: i64,
    pub trading_platform_fee_bps: i64,
    pub trading_treasury_fee_bps: i64,
    pub reservation_creator_fee_bps: i64,
    pub reservation_platform_fee_bps: i64,
    pub reservation_treasury_fee_bps: i64,
    pub max_reservers_per_pool: i64,
    pub base_price: i64,
    pub quadratic_coefficient: i64,
    pub max_hold_percent_bps: i64,
    pub trading_enabled: bool,
    pub updated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_price_history)]
pub struct NewSptPriceHistory {
    pub pool_id: String,
    pub price: i64,
    pub circulating_supply: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = spt_revenue)]
pub struct NewSptRevenue {
    pub pool_id: String,
    pub transaction_type: String,
    pub trader: String,
    pub creator_address: String,
    pub platform_address: String,
    pub treasury_address: String,
    pub creator_fee: i64,
    pub platform_fee: i64,
    pub treasury_fee: i64,
    pub total_fee: i64,
    pub token_amount: i64,
    pub myso_amount: i64,
    pub token_price: i64,
    pub revenue_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl NewSptRevenue {
    pub fn from_buy_event(
        pool_id: String,
        trader: String,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        creator_fee: i64,
        platform_fee: i64,
        treasury_fee: i64,
        token_amount: i64,
        myso_amount: i64,
        token_price: i64,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            pool_id,
            transaction_type: TRANSACTION_TYPE_BUY.to_string(),
            trader,
            creator_address,
            platform_address,
            treasury_address,
            creator_fee,
            platform_fee,
            treasury_fee,
            total_fee: creator_fee + platform_fee + treasury_fee,
            token_amount,
            myso_amount,
            token_price,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
        }
    }

    pub fn from_sell_event(
        pool_id: String,
        trader: String,
        creator_address: String,
        platform_address: String,
        treasury_address: String,
        creator_fee: i64,
        platform_fee: i64,
        treasury_fee: i64,
        token_amount: i64,
        myso_amount: i64,
        token_price: i64,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            pool_id,
            transaction_type: TRANSACTION_TYPE_SELL.to_string(),
            trader,
            creator_address,
            platform_address,
            treasury_address,
            creator_fee,
            platform_fee,
            treasury_fee,
            total_fee: creator_fee + platform_fee + treasury_fee,
            token_amount,
            myso_amount,
            token_price,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
        }
    }
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = unified_revenue)]
pub struct NewUnifiedRevenue {
    pub revenue_source: String,
    pub revenue_type: String,
    pub creator_address: String,
    pub platform_address: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub payer_address: String,
    pub recipient_address: String,
    pub revenue_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

impl NewUnifiedRevenue {
    pub fn from_spt(
        revenue_type: String,
        creator_address: String,
        platform_address: Option<String>,
        amount: i64,
        pool_id: String,
        payer_address: String,
        recipient_address: String,
        revenue_time: i64,
        transaction_id: String,
    ) -> Self {
        Self {
            revenue_source: REVENUE_SOURCE_SPT.to_string(),
            revenue_type,
            creator_address,
            platform_address,
            amount,
            currency: CURRENCY_MYSO.to_string(),
            content_id: Some(pool_id),
            content_type: Some(CONTENT_TYPE_TOKEN.to_string()),
            payer_address,
            recipient_address,
            revenue_time,
            time: chrono::Utc::now(),
            transaction_id,
        }
    }
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_proof_tokens_config)]
pub struct NewSocialProofTokensConfig {
    pub trading_enabled: bool,
    pub admin_address: String,
    pub reason: String,
    pub timestamp_ms: i64,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = social_proof_tokens_events)]
pub struct NewSocialProofTokensEvent {
    pub event_type: String,
    pub event_data: serde_json::Value,
    pub event_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// =============================================================================
// UPGRADE MODELS (from upgrade.move)
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = upgrade_events)]
pub struct NewUpgradeEvent {
    pub package_id: String,
    pub version: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = object_migrated_events)]
pub struct NewObjectMigratedEvent {
    pub object_id: String,
    pub object_type: String,
    pub old_version: i64,
    pub new_version: i64,
    pub migrated_by: String,
    pub event_id: String,
    pub transaction_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// =============================================================================
// PROFILE SUBSCRIPTION MODELS
// =============================================================================

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_subscription_services)]
pub struct NewProfileSubscriptionService {
    pub service_id: String,
    pub profile_owner: String,
    pub profile_id: String,
    pub monthly_fee: i64,
    pub active: bool,
    pub subscriber_count: i64,
    pub created_at: i64,
    pub updated_at: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = profile_subscriptions)]
pub struct NewProfileSubscription {
    pub subscription_id: String,
    pub service_id: String,
    pub subscriber: String,
    pub created_at: i64,
    pub expires_at: i64,
    pub auto_renew: bool,
    pub renewal_balance: i64,
    pub renewal_count: i64,
    pub cancelled_at: Option<i64>,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = subscription_events)]
pub struct NewSubscriptionEvent {
    pub event_type: String,
    pub subscription_id: Option<String>,
    pub service_id: Option<String>,
    pub subscriber: Option<String>,
    pub event_data: serde_json::Value,
    pub event_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = subscription_revenue)]
pub struct NewSubscriptionRevenue {
    pub service_id: String,
    pub subscription_id: Option<String>,
    pub from_address: String,
    pub to_address: String,
    pub amount: i64,
    pub revenue_type: String,
    pub payment_time: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub processing_success: bool,
    pub processing_error: Option<String>,
}
