// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Integer, Text};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{Platform, Profile};
use myso_indexer_alt_social_schema::schema::{platforms, profiles};

use crate::error::SocialError;
use crate::reader::types::PostBasicRow;
use myso_pg_db::Db;

const PROFILE_SEARCH_EXPR: &str =
    "(coalesce(username, '') || ' ' || coalesce(display_name, '') || ' ' || coalesce(bio, ''))";
const PLATFORM_SEARCH_EXPR: &str =
    "(name || ' ' || coalesce(tagline, '') || ' ' || coalesce(description, ''))";

#[derive(QueryableByName)]
struct IdRow {
    #[diesel(sql_type = Integer)]
    id: i32,
}

#[derive(QueryableByName)]
struct CountRow {
    #[diesel(sql_type = BigInt)]
    cnt: i64,
}

pub(crate) async fn search(db: &Db, q: &str, limit: i64) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;

    let profiles_result = search_profiles_with_conn(&mut conn, q, limit).await?;
    let posts_result = search_posts_with_conn(&mut conn, q, limit).await?;
    let platforms_result = search_platforms_with_conn(&mut conn, q, limit).await?;
    let platforms_count = count_platforms_bm25(&mut conn, q).await?;

    Ok(serde_json::json!({
        "profiles": profiles_result,
        "posts": posts_result,
        "platforms": platforms_result,
        "platforms_count": platforms_count,
    }))
}

async fn search_platforms_with_conn(
    conn: &mut diesel_async::AsyncPgConnection,
    q: &str,
    limit: i64,
) -> Result<Vec<Platform>, SocialError> {
    let sql = format!(
        r#"
        SELECT id
        FROM platforms
        WHERE deleted_at IS NULL
        ORDER BY {PLATFORM_SEARCH_EXPR} <@> $1
        LIMIT $2
        "#
    );
    let ids: Vec<IdRow> = diesel::sql_query(sql)
        .bind::<Text, _>(q)
        .bind::<BigInt, _>(limit)
        .load(conn)
        .await?;
    load_platforms_ordered(conn, ids.into_iter().map(|r| r.id).collect()).await
}

async fn count_platforms_bm25(
    conn: &mut diesel_async::AsyncPgConnection,
    q: &str,
) -> Result<i64, SocialError> {
    let sql = format!(
        r#"
        SELECT COUNT(*)::bigint AS cnt
        FROM platforms
        WHERE deleted_at IS NULL
          AND {PLATFORM_SEARCH_EXPR} <@> $1 < 0
        "#
    );
    let row: CountRow = diesel::sql_query(sql)
        .bind::<Text, _>(q)
        .get_result(conn)
        .await?;
    Ok(row.cnt)
}

pub(crate) async fn search_profiles(
    db: &Db,
    q: &str,
    limit: i64,
) -> Result<Vec<Profile>, SocialError> {
    let mut conn = db.connect().await?;
    search_profiles_with_conn(&mut conn, q, limit).await
}

async fn search_profiles_with_conn(
    conn: &mut diesel_async::AsyncPgConnection,
    q: &str,
    limit: i64,
) -> Result<Vec<Profile>, SocialError> {
    let exact_sql = r#"
        SELECT id
        FROM profiles
        WHERE owner_address = $1 OR lower(username) = lower($1)
        LIMIT $2
    "#;
    let exact_ids: Vec<IdRow> = diesel::sql_query(exact_sql)
        .bind::<Text, _>(q)
        .bind::<BigInt, _>(limit)
        .load(conn)
        .await?;
    let mut exact =
        load_profiles_ordered(conn, exact_ids.into_iter().map(|r| r.id).collect()).await?;

    let exact_id_set: std::collections::HashSet<i32> = exact.iter().map(|p| p.id).collect();
    let remaining = limit - exact.len() as i64;
    if remaining <= 0 {
        return Ok(exact.into_iter().take(limit as usize).collect());
    }

    let overfetch = remaining + exact_id_set.len() as i64;
    let bm25_sql = format!(
        r#"
        SELECT id
        FROM profiles
        ORDER BY {PROFILE_SEARCH_EXPR} <@> $1
        LIMIT $2
        "#
    );
    let bm25_ids: Vec<IdRow> = diesel::sql_query(bm25_sql)
        .bind::<Text, _>(q)
        .bind::<BigInt, _>(overfetch)
        .load(conn)
        .await?;

    let bm25_ids: Vec<i32> = bm25_ids
        .into_iter()
        .map(|r| r.id)
        .filter(|id| !exact_id_set.contains(id))
        .take(remaining as usize)
        .collect();

    let mut bm25_profiles = load_profiles_ordered(conn, bm25_ids).await?;
    exact.append(&mut bm25_profiles);
    Ok(exact)
}

