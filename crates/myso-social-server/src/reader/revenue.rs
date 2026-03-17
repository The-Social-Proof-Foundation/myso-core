// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::dsl::sum;
use diesel::expression_methods::ExpressionMethods;
use diesel::sql_types::{BigInt, Date, Double, Nullable, Text, Timestamp, Timestamptz};
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    REVENUE_SOURCE_MYDATA, REVENUE_SOURCE_SPT, REVENUE_SOURCE_SUBSCRIPTION, REVENUE_SOURCE_TIPS,
    UnifiedRevenue,
};
use myso_indexer_alt_social_schema::schema::{ecosystem_treasury, unified_revenue};
use myso_pg_db::Db;

use crate::error::SocialError;

fn pct(a: i64, b: i64) -> f64 {
    if b == 0 {
        0.0
    } else {
        (a as f64 / b as f64) * 100.0
    }
}

async fn get_revenue_leaderboard_internal(
    db: &Db,
    limit: i64,
    min_revenue: i64,
    revenue_source: Option<&str>,
) -> Result<Vec<serde_json::Value>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = "
        SELECT creator_address, total_revenue, total_subscription_revenue,
               total_mydata_revenue, total_spt_revenue, total_tips_revenue,
               total_transactions, total_unique_payers,
               ROW_NUMBER() OVER (ORDER BY total_revenue DESC) as rank
        FROM spt_creator_revenue_summary
        WHERE total_revenue >= $1
    "
    .to_string();
    if let Some(src) = revenue_source {
        match src {
            x if x == REVENUE_SOURCE_SUBSCRIPTION => {
                query.push_str(" AND total_subscription_revenue > 0")
            }
            x if x == REVENUE_SOURCE_MYDATA => query.push_str(" AND total_mydata_revenue > 0"),
            x if x == REVENUE_SOURCE_SPT => query.push_str(" AND total_spt_revenue > 0"),
            x if x == REVENUE_SOURCE_TIPS => query.push_str(" AND total_tips_revenue > 0"),
            _ => {}
        }
    }
    query.push_str(" ORDER BY total_revenue DESC LIMIT $2");
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        creator_address: String,
        #[diesel(sql_type = BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_subscription_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_mydata_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_spt_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_tips_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_transactions: i64,
        #[diesel(sql_type = BigInt)]
        total_unique_payers: i64,
        #[diesel(sql_type = BigInt)]
        rank: i64,
    }
    let rows: Vec<Row> = diesel::sql_query(query)
        .bind::<BigInt, _>(min_revenue)
        .bind::<BigInt, _>(limit)
        .load(&mut conn)
        .await?;
    Ok(rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "rank": r.rank,
                "creator_address": r.creator_address,
                "total_revenue": r.total_revenue,
                "revenue_breakdown": {
                    "subscription_revenue": r.total_subscription_revenue,
                    "mydata_revenue": r.total_mydata_revenue,
                    "spt_revenue": r.total_spt_revenue,
                    "tips_revenue": r.total_tips_revenue,
                    "posts_revenue": 0
                },
                "growth_rate": serde_json::Value::Null,
                "transaction_count": r.total_transactions,
                "unique_payers": r.total_unique_payers
            })
        })
        .collect())
}

async fn get_revenue_chart_data_internal(
    db: &Db,
    creator_address: Option<&str>,
    hours: i64,
) -> Result<Vec<serde_json::Value>, SocialError> {
    let mut conn = db.connect().await?;
    let start_time = chrono::Utc::now() - chrono::Duration::hours(hours);
    let start_naive = start_time.naive_utc();
    let query = if creator_address.is_some() {
        "SELECT time_bucket('1 hour', time) as bucket, revenue_source, SUM(amount) as total_revenue,
                COUNT(*) as transaction_count, COUNT(DISTINCT creator_address) as unique_creators,
                COUNT(DISTINCT payer_address) as unique_payers
         FROM unified_revenue WHERE time >= $1 AND creator_address = $2
         GROUP BY bucket, revenue_source ORDER BY bucket ASC"
    } else {
        "SELECT time_bucket('1 hour', time) as bucket, revenue_source, SUM(amount) as total_revenue,
                COUNT(*) as transaction_count, COUNT(DISTINCT creator_address) as unique_creators,
                COUNT(DISTINCT payer_address) as unique_payers
         FROM unified_revenue WHERE time >= $1
         GROUP BY bucket, revenue_source ORDER BY bucket ASC"
    };
    #[derive(QueryableByName)]
    struct ChartRow {
        #[diesel(sql_type = Timestamp)]
        bucket: chrono::NaiveDateTime,
        #[diesel(sql_type = Text)]
        revenue_source: String,
        #[diesel(sql_type = BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = BigInt)]
        transaction_count: i64,
        #[diesel(sql_type = BigInt)]
        unique_creators: i64,
        #[diesel(sql_type = BigInt)]
        unique_payers: i64,
    }
    let rows: Vec<ChartRow> = if let Some(addr) = creator_address {
        diesel::sql_query(query)
            .bind::<Timestamp, _>(start_naive)
            .bind::<Text, _>(addr)
            .load(&mut conn)
            .await?
    } else {
        diesel::sql_query(query)
            .bind::<Timestamp, _>(start_naive)
            .load(&mut conn)
            .await?
    };
    Ok(rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "timestamp": r.bucket.and_utc().to_rfc3339(),
                "revenue_source": r.revenue_source,
                "total_revenue": r.total_revenue,
                "transaction_count": r.transaction_count,
                "unique_creators": r.unique_creators,
                "unique_payers": r.unique_payers
            })
        })
        .collect())
}

