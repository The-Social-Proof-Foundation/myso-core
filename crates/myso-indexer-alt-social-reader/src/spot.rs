// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::sql_types::{BigInt, Text};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{SpotBetRow, SpotRecordRow};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

pub(crate) async fn get_spot_record(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SpotRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, post_id, status, outcome, betting_options, option_escrow,
               created_epoch, resolution_window_epochs, max_resolution_window_epochs,
               last_resolution_epoch
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
               sb.timestamp_epoch, sb.transaction_id,
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
