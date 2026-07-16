// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! ICS/VCALENDAR feed parser for upcoming events.

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, TimeZone, Utc};

use crate::events::types::{generate_keywords, normalize_discovered_event};
use crate::events::{
    DiscoveredEvent, EventCategory, EventProvider, EventResolverHints, ProviderContext,
    ProviderHealth,
};
use crate::sources::http_fetch::HttpFetchClient;

pub struct IcalFeedProvider;

#[async_trait]
impl EventProvider for IcalFeedProvider {
    fn id(&self) -> &str {
        "ical_feed"
    }

    async fn discover(&self, ctx: &ProviderContext) -> anyhow::Result<Vec<DiscoveredEvent>> {
        let feed_url = ctx
            .config
            .get("feed_url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| anyhow::anyhow!("ical_feed provider missing config.feed_url"))?;
        let category = ctx
            .config
            .get("category")
            .and_then(|v| v.as_str())
            .unwrap_or("other");
        let default_predicate = ctx
            .config
            .get("match_predicate")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        let priority = ctx
            .config
            .get("priority")
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        let body = if ctx.live_fetch {
            HttpFetchClient::new().get_text(feed_url).await?.body
        } else {
            return Ok(Vec::new());
        };

        let parsed = parse_ics(&body)?;
        let now = Utc::now();
        let mut out = Vec::new();
        for (idx, vevent) in parsed.into_iter().enumerate() {
            let end_at = vevent.end_at.unwrap_or_else(|| {
                vevent
                    .start_at
                    .unwrap_or(now)
                    .date_naive()
                    .and_hms_milli_opt(23, 59, 59, 999)
                    .map(|t| Utc.from_utc_datetime(&t))
                    .unwrap_or(now)
            });
            if end_at <= now {
                continue;
            }
            let external_id = vevent
                .uid
                .clone()
                .unwrap_or_else(|| format!("ical-{idx}-{}", vevent.summary));
            let label = vevent.summary.clone();
            let keywords = generate_keywords(&label, &[]);
            let ev = DiscoveredEvent {
                external_id,
                label,
                category: EventCategory::from_str(category),
                start_at: vevent.start_at,
                end_at,
                keywords,
                entities: vec![],
                resolver_hints: EventResolverHints {
                    feed_url: Some(feed_url.to_string()),
                    match_predicate: default_predicate.clone(),
                    preferred_source_keys: vec!["rss_event".to_string()],
                },
                provenance: serde_json::json!({
                    "source": "ical_feed",
                    "feed_url": feed_url,
                    "uid": vevent.uid,
                }),
                priority,
                enabled: true,
            };
            if let Some(normalized) = normalize_discovered_event(ev) {
                out.push(normalized);
            }
        }
        Ok(out)
    }

    async fn health(&self) -> ProviderHealth {
        ProviderHealth::ok("ical_feed ready")
    }
}

#[derive(Debug, Clone)]
struct ParsedVevent {
    uid: Option<String>,
    summary: String,
    start_at: Option<DateTime<Utc>>,
    end_at: Option<DateTime<Utc>>,
}

fn parse_ics(body: &str) -> anyhow::Result<Vec<ParsedVevent>> {
    let mut events = Vec::new();
    let mut in_event = false;
    let mut current = ParsedVevent {
        uid: None,
        summary: String::new(),
        start_at: None,
        end_at: None,
    };

    for line in unfold_ics_lines(body) {
        if line == "BEGIN:VEVENT" {
            in_event = true;
            current = ParsedVevent {
                uid: None,
                summary: String::new(),
                start_at: None,
                end_at: None,
            };
            continue;
        }
        if line == "END:VEVENT" {
            if in_event && !current.summary.is_empty() {
                events.push(current.clone());
            }
            in_event = false;
            continue;
        }
        if !in_event {
            continue;
        }
        if let Some((key, value)) = line.split_once(':') {
            let key = key.split(';').next().unwrap_or(key);
            match key {
                "UID" => current.uid = Some(value.to_string()),
                "SUMMARY" => current.summary = unescape_ics(value),
                "DTSTART" => current.start_at = parse_ics_datetime(value),
                "DTEND" => current.end_at = parse_ics_datetime(value),
                _ => {}
            }
        }
    }
    Ok(events)
}

fn unfold_ics_lines(body: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current = String::new();
    for raw in body.lines() {
        if raw.starts_with(' ') || raw.starts_with('\t') {
            if let Some(stripped) = raw.strip_prefix(' ').or_else(|| raw.strip_prefix('\t')) {
                current.push_str(stripped);
            }
        } else {
            if !current.is_empty() {
                lines.push(current.clone());
            }
            current = raw.to_string();
        }
    }
    if !current.is_empty() {
        lines.push(current);
    }
    lines
}

fn unescape_ics(s: &str) -> String {
    s.replace("\\n", "\n")
        .replace("\\,", ",")
        .replace("\\;", ";")
        .replace("\\\\", "\\")
}

fn parse_ics_datetime(value: &str) -> Option<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(value) {
        return Some(dt.with_timezone(&Utc));
    }
    if value.len() == 8 {
        let date = NaiveDate::parse_from_str(value, "%Y%m%d").ok()?;
        return Some(Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0)?));
    }
    if value.len() >= 15 && value.ends_with('Z') {
        let naive = chrono::NaiveDateTime::parse_from_str(value, "%Y%m%dT%H%M%SZ").ok()?;
        return Some(DateTime::from_naive_utc_and_offset(naive, Utc));
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_vevent() {
        let ics = r#"BEGIN:VCALENDAR
BEGIN:VEVENT
UID:test-uid-1
SUMMARY:World Cup Final
DTSTART:20260719T000000Z
DTEND:20260719T235959Z
END:VEVENT
END:VCALENDAR"#;
        let events = parse_ics(ics).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].summary, "World Cup Final");
        assert_eq!(events[0].uid.as_deref(), Some("test-uid-1"));
    }
}
