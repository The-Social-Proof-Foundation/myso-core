// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::JoinOnDsl;
use diesel::NullableExpressionMethods;
use diesel::sql_types::BigInt;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::{platforms, profiles, social_graph_relationships, username_registry};

use crate::error::SocialError;
use crate::reader::types::SystemStatsResponse;
use myso_pg_db::Db;

pub(crate) async fn get_system_stats(db: &Db) -> Result<SystemStatsResponse, SocialError> {
    let mut conn = db.connect().await?;
    let profiles_count: i64 = profiles::table.count().get_result(&mut conn).await?;
    let platforms_count: i64 = platforms::table.count().get_result(&mut conn).await?;
    let social_relationships_count: i64 = social_graph_relationships::table
        .count()
        .get_result(&mut conn)
        .await?;
    let query = "
        SELECT
            (SELECT COUNT(*) FROM posts WHERE deleted_at IS NULL)::bigint as total_posts,
            (SELECT COUNT(*) FROM comments WHERE deleted_at IS NULL)::bigint as total_comments,
            (SELECT COUNT(*) FROM reactions)::bigint as total_reactions,
            (SELECT COUNT(*) FROM spt_pools)::bigint as social_proof_tokens
    ";
    #[derive(QueryableByName)]
    struct StatsRow {
        #[diesel(sql_type = BigInt)]
        total_posts: i64,
        #[diesel(sql_type = BigInt)]
        total_comments: i64,
        #[diesel(sql_type = BigInt)]
        total_reactions: i64,
        #[diesel(sql_type = BigInt)]
        social_proof_tokens: i64,
    }
    let row = diesel::sql_query(query)
        .get_result::<StatsRow>(&mut conn)
        .await?;
    Ok(SystemStatsResponse {
        profiles: profiles_count,
        platforms: platforms_count,
        total_posts: row.total_posts,
        total_comments: row.total_comments,
        total_reactions: row.total_reactions,
        social_proof_tokens: row.social_proof_tokens,
        total_social_relationships: social_relationships_count,
    })
}

pub(crate) async fn check_username_availability(
    db: &Db,
    username: &str,
    exclude_address: Option<&str>,
) -> Result<bool, SocialError> {
    let mut conn = db.connect().await?;
    let base = username_registry::table
        .inner_join(
            profiles::table.on(profiles::profile_id.eq(username_registry::profile_id.nullable())),
        )
        .filter(username_registry::username.eq(username));
    let count: i64 = match exclude_address {
        Some(addr) => base
            .filter(profiles::owner_address.ne(addr))
            .count()
            .get_result(&mut conn)
            .await?,
        None => base.count().get_result(&mut conn).await?,
    };
    Ok(count == 0)
}
