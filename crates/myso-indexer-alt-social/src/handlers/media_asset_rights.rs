// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Media asset rights governance dispute events (`proof_of_creativity` + `media_asset`).

use serde::Deserialize;

use super::common;
use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    GOV_LINK_STATUS_ACTIVE, GOV_LINK_STATUS_IMPLEMENTED, GOV_LINK_STATUS_REJECTED,
    NewMediaAssetGovernanceLink, NewMediaAssetRightsUpdate,
};

use super::media_asset::{bytes_from_json, chain_time, transaction_id_from_event_id};

const GOV_CLEAR_OUTCOME_IMPLEMENTED: u8 = 1;
const GOV_CLEAR_OUTCOME_REJECTED: u8 = 2;

fn id_from_json(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    }
}

#[derive(Debug, Deserialize)]
struct MediaAssetRightsDisputeProposedEvent {
    media_asset_id: serde_json::Value,
    proposal_id: serde_json::Value,
    submitter: String,
    claims_commitment: serde_json::Value,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct MediaAssetGovernanceProposalClearedEvent {
    media_asset_id: serde_json::Value,
    proposal_id: serde_json::Value,
    outcome: u8,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct MediaAssetRightsUpdatedEvent {
    media_asset_id: serde_json::Value,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    rights_version: u64,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

pub fn handle_media_asset_rights_poc_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let tx_id = transaction_id_from_event_id(event_id);
    match event_name {
        "MediaAssetRightsDisputeProposedEvent" => {
            process_rights_dispute_proposed(data, event_id, &tx_id)
        }
        "MediaAssetGovernanceProposalLinkedEvent" => None,
        "MediaAssetGovernanceProposalClearedEvent" => {
            process_governance_proposal_cleared(data, event_id, &tx_id)
        }
        _ => None,
    }
}

pub fn handle_media_asset_rights_updated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let tx_id = transaction_id_from_event_id(event_id);
    let ev: MediaAssetRightsUpdatedEvent = common::deserialize_social_event_json(
        "media_asset",
        "MediaAssetRightsUpdatedEvent",
        event_id,
        data,
        "media_asset MediaAssetRightsUpdatedEvent JSON did not match",
    )?;
    let media_asset_id = id_from_json(&ev.media_asset_id)?;
    let link = NewMediaAssetRightsUpdate {
        media_asset_id: media_asset_id.clone(),
        rights_version: ev.rights_version as i64,
        proposal_id: None,
        transaction_id: tx_id,
        time: chain_time(ev.timestamp),
    };
    Some(vec![SocialEventRow::MediaAssetRightsUpdate(link)])
}

fn process_rights_dispute_proposed(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: MediaAssetRightsDisputeProposedEvent = common::deserialize_social_event_json(
        "proof_of_creativity",
        "MediaAssetRightsDisputeProposedEvent",
        event_id,
        data,
        "MediaAssetRightsDisputeProposedEvent JSON did not match",
    )?;
    let media_asset_id = id_from_json(&ev.media_asset_id)?;
    let proposal_id = id_from_json(&ev.proposal_id)?;
    if ev.submitter.is_empty() {
        return None;
    }
    let claims_commitment = bytes_from_json(&ev.claims_commitment)?;
    if claims_commitment.len() != 32 {
        return None;
    }
    let link = NewMediaAssetGovernanceLink {
        media_asset_id,
        proposal_id,
        submitter: ev.submitter,
        claims_commitment,
        status: GOV_LINK_STATUS_ACTIVE,
        related_post_id: None,
        rights_disputes_submitted: 1,
        transaction_id: tx_id.to_string(),
        time: chain_time(ev.timestamp),
    };
    Some(vec![SocialEventRow::MediaAssetGovernanceLink(link)])
}

fn process_governance_proposal_cleared(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: MediaAssetGovernanceProposalClearedEvent = common::deserialize_social_event_json(
        "proof_of_creativity",
        "MediaAssetGovernanceProposalClearedEvent",
        event_id,
        data,
        "MediaAssetGovernanceProposalClearedEvent JSON did not match",
    )?;
    let media_asset_id = id_from_json(&ev.media_asset_id)?;
    let proposal_id = id_from_json(&ev.proposal_id)?;
    let status = match ev.outcome {
        GOV_CLEAR_OUTCOME_IMPLEMENTED => GOV_LINK_STATUS_IMPLEMENTED,
        GOV_CLEAR_OUTCOME_REJECTED => GOV_LINK_STATUS_REJECTED,
        _ => return None,
    };
    Some(vec![SocialEventRow::MediaAssetGovernanceLinkStatusUpdate {
        media_asset_id,
        proposal_id,
        status,
        transaction_id: tx_id.to_string(),
        time: chain_time(ev.timestamp),
    }])
}
