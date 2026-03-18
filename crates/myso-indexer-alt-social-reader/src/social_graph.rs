// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::QueryableByName;
use diesel::sql_types::{Array, BigInt, Bool, Nullable, Text, Timestamp};
use diesel_async::RunQueryDsl;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone)]
pub struct BlockedProfileRow {
    pub blocked_address: String,
    pub blocked_username: String,
    pub blocked_display_name: Option<String>,
    pub blocked_profile_photo: Option<String>,
    pub first_blocked_at: NaiveDateTime,
    pub last_blocked_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct BlockedPlatformRow {
    pub platform_id: String,
    pub platform_name: String,
    pub blocked_by: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone)]
pub struct ProfileSummaryRow {
    pub owner_address: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub profile_photo: Option<String>,
}

pub(crate) async fn get_profile_summaries_for_addresses(
    conn: &mut Connection<'_>,
    addresses: &[String],
    metrics: &DbReaderMetrics,
) -> anyhow::Result<HashMap<String, ProfileSummaryRow>> {
    if addresses.is_empty() {
        return Ok(HashMap::new());
    }
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        owner_address: String,
        #[diesel(sql_type = Nullable<Text>)]
        username: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        display_name: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        profile_photo: Option<String>,
    }
    let query = "
        SELECT DISTINCT ON (owner_address) owner_address, username, display_name, profile_photo
        FROM profiles
        WHERE owner_address = ANY($1::TEXT[])
        ORDER BY owner_address, updated_at DESC
    ";
    let rows = diesel::sql_query(query)
        .bind::<Array<Text>, _>(addresses)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    let mut result = HashMap::new();
    for row in rows {
        result.insert(
            row.owner_address.clone(),
            ProfileSummaryRow {
                owner_address: row.owner_address,
                username: row.username,
                display_name: row.display_name,
                profile_photo: row.profile_photo,
            },
        );
    }
    for addr in addresses {
        if !result.contains_key(addr) {
            result.insert(
                addr.clone(),
                ProfileSummaryRow {
                    owner_address: addr.clone(),
                    username: None,
                    display_name: None,
                    profile_photo: None,
                },
            );
        }
    }
    Ok(result)
}

pub(crate) async fn get_followers(
    conn: &mut Connection<'_>,
    address: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<ProfileSummaryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        addr: String,
        #[diesel(sql_type = Nullable<Text>)]
        username: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        display_name: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        profile_photo: Option<String>,
    }
    let query = "
        SELECT sgr.follower_address AS addr, p.username, p.display_name, p.profile_photo
        FROM social_graph_relationships sgr
        LEFT JOIN profiles p ON p.owner_address = sgr.follower_address
        WHERE sgr.following_address = $1
        ORDER BY sgr.created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| ProfileSummaryRow {
            owner_address: r.addr,
            username: r.username,
            display_name: r.display_name,
            profile_photo: r.profile_photo,
        })
        .collect())
}

pub(crate) async fn get_following(
    conn: &mut Connection<'_>,
    address: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<ProfileSummaryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        addr: String,
        #[diesel(sql_type = Nullable<Text>)]
        username: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        display_name: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        profile_photo: Option<String>,
    }
    let query = "
        SELECT sgr.following_address AS addr, p.username, p.display_name, p.profile_photo
        FROM social_graph_relationships sgr
        LEFT JOIN profiles p ON p.owner_address = sgr.following_address
        WHERE sgr.follower_address = $1
        ORDER BY sgr.created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| ProfileSummaryRow {
            owner_address: r.addr,
            username: r.username,
            display_name: r.display_name,
            profile_photo: r.profile_photo,
        })
        .collect())
}

