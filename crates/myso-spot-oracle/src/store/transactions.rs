// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TransactionRow {
    pub id: Uuid,
    pub market_id: Option<Uuid>,
    pub tx_kind: String,
    pub nonce: String,
    pub digest: Option<String>,
    pub status: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

pub async fn insert_transaction(
    pool: &PgPool,
    market_id: Option<Uuid>,
    tx_kind: &str,
    nonce: &str,
) -> anyhow::Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO transactions (market_id, tx_kind, nonce, status)
        VALUES ($1, $2, $3, 'pending')
        ON CONFLICT (market_id, tx_kind, nonce) DO UPDATE SET updated_at = NOW()
        RETURNING id
        "#,
    )
    .bind(market_id)
    .bind(tx_kind)
    .bind(nonce)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn update_transaction_status(
    pool: &PgPool,
    tx_id: Uuid,
    status: &str,
    digest: Option<&str>,
    error: Option<&str>,
) -> anyhow::Result<()> {
    let now = Utc::now();
    let (submitted, confirmed) = match status {
        "submitted" => (Some(now), None),
        "confirmed" => (None, Some(now)),
        _ => (None, None),
    };
    sqlx::query(
        r#"
        UPDATE transactions SET
            status = $2,
            digest = COALESCE($3, digest),
            last_error = $4,
            submitted_at = COALESCE($5, submitted_at),
            confirmed_at = COALESCE($6, confirmed_at),
            attempts = attempts + 1,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(tx_id)
    .bind(status)
    .bind(digest)
    .bind(error)
    .bind(submitted)
    .bind(confirmed)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn list_pending_transactions(
    pool: &PgPool,
    limit: i64,
) -> anyhow::Result<Vec<TransactionRow>> {
    let rows = sqlx::query_as::<_, TransactionRow>(
        r#"
        SELECT id, market_id, tx_kind, nonce, digest, status, attempts, last_error,
               submitted_at, confirmed_at, created_at
        FROM transactions
        WHERE status IN ('pending', 'submitted')
        ORDER BY created_at ASC
        LIMIT $1
        "#,
    )
    .bind(limit)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}
