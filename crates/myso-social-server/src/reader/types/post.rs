// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Date, Integer, Jsonb, Nullable, SmallInt, Text};
use diesel::QueryableByName;
use serde::Serialize;

#[derive(Debug, Serialize, QueryableByName)]
pub struct PostBasicRow {
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = Text)]
    pub profile_id: String,
    #[diesel(sql_type = Text)]
    pub content: String,
    #[diesel(sql_type = Text)]
    pub post_type: String,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub deleted_at: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub reaction_count: i64,
    #[diesel(sql_type = BigInt)]
    pub comment_count: i64,
    #[diesel(sql_type = BigInt)]
    pub repost_count: i64,
    #[diesel(sql_type = BigInt)]
    pub tips_received: i64,
    #[diesel(sql_type = Nullable<Text>)]
    pub mydata_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub poc_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub revenue_redirect_to: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub revenue_redirect_percentage: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub poc_reasoning: Option<String>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub poc_evidence_urls: Option<serde_json::Value>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub poc_similarity_score: Option<i64>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub poc_media_type: Option<i16>,
    #[diesel(sql_type = Nullable<Text>)]
    pub poc_oracle_address: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub poc_analyzed_at: Option<i64>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub poc_outcome: Option<i16>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub poc_redirection_kind: Option<i16>,
    #[diesel(sql_type = SmallInt)]
    pub poc_disputes_submitted: i16,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub composition_status: Option<i16>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub monetization_status: Option<i16>,
    #[diesel(sql_type = Nullable<Jsonb>)]
    pub media_asset_ids: Option<serde_json::Value>,
    #[diesel(sql_type = Nullable<Text>)]
    pub actor_address: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub sub_agent_id: Option<String>,
    #[diesel(sql_type = Nullable<SmallInt>)]
    pub action_identity_class: Option<i16>,
}

#[derive(Debug, Serialize)]
pub struct BlockedEventRow {
    pub event_type: String,
    pub blocked_address: Option<String>,
    pub processed_at: chrono::NaiveDateTime,
    pub event_id: Option<String>,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct PostConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub max_content_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_media_urls: i64,
    #[diesel(sql_type = BigInt)]
    pub max_mentions: i64,
    #[diesel(sql_type = BigInt)]
    pub max_metadata_size: i64,
    #[diesel(sql_type = BigInt)]
    pub max_description_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_reaction_length: i64,
    #[diesel(sql_type = BigInt)]
    pub commenter_tip_percentage: i64,
    #[diesel(sql_type = BigInt)]
    pub repost_tip_percentage: i64,
    #[diesel(sql_type = BigInt)]
    pub min_promotion_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub max_promotion_amount: i64,
    #[diesel(sql_type = BigInt)]
    pub min_view_duration_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub ecosystem_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
}

#[derive(Debug, Serialize)]
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
}

#[derive(Debug, Serialize)]
pub struct ReactionRow {
    pub user_address: String,
    pub reaction_text: String,
    pub created_at: i64,
    pub principal_owner: Option<String>,
    pub actor_address: Option<String>,
    pub sub_agent_id: Option<String>,
    pub action_identity_class: Option<i16>,
}

#[derive(Debug, Serialize)]
pub struct RepostRow {
    pub repost_id: String,
    pub original_post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub created_at: i64,
    pub actor_address: Option<String>,
    pub sub_agent_id: Option<String>,
    pub action_identity_class: Option<i16>,
}

#[derive(Debug, Serialize)]
pub struct PromotedPostRow {
    pub promotion_id: String,
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub payment_per_view: i64,
    pub total_budget: i64,
    pub remaining_budget: i64,
    pub active: bool,
    pub created_at: i64,
}

#[derive(Debug, Serialize)]
pub struct PromotionViewRow {
    pub post_id: String,
    pub promotion_id: String,
    pub viewer: String,
    pub payment_amount: i64,
    pub platform_fee: i64,
    pub ecosystem_fee: i64,
    pub recipient_amount: i64,
    pub view_duration: i64,
    pub platform_id: String,
    pub timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct PromotionStatsRow {
    pub total_views: i64,
    pub total_spent: i64,
    pub remaining_budget: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct PromotionTimeSeriesRow {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = BigInt)]
    pub views: i64,
    #[diesel(sql_type = BigInt)]
    pub spent: i64,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct PromotionHourlyRow {
    #[diesel(sql_type = Integer)]
    pub hour: i32,
    #[diesel(sql_type = BigInt)]
    pub views: i64,
    #[diesel(sql_type = BigInt)]
    pub spent: i64,
}
