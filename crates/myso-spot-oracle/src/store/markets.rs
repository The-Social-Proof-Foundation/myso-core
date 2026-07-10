// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::claim::lifecycle::{apply_transition, default_context_for, LifecycleEvent, TransitionContext};
use crate::types::MarketStatus;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct MarketRow {
    pub id: Uuid,
    pub post_id: String,
    pub spot_market_object_id: Option<String>,
    pub creator: Option<String>,
    pub claim_text: String,
    pub betting_options: serde_json::Value,
    pub status: String,
    pub status_reason: Option<String>,
    pub on_chain_status: Option<i16>,
    pub last_transition_at: DateTime<Utc>,
    pub review_id: Option<Uuid>,
    pub resolver_definition_id: Option<Uuid>,
    pub resolution_window_ms: i64,
    pub max_resolution_window_ms: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

const MARKET_COLUMNS: &str = r#"
    id, post_id, spot_market_object_id, creator, claim_text, betting_options, status,
    status_reason, on_chain_status, last_transition_at,
    review_id, resolver_definition_id, resolution_window_ms, max_resolution_window_ms,
    created_at, updated_at
"#;

pub async fn market_exists(pool: &PgPool, post_id: &str) -> anyhow::Result<bool> {
    let row: (bool,) = sqlx::query_as("SELECT EXISTS(SELECT 1 FROM markets WHERE post_id = $1)")
        .bind(post_id)
        .fetch_one(pool)
        .await?;
    Ok(row.0)
}

pub async fn insert_market(
    pool: &PgPool,
    post_id: &str,
    creator: &str,
    claim_text: &str,
) -> anyhow::Result<Uuid> {
    // markets.post_id is indexed but not UNIQUE after claim→market redesign; use NOT EXISTS.
    let row: Option<(Uuid,)> = sqlx::query_as(
        r#"
        INSERT INTO markets (post_id, creator, claim_text, status)
        SELECT $1, $2, $3, 'post_created'
        WHERE NOT EXISTS (SELECT 1 FROM markets WHERE post_id = $1)
        RETURNING id
        "#,
    )
    .bind(post_id)
    .bind(creator)
    .bind(claim_text)
    .fetch_optional(pool)
    .await?;
    if let Some(id) = row {
        return Ok(id.0);
    }
    let existing: (Uuid,) = sqlx::query_as("SELECT id FROM markets WHERE post_id = $1 LIMIT 1")
        .bind(post_id)
        .fetch_one(pool)
        .await?;
    Ok(existing.0)
}

pub async fn get_market_by_post_id(pool: &PgPool, post_id: &str) -> anyhow::Result<Option<MarketRow>> {
    let row = sqlx::query_as::<_, MarketRow>(&format!(
        "SELECT {MARKET_COLUMNS} FROM markets WHERE post_id = $1"
    ))
    .bind(post_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn get_market(pool: &PgPool, market_id: Uuid) -> anyhow::Result<Option<MarketRow>> {
    let row = sqlx::query_as::<_, MarketRow>(&format!(
        "SELECT {MARKET_COLUMNS} FROM markets WHERE id = $1"
    ))
    .bind(market_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn list_markets(
    pool: &PgPool,
    status: Option<&str>,
    limit: i64,
) -> anyhow::Result<Vec<MarketRow>> {
    let rows = if let Some(status) = status {
        sqlx::query_as::<_, MarketRow>(&format!(
            "SELECT {MARKET_COLUMNS} FROM markets WHERE status = $1 ORDER BY created_at DESC LIMIT $2"
        ))
        .bind(status)
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, MarketRow>(&format!(
            "SELECT {MARKET_COLUMNS} FROM markets ORDER BY created_at DESC LIMIT $1"
        ))
        .bind(limit)
        .fetch_all(pool)
        .await?
    };
    Ok(rows)
}

pub async fn apply_market_transition(
    pool: &PgPool,
    market_id: Uuid,
    event: &LifecycleEvent,
    ctx: &TransitionContext,
) -> anyhow::Result<MarketStatus> {
    let market = get_market(pool, market_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("market not found"))?;
    let current = MarketStatus::from_str(&market.status).unwrap_or(MarketStatus::PostCreated);
    let next = apply_transition(current, event)?;
    sqlx::query(
        r#"
        UPDATE markets SET
            status = $2,
            status_reason = COALESCE($3, status_reason),
            on_chain_status = COALESCE($4, on_chain_status),
            last_transition_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(market_id)
    .bind(next.as_str())
    .bind(&ctx.status_reason)
    .bind(ctx.on_chain_status)
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO market_transitions (market_id, from_status, to_status, trigger, job_id, tx_digest, status_reason)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(market_id)
    .bind(current.as_str())
    .bind(next.as_str())
    .bind(&ctx.trigger)
    .bind(ctx.job_id)
    .bind(&ctx.tx_digest)
    .bind(&ctx.status_reason)
    .execute(pool)
    .await?;

    Ok(next)
}

pub async fn update_market_status(
    pool: &PgPool,
    market_id: Uuid,
    status: &str,
    review_id: Option<Uuid>,
    resolver_definition_id: Option<Uuid>,
    betting_options: Option<&serde_json::Value>,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE markets SET
            status = $2,
            review_id = COALESCE($3, review_id),
            resolver_definition_id = COALESCE($4, resolver_definition_id),
            betting_options = COALESCE($5, betting_options),
            last_transition_at = NOW(),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(market_id)
    .bind(status)
    .bind(review_id)
    .bind(resolver_definition_id)
    .bind(betting_options)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn transition_with_metadata(
    pool: &PgPool,
    market_id: Uuid,
    event: LifecycleEvent,
    review_id: Option<Uuid>,
    resolver_definition_id: Option<Uuid>,
    betting_options: Option<&serde_json::Value>,
    ctx: TransitionContext,
) -> anyhow::Result<()> {
    let next = apply_market_transition(pool, market_id, &event, &ctx).await?;
    if review_id.is_some() || resolver_definition_id.is_some() || betting_options.is_some() {
        update_market_status(
            pool,
            market_id,
            next.as_str(),
            review_id,
            resolver_definition_id,
            betting_options,
        )
        .await?;
    }
    Ok(())
}

pub async fn set_spot_market_object_id(
    pool: &PgPool,
    market_id: Uuid,
    spot_market_object_id: &str,
    ctx: TransitionContext,
) -> anyhow::Result<()> {
    apply_market_transition(pool, market_id, &LifecycleEvent::CreateTxConfirmed, &ctx).await?;
    sqlx::query(
        "UPDATE markets SET spot_market_object_id = $2, updated_at = NOW() WHERE id = $1",
    )
    .bind(market_id)
    .bind(spot_market_object_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Legacy name — stores the on-chain SpotMarket object id.
pub async fn set_spot_record_id(
    pool: &PgPool,
    market_id: Uuid,
    spot_market_object_id: &str,
) -> anyhow::Result<()> {
    let mut ctx = default_context_for(&LifecycleEvent::CreateTxConfirmed);
    ctx.on_chain_status = Some(1);
    set_spot_market_object_id(pool, market_id, spot_market_object_id, ctx).await
}

pub async fn set_market_resolution_timing(
    pool: &PgPool,
    market_id: Uuid,
    resolution_window_ms: i64,
    max_resolution_window_ms: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE markets SET
            resolution_window_ms = $2,
            max_resolution_window_ms = $3,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(market_id)
    .bind(resolution_window_ms)
    .bind(max_resolution_window_ms)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn max_ingested_created_at_ms(pool: &PgPool) -> anyhow::Result<Option<i64>> {
    let row: Option<(Option<i64>,)> = sqlx::query_as(
        r#"
        SELECT MAX((payload->>'created_at_ms')::bigint)
        FROM spot_jobs
        WHERE job_type = 'ReviewPost'
        "#,
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.and_then(|r| r.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::claim::lifecycle::LifecycleEvent;

    #[test]
    fn market_status_roundtrip() {
        assert_eq!(
            MarketStatus::from_str("waiting"),
            Some(MarketStatus::Waiting)
        );
        assert_eq!(
            MarketStatus::from_str("active"),
            Some(MarketStatus::Waiting)
        );
    }

    #[test]
    fn transition_context_defaults() {
        let ctx = crate::claim::lifecycle::default_context_for(&LifecycleEvent::ReviewAccepted);
        assert_eq!(ctx.trigger, "review_accepted");
    }
}
