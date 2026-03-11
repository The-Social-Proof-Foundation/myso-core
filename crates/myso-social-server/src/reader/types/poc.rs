// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct PocBadgeRow {
    pub badge_id: String,
    pub post_id: String,
    pub media_type: i16,
    pub issued_by: String,
    pub issued_at: i64,
    pub revoked: bool,
}

#[derive(Debug, Serialize)]
pub struct PocRevenueRedirectionRow {
    pub redirection_id: String,
    pub accused_post_id: String,
    pub original_post_id: String,
    pub redirect_percentage: i64,
    pub similarity_score: i64,
    pub created_at: i64,
    pub removed: bool,
}

#[derive(Debug, Serialize)]
pub struct PocAnalysisResultRow {
    pub post_id: String,
    pub media_type: i16,
    pub similarity_detected: bool,
    pub highest_similarity_score: i64,
    pub oracle_address: String,
    pub analysis_timestamp: i64,
}

#[derive(Debug, Serialize)]
pub struct PocDisputeRow {
    pub dispute_id: String,
    pub post_id: String,
    pub disputer: String,
    pub dispute_type: i16,
    pub evidence: String,
    pub status: i16,
    pub stake_amount: i64,
    pub submitted_at: i64,
    pub resolved_at: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct PocDisputeVoteRow {
    pub dispute_id: String,
    pub voter: String,
    pub vote_choice: i16,
    pub stake_amount: i64,
    pub voted_at: i64,
}

#[derive(Debug, Serialize)]
pub struct PocConfigRow {
    pub image_threshold: i64,
    pub video_threshold: i64,
    pub audio_threshold: i64,
    pub revenue_redirect_percentage: i64,
    pub dispute_cost: i64,
    pub updated_at: i64,
}
