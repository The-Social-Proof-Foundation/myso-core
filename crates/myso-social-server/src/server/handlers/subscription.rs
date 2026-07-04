// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use crate::error::SocialError;

use super::super::{AppState, PageParams, SubscriptionRevenueQuery, SubscriptionsQuery};

pub async fn list_subscriptions(
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

pub async fn list_subscription_services(
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

pub async fn list_subscription_revenue(
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

pub async fn get_subscription_status(
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

pub async fn get_subscriber_summary(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<crate::reader::SubscriberSummaryRow>, SocialError> {
    let summary = state.reader.get_subscriber_summary(&address).await?;
    Ok(Json(summary))
}

pub async fn list_profile_subscription_services(
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

pub async fn get_profile_subscription_service(
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

pub async fn list_subscriptions_by_subscriber(
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

pub async fn get_subscription_by_id(
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

pub async fn get_subscription_revenue_by_service(
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

pub async fn check_subscription_access(
    State(state): State<Arc<AppState>>,
    Path((subscriber, service_id)): Path<(String, String)>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let has_access = state
        .reader
        .check_subscription_access(&subscriber, &service_id)
        .await?;
    Ok(Json(serde_json::json!({ "has_access": has_access })))
}

pub async fn get_subscription_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<crate::reader::SubscriptionConfigInfo>, SocialError> {
    state
        .reader
        .get_subscription_configuration()
        .await?
        .ok_or_else(|| SocialError::not_found("Subscription configuration"))
        .map(Json)
}
