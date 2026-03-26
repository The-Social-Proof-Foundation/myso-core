// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use diesel::sql_types::{Array, BigInt, Date, Integer, Nullable, SmallInt, Text, Timestamp};
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::PgTextExpressionMethods;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::{
    blocked_events, blocked_profiles, platform_blocked_profiles, profile_events, profiles,
    social_graph_relationships, spt_exchange_config, wallet_social_graph,
};

use crate::error::SocialError;
use crate::reader::types::PostBasicRow;
use crate::reader::types::{
    BlockedEventRow, BlockedPlatformRow, BlockedProfileRow, ChartSummary, DailyStatsPoint,
    DateRange, FollowDetail, FollowStatsRow, FollowsQuery, PaginationInfo, PlatformMembershipRow,
    ProfileBadgeRow, ProfileEventRow, ProfilePlatformEventRow, ReservationPoolInfo,
    ReservationStatus, SelectedBadgeInfo, SocialGraphChartData, SocialGraphChartQuery,
    SocialProofTokenInfo, UniversalUserResult,
};
use myso_pg_db::Db;

pub(crate) async fn enrich_users_with_universal_data(
    conn: &mut diesel_async::AsyncPgConnection,
    wallet_addresses: Vec<String>,
) -> Result<HashMap<String, UniversalUserResult>, SocialError> {
    if wallet_addresses.is_empty() {
        return Ok(HashMap::new());
    }
    let query = diesel::sql_query(
        r#"
        WITH latest_profiles AS (
            SELECT DISTINCT ON (COALESCE(profile_id, owner_address)) *
            FROM profiles
            WHERE owner_address = ANY($1::TEXT[])
            ORDER BY COALESCE(profile_id, owner_address), updated_at DESC
        )
        SELECT
            p.owner_address,
            p.username,
            p.display_name,
            p.profile_photo,
            p.social_proof_token_address,
            pb.badge_id as badge_id,
            pb.badge_name,
            pb.badge_icon_url,
            pb.badge_media_url,
            pb.platform_id as badge_platform_id,
            pb.badge_type,
            spt.pool_id as spt_pool_id,
            rp.pool_id as reservation_pool_id,
            rp.total_reserved,
            rp.required_threshold,
            rp.status as reservation_status
        FROM latest_profiles p
        LEFT JOIN profile_badges pb ON
            p.selected_badge_id IS NOT NULL AND
            pb.badge_id = p.selected_badge_id AND
            pb.profile_id = p.profile_id AND
            pb.revoked = false
        LEFT JOIN LATERAL (
            SELECT * FROM spt_pools spt_row
            WHERE spt_row.associated_id = ('profile_' || p.owner_address)
               OR (p.profile_id IS NOT NULL AND spt_row.associated_id = p.profile_id)
            ORDER BY spt_row.time DESC
            LIMIT 1
        ) spt ON true
        LEFT JOIN LATERAL (
            SELECT * FROM spt_reservation_pools sr
            WHERE (p.reservation_pool_address IS NOT NULL AND sr.pool_id = p.reservation_pool_address)
               OR sr.associated_id = ('profile_' || p.owner_address)
               OR (p.profile_id IS NOT NULL AND sr.associated_id = p.profile_id)
            ORDER BY
                (p.reservation_pool_address IS NOT NULL AND sr.pool_id IS NOT DISTINCT FROM p.reservation_pool_address) DESC,
                sr.time DESC
            LIMIT 1
        ) rp ON true
        WHERE p.owner_address = ANY($1::TEXT[])
        "#,
    )
    .bind::<Array<Text>, _>(&wallet_addresses);

    #[derive(QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct EnrichmentRow {
        #[diesel(sql_type = Text)]
        owner_address: String,
        #[diesel(sql_type = Text)]
        username: String,
        #[diesel(sql_type = Nullable<Text>)]
        display_name: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        profile_photo: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        social_proof_token_address: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        badge_id: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        badge_name: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        badge_icon_url: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        badge_media_url: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        badge_platform_id: Option<String>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        badge_type: Option<i16>,
        #[diesel(sql_type = Nullable<Text>)]
        spt_pool_id: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        reservation_pool_id: Option<String>,
        #[diesel(sql_type = Nullable<BigInt>)]
        total_reserved: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        required_threshold: Option<i64>,
        #[diesel(sql_type = Nullable<Text>)]
        reservation_status: Option<String>,
    }

    let rows: Vec<EnrichmentRow> = query.load::<EnrichmentRow>(conn).await?;

    let mut result = HashMap::new();
    for row in rows {
        let selected_badge =
            if let (Some(badge_id), Some(badge_name), Some(platform_id), Some(badge_type)) = (
                row.badge_id.clone(),
                row.badge_name.clone(),
                row.badge_platform_id.clone(),
                row.badge_type,
            ) {
                Some(SelectedBadgeInfo {
                    badge_id,
                    badge_name,
                    badge_icon_url: row.badge_icon_url.clone(),
                    badge_media_url: row.badge_media_url.clone(),
                    platform_id,
                    badge_type,
                })
            } else {
                None
            };

        let reservation_status = match row.reservation_status.as_deref() {
            Some("active") => ReservationStatus::Active,
            Some("threshold_met") => ReservationStatus::ThresholdMet,
            Some("inactive") => ReservationStatus::Inactive,
            _ => ReservationStatus::None,
        };

        let reservation_percentage = if let (Some(total_reserved), Some(required_threshold)) =
            (row.total_reserved, row.required_threshold)
        {
            if required_threshold > 0 {
                (total_reserved as f64 / required_threshold as f64 * 100.0)
                    .min(100.0)
                    .max(0.0)
            } else {
                0.0
            }
        } else {
            0.0
        };

        let spt_pool_id = row.spt_pool_id.clone();
        let reservation_pool_id = row.reservation_pool_id.clone();
        let social_proof_token_address = row.social_proof_token_address.clone();
        let is_active = spt_pool_id.is_some();

        let social_proof_token = if spt_pool_id.is_some()
            || reservation_pool_id.is_some()
            || social_proof_token_address.is_some()
        {
            Some(SocialProofTokenInfo {
                pool_id: spt_pool_id,
                token_address: social_proof_token_address,
                is_active,
                reservation_pool_id,
                reservation_percentage,
                reservation_status: reservation_status.clone(),
                total_reserved: row.total_reserved.unwrap_or(0),
                required_threshold: row.required_threshold.unwrap_or(0),
            })
        } else {
            None
        };

        let user_result = UniversalUserResult {
            owner_address: row.owner_address.clone(),
            wallet_address: row.owner_address.clone(),
            username: Some(row.username),
            fullname: row.display_name,
            profile_photo: row.profile_photo,
            social_proof_token,
            selected_badge,
        };
        result.insert(row.owner_address, user_result);
    }

    for wallet_address in wallet_addresses {
        if !result.contains_key(&wallet_address) {
            result.insert(
                wallet_address.clone(),
                UniversalUserResult {
                    owner_address: wallet_address.clone(),
                    wallet_address: wallet_address.clone(),
                    username: None,
                    fullname: None,
                    profile_photo: None,
                    social_proof_token: None,
                    selected_badge: None,
                },
            );
        }
    }

    Ok(result)
}

