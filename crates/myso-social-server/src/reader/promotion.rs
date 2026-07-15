// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Bool, Text};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::promotion_views;

use crate::error::SocialError;
use crate::reader::types::{
    PromotedPostRow, PromotionHourlyRow, PromotionStatsRow, PromotionTimeSeriesRow,
    PromotionViewRow,
};
use myso_pg_db::Db;

pub(crate) async fn list_promotions(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<PromotedPostRow>, SocialError> {
    let mut conn = db.connect().await?;
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
        .bind::<BigInt, _>(offset)
        .load::<Row>(&mut conn)
        .await?;
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

pub(crate) async fn get_promotion_by_post_id(
    db: &Db,
    post_id: &str,
) -> Result<Option<PromotedPostRow>, SocialError> {
    let mut conn = db.connect().await?;
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
    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<Row>(&mut conn)
        .await
        .optional()?;
    Ok(result.map(|r| PromotedPostRow {
        promotion_id: r.promotion_id,
        post_id: r.post_id,
        owner: r.owner,
        profile_id: r.profile_id,
        payment_per_view: r.payment_per_view,
        total_budget: r.total_budget,
        remaining_budget: r.remaining_budget,
        active: r.active,
        created_at: r.created_at,
    }))
}

pub(crate) async fn get_promotion_views(
    db: &Db,
    promotion_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PromotionViewRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = promotion_views::table
        .filter(promotion_views::promotion_id.eq(promotion_id))
        .order_by(promotion_views::timestamp.desc())
        .limit(limit)
        .offset(offset)
        .select((
            promotion_views::post_id,
            promotion_views::promotion_id,
            promotion_views::viewer,
            promotion_views::payment_amount,
            promotion_views::platform_fee,
            promotion_views::ecosystem_fee,
            promotion_views::recipient_amount,
            promotion_views::view_duration,
            promotion_views::platform_id,
            promotion_views::timestamp,
        ))
        .load::<(String, String, String, i64, i64, i64, i64, i64, String, i64)>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(
                post_id,
                promotion_id,
                viewer,
                payment_amount,
                platform_fee,
                ecosystem_fee,
                recipient_amount,
                view_duration,
                platform_id,
                timestamp,
            )| PromotionViewRow {
                post_id,
                promotion_id,
                viewer,
                payment_amount,
                platform_fee,
                ecosystem_fee,
                recipient_amount,
                view_duration,
                platform_id,
                timestamp,
            },
        )
        .collect())
}

pub(crate) async fn get_promotion_stats(
    db: &Db,
    promotion_id: &str,
) -> Result<Option<PromotionStatsRow>, SocialError> {
    let mut conn = db.connect().await?;
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
        .get_result::<ViewsRow>(&mut conn)
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
        .get_result::<BudgetRow>(&mut conn)
        .await
        .optional()?
        .map(|r| r.val);
    Ok(Some(PromotionStatsRow {
        total_views: views.cnt,
        total_spent: views.spent,
        remaining_budget: remaining.unwrap_or(0),
    }))
}

pub(crate) async fn get_promotion_time_series(
    db: &Db,
    promotion_id: &str,
    limit: i64,
) -> Result<Vec<PromotionTimeSeriesRow>, SocialError> {
    let mut conn = db.connect().await?;
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
        .load::<PromotionTimeSeriesRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_promotion_hourly(
    db: &Db,
    promotion_id: &str,
    limit: i64,
) -> Result<Vec<PromotionHourlyRow>, SocialError> {
    let mut conn = db.connect().await?;
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
        .load::<PromotionHourlyRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_top_performing_promotions(
    db: &Db,
    limit: i64,
) -> Result<Vec<PromotedPostRow>, SocialError> {
    let mut conn = db.connect().await?;
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
        .load::<Row>(&mut conn)
        .await?;
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
    db: &Db,
    limit: i64,
) -> Result<Vec<PromotionTimeSeriesRow>, SocialError> {
    let mut conn = db.connect().await?;
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
        .load::<PromotionTimeSeriesRow>(&mut conn)
        .await?;
    Ok(results)
}
