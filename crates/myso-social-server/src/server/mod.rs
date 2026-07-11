// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

mod auth;
mod handlers;

use axum::http::Method;
use axum::middleware;
use axum::routing::{get, post};
use axum::Router;
use myso_indexer_alt_metrics::{MetricsArgs, MetricsService};
use myso_pg_db::DbArgs;
use prometheus::Registry;
use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::oneshot;
use tower_http::cors::{AllowMethods, Any, CorsLayer};
use url::Url;

use crate::reader::Reader;
use crate::workflow_client::WorkflowClient;
use auth::{
    org_auditor_access_middleware, org_dashboard_access_middleware, wallet_auth_middleware,
    DEFAULT_WALLET_AUTH_TTL_SECONDS,
};
use myso_futures::service::Service;

#[derive(Clone)]
pub struct AppState {
    pub(crate) reader: Reader,
    pub(crate) workflow: Option<WorkflowClient>,
    pub(crate) wallet_auth_ttl_seconds: i64,
}

fn wallet_auth_ttl_seconds() -> i64 {
    std::env::var("WALLET_AUTH_TTL_SECONDS")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(DEFAULT_WALLET_AUTH_TTL_SECONDS)
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
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }
    pub fn offset(&self) -> i64 {
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
    pub platform_id: Option<String>,
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
pub struct GovernanceDelegateGetQuery {
    pub registry_type: Option<i16>,
    pub governance_registry_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct GovernanceNomineeQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub platform_id: Option<String>,
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
pub struct SptUserHoldingsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    #[serde(default)]
    pub include_reservations: Option<bool>,
}

impl SptUserHoldingsQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(50).min(100)
    }
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let limit = self.limit();
        self.offset.unwrap_or_else(|| (page - 1) * limit)
    }
}

#[derive(Debug, Deserialize)]
pub struct SptReservationVolumeQuery {
    /// `hour` or `day` (default `hour`).
    pub interval: Option<String>,
    pub limit: Option<i64>,
    /// Start of range (epoch milliseconds, UTC).
    pub from: Option<i64>,
    /// End of range (epoch milliseconds, UTC).
    pub to: Option<i64>,
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
    pub governance: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct ModeratorsQuery {
    pub permission: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

impl ModeratorsQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).min(100)
    }

    pub fn offset(&self) -> i64 {
        self.offset
            .unwrap_or_else(|| (self.page.unwrap_or(1).max(1) - 1) * self.limit())
    }
}

#[derive(Debug, Deserialize)]
pub struct PostsQuery {
    pub owner: Option<String>,
    pub post_type: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
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
    pub owner_address: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

impl VestingWalletsQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(50).min(100)
    }
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let limit = self.limit();
        self.offset.unwrap_or_else(|| (page - 1) * limit)
    }
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
}

#[derive(Debug, Deserialize)]
pub struct VestingEventsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    pub owner_address: Option<String>,
}

impl VestingEventsQuery {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(50).min(100)
    }
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let limit = self.limit();
        self.offset.unwrap_or_else(|| (page - 1) * limit)
    }
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
}

#[derive(Debug, Deserialize)]
pub struct VestingPageParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
}

impl VestingPageParams {
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(50).min(100)
    }
    pub fn offset(&self) -> i64 {
        let page = self.page.unwrap_or(1).max(1);
        let limit = self.limit();
        self.offset.unwrap_or_else(|| (page - 1) * limit)
    }
    pub fn page(&self) -> i64 {
        self.page.unwrap_or(1).max(1)
    }
}

/// Build and return the social API server as a Service without running it.
/// Callers can merge this into a larger Service and run it together.
pub async fn start_server(
    server_port: u16,
    database_url: Url,
    write_database_url: Option<Url>,
    db_args: DbArgs,
    metrics_address: std::net::SocketAddr,
    registry: &Registry,
) -> Result<Service, anyhow::Error> {
    let metrics = MetricsService::new(MetricsArgs { metrics_address }, registry.clone());

    let reader = Reader::new(database_url, write_database_url, db_args).await?;
    let workflow = WorkflowClient::from_env();
    let state = Arc::new(AppState {
        reader,
        workflow,
        wallet_auth_ttl_seconds: wallet_auth_ttl_seconds(),
    });

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
    write_database_url: Option<Url>,
    db_args: DbArgs,
    metrics_address: std::net::SocketAddr,
) -> Result<(), anyhow::Error> {
    let registry = Registry::new_custom(Some("social_api".into()), None)
        .expect("Failed to create Prometheus registry.");

    let service = start_server(
        server_port,
        database_url,
        write_database_url,
        db_args,
        metrics_address,
        &registry,
    )
    .await?;

    println!("Social API server started on port {}", server_port);

    service.main().await?;

    Ok(())
}