async fn resolve_profile_input(
    conn: &mut diesel_async::AsyncPgConnection,
    input: &str,
) -> Result<(Option<String>, String), diesel::result::Error> {
    let normalized = input.to_lowercase();
    let profile_info = profiles::table
        .filter(
            profiles::owner_address
                .ilike(&normalized)
                .or(profiles::profile_id.ilike(&normalized))
                .or(profiles::username.ilike(&normalized)),
        )
        .select((profiles::profile_id, profiles::owner_address))
        .first::<(Option<String>, String)>(conn)
        .await;

    match profile_info {
        Ok((profile_id, owner_address)) => Ok((profile_id, owner_address)),
        Err(diesel::result::Error::NotFound) => Ok((None, input.to_string())),
        Err(e) => Err(e),
    }
}

async fn get_reservation_pool_info_for_profiles(
    conn: &mut diesel_async::AsyncPgConnection,
    wallet_addresses: Vec<String>,
) -> Result<HashMap<String, ReservationPoolInfo>, SocialError> {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;

    if wallet_addresses.is_empty() {
        return Ok(HashMap::new());
    }

    let profile_threshold: i64 = spt_exchange_config::table
        .order_by(spt_exchange_config::time.desc())
        .select(spt_exchange_config::profile_threshold)
        .first(conn)
        .await
        .unwrap_or(10_000_000_000_000);

    let associated_ids: Vec<String> = wallet_addresses
        .iter()
        .map(|addr| format!("profile_{}", addr))
        .collect();

    let query = diesel::sql_query(
        r#"
        WITH latest_pools AS (
            SELECT DISTINCT ON (associated_id)
                associated_id, pool_id, total_reserved, required_threshold, status
            FROM spt_reservation_pools
            WHERE associated_id = ANY($1::TEXT[])
            ORDER BY associated_id, time DESC
        )
        SELECT associated_id, pool_id, total_reserved, required_threshold, status
        FROM latest_pools
        "#,
    )
    .bind::<Array<Text>, _>(&associated_ids);

    #[derive(QueryableByName)]
    struct PoolRow {
        #[diesel(sql_type = Text)]
        associated_id: String,
        #[diesel(sql_type = Text)]
        pool_id: String,
        #[diesel(sql_type = BigInt)]
        total_reserved: i64,
        #[diesel(sql_type = BigInt)]
        required_threshold: i64,
        #[diesel(sql_type = Text)]
        status: String,
    }

    let pools: Vec<PoolRow> = query.load::<PoolRow>(conn).await?;
    let mut result = HashMap::new();
    for pool in pools {
        if let Some(owner_address) = pool.associated_id.strip_prefix("profile_") {
            let claimed_percentage = if profile_threshold > 0 {
                (pool.total_reserved as f64 / profile_threshold as f64 * 100.0)
                    .min(100.0)
                    .max(0.0)
            } else {
                0.0
            };
            let is_active =
                pool.status == "active" && pool.total_reserved < pool.required_threshold;
            result.insert(
                owner_address.to_string(),
                ReservationPoolInfo {
                    claimed_percentage,
                    is_active,
                    total_reserved: pool.total_reserved,
                    required_threshold: pool.required_threshold,
                    pool_id: Some(pool.pool_id),
                },
            );
        }
    }
    Ok(result)
}

