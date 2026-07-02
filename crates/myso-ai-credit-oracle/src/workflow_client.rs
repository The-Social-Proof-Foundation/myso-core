// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Fire-and-forget workflow-inbox ingest (ApprovalRequest items on the messaging-stack
//! relayer). No-op when `AI_CREDIT_WORKFLOW_RELAYER_URL` is unset.

use anyhow::Result;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct WorkflowItemIngest {
    pub idempotency_key: String,
    pub recipient_address: String,
    pub item_type: String,
    pub title: String,
    pub body: Option<String>,
    pub payload: serde_json::Value,
    pub organization_id: Option<String>,
    pub account_id: Option<String>,
    pub source_service: String,
    pub action_deadline_ms: Option<i64>,
}

#[derive(Clone)]
pub struct WorkflowClient {
    base_url: String,
    secret: Option<String>,
    client: reqwest::Client,
}

impl WorkflowClient {
    pub fn new(base_url: String, secret: Option<String>) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            secret,
            client: reqwest::Client::new(),
        }
    }

    pub fn from_args(url: Option<&String>, secret: Option<&String>) -> Option<Self> {
        url.map(|u| Self::new(u.clone(), secret.cloned()))
    }

    pub async fn ingest_item(&self, item: &WorkflowItemIngest) -> Result<()> {
        let url = format!("{}/internal/workflow/items", self.base_url);
        let mut builder = self.client.post(&url).json(item);
        if let Some(secret) = &self.secret {
            builder = builder.header("x-internal-sync-secret", secret);
        }
        let resp = builder.send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("workflow item ingest status {}", resp.status());
        }
        Ok(())
    }
}

/// Idempotency key shared with the chain lifecycle sync in the workflow relayer, so an
/// on-chain approval transitions exactly the item this request created.
pub fn approval_idempotency_key(balance_id: &str, agent_object_id: &str) -> String {
    format!("approval:{}:{}", balance_id, agent_object_id)
}
