// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use diesel::sql_types::{BigInt, Bytea, Jsonb, SmallInt, Text};
use diesel::QueryableByName;
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

/// Matches `post::POC_REDIRECT_*` / Move `poc_redirection_kind` on posts.
pub const POC_REDIRECT_NONE: i16 = 0;
pub const POC_REDIRECT_WALLET: i16 = 1;
pub const POC_REDIRECT_ESCROW: i16 = 2;

/// Matches `media_asset::payout_wallet` / `payout_escrow`.
pub const MANIFEST_PAYOUT_WALLET: u8 = 0;
pub const MANIFEST_PAYOUT_ESCROW: u8 = 1;
pub const MANIFEST_BPS_TOTAL: u64 = 10_000;

/// Parsed revenue-manifest entry used to derive post PoC redirect fields.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestEntryView {
    pub beneficiary: String,
    pub share_bps: u64,
    pub payout_mode: u8,
}

/// `posts.poc_redirection_kind` + `posts.revenue_redirect_to` derived from a manifest write.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManifestPocRedirect {
    pub poc_redirection_kind: i16,
    pub revenue_redirect_to: Option<String>,
}

fn json_u64(value: &serde_json::Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_i64().and_then(|n| u64::try_from(n).ok()))
        .or_else(|| value.as_str().and_then(|s| s.parse().ok()))
}

fn json_address(value: &serde_json::Value) -> Option<String> {
    if let Some(s) = value.as_str() {
        let trimmed = s.trim();
        if trimmed.starts_with("0x") && trimmed.len() > 2 {
            return Some(trimmed.to_string());
        }
    }
    if let Some(s) = value.get("address").and_then(|v| v.as_str()) {
        let trimmed = s.trim();
        if trimmed.starts_with("0x") && trimmed.len() > 2 {
            return Some(trimmed.to_string());
        }
    }
    None
}

fn normalize_address(raw: &str) -> String {
    raw.trim().to_ascii_lowercase()
}

fn manifest_entry_object(
    value: &serde_json::Value,
) -> Option<&serde_json::Map<String, serde_json::Value>> {
    let object = value.as_object()?;
    if let Some(fields) = object.get("fields").and_then(|v| v.as_object()) {
        return Some(fields);
    }
    Some(object)
}

fn parse_manifest_entry(value: &serde_json::Value) -> Option<ManifestEntryView> {
    let fields = manifest_entry_object(value)?;
    let beneficiary = fields.get("beneficiary").and_then(json_address)?;
    let share_bps = fields
        .get("share_bps")
        .or_else(|| fields.get("shareBps"))
        .and_then(json_u64)?;
    let payout_mode = fields
        .get("payout_mode")
        .or_else(|| fields.get("payoutMode"))
        .and_then(json_u64)
        .and_then(|n| u8::try_from(n).ok())
        .unwrap_or(MANIFEST_PAYOUT_WALLET);
    Some(ManifestEntryView {
        beneficiary,
        share_bps,
        payout_mode,
    })
}

/// Parse `revenue_manifests.entries_json` (array, or `{ entries | entries_json: [...] }`).
pub fn parse_manifest_entries(entries_json: &serde_json::Value) -> Vec<ManifestEntryView> {
    let array = entries_json.as_array().or_else(|| {
        entries_json
            .get("entries")
            .and_then(|v| v.as_array())
            .or_else(|| entries_json.get("entries_json").and_then(|v| v.as_array()))
    });
    array
        .map(|items| items.iter().filter_map(parse_manifest_entry).collect())
        .unwrap_or_default()
}

pub fn manifest_uses_escrow_redirect(entries_json: &serde_json::Value) -> bool {
    parse_manifest_entries(entries_json)
        .iter()
        .any(|e| e.payout_mode == MANIFEST_PAYOUT_ESCROW && e.share_bps > 0)
}

pub fn manifest_escrow_beneficiaries(entries_json: &serde_json::Value) -> Vec<String> {
    parse_manifest_entries(entries_json)
        .into_iter()
        .filter(|e| e.payout_mode == MANIFEST_PAYOUT_ESCROW && e.share_bps > 0)
        .map(|e| e.beneficiary)
        .collect()
}