pub(crate) async fn get_profile_posts(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PostBasicRow>, SocialError> {
    let mut conn = db.connect().await?;
    let profile_id_opt: Option<String> = profiles::table
        .filter(profiles::owner_address.eq(address))
        .select(profiles::profile_id)
        .first::<Option<String>>(&mut conn)
        .await
        .optional()?
        .flatten();
    let query = "
        SELECT post_id, owner, profile_id, content, post_type, created_at, deleted_at,
               reaction_count, comment_count, repost_count, tips_received
        FROM posts
        WHERE (owner = $1 OR ($2::text IS NOT NULL AND profile_id = $2))
          AND deleted_at IS NULL
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<Nullable<Text>, _>(profile_id_opt.as_deref())
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PostBasicRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_profile_events(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileEventRow>, SocialError> {
    let mut conn = db.connect().await?;
    let profile_id_opt: Option<String> = profiles::table
        .filter(profiles::owner_address.eq(address))
        .select(profiles::profile_id)
        .first::<Option<String>>(&mut conn)
        .await
        .optional()?
        .flatten();
    let profile_ids: Vec<String> = if let Some(pid) = &profile_id_opt {
        vec![address.to_string(), pid.clone()]
    } else {
        vec![address.to_string()]
    };
    let results = profile_events::table
        .filter(profile_events::profile_id.eq_any(&profile_ids))
        .order_by(profile_events::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            profile_events::event_type,
            profile_events::profile_id,
            profile_events::event_data,
            profile_events::event_id,
            profile_events::created_at,
        ))
        .load::<(
            String,
            String,
            serde_json::Value,
            Option<String>,
            chrono::NaiveDateTime,
        )>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(event_type, profile_id, event_data, event_id, created_at)| ProfileEventRow {
                event_type,
                profile_id,
                event_data,
                event_id,
                created_at,
            },
        )
        .collect())
}

pub(crate) async fn get_profile_platform_memberships(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PlatformMembershipRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT p.platform_id, p.name, p.is_approved, pm.joined_at
        FROM platform_memberships pm
        INNER JOIN platforms p ON pm.platform_id = p.platform_id
        WHERE pm.wallet_address = $1
        ORDER BY pm.joined_at DESC
        LIMIT $2 OFFSET $3
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        platform_id: String,
        #[diesel(sql_type = Text)]
        name: String,
        #[diesel(sql_type = diesel::sql_types::Bool)]
        is_approved: bool,
        #[diesel(sql_type = Timestamp)]
        joined_at: chrono::NaiveDateTime,
    }
    let results = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(|r| PlatformMembershipRow {
            platform_id: r.platform_id,
            name: r.name,
            is_approved: r.is_approved,
            joined_at: r.joined_at,
        })
        .collect())
}

pub(crate) async fn get_profile_platform_events(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<(Vec<ProfilePlatformEventRow>, i64), SocialError> {
    let mut conn = db.connect().await?;
    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        count: i64,
    }
    let total: i64 = diesel::sql_query(
        "SELECT COUNT(*)::bigint as count FROM platform_events
         WHERE event_type IN ('UserJoinedPlatform', 'UserLeftPlatform')
         AND event_data->>'wallet_address' = $1",
    )
    .bind::<Text, _>(address)
    .get_result::<CountRow>(&mut conn)
    .await?
    .count;
    let query = "
        SELECT event_type, platform_id, created_at, event_id, event_data
        FROM platform_events
        WHERE event_type IN ('UserJoinedPlatform', 'UserLeftPlatform')
        AND event_data->>'wallet_address' = $1
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        event_type: String,
        #[diesel(sql_type = Text)]
        platform_id: String,
        #[diesel(sql_type = Timestamp)]
        created_at: chrono::NaiveDateTime,
        #[diesel(sql_type = Nullable<Text>)]
        event_id: Option<String>,
        #[diesel(sql_type = diesel::sql_types::Jsonb)]
        event_data: serde_json::Value,
    }
    let results = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(&mut conn)
        .await?;
    let events = results
        .into_iter()
        .map(|r| ProfilePlatformEventRow {
            event_type: r.event_type,
            platform_id: r.platform_id,
            created_at: r.created_at,
            event_id: r.event_id,
            event_data: r.event_data,
        })
        .collect();
    Ok((events, total))
}

pub(crate) async fn get_blocking_history(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<BlockedEventRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = blocked_events::table
        .filter(blocked_events::blocker_address.eq(address))
        .order_by(blocked_events::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            blocked_events::event_type,
            blocked_events::blocked_address,
            blocked_events::processed_at,
            blocked_events::event_id,
        ))
        .load::<(
            String,
            Option<String>,
            chrono::NaiveDateTime,
            Option<String>,
        )>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(event_type, blocked_address, processed_at, event_id)| BlockedEventRow {
                event_type,
                blocked_address,
                processed_at,
                event_id,
            },
        )
        .collect())
}

