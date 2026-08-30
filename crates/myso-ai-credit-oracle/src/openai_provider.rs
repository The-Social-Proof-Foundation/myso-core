// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! OpenAI-compatible provider surface for OpenClaw / Hermes.
//!
//! Maps `Authorization: Bearer <token>` into the oracle's internal
//! `InferenceRequest` fields and reuses `run_inference_core`.

use std::time::{SystemTime, UNIX_EPOCH};

use axum::extract::State;
use axum::http::{header, HeaderMap, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::config::OracleArgs;
use crate::openrouter_client::{ChatMessage as RouterChatMessage, OpenRouterToolCall, ToolCall};
use crate::server::{run_inference_core, AppState, InferenceRequest, InferenceResponse};

#[derive(Debug, Clone)]
pub struct ProviderIdentity {
    pub token: String,
    pub owner: String,
    pub balance_id: String,
    pub memory_account_id: String,
    pub agent_object_id: String,
    pub models: Vec<String>,
}

impl ProviderIdentity {
    pub fn from_args(args: &OracleArgs) -> Option<Self> {
        if !args.openai_provider_configured() {
            return None;
        }
        Some(Self {
            token: args.provider_token.clone()?.trim().to_string(),
            owner: args.provider_owner.clone()?.trim().to_string(),
            balance_id: args.provider_balance_id.clone()?.trim().to_string(),
            memory_account_id: args.provider_memory_account_id.clone()?.trim().to_string(),
            agent_object_id: args.provider_agent_object_id.clone()?.trim().to_string(),
            models: args.provider_model_ids(),
        })
    }
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    let value = headers.get(header::AUTHORIZATION)?.to_str().ok()?;
    let token = value
        .strip_prefix("Bearer ")
        .or_else(|| value.strip_prefix("bearer "))?
        .trim();
    if token.is_empty() {
        None
    } else {
        Some(token.to_string())
    }
}

fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff = 0u8;
    for (x, y) in a.bytes().zip(b.bytes()) {
        diff |= x ^ y;
    }
    diff == 0
}

#[derive(Debug)]
pub enum ProviderError {
    Unauthorized(String),
    BadRequest(String),
    PaymentRequired(String),
    Conflict(String),
    Upstream(StatusCode, String),
}

impl ProviderError {
    fn openai_code(&self) -> &'static str {
        match self {
            ProviderError::Unauthorized(_) => "invalid_api_key",
            ProviderError::BadRequest(_) => "invalid_request_error",
            ProviderError::PaymentRequired(_) => "insufficient_quota",
            ProviderError::Conflict(_) => "conflict",
            ProviderError::Upstream(_, _) => "server_error",
        }
    }

    /// OpenClaw remaps every `type: invalid_request_error` to a fake "schema or
    /// tool payload" failure, including HTTP 402 billing rejects. Keep payment
    /// and auth types distinct from request-schema errors.
    fn openai_type(&self) -> &'static str {
        match self {
            ProviderError::Unauthorized(_) => "authentication_error",
            ProviderError::PaymentRequired(_) => "insufficient_quota",
            ProviderError::Upstream(_, _) => "api_error",
            ProviderError::BadRequest(_) | ProviderError::Conflict(_) => "invalid_request_error",
        }
    }

    fn status(&self) -> StatusCode {
        match self {
            ProviderError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ProviderError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ProviderError::PaymentRequired(_) => StatusCode::PAYMENT_REQUIRED,
            ProviderError::Conflict(_) => StatusCode::CONFLICT,
            ProviderError::Upstream(status, _) => *status,
        }
    }

    fn message(&self) -> &str {
        match self {
            ProviderError::Unauthorized(msg)
            | ProviderError::BadRequest(msg)
            | ProviderError::PaymentRequired(msg)
            | ProviderError::Conflict(msg)
            | ProviderError::Upstream(_, msg) => msg,
        }
    }
}

impl IntoResponse for ProviderError {
    fn into_response(self) -> Response {
        (
            self.status(),
            Json(json!({
                "error": {
                    "message": self.message(),
                    "type": self.openai_type(),
                    "code": self.openai_code(),
                }
            })),
        )
            .into_response()
    }
}

pub fn authenticate_provider(
    identity: &ProviderIdentity,
    headers: &HeaderMap,
) -> Result<(), ProviderError> {
    let presented = extract_bearer_token(headers)
        .ok_or_else(|| ProviderError::Unauthorized("missing Authorization: Bearer token".into()))?;
    if !constant_time_eq(&identity.token, &presented) {
        return Err(ProviderError::Unauthorized(
            "invalid AI credit provider token".into(),
        ));
    }
    Ok(())
}