pub(crate) async fn get_revenue_dashboard(db: &Db) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;
    let dashboard_query = r#"
        SELECT revenue_source, total_revenue_24h, total_transactions_24h, largest_transaction_24h
        FROM revenue_dashboard_24h
        ORDER BY total_revenue_24h DESC
    "#;
    #[derive(QueryableByName)]
    struct DashboardRow {
        #[diesel(sql_type = Text)]
        revenue_source: String,
        #[diesel(sql_type = BigInt)]
        total_revenue_24h: i64,
        #[diesel(sql_type = BigInt)]
        total_transactions_24h: i64,
        #[diesel(sql_type = BigInt)]
        largest_transaction_24h: i64,
    }
    let dashboard_rows: Vec<DashboardRow> =
        diesel::sql_query(dashboard_query).load(&mut conn).await?;
    let total_revenue_24h: i64 = dashboard_rows.iter().map(|r| r.total_revenue_24h).sum();
    let total_transactions_24h: i64 = dashboard_rows
        .iter()
        .map(|r| r.total_transactions_24h)
        .sum();
    let largest_transaction_24h = dashboard_rows
        .iter()
        .map(|r| r.largest_transaction_24h)
        .max()
        .unwrap_or(0);

    let unique_query = r#"
        SELECT COUNT(DISTINCT creator_address) as unique_creators_24h,
               COUNT(DISTINCT payer_address) as unique_payers_24h
        FROM unified_revenue
        WHERE time >= NOW() - INTERVAL '24 hours' AND amount > 0 AND currency = 'MYS'
    "#;
    #[derive(QueryableByName)]
    struct UniqueRow {
        #[diesel(sql_type = BigInt)]
        unique_creators_24h: i64,
        #[diesel(sql_type = BigInt)]
        unique_payers_24h: i64,
    }
    let unique: UniqueRow = diesel::sql_query(unique_query)
        .get_result(&mut conn)
        .await?;

    let revenue_by_source: Vec<serde_json::Value> = dashboard_rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "revenue_source": r.revenue_source,
                "total_revenue": r.total_revenue_24h,
                "transaction_count": r.total_transactions_24h,
                "percentage_of_total": pct(r.total_revenue_24h, total_revenue_24h),
                "growth_rate": serde_json::Value::Null
            })
        })
        .collect();

    let top_creators = get_revenue_leaderboard_internal(db, 10, 0, None).await?;
    let recent_trends = get_revenue_chart_data_internal(db, None, 24).await?;

    Ok(serde_json::json!({
        "total_revenue_24h": total_revenue_24h,
        "total_transactions_24h": total_transactions_24h,
        "unique_creators_24h": unique.unique_creators_24h,
        "unique_payers_24h": unique.unique_payers_24h,
        "largest_transaction_24h": largest_transaction_24h,
        "revenue_by_source": revenue_by_source,
        "top_creators": top_creators,
        "recent_trends": recent_trends
    }))
}

pub(crate) async fn get_revenue_leaderboard(
    db: &Db,
    limit: i64,
    min_revenue: i64,
    revenue_source: Option<&str>,
) -> Result<Vec<serde_json::Value>, SocialError> {
    get_revenue_leaderboard_internal(db, limit, min_revenue, revenue_source).await
}