pub(crate) async fn get_profile_badges(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileBadgeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let profile_id_opt: Option<String> = profiles::table
        .filter(profiles::owner_address.eq(address))
        .select(profiles::profile_id)
        .first::<Option<String>>(&mut conn)
        .await
        .optional()?
        .flatten();
    let id2 = profile_id_opt.as_deref().unwrap_or(address).to_string();
    let query = "
        SELECT pb.badge_id, pb.badge_name, pb.badge_description, pb.badge_media_url,
               pb.badge_icon_url, pb.platform_id, pb.assigned_by, pb.assigned_at,
               pb.revoked, pb.revoked_at, pb.revoked_by, pb.badge_type
        FROM (
            SELECT DISTINCT ON (badge_id) *
            FROM profile_badges
            WHERE profile_id = $1 OR profile_id = $2
            ORDER BY badge_id, time DESC
        ) pb
        WHERE pb.revoked = false
        ORDER BY pb.assigned_at DESC
        LIMIT $3 OFFSET $4
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<Text, _>(&id2)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProfileBadgeRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_following(
    db: &Db,
    address: &str,
    query: &FollowsQuery,
) -> Result<(Vec<FollowDetail>, PaginationInfo), SocialError> {
    let mut conn = db.connect().await?;
    let limit = query.limit();
    let offset = query.offset();
    let page = query.page.unwrap_or(1).max(1);

    let (resolved_profile_id, resolved_owner_address) = match profiles::table
        .filter(
            profiles::owner_address
                .ilike(address)
                .or(profiles::profile_id.ilike(address)),
        )
        .select((profiles::profile_id, profiles::owner_address))
        .first::<(Option<String>, String)>(&mut conn)
        .await
    {
        Ok((pid, addr)) => (pid, addr),
        Err(diesel::result::Error::NotFound) => {
            let wallet_exists = wallet_social_graph::table
                .filter(wallet_social_graph::wallet_address.eq(address))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0)
                > 0;
            let has_relationships = social_graph_relationships::table
                .filter(social_graph_relationships::follower_address.eq(address))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0)
                > 0;
            if !wallet_exists && !has_relationships {
                return Ok((
                    vec![],
                    PaginationInfo {
                        total: 0,
                        limit,
                        offset,
                        page,
                        total_pages: 0,
                    },
                ));
            }
            (None, address.to_string())
        }
        Err(e) => return Err(e.into()),
    };

    let search_filter = query
        .search
        .as_ref()
        .filter(|t| !t.trim().is_empty())
        .map(|t| format!("%{}%", t.trim()));
    let search_suffix = search_filter
        .as_ref()
        .map(|_| " AND (p.username ILIKE $3 OR p.display_name ILIKE $3 OR sgr.following_address ILIKE $3)")
        .unwrap_or("");
    let order_sql = match query.sort.as_ref().map(|s| s.to_lowercase()).as_deref() {
        Some("earliest") => "sgr.created_at ASC",
        Some("alphabetical") => "COALESCE(p.username, sgr.following_address) ASC",
        Some("followers_count") => "COALESCE(p.followers_count, 0) DESC",
        _ => "sgr.created_at DESC",
    };

    let (data_sql, count_sql) = if search_filter.is_some() {
        (
            format!(
                r#"SELECT p.id, p.profile_id, sgr.following_address AS addr,
                   p.username, p.display_name, p.profile_photo
                FROM social_graph_relationships sgr
                LEFT JOIN profiles p ON p.owner_address = sgr.following_address
                WHERE (sgr.follower_address = $1 OR sgr.follower_address = $2){}
                ORDER BY {}
                LIMIT $4 OFFSET $5"#,
                search_suffix, order_sql,
            ),
            format!(
                r#"SELECT COUNT(*)::bigint as cnt
                FROM social_graph_relationships sgr
                LEFT JOIN profiles p ON p.owner_address = sgr.following_address
                WHERE (sgr.follower_address = $1 OR sgr.follower_address = $2){}"#,
                search_suffix,
            ),
        )
    } else {
        (
            format!(
                r#"SELECT p.id, p.profile_id, sgr.following_address AS addr,
                   p.username, p.display_name, p.profile_photo
                FROM social_graph_relationships sgr
                LEFT JOIN profiles p ON p.owner_address = sgr.following_address
                WHERE (sgr.follower_address = $1 OR sgr.follower_address = $2)
                ORDER BY {}
                LIMIT $3 OFFSET $4"#,
                order_sql,
            ),
            format!(
                r#"SELECT COUNT(*)::bigint as cnt
                FROM social_graph_relationships sgr
                WHERE (sgr.follower_address = $1 OR sgr.follower_address = $2)"#,
            ),
        )
    };

    #[derive(QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        cnt: i64,
    }

    #[derive(QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct FollowRow {
        #[diesel(sql_type = Nullable<Integer>)]
        id: Option<i32>,
        #[diesel(sql_type = Nullable<Text>)]
        profile_id: Option<String>,
        #[diesel(sql_type = Text)]
        addr: String,
        #[diesel(sql_type = Nullable<Text>)]
        username: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        display_name: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        profile_photo: Option<String>,
    }

    let follows: Vec<FollowRow> = if let Some(ref pat) = search_filter {
        diesel::sql_query(&data_sql)
            .bind::<Text, _>(&resolved_owner_address)
            .bind::<Text, _>(
                resolved_profile_id
                    .as_ref()
                    .unwrap_or(&resolved_owner_address),
            )
            .bind::<Text, _>(pat)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(&mut conn)
            .await?
    } else {
        diesel::sql_query(&data_sql)
            .bind::<Text, _>(&resolved_owner_address)
            .bind::<Text, _>(
                resolved_profile_id
                    .as_ref()
                    .unwrap_or(&resolved_owner_address),
            )
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(&mut conn)
            .await?
    };

    let total_count: i64 = if let Some(ref pat) = search_filter {
        let row: CountRow = diesel::sql_query(&count_sql)
            .bind::<Text, _>(&resolved_owner_address)
            .bind::<Text, _>(
                resolved_profile_id
                    .as_ref()
                    .unwrap_or(&resolved_owner_address),
            )
            .bind::<Text, _>(pat)
            .get_result(&mut conn)
            .await?;
        row.cnt
    } else {
        let row: CountRow = diesel::sql_query(&count_sql)
            .bind::<Text, _>(&resolved_owner_address)
            .bind::<Text, _>(
                resolved_profile_id
                    .as_ref()
                    .unwrap_or(&resolved_owner_address),
            )
            .get_result(&mut conn)
            .await?;
        row.cnt
    };

    let follows: Vec<(
        Option<i32>,
        Option<String>,
        String,
        Option<String>,
        Option<String>,
        Option<String>,
    )> = follows
        .into_iter()
        .map(|r| {
            (
                r.id,
                r.profile_id,
                r.addr,
                r.username,
                r.display_name,
                r.profile_photo,
            )
        })
        .collect();
    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    let wallet_addresses: Vec<String> = follows
        .iter()
        .map(|(_, _, addr, _, _, _)| addr.clone())
        .collect();
    let reservation_info =
        get_reservation_pool_info_for_profiles(&mut conn, wallet_addresses).await?;

    let (viewer_profile_id, viewer_wallet_address) = if let Some(ref vid) = query.viewer_id {
        match resolve_profile_input(&mut conn, vid).await {
            Ok((pid, addr)) => (pid, addr),
            Err(_) => (None, String::new()),
        }
    } else {
        (None, String::new())
    };

    let mut follows_detail = Vec::new();
    for (id_opt, followed_profile_id, owner_address, username_opt, display_name, profile_photo) in
        follows
    {
        let id = id_opt.unwrap_or(0);
        let (is_following, follows_back) = if !viewer_wallet_address.is_empty() {
            let viewer_follows_this = social_graph_relationships::table
                .filter(
                    social_graph_relationships::follower_address
                        .eq(&viewer_wallet_address)
                        .or(social_graph_relationships::follower_address
                            .eq(viewer_profile_id.as_ref().unwrap_or(&viewer_wallet_address)))
                        .and(
                            social_graph_relationships::following_address
                                .eq(followed_profile_id.as_ref().unwrap_or(&owner_address))
                                .or(social_graph_relationships::following_address
                                    .eq(&owner_address)),
                        ),
                )
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0)
                > 0;

            let this_follows_viewer = social_graph_relationships::table
                .filter(
                    (social_graph_relationships::follower_address
                        .eq(followed_profile_id.as_ref().unwrap_or(&owner_address))
                        .or(social_graph_relationships::follower_address.eq(&owner_address)))
                    .and(
                        social_graph_relationships::following_address
                            .eq(&viewer_wallet_address)
                            .or(social_graph_relationships::following_address
                                .eq(viewer_profile_id.as_ref().unwrap_or(&viewer_wallet_address))),
                    ),
                )
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0)
                > 0;

            (viewer_follows_this, this_follows_viewer)
        } else {
            (false, false)
        };

        let res_info = if id > 0 {
            reservation_info.get(&owner_address).cloned()
        } else {
            None
        };

        follows_detail.push(FollowDetail {
            id,
            profile_id: followed_profile_id,
            owner_address,
            username: username_opt.unwrap_or_default(),
            display_name,
            profile_photo,
            follows_back,
            is_following,
            reservation_pool: res_info,
        });
    }

    Ok((
        follows_detail,
        PaginationInfo {
            total: total_count,
            limit,
            offset,
            page,
            total_pages,
        },
    ))
}