fn map_core_status(status: StatusCode) -> ProviderError {
    match status {
        StatusCode::PAYMENT_REQUIRED => {
            // Phrase must match OpenClaw's `/insufficient[_ ]balance/` billing detector.
            ProviderError::PaymentRequired("insufficient_balance".into())
        }
        StatusCode::CONFLICT => ProviderError::Conflict(
            "AI inference with this idempotency key is still reconciling".into(),
        ),
        StatusCode::BAD_REQUEST | StatusCode::PAYLOAD_TOO_LARGE => {
            ProviderError::BadRequest("invalid inference request".into())
        }
        StatusCode::UNAUTHORIZED => {
            ProviderError::Unauthorized("oracle rejected the request".into())
        }
        other => ProviderError::Upstream(other, format!("AI credit inference failed ({other})")),
    }
}

#[derive(Debug, Deserialize)]
pub struct ChatCompletionsRequest {
    pub model: Option<String>,
    #[serde(default)]
    pub messages: Vec<ChatMessage>,
    pub max_tokens: Option<u32>,
    pub max_completion_tokens: Option<u32>,
    #[serde(default)]
    pub stream: bool,
    #[allow(dead_code)]
    pub temperature: Option<f64>,
    #[serde(default)]
    pub tools: Option<Value>,
    #[serde(default)]
    pub tool_choice: Option<Value>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChatMessage {
    pub role: String,
    #[serde(default)]
    pub content: ChatContent,
    #[serde(default)]
    pub tool_call_id: Option<String>,
    #[serde(default)]
    pub tool_calls: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChatContent {
    Text(String),
    Parts(Vec<Value>),
    Null,
}

impl Default for ChatContent {
    fn default() -> Self {
        ChatContent::Null
    }
}

impl ChatContent {
    pub fn as_text(&self) -> String {
        match self {
            ChatContent::Text(s) => s.clone(),
            ChatContent::Parts(parts) => parts
                .iter()
                .filter_map(|part| {
                    if let Some(s) = part.as_str() {
                        return Some(s.to_string());
                    }
                    part.get("text")
                        .and_then(|t| t.as_str())
                        .map(String::from)
                        .or_else(|| {
                            part.get("content")
                                .and_then(|t| t.as_str())
                                .map(String::from)
                        })
                })
                .collect::<Vec<_>>()
                .join("\n"),
            ChatContent::Null => String::new(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ResponsesRequest {
    pub model: Option<String>,
    pub input: Option<Value>,
    pub instructions: Option<String>,
    pub max_output_tokens: Option<u32>,
    pub max_tokens: Option<u32>,
    #[serde(default)]
    pub stream: bool,
    #[serde(default)]
    pub messages: Vec<ChatMessage>,
    #[serde(default)]
    pub tools: Option<Value>,
    #[serde(default)]
    pub tool_choice: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct InferencePrompt {
    pub system_prompt: Option<String>,
    pub prompt: String,
    pub max_tokens: u32,
    pub model_id: String,
    pub chat_messages: Vec<RouterChatMessage>,
    pub tools: Option<Value>,
    pub tool_choice: Option<Value>,
}

pub fn extract_chat_prompt(
    req: &ChatCompletionsRequest,
    default_model: &str,
) -> Result<InferencePrompt, ProviderError> {
    if req.stream {
        return Err(ProviderError::BadRequest(
            "streaming is not supported yet on the AI credit OpenAI-compatible provider".into(),
        ));
    }
    if req.messages.is_empty() {
        return Err(ProviderError::BadRequest("messages cannot be empty".into()));
    }

    let chat_messages = openai_chat_to_router(&req.messages);
    let (system_prompt, prompt) = flatten_router_messages(&chat_messages);
    if prompt.trim().is_empty() {
        return Err(ProviderError::BadRequest(
            "at least one non-empty user/assistant/tool message is required".into(),
        ));
    }
    Ok(InferencePrompt {
        system_prompt,
        prompt,
        max_tokens: clamp_provider_max_tokens(req.max_tokens.or(req.max_completion_tokens)),
        model_id: resolve_model_id(req.model.as_deref(), default_model),
        chat_messages,
        tools: req.tools.as_ref().and_then(convert_to_openrouter_tools),
        tool_choice: req.tool_choice.clone(),
    })
}

const MAX_OPENAI_PROVIDER_OUTPUT_TOKENS: u32 = 4096;

fn clamp_provider_max_tokens(requested: Option<u32>) -> u32 {
    match requested {
        Some(0) | None => 512,
        Some(n) => n.min(MAX_OPENAI_PROVIDER_OUTPUT_TOKENS),
    }
}

fn resolve_model_id(model: Option<&str>, default_model: &str) -> String {
    model
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(default_model)
        .to_string()
}

fn flatten_input_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Array(items) => items
            .iter()
            .map(flatten_input_value)
            .filter(|s| !s.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n"),
        Value::Object(map) => {
            if let Some(text) = map.get("text").and_then(|v| v.as_str()) {
                return text.to_string();
            }
            if let Some(output) = map.get("output") {
                return flatten_input_value(output);
            }
            if let Some(content) = map.get("content") {
                return flatten_input_value(content);
            }
            if let Some(input_text) = map.get("input_text").and_then(|v| v.as_str()) {
                return input_text.to_string();
            }
            String::new()
        }
        _ => String::new(),
    }
}

pub fn convert_to_openrouter_tools(tools: &Value) -> Option<Value> {
    let items = tools.as_array()?;
    let converted: Vec<Value> = items
        .iter()
        .filter_map(|tool| {
            if tool.get("function").is_some() {
                return Some(tool.clone());
            }
            let name = tool.get("name").and_then(|v| v.as_str())?;
            Some(json!({
                "type": "function",
                "function": {
                    "name": name,
                    "description": tool.get("description").cloned().unwrap_or(json!("")),
                    "parameters": tool.get("parameters").cloned().unwrap_or(json!({
                        "type": "object",
                        "properties": {}
                    })),
                }
            }))
        })
        .collect();
    if converted.is_empty() {
        None
    } else {
        Some(Value::Array(converted))
    }
}

fn parse_openai_tool_calls(raw: Option<&Vec<Value>>) -> Option<Vec<OpenRouterToolCall>> {
    let items = raw?;
    let parsed: Vec<OpenRouterToolCall> = items
        .iter()
        .filter_map(|value| {
            if let Ok(call) = serde_json::from_value::<OpenRouterToolCall>(value.clone()) {
                return Some(call);
            }
            let id = value
                .get("id")
                .or_else(|| value.get("call_id"))
                .and_then(|v| v.as_str())?
                .to_string();
            let name = value
                .get("name")
                .or_else(|| value.get("function").and_then(|f| f.get("name")))
                .and_then(|v| v.as_str())?
                .to_string();
            let arguments = match value
                .get("arguments")
                .or_else(|| value.get("function").and_then(|f| f.get("arguments")))
            {
                Some(Value::String(s)) => s.clone(),
                Some(other) => other.to_string(),
                None => "{}".to_string(),
            };
            Some(OpenRouterToolCall::function(id, name, arguments))
        })
        .collect();
    if parsed.is_empty() {
        None
    } else {
        Some(parsed)
    }
}

fn openai_chat_to_router(messages: &[ChatMessage]) -> Vec<RouterChatMessage> {
    messages
        .iter()
        .filter_map(|msg| {
            let text = msg.content.as_text();
            let tool_calls = parse_openai_tool_calls(msg.tool_calls.as_ref());
            if text.trim().is_empty() && tool_calls.is_none() && msg.tool_call_id.is_none() {
                return None;
            }
            Some(RouterChatMessage {
                role: msg.role.clone(),
                content: if text.is_empty() { None } else { Some(text) },
                tool_call_id: msg.tool_call_id.clone(),
                tool_calls,
            })
        })
        .collect()
}

fn flatten_router_messages(messages: &[RouterChatMessage]) -> (Option<String>, String) {
    let mut system_parts = Vec::new();
    let mut user_parts = Vec::new();
    for msg in messages {
        match msg.role.as_str() {
            "system" | "developer" => {
                if let Some(text) = &msg.content {
                    if !text.trim().is_empty() {
                        system_parts.push(text.clone());
                    }
                }
            }
            "assistant" => {
                if let Some(text) = &msg.content {
                    if !text.trim().is_empty() {
                        user_parts.push(format!("Assistant: {text}"));
                    }
                }
                if let Some(calls) = &msg.tool_calls {
                    for call in calls {
                        user_parts.push(format!(
                            "Assistant tool_call {}: {}",
                            call.function.name, call.function.arguments
                        ));
                    }
                }
            }
            "tool" => {
                let id = msg.tool_call_id.as_deref().unwrap_or("");
                let text = msg.content.as_deref().unwrap_or("");
                user_parts.push(format!("Tool {id}: {text}"));
            }
            _ => {
                if let Some(text) = &msg.content {
                    if !text.trim().is_empty() {
                        user_parts.push(text.clone());
                    }
                }
            }
        }
    }
    let system_prompt = if system_parts.is_empty() {
        None
    } else {
        Some(system_parts.join("\n\n"))
    };
    (system_prompt, user_parts.join("\n\n"))
}

fn responses_item_to_messages(item: &Value) -> Vec<RouterChatMessage> {
    let item_type = item.get("type").and_then(|v| v.as_str()).unwrap_or("");
    match item_type {
        "function_call" => {
            let call_id = item
                .get("call_id")
                .or_else(|| item.get("id"))
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let name = item
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let arguments = match item.get("arguments") {
                Some(Value::String(s)) => s.clone(),
                Some(other) => other.to_string(),
                None => "{}".to_string(),
            };
            vec![RouterChatMessage {
                role: "assistant".into(),
                content: None,
                tool_call_id: None,
                tool_calls: Some(vec![OpenRouterToolCall::function(call_id, name, arguments)]),
            }]
        }
        "function_call_output" => {
            let call_id = item
                .get("call_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let output = match item.get("output") {
                Some(Value::String(s)) => s.clone(),
                Some(other) => flatten_input_value(other),
                None => String::new(),
            };
            vec![RouterChatMessage {
                role: "tool".into(),
                content: Some(output),
                tool_call_id: Some(call_id),
                tool_calls: None,
            }]
        }
        _ => {
            let role = item
                .get("role")
                .and_then(|v| v.as_str())
                .unwrap_or("user")
                .to_string();
            let text = item
                .get("content")
                .map(flatten_input_value)
                .filter(|s| !s.trim().is_empty())
                .or_else(|| item.get("text").and_then(|v| v.as_str()).map(String::from))
                .unwrap_or_else(|| flatten_input_value(item));
            if text.trim().is_empty() {
                Vec::new()
            } else {
                vec![RouterChatMessage::text(role, text)]
            }
        }
    }
}

pub fn responses_input_to_messages(input: &Value) -> Vec<RouterChatMessage> {
    match input {
        Value::String(s) => vec![RouterChatMessage::text("user", s.clone())],
        Value::Array(items) => items.iter().flat_map(responses_item_to_messages).collect(),
        Value::Object(_) => responses_item_to_messages(input),
        _ => Vec::new(),
    }
}

pub fn extract_responses_prompt(
    req: &ResponsesRequest,
    default_model: &str,
) -> Result<InferencePrompt, ProviderError> {
    if req.stream {
        return Err(ProviderError::BadRequest(
            "streaming is not supported yet on the AI credit OpenAI-compatible provider".into(),
        ));
    }

    let mut chat_messages = Vec::new();
    if let Some(instructions) = &req.instructions {
        if !instructions.trim().is_empty() {
            chat_messages.push(RouterChatMessage::text("system", instructions.clone()));
        }
    }
    if let Some(input) = &req.input {
        chat_messages.extend(responses_input_to_messages(input));
    }
    let has_non_system = chat_messages.iter().any(|msg| msg.role != "system");
    if !has_non_system && !req.messages.is_empty() {
        chat_messages.extend(openai_chat_to_router(&req.messages));
    }

    let (system_prompt, prompt) = flatten_router_messages(&chat_messages);
    if prompt.trim().is_empty() {
        return Err(ProviderError::BadRequest(
            "input (or messages) cannot be empty".into(),
        ));
    }

    Ok(InferencePrompt {
        system_prompt,
        prompt,
        max_tokens: clamp_provider_max_tokens(req.max_output_tokens.or(req.max_tokens)),
        model_id: resolve_model_id(req.model.as_deref(), default_model),
        chat_messages,
        tools: req.tools.as_ref().and_then(convert_to_openrouter_tools),
        tool_choice: req.tool_choice.clone(),
    })
}

fn now_epoch_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn tool_calls_as_openai(tool_calls: &[ToolCall]) -> Vec<Value> {
    tool_calls
        .iter()
        .map(|call| {
            json!({
                "id": call.id,
                "type": "function",
                "function": {
                    "name": call.name,
                    "arguments": call.arguments,
                }
            })
        })
        .collect()
}

pub fn chat_completion_response(model: &str, result: &InferenceResponse) -> Value {
    let id = format!("chatcmpl-{}", uuid::Uuid::new_v4());
    let has_tools = !result.tool_calls.is_empty();
    let mut message = serde_json::Map::new();
    message.insert("role".into(), json!("assistant"));
    if result.content.is_empty() && has_tools {
        message.insert("content".into(), Value::Null);
    } else {
        message.insert("content".into(), json!(result.content));
    }
    if has_tools {
        message.insert(
            "tool_calls".into(),
            json!(tool_calls_as_openai(&result.tool_calls)),
        );
    }
    json!({
        "id": id,
        "object": "chat.completion",
        "created": now_epoch_secs(),
        "model": model,
        "choices": [{
            "index": 0,
            "message": message,
            "finish_reason": if has_tools { "tool_calls" } else { "stop" },
        }],
        "usage": {
            "prompt_tokens": result.tokens_in,
            "completion_tokens": result.tokens_out,
            "total_tokens": result.tokens_in.saturating_add(result.tokens_out),
        }
    })
}

pub fn responses_api_response(model: &str, result: &InferenceResponse) -> Value {
    let id = format!("resp_{}", uuid::Uuid::new_v4().simple());
    let mut output = Vec::new();
    if !result.content.is_empty() || result.tool_calls.is_empty() {
        output.push(json!({
            "type": "message",
            "id": format!("msg_{}", uuid::Uuid::new_v4().simple()),
            "role": "assistant",
            "status": "completed",
            "content": [{
                "type": "output_text",
                "text": result.content,
            }],
        }));
    }
    for call in &result.tool_calls {
        output.push(json!({
            "type": "function_call",
            "id": format!("fc_{}", uuid::Uuid::new_v4().simple()),
            "call_id": call.id,
            "name": call.name,
            "arguments": call.arguments,
        }));
    }
    json!({
        "id": id,
        "object": "response",
        "created_at": now_epoch_secs(),
        "status": "completed",
        "model": model,
        "output": output,
        "usage": {
            "input_tokens": result.tokens_in,
            "output_tokens": result.tokens_out,
            "total_tokens": result.tokens_in.saturating_add(result.tokens_out),
        }
    })
}

fn build_inference_request(
    identity: &ProviderIdentity,
    prompt: InferencePrompt,
) -> InferenceRequest {
    InferenceRequest {
        owner: identity.owner.clone(),
        balance_id: identity.balance_id.clone(),
        memory_account_id: identity.memory_account_id.clone(),
        agent_object_id: identity.agent_object_id.clone(),
        model_id: prompt.model_id,
        system_prompt: prompt.system_prompt,
        prompt: prompt.prompt,
        max_tokens: Some(prompt.max_tokens),
        idempotency_key: uuid::Uuid::new_v4().to_string(),
        chat_messages: prompt.chat_messages,
        tools: prompt.tools,
        tool_choice: prompt.tool_choice,
    }
}

fn require_identity(state: &AppState) -> Result<ProviderIdentity, ProviderError> {
    ProviderIdentity::from_args(&state.oracle_args).ok_or_else(|| {
        ProviderError::Unauthorized("AI credit OpenAI provider is not configured".into())
    })
}

fn default_model_id(identity: &ProviderIdentity) -> String {
    identity
        .models
        .first()
        .cloned()
        .unwrap_or_else(|| "openai/gpt-4o-mini".to_string())
}

pub async fn list_models(State(state): State<AppState>, headers: HeaderMap) -> Response {
    let identity = match require_identity(&state) {
        Ok(identity) => identity,
        Err(err) => return err.into_response(),
    };
    if let Err(err) = authenticate_provider(&identity, &headers) {
        return err.into_response();
    }

    let created = now_epoch_secs();
    let mut model_ids = identity.models.clone();
    if model_ids.is_empty() {
        let catalog = state.catalog.read().await;
        model_ids = catalog
            .to_response()
            .models
            .into_iter()
            .filter_map(|m| m.aliases.into_iter().next())
            .collect();
    }
    let data: Vec<Value> = model_ids
        .into_iter()
        .map(|id| {
            json!({
                "id": id,
                "object": "model",
                "created": created,
                "owned_by": "myso-ai-credit",
            })
        })
        .collect();
    Json(json!({
        "object": "list",
        "data": data,
    }))
    .into_response()
}

pub async fn chat_completions(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<ChatCompletionsRequest>,
) -> Response {
    let identity = match require_identity(&state) {
        Ok(identity) => identity,
        Err(err) => return err.into_response(),
    };
    if let Err(err) = authenticate_provider(&identity, &headers) {
        return err.into_response();
    }
    let default_model = default_model_id(&identity);
    let prompt = match extract_chat_prompt(&body, &default_model) {
        Ok(prompt) => prompt,
        Err(err) => return err.into_response(),
    };
    let model = prompt.model_id.clone();
    let req = build_inference_request(&identity, prompt);
    match run_inference_core(&state, req).await {
        Ok(result) => Json(chat_completion_response(&model, &result)).into_response(),
        Err(status) => map_core_status(status).into_response(),
    }
}

pub async fn responses(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(body): Json<ResponsesRequest>,
) -> Response {
    let identity = match require_identity(&state) {
        Ok(identity) => identity,
        Err(err) => return err.into_response(),
    };
    if let Err(err) = authenticate_provider(&identity, &headers) {
        return err.into_response();
    }
    let default_model = default_model_id(&identity);
    let prompt = match extract_responses_prompt(&body, &default_model) {
        Ok(prompt) => prompt,
        Err(err) => return err.into_response(),
    };
    let model = prompt.model_id.clone();
    let req = build_inference_request(&identity, prompt);
    match run_inference_core(&state, req).await {
        Ok(result) => Json(responses_api_response(&model, &result)).into_response(),
        Err(status) => map_core_status(status).into_response(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;

    fn test_identity() -> ProviderIdentity {
        ProviderIdentity {
            token: "local-openclaw-token".into(),
            owner: "0xowner".into(),
            balance_id: "0xbal".into(),
            memory_account_id: "0xmem".into(),
            agent_object_id: "0xagent".into(),
            models: vec!["openai/gpt-4o-mini".into()],
        }
    }

    #[test]
    fn extract_chat_prompt_splits_system_and_user() {
        let req = ChatCompletionsRequest {
            model: Some("openai/gpt-4o".into()),
            messages: vec![
                ChatMessage {
                    role: "system".into(),
                    content: ChatContent::Text("Be brief".into()),
                    tool_call_id: None,
                    tool_calls: None,
                },
                ChatMessage {
                    role: "user".into(),
                    content: ChatContent::Text("Hello".into()),
                    tool_call_id: None,
                    tool_calls: None,
                },
            ],
            max_tokens: Some(64),
            max_completion_tokens: None,
            stream: false,
            temperature: None,
            tools: None,
            tool_choice: None,
        };
        let prompt = extract_chat_prompt(&req, "openai/gpt-4o-mini").unwrap();
        assert_eq!(prompt.system_prompt.as_deref(), Some("Be brief"));
        assert_eq!(prompt.prompt, "Hello");
        assert_eq!(prompt.max_tokens, 64);
        assert_eq!(prompt.model_id, "openai/gpt-4o");
    }

    #[test]
    fn extract_chat_prompt_rejects_stream() {
        let req = ChatCompletionsRequest {
            model: None,
            messages: vec![ChatMessage {
                role: "user".into(),
                content: ChatContent::Text("hi".into()),
                tool_call_id: None,
                tool_calls: None,
            }],
            max_tokens: None,
            max_completion_tokens: None,
            stream: true,
            temperature: None,
            tools: None,
            tool_choice: None,
        };
        assert!(extract_chat_prompt(&req, "openai/gpt-4o-mini").is_err());
    }

    #[test]
    fn extract_responses_prompt_from_string_input() {
        let req = ResponsesRequest {
            model: None,
            input: Some(Value::String("ping".into())),
            instructions: Some("sys".into()),
            max_output_tokens: Some(32),
            max_tokens: None,
            stream: false,
            messages: vec![],
            tools: None,
            tool_choice: None,
        };
        let prompt = extract_responses_prompt(&req, "openai/gpt-4o-mini").unwrap();
        assert_eq!(prompt.prompt, "ping");
        assert_eq!(prompt.system_prompt.as_deref(), Some("sys"));
        assert_eq!(prompt.max_tokens, 32);
    }

    #[test]
    fn authenticate_provider_rejects_missing_and_bad_tokens() {
        let identity = test_identity();
        let headers = HeaderMap::new();
        assert!(authenticate_provider(&identity, &headers).is_err());

        let mut headers = HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer wrong"),
        );
        assert!(authenticate_provider(&identity, &headers).is_err());

        headers.insert(
            header::AUTHORIZATION,
            HeaderValue::from_static("Bearer local-openclaw-token"),
        );
        assert!(authenticate_provider(&identity, &headers).is_ok());
    }

    #[test]
    fn chat_and_responses_shapes() {
        let result = InferenceResponse {
            receipt_id: None,
            amount_mist: 14,
            settlement_nonce: None,
            signature: None,
            receipt: None,
            reservation_nonce: Some(1),
            reserved_mist: Some(20),
            reserve_digest: None,
            capture_digest: None,
            billing_state: "captured".into(),
            tokens_in: 11,
            tokens_out: 3,
            model_id: "openai/gpt-4o-mini".into(),
            content: "AI_CREDIT_OK".into(),
            tool_calls: vec![],
            provider_cost_usd_micros: 1,
            upstream_cost_usd_micros: None,
            provider_generation_id: None,
        };
        let chat = chat_completion_response("openai/gpt-4o-mini", &result);
        assert_eq!(chat["choices"][0]["message"]["content"], "AI_CREDIT_OK");
        let responses = responses_api_response("openai/gpt-4o-mini", &result);
        assert_eq!(responses["output"][0]["content"][0]["text"], "AI_CREDIT_OK");
    }

    #[test]
    fn build_inference_request_maps_provider_identity() {
        let identity = test_identity();
        let prompt = InferencePrompt {
            system_prompt: Some("sys".into()),
            prompt: "hello".into(),
            max_tokens: 32,
            model_id: "openai/gpt-4o-mini".into(),
            chat_messages: vec![],
            tools: None,
            tool_choice: None,
        };
        let req = build_inference_request(&identity, prompt);
        assert_eq!(req.owner, "0xowner");
        assert_eq!(req.balance_id, "0xbal");
        assert_eq!(req.memory_account_id, "0xmem");
        assert_eq!(req.agent_object_id, "0xagent");
        assert_eq!(req.model_id, "openai/gpt-4o-mini");
        assert_eq!(req.system_prompt.as_deref(), Some("sys"));
        assert_eq!(req.prompt, "hello");
        assert_eq!(req.max_tokens, Some(32));
        assert!(req.chat_messages.is_empty());
        assert!(req.tools.is_none());
        assert!(!req.idempotency_key.is_empty());
    }

    #[test]
    fn convert_responses_tools_to_openrouter_functions() {
        let tools = json!([{
            "type": "function",
            "name": "web_search",
            "description": "Search the web",
            "parameters": { "type": "object", "properties": { "query": { "type": "string" } } },
            "strict": false
        }]);
        let converted = convert_to_openrouter_tools(&tools).expect("tools");
        assert_eq!(converted[0]["type"], "function");
        assert_eq!(converted[0]["function"]["name"], "web_search");
        assert_eq!(converted[0]["function"]["description"], "Search the web");
        assert!(converted[0]["function"]["parameters"]["properties"]["query"].is_object());
    }

    #[test]
    fn extract_responses_prompt_keeps_function_call_and_output() {
        let req = ResponsesRequest {
            model: Some("openai/gpt-4o-mini".into()),
            input: Some(json!([
                {
                    "type": "message",
                    "role": "user",
                    "content": [{ "type": "input_text", "text": "Cowboys preseason score?" }]
                },
                {
                    "type": "function_call",
                    "call_id": "call_1",
                    "name": "web_search",
                    "arguments": "{\"query\":\"Cowboys latest preseason game\"}"
                },
                {
                    "type": "function_call_output",
                    "call_id": "call_1",
                    "output": "Cowboys beat the Rams 31-21"
                }
            ])),
            instructions: Some("Be brief".into()),
            max_output_tokens: Some(128),
            max_tokens: None,
            stream: false,
            messages: vec![],
            tools: Some(json!([{
                "type": "function",
                "name": "web_search",
                "description": "Search the web",
                "parameters": { "type": "object", "properties": {} }
            }])),
            tool_choice: None,
        };
        let prompt = extract_responses_prompt(&req, "openai/gpt-4o-mini").unwrap();
        assert!(prompt.prompt.contains("Cowboys preseason score?"));
        assert!(prompt.prompt.contains("Assistant tool_call web_search"));
        assert!(prompt
            .prompt
            .contains("Tool call_1: Cowboys beat the Rams 31-21"));
        assert_eq!(prompt.chat_messages.len(), 4);
        assert_eq!(prompt.chat_messages[0].role, "system");
        assert_eq!(prompt.chat_messages[2].role, "assistant");
        assert_eq!(
            prompt.chat_messages[2].tool_calls.as_ref().unwrap()[0]
                .function
                .name,
            "web_search"
        );
        assert_eq!(prompt.chat_messages[3].role, "tool");
        assert_eq!(
            prompt.chat_messages[3].tool_call_id.as_deref(),
            Some("call_1")
        );
        assert_eq!(
            prompt.tools.as_ref().unwrap()[0]["function"]["name"],
            "web_search"
        );
    }

    #[test]
    fn responses_api_emits_function_call_items() {
        let result = InferenceResponse {
            receipt_id: None,
            amount_mist: 14,
            settlement_nonce: None,
            signature: None,
            receipt: None,
            reservation_nonce: Some(1),
            reserved_mist: Some(20),
            reserve_digest: None,
            capture_digest: None,
            billing_state: "captured".into(),
            tokens_in: 11,
            tokens_out: 3,
            model_id: "openai/gpt-4o-mini".into(),
            content: String::new(),
            tool_calls: vec![ToolCall {
                id: "call_1".into(),
                name: "web_search".into(),
                arguments: "{\"query\":\"Cowboys\"}".into(),
            }],
            provider_cost_usd_micros: 1,
            upstream_cost_usd_micros: None,
            provider_generation_id: None,
        };
        let responses = responses_api_response("openai/gpt-4o-mini", &result);
        assert_eq!(responses["output"][0]["type"], "function_call");
        assert_eq!(responses["output"][0]["call_id"], "call_1");
        assert_eq!(responses["output"][0]["name"], "web_search");
        assert_eq!(
            responses["output"][0]["arguments"],
            "{\"query\":\"Cowboys\"}"
        );
        let chat = chat_completion_response("openai/gpt-4o-mini", &result);
        assert_eq!(chat["choices"][0]["finish_reason"], "tool_calls");
        assert_eq!(
            chat["choices"][0]["message"]["tool_calls"][0]["function"]["name"],
            "web_search"
        );
    }

    #[test]
    fn clamp_provider_max_tokens_defaults_and_caps() {
        assert_eq!(clamp_provider_max_tokens(None), 512);
        assert_eq!(clamp_provider_max_tokens(Some(0)), 512);
        assert_eq!(clamp_provider_max_tokens(Some(64)), 64);
        assert_eq!(
            clamp_provider_max_tokens(Some(200_000)),
            MAX_OPENAI_PROVIDER_OUTPUT_TOKENS
        );
    }

    #[test]
    fn payment_required_is_not_invalid_request_error() {
        let err = map_core_status(StatusCode::PAYMENT_REQUIRED);
        assert_eq!(err.openai_type(), "insufficient_quota");
        assert_eq!(err.openai_code(), "insufficient_quota");
        assert_eq!(err.message(), "insufficient_balance");
        assert_eq!(err.status(), StatusCode::PAYMENT_REQUIRED);
        let schema = map_core_status(StatusCode::BAD_REQUEST);
        assert_eq!(schema.openai_type(), "invalid_request_error");
    }

    #[test]
    fn provider_identity_requires_all_core_fields() {
        let mut args = OracleArgs {
            provider_token: None,
            provider_owner: None,
            provider_balance_id: None,
            provider_memory_account_id: None,
            provider_agent_object_id: None,
            provider_models: None,
            ..minimal_test_args()
        };
        assert!(ProviderIdentity::from_args(&args).is_none());
        args.provider_token = Some("t".into());
        args.provider_owner = Some("0xo".into());
        args.provider_balance_id = Some("0xb".into());
        args.provider_memory_account_id = Some("0xm".into());
        args.provider_agent_object_id = Some("0xa".into());
        let identity = ProviderIdentity::from_args(&args).unwrap();
        assert_eq!(identity.token, "t");
    }

    fn minimal_test_args() -> OracleArgs {
        OracleArgs {
            listen_addr: "0.0.0.0:8095".into(),
            database_url: "postgres://localhost/test".into(),
            database_max_connections: 10,
            outbox_lease_secs: 60,
            replica_count: 1,
            legacy_usage_enabled: true,
            private_key_hex: "00".repeat(32),
            settlement_secret: None,
            myso_rpc: "http://127.0.0.1:9000".into(),
            receipt_store_path: std::path::PathBuf::from("test.json"),
            config_object_id: None,
            settlement_key_hex: None,
            reservation_price_buffer_bps: 2500,
            reservation_capture_window_secs: 600,
            reservation_hard_expiry_secs: 1800,
            settlement_interval_secs: 60,
            settle_threshold_mist: 10_000_000_000,
            settle_max_age_secs: 180,
            settle_min_count: 8,
            settle_warn_age_secs: 240,
            social_server_url: "http://127.0.0.1:9126".into(),
            pricing_catalog_path: std::path::PathBuf::from("config/pricing_catalog.toml"),
            ecosystem_margin_pct: 0.125,
            graphql_url: "http://127.0.0.1:9125/graphql".into(),
            markup_refresh_interval_secs: 300,
            markup_graphql_enabled: false,
            usage_sync_secret: None,
            strict_catalog: false,
            myso_price_oracle_url: "https://myso-price-oracle-testnet.up.railway.app".into(),
            price_refresh_interval_secs: 60,
            myso_price_max_stale_secs: 300,
            myso_price_enabled: false,
            openrouter_api_key: None,
            catalog_sync_enabled: false,
            catalog_sync_interval_secs: 86400,
            catalog_sync_on_startup: true,
            openrouter_api_url: "https://openrouter.ai/api/v1/models".into(),
            openrouter_chat_url: "https://openrouter.ai/api/v1/chat/completions".into(),
            inference_enabled: false,
            catalog_max_drift_pct: 50.0,
            approvals_enabled: false,
            approval_lookup_ttl_secs: 5,
            approval_min_remaining_secs: 180,
            workflow_relayer_url: None,
            workflow_sync_secret: None,
            audit_sync_secret: None,
            oracle_api_secret: None,
            require_secrets: false,
            agent_auth_enabled: false,
            agent_auth_ttl_secs: 300,
            require_settlement_secret: false,
            receipt_store_recover: false,
            ingest_reconcile_interval_secs: 30,
            ingest_backlog_warn_age_secs: 300,
            provider_token: None,
            provider_owner: None,
            provider_balance_id: None,
            provider_memory_account_id: None,
            provider_agent_object_id: None,
            provider_models: None,
        }
    }
}
