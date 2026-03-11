// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::http::Method;
use axum::routing::get;
use axum::{http::StatusCode, Json, Router};
use myso_indexer_alt_metrics::{MetricsArgs, MetricsService};
use myso_indexer_alt_social_schema::models::Profile;
use myso_pg_db::DbArgs;
use prometheus::Registry;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tower_http::cors::{AllowMethods, Any, CorsLayer};
use url::Url;

use crate::error::SocialError;
use crate::reader::Reader;
use myso_futures::service::Service;

#[derive(Clone)]
pub struct AppState {
    reader: Reader,
}

#[derive(Debug, Deserialize)]
pub struct ProfileQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct MyDataQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub creator: Option<String>,
    pub media_type: Option<String>,
    pub platform_id: Option<String>,
    pub sort_by: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PageParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

impl PageParams {
    fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }
    fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let limit = self.limit();
        self.offset.unwrap_or_else(|| (page - 1) * limit)
    }
}

#[derive(Debug, Deserialize)]
pub struct GovernanceProposalQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub status: Option<i16>,
    pub proposal_type: Option<i16>,
    pub submitter: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GovernanceDelegateQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub registry_type: Option<i16>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct GovernanceNomineeQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub registry_type: Option<i16>,
    pub status: Option<i16>,
}

#[derive(Debug, Deserialize)]
pub struct InsurancePolicyFilters {
    pub insured: Option<String>,
    pub market_id: Option<String>,
    pub vault_id: Option<String>,
    pub status: Option<i16>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SptPoolsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub owner: Option<String>,
    pub token_type: Option<i16>,
}

