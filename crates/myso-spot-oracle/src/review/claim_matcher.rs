// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Unified claim matcher: registry/graph wins over LLM for identities and deadlines.

use crate::events::registry::{EventRegistry, ScheduledEventRecord};
use crate::review::context_deadline::{apply_deadline_resolution, resolve_context_deadline};
use crate::store::reviews::ExtractedClaim;
use crate::types::ClaimCategory;

#[derive(Debug, Clone)]
pub struct ClaimMatch {
    pub matched_event: Option<ScheduledEventRecord>,
    pub entity_ref: Option<String>,
    pub competition_ref: Option<String>,
    pub event_ref: Option<String>,
    pub metric_ref: Option<String>,
    pub domain: String,
    pub match_tier: &'static str,
}

impl ClaimMatch {
    pub fn empty() -> Self {
        Self {
            matched_event: None,
            entity_ref: None,
            competition_ref: None,
            event_ref: None,
            metric_ref: None,
            domain: "unknown".to_string(),
            match_tier: "none",
        }
    }
}

/// Match claim against the event registry and reconcile LLM extraction (registry always wins).
pub fn match_and_reconcile(
    content: &str,
    extracted: &mut ExtractedClaim,
    registry: &EventRegistry,
) -> ClaimMatch {
    let matched = registry.match_event(content);
    let mut result = ClaimMatch {
        matched_event: matched.clone(),
        event_ref: matched.as_ref().map(|e| e.external_id.clone()),
        competition_ref: matched.as_ref().map(|e| e.category.clone()),
        domain: matched
            .as_ref()
            .map(|e| e.category.clone())
            .unwrap_or_else(|| infer_domain_from_text(content)),
        match_tier: if matched.is_some() {
            "event_registry"
        } else {
            "none"
        },
        ..ClaimMatch::empty()
    };

    if let Some(ref ev) = matched {
        reconcile_with_event(content, extracted, ev, registry);
        result.entity_ref = infer_entity_ref(content, extracted, registry, Some(ev));
        result.domain = ev.category.clone();
    } else {
        result.entity_ref = infer_entity_ref(content, extracted, registry, None);
        if extracted.metric.as_deref() == Some("price") {
            result.metric_ref = Some(format!("price_usd:{}", extracted.subject));
            result.domain = "crypto".to_string();
        }
    }

    if let Some(metric) = extracted.metric.as_ref() {
        if result.metric_ref.is_none() {
            let threshold_suffix = extracted
                .threshold
                .as_ref()
                .map(|t| format!(":{t}"))
                .unwrap_or_default();
            result.metric_ref = Some(format!("{metric}{threshold_suffix}"));
        }
    }

    result
}

fn reconcile_with_event(
    content: &str,
    extracted: &mut ExtractedClaim,
    ev: &ScheduledEventRecord,
    registry: &EventRegistry,
) {
    extracted.claim_category = ClaimCategory::EventOccurrence;
    extracted.resolver_hints.matched_event_id = Some(ev.external_id.clone());
    if extracted.resolver_hints.feed_url.is_none() {
        extracted.resolver_hints.feed_url = ev.feed_url.clone();
    }
    if extracted.resolver_hints.match_predicate.is_none() {
        extracted.resolver_hints.match_predicate = ev.match_predicate.clone();
    }
    if extracted.resolver_hints.preferred_sources.is_empty() {
        if let Ok(domains) = crate::knowledge::config::load_knowledge_domains(
            "crates/myso-spot-oracle/config/knowledge_domains.localnet.yaml",
        ) {
            let preferred =
                crate::knowledge::config::preferred_sources_for_domain(&domains, &ev.category);
            if !preferred.is_empty() {
                extracted.resolver_hints.preferred_sources = preferred;
            }
        }
    }

    if let Some(entity) = infer_entity_ref(content, extracted, registry, Some(ev)) {
        extracted.subject = entity.replace('_', " ");
    }

    if extracted.deadline.is_none() {
        if let Some(resolution) =
            resolve_context_deadline(content, extracted.claim_category, registry)
        {
            apply_deadline_resolution(extracted, &resolution);
        } else if let Some(resolution) = resolve_context_deadline_from_event(ev) {
            apply_deadline_resolution(extracted, &resolution);
        }
    }
}

fn resolve_context_deadline_from_event(
    ev: &ScheduledEventRecord,
) -> Option<crate::review::context_deadline::DeadlineResolution> {
    use crate::events::registry::event_deadline;
    use crate::events::types::EventCategory;
    use crate::review::context_deadline::{
        DeadlineProvenance, DeadlineProvenanceSource, DeadlineResolution,
    };
    use chrono::Utc;

    Some(DeadlineResolution {
        deadline: event_deadline(ev),
        provenance: DeadlineProvenance {
            source: DeadlineProvenanceSource::EventRegistry,
            event_id: Some(ev.external_id.clone()),
            confidence: "high",
            inferred_at: Utc::now(),
        },
        matched_event: Some(ev.clone()),
        event_category: Some(EventCategory::from_str(&ev.category)),
    })
}