pub(crate) async fn get_followers(
    db: &Db,
    address: &str,
    query: &FollowsQuery,
) -> Result<(Vec<FollowDetail>, PaginationInfo), SocialError> {
    let mut conn = db.connect().await?;
    let limit = query.limit();
    let offset = query.offset();
    let page = query.page.unwrap_or(1).max(1);

    let (resolved_profile_id, resolved_owner_address) = match profiles::table
        .filter(
            profiles::owner_address
                .ilike(address)
                .or(profiles::profile_id.ilike(address)),
        )
        .select((profiles::profile_id, profiles::owner_address))
        .first::<(Option<String>, String)>(&mut conn)
        .await
    {
        Ok((pid, addr)) => (pid, addr),
        Err(diesel::result::Error::NotFound) => {
            let wallet_exists = wallet_social_graph::table
                .filter(wallet_social_graph::wallet_address.eq(address))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0)
                > 0;
            let has_relationships = social_graph_relationships::table
                .filter(social_graph_relationships::following_address.eq(address))
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0)
                > 0;
            if !wallet_exists && !has_relationships {
                return Ok((
                    vec![],
                    PaginationInfo {
                        total: 0,
                        limit,
                        offset,
                        page,
                        total_pages: 0,
                    },
                ));
            }
            (None, address.to_string())
        }
        Err(e) => return Err(e.into()),
    };

    let search_filter = query
        .search
        .as_ref()
        .filter(|t| !t.trim().is_empty())
        .map(|t| format!("%{}%", t.trim()));
    let search_suffix = search_filter
        .as_ref()
        .map(|_| {
            " AND (p.username ILIKE $3 OR p.display_name ILIKE $3 OR sgr.follower_address ILIKE $3)"
        })
        .unwrap_or("");
    let order_sql = match query.sort.as_ref().map(|s| s.to_lowercase()).as_deref() {
        Some("earliest") => "sgr.created_at ASC",
        Some("alphabetical") => "COALESCE(p.username, sgr.follower_address) ASC",
        Some("followers_count") => "COALESCE(p.followers_count, 0) DESC",
        _ => "sgr.created_at DESC",
    };

    let (data_sql, count_sql) = if search_filter.is_some() {
        (
            format!(
                r#"SELECT p.id, p.profile_id, sgr.follower_address AS addr,
                   p.username, p.display_name, p.profile_photo
                FROM social_graph_relationships sgr
                LEFT JOIN profiles p ON p.owner_address = sgr.follower_address
                WHERE (sgr.following_address = $1 OR sgr.following_address = $2){}
                ORDER BY {}
                LIMIT $4 OFFSET $5"#,
                search_suffix, order_sql,
            ),
            format!(
                r#"SELECT COUNT(*)::bigint as cnt
                FROM social_graph_relationships sgr
                LEFT JOIN profiles p ON p.owner_address = sgr.follower_address
                WHERE (sgr.following_address = $1 OR sgr.following_address = $2){}"#,
                search_suffix,
            ),
        )
    } else {
        (
            format!(
                r#"SELECT p.id, p.profile_id, sgr.follower_address AS addr,
                   p.username, p.display_name, p.profile_photo
                FROM social_graph_relationships sgr
                LEFT JOIN profiles p ON p.owner_address = sgr.follower_address
                WHERE (sgr.following_address = $1 OR sgr.following_address = $2)
                ORDER BY {}
                LIMIT $3 OFFSET $4"#,
                order_sql,
            ),
            format!(
                r#"SELECT COUNT(*)::bigint as cnt
                FROM social_graph_relationships sgr
                WHERE (sgr.following_address = $1 OR sgr.following_address = $2)"#,
            ),
        )
    };

    #[derive(QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct FollowerCountRow {
        #[diesel(sql_type = BigInt)]
        cnt: i64,
    }

    #[derive(QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct FollowerRow {
        #[diesel(sql_type = Nullable<Integer>)]
        id: Option<i32>,
        #[diesel(sql_type = Nullable<Text>)]
        profile_id: Option<String>,
        #[diesel(sql_type = Text)]
        addr: String,
        #[diesel(sql_type = Nullable<Text>)]
        username: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        display_name: Option<String>,
        #[diesel(sql_type = Nullable<Text>)]
        profile_photo: Option<String>,
    }

    let follows: Vec<FollowerRow> = if let Some(ref pat) = search_filter {
        diesel::sql_query(&data_sql)
            .bind::<Text, _>(&resolved_owner_address)
            .bind::<Text, _>(
                resolved_profile_id
                    .as_ref()
                    .unwrap_or(&resolved_owner_address),
            )
            .bind::<Text, _>(pat)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(&mut conn)
            .await?
    } else {
        diesel::sql_query(&data_sql)
            .bind::<Text, _>(&resolved_owner_address)
            .bind::<Text, _>(
                resolved_profile_id
                    .as_ref()
                    .unwrap_or(&resolved_owner_address),
            )
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load(&mut conn)
            .await?
    };

    let total_count: i64 = if let Some(ref pat) = search_filter {
        let row: FollowerCountRow = diesel::sql_query(&count_sql)
            .bind::<Text, _>(&resolved_owner_address)
            .bind::<Text, _>(
                resolved_profile_id
                    .as_ref()
                    .unwrap_or(&resolved_owner_address),
            )
            .bind::<Text, _>(pat)
            .get_result(&mut conn)
            .await?;
        row.cnt
    } else {
        let row: FollowerCountRow = diesel::sql_query(&count_sql)
            .bind::<Text, _>(&resolved_owner_address)
            .bind::<Text, _>(
                resolved_profile_id
                    .as_ref()
                    .unwrap_or(&resolved_owner_address),
            )
            .get_result(&mut conn)
            .await?;
        row.cnt
    };

    let follows: Vec<(
        Option<i32>,
        Option<String>,
        String,
        Option<String>,
        Option<String>,
        Option<String>,
    )> = follows
        .into_iter()
        .map(|r| {
            (
                r.id,
                r.profile_id,
                r.addr,
                r.username,
                r.display_name,
                r.profile_photo,
            )
        })
        .collect();
    let total_pages = (total_count as f64 / limit as f64).ceil() as i64;

    let wallet_addresses: Vec<String> = follows
        .iter()
        .map(|(_, _, addr, _, _, _)| addr.clone())
        .collect();
    let reservation_info =
        get_reservation_pool_info_for_profiles(&mut conn, wallet_addresses).await?;

    let (viewer_profile_id, viewer_wallet_address) = if let Some(ref vid) = query.viewer_id {
        match resolve_profile_input(&mut conn, vid).await {
            Ok((pid, addr)) => (pid, addr),
            Err(_) => (None, String::new()),
        }
    } else {
        (None, String::new())
    };

    let mut follows_detail = Vec::new();
    for (id_opt, follower_profile_id, owner_address, username_opt, display_name, profile_photo) in
        follows
    {
        let id = id_opt.unwrap_or(0);
        let (is_following, follows_back) = if !viewer_wallet_address.is_empty() {
            let viewer_follows_this = social_graph_relationships::table
                .filter(
                    social_graph_relationships::follower_address
                        .eq(&viewer_wallet_address)
                        .or(social_graph_relationships::follower_address
                            .eq(viewer_profile_id.as_ref().unwrap_or(&viewer_wallet_address)))
                        .and(
                            social_graph_relationships::following_address
                                .eq(follower_profile_id.as_ref().unwrap_or(&owner_address))
                                .or(social_graph_relationships::following_address
                                    .eq(&owner_address)),
                        ),
                )
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0)
                > 0;

            let this_follows_viewer = social_graph_relationships::table
                .filter(
                    (social_graph_relationships::follower_address
                        .eq(follower_profile_id.as_ref().unwrap_or(&owner_address))
                        .or(social_graph_relationships::follower_address.eq(&owner_address)))
                    .and(
                        social_graph_relationships::following_address
                            .eq(&viewer_wallet_address)
                            .or(social_graph_relationships::following_address
                                .eq(viewer_profile_id.as_ref().unwrap_or(&viewer_wallet_address))),
                    ),
                )
                .count()
                .get_result::<i64>(&mut conn)
                .await
                .unwrap_or(0)
                > 0;

            (viewer_follows_this, this_follows_viewer)
        } else {
            (false, false)
        };

        let res_info = if id > 0 {
            reservation_info.get(&owner_address).cloned()
        } else {
            None
        };

        follows_detail.push(FollowDetail {
            id,
            profile_id: follower_profile_id,
            owner_address,
            username: username_opt.unwrap_or_default(),
            display_name,
            profile_photo,
            follows_back,
            is_following,
            reservation_pool: res_info,
        });
    }

    Ok((
        follows_detail,
        PaginationInfo {
            total: total_count,
            limit,
            offset,
            page,
            total_pages,
        },
    ))
}

