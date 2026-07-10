// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! SQLx repositories for `spot_event_providers` and `spot_scheduled_events`.

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::events::config::EventProviderConfig;
use crate::events::types::DiscoveredEvent;

/// Deterministic UUID v5 namespace for event provider rows.
const PROVIDER_NAMESPACE: Uuid = Uuid::from_bytes([
    0x6d, 0x79, 0x73, 0x6f, 0x2d, 0x73, 0x70, 0x6f, 0x74, 0x2d, 0x65, 0x76, 0x74, 0x2d, 0x76, 0x31,
]);

/// Deterministic UUID v5 namespace for scheduled event rows.
const EVENT_NAMESPACE: Uuid = Uuid::from_bytes([
    0x6d, 0x79, 0x73, 0x6f, 0x2d, 0x73, 0x70, 0x6f, 0x74, 0x2d, 0x65, 0x76, 0x74, 0x2d, 0x72, 0x31,
]);

pub fn provider_uuid(provider_key: &str) -> Uuid {
    Uuid::new_v5(&PROVIDER_NAMESPACE, provider_key.as_bytes())
}

pub fn event_uuid(provider_key: &str, external_id: &str) -> Uuid {
    Uuid::new_v5(
        &EVENT_NAMESPACE,
        format!("{provider_key}:{external_id}").as_bytes(),
    )
}