pub(crate) async fn check_following(
    conn: &mut Connection<'_>,
    follower_address: &str,
    following_address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<bool> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct FollowRow {
        #[diesel(sql_type = Bool)]
        exists: bool,
    }
    let result = diesel::sql_query(
        "SELECT EXISTS(
            SELECT 1 FROM social_graph_relationships
            WHERE follower_address = $1 AND following_address = $2
        ) as exists",
    )
    .bind::<diesel::sql_types::Text, _>(follower_address)
    .bind::<diesel::sql_types::Text, _>(following_address)
    .get_result::<FollowRow>(conn)
    .await?;
    metrics.requests_succeeded.inc();
    Ok(result.exists)
}

pub(crate) async fn get_blocked_profiles(
    conn: &mut Connection<'_>,
    blocker_address: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<BlockedProfileRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        blocked_address: String,
        #[diesel(sql_type = Text)]
        blocked_username: String,
        #[diesel(sql_type = Nullable<Text>)]
        blocked_display_name: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        blocked_profile_photo: Option<String>,
        #[diesel(sql_type = Timestamp)]
        first_blocked_at: NaiveDateTime,
        #[diesel(sql_type = Timestamp)]
        last_blocked_at: NaiveDateTime,
    }
    let query = "
        SELECT blocked_address, blocked_username, blocked_display_name, blocked_profile_photo,
               first_blocked_at, last_blocked_at
        FROM blocked_profiles
        WHERE blocker_address = $1
        ORDER BY last_blocked_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(blocker_address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| BlockedProfileRow {
            blocked_address: r.blocked_address,
            blocked_username: r.blocked_username,
            blocked_display_name: r.blocked_display_name,
            blocked_profile_photo: r.blocked_profile_photo,
            first_blocked_at: r.first_blocked_at,
            last_blocked_at: r.last_blocked_at,
        })
        .collect())
}

pub(crate) async fn get_blocked_platforms(
    conn: &mut Connection<'_>,
    wallet_address: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<BlockedPlatformRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        platform_id: String,
        #[diesel(sql_type = Text)]
        platform_name: String,
        #[diesel(sql_type = Text)]
        blocked_by: String,
        #[diesel(sql_type = Timestamp)]
        created_at: NaiveDateTime,
    }
    let query = "
        SELECT p.platform_id, p.name AS platform_name, pbp.blocked_by, pbp.created_at
        FROM platform_blocked_profiles pbp
        INNER JOIN platforms p ON pbp.platform_id = p.platform_id
        WHERE pbp.wallet_address = $1
        ORDER BY pbp.created_at DESC
        LIMIT $2 OFFSET $3
    ";
    let rows = diesel::sql_query(query)
        .bind::<Text, _>(wallet_address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| BlockedPlatformRow {
            platform_id: r.platform_id,
            platform_name: r.platform_name,
            blocked_by: r.blocked_by,
            created_at: r.created_at,
        })
        .collect())
}

pub(crate) async fn check_profile_blocked(
    conn: &mut Connection<'_>,
    blocker: &str,
    blocked: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<bool> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct ExistsRow {
        #[diesel(sql_type = Bool)]
        exists: bool,
    }
    let result = diesel::sql_query(
        "SELECT EXISTS(
            SELECT 1 FROM blocked_profiles
            WHERE blocker_address = $1 AND blocked_address = $2
        ) as exists",
    )
    .bind::<Text, _>(blocker)
    .bind::<Text, _>(blocked)
    .get_result::<ExistsRow>(conn)
    .await?;
    metrics.requests_succeeded.inc();
    Ok(result.exists)
}

pub(crate) async fn check_platform_blocked(
    conn: &mut Connection<'_>,
    profile_address: &str,
    platform_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<bool> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct ExistsRow {
        #[diesel(sql_type = Bool)]
        exists: bool,
    }
    let result = diesel::sql_query(
        "SELECT EXISTS(
            SELECT 1 FROM platform_blocked_profiles
            WHERE wallet_address = $1 AND platform_id = $2
        ) as exists",
    )
    .bind::<Text, _>(profile_address)
    .bind::<Text, _>(platform_id)
    .get_result::<ExistsRow>(conn)
    .await?;
    metrics.requests_succeeded.inc();
    Ok(result.exists)
}
