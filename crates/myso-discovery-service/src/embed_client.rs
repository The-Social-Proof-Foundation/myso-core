// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct EmbedRequest {
    pub discovery_asset_id: Uuid,
    pub external_source_url: String,
    pub media_type: String,
    pub embedding_version: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_x_handle: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator_confidence: Option<f64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EmbedResponse {
    pub media_id: String,
    pub work_confidence: f64,
    pub embedding_version: String,
    pub embedding_model: String,
    #[serde(default)]
    pub identity_hash: Option<String>,
}

pub struct EmbedClient {
    client: Client,
    endpoint: String,
    secret: Option<String>,
}

impl EmbedClient {
    pub fn new(endpoint: String, secret: Option<String>) -> Self {
        Self {
            client: Client::new(),
            endpoint,
            secret,
        }
    }

    pub async fn embed(&self, request: EmbedRequest) -> anyhow::Result<EmbedResponse> {
        let mut req = self.client.post(&self.endpoint).json(&request);
        if let Some(secret) = &self.secret {
            req = req.header("Authorization", format!("Bearer {secret}"));
        }
        let resp = req.send().await?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            anyhow::bail!("embed request failed {status}: {body}");
        }
        Ok(resp.json().await?)
    }
}
