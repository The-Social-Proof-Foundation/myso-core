// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Factual response cache backed by `discovery_factual_cache`.

use chrono::{DateTime, Utc};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct CacheEntry {
    pub cache_key: String,
    pub source_url: String,
    pub content_hash: String,
    pub normalized_payload: serde_json::Value,
    pub fetched_at: DateTime<Utc>,
}

pub async fn get(pool: &PgPool, cache_key: &str) -> anyhow::Result<Option<CacheEntry>> {
    let row: Option<(
        String,
        String,
        String,
        serde_json::Value,
        DateTime<Utc>,
    )> = sqlx::query_as(
        r#"
        SELECT cache_key, source_url, content_hash, normalized_payload, fetched_at
        FROM discovery_factual_cache
        WHERE cache_key = $1 AND expires_at > NOW()
        "#,
    )
    .bind(cache_key)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(
        |(cache_key, source_url, content_hash, normalized_payload, fetched_at)| CacheEntry {
            cache_key,
            source_url,
            content_hash,
            normalized_payload,
            fetched_at,
        },
    ))
}

pub async fn put(
    pool: &PgPool,
    cache_key: &str,
    source_id: Option<uuid::Uuid>,
    kind: &str,
    source_url: &str,
    content_hash: &str,
    normalized_payload: &serde_json::Value,
    ttl_secs: i64,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        INSERT INTO discovery_factual_cache
            (cache_key, source_id, kind, source_url, content_hash, normalized_payload, fetched_at, expires_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW(), NOW() + ($7 || ' seconds')::interval)
        ON CONFLICT (cache_key) DO UPDATE SET
            source_url = EXCLUDED.source_url,
            content_hash = EXCLUDED.content_hash,
            normalized_payload = EXCLUDED.normalized_payload,
            fetched_at = NOW(),
            expires_at = EXCLUDED.expires_at
        "#,
    )
    .bind(cache_key)
    .bind(source_id)
    .bind(kind)
    .bind(source_url)
    .bind(content_hash)
    .bind(normalized_payload)
    .bind(ttl_secs.to_string())
    .execute(pool)
    .await?;
    Ok(())
}

pub fn price_cache_key(source_id: &str, asset: &str, quote: &str) -> String {
    format!("price:{source_id}:{asset}:{quote}")
}

pub fn release_cache_key(source_id: &str, owner: &str, repo: &str) -> String {
    format!("release:{source_id}:{owner}:{repo}")
}

pub fn events_cache_key(source_id: &str, feed: &str) -> String {
    format!("events:{source_id}:{feed}")
}
