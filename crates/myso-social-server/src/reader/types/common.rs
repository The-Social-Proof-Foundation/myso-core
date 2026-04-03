// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct SystemStatsResponse {
    pub profiles: i64,
    pub platforms: i64,
    pub total_posts: i64,
    pub total_comments: i64,
    pub total_reactions: i64,
    pub social_proof_tokens: i64,
    pub total_social_relationships: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    pub start_date: String,
    pub end_date: String,
    pub days: i32,
    pub bucket: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChartSummary {
    pub total_follows: i64,
    pub total_unfollows: i64,
}
