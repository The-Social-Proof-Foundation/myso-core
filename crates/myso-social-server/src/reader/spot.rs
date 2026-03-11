// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Jsonb, Nullable, SmallInt, Text};
use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::{SpotBetRow, SpotConfigInfo, SpotRecordResponse, SpotTransferRow};

pub(crate) async fn get_spot_record(
    db: &Db,
    post_id: &str,
) -> Result<Option<SpotRecordResponse>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT post_id, status, outcome, betting_options, option_escrow, resolution_window_epochs,
               max_resolution_window_epochs, created_epoch, last_resolution_epoch
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
        resolution_window_epochs: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        max_resolution_window_epochs: Option<i64>,
        #[diesel(sql_type = BigInt)]
        created_epoch: i64,
        #[diesel(sql_type = Nullable<BigInt>)]
        last_resolution_epoch: Option<i64>,
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
            resolution_window_epochs: r.resolution_window_epochs,
            max_resolution_window_epochs: r.max_resolution_window_epochs,
            created_epoch: r.created_epoch,
            last_resolution_epoch: r.last_resolution_epoch,
        }
    }))
}

pub(crate) async fn list_spot_bets(
    db: &Db,
    post_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SpotBetRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT post_id, user_address, option_id, escrow_amount, amm_amount, timestamp_epoch
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
        SELECT user_address, amount, timestamp_epoch
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
        SELECT user_address, amount, timestamp_epoch
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
        SELECT updated_by, enable_flag, confidence_threshold_bps, resolution_window_epochs,
               max_resolution_window_epochs, payout_delay_ms, fee_bps, fee_split_bps_platform,
               oracle_address, max_single_bet, version, timestamp_ms, time, transaction_id
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
