// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Generates recurring public event dates from calendar templates.

use async_trait::async_trait;
use chrono::{Datelike, Utc};

use crate::events::calendar::{
    end_of_utc_day, is_us_midterm_year, is_us_presidential_year, us_election_day,
};
use crate::events::types::{generate_keywords, normalize_discovered_event};
use crate::events::{
    DiscoveredEvent, EventCategory, EventProvider, EventResolverHints, ProviderContext,
    ProviderHealth,
};

pub struct CalendarTemplateProvider;

#[async_trait]
impl EventProvider for CalendarTemplateProvider {
    fn id(&self) -> &str {
        "calendar_template"
    }

    async fn discover(&self, ctx: &ProviderContext) -> anyhow::Result<Vec<DiscoveredEvent>> {
        let horizon_years = ctx
            .config
            .get("horizon_years")
            .and_then(|v| v.as_i64())
            .unwrap_or(6) as i32;
        let templates: Vec<String> = ctx
            .config
            .get("templates")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            })
            .unwrap_or_default();

        let now_year = Utc::now().year();
        let mut out = Vec::new();

        for template in &templates {
            match template.as_str() {
                "us_presidential_election" => {
                    for year in now_year..=now_year + horizon_years {
                        if !is_us_presidential_year(year) {
                            continue;
                        }
                        let end_at = end_of_utc_day(us_election_day(year));
                        if end_at <= Utc::now() {
                            continue;
                        }
                        let label = format!("{year} U.S. Presidential Election");
                        let external_id = format!("us_presidential_election_{year}");
                        let mut keywords = vec![
                            "presidential election".to_string(),
                            "president election".to_string(),
                            "presedintial election".to_string(),
                            format!("election {year}"),
                            format!("{year} election"),
                            year.to_string(),
                        ];
                        keywords.extend(generate_keywords(&label, &[]));
                        keywords.sort();
                        keywords.dedup();
                        let ev = DiscoveredEvent {
                            external_id,
                            label,
                            category: EventCategory::Election,
                            start_at: None,
                            end_at,
                            keywords,
                            entities: vec![],
                            resolver_hints: EventResolverHints {
                                feed_url: None,
                                match_predicate: Some("election".to_string()),
                                preferred_source_keys: vec![
                                    "wikipedia".to_string(),
                                    "ap-news-rss".to_string(),
                                    "reuters-world-rss".to_string(),
                                ],
                            },
                            provenance: serde_json::json!({
                                "source": "calendar_template",
                                "template": "us_presidential_election",
                                "year": year,
                            }),
                            priority: 95,
                            enabled: true,
                        };
                        if let Some(normalized) = normalize_discovered_event(ev) {
                            out.push(normalized);
                        }
                    }
                }
                "us_midterm_election" => {
                    for year in now_year..=now_year + horizon_years {
                        if !is_us_midterm_year(year) {
                            continue;
                        }
                        let end_at = end_of_utc_day(us_election_day(year));
                        if end_at <= Utc::now() {
                            continue;
                        }
                        let label = format!("{year} U.S. Midterm Election");
                        let external_id = format!("us_midterm_election_{year}");
                        let keywords = vec![
                            "midterm election".to_string(),
                            "midterm elections".to_string(),
                            format!("election {year}"),
                            format!("{year} election"),
                            year.to_string(),
                        ];
                        let ev = DiscoveredEvent {
                            external_id,
                            label,
                            category: EventCategory::Election,
                            start_at: None,
                            end_at,
                            keywords,
                            entities: vec![],
                            resolver_hints: EventResolverHints {
                                feed_url: None,
                                match_predicate: Some("election".to_string()),
                                preferred_source_keys: vec![
                                    "wikipedia".to_string(),
                                    "ap-news-rss".to_string(),
                                ],
                            },
                            provenance: serde_json::json!({
                                "source": "calendar_template",
                                "template": "us_midterm_election",
                                "year": year,
                            }),
                            priority: 85,
                            enabled: true,
                        };
                        if let Some(normalized) = normalize_discovered_event(ev) {
                            out.push(normalized);
                        }
                    }
                }
                "fifa_world_cup" => {
                    for year in now_year..=now_year + horizon_years {
                        if year % 4 != 2 {
                            continue;
                        }
                        let end_date = chrono::NaiveDate::from_ymd_opt(year + 1, 7, 19)
                            .unwrap_or_else(|| {
                                chrono::NaiveDate::from_ymd_opt(year, 7, 19).unwrap()
                            });
                        let end_at = end_of_utc_day(end_date);
                        if end_at <= Utc::now() {
                            continue;
                        }
                        let label = format!("FIFA World Cup {year}");
                        let external_id = format!("fifa_world_cup_{year}");
                        let mut keywords = vec![
                            "fifa".to_string(),
                            "world cup".to_string(),
                            "worldcup".to_string(),
                            "soccer".to_string(),
                            "football tournament".to_string(),
                            format!("world cup {year}"),
                        ];
                        keywords.extend(generate_keywords(&label, &[]));
                        keywords.sort();
                        keywords.dedup();
                        let ev = DiscoveredEvent {
                            external_id,
                            label,
                            category: EventCategory::Sports,
                            start_at: None,
                            end_at,
                            keywords,
                            entities: vec![],
                            resolver_hints: EventResolverHints {
                                feed_url: Some(
                                    "https://www.fifa.com/fifaplus/en/articles/rss.xml".to_string(),
                                ),
                                match_predicate: Some("world cup".to_string()),
                                preferred_source_keys: vec![
                                    "fifa-news-rss".to_string(),
                                    "rss_event".to_string(),
                                ],
                            },
                            provenance: serde_json::json!({
                                "source": "calendar_template",
                                "template": "fifa_world_cup",
                                "year": year,
                            }),
                            priority: 100,
                            enabled: true,
                        };
                        if let Some(normalized) = normalize_discovered_event(ev) {
                            out.push(normalized);
                        }
                    }
                }
                _ => {}
            }
        }
        Ok(out)
    }

    async fn health(&self) -> ProviderHealth {
        ProviderHealth::ok("calendar_template ready")
    }
}
