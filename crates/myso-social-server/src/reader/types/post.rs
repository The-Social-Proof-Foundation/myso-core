// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Date, Integer, Jsonb, Nullable, SmallInt, Text};
use diesel::QueryableByName;
use serde::{Deserialize, Serialize};

use super::social_graph::PaginationInfo;

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

/// A promotion (advertising campaign) with its aggregated view and spend metrics.
///
/// Budget fields come from the newest `promoted_posts` row for the campaign; the
/// view-derived aggregates come from confirmed rows in `promotion_views`.
#[derive(Debug, Serialize)]
pub struct PromotedPostSummaryRow {
    pub promotion_id: String,
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub payment_per_view: i64,
    pub total_budget: i64,
    pub remaining_budget: i64,
    pub active: bool,
    pub created_at: i64,
    /// Confirmed views.
    pub views: i64,
    /// Distinct viewers; a viewer counts once per campaign.
    pub unique_viewers: i64,
    /// Gross advertiser spend: sum of `payment_amount` over confirmed views.
    pub total_spent: i64,
    /// Sum of withdrawals taken out of the campaign budget.
    pub total_withdrawn: i64,
    pub platform_fee_total: i64,
    pub ecosystem_fee_total: i64,
    /// Net amount transferred to viewers across all confirmed views.
    pub recipient_total: i64,
    pub first_view_at: Option<i64>,
    pub last_view_at: Option<i64>,
}

/// Cross-check of a campaign budget against its confirmed spend and withdrawals.
///
/// The identity `total_budget == total_spent + total_withdrawn + remaining_budget`
/// holds when the indexed ledger agrees with the campaign row.
#[derive(Debug, Serialize)]
pub struct PromotionReconciliation {
    pub total_budget: i64,
    pub total_spent: i64,
    pub total_withdrawn: i64,
    pub remaining_budget: i64,
    /// `total_spent + total_withdrawn`.
    pub accounted_for: i64,
    /// `total_budget - accounted_for - remaining_budget`. Zero when reconciled.
    pub unaccounted: i64,
    /// Remaining budget implied by the newest budget event, when one exists.
    pub ledger_remaining_budget: Option<i64>,
    pub budget_events: i64,
    pub status_events: i64,
    /// True when `unaccounted == 0` and the ledger agrees with `remaining_budget`.
    pub balanced: bool,
}

/// Full detail for a single campaign, including budget reconciliation.
#[derive(Debug, Serialize)]
pub struct PromotionDetailRow {
    pub promotion_id: String,
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub payment_per_view: i64,
    pub total_budget: i64,
    pub remaining_budget: i64,
    pub active: bool,
    pub created_at: i64,
    pub views: i64,
    pub unique_viewers: i64,
    pub total_spent: i64,
    pub total_withdrawn: i64,
    pub platform_fee_total: i64,
    pub ecosystem_fee_total: i64,
    pub recipient_total: i64,
    pub first_view_at: Option<i64>,
    pub last_view_at: Option<i64>,
    pub last_budget_event_at: Option<i64>,
    pub last_status_event_at: Option<i64>,
    pub reconciliation: PromotionReconciliation,
}

/// Advertiser-level rollup across every campaign matching the same filters as the list.
#[derive(Debug, Serialize)]
pub struct PromotionRollupRow {
    /// Number of campaigns in scope; usable as the pagination total for the campaign list.
    pub campaign_count: i64,
    pub active_campaigns: i64,
    pub total_budget: i64,
    pub total_spent: i64,
    pub total_withdrawn: i64,
    pub total_remaining_budget: i64,
    pub total_views: i64,
    /// Sum of per-campaign unique viewers; a viewer active in several campaigns counts once each.
    pub unique_viewers: i64,
    pub platform_fee_total: i64,
    pub ecosystem_fee_total: i64,
    pub recipient_total: i64,
    pub first_view_at: Option<i64>,
    pub last_view_at: Option<i64>,
}

/// One entry of a campaign budget ledger (`view_payment` or `withdrawal`).
#[derive(Debug, Serialize)]
pub struct PromotionBudgetEventRow {
    pub promotion_id: String,
    pub post_id: String,
    pub event_type: String,
    /// Gross amount debited (`view_payment`) or withdrawn (`withdrawal`).
    pub amount: i64,
    /// Budget remaining immediately after this event.
    pub remaining_budget: i64,
    pub timestamp: i64,
    pub transaction_id: String,
}

/// One entry of a campaign status history.
///
/// The indexer records activation toggles here as `status_toggled`. A withdrawal also
/// deactivates the campaign but is recorded in the budget ledger instead, so a complete
/// activity timeline combines this history with the budget events.
#[derive(Debug, Serialize)]
pub struct PromotionStatusEventRow {
    pub promotion_id: String,
    pub post_id: String,
    pub event_type: String,
    /// Account that triggered the change.
    pub triggered_by: String,
    /// Resulting active flag; `None` for event types that do not carry one.
    pub new_status: Option<bool>,
    pub amount: Option<i64>,
    pub timestamp: i64,
    pub transaction_id: String,
}

