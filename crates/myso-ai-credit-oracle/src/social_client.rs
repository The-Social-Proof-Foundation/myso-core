// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{Context, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct AiCreditBalanceResponse {
    pub balance: AiCreditBalanceRow,
    pub credits: i64,
    #[serde(default)]
    pub agent_budgets: Vec<AiCreditAgentBudgetRow>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiCreditBalanceRow {
    pub balance_id: String,
    pub memory_account_id: String,
    pub principal_owner: String,
    pub balance_mist: i64,
    pub reserved_mist: i64,
    pub settlement_nonce: i64,
    pub active: bool,
    pub daily_cap_mist: Option<i64>,
    pub monthly_cap_mist: Option<i64>,
    pub spent_day_mist: i64,
    pub spent_month_mist: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AiCreditAgentBudgetRow {
    pub balance_id: String,
    pub agent_object_id: String,
    pub budget_mist: Option<i64>,
    pub spent_mist: i64,
    pub daily_cap_mist: Option<i64>,
    pub monthly_cap_mist: Option<i64>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SocialSubAgent {
    pub agent_object_id: String,
    pub account_id: String,
    pub capabilities: i64,
    pub active: bool,
    pub expires_at_ms: Option<i64>,
    pub revoked_at_ms: Option<i64>,
}

#[derive(Clone)]
pub struct SocialClient {
    base_url: String,
    client: reqwest::Client,
    usage_sync_secret: Option<String>,
}

impl SocialClient {
    pub fn new(base_url: String, usage_sync_secret: Option<String>) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            client: reqwest::Client::new(),
            usage_sync_secret,
        }
    }

    pub async fn get_ai_credit_balance(&self, owner: &str) -> Result<Option<AiCreditBalanceResponse>> {
        let url = format!("{}/profiles/{}/ai-credit", self.base_url, owner);
        let resp = self.client.get(&url).send().await?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !resp.status().is_success() {
            anyhow::bail!("social-server ai-credit status {}", resp.status());
        }
        Ok(Some(resp.json().await?))
    }

    pub async fn get_sub_agent_by_object_id(&self, agent_object_id: &str) -> Result<SocialSubAgent> {
        let url = format!("{}/sub-agents/by-object/{}", self.base_url, agent_object_id);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("social-server sub-agent status {}", resp.status());
        }
        resp.json().await.context("parse sub-agent")
    }

    pub async fn ingest_usage_line(&self, req: &IngestUsageLineRequest) -> Result<()> {
        let url = format!("{}/internal/ai-credit/usage-lines", self.base_url);
        let mut builder = self.client.post(&url).json(req);
        if let Some(secret) = &self.usage_sync_secret {
            builder = builder.header("x-ai-credit-sync-secret", secret);
        }
        let resp = builder.send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("ingest usage-line status {}", resp.status());
        }
        Ok(())
    }
}

#[derive(Debug, serde::Serialize)]
pub struct IngestUsageLineRequest {
    pub receipt_id: String,
    pub balance_id: String,
    pub agent_object_id: String,
    pub usage_kind: i16,
    pub amount_mist: i64,
    pub model_id: Option<String>,
    pub tool_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}