/// Derive `poc_redirection_kind` and `revenue_redirect_to` from a newly written manifest.
/// Escrow entry with share > 0 → 2; otherwise a present manifest → 1.
/// `revenue_redirect_to` is the first non-owner beneficiary (Move `manifest_redirect_beneficiary`).
pub fn derive_poc_redirect_from_manifest(
    entries_json: &serde_json::Value,
    post_owner: Option<&str>,
) -> ManifestPocRedirect {
    let entries = parse_manifest_entries(entries_json);
    let poc_redirection_kind = if entries
        .iter()
        .any(|e| e.payout_mode == MANIFEST_PAYOUT_ESCROW && e.share_bps > 0)
    {
        POC_REDIRECT_ESCROW
    } else {
        POC_REDIRECT_WALLET
    };
    let owner_norm = post_owner.map(normalize_address);
    let revenue_redirect_to = entries.iter().find_map(|e| {
        let beneficiary_norm = normalize_address(&e.beneficiary);
        if owner_norm
            .as_ref()
            .is_some_and(|owner| owner == &beneficiary_norm)
        {
            None
        } else {
            Some(e.beneficiary.clone())
        }
    });
    ManifestPocRedirect {
        poc_redirection_kind,
        revenue_redirect_to,
    }
}

/// Creator-fee amount that reservation fee paths pass through the post revenue manifest.
/// Mirrors `social_proof_tokens::reservation_creator_fee_for_vault_check`.
pub fn reservation_creator_fee_for_vault_check(
    reservation_amount: u64,
    reservation_creator_fee_bps: u64,
    reservation_platform_fee_bps: u64,
    reservation_treasury_fee_bps: u64,
    non_platform_platform_to_creator_bps: u64,
    platform: bool,
) -> u64 {
    let total_bps = reservation_creator_fee_bps
        .saturating_add(reservation_platform_fee_bps)
        .saturating_add(reservation_treasury_fee_bps);
    if reservation_amount == 0 || total_bps == 0 {
        return 0;
    }
    let fee_amount = reservation_amount.saturating_mul(total_bps) / 10_000;
    let creator_fee = fee_amount.saturating_mul(reservation_creator_fee_bps) / total_bps;
    if platform {
        creator_fee
    } else {
        let platform_fee = fee_amount.saturating_mul(reservation_platform_fee_bps) / total_bps;
        creator_fee + platform_fee.saturating_mul(non_platform_platform_to_creator_bps) / 10_000
    }
}

/// True when the creator-fee slice hits an escrow manifest entry (Move tip/reserve vault predicate).
pub fn post_reserve_requires_beneficiary_vault_for_amount(
    entries_json: &serde_json::Value,
    creator_fee_amount: u64,
) -> bool {
    if creator_fee_amount == 0 {
        return false;
    }
    parse_manifest_entries(entries_json).iter().any(|e| {
        e.payout_mode == MANIFEST_PAYOUT_ESCROW
            && creator_fee_amount.saturating_mul(e.share_bps) / MANIFEST_BPS_TOTAL > 0
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(beneficiary: &str, share_bps: u64, payout_mode: u8) -> serde_json::Value {
        serde_json::json!({
            "beneficiary": beneficiary,
            "share_bps": share_bps,
            "payout_mode": payout_mode,
        })
    }

    #[test]
    fn derive_wallet_manifest_sets_kind_one() {
        let owner = "0xabc";
        let json = serde_json::json!([
            entry(owner, 7_000, MANIFEST_PAYOUT_WALLET),
            entry("0xdef", 3_000, MANIFEST_PAYOUT_WALLET),
        ]);
        let derived = derive_poc_redirect_from_manifest(&json, Some(owner));
        assert_eq!(derived.poc_redirection_kind, POC_REDIRECT_WALLET);
        assert_eq!(derived.revenue_redirect_to.as_deref(), Some("0xdef"));
        assert!(!manifest_uses_escrow_redirect(&json));
    }

    #[test]
    fn derive_escrow_manifest_sets_kind_two() {
        let owner = "0xABC";
        let json = serde_json::json!([
            entry("0xabc", 5_000, MANIFEST_PAYOUT_WALLET),
            entry("0xparent", 5_000, MANIFEST_PAYOUT_ESCROW),
        ]);
        let derived = derive_poc_redirect_from_manifest(&json, Some(owner));
        assert_eq!(derived.poc_redirection_kind, POC_REDIRECT_ESCROW);
        assert_eq!(derived.revenue_redirect_to.as_deref(), Some("0xparent"));
        assert_eq!(
            manifest_escrow_beneficiaries(&json),
            vec!["0xparent".to_string()]
        );
    }

    #[test]
    fn vault_required_uses_creator_fee_slice() {
        let json = serde_json::json!([entry("0xparent", 100, MANIFEST_PAYOUT_ESCROW)]);
        assert!(!post_reserve_requires_beneficiary_vault_for_amount(
            &json, 50
        ));
        assert!(post_reserve_requires_beneficiary_vault_for_amount(
            &json, 100
        ));
        let creator_fee =
            reservation_creator_fee_for_vault_check(1_000_000_000, 100, 25, 25, 5_000, true);
        assert_eq!(creator_fee, 10_000_000);
    }
}
