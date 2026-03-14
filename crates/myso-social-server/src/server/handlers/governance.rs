// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use axum::extract::{Path, Query, State};
use axum::Json;
use std::sync::Arc;

use myso_indexer_alt_social_schema::{
    PROPOSAL_TYPE_ECOSYSTEM, PROPOSAL_TYPE_PLATFORM, PROPOSAL_TYPE_PROOF_OF_CREATIVITY,
};

use crate::error::SocialError;

use super::super::{
    AppState, GovernanceDelegateQuery, GovernanceNomineeQuery, GovernanceProposalQuery, PageParams,
};

fn is_valid_proposal_type(t: i16) -> bool {
    t == PROPOSAL_TYPE_ECOSYSTEM
        || t == PROPOSAL_TYPE_PROOF_OF_CREATIVITY
        || t == PROPOSAL_TYPE_PLATFORM
}

pub async fn list_governance_proposals(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GovernanceProposalQuery>,
) -> Result<Json<Vec<crate::reader::ProposalRow>>, SocialError> {
    if let Some(pt) = params.proposal_type {
        if !is_valid_proposal_type(pt) {
            return Err(SocialError::bad_request(
                "Invalid proposal_type: must be 0 (Ecosystem), 1 (Proof of Creativity), or 3 (Platform)",
            ));
        }
    }

    let limit = params.limit.unwrap_or(20).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_proposals(
            limit,
            offset,
            params.status,
            params.proposal_type,
            params.submitter.as_deref(),
        )
        .await?;
    Ok(Json(data))
}

pub async fn get_governance_proposal(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, SocialError> {
    let proposal = state
        .reader
        .get_proposal_by_id(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Proposal '{}'", id)))?;
    let delegate_votes = state.reader.get_proposal_delegate_votes(&id).await?;
    let community_votes_count = state.reader.get_proposal_community_votes_count(&id).await?;
    let reward_distributions = state.reader.get_proposal_reward_distributions(&id).await?;

    let mut proposal_json =
        serde_json::to_value(&proposal).map_err(|e| SocialError::internal(e.to_string()))?;
    if let Some(obj) = proposal_json.as_object_mut() {
        obj.insert(
            "object_id".to_string(),
            serde_json::Value::String(proposal.id.clone()),
        );
    }

    Ok(Json(serde_json::json!({
        "proposal": proposal_json,
        "delegate_votes": delegate_votes,
        "community_votes_count": community_votes_count,
        "reward_distributions": reward_distributions
    })))
}

pub async fn get_governance_proposal_community_votes(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::CommunityVoteRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_proposal_community_votes(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_governance_proposal_anonymous_stats(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<crate::reader::AnonymousVotingStatsRow>, SocialError> {
    let stats = state
        .reader
        .get_proposal_anonymous_stats(&id)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Anonymous stats for proposal '{}'", id)))?;
    Ok(Json(stats))
}

pub async fn get_governance_proposal_anonymous_votes(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::AnonymousVoteRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state
        .reader
        .get_proposal_anonymous_votes(&id, limit, offset)
        .await?;
    Ok(Json(data))
}

pub async fn get_governance_proposal_decryption_failures(
    State(state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<Json<Vec<crate::reader::VoteDecryptionFailureRow>>, SocialError> {
    let data = state.reader.get_proposal_decryption_failures(&id).await?;
    Ok(Json(data))
}

pub async fn list_governance_delegates(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GovernanceDelegateQuery>,
) -> Result<Json<Vec<crate::reader::DelegateRow>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_delegates(limit, offset, params.registry_type, params.is_active)
        .await?;
    Ok(Json(data))
}

pub async fn get_governance_delegate(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<crate::reader::DelegateRow>, SocialError> {
    let delegate = state
        .reader
        .get_delegate_by_address(&address)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Delegate '{}'", address)))?;
    Ok(Json(delegate))
}

pub async fn get_governance_delegate_proposals(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<Vec<crate::reader::ProposalRow>>, SocialError> {
    let data = state.reader.get_delegate_proposals(&address).await?;
    Ok(Json(data))
}

pub async fn get_governance_delegate_ratings(
    State(state): State<Arc<AppState>>,
    Path(address): Path<String>,
) -> Result<Json<Vec<crate::reader::DelegateRatingRow>>, SocialError> {
    let data = state.reader.get_delegate_ratings(&address).await?;
    Ok(Json(data))
}

pub async fn list_governance_nominees(
    State(state): State<Arc<AppState>>,
    Query(params): Query<GovernanceNomineeQuery>,
) -> Result<Json<Vec<crate::reader::NominatedDelegateRow>>, SocialError> {
    let limit = params.limit.unwrap_or(50).min(100);
    let offset = params.offset.unwrap_or(0);
    let data = state
        .reader
        .list_nominees(limit, offset, params.registry_type, params.status)
        .await?;
    Ok(Json(data))
}

pub async fn list_governance_registries(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<crate::reader::GovernanceRegistryRow>>, SocialError> {
    let data = state.reader.list_governance_registries().await?;
    Ok(Json(data))
}

pub async fn get_governance_registry(
    State(state): State<Arc<AppState>>,
    Path(registry_type): Path<String>,
) -> Result<Json<crate::reader::GovernanceRegistryRow>, SocialError> {
    let registry_type: i16 = registry_type
        .parse()
        .map_err(|_| SocialError::bad_request("Invalid registry_type"))?;
    let registry = state
        .reader
        .get_governance_registry_by_type(registry_type)
        .await?
        .ok_or_else(|| SocialError::not_found(format!("Registry type '{}'", registry_type)))?;
    Ok(Json(registry))
}

pub async fn list_governance_events(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::GovernanceEventRow>>, SocialError> {
    let limit = params.limit();
    let offset = params.offset();
    let data = state.reader.list_governance_events(limit, offset).await?;
    Ok(Json(data))
}

pub async fn get_governance_anonymous_voting_trends(
    State(state): State<Arc<AppState>>,
    Query(params): Query<PageParams>,
) -> Result<Json<Vec<crate::reader::AnonymousVotingTrendRow>>, SocialError> {
    let limit = params.limit().min(90);
    let data = state.reader.get_anonymous_voting_trends(limit).await?;
    Ok(Json(data))
}
