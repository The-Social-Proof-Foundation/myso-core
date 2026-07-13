// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! HTTP client for OpenRouter model pricing (catalog sync) and chat completions (inference proxy).

use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct OpenRouterModelRate {
    pub id: String,
    pub input_usd_per_1m: f64,
    pub output_usd_per_1m: f64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatMessage<'a> {
    pub role: &'a str,
    pub content: &'a str,
}

#[derive(Debug, Clone)]
pub struct ChatCompletionResult {
    pub content: String,
    pub prompt_tokens: u64,
    pub completion_tokens: u64,
    pub total_tokens: u64,
    /// Actual amount charged to the OpenRouter account, expressed in integer
    /// USD micros and rounded up.  This is the billing authority; token counts
    /// are retained for reservation estimates and audit only.
    pub provider_cost_usd_micros: u64,
    /// Upstream provider cost, when OpenRouter reports it.  This is useful for
    /// margin/reconciliation reporting but is not the customer billing basis.
    pub upstream_cost_usd_micros: Option<u64>,
    /// OpenRouter generation identifier used for durable reconciliation.
    pub generation_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct OpenRouterClient {
    models_url: String,
    chat_url: String,
    api_key: String,
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct ModelsResponse {
    #[serde(default)]
    data: Vec<ModelEntry>,
}

#[derive(Debug, Deserialize)]
struct ModelEntry {
    id: String,
    #[serde(default)]
    pricing: Option<PricingEntry>,
}

#[derive(Debug, Deserialize)]
struct PricingEntry {
    #[serde(default)]
    prompt: Option<serde_json::Value>,
    #[serde(default)]
    completion: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct ChatCompletionRequest<'a> {
    model: &'a str,
    messages: Vec<ChatMessage<'a>>,
    max_tokens: u32,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionResponse {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    choices: Vec<ChatChoice>,
    #[serde(default)]
    usage: Option<ChatUsage>,
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

#[derive(Debug, Deserialize)]
struct ChatUsage {
    #[serde(default)]
    prompt_tokens: u64,
    #[serde(default)]
    completion_tokens: u64,
    #[serde(default)]
    total_tokens: u64,
    #[serde(default)]
    cost: Option<serde_json::Value>,
    #[serde(default)]
    cost_details: Option<ChatCostDetails>,
}

#[derive(Debug, Deserialize)]
struct ChatCostDetails {
    #[serde(default)]
    upstream_inference_cost: Option<serde_json::Value>,
}

impl OpenRouterClient {
    pub fn new(
        models_url: impl Into<String>,
        chat_url: impl Into<String>,
        api_key: impl Into<String>,
    ) -> Self {
        Self {
            models_url: models_url.into().trim_end_matches('/').to_string(),
            chat_url: chat_url.into().trim_end_matches('/').to_string(),
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(120))
                .build()
                .expect("reqwest client"),
            api_key: api_key.into(),
        }
    }

    pub async fn fetch_model_rates(&self) -> Result<HashMap<String, OpenRouterModelRate>> {
        let response = self
            .http
            .get(&self.models_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .context("openrouter models request")?
            .error_for_status()
            .context("openrouter models status")?
            .json::<ModelsResponse>()
            .await
            .context("openrouter models json")?;

        Ok(parse_model_rates(response.data))
    }

    pub async fn chat_completions(
        &self,
        model: &str,
        messages: &[ChatMessage<'_>],
        max_tokens: u32,
    ) -> Result<ChatCompletionResult> {
        let body = ChatCompletionRequest {
            model,
            messages: messages.to_vec(),
            max_tokens,
        };
        let response = self
            .http
            .post(&self.chat_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("openrouter chat completions request")?
            .error_for_status()
            .context("openrouter chat completions status")?
            .json::<ChatCompletionResponse>()
            .await
            .context("openrouter chat completions json")?;

        parse_chat_completion(response)
    }
}

fn parse_chat_completion(response: ChatCompletionResponse) -> Result<ChatCompletionResult> {
    let content = response
        .choices
        .first()
        .and_then(|c| c.message.as_ref())
        .and_then(|m| m.content.clone())
        .unwrap_or_default();
    let usage = response
        .usage
        .context("openrouter response missing usage")?;
    anyhow::ensure!(
        usage.prompt_tokens > 0 || usage.completion_tokens > 0 || usage.total_tokens > 0,
        "openrouter usage tokens are zero"
    );
    let provider_cost_usd_micros = parse_usd_micros(
        usage
            .cost
            .as_ref()
            .context("openrouter response missing usage.cost")?,
    )?;
    let upstream_cost_usd_micros = usage
        .cost_details
        .as_ref()
        .and_then(|d| d.upstream_inference_cost.as_ref())
        .map(parse_usd_micros)
        .transpose()?;
    Ok(ChatCompletionResult {
        content,
        prompt_tokens: usage.prompt_tokens,
        completion_tokens: usage.completion_tokens,
        total_tokens: usage.total_tokens,
        provider_cost_usd_micros,
        upstream_cost_usd_micros,
        generation_id: response.id,
    })
}

/// Convert an OpenRouter USD value into integer micros without using floating
/// point for settlement math. Values finer than one micro are rounded up so a
/// non-zero provider charge can never become a zero customer charge.
fn parse_usd_micros(value: &serde_json::Value) -> Result<u64> {
    let raw = match value {
        serde_json::Value::String(s) => s.clone(),
        serde_json::Value::Number(n) => n.to_string(),
        other => anyhow::bail!("unexpected cost type: {other}"),
    };
    anyhow::ensure!(!raw.starts_with('-'), "cost must be non-negative");

    let (mantissa, exponent) = match raw.split_once(|c| c == 'e' || c == 'E') {
        Some((m, e)) => (m, e.parse::<i32>().context("invalid cost exponent")?),
        None => (raw.as_str(), 0),
    };
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = format!("{}{}", if whole.is_empty() { "0" } else { whole }, fraction);
    let integer = digits.parse::<u128>().context("invalid cost number")?;
    let decimal_places = fraction.len() as i32;
    let scale = 6 + exponent - decimal_places;
    let micros = if scale >= 0 {
        integer
            .checked_mul(10u128.pow(scale as u32))
            .context("cost overflow")?
    } else {
        let divisor = 10u128.pow((-scale) as u32);
        integer.checked_add(divisor - 1).context("cost overflow")? / divisor
    };
    u64::try_from(micros).context("cost exceeds u64 micros")
}

fn parse_model_rates(entries: Vec<ModelEntry>) -> HashMap<String, OpenRouterModelRate> {
    let mut out = HashMap::new();
    for entry in entries {
        let Some(pricing) = entry.pricing else {
            continue;
        };
        let Ok(prompt_usd) = parse_usd_per_token(pricing.prompt.as_ref()) else {
            continue;
        };
        let Ok(completion_usd) = parse_usd_per_token(pricing.completion.as_ref()) else {
            continue;
        };
        let rate = OpenRouterModelRate {
            id: entry.id.clone(),
            input_usd_per_1m: prompt_usd * 1_000_000.0,
            output_usd_per_1m: completion_usd * 1_000_000.0,
        };
        out.insert(entry.id.to_lowercase(), rate);
    }
    out
}

fn parse_usd_per_token(value: Option<&serde_json::Value>) -> Result<f64> {
    let value = value.context("missing pricing field")?;
    let raw = match value {
        serde_json::Value::String(s) => s
            .parse::<f64>()
            .with_context(|| format!("invalid pricing string: {s}"))?,
        serde_json::Value::Number(n) => n.as_f64().context("invalid pricing number")?,
        other => anyhow::bail!("unexpected pricing type: {other}"),
    };
    anyhow::ensure!(
        raw.is_finite() && raw >= 0.0,
        "pricing must be non-negative finite"
    );
    Ok(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_models_response_fixture() {
        let json = r#"{
            "data": [
                {
                    "id": "openai/gpt-4o-mini",
                    "pricing": { "prompt": "0.00000015", "completion": "0.0000006" }
                },
                {
                    "id": "no-pricing-model"
                }
            ]
        }"#;
        let body: ModelsResponse = serde_json::from_str(json).unwrap();
        let rates = parse_model_rates(body.data);
        let rate = rates.get("openai/gpt-4o-mini").expect("model present");
        assert!((rate.input_usd_per_1m - 0.15).abs() < f64::EPSILON);
        assert!((rate.output_usd_per_1m - 0.6).abs() < f64::EPSILON);
        assert!(!rates.contains_key("no-pricing-model"));
    }

    #[test]
    fn parse_chat_completion_fixture() {
        let json = r#"{
            "id": "gen-test-1",
            "choices": [
                { "message": { "content": "hello" } }
            ],
            "usage": {
                "prompt_tokens": 12,
                "completion_tokens": 3,
                "total_tokens": 15,
                "cost": 0.0000012,
                "cost_details": { "upstream_inference_cost": "0.0000008" }
            }
        }"#;
        let body: ChatCompletionResponse = serde_json::from_str(json).unwrap();
        let result = parse_chat_completion(body).unwrap();
        assert_eq!(result.content, "hello");
        assert_eq!(result.prompt_tokens, 12);
        assert_eq!(result.completion_tokens, 3);
        assert_eq!(result.total_tokens, 15);
        assert_eq!(result.provider_cost_usd_micros, 2);
        assert_eq!(result.upstream_cost_usd_micros, Some(1));
        assert_eq!(result.generation_id.as_deref(), Some("gen-test-1"));
    }

    #[test]
    fn usd_micros_rounds_non_zero_cost_up() {
        assert_eq!(parse_usd_micros(&serde_json::json!(0)).unwrap(), 0);
        assert_eq!(
            parse_usd_micros(&serde_json::json!("0.00000001")).unwrap(),
            1
        );
        assert_eq!(
            parse_usd_micros(&serde_json::json!("1.234567")).unwrap(),
            1_234_567
        );
        assert_eq!(parse_usd_micros(&serde_json::json!("1e-7")).unwrap(), 1);
    }
}
