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
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub followers_count: i32,
    pub following_count: i32,
    pub post_count: i32,
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
    pub mastodon_username: Option<String>,
    pub facebook_username: Option<String>,
    pub reddit_username: Option<String>,
    pub github_username: Option<String>,
    pub block_list_address: Option<String>,
    pub social_proof_token_address: Option<String>,
    pub reservation_pool_address: Option<String>,
    pub social_proof_token: Option<SocialProofTokenInfo>,
    pub selected_badge: Option<SelectedBadgeInfo>,
    pub selected_badge_id: Option<String>,
    pub selected_ecosystem_badge_id: Option<String>,
}

fn to_iso8601_utc(dt: chrono::NaiveDateTime) -> String {
    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(dt, chrono::Utc)
        .to_rfc3339_opts(chrono::SecondsFormat::Millis, true)
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

    let associated_ids: Vec<String> = wallet_addresses
        .iter()
        .map(|addr| format!("profile_{}", addr))
        .collect();
    let query = diesel::sql_query(
        r#"
        WITH latest_profiles AS (
            SELECT DISTINCT ON (COALESCE(profile_id, owner_address)) *
            FROM profiles
            WHERE owner_address = ANY($1::TEXT[])
            ORDER BY COALESCE(profile_id, owner_address), updated_at DESC
        ),
        latest_spt_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_pools
            WHERE associated_id = ANY($2::TEXT[])
            ORDER BY pool_id, time DESC
        ),
        latest_reservation_pools AS (
            SELECT DISTINCT ON (pool_id) *
            FROM spt_reservation_pools
            WHERE associated_id = ANY($2::TEXT[])
            ORDER BY pool_id, time DESC
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
        LEFT JOIN latest_spt_pools spt ON
            spt.associated_id = 'profile_' || p.owner_address
        LEFT JOIN latest_reservation_pools rp ON
            rp.associated_id = 'profile_' || p.owner_address
        WHERE p.owner_address = ANY($1::TEXT[])
        "#,
    )
    .bind::<Array<Text>, _>(&wallet_addresses)
    .bind::<Array<Text>, _>(&associated_ids);

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
            if let Some(e) = enriched.get(address) {
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
                Ok((fc, fg, _bc, created_at, updated_at)) => ProfileByAddressResponse {
                    id: None,
                    owner_address: address.to_string(),
                    profile_id: None,
                    username: None,
                    display_name: None,
                    bio: None,
                    profile_photo: None,
                    cover_photo: None,
                    website: None,
                    created_at: Some(to_iso8601_utc(created_at)),
                    updated_at: Some(to_iso8601_utc(updated_at)),
                    followers_count: fc,
                    following_count: fg,
                    post_count: 0,
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
                    mastodon_username: None,
                    facebook_username: None,
                    reddit_username: None,
                    github_username: None,
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
                    mastodon_username: None,
                    facebook_username: None,
                    reddit_username: None,
                    github_username: None,
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
        created_at: Some(to_iso8601_utc(p.created_at)),
        updated_at: Some(to_iso8601_utc(p.updated_at)),
        followers_count: p.followers_count,
        following_count: p.following_count,
        post_count: p.post_count,
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
        mastodon_username: None,
        facebook_username: p.facebook_username,
        reddit_username: p.reddit_username,
        github_username: p.github_username,
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
