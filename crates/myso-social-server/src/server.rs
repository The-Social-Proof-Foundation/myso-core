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
pub struct UpgradeMigrationsQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub page: Option<i64>,
    pub object_id: Option<String>,
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
        .route("/profiles", get(latest_profiles))
        .route("/profiles/address/:address", get(get_profile_by_address))
        .route("/profiles/username/:username", get(get_profile_by_username))
        .route("/profiles/:owner/subscription-services", get(list_profile_subscription_services))
        .route("/subscription-services/:service_id", get(get_profile_subscription_service))
        .route("/subscription-services/:service_id/revenue", get(get_subscription_revenue_by_service))
        .route("/subscriptions/subscriber/:address", get(list_subscriptions_by_subscriber))
        .route("/subscriptions/:subscription_id", get(get_subscription_by_id))
        .route("/subscription-access/:subscriber/:service_id", get(check_subscription_access))
        .route("/mydata", get(list_mydata))
        .route("/mydata/configuration", get(get_mydata_configuration))
        .route("/mydata/popular", get(get_popular_mydata))
        .route("/mydata/:id", get(get_mydata_by_id))
        .route("/mydata/:id/purchases", get(get_mydata_purchases))
        .route("/mydata/:id/subscriptions", get(get_mydata_subscriptions))
        .route("/mydata/:id/revenue", get(get_mydata_revenue))
        .route("/mydata/:id/access-logs", get(get_mydata_access_logs))
        .route("/mydata/:id/stats", get(get_mydata_stats))
        .route("/mydata/:id/revenue-timeline", get(get_mydata_revenue_timeline))
        .route("/mydata/:id/access-analytics", get(get_mydata_access_analytics))
        .route("/creators/:id/mydata", get(get_creator_mydata))
        .route("/insurance/config", get(get_insurance_config))
        .route("/insurance/vaults", get(list_insurance_vaults))
        .route("/insurance/vaults/:vault_id", get(get_insurance_vault))
        .route("/insurance/vaults/:vault_id/transactions", get(list_insurance_vault_transactions))
        .route("/insurance/vaults/:vault_id/exposures", get(get_insurance_vault_exposures))
        .route("/insurance/policies", get(list_insurance_policies))
        .route("/insurance/policies/:policy_id", get(get_insurance_policy))
        .route("/insurance/markets/:market_id/policies", get(list_insurance_market_policies))
        .route("/spot/records/:post_id", get(get_spot_record))
        .route("/spot/records/:post_id/bets", get(list_spot_bets))
        .route("/spot/records/:post_id/payouts", get(list_spot_payouts))
        .route("/spot/records/:post_id/refunds", get(list_spot_refunds))
        .route("/spot/configuration", get(get_spot_configuration))
        .route("/spt/pools", get(list_spt_pools))
        .route("/spt/pools/:id", get(get_spt_pool))
        .route("/spt/pools/:id/transactions", get(get_spt_pool_transactions))
        .route("/spt/pools/:id/holdings", get(get_spt_pool_holdings))
        .route("/spt/pools/:id/price-history", get(get_spt_pool_price_history))
        .route("/spt/pools/:id/revenue", get(get_spt_pool_revenue))
        .route("/spt/config", get(get_spt_config))
        .route("/spt/reservation-pools/:id", get(get_spt_reservation_pool))
        .route("/spt/reservation-pools/:id/reservations", get(list_spt_reservation_pool_reservations))
        .route("/governance/proposals", get(list_governance_proposals))
        .route("/governance/proposals/:id", get(get_governance_proposal))
        .route("/governance/proposals/:id/community-votes", get(get_governance_proposal_community_votes))
        .route("/governance/proposals/:id/anonymous-stats", get(get_governance_proposal_anonymous_stats))
        .route("/governance/proposals/:id/anonymous-votes", get(get_governance_proposal_anonymous_votes))
        .route("/governance/proposals/:id/decryption-failures", get(get_governance_proposal_decryption_failures))
        .route("/governance/delegates", get(list_governance_delegates))
        .route("/governance/delegates/:address", get(get_governance_delegate))
        .route("/governance/delegates/:address/proposals", get(get_governance_delegate_proposals))
        .route("/governance/delegates/:address/ratings", get(get_governance_delegate_ratings))
        .route("/governance/nominees", get(list_governance_nominees))
        .route("/governance/registries", get(list_governance_registries))
        .route("/governance/registries/:registry_type", get(get_governance_registry))
        .route("/governance/events", get(list_governance_events))
        .route("/governance/anonymous-voting/trends", get(get_governance_anonymous_voting_trends))
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
