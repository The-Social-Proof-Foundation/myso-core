// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! In-memory event registry for keyword/entity matching at review time.

use chrono::{DateTime, TimeZone, Utc};
use std::sync::RwLock;

use crate::events::types::EventEntity;
use crate::store::events::ScheduledEventRow;

/// Resolved scheduled event used by the review pipeline.
#[derive(Debug, Clone)]
pub struct ScheduledEventRecord {
    pub id: uuid::Uuid,
    pub provider_key: String,
    pub external_id: String,
    pub label: String,
    pub category: String,
    pub start_at_ms: Option<i64>,
    pub end_at_ms: i64,
    pub keywords: Vec<String>,
    pub entities: Vec<EventEntity>,
    pub feed_url: Option<String>,
    pub match_predicate: Option<String>,
    pub preferred_source_keys: Vec<String>,
    pub priority: i32,
}

#[derive(Debug, Default)]
pub struct EventRegistry {
    inner: RwLock<Vec<ScheduledEventRecord>>,
}

impl EventRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_rows(rows: Vec<ScheduledEventRow>) -> Self {
        let events: Vec<ScheduledEventRecord> = rows
            .into_iter()
            .filter_map(ScheduledEventRecord::from_row)
            .collect();
        Self {
            inner: RwLock::new(events),
        }
    }

    pub fn reload(&self, rows: Vec<ScheduledEventRow>) {
        let events: Vec<ScheduledEventRecord> = rows
            .into_iter()
            .filter_map(ScheduledEventRecord::from_row)
            .collect();
        if let Ok(mut guard) = self.inner.write() {
            *guard = events;
        }
    }

    pub fn len(&self) -> usize {
        self.inner.read().map(|g| g.len()).unwrap_or(0)
    }

    pub fn all(&self) -> Vec<ScheduledEventRecord> {
        self.inner.read().map(|g| g.clone()).unwrap_or_default()
    }

    /// Match a scheduled event referenced in claim text (active events only).
    pub fn match_event(&self, content: &str) -> Option<ScheduledEventRecord> {
        let normalized = normalize_claim_text(content);
        let claim_years = extract_years(&normalized);
        let now_ms = Utc::now().timestamp_millis();
        let guard = self.inner.read().ok()?;
        let mut best: Option<(ScheduledEventRecord, i32)> = None;

        for ev in guard.iter() {
            if ev.end_at_ms < now_ms {
                continue;
            }
            let score = score_event_match(&normalized, &claim_years, ev);
            if score <= 0 {
                continue;
            }
            match &best {
                None => best = Some((ev.clone(), score)),
                Some((prev, prev_score)) => {
                    if score > *prev_score
                        || (score == *prev_score && ev.priority > prev.priority)
                    {
                        best = Some((ev.clone(), score));
                    }
                }
            }
        }
        best.map(|(ev, _)| ev)
    }

    /// Score how well claim text matches an event (higher is better). Zero = no match.
    pub fn score_match(&self, content: &str, ev: &ScheduledEventRecord) -> i32 {
        let normalized = normalize_claim_text(content);
        let claim_years = extract_years(&normalized);
        score_event_match(&normalized, &claim_years, ev)
    }

    /// Normalize a raw entity token using aliases from the matched event.
    pub fn normalize_entity(&self, raw: &str, matched: Option<&ScheduledEventRecord>) -> String {
        let token = raw
            .split_whitespace()
            .last()
            .unwrap_or(raw)
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_lowercase();
        if let Some(ev) = matched {
            for entity in &ev.entities {
                if entity.name.to_lowercase() == token {
                    return entity.name.clone();
                }
                for alias in &entity.aliases {
                    if alias.to_lowercase() == token {
                        return entity.name.clone();
                    }
                }
            }
        }
        token
    }
}

impl ScheduledEventRecord {
    fn from_row(row: ScheduledEventRow) -> Option<Self> {
        if !row.enabled {
            return None;
        }
        let mut record = Self {
            id: row.id,
            provider_key: row.provider_key,
            external_id: row.external_id,
            label: row.label,
            category: row.category,
            start_at_ms: row.start_at_ms,
            end_at_ms: row.end_at_ms,
            keywords: row.keywords.iter().map(|k| k.to_lowercase()).collect(),
            entities: serde_json::from_value(row.entities).unwrap_or_default(),
            feed_url: row.feed_url,
            match_predicate: row.match_predicate,
            preferred_source_keys: row.preferred_source_keys,
            priority: row.priority,
        };
        apply_admin_override(&mut record, &row.admin_override);
        if record.keywords.is_empty() {
            return None;
        }
        Some(record)
    }
}

