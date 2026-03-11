// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams, UpgradeMigrationsQuery};

pub async fn list_upgrade_events(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::UpgradeEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.get_upgrade_events(limit, offset).await?;
    Ok(Json(data))
}

pub async fn list_object_migrated_events(
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
