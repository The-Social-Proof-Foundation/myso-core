// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Text};
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;

use crate::error::SocialError;
use crate::reader::types::{
    PocAnalysisResultRow, PocBadgeRow, PocConfigRow, PocDisputeRow, PocDisputeVoteRow,
    PocRevenueRedirectionRow,
};
use myso_pg_db::Db;

pub(crate) async fn list_poc_badges(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocBadgeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT badge_id, post_id, media_type, issued_by, issued_at, COALESCE(revoked, false) AS revoked,
               beneficiary_address, matched_anchor_id, media_index
        FROM (
            SELECT DISTINCT ON (badge_id) *
            FROM poc_badges
            ORDER BY badge_id, time DESC
        ) sub
        WHERE COALESCE(revoked, false) = false
        ORDER BY issued_at DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocBadgeRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_poc_badge_by_id(
    db: &Db,
    badge_id: &str,
) -> Result<Option<PocBadgeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT badge_id, post_id, media_type, issued_by, issued_at, COALESCE(revoked, false) AS revoked,
               beneficiary_address, matched_anchor_id, media_index
        FROM (
            SELECT DISTINCT ON (badge_id) *
            FROM poc_badges
            WHERE badge_id = $1
            ORDER BY badge_id, time DESC
        ) sub
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(badge_id)
        .get_result::<PocBadgeRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn list_poc_revenue_redirections(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocRevenueRedirectionRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT redirection_id, accused_post_id, original_post_id, redirect_percentage,
               similarity_score, created_at, removed
        FROM (
            SELECT DISTINCT ON (redirection_id) *
            FROM poc_revenue_redirections
            ORDER BY redirection_id, time DESC
        ) sub
        WHERE COALESCE(removed, false) = false
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocRevenueRedirectionRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_poc_analysis_results(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocAnalysisResultRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT post_id, similarity_detected, highest_similarity_score, media_type,
               oracle_address, original_creator, analysis_timestamp,
               reasoning, evidence_urls
        FROM poc_analysis_results
        ORDER BY analysis_timestamp DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocAnalysisResultRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_poc_disputes(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocDisputeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT dispute_id, post_id, disputer, dispute_type, evidence, status,
               resolution, stake_amount, voting_start_ms, voting_end_ms,
               winning_side, total_winning_stake, total_losing_stake,
               submitted_at, resolved_at, dispute_round, effective_dispute_fee,
               required_total_stake_quorum, quorum_met
        FROM (
            SELECT DISTINCT ON (dispute_id) *
            FROM poc_disputes
            ORDER BY dispute_id, time DESC
        ) sub
        ORDER BY submitted_at DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocDisputeRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_poc_dispute_by_id(
    db: &Db,
    dispute_id: &str,
) -> Result<Option<PocDisputeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT dispute_id, post_id, disputer, dispute_type, evidence, status,
               resolution, stake_amount, voting_start_ms, voting_end_ms,
               winning_side, total_winning_stake, total_losing_stake,
               submitted_at, resolved_at, dispute_round, effective_dispute_fee,
               required_total_stake_quorum, quorum_met
        FROM (
            SELECT DISTINCT ON (dispute_id) *
            FROM poc_disputes
            WHERE dispute_id = $1
            ORDER BY dispute_id, time DESC
        ) sub
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(dispute_id)
        .get_result::<PocDisputeRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_poc_dispute_votes(
    db: &Db,
    dispute_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocDisputeVoteRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT dispute_id, voter, vote_choice, stake_amount, voted_at,
               reward_claimed, reward_amount
        FROM (
            SELECT DISTINCT ON (dispute_id, voter) *
            FROM poc_dispute_votes
            WHERE dispute_id = $1
            ORDER BY dispute_id, voter, time DESC
        ) sub
        ORDER BY voted_at DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(dispute_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocDisputeVoteRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_poc_analytics(db: &Db) -> Result<serde_json::Value, SocialError> {
    use diesel::sql_types::BigInt as DieselBigInt;
    use diesel::QueryableByName;

    let mut conn = db.connect().await?;
    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = DieselBigInt)]
        count: i64,
    }
    let badges_count: i64 = diesel::sql_query(
        "SELECT COUNT(DISTINCT badge_id)::bigint as count FROM poc_badges WHERE revoked = false",
    )
    .get_result::<CountRow>(&mut conn)
    .await
    .map(|r| r.count)?;
    let disputes_count: i64 =
        diesel::sql_query("SELECT COUNT(DISTINCT dispute_id)::bigint as count FROM poc_disputes")
            .get_result::<CountRow>(&mut conn)
            .await
            .map(|r| r.count)?;
    let redirections_count: i64 = diesel::sql_query(
        "SELECT COUNT(DISTINCT redirection_id)::bigint as count FROM poc_revenue_redirections WHERE removed = false",
    )
    .get_result::<CountRow>(&mut conn)
    .await
    .map(|r| r.count)?;
    Ok(serde_json::json!({
        "total_badges": badges_count,
        "total_disputes": disputes_count,
        "total_revenue_redirections": redirections_count,
    }))
}

pub(crate) async fn get_poc_configuration(db: &Db) -> Result<Option<PocConfigRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT image_threshold, video_threshold, audio_threshold,
               revenue_redirect_percentage, dispute_cost,
               min_vote_stake, max_vote_stake, voting_duration_ms,
               max_reasoning_length, max_evidence_urls, max_votes_per_dispute,
               oracle_address, claim_treasury_fee_bps, max_referral_bps,
               video_embedded_audio_redirect_bps,
               dispute_quorum_base_stake,
               dispute_second_round_fee_multiplier_bps,
               dispute_second_round_quorum_multiplier_bps,
               username_beneficiary_join_referral_bps,
               updated_by, updated_at, transaction_id
        FROM poc_configuration
        ORDER BY updated_at DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .get_result::<PocConfigRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}
