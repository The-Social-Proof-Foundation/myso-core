// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! GraphQL client for on-chain AI credit config (oracle markup bps).

use std::time::Duration;

use serde::Deserialize;

const AI_CREDIT_MARKUP_QUERY: &str = "query { aiCreditConfiguration { oracleMarkupBps } }";

pub const MAX_ORACLE_MARKUP_BPS: u64 = 10_000;

#[derive(Debug, Clone)]
pub struct MarkupConfigClient {
    graphql_url: String,
    social_server_url: String,
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct GraphqlEnvelope {
    data: Option<GraphqlData>,
    errors: Option<Vec<GraphqlError>>,
}

#[derive(Debug, Deserialize)]
struct GraphqlData {
    #[serde(rename = "aiCreditConfiguration")]
    ai_credit_configuration: Option<AiCreditConfiguration>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct AiCreditConfiguration {
    oracle_markup_bps: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct GraphqlError {
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RestAiCreditConfig {
    oracle_markup_bps: i64,
}

impl MarkupConfigClient {
    pub fn new(graphql_url: impl Into<String>, social_server_url: impl Into<String>) -> Self {
        Self {
            graphql_url: graphql_url.into(),
            social_server_url: social_server_url.into().trim_end_matches('/').to_string(),
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("reqwest client"),
        }
    }

    pub fn graphql_url(&self) -> &str {
        &self.graphql_url
    }

    pub async fn fetch_oracle_markup_bps(&self) -> Result<u64, String> {
        match self.fetch_oracle_markup_bps_graphql().await {
            Ok(bps) => Ok(bps),
            Err(graphql_err) => match self.fetch_oracle_markup_bps_rest().await {
                Ok(bps) => {
                    tracing::info!(
                        graphql_error = %graphql_err,
                        oracle_markup_bps = bps,
                        "GraphQL markup fetch failed; using social-server REST fallback"
                    );
                    Ok(bps)
                }
                Err(rest_err) => Err(format!(
                    "GraphQL markup fetch failed ({graphql_err}); REST fallback failed ({rest_err})"
                )),
            },
        }
    }

    async fn fetch_oracle_markup_bps_graphql(&self) -> Result<u64, String> {
        let resp = self
            .http
            .post(&self.graphql_url)
            .json(&serde_json::json!({ "query": AI_CREDIT_MARKUP_QUERY }))
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if !resp.status().is_success() {
            return Err(format!("GraphQL HTTP {}", resp.status()));
        }
        let body = resp.text().await.map_err(|e| e.to_string())?;
        parse_oracle_markup_bps_from_graphql_body(&body)
    }

    async fn fetch_oracle_markup_bps_rest(&self) -> Result<u64, String> {
        let url = format!("{}/ai-credit/config", self.social_server_url);
        let resp = self
            .http
            .get(&url)
            .send()
            .await
            .map_err(|e| e.to_string())?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Err("AI credit config not found on social-server".into());
        }
        if !resp.status().is_success() {
            return Err(format!("REST config HTTP {}", resp.status()));
        }
        let row: RestAiCreditConfig = resp.json().await.map_err(|e| e.to_string())?;
        validate_oracle_markup_bps(row.oracle_markup_bps)
    }
}

pub fn parse_oracle_markup_bps_from_graphql_body(body: &str) -> Result<u64, String> {
    let envelope: GraphqlEnvelope = serde_json::from_str(body).map_err(|e| e.to_string())?;
    if let Some(errors) = envelope.errors {
        if let Some(first) = errors.first() {
            return Err(first
                .message
                .clone()
                .unwrap_or_else(|| "GraphQL query failed".into()));
        }
    }
    let config = envelope
        .data
        .and_then(|d| d.ai_credit_configuration)
        .ok_or_else(|| "GraphQL response missing aiCreditConfiguration".to_string())?;
    let bps = config
        .oracle_markup_bps
        .ok_or_else(|| "GraphQL response missing oracleMarkupBps".to_string())?;
    validate_oracle_markup_bps(bps)
}

pub fn validate_oracle_markup_bps(bps: i64) -> Result<u64, String> {
    if bps < 0 {
        return Err(format!("oracle_markup_bps must be non-negative, got {bps}"));
    }
    let bps = bps as u64;
    if bps > MAX_ORACLE_MARKUP_BPS {
        return Err(format!(
            "oracle_markup_bps exceeds max {MAX_ORACLE_MARKUP_BPS}, got {bps}"
        ));
    }
    Ok(bps)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_happy_path() {
        let body = r#"{"data":{"aiCreditConfiguration":{"oracleMarkupBps":1500}}}"#;
        assert_eq!(
            parse_oracle_markup_bps_from_graphql_body(body).unwrap(),
            1500
        );
    }

    #[test]
    fn parse_graphql_errors() {
        let body = r#"{"errors":[{"message":"field not found"}]}"#;
        let err = parse_oracle_markup_bps_from_graphql_body(body).unwrap_err();
        assert!(err.contains("field not found"));
    }

    #[test]
    fn parse_missing_configuration() {
        let body = r#"{"data":{}}"#;
        let err = parse_oracle_markup_bps_from_graphql_body(body).unwrap_err();
        assert!(err.contains("missing aiCreditConfiguration"));
    }

    #[test]
    fn validate_rejects_negative_and_overflow() {
        assert!(validate_oracle_markup_bps(-1).is_err());
        assert!(validate_oracle_markup_bps(10_001).is_err());
        assert_eq!(validate_oracle_markup_bps(0).unwrap(), 0);
        assert_eq!(validate_oracle_markup_bps(1500).unwrap(), 1500);
    }
}