#[derive(Debug, Deserialize)]
pub struct TimeRangeParams {
    pub from: Option<i64>,
    pub to: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct UpgradeMigrationsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    pub object_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UsernameAvailabilityQuery {
    pub exclude_address: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RevenueQuery {
    pub creator_address: Option<String>,
    pub platform_address: Option<String>,
    pub revenue_source: Option<String>,
    pub revenue_type: Option<String>,
    pub content_id: Option<String>,
    pub content_type: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub period: Option<String>,
    pub points: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RevenueLeaderboardQuery {
    pub revenue_source: Option<String>,
    pub period_days: Option<i64>,
    pub limit: Option<i64>,
    pub min_revenue: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct TreasuryHistoryQuery {
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionAnalyticsQuery {
    pub service_id: Option<String>,
    pub profile_owner: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub period: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct PlatformsQuery {
    pub approved: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PostsQuery {
    pub owner: Option<String>,
    pub post_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

/// Build and return the social API server as a Service without running it.
/// Callers can merge this into a larger Service and run it together.
pub async fn start_server(
    server_port: u16,
    database_url: Url,
    db_args: DbArgs,
    metrics_address: std::net::SocketAddr,
    registry: &Registry,
) -> Result<Service, anyhow::Error> {
    let metrics = MetricsService::new(MetricsArgs { metrics_address }, registry.clone());

    let reader = Reader::new(database_url, db_args).await?;
    let state = Arc::new(AppState { reader });

    let socket_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), server_port);

    let s_metrics = metrics.run().await?;

    let listener = TcpListener::bind(socket_address).await?;
    let (stx, srx) = oneshot::channel::<()>();

    Ok(Service::new()
        .attach(s_metrics)
        .with_shutdown_signal(async move {
            let _ = stx.send(());
        })
        .spawn(async move {
            axum::serve(listener, make_router(state))
                .with_graceful_shutdown(async move {
                    let _ = srx.await;
                })
                .await?;

            Ok(())
        }))
}

pub async fn run_server(
    server_port: u16,
    database_url: Url,
    db_args: DbArgs,
    metrics_address: std::net::SocketAddr,
) -> Result<(), anyhow::Error> {
    let registry = Registry::new_custom(Some("social_api".into()), None)
        .expect("Failed to create Prometheus registry.");

    let service = start_server(
        server_port,
        database_url,
        db_args,
        metrics_address,
        &registry,
    )
    .await?;

    println!("Social API server started on port {}", server_port);

    service.main().await?;

    Ok(())
}

fn make_router(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_methods(AllowMethods::list(vec![
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::OPTIONS,
        ]))
        .allow_headers(Any)
        .allow_origin(Any);

    Router::new()
        .route("/health", get(health_check))
        .route("/stats/system", get(get_system_stats))
        .route("/profiles", get(latest_profiles))
        .route("/profiles/address/:address", get(get_profile_by_address))
        .route("/profiles/username/:username", get(get_profile_by_username))
        .route(
            "/profiles/username/:username/availability",
            get(check_username_availability),
        )
        .route("/profiles/:address/posts", get(get_profile_posts))
        .route("/profiles/:address/events", get(get_profile_events))
        .route(
            "/profiles/:address/platform-memberships",
            get(get_profile_platform_memberships),
        )
        .route(
            "/profiles/:address/platforms",
            get(get_profile_platform_events),
        )
        .route(
            "/profiles/:address/blocking-history",
            get(get_profile_blocking_history),
        )
        .route("/profiles/:address/badges", get(get_profile_badges))
        .route("/profiles/:address/following", get(get_profile_following))
        .route("/profiles/:address/followers", get(get_profile_followers))
        .route(
            "/profiles/:address/social-stats",
            get(get_profile_social_stats),
        )
        .route("/profiles/:address/blocked", get(get_profile_blocked))
        .route(
            "/profiles/:address/blocked-platforms",
            get(get_profile_blocked_platforms),
        )
        .route(
            "/social-graph/check/:follower/:following",
            get(check_social_graph_following),
        )
        .route("/social-graph/chart-data", get(get_social_graph_chart_data))
        .route(
            "/blocklist/check/profile/:blocker/:blocked",
            get(check_profile_blocked),
        )
        .route(
            "/blocklist/check/platform/:profile/:platform",
            get(check_platform_blocked),
        )
        .route("/badges", get(list_badges))
        .route("/badges/:badge_id", get(get_badge_by_id))
        .route("/platforms", get(list_platforms))
        .route("/platforms/approved", get(list_platforms_approved))
        .route("/platforms/:id", get(get_platform_by_id))
        .route("/platforms/:id/moderators", get(get_platform_moderators))
        .route("/platforms/:id/approval", get(get_platform_approval))
        .route("/platforms/:id/blocked", get(get_platform_blocked))
        .route("/platforms/:id/members", get(get_platform_members))
        .route(
            "/platforms/:id/membership/:profile_address",
            get(check_platform_membership),
        )
        .route("/platforms/:id/events", get(get_platform_events))
        .route("/posts", get(list_posts))
        .route("/posts/configuration", get(get_post_config))
        .route("/posts/trending", get(get_trending_posts))
        .route("/posts/:id", get(get_post_by_id))
        .route("/posts/:id/comments", get(get_post_comments))
        .route("/posts/:id/reactions", get(get_post_reactions))
        .route("/posts/:id/reposts", get(get_post_reposts))
        .route("/posts/:id/promotion", get(get_post_promotion))
        .route("/posts/:id/poc-badges", get(get_post_poc_badges))
        .route(
            "/posts/:id/revenue-redirections",
            get(get_post_revenue_redirections),
        )
        .route("/promotions", get(list_promotions))
        .route(
            "/promotions/analytics/top-performing",
            get(get_top_performing_promotions),
        )
        .route(
            "/promotions/analytics/spending-trends",
            get(get_spending_trends),
        )
        .route("/promotions/:id/views", get(get_promotion_views))
        .route("/promotions/:id/stats", get(get_promotion_stats))
        .route(
            "/promotions/:id/analytics/time-series",
            get(get_promotion_time_series),
        )
        .route(
            "/promotions/:id/analytics/hourly",
            get(get_promotion_hourly),
        )
        .route("/poc/badges", get(list_poc_badges))
        .route("/poc/badges/:id", get(get_poc_badge_by_id))
        .route(
            "/poc/revenue-redirections",
            get(list_poc_revenue_redirections),
        )
        .route("/poc/analysis-results", get(list_poc_analysis_results))
        .route("/poc/disputes", get(list_poc_disputes))
        .route("/poc/disputes/:id", get(get_poc_dispute_by_id))
        .route("/poc/disputes/:id/votes", get(get_poc_dispute_votes))
        .route("/poc/analytics", get(get_poc_analytics))
        .route("/poc/configuration", get(get_poc_configuration))
        .route("/subscriptions", get(list_subscriptions))
        .route("/subscription-services", get(list_subscription_services))
        .route("/subscription-revenue", get(list_subscription_revenue))
        .route("/subscriptions/:id/status", get(get_subscription_status))
        .route("/subscribers/:address/summary", get(get_subscriber_summary))
        .route("/vesting/wallets", get(list_vesting_wallets))
        .route("/vesting/wallets/active", get(list_vesting_wallets_active))
        .route("/vesting/wallets/:wallet_id", get(get_vesting_wallet))
        .route(
            "/vesting/wallets/:wallet_id/events",
            get(get_vesting_wallet_events),
        )
        .route(
            "/vesting/wallets/:wallet_id/claimable",
            get(get_vesting_claimable),
        )
        .route(
            "/vesting/users/:address/wallets",
            get(get_user_vesting_wallets),
        )
        .route("/vesting/events", get(list_vesting_events))
        .route("/vesting/analytics", get(get_vesting_analytics))
        .route("/vesting/leaderboard", get(get_vesting_leaderboard))
        .route("/revenue/dashboard", get(get_revenue_dashboard))
        .route("/revenue/leaderboard", get(get_revenue_leaderboard))
        .route("/revenue/chart-data", get(get_revenue_chart_data))
        .route("/revenue/unified", get(get_unified_revenue))
        .route(
            "/revenue/creators/:address/stats",
            get(get_creator_revenue_stats),
        )
        .route(
            "/revenue/platforms/:address/stats",
            get(get_platform_revenue_stats),
        )
        .route("/treasury/current", get(get_treasury_current))
        .route("/treasury/history", get(get_treasury_history))
        .route("/search", get(search))
        .route(
            "/profiles/:owner/subscription-services",
            get(list_profile_subscription_services),
        )
        .route(
            "/subscription-services/:service_id",
            get(get_profile_subscription_service),
        )
        .route(
            "/subscription-services/:service_id/revenue",
            get(get_subscription_revenue_by_service),
        )
        .route(
            "/subscriptions/subscriber/:address",
            get(list_subscriptions_by_subscriber),
        )
        .route(
            "/subscriptions/:subscription_id",
            get(get_subscription_by_id),
        )
        .route(
            "/subscription-access/:subscriber/:service_id",
            get(check_subscription_access),
        )
        .route("/subscription-analytics", get(get_subscription_analytics))
        .route("/service-performance", get(get_service_performance))
        .route("/mydata", get(list_mydata))
        .route("/mydata/configuration", get(get_mydata_configuration))
        .route("/mydata/popular", get(get_popular_mydata))
        .route("/mydata/:id", get(get_mydata_by_id))
        .route("/mydata/:id/purchases", get(get_mydata_purchases))
        .route("/mydata/:id/subscriptions", get(get_mydata_subscriptions))
        .route("/mydata/:id/revenue", get(get_mydata_revenue))
        .route("/mydata/:id/access-logs", get(get_mydata_access_logs))
        .route("/mydata/:id/stats", get(get_mydata_stats))
        .route(
            "/mydata/:id/revenue-timeline",
            get(get_mydata_revenue_timeline),
        )
        .route(
            "/mydata/:id/access-analytics",
            get(get_mydata_access_analytics),
        )
        .route("/creators/:id/mydata", get(get_creator_mydata))
        .route("/insurance/config", get(get_insurance_config))
        .route("/insurance/vaults", get(list_insurance_vaults))
        .route("/insurance/vaults/:vault_id", get(get_insurance_vault))
        .route(
            "/insurance/vaults/:vault_id/transactions",
            get(list_insurance_vault_transactions),
        )
        .route(
            "/insurance/vaults/:vault_id/exposures",
            get(get_insurance_vault_exposures),
        )
        .route("/insurance/policies", get(list_insurance_policies))
        .route("/insurance/policies/:policy_id", get(get_insurance_policy))
        .route(
            "/insurance/markets/:market_id/policies",
            get(list_insurance_market_policies),
        )
        .route("/spot/records/:post_id", get(get_spot_record))
        .route("/spot/records/:post_id/bets", get(list_spot_bets))
        .route("/spot/records/:post_id/payouts", get(list_spot_payouts))
        .route("/spot/records/:post_id/refunds", get(list_spot_refunds))
        .route("/spot/configuration", get(get_spot_configuration))
        .route("/spt/pools", get(list_spt_pools))
        .route(
            "/spt/pools/by-associated-id/:id",
            get(get_spt_pool_by_associated_id),
        )
        .route("/spt/popular", get(get_spt_popular))
        .route("/spt/users/:address/holdings", get(get_spt_user_holdings))
        .route(
            "/spt/users/:address/reservations",
            get(get_spt_user_reservations),
        )
        .route(
            "/spt/analytics/top-performers",
            get(get_spt_analytics_top_performers),
        )
        .route(
            "/spt/portfolios/:address/performance",
            get(get_spt_portfolio_performance),
        )
        .route(
            "/spt/creators/:address/revenue-streams",
            get(get_spt_creator_revenue_streams),
        )
        .route("/spt/market-sentiment", get(get_spt_market_sentiment))
        .route(
            "/spt/pools/:id/liquidity-profile",
            get(get_spt_liquidity_profile),
        )
        .route("/spt/pools/:id", get(get_spt_pool))
        .route(
            "/spt/pools/:id/transactions",
            get(get_spt_pool_transactions),
        )
        .route("/spt/pools/:id/holdings", get(get_spt_pool_holdings))
        .route(
            "/spt/pools/:id/price-history",
            get(get_spt_pool_price_history),
        )
        .route("/spt/pools/:id/revenue", get(get_spt_pool_revenue))
        .route("/spt/config", get(get_spt_config))
        .route("/spt/reservation-pools", get(list_spt_reservation_pools))
        .route("/spt/reservation-pools/:id", get(get_spt_reservation_pool))
        .route(
            "/spt/reservation-pools/:id/reservations",
            get(list_spt_reservation_pool_reservations),
        )
        .route("/governance/proposals", get(list_governance_proposals))
        .route("/governance/proposals/:id", get(get_governance_proposal))
        .route(
            "/governance/proposals/:id/community-votes",
            get(get_governance_proposal_community_votes),
        )
        .route(
            "/governance/proposals/:id/anonymous-stats",
            get(get_governance_proposal_anonymous_stats),
        )
        .route(
            "/governance/proposals/:id/anonymous-votes",
            get(get_governance_proposal_anonymous_votes),
        )
        .route(
            "/governance/proposals/:id/decryption-failures",
            get(get_governance_proposal_decryption_failures),
        )
        .route("/governance/delegates", get(list_governance_delegates))
        .route(
            "/governance/delegates/:address",
            get(get_governance_delegate),
        )
        .route(
            "/governance/delegates/:address/proposals",
            get(get_governance_delegate_proposals),
        )
        .route(
            "/governance/delegates/:address/ratings",
            get(get_governance_delegate_ratings),
        )
        .route("/governance/nominees", get(list_governance_nominees))
        .route("/governance/registries", get(list_governance_registries))
        .route(
            "/governance/registries/:registry_type",
            get(get_governance_registry),
        )
        .route("/governance/events", get(list_governance_events))
        .route(
            "/governance/anonymous-voting/trends",
            get(get_governance_anonymous_voting_trends),
        )
        .route("/upgrade/events", get(list_upgrade_events))
        .route("/upgrade/migrations", get(list_object_migrated_events))
        .with_state(state)
        .layer(cors)
}

async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "healthy",
            "message": "Social API server is running",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    )
}

async fn latest_profiles(
    State(state): State<Arc<AppState>>,
    Query(query): Query<ProfileQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);
    let page = query.page.unwrap_or(1);
    let offset = if page > 1 { (page - 1) * limit } else { offset };

    let total_count = state.reader.get_profile_count().await?;
    let total_pages = ((total_count as f64) / (limit as f64)).ceil() as i64;

    let profiles = state.reader.get_profiles(limit, offset).await?;

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

async fn get_profile_by_address(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<Profile>, SocialError> {
    let profile = state
        .reader
        .get_profile_by_address(&address)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Profile for address '{}'", address)))?;
    Ok(Json(profile))
}

async fn get_profile_by_username(
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

async fn get_system_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::SystemStatsResponse>, SocialError> {
    let stats = state.reader.get_system_stats().await?;
    Ok(Json(stats))
}

async fn check_username_availability(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    Query(query): Query<UsernameAvailabilityQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let available = state
        .reader
        .check_username_availability(&username, query.exclude_address.as_deref())
        .await?;
    Ok(Json(serde_json::json!({ "available": available })))
}

async fn get_profile_posts(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PostBasicRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let posts = state
        .reader
        .get_profile_posts(&address, limit, offset)
        .await?;
    Ok(Json(posts))
}

async fn get_profile_events(
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

async fn get_profile_platform_memberships(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformMembershipRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let memberships = state
        .reader
        .get_profile_platform_memberships(&address, limit, offset)
        .await?;
    Ok(Json(memberships))
}

async fn get_profile_platform_events(
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

async fn get_profile_blocking_history(
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

async fn get_profile_badges(
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

async fn get_profile_following(
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

async fn get_profile_followers(
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

async fn get_profile_social_stats(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<crate::reader::FollowStatsRow>, SocialError> {
    let stats = state.reader.get_social_stats(&address).await?;
    Ok(Json(stats))
}

async fn get_profile_blocked(
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

async fn get_profile_blocked_platforms(
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

async fn check_social_graph_following(
    State(state): State<Arc<AppState>>,
    Path((follower, following)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let (is_following, following_back) =
        state.reader.check_following(&follower, &following).await?;
    Ok(Json(serde_json::json!({
        "is_following": is_following,
        "following_back": following_back
    })))
}

async fn get_social_graph_chart_data(
    State(state): State<Arc<AppState>>,
    Query(query): Query<crate::reader::SocialGraphChartQuery>,
) -> Result<Json<crate::reader::SocialGraphChartData>, SocialError> {
    let data = state.reader.get_social_graph_chart_data(&query).await?;
    Ok(Json(data))
}

async fn check_profile_blocked(
    State(state): State<Arc<AppState>>,
    Path((blocker, blocked)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let is_blocked = state
        .reader
        .check_profile_blocked(&blocker, &blocked)
        .await?;
    Ok(Json(serde_json::json!({ "is_blocked": is_blocked })))
}

async fn check_platform_blocked(
    State(state): State<Arc<AppState>>,
    Path((profile, platform)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let is_blocked = state
        .reader
        .check_platform_blocked(&profile, &platform)
        .await?;
    Ok(Json(serde_json::json!({ "is_blocked": is_blocked })))
}

async fn list_badges(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfileBadgeRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let badges = state.reader.list_badges(limit, offset).await?;
    Ok(Json(badges))
}

async fn get_badge_by_id(
    State(state): State<Arc<AppState>>,
    Path(badge_id): Path<String>,
) -> Result<Json<crate::reader::ProfileBadgeRow>, SocialError> {
    let badge = state
        .reader
        .get_badge_by_id(&badge_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Badge '{}'", badge_id)))?;
    Ok(Json(badge))
}

async fn list_platforms(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PlatformsQuery>,
) -> Result<Json<Vec<crate::reader::PlatformRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let page = params.page.unwrap_or(1).max(1);
    let offset = params.offset.unwrap_or_else(|| (page - 1) * limit);
    let approved_only = params.approved.unwrap_or(false);
    let platforms = state
        .reader
        .list_platforms(approved_only, limit, offset)
        .await?;
    Ok(Json(platforms))
}

async fn list_platforms_approved(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let platforms = state.reader.list_platforms(true, limit, offset).await?;
    Ok(Json(platforms))
}

async fn get_platform_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PlatformRow>, SocialError> {
    let platform = state
        .reader
        .get_platform_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Platform '{}'", id)))?;
    Ok(Json(platform))
}

async fn get_platform_moderators(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformModeratorRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let moderators = state
        .reader
        .get_platform_moderators(&id, limit, offset)
        .await?;
    Ok(Json(moderators))
}

async fn get_platform_approval(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PlatformApprovalRow>, SocialError> {
    let approval = state
        .reader
        .get_platform_approval(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Platform '{}'", id)))?;
    Ok(Json(approval))
}

async fn get_platform_blocked(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformBlockedProfileRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let blocked = state
        .reader
        .get_platform_blocked_profiles(&id, limit, offset)
        .await?;
    Ok(Json(blocked))
}

async fn get_platform_members(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformMemberRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let members = state
        .reader
        .get_platform_members(&id, limit, offset)
        .await?;
    Ok(Json(members))
}

async fn check_platform_membership(
    State(state): State<Arc<AppState>>,
    Path((id, profile_address)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let is_member = state
        .reader
        .check_platform_membership(&id, &profile_address)
        .await?;
    Ok(Json(serde_json::json!({ "is_member": is_member })))
}

async fn get_platform_events(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PlatformEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let events = state.reader.get_platform_events(&id, limit, offset).await?;
    Ok(Json(events))
}

async fn list_posts(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PostsQuery>,
) -> Result<Json<Vec<crate::reader::PostBasicRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params
        .offset
        .unwrap_or_else(|| (params.page.unwrap_or(1).max(1) - 1) * limit);
    let posts = state
        .reader
        .list_posts(
            params.owner.as_deref(),
            params.post_type.as_deref(),
            limit,
            offset,
        )
        .await?;
    Ok(Json(posts))
}

async fn get_post_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::PostConfigRow>, SocialError> {
    let config = state
        .reader
        .get_post_config()
        .await?
        .ok_or_else(|| SocialError::not_found("Post configuration".to_string()))?;
    Ok(Json(config))
}

async fn get_trending_posts(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PostBasicRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let posts = state.reader.get_trending_posts(limit, offset).await?;
    Ok(Json(posts))
}

async fn get_post_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PostBasicRow>, SocialError> {
    let post = state
        .reader
        .get_post_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Post '{}'", id)))?;
    Ok(Json(post))
}

async fn get_post_comments(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::CommentRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let comments = state.reader.get_post_comments(&id, limit, offset).await?;
    Ok(Json(comments))
}

async fn get_post_reactions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ReactionRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let reactions = state.reader.get_post_reactions(&id, limit, offset).await?;
    Ok(Json(reactions))
}

async fn get_post_reposts(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::RepostRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let reposts = state.reader.get_post_reposts(&id, limit, offset).await?;
    Ok(Json(reposts))
}

async fn get_post_promotion(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PromotedPostRow>, SocialError> {
    let promotion = state
        .reader
        .get_promotion_by_post_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Promotion for post '{}'", id)))?;
    Ok(Json(promotion))
}

async fn get_post_poc_badges(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocBadgeRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let badges = state.reader.get_post_poc_badges(&id, limit, offset).await?;
    Ok(Json(badges))
}

async fn get_post_revenue_redirections(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocRevenueRedirectionRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let redirections = state
        .reader
        .get_post_revenue_redirections(&id, limit, offset)
        .await?;
    Ok(Json(redirections))
}

async fn list_promotions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotedPostRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let promotions = state.reader.list_promotions(limit, offset).await?;
    Ok(Json(promotions))
}

async fn get_promotion_views(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotionViewRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let views = state.reader.get_promotion_views(&id, limit, offset).await?;
    Ok(Json(views))
}

async fn get_promotion_stats(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PromotionStatsRow>, SocialError> {
    let stats = state
        .reader
        .get_promotion_stats(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Promotion '{}'", id)))?;
    Ok(Json(stats))
}

async fn get_promotion_time_series(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotionTimeSeriesRow>>, SocialError> {
    let limit = params.limit();
    let data = state.reader.get_promotion_time_series(&id, limit).await?;
    Ok(Json(data))
}

async fn get_promotion_hourly(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotionHourlyRow>>, SocialError> {
    let limit = params.limit();
    let data = state.reader.get_promotion_hourly(&id, limit).await?;
    Ok(Json(data))
}

async fn get_top_performing_promotions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotedPostRow>>, SocialError> {
    let limit = params.limit();
    let promotions = state.reader.get_top_performing_promotions(limit).await?;
    Ok(Json(promotions))
}

async fn get_spending_trends(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PromotionTimeSeriesRow>>, SocialError> {
    let limit = params.limit();
    let data = state.reader.get_spending_trends(limit).await?;
    Ok(Json(data))
}

async fn list_poc_badges(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocBadgeRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let badges = state.reader.list_poc_badges(limit, offset).await?;
    Ok(Json(badges))
}

async fn get_poc_badge_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PocBadgeRow>, SocialError> {
    let badge = state
        .reader
        .get_poc_badge_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("PoC badge '{}'", id)))?;
    Ok(Json(badge))
}

async fn list_poc_revenue_redirections(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocRevenueRedirectionRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let redirections = state
        .reader
        .list_poc_revenue_redirections(limit, offset)
        .await?;
    Ok(Json(redirections))
}

async fn list_poc_analysis_results(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocAnalysisResultRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let results = state
        .reader
        .list_poc_analysis_results(limit, offset)
        .await?;
    Ok(Json(results))
}

async fn list_poc_disputes(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocDisputeRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let disputes = state.reader.list_poc_disputes(limit, offset).await?;
    Ok(Json(disputes))
}

async fn get_poc_dispute_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::PocDisputeRow>, SocialError> {
    let dispute = state
        .reader
        .get_poc_dispute_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("PoC dispute '{}'", id)))?;
    Ok(Json(dispute))
}

async fn get_poc_dispute_votes(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::PocDisputeVoteRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let votes = state
        .reader
        .get_poc_dispute_votes(&id, limit, offset)
        .await?;
    Ok(Json(votes))
}

async fn get_poc_analytics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let analytics = state.reader.get_poc_analytics().await?;
    Ok(Json(analytics))
}

async fn get_poc_configuration(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::PocConfigRow>, SocialError> {
    let config = state
        .reader
        .get_poc_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("PoC configuration".to_string()))?;
    Ok(Json(config))
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionsQuery {
    pub subscriber: Option<String>,
    pub service_id: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct SubscriptionRevenueQuery {
    pub service_id: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct VestingWalletsQuery {
    pub active: Option<bool>,
    pub owner: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

async fn list_subscriptions(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SubscriptionsQuery>,
) -> Result<Json<Vec<crate::reader::ProfileSubscriptionInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params
        .offset
        .unwrap_or_else(|| (params.page.unwrap_or(1).max(1) - 1) * limit);
    let subscriptions = state
        .reader
        .list_subscriptions(
            params.subscriber.as_deref(),
            params.service_id.as_deref(),
            limit,
            offset,
        )
        .await?;
    Ok(Json(subscriptions))
}

async fn list_subscription_services(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfileSubscriptionServiceInfo>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let services = state
        .reader
        .list_subscription_services(limit, offset)
        .await?;
    Ok(Json(services))
}

async fn list_subscription_revenue(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SubscriptionRevenueQuery>,
) -> Result<Json<Vec<crate::reader::ProfileSubscriptionRevenueRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params
        .offset
        .unwrap_or_else(|| (params.page.unwrap_or(1).max(1) - 1) * limit);
    let revenue = state
        .reader
        .list_subscription_revenue(params.service_id.as_deref(), limit, offset)
        .await?;
    Ok(Json(revenue))
}

async fn get_subscription_status(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let sub = state
        .reader
        .get_subscription_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Subscription '{}'", id)))?;
    let now_ms = chrono::Utc::now().timestamp_millis();
    let active = sub.cancelled_at.is_none() && sub.expires_at > now_ms;
    Ok(Json(serde_json::json!({
        "subscription_id": sub.subscription_id,
        "service_id": sub.service_id,
        "active": active,
        "expires_at": sub.expires_at,
        "cancelled_at": sub.cancelled_at,
    })))
}

async fn get_subscriber_summary(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<crate::reader::SubscriberSummaryRow>, SocialError> {
    let summary = state.reader.get_subscriber_summary(&address).await?;
    Ok(Json(summary))
}

async fn list_vesting_wallets(
    State(state): State<Arc<AppState>>,
    Query(params): Query<VestingWalletsQuery>,
) -> Result<Json<Vec<crate::reader::VestingWalletRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params
        .offset
        .unwrap_or_else(|| (params.page.unwrap_or(1).max(1) - 1) * limit);
    let wallets = state
        .reader
        .list_vesting_wallets(
            params.active.unwrap_or(false),
            params.owner.as_deref(),
            limit,
            offset,
        )
        .await?;
    Ok(Json(wallets))
}

async fn list_vesting_wallets_active(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::VestingWalletRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let wallets = state
        .reader
        .list_vesting_wallets(true, None, limit, offset)
        .await?;
    Ok(Json(wallets))
}

async fn get_vesting_wallet(
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<String>,
) -> Result<Json<crate::reader::VestingWalletRow>, SocialError> {
    let wallet = state
        .reader
        .get_vesting_wallet_by_id(&wallet_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Vesting wallet '{}'", wallet_id)))?;
    Ok(Json(wallet))
}

async fn get_vesting_wallet_events(
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::VestingEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let events = state
        .reader
        .get_vesting_wallet_events(&wallet_id, limit, offset)
        .await?;
    Ok(Json(events))
}

async fn get_vesting_claimable(
    State(state): State<Arc<AppState>>,
    Path(wallet_id): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let amount = state
        .reader
        .get_vesting_claimable(&wallet_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Vesting wallet '{}'", wallet_id)))?;
    Ok(Json(serde_json::json!({ "claimable": amount })))
}

async fn get_user_vesting_wallets(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::VestingWalletRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let wallets = state
        .reader
        .get_user_vesting_wallets(&address, limit, offset)
        .await?;
    Ok(Json(wallets))
}

async fn list_vesting_events(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::VestingEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let events = state.reader.list_vesting_events(limit, offset).await?;
    Ok(Json(events))
}

async fn get_vesting_analytics(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let analytics = state.reader.get_vesting_analytics().await?;
    Ok(Json(analytics))
}

async fn get_vesting_leaderboard(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::VestingWalletRow>>, SocialError> {
    let limit = params.limit();
    let wallets = state.reader.get_vesting_leaderboard(limit).await?;
    Ok(Json(wallets))
}

async fn list_profile_subscription_services(
    State(state): State<Arc<AppState>>,
    Path(owner): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfileSubscriptionServiceInfo>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_profile_subscription_services_by_owner(&owner, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_profile_subscription_service(
    State(state): State<Arc<AppState>>,
    Path(service_id): Path<String>,
) -> Result<Json<crate::reader::ProfileSubscriptionServiceInfo>, SocialError> {
    let service = state
        .reader
        .get_profile_subscription_service(&service_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Subscription service '{}'", service_id)))?;
    Ok(Json(service))
}

async fn list_subscriptions_by_subscriber(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfileSubscriptionInfo>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_active_subscriptions_by_subscriber(&address, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_subscription_by_id(
    State(state): State<Arc<AppState>>,
    Path(subscription_id): Path<String>,
) -> Result<Json<crate::reader::ProfileSubscriptionInfo>, SocialError> {
    let subscription = state
        .reader
        .get_subscription_by_id(&subscription_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Subscription '{}'", subscription_id)))?;
    Ok(Json(subscription))
}

async fn get_subscription_revenue_by_service(
    State(state): State<Arc<AppState>>,
    Path(service_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::ProfileSubscriptionRevenueRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_subscription_revenue_by_service(&service_id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn check_subscription_access(
    State(state): State<Arc<AppState>>,
    Path((subscriber, service_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let has_access = state
        .reader
        .check_subscription_access(&subscriber, &service_id)
        .await?;
    Ok(Json(serde_json::json!({ "has_access": has_access })))
}

async fn list_mydata(
    State(state): State<Arc<AppState>>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataBasic>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_mydata(
            limit,
            offset,
            params.creator.as_deref(),
            params.media_type.as_deref(),
            params.platform_id.as_deref(),
            params.sort_by.as_deref(),
        )
        .await?;
    Ok(Json(data))
}

async fn get_mydata_configuration(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::MyDataConfigInfo>, SocialError> {
    let config = state
        .reader
        .get_mydata_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("MyData configuration".to_string()))?;
    Ok(Json(config))
}

async fn get_popular_mydata(
    State(state): State<Arc<AppState>>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataBasic>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state.reader.get_popular_mydata(limit, offset).await?;
    Ok(Json(data))
}

async fn get_mydata_by_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::MyDataBasic>, SocialError> {
    let data = state
        .reader
        .get_mydata_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("MyData '{}'", id)))?;
    Ok(Json(data))
}

async fn get_mydata_purchases(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::PurchaseInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .get_mydata_purchases(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_mydata_subscriptions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::SubscriptionInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .get_mydata_subscriptions(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_mydata_revenue(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::RevenueInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(30).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state.reader.get_mydata_revenue(&id, limit, offset).await?;
    Ok(Json(data))
}

async fn get_mydata_access_logs(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::AccessLogInfo>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(200);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .get_mydata_access_logs(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_creator_mydata(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<MyDataQuery>,
) -> Result<Json<Vec<crate::reader::MyDataBasic>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state.reader.get_creator_mydata(&id, limit, offset).await?;
    Ok(Json(data))
}

async fn get_mydata_stats(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::MyDataStatsResponse>, SocialError> {
    let stats = state
        .reader
        .get_mydata_stats(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("MyData '{}'", id)))?;
    Ok(Json(stats))
}

async fn get_mydata_revenue_timeline(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<crate::reader::DailyRevenue>>, SocialError> {
    let data = state.reader.get_mydata_revenue_timeline(&id).await?;
    Ok(Json(data))
}

async fn get_mydata_access_analytics(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<crate::reader::AccessAnalytics>>, SocialError> {
    let data = state.reader.get_mydata_access_analytics(&id).await?;
    Ok(Json(data))
}

async fn get_insurance_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::InsuranceConfigInfo>, SocialError> {
    let config = state
        .reader
        .get_insurance_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("Insurance configuration".to_string()))?;
    Ok(Json(config))
}

async fn list_insurance_vaults(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::InsuranceVaultRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.list_insurance_vaults(limit, offset).await?;
    Ok(Json(data))
}

async fn get_insurance_vault(
    State(state): State<Arc<AppState>>,
    Path(vault_id): Path<String>,
) -> Result<Json<crate::reader::InsuranceVaultInfo>, SocialError> {
    let vault = state
        .reader
        .get_insurance_vault(&vault_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Insurance vault '{}'", vault_id)))?;
    Ok(Json(vault))
}

async fn list_insurance_vault_transactions(
    State(state): State<Arc<AppState>>,
    Path(vault_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::InsuranceVaultTransactionRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_insurance_vault_transactions(&vault_id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_insurance_vault_exposures(
    State(state): State<Arc<AppState>>,
    Path(vault_id): Path<String>,
) -> Result<Json<Vec<crate::reader::InsuranceVaultExposureRow>>, SocialError> {
    let data = state
        .reader
        .get_insurance_vault_exposures(&vault_id)
        .await?;
    Ok(Json(data))
}

async fn list_insurance_policies(
    State(state): State<Arc<AppState>>,
    Query(filters): Query<InsurancePolicyFilters>,
) -> Result<Json<Vec<crate::reader::InsurancePolicyRow>>, SocialError> {
    let limit = filters.limit.unwrap_or(20).min(100);
    let page = filters.page.unwrap_or(1).max(1);
    let offset = filters.offset.unwrap_or_else(|| (page - 1) * limit);
    let data = state
        .reader
        .list_insurance_policies(
            filters.insured.as_deref(),
            filters.market_id.as_deref(),
            filters.vault_id.as_deref(),
            filters.status,
            limit,
            offset,
        )
        .await?;
    Ok(Json(data))
}

async fn get_insurance_policy(
    State(state): State<Arc<AppState>>,
    Path(policy_id): Path<String>,
) -> Result<Json<crate::reader::InsurancePolicyInfo>, SocialError> {
    let policy = state
        .reader
        .get_insurance_policy(&policy_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Insurance policy '{}'", policy_id)))?;
    Ok(Json(policy))
}

async fn list_insurance_market_policies(
    State(state): State<Arc<AppState>>,
    Path(market_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::InsurancePolicyRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_insurance_market_policies(&market_id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_spot_record(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
) -> Result<Json<crate::reader::SpotRecordResponse>, SocialError> {
    let record = state
        .reader
        .get_spot_record(&post_id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("SPoT record '{}'", post_id)))?;
    Ok(Json(record))
}

async fn list_spot_bets(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SpotBetRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.list_spot_bets(&post_id, limit, offset).await?;
    Ok(Json(data))
}

async fn list_spot_payouts(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SpotTransferRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_spot_payouts(&post_id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn list_spot_refunds(
    State(state): State<Arc<AppState>>,
    Path(post_id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SpotTransferRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_spot_refunds(&post_id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_spot_configuration(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::SpotConfigInfo>, SocialError> {
    let config = state
        .reader
        .get_spot_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("SPoT configuration".to_string()))?;
    Ok(Json(config))
}

async fn list_spt_pools(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SptPoolsQuery>,
) -> Result<Json<Vec<crate::reader::SptPoolRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_spt_pools(limit, offset, params.owner.as_deref(), params.token_type)
        .await?;
    Ok(Json(data))
}

async fn get_spt_pool_by_associated_id(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::SptPoolRow>, SocialError> {
    let pool = state
        .reader
        .get_spt_pool_by_associated_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("SPT pool for associated ID '{}'", id)))?;
    Ok(Json(pool))
}

async fn get_spt_popular(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptPoolRow>>, SocialError> {
    let limit = params.limit();
    let pools = state.reader.get_spt_popular(limit).await?;
    Ok(Json(pools))
}

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub limit: Option<i64>,
}

async fn get_spt_user_holdings(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<(String, i64, i64)>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let holdings = state
        .reader
        .get_spt_user_holdings(&address, limit, offset)
        .await?;
    Ok(Json(holdings))
}

async fn get_spt_user_reservations(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<(String, i64, i64)>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let reservations = state
        .reader
        .get_spt_user_reservations(&address, limit, offset)
        .await?;
    Ok(Json(reservations))
}

async fn search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let q = params.q.as_deref().unwrap_or("").trim();
    if q.is_empty() {
        return Ok(Json(serde_json::json!({
            "profiles": [],
            "posts": [],
            "platforms_count": 0,
        })));
    }
    let limit = params.limit.unwrap_or(20).min(100);
    let results = state.reader.search(q, limit).await?;
    Ok(Json(results))
}

async fn get_spt_pool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::SptPoolRow>, SocialError> {
    let pool = state
        .reader
        .get_spt_pool(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("SPT pool '{}'", id)))?;
    Ok(Json(pool))
}

async fn get_spt_pool_transactions(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptTransactionRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_spt_transactions(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_spt_pool_holdings(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptHoldingRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.get_spt_holdings(&id, limit, offset).await?;
    Ok(Json(data))
}

async fn get_spt_pool_price_history(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptPriceHistoryRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_spt_price_history(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_spt_pool_revenue(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptRevenueRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.get_spt_revenue(&id, limit, offset).await?;
    Ok(Json(data))
}

async fn get_revenue_dashboard(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let data = state.reader.get_revenue_dashboard().await?;
    Ok(Json(data))
}

async fn get_revenue_leaderboard(
    State(state): State<Arc<AppState>>,
    Query(params): Query<RevenueLeaderboardQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let min_revenue = params.min_revenue.unwrap_or(0);
    let leaderboard = state
        .reader
        .get_revenue_leaderboard(limit, min_revenue, params.revenue_source.as_deref())
        .await?;
    Ok(Json(serde_json::json!({
        "leaderboard": leaderboard,
        "min_revenue": min_revenue,
        "limit": limit
    })))
}

async fn get_revenue_chart_data(
    State(state): State<Arc<AppState>>,
    Query(params): Query<RevenueQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let period = params.period.as_deref().unwrap_or("day");
    let points = params.points.unwrap_or(30);
    let end_date = params
        .end_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|| chrono::Utc::now().naive_utc());
    let start_date = params
        .start_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|| {
            end_date
                - match period {
                    "hour" => chrono::Duration::hours(points),
                    "day" => chrono::Duration::days(points),
                    "week" => chrono::Duration::weeks(points),
                    "month" => chrono::Duration::days(points * 30),
                    _ => chrono::Duration::days(points),
                }
        });
    let chart_data = state
        .reader
        .get_revenue_chart_data(
            params.creator_address.as_deref(),
            period,
            start_date,
            end_date,
            points,
        )
        .await?;
    Ok(Json(serde_json::json!({
        "chart_data": chart_data,
        "period": period,
        "start_date": start_date,
        "end_date": end_date
    })))
}

async fn get_unified_revenue(
    State(state): State<Arc<AppState>>,
    Query(params): Query<RevenueQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let start_date = params
        .start_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc());
    let end_date = params
        .end_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc());
    let (records, total_count, total_amount) = state
        .reader
        .get_unified_revenue(
            params.creator_address.as_deref(),
            params.platform_address.as_deref(),
            params.revenue_source.as_deref(),
            params.revenue_type.as_deref(),
            params.content_id.as_deref(),
            params.content_type.as_deref(),
            start_date,
            end_date,
            limit,
            offset,
        )
        .await?;
    let total_pages = if limit > 0 {
        ((total_count + limit - 1) / limit).max(1)
    } else {
        1
    };
    let format_myso = |v: i64| format!("{:.9}", v as f64 / 1_000_000_000.0);
    Ok(Json(serde_json::json!({
        "revenue_records": records,
        "total_count": total_count,
        "total_amount": total_amount,
        "total_amount_formatted": format_myso(total_amount),
        "pagination": {
            "total": total_count,
            "limit": limit,
            "offset": offset,
            "page": (offset / limit) + 1,
            "total_pages": total_pages
        }
    })))
}

async fn get_creator_revenue_stats(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let stats = state
        .reader
        .get_creator_revenue_stats(&address)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Creator '{}'", address)))?;
    Ok(Json(stats))
}

async fn get_platform_revenue_stats(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let stats = state.reader.get_platform_revenue_stats(&address).await?;
    Ok(Json(stats))
}

async fn get_treasury_current(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let treasury = state
        .reader
        .get_current_treasury()
        .await?
        .ok_or_else(|| SocialError::not_found("Treasury".to_string()))?;
    Ok(Json(treasury))
}

async fn get_subscription_analytics(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SubscriptionAnalyticsQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let end_date = params
        .end_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|| chrono::Utc::now().naive_utc());
    let start_date = params
        .start_date
        .as_ref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.naive_utc())
        .unwrap_or_else(|| end_date - chrono::Duration::days(30));
    let analytics = state
        .reader
        .get_subscription_analytics(
            params.service_id.as_deref(),
            params.profile_owner.as_deref(),
            start_date,
            end_date,
        )
        .await?;
    Ok(Json(serde_json::json!({
        "analytics": analytics,
        "period_start": start_date,
        "period_end": end_date
    })))
}

async fn get_service_performance(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SubscriptionAnalyticsQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let services = state
        .reader
        .get_service_performance(params.profile_owner.as_deref())
        .await?;
    Ok(Json(serde_json::json!({
        "total_count": services.len() as i64,
        "services": services
    })))
}

async fn get_treasury_history(
    State(state): State<Arc<AppState>>,
    Query(params): Query<TreasuryHistoryQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit.unwrap_or(100).min(100);
    let history = state.reader.get_treasury_history(limit).await?;
    Ok(Json(serde_json::json!({
        "history": history,
        "count": history.len()
    })))
}

async fn get_spt_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::SptExchangeConfigRow>, SocialError> {
    let config = state
        .reader
        .get_spt_exchange_config()
        .await?
        .ok_or_else(|| SocialError::not_found("SPT exchange configuration".to_string()))?;
    Ok(Json(config))
}

async fn get_spt_analytics_top_performers(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<serde_json::Value>>, SocialError> {
    let data = state.reader.get_spt_analytics_top_performers().await?;
    Ok(Json(data))
}

async fn get_spt_portfolio_performance(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let data = state.reader.get_spt_portfolio_performance(&address).await?;
    Ok(Json(data))
}

async fn get_spt_creator_revenue_streams(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
    Query(time_range): Query<TimeRangeParams>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let from_ts = time_range
        .from
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
        .unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(30));
    let to_ts = time_range
        .to
        .and_then(|ts| chrono::DateTime::from_timestamp(ts, 0))
        .unwrap_or_else(chrono::Utc::now);
    let data = state
        .reader
        .get_spt_creator_revenue_streams(&address, from_ts, to_ts)
        .await?;
    Ok(Json(data))
}

async fn get_spt_market_sentiment(
    State(state): State<Arc<AppState>>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let data = state.reader.get_spt_market_sentiment().await?;
    Ok(Json(data))
}

async fn get_spt_liquidity_profile(
    State(state): State<Arc<AppState>>,
    Path(pool_id): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let data = state.reader.get_spt_liquidity_profile(&pool_id).await?;
    Ok(Json(data))
}

async fn list_spt_reservation_pools(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let (pools, total) = state
        .reader
        .list_spt_reservation_pools(limit, offset)
        .await?;
    let total_pages = if limit > 0 {
        ((total + limit - 1) / limit).max(1)
    } else {
        1
    };
    Ok(Json(serde_json::json!({
        "data": pools,
        "pagination": {
            "page": (offset / limit) + 1,
            "limit": limit,
            "total": total,
            "total_pages": total_pages
        }
    })))
}

async fn get_spt_reservation_pool(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::SptReservationPoolRow>, SocialError> {
    let pool = state
        .reader
        .get_spt_reservation_pool(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("SPT reservation pool '{}'", id)))?;
    Ok(Json(pool))
}

async fn list_spt_reservation_pool_reservations(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::SptReservationRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .list_spt_reservations(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn list_governance_proposals(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GovernanceProposalQuery>,
) -> Result<Json<Vec<crate::reader::ProposalRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_proposals(
            limit,
            offset,
            params.status,
            params.proposal_type,
            params.submitter.as_deref(),
        )
        .await?;
    Ok(Json(data))
}

async fn get_governance_proposal(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let proposal = state
        .reader
        .get_proposal_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Proposal '{}'", id)))?;
    let delegate_votes = state.reader.get_proposal_delegate_votes(&id).await?;
    let community_votes_count = state.reader.get_proposal_community_votes_count(&id).await?;
    let reward_distributions = state.reader.get_proposal_reward_distributions(&id).await?;
    Ok(Json(serde_json::json!({
        "proposal": proposal,
        "delegate_votes": delegate_votes,
        "community_votes_count": community_votes_count,
        "reward_distributions": reward_distributions
    })))
}

async fn get_governance_proposal_community_votes(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::CommunityVoteRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_proposal_community_votes(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn list_governance_delegates(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GovernanceDelegateQuery>,
) -> Result<Json<Vec<crate::reader::DelegateRow>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_delegates(limit, offset, params.registry_type, params.is_active)
        .await?;
    Ok(Json(data))
}

async fn get_governance_delegate(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<crate::reader::DelegateRow>, SocialError> {
    let delegate = state
        .reader
        .get_delegate_by_address(&address)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Delegate '{}'", address)))?;
    Ok(Json(delegate))
}

async fn get_governance_delegate_proposals(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<Vec<crate::reader::ProposalRow>>, SocialError> {
    let data = state.reader.get_delegate_proposals(&address).await?;
    Ok(Json(data))
}

async fn get_governance_delegate_ratings(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<Vec<crate::reader::DelegateRatingRow>>, SocialError> {
    let data = state.reader.get_delegate_ratings(&address).await?;
    Ok(Json(data))
}

async fn list_governance_nominees(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GovernanceNomineeQuery>,
) -> Result<Json<Vec<crate::reader::NominatedDelegateRow>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_nominees(limit, offset, params.registry_type, params.status)
        .await?;
    Ok(Json(data))
}

async fn list_governance_registries(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::reader::GovernanceRegistryRow>>, SocialError> {
    let data = state.reader.list_governance_registries().await?;
    Ok(Json(data))
}

async fn get_governance_registry(
    State(state): State<Arc<AppState>>,
    Path(registry_type): Path<String>,
) -> Result<Json<crate::reader::GovernanceRegistryRow>, SocialError> {
    let registry_type: i16 = registry_type
        .parse()
        .map_err(|_| SocialError::bad_request("Invalid registry_type"))?;
    let registry = state
        .reader
        .get_governance_registry_by_type(registry_type)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Registry type '{}'", registry_type)))?;
    Ok(Json(registry))
}

async fn list_governance_events(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::GovernanceEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.list_governance_events(limit, offset).await?;
    Ok(Json(data))
}

async fn list_upgrade_events(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::UpgradeEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.get_upgrade_events(limit, offset).await?;
    Ok(Json(data))
}

async fn list_object_migrated_events(
    State(state): State<Arc<AppState>>,
    Query(params): Query<UpgradeMigrationsQuery>,
) -> Result<Json<Vec<crate::reader::ObjectMigratedEventRow>>, SocialError> {
    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let page = params.page.unwrap_or(1).max(1);
    let offset = if page > 1 { (page - 1) * limit } else { offset };
    let data = state
        .reader
        .get_object_migrated_events(limit, offset, params.object_id.as_deref())
        .await?;
    Ok(Json(data))
}

async fn get_governance_proposal_anonymous_stats(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::AnonymousVotingStatsRow>, SocialError> {
    let stats = state
        .reader
        .get_proposal_anonymous_stats(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Anonymous stats for proposal '{}'", id)))?;
    Ok(Json(stats))
}

async fn get_governance_proposal_anonymous_votes(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::AnonymousVoteRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_proposal_anonymous_votes(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

async fn get_governance_proposal_decryption_failures(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<crate::reader::VoteDecryptionFailureRow>>, SocialError> {
    let data = state.reader.get_proposal_decryption_failures(&id).await?;
    Ok(Json(data))
}

async fn get_governance_anonymous_voting_trends(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::AnonymousVotingTrendRow>>, SocialError> {
    let limit = params.limit().min(90);
    let data = state.reader.get_anonymous_voting_trends(limit).await?;
    Ok(Json(data))
}
