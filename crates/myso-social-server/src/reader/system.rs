// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::BigInt;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_reader::UsernameAvailabilityDetail;

use crate::error::SocialError;
use crate::reader::types::SystemStatsResponse;
use myso_pg_db::Db;

pub(crate) async fn get_system_stats(db: &Db) -> Result<SystemStatsResponse, SocialError> {
    let mut conn = db.connect().await?;
    use diesel::QueryDsl;
    use myso_indexer_alt_social_schema::schema::{platforms, profiles, social_graph_relationships};
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
) -> Result<UsernameAvailabilityDetail, SocialError> {
    myso_indexer_alt_social_reader::get_username_availability_for_db(db, username, exclude_address)
        .await
        .map_err(|e| SocialError::internal(e.to_string()))
}
