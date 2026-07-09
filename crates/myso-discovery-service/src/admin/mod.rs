// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Secret-gated admin routes: exclude assets and request source replay.

use std::sync::Arc;

use axum::extract::State;
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::lifecycle::{AssetLifecycleState, LifecycleEvent};
use crate::runtime::AppState;

fn check_admin_secret(headers: &HeaderMap, expected: Option<&str>) -> bool {
    match expected {
        None => false,
        Some(secret) => headers
            .get("x-discovery-admin-secret")
            .and_then(|v| v.to_str().ok())
            == Some(secret),
    }
}

#[derive(Debug, Deserialize)]
pub struct ExcludeAssetRequest {
    pub discovery_asset_id: Uuid,
    pub reason: String,
    #[serde(default)]
    pub requested_by: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ExcludeAssetResponse {
    pub discovery_asset_id: Uuid,
    pub lifecycle_state: String,
}

pub async fn exclude_asset(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<ExcludeAssetRequest>,
) -> Result<Json<ExcludeAssetResponse>, StatusCode> {
    if !check_admin_secret(&headers, state.args.admin_secret.as_deref()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if body.reason.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    state
        .store
        .insert_exclusion(
            "discovery_asset",
            body.discovery_asset_id,
            &body.reason,
            body.requested_by.as_deref(),
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let state_after = state
        .store
        .transition_asset(body.discovery_asset_id, LifecycleEvent::Exclude)
        .await
        .unwrap_or(AssetLifecycleState::Excluded);
    Ok(Json(ExcludeAssetResponse {
        discovery_asset_id: body.discovery_asset_id,
        lifecycle_state: state_after.as_str().to_string(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct ReplaySourceRequest {
    /// YAML source string id (e.g. `coindesk-rss`), not the UUID PK.
    pub source_id: String,
}

#[derive(Debug, Serialize)]
pub struct ReplaySourceResponse {
    pub source_id: String,
    pub source_db_id: Uuid,
    pub queued: bool,
}

pub async fn replay_source(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<ReplaySourceRequest>,
) -> Result<Json<ReplaySourceResponse>, StatusCode> {
    if !check_admin_secret(&headers, state.args.admin_secret.as_deref()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    if body.source_id.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }
    let source_db_id = state
        .store
        .find_source_db_id(&body.source_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    state
        .store
        .request_source_replay(source_db_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ReplaySourceResponse {
        source_id: body.source_id,
        source_db_id,
        queued: true,
    }))
}
