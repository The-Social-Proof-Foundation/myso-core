// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Derivative graph, resolved policy, license, and discovery event indexing (Phases 2–5).

use serde::Deserialize;

use super::common;
use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewDetectedAssetRelationship, NewLicenseInstance, NewLicenseTemplateVersion,
    NewMediaAsset, NewMediaAssetAncestrySnapshot, NewMediaAssetDerivativeEdge,
    NewMediaAssetResolvedObligation, NewMediaAssetResolvedPolicy,
};

use super::media_asset::{bytes_from_json, chain_time, transaction_id_from_event_id};

fn id_from_json(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::String(s) if !s.is_empty() => Some(s.clone()),
        _ => None,
    }
}

fn u64_from_json(value: &serde_json::Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|s| s.parse().ok()))
}

#[derive(Debug, Deserialize)]
struct MediaAssetIdentity {
    content_commitment: serde_json::Value,
    media_type: u8,
    asset_kind: u8,
    creator: String,
}

#[derive(Debug, Deserialize)]
struct DerivativeRelationshipJson {
    relationship_id: serde_json::Value,
    parent_asset_id: serde_json::Value,
    child_asset_id: serde_json::Value,
    relationship_type: u8,
    license_instance_id: serde_json::Value,
    template_version_id: serde_json::Value,
    parent_share_bps: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct DerivativeAssetFinalizedEvent {
    pending_id: String,
    child_asset_id: String,
    identity: MediaAssetIdentity,
    #[serde(default)]
    parent_count: u64,
    ancestry_version: serde_json::Value,
    ancestor_ids: Vec<serde_json::Value>,
    #[serde(default)]
    ancestry_hash: Option<serde_json::Value>,
    finalized_edges: Vec<DerivativeRelationshipJson>,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct OriginalAssetFinalizedEvent {
    pending_id: String,
    child_asset_id: String,
    identity: MediaAssetIdentity,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct ResolvedPolicyUpdatedEvent {
    media_asset_id: String,
    policy_version: serde_json::Value,
    effective_rights: serde_json::Value,
    derivatives_allowed: bool,
    attribution_required: bool,
    commercial_allowed: bool,
    lineage: serde_json::Value,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct ResolvedObligationRecordedEvent {
    media_asset_id: String,
    policy_version: serde_json::Value,
    obligation_index: serde_json::Value,
    beneficiary_asset_id: Option<serde_json::Value>,
    beneficiary_address: String,
    share_bps: serde_json::Value,
    source_relationship_id: Option<serde_json::Value>,
    source_license_instance_id: Option<serde_json::Value>,
    obligation_kind: u8,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct LicenseTemplatePublishedEvent {
    template_version_id: String,
    family_id: String,
    version: serde_json::Value,
    creator: String,
    #[serde(default)]
    granted_rights: Option<serde_json::Value>,
    #[serde(default)]
    allow_derivatives: Option<bool>,
    #[serde(default)]
    attribution_required: Option<bool>,
    #[serde(default)]
    royalty_bps: Option<serde_json::Value>,
    #[serde(default)]
    derivative_royalty_bps: Option<serde_json::Value>,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct LicenseInstanceAcceptedEvent {
    license_instance_id: String,
    template_version_id: String,
    licensor_asset_id: String,
    licensee: String,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct LicenseInstanceRevokedEvent {
    license_instance_id: String,
    template_version_id: String,
    revoked_by: String,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

fn new_media_asset_from_identity(
    child_asset_id: String,
    identity: &MediaAssetIdentity,
    tx_id: &str,
    timestamp: u64,
) -> Option<NewMediaAsset> {
    let content_commitment = bytes_from_json(&identity.content_commitment)?;
    Some(NewMediaAsset {
        media_asset_id: child_asset_id,
        content_commitment,
        media_type: i16::from(identity.media_type),
        asset_kind: i16::from(identity.asset_kind),
        originality_status: 0,
        provenance_status: 1,
        lineage_parent_id: None,
        rights_version: 1,
        economics_version: 1,
        registered_by: identity.creator.clone(),
        registered_at: timestamp as i64,
        verified_at: Some(timestamp as i64),
        transaction_id: tx_id.to_string(),
        time: chain_time(timestamp),
    })
}

pub fn handle_graph_media_asset_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let tx_id = transaction_id_from_event_id(event_id);
    match event_name {
        "DerivativeAssetFinalizedEvent" => {
            let ev: DerivativeAssetFinalizedEvent = common::deserialize_social_event_json(
                "media_asset",
                event_name,
                event_id,
                data,
                "DerivativeAssetFinalizedEvent parse failed",
            )?;
            if ev.child_asset_id.is_empty() || ev.pending_id.is_empty() {
                return None;
            }
            if ev.parent_count > 0 && ev.finalized_edges.len() as u64 != ev.parent_count {
                return None;
            }
            let now = chain_time(ev.timestamp);
            let ancestry_version = u64_from_json(&ev.ancestry_version).unwrap_or(0) as i64;
            let ancestry_hash = ev
                .ancestry_hash
                .as_ref()
                .and_then(bytes_from_json)
                .map(|b| hex::encode(b));
            let ancestor_ids: Vec<String> = ev
                .ancestor_ids
                .iter()
                .filter_map(id_from_json)
                .collect();
            let mut rows = vec![SocialEventRow::MediaAsset(
                new_media_asset_from_identity(
                    ev.child_asset_id.clone(),
                    &ev.identity,
                    &tx_id,
                    ev.timestamp,
                )?,
            )];
            rows.push(SocialEventRow::MediaAssetAncestrySnapshot(
                NewMediaAssetAncestrySnapshot {
                    media_asset_id: ev.child_asset_id.clone(),
                    ancestry_version,
                    ancestor_ids: serde_json::json!(ancestor_ids),
                    ancestry_hash,
                    transaction_id: tx_id.clone(),
                    time: now,
                },
            ));
            for edge in ev.finalized_edges {
                let relationship_id = u64_from_json(&edge.relationship_id)? as i64;
                let parent_asset_id = id_from_json(&edge.parent_asset_id)?;
                let edge_child_asset_id = id_from_json(&edge.child_asset_id)?;
                if edge_child_asset_id != ev.child_asset_id {
                    return None;
                }
                let license_instance_id = id_from_json(&edge.license_instance_id)?;
                let template_version_id = id_from_json(&edge.template_version_id)?;
                let parent_share_bps = u64_from_json(&edge.parent_share_bps)? as i64;
                rows.push(SocialEventRow::MediaAssetDerivativeEdge(
                    NewMediaAssetDerivativeEdge {
                        child_asset_id: ev.child_asset_id.clone(),
                        parent_asset_id,
                        relationship_id,
                        relationship_type: i16::from(edge.relationship_type),
                        license_instance_id,
                        template_version_id,
                        parent_share_bps,
                        ancestry_version,
                        transaction_id: tx_id.clone(),
                        time: now,
                    },
                ));
            }
            Some(rows)
        }
        "OriginalAssetFinalizedEvent" => {
            let ev: OriginalAssetFinalizedEvent = common::deserialize_social_event_json(
                "media_asset",
                event_name,
                event_id,
                data,
                "OriginalAssetFinalizedEvent parse failed",
            )?;
            if ev.child_asset_id.is_empty() || ev.pending_id.is_empty() {
                return None;
            }
            Some(vec![SocialEventRow::MediaAsset(
                new_media_asset_from_identity(
                    ev.child_asset_id,
                    &ev.identity,
                    &tx_id,
                    ev.timestamp,
                )?,
            )])
        }
        "ResolvedPolicyUpdatedEvent" => {
            let ev: ResolvedPolicyUpdatedEvent = common::deserialize_social_event_json(
                "media_asset",
                event_name,
                event_id,
                data,
                "ResolvedPolicyUpdatedEvent parse failed",
            )?;
            let policy_version = u64_from_json(&ev.policy_version)? as i64;
            let effective_rights = u64_from_json(&ev.effective_rights)? as i64;
            let lineage_hash = ev
                .lineage
                .get("lineage_hash")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            Some(vec![SocialEventRow::MediaAssetResolvedPolicy(
                NewMediaAssetResolvedPolicy {
                    media_asset_id: ev.media_asset_id,
                    policy_version,
                    effective_rights,
                    derivatives_allowed: ev.derivatives_allowed,
                    attribution_required: ev.attribution_required,
                    commercial_allowed: ev.commercial_allowed,
                    lineage_json: ev.lineage,
                    lineage_hash,
                    transaction_id: tx_id,
                    time: chain_time(ev.timestamp),
                },
            )])
        }
        "ResolvedObligationRecordedEvent" => {
            let ev: ResolvedObligationRecordedEvent = common::deserialize_social_event_json(
                "media_asset",
                event_name,
                event_id,
                data,
                "ResolvedObligationRecordedEvent parse failed",
            )?;
            let policy_version = u64_from_json(&ev.policy_version)? as i64;
            let obligation_index = u64_from_json(&ev.obligation_index)? as i32;
            let share_bps = u64_from_json(&ev.share_bps)? as i64;
            Some(vec![SocialEventRow::MediaAssetResolvedObligation(
                NewMediaAssetResolvedObligation {
                    media_asset_id: ev.media_asset_id,
                    policy_version,
                    obligation_index,
                    beneficiary_asset_id: ev
                        .beneficiary_asset_id
                        .as_ref()
                        .and_then(id_from_json),
                    beneficiary_address: ev.beneficiary_address,
                    share_bps,
                    source_relationship_id: ev
                        .source_relationship_id
                        .as_ref()
                        .and_then(u64_from_json)
                        .map(|v| v as i64),
                    source_license_instance_id: ev
                        .source_license_instance_id
                        .as_ref()
                        .and_then(id_from_json),
                    obligation_kind: i16::from(ev.obligation_kind),
                    transaction_id: tx_id,
                    time: chain_time(ev.timestamp),
                },
            )])
        }
        _ => None,
    }
}

const DETECTED_STATUS_PROPOSED: i16 = 0;
const DETECTED_STATUS_ACCEPTED: i16 = 1;
const DETECTED_STATUS_REJECTED: i16 = 2;
const DETECTED_STATUS_FINALIZED: i16 = 3;

#[derive(Debug, Deserialize)]
struct DetectedRelationshipProposedEvent {
    proposal_id: String,
    accused_pending_id: String,
    original_asset_id: String,
    similarity_bps: serde_json::Value,
    #[serde(default)]
    evidence_commitment: Option<serde_json::Value>,
    detected_by: String,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct DetectedRelationshipStatusEvent {
    proposal_id: String,
    accused_pending_id: String,
    original_asset_id: String,
    similarity_bps: serde_json::Value,
    #[serde(default)]
    evidence_commitment: Option<serde_json::Value>,
    detected_by: String,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct RelationshipFinalizedEvent {
    proposal_id: String,
    accused_pending_id: String,
    child_asset_id: String,
    #[serde(default)]
    parent_asset_id: Option<String>,
    #[serde(default)]
    original_asset_id: Option<String>,
    #[serde(default)]
    similarity_bps: Option<serde_json::Value>,
    #[serde(default)]
    detected_by: Option<String>,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

fn new_detected_relationship(
    proposal_id: String,
    accused_pending_id: String,
    accused_asset_id: Option<String>,
    original_asset_id: String,
    similarity_bps: i64,
    evidence_commitment: Option<Vec<u8>>,
    detected_by: String,
    detected_at: i64,
    status: i16,
    tx_id: &str,
    timestamp: u64,
) -> SocialEventRow {
    SocialEventRow::DetectedAssetRelationship(NewDetectedAssetRelationship {
        proposal_id,
        accused_pending_id,
        accused_asset_id,
        original_asset_id,
        similarity_bps,
        evidence_commitment,
        detected_by,
        detected_at,
        status,
        transaction_id: tx_id.to_string(),
        time: chain_time(timestamp),
    })
}

pub fn handle_discovery_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
    tx_sender: Option<&str>,
) -> Option<Vec<SocialEventRow>> {
    let tx_id = transaction_id_from_event_id(event_id);
    match event_name {
        "DetectedRelationshipProposedEvent" => {
            let ev: DetectedRelationshipProposedEvent = common::deserialize_social_event_json(
                "proof_of_creativity",
                event_name,
                event_id,
                data,
                "DetectedRelationshipProposedEvent parse failed",
            )?;
            let similarity_bps = u64_from_json(&ev.similarity_bps)? as i64;
            let evidence = ev
                .evidence_commitment
                .as_ref()
                .and_then(bytes_from_json);
            let detected_by = if ev.detected_by.is_empty() {
                tx_sender.unwrap_or("unknown").to_string()
            } else {
                ev.detected_by
            };
            Some(vec![new_detected_relationship(
                ev.proposal_id,
                ev.accused_pending_id,
                None,
                ev.original_asset_id,
                similarity_bps,
                evidence,
                detected_by,
                ev.timestamp as i64,
                DETECTED_STATUS_PROPOSED,
                &tx_id,
                ev.timestamp,
            )])
        }
        "DetectedRelationshipAcceptedEvent" => {
            let ev: DetectedRelationshipStatusEvent = common::deserialize_social_event_json(
                "proof_of_creativity",
                event_name,
                event_id,
                data,
                "DetectedRelationshipAcceptedEvent parse failed",
            )?;
            let similarity_bps = u64_from_json(&ev.similarity_bps)? as i64;
            let evidence = ev
                .evidence_commitment
                .as_ref()
                .and_then(bytes_from_json);
            Some(vec![new_detected_relationship(
                ev.proposal_id,
                ev.accused_pending_id,
                None,
                ev.original_asset_id,
                similarity_bps,
                evidence,
                ev.detected_by,
                ev.timestamp as i64,
                DETECTED_STATUS_ACCEPTED,
                &tx_id,
                ev.timestamp,
            )])
        }
        "DetectedRelationshipRejectedEvent" => {
            let ev: DetectedRelationshipStatusEvent = common::deserialize_social_event_json(
                "proof_of_creativity",
                event_name,
                event_id,
                data,
                "DetectedRelationshipRejectedEvent parse failed",
            )?;
            let similarity_bps = u64_from_json(&ev.similarity_bps)? as i64;
            let evidence = ev
                .evidence_commitment
                .as_ref()
                .and_then(bytes_from_json);
            Some(vec![new_detected_relationship(
                ev.proposal_id,
                ev.accused_pending_id,
                None,
                ev.original_asset_id,
                similarity_bps,
                evidence,
                ev.detected_by,
                ev.timestamp as i64,
                DETECTED_STATUS_REJECTED,
                &tx_id,
                ev.timestamp,
            )])
        }
        "RelationshipFinalizedEvent" => {
            let ev: RelationshipFinalizedEvent = common::deserialize_social_event_json(
                "proof_of_creativity",
                event_name,
                event_id,
                data,
                "RelationshipFinalizedEvent parse failed",
            )?;
            let original_asset_id = ev
                .original_asset_id
                .or(ev.parent_asset_id)
                .unwrap_or_default();
            let similarity_bps = ev
                .similarity_bps
                .as_ref()
                .and_then(u64_from_json)
                .unwrap_or(0) as i64;
            let detected_by = ev
                .detected_by
                .unwrap_or_else(|| tx_sender.unwrap_or("unknown").to_string());
            Some(vec![new_detected_relationship(
                ev.proposal_id,
                ev.accused_pending_id,
                Some(ev.child_asset_id),
                original_asset_id,
                similarity_bps,
                None,
                detected_by,
                ev.timestamp as i64,
                DETECTED_STATUS_FINALIZED,
                &tx_id,
                ev.timestamp,
            )])
        }
        _ => None,
    }
}

pub fn handle_license_template_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let tx_id = transaction_id_from_event_id(event_id);
    match event_name {
        "LicenseTemplatePublishedEvent" => {
            let ev: LicenseTemplatePublishedEvent = common::deserialize_social_event_json(
                "license_template",
                event_name,
                event_id,
                data,
                "LicenseTemplatePublishedEvent parse failed",
            )?;
            Some(vec![SocialEventRow::LicenseTemplateVersion(
                NewLicenseTemplateVersion {
                    template_version_id: ev.template_version_id,
                    family_id: ev.family_id,
                    version: u64_from_json(&ev.version).unwrap_or(1) as i64,
                    creator: ev.creator,
                    granted_rights: ev
                        .granted_rights
                        .as_ref()
                        .and_then(u64_from_json)
                        .unwrap_or(u64::MAX) as i64,
                    allow_derivatives: ev.allow_derivatives.unwrap_or(true),
                    attribution_required: ev.attribution_required.unwrap_or(false),
                    royalty_bps: ev
                        .royalty_bps
                        .as_ref()
                        .and_then(u64_from_json)
                        .unwrap_or(0) as i64,
                    derivative_royalty_bps: ev
                        .derivative_royalty_bps
                        .as_ref()
                        .and_then(u64_from_json)
                        .unwrap_or(0) as i64,
                    transaction_id: tx_id,
                    time: chain_time(ev.timestamp),
                },
            )])
        }
        "LicenseInstanceRevokedEvent" => {
            let ev: LicenseInstanceRevokedEvent = common::deserialize_social_event_json(
                "license_template",
                event_name,
                event_id,
                data,
                "LicenseInstanceRevokedEvent parse failed",
            )?;
            Some(vec![SocialEventRow::LicenseInstance(NewLicenseInstance {
                license_instance_id: ev.license_instance_id,
                template_version_id: ev.template_version_id,
                licensor_asset_id: String::new(),
                licensee: ev.revoked_by,
                status: 3,
                accepted_at: ev.timestamp as i64,
                transaction_id: tx_id,
                time: chain_time(ev.timestamp),
            })])
        }
        "LicenseInstanceAcceptedEvent" => {
            let ev: LicenseInstanceAcceptedEvent = common::deserialize_social_event_json(
                "license_template",
                event_name,
                event_id,
                data,
                "LicenseInstanceAcceptedEvent parse failed",
            )?;
            Some(vec![SocialEventRow::LicenseInstance(NewLicenseInstance {
                license_instance_id: ev.license_instance_id,
                template_version_id: ev.template_version_id,
                licensor_asset_id: ev.licensor_asset_id,
                licensee: ev.licensee,
                status: 1,
                accepted_at: ev.timestamp as i64,
                transaction_id: tx_id,
                time: chain_time(ev.timestamp),
            })])
        }
        _ => None,
    }
}