pub(crate) async fn get_revenue_chart_data(
    db: &Db,
    creator_address: Option<&str>,
    period: &str,
    start_date: chrono::NaiveDateTime,
    end_date: chrono::NaiveDateTime,
    _points: i64,
) -> Result<Vec<serde_json::Value>, SocialError> {
    let time_bucket = match period {
        "hour" => "1 hour",
        "day" => "1 day",
        "week" => "1 week",
        "month" => "1 month",
        _ => "1 day",
    };
    let mut conn = db.connect().await?;
    let (query, has_creator) = if creator_address.is_some() {
        (
            format!(
                "SELECT time_bucket('{}', time) as bucket, revenue_source, SUM(amount) as total_revenue,
                        COUNT(*) as transaction_count, COUNT(DISTINCT creator_address) as unique_creators,
                        COUNT(DISTINCT payer_address) as unique_payers
                 FROM unified_revenue WHERE time BETWEEN $1 AND $2 AND creator_address = $3
                 GROUP BY bucket, revenue_source ORDER BY bucket ASC",
                time_bucket
            ),
            true,
        )
    } else {
        (
            format!(
                "SELECT time_bucket('{}', time) as bucket, revenue_source, SUM(amount) as total_revenue,
                        COUNT(*) as transaction_count, COUNT(DISTINCT creator_address) as unique_creators,
                        COUNT(DISTINCT payer_address) as unique_payers
                 FROM unified_revenue WHERE time BETWEEN $1 AND $2
                 GROUP BY bucket, revenue_source ORDER BY bucket ASC",
                time_bucket
            ),
            false,
        )
    };
    #[derive(QueryableByName)]
    struct ChartRow {
        #[diesel(sql_type = Timestamp)]
        bucket: chrono::NaiveDateTime,
        #[diesel(sql_type = Text)]
        revenue_source: String,
        #[diesel(sql_type = BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = BigInt)]
        transaction_count: i64,
        #[diesel(sql_type = BigInt)]
        unique_creators: i64,
        #[diesel(sql_type = BigInt)]
        unique_payers: i64,
    }
    let rows: Vec<ChartRow> = if has_creator {
        diesel::sql_query(&query)
            .bind::<Timestamp, _>(start_date)
            .bind::<Timestamp, _>(end_date)
            .bind::<Text, _>(creator_address.unwrap())
            .load(&mut conn)
            .await?
    } else {
        diesel::sql_query(&query)
            .bind::<Timestamp, _>(start_date)
            .bind::<Timestamp, _>(end_date)
            .load(&mut conn)
            .await?
    };
    Ok(rows
        .into_iter()
        .map(|r| {
            serde_json::json!({
                "timestamp": r.bucket.and_utc().to_rfc3339(),
                "revenue_source": r.revenue_source,
                "total_revenue": r.total_revenue,
                "transaction_count": r.transaction_count,
                "unique_creators": r.unique_creators,
                "unique_payers": r.unique_payers
            })
        })
        .collect())
}