fn apply_admin_override(record: &mut ScheduledEventRecord, override_json: &serde_json::Value) {
    if override_json.is_null() || override_json.as_object().is_none_or(|o| o.is_empty()) {
        return;
    }
    if let Some(false) = override_json.get("enabled").and_then(|v| v.as_bool()) {
        record.end_at_ms = 0;
        return;
    }
    if let Some(end_ms) = override_json.get("end_at_ms").and_then(|v| v.as_i64()) {
        record.end_at_ms = end_ms;
    }
    if let Some(extra) = override_json.get("keywords").and_then(|v| v.as_array()) {
        for kw in extra {
            if let Some(s) = kw.as_str() {
                let lower = s.to_lowercase();
                if !lower.is_empty() && !record.keywords.contains(&lower) {
                    record.keywords.push(lower);
                }
            }
        }
    }
    if let Some(extra) = override_json.get("entities").and_then(|v| v.as_array()) {
        for ent in extra {
            if let Ok(e) = serde_json::from_value::<EventEntity>(ent.clone()) {
                record.entities.push(e);
            }
        }
    }
    if let Some(feed) = override_json.get("feed_url").and_then(|v| v.as_str()) {
        record.feed_url = Some(feed.to_string());
    }
    if let Some(pred) = override_json.get("match_predicate").and_then(|v| v.as_str()) {
        record.match_predicate = Some(pred.to_string());
    }
    if let Some(pri) = override_json.get("priority").and_then(|v| v.as_i64()) {
        record.priority = pri as i32;
    }
}

pub fn event_deadline(record: &ScheduledEventRecord) -> DateTime<Utc> {
    DateTime::from_timestamp_millis(record.end_at_ms).unwrap_or_else(Utc::now)
}

pub fn event_deadline_end_of_day(end_at_ms: i64) -> DateTime<Utc> {
    let dt = DateTime::from_timestamp_millis(end_at_ms).unwrap_or_else(Utc::now);
    let date = dt.date_naive();
    date.and_hms_milli_opt(23, 59, 59, 999)
        .map(|t| Utc.from_utc_datetime(&t))
        .unwrap_or(dt)
}

