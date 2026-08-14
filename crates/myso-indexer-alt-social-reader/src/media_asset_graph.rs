// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::sql_types::{BigInt, Text};
use diesel_async::RunQueryDsl;

use crate::metrics::DbReaderMetrics;
use myso_indexer_alt_social_schema::models::{
    AncestrySnapshotRow, DerivativeEdgeRow, DetectedRelationshipRow, ResolvedObligationRow,
    ResolvedPolicyRow,
};
use myso_pg_db::Connection;

pub(crate) async fn list_derivative_edges_for_child(
    conn: &mut Connection<'_>,
    child_asset_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<DerivativeEdgeRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT child_asset_id, parent_asset_id, relationship_id, relationship_type,
               license_instance_id, template_version_id, parent_share_bps, ancestry_version
        FROM (
            SELECT DISTINCT ON (child_asset_id, parent_asset_id, relationship_id) *
            FROM media_asset_derivative_edges
            WHERE child_asset_id = $1
            ORDER BY child_asset_id, parent_asset_id, relationship_id, time DESC
        ) sub
        ORDER BY relationship_id ASC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(child_asset_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<DerivativeEdgeRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_derivative_edges_for_parent(
    conn: &mut Connection<'_>,
    parent_asset_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<DerivativeEdgeRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT child_asset_id, parent_asset_id, relationship_id, relationship_type,
               license_instance_id, template_version_id, parent_share_bps, ancestry_version
        FROM (
            SELECT DISTINCT ON (child_asset_id, parent_asset_id, relationship_id) *
            FROM media_asset_derivative_edges
            WHERE parent_asset_id = $1
            ORDER BY child_asset_id, parent_asset_id, relationship_id, time DESC
        ) sub
        ORDER BY relationship_id DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(parent_asset_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<DerivativeEdgeRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_ancestry_snapshot(
    conn: &mut Connection<'_>,
    media_asset_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<AncestrySnapshotRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT media_asset_id, ancestry_version, ancestor_ids, ancestry_hash
        FROM (
            SELECT DISTINCT ON (media_asset_id) *
            FROM media_asset_ancestry_snapshots
            WHERE media_asset_id = $1
            ORDER BY media_asset_id, ancestry_version DESC, time DESC
        ) sub
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(media_asset_id)
        .get_result::<AncestrySnapshotRow>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_resolved_policy(
    conn: &mut Connection<'_>,
    media_asset_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<ResolvedPolicyRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT media_asset_id, policy_version, effective_rights, derivatives_allowed,
               attribution_required, commercial_allowed, lineage_json, lineage_hash
        FROM (
            SELECT DISTINCT ON (media_asset_id) *
            FROM media_asset_resolved_policies
            WHERE media_asset_id = $1
            ORDER BY media_asset_id, policy_version DESC, time DESC
        ) sub
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(media_asset_id)
        .get_result::<ResolvedPolicyRow>(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_resolved_obligations(
    conn: &mut Connection<'_>,
    media_asset_id: &str,
    policy_version: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<ResolvedObligationRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT media_asset_id, policy_version, obligation_index, beneficiary_asset_id,
               beneficiary_address, share_bps, source_relationship_id,
               source_license_instance_id, obligation_kind
        FROM (
            SELECT DISTINCT ON (media_asset_id, policy_version, obligation_index) *
            FROM media_asset_resolved_obligations
            WHERE media_asset_id = $1 AND policy_version = $2
            ORDER BY media_asset_id, policy_version, obligation_index, time DESC
        ) sub
        ORDER BY obligation_index ASC
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(media_asset_id)
        .bind::<BigInt, _>(policy_version)
        .load::<ResolvedObligationRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_detected_relationships(
    conn: &mut Connection<'_>,
    asset_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<DetectedRelationshipRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let query = "
        SELECT proposal_id, accused_pending_id, accused_asset_id, original_asset_id,
               similarity_bps, status, detected_at
        FROM (
            SELECT DISTINCT ON (proposal_id) *
            FROM detected_asset_relationships
            WHERE accused_pending_id = $1 OR accused_asset_id = $1 OR original_asset_id = $1
            ORDER BY proposal_id, time DESC
        ) sub
        ORDER BY detected_at DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(asset_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<DetectedRelationshipRow>(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(results)
}
