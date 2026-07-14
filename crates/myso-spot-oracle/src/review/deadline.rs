// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Deterministic UTC deadline extraction from natural-language claim text.

use chrono::{DateTime, Datelike, Duration, NaiveDate, TimeZone, Timelike, Utc};

use crate::events::registry::{event_deadline, EventRegistry};
use crate::types::ClaimCategory;

pub const MIN_DEADLINE_LEAD: Duration = Duration::minutes(5);
pub const MAX_DEADLINE_HORIZON: Duration = Duration::days(730);
pub const DEFAULT_PRICE_MARKET_SPACING: Duration = Duration::minutes(30);

#[derive(Debug, Clone, Copy)]
pub struct DeadlinePolicy {
    pub min_lead: Duration,
    pub max_horizon: Duration,
}

impl Default for DeadlinePolicy {
    fn default() -> Self {
        Self {
            min_lead: MIN_DEADLINE_LEAD,
            max_horizon: MAX_DEADLINE_HORIZON,
        }
    }
}

impl DeadlinePolicy {
    pub fn from_secs(min_lead_secs: u64, max_horizon_secs: u64) -> Self {
        Self {
            min_lead: Duration::seconds(min_lead_secs as i64),
            max_horizon: Duration::seconds(max_horizon_secs as i64),
        }
    }

