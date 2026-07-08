// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use anyhow::Context;
use serde::Deserialize;

use crate::store::reviews::ExtractedClaim;

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
    ) -> anyhow::Result<(ExtractedClaim, String)> {
        let system = r#"You extract structured claim fields from social posts for prediction markets.
Return JSON only with keys: subject, predicate, object, metric (optional), comparison (optional: lt|lte|gt|gte|eq|neq),
threshold (optional decimal string), deadline (optional ISO-8601 UTC), outcome_type (binary|multi_choice|scalar),
suggested_sources (string array), suggested_options (string array, 2-10 unique labels).
Do NOT include approve/reject/resolution fields."#;
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
        let claim: ExtractedClaim = serde_json::from_str(content_json)
            .context("parse extracted claim JSON")?;
        Ok((claim, raw))
    }
}

/// Deterministic heuristic extraction when no LLM API key is configured (dev/test).
pub fn extract_claim_heuristic(content: &str) -> ExtractedClaim {
    let lower = content.to_lowercase();
    let comparison = if lower.contains("above") || lower.contains("exceed") || lower.contains("over") {
        Some(crate::types::ComparisonOp::Gt)
    } else if lower.contains("below") || lower.contains("under") {
        Some(crate::types::ComparisonOp::Lt)
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
    ExtractedClaim {
        subject,
        predicate: "price".to_string(),
        object: "usd".to_string(),
        metric: Some("price".to_string()),
        comparison,
        threshold,
        deadline: None,
        outcome_type: crate::review::canonicalize::OutcomeType::Binary,
        suggested_sources: vec!["coingecko".to_string()],
        suggested_options: vec!["Yes".to_string(), "No".to_string()],
    }
}

fn extract_threshold(lower: &str) -> Option<String> {
    for token in lower.split_whitespace() {
        let cleaned = token.trim_matches(|c: char| !c.is_ascii_digit() && c != '.' && c != ',');
        if cleaned.contains('$') || cleaned.ends_with('k') {
            let num = cleaned
                .trim_start_matches('$')
                .trim_end_matches('k')
                .replace(',', "");
            if let Ok(mut v) = num.parse::<f64>() {
                if token.contains('k') {
                    v *= 1000.0;
                }
                return Some(format!("{v}"));
            }
        }
    }
    if lower.contains("$1") || lower.contains("above $1") {
        return Some("1".to_string());
    }
    None
}
