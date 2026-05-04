// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text};
use serde::Serialize;

use super::common::DateRange;
use super::social_graph::DailyStatsPoint;

#[derive(Debug, Serialize)]
pub struct ProfileEventRow {
    pub event_type: String,
    pub profile_id: String,
    pub event_data: serde_json::Value,
    pub event_id: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct ProfilePlatformEventRow {
    pub event_type: String,
    pub platform_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub event_id: Option<String>,
    pub event_data: serde_json::Value,
}

#[derive(Debug, Serialize, QueryableByName)]
pub struct ProfileBadgeRow {
    #[diesel(sql_type = Text)]
    pub badge_id: String,
    #[diesel(sql_type = Text)]
    pub badge_name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub badge_description: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub badge_media_url: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub badge_icon_url: Option<String>,
    #[diesel(sql_type = Text)]
    pub platform_id: String,
    #[diesel(sql_type = Text)]
    pub assigned_by: String,
    #[diesel(sql_type = BigInt)]
    pub assigned_at: i64,
    #[diesel(sql_type = Bool)]
    pub revoked: bool,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub revoked_at: Option<i64>,
    #[diesel(sql_type = Nullable<Text>)]
    pub revoked_by: Option<String>,
    #[diesel(sql_type = SmallInt)]
    pub badge_type: i16,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDailyStatsSummary {
    pub total_profile_created: i64,
    pub total_profile_updated: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDailyStatsChartData {
    pub chart_data: Vec<DailyStatsPoint>,
    pub date_range: DateRange,
    pub summary: ProfileDailyStatsSummary,
}
