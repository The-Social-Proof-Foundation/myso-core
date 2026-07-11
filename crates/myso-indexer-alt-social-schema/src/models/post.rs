// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    comments, posts, posts_deletion_events, posts_moderation_events, posts_reports,
    posts_transfers, reaction_counts, reactions, reposts, tips,
};

pub const MAX_CONTENT_LENGTH: usize = 5000;
pub const MAX_MEDIA_URLS: usize = 10;
pub const MAX_MENTIONS: usize = 10;
pub const MAX_METADATA_SIZE: usize = 10000;
pub const MAX_DESCRIPTION_LENGTH: usize = 500;
pub const MAX_REACTION_LENGTH: usize = 20;
pub const COMMENTER_TIP_PERCENTAGE: i32 = 80;
pub const REPOST_TIP_PERCENTAGE: i32 = 50;
pub const MIN_PROMOTION_AMOUNT: i64 = 1000;
pub const MAX_PROMOTION_AMOUNT: i64 = 100_000_000;
pub const MIN_VIEW_DURATION: i64 = 3000;
pub const POST_TYPE_STANDARD: &str = "standard";
pub const POST_TYPE_REPOST: &str = "repost";
pub const POST_TYPE_QUOTE_REPOST: &str = "quote_repost";
pub const REPORT_REASON_SPAM: i16 = 1;
pub const REPORT_REASON_OFFENSIVE: i16 = 2;
pub const REPORT_REASON_MISINFORMATION: i16 = 3;
pub const REPORT_REASON_ILLEGAL: i16 = 4;
pub const REPORT_REASON_IMPERSONATION: i16 = 5;
pub const REPORT_REASON_HARASSMENT: i16 = 6;
pub const REPORT_REASON_OTHER: i16 = 99;
pub const MODERATION_APPROVED: i16 = 1;
pub const MODERATION_FLAGGED: i16 = 2;
pub const PERMISSION_ALLOW_COMMENTS: i32 = 1;
pub const PERMISSION_ALLOW_REACTIONS: i32 = 2;
pub const PERMISSION_ALLOW_REPOSTS: i32 = 4;
pub const PERMISSION_ALLOW_QUOTES: i32 = 8;
pub const PERMISSION_ALLOW_TIPS: i32 = 16;
pub const ENABLE_SPT: i32 = 1;
pub const ENABLE_SPOT: i32 = 4;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
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
    pub total_tip_volume: i64,
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
    pub poc_outcome: Option<i16>,
    pub poc_redirection_kind: Option<i16>,
    pub poc_disputes_submitted: i16,
    pub revenue_redirect_to: Option<String>,
    pub revenue_redirect_percentage: Option<i64>,
    pub requires_subscription: Option<bool>,
    pub subscription_service_id: Option<String>,
    pub subscription_price: Option<i64>,
    pub subscription_min_tier_level: Option<i64>,
    pub post_access_kind: Option<String>,
    pub encrypted_content_hash: Option<String>,
    pub promotion_id: Option<String>,
    pub enable_spt: bool,
    pub enable_spot: bool,
    pub spot_id: Option<String>,
    pub spot_claim_id: Option<String>,
    pub spt_id: Option<String>,
    pub platform_id: Option<String>,
    pub permissions: Option<i16>,
    pub sub_agent_id: Option<String>,
    pub action_identity_class: Option<i16>,
    pub organization_id: Option<String>,
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
    pub actor_address: Option<String>,
    pub sub_agent_id: Option<String>,
    pub action_identity_class: Option<i16>,
    pub organization_id: Option<String>,
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
    pub principal_owner: Option<String>,
    pub actor_address: Option<String>,
    pub sub_agent_id: Option<String>,
    pub action_identity_class: Option<i16>,
    pub organization_id: Option<String>,
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
    pub actor_address: Option<String>,
    pub sub_agent_id: Option<String>,
    pub action_identity_class: Option<i16>,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = tips)]
pub struct NewTip {
    pub tipper: String,
    pub recipient: String,
    pub object_id: String,
    pub amount: i64,
    pub is_post: bool,
    pub coin_type: String,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
    pub organization_id: Option<String>,
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

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = posts_moderation_events)]
pub struct PostModerationEventRow {
    pub id: i32,
    pub object_id: String,
    pub platform_id: String,
    pub removed: bool,
    pub moderated_by: String,
    pub moderated_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = posts_reports)]
pub struct PostReport {
    pub id: i32,
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

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = posts_deletion_events)]
pub struct PostDeletionEventRow {
    pub id: i32,
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

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = posts_transfers)]
pub struct NewPostTransfer {
    pub object_id: String,
    pub previous_owner: String,
    pub new_owner: String,
    pub is_post: bool,
    pub transferred_at: i64,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = posts_transfers)]
pub struct PostTransfer {
    pub id: i32,
    pub object_id: String,
    pub previous_owner: String,
    pub new_owner: String,
    pub is_post: bool,
    pub transferred_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentRow {
    pub comment_id: String,
    pub post_id: String,
    pub parent_comment_id: Option<String>,
    pub owner: String,
    pub profile_id: String,
    pub content: String,
    pub created_at: i64,
    pub reaction_count: i64,
    pub comment_count: i64,
    pub actor_address: Option<String>,
    pub sub_agent_id: Option<String>,
    pub action_identity_class: Option<i16>,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReactionRow {
    pub user_address: String,
    pub reaction_text: String,
    pub created_at: i64,
    pub principal_owner: Option<String>,
    pub actor_address: Option<String>,
    pub sub_agent_id: Option<String>,
    pub action_identity_class: Option<i16>,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepostRow {
    pub repost_id: String,
    pub original_post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub created_at: i64,
    pub actor_address: Option<String>,
    pub sub_agent_id: Option<String>,
    pub action_identity_class: Option<i16>,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TipRow {
    pub tipper: String,
    pub recipient: String,
    pub amount: i64,
    pub created_at: i64,
    pub organization_id: Option<String>,
}
