// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use myso_indexer_alt_social_reader::{ProfilePnLWindow, ProfilePnLWindowResult};
use myso_indexer_alt_social_schema::models::Profile;
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;
use crate::reader::{ProfileByAddressResponse, WalletMessagingPolicyResponse};

use super::super::{AppState, PageParams, ProfileQuery};

pub async fn get_profile_daily_stats_chart(
    State(state): State<Arc<AppState>>,
    Query(query): Query<crate::reader::SocialGraphChartQuery>,
) -> Result<Json<crate::reader::ProfileDailyStatsChartData>, SocialError> {
    let data = state.reader.get_profile_daily_stats_chart(&query).await?;
    Ok(Json(data))
}

pub async fn latest_profiles(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ProfileQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    let total_count = state.reader.get_profile_count().await?;
    let total_pages = ((total_count as f64) / (limit as f64)).ceil() as i64;

    let profiles = state.reader.get_profiles_enriched(limit, offset).await?;

    Ok(Json(serde_json::json!({
        "profiles": profiles,
        "pagination": {
            "total": total_count,
            "limit": limit,
            "offset": offset,
            "page": page,
            "total_pages": total_pages
        }
    })))
}

pub async fn get_profile_by_address(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<ProfileByAddressResponse>, SocialError> {
    let result = state
        .reader
        .get_profile_or_wallet_by_address(&address)
        .await?;
    Ok(Json(result))
}

pub async fn get_profile_by_username(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
) -> Result<Json<Profile>, SocialError> {
    let profile = state
        .reader
        .get_profile_by_username(&username)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Profile for username '{}'", username)))?;
    Ok(Json(profile))
}

pub async fn get_profile_posts(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<myso_indexer_alt_social_reader::PostRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let posts = state
        .reader
        .get_profile_posts(&address, limit, offset)
        .await?;
    Ok(Json(posts))
}

pub async fn get_profile_events(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfileEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let events = state
        .reader
        .get_profile_events(&address, limit, offset)
        .await?;
    Ok(Json(events))
}

pub async fn get_profile_platform_memberships(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfilePlatformMembershipRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let memberships = state
        .reader
        .get_profile_platform_memberships(&address, limit, offset)
        .await?;
    Ok(Json(memberships))
}

pub async fn get_profile_platform_events(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let (events, total) = state
        .reader
        .get_profile_platform_events(&address, limit, offset)
        .await?;
    Ok(Json(serde_json::json!({
        "events": events,
        "total": total
    })))
}

pub async fn get_profile_blocking_history(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::BlockedEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let history = state
        .reader
        .get_blocking_history(&address, limit, offset)
        .await?;
    Ok(Json(history))
}

pub async fn get_profile_badges(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfileBadgeRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let badges = state
        .reader
        .get_profile_badges(&address, limit, offset)
        .await?;
    Ok(Json(badges))
}

pub async fn get_profile_following(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<crate::reader::FollowsQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let (profiles, pagination) = state.reader.get_following(&address, &query).await?;
    Ok(Json(serde_json::json!({
        "profiles": profiles,
        "pagination": pagination
    })))
}

pub async fn get_profile_followers(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<crate::reader::FollowsQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let (profiles, pagination) = state.reader.get_followers(&address, &query).await?;
    Ok(Json(serde_json::json!({
        "profiles": profiles,
        "pagination": pagination
    })))
}

pub async fn get_profile_recommendations(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<crate::reader::FollowsQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let (recommendations, pagination) = state
        .reader
        .get_follow_recommendations(&address, &query)
        .await?;
    Ok(Json(serde_json::json!({
        "recommendations": recommendations,
        "pagination": pagination
    })))
}

pub async fn get_profile_social_stats(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<crate::reader::FollowStatsRow>, SocialError> {
    let stats = state.reader.get_social_stats(&address).await?;
    Ok(Json(stats))
}

pub async fn get_wallet_messaging_policy(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<WalletMessagingPolicyResponse>, SocialError> {
    let policy = state
        .reader
        .get_wallet_messaging_policy(&address)
        .await?
        .ok_or_else(|| {
            SocialError::not_found(format!("No messaging policy for wallet '{}'", address))
        })?;
    Ok(Json(policy))
}

pub async fn get_profile_blocked(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::BlockedProfileRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let blocked = state
        .reader
        .get_blocked_profiles(&address, limit, offset)
        .await?;
    Ok(Json(blocked))
}

pub async fn get_profile_blocked_platforms(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::BlockedPlatformRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let platforms = state
        .reader
        .get_blocked_platforms(&address, limit, offset)
        .await?;
    Ok(Json(platforms))
}

pub async fn get_username_offers(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::UsernameOffer>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let offers = state
        .reader
        .list_username_offers_by_username(&username, limit, offset)
        .await?;
    Ok(Json(offers))
}

pub async fn get_profile_username_offers(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::UsernameOffer>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let offers = state
        .reader
        .list_username_offers_by_profile(&address, limit, offset)
        .await?;
    Ok(Json(offers))
}

pub async fn get_username_sale_fees(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::UsernameSaleFee>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let fees = state
        .reader
        .list_username_sale_fees(&address, limit, offset)
        .await?;
    Ok(Json(fees))
}

#[derive(Debug, Deserialize)]
pub struct ProfilePnLQuery {
    /// Comma-separated windows: `days_7`, `days_30`, `days_180`, `days_365`, `all`.
    pub windows: Option<String>,
}

pub async fn get_profile_pnl(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(query): Query<ProfilePnLQuery>,
) -> Result<Json<Vec<ProfilePnLWindowResult>>, SocialError> {
    let windows = parse_profile_pnl_windows(query.windows.as_deref())?;
    let rows = state.reader.get_profile_pnl(&address, &windows).await?;
    Ok(Json(rows))
}

pub async fn get_profile_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::ProfileConfigInfo>, SocialError> {
    state
        .reader
        .get_profile_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("Profile configuration"))
        .map(Json)
}

fn parse_profile_pnl_windows(raw: Option<&str>) -> Result<Vec<ProfilePnLWindow>, SocialError> {
    let Some(raw) = raw.map(str::trim).filter(|s| !s.is_empty()) else {
        return Ok(vec![
            ProfilePnLWindow::Days7,
            ProfilePnLWindow::Days30,
            ProfilePnLWindow::All,
        ]);
    };
    let mut out = Vec::new();
    for part in raw.split(',') {
        let part = part.trim();
        let w = match part {
            "days_7" | "7d" => ProfilePnLWindow::Days7,
            "days_30" | "30d" => ProfilePnLWindow::Days30,
            "days_180" | "180d" => ProfilePnLWindow::Days180,
            "days_365" | "365d" | "1y" => ProfilePnLWindow::Days365,
            "all" | "all_time" => ProfilePnLWindow::All,
            _ => {
                return Err(SocialError::bad_request(format!(
                    "unknown pnl window '{part}'; use days_7, days_30, days_180, days_365, or all"
                )));
            }
        };
        out.push(w);
    }
    if out.is_empty() {
        return Ok(vec![
            ProfilePnLWindow::Days7,
            ProfilePnLWindow::Days30,
            ProfilePnLWindow::All,
        ]);
    }
    Ok(out)
}

#[cfg(test)]
mod pnl_window_parse_tests {
    use super::parse_profile_pnl_windows;
    use myso_indexer_alt_social_reader::ProfilePnLWindow;

    #[test]
    fn default_windows_when_missing() {
        let w = parse_profile_pnl_windows(None).unwrap();
        assert_eq!(
            w,
            vec![
                ProfilePnLWindow::Days7,
                ProfilePnLWindow::Days30,
                ProfilePnLWindow::All,
            ]
        );
    }

    #[test]
    fn parses_compact_aliases() {
        let w = parse_profile_pnl_windows(Some("7d,30d,all")).unwrap();
        assert_eq!(
            w,
            vec![
                ProfilePnLWindow::Days7,
                ProfilePnLWindow::Days30,
                ProfilePnLWindow::All,
            ]
        );
    }
}
