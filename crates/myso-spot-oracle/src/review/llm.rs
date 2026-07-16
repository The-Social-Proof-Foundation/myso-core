// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use anyhow::Context;
use serde::Deserialize;

use crate::events::registry::EventRegistry;
use crate::review::context_deadline::{apply_deadline_resolution, resolve_context_deadline};
use crate::review::deadline::resolve_claim_deadline;
use crate::store::reviews::ExtractedClaim;
use crate::types::{ClaimCategory, ComparisonOp, ResolverHints};

#[derive(Debug, Clone)]
pub struct LlmClient {
    api_url: String,
    api_key: String,
    model: String,
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    #[serde(default)]
    choices: Vec<ChatChoice>,
}

#[derive(Debug, Deserialize)]
struct ChatChoice {
    #[serde(default)]
    message: Option<ChatChoiceMessage>,
}

#[derive(Debug, Deserialize)]
struct ChatChoiceMessage {
    #[serde(default)]
    content: Option<String>,
}

impl LlmClient {
    pub fn new(api_url: String, api_key: String, model: String) -> Self {
        Self {
            api_url,
            api_key,
            model,
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .expect("reqwest client"),
        }
    }

    pub async fn extract_claim(
        &self,
        content: &str,
        post_type: Option<&str>,
        registry: &EventRegistry,
    ) -> anyhow::Result<(ExtractedClaim, String)> {
        let system = r#"You extract structured claim fields from social posts for prediction markets.
Return JSON only with keys:
  subject, predicate, object, metric (optional),
  comparison (optional: lt|lte|gt|gte|eq|neq),
  threshold (optional decimal string),
  deadline (required ISO-8601 UTC when the claim must be evaluated; omit only if a known public event end date applies — e.g. FIFA World Cup, U.S. presidential election — and set feed_url + match_predicate instead),
  outcome_type (binary|multi_choice|scalar),
  suggested_sources (string array),
  suggested_options (string array, 2-10 unique labels),
  claim_category (price_threshold|release_published|event_occurrence|custom_http|unsupported),
  time_class (future|past|unsupported): future = resolves after now (opens a betting market);
    past = asserts something that already happened and is objectively checkable against public
    sources (verify it, do NOT open a market); unsupported = not objectively checkable.
  Do NOT invent event identities or deadlines — leave deadline empty when uncertain so the
  registry matcher can ground the claim from scheduled events (elections, sports, earnings, etc.).
  Use claim_category=event_occurrence for sports outcomes (World Cup, Super Bowl) and elections.
  resolver_hints (object with optional fields):
    owner, repo, tag_predicate,
    feed_url, match_predicate, match_fields (string array),
    url, json_path, comparison, expected,
    preferred_sources (string array).
Do NOT include approve/reject/resolution fields.
If the claim is not objectively verifiable from public data, set claim_category to unsupported."#;
        let user = format!(
            "Post type: {}\nContent:\n{}",
            post_type.unwrap_or("text"),
            content
        );
        let body = serde_json::json!({
            "model": self.model,
            "messages": [
                {"role": "system", "content": system},
                {"role": "user", "content": user}
            ],
            "response_format": {"type": "json_object"},
            "max_tokens": 1024
        });
        let resp = self
            .http
            .post(&self.api_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&body)
            .send()
            .await
            .context("openrouter request")?;
        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            anyhow::bail!("openrouter status {status}: {text}");
        }
        let raw = resp.text().await?;
        let parsed_resp: ChatCompletionResponse = serde_json::from_str(&raw)?;
        let content_json = parsed_resp
            .choices
            .first()
            .and_then(|c| c.message.as_ref())
            .and_then(|m| m.content.as_ref())
            .context("empty LLM response")?;
        let mut claim: ExtractedClaim =
            serde_json::from_str(content_json).context("parse extracted claim JSON")?;
        if claim.deadline.is_none() {
            if let Some(resolution) =
                resolve_context_deadline(content, claim.claim_category, registry)
            {
                apply_deadline_resolution(&mut claim, &resolution);
            } else {
                claim.deadline = resolve_claim_deadline(content, claim.claim_category, registry);
            }
        }
        // Unsupported category implies unsupported time class; otherwise trust the model but fall
        // back to the tense heuristic when it left the default.
        if claim.claim_category == crate::types::ClaimCategory::Unsupported {
            claim.time_class = crate::types::TimeClass::Unsupported;
        } else if claim.time_class == crate::types::TimeClass::Future {
            claim.time_class = classify_time_class(content);
        }
        Ok((claim, raw))
    }

    pub async fn extract_and_match(
        &self,
        content: &str,
        post_type: Option<&str>,
        registry: &EventRegistry,
    ) -> anyhow::Result<(ExtractedClaim, String)> {
        let (mut claim, raw) = self.extract_claim(content, post_type, registry).await?;
        let _ = crate::review::claim_matcher::match_and_reconcile(content, &mut claim, registry);
        Ok((claim, raw))
    }
}

