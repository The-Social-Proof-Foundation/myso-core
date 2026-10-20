// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Jsonb, Nullable, SmallInt, Text};
use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::{
    PendingSpotPostRow, SpotBetRow, SpotConfigInfo, SpotCreatorStatsResponse,
    SpotPendingCreatorPayoutRow, SpotRecordResponse, SpotRouteResponse, SpotTransferRow,
};

pub(crate) async fn get_spot_record(
    db: &Db,
    post_id: &str,
) -> Result<Option<SpotRecordResponse>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT post_id, status, outcome, betting_options, option_escrow, resolution_window_ms,
               max_resolution_window_ms, created_at_ms, last_resolution_at_ms, record_object_id,
               active_proposal_id, oracle_proposed_outcome, proposed_outcome, dao_escalated_at_ms
        FROM spot_records
        WHERE post_id = $1
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        post_id: String,
        #[diesel(sql_type = SmallInt)]
        status: i16,
        #[diesel(sql_type = Nullable<SmallInt>)]
        outcome: Option<i16>,
        #[diesel(sql_type = Nullable<Jsonb>)]
        betting_options: Option<serde_json::Value>,
        #[diesel(sql_type = Nullable<Jsonb>)]
        option_escrow: Option<serde_json::Value>,
        #[diesel(sql_type = Nullable<BigInt>)]
        resolution_window_ms: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        max_resolution_window_ms: Option<i64>,
        #[diesel(sql_type = BigInt)]
        created_at_ms: i64,
        #[diesel(sql_type = Nullable<BigInt>)]
        last_resolution_at_ms: Option<i64>,
        #[diesel(sql_type = Nullable<Text>)]
        record_object_id: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        active_proposal_id: Option<String>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        oracle_proposed_outcome: Option<i16>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        proposed_outcome: Option<i16>,
        #[diesel(sql_type = Nullable<BigInt>)]
        dao_escalated_at_ms: Option<i64>,
    }
    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<Row>(&mut conn)
        .await
        .optional()?;
    Ok(result.map(|r| {
        let betting_options: Vec<String> = r
            .betting_options
            .and_then(|v| serde_json::from_value::<Vec<String>>(v).ok())
            .unwrap_or_default();
        let option_escrow: std::collections::HashMap<String, i64> = r
            .option_escrow
            .and_then(|v| serde_json::from_value::<std::collections::HashMap<String, i64>>(v).ok())
            .unwrap_or_default();
        SpotRecordResponse {
            post_id: r.post_id,
            status: r.status,
            outcome: r.outcome,
            betting_options,
            option_escrow,
            resolution_window_ms: r.resolution_window_ms,
            max_resolution_window_ms: r.max_resolution_window_ms,
            created_at_ms: r.created_at_ms,
            last_resolution_at_ms: r.last_resolution_at_ms,
            record_object_id: r.record_object_id,
            active_proposal_id: r.active_proposal_id,
            oracle_proposed_outcome: r.oracle_proposed_outcome,
            proposed_outcome: r.proposed_outcome,
            dao_escalated_at_ms: r.dao_escalated_at_ms,
        }
    }))
}

pub(crate) async fn list_contested_spot_records(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<SpotRecordResponse>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT post_id, status, outcome, betting_options, option_escrow, resolution_window_ms,
               max_resolution_window_ms, created_at_ms, last_resolution_at_ms, record_object_id,
               active_proposal_id, oracle_proposed_outcome, proposed_outcome, dao_escalated_at_ms
        FROM spot_records
        WHERE status = 2
        ORDER BY dao_escalated_at_ms DESC NULLS LAST, updated_at DESC
        LIMIT $1 OFFSET $2
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        post_id: String,
        #[diesel(sql_type = SmallInt)]
        status: i16,
        #[diesel(sql_type = Nullable<SmallInt>)]
        outcome: Option<i16>,
        #[diesel(sql_type = Nullable<Jsonb>)]
        betting_options: Option<serde_json::Value>,
        #[diesel(sql_type = Nullable<Jsonb>)]
        option_escrow: Option<serde_json::Value>,
        #[diesel(sql_type = Nullable<BigInt>)]
        resolution_window_ms: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        max_resolution_window_ms: Option<i64>,
        #[diesel(sql_type = BigInt)]
        created_at_ms: i64,
        #[diesel(sql_type = Nullable<BigInt>)]
        last_resolution_at_ms: Option<i64>,
        #[diesel(sql_type = Nullable<Text>)]
        record_object_id: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        active_proposal_id: Option<String>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        oracle_proposed_outcome: Option<i16>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        proposed_outcome: Option<i16>,
        #[diesel(sql_type = Nullable<BigInt>)]
        dao_escalated_at_ms: Option<i64>,
    }
    let rows = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(&mut conn)
        .await?;
    Ok(rows
        .into_iter()
        .map(|r| {
            let betting_options: Vec<String> = r
                .betting_options
                .and_then(|v| serde_json::from_value::<Vec<String>>(v).ok())
                .unwrap_or_default();
            let option_escrow: std::collections::HashMap<String, i64> = r
                .option_escrow
                .and_then(|v| {
                    serde_json::from_value::<std::collections::HashMap<String, i64>>(v).ok()
                })
                .unwrap_or_default();
            SpotRecordResponse {
                post_id: r.post_id,
                status: r.status,
                outcome: r.outcome,
                betting_options,
                option_escrow,
                resolution_window_ms: r.resolution_window_ms,
                max_resolution_window_ms: r.max_resolution_window_ms,
                created_at_ms: r.created_at_ms,
                last_resolution_at_ms: r.last_resolution_at_ms,
                record_object_id: r.record_object_id,
                active_proposal_id: r.active_proposal_id,
                oracle_proposed_outcome: r.oracle_proposed_outcome,
                proposed_outcome: r.proposed_outcome,
                dao_escalated_at_ms: r.dao_escalated_at_ms,
            }
        })
        .collect())
}

