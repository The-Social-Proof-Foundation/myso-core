// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Date, Text};
use diesel::QueryableByName;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct SocialGraphAddressRow {
    pub address: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalUserResult {
    pub wallet_address: String,
    pub username: Option<String>,
    pub fullname: Option<String>,
    pub profile_photo: Option<String>,
    pub social_proof_token: Option<SocialProofTokenInfo>,
    pub selected_badge: Option<SelectedBadgeInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialProofTokenInfo {
    pub pool_id: Option<String>,
    pub token_address: Option<String>,
    pub is_active: bool,
    pub reservation_pool_id: Option<String>,
    pub reservation_percentage: f64,
    pub reservation_status: ReservationStatus,
    pub total_reserved: i64,
    pub required_threshold: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationStatus {
    Active,
    ThresholdMet,
    Inactive,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectedBadgeInfo {
    pub badge_id: String,
    pub badge_name: String,
    pub badge_icon_url: Option<String>,
    pub badge_media_url: Option<String>,
    pub platform_id: String,
    pub badge_type: i16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FollowDetail {
    pub id: i32,
    pub profile_id: Option<String>,
    #[serde(flatten)]
    pub user: UniversalUserResult,
    pub follows_back: bool,
    pub is_following: bool,
}

#[derive(Debug, Deserialize)]
pub struct FollowsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    pub viewer_id: Option<String>,
    pub sort: Option<String>,
    pub search: Option<String>,
}

impl FollowsQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(50).min(100)
    }
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let limit = self.limit();
        self.offset.unwrap_or_else(|| (page - 1) * limit)
    }
}

#[derive(Debug, Serialize)]
pub struct PaginationInfo {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub page: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize)]
pub struct SocialGraphChartQuery {
    pub bucket: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct DailyStatsPoint {
    pub day: String,
    pub event_type: String,
    pub event_count: i64,
}

#[derive(Debug, Serialize)]
pub struct SocialGraphChartData {
    pub chart_data: Vec<DailyStatsPoint>,
    pub date_range: super::common::DateRange,
    pub summary: super::common::ChartSummary,
}

#[derive(Debug, Serialize)]
pub struct FollowStatsRow {
    pub profile_id: Option<String>,
    pub wallet_address: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub profile_photo: Option<String>,
    pub followers_count: i64,
    pub following_count: i64,
    pub blocked_count: i64,
}

#[derive(Debug, Serialize)]
pub struct SocialStatsRow {
    pub followers_count: i64,
    pub following_count: i64,
    pub blocked_count: i64,
}

#[derive(Debug, Serialize)]
pub struct BlockedProfileRow {
    pub blocked_address: String,
    pub blocked_username: String,
    pub blocked_display_name: Option<String>,
    pub blocked_profile_photo: Option<String>,
    pub first_blocked_at: chrono::NaiveDateTime,
    pub last_blocked_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct BlockedPlatformRow {
    pub platform_id: String,
    pub name: String,
    pub blocked_by: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct SocialGraphChartRow {
    #[diesel(sql_type = Date)]
    pub day: chrono::NaiveDate,
    #[diesel(sql_type = Text)]
    pub event_type: String,
    #[diesel(sql_type = BigInt)]
    pub count: i64,
}