/// Deterministic heuristic extraction when no LLM API key is configured (dev/test).
pub fn extract_claim_heuristic(content: &str, registry: &EventRegistry) -> ExtractedClaim {
    if let Some(sports) = extract_sports_comparison(content, registry) {
        return sports;
    }
    if let Some(outcome) = extract_outcome_claim(content, registry) {
        return outcome;
    }
    if let Some(price) = extract_price_claim_if_price_cues(content, registry) {
        return price;
    }
    extract_unsupported_claim(content)
}

/// Heuristic time-class classification: explicit future intent wins; else past-result cues
/// mark a claim as `past`. Used by the heuristic extractor and as an LLM fallback.
pub fn classify_time_class(content: &str) -> crate::types::TimeClass {
    use crate::types::TimeClass;
    let lower = content.to_lowercase();
    if lower.contains("will ")
        || lower.contains("by tomorrow")
        || lower.contains("next ")
        || lower.contains("going to ")
        || lower.contains("by the end of")
        || lower.contains("by 20")
    {
        return TimeClass::Future;
    }
    const PAST_CUES: [&str; 11] = [
        " won ",
        " won.",
        " was ",
        " were ",
        " did ",
        " beat ",
        " lost ",
        " finished ",
        " happened",
        " already ",
        " back in ",
    ];
    if PAST_CUES.iter().any(|c| lower.contains(c)) {
        return TimeClass::Past;
    }
    TimeClass::Future
}

fn extract_sports_comparison(content: &str, registry: &EventRegistry) -> Option<ExtractedClaim> {
    let lower = content.to_lowercase();
    let matched = registry.match_event(content);
    let is_sports = lower.contains("goal")
        || lower.contains("score")
        || lower.contains("more than")
        || matched.is_some();
    if !is_sports {
        return None;
    }

    let ev = matched?;
    let category = ClaimCategory::EventOccurrence;
    let (subject, object) = parse_comparison_entities(&lower, registry, Some(&ev))?;

    Some(ExtractedClaim {
        subject,
        predicate: if lower.contains("goal") {
            "goals".to_string()
        } else {
            "outcome".to_string()
        },
        object,
        metric: None,
        comparison: Some(ComparisonOp::Gt),
        threshold: None,
        deadline: resolve_claim_deadline(content, category, registry),
        outcome_type: crate::review::canonicalize::OutcomeType::Binary,
        suggested_sources: vec!["rss_event".to_string()],
        suggested_options: vec!["Yes".to_string(), "No".to_string()],
        claim_category: category,
        time_class: classify_time_class(content),
        resolver_hints: ResolverHints {
            feed_url: ev.feed_url.clone(),
            match_predicate: ev.match_predicate.clone(),
            preferred_sources: if ev.preferred_source_keys.is_empty() {
                vec!["rss_event".to_string()]
            } else {
                ev.preferred_source_keys.clone()
            },
            ..Default::default()
        },
    })
}

