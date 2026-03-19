// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::sql_types::{BigInt, Text};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{
    PocAnalysisResultRow, PocBadgeRow, PocConfigRow, PocDisputeRow, PocRevenueRedirectionRow,
};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

pub(crate) async fn get_poc_analysis_for_post(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocAnalysisResultRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT post_id, similarity_detected, highest_similarity_score, media_type,
               oracle_address, original_creator, analysis_timestamp
        FROM (
            SELECT DISTINCT ON (post_id) *
            FROM poc_analysis_results
            WHERE post_id = $1
            ORDER BY post_id, time DESC
        ) sub
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<PocAnalysisResultRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_poc_badges_for_post(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocBadgeRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT badge_id, post_id, media_type, issued_by, issued_at, revoked
        FROM (
            SELECT DISTINCT ON (badge_id) *
            FROM poc_badges
            WHERE post_id = $1
            ORDER BY badge_id, time DESC
        ) sub
        WHERE revoked = false
        ORDER BY issued_at DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocBadgeRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_post_revenue_redirections(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocRevenueRedirectionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT redirection_id, accused_post_id, original_post_id, redirect_percentage,
               similarity_score, created_at, removed
        FROM (
            SELECT DISTINCT ON (redirection_id) *
            FROM poc_revenue_redirections
            WHERE accused_post_id = $1 OR original_post_id = $1
            ORDER BY redirection_id, time DESC
        ) sub
        WHERE removed = false
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocRevenueRedirectionRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_poc_disputes_for_post(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocDisputeRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT dispute_id, post_id, disputer, dispute_type, evidence, status,
               resolution, stake_amount, submitted_at, resolved_at
        FROM (
            SELECT DISTINCT ON (dispute_id) *
            FROM poc_disputes
            WHERE post_id = $1
            ORDER BY dispute_id, time DESC
        ) sub
        ORDER BY submitted_at DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocDisputeRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_poc_configuration(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT image_threshold, video_threshold, audio_threshold,
               revenue_redirect_percentage, dispute_cost, oracle_address, updated_at
        FROM poc_configuration
        ORDER BY updated_at DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<PocConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}
