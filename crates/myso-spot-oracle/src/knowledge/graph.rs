// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! In-memory Knowledge Graph for entity/event/metric matching and relationship traversal.

use std::collections::HashMap;
use std::sync::RwLock;

use crate::events::registry::{normalize_claim_text, EventRegistry, ScheduledEventRecord};
use crate::events::types::EventEntity;
use crate::knowledge::types::DiscoveredKnowledge;
use crate::review::claim_matcher::{has_election_cues, has_sports_cues, ClaimMatch};
use crate::store::reviews::ExtractedClaim;

#[derive(Debug, Clone)]
pub struct KnowledgeEntityRecord {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub aliases: Vec<String>,
    pub domain: String,
}

#[derive(Debug, Clone)]
pub struct KnowledgeEventRecord {
    pub id: String,
    pub competition_id: Option<String>,
    pub label: String,
    pub domain: String,
    pub end_at_ms: i64,
    pub keywords: Vec<String>,
    pub entities: Vec<EventEntity>,
    pub feed_url: Option<String>,
    pub match_predicate: Option<String>,
    pub preferred_source_keys: Vec<String>,
    pub priority: i32,
}

#[derive(Debug, Default)]
pub struct KnowledgeGraph {
    entities: RwLock<HashMap<String, KnowledgeEntityRecord>>,
    events: RwLock<Vec<KnowledgeEventRecord>>,
    relationships: RwLock<Vec<(String, String, String)>>,
    event_registry: EventRegistry,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_discovered(knowledge: &DiscoveredKnowledge) -> Self {
        let graph = Self::new();
        graph.ingest(knowledge);
        graph
    }

    pub fn with_event_registry(registry: EventRegistry) -> Self {
        Self {
            event_registry: registry,
            ..Self::default()
        }
    }

    pub fn ingest(&self, knowledge: &DiscoveredKnowledge) {
        if let Ok(mut entities) = self.entities.write() {
            for ent in &knowledge.entities {
                entities.insert(
                    ent.id.clone(),
                    KnowledgeEntityRecord {
                        id: ent.id.clone(),
                        kind: ent.kind.clone(),
                        name: ent.name.clone(),
                        aliases: ent.aliases.clone(),
                        domain: ent.domain.clone(),
                    },
                );
            }
        }
        if let Ok(mut events) = self.events.write() {
            for ev in &knowledge.events {
                events.push(KnowledgeEventRecord {
                    id: ev.id.clone(),
                    competition_id: ev.competition_id.clone(),
                    label: ev.label.clone(),
                    domain: ev.domain.clone(),
                    end_at_ms: ev.end_at.timestamp_millis(),
                    keywords: ev.keywords.iter().map(|k| k.to_lowercase()).collect(),
                    entities: ev.entities.clone(),
                    feed_url: ev.feed_url.clone(),
                    match_predicate: ev.match_predicate.clone(),
                    preferred_source_keys: ev.preferred_source_keys.clone(),
                    priority: ev.priority,
                });
            }
        }
        if let Ok(mut rels) = self.relationships.write() {
            for rel in &knowledge.relationships {
                rels.push((
                    rel.subject_id.clone(),
                    rel.rel_type.clone(),
                    rel.object_id.clone(),
                ));
            }
        }
    }

    pub fn reload_from_event_registry(&mut self, registry: &EventRegistry) {
        let rows: Vec<crate::store::events::ScheduledEventRow> = registry
            .all()
            .into_iter()
            .map(|ev| crate::store::events::ScheduledEventRow {
                id: ev.id,
                provider_key: ev.provider_key,
                external_id: ev.external_id,
                label: ev.label,
                category: ev.category,
                start_at_ms: ev.start_at_ms,
                end_at_ms: ev.end_at_ms,
                keywords: ev.keywords,
                entities: serde_json::to_value(&ev.entities).unwrap_or_default(),
                feed_url: ev.feed_url,
                match_predicate: ev.match_predicate,
                preferred_source_keys: ev.preferred_source_keys,
                priority: ev.priority,
                enabled: true,
                provenance: serde_json::json!({}),
                admin_override: serde_json::json!({}),
            })
            .collect();
        self.event_registry = EventRegistry::from_rows(rows);
    }