fn extract_outcome_claim(content: &str, registry: &EventRegistry) -> Option<ExtractedClaim> {
    let lower = content.to_lowercase();
    let matched = registry.match_event(content);

    let is_outcome = lower.contains("will win")
        || lower.contains("will lose")
        || lower.contains("will be elected")
        || lower.contains("will become president")
        || lower.contains("will become the president")
        || matched.is_some();

    if !is_outcome {
        return None;
    }

    let category = ClaimCategory::EventOccurrence;
    let (subject, object, predicate) = parse_outcome_entities(&lower, registry, matched.as_ref())?;

    let mut claim = ExtractedClaim {
        subject,
        predicate,
        object,
        metric: None,
        comparison: None,
        threshold: None,
        deadline: None,
        outcome_type: crate::review::canonicalize::OutcomeType::Binary,
        suggested_sources: vec!["rss_event".to_string()],
        suggested_options: vec!["Yes".to_string(), "No".to_string()],
        claim_category: category,
        time_class: classify_time_class(content),
        resolver_hints: ResolverHints::default(),
    };

    if let Some(resolution) = resolve_context_deadline(content, category, registry) {
        apply_deadline_resolution(&mut claim, &resolution);
        if let Some(ref ev) = resolution.matched_event {
            claim.resolver_hints.feed_url = ev.feed_url.clone();
            claim.resolver_hints.match_predicate = ev.match_predicate.clone();
            if !ev.preferred_source_keys.is_empty() {
                claim.resolver_hints.preferred_sources = ev.preferred_source_keys.clone();
            }
        }
    } else {
        claim.deadline = resolve_claim_deadline(content, category, registry);
    }

    if claim.resolver_hints.match_predicate.is_none() {
        claim.resolver_hints.match_predicate = Some(claim.subject.clone());
    }

    Some(claim)
}

fn parse_outcome_entities(
    lower: &str,
    registry: &EventRegistry,
    matched: Option<&crate::events::registry::ScheduledEventRecord>,
) -> Option<(String, String, String)> {
    if let Some(win_idx) = lower.find(" will win ") {
        let subject_raw = lower[..win_idx].trim();
        let subject = normalize_outcome_subject(subject_raw, registry, matched);
        let object = if lower.contains("election") || lower.contains("president") {
            "election".to_string()
        } else if lower.contains("world cup") || lower.contains("tournament") {
            "tournament".to_string()
        } else {
            "event".to_string()
        };
        return Some((subject, object, "win".to_string()));
    }
    if let Some(elect_idx) = lower.find(" will be elected") {
        let subject_raw = lower[..elect_idx].trim();
        let subject = normalize_outcome_subject(subject_raw, registry, matched);
        return Some((subject, "election".to_string(), "elected".to_string()));
    }
    if let Some(pres_idx) = lower
        .find(" will become president")
        .or_else(|| lower.find(" will become the president"))
    {
        let subject_raw = lower[..pres_idx].trim();
        let subject = normalize_outcome_subject(subject_raw, registry, matched);
        return Some((subject, "presidency".to_string(), "win".to_string()));
    }
    if let Some(ev) = matched {
        let subject = ev
            .entities
            .first()
            .map(|e| e.name.clone())
            .unwrap_or_else(|| "outcome".to_string());
        return Some((subject, ev.label.clone(), "outcome".to_string()));
    }
    None
}

fn normalize_outcome_subject(
    raw: &str,
    registry: &EventRegistry,
    matched: Option<&crate::events::registry::ScheduledEventRecord>,
) -> String {
    let trimmed = raw.trim();
    if let Some(ev) = matched {
        for entity in &ev.entities {
            let name_lower = entity.name.to_lowercase();
            if trimmed.contains(&name_lower) {
                return entity.name.clone();
            }
            for alias in &entity.aliases {
                if trimmed.contains(&alias.to_lowercase()) {
                    return entity.name.clone();
                }
            }
        }
    }
    registry.normalize_entity(trimmed, matched)
}