pub async fn upsert_provider_rows(
    pool: &PgPool,
    providers: &[EventProviderConfig],
) -> anyhow::Result<()> {
    for cfg in providers {
        let config_json = serde_json::to_value(&cfg.config)?;
        sqlx::query(
            r#"
            INSERT INTO spot_event_providers
                (id, provider_key, provider_type, enabled, poll_interval_secs, config)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                provider_key = EXCLUDED.provider_key,
                provider_type = EXCLUDED.provider_type,
                enabled = EXCLUDED.enabled,
                poll_interval_secs = EXCLUDED.poll_interval_secs,
                config = EXCLUDED.config,
                updated_at = NOW()
            "#,
        )
        .bind(provider_uuid(&cfg.id))
        .bind(&cfg.id)
        .bind(&cfg.provider_type)
        .bind(cfg.enabled)
        .bind(cfg.poll_interval_secs as i32)
        .bind(&config_json)
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn list_enabled_providers(pool: &PgPool) -> anyhow::Result<Vec<EventProviderRow>> {
    let rows = sqlx::query_as::<_, EventProviderRow>(
        r#"
        SELECT id, provider_key, provider_type, enabled, poll_interval_secs, config,
               last_sync_at, last_sync_status, health_healthy, health_message
        FROM spot_event_providers
        WHERE enabled = true
        ORDER BY provider_key ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn list_all_providers(pool: &PgPool) -> anyhow::Result<Vec<EventProviderRow>> {
    let rows = sqlx::query_as::<_, EventProviderRow>(
        r#"
        SELECT id, provider_key, provider_type, enabled, poll_interval_secs, config,
               last_sync_at, last_sync_status, health_healthy, health_message
        FROM spot_event_providers
        ORDER BY provider_key ASC
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn get_provider_by_key(
    pool: &PgPool,
    provider_key: &str,
) -> anyhow::Result<Option<EventProviderRow>> {
    let row = sqlx::query_as::<_, EventProviderRow>(
        r#"
        SELECT id, provider_key, provider_type, enabled, poll_interval_secs, config,
               last_sync_at, last_sync_status, health_healthy, health_message
        FROM spot_event_providers
        WHERE provider_key = $1
        "#,
    )
    .bind(provider_key)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn update_provider_sync_status(
    pool: &PgPool,
    provider_key: &str,
    status: &str,
    healthy: bool,
    message: &str,
) -> anyhow::Result<()> {
    sqlx::query(
        r#"
        UPDATE spot_event_providers
        SET last_sync_at = NOW(),
            last_sync_status = $2,
            health_healthy = $3,
            health_message = $4,
            updated_at = NOW()
        WHERE provider_key = $1
        "#,
    )
    .bind(provider_key)
    .bind(status)
    .bind(healthy)
    .bind(message)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn upsert_discovered_events(
    pool: &PgPool,
    provider_key: &str,
    events: &[DiscoveredEvent],
) -> anyhow::Result<usize> {
    let mut count = 0usize;
    for ev in events {
        let keywords: Vec<String> = ev.keywords.clone();
        let entities = serde_json::to_value(&ev.entities)?;
        let provenance = ev.provenance.clone();
        let preferred = ev.resolver_hints.preferred_source_keys.clone();
        let row_id = event_uuid(provider_key, &ev.external_id);

        sqlx::query(
            r#"
            INSERT INTO spot_scheduled_events
                (id, provider_key, external_id, label, category,
                 start_at_ms, end_at_ms, keywords, entities,
                 feed_url, match_predicate, preferred_source_keys,
                 priority, enabled, provenance)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)
            ON CONFLICT (provider_key, external_id) DO UPDATE SET
                label = EXCLUDED.label,
                category = EXCLUDED.category,
                start_at_ms = EXCLUDED.start_at_ms,
                end_at_ms = EXCLUDED.end_at_ms,
                keywords = EXCLUDED.keywords,
                entities = EXCLUDED.entities,
                feed_url = EXCLUDED.feed_url,
                match_predicate = EXCLUDED.match_predicate,
                preferred_source_keys = EXCLUDED.preferred_source_keys,
                priority = EXCLUDED.priority,
                enabled = EXCLUDED.enabled,
                provenance = EXCLUDED.provenance,
                updated_at = NOW()
            "#,
        )
        .bind(row_id)
        .bind(provider_key)
        .bind(&ev.external_id)
        .bind(&ev.label)
        .bind(ev.category.as_str())
        .bind(ev.start_at.map(|t| t.timestamp_millis()))
        .bind(ev.end_at.timestamp_millis())
        .bind(&keywords)
        .bind(&entities)
        .bind(ev.resolver_hints.feed_url.as_deref())
        .bind(ev.resolver_hints.match_predicate.as_deref())
        .bind(&preferred)
        .bind(ev.priority)
        .bind(ev.enabled)
        .bind(&provenance)
        .execute(pool)
        .await?;
        count += 1;
    }
    Ok(count)
}

pub async fn list_active_scheduled_events(pool: &PgPool) -> anyhow::Result<Vec<ScheduledEventRow>> {
    let now_ms = Utc::now().timestamp_millis();
    let rows = sqlx::query_as::<_, ScheduledEventRow>(
        r#"
        SELECT id, provider_key, external_id, label, category,
               start_at_ms, end_at_ms, keywords, entities,
               feed_url, match_predicate, preferred_source_keys,
               priority, enabled, provenance, admin_override
        FROM spot_scheduled_events
        WHERE enabled = true AND end_at_ms >= $1
        ORDER BY priority DESC, end_at_ms ASC
        "#,
    )
    .bind(now_ms)
    .fetch_all(pool)
    .await?;
    Ok(rows)
}

pub async fn list_scheduled_events(
    pool: &PgPool,
    active_only: bool,
    limit: i64,
) -> anyhow::Result<Vec<ScheduledEventRow>> {
    let now_ms = Utc::now().timestamp_millis();
    let rows = if active_only {
        sqlx::query_as::<_, ScheduledEventRow>(
            r#"
            SELECT id, provider_key, external_id, label, category,
                   start_at_ms, end_at_ms, keywords, entities,
                   feed_url, match_predicate, preferred_source_keys,
                   priority, enabled, provenance, admin_override
            FROM spot_scheduled_events
            WHERE enabled = true AND end_at_ms >= $1
            ORDER BY priority DESC, end_at_ms ASC
            LIMIT $2
            "#,
        )
        .bind(now_ms)
        .bind(limit)
        .fetch_all(pool)
        .await?
    } else {
        sqlx::query_as::<_, ScheduledEventRow>(
            r#"
            SELECT id, provider_key, external_id, label, category,
                   start_at_ms, end_at_ms, keywords, entities,
                   feed_url, match_predicate, preferred_source_keys,
                   priority, enabled, provenance, admin_override
            FROM spot_scheduled_events
            ORDER BY end_at_ms DESC
            LIMIT $1
            "#,
        )
        .bind(limit)
        .fetch_all(pool)
        .await?
    };
    Ok(rows)
}

pub async fn count_active_scheduled_events(pool: &PgPool) -> anyhow::Result<i64> {
    let now_ms = Utc::now().timestamp_millis();
    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM spot_scheduled_events
        WHERE enabled = true AND end_at_ms >= $1
        "#,
    )
    .bind(now_ms)
    .fetch_one(pool)
    .await?;
    Ok(count)
}

pub async fn patch_event_override(
    pool: &PgPool,
    event_id: Uuid,
    override_json: &serde_json::Value,
) -> anyhow::Result<bool> {
    let result = sqlx::query(
        r#"
        UPDATE spot_scheduled_events
        SET admin_override = admin_override || $2::jsonb,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(event_id)
    .bind(override_json)
    .execute(pool)
    .await?;
    Ok(result.rows_affected() > 0)
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct EventProviderRow {
    pub id: Uuid,
    pub provider_key: String,
    pub provider_type: String,
    pub enabled: bool,
    pub poll_interval_secs: i32,
    pub config: serde_json::Value,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub last_sync_status: Option<String>,
    pub health_healthy: Option<bool>,
    pub health_message: Option<String>,
}

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct ScheduledEventRow {
    pub id: Uuid,
    pub provider_key: String,
    pub external_id: String,
    pub label: String,
    pub category: String,
    pub start_at_ms: Option<i64>,
    pub end_at_ms: i64,
    pub keywords: Vec<String>,
    pub entities: serde_json::Value,
    pub feed_url: Option<String>,
    pub match_predicate: Option<String>,
    pub preferred_source_keys: Vec<String>,
    pub priority: i32,
    pub enabled: bool,
    pub provenance: serde_json::Value,
    pub admin_override: serde_json::Value,
}