pub(crate) async fn list_spot_bets(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SpotBetRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT post_id, user_address, option_id, escrow_amount, amm_amount, timestamp_ms
        FROM spot_bets
        WHERE post_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SpotBetRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_spot_payouts(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SpotTransferRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT user_address, amount, timestamp_ms
        FROM spot_payouts
        WHERE post_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SpotTransferRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_spot_refunds(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SpotTransferRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT user_address, amount, timestamp_ms
        FROM spot_refunds
        WHERE post_id = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SpotTransferRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_spot_configuration(db: &Db) -> Result<Option<SpotConfigInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT updated_by, truth_enabled, confidence_threshold_bps, resolution_window_ms,
               max_resolution_window_ms, payout_delay_ms, platform_fee_bps, ecosystem_fee_bps,
               COALESCE(creator_fee_bps, 100) AS creator_fee_bps,
               COALESCE(creator_claim_window_ms, 2592000000) AS creator_claim_window_ms,
               COALESCE(expired_creator_ecosystem_bps, 10000) AS expired_creator_ecosystem_bps,
               min_betting_options, max_betting_options, min_reasoning_length, max_reasoning_length,
               max_evidence_urls, oracle_address, max_single_bet, max_bets_per_record,
               max_claim_per_post, spot_governance_registry_id, version, updated_at, time,
               transaction_id
        FROM spot_config
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .get_result::<SpotConfigInfo>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

/// Posts awaiting SPoT analysis finalization (`spot_analysis_status = 0`, i.e. pending).
/// SPoT is now always-on, so every non-deleted post is pending until the oracle finalizes it.
/// Consumed by the SPoT oracle's PostPoller via the secret-gated `GET /spot/pending-posts`
/// endpoint. Cursor is the last `created_at` (ms).
pub(crate) async fn list_pending_spot_posts(
    db: &Db,
    limit: i64,
    cursor_ms: Option<i64>,
) -> Result<Vec<PendingSpotPostRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT post_id, owner, content, created_at, post_type
        FROM posts
        WHERE spot_analysis_status = 0
          AND deleted_at IS NULL
          AND ($1::bigint IS NULL OR created_at > $1)
        ORDER BY created_at ASC
        LIMIT $2
    ";
    let results = diesel::sql_query(query)
        .bind::<Nullable<BigInt>, _>(cursor_ms)
        .bind::<BigInt, _>(limit)
        .load::<PendingSpotPostRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_spot_route(
    db: &Db,
    post_id: &str,
) -> Result<Option<SpotRouteResponse>, SocialError> {
    let mut conn = db.connect().await?;
    // Multi-claim: route via spot_post_links (per-claim link rows). Single-claim baseline picks
    // the latest link for the post; opt-in columns on `posts` no longer exist.
    let query = "
        WITH post_row AS (
            SELECT post_id, spot_analysis_status
            FROM posts
            WHERE post_id = $1 AND deleted_at IS NULL
            ORDER BY created_at DESC
            LIMIT 1
        ),
        link_row AS (
            SELECT spl.claim_object_id, spl.market_object_id, spl.link_kind
            FROM spot_post_links spl
            WHERE spl.post_id = $1
            ORDER BY spl.created_at DESC
            LIMIT 1
        ),
        claim_id AS (
            SELECT (SELECT claim_object_id FROM link_row) AS claim_object_id
        ),
        open_market AS (
            SELECT sm.market_object_id
            FROM spot_markets sm
            JOIN claim_id c ON c.claim_object_id = sm.claim_object_id
            WHERE sm.status = 1
            ORDER BY sm.created_at_ms DESC NULLS LAST
            LIMIT 1
        )
        SELECT
            pr.post_id,
            (SELECT claim_object_id FROM claim_id) AS claim_object_id,
            COALESCE(
                (SELECT market_object_id FROM link_row WHERE market_object_id IS NOT NULL),
                (SELECT market_object_id FROM open_market)
            ) AS target_market_id,
            (SELECT link_kind FROM link_row) AS link_kind,
            CASE
                WHEN COALESCE(
                    (SELECT market_object_id FROM link_row WHERE market_object_id IS NOT NULL),
                    (SELECT market_object_id FROM open_market)
                ) IS NOT NULL THEN 'open_market'
                WHEN (SELECT claim_object_id FROM claim_id) IS NOT NULL THEN 'claim_without_open_market'
                WHEN pr.spot_analysis_status = 0 THEN 'pending'
                ELSE 'no_actionable'
            END AS routing_reason
        FROM post_row pr
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<SpotRouteResponse>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn list_pending_creator_payouts(
    db: &Db,
    creator: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SpotPendingCreatorPayoutRow>, SocialError> {
    let mut conn = db.connect().await?;
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
        .load::<SpotPendingCreatorPayoutRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_spot_creator_stats(
    db: &Db,
    creator: &str,
) -> Result<SpotCreatorStatsResponse, SocialError> {
    let mut conn = db.connect().await?;
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
        .get_result::<SpotCreatorStatsResponse>(&mut conn)
        .await?;
    Ok(result)
}

pub(crate) async fn list_expired_creator_payouts(
    db: &Db,
    market_object_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SpotPendingCreatorPayoutRow>, SocialError> {
    let mut conn = db.connect().await?;
    let now_ms = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0);
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
        .load::<SpotPendingCreatorPayoutRow>(&mut conn)
        .await?;
    Ok(results)
}
