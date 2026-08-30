// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! HTTP client for OpenRouter model pricing (catalog sync) and chat completions (inference proxy).

use std::collections::HashMap;
use std::time::Duration;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;

const ASSISTANT_ENVELOPE_VERSION: &str = "myso_assistant_v1";

#[derive(Debug, Clone)]
pub struct OpenRouterModelRate {
    pub id: String,
    pub input_usd_per_1m: f64,
    pub output_usd_per_1m: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenRouterFunction {
    pub name: String,
    #[serde(default)]
    pub arguments: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenRouterToolCall {
    pub id: String,
    #[serde(default = "default_function_type", rename = "type")]
    pub kind: String,
    pub function: OpenRouterFunction,
}

fn default_function_type() -> String {
    "function".to_string()
}

impl OpenRouterToolCall {
    pub fn function(
        id: impl Into<String>,
        name: impl Into<String>,
        arguments: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            kind: default_function_type(),
            function: OpenRouterFunction {
                name: name.into(),
                arguments: arguments.into(),
            },
        }
    }

    pub fn to_tool_call(&self) -> ToolCall {
        ToolCall {
            id: self.id.clone(),
            name: self.function.name.clone(),
            arguments: self.function.arguments.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<OpenRouterToolCall>>,
}

impl ChatMessage {
    pub fn text(role: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            content: Some(content.into()),
            tool_call_id: None,
            tool_calls: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AssistantEnvelope {
    v: String,
    text: String,
    #[serde(default)]
    tool_calls: Vec<ToolCall>,
}

/// Persist text plus tool calls in the reservation `content` column without a
/// schema migration. Plain-text replies stay plain strings.
pub fn encode_assistant_content(text: &str, tool_calls: &[ToolCall]) -> String {
    if tool_calls.is_empty() {
        return text.to_string();
    }
    serde_json::to_string(&AssistantEnvelope {
        v: ASSISTANT_ENVELOPE_VERSION.to_string(),
        text: text.to_string(),
        tool_calls: tool_calls.to_vec(),
    })
    .unwrap_or_else(|_| text.to_string())
}

pub fn decode_assistant_content(content: &str) -> (String, Vec<ToolCall>) {
    match serde_json::from_str::<AssistantEnvelope>(content) {
        Ok(env) if env.v == ASSISTANT_ENVELOPE_VERSION => (env.text, env.tool_calls),
        _ => (content.to_string(), Vec::new()),
    }
}

#[derive(Debug, Clone)]
pub struct ChatCompletionResult {
    pub content: String,
    pub tool_calls: Vec<ToolCall>,
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
    messages: &'a [ChatMessage],
    max_tokens: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    tools: Option<&'a Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tool_choice: Option<&'a Value>,
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
    #[serde(default)]
    tool_calls: Vec<OpenRouterToolCall>,
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
        messages: &[ChatMessage],
        max_tokens: u32,
        tools: Option<&Value>,
        tool_choice: Option<&Value>,
    ) -> Result<ChatCompletionResult> {
        let body = ChatCompletionRequest {
            model,
            messages,
            max_tokens,
            tools,
            tool_choice,
        };
        let response = self
            .http
            .post(&self.chat_url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("openrouter chat completions request")?;
        let status = response.status();
        let bytes = response
            .bytes()
            .await
            .context("openrouter chat completions body")?;
        if !status.is_success() {
            let preview = String::from_utf8_lossy(&bytes);
            let preview = preview.chars().take(500).collect::<String>();
            anyhow::bail!("openrouter chat completions status {status}: {preview}");
        }
        let parsed = serde_json::from_slice::<ChatCompletionResponse>(&bytes)
            .context("openrouter chat completions json")?;
        parse_chat_completion(parsed)
    }
}

fn parse_chat_completion(response: ChatCompletionResponse) -> Result<ChatCompletionResult> {
    let message = response.choices.first().and_then(|c| c.message.as_ref());
    let content = message.and_then(|m| m.content.clone()).unwrap_or_default();
    let tool_calls = message
        .map(|m| {
            m.tool_calls
                .iter()
                .map(OpenRouterToolCall::to_tool_call)
                .collect()
        })
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
        tool_calls,
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

    #[tokio::test]
    async fn chat_completions_hits_mocked_openrouter() {
        use axum::routing::post;
        use axum::{Json, Router};
        use serde_json::json;

        async fn mock_chat(Json(body): Json<serde_json::Value>) -> Json<serde_json::Value> {
            assert_eq!(body["model"], "openai/gpt-4o-mini");
            assert_eq!(body["max_tokens"], 32);
            assert!(body.get("tools").is_none());
            Json(json!({
                "id": "gen-mock-1",
                "choices": [{ "message": { "content": "AI_CREDIT_OK" } }],
                "usage": {
                    "prompt_tokens": 5,
                    "completion_tokens": 3,
                    "total_tokens": 8,
                    "cost": 0.000002,
                    "cost_details": { "upstream_inference_cost": 0.000001 }
                }
            }))
        }

        let app = Router::new().route("/chat/completions", post(mock_chat));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind mock openrouter");
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });
        tokio::task::yield_now().await;

        let client = OpenRouterClient::new(
            format!("http://{addr}/models"),
            format!("http://{addr}/chat/completions"),
            "test-key",
        );
        let messages = [ChatMessage::text("user", "ping")];
        let result = client
            .chat_completions("openai/gpt-4o-mini", &messages, 32, None, None)
            .await
            .expect("mocked chat");
        assert_eq!(result.content, "AI_CREDIT_OK");
        assert!(result.tool_calls.is_empty());
        assert_eq!(result.prompt_tokens, 5);
        assert_eq!(result.completion_tokens, 3);
        assert_eq!(result.provider_cost_usd_micros, 2);
        assert_eq!(result.generation_id.as_deref(), Some("gen-mock-1"));
    }

    #[tokio::test]
    async fn chat_completions_forwards_tools_and_parses_tool_calls() {
        use axum::routing::post;
        use axum::{Json, Router};
        use serde_json::json;

        async fn mock_chat(Json(body): Json<serde_json::Value>) -> Json<serde_json::Value> {
            assert_eq!(body["tools"][0]["function"]["name"], "web_search");
            assert_eq!(body["messages"][1]["role"], "tool");
            assert_eq!(body["messages"][1]["tool_call_id"], "call_1");
            Json(json!({
                "id": "gen-tools-1",
                "choices": [{
                    "message": {
                        "content": null,
                        "tool_calls": [{
                            "id": "call_2",
                            "type": "function",
                            "function": {
                                "name": "web_search",
                                "arguments": "{\"query\":\"Cowboys\"}"
                            }
                        }]
                    }
                }],
                "usage": {
                    "prompt_tokens": 20,
                    "completion_tokens": 8,
                    "total_tokens": 28,
                    "cost": 0.000003
                }
            }))
        }

        let app = Router::new().route("/chat/completions", post(mock_chat));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind mock openrouter");
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });
        tokio::task::yield_now().await;

        let client = OpenRouterClient::new(
            format!("http://{addr}/models"),
            format!("http://{addr}/chat/completions"),
            "test-key",
        );
        let messages = [
            ChatMessage::text("user", "score?"),
            ChatMessage {
                role: "tool".into(),
                content: Some("Cowboys won".into()),
                tool_call_id: Some("call_1".into()),
                tool_calls: None,
            },
        ];
        let tools = json!([{
            "type": "function",
            "function": {
                "name": "web_search",
                "description": "Search the web",
                "parameters": { "type": "object", "properties": {} }
            }
        }]);
        let result = client
            .chat_completions("openai/gpt-4o-mini", &messages, 64, Some(&tools), None)
            .await
            .expect("mocked tool chat");
        assert_eq!(result.tool_calls.len(), 1);
        assert_eq!(result.tool_calls[0].id, "call_2");
        assert_eq!(result.tool_calls[0].name, "web_search");
        assert_eq!(result.tool_calls[0].arguments, "{\"query\":\"Cowboys\"}");
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
        assert!(result.tool_calls.is_empty());
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

    #[tokio::test]
    async fn chat_completions_includes_status_and_body_on_error() {
        use axum::http::StatusCode;
        use axum::routing::post;
        use axum::{Json, Router};
        use serde_json::json;

        async fn mock_reject(
            Json(_body): Json<serde_json::Value>,
        ) -> (StatusCode, Json<serde_json::Value>) {
            (
                StatusCode::BAD_REQUEST,
                Json(json!({ "error": { "message": "bad tools" } })),
            )
        }

        let app = Router::new().route("/chat/completions", post(mock_reject));
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind mock openrouter");
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            axum::serve(listener, app).await.ok();
        });
        tokio::task::yield_now().await;

        let client = OpenRouterClient::new(
            format!("http://{addr}/models"),
            format!("http://{addr}/chat/completions"),
            "test-key",
        );
        let messages = [ChatMessage::text("user", "ping")];
        let error = client
            .chat_completions("openai/gpt-4o-mini", &messages, 32, None, None)
            .await
            .expect_err("400 should fail");
        let text = error.to_string();
        assert!(text.contains("status 400"), "{text}");
        assert!(text.contains("bad tools"), "{text}");
    }

    #[test]
    fn assistant_envelope_round_trips_tool_calls() {
        let calls = vec![ToolCall {
            id: "call_1".into(),
            name: "web_search".into(),
            arguments: "{\"q\":\"x\"}".into(),
        }];
        let encoded = encode_assistant_content("", &calls);
        let (text, decoded) = decode_assistant_content(&encoded);
        assert_eq!(text, "");
        assert_eq!(decoded, calls);
        assert_eq!(decode_assistant_content("plain"), ("plain".into(), vec![]));
    }
}
