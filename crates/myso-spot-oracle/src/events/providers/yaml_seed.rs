// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Bootstrap provider that loads seed events from a YAML file (localnet/dev).

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use serde::Deserialize;

use crate::events::{
    DiscoveredEvent, EventCategory, EventEntity, EventProvider, EventResolverHints,
    ProviderContext, ProviderHealth,
};
use crate::events::types::{generate_keywords, normalize_discovered_event};

pub struct YamlSeedProvider;

#[derive(Debug, Deserialize)]
struct SeedEventsFile {
    events: Vec<SeedEvent>,
}

#[derive(Debug, Deserialize)]
struct SeedEvent {
    id: String,
    label: String,
    #[serde(default)]
    category: String,
    #[serde(default)]
    keywords: Vec<String>,
    #[serde(default)]
    entities: Vec<SeedEntity>,
    end_date: String,
    #[serde(default)]
    feed_url: Option<String>,
    #[serde(default)]
    match_predicate: Option<String>,
    #[serde(default)]
    preferred_source_keys: Vec<String>,
    #[serde(default)]
    priority: i32,
    #[serde(default = "default_enabled")]
    enabled: bool,
}

#[derive(Debug, Deserialize)]
struct SeedEntity {
    name: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default)]
    role: Option<String>,
}

fn default_enabled() -> bool {
    true
}

#[async_trait]
impl EventProvider for YamlSeedProvider {
    fn id(&self) -> &str {
        "yaml_seed"
    }

    async fn discover(&self, ctx: &ProviderContext) -> anyhow::Result<Vec<DiscoveredEvent>> {
        let path = ctx
            .config
            .get("events_file")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("yaml_seed provider missing config.events_file"))?;
        let raw = std::fs::read_to_string(path)?;
        let file: SeedEventsFile = serde_yaml::from_str(&raw)?;
        let mut out = Vec::new();
        for seed in file.events {
            let end_at = parse_end_date(&seed.end_date)?;
            let entities: Vec<EventEntity> = seed
                .entities
                .into_iter()
                .map(|e| EventEntity {
                    name: e.name,
                    aliases: e.aliases,
                    role: e.role,
                })
                .collect();
            let mut keywords = seed.keywords;
            if keywords.is_empty() {
                keywords = generate_keywords(&seed.label, &entities);
            }
            let ev = DiscoveredEvent {
                external_id: seed.id,
                label: seed.label,
                category: EventCategory::from_str(&seed.category),
                start_at: None,
                end_at,
                keywords,
                entities,
                resolver_hints: EventResolverHints {
                    feed_url: seed.feed_url,
                    match_predicate: seed.match_predicate,
                    preferred_source_keys: seed.preferred_source_keys,
                },
                provenance: serde_json::json!({
                    "source": "yaml_seed",
                    "events_file": path,
                }),
                priority: seed.priority,
                enabled: seed.enabled,
            };
            if let Some(normalized) = normalize_discovered_event(ev) {
                out.push(normalized);
            }
        }
        Ok(out)
    }

    async fn health(&self) -> ProviderHealth {
        ProviderHealth::ok("yaml_seed ready")
    }
}

fn parse_end_date(s: &str) -> anyhow::Result<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Ok(dt.with_timezone(&Utc));
    }
    let date = NaiveDate::parse_from_str(s, "%Y-%m-%d")
        .map_err(|e| anyhow::anyhow!("invalid end_date {s}: {e}"))?;
    Ok(date
        .and_hms_milli_opt(23, 59, 59, 999)
        .map(|t| Utc.from_utc_datetime(&t))
        .unwrap_or_else(|| Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap())))
}