pub(crate) async fn get_social_stats(
    db: &Db,
    address: &str,
) -> Result<FollowStatsRow, SocialError> {
    let mut conn = db.connect().await?;
    let (resolved_profile_id, resolved_owner_address) =
        resolve_profile_input(&mut conn, address).await?;

    let profile = profiles::table
        .filter(profiles::owner_address.eq(&resolved_owner_address))
        .select((
            profiles::profile_id,
            profiles::username,
            profiles::display_name,
            profiles::profile_photo,
            profiles::followers_count,
            profiles::following_count,
            profiles::blocked_count,
        ))
        .first::<(
            Option<String>,
            String,
            Option<String>,
            Option<String>,
            i32,
            i32,
            i32,
        )>(&mut conn)
        .await
        .optional()?;
    if let Some((profile_id, username, display_name, profile_photo, fc, fg, bc)) = profile {
        return Ok(FollowStatsRow {
            profile_id,
            wallet_address: resolved_owner_address,
            username: Some(username),
            display_name,
            profile_photo,
            followers_count: fc as i64,
            following_count: fg as i64,
            blocked_count: bc as i64,
        });
    }
    let ws = wallet_social_graph::table
        .filter(wallet_social_graph::wallet_address.eq(&resolved_owner_address))
        .select((
            wallet_social_graph::followers_count,
            wallet_social_graph::following_count,
            wallet_social_graph::blocked_count,
        ))
        .first::<(i32, i32, i32)>(&mut conn)
        .await
        .optional()?;
    if let Some((followers_count, following_count, blocked_count)) = ws {
        return Ok(FollowStatsRow {
            profile_id: resolved_profile_id,
            wallet_address: resolved_owner_address,
            username: None,
            display_name: None,
            profile_photo: None,
            followers_count: followers_count as i64,
            following_count: following_count as i64,
            blocked_count: blocked_count as i64,
        });
    }
    Ok(FollowStatsRow {
        profile_id: resolved_profile_id,
        wallet_address: resolved_owner_address,
        username: None,
        display_name: None,
        profile_photo: None,
        followers_count: 0,
        following_count: 0,
        blocked_count: 0,
    })
}

