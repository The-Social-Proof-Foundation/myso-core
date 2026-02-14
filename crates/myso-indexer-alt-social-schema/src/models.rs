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
    platform_token_airdrops, platforms, posts, posts_deletion_events, posts_moderation_events,
    posts_reports, profile_badges, profile_events, profiles, proposals, reaction_counts, reactions,
    reposts, reward_distributions, social_graph_events, social_graph_relationships, tips,
    vote_decryption_failures,
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
    pub encrypted_vote_data: Option<Vec<u8>>,
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
