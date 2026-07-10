// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Inline HTTP fetch client for SPoT trusted-source adapters.
//! Copied from discovery-core `HttpFetchClient` so SPoT has no Discovery dependency.

use anyhow::{anyhow, Context};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Clone)]
pub struct HttpFetchClient {
    client: reqwest::Client,
}

impl HttpFetchClient {
    pub fn new() -> Self {
        let client = reqwest::Client::builder()
            .user_agent("myso-spot-oracle/1.0 (+https://mysocial.network)")
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("reqwest client build must not fail");
        Self { client }
    }

    pub fn with_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    pub fn inner(&self) -> &reqwest::Client {
        &self.client
    }

    pub async fn get_text(&self, url: &str) -> anyhow::Result<FetchedBody> {
        let resp = self
            .client
            .get(url)
            .send()
            .await
            .with_context(|| format!("HTTP GET failed for {url}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!(
                "HTTP {status} for {url}: {}",
                body.chars().take(500).collect::<String>()
            ));
        }
        let body = resp
            .text()
            .await
            .with_context(|| format!("reading body for {url}"))?;
        Ok(FetchedBody::new(url.to_string(), body))
    }

    pub async fn get_json(&self, url: &str) -> anyhow::Result<serde_json::Value> {
        let fetched = self.get_text(url).await?;
        serde_json::from_str(&fetched.body).with_context(|| format!("decoding JSON for {url}"))
    }

    pub async fn get_text_authed(
        &self,
        url: &str,
        api_key_env: Option<&str>,
    ) -> anyhow::Result<FetchedBody> {
        let mut req = self.client.get(url);
        if let Some(env) = api_key_env {
            if let Ok(key) = std::env::var(env) {
                if !key.is_empty() {
                    req = req.bearer_auth(key);
                }
            }
        }
        let resp = req
            .send()
            .await
            .with_context(|| format!("HTTP GET failed for {url}"))?;
        let status = resp.status();
        if !status.is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!(
                "HTTP {status} for {url}: {}",
                body.chars().take(500).collect::<String>()
            ));
        }
        let body = resp
            .text()
            .await
            .with_context(|| format!("reading body for {url}"))?;
        Ok(FetchedBody::new(url.to_string(), body))
    }
}

impl Default for HttpFetchClient {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FetchedBody {
    pub url: String,
    pub body: String,
    /// SHA-256 hex of the response body.
    pub content_hash: String,
}

impl FetchedBody {
    pub fn new(url: String, body: String) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(body.as_bytes());
        let hash = hasher.finalize();
        Self {
            url,
            body,
            content_hash: hex::encode(hash),
        }
    }
}
