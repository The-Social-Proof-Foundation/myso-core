// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use crate::error::SocialError;
use crate::reader::agent_task::{TaskFilter, TopAgentSort};
use crate::reader::{
    AgentReputationInfo, AgentTaskInfo, PrincipalReputationInfo, TaskConfigInfo, TaskDetailInfo,
    TaskReceiptInfo, TaskStatsInfo, TopAgentInfo,
};

use super::super::AppState;

#[derive(Debug, Deserialize)]
pub struct TaskListQuery {
    pub buyer_principal: Option<String>,
    pub buyer_agent_id: Option<String>,
    pub seller_principal: Option<String>,
    pub seller_agent_id: Option<String>,
    pub organization_id: Option<String>,
    pub room_group_id: Option<String>,
    /// Comma-separated status codes, e.g. `0,1`.
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

fn page(limit: Option<i64>, offset: Option<i64>) -> (i64, i64) {
    (
        limit.unwrap_or(50).clamp(1, 200),
        offset.unwrap_or(0).max(0),
    )
}

fn non_empty(v: Option<String>) -> Option<String> {
    v.filter(|s| !s.trim().is_empty())
}

/// `status=0,1` -> `[0, 1]`. Any entry outside the seven task states is a bad request.
fn parse_statuses(raw: Option<&str>) -> Result<Vec<i16>, SocialError> {
    let Some(raw) = raw.filter(|s| !s.trim().is_empty()) else {
        return Ok(Vec::new());
    };
    raw.split(',')
        .map(|part| {
            part.trim()
                .parse::<i16>()
                .ok()
                .filter(|v| (0..=6).contains(v))
                .ok_or_else(|| SocialError::bad_request("status must be a list of codes 0 to 6"))
        })
        .collect()
}

pub async fn list_tasks(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TaskListQuery>,
) -> Result<Json<Vec<AgentTaskInfo>>, SocialError> {
    let filter = TaskFilter {
        buyer_principal: non_empty(query.buyer_principal),
        buyer_agent_id: non_empty(query.buyer_agent_id),
        seller_principal: non_empty(query.seller_principal),
        seller_agent_id: non_empty(query.seller_agent_id),
        organization_id: non_empty(query.organization_id),
        room_group_id: non_empty(query.room_group_id),
        statuses: parse_statuses(query.status.as_deref())?,
    };
    if !filter.is_listable() {
        return Err(SocialError::bad_request(
            "narrow the listing with status, or scope it with buyer_principal, buyer_agent_id, \
             seller_principal, seller_agent_id, organization_id or room_group_id",
        ));
    }
    let (limit, offset) = page(query.limit, query.offset);
    state
        .reader
        .list_tasks(&filter, limit, offset)
        .await
        .map(Json)
}

pub async fn get_task(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> Result<Json<TaskDetailInfo>, SocialError> {
    state
        .reader
        .get_task(&task_id)
        .await?
        .ok_or_else(|| SocialError::not_found("Task"))
        .map(Json)
}

/// Resolve task-bound room metadata. The authenticated MCP facade must first
/// prove that the caller is a buyer or seller on this task. This endpoint only
/// returns the indexed room reference; it grants no messaging permission.
pub async fn get_task_room(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> Result<Json<crate::reader::AgentRoomInfo>, SocialError> {
    state
        .reader
        .get_task_room(&task_id)
        .await?
        .ok_or_else(|| SocialError::not_found("Task room"))
        .map(Json)
}

pub async fn get_task_receipt(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> Result<Json<TaskReceiptInfo>, SocialError> {
    state
        .reader
        .get_task_receipt(&task_id)
        .await?
        .ok_or_else(|| SocialError::not_found("Task settlement receipt"))
        .map(Json)
}

pub async fn list_task_history(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
    Query(query): Query<PageQuery>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let limit = query.limit.unwrap_or(50).clamp(1, 100);
    let offset = query.offset.unwrap_or(0).clamp(0, 100_000);
    let events = state
        .reader
        .list_task_history(&task_id, limit, offset)
        .await?;
    let next_offset = if events.len() as i64 == limit && offset + limit <= 100_000 {
        Some(offset + limit)
    } else {
        None
    };
    Ok(Json(serde_json::json!({
        "taskId": task_id,
        "events": events,
        "limit": limit,
        "offset": offset,
        "nextOffset": next_offset
    })))
}

pub async fn list_agent_receipts(
    State(state): State<Arc<AppState>>,
    Path(agent_object_id): Path<String>,
    Query(query): Query<PageQuery>,
) -> Result<Json<Vec<TaskReceiptInfo>>, SocialError> {
    let (limit, offset) = page(query.limit, query.offset);
    state
        .reader
        .list_agent_receipts(&agent_object_id, limit, offset)
        .await
        .map(Json)
}

pub async fn get_task_config(
    State(state): State<Arc<AppState>>,
) -> Result<Json<TaskConfigInfo>, SocialError> {
    state
        .reader
        .get_task_config()
        .await?
        .ok_or_else(|| SocialError::not_found("Task configuration"))
        .map(Json)
}

/// `GET /task/stats`: counts and volume for the whole market.
pub async fn get_task_stats(
    State(state): State<Arc<AppState>>,
) -> Result<Json<TaskStatsInfo>, SocialError> {
    let now_ms = chrono::Utc::now().timestamp_millis();
    state.reader.get_task_stats(now_ms).await.map(Json)
}

#[derive(Debug, Deserialize)]
pub struct TopAgentsQuery {
    /// `score` (default), `reviews` or `volume`.
    pub sort: Option<String>,
    /// Agents with fewer seller reviews are left out. Defaults to 1.
    pub min_reviews: Option<i64>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// `GET /agents/top`: active seller agents ranked by reputation.
pub async fn list_top_agents(
    State(state): State<Arc<AppState>>,
    Query(query): Query<TopAgentsQuery>,
) -> Result<Json<Vec<TopAgentInfo>>, SocialError> {
    let sort = TopAgentSort::parse(query.sort.as_deref())?;
    let min_reviews = query.min_reviews.unwrap_or(1).max(0);
    let (limit, offset) = page(query.limit, query.offset);
    state
        .reader
        .list_top_agents(sort, min_reviews, limit, offset)
        .await
        .map(Json)
}

pub async fn get_agent_reputation(
    State(state): State<Arc<AppState>>,
    Path(agent_object_id): Path<String>,
) -> Result<Json<AgentReputationInfo>, SocialError> {
    state
        .reader
        .get_agent_reputation(&agent_object_id)
        .await?
        .ok_or_else(|| SocialError::not_found("Agent"))
        .map(Json)
}

pub async fn get_principal_reputation(
    State(state): State<Arc<AppState>>,
    Path(principal): Path<String>,
) -> Result<Json<PrincipalReputationInfo>, SocialError> {
    state
        .reader
        .get_principal_reputation(&principal)
        .await
        .map(Json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statuses_parse_and_reject_unknown_codes() {
        assert_eq!(parse_statuses(None).unwrap(), Vec::<i16>::new());
        assert_eq!(parse_statuses(Some("")).unwrap(), Vec::<i16>::new());
        assert_eq!(parse_statuses(Some("0, 1,5")).unwrap(), vec![0, 1, 5]);
        assert_eq!(parse_statuses(Some("6")).unwrap(), vec![6]);
        assert!(parse_statuses(Some("7")).is_err());
        assert!(parse_statuses(Some("-1")).is_err());
        assert!(parse_statuses(Some("0,x")).is_err());
    }

    #[test]
    fn paging_is_clamped() {
        assert_eq!(page(None, None), (50, 0));
        assert_eq!(page(Some(0), Some(-4)), (1, 0));
        assert_eq!(page(Some(10_000), Some(7)), (200, 7));
    }

    #[test]
    fn an_unscoped_filter_is_rejected_and_any_scope_is_accepted() {
        assert!(!TaskFilter::default().is_scoped());
        let scoped = TaskFilter {
            room_group_id: Some("0xroom".into()),
            ..Default::default()
        };
        assert!(scoped.is_scoped());
    }

    #[test]
    fn a_status_only_listing_is_the_market_feed() {
        let filter = TaskFilter {
            statuses: parse_statuses(Some("0")).unwrap(),
            ..Default::default()
        };
        assert!(filter.is_listable());
        assert!(!TaskFilter::default().is_listable());
    }

    #[test]
    fn blank_filter_values_are_dropped() {
        assert_eq!(non_empty(Some("  ".into())), None);
        assert_eq!(non_empty(Some("0x1".into())), Some("0x1".into()));
    }
}
