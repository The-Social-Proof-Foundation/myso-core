// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Int2, Text};
use diesel_async::RunQueryDsl;

use crate::metrics::DbReaderMetrics;
use myso_indexer_alt_social_schema::models::{
    CompositionAnalysisRow, GOV_LINK_STATUS_ACTIVE, LicenseInstanceRow, LicenseTemplateVersionRow,
    MediaAssetGovernanceLinkRow, MediaAssetRightsUpdateRow, MediaAssetRow, MediaAssetUsageRow,
    RevenueManifestRow,
};
use myso_pg_db::Connection;

pub(crate) async fn get_media_asset_by_id(
    conn: &mut Connection<'_>,
    media_asset_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MediaAssetRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT media_asset_id, content_commitment, media_type, asset_kind, originality_status,
               provenance_status, lineage_parent_id, rights_version, economics_version,
               COALESCE(authorization_version, 1) AS authorization_version,
               COALESCE(future_usage_paused, FALSE) AS future_usage_paused,
               registered_by, registered_at, verified_at
        FROM (
            SELECT DISTINCT ON (media_asset_id) *
            FROM media_assets
            WHERE media_asset_id = $1
            ORDER BY media_asset_id, time DESC
        ) sub
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(media_asset_id)
        .get_result::<MediaAssetRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_active_rights_proposal_id_for_asset(
    conn: &mut Connection<'_>,
    media_asset_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<String>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    #[derive(QueryableByName)]
    struct ActiveProposalId {
        #[diesel(sql_type = Text)]
        proposal_id: String,
    }

    let query = "
        SELECT proposal_id
        FROM media_asset_governance_links
        WHERE media_asset_id = $1 AND status = $2
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(media_asset_id)
        .bind::<Int2, _>(GOV_LINK_STATUS_ACTIVE)
        .get_result::<ActiveProposalId>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result.map(|r| r.proposal_id))
}

pub(crate) async fn get_media_asset_id_for_rights_proposal(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<String>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    #[derive(QueryableByName)]
    struct AssetIdRow {
        #[diesel(sql_type = Text)]
        media_asset_id: String,
    }

    let query = "
        SELECT media_asset_id
        FROM media_asset_governance_links
        WHERE proposal_id = $1
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .get_result::<AssetIdRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result.map(|r| r.media_asset_id))
}

pub(crate) async fn count_rights_disputes_submitted(
    conn: &mut Connection<'_>,
    media_asset_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<i64> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        count: i64,
    }

    let query = "
        SELECT COALESCE(MAX(rights_disputes_submitted), 0) AS count
        FROM media_asset_governance_links
        WHERE media_asset_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(media_asset_id)
        .get_result::<CountRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(result.count)
}

pub(crate) async fn list_media_asset_governance_links(
    conn: &mut Connection<'_>,
    media_asset_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MediaAssetGovernanceLinkRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT media_asset_id, proposal_id, submitter, claims_commitment, status,
               related_post_id, rights_disputes_submitted, transaction_id, time
        FROM (
            SELECT DISTINCT ON (proposal_id) *
            FROM media_asset_governance_links
            WHERE media_asset_id = $1
            ORDER BY proposal_id, time DESC
        ) sub
        ORDER BY time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(media_asset_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MediaAssetGovernanceLinkRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_media_asset_rights_updates(
    conn: &mut Connection<'_>,
    media_asset_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MediaAssetRightsUpdateRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT media_asset_id, rights_version, proposal_id, transaction_id, time
        FROM media_asset_rights_updates
        WHERE media_asset_id = $1
        ORDER BY rights_version DESC, time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(media_asset_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MediaAssetRightsUpdateRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_media_asset_usages(
    conn: &mut Connection<'_>,
    asset_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<MediaAssetUsageRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT container_id, container_type, asset_id, usage_class, position
        FROM (
            SELECT DISTINCT ON (container_id, asset_id, usage_class, position) *
            FROM media_asset_usages
            WHERE asset_id = $1
            ORDER BY container_id, asset_id, usage_class, position, time DESC
        ) sub
        ORDER BY position ASC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(asset_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<MediaAssetUsageRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_composition_analysis_for_post(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<CompositionAnalysisRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT post_id, analyzed_at, usage_context, composition_status,
               monetization_status, analysis_json
        FROM (
            SELECT DISTINCT ON (post_id) *
            FROM composition_analysis_records
            WHERE post_id = $1
            ORDER BY post_id, analyzed_at DESC, time DESC
        ) sub
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<CompositionAnalysisRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_revenue_manifest_for_post(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<RevenueManifestRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT post_id, manifest_version, entries_json
        FROM (
            SELECT DISTINCT ON (post_id) *
            FROM revenue_manifests
            WHERE post_id = $1
            ORDER BY post_id, manifest_version DESC, time DESC
        ) sub
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<RevenueManifestRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_license_templates_for_asset(
    conn: &mut Connection<'_>,
    media_asset_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<LicenseTemplateVersionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT t.template_version_id, t.family_id, t.version, t.creator, t.granted_rights,
               t.allow_derivatives, t.attribution_required, t.royalty_bps, t.derivative_royalty_bps,
               COALESCE(t.instance_revocable, TRUE) AS instance_revocable,
               t.legal_terms_uri, t.legal_terms_hash, t.governing_law,
               COALESCE(t.license_schema_version, 1) AS license_schema_version
        FROM (
            SELECT DISTINCT ON (template_version_id) *
            FROM license_template_versions
            ORDER BY template_version_id, time DESC
        ) t
        WHERE EXISTS (
            SELECT 1 FROM (
                SELECT DISTINCT ON (license_instance_id) *
                FROM license_instances
                ORDER BY license_instance_id, time DESC
            ) i
            WHERE i.template_version_id = t.template_version_id
              AND i.licensor_asset_id = $1
        )
           OR EXISTS (
            SELECT 1 FROM (
                SELECT DISTINCT ON (child_asset_id, parent_asset_id, relationship_id) *
                FROM media_asset_derivative_edges
                ORDER BY child_asset_id, parent_asset_id, relationship_id, time DESC
            ) e
            WHERE e.template_version_id = t.template_version_id
              AND (e.parent_asset_id = $1 OR e.child_asset_id = $1)
        )
        ORDER BY t.version DESC, t.template_version_id
        LIMIT $2 OFFSET $3
    ";

    let rows = diesel::sql_query(query)
        .bind::<Text, _>(media_asset_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<LicenseTemplateVersionRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn list_license_instances_for_asset(
    conn: &mut Connection<'_>,
    media_asset_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<LicenseInstanceRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT license_instance_id, template_version_id, licensor_asset_id, licensee, status, accepted_at
        FROM (
            SELECT DISTINCT ON (license_instance_id) *
            FROM license_instances
            WHERE licensor_asset_id = $1
            ORDER BY license_instance_id, time DESC
        ) sub
        ORDER BY accepted_at DESC
        LIMIT $2 OFFSET $3
    ";

    let rows = diesel::sql_query(query)
        .bind::<Text, _>(media_asset_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<LicenseInstanceRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(rows)
}
