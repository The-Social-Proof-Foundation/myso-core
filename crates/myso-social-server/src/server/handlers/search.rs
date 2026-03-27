// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Query, State};
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::AppState;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: Option<String>,
    pub limit: Option<i64>,
}

pub async fn search(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let q = params.q.as_deref().unwrap_or("").trim();
    if q.is_empty() {
        return Ok(Json(serde_json::json!({
            "profiles": [],
            "posts": [],
            "platforms": [],
            "platforms_count": 0,
        })));
    }
    let limit = params.limit.unwrap_or(20).min(100);
    let results = state.reader.search(q, limit).await?;
    Ok(Json(results))
}