async fn load_profiles_ordered(
    conn: &mut diesel_async::AsyncPgConnection,
    ids: Vec<i32>,
) -> Result<Vec<Profile>, SocialError> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let rows: Vec<Profile> = profiles::table
        .filter(profiles::id.eq_any(&ids))
        .select(Profile::as_select())
        .load(conn)
        .await?;
    let mut by_id: std::collections::HashMap<i32, Profile> =
        rows.into_iter().map(|p| (p.id, p)).collect();
    Ok(ids
        .into_iter()
        .filter_map(|id| by_id.remove(&id))
        .collect())
}

async fn load_platforms_ordered(
    conn: &mut diesel_async::AsyncPgConnection,
    ids: Vec<i32>,
) -> Result<Vec<Platform>, SocialError> {
    if ids.is_empty() {
        return Ok(vec![]);
    }
    let rows: Vec<Platform> = platforms::table
        .filter(platforms::id.eq_any(&ids))
        .select(Platform::as_select())
        .load(conn)
        .await?;
    let mut by_id: std::collections::HashMap<i32, Platform> =
        rows.into_iter().map(|p| (p.id, p)).collect();
    Ok(ids
        .into_iter()
        .filter_map(|id| by_id.remove(&id))
        .collect())
}

pub(crate) async fn search_posts(
    db: &Db,
    q: &str,
    limit: i64,
) -> Result<Vec<PostBasicRow>, SocialError> {
    let mut conn = db.connect().await?;
    search_posts_with_conn(&mut conn, q, limit).await
}

async fn search_posts_with_conn(
    conn: &mut diesel_async::AsyncPgConnection,
    q: &str,
    limit: i64,
) -> Result<Vec<PostBasicRow>, SocialError> {
    let candidate_limit = limit.saturating_mul(10).max(limit);
    // Overfetch BM25 hits, keep latest version per post_id, re-rank by BM25 score.
    let query = r#"
        WITH candidates AS (
            SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
                   COALESCE(reaction_count, 0) AS reaction_count,
                   COALESCE(comment_count, 0) AS comment_count,
                   COALESCE(repost_count, 0) AS repost_count,
                   COALESCE(tips_received, 0) AS tips_received,
                   mydata_id, poc_id, revenue_redirect_to, revenue_redirect_percentage,
                   poc_reasoning, poc_evidence_urls, poc_similarity_score, poc_media_type,
                   poc_oracle_address, poc_analyzed_at,
                   poc_outcome, poc_redirection_kind, poc_disputes_submitted,
                   NULL::text AS actor_address, sub_agent_id, action_identity_class,
                   content <@> $1 AS bm25_score,
                   time,
                   ROW_NUMBER() OVER (PARTITION BY post_id ORDER BY time DESC) AS rn
            FROM (
                SELECT *
                FROM posts
                WHERE deleted_at IS NULL
                ORDER BY content <@> $1
                LIMIT $2
            ) ranked_hits
        )
        SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
               reaction_count, comment_count, repost_count, tips_received, mydata_id,
               poc_id, revenue_redirect_to, revenue_redirect_percentage,
               poc_reasoning, poc_evidence_urls, poc_similarity_score, poc_media_type,
               poc_oracle_address, poc_analyzed_at,
               poc_outcome, poc_redirection_kind, poc_disputes_submitted,
               actor_address, sub_agent_id, action_identity_class
        FROM candidates
        WHERE rn = 1
        ORDER BY bm25_score
        LIMIT $3
    "#;

    let results = diesel::sql_query(query)
        .bind::<Text, _>(q)
        .bind::<BigInt, _>(candidate_limit)
        .bind::<BigInt, _>(limit)
        .load::<PostBasicRow>(conn)
        .await?;

    Ok(results)
}
