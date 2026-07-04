// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::SelectableHelper;
use diesel::sql_types::{Array, BigInt, Nullable, SmallInt, Text};
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::Profile;
use myso_indexer_alt_social_schema::schema::{profiles, wallet_social_graph};
use serde::Serialize;

use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, Serialize)]
pub struct SocialProofTokenInfo {
    pub pool_id: Option<String>,
    pub token_address: Option<String>,
    pub is_active: bool,
    pub reservation_pool_id: Option<String>,
    pub reservation_percentage: f64,
    #[serde(rename = "reservation_status")]
    pub reservation_status: ReservationStatus,
    pub total_reserved: i64,
    pub required_threshold: i64,
    pub circulating_supply: Option<i64>,
    pub base_price: Option<i64>,
    pub current_price: Option<i64>,
    /// `current_price * circulating_supply` as decimal string (avoids i64 overflow).
    pub market_cap: Option<String>,
    pub price_change_24h: Option<f64>,
    pub volume_24h: Option<i64>,
    pub creator_earnings: Option<i64>,
    pub platform_earnings: Option<i64>,
    pub ecosystem_earnings: Option<i64>,
    pub owner: Option<String>,
    pub created_at: Option<i64>,
    pub token_type: Option<i16>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationStatus {
    Active,
    ThresholdMet,
    Inactive,
    None,
}

#[derive(Debug, Clone, Serialize)]
pub struct SelectedBadgeInfo {
    pub badge_id: String,
    pub badge_name: String,
    pub badge_icon_url: Option<String>,
    pub badge_media_url: Option<String>,
    pub platform_id: String,
    pub badge_type: i16,
}

#[derive(Debug, Clone)]
pub struct UniversalUserResult {
    pub wallet_address: String,
    pub username: Option<String>,
    pub fullname: Option<String>,
    pub profile_photo: Option<String>,
    pub social_proof_token: Option<SocialProofTokenInfo>,
    pub selected_badge: Option<SelectedBadgeInfo>,
}

/// Unified profile response for profile-by-address queries.
#[derive(Debug, Clone, Serialize)]
pub struct ProfileByAddressResponse {
    pub id: Option<i32>,
    pub owner_address: String,
    pub profile_id: Option<String>,
    pub username: Option<String>,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub cover_photo: Option<String>,
    pub website: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
    pub followers_count: i32,
    pub following_count: i32,
    pub post_count: i32,
    pub blocked_count: i32,
    pub min_offer_amount: Option<i64>,
    pub birthdate: Option<String>,
    pub current_location: Option<String>,
    pub raised_location: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub political_view: Option<String>,
    pub religion: Option<String>,
    pub education: Option<String>,
    pub primary_language: Option<String>,
    pub relationship_status: Option<String>,
    pub x_username: Option<String>,
    pub block_list_address: Option<String>,
    pub social_proof_token_address: Option<String>,
    pub reservation_pool_address: Option<String>,
    pub social_proof_token: Option<SocialProofTokenInfo>,
    pub selected_badge: Option<SelectedBadgeInfo>,
    pub selected_badge_id: Option<String>,
    pub selected_ecosystem_badge_id: Option<String>,
}

async fn enrich_users_with_universal_data(
    conn: &mut Connection<'_>,
    wallet_addresses: Vec<String>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<HashMap<String, UniversalUserResult>> {
    if wallet_addresses.is_empty() {
        return Ok(HashMap::new());
    }
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let wallet_lower: Vec<String> = wallet_addresses.iter().map(|a| a.to_lowercase()).collect();

    let query = diesel::sql_query(
        r#"
        WITH latest_profiles AS (
            SELECT DISTINCT ON (COALESCE(profile_id, owner_address)) *
            FROM profiles
            WHERE LOWER(owner_address) = ANY($1::TEXT[])
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
            spt.circulating_supply as spt_circulating_supply,
            spt.base_price as spt_base_price,
            spt.owner as spt_owner,
            spt.created_at as spt_created_at,
            spt.token_type as spt_token_type,
            ph.price as current_price,
            ph24.price as price_24h_ago,
            (COALESCE(vol24.vol, 0) + COALESCE(res_vol24.vol, 0))::bigint as volume_24h,
            COALESCE(rev.creator_earnings, 0)::bigint as creator_earnings,
            COALESCE(rev.platform_earnings, 0)::bigint as platform_earnings,
            COALESCE(rev.ecosystem_earnings, 0)::bigint as ecosystem_earnings,
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
            WHERE LOWER(spt_row.associated_id) = LOWER('profile_' || p.owner_address)
               OR (p.profile_id IS NOT NULL AND LOWER(spt_row.associated_id) = LOWER(p.profile_id))
            ORDER BY spt_row.time DESC
            LIMIT 1
        ) spt ON true
        LEFT JOIN LATERAL (
            SELECT price FROM spt_price_history
            WHERE pool_id = spt.pool_id
            ORDER BY time DESC LIMIT 1
        ) ph ON spt.pool_id IS NOT NULL
        LEFT JOIN LATERAL (
            SELECT price FROM spt_price_history
            WHERE pool_id = spt.pool_id AND time <= NOW() - INTERVAL '24 hours'
            ORDER BY time DESC LIMIT 1
        ) ph24 ON spt.pool_id IS NOT NULL
        LEFT JOIN LATERAL (
            SELECT COALESCE(SUM(myso_amount), 0)::bigint as vol FROM spt_transactions
            WHERE pool_id = spt.pool_id AND time >= NOW() - INTERVAL '24 hours'
        ) vol24 ON spt.pool_id IS NOT NULL
        LEFT JOIN LATERAL (
            SELECT * FROM spt_reservation_pools sr
            WHERE (p.reservation_pool_address IS NOT NULL
                   AND LOWER(TRIM(sr.pool_id)) = LOWER(TRIM(p.reservation_pool_address)))
               OR LOWER(sr.associated_id) = LOWER('profile_' || p.owner_address)
               OR (p.profile_id IS NOT NULL AND LOWER(sr.associated_id) = LOWER(p.profile_id))
            ORDER BY
                (p.reservation_pool_address IS NOT NULL
                 AND LOWER(TRIM(sr.pool_id)) = LOWER(TRIM(p.reservation_pool_address))) DESC,
                sr.time DESC
            LIMIT 1
        ) rp ON true
        LEFT JOIN LATERAL (
            SELECT
                COALESCE(SUM(creator_fee), 0)::bigint AS creator_earnings,
                COALESCE(SUM(platform_fee), 0)::bigint AS platform_earnings,
                COALESCE(SUM(treasury_fee), 0)::bigint AS ecosystem_earnings
            FROM spt_revenue sr
            WHERE (spt.pool_id IS NOT NULL AND sr.pool_id = spt.pool_id)
               OR (rp.pool_id IS NOT NULL AND sr.pool_id = rp.pool_id)
        ) rev ON spt.pool_id IS NOT NULL OR rp.pool_id IS NOT NULL
        LEFT JOIN LATERAL (
            SELECT COALESCE(SUM(amount), 0)::bigint as vol FROM spt_reservations
            WHERE pool_id = rp.pool_id AND time >= NOW() - INTERVAL '24 hours'
        ) res_vol24 ON rp.pool_id IS NOT NULL
        WHERE LOWER(p.owner_address) = ANY($1::TEXT[])
        "#,
    )
    .bind::<Array<Text>, _>(&wallet_lower);

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
        #[diesel(sql_type = Nullable<BigInt>)]
        spt_circulating_supply: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        spt_base_price: Option<i64>,
        #[diesel(sql_type = Nullable<Text>)]
        spt_owner: Option<String>,
        #[diesel(sql_type = Nullable<BigInt>)]
        spt_created_at: Option<i64>,
        #[diesel(sql_type = Nullable<SmallInt>)]
        spt_token_type: Option<i16>,
        #[diesel(sql_type = Nullable<BigInt>)]
        current_price: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        price_24h_ago: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        volume_24h: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        creator_earnings: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        platform_earnings: Option<i64>,
        #[diesel(sql_type = Nullable<BigInt>)]
        ecosystem_earnings: Option<i64>,
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
    metrics.requests_succeeded.inc();

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

        let (market_cap, price_change_24h) =
            if let (Some(current), Some(circ)) = (row.current_price, row.spt_circulating_supply) {
                let cap = ((current as i128) * (circ as i128)).to_string();
                let change = row.price_24h_ago.and_then(|prev| {
                    if prev > 0 {
                        Some(((current - prev) as f64 / prev as f64) * 100.0)
                    } else {
                        None
                    }
                });
                (Some(cap), change)
            } else {
                (None, None)
            };

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
                circulating_supply: row.spt_circulating_supply,
                base_price: row.spt_base_price,
                current_price: row.current_price,
                market_cap,
                price_change_24h,
                volume_24h: row.volume_24h,
                creator_earnings: row.creator_earnings,
                platform_earnings: row.platform_earnings,
                ecosystem_earnings: row.ecosystem_earnings,
                owner: row.spt_owner,
                created_at: row.spt_created_at,
                token_type: row.spt_token_type,
            })
        } else {
            None
        };

        let owner_key = row.owner_address.to_lowercase();
        let user_result = UniversalUserResult {
            wallet_address: owner_key.clone(),
            username: Some(row.username),
            fullname: row.display_name,
            profile_photo: row.profile_photo,
            social_proof_token,
            selected_badge,
        };
        result.insert(owner_key, user_result);
    }

    for wallet_address in &wallet_addresses {
        let key = wallet_address.to_lowercase();
        if !result.contains_key(&key) {
            result.insert(
                key.clone(),
                UniversalUserResult {
                    wallet_address: key,
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

/// Get enriched profile summary data (badge, SPT, reservation %) for a single address.
pub(crate) async fn get_profile_summary_enriched(
    conn: &mut Connection<'_>,
    address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<UniversalUserResult>> {
    let enriched =
        enrich_users_with_universal_data(conn, vec![address.to_string()], metrics).await?;
    Ok(enriched.get(&address.to_lowercase()).cloned())
}

pub(crate) async fn get_profile_by_address(
    conn: &mut Connection<'_>,
    address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<Profile>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let result = profiles::table
        .filter(profiles::owner_address.eq(address))
        .select(Profile::as_select())
        .first::<Profile>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_profile_or_wallet_by_address(
    conn: &mut Connection<'_>,
    address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<ProfileByAddressResponse> {
    let profile_result = profiles::table
        .filter(profiles::owner_address.eq(address))
        .select(Profile::as_select())
        .first::<Profile>(conn)
        .await;

    match profile_result {
        Ok(profile) => {
            let enriched =
                enrich_users_with_universal_data(conn, vec![address.to_string()], metrics).await?;
            let mut response = profile_to_response(profile);
            if let Some(e) = enriched.get(&address.to_lowercase()) {
                response.social_proof_token = e.social_proof_token.clone();
                response.selected_badge = e.selected_badge.clone();
            }
            Ok(response)
        }
        Err(diesel::result::Error::NotFound) => {
            let wallet_result = wallet_social_graph::table
                .filter(wallet_social_graph::wallet_address.eq(address))
                .select((
                    wallet_social_graph::followers_count,
                    wallet_social_graph::following_count,
                    wallet_social_graph::blocked_count,
                    wallet_social_graph::created_at,
                    wallet_social_graph::updated_at,
                ))
                .first::<(i32, i32, i32, chrono::NaiveDateTime, chrono::NaiveDateTime)>(conn)
                .await;

            let wallet_only = match wallet_result {
                Ok((fc, fg, bc, created_at, updated_at)) => ProfileByAddressResponse {
                    id: None,
                    owner_address: address.to_string(),
                    profile_id: None,
                    username: None,
                    display_name: None,
                    bio: None,
                    profile_photo: None,
                    cover_photo: None,
                    website: None,
                    created_at: Some(created_at.and_utc().timestamp_millis()),
                    updated_at: Some(updated_at.and_utc().timestamp_millis()),
                    followers_count: fc,
                    following_count: fg,
                    post_count: 0,
                    blocked_count: bc,
                    min_offer_amount: None,
                    birthdate: None,
                    current_location: None,
                    raised_location: None,
                    phone: None,
                    email: None,
                    gender: None,
                    political_view: None,
                    religion: None,
                    education: None,
                    primary_language: None,
                    relationship_status: None,
                    x_username: None,
                    block_list_address: None,
                    social_proof_token_address: None,
                    reservation_pool_address: None,
                    social_proof_token: None,
                    selected_badge: None,
                    selected_badge_id: None,
                    selected_ecosystem_badge_id: None,
                },
                Err(_) => ProfileByAddressResponse {
                    id: None,
                    owner_address: address.to_string(),
                    profile_id: None,
                    username: None,
                    display_name: None,
                    bio: None,
                    profile_photo: None,
                    cover_photo: None,
                    website: None,
                    created_at: None,
                    updated_at: None,
                    followers_count: 0,
                    following_count: 0,
                    post_count: 0,
                    blocked_count: 0,
                    min_offer_amount: None,
                    birthdate: None,
                    current_location: None,
                    raised_location: None,
                    phone: None,
                    email: None,
                    gender: None,
                    political_view: None,
                    religion: None,
                    education: None,
                    primary_language: None,
                    relationship_status: None,
                    x_username: None,
                    block_list_address: None,
                    social_proof_token_address: None,
                    reservation_pool_address: None,
                    social_proof_token: None,
                    selected_badge: None,
                    selected_badge_id: None,
                    selected_ecosystem_badge_id: None,
                },
            };
            Ok(wallet_only)
        }
        Err(e) => Err(e.into()),
    }
}

fn profile_to_response(p: Profile) -> ProfileByAddressResponse {
    ProfileByAddressResponse {
        id: Some(p.id),
        owner_address: p.owner_address,
        profile_id: p.profile_id,
        username: Some(p.username),
        display_name: p.display_name,
        bio: p.bio,
        profile_photo: p.profile_photo,
        cover_photo: p.cover_photo,
        website: p.website,
        created_at: Some(p.created_at.and_utc().timestamp_millis()),
        updated_at: Some(p.updated_at.and_utc().timestamp_millis()),
        followers_count: p.followers_count,
        following_count: p.following_count,
        post_count: p.post_count,
        blocked_count: p.blocked_count,
        min_offer_amount: p.min_offer_amount,
        birthdate: p.birthdate,
        current_location: p.current_location,
        raised_location: p.raised_location,
        phone: p.phone,
        email: p.email,
        gender: p.gender,
        political_view: p.political_view,
        religion: p.religion,
        education: p.education,
        primary_language: p.primary_language,
        relationship_status: p.relationship_status,
        x_username: p.x_username,
        block_list_address: None,
        social_proof_token_address: p.social_proof_token_address,
        reservation_pool_address: p.reservation_pool_address,
        social_proof_token: None,
        selected_badge: None,
        selected_badge_id: p.selected_badge_id,
        selected_ecosystem_badge_id: p.selected_ecosystem_badge_id,
    }
}

pub(crate) async fn get_profiles(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<Profile>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let results = profiles::table
        .order_by(profiles::id.desc())
        .limit(limit)
        .offset(offset)
        .select(Profile::as_select())
        .load::<Profile>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

#[derive(Debug, Clone, QueryableByName)]
pub struct ProfileBadgeRow {
    #[diesel(sql_type = Text)]
    pub badge_id: String,
    #[diesel(sql_type = Text)]
    pub badge_name: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub badge_description: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub badge_media_url: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub badge_icon_url: Option<String>,
    #[diesel(sql_type = Text)]
    pub platform_id: String,
    #[diesel(sql_type = Text)]
    pub assigned_by: String,
    #[diesel(sql_type = BigInt)]
    pub assigned_at: i64,
    #[diesel(sql_type = SmallInt)]
    pub badge_type: i16,
}

pub(crate) async fn get_profile_badges(
    conn: &mut Connection<'_>,
    address: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<ProfileBadgeRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let profile_id_opt: Option<String> = profiles::table
        .filter(profiles::owner_address.eq(address))
        .select(profiles::profile_id)
        .first::<Option<String>>(conn)
        .await
        .optional()?
        .flatten();
    let id2 = profile_id_opt.as_deref().unwrap_or(address).to_string();
    let query = "
        SELECT pb.badge_id, pb.badge_name, pb.badge_description, pb.badge_media_url,
               pb.badge_icon_url, pb.platform_id, pb.assigned_by, pb.assigned_at, pb.badge_type
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
        .load::<ProfileBadgeRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

#[derive(Debug, Clone, QueryableByName)]
pub struct ProfileConfigRow {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub max_vesting_pieces: i64,
    #[diesel(sql_type = BigInt)]
    pub curve_factor_min: i64,
    #[diesel(sql_type = BigInt)]
    pub curve_factor_max: i64,
    #[diesel(sql_type = BigInt)]
    pub curve_precision: i64,
    #[diesel(sql_type = BigInt)]
    pub min_claim_threshold_divisor: i64,
    #[diesel(sql_type = BigInt)]
    pub min_username_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_username_length: i64,
    #[diesel(sql_type = BigInt)]
    pub profile_sale_fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// Latest profile configuration.
pub(crate) async fn get_profile_config(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<ProfileConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT updated_by, max_vesting_pieces, curve_factor_min, curve_factor_max, curve_precision,
               min_claim_threshold_divisor, min_username_length, max_username_length,
               profile_sale_fee_bps, version, updated_at, time, transaction_id
        FROM profile_config
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<ProfileConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

#[derive(Debug, Clone, QueryableByName)]
pub struct EcosystemTreasuryRow {
    #[diesel(sql_type = Text)]
    pub treasury_address: String,
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}

/// Latest ecosystem treasury configuration (treasury address).
pub(crate) async fn get_ecosystem_treasury(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<EcosystemTreasuryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT treasury_address, updated_by, version, updated_at, time, transaction_id
        FROM ecosystem_treasury
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<EcosystemTreasuryRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}
