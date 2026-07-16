// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Context-Aware Deadline Resolver (CADR) — infers resolution deadlines from
//! claim text, scheduled events, and calendar templates after provability passes.

use chrono::{DateTime, Utc};

use crate::events::calendar::{
    end_of_utc_day, quarter_end, us_midterm_election_deadline, us_presidential_election_deadline,
};
use crate::events::registry::{event_deadline, EventRegistry, ScheduledEventRecord};
use crate::events::types::EventCategory;
use crate::review::deadline::{
    parse_calendar_text_deadline, parse_explicit_deadline_from_text,
    bucket_deadline_for_market_key, ceil_to_spacing,
    DEFAULT_PRICE_MARKET_SPACING, MIN_DEADLINE_LEAD,
};
use crate::types::ClaimCategory;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeadlineProvenanceSource {
    ExplicitText,
    EventRegistry,
    CalendarTemplate,
    PriceSpacing,
}

impl DeadlineProvenanceSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ExplicitText => "text",
            Self::EventRegistry => "event_registry",
            Self::CalendarTemplate => "calendar_template",
            Self::PriceSpacing => "price_spacing",
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeadlineProvenance {
    pub source: DeadlineProvenanceSource,
    pub event_id: Option<String>,
    pub confidence: &'static str,
    pub inferred_at: DateTime<Utc>,
}

