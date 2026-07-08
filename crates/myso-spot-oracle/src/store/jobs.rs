// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct SpotJob {
    pub id: Uuid,
    pub job_type: String,
    pub market_id: Option<Uuid>,
    pub resolver_definition_id: Option<Uuid>,
    pub priority_score: i64,
    pub status: String,
    pub attempts: i32,
    pub max_attempts: i32,
    pub run_after: DateTime<Utc>,
    pub last_error: Option<String>,
    pub payload: serde_json::Value,
}

pub async fn enqueue_job(
    pool: &PgPool,
    job_type: &str,
    market_id: Option<Uuid>,
    resolver_definition_id: Option<Uuid>,
    priority_score: i64,
    run_after: DateTime<Utc>,
    payload: serde_json::Value,
) -> anyhow::Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO spot_jobs (job_type, market_id, resolver_definition_id, priority_score, run_after, payload)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(job_type)
    .bind(market_id)
    .bind(resolver_definition_id)
    .bind(priority_score)
    .bind(run_after)
    .bind(payload)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn claim_next_job(pool: &PgPool) -> anyhow::Result<Option<SpotJob>> {
    let mut tx = pool.begin().await?;
    let row = sqlx::query_as::<_, SpotJob>(
        r#"
        SELECT id, job_type, market_id, resolver_definition_id, priority_score, status,
               attempts, max_attempts, run_after, last_error, payload
        FROM spot_jobs
        WHERE status = 'pending' AND run_after <= NOW()
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
        "UPDATE spot_jobs SET status = 'processing', attempts = attempts + 1, updated_at = NOW() WHERE id = $1",
    )
    .bind(job.id)
    .execute(&mut *tx)
    .await?;
    tx.commit().await?;
    Ok(Some(job))
}

pub async fn complete_job(
    pool: &PgPool,
    job_id: Uuid,
    status: &str,
    error: Option<&str>,
) -> anyhow::Result<()> {
    sqlx::query(
        "UPDATE spot_jobs SET status = $1, last_error = $2, updated_at = NOW() WHERE id = $3",
    )
    .bind(status)
    .bind(error)
    .bind(job_id)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn requeue_job(
    pool: &PgPool,
    job_id: Uuid,
    run_after: DateTime<Utc>,
    error: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE spot_jobs SET
            status = CASE WHEN attempts >= max_attempts THEN 'dead_letter' ELSE 'pending' END,
            run_after = $2,
            last_error = $3,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(job_id)
    .bind(run_after)
    .bind(error)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_jobs(
    pool: &PgPool,
    status: Option<&str>,
    limit: i64,
) -> anyhow::Result<Vec<SpotJob>> {
    let rows = if let Some(status) = status {
        sqlx::query_as::<_, SpotJob>(
            r#"
            SELECT id, job_type, market_id, resolver_definition_id, priority_score, status,
                   attempts, max_attempts, run_after, last_error, payload
            FROM spot_jobs WHERE status = $1
            ORDER BY created_at DESC LIMIT $2
            "#,
        )
        .bind(status)
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, SpotJob>(
            r#"
            SELECT id, job_type, market_id, resolver_definition_id, priority_score, status,
                   attempts, max_attempts, run_after, last_error, payload
            FROM spot_jobs ORDER BY created_at DESC LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(pool)
        .await?
    };
    Ok(rows)
}
