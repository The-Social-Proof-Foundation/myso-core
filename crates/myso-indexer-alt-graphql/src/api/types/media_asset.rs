// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::{
    AncestrySnapshotRow, CompositionAnalysisRow, DerivativeEdgeRow, DetectedRelationshipRow,
    MediaAssetRightsUpdateRow, MediaAssetRow, MediaAssetUsageRow,
    ResolvedObligationRow, ResolvedPolicyRow, RevenueManifestRow,
};

use crate::api::scalars::date_time::DateTime;
use crate::api::types::governance::Proposal;
use crate::api::types::media_asset_enums::{
    DerivativeRelationshipType, DetectedAssetRelationshipStatus, MediaAssetKind,
    MediaAssetMediaType, MediaAssetOriginalityStatus, MediaAssetProvenanceStatus,
    PostCompositionStatus, PostMonetizationStatus,
};

use crate::api::scalars::json::Json;

#[derive(Clone)]
pub(crate) struct MediaAsset {
    inner: MediaAssetRow,
}

impl MediaAsset {
    pub(crate) fn from_row(inner: MediaAssetRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MediaAsset {
    async fn media_asset_id(&self) -> &str {
        &self.inner.media_asset_id
    }

    async fn media_type(&self) -> MediaAssetMediaType {
        MediaAssetMediaType::from(self.inner.media_type)
    }

    async fn asset_kind(&self) -> MediaAssetKind {
        MediaAssetKind::from(self.inner.asset_kind)
    }

    async fn originality_status(&self) -> MediaAssetOriginalityStatus {
        MediaAssetOriginalityStatus::from(self.inner.originality_status)
    }

    async fn provenance_status(&self) -> MediaAssetProvenanceStatus {
        MediaAssetProvenanceStatus::from(self.inner.provenance_status)
    }

    async fn lineage_parent_id(&self) -> Option<&str> {
        self.inner.lineage_parent_id.as_deref()
    }

    async fn rights_version(&self) -> i64 {
        self.inner.rights_version
    }

    async fn economics_version(&self) -> i64 {
        self.inner.economics_version
    }

    async fn registered_by(&self) -> &str {
        &self.inner.registered_by
    }

    async fn registered_at(&self) -> i64 {
        self.inner.registered_at
    }

    async fn verified_at(&self) -> Option<i64> {
        self.inner.verified_at
    }

    async fn usages(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<MediaAssetUsage>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        reader
            .list_media_asset_usages(&self.inner.media_asset_id, limit, offset)
            .await
            .ok()
            .map(|rows| rows.into_iter().map(MediaAssetUsage::from_row).collect())
    }

    async fn derivative_graph(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<DerivativeGraph> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(200) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let parent_edges = reader
            .list_derivative_edges_for_child(&self.inner.media_asset_id, limit, offset)
            .await
            .ok()?;
        let child_edges = reader
            .list_derivative_edges_for_parent(&self.inner.media_asset_id, limit, offset)
            .await
            .ok()?;
        Some(DerivativeGraph {
            asset_id: self.inner.media_asset_id.clone(),
            parent_edges,
            child_edges,
        })
    }

    async fn ancestry(&self, ctx: &Context<'_>) -> Option<AncestrySnapshot> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_ancestry_snapshot(&self.inner.media_asset_id)
            .await
            .ok()
            .flatten()
            .map(AncestrySnapshot::from_row)
    }

    async fn resolved_policy(&self, ctx: &Context<'_>) -> Option<ResolvedPolicy> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_resolved_policy(&self.inner.media_asset_id)
            .await
            .ok()
            .flatten()
            .map(ResolvedPolicy::from_row)
    }

    async fn detected_relationships(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<DetectedAssetRelationship>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        reader
            .list_detected_relationships(&self.inner.media_asset_id, limit, offset)
            .await
            .ok()
            .map(|rows| {
                rows.into_iter()
                    .map(DetectedAssetRelationship::from_row)
                    .collect()
            })
    }

    async fn rights_disputes_submitted(&self, ctx: &Context<'_>) -> Option<i64> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .count_media_asset_rights_disputes_submitted(&self.inner.media_asset_id)
            .await
            .ok()
    }

    async fn active_rights_proposal(&self, ctx: &Context<'_>) -> Option<Proposal> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let proposal_id = reader
            .get_active_rights_proposal_for_asset(&self.inner.media_asset_id)
            .await
            .ok()??;
        reader
            .get_proposal_by_id(&proposal_id)
            .await
            .ok()
            .flatten()
            .map(Proposal::from_row)
    }

    async fn rights_proposals(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<Proposal>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        let links = reader
            .list_media_asset_rights_proposals(&self.inner.media_asset_id, limit, offset)
            .await
            .ok()?;
        let mut proposals = Vec::with_capacity(links.len());
        for link in links {
            if let Ok(Some(row)) = reader.get_proposal_by_id(&link.proposal_id).await {
                proposals.push(Proposal::from_row(row));
            }
        }
        Some(proposals)
    }

    async fn rights_updates(
        &self,
        ctx: &Context<'_>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<MediaAssetRightsUpdate>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(20).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        reader
            .list_media_asset_rights_updates(&self.inner.media_asset_id, limit, offset)
            .await
            .ok()
            .map(|rows| {
                rows.into_iter()
                    .map(MediaAssetRightsUpdate::from_row)
                    .collect()
            })
    }
}

#[derive(Clone)]
pub(crate) struct DerivativeGraph {
    asset_id: String,
    parent_edges: Vec<DerivativeEdgeRow>,
    child_edges: Vec<DerivativeEdgeRow>,
}

#[Object]
impl DerivativeGraph {
    async fn asset_id(&self) -> &str {
        &self.asset_id
    }

    async fn parent_edges(&self) -> Vec<DerivativeEdge> {
        self.parent_edges
            .iter()
            .cloned()
            .map(DerivativeEdge::from_row)
            .collect()
    }

    async fn child_edges(&self) -> Vec<DerivativeEdge> {
        self.child_edges
            .iter()
            .cloned()
            .map(DerivativeEdge::from_row)
            .collect()
    }
}

#[derive(Clone)]
pub(crate) struct DerivativeEdge {
    inner: DerivativeEdgeRow,
}

impl DerivativeEdge {
    pub(crate) fn from_row(inner: DerivativeEdgeRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl DerivativeEdge {
    async fn child_asset_id(&self) -> &str {
        &self.inner.child_asset_id
    }

    async fn parent_asset_id(&self) -> &str {
        &self.inner.parent_asset_id
    }

    async fn relationship_id(&self) -> i64 {
        self.inner.relationship_id
    }

    async fn relationship_type(&self) -> DerivativeRelationshipType {
        DerivativeRelationshipType::from(self.inner.relationship_type)
    }

    async fn license_instance_id(&self) -> &str {
        &self.inner.license_instance_id
    }

    async fn template_version_id(&self) -> &str {
        &self.inner.template_version_id
    }

    async fn parent_share_bps(&self) -> i64 {
        self.inner.parent_share_bps
    }

    async fn ancestry_version(&self) -> i64 {
        self.inner.ancestry_version
    }
}

#[derive(Clone)]
pub(crate) struct AncestrySnapshot {
    inner: AncestrySnapshotRow,
}

impl AncestrySnapshot {
    pub(crate) fn from_row(inner: AncestrySnapshotRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl AncestrySnapshot {
    async fn media_asset_id(&self) -> &str {
        &self.inner.media_asset_id
    }

    async fn ancestry_version(&self) -> i64 {
        self.inner.ancestry_version
    }

    async fn ancestor_ids(&self) -> Option<Json> {
        Json::try_from(self.inner.ancestor_ids.clone()).ok()
    }

    async fn ancestry_hash(&self) -> Option<&str> {
        self.inner.ancestry_hash.as_deref()
    }
}

#[derive(Clone)]
pub(crate) struct ResolvedPolicy {
    inner: ResolvedPolicyRow,
}

impl ResolvedPolicy {
    pub(crate) fn from_row(inner: ResolvedPolicyRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ResolvedPolicy {
    async fn media_asset_id(&self) -> &str {
        &self.inner.media_asset_id
    }

    async fn policy_version(&self) -> i64 {
        self.inner.policy_version
    }

    async fn effective_rights(&self) -> i64 {
        self.inner.effective_rights
    }

    async fn derivatives_allowed(&self) -> bool {
        self.inner.derivatives_allowed
    }

    async fn attribution_required(&self) -> bool {
        self.inner.attribution_required
    }

    async fn commercial_allowed(&self) -> bool {
        self.inner.commercial_allowed
    }

    async fn lineage_json(&self) -> Option<Json> {
        Json::try_from(self.inner.lineage_json.clone()).ok()
    }

    async fn lineage_hash(&self) -> &str {
        &self.inner.lineage_hash
    }

    async fn obligations(&self, ctx: &Context<'_>) -> Option<Vec<ResolvedObligation>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_resolved_obligations(&self.inner.media_asset_id, self.inner.policy_version)
            .await
            .ok()
            .map(|rows| rows.into_iter().map(ResolvedObligation::from_row).collect())
    }
}

#[derive(Clone)]
pub(crate) struct ResolvedObligation {
    inner: ResolvedObligationRow,
}

impl ResolvedObligation {
    pub(crate) fn from_row(inner: ResolvedObligationRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl ResolvedObligation {
    async fn obligation_index(&self) -> i32 {
        self.inner.obligation_index
    }

    async fn beneficiary_asset_id(&self) -> Option<&str> {
        self.inner.beneficiary_asset_id.as_deref()
    }

    async fn beneficiary_address(&self) -> &str {
        &self.inner.beneficiary_address
    }

    async fn share_bps(&self) -> i64 {
        self.inner.share_bps
    }

    async fn source_relationship_id(&self) -> Option<i64> {
        self.inner.source_relationship_id
    }

    async fn source_license_instance_id(&self) -> Option<&str> {
        self.inner.source_license_instance_id.as_deref()
    }

    async fn obligation_kind(&self) -> i16 {
        self.inner.obligation_kind
    }
}

#[derive(Clone)]
pub(crate) struct DetectedAssetRelationship {
    inner: DetectedRelationshipRow,
}

impl DetectedAssetRelationship {
    pub(crate) fn from_row(inner: DetectedRelationshipRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl DetectedAssetRelationship {
    async fn proposal_id(&self) -> &str {
        &self.inner.proposal_id
    }

    async fn accused_pending_id(&self) -> &str {
        &self.inner.accused_pending_id
    }

    async fn accused_asset_id(&self) -> Option<&str> {
        self.inner.accused_asset_id.as_deref()
    }

    async fn original_asset_id(&self) -> &str {
        &self.inner.original_asset_id
    }

    async fn similarity_bps(&self) -> i64 {
        self.inner.similarity_bps
    }

    async fn status(&self) -> DetectedAssetRelationshipStatus {
        DetectedAssetRelationshipStatus::from(self.inner.status)
    }

    async fn detected_at(&self) -> i64 {
        self.inner.detected_at
    }
}

#[derive(Clone)]
pub(crate) struct MediaAssetUsage {
    inner: MediaAssetUsageRow,
}

impl MediaAssetUsage {
    pub(crate) fn from_row(inner: MediaAssetUsageRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MediaAssetUsage {
    async fn container_id(&self) -> &str {
        &self.inner.container_id
    }

    async fn container_type(&self) -> i16 {
        self.inner.container_type
    }

    async fn asset_id(&self) -> &str {
        &self.inner.asset_id
    }

    async fn usage_class(&self) -> i16 {
        self.inner.usage_class
    }

    async fn position(&self) -> i16 {
        self.inner.position
    }
}

#[derive(Clone)]
pub(crate) struct CompositionAnalysis {
    inner: CompositionAnalysisRow,
}

impl CompositionAnalysis {
    pub(crate) fn from_row(inner: CompositionAnalysisRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl CompositionAnalysis {
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    async fn analyzed_at(&self) -> i64 {
        self.inner.analyzed_at
    }

    async fn usage_context(&self) -> i16 {
        self.inner.usage_context
    }

    async fn composition_status(&self) -> PostCompositionStatus {
        PostCompositionStatus::from(self.inner.composition_status)
    }

    async fn monetization_status(&self) -> PostMonetizationStatus {
        PostMonetizationStatus::from(self.inner.monetization_status)
    }

    async fn analysis_json(&self) -> Option<Json> {
        Json::try_from(self.inner.analysis_json.clone()).ok()
    }
}

#[derive(Clone)]
pub(crate) struct RevenueManifest {
    inner: RevenueManifestRow,
}

impl RevenueManifest {
    pub(crate) fn from_row(inner: RevenueManifestRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl RevenueManifest {
    async fn post_id(&self) -> &str {
        &self.inner.post_id
    }

    async fn manifest_version(&self) -> i64 {
        self.inner.manifest_version
    }

    async fn entries_json(&self) -> Option<Json> {
        Json::try_from(self.inner.entries_json.clone()).ok()
    }
}

#[derive(Clone)]
pub(crate) struct MediaAssetRightsUpdate {
    inner: MediaAssetRightsUpdateRow,
}

impl MediaAssetRightsUpdate {
    pub(crate) fn from_row(inner: MediaAssetRightsUpdateRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MediaAssetRightsUpdate {
    async fn media_asset_id(&self) -> &str {
        &self.inner.media_asset_id
    }

    async fn rights_version(&self) -> i64 {
        self.inner.rights_version
    }

    async fn proposal_id(&self) -> Option<&str> {
        self.inner.proposal_id.as_deref()
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn time(&self) -> DateTime {
        DateTime::from_chrono(self.inner.time)
    }
}
