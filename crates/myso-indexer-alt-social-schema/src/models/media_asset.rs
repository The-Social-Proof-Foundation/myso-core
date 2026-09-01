// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::QueryableByName;
use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bytea, Jsonb, SmallInt, Text};
use serde::{Deserialize, Serialize};

use crate::schema::{
    composition_analysis_records, detected_asset_relationships, fingerprint_observations,
    license_instances, license_template_versions, media_asset_ancestry_snapshots,
    media_asset_derivative_edges, media_asset_governance_links, media_asset_resolved_obligations,
    media_asset_resolved_policies, media_asset_rights_updates, media_asset_usages, media_assets,
    post_usage_decision_events, revenue_manifests,
};

pub const GOV_LINK_STATUS_ACTIVE: i16 = 1;
pub const GOV_LINK_STATUS_IMPLEMENTED: i16 = 2;
pub const GOV_LINK_STATUS_REJECTED: i16 = 3;

pub const COMPOSITION_NONE: i16 = 0;
pub const COMPOSITION_PENDING: i16 = 1;
pub const COMPOSITION_VERIFIED: i16 = 2;
pub const COMPOSITION_INVALID: i16 = 3;

pub const MONETIZATION_NONE: i16 = 0;
pub const MONETIZATION_PENDING: i16 = 1;
pub const MONETIZATION_ENABLED: i16 = 2;
pub const MONETIZATION_RESTRICTED: i16 = 3;

pub const CONTAINER_TYPE_POST: i16 = 1;
pub const CONTAINER_TYPE_PROFILE: i16 = 2;

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = media_assets)]
pub struct NewMediaAsset {
    pub media_asset_id: String,
    pub content_commitment: Vec<u8>,
    pub media_type: i16,
    #[serde(default)]
    pub asset_kind: i16,
    pub originality_status: i16,
    pub provenance_status: i16,
    pub lineage_parent_id: Option<String>,
    pub rights_version: i64,
    pub economics_version: i64,
    pub registered_by: String,
    pub registered_at: i64,
    pub verified_at: Option<i64>,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = fingerprint_observations)]
pub struct NewFingerprintObservation {
    pub fingerprint_commitment: Vec<u8>,
    pub media_asset_id: String,
    pub linked_at: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = media_asset_usages)]
pub struct NewMediaAssetUsage {
    pub container_id: String,
    pub container_type: i16,
    pub asset_id: String,
    pub usage_class: i16,
    pub position: i16,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = composition_analysis_records)]
pub struct NewCompositionAnalysisRecord {
    pub post_id: String,
    pub analyzed_at: i64,
    pub usage_context: i16,
    pub composition_status: i16,
    pub monetization_status: i16,
    pub analysis_json: serde_json::Value,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = revenue_manifests)]
pub struct NewRevenueManifestRecord {
    pub post_id: String,
    pub manifest_version: i64,
    pub entries_json: serde_json::Value,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = media_asset_derivative_edges)]
pub struct NewMediaAssetDerivativeEdge {
    pub child_asset_id: String,
    pub parent_asset_id: String,
    pub relationship_id: i64,
    pub relationship_type: i16,
    pub license_instance_id: String,
    pub template_version_id: String,
    pub parent_share_bps: i64,
    pub ancestry_version: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = media_asset_ancestry_snapshots)]
pub struct NewMediaAssetAncestrySnapshot {
    pub media_asset_id: String,
    pub ancestry_version: i64,
    pub ancestor_ids: serde_json::Value,
    pub ancestry_hash: Option<String>,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = license_template_versions)]
pub struct NewLicenseTemplateVersion {
    pub template_version_id: String,
    pub family_id: String,
    pub version: i64,
    pub creator: String,
    pub granted_rights: i64,
    pub allow_derivatives: bool,
    pub attribution_required: bool,
    pub royalty_bps: i64,
    pub derivative_royalty_bps: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = license_instances)]
pub struct NewLicenseInstance {
    pub license_instance_id: String,
    pub template_version_id: String,
    pub licensor_asset_id: String,
    pub licensee: String,
    pub status: i16,
    pub accepted_at: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = media_asset_resolved_policies)]
