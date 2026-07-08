// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use sqlx::PgPool;
use uuid::Uuid;

use crate::lifecycle::{AssetLifecycleState, LifecycleEvent, transition};
use crate::sources::DiscoveryAssetRecord;

pub struct DiscoveryStore {
    pool: PgPool,
}

impl DiscoveryStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn audit(
        &self,
        entity_type: &str,
        entity_id: Uuid,
        action: &str,
        details: serde_json::Value,
    ) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO discovery_audit_log (entity_type, entity_id, action, details)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(entity_type)
        .bind(entity_id)
        .bind(action)
        .bind(details)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn upsert_asset(
        &self,
        source_id: Option<Uuid>,
        record: &DiscoveryAssetRecord,
        priority_score: i64,
        lifecycle: AssetLifecycleState,
    ) -> anyhow::Result<Uuid> {
        let row: (Uuid,) = sqlx::query_as(
            r#"
            INSERT INTO discovery_assets (
                source_id, external_source_url, canonical_metadata, media_type,
                lifecycle_state, source_trust_score, creator_confidence, priority_score
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (external_source_url) DO UPDATE SET
                canonical_metadata = EXCLUDED.canonical_metadata,
                lifecycle_state = EXCLUDED.lifecycle_state,
                source_trust_score = EXCLUDED.source_trust_score,
                creator_confidence = EXCLUDED.creator_confidence,
                priority_score = EXCLUDED.priority_score,
                updated_at = NOW()
            RETURNING id
            "#,
        )
        .bind(source_id)
        .bind(&record.external_source_url)
        .bind(&record.canonical_metadata)
        .bind(&record.media_type)
        .bind(lifecycle.as_str())
        .bind(record.source_trust_score)
        .bind(record.creator_confidence)
        .bind(priority_score)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }

    pub async fn transition_asset(
        &self,
        asset_id: Uuid,
        event: LifecycleEvent,
    ) -> anyhow::Result<AssetLifecycleState> {
        let current: (String,) =
            sqlx::query_as("SELECT lifecycle_state FROM discovery_assets WHERE id = $1")
                .bind(asset_id)
                .fetch_one(&self.pool)
                .await?;
        let from = AssetLifecycleState::parse(&current.0)
            .ok_or_else(|| anyhow::anyhow!("unknown lifecycle state {}", current.0))?;
        let to = transition(from, event)?;
        sqlx::query(
            "UPDATE discovery_assets SET lifecycle_state = $1, updated_at = NOW() WHERE id = $2",
        )
        .bind(to.as_str())
        .bind(asset_id)
        .execute(&self.pool)
        .await?;
        self.audit(
            "discovery_asset",
            asset_id,
            "lifecycle_transition",
            serde_json::json!({ "from": from.as_str(), "to": to.as_str(), "event": format!("{:?}", event) }),
        )
        .await?;
        Ok(to)
    }

    pub async fn enqueue_job(
        &self,
        job_type: &str,
        asset_id: Uuid,
        priority_score: i64,
        payload: serde_json::Value,
    ) -> anyhow::Result<Uuid> {
        let row: (Uuid,) = sqlx::query_as(
            r#"
            INSERT INTO discovery_jobs (job_type, discovery_asset_id, priority_score, payload)
            VALUES ($1, $2, $3, $4)
            RETURNING id
            "#,
        )
        .bind(job_type)
        .bind(asset_id)
        .bind(priority_score)
        .bind(payload)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }

    pub async fn claim_next_job(&self) -> anyhow::Result<Option<DiscoveryJob>> {
        let mut tx = self.pool.begin().await?;
        let row = sqlx::query_as::<_, DiscoveryJob>(
            r#"
            SELECT id, job_type, discovery_asset_id, priority_score, status, attempts, payload
            FROM discovery_jobs
            WHERE status = 'pending'
            ORDER BY priority_score DESC, created_at ASC
            FOR UPDATE SKIP LOCKED
            LIMIT 1
            "#,
        )
        .fetch_optional(&mut *tx)
        .await?;

        let Some(job) = row else {
            tx.commit().await?;
            return Ok(None);
        };

        sqlx::query(
            "UPDATE discovery_jobs SET status = 'processing', attempts = attempts + 1, updated_at = NOW() WHERE id = $1",
        )
        .bind(job.id)
        .execute(&mut *tx)
        .await?;
        tx.commit().await?;
        Ok(Some(job))
    }

    pub async fn complete_job(&self, job_id: Uuid, status: &str, error: Option<&str>) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE discovery_jobs SET status = $1, last_error = $2, updated_at = NOW() WHERE id = $3",
        )
        .bind(status)
        .bind(error)
        .bind(job_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn stats(&self) -> anyhow::Result<DiscoveryStats> {
        let assets: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM discovery_assets")
            .fetch_one(&self.pool)
            .await?;
        let indexed: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM discovery_assets WHERE lifecycle_state = 'indexed'",
        )
        .fetch_one(&self.pool)
        .await?;
        let pending_jobs: (i64,) =
            sqlx::query_as("SELECT COUNT(*) FROM discovery_jobs WHERE status = 'pending'")
                .fetch_one(&self.pool)
                .await?;
        Ok(DiscoveryStats {
            total_assets: assets.0,
            indexed_assets: indexed.0,
            pending_jobs: pending_jobs.0,
        })
    }

    pub async fn record_provenance_hit(
        &self,
        network: &str,
        post_id: &str,
        query_media_id: Option<&str>,
        discovery_asset_id: Option<Uuid>,
        creator_candidate_id: Option<Uuid>,
        similarity_score: f64,
        match_type: Option<&str>,
        work_confidence: f64,
        creator_confidence: f64,
        decision: &str,
        vault_provisioned: bool,
        vault_identity_hash: Option<&str>,
    ) -> anyhow::Result<Uuid> {
        let row: (Uuid,) = sqlx::query_as(
            r#"
            INSERT INTO provenance_hits (
                network, post_id, query_media_id, discovery_asset_id, creator_candidate_id,
                similarity_score, match_type, work_confidence, creator_confidence,
                decision, vault_provisioned, vault_identity_hash
            ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12)
            RETURNING id
            "#,
        )
        .bind(network)
        .bind(post_id)
        .bind(query_media_id)
        .bind(discovery_asset_id)
        .bind(creator_candidate_id)
        .bind(similarity_score)
        .bind(match_type)
        .bind(work_confidence)
        .bind(creator_confidence)
        .bind(decision)
        .bind(vault_provisioned)
        .bind(vault_identity_hash)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct DiscoveryJob {
    pub id: Uuid,
    pub job_type: String,
    pub discovery_asset_id: Option<Uuid>,
    pub priority_score: i64,
    pub status: String,
    pub attempts: i32,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscoveryStats {
    pub total_assets: i64,
    pub indexed_assets: i64,
    pub pending_jobs: i64,
}
