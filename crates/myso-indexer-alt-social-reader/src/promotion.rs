// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Text};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{
    PromotedPostRow, PromotionHourlyRow, PromotionStatsRow, PromotionTimeSeriesRow,
    PromotionViewRow,
};
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

pub(crate) async fn get_promotion_views(
    conn: &mut Connection<'_>,
    promotion_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PromotionViewRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT post_id, promotion_id, viewer, payment_amount, view_duration, platform_id, timestamp
        FROM promotion_views
        WHERE promotion_id = $1
        ORDER BY timestamp DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(promotion_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PromotionViewRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_promotion_stats(
    conn: &mut Connection<'_>,
    promotion_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PromotionStatsRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let views_query = "
        SELECT COUNT(*)::bigint as cnt, COALESCE(SUM(payment_amount), 0)::bigint as spent
        FROM promotion_views WHERE promotion_id = $1
    ";
    #[derive(QueryableByName)]
    struct ViewsRow {
        #[diesel(sql_type = BigInt)]
        cnt: i64,
        #[diesel(sql_type = BigInt)]
        spent: i64,
    }
    let views = diesel::sql_query(views_query)
        .bind::<Text, _>(promotion_id)
        .get_result::<ViewsRow>(conn)
        .await?;

    let budget_query = "
        SELECT remaining_budget as val FROM (
            SELECT DISTINCT ON (promotion_id) remaining_budget
            FROM promoted_posts WHERE promotion_id = $1
            ORDER BY promotion_id, time DESC
        ) sub
    ";
    #[derive(QueryableByName)]
    struct BudgetRow {
        #[diesel(sql_type = BigInt)]
        val: i64,
    }
    let remaining: Option<i64> = diesel::sql_query(budget_query)
        .bind::<Text, _>(promotion_id)
        .get_result::<BudgetRow>(conn)
        .await
        .optional()?
        .map(|r| r.val);

    metrics.requests_succeeded.inc();
    Ok(Some(PromotionStatsRow {
        total_views: views.cnt,
        total_spent: views.spent,
        remaining_budget: remaining.unwrap_or(0),
    }))
}

pub(crate) async fn get_promotion_time_series(
    conn: &mut Connection<'_>,
    promotion_id: &str,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PromotionTimeSeriesRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT DATE(to_timestamp(timestamp/1000)) as day,
               COUNT(*)::bigint as views,
               COALESCE(SUM(payment_amount), 0)::bigint as spent
        FROM promotion_views
        WHERE promotion_id = $1
          AND timestamp >= EXTRACT(EPOCH FROM NOW() - INTERVAL '30 days') * 1000
        GROUP BY DATE(to_timestamp(timestamp/1000))
        ORDER BY day ASC
        LIMIT $2
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(promotion_id)
        .bind::<BigInt, _>(limit)
        .load::<PromotionTimeSeriesRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_promotion_hourly(
    conn: &mut Connection<'_>,
    promotion_id: &str,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PromotionHourlyRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT EXTRACT(HOUR FROM to_timestamp(timestamp/1000))::int as hour,
               COUNT(*)::bigint as views,
               COALESCE(SUM(payment_amount), 0)::bigint as spent
        FROM promotion_views
        WHERE promotion_id = $1
          AND timestamp >= EXTRACT(EPOCH FROM NOW() - INTERVAL '7 days') * 1000
        GROUP BY EXTRACT(HOUR FROM to_timestamp(timestamp/1000))
        ORDER BY hour ASC
        LIMIT $2
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(promotion_id)
        .bind::<BigInt, _>(limit)
        .load::<PromotionHourlyRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_top_performing_promotions(
    conn: &mut Connection<'_>,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PromotedPostRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT pp.promotion_id, pp.post_id, pp.owner, pp.profile_id, pp.payment_per_view,
               pp.total_budget, pp.remaining_budget, pp.active, pp.created_at
        FROM (
            SELECT DISTINCT ON (promotion_id) promotion_id, post_id, owner, profile_id,
                   payment_per_view, total_budget, remaining_budget, active, created_at
            FROM promoted_posts
            ORDER BY promotion_id, time DESC
        ) pp
        JOIN (
            SELECT promotion_id, COUNT(*) as view_count
            FROM promotion_views
            GROUP BY promotion_id
        ) pv ON pp.promotion_id = pv.promotion_id
        ORDER BY pv.view_count DESC
        LIMIT $1
    ";

    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        promotion_id: String,
        #[diesel(sql_type = Text)]
        post_id: String,
        #[diesel(sql_type = Text)]
        owner: String,
        #[diesel(sql_type = Text)]
        profile_id: String,
        #[diesel(sql_type = BigInt)]
        payment_per_view: i64,
        #[diesel(sql_type = BigInt)]
        total_budget: i64,
        #[diesel(sql_type = BigInt)]
        remaining_budget: i64,
        #[diesel(sql_type = Bool)]
        active: bool,
        #[diesel(sql_type = BigInt)]
        created_at: i64,
    }

    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .load::<Row>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results
        .into_iter()
        .map(|r| PromotedPostRow {
            promotion_id: r.promotion_id,
            post_id: r.post_id,
            owner: r.owner,
            profile_id: r.profile_id,
            payment_per_view: r.payment_per_view,
            total_budget: r.total_budget,
            remaining_budget: r.remaining_budget,
            active: r.active,
            created_at: r.created_at,
        })
        .collect())
}

pub(crate) async fn get_spending_trends(
    conn: &mut Connection<'_>,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PromotionTimeSeriesRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT DATE(to_timestamp(timestamp/1000)) as day,
               COUNT(*)::bigint as views,
               COALESCE(SUM(payment_amount), 0)::bigint as spent
        FROM promotion_views
        WHERE timestamp >= EXTRACT(EPOCH FROM NOW() - INTERVAL '30 days') * 1000
        GROUP BY DATE(to_timestamp(timestamp/1000))
        ORDER BY day ASC
        LIMIT $1
    ";

    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .load::<PromotionTimeSeriesRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}
