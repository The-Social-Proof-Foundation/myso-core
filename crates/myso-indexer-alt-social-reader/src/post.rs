// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, QueryableByName)]
pub struct PostRow {
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = Text)]
    pub owner: String,
    #[diesel(sql_type = Text)]
    pub profile_id: String,
    #[diesel(sql_type = Text)]
    pub content: String,
    #[diesel(sql_type = Text)]
    pub post_type: String,
    #[diesel(sql_type = BigInt)]
    pub created_at: i64,
    #[diesel(sql_type = Nullable<BigInt>)]
    pub deleted_at: Option<i64>,
    #[diesel(sql_type = BigInt)]
    pub reaction_count: i64,
    #[diesel(sql_type = BigInt)]
    pub comment_count: i64,
    #[diesel(sql_type = BigInt)]
    pub repost_count: i64,
    #[diesel(sql_type = BigInt)]
    pub tips_received: i64,
}

pub(crate) async fn get_post_by_id(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PostRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let result = diesel::sql_query(
        "SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
                reaction_count, comment_count, repost_count, tips_received
         FROM posts
         WHERE (post_id = $1 OR id = $1) AND deleted_at IS NULL
         ORDER BY created_at DESC
         LIMIT 1",
    )
    .bind::<Text, _>(post_id)
    .get_result::<PostRow>(conn)
    .await
    .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_posts(
    conn: &mut Connection<'_>,
    owner: Option<&str>,
    post_type: Option<&str>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PostRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
               reaction_count, comment_count, repost_count, tips_received
        FROM posts
        WHERE deleted_at IS NULL
        AND ($1::TEXT IS NULL OR owner = $1)
        AND ($2::TEXT IS NULL OR post_type = $2)
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
    ";
    let results = diesel::sql_query(query)
        .bind::<Nullable<Text>, _>(owner)
        .bind::<Nullable<Text>, _>(post_type)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PostRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}
