// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! HTTP client for OpenRouter model pricing (catalog sync only — not inference).

use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct OpenRouterModelRate {
    pub id: String,
    pub input_usd_per_1m: f64,
    pub output_usd_per_1m: f64,
}

#[derive(Debug, Clone)]
pub struct OpenRouterClient {
    api_url: String,
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

impl OpenRouterClient {
    pub fn new(api_url: impl Into<String>, api_key: impl Into<String>) -> Self {
        Self {
            api_url: api_url.into().trim_end_matches('/').to_string(),
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .expect("reqwest client"),
            api_key: api_key.into(),
        }
    }

    pub async fn fetch_model_rates(&self) -> Result<HashMap<String, OpenRouterModelRate>> {
        let response = self
            .http
            .get(&self.api_url)
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
    anyhow::ensure!(raw.is_finite() && raw >= 0.0, "pricing must be non-negative finite");
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
}
