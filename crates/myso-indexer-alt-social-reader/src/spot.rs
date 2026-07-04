// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Text, Timestamptz};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{
    SpotBetRow, SpotBetWithdrawalRow, SpotPayoutRow, SpotRecordRow, SpotRefundRow,
    SpotResolutionRow,
};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName)]
pub struct SpotConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    pub enable_flag: bool,
    #[diesel(sql_type = BigInt)]
    pub confidence_threshold_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub resolution_window_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub max_resolution_window_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub payout_delay_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub platform_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub ecosystem_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub min_betting_options: i64,
    #[diesel(sql_type = BigInt)]
    pub max_betting_options: i64,
    #[diesel(sql_type = BigInt)]
    pub min_reasoning_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_reasoning_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_evidence_urls: i64,
    #[diesel(sql_type = Text)]
    pub oracle_address: String,
    #[diesel(sql_type = BigInt)]
    pub max_single_bet: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

pub(crate) async fn get_spot_record(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SpotRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, post_id, status, outcome, betting_options, option_escrow,
               created_at_ms, resolution_window_ms, max_resolution_window_ms,
               last_resolution_at_ms, transaction_id, record_object_id,
               active_proposal_id, oracle_proposed_outcome, proposed_outcome,
               dao_escalated_at_ms
        FROM spot_records
        WHERE post_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<SpotRecordRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_spot_bets(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SpotBetRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT sb.id, sb.post_id, sb.user_address, sb.option_id, sb.escrow_amount, sb.amm_amount,
               sb.timestamp_ms, sb.transaction_id,
               CASE WHEN sr.betting_options IS NOT NULL AND jsonb_array_length(sr.betting_options) > sb.option_id
                    THEN jsonb_array_element_text(sr.betting_options, sb.option_id)
                    ELSE NULL
               END AS option_label
        FROM spot_bets sb
        LEFT JOIN spot_records sr ON sr.post_id = sb.post_id
        WHERE sb.post_id = $1
        ORDER BY sb.time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SpotBetRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_spot_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SpotConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, enable_flag, confidence_threshold_bps, resolution_window_ms,
               max_resolution_window_ms, payout_delay_ms, platform_fee_bps, ecosystem_fee_bps,
               min_betting_options, max_betting_options, min_reasoning_length, max_reasoning_length,
               max_evidence_urls, oracle_address, max_single_bet, version, updated_at, time,
               transaction_id
        FROM spot_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<SpotConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_spot_payouts(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SpotPayoutRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, post_id, user_address, amount, timestamp_ms, transaction_id
        FROM spot_payouts
        WHERE post_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SpotPayoutRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_spot_refunds(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SpotRefundRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, post_id, user_address, amount, timestamp_ms, transaction_id
        FROM spot_refunds
        WHERE post_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SpotRefundRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_spot_resolution(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SpotResolutionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, post_id, outcome, total_escrow, fee_taken, resolved_at_ms,
               transaction_id, reasoning, evidence_urls
        FROM spot_resolutions
        WHERE post_id = $1
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<SpotResolutionRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_spot_bet_withdrawals(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SpotBetWithdrawalRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, post_id, user_address, option_id, amount, fee_taken, timestamp_ms,
               transaction_id
        FROM spot_bet_withdrawals
        WHERE post_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SpotBetWithdrawalRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_spot_record_by_active_proposal_id(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SpotRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, post_id, status, outcome, betting_options, option_escrow,
               created_at_ms, resolution_window_ms, max_resolution_window_ms,
               last_resolution_at_ms, transaction_id, record_object_id,
               active_proposal_id, oracle_proposed_outcome, proposed_outcome,
               dao_escalated_at_ms
        FROM spot_records
        WHERE active_proposal_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .get_result::<SpotRecordRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_spot_record_by_object_id(
    conn: &mut Connection<'_>,
    record_object_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SpotRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, post_id, status, outcome, betting_options, option_escrow,
               created_at_ms, resolution_window_ms, max_resolution_window_ms,
               last_resolution_at_ms, transaction_id, record_object_id,
               active_proposal_id, oracle_proposed_outcome, proposed_outcome,
               dao_escalated_at_ms
        FROM spot_records
        WHERE record_object_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(record_object_id)
        .get_result::<SpotRecordRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_contested_spot_records(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SpotRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, post_id, status, outcome, betting_options, option_escrow,
               created_at_ms, resolution_window_ms, max_resolution_window_ms,
               last_resolution_at_ms, transaction_id, record_object_id,
               active_proposal_id, oracle_proposed_outcome, proposed_outcome,
               dao_escalated_at_ms
        FROM spot_records
        WHERE status = 2
        ORDER BY dao_escalated_at_ms DESC NULLS LAST, updated_at DESC
        LIMIT $1 OFFSET $2
    ";

    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SpotRecordRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}