pub struct NewMediaAssetResolvedPolicy {
    pub media_asset_id: String,
    pub policy_version: i64,
    pub effective_rights: i64,
    pub derivatives_allowed: bool,
    pub attribution_required: bool,
    pub commercial_allowed: bool,
    pub lineage_json: serde_json::Value,
    pub lineage_hash: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = media_asset_resolved_obligations)]
pub struct NewMediaAssetResolvedObligation {
    pub media_asset_id: String,
    pub policy_version: i64,
    pub obligation_index: i32,
    pub beneficiary_asset_id: Option<String>,
    pub beneficiary_address: String,
    pub share_bps: i64,
    pub source_relationship_id: Option<i64>,
    pub source_license_instance_id: Option<String>,
    pub obligation_kind: i16,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = post_usage_decision_events)]
pub struct NewPostUsageDecisionEvent {
    pub post_id: String,
    pub binding_id: i64,
    pub playback_permitted: bool,
    pub payout_permitted: bool,
    pub policy_reason_code: i16,
    pub policy_version: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = detected_asset_relationships)]
pub struct NewDetectedAssetRelationship {
    pub proposal_id: String,
    pub accused_pending_id: String,
    pub accused_asset_id: Option<String>,
    pub original_asset_id: String,
    pub similarity_bps: i64,
    pub evidence_commitment: Option<Vec<u8>>,
    pub detected_by: String,
    pub detected_at: i64,
    pub status: i16,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = media_asset_governance_links)]
pub struct NewMediaAssetGovernanceLink {
    pub media_asset_id: String,
    pub proposal_id: String,
    pub submitter: String,
    pub claims_commitment: Vec<u8>,
    pub status: i16,
    pub related_post_id: Option<String>,
    pub rights_disputes_submitted: i16,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = media_asset_rights_updates)]