/// Per-platform split of a campaign's confirmed views.
#[derive(Debug, Serialize)]
pub struct PromotionPlatformBreakdownRow {
    pub platform_id: String,
    pub views: i64,
    pub unique_viewers: i64,
    pub spent: i64,
    pub platform_fees: i64,
    pub ecosystem_fees: i64,
    pub recipient_total: i64,
    /// Mean confirmed view duration in milliseconds.
    pub avg_view_duration: f64,
    pub first_view_at: Option<i64>,
    pub last_view_at: Option<i64>,
}

/// Window bounds actually applied to an analytics read, in epoch milliseconds.
#[derive(Debug, Serialize)]
pub struct PromotionWindowInfo {
    pub start_ms: i64,
    /// Exclusive upper bound.
    pub end_ms: i64,
    /// Relative bucket that produced the window when explicit bounds were absent.
    pub bucket: String,
    pub days: i64,
}

/// Filters shared by campaign lists, counts, and rollups.
#[derive(Debug, Default, Deserialize)]
pub struct PromotionListQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    /// Advertiser wallet address (`promoted_posts.owner`).
    pub owner: Option<String>,
    /// Advertiser profile id (`promoted_posts.profile_id`).
    pub profile_id: Option<String>,
    /// Restrict to campaigns with confirmed views on this platform.
    pub platform_id: Option<String>,
    /// Restrict to the campaign attached to this post.
    pub post_id: Option<String>,
    /// `active`, `inactive`, or `all` (default).
    pub status: Option<String>,
}

impl PromotionListQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).clamp(1, 100)
    }

    pub fn offset(&self) -> i64 {
        let limit = self.limit();
        let page = self.page.unwrap_or(1).max(1);
        self.offset.unwrap_or_else(|| (page - 1) * limit)
    }

    pub fn owner_filter(&self) -> Option<&str> {
        trimmed(self.owner.as_deref())
    }

    pub fn profile_filter(&self) -> Option<&str> {
        trimmed(self.profile_id.as_deref())
    }

    pub fn platform_filter(&self) -> Option<&str> {
        trimmed(self.platform_id.as_deref())
    }

    pub fn post_filter(&self) -> Option<&str> {
        trimmed(self.post_id.as_deref())
    }

    /// `None` means every campaign regardless of status.
    pub fn status_filter(&self) -> Result<Option<bool>, String> {
        match trimmed(self.status.as_deref())
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            None | Some("all") => Ok(None),
            Some("active") => Ok(Some(true)),
            Some("inactive") => Ok(Some(false)),
            Some(other) => Err(format!(
                "Invalid status '{}'. Must be one of: active, inactive, all",
                other
            )),
        }
    }
}

/// Window and scope options for promotion analytics reads.
#[derive(Debug, Default, Deserialize)]
pub struct PromotionAnalyticsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    /// Relative window used when `from`/`to` are absent: `7d`, `30d` (default), `90d`, `180d`, `1y`.
    pub bucket: Option<String>,
    /// Inclusive lower bound, epoch milliseconds.
    pub from: Option<i64>,
    /// Exclusive upper bound, epoch milliseconds.
    pub to: Option<i64>,
    /// Restrict to views recorded on this platform.
    pub platform_id: Option<String>,
    /// Restrict to campaigns owned by this advertiser address (global rollups only).
    pub owner: Option<String>,
}

impl PromotionAnalyticsQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).clamp(1, 100)
    }

    /// Chart series need one row per bucket in the window, so the cap is wider than for
    /// paginated event lists and defaults to covering a full year of daily buckets.
    pub fn series_limit(&self) -> i64 {
        self.limit.unwrap_or(366).clamp(1, 1000)
    }

    pub fn offset(&self) -> i64 {
        let limit = self.limit();
        let page = self.page.unwrap_or(1).max(1);
        self.offset.unwrap_or_else(|| (page - 1) * limit)
    }

    pub fn platform_filter(&self) -> Option<&str> {
        trimmed(self.platform_id.as_deref())
    }

    pub fn owner_filter(&self) -> Option<&str> {
        trimmed(self.owner.as_deref())
    }
}

fn trimmed(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|v| !v.is_empty())
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
    pub unique_viewers: i64,
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
    pub unique_viewers: i64,
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
    pub unique_viewers: i64,
    #[diesel(sql_type = BigInt)]
    pub spent: i64,
}

/// Budget ledger plus the reconciliation check for one campaign.
#[derive(Debug, Serialize)]
pub struct PromotionBudgetHistoryResponse {
    pub promotion_id: String,
    pub events: Vec<PromotionBudgetEventRow>,
    pub reconciliation: PromotionReconciliation,
    pub pagination: PaginationInfo,
}

/// Status history for one campaign.
#[derive(Debug, Serialize)]
pub struct PromotionStatusHistoryResponse {
    pub promotion_id: String,
    pub events: Vec<PromotionStatusEventRow>,
    pub pagination: PaginationInfo,
}

/// Per-platform view breakdown for one campaign over a resolved window.
#[derive(Debug, Serialize)]
pub struct PromotionPlatformBreakdownResponse {
    pub promotion_id: String,
    pub window: PromotionWindowInfo,
    pub platforms: Vec<PromotionPlatformBreakdownRow>,
    pub pagination: PaginationInfo,
}
