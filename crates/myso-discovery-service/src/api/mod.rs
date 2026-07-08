// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::lifecycle::LifecycleEvent;
use crate::runtime::AppState;

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
}

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse { status: "ok" })
}

pub async fn ready(State(state): State<Arc<AppState>>) -> Result<Json<HealthResponse>, StatusCode> {
    sqlx::query("SELECT 1")
        .execute(state.store.pool())
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
    Ok(Json(HealthResponse { status: "ready" }))
}

pub async fn stats(State(state): State<Arc<AppState>>) -> Result<Json<crate::store::DiscoveryStats>, StatusCode> {
    state
        .store
        .stats()
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[derive(Debug, Deserialize)]
pub struct LifecycleCallback {
    pub discovery_asset_id: Uuid,
    pub event: String,
}

pub async fn lifecycle_callback(
    State(state): State<Arc<AppState>>,
    Json(body): Json<LifecycleCallback>,
) -> Result<StatusCode, StatusCode> {
    let event = parse_event(&body.event).ok_or(StatusCode::BAD_REQUEST)?;
    state
        .store
        .transition_asset(body.discovery_asset_id, event)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct ProvenanceHitRequest {
    pub network: String,
    pub post_id: String,
    pub query_media_id: Option<String>,
    pub discovery_asset_id: Option<Uuid>,
    pub creator_candidate_id: Option<Uuid>,
    pub similarity_score: f64,
    pub match_type: Option<String>,
    pub work_confidence: f64,
    pub creator_confidence: f64,
    pub decision: String,
    pub vault_provisioned: bool,
    pub vault_identity_hash: Option<String>,
}

pub async fn provenance_hit(
    State(state): State<Arc<AppState>>,
    Json(body): Json<ProvenanceHitRequest>,
) -> Result<StatusCode, StatusCode> {
    state
        .store
        .record_provenance_hit(
            &body.network,
            &body.post_id,
            body.query_media_id.as_deref(),
            body.discovery_asset_id,
            body.creator_candidate_id,
            body.similarity_score,
            body.match_type.as_deref(),
            body.work_confidence,
            body.creator_confidence,
            &body.decision,
            body.vault_provisioned,
            body.vault_identity_hash.as_deref(),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

fn parse_event(raw: &str) -> Option<LifecycleEvent> {
    match raw {
        "match_detected" => Some(LifecycleEvent::MatchDetected),
        "provenance_confirmed" => Some(LifecycleEvent::ProvenanceConfirmed),
        "vault_eligible" => Some(LifecycleEvent::VaultEligible),
        "vault_created" => Some(LifecycleEvent::VaultCreated),
        "claimed" => Some(LifecycleEvent::Claimed),
        _ => None,
    }
}
