// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text, Timestamp};
use diesel_async::RunQueryDsl;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName)]
pub struct PlatformRow {
    #[diesel(sql_type = Text)]
    pub platform_id: String,
    #[diesel(sql_type = Text)]
    pub name: String,
    #[diesel(sql_type = Text)]
    pub tagline: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub description: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub logo: Option<String>,
    #[diesel(sql_type = Text)]
    pub developer_address: String,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
    #[diesel(sql_type = Bool)]
    pub is_approved: bool,
    #[diesel(sql_type = Text)]
    pub primary_category: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub secondary_category: Option<String>,
    #[diesel(sql_type = Timestamp)]
    pub created_at: NaiveDateTime,
    #[diesel(sql_type = Timestamp)]
    pub updated_at: NaiveDateTime,
}

pub(crate) async fn get_platform_by_id(
    conn: &mut Connection<'_>,
    platform_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PlatformRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let result = diesel::sql_query(
        "SELECT platform_id, name, tagline, description, logo, developer_address,
                status, is_approved, primary_category, secondary_category, created_at, updated_at
         FROM platforms
         WHERE platform_id = $1 AND deleted_at IS NULL
         LIMIT 1",
    )
    .bind::<Text, _>(platform_id)
    .get_result::<PlatformRow>(conn)
    .await
    .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_platforms(
    conn: &mut Connection<'_>,
    approved_only: bool,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PlatformRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT platform_id, name, tagline, description, logo, developer_address,
               status, is_approved, primary_category, secondary_category, created_at, updated_at
        FROM platforms
        WHERE deleted_at IS NULL
        AND ($1::BOOL = FALSE OR is_approved = TRUE)
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Bool, _>(approved_only)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PlatformRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}