pub(crate) async fn get_unified_revenue(
    db: &Db,
    creator_address: Option<&str>,
    platform_address: Option<&str>,
    revenue_source: Option<&str>,
    revenue_type: Option<&str>,
    content_id: Option<&str>,
    content_type: Option<&str>,
    start_date: Option<chrono::NaiveDateTime>,
    end_date: Option<chrono::NaiveDateTime>,
    limit: i64,
    offset: i64,
) -> Result<(Vec<UnifiedRevenue>, i64, i64), SocialError> {
    let mut conn = db.connect().await?;
    let mut query = unified_revenue::table.into_boxed();
    if let Some(a) = creator_address {
        query = query.filter(unified_revenue::creator_address.eq(a));
    }
    if let Some(a) = platform_address {
        query = query.filter(unified_revenue::platform_address.eq(a));
    }
    if let Some(s) = revenue_source {
        query = query.filter(unified_revenue::revenue_source.eq(s));
    }
    if let Some(t) = revenue_type {
        query = query.filter(unified_revenue::revenue_type.eq(t));
    }
    if let Some(c) = content_id {
        query = query.filter(unified_revenue::content_id.eq(c));
    }
    if let Some(c) = content_type {
        query = query.filter(unified_revenue::content_type.eq(c));
    }
    if let Some(d) = start_date {
        query = query.filter(unified_revenue::time.ge(
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
        ));
    }
    if let Some(d) = end_date {
        query = query.filter(unified_revenue::time.le(
            chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
        ));
    }
    let total_count: i64 = {
        let mut q = unified_revenue::table.into_boxed();
        if let Some(a) = creator_address {
            q = q.filter(unified_revenue::creator_address.eq(a));
        }
        if let Some(a) = platform_address {
            q = q.filter(unified_revenue::platform_address.eq(a));
        }
        if let Some(s) = revenue_source {
            q = q.filter(unified_revenue::revenue_source.eq(s));
        }
        if let Some(t) = revenue_type {
            q = q.filter(unified_revenue::revenue_type.eq(t));
        }
        if let Some(c) = content_id {
            q = q.filter(unified_revenue::content_id.eq(c));
        }
        if let Some(c) = content_type {
            q = q.filter(unified_revenue::content_type.eq(c));
        }
        if let Some(d) = start_date {
            q = q.filter(unified_revenue::time.ge(
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
            ));
        }
        if let Some(d) = end_date {
            q = q.filter(unified_revenue::time.le(
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
            ));
        }
        q.count().get_result(&mut conn).await?
    };
    let total_amount: Option<bigdecimal::BigDecimal> = {
        let mut q = unified_revenue::table.into_boxed();
        if let Some(a) = creator_address {
            q = q.filter(unified_revenue::creator_address.eq(a));
        }
        if let Some(a) = platform_address {
            q = q.filter(unified_revenue::platform_address.eq(a));
        }
        if let Some(s) = revenue_source {
            q = q.filter(unified_revenue::revenue_source.eq(s));
        }
        if let Some(t) = revenue_type {
            q = q.filter(unified_revenue::revenue_type.eq(t));
        }
        if let Some(c) = content_id {
            q = q.filter(unified_revenue::content_id.eq(c));
        }
        if let Some(c) = content_type {
            q = q.filter(unified_revenue::content_type.eq(c));
        }
        if let Some(d) = start_date {
            q = q.filter(unified_revenue::time.ge(
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
            ));
        }
        if let Some(d) = end_date {
            q = q.filter(unified_revenue::time.le(
                chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(d, chrono::Utc),
            ));
        }
        q.select(sum(unified_revenue::amount))
            .get_result(&mut conn)
            .await?
    };
    let total_amount: i64 = total_amount
        .and_then(|bd| bigdecimal::ToPrimitive::to_i64(&bd))
        .unwrap_or(0);
    let records: Vec<UnifiedRevenue> = query
        .order_by(unified_revenue::time.desc())
        .limit(limit)
        .offset(offset)
        .select(UnifiedRevenue::as_select())
        .load(&mut conn)
        .await?;
    Ok((records, total_count, total_amount))
}

pub(crate) async fn get_creator_revenue_stats(
    db: &Db,
    creator_address: &str,
) -> Result<Option<serde_json::Value>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT creator_address, total_revenue, total_subscription_revenue, total_mydata_revenue,
               total_spt_revenue, total_tips_revenue, total_transactions, total_unique_payers,
               largest_single_transaction, active_days, last_revenue_date
        FROM spt_creator_revenue_summary WHERE creator_address = $1
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        creator_address: String,
        #[diesel(sql_type = BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_subscription_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_mydata_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_spt_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_tips_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_transactions: i64,
        #[diesel(sql_type = BigInt)]
        total_unique_payers: i64,
        #[diesel(sql_type = BigInt)]
        largest_single_transaction: i64,
        #[diesel(sql_type = BigInt)]
        active_days: i64,
        #[diesel(sql_type = Nullable<Timestamptz>)]
        last_revenue_date: Option<chrono::DateTime<chrono::Utc>>,
    }
    let result: Option<Row> = diesel::sql_query(query)
        .bind::<Text, _>(creator_address)
        .get_result(&mut conn)
        .await
        .optional()?;
    Ok(result.map(|r| {
        serde_json::json!({
            "creator_address": r.creator_address,
            "total_revenue": r.total_revenue,
            "subscription_revenue": r.total_subscription_revenue,
            "mydata_revenue": r.total_mydata_revenue,
            "spt_revenue": r.total_spt_revenue,
            "tips_revenue": r.total_tips_revenue,
            "posts_revenue": 0,
            "total_transactions": r.total_transactions,
            "unique_payers": r.total_unique_payers,
            "largest_transaction": r.largest_single_transaction,
            "active_days": r.active_days,
            "last_revenue_date": r.last_revenue_date.map(|d: chrono::DateTime<chrono::Utc>| d.to_rfc3339()),
            "revenue_rank": serde_json::Value::Null
        })
    }))
}