fn has_price_cues(lower: &str) -> bool {
    lower.contains('$')
        || contains_word(lower, "btc")
        || contains_word(lower, "bitcoin")
        || contains_word(lower, "eth")
        || contains_word(lower, "ethereum")
        || contains_word(lower, "price")
        || lower.contains("trade above")
        || lower.contains("trading above")
        || lower.contains(" per token")
}

fn contains_word(haystack: &str, word: &str) -> bool {
    haystack
        .split(|c: char| !c.is_alphanumeric())
        .any(|w| w.eq_ignore_ascii_case(word))
}

fn extract_price_claim_if_price_cues(
    content: &str,
    registry: &EventRegistry,
) -> Option<ExtractedClaim> {
    let lower = content.to_lowercase();
    if !has_price_cues(&lower) {
        return None;
    }
    Some(extract_price_claim(content, registry))
}

fn extract_unsupported_claim(content: &str) -> ExtractedClaim {
    ExtractedClaim {
        subject: String::new(),
        predicate: String::new(),
        object: String::new(),
        metric: None,
        comparison: None,
        threshold: None,
        deadline: None,
        outcome_type: crate::review::canonicalize::OutcomeType::Binary,
        suggested_sources: vec![],
        suggested_options: vec!["Yes".to_string(), "No".to_string()],
        claim_category: ClaimCategory::Unsupported,
        time_class: classify_time_class(content),
        resolver_hints: ResolverHints::default(),
    }
}

fn parse_comparison_entities(
    lower: &str,
    registry: &EventRegistry,
    matched: Option<&crate::events::registry::ScheduledEventRecord>,
) -> Option<(String, String)> {
    let than_idx = lower.rfind(" than ")?;
    let left = lower[..than_idx].trim();
    let right = lower[than_idx + 6..]
        .trim()
        .trim_end_matches('.')
        .trim_start_matches("the ")
        .trim();

    let subject_raw = left.split(" will ").next()?.trim();
    let subject = registry.normalize_entity(subject_raw, matched);
    let object = registry.normalize_entity(right, matched);
    if subject.is_empty() || object.is_empty() {
        return None;
    }
    Some((subject, object))
}

fn extract_price_claim(content: &str, registry: &EventRegistry) -> ExtractedClaim {
    let lower = content.to_lowercase();
    let comparison =
        if lower.contains("above") || lower.contains("exceed") || lower.contains("over") {
            Some(ComparisonOp::Gt)
        } else if lower.contains("below") || lower.contains("under") {
            Some(ComparisonOp::Lt)
        } else {
            None
        };
    let threshold = extract_threshold(&lower);
    let subject = if lower.contains("btc") || lower.contains("bitcoin") {
        "bitcoin".to_string()
    } else if lower.contains("eth") || lower.contains("ethereum") {
        "ethereum".to_string()
    } else {
        "unknown".to_string()
    };
    let category = ClaimCategory::PriceThreshold;
    ExtractedClaim {
        subject,
        predicate: "price".to_string(),
        object: "usd".to_string(),
        metric: Some("price".to_string()),
        comparison,
        threshold,
        deadline: resolve_claim_deadline(content, category, registry),
        outcome_type: crate::review::canonicalize::OutcomeType::Binary,
        suggested_sources: vec!["coingecko".to_string()],
        suggested_options: vec!["Yes".to_string(), "No".to_string()],
        claim_category: category,
        time_class: classify_time_class(content),
        resolver_hints: ResolverHints {
            preferred_sources: vec!["coingecko".to_string()],
            ..Default::default()
        },
    }
}

fn extract_threshold(lower: &str) -> Option<String> {
    if let Some(idx) = lower.find('$') {
        let rest = &lower[idx + 1..];
        let num: String = rest
            .chars()
            .take_while(|c| c.is_ascii_digit() || *c == '.' || *c == ',')
            .collect();
        let num = num.replace(',', "");
        if !num.is_empty() {
            if let Ok(v) = num.parse::<f64>() {
                if v.fract() == 0.0 {
                    return Some(format!("{}", v as i64));
                }
                return Some(format!("{v}"));
            }
        }
    }
    for token in lower.split_whitespace() {
        if token.ends_with('k') {
            let num = token.trim_end_matches('k').replace(',', "");
            if let Ok(mut v) = num.parse::<f64>() {
                v *= 1000.0;
                if v.fract() == 0.0 {
                    return Some(format!("{}", v as i64));
                }
                return Some(format!("{v}"));
            }
        }
    }
    None
}

