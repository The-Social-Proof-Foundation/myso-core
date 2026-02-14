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
