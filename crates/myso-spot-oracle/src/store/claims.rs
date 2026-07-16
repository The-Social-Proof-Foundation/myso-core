// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SpotClaimRow {
    pub id: Uuid,
    pub semantic_claim_hash: String,
    pub spot_claim_object_id: Option<String>,
    pub canonical_claim_id: Option<Uuid>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SpotMarketRow {
    pub id: Uuid,
    pub claim_id: Uuid,
    pub market_key_hash: String,
    pub spot_market_object_id: Option<String>,
    pub deadline: Option<DateTime<Utc>>,
    pub status: String,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PostClaimLinkRow {
    pub id: Uuid,
    pub post_id: String,
    pub claim_id: Uuid,
    pub market_id: Option<Uuid>,
    pub oracle_market_id: Option<Uuid>,
    pub link_kind: String,
}

pub async fn get_claim_by_id(pool: &PgPool, claim_id: Uuid) -> anyhow::Result<Option<SpotClaimRow>> {
    let row = sqlx::query_as::<_, SpotClaimRow>(
        r#"
        SELECT id, semantic_claim_hash, spot_claim_object_id, canonical_claim_id
        FROM spot_claims WHERE id = $1
        "#,
    )
    .bind(claim_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn get_spot_market_by_id(
    pool: &PgPool,
    market_id: Uuid,
) -> anyhow::Result<Option<SpotMarketRow>> {
    let row = sqlx::query_as::<_, SpotMarketRow>(
        r#"
        SELECT id, claim_id, market_key_hash, spot_market_object_id, deadline, status
        FROM spot_markets WHERE id = $1
        "#,
    )
    .bind(market_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn find_claim_by_semantic_hash(
    pool: &PgPool,
    semantic_hash: &str,
) -> anyhow::Result<Option<SpotClaimRow>> {
    let row = sqlx::query_as::<_, SpotClaimRow>(
        r#"
        SELECT id, semantic_claim_hash, spot_claim_object_id, canonical_claim_id
        FROM spot_claims WHERE semantic_claim_hash = $1
        "#,
    )
    .bind(semantic_hash)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn find_market_by_key_hash(
    pool: &PgPool,
    market_key_hash: &str,
) -> anyhow::Result<Option<SpotMarketRow>> {
    let row = sqlx::query_as::<_, SpotMarketRow>(
        r#"
        SELECT id, claim_id, market_key_hash, spot_market_object_id, deadline, status
        FROM spot_markets WHERE market_key_hash = $1
        "#,
    )
    .bind(market_key_hash)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn market_key_hash_exists(pool: &PgPool, market_key_hash: &str) -> anyhow::Result<bool> {
    let row: (bool,) = sqlx::query_as(
        "SELECT EXISTS(SELECT 1 FROM spot_markets WHERE market_key_hash = $1)",
    )
    .bind(market_key_hash)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn insert_spot_claim(
    pool: &PgPool,
    semantic_hash: &str,
    canonical_claim_id: Uuid,
) -> anyhow::Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO spot_claims (semantic_claim_hash, canonical_claim_id)
        VALUES ($1, $2)
        ON CONFLICT (semantic_claim_hash) DO UPDATE SET canonical_claim_id = EXCLUDED.canonical_claim_id
        RETURNING id
        "#,
    )
    .bind(semantic_hash)
    .bind(canonical_claim_id)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn find_market_by_object_id(
    pool: &PgPool,
    object_id: &str,
) -> anyhow::Result<Option<SpotMarketRow>> {
    let row = sqlx::query_as::<_, SpotMarketRow>(
        r#"
        SELECT id, claim_id, market_key_hash, spot_market_object_id, deadline, status
        FROM spot_markets WHERE spot_market_object_id = $1
        "#,
    )
    .bind(object_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn find_market_by_outcome_identity_hash(
    pool: &PgPool,
    outcome_identity_hash: &str,
) -> anyhow::Result<Option<SpotMarketRow>> {
    let row = sqlx::query_as::<_, SpotMarketRow>(
        r#"
        SELECT id, claim_id, market_key_hash, spot_market_object_id, deadline, status
        FROM spot_markets WHERE outcome_identity_hash = $1
        "#,
    )
    .bind(outcome_identity_hash)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn find_market_by_graph_refs(
    pool: &PgPool,
    event_ref: &str,
    entity_ref: &str,
    deadline_day: &str,
) -> anyhow::Result<Option<SpotMarketRow>> {
    let row = sqlx::query_as::<_, SpotMarketRow>(
        r#"
        SELECT id, claim_id, market_key_hash, spot_market_object_id, deadline, status
        FROM spot_markets
        WHERE event_ref = $1 AND entity_ref = $2 AND deadline_day = $3
        LIMIT 1
        "#,
    )
    .bind(event_ref)
    .bind(entity_ref)
    .bind(deadline_day)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn insert_spot_market(
    pool: &PgPool,
    claim_id: Uuid,
    market_key_hash: &str,
    deadline: Option<DateTime<Utc>>,
    betting_options: &serde_json::Value,
    resolver_definition_id: Option<Uuid>,
    entity_ref: Option<&str>,
    competition_ref: Option<&str>,
    event_ref: Option<&str>,
    metric_ref: Option<&str>,
    outcome_identity_hash: Option<&str>,
    deadline_day: Option<&str>,
) -> anyhow::Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO spot_markets (
            claim_id, market_key_hash, deadline, betting_options, resolver_definition_id, status,
            entity_ref, competition_ref, event_ref, metric_ref, outcome_identity_hash, deadline_day
        )
        VALUES ($1, $2, $3, $4, $5, 'pending_create', $6, $7, $8, $9, $10, $11)
        ON CONFLICT (market_key_hash) DO UPDATE SET updated_at = NOW()
        RETURNING id
        "#,
    )
    .bind(claim_id)
    .bind(market_key_hash)
    .bind(deadline)
    .bind(betting_options)
    .bind(resolver_definition_id)
    .bind(entity_ref)
    .bind(competition_ref)
    .bind(event_ref)
    .bind(metric_ref)
    .bind(outcome_identity_hash)
    .bind(deadline_day)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn upsert_post_claim_link(
    pool: &PgPool,
    post_id: &str,
    claim_id: Uuid,
    market_id: Option<Uuid>,
    oracle_market_id: Option<Uuid>,
    review_id: Option<Uuid>,
    link_kind: &str,
) -> anyhow::Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO post_claim_links (post_id, claim_id, market_id, oracle_market_id, review_id, link_kind)
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (post_id) DO UPDATE SET
            claim_id = EXCLUDED.claim_id,
            market_id = EXCLUDED.market_id,
            oracle_market_id = EXCLUDED.oracle_market_id,
            review_id = EXCLUDED.review_id,
            link_kind = EXCLUDED.link_kind
        RETURNING id
        "#,
    )
    .bind(post_id)
    .bind(claim_id)
    .bind(market_id)
    .bind(oracle_market_id)
    .bind(review_id)
    .bind(link_kind)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn set_spot_claim_object_id(
    pool: &PgPool,
    claim_id: Uuid,
    object_id: &str,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE spot_claims SET spot_claim_object_id = $2 WHERE id = $1")
        .bind(claim_id)
        .bind(object_id)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn set_spot_market_object_id(
    pool: &PgPool,
    market_id: Uuid,
    object_id: &str,
) -> anyhow::Result<()> {
    sqlx::query("UPDATE spot_markets SET spot_market_object_id = $2, status = 'waiting' WHERE id = $1")
        .bind(market_id)
        .bind(object_id)
        .execute(pool)
        .await?;
    Ok(())
}
