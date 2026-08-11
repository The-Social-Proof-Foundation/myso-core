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

/// Escape `%` / `_` and wrap as contains pattern `%q%`. Empty after trim → `None`.
fn like_contains_pattern(q: &str) -> Option<String> {
    let trimmed = q.trim();
    if trimmed.is_empty() {
        return None;
    }
    let escaped = trimmed.replace('\\', "\\\\").replace('%', "\\%").replace('_', "\\_");
    Some(format!("%{escaped}%"))
}

pub(crate) async fn search(db: &Db, q: &str, limit: i64) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;

    let profiles_result = search_profiles_with_conn(&mut conn, q, limit).await?;
    let posts_result = search_posts_with_conn(&mut conn, q, limit).await?;
    let platforms_result = search_platforms_with_conn(&mut conn, q, limit).await?;
    let platforms_count = count_platforms_like(&mut conn, q).await?;

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
    let Some(pat) = like_contains_pattern(q) else {
        return Ok(vec![]);
    };
    let sql = r#"
        SELECT id
        FROM platforms
        WHERE deleted_at IS NULL
          AND (
            name ILIKE $1 ESCAPE '\'
            OR coalesce(tagline, '') ILIKE $1 ESCAPE '\'
            OR coalesce(description, '') ILIKE $1 ESCAPE '\'
          )
        ORDER BY name ASC
        LIMIT $2
    "#;
    let ids: Vec<IdRow> = diesel::sql_query(sql)
        .bind::<Text, _>(&pat)
        .bind::<BigInt, _>(limit)
        .load(conn)
        .await?;
    load_platforms_ordered(conn, ids.into_iter().map(|r| r.id).collect()).await
}

async fn count_platforms_like(
    conn: &mut diesel_async::AsyncPgConnection,
    q: &str,
) -> Result<i64, SocialError> {
    let Some(pat) = like_contains_pattern(q) else {
        return Ok(0);
    };
    let sql = r#"
        SELECT COUNT(*)::bigint AS cnt
        FROM platforms
        WHERE deleted_at IS NULL
          AND (
            name ILIKE $1 ESCAPE '\'
            OR coalesce(tagline, '') ILIKE $1 ESCAPE '\'
            OR coalesce(description, '') ILIKE $1 ESCAPE '\'
          )
    "#;
    let row: CountRow = diesel::sql_query(sql)
        .bind::<Text, _>(&pat)
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
    // 1) Exact: full wallet or full username
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
    let mut out = load_profiles_ordered(conn, exact_ids.into_iter().map(|r| r.id).collect()).await?;

    let seen: std::collections::HashSet<i32> = out.iter().map(|p| p.id).collect();
    let remaining = limit - out.len() as i64;
    if remaining <= 0 {
        return Ok(out.into_iter().take(limit as usize).collect());
    }

    // 2) Substring ILIKE on address / username / display_name / bio
    let Some(pat) = like_contains_pattern(q) else {
        return Ok(out);
    };
    let overfetch = remaining + seen.len() as i64;
    let like_sql = r#"
        SELECT id
        FROM profiles
        WHERE owner_address ILIKE $1 ESCAPE '\'
           OR coalesce(username, '') ILIKE $1 ESCAPE '\'
           OR coalesce(display_name, '') ILIKE $1 ESCAPE '\'
           OR coalesce(bio, '') ILIKE $1 ESCAPE '\'
        ORDER BY username ASC NULLS LAST
        LIMIT $2
    "#;
    let like_ids: Vec<IdRow> = diesel::sql_query(like_sql)
        .bind::<Text, _>(&pat)
        .bind::<BigInt, _>(overfetch)
        .load(conn)
        .await?;
    let like_ids: Vec<i32> = like_ids
        .into_iter()
        .map(|r| r.id)
        .filter(|id| !seen.contains(id))
        .take(remaining as usize)
        .collect();

    let mut like_profiles = load_profiles_ordered(conn, like_ids).await?;
    out.append(&mut like_profiles);
    Ok(out)
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
    let Some(pat) = like_contains_pattern(q) else {
        return Ok(vec![]);
    };
    let candidate_limit = limit.saturating_mul(10).max(limit);
    // Overfetch LIKE hits, keep latest version per post_id.
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
                   time,
                   ROW_NUMBER() OVER (PARTITION BY post_id ORDER BY time DESC) AS rn
            FROM (
                SELECT *
                FROM posts
                WHERE deleted_at IS NULL
                  AND (
                    coalesce(content, '') ILIKE $1 ESCAPE '\'
                    OR coalesce(post_id, '') ILIKE $1 ESCAPE '\'
                    OR coalesce(owner, '') ILIKE $1 ESCAPE '\'
                  )
                ORDER BY time DESC
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
        ORDER BY time DESC
        LIMIT $3
    "#;

    let results = diesel::sql_query(query)
        .bind::<Text, _>(&pat)
        .bind::<BigInt, _>(candidate_limit)
        .bind::<BigInt, _>(limit)
        .load::<PostBasicRow>(conn)
        .await?;

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::like_contains_pattern;

    #[test]
    fn contains_pattern_wraps_percent() {
        assert_eq!(like_contains_pattern("bran").as_deref(), Some("%bran%"));
    }

    #[test]
    fn contains_pattern_escapes_like_metachars() {
        assert_eq!(
            like_contains_pattern("br%an_").as_deref(),
            Some("%br\\%an\\_%")
        );
    }

    #[test]
    fn contains_pattern_empty() {
        assert_eq!(like_contains_pattern("   "), None);
        assert_eq!(like_contains_pattern(""), None);
    }
}
