// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct EvidenceRow {
    pub id: Uuid,
    pub market_id: Uuid,
    pub resolver_job_id: Option<Uuid>,
    pub adapter_id: String,
    pub source_url: String,
    pub content_hash: String,
    pub raw_response: Option<String>,
    pub fetched_at: DateTime<Utc>,
}

pub async fn insert_evidence(
    pool: &PgPool,
    market_id: Uuid,
    resolver_job_id: Option<Uuid>,
    adapter_id: &str,
    source_url: &str,
    content_hash: &str,
    raw_response: Option<&str>,
) -> anyhow::Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO evidence (market_id, resolver_job_id, adapter_id, source_url, content_hash, raw_response)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(market_id)
    .bind(resolver_job_id)
    .bind(adapter_id)
    .bind(source_url)
    .bind(content_hash)
    .bind(raw_response)
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
        SELECT id, market_id, resolver_job_id, adapter_id, source_url, content_hash, raw_response, fetched_at
        FROM evidence WHERE market_id = $1 ORDER BY fetched_at DESC
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