pub struct NewMediaAssetRightsUpdate {
    pub media_asset_id: String,
    pub rights_version: i64,
    pub proposal_id: Option<String>,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MediaAssetGovernanceLinkRow {
    #[diesel(sql_type = Text)]
    pub media_asset_id: String,
    #[diesel(sql_type = Text)]
    pub proposal_id: String,
    #[diesel(sql_type = Text)]
    pub submitter: String,
    #[diesel(sql_type = Bytea)]
    pub claims_commitment: Vec<u8>,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
    #[diesel(sql_type = diesel::sql_types::Nullable<Text>)]
    pub related_post_id: Option<String>,
    #[diesel(sql_type = SmallInt)]
    pub rights_disputes_submitted: i16,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MediaAssetRightsUpdateRow {
    #[diesel(sql_type = Text)]
    pub media_asset_id: String,
    #[diesel(sql_type = BigInt)]
    pub rights_version: i64,
    #[diesel(sql_type = diesel::sql_types::Nullable<Text>)]
    pub proposal_id: Option<String>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
    #[diesel(sql_type = diesel::sql_types::Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MediaAssetRow {
    #[diesel(sql_type = Text)]
    pub media_asset_id: String,
    #[diesel(sql_type = Bytea)]
    pub content_commitment: Vec<u8>,
    #[diesel(sql_type = SmallInt)]
    pub media_type: i16,
    #[diesel(sql_type = SmallInt)]
    pub asset_kind: i16,
    #[diesel(sql_type = SmallInt)]
    pub originality_status: i16,
    #[diesel(sql_type = SmallInt)]
    pub provenance_status: i16,
    #[diesel(sql_type = diesel::sql_types::Nullable<Text>)]
    pub lineage_parent_id: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub rights_version: i64,
    #[diesel(sql_type = BigInt)]
    pub economics_version: i64,
    #[diesel(sql_type = Text)]
    pub registered_by: String,
    #[diesel(sql_type = BigInt)]
    pub registered_at: i64,
    #[diesel(sql_type = diesel::sql_types::Nullable<BigInt>)]
    pub verified_at: Option<i64>,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct MediaAssetUsageRow {
    #[diesel(sql_type = Text)]
    pub container_id: String,
    #[diesel(sql_type = SmallInt)]
    pub container_type: i16,
    #[diesel(sql_type = Text)]
    pub asset_id: String,
    #[diesel(sql_type = SmallInt)]
    pub usage_class: i16,
    #[diesel(sql_type = SmallInt)]
    pub position: i16,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct CompositionAnalysisRow {
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = BigInt)]
    pub analyzed_at: i64,
    #[diesel(sql_type = SmallInt)]
    pub usage_context: i16,
    #[diesel(sql_type = SmallInt)]
    pub composition_status: i16,
    #[diesel(sql_type = SmallInt)]
    pub monetization_status: i16,
    #[diesel(sql_type = Jsonb)]
    pub analysis_json: serde_json::Value,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct RevenueManifestRow {
    #[diesel(sql_type = Text)]
    pub post_id: String,
    #[diesel(sql_type = BigInt)]
    pub manifest_version: i64,
    #[diesel(sql_type = Jsonb)]
    pub entries_json: serde_json::Value,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct DerivativeEdgeRow {
    #[diesel(sql_type = Text)]
    pub child_asset_id: String,
    #[diesel(sql_type = Text)]
    pub parent_asset_id: String,
    #[diesel(sql_type = BigInt)]
    pub relationship_id: i64,
    #[diesel(sql_type = SmallInt)]
    pub relationship_type: i16,
    #[diesel(sql_type = Text)]
    pub license_instance_id: String,
    #[diesel(sql_type = Text)]
    pub template_version_id: String,
    #[diesel(sql_type = BigInt)]
    pub parent_share_bps: i64,
    #[diesel(sql_type = BigInt)]
    pub ancestry_version: i64,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct AncestrySnapshotRow {
    #[diesel(sql_type = Text)]
    pub media_asset_id: String,
    #[diesel(sql_type = BigInt)]
    pub ancestry_version: i64,
    #[diesel(sql_type = Jsonb)]
    pub ancestor_ids: serde_json::Value,
    #[diesel(sql_type = diesel::sql_types::Nullable<Text>)]
    pub ancestry_hash: Option<String>,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct ResolvedPolicyRow {
    #[diesel(sql_type = Text)]
    pub media_asset_id: String,
    #[diesel(sql_type = BigInt)]
    pub policy_version: i64,
    #[diesel(sql_type = BigInt)]
    pub effective_rights: i64,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub derivatives_allowed: bool,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub attribution_required: bool,
    #[diesel(sql_type = diesel::sql_types::Bool)]
    pub commercial_allowed: bool,
    #[diesel(sql_type = Jsonb)]
    pub lineage_json: serde_json::Value,
    #[diesel(sql_type = Text)]
    pub lineage_hash: String,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct ResolvedObligationRow {
    #[diesel(sql_type = Text)]
    pub media_asset_id: String,
    #[diesel(sql_type = BigInt)]
    pub policy_version: i64,
    #[diesel(sql_type = diesel::sql_types::Integer)]
    pub obligation_index: i32,
    #[diesel(sql_type = diesel::sql_types::Nullable<Text>)]
    pub beneficiary_asset_id: Option<String>,
    #[diesel(sql_type = Text)]
    pub beneficiary_address: String,
    #[diesel(sql_type = BigInt)]
    pub share_bps: i64,
    #[diesel(sql_type = diesel::sql_types::Nullable<BigInt>)]
    pub source_relationship_id: Option<i64>,
    #[diesel(sql_type = diesel::sql_types::Nullable<Text>)]
    pub source_license_instance_id: Option<String>,
    #[diesel(sql_type = SmallInt)]
    pub obligation_kind: i16,
}

#[derive(Debug, Clone, QueryableByName, Serialize, Deserialize)]
pub struct DetectedRelationshipRow {
    #[diesel(sql_type = Text)]
    pub proposal_id: String,
    #[diesel(sql_type = Text)]
    pub accused_pending_id: String,
    #[diesel(sql_type = diesel::sql_types::Nullable<Text>)]
    pub accused_asset_id: Option<String>,
    #[diesel(sql_type = Text)]
    pub original_asset_id: String,
    #[diesel(sql_type = BigInt)]
    pub similarity_bps: i64,
    #[diesel(sql_type = SmallInt)]
    pub status: i16,
    #[diesel(sql_type = BigInt)]
    pub detected_at: i64,
}
