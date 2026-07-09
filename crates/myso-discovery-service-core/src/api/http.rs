// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE};
use tracing::debug;

use super::{
    DiscoveryClient, EventsQuery, NormalizedEvent, NormalizedPrice, NormalizedRelease,
    PriceQuery, RefreshRequest, ReleaseQuery, SourceHealthResponse, SourceSummary,
};

const CLIENT_SECRET_HEADER: &str = "x-discovery-client-secret";

#[derive(Debug, Clone)]
pub struct HttpDiscoveryClient {
    base_url: String,
    http: reqwest::Client,
    secret: Option<String>,
}

impl HttpDiscoveryClient {
    pub fn new(base_url: impl Into<String>, secret: Option<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http: reqwest::Client::builder()
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("reqwest client"),
            secret,
        }
    }

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        if let Some(secret) = &self.secret {
            if let Ok(v) = HeaderValue::from_str(secret) {
                headers.insert(CLIENT_SECRET_HEADER, v);
            }
        }
        headers
    }

    async fn get_json<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, String)],
    ) -> anyhow::Result<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!(url = %url, "discovery client GET");
        let mut req = self.http.get(&url).headers(self.headers());
        for (k, v) in query {
            req = req.query(&[(k, v)]);
        }
        let resp = req.send().await?;
        if !resp.status().is_success() {
            anyhow::bail!("discovery GET {} status {}", path, resp.status());
        }
        Ok(resp.json().await?)
    }

    async fn post_json<T: serde::de::DeserializeOwned, B: serde::Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> anyhow::Result<T> {
        let url = format!("{}{}", self.base_url, path);
        debug!(url = %url, "discovery client POST");
        let resp = self
            .http
            .post(&url)
            .headers(self.headers())
            .json(body)
            .send()
            .await?;
        if !resp.status().is_success() {
            anyhow::bail!("discovery POST {} status {}", path, resp.status());
        }
        Ok(resp.json().await?)
    }
}

#[async_trait]
impl DiscoveryClient for HttpDiscoveryClient {
    async fn list_sources(&self) -> anyhow::Result<Vec<SourceSummary>> {
        self.get_json("/v1/sources", &[]).await
    }

    async fn source_health(&self, source_id: &str) -> anyhow::Result<SourceHealthResponse> {
        self.get_json(
            &format!("/v1/sources/{source_id}/health"),
            &[],
        )
        .await
    }

    async fn all_sources_health(&self) -> anyhow::Result<Vec<SourceHealthResponse>> {
        self.get_json("/v1/sources/health", &[]).await
    }

    async fn get_price(&self, query: &PriceQuery) -> anyhow::Result<NormalizedPrice> {
        let mut params = vec![
            ("asset", query.asset.clone()),
            ("quote", query.quote.clone()),
        ];
        if let Some(source_id) = &query.source_id {
            params.push(("source_id", source_id.clone()));
        }
        if query.refresh {
            params.push(("refresh", "true".to_string()));
        }
        self.get_json("/v1/prices", &params).await
    }

    async fn get_release(&self, query: &ReleaseQuery) -> anyhow::Result<NormalizedRelease> {
        let mut params = vec![
            ("owner", query.owner.clone()),
            ("repo", query.repo.clone()),
        ];
        if let Some(tag) = &query.tag {
            params.push(("tag", tag.clone()));
        }
        if let Some(source_id) = &query.source_id {
            params.push(("source_id", source_id.clone()));
        }
        if query.refresh {
            params.push(("refresh", "true".to_string()));
        }
        self.get_json("/v1/releases", &params).await
    }

    async fn get_events(&self, query: &EventsQuery) -> anyhow::Result<Vec<NormalizedEvent>> {
        let mut params = Vec::new();
        if let Some(source_id) = &query.source_id {
            params.push(("source_id", source_id.clone()));
        }
        if let Some(feed) = &query.feed {
            params.push(("feed", feed.clone()));
        }
        if let Some(since) = &query.since {
            params.push(("since", since.to_rfc3339()));
        }
        if let Some(q) = &query.query {
            params.push(("query", q.clone()));
        }
        if query.refresh {
            params.push(("refresh", "true".to_string()));
        }
        self.get_json("/v1/events", &params).await
    }

    async fn refresh_source(&self, request: &RefreshRequest) -> anyhow::Result<serde_json::Value> {
        self.post_json("/v1/refresh", request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_strips_trailing_slash() {
        let c = HttpDiscoveryClient::new("http://127.0.0.1:8096/", None);
        assert_eq!(c.base_url, "http://127.0.0.1:8096");
    }
}
