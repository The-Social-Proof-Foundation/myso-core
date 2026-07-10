// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Core types for the Event Provider framework.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Event domain category for matching and metrics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventCategory {
    Sports,
    Election,
    Macro,
    Release,
    Other,
}

impl EventCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sports => "sports",
            Self::Election => "election",
            Self::Macro => "macro",
            Self::Release => "release",
            Self::Other => "other",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "sports" => Self::Sports,
            "election" => Self::Election,
            "macro" => Self::Macro,
            "release" => Self::Release,
            _ => Self::Other,
        }
    }
}

/// Participant in an event (player, team, candidate, etc.).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEntity {
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub role: Option<String>,
}

/// Resolver hints propagated from a discovered event into claim compilation.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EventResolverHints {
    #[serde(default)]
    pub feed_url: Option<String>,
    #[serde(default)]
    pub match_predicate: Option<String>,
    #[serde(default)]
    pub preferred_source_keys: Vec<String>,
}

/// Normalized event emitted by a provider before Postgres upsert.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredEvent {
    pub external_id: String,
    pub label: String,
    pub category: EventCategory,
    #[serde(default)]
    pub start_at: Option<DateTime<Utc>>,
    pub end_at: DateTime<Utc>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub entities: Vec<EventEntity>,
    #[serde(default)]
    pub resolver_hints: EventResolverHints,
    #[serde(default)]
    pub provenance: serde_json::Value,
    #[serde(default = "default_priority")]
    pub priority: i32,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_priority() -> i32 {
    0
}

fn default_enabled() -> bool {
    true
}

/// Runtime context passed to providers during discovery.
#[derive(Debug, Clone)]
pub struct ProviderContext {
    pub provider_key: String,
    pub config: serde_json::Value,
    pub live_fetch: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderHealth {
    pub healthy: bool,
    pub message: String,
}

impl ProviderHealth {
    pub fn ok(message: impl Into<String>) -> Self {
        Self {
            healthy: true,
            message: message.into(),
        }
    }

    pub fn unhealthy(message: impl Into<String>) -> Self {
        Self {
            healthy: false,
            message: message.into(),
        }
    }
}

const MAX_KEYWORDS: usize = 64;

/// Normalize and validate a discovered event before upsert.
pub fn normalize_discovered_event(mut ev: DiscoveredEvent) -> Option<DiscoveredEvent> {
    if ev.external_id.trim().is_empty() || ev.label.trim().is_empty() {
        return None;
    }
    if ev.end_at <= Utc::now() {
        ev.enabled = false;
    }
    if ev.keywords.is_empty() {
        ev.keywords = generate_keywords(&ev.label, &ev.entities);
    }
    ev.keywords = ev
        .keywords
        .into_iter()
        .map(|k| k.trim().to_lowercase())
        .filter(|k| !k.is_empty())
        .take(MAX_KEYWORDS)
        .collect();
    if ev.keywords.is_empty() {
        return None;
    }
    Some(ev)
}

pub fn generate_keywords(label: &str, entities: &[EventEntity]) -> Vec<String> {
    let mut out = Vec::new();
    let lower_label = label.to_lowercase();
    for token in lower_label.split(|c: char| !c.is_alphanumeric()) {
        if token.len() >= 3 {
            out.push(token.to_string());
        }
    }
    if lower_label.len() >= 4 {
        out.push(lower_label.clone());
    }
    for entity in entities {
        out.push(entity.name.to_lowercase());
        for alias in &entity.aliases {
            let a = alias.to_lowercase();
            if !a.is_empty() {
                out.push(a);
            }
        }
    }
    out.sort();
    out.dedup();
    out.truncate(MAX_KEYWORDS);
    out
}