    pub fn validate(&self, deadline: DateTime<Utc>) -> DeadlineValidation {
        let now = Utc::now();
        if deadline <= now + self.min_lead {
            DeadlineValidation::InPast
        } else if deadline > now + self.max_horizon {
            DeadlineValidation::TooFar
        } else {
            DeadlineValidation::Ok
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeadlineValidation {
    Ok,
    InPast,
    TooFar,
}

/// Parse an absolute resolution deadline from claim text (UTC). Returns `None` when
/// no unambiguous expiration is present.
pub fn parse_deadline_from_text(content: &str) -> Option<DateTime<Utc>> {
    let lower = content.to_lowercase();
    let now = Utc::now();

    if let Some(dt) = parse_relative_duration(&lower, now) {
        return Some(dt);
    }
    if let Some(dt) = parse_tomorrow_phrases(&lower, now) {
        return Some(dt);
    }
    if let Some(dt) = parse_iso_date(&lower) {
        return Some(end_of_utc_day(dt));
    }
    if let Some(dt) = parse_named_date(&lower) {
        return Some(end_of_utc_day(dt));
    }
    None
}

/// Resolve the claim evaluation deadline: explicit text, known event end, or (price only)
/// the next spacing boundary for ongoing markets.
pub fn resolve_claim_deadline(
    content: &str,
    category: ClaimCategory,
    registry: &EventRegistry,
) -> Option<DateTime<Utc>> {
    if let Some(dt) = parse_deadline_from_text(content) {
        return Some(dt);
    }
    if let Some(ev) = registry.match_event(content) {
        return Some(event_deadline(&ev));
    }
    if category == ClaimCategory::PriceThreshold {
        let earliest = Utc::now() + MIN_DEADLINE_LEAD;
        return Some(ceil_to_spacing(earliest, DEFAULT_PRICE_MARKET_SPACING));
    }
    None
}

/// Floor a deadline to a wall-clock spacing bucket for shared market identity (price claims).
pub fn bucket_deadline_for_market_key(
    deadline: DateTime<Utc>,
    spacing: Duration,
) -> DateTime<Utc> {
    floor_to_spacing(deadline, spacing)
}

pub fn ceil_to_spacing(dt: DateTime<Utc>, spacing: Duration) -> DateTime<Utc> {
    let spacing_minutes = spacing.num_minutes().max(1);
    let date = dt.date_naive();
    let total_minutes = date.num_days_from_ce() as i64 * 24 * 60
        + i64::from(dt.hour()) * 60
        + i64::from(dt.minute());
    let rem = total_minutes % spacing_minutes;
    if rem == 0 && dt.second() == 0 && dt.nanosecond() == 0 {
        return dt;
    }
    let ceil_minutes = total_minutes + (spacing_minutes - rem);
    minutes_since_epoch_to_utc(ceil_minutes)
}

fn floor_to_spacing(dt: DateTime<Utc>, spacing: Duration) -> DateTime<Utc> {
    let spacing_minutes = spacing.num_minutes().max(1);
    let date = dt.date_naive();
    let total_minutes = date.num_days_from_ce() as i64 * 24 * 60
        + i64::from(dt.hour()) * 60
        + i64::from(dt.minute());
    let floored = total_minutes - (total_minutes % spacing_minutes);
    minutes_since_epoch_to_utc(floored)
}

fn minutes_since_epoch_to_utc(total_minutes: i64) -> DateTime<Utc> {
    let days = total_minutes.div_euclid(24 * 60);
    let minutes = total_minutes.rem_euclid(24 * 60);
    let hours = minutes / 60;
    let mins = minutes % 60;
    NaiveDate::from_num_days_from_ce_opt(days as i32)
        .and_then(|d| d.and_hms_opt(hours as u32, mins as u32, 0))
        .map(|t| Utc.from_utc_datetime(&t))
        .unwrap_or_else(Utc::now)
}

fn parse_tomorrow_phrases(lower: &str, now: DateTime<Utc>) -> Option<DateTime<Utc>> {
    let tomorrow_end = end_of_utc_day(now.date_naive() + Duration::days(1));
    if lower.contains("end of tomorrow")
        || lower.contains("by the end of tomorrow")
        || lower.contains("by end of tomorrow")
    {
        return Some(tomorrow_end);
    }
    if lower.contains("by tomorrow") || lower.contains("until tomorrow") {
        return Some(tomorrow_end);
    }
    if contains_word(lower, "tomorrow") {
        return Some(tomorrow_end);
    }
    None
}

fn parse_relative_duration(lower: &str, now: DateTime<Utc>) -> Option<DateTime<Utc>> {
    // Longer unit spellings first so "seconds"/"minutes" win over "second"/"min".
    for (unit, secs) in [
        ("seconds", 1),
        ("second", 1),
        ("secs", 1),
        ("sec", 1),
        ("minutes", 60),
        ("minute", 60),
        ("mins", 60),
        ("min", 60),
        ("hours", 3600),
        ("hour", 3600),
        ("hrs", 3600),
        ("hr", 3600),
        ("days", 86400),
        ("day", 86400),
    ] {
        let needle = format!("in ");
        if let Some(idx) = lower.find(&needle) {
            let rest = &lower[idx + needle.len()..];
            let num_str: String = rest.chars().take_while(|c| c.is_ascii_digit()).collect();
            if num_str.is_empty() {
                continue;
            }
            if let Ok(n) = num_str.parse::<i64>() {
                if n <= 0 {
                    continue;
                }
                let after_num = rest[num_str.len()..].trim_start();
                if after_num.starts_with(unit) {
                    return Some(now + Duration::seconds(n * secs));
                }
            }
        }
    }
    None
}

fn parse_iso_date(lower: &str) -> Option<NaiveDate> {
    for token in lower.split(|c: char| !c.is_ascii_digit() && c != '-') {
        if token.len() == 10 && token.as_bytes()[4] == b'-' && token.as_bytes()[7] == b'-' {
            if let Ok(d) = NaiveDate::parse_from_str(token, "%Y-%m-%d") {
                return Some(d);
            }
        }
    }
    None
}

fn parse_named_date(lower: &str) -> Option<NaiveDate> {
    const MONTHS: [(&str, u32); 12] = [
        ("january", 1),
        ("february", 2),
        ("march", 3),
        ("april", 4),
        ("may", 5),
        ("june", 6),
        ("july", 7),
        ("august", 8),
        ("september", 9),
        ("october", 10),
        ("november", 11),
        ("december", 12),
    ];
    for (name, month) in MONTHS {
        if let Some(idx) = lower.find(name) {
            let rest = &lower[idx + name.len()..];
            let day_year: String = rest
                .chars()
                .skip_while(|c| !c.is_ascii_digit())
                .take_while(|c| c.is_ascii_digit() || *c == ',')
                .collect();
            let parts: Vec<&str> = day_year
                .split(|c: char| !c.is_ascii_digit())
                .filter(|s| !s.is_empty())
                .collect();
            if parts.len() >= 2 {
                if let (Ok(day), Ok(year)) = (parts[0].parse::<u32>(), parts[1].parse::<i32>()) {
                    if let Some(d) = NaiveDate::from_ymd_opt(year, month, day) {
                        return Some(d);
                    }
                }
            }
        }
    }
    None
}

fn end_of_utc_day(date: NaiveDate) -> DateTime<Utc> {
    date.and_hms_milli_opt(23, 59, 59, 999)
        .map(|t| Utc.from_utc_datetime(&t))
        .unwrap_or_else(|| Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap()))
}

fn contains_word(haystack: &str, word: &str) -> bool {
    haystack.split_whitespace().any(|w| {
        w.trim_matches(|c: char| !c.is_alphanumeric())
            .eq_ignore_ascii_case(word)
    }) || haystack.contains(word)
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;
    use crate::events::registry::EventRegistry;
    use crate::events::types::EventEntity;
    use crate::store::events::ScheduledEventRow;
    use uuid::Uuid;

    fn test_registry() -> EventRegistry {
        EventRegistry::from_rows(vec![ScheduledEventRow {
            id: Uuid::new_v4(),
            provider_key: "localnet-seed".to_string(),
            external_id: "fifa_world_cup_2026".to_string(),
            label: "FIFA World Cup 2026".to_string(),
            category: "sports".to_string(),
            start_at_ms: None,
            end_at_ms: NaiveDate::from_ymd_opt(2026, 7, 19)
                .unwrap()
                .and_hms_milli_opt(23, 59, 59, 999)
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
        }])
    }

    #[test]
    fn parses_end_of_tomorrow() {
        let claim = "I think ETH will be trading above $1700 per token by the end of tomorrow.";
        let dt = parse_deadline_from_text(claim).expect("deadline");
        let now = Utc::now();
        let tomorrow = now.date_naive() + Duration::days(1);
        assert_eq!(dt.date_naive(), tomorrow);
        assert_eq!(dt.time().hour(), 23);
    }

    #[test]
    fn parses_in_minutes() {
        let before = Utc::now();
        let dt = parse_deadline_from_text("Will BTC trade above $100 in 3 minutes?").expect("deadline");
        let after = Utc::now();
        assert!(dt >= before + Duration::minutes(2));
        assert!(dt <= after + Duration::minutes(4));
    }

    #[test]
    fn parses_in_seconds() {
        let before = Utc::now();
        let dt = parse_deadline_from_text("Will BTC trade above $1 in 15 seconds?").expect("deadline");
        let after = Utc::now();
        assert!(dt >= before + Duration::seconds(10));
        assert!(dt <= after + Duration::seconds(20));
    }

    #[test]
    fn missing_deadline_returns_none() {
        assert!(parse_deadline_from_text("ETH will be above $1700").is_none());
    }

    #[test]
    fn iso_date_end_of_day() {
        let dt = parse_deadline_from_text("Will ETH exceed $2000 by 2027-07-31?").expect("deadline");
        assert_eq!(dt.date_naive(), NaiveDate::from_ymd_opt(2027, 7, 31).unwrap());
        assert_eq!(dt.time().hour(), 23);
    }

    #[test]
    fn price_ongoing_default_deadline() {
        let reg = EventRegistry::new();
        let dt = resolve_claim_deadline("BTC above $100", ClaimCategory::PriceThreshold, &reg)
            .expect("ongoing price deadline");
        assert!(dt > Utc::now() + Duration::minutes(4));
    }

    #[test]
    fn event_deadline_for_fifa_context() {
        let reg = test_registry();
        let dt = resolve_claim_deadline(
            "Messy will have more goals than the Muppet.",
            ClaimCategory::EventOccurrence,
            &reg,
        )
        .expect("fifa deadline");
        assert_eq!(dt.date_naive(), NaiveDate::from_ymd_opt(2026, 7, 19).unwrap());
    }

    #[test]
    fn price_deadlines_in_same_bucket_share_key() {
        let spacing = Duration::minutes(30);
        let base = Utc::now() + Duration::minutes(10);
        let a = base + Duration::minutes(3);
        let b = base + Duration::minutes(8);
        assert_eq!(
            bucket_deadline_for_market_key(a, spacing),
            bucket_deadline_for_market_key(b, spacing)
        );
    }
}