impl DeadlineProvenance {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "source": self.source.as_str(),
            "event_id": self.event_id,
            "confidence": self.confidence,
            "inferred_at": self.inferred_at.to_rfc3339(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct DeadlineResolution {
    pub deadline: DateTime<Utc>,
    pub provenance: DeadlineProvenance,
    pub matched_event: Option<ScheduledEventRecord>,
    pub event_category: Option<EventCategory>,
}

/// Resolve a claim deadline using CADR tiers: explicit text → event registry → calendar templates → price spacing.
pub fn resolve_context_deadline(
    content: &str,
    category: ClaimCategory,
    registry: &EventRegistry,
) -> Option<DeadlineResolution> {
    let now = Utc::now();

    if let Some(dt) = parse_explicit_deadline_from_text(content) {
        return Some(DeadlineResolution {
            deadline: dt,
            provenance: DeadlineProvenance {
                source: DeadlineProvenanceSource::ExplicitText,
                event_id: None,
                confidence: "high",
                inferred_at: now,
            },
            matched_event: None,
            event_category: None,
        });
    }

    if let Some(ev) = registry.match_event(content) {
        let event_cat = EventCategory::from_str(&ev.category);
        return Some(DeadlineResolution {
            deadline: event_deadline(&ev),
            provenance: DeadlineProvenance {
                source: DeadlineProvenanceSource::EventRegistry,
                event_id: Some(ev.external_id.clone()),
                confidence: "high",
                inferred_at: now,
            },
            matched_event: Some(ev),
            event_category: Some(event_cat),
        });
    }

    if let Some(dt) = parse_calendar_template_deadline(content) {
        let normalized = crate::events::registry::normalize_claim_text(content);
        let event_category = if crate::review::claim_matcher::has_sports_cues(&normalized) {
            Some(EventCategory::Sports)
        } else if crate::review::claim_matcher::has_election_cues(&normalized) {
            Some(EventCategory::Election)
        } else {
            None
        };
        return Some(DeadlineResolution {
            deadline: dt,
            provenance: DeadlineProvenance {
                source: DeadlineProvenanceSource::CalendarTemplate,
                event_id: None,
                confidence: "high",
                inferred_at: now,
            },
            matched_event: None,
            event_category,
        });
    }

    if let Some(dt) = parse_calendar_text_deadline(content) {
        return Some(DeadlineResolution {
            deadline: dt,
            provenance: DeadlineProvenance {
                source: DeadlineProvenanceSource::ExplicitText,
                event_id: None,
                confidence: "medium",
                inferred_at: now,
            },
            matched_event: None,
            event_category: None,
        });
    }

    if category == ClaimCategory::PriceThreshold {
        let earliest = Utc::now() + MIN_DEADLINE_LEAD;
        let dt = ceil_to_spacing(earliest, DEFAULT_PRICE_MARKET_SPACING);
        return Some(DeadlineResolution {
            deadline: dt,
            provenance: DeadlineProvenance {
                source: DeadlineProvenanceSource::PriceSpacing,
                event_id: None,
                confidence: "medium",
                inferred_at: now,
            },
            matched_event: None,
            event_category: None,
        });
    }

    None
}

/// Apply inferred deadline + provenance onto extracted claim fields.
pub fn apply_deadline_resolution(
    extracted: &mut crate::store::reviews::ExtractedClaim,
    resolution: &DeadlineResolution,
) {
    extracted.deadline = Some(resolution.deadline);
    extracted.resolver_hints.deadline_provenance = Some(resolution.provenance.to_json());
    if let Some(ref ev) = resolution.matched_event {
        extracted.resolver_hints.matched_event_id = Some(ev.external_id.clone());
        if extracted.resolver_hints.feed_url.is_none() {
            extracted.resolver_hints.feed_url = ev.feed_url.clone();
        }
        if extracted.resolver_hints.match_predicate.is_none() {
            extracted.resolver_hints.match_predicate = ev.match_predicate.clone();
        }
        if extracted.resolver_hints.preferred_sources.is_empty()
            && !ev.preferred_source_keys.is_empty()
        {
            extracted.resolver_hints.preferred_sources = ev.preferred_source_keys.clone();
        }
    }
}

fn parse_calendar_template_deadline(content: &str) -> Option<DateTime<Utc>> {
    let normalized = crate::events::registry::normalize_claim_text(content);
    let years = crate::events::registry::extract_years(&normalized);
    let sports_cues = crate::review::claim_matcher::has_sports_cues(&normalized);
    let election_cues = crate::review::claim_matcher::has_election_cues(&normalized);

    if sports_cues && !election_cues {
        for year in &years {
            if year % 4 == 2 || year % 4 == 3 {
                let cup_year = if year % 4 == 3 { *year } else { *year + 1 };
                if let Some(date) = chrono::NaiveDate::from_ymd_opt(cup_year, 7, 19) {
                    return Some(end_of_utc_day(date));
                }
            }
        }
        if normalized.contains("world cup") || normalized.contains("fifa") {
            if let Some(year) = years.first() {
                let cup_year = if *year % 4 == 2 { *year + 1 } else { *year };
                if let Some(date) = chrono::NaiveDate::from_ymd_opt(cup_year, 7, 19) {
                    return Some(end_of_utc_day(date));
                }
            }
        }
        return None;
    }

    if normalized.contains("presidential election")
        || normalized.contains("president election")
        || (normalized.contains("election") && normalized.contains("president"))
    {
        for year in &years {
            if let Some(dt) = us_presidential_election_deadline(*year) {
                return Some(dt);
            }
        }
    }

    if normalized.contains("midterm election") || normalized.contains("midterm elections") {
        for year in &years {
            if let Some(dt) = us_midterm_election_deadline(*year) {
                return Some(dt);
            }
        }
    }

    if normalized.contains("election") && election_cues {
        for year in &years {
            if let Some(dt) = us_presidential_election_deadline(*year) {
                return Some(dt);
            }
            if let Some(dt) = us_midterm_election_deadline(*year) {
                return Some(dt);
            }
        }
    }

    if let Some((year, q)) = parse_quarter(&normalized) {
        if let Some(date) = quarter_end(year, q) {
            return Some(end_of_utc_day(date));
        }
    }

    if years.len() == 1 && normalized.contains("election") {
        return us_presidential_election_deadline(years[0])
            .or_else(|| us_midterm_election_deadline(years[0]));
    }

    None
}

fn parse_quarter(normalized: &str) -> Option<(i32, u32)> {
    for token in normalized.split_whitespace() {
        if token.len() == 6 && token.starts_with('q') {
            if let Ok(q) = token[1..2].parse::<u32>() {
                if let Ok(year) = token[2..].parse::<i32>() {
                    if (1..=4).contains(&q) {
                        return Some((year, q));
                    }
                }
            }
        }
    }
    for (pat, q) in [("q1", 1u32), ("q2", 2), ("q3", 3), ("q4", 4)] {
        if let Some(idx) = normalized.find(pat) {
            let rest = &normalized[idx + 2..];
            for token in rest.split_whitespace() {
                if token.len() == 4 {
                    if let Ok(year) = token.parse::<i32>() {
                        return Some((year, q));
                    }
                }
            }
        }
    }
    None
}

pub fn bucket_price_deadline(deadline: DateTime<Utc>) -> DateTime<Utc> {
    bucket_deadline_for_market_key(deadline, DEFAULT_PRICE_MARKET_SPACING)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::registry::EventRegistry;
    use crate::events::types::EventEntity;
    use crate::store::events::ScheduledEventRow;
    use chrono::NaiveDate;
    use uuid::Uuid;

    fn election_registry() -> EventRegistry {
        EventRegistry::from_rows(vec![ScheduledEventRow {
            id: Uuid::new_v4(),
            provider_key: "localnet-seed".to_string(),
            external_id: "us_presidential_election_2028".to_string(),
            label: "2028 U.S. Presidential Election".to_string(),
            category: "election".to_string(),
            start_at_ms: None,
            end_at_ms: NaiveDate::from_ymd_opt(2028, 11, 7)
                .unwrap()
                .and_hms_milli_opt(23, 59, 59, 999)
                .unwrap()
                .and_utc()
                .timestamp_millis(),
            keywords: vec![
                "presidential election".to_string(),
                "presedintial election".to_string(),
                "2028".to_string(),
            ],
            entities: serde_json::to_value(vec![EventEntity {
                name: "jd vance".to_string(),
                aliases: vec!["vance".to_string(), "j.d. vance".to_string()],
                role: Some("candidate".to_string()),
            }])
            .unwrap(),
            feed_url: Some("https://apnews.com/hub/ap-top-news?output=rss".to_string()),
            match_predicate: Some("election".to_string()),
            preferred_source_keys: vec!["wikipedia".to_string(), "ap-news-rss".to_string()],
            priority: 90,
            enabled: true,
            provenance: serde_json::json!({}),
            admin_override: serde_json::json!({}),
        }])
    }

    #[test]
    fn vance_2028_typo_resolves_via_event_registry() {
        let reg = election_registry();
        let claim = "JD Vance will win the 2028 presedintial election.";
        let res = resolve_context_deadline(claim, ClaimCategory::EventOccurrence, &reg)
            .expect("deadline");
        assert_eq!(res.deadline.date_naive(), NaiveDate::from_ymd_opt(2028, 11, 7).unwrap());
        assert_eq!(
            res.provenance.source,
            DeadlineProvenanceSource::EventRegistry
        );
        assert!(res.matched_event.is_some());
    }

    #[test]
    fn calendar_template_presidential_year() {
        let reg = EventRegistry::new();
        let res = resolve_context_deadline(
            "Who wins the 2028 presidential election?",
            ClaimCategory::EventOccurrence,
            &reg,
        )
        .expect("calendar");
        assert_eq!(res.deadline.date_naive(), NaiveDate::from_ymd_opt(2028, 11, 7).unwrap());
        assert_eq!(
            res.provenance.source,
            DeadlineProvenanceSource::CalendarTemplate
        );
    }
}
