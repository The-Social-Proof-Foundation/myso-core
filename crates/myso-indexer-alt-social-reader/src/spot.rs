// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Jsonb, Nullable, SmallInt, Text, Timestamptz};
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
    pub truth_enabled: bool,
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
    #[diesel(sql_type = BigInt)]
    pub creator_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub creator_claim_window_ms: i64,
    #[diesel(sql_type = BigInt)]
    pub expired_creator_ecosystem_bps: i64,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct SpotClaimRow {
    #[diesel(sql_type = Text)]
    pub claim_object_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub semantic_claim_hash: Option<String>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub created_at_ms: Option<i64>,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct SpotMarketRow {
    #[diesel(sql_type = Text)]
    pub market_object_id: String,
    #[diesel(sql_type = Text)]
    pub claim_object_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub market_key_hash: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub primary_post_id: Option<String>,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub deadline_ms: Option<i64>,
    #[diesel(sql_type = Jsonb)]
    pub betting_options: serde_json::Value,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub creator_fee_total: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub winner_pool: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub resolution_timestamp_ms: Option<i64>,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub created_at_ms: Option<i64>,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct SpotRouteRow {
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub claim_object_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub target_market_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub link_kind: Option<String>,
    #[diesel(sql_type = Text)]
    pub routing_reason: String,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct SpotPendingCreatorPayoutRow {
    #[diesel(sql_type = BigInt)]
    pub payout_id: i64,
    #[diesel(sql_type = Text)]
    pub market_object_id: String,
    #[diesel(sql_type = Text)]
    pub creator: String,
    #[diesel(sql_type = Text)]
    pub referrer_post_id: String,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = BigInt)]
    pub expires_at_ms: i64,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct SpotClaimEarningsRow {
    #[diesel(sql_type = Text)]
    pub claim_object_id: String,
    #[diesel(sql_type = BigInt)]
    pub total_amount: i64,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct SpotPostEarningsRow {
    #[diesel(sql_type = Text)]
    pub referrer_post_id: String,
    #[diesel(sql_type = BigInt)]
    pub total_amount: i64,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct SpotMarketEarningsRow {
    #[diesel(sql_type = Text)]
    pub market_object_id: String,
    #[diesel(sql_type = BigInt)]
    pub total_amount: i64,
}

#[derive(Debug, Clone, QueryableByName)]
pub struct SpotCreatorStatsRow {
    #[diesel(sql_type = Text)]
    pub creator: String,
    #[diesel(sql_type = BigInt)]
    pub lifetime_earnings: i64,
    #[diesel(sql_type = BigInt)]
    pub earnings_last_30d: i64,
    #[diesel(sql_type = BigInt)]
    pub pending_earnings: i64,
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
               dao_escalated_at_ms, claim_object_id, market_object_id,
               primary_post_id, market_key_hash, creator_fee_total
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
        SELECT updated_by, truth_enabled, confidence_threshold_bps, resolution_window_ms,
               max_resolution_window_ms, payout_delay_ms, platform_fee_bps, ecosystem_fee_bps,
               min_betting_options, max_betting_options, min_reasoning_length, max_reasoning_length,
               max_evidence_urls, oracle_address, max_single_bet, version, updated_at, time,
               transaction_id, creator_fee_bps, creator_claim_window_ms, expired_creator_ecosystem_bps
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
               dao_escalated_at_ms, claim_object_id, market_object_id,
               primary_post_id, market_key_hash, creator_fee_total
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
               dao_escalated_at_ms, claim_object_id, market_object_id,
               primary_post_id, market_key_hash, creator_fee_total
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
               dao_escalated_at_ms, claim_object_id, market_object_id,
               primary_post_id, market_key_hash, creator_fee_total
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

pub(crate) async fn get_spot_route(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SpotRouteRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        WITH post_row AS (
            SELECT post_id, enable_spot, spot_id, spot_claim_id
            FROM posts
            WHERE post_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT 1
        ),
        link_row AS (
            SELECT spl.claim_object_id, spl.market_object_id, spl.link_kind
            FROM spot_post_links spl
            WHERE spl.post_id = $1
        ),
        claim_id AS (
            SELECT COALESCE(
                (SELECT spot_claim_id FROM post_row),
                (SELECT claim_object_id FROM link_row)
            ) AS claim_object_id
        ),
        open_market AS (
            SELECT sm.market_object_id
            FROM spot_markets sm
            JOIN claim_id c ON c.claim_object_id = sm.claim_object_id
            WHERE sm.status = 1
            ORDER BY sm.created_at_ms DESC NULLS LAST
            LIMIT 1
        ),
        legacy_record AS (
            SELECT sr.record_object_id AS market_object_id
            FROM spot_records sr
            WHERE sr.post_id = $1 AND sr.status = 1
            ORDER BY sr.created_at_ms DESC
            LIMIT 1
        )
        SELECT
            pr.post_id,
            c.claim_object_id,
            COALESCE(
                pr.spot_id,
                (SELECT market_object_id FROM link_row WHERE market_object_id IS NOT NULL),
                (SELECT market_object_id FROM open_market),
                (SELECT market_object_id FROM legacy_record)
            ) AS target_market_id,
            (SELECT link_kind FROM link_row) AS link_kind,
            CASE
                WHEN COALESCE(
                    pr.spot_id,
                    (SELECT market_object_id FROM link_row WHERE market_object_id IS NOT NULL),
                    (SELECT market_object_id FROM open_market)
                ) IS NOT NULL THEN 'open_market'
                WHEN c.claim_object_id IS NOT NULL THEN 'claim_without_open_market'
                WHEN (SELECT market_object_id FROM legacy_record) IS NOT NULL THEN 'legacy_record'
                WHEN pr.enable_spot = true THEN 'pending_spot'
                ELSE 'not_spot_enabled'
            END AS routing_reason
        FROM post_row pr
        LEFT JOIN claim_id c ON true
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<SpotRouteRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_spot_claim_by_object_id(
    conn: &mut Connection<'_>,
    claim_object_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SpotClaimRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT claim_object_id, semantic_claim_hash, created_at_ms
        FROM spot_claims
        WHERE claim_object_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(claim_object_id)
        .get_result::<SpotClaimRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_spot_market_by_object_id(
    conn: &mut Connection<'_>,
    market_object_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SpotMarketRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT market_object_id, claim_object_id, market_key_hash, primary_post_id, status,
               resolution_at_ms AS deadline_ms, betting_options, creator_fee_total,
               NULL::bigint AS winner_pool, resolution_timestamp_ms, created_at_ms
        FROM spot_markets
        WHERE market_object_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(market_object_id)
        .get_result::<SpotMarketRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_pending_creator_payouts(
    conn: &mut Connection<'_>,
    creator: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SpotPendingCreatorPayoutRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT payout_id, market_object_id, creator_address AS creator, referrer_post_id, amount, expires_at_ms
        FROM spot_creator_payouts
        WHERE creator_address = $1 AND status = 'accrued'
        ORDER BY expires_at_ms ASC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(creator)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SpotPendingCreatorPayoutRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_expired_unclaimed_creator_payouts(
    conn: &mut Connection<'_>,
    market_object_id: &str,
    now_ms: i64,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SpotPendingCreatorPayoutRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT payout_id, market_object_id, creator_address AS creator, referrer_post_id, amount, expires_at_ms
        FROM spot_creator_payouts
        WHERE market_object_id = $1
          AND status = 'accrued'
          AND expires_at_ms <= $2
        ORDER BY expires_at_ms ASC
        LIMIT $3 OFFSET $4
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(market_object_id)
        .bind::<BigInt, _>(now_ms)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SpotPendingCreatorPayoutRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_spot_creator_stats(
    conn: &mut Connection<'_>,
    creator: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<SpotCreatorStatsRow> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT
            $1::text AS creator,
            COALESCE((
                SELECT SUM(amount)::bigint FROM spot_creator_payouts
                WHERE creator_address = $1 AND status = 'claimed'
            ), 0) AS lifetime_earnings,
            COALESCE((
                SELECT SUM(amount)::bigint FROM spot_creator_payouts
                WHERE creator_address = $1 AND status = 'claimed'
                  AND claimed_at_ms >= (EXTRACT(EPOCH FROM NOW()) * 1000)::bigint - 2592000000
            ), 0) AS earnings_last_30d,
            COALESCE((
                SELECT SUM(amount)::bigint FROM spot_creator_payouts
                WHERE creator_address = $1 AND status = 'accrued'
            ), 0) AS pending_earnings
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(creator)
        .get_result::<SpotCreatorStatsRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_spot_creator_top_claims(
    conn: &mut Connection<'_>,
    creator: &str,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SpotClaimEarningsRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT sm.claim_object_id, SUM(scp.amount)::bigint AS total_amount
        FROM spot_creator_payouts scp
        JOIN spot_markets sm ON sm.market_object_id = scp.market_object_id
        WHERE scp.creator_address = $1
        GROUP BY sm.claim_object_id
        ORDER BY total_amount DESC
        LIMIT $2
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(creator)
        .bind::<BigInt, _>(limit)
        .load::<SpotClaimEarningsRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_spot_creator_earnings_by_post(
    conn: &mut Connection<'_>,
    creator: &str,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SpotPostEarningsRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT referrer_post_id, SUM(amount)::bigint AS total_amount
        FROM spot_creator_payouts
        WHERE creator_address = $1
        GROUP BY referrer_post_id
        ORDER BY total_amount DESC
        LIMIT $2
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(creator)
        .bind::<BigInt, _>(limit)
        .load::<SpotPostEarningsRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_spot_creator_earnings_by_market(
    conn: &mut Connection<'_>,
    creator: &str,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SpotMarketEarningsRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT market_object_id, SUM(amount)::bigint AS total_amount
        FROM spot_creator_payouts
        WHERE creator_address = $1
        GROUP BY market_object_id
        ORDER BY total_amount DESC
        LIMIT $2
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(creator)
        .bind::<BigInt, _>(limit)
        .load::<SpotMarketEarningsRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}