pub(crate) async fn get_platform_revenue_stats(
    db: &Db,
    platform_address: &str,
) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT platform_address, total_revenue, total_subscription_revenue, total_mydata_revenue,
               total_spt_revenue, total_transactions, total_creators, total_payers,
               avg_transaction_amount, active_months, last_active_month
        FROM platform_revenue_summary WHERE platform_address = $1
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        platform_address: String,
        #[diesel(sql_type = BigInt)]
        total_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_subscription_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_mydata_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_spt_revenue: i64,
        #[diesel(sql_type = BigInt)]
        total_transactions: i64,
        #[diesel(sql_type = BigInt)]
        total_creators: i64,
        #[diesel(sql_type = BigInt)]
        total_payers: i64,
        #[diesel(sql_type = Double)]
        avg_transaction_amount: f64,
        #[diesel(sql_type = BigInt)]
        active_months: i64,
        #[diesel(sql_type = Nullable<Date>)]
        last_active_month: Option<chrono::NaiveDate>,
    }
    let result: Option<Row> = diesel::sql_query(query)
        .bind::<Text, _>(platform_address)
        .get_result(&mut conn)
        .await
        .optional()?;
    Ok(result.map_or_else(
        || {
            serde_json::json!({
                "platform_address": platform_address,
                "total_revenue": 0,
                "subscription_revenue": 0,
                "mydata_revenue": 0,
                "spt_revenue": 0,
                "total_transactions": 0,
                "unique_creators": 0,
                "unique_payers": 0,
                "avg_transaction_amount": 0.0,
                "active_months": 0,
                "last_active_month": serde_json::Value::Null
            })
        },
        |r| {
            serde_json::json!({
                "platform_address": r.platform_address,
                "total_revenue": r.total_revenue,
                "subscription_revenue": r.total_subscription_revenue,
                "mydata_revenue": r.total_mydata_revenue,
                "spt_revenue": r.total_spt_revenue,
                "total_transactions": r.total_transactions,
                "unique_creators": r.total_creators,
                "unique_payers": r.total_payers,
                "avg_transaction_amount": r.avg_transaction_amount,
                "active_months": r.active_months,
                "last_active_month": r.last_active_month.map(|d: chrono::NaiveDate| d.format("%Y-%m-%d").to_string())
            })
        },
    ))
}

pub(crate) async fn get_current_treasury(
    db: &Db,
) -> Result<Option<serde_json::Value>, SocialError> {
    use diesel::prelude::QueryableByName;

    #[derive(QueryableByName)]
    struct TreasuryRow {
        #[diesel(sql_type = Text)]
        treasury_address: String,
        #[diesel(sql_type = Text)]
        updated_by: String,
        #[diesel(sql_type = BigInt)]
        timestamp_ms: i64,
        #[diesel(sql_type = Timestamptz)]
        time: chrono::DateTime<chrono::Utc>,
        #[diesel(sql_type = Text)]
        transaction_id: String,
    }

    let mut conn = db.connect().await?;
    let row = diesel::sql_query(
        "SELECT treasury_address, updated_by, timestamp_ms, time, transaction_id \
         FROM ecosystem_treasury ORDER BY time DESC LIMIT 1",
    )
    .get_result::<TreasuryRow>(&mut conn)
    .await
    .optional()?;

    Ok(row.map(|r| {
        serde_json::json!({
            "treasury_address": r.treasury_address,
            "updated_by": r.updated_by,
            "timestamp_ms": r.timestamp_ms,
            "time": r.time.timestamp(),
            "transaction_id": r.transaction_id
        })
    }))
}

pub(crate) async fn get_treasury_history(
    db: &Db,
    limit: i64,
) -> Result<Vec<serde_json::Value>, SocialError> {
    let mut conn = db.connect().await?;
    let rows: Vec<(String, String, i64, chrono::DateTime<chrono::Utc>, String)> =
        ecosystem_treasury::table
            .order_by(ecosystem_treasury::time.desc())
            .limit(limit)
            .select((
                ecosystem_treasury::treasury_address,
                ecosystem_treasury::updated_by,
                ecosystem_treasury::timestamp_ms,
                ecosystem_treasury::time,
                ecosystem_treasury::transaction_id,
            ))
            .load(&mut conn)
            .await?;
    Ok(rows
        .into_iter()
        .map(
            |(treasury_address, updated_by, timestamp_ms, time, transaction_id)| {
                serde_json::json!({
                    "treasury_address": treasury_address,
                    "updated_by": updated_by,
                    "timestamp_ms": timestamp_ms,
                    "time": time.timestamp(),
                    "transaction_id": transaction_id
                })
            },
        )
        .collect())
}
