// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::time::Duration;

use anyhow::Context;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct PendingSpotPost {
    pub post_id: String,
    pub owner: String,
    pub content: String,
    pub created_at: i64,
    #[serde(default)]
    pub post_type: Option<String>,
}

#[derive(Clone)]
pub struct SocialClient {
    base_url: String,
    sync_secret: Option<String>,
    client: reqwest::Client,
}

impl SocialClient {
    pub fn new(base_url: String, sync_secret: Option<String>) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("reqwest client");
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            sync_secret,
            client,
        }
    }

    /// Fetch posts awaiting SPoT analysis finalization (`spot_analysis_status = pending`).
    pub async fn list_pending_spot_posts(
        &self,
        limit: i64,
        cursor_ms: Option<i64>,
    ) -> anyhow::Result<Vec<PendingSpotPost>> {
        let mut url = format!("{}/spot/pending-posts?limit={}", self.base_url, limit);
        if let Some(cursor) = cursor_ms {
            url.push_str(&format!("&cursor={cursor}"));
        }
        let mut req = self.client.get(&url);
        if let Some(secret) = &self.sync_secret {
            req = req.header("x-spot-oracle-sync-secret", secret);
        }
        let resp = req.send().await.context("pending-posts request")?;
        if !resp.status().is_success() {
            anyhow::bail!("pending-posts status {}", resp.status());
        }
        resp.json().await.context("parse pending-posts")
    }

    pub async fn get_spot_record(
        &self,
        post_id: &str,
    ) -> anyhow::Result<Option<serde_json::Value>> {
        let url = format!("{}/spot/records/{post_id}", self.base_url);
        let resp = self.client.get(&url).send().await?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !resp.status().is_success() {
            anyhow::bail!("spot record status {}", resp.status());
        }
        Ok(Some(resp.json().await?))
    }
}
