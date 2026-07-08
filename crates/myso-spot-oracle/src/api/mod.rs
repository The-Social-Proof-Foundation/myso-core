// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::routing::{get, post};
use axum::{Json, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

use crate::config::OracleArgs;
use crate::metrics::OracleMetrics;
use crate::sources::ResolverRegistry;
use crate::store::OracleStore;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<OracleStore>,
    pub args: Arc<OracleArgs>,
    pub metrics: Arc<OracleMetrics>,
    pub sources: Arc<ResolverRegistry>,
    pub cancel: CancellationToken,
}

pub fn router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/v1/markets", get(list_markets))
        .route("/v1/markets/:id", get(get_market))
        .route("/v1/reviews/:id", get(get_review))
        .route("/v1/jobs", get(list_jobs))
        .route("/v1/evidence/:market_id", get(list_evidence))
        .route("/v1/sources/health", get(sources_health))
        .route("/v1/markets/:id/recheck", post(recheck_market))
        .with_state(state)
}

fn check_admin_secret(headers: &HeaderMap, expected: Option<&str>) -> bool {
    match expected {
        None => true,
        Some(secret) => headers
            .get("x-spot-oracle-admin-secret")
            .and_then(|v| v.to_str().ok())
            == Some(secret),
    }
}

async fn health(State(state): State<Arc<AppState>>) -> Json<HealthResponse> {
    let db_ok = sqlx::query("SELECT 1").execute(state.store.pool()).await.is_ok();
    Json(HealthResponse {
        status: if db_ok { "ok" } else { "degraded" },
        workers_enabled: state.args.enabled,
        db_ok,
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

async fn ready(State(state): State<Arc<AppState>>) -> Json<ReadyResponse> {
    let pool: &PgPool = state.store.pool();
    let pending = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM spot_jobs WHERE status = 'pending'",
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0);
    let processing = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM spot_jobs WHERE status = 'processing'",
    )
    .fetch_one(pool)
    .await
    .unwrap_or(0);
    state.metrics.queue_depth.with_label_values(&["pending"]).set(pending);
    state
        .metrics
        .queue_depth
        .with_label_values(&["processing"])
        .set(processing);
    Json(ReadyResponse {
        ready: true,
        pending_jobs: pending,
        processing_jobs: processing,
    })
}

#[derive(Debug, Deserialize)]
struct ListQuery {
    status: Option<String>,
    limit: Option<i64>,
}

async fn list_markets(
    State(state): State<Arc<AppState>>,
    Query(q): Query<ListQuery>,
) -> Json<serde_json::Value> {
    let limit = q.limit.unwrap_or(50).clamp(1, 500);
    let markets = state
        .store
        .list_markets(q.status.as_deref(), limit)
        .await
        .unwrap_or_default();
    Json(serde_json::json!({ "markets": markets }))
}

async fn get_market(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let market = state.store.get_market(id).await.ok().flatten();
    Json(serde_json::json!({ "market": market }))
}

async fn get_review(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let review = state.store.get_review(id).await.ok().flatten();
    Json(serde_json::json!({ "review": review }))
}

async fn list_jobs(
    State(state): State<Arc<AppState>>,
    Query(q): Query<ListQuery>,
) -> Json<serde_json::Value> {
    let limit = q.limit.unwrap_or(50).clamp(1, 500);
    let jobs = state
        .store
        .list_jobs(q.status.as_deref(), limit)
        .await
        .unwrap_or_default();
    Json(serde_json::json!({ "jobs": jobs }))
}

async fn list_evidence(
    State(state): State<Arc<AppState>>,
    Path(market_id): Path<Uuid>,
) -> Json<serde_json::Value> {
    let evidence = state
        .store
        .list_evidence_for_market(market_id)
        .await
        .unwrap_or_default();
    Json(serde_json::json!({ "evidence": evidence }))
}

async fn sources_health(State(state): State<Arc<AppState>>) -> Json<serde_json::Value> {
    let mut out = Vec::new();
    for src in state.sources.all() {
        let health = src.health().await;
        out.push(serde_json::json!({
            "id": src.id(),
            "healthy": health.healthy,
            "message": health.message,
        }));
    }
    Json(serde_json::json!({ "sources": out }))
}

async fn recheck_market(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
    headers: HeaderMap,
) -> Json<serde_json::Value> {
    if !check_admin_secret(&headers, state.args.admin_secret.as_deref()) {
        return Json(serde_json::json!({ "error": "unauthorized" }));
    }
    let market = match state.store.get_market(id).await.ok().flatten() {
        Some(m) => m,
        None => return Json(serde_json::json!({ "error": "not_found" })),
    };
    if let Some(def_id) = market.resolver_definition_id {
        let _ = state
            .store
            .enqueue_job(
                "ResolveMarket",
                Some(id),
                Some(def_id),
                300,
                chrono::Utc::now(),
                serde_json::json!({}),
            )
            .await;
    }
    Json(serde_json::json!({ "ok": true, "market_id": id }))
}

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub workers_enabled: bool,
    pub db_ok: bool,
    pub version: String,
}

#[derive(Debug, Serialize)]
pub struct ReadyResponse {
    pub ready: bool,
    pub pending_jobs: i64,
    pub processing_jobs: i64,
}
