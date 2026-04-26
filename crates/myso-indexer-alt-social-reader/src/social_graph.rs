// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use chrono::NaiveDateTime;
use diesel::QueryableByName;
use diesel::sql_types::{Array, BigInt, Bool, Integer, Jsonb, Nullable, SmallInt, Text, Timestamp};
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::ProfilePlatformMembershipRow;
use myso_pg_db::Connection;
use serde_json::Value as JsonValue;

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

/// Per-subject social edges from a viewer's perspective (follow + block), for batch list APIs.
#[derive(Debug, Clone, Copy, Default)]
pub struct ViewerSocialContext {
    pub is_following: bool,
    pub follows_viewer: bool,
    pub blocked_by_viewer: bool,
    pub blocked_by_subject: bool,
}

#[derive(Debug, Clone)]
pub struct ProfileSummaryRow {
    pub owner_address: String,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub profile_photo: Option<String>,
    pub bio: Option<String>,
    pub selected_badge_id: Option<String>,
    pub social_proof_token_address: Option<String>,
    pub reservation_pool_address: Option<String>,
    /// Follower count (from profiles or wallet_social_graph). Present for both profile and wallet-only.
    pub followers_count: Option<i32>,
    /// Following count (from profiles or wallet_social_graph). Present for both profile and wallet-only.
    pub following_count: Option<i32>,
    /// Post count (from profiles). Present for both profile and wallet-only.
    pub post_count: Option<i32>,
    /// Blocked count (from profiles or wallet_social_graph). Present for both profile and wallet-only.
    pub blocked_count: Option<i32>,
    /// Viewer follows this profile. Set when viewer_address provided in followers/following query.
    pub is_following: Option<bool>,
    /// This profile follows viewer ("Follows you" badge). Set when viewer_address provided.
    pub follows_viewer: Option<bool>,
    /// Viewer (wallet / profile owner refs) has blocked this subject's address.
    pub blocked_by_viewer: Option<bool>,
    /// This subject has blocked the viewer.
    pub blocked_by_subject: Option<bool>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FollowSortBy {
    Latest,
    Earliest,
    Alphabetical,
    MostFollowers,
}

fn follow_order_clause(sort: FollowSortBy, addr_col: &str) -> String {
    match sort {
        FollowSortBy::Latest => "sgr.created_at DESC".to_string(),
        FollowSortBy::Earliest => "sgr.created_at ASC".to_string(),
        FollowSortBy::Alphabetical => {
            format!("COALESCE(p.username, p.display_name, {}) ASC", addr_col)
        }
        FollowSortBy::MostFollowers => {
            "COALESCE(p.followers_count, wsg.followers_count, 0) DESC".to_string()
        }
    }
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
        #[diesel(sql_type = Nullable<Text>)]
        bio: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        selected_badge_id: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        social_proof_token_address: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        reservation_pool_address: Option<String>,
        #[diesel(sql_type = BigInt)]
        post_count: i64,
        #[diesel(sql_type = BigInt)]
        blocked_count: i64,
    }
    let query = "
        SELECT DISTINCT ON (owner_address) owner_address, username, display_name, profile_photo,
               bio, selected_badge_id, social_proof_token_address, reservation_pool_address,
               post_count, blocked_count
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
                bio: row.bio,
                selected_badge_id: row.selected_badge_id,
                social_proof_token_address: row.social_proof_token_address,
                reservation_pool_address: row.reservation_pool_address,
                followers_count: None,
                following_count: None,
                post_count: Some(row.post_count as i32),
                blocked_count: Some(row.blocked_count as i32),
                is_following: None,
                follows_viewer: None,
                blocked_by_viewer: None,
                blocked_by_subject: None,
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
                    bio: None,
                    selected_badge_id: None,
                    social_proof_token_address: None,
                    reservation_pool_address: None,
                    followers_count: None,
                    following_count: None,
                    post_count: None,
                    blocked_count: None,
                    is_following: None,
                    follows_viewer: None,
                    blocked_by_viewer: None,
                    blocked_by_subject: None,
                },
            );
        }
    }
    Ok(result)
}