fn enterprise_dashboard_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    use handlers::{
        list_org_invitations, list_org_memory_permissions, list_org_role_assignments,
        list_org_roles, list_org_spend_approvals, list_org_spend_breakdown,
    };

    Router::new()
        .route(
            "/organizations/:id/memory-permissions",
            get(list_org_memory_permissions),
        )
        .route("/organizations/:id/roles", get(list_org_roles))
        .route(
            "/organizations/:id/role-assignments",
            get(list_org_role_assignments),
        )
        .route("/organizations/:id/invitations", get(list_org_invitations))
        .route(
            "/organizations/:id/spend-breakdown",
            get(list_org_spend_breakdown),
        )
        .route(
            "/organizations/:id/approvals",
            get(list_org_spend_approvals),
        )
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            org_dashboard_access_middleware,
        ))
        .route_layer(middleware::from_fn_with_state(
            state,
            wallet_auth_middleware,
        ))
}

fn enterprise_auditor_routes(state: Arc<AppState>) -> Router<Arc<AppState>> {
    use handlers::list_org_audit_logs;

    Router::new()
        .route("/organizations/:id/audit-logs", get(list_org_audit_logs))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            org_auditor_access_middleware,
        ))
        .route_layer(middleware::from_fn_with_state(
            state,
            wallet_auth_middleware,
        ))
}

