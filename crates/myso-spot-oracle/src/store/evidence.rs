// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::evidence::{EvidenceBundle, EvidenceRecord};

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct EvidenceRow {
    pub id: Uuid,
    pub market_id: Uuid,
    pub resolver_job_id: Option<Uuid>,
    pub bundle_id: Option<Uuid>,
    pub adapter_id: String,
    pub source_url: String,
    pub content_hash: String,
    pub payload: serde_json::Value,
    pub provenance: serde_json::Value,
    pub signature: Option<serde_json::Value>,
    pub raw_response: Option<String>,
    pub fetched_at: DateTime<Utc>,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct EvidenceBundleRow {
    pub id: Uuid,
    pub market_id: Uuid,
    pub resolver_job_id: Option<Uuid>,
    pub bundle_hash: String,
    pub created_at: DateTime<Utc>,
}

pub async fn insert_evidence_bundle(
    pool: &PgPool,
    bundle: &EvidenceBundle,
) -> anyhow::Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO evidence_bundles (market_id, resolver_job_id, bundle_hash)
        VALUES ($1, $2, $3)
        RETURNING id
        "#,
    )
    .bind(bundle.market_id)
    .bind(bundle.resolver_job_id)
    .bind(&bundle.bundle_hash)
    .fetch_one(pool)
    .await?;
    let bundle_id = row.0;

    for record in &bundle.records {
        insert_evidence_record(pool, bundle.market_id, bundle.resolver_job_id, bundle_id, record)
            .await?;
    }
    Ok(bundle_id)
}

async fn insert_evidence_record(
    pool: &PgPool,
    market_id: Uuid,
    resolver_job_id: Uuid,
    bundle_id: Uuid,
    record: &EvidenceRecord,
) -> anyhow::Result<Uuid> {
    let payload = serde_json::to_value(&record.payload)?;
    let provenance = serde_json::to_value(&record.provenance)?;
    let signature = record
        .signature
        .as_ref()
        .map(serde_json::to_value)
        .transpose()?;
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO evidence (market_id, resolver_job_id, bundle_id, adapter_id, source_url, content_hash, payload, provenance, signature, raw_response)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id
        "#,
    )
    .bind(market_id)
    .bind(resolver_job_id)
    .bind(bundle_id)
    .bind(&record.adapter_id)
    .bind(&record.provenance.source_url)
    .bind(&record.provenance.content_hash)
    .bind(payload)
    .bind(provenance)
    .bind(signature)
    .bind(record.raw_response.as_deref())
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn list_evidence_for_market(
    pool: &PgPool,
    market_id: Uuid,
) -> anyhow::Result<Vec<EvidenceRow>> {
    let rows = sqlx::query_as::<_, EvidenceRow>(
        r#"
        SELECT id, market_id, resolver_job_id, bundle_id, adapter_id, source_url, content_hash,
               payload, provenance, signature, raw_response, fetched_at
        FROM evidence WHERE market_id = $1 ORDER BY fetched_at DESC
        "#,
    )
    .bind(market_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn list_bundles_for_market(
    pool: &PgPool,
    market_id: Uuid,
) -> anyhow::Result<Vec<EvidenceBundleRow>> {
    let rows = sqlx::query_as::<_, EvidenceBundleRow>(
        r#"
        SELECT id, market_id, resolver_job_id, bundle_hash, created_at
        FROM evidence_bundles WHERE market_id = $1 ORDER BY created_at DESC
        "#,
    )
    .bind(market_id)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn upsert_resolver_state(
    pool: &PgPool,
    market_id: Uuid,
    outcome_draft: Option<&str>,
    confidence_bps: Option<i32>,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO resolver_state (market_id, outcome_draft, confidence_bps, last_poll_at, updated_at)
        VALUES ($1, $2, $3, NOW(), NOW())
        ON CONFLICT (market_id) DO UPDATE SET
            outcome_draft = EXCLUDED.outcome_draft,
            confidence_bps = EXCLUDED.confidence_bps,
            last_poll_at = NOW(),
            updated_at = NOW()
        "#,
    )
    .bind(market_id)
    .bind(outcome_draft)
    .bind(confidence_bps)
    .execute(pool)
    .await?;
    Ok(())
}
