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
    #[serde(default)]
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
    #[serde(default)]
    pub require_approval_above_mist: Option<i64>,
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
    #[serde(default)]
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct SpendApprovalRow {
    pub balance_id: String,
    pub agent_object_id: String,
    pub status: String,
    pub requested_amount_mist: Option<i64>,
    pub threshold_mist: Option<i64>,
    pub approval_nonce: Option<i64>,
    pub max_amount_mist: Option<i64>,
    pub expires_at_ms: Option<i64>,
    pub approved_by: Option<String>,
    pub approved_by_agent_id: Option<String>,
    pub organization_id: Option<String>,
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

    pub async fn get_ai_credit_balance(
        &self,
        owner: &str,
    ) -> Result<Option<AiCreditBalanceResponse>> {
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

    pub async fn get_sub_agent_by_object_id(
        &self,
        agent_object_id: &str,
    ) -> Result<SocialSubAgent> {
        let url = format!("{}/sub-agents/by-object/{}", self.base_url, agent_object_id);
        let resp = self.client.get(&url).send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("social-server sub-agent status {}", resp.status());
        }
        resp.json().await.context("parse sub-agent")
    }

    pub async fn ingest_usage_line(&self, req: &IngestUsageLineRequest) -> Result<()> {
        self.ingest_usage_line_with_retries(req, 1).await
    }

    pub async fn ingest_usage_line_with_retries(
        &self,
        req: &IngestUsageLineRequest,
        max_attempts: u32,
    ) -> Result<()> {
        let mut last_err = None;
        for attempt in 0..max_attempts {
            if attempt > 0 {
                let delay_ms = 100u64 * 2u64.saturating_pow(attempt - 1);
                tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
            }
            match self.ingest_usage_line_once(req).await {
                Ok(()) => return Ok(()),
                Err(err) => last_err = Some(err),
            }
        }
        Err(last_err.unwrap_or_else(|| anyhow::anyhow!("ingest failed")))
    }

    async fn ingest_usage_line_once(&self, req: &IngestUsageLineRequest) -> Result<()> {
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

    pub async fn get_spend_approvals(
        &self,
        owner: &str,
        agent_object_id: Option<&str>,
        status: Option<&str>,
    ) -> Result<Vec<SpendApprovalRow>> {
        let mut url = format!("{}/profiles/{}/ai-credit/approvals", self.base_url, owner);
        let mut params = Vec::new();
        if let Some(agent) = agent_object_id {
            params.push(format!("agent={}", agent));
        }
        if let Some(status) = status {
            params.push(format!("status={}", status));
        }
        if !params.is_empty() {
            url = format!("{}?{}", url, params.join("&"));
        }
        let resp = self.client.get(&url).send().await?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(Vec::new());
        }
        if !resp.status().is_success() {
            anyhow::bail!("social-server approvals status {}", resp.status());
        }
        resp.json().await.context("parse spend approvals")
    }

    /// Idempotent `requested` upsert used when an over-threshold spend is rejected.
    pub async fn ingest_requested_approval(&self, req: &IngestApprovalRequest) -> Result<()> {
        let url = format!("{}/internal/ai-credit/approvals", self.base_url);
        let mut builder = self.client.post(&url).json(req);
        if let Some(secret) = &self.usage_sync_secret {
            builder = builder.header("x-ai-credit-sync-secret", secret);
        }
        let resp = builder.send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("ingest approval status {}", resp.status());
        }
        Ok(())
    }

    /// Batch audit-log push (idempotent per entry via `idempotency_key`).
    pub async fn ingest_audit_logs(
        &self,
        audit_sync_secret: Option<&str>,
        entries: Vec<IngestAuditLogEntry>,
    ) -> Result<()> {
        if entries.is_empty() {
            return Ok(());
        }
        let url = format!("{}/internal/audit/logs", self.base_url);
        let mut builder = self
            .client
            .post(&url)
            .json(&serde_json::json!({ "entries": entries }));
        if let Some(secret) = audit_sync_secret {
            builder = builder.header("x-audit-sync-secret", secret);
        }
        let resp = builder.send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("ingest audit logs status {}", resp.status());
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
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct IngestApprovalRequest {
    pub balance_id: String,
    pub agent_object_id: String,
    pub requested_amount_mist: Option<i64>,
    pub threshold_mist: Option<i64>,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct IngestAuditLogEntry {
    pub source: String,
    pub actor_address: String,
    pub actor_type: String,
    pub action: String,
    pub target_type: String,
    pub target_id: String,
    pub organization_id: Option<String>,
    pub account_id: Option<String>,
    pub prev_state: Option<serde_json::Value>,
    pub new_state: Option<serde_json::Value>,
    pub tx_digest: Option<String>,
    pub idempotency_key: Option<String>,
    pub metadata: Option<serde_json::Value>,
}