fn infer_entity_ref(
    content: &str,
    extracted: &ExtractedClaim,
    registry: &EventRegistry,
    matched: Option<&ScheduledEventRecord>,
) -> Option<String> {
    let subject_lower = extracted.subject.trim().to_lowercase();
    if subject_lower.is_empty() {
        return None;
    }

    if let Some(ev) = matched {
        for entity in &ev.entities {
            let name_lower = entity.name.to_lowercase();
            if subject_lower == name_lower
                || subject_lower.contains(&name_lower)
                || content.to_lowercase().contains(&name_lower)
            {
                return Some(slugify_ref(&entity.name));
            }
            for alias in &entity.aliases {
                let alias_lower = alias.to_lowercase();
                if subject_lower.contains(&alias_lower)
                    || content.to_lowercase().contains(&alias_lower)
                {
                    return Some(slugify_ref(&entity.name));
                }
            }
        }
    }

    let normalized = registry.normalize_entity(&extracted.subject, matched);
    if normalized.is_empty() {
        None
    } else {
        Some(slugify_ref(&normalized))
    }
}

fn slugify_ref(name: &str) -> String {
    name.trim()
        .to_lowercase()
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c
            } else if c.is_whitespace() || c == '-' || c == '.' {
                '_'
            } else {
                '_'
            }
        })
        .collect::<String>()
        .split('_')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("_")
}

fn infer_domain_from_text(content: &str) -> String {
    let normalized = crate::events::registry::normalize_claim_text(content);
    if has_sports_cues(&normalized) {
        return "sports".to_string();
    }
    if normalized.contains("election") || normalized.contains("president") {
        return "election".to_string();
    }
    if normalized.contains("price")
        || normalized.contains("btc")
        || normalized.contains("bitcoin")
        || normalized.contains('$')
    {
        return "crypto".to_string();
    }
    if normalized.contains("earnings") || normalized.contains("eps") {
        return "finance".to_string();
    }
    "unknown".to_string()
}

pub fn has_sports_cues(normalized: &str) -> bool {
    normalized.contains("fifa")
        || normalized.contains("world cup")
        || normalized.contains("super bowl")
        || normalized.contains("tournament")
        || normalized.contains("soccer")
        || normalized.contains("football")
        || normalized.contains("nfl")
        || normalized.contains("goal")
}

pub fn has_election_cues(normalized: &str) -> bool {
    normalized.contains("election")
        || normalized.contains("president")
        || normalized.contains("elected")
        || normalized.contains("inauguration")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::types::EventEntity;
    use crate::review::canonicalize::OutcomeType;
    use crate::store::events::ScheduledEventRow;
    use crate::types::ResolverHints;
    use chrono::NaiveDate;
    use uuid::Uuid;

    fn fifa_registry() -> EventRegistry {
        EventRegistry::from_rows(vec![ScheduledEventRow {
            id: Uuid::new_v4(),
            provider_key: "localnet-seed".to_string(),
            external_id: "fifa_world_cup_2026".to_string(),
            label: "FIFA World Cup 2026".to_string(),
            category: "sports".to_string(),
            start_at_ms: None,
            end_at_ms: NaiveDate::from_ymd_opt(2026, 7, 19)
                .unwrap()
                .and_hms_opt(23, 59, 59)
                .unwrap()
                .and_utc()
                .timestamp_millis(),
            keywords: vec![
                "fifa".to_string(),
                "world cup".to_string(),
                "spain".to_string(),
            ],
            entities: serde_json::to_value(vec![EventEntity {
                name: "spain".to_string(),
                aliases: vec!["spain".to_string()],
                role: Some("nation".to_string()),
            }])
            .unwrap(),
            feed_url: Some("https://www.fifa.com/fifaplus/en/articles/rss.xml".to_string()),
            match_predicate: Some("world cup".to_string()),
            preferred_source_keys: vec!["fifa-news-rss".to_string()],
            priority: 100,
            enabled: true,
            provenance: serde_json::json!({}),
            admin_override: serde_json::json!({}),
        }])
    }

    #[test]
    fn spain_world_cup_matches_sports_not_election() {
        let registry = fifa_registry();
        let mut extracted = ExtractedClaim {
            subject: "Spain".to_string(),
            predicate: "win".to_string(),
            object: "election".to_string(),
            metric: None,
            comparison: None,
            threshold: None,
            deadline: None,
            outcome_type: OutcomeType::Binary,
            suggested_sources: vec![],
            suggested_options: vec!["Yes".to_string(), "No".to_string()],
            claim_category: ClaimCategory::EventOccurrence,
            time_class: crate::types::TimeClass::Future,
            resolver_hints: ResolverHints::default(),
        };
        let claim = "Spain will win the FIFA World Cup";
        let matched = match_and_reconcile(claim, &mut extracted, &registry);
        assert_eq!(matched.domain, "sports");
        assert_eq!(matched.event_ref.as_deref(), Some("fifa_world_cup_2026"));
        assert_eq!(matched.entity_ref.as_deref(), Some("spain"));
        assert_eq!(extracted.claim_category, ClaimCategory::EventOccurrence);
        assert!(extracted.deadline.is_some());
    }
}
