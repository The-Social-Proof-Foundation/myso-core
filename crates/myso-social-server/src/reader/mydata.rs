// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::OptionalExtension;
use diesel_async::RunQueryDsl;
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::{
    AccessAnalytics, AccessLogInfo, DailyRevenue, MyDataBasic, MyDataConfigInfo,
    MyDataStatsResponse, PurchaseInfo, RevenueInfo, SubscriptionInfo,
};

pub(crate) async fn get_mydata_by_id(
    db: &Db,
    mydata_id: &str,
) -> Result<Option<MyDataBasic>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT mydata_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
               created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
               geographic_region, data_quality, sample_size, is_updating, update_frequency
        FROM mydata_data
        WHERE mydata_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .get_result::<MyDataBasic>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn list_mydata(
    db: &Db,
    limit: i64,
    offset: i64,
    creator: Option<&str>,
    media_type: Option<&str>,
    platform_id: Option<&str>,
    sort_by: Option<&str>,
) -> Result<Vec<MyDataBasic>, SocialError> {
    let mut conn = db.connect().await?;
    let sort_clause = match sort_by {
        Some("price") => " ORDER BY COALESCE(one_time_price, subscription_price) DESC",
        Some("updated") => " ORDER BY last_updated DESC",
        _ => " ORDER BY created_at DESC",
    };
    let query = format!(
        "
        SELECT mydata_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
               created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
               geographic_region, data_quality, sample_size, is_updating, update_frequency
        FROM mydata_data
        WHERE ($1::text IS NULL OR owner = $1)
          AND ($2::text IS NULL OR media_type = $2)
          AND ($3::text IS NULL OR platform_id = $3)
        {}
        LIMIT $4 OFFSET $5
        ",
        sort_clause
    );
    let results = diesel::sql_query(&query)
        .bind::<Nullable<Text>, _>(creator)
        .bind::<Nullable<Text>, _>(media_type)
        .bind::<Nullable<Text>, _>(platform_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataBasic>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_mydata_configuration(
    db: &Db,
) -> Result<Option<MyDataConfigInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT updated_by, enable_flag, max_tags, max_subscription_days,
               max_free_access_grants, timestamp_ms, time, transaction_id
        FROM mydata_config
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .get_result::<MyDataConfigInfo>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_popular_mydata(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<MyDataBasic>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT DISTINCT
            d.mydata_id, d.owner, d.media_type, d.tags, d.platform_id, d.timestamp_start, d.timestamp_end,
            d.created_at, d.last_updated, d.one_time_price, d.subscription_price, d.subscription_duration_days,
            d.geographic_region, d.data_quality, d.sample_size, d.is_updating, d.update_frequency
        FROM mydata_data d
        LEFT JOIN mydata_purchases p ON d.mydata_id = p.mydata_id
        LEFT JOIN mydata_revenue r ON d.mydata_id = r.mydata_id
        LEFT JOIN mydata_access_logs a ON d.mydata_id = a.mydata_id
        WHERE (d.one_time_price IS NOT NULL OR d.subscription_price IS NOT NULL)
        GROUP BY d.mydata_id, d.owner, d.media_type, d.tags, d.platform_id, d.timestamp_start, d.timestamp_end,
                 d.created_at, d.last_updated, d.one_time_price, d.subscription_price, d.subscription_duration_days,
                 d.geographic_region, d.data_quality, d.sample_size, d.is_updating, d.update_frequency
        ORDER BY (COUNT(p.id) + COUNT(r.id) + COUNT(a.id)) DESC, d.created_at DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataBasic>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_mydata_purchases(
    db: &Db,
    mydata_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PurchaseInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT id, mydata_id, buyer, price, purchase_type, purchase_time, time, transaction_id
        FROM mydata_purchases
        WHERE mydata_id = $1
        ORDER BY purchase_time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PurchaseInfo>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_mydata_subscriptions(
    db: &Db,
    mydata_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<SubscriptionInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT id, mydata_id, subscriber, subscription_start, subscription_end, price, time, transaction_id
        FROM mydata_subscriptions
        WHERE mydata_id = $1
        ORDER BY subscription_start DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<SubscriptionInfo>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_mydata_revenue(
    db: &Db,
    mydata_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<RevenueInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT id, mydata_id, from_address, to_address, amount, revenue_type, revenue_time, time, transaction_id
        FROM mydata_revenue
        WHERE mydata_id = $1
        ORDER BY revenue_time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<RevenueInfo>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_mydata_access_logs(
    db: &Db,
    mydata_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<AccessLogInfo>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT id, mydata_id, user_address, access_type, access_time, time, transaction_id
        FROM mydata_access_logs
        WHERE mydata_id = $1
        ORDER BY access_time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<AccessLogInfo>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_creator_mydata(
    db: &Db,
    creator: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<MyDataBasic>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT mydata_id, owner, media_type, tags, platform_id, timestamp_start, timestamp_end,
               created_at, last_updated, one_time_price, subscription_price, subscription_duration_days,
               geographic_region, data_quality, sample_size, is_updating, update_frequency
        FROM mydata_data
        WHERE owner = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(creator)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataBasic>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_mydata_stats(
    db: &Db,
    mydata_id: &str,
) -> Result<Option<MyDataStatsResponse>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT
            d.mydata_id, d.owner, d.media_type,
            COALESCE((SELECT SUM(amount) FROM mydata_revenue WHERE mydata_id = $1), 0) as total_revenue,
            (SELECT COUNT(*) FROM mydata_purchases WHERE mydata_id = $1) as purchase_count,
            (SELECT COUNT(*) FROM mydata_subscriptions WHERE mydata_id = $1) as subscription_count,
            (SELECT COUNT(*) FROM mydata_access_logs WHERE mydata_id = $1) as access_count,
            d.one_time_price, d.subscription_price, d.created_at, d.last_updated
        FROM mydata_data d
        WHERE d.mydata_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .get_result::<MyDataStatsResponse>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_mydata_revenue_timeline(
    db: &Db,
    mydata_id: &str,
) -> Result<Vec<DailyRevenue>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT
            time_bucket('1 day', to_timestamp(revenue_time))::date as day,
            SUM(amount) as daily_revenue,
            COUNT(*) as daily_transactions
        FROM mydata_revenue
        WHERE mydata_id = $1
        GROUP BY time_bucket('1 day', to_timestamp(revenue_time))
        ORDER BY day DESC
        LIMIT 30
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .load::<DailyRevenue>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_mydata_access_analytics(
    db: &Db,
    mydata_id: &str,
) -> Result<Vec<AccessAnalytics>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT
            time_bucket('1 day', to_timestamp(access_time))::date as day,
            access_type,
            COUNT(DISTINCT user_address) as unique_users,
            COUNT(*) as total_accesses
        FROM mydata_access_logs
        WHERE mydata_id = $1
        GROUP BY time_bucket('1 day', to_timestamp(access_time)), access_type
        ORDER BY day DESC, access_type
        LIMIT 100
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .load::<AccessAnalytics>(&mut conn)
        .await?;
    Ok(results)
}