    pub fn match_claim(&self, content: &str, extracted: &ExtractedClaim) -> ClaimMatch {
        let normalized = normalize_claim_text(content);
        let matched_event = self.event_registry.match_event(content);
        let entity_ref = self.match_entity(&normalized, extracted, matched_event.as_ref());
        let metric_ref = extracted.metric.as_ref().map(|metric| {
            let threshold_suffix = extracted
                .threshold
                .as_ref()
                .map(|t| format!(":{t}"))
                .unwrap_or_default();
            format!("{metric}{threshold_suffix}")
        });

        let domain = matched_event
            .as_ref()
            .map(|e| e.category.clone())
            .unwrap_or_else(|| {
                if has_sports_cues(&normalized) {
                    "sports".to_string()
                } else if has_election_cues(&normalized) {
                    "election".to_string()
                } else {
                    "unknown".to_string()
                }
            });

        let match_tier = if matched_event.is_some() {
            "knowledge_graph"
        } else if entity_ref.is_some() {
            "entity_registry"
        } else {
            "none"
        };

        ClaimMatch {
            matched_event: matched_event.clone(),
            entity_ref,
            competition_ref: matched_event.as_ref().map(|e| e.category.clone()),
            event_ref: matched_event.as_ref().map(|e| e.external_id.clone()),
            metric_ref,
            domain,
            match_tier,
        }
    }

    fn match_entity(
        &self,
        normalized: &str,
        extracted: &ExtractedClaim,
        matched_event: Option<&ScheduledEventRecord>,
    ) -> Option<String> {
        if let Some(ev) = matched_event {
            for entity in &ev.entities {
                if self.entity_token_match(normalized, &entity.name, &entity.aliases) {
                    return Some(slugify(&entity.name));
                }
            }
        }

        let entities = self.entities.read().ok()?;
        for ent in entities.values() {
            if self.entity_token_match(normalized, &ent.name, &ent.aliases) {
                return Some(ent.id.clone());
            }
        }

        let subject = extracted.subject.trim().to_lowercase();
        if !subject.is_empty() {
            return Some(slugify(&subject));
        }
        None
    }

    fn entity_token_match(&self, normalized: &str, name: &str, aliases: &[String]) -> bool {
        let name_lower = name.to_lowercase();
        if normalized.contains(&name_lower) {
            return true;
        }
        aliases.iter().any(|alias| {
            let a = alias.to_lowercase();
            !a.is_empty() && normalized.contains(&a)
        })
    }
}

fn slugify(name: &str) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::types::EventEntity;
    use crate::knowledge::types::{DiscoveredEntity, DiscoveredEvent, DiscoveredKnowledge};
    use crate::review::canonicalize::OutcomeType;
    use crate::types::{ClaimCategory, ResolverHints};
    use chrono::Utc;

    #[test]
    fn graph_matches_entity_from_registry() {
        let knowledge = DiscoveredKnowledge {
            entities: vec![DiscoveredEntity {
                id: "spain".to_string(),
                kind: "nation".to_string(),
                name: "Spain".to_string(),
                aliases: vec!["spain".to_string()],
                domain: "sports".to_string(),
                external_refs: serde_json::json!({}),
                provenance: serde_json::json!({}),
            }],
            events: vec![DiscoveredEvent {
                id: "fifa_world_cup_2026".to_string(),
                competition_id: Some("fifa_world_cup".to_string()),
                label: "FIFA World Cup 2026".to_string(),
                domain: "sports".to_string(),
                start_at: None,
                end_at: Utc::now() + chrono::Duration::days(30),
                keywords: vec!["fifa".to_string(), "world cup".to_string()],
                entities: vec![EventEntity {
                    name: "spain".to_string(),
                    aliases: vec!["spain".to_string()],
                    role: Some("nation".to_string()),
                }],
                feed_url: None,
                match_predicate: Some("world cup".to_string()),
                preferred_source_keys: vec![],
                priority: 100,
                provenance: serde_json::json!({}),
            }],
            ..Default::default()
        };
        let graph = KnowledgeGraph::from_discovered(&knowledge);
        let extracted = ExtractedClaim {
            subject: "Spain".to_string(),
            predicate: "win".to_string(),
            object: "tournament".to_string(),
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
        let matched = graph.match_claim("Spain will win the FIFA World Cup", &extracted);
        assert_eq!(matched.entity_ref.as_deref(), Some("spain"));
    }
}
