// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Text};
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::PgTextExpressionMethods;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{Platform, Profile};
use myso_indexer_alt_social_schema::schema::{platforms, profiles};

use crate::error::SocialError;
use crate::reader::types::PostBasicRow;
use myso_pg_db::Db;

pub(crate) async fn search(db: &Db, q: &str, limit: i64) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;

    let profiles_result = search_profiles_with_conn(&mut conn, q, limit).await?;
    let posts_result = search_posts_with_conn(&mut conn, q, limit).await?;
    let platforms_result = search_platforms_with_conn(&mut conn, q, limit).await?;
    let platforms_count: i64 = platforms::table
        .filter(platforms::name.ilike(&format!("%{}%", q)))
        .filter(platforms::deleted_at.is_null())
        .count()
        .get_result(&mut conn)
        .await?;

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
    let pattern = format!("%{}%", q);
    let rows: Vec<Platform> = platforms::table
        .filter(platforms::name.ilike(&pattern))
        .filter(platforms::deleted_at.is_null())
        .order(platforms::updated_at.desc())
        .limit(limit)
        .select(Platform::as_select())
        .load(conn)
        .await?;
    Ok(rows)
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
    let pattern = format!("%{}%", q);
    let profiles: Vec<Profile> = profiles::table
        .filter(
            profiles::username
                .ilike(&pattern)
                .or(profiles::display_name.ilike(&pattern))
                .or(profiles::owner_address.eq(q)),
        )
        .limit(limit)
        .select(Profile::as_select())
        .load(conn)
        .await?;
    Ok(profiles)
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
    let pattern = format!("%{}%", q);
    let query = r#"
        WITH ranked AS (
            SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
                   reaction_count, comment_count, repost_count, tips_received,
                   ROW_NUMBER() OVER (PARTITION BY post_id ORDER BY time DESC) as rn
            FROM posts
            WHERE deleted_at IS NULL AND content ILIKE $1
        )
        SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
               reaction_count, comment_count, repost_count, tips_received
        FROM ranked
        WHERE rn = 1
        ORDER BY created_at DESC
        LIMIT $2
    "#;

    let results = diesel::sql_query(query)
        .bind::<Text, _>(&pattern)
        .bind::<BigInt, _>(limit)
        .load::<PostBasicRow>(conn)
        .await?;

    Ok(results)
}