pub(crate) async fn get_blocked_profiles(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<BlockedProfileRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = blocked_profiles::table
        .filter(blocked_profiles::blocker_address.eq(address))
        .order_by(blocked_profiles::last_blocked_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            blocked_profiles::blocked_address,
            blocked_profiles::blocked_username,
            blocked_profiles::blocked_display_name,
            blocked_profiles::blocked_profile_photo,
            blocked_profiles::first_blocked_at,
            blocked_profiles::last_blocked_at,
        ))
        .load::<(
            String,
            String,
            Option<String>,
            Option<String>,
            chrono::NaiveDateTime,
            chrono::NaiveDateTime,
        )>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(
                blocked_address,
                blocked_username,
                blocked_display_name,
                blocked_profile_photo,
                first_blocked_at,
                last_blocked_at,
            )| BlockedProfileRow {
                blocked_address,
                blocked_username,
                blocked_display_name,
                blocked_profile_photo,
                first_blocked_at,
                last_blocked_at,
            },
        )
        .collect())
}

pub(crate) async fn get_blocked_platforms(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<BlockedPlatformRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT p.platform_id, p.name, pbp.blocked_by, pbp.created_at
        FROM platform_blocked_profiles pbp
        INNER JOIN platforms p ON pbp.platform_id = p.platform_id
        WHERE pbp.wallet_address = $1
        ORDER BY pbp.created_at DESC
        LIMIT $2 OFFSET $3
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        platform_id: String,
        #[diesel(sql_type = Text)]
        name: String,
        #[diesel(sql_type = Text)]
        blocked_by: String,
        #[diesel(sql_type = Timestamp)]
        created_at: chrono::NaiveDateTime,
    }
    let results = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(|r| BlockedPlatformRow {
            platform_id: r.platform_id,
            name: r.name,
            blocked_by: r.blocked_by,
            created_at: r.created_at,
        })
        .collect())
}

