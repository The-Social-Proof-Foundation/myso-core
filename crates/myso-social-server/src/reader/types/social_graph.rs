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

/// Unified profile response for GET /profiles/address/:address.
/// Single JSON shape for both profile-owning and wallet-only addresses.
#[derive(Debug, Clone, Serialize)]
pub struct ProfileByAddressResponse {
    pub id: Option<i32>,
    pub owner_address: String,
    pub profile_id: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub cover_photo: Option<String>,
    pub website: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
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
    pub mastodon_username: Option<String>,
    pub facebook_username: Option<String>,
    pub reddit_username: Option<String>,
    pub github_username: Option<String>,
    pub block_list_address: Option<String>,
    pub social_proof_token_address: Option<String>,
    pub reservation_pool_address: Option<String>,
    pub social_proof_token: Option<SocialProofTokenInfo>,
    pub selected_badge: Option<SelectedBadgeInfo>,
    pub selected_badge_id: Option<String>,
    pub selected_ecosystem_badge_id: Option<String>,
}

fn to_iso8601_utc(dt: chrono::NaiveDateTime) -> String {
    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
}

impl From<myso_indexer_alt_social_schema::models::Profile> for ProfileByAddressResponse {
    fn from(p: myso_indexer_alt_social_schema::models::Profile) -> Self {
        Self {
            id: Some(p.id),
            owner_address: p.owner_address,
            profile_id: p.profile_id,
            username: Some(p.username),
            display_name: p.display_name,
            bio: p.bio,
            profile_photo: p.profile_photo,
            cover_photo: p.cover_photo,
            website: p.website,
            created_at: Some(to_iso8601_utc(p.created_at)),
            updated_at: Some(to_iso8601_utc(p.updated_at)),
            followers_count: p.followers_count,
            following_count: p.following_count,
            post_count: p.post_count,
            min_offer_amount: p.min_offer_amount,
            birthdate: p.birthdate,
            current_location: p.current_location,
            raised_location: p.raised_location,
            phone: p.phone,
            email: p.email,
            gender: p.gender,
            political_view: p.political_view,
            religion: p.religion,
            education: p.education,
            primary_language: p.primary_language,
            relationship_status: p.relationship_status,
            x_username: p.x_username,
            mastodon_username: None,
            facebook_username: p.facebook_username,
            reddit_username: p.reddit_username,
            github_username: p.github_username,
            block_list_address: None,
            social_proof_token_address: p.social_proof_token_address,
            reservation_pool_address: p.reservation_pool_address,
            social_proof_token: None,
            selected_badge: None,
            selected_badge_id: p.selected_badge_id,
            selected_ecosystem_badge_id: p.selected_ecosystem_badge_id,
        }
    }
}

impl ProfileByAddressResponse {
    /// Apply enrichment (social_proof_token, selected_badge) from UniversalUserResult.
    pub fn with_enrichment(mut self, enriched: &UniversalUserResult) -> Self {
        self.social_proof_token = enriched.social_proof_token.clone();
        self.selected_badge = enriched.selected_badge.clone();
        self
    }
}

impl From<WalletOnlyProfile> for ProfileByAddressResponse {
    fn from(w: WalletOnlyProfile) -> Self {
        Self {
            id: w.id,
            owner_address: w.owner_address,
            profile_id: w.profile_id,
            username: w.username,
            display_name: w.display_name,
            bio: w.bio,
            profile_photo: w.profile_photo,
            cover_photo: w.cover_photo,
            website: w.website,
            created_at: w.created_at,
            updated_at: w.updated_at,
            followers_count: w.followers_count,
            following_count: w.following_count,
            post_count: w.post_count,
            min_offer_amount: w.min_offer_amount,
            birthdate: w.birthdate,
            current_location: w.current_location,
            raised_location: w.raised_location,
            phone: w.phone,
            email: w.email,
            gender: w.gender,
            political_view: w.political_view,
            religion: w.religion,
            education: w.education,
            primary_language: w.primary_language,
            relationship_status: w.relationship_status,
            x_username: w.x_username,
            mastodon_username: None,
            facebook_username: w.facebook_username,
            reddit_username: w.reddit_username,
            github_username: w.github_username,
            block_list_address: None,
            social_proof_token_address: w.social_proof_token_address,
            reservation_pool_address: w.reservation_pool_address,
            social_proof_token: None,
            selected_badge: None,
            selected_badge_id: w.selected_badge_id,
            selected_ecosystem_badge_id: None,
        }
    }
}

/// Minimal profile-like structure for wallet addresses without profiles.
/// Mirrors the JSON shape returned by mys-indexer when falling back to wallet_social_graph.
#[derive(Debug, Clone, Serialize)]
pub struct WalletOnlyProfile {
    pub id: Option<i32>,
    pub owner_address: String,
    pub profile_id: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub website: Option<String>,
    pub cover_photo: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub blocked_count: i32,
    pub post_count: i32,
    pub min_offer_amount: Option<i64>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
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
    pub reservation_pool: Option<String>,
}

impl WalletOnlyProfile {
    pub fn new(
        owner_address: String,
        followers_count: i32,
        following_count: i32,
        blocked_count: i32,
        created_at: Option<chrono::NaiveDateTime>,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> Self {
        Self {
            id: None,
            owner_address,
            profile_id: None,
            username: None,
            display_name: None,
            bio: None,
            profile_photo: None,
            website: None,
            cover_photo: None,
            followers_count,
            following_count,
            blocked_count,
            post_count: 0,
            min_offer_amount: None,
            created_at: created_at.map(to_iso8601_utc),
            updated_at: updated_at.map(to_iso8601_utc),
            birthdate: None,
            current_location: None,
            raised_location: None,
            phone: None,
            email: None,
            gender: None,
            political_view: None,
            religion: None,
            education: None,
            primary_language: None,
            relationship_status: None,
            x_username: None,
            facebook_username: None,
            reddit_username: None,
            github_username: None,
            instagram_username: None,
            linkedin_username: None,
            twitch_username: None,
            social_proof_token_address: None,
            reservation_pool_address: None,
            selected_badge_id: None,
            paid_messaging_enabled: false,
            paid_messaging_min_cost: None,
            reservation_pool: None,
        }
    }
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