fn make_router(state: Arc<AppState>) -> Router {
    use handlers::*;

    let enterprise_dashboard = enterprise_dashboard_routes(state.clone());
    let enterprise_auditor = enterprise_auditor_routes(state.clone());

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
        .route("/profiles/daily-stats", get(get_profile_daily_stats_chart))
        .route("/profiles/address/:address", get(get_profile_by_address))
        .route(
            "/wallets/:address/messaging-policy",
            get(get_wallet_messaging_policy),
        )
        .route("/profiles/username/:username", get(get_profile_by_username))
        .route(
            "/profiles/username/:username/availability",
            get(check_username_availability),
        )
        .route("/profiles/:address/posts", get(get_profile_posts))
        .route(
            "/profiles/:address/sub-agents",
            get(list_profile_sub_agents),
        )
        .route(
            "/profiles/:address/memory-account",
            get(get_profile_memory_account),
        )
        .route(
            "/profiles/:address/ai-credit",
            get(get_profile_ai_credit_balance),
        )
        .route(
            "/profiles/:address/ai-credit/approvals",
            get(list_profile_spend_approvals),
        )
        .route(
            "/profiles/:address/audit-logs",
            get(list_profile_audit_logs),
        )
        .route("/ai-credit/config", get(get_ai_credit_config))
        .route(
            "/ai-credit/:balance_id/usage-history",
            get(list_ai_credit_usage_history),
        )
        .route(
            "/internal/ai-credit/usage-lines",
            post(ingest_usage_line_internal),
        )
        .route(
            "/internal/ai-credit/approvals",
            post(ingest_approval_internal),
        )
        .route(
            "/internal/memory/usage-stats",
            post(ingest_memory_usage_stats_internal),
        )
        .route(
            "/internal/memory/access-requests",
            post(ingest_memory_access_request_internal),
        )
        .route(
            "/internal/organizations/:id/summary",
            get(get_org_summary_internal),
        )
        .route("/internal/audit/logs", post(ingest_audit_logs_internal))
        .route("/sub-agents/:derivedAddress", get(get_sub_agent))
        .route(
            "/sub-agents/by-object/:agentObjectId",
            get(get_sub_agent_by_object_id),
        )
        .route(
            "/sub-agents/:agentObjectId/children",
            get(list_sub_agent_children),
        )
        .route(
            "/organizations/categories",
            get(list_organization_categories),
        )
        .route(
            "/organizations/leaderboard",
            get(get_organization_leaderboard),
        )
        .route("/organizations/:id", get(get_agentic_organization))
        .route(
            "/organizations/:id/statistics",
            get(get_organization_statistics),
        )
        .merge(enterprise_dashboard)
        .merge(enterprise_auditor)
        .route(
            "/profiles/:address/organizations",
            get(list_profile_organizations),
        )
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
            "/profiles/:address/recommendations",
            get(get_profile_recommendations),
        )
        .route(
            "/profiles/:address/social-stats",
            get(get_profile_social_stats),
        )
        .route("/profiles/:address/stats", get(get_profile_social_stats))
        .route("/profiles/:address/blocked", get(get_profile_blocked))
        .route(
            "/profiles/:address/blocked-platforms",
            get(get_profile_blocked_platforms),
        )
        .route("/usernames/:username/offers", get(get_username_offers))
        .route(
            "/profiles/:address/username-offers",
            get(get_profile_username_offers),
        )
        .route(
            "/profiles/:address/username-sale-fees",
            get(get_username_sale_fees),
        )
        .route("/profiles/:address/pnl", get(get_profile_pnl))
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
            "/blocklist/check/either/:a/:b",
            get(check_either_profile_blocked),
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
        .route(
            "/platforms/:id/user-access/:user_address",
            get(get_platform_user_access),
        )
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
        .route("/posts/:id/transfers", get(get_post_transfers))
        .route("/posts/:id/reports", get(get_post_reports))
        .route(
            "/posts/:id/moderation-events",
            get(get_post_moderation_events),
        )
        .route("/posts/:id/deletion-events", get(get_post_deletion_events))
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
        .route(
            "/poc/username-beneficiaries/by-username/:username",
            get(get_poc_username_beneficiary_by_username),
        )
        .route(
            "/poc/username-beneficiaries/:id",
            get(get_poc_username_beneficiary_by_id),
        )
        .route(
            "/poc/username-beneficiaries",
            get(list_poc_username_beneficiaries),
        )
        .route(
            "/poc/beneficiary-vaults/by-beneficiary/:address",
            get(get_poc_beneficiary_vault_by_beneficiary),
        )
        .route(
            "/poc/beneficiary-vaults/:vault_id/coin-balances",
            get(list_poc_beneficiary_vault_coin_balances),
        )
        .route(
            "/poc/beneficiary-vaults/:vault_id/deposits",
            get(list_poc_vault_deposits),
        )
        .route(
            "/poc/beneficiary-vaults/:vault_id/claims",
            get(list_poc_vault_claims),
        )
        .route(
            "/poc/beneficiary-vaults/:vault_id",
            get(get_poc_beneficiary_vault_by_vault_id),
        )
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
            "/subscription-services/:service_id/plans",
            get(list_profile_subscription_plans),
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
        .route("/mydata/broad-pools", get(list_mydata_broad_pools))
        .route(
            "/mydata/broad-pools/:pool_id/sub-pools",
            get(list_mydata_sub_pools_for_broad_pool),
        )
        .route(
            "/mydata/sub-pools/:sub_pool_id/listings",
            get(list_mydata_listings_for_sub_pool),
        )
        .route(
            "/mydata/snapshots/:snapshot_id/anchor",
            get(get_mydata_snapshot_anchor),
        )
        .route(
            "/mydata/snapshots/:snapshot_id/distribution",
            get(get_mydata_distribution_round),
        )
        .route(
            "/mydata/distribution-rounds",
            get(list_mydata_distribution_rounds),
        )
        .route(
            "/mydata/snapshots/:snapshot_id/merkle-root",
            get(get_mydata_merkle_root),
        )
        .route(
            "/mydata/snapshots/:snapshot_id/escrow",
            get(get_mydata_snapshot_escrow),
        )
        .route(
            "/mydata/snapshots/:snapshot_id/claims",
            get(list_mydata_claims_for_snapshot),
        )
        .route("/mydata/:id", get(get_mydata_by_id))
        .route(
            "/mydata/:id/sub-pools",
            get(list_mydata_sub_pools_for_mydata_listing),
        )
        .route(
            "/mydata/:id/has-access/:address",
            get(get_mydata_has_access),
        )
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
        .route("/insurance/configuration", get(get_insurance_config))
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
        .route("/spot/route/:post_id", get(get_spot_route))
        .route("/spot/creators/:address/pending", get(list_spot_pending_creator_payouts))
        .route("/spot/creators/:address/stats", get(get_spot_creator_stats))
        .route(
            "/spot/markets/:market_id/expired-creator-payouts",
            get(list_expired_spot_creator_payouts),
        )
        .route("/spot/contested-records", get(list_contested_spot_records))
        .route("/spot/records/:post_id", get(get_spot_record))
        .route("/spot/records/:post_id/bets", get(list_spot_bets))
        .route("/spot/records/:post_id/payouts", get(list_spot_payouts))
        .route("/spot/records/:post_id/refunds", get(list_spot_refunds))
        .route("/spot/configuration", get(get_spot_configuration))
        .route("/spot/pending-posts", get(list_pending_spot_posts))
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
        .route("/spt/configuration", get(get_spt_config))
        .route("/subscription/configuration", get(get_subscription_config))
        .route("/profile/configuration", get(get_profile_config))
        .route("/memory/configuration", get(get_memory_config))
        .route("/platform/configuration", get(get_platform_config))
        .route("/messaging/configuration", get(get_messaging_config))
        .route(
            "/wallets/:address/paid-messages",
            get(get_paid_message_history),
        )
        .route(
            "/wallets/:address/messaging-revenue",
            get(get_messaging_revenue_summary),
        )
        .route(
            "/organizations/:organization_id/messaging-groups",
            get(get_agent_groups),
        )
        .route("/spt/reservation-pools", get(list_spt_reservation_pools))
        .route("/spt/reservation-pools/:id", get(get_spt_reservation_pool))
        .route(
            "/spt/reservation-pools/:id/reservations",
            get(list_spt_reservation_pool_reservations),
        )
        .route(
            "/spt/reservation-pools/:id/volume-history",
            get(get_spt_reservation_pool_volume_history),
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
            "/governance/registries/platform/:platform_id",
            get(get_governance_registry_by_platform),
        )
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
