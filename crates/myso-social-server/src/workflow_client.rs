// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Fire-and-forget workflow-inbox ingest on the messaging-stack relayer.
//! No-op when `WORKFLOW_RELAYER_URL` is unset.

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

    pub fn from_env() -> Option<Self> {
        let url = std::env::var("WORKFLOW_RELAYER_URL").ok()?;
        let secret = std::env::var("WORKFLOW_SYNC_SECRET").ok();
        Some(Self::new(url, secret))
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

/// Idempotency key shared with the chain lifecycle sync in the workflow relayer.
pub fn memory_access_idempotency_key(
    organization_id: &str,
    member_address: &str,
    permissions_mask: i64,
) -> String {
    format!("memory_access:{organization_id}:{member_address}:{permissions_mask}")
}
