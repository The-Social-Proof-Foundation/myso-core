// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Text};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{MyDataPurchaseRow, MyDataRecordRow};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName)]
pub struct MyDataConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = Bool)]
    pub enable_flag: bool,
    #[diesel(sql_type = BigInt)]
    pub max_tags: i64,
    #[diesel(sql_type = BigInt)]
    pub max_subscription_days: i64,
    #[diesel(sql_type = BigInt)]
    pub max_free_access_grants: i64,
    #[diesel(sql_type = BigInt)]
    pub timestamp_ms: i64,
}

pub(crate) async fn get_mydata_record(
    conn: &mut Connection<'_>,
    mydata_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MyDataRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT mydata_id, owner, media_type, tags, one_time_price, subscription_price
        FROM mydata_data
        WHERE mydata_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(mydata_id)
        .get_result::<MyDataRecordRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_mydata_records_by_owner(
    conn: &mut Connection<'_>,
    owner: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataRecordRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT mydata_id, owner, media_type, tags, one_time_price, subscription_price
        FROM mydata_data
        WHERE owner = $1
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(owner)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataRecordRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_mydata_purchases_by_buyer(
    conn: &mut Connection<'_>,
    buyer: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MyDataPurchaseRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, mydata_id, buyer, price, purchase_type, purchase_time, time, transaction_id
        FROM mydata_purchases
        WHERE buyer = $1
        ORDER BY purchase_time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(buyer)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MyDataPurchaseRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_mydata_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MyDataConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, enable_flag, max_tags, max_subscription_days, max_free_access_grants,
               timestamp_ms
        FROM mydata_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<MyDataConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}