pub(crate) async fn resolve_profile_address(
    conn: &mut Connection<'_>,
    address: &str,
) -> anyhow::Result<(Option<String>, String)> {
    #[derive(QueryableByName)]
    struct ResolveRow {
        #[diesel(sql_type = Nullable<Text>)]
        profile_id: Option<String>,
        #[diesel(sql_type = Text)]
        owner_address: String,
    }
    let result = diesel::sql_query(
        "SELECT profile_id, owner_address FROM profiles
         WHERE owner_address = $1 OR profile_id = $1
         ORDER BY updated_at DESC LIMIT 1",
    )
    .bind::<Text, _>(address)
    .get_result::<ResolveRow>(conn)
    .await;
    match result {
        Ok(r) => Ok((r.profile_id, r.owner_address)),
        Err(diesel::result::Error::NotFound) => Ok((None, address.to_string())),
        Err(e) => Err(e.into()),
    }
}

pub async fn batch_viewer_social_context(
    conn: &mut Connection<'_>,
    list_addresses: &[String],
    viewer_profile_id: &Option<String>,
    viewer_owner: &str,
) -> anyhow::Result<HashMap<String, ViewerSocialContext>> {
    if list_addresses.is_empty() {
        return Ok(HashMap::new());
    }
    let mut viewer_refs = vec![viewer_owner];
    if let Some(pid) = viewer_profile_id {
        if pid != viewer_owner {
            viewer_refs.push(pid.as_str());
        }
    }
    if viewer_refs.iter().all(|s| s.is_empty()) {
        return Ok(HashMap::new());
    }
    let addrs: Vec<&str> = list_addresses.iter().map(|s| s.as_str()).collect();
    #[derive(QueryableByName)]
    struct ViewerContextRow {
        #[diesel(sql_type = Text)]
        addr: String,
        #[diesel(sql_type = Bool)]
        is_following: bool,
        #[diesel(sql_type = Bool)]
        follows_viewer: bool,
        #[diesel(sql_type = Bool)]
        blocked_by_viewer: bool,
        #[diesel(sql_type = Bool)]
        blocked_by_subject: bool,
    }
    let query = r#"
        WITH list_addrs AS (SELECT unnest($1::TEXT[]) AS addr),
        viewer_follows AS (
            SELECT la.addr,
                EXISTS(SELECT 1 FROM social_graph_relationships sgr
                    WHERE sgr.follower_address = ANY($2::TEXT[])
                    AND sgr.following_address = la.addr) AS is_following
            FROM list_addrs la
        ),
        this_follows_viewer AS (
            SELECT la.addr,
                EXISTS(SELECT 1 FROM social_graph_relationships sgr
                    WHERE sgr.follower_address = la.addr
                    AND sgr.following_address = ANY($2::TEXT[])) AS follows_viewer
            FROM list_addrs la
        ),
        viewer_blocked_subject AS (
            SELECT la.addr,
                EXISTS(SELECT 1 FROM blocked_profiles bp
                    WHERE bp.blocker_address = ANY($2::TEXT[])
                    AND bp.blocked_address = la.addr) AS blocked_by_viewer
            FROM list_addrs la
        ),
        subject_blocked_viewer AS (
            SELECT la.addr,
                EXISTS(SELECT 1 FROM blocked_profiles bp
                    WHERE bp.blocker_address = la.addr
                    AND bp.blocked_address = ANY($2::TEXT[])) AS blocked_by_subject
            FROM list_addrs la
        )
        SELECT vf.addr, vf.is_following, tf.follows_viewer,
               vb.blocked_by_viewer, sb.blocked_by_subject
        FROM viewer_follows vf
        JOIN this_follows_viewer tf ON vf.addr = tf.addr
        JOIN viewer_blocked_subject vb ON vf.addr = vb.addr
        JOIN subject_blocked_viewer sb ON vf.addr = sb.addr
    "#;
    let rows: Vec<ViewerContextRow> = diesel::sql_query(query)
        .bind::<Array<Text>, _>(addrs)
        .bind::<Array<Text>, _>(viewer_refs)
        .load(conn)
        .await?;
    Ok(rows
        .into_iter()
        .map(|r| {
            (
                r.addr,
                ViewerSocialContext {
                    is_following: r.is_following,
                    follows_viewer: r.follows_viewer,
                    blocked_by_viewer: r.blocked_by_viewer,
                    blocked_by_subject: r.blocked_by_subject,
                },
            )
        })
        .collect())
}