#[cfg(test)]
mod threshold_tests {
    use super::extract_claim_heuristic;
    use crate::events::registry::EventRegistry;
    use crate::events::types::EventEntity;
    use crate::review::deadline::parse_deadline_from_text;
    use crate::store::events::ScheduledEventRow;
    use crate::types::ClaimCategory;
    use uuid::Uuid;

    fn test_registry() -> EventRegistry {
        EventRegistry::from_rows(vec![ScheduledEventRow {
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
        }])
    }

    #[test]
    fn dollar_amounts_are_parsed_distinctly() {
        let reg = test_registry();
        let a = extract_claim_heuristic("Will ETH trade above $100 by tomorrow?", &reg);
        let b = extract_claim_heuristic("Will ETH trade above $1700 by tomorrow?", &reg);
        assert_eq!(a.threshold.as_deref(), Some("100"));
        assert_eq!(b.threshold.as_deref(), Some("1700"));
        assert!(a.deadline.is_some());
        assert!(b.deadline.is_some());
    }

    #[test]
    fn eth_1700_tomorrow_phrase_has_deadline() {
        let reg = test_registry();
        let claim = extract_claim_heuristic(
            "I think ETH will be trading above $1700 per token by the end of tomorrow.",
            &reg,
        );
        assert_eq!(claim.threshold.as_deref(), Some("1700"));
        assert!(parse_deadline_from_text(
            "I think ETH will be trading above $1700 per token by the end of tomorrow."
        )
        .is_some());
        assert!(claim.deadline.is_some());
    }

    #[test]
    fn ongoing_price_gets_spacing_deadline() {
        let reg = test_registry();
        let claim = extract_claim_heuristic("ETH will be above $1700", &reg);
        assert_eq!(claim.claim_category, ClaimCategory::PriceThreshold);
        assert!(claim.deadline.is_some());
    }

    #[test]
    fn messi_goals_claim_infers_fifa() {
        let reg = test_registry();
        let claim = extract_claim_heuristic("Messy will have more goals than the Muppet.", &reg);
        assert_eq!(claim.claim_category, ClaimCategory::EventOccurrence);
        assert_eq!(claim.subject, "lionel messi");
        assert_eq!(claim.object, "kylian mbappe");
        assert!(claim.deadline.is_some());
        assert!(claim.resolver_hints.feed_url.is_some());
    }

    fn election_registry() -> EventRegistry {
        EventRegistry::from_rows(vec![ScheduledEventRow {
            id: Uuid::new_v4(),
            provider_key: "localnet-seed".to_string(),
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
            keywords: vec![
                "presidential election".to_string(),
                "presedintial election".to_string(),
                "2028".to_string(),
            ],
            entities: serde_json::to_value(vec![EventEntity {
                name: "jd vance".to_string(),
                aliases: vec!["vance".to_string()],
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
    fn vance_2028_election_outcome_claim() {
        let reg = election_registry();
        let claim =
            extract_claim_heuristic("JD Vance will win the 2028 presedintial election.", &reg);
        assert_eq!(claim.claim_category, ClaimCategory::EventOccurrence);
        assert_eq!(claim.subject, "jd vance");
        assert!(claim.deadline.is_some());
        assert_eq!(
            claim.deadline.unwrap().date_naive(),
            chrono::NaiveDate::from_ymd_opt(2028, 11, 7).unwrap()
        );
        assert!(claim.resolver_hints.deadline_provenance.is_some());
    }

    #[test]
    fn vague_claim_is_unsupported_not_price() {
        let reg = test_registry();
        let claim = extract_claim_heuristic("Something vague will happen someday.", &reg);
        assert_eq!(claim.claim_category, ClaimCategory::Unsupported);
    }
}