pub(crate) async fn check_following(
    db: &Db,
    follower: &str,
    following: &str,
) -> Result<(bool, bool), SocialError> {
    let mut conn = db.connect().await?;
    let (follower_profile_id, follower_owner_address) =
        resolve_profile_input(&mut conn, follower).await?;
    let (following_profile_id, following_owner_address) =
        resolve_profile_input(&mut conn, following).await?;

    let follower_refs: Vec<&str> = vec![
        &follower_owner_address,
        follower_profile_id
            .as_ref()
            .unwrap_or(&follower_owner_address),
    ];
    let following_refs: Vec<&str> = vec![
        &following_owner_address,
        following_profile_id
            .as_ref()
            .unwrap_or(&following_owner_address),
    ];

    let is_following: i64 = social_graph_relationships::table
        .filter(social_graph_relationships::follower_address.eq_any(&follower_refs))
        .filter(social_graph_relationships::following_address.eq_any(&following_refs))
        .count()
        .get_result(&mut conn)
        .await?;

    let following_back: i64 = social_graph_relationships::table
        .filter(social_graph_relationships::follower_address.eq_any(&following_refs))
        .filter(social_graph_relationships::following_address.eq_any(&follower_refs))
        .count()
        .get_result(&mut conn)
        .await?;

    Ok((is_following > 0, following_back > 0))
}

pub(crate) async fn get_social_graph_chart_data(
    db: &Db,
    query: &SocialGraphChartQuery,
) -> Result<SocialGraphChartData, SocialError> {
    fn bucket_to_days(bucket: &str) -> Result<i32, String> {
        match bucket.to_lowercase().as_str() {
            "7d" => Ok(7),
            "30d" => Ok(30),
            "90d" => Ok(90),
            "180d" => Ok(180),
            "1y" => Ok(365),
            _ => Err(format!(
                "Invalid bucket '{}'. Must be one of: 7d, 30d, 90d, 180d, 1y",
                bucket
            )),
        }
    }

    let bucket_str = query.bucket.as_deref().unwrap_or("30d").to_lowercase();
    let days = bucket_to_days(&bucket_str).map_err(crate::error::SocialError::bad_request)?;

    let mut conn = db.connect().await?;
    let end_date = chrono::Utc::now().date_naive();
    let start_date = end_date - chrono::Duration::days(days as i64);

    #[derive(QueryableByName)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    struct ChartRow {
        #[diesel(sql_type = Date)]
        day: chrono::NaiveDate,
        #[diesel(sql_type = Text)]
        event_type: String,
        #[diesel(sql_type = BigInt)]
        event_count: i64,
    }

    let rows: Vec<ChartRow> = diesel::sql_query(
        r#"SELECT day::DATE as day, event_type, event_count::BIGINT as event_count
           FROM social_graph_daily_stats
           WHERE day >= $1::DATE
           ORDER BY day ASC, event_type ASC"#,
    )
    .bind::<Date, _>(start_date)
    .load::<ChartRow>(&mut conn)
    .await?;

    let chart_data: Vec<DailyStatsPoint> = rows
        .into_iter()
        .map(|r| DailyStatsPoint {
            day: r.day.format("%Y-%m-%d").to_string(),
            event_type: r.event_type,
            event_count: r.event_count,
        })
        .collect();

    let total_follows: i64 = chart_data
        .iter()
        .filter(|p| p.event_type == "follow")
        .map(|p| p.event_count)
        .sum();
    let total_unfollows: i64 = chart_data
        .iter()
        .filter(|p| p.event_type == "unfollow")
        .map(|p| p.event_count)
        .sum();

    Ok(SocialGraphChartData {
        chart_data,
        date_range: DateRange {
            start_date: start_date.format("%Y-%m-%d").to_string(),
            end_date: end_date.format("%Y-%m-%d").to_string(),
            days,
            bucket: bucket_str,
        },
        summary: ChartSummary {
            total_follows,
            total_unfollows,
        },
    })
}

pub(crate) async fn check_profile_blocked(
    db: &Db,
    blocker: &str,
    blocked: &str,
) -> Result<bool, SocialError> {
    let mut conn = db.connect().await?;
    let count: i64 = blocked_profiles::table
        .filter(blocked_profiles::blocker_address.eq(blocker))
        .filter(blocked_profiles::blocked_address.eq(blocked))
        .count()
        .get_result(&mut conn)
        .await?;
    Ok(count > 0)
}

pub(crate) async fn check_platform_blocked(
    db: &Db,
    profile_address: &str,
    platform_id: &str,
) -> Result<bool, SocialError> {
    let mut conn = db.connect().await?;
    let count: i64 = platform_blocked_profiles::table
        .filter(platform_blocked_profiles::wallet_address.eq(profile_address))
        .filter(platform_blocked_profiles::platform_id.eq(platform_id))
        .count()
        .get_result(&mut conn)
        .await?;
    Ok(count > 0)
}

pub(crate) async fn list_badges(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileBadgeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT DISTINCT ON (badge_id) badge_id, badge_name, badge_description, badge_media_url,
               badge_icon_url, platform_id, assigned_by, assigned_at, revoked, revoked_at, revoked_by, badge_type
        FROM profile_badges
        WHERE revoked = false
        ORDER BY badge_id, time DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProfileBadgeRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_badge_by_id(
    db: &Db,
    badge_id: &str,
) -> Result<Option<ProfileBadgeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT badge_id, badge_name, badge_description, badge_media_url, badge_icon_url,
               platform_id, assigned_by, assigned_at, revoked, revoked_at, revoked_by, badge_type
        FROM (
            SELECT DISTINCT ON (badge_id) *
            FROM profile_badges
            WHERE badge_id = $1
            ORDER BY badge_id, time DESC
        ) sub
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(badge_id)
        .get_result::<ProfileBadgeRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}
