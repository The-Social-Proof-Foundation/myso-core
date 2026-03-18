// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Text};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::PromotedPostRow;
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

pub(crate) async fn get_promotion(
    conn: &mut Connection<'_>,
    promotion_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PromotedPostRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT promotion_id, post_id, owner, profile_id, payment_per_view, total_budget,
               remaining_budget, active, created_at
        FROM (
            SELECT DISTINCT ON (promotion_id) *
            FROM promoted_posts
            WHERE promotion_id = $1
            ORDER BY promotion_id, time DESC
        ) sub
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(promotion_id)
        .get_result::<PromotedPostRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_promotion_by_post_id(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PromotedPostRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT promotion_id, post_id, owner, profile_id, payment_per_view, total_budget,
               remaining_budget, active, created_at
        FROM (
            SELECT DISTINCT ON (promotion_id) *
            FROM promoted_posts
            WHERE post_id = $1
            ORDER BY promotion_id, time DESC
        ) sub
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<PromotedPostRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_promotion_views_count(
    conn: &mut Connection<'_>,
    promotion_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<i64> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        cnt: i64,
    }

    let query = "SELECT COUNT(*)::bigint AS cnt FROM promotion_views WHERE promotion_id = $1";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(promotion_id)
        .get_result::<CountRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(result.cnt)
}

pub(crate) async fn list_promoted_posts(
    conn: &mut Connection<'_>,
    platform_id: Option<&str>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PromotedPostRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let results = if let Some(pid) = platform_id {
        let query = "
            SELECT pp.promotion_id, pp.post_id, pp.owner, pp.profile_id, pp.payment_per_view,
                   pp.total_budget, pp.remaining_budget, pp.active, pp.created_at
            FROM (
                SELECT DISTINCT ON (promotion_id) promotion_id, post_id, owner, profile_id,
                       payment_per_view, total_budget, remaining_budget, active, created_at
                FROM promoted_posts
                WHERE promotion_id IN (
                    SELECT DISTINCT promotion_id FROM promotion_views WHERE platform_id = $1
                )
                ORDER BY promotion_id, time DESC
            ) pp
            ORDER BY pp.created_at DESC
            LIMIT $2 OFFSET $3
        ";

        diesel::sql_query(query)
            .bind::<Text, _>(pid)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<PromotedPostRow>(conn)
            .await?
    } else {
        let query = "
            SELECT promotion_id, post_id, owner, profile_id, payment_per_view, total_budget,
                   remaining_budget, active, created_at
            FROM (
                SELECT DISTINCT ON (promotion_id) *
                FROM promoted_posts
                ORDER BY promotion_id, time DESC
            ) sub
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
        ";

        diesel::sql_query(query)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<PromotedPostRow>(conn)
            .await?
    };

    metrics.requests_succeeded.inc();
    Ok(results)
}