pub(crate) async fn get_followers(
    conn: &mut Connection<'_>,
    address: &str,
    sort: FollowSortBy,
    search: Option<&str>,
    viewer_address: Option<&str>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<(Vec<ProfileSummaryRow>, i64)> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let (profile_id, owner_address) = resolve_profile_address(conn, address).await?;
    let ref1 = &owner_address;
    let ref2 = profile_id.as_ref().unwrap_or(&owner_address);

    let search_filter = search
        .filter(|s| !s.trim().is_empty())
        .map(|s| format!("%{}%", s.trim()));
    let search_suffix = search_filter
        .as_ref()
        .map(|_| " AND (p.username ILIKE $3 OR p.display_name ILIKE $3 OR sgr.follower_address ILIKE $3)")
        .unwrap_or_default();

    let order_clause = follow_order_clause(sort, "sgr.follower_address");
    let needs_join = matches!(
        sort,
        FollowSortBy::Alphabetical | FollowSortBy::MostFollowers
    );

    let (data_sql, count_sql) = if needs_join {
        let join_clause = "LEFT JOIN profiles p ON p.owner_address = sgr.follower_address
            LEFT JOIN wallet_social_graph wsg ON wsg.wallet_address = sgr.follower_address AND p.owner_address IS NULL";
        let data_sql = if search_filter.is_some() {
            format!(
                r#"SELECT sgr.follower_address AS addr, p.username, p.display_name, p.profile_photo,
                   p.bio, p.selected_badge_id, p.social_proof_token_address, p.reservation_pool_address,
                   COALESCE(p.post_count, 0)::bigint AS post_count, COALESCE(p.blocked_count, wsg.blocked_count, 0)::bigint AS blocked_count,
                   COALESCE(p.followers_count, wsg.followers_count) AS followers_count, COALESCE(p.following_count, wsg.following_count) AS following_count
                FROM social_graph_relationships sgr
                {}
                WHERE (sgr.following_address = $1 OR sgr.following_address = $2){}
                ORDER BY {}
                LIMIT $4 OFFSET $5"#,
                join_clause, search_suffix, order_clause
            )
        } else {
            format!(
                r#"SELECT sgr.follower_address AS addr, p.username, p.display_name, p.profile_photo,
                   p.bio, p.selected_badge_id, p.social_proof_token_address, p.reservation_pool_address,
                   COALESCE(p.post_count, 0)::bigint AS post_count, COALESCE(p.blocked_count, wsg.blocked_count, 0)::bigint AS blocked_count,
                   COALESCE(p.followers_count, wsg.followers_count) AS followers_count, COALESCE(p.following_count, wsg.following_count) AS following_count
                FROM social_graph_relationships sgr
                {}
                WHERE (sgr.following_address = $1 OR sgr.following_address = $2)
                ORDER BY {}
                LIMIT $3 OFFSET $4"#,
                join_clause, order_clause
            )
        };
        let count_sql = if search_filter.is_some() {
            format!(
                r#"SELECT COUNT(*)::bigint AS cnt FROM social_graph_relationships sgr
                LEFT JOIN profiles p ON p.owner_address = sgr.follower_address
                WHERE (sgr.following_address = $1 OR sgr.following_address = $2){}"#,
                search_suffix
            )
        } else {
            format!(
                r#"SELECT COUNT(*)::bigint AS cnt FROM social_graph_relationships sgr
                WHERE (sgr.following_address = $1 OR sgr.following_address = $2)"#
            )
        };
        (data_sql, count_sql)
    } else {
        let data_sql = if search_filter.is_some() {
            format!(
                r#"SELECT sgr.follower_address AS addr, p.username, p.display_name, p.profile_photo,
                   p.bio, p.selected_badge_id, p.social_proof_token_address, p.reservation_pool_address,
                   COALESCE(p.post_count, 0)::bigint AS post_count, COALESCE(p.blocked_count, 0)::bigint AS blocked_count,
                   NULL::int AS followers_count, NULL::int AS following_count
                FROM social_graph_relationships sgr
                LEFT JOIN LATERAL (
                    SELECT username, display_name, profile_photo, bio, selected_badge_id,
                           social_proof_token_address, reservation_pool_address, post_count, blocked_count
                    FROM profiles WHERE owner_address = sgr.follower_address ORDER BY updated_at DESC LIMIT 1
                ) p ON true
                WHERE (sgr.following_address = $1 OR sgr.following_address = $2){}
                ORDER BY {}
                LIMIT $4 OFFSET $5"#,
                search_suffix, order_clause
            )
        } else {
            format!(
                r#"SELECT sgr.follower_address AS addr, p.username, p.display_name, p.profile_photo,
                   p.bio, p.selected_badge_id, p.social_proof_token_address, p.reservation_pool_address,
                   COALESCE(p.post_count, 0)::bigint AS post_count, COALESCE(p.blocked_count, 0)::bigint AS blocked_count,
                   NULL::int AS followers_count, NULL::int AS following_count
                FROM social_graph_relationships sgr
                LEFT JOIN LATERAL (
                    SELECT username, display_name, profile_photo, bio, selected_badge_id,
                           social_proof_token_address, reservation_pool_address, post_count, blocked_count
                    FROM profiles WHERE owner_address = sgr.follower_address ORDER BY updated_at DESC LIMIT 1
                ) p ON true
                WHERE (sgr.following_address = $1 OR sgr.following_address = $2)
                ORDER BY {}
                LIMIT $3 OFFSET $4"#,
                order_clause
            )
        };
        let count_sql = format!(
            r#"SELECT COUNT(*)::bigint AS cnt FROM social_graph_relationships sgr
            LEFT JOIN profiles p ON p.owner_address = sgr.follower_address
            WHERE (sgr.following_address = $1 OR sgr.following_address = $2){}"#,
            search_suffix
        );
        (data_sql, count_sql)
    };

    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        cnt: i64,
    }
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
        #[diesel(sql_type = Nullable<Text>)]
        bio: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        selected_badge_id: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        social_proof_token_address: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        reservation_pool_address: Option<String>,
        #[diesel(sql_type = Nullable<BigInt>)]
        post_count: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        blocked_count: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        followers_count: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        following_count: Option<i64>,
    }

    let total_count: i64 = if let Some(ref pat) = search_filter {
        diesel::sql_query(&count_sql)
            .bind::<Text, _>(ref1)
            .bind::<Text, _>(ref2)
            .bind::<Text, _>(pat)
            .get_result::<CountRow>(conn)
            .await?
            .cnt
    } else {
        diesel::sql_query(&count_sql)
            .bind::<Text, _>(ref1)
            .bind::<Text, _>(ref2)
            .get_result::<CountRow>(conn)
            .await?
            .cnt
    };

    let rows: Vec<Row> = if let Some(ref pat) = search_filter {
        diesel::sql_query(&data_sql)
            .bind::<Text, _>(ref1)
            .bind::<Text, _>(ref2)
            .bind::<Text, _>(pat)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(conn)
            .await?
    } else {
        diesel::sql_query(&data_sql)
            .bind::<Text, _>(ref1)
            .bind::<Text, _>(ref2)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(conn)
            .await?
    };

    let addresses: Vec<String> = rows.iter().map(|r| r.addr.clone()).collect();
    let viewer_ctx: HashMap<String, ViewerSocialContext> = if let Some(v) = viewer_address {
        let (v_pid, v_owner) = resolve_profile_address(conn, v).await?;
        batch_viewer_social_context(conn, &addresses, &v_pid, &v_owner).await?
    } else {
        HashMap::new()
    };

    let result: Vec<ProfileSummaryRow> = rows
        .into_iter()
        .map(|r| {
            let ctx = viewer_ctx.get(&r.addr).copied().unwrap_or_default();
            ProfileSummaryRow {
                owner_address: r.addr,
                username: r.username,
                display_name: r.display_name,
                profile_photo: r.profile_photo,
                bio: r.bio,
                selected_badge_id: r.selected_badge_id,
                social_proof_token_address: r.social_proof_token_address,
                reservation_pool_address: r.reservation_pool_address,
                followers_count: r.followers_count.map(|v| v as i32),
                following_count: r.following_count.map(|v| v as i32),
                post_count: r.post_count.map(|v| v as i32),
                blocked_count: r.blocked_count.map(|v| v as i32),
                is_following: viewer_address.map(|_| ctx.is_following),
                follows_viewer: viewer_address.map(|_| ctx.follows_viewer),
                blocked_by_viewer: viewer_address.map(|_| ctx.blocked_by_viewer),
                blocked_by_subject: viewer_address.map(|_| ctx.blocked_by_subject),
            }
        })
        .collect();

    metrics.requests_succeeded.inc();
    Ok((result, total_count))
}

