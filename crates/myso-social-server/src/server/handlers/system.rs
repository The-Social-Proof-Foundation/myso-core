// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::AppState;

#[derive(Debug, Deserialize)]
pub struct UsernameAvailabilityQuery {
    pub exclude_address: Option<String>,
}

pub async fn get_system_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::SystemStatsResponse>, SocialError> {
    let stats = state.reader.get_system_stats().await?;
    Ok(Json(stats))
}

pub async fn check_username_availability(
    State(state): State<Arc<AppState>>,
    Path(username): Path<String>,
    Query(query): Query<UsernameAvailabilityQuery>,
) -> Result<Json<myso_indexer_alt_social_reader::UsernameAvailabilityDetail>, SocialError> {
    let detail = state
        .reader
        .check_username_availability(&username, query.exclude_address.as_deref())
        .await?;
    Ok(Json(detail))
}