/// Normalize claim text for event matching: lowercase, synonym map, collapse whitespace.
pub fn normalize_claim_text(content: &str) -> String {
    let lower = content.to_lowercase();
    let mut out = lower;
    for (from, to) in CLAIM_SYNONYMS {
        out = out.replace(from, to);
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

const CLAIM_SYNONYMS: &[(&str, &str)] = &[
    ("presedintial", "presidential"),
    ("presidental", "presidential"),
    ("presidiential", "presidential"),
    ("prez", "president"),
    ("potus", "president"),
    ("worldcup", "world cup"),
];

/// Extract four-digit years (2020–2099) from normalized claim text.
pub fn extract_years(normalized: &str) -> Vec<i32> {
    let mut years = Vec::new();
    for token in normalized.split_whitespace() {
        if token.len() == 4 {
            if let Ok(y) = token.parse::<i32>() {
                if (2020..=2099).contains(&y) {
                    years.push(y);
                }
            }
        }
    }
    for token in normalized.split(|c: char| !c.is_ascii_digit()) {
        if token.len() == 4 {
            if let Ok(y) = token.parse::<i32>() {
                if (2020..=2099).contains(&y) && !years.contains(&y) {
                    years.push(y);
                }
            }
        }
    }
    years
}

fn score_event_match(
    normalized: &str,
    claim_years: &[i32],
    ev: &ScheduledEventRecord,
) -> i32 {
    let sports_cues = crate::review::claim_matcher::has_sports_cues(normalized);
    let election_cues = crate::review::claim_matcher::has_election_cues(normalized);

    if sports_cues && !election_cues && ev.category == "election" {
        return 0;
    }
    if election_cues && !sports_cues && ev.category == "sports" {
        return 0;
    }

    let mut score: i32 = 0;

    // Keyword substring hits (longer keywords score higher).
    let mut longest_kw = 0usize;
    for kw in &ev.keywords {
        if kw.len() == 4 && kw.chars().all(|c| c.is_ascii_digit()) {
            continue;
        }
        if normalized.contains(kw) {
            longest_kw = longest_kw.max(kw.len());
            score += kw.len() as i32;
        } else if kw.len() >= 8 && fuzzy_contains(normalized, kw) {
            score += (kw.len() as i32) / 2;
            longest_kw = longest_kw.max(kw.len() / 2);
        }
    }

    // Year in claim matches event label or external_id (requires domain keyword).
    for year in claim_years {
        let year_str = year.to_string();
        if ev.label.contains(&year_str) || ev.external_id.contains(&year_str) {
            if ev.category == "election" && election_cues {
                score += 50;
                if normalized.contains("election") || normalized.contains("president") {
                    score += 30;
                }
            } else if ev.category == "sports" && sports_cues {
                score += 50;
                score += 10;
            } else if ev.category != "election" && ev.category != "sports" {
                score += 40;
            }
        }
    }

    // Entity alias hits (candidates, players, nations).
    for entity in &ev.entities {
        let name_lower = entity.name.to_lowercase();
        if normalized.contains(&name_lower) {
            score += 40;
            if ev.category == "sports" && sports_cues {
                score += 20;
            }
            if ev.category == "election" && election_cues {
                score += 20;
            }
        }
        for alias in &entity.aliases {
            let a = alias.to_lowercase();
            if !a.is_empty() && normalized.contains(&a) {
                score += 35;
            }
        }
    }

    // Outcome phrases boost matching category only.
    if normalized.contains("will win")
        || normalized.contains("will lose")
        || normalized.contains("will be elected")
    {
        if ev.category == "election" && election_cues {
            score += 20;
        }
        if ev.category == "sports" && sports_cues {
            score += 20;
        }
    }

    // Require at least one non-year keyword signal.
    if longest_kw == 0 && score < 50 {
        return 0;
    }

    score + ev.priority / 10
}

/// Fuzzy substring: allow edit distance <= 2 for words in normalized text.
fn fuzzy_contains(haystack: &str, needle: &str) -> bool {
    if needle.len() < 8 {
        return false;
    }
    for word in haystack.split_whitespace() {
        if word.len() >= needle.len().saturating_sub(2) && levenshtein(word, needle) <= 2 {
            return true;
        }
    }
    false
}

fn levenshtein(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let mut prev: Vec<usize> = (0..=b.len()).collect();
    for (i, ca) in a.iter().enumerate() {
        let mut curr = vec![i + 1];
        for (j, cb) in b.iter().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            curr.push(
                (prev[j] + 1)
                    .min(curr[j] + 1)
                    .min(prev[j + 1] + cost),
            );
        }
        prev = curr;
    }
    *prev.last().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::types::EventEntity;
    use crate::store::events::ScheduledEventRow;
    use uuid::Uuid;

    fn fifa_row() -> ScheduledEventRow {
        ScheduledEventRow {
            id: Uuid::new_v4(),
            provider_key: "localnet-seed".to_string(),
            external_id: "fifa_world_cup_2026".to_string(),
            label: "FIFA World Cup 2026".to_string(),
            category: "sports".to_string(),
            start_at_ms: None,
            end_at_ms: chrono::NaiveDate::from_ymd_opt(2027, 7, 19)
                .unwrap()
                .and_hms_opt(23, 59, 59)
                .unwrap()
                .and_utc()
                .timestamp_millis(),
            keywords: vec![
                "fifa".to_string(),
                "world cup".to_string(),
                "messi".to_string(),
                "messy".to_string(),
                "mbappe".to_string(),
                "muppet".to_string(),
            ],
            entities: serde_json::to_value(vec![
                EventEntity {
                    name: "lionel messi".to_string(),
                    aliases: vec!["messi".to_string(), "messy".to_string()],
                    role: Some("player".to_string()),
                },
                EventEntity {
                    name: "kylian mbappe".to_string(),
                    aliases: vec!["mbappe".to_string(), "muppet".to_string()],
                    role: Some("player".to_string()),
                },
            ])
            .unwrap(),
            feed_url: Some("https://www.fifa.com/fifaplus/en/articles/rss.xml".to_string()),
            match_predicate: Some("world cup".to_string()),
            preferred_source_keys: vec!["rss_event".to_string()],
            priority: 100,
            enabled: true,
            provenance: serde_json::json!({}),
            admin_override: serde_json::json!({}),
        }
    }

    #[test]
    fn messi_claim_matches_fifa() {
        let reg = EventRegistry::from_rows(vec![fifa_row()]);
        let ev = reg
            .match_event("Messy will have more goals than the Muppet.")
            .expect("fifa");
        assert_eq!(ev.external_id, "fifa_world_cup_2026");
    }

    #[test]
    fn explicit_fifa_keyword_matches() {
        let reg = EventRegistry::from_rows(vec![fifa_row()]);
        let ev = reg
            .match_event("Brazil wins FIFA World Cup")
            .expect("fifa");
        assert_eq!(ev.external_id, "fifa_world_cup_2026");
    }

    #[test]
    fn spain_world_cup_beats_election_year_only() {
        let election = ScheduledEventRow {
            id: Uuid::new_v4(),
            provider_key: "calendar".to_string(),
            external_id: "us_presidential_election_2028".to_string(),
            label: "2028 U.S. Presidential Election".to_string(),
            category: "election".to_string(),
            start_at_ms: None,
            end_at_ms: chrono::NaiveDate::from_ymd_opt(2028, 11, 7)
                .unwrap()
                .and_hms_milli_opt(23, 59, 59, 999)
                .unwrap()
                .and_utc()
                .timestamp_millis(),
            keywords: vec!["presidential election".to_string(), "2028".to_string()],
            entities: serde_json::json!([]),
            feed_url: None,
            match_predicate: Some("election".to_string()),
            preferred_source_keys: vec![],
            priority: 90,
            enabled: true,
            provenance: serde_json::json!({}),
            admin_override: serde_json::json!({}),
        };
        let reg = EventRegistry::from_rows(vec![fifa_row(), election]);
        let ev = reg
            .match_event("Spain will win the FIFA World Cup in 2026")
            .expect("fifa");
        assert_eq!(ev.external_id, "fifa_world_cup_2026");
    }
}