pub(crate) async fn get_following(
    conn: &mut Connection<'_>,
    address: &str,
    viewer_address: Option<&str>,
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
        #[diesel(sql_type = Nullable<Text>)]
        bio: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        selected_badge_id: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        social_proof_token_address: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        reservation_pool_address: Option<String>,
        #[diesel(sql_type = Nullable<BigInt>)]
        post_count: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        blocked_count: Option<i64>,
    }
    let query = "
        SELECT sgr.following_address AS addr, p.username, p.display_name, p.profile_photo,
               p.bio, p.selected_badge_id, p.social_proof_token_address, p.reservation_pool_address,
               p.post_count, p.blocked_count
        FROM social_graph_relationships sgr
        LEFT JOIN LATERAL (
            SELECT username, display_name, profile_photo, bio, selected_badge_id,
                   social_proof_token_address, reservation_pool_address, post_count, blocked_count
            FROM profiles
            WHERE owner_address = sgr.following_address
            ORDER BY updated_at DESC
            LIMIT 1
        ) p ON true
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
    let addresses: Vec<String> = rows.iter().map(|r| r.addr.clone()).collect();
    let viewer_ctx: HashMap<String, ViewerSocialContext> = if let Some(v) = viewer_address {
        let (v_pid, v_owner) = resolve_profile_address(conn, v).await?;
        batch_viewer_social_context(conn, &addresses, &v_pid, &v_owner).await?
    } else {
        HashMap::new()
    };
    metrics.requests_succeeded.inc();
    Ok(rows
        .into_iter()
        .map(|r| {
            let ctx = viewer_ctx.get(&r.addr).copied().unwrap_or_default();
            ProfileSummaryRow {
                owner_address: r.addr,
                username: r.username,
                display_name: r.display_name,
                profile_photo: r.profile_photo,
                bio: r.bio,
                selected_badge_id: r.selected_badge_id,
                social_proof_token_address: r.social_proof_token_address,
                reservation_pool_address: r.reservation_pool_address,
                followers_count: None,
                following_count: None,
                post_count: r.post_count.map(|v| v as i32),
                blocked_count: r.blocked_count.map(|v| v as i32),
                is_following: viewer_address.map(|_| ctx.is_following),
                follows_viewer: viewer_address.map(|_| ctx.follows_viewer),
                blocked_by_viewer: viewer_address.map(|_| ctx.blocked_by_viewer),
                blocked_by_subject: viewer_address.map(|_| ctx.blocked_by_subject),
            }
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

pub(crate) async fn count_profile_platform_memberships(
    conn: &mut Connection<'_>,
    address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<i64> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        count: i64,
    }
    let total = diesel::sql_query(
        "SELECT COUNT(*)::bigint AS count
         FROM platform_memberships pm
         INNER JOIN platforms p ON pm.platform_id = p.platform_id
         WHERE pm.wallet_address = $1",
    )
    .bind::<Text, _>(address)
    .get_result::<CountRow>(conn)
    .await?
    .count;
    metrics.requests_succeeded.inc();
    Ok(total)
}

pub(crate) async fn get_profile_platform_memberships(
    conn: &mut Connection<'_>,
    address: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<ProfilePlatformMembershipRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Integer)]
        membership_id: i32,
        #[diesel(sql_type = Timestamp)]
        joined_at: NaiveDateTime,
        #[diesel(sql_type = Integer)]
        platform_db_id: i32,
        #[diesel(sql_type = Text)]
        platform_id: String,
        #[diesel(sql_type = Text)]
        name: String,
        #[diesel(sql_type = Text)]
        tagline: String,
        #[diesel(sql_type = Nullable<Text>)]
        description: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        logo: Option<String>,
        #[diesel(sql_type = Text)]
        developer_address: String,
        #[diesel(sql_type = Nullable<Text>)]
        terms_of_service: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        privacy_policy: Option<String>,
        #[diesel(sql_type = Nullable<Jsonb>)]
        platform_names: Option<JsonValue>,
        #[diesel(sql_type = Nullable<Jsonb>)]
        links: Option<JsonValue>,
        #[diesel(sql_type = SmallInt)]
        status: i16,
        #[diesel(sql_type = Nullable<Text>)]
        release_date: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        shutdown_date: Option<String>,
        #[diesel(sql_type = Timestamp)]
        platform_created_at: NaiveDateTime,
        #[diesel(sql_type = Timestamp)]
        platform_updated_at: NaiveDateTime,
        #[diesel(sql_type = Bool)]
        is_approved: bool,
        #[diesel(sql_type = Nullable<Timestamp>)]
        approval_changed_at: Option<NaiveDateTime>,
        #[diesel(sql_type = Nullable<Text>)]
        approved_by: Option<String>,
        #[diesel(sql_type = Nullable<Bool>)]
        wants_dao_governance: Option<bool>,
        #[diesel(sql_type = Nullable<Text>)]
        governance_registry_id: Option<String>,
        #[diesel(sql_type = Nullable<BigInt>)]
        delegate_count: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        delegate_term_epochs: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        max_votes_per_user: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        proposal_submission_cost: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        quadratic_base_cost: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        quorum_votes: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        voting_period_epochs: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        treasury: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        version: Option<i64>,
        #[diesel(sql_type = Text)]
        primary_category: String,
        #[diesel(sql_type = Nullable<Text>)]
        secondary_category: Option<String>,
        #[diesel(sql_type = Nullable<Timestamp>)]
        deleted_at: Option<NaiveDateTime>,
        #[diesel(sql_type = BigInt)]
        moderator_count: i64,
        #[diesel(sql_type = BigInt)]
        blocked_profiles_count: i64,
    }
    let query = "
        SELECT pm.id AS membership_id,
               pm.joined_at,
               p.id AS platform_db_id,
               p.platform_id,
               p.name,
               p.tagline,
               p.description,
               p.logo,
               p.developer_address,
               p.terms_of_service,
               p.privacy_policy,
               p.platforms AS platform_names,
               p.links,
               p.status,
               p.release_date,
               p.shutdown_date,
               p.created_at AS platform_created_at,
               p.updated_at AS platform_updated_at,
               p.is_approved,
               p.approval_changed_at,
               p.approved_by,
               p.wants_dao_governance,
               p.governance_registry_id,
               p.delegate_count,
               p.delegate_term_epochs,
               p.max_votes_per_user,
               p.proposal_submission_cost,
               p.quadratic_base_cost,
               p.quorum_votes,
               p.voting_period_epochs,
               p.treasury,
               p.version,
               p.primary_category,
               p.secondary_category,
               p.deleted_at,
               (SELECT COUNT(*)::bigint FROM platform_moderators m WHERE m.platform_id = p.platform_id)
                   AS moderator_count,
               (SELECT COUNT(*)::bigint FROM platform_blocked_profiles b WHERE b.platform_id = p.platform_id)
                   AS blocked_profiles_count
        FROM platform_memberships pm
        INNER JOIN platforms p ON pm.platform_id = p.platform_id
        WHERE pm.wallet_address = $1
        ORDER BY pm.joined_at DESC
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
        .map(|r| ProfilePlatformMembershipRow {
            membership_id: r.membership_id,
            joined_at: r.joined_at,
            platform_db_id: r.platform_db_id,
            platform_id: r.platform_id,
            name: r.name,
            tagline: r.tagline,
            description: r.description,
            logo: r.logo,
            developer_address: r.developer_address,
            terms_of_service: r.terms_of_service,
            privacy_policy: r.privacy_policy,
            platform_names: r.platform_names,
            links: r.links,
            status: r.status,
            release_date: r.release_date,
            shutdown_date: r.shutdown_date,
            platform_created_at: r.platform_created_at,
            platform_updated_at: r.platform_updated_at,
            is_approved: r.is_approved,
            approval_changed_at: r.approval_changed_at,
            approved_by: r.approved_by,
            wants_dao_governance: r.wants_dao_governance,
            governance_registry_id: r.governance_registry_id,
            delegate_count: r.delegate_count,
            delegate_term_epochs: r.delegate_term_epochs,
            max_votes_per_user: r.max_votes_per_user,
            proposal_submission_cost: r.proposal_submission_cost,
            quadratic_base_cost: r.quadratic_base_cost,
            quorum_votes: r.quorum_votes,
            voting_period_epochs: r.voting_period_epochs,
            treasury: r.treasury,
            version: r.version,
            primary_category: r.primary_category,
            secondary_category: r.secondary_category,
            deleted_at: r.deleted_at,
            moderator_count: r.moderator_count,
            blocked_profiles_count: r.blocked_profiles_count,
        })
        .collect())
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
