// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Public, unauthenticated jury reads (on-chain facts only; nothing secret).

use axum::extract::{Path, Query, State};
use axum::http::HeaderMap;
use axum::Json;
use serde::Deserialize;
use std::sync::Arc;

use super::enterprise::check_required_sync_secret;
use crate::error::SocialError;
use crate::reader::jury::CaseFilter;
use crate::reader::{
    CrankInfo, JurorAssignmentInfo, JurorRosterInfo, JuryCaseDetailInfo, JuryCaseInfo,
    ProposalJuryInfo, ProposalVoteRefundsInfo, RegistryJuryPolicyView, TaskDisputeDetailInfo,
};

use super::super::AppState;

#[derive(Debug, Deserialize)]
pub struct CaseListQuery {
    /// 1 task, 2 governance proposal.
    pub subject_kind: Option<i16>,
    pub subject_id: Option<String>,
    /// Comma-separated case states, e.g. `3,4`.
    pub state: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct RosterQuery {
    /// Only agents owned by this principal.
    pub owner: Option<String>,
    /// Comma-separated roster status codes (0 left, 1 active, 3 suspended).
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
pub struct StatusPageQuery {
    /// Comma-separated status codes.
    pub status: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

fn page(limit: Option<i64>, offset: Option<i64>) -> (i64, i64) {
    (
        limit.unwrap_or(50).clamp(1, 200),
        offset.unwrap_or(0).max(0),
    )
}

/// `3,4` -> `[3, 4]`; every entry must fall inside `range`.
fn parse_codes(
    raw: Option<&str>,
    range: std::ops::RangeInclusive<i16>,
    what: &str,
) -> Result<Vec<i16>, SocialError> {
    let Some(raw) = raw.filter(|s| !s.trim().is_empty()) else {
        return Ok(Vec::new());
    };
    raw.split(',')
        .map(|part| {
            part.trim()
                .parse::<i16>()
                .ok()
                .filter(|v| range.contains(v))
                .ok_or_else(|| {
                    SocialError::bad_request(format!(
                        "{what} must be a list of codes {} to {}",
                        range.start(),
                        range.end()
                    ))
                })
        })
        .collect()
}

pub async fn list_jury_cases(
    State(state): State<Arc<AppState>>,
    Query(query): Query<CaseListQuery>,
) -> Result<Json<Vec<JuryCaseInfo>>, SocialError> {
    let filter = CaseFilter {
        subject_kind: query.subject_kind,
        subject_id: query.subject_id.filter(|s| !s.trim().is_empty()),
        states: parse_codes(query.state.as_deref(), 1..=8, "state")?,
    };
    if !filter.is_scoped() {
        return Err(SocialError::bad_request(
            "scope the listing with subject_id or state",
        ));
    }
    let (limit, offset) = page(query.limit, query.offset);
    state
        .reader
        .list_jury_cases(&filter, limit, offset)
        .await
        .map(Json)
}

pub async fn get_jury_case(
    State(state): State<Arc<AppState>>,
    Path(case_id): Path<String>,
) -> Result<Json<JuryCaseDetailInfo>, SocialError> {
    state
        .reader
        .get_jury_case(&case_id)
        .await?
        .ok_or_else(|| SocialError::not_found("Jury case"))
        .map(Json)
}

#[derive(Debug, Deserialize)]
pub struct HealthQuery {
    /// The jury config's `min_eligible_roster`, which is not indexed, so the caller supplies it.
    #[serde(rename = "minRoster")]
    pub min_roster: Option<i64>,
}

/// Whether the jury is keeping up: undelivered events, cranks nobody ran and the size of the roster.
/// Public like the rest of the jury reads (counts only), so a monitor can poll it without a secret.
pub async fn get_jury_health(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HealthQuery>,
) -> Result<Json<crate::reader::JuryHealthInfo>, SocialError> {
    let min_roster = query.min_roster.unwrap_or(20).clamp(0, 10_000);
    state.reader.jury_health(min_roster).await.map(Json)
}

/// The written reasons behind a case's revealed votes. Public: a vote's reason is public from its reveal.
pub async fn list_jury_rationales(
    State(state): State<Arc<AppState>>,
    Path(case_id): Path<String>,
) -> Result<Json<Vec<crate::reader::JuryRationaleInfo>>, SocialError> {
    state.reader.list_jury_rationales(&case_id).await.map(Json)
}

/// A juror's runner publishes the reason behind a vote it revealed. Needs the shared secret, and the
/// text must hash to the digest the chain recorded for that seat.
pub async fn store_jury_rationale(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(req): Json<crate::reader::StoreRationaleRequest>,
) -> Result<Json<serde_json::Value>, SocialError> {
    check_required_sync_secret(&headers, "x-internal-sync-secret", "INTERNAL_SYNC_SECRET")?;
    if req.rationale.trim().is_empty() || req.rationale.len() > MAX_RATIONALE_BYTES {
        return Err(SocialError::bad_request(
            "rationale must be 1 to 16384 bytes",
        ));
    }
    let outcome = state.reader.store_jury_rationale(&req).await?;
    Ok(Json(serde_json::json!({ "status": outcome })))
}

/// More than a juror's reason ever needs; the runner caps its own at a few thousand characters.
const MAX_RATIONALE_BYTES: usize = 16 * 1024;

pub async fn get_task_dispute(
    State(state): State<Arc<AppState>>,
    Path(task_id): Path<String>,
) -> Result<Json<TaskDisputeDetailInfo>, SocialError> {
    state
        .reader
        .get_task_dispute(&task_id)
        .await?
        .ok_or_else(|| SocialError::not_found("Task dispute"))
        .map(Json)
}

pub async fn list_jury_roster(
    State(state): State<Arc<AppState>>,
    Query(query): Query<RosterQuery>,
) -> Result<Json<Vec<JurorRosterInfo>>, SocialError> {
    let statuses = parse_codes(query.status.as_deref(), 0..=3, "status")?;
    let owner = query.owner.filter(|s| !s.trim().is_empty());
    let (limit, offset) = page(query.limit, query.offset);
    state
        .reader
        .list_jury_roster(owner.as_deref(), &statuses, limit, offset)
        .await
        .map(Json)
}

pub async fn get_juror(
    State(state): State<Arc<AppState>>,
    Path(agent_id): Path<String>,
) -> Result<Json<JurorRosterInfo>, SocialError> {
    state
        .reader
        .get_juror(&agent_id)
        .await?
        .ok_or_else(|| SocialError::not_found("Juror"))
        .map(Json)
}

/// A juror's duties. Default (no `status`): open duties ordered by the next deadline, so a
/// runner polls this and acts on the first row. Pass `status=4,5,6` for history, newest first.
pub async fn list_juror_assignments(
    State(state): State<Arc<AppState>>,
    Path(agent_id): Path<String>,
    Query(query): Query<StatusPageQuery>,
) -> Result<Json<Vec<JurorAssignmentInfo>>, SocialError> {
    let statuses = parse_codes(query.status.as_deref(), 1..=6, "status")?;
    let (limit, offset) = page(query.limit, query.offset);
    state
        .reader
        .list_juror_assignments(&agent_id, &statuses, limit, offset)
        .await
        .map(Json)
}

/// Whether the jury has been asked to take over a proposal's council or community step.
pub async fn get_proposal_jury(
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<String>,
) -> Result<Json<ProposalJuryInfo>, SocialError> {
    state.reader.get_proposal_jury(&proposal_id).await.map(Json)
}

/// A refunded proposal's voters, what each can claim and who already has.
pub async fn get_proposal_vote_refunds(
    State(state): State<Arc<AppState>>,
    Path(proposal_id): Path<String>,
) -> Result<Json<ProposalVoteRefundsInfo>, SocialError> {
    state
        .reader
        .get_proposal_vote_refunds(&proposal_id)
        .await
        .map(Json)
}

pub async fn get_registry_jury_policy(
    State(state): State<Arc<AppState>>,
    Path(registry_id): Path<String>,
) -> Result<Json<RegistryJuryPolicyView>, SocialError> {
    state
        .reader
        .get_registry_jury_policy(&registry_id)
        .await?
        .ok_or_else(|| SocialError::not_found("Jury policy"))
        .map(Json)
}

/// What anyone can crank right now, oldest first within each kind. This is the public operator
/// interface: the reference `jury-runner` and any third-party operator read it, and a crank another
/// operator ran first is an expected abort on chain.
pub async fn list_jury_cranks(
    State(state): State<Arc<AppState>>,
    Query(query): Query<PageQuery>,
) -> Result<Json<Vec<CrankInfo>>, SocialError> {
    let (limit, _) = page(query.limit, None);
    let now_ms = chrono::Utc::now().timestamp_millis();
    state.reader.list_jury_cranks(now_ms, limit).await.map(Json)
}

#[derive(Debug, Deserialize)]
pub struct PageQuery {
    pub limit: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_parse_inside_their_range_only() {
        assert_eq!(
            parse_codes(None, 1..=8, "state").unwrap(),
            Vec::<i16>::new()
        );
        assert_eq!(
            parse_codes(Some(" "), 1..=8, "state").unwrap(),
            Vec::<i16>::new()
        );
        assert_eq!(
            parse_codes(Some("3, 4"), 1..=8, "state").unwrap(),
            vec![3, 4]
        );
        assert!(parse_codes(Some("0"), 1..=8, "state").is_err());
        assert!(parse_codes(Some("9"), 1..=8, "state").is_err());
        assert!(parse_codes(Some("1,x"), 1..=8, "state").is_err());
    }

    #[test]
    fn paging_is_clamped() {
        assert_eq!(page(None, None), (50, 0));
        assert_eq!(page(Some(0), Some(-1)), (1, 0));
        assert_eq!(page(Some(5_000), Some(3)), (200, 3));
    }
}
