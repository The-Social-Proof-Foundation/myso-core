// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! HTTP client for the remote MySo price oracle monitoring API.

use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use serde::Deserialize;

pub const MIN_MYSO_USD: f64 = 1e-6;
pub const MAX_MYSO_USD: f64 = 1_000_000.0;

#[derive(Debug, Clone)]
pub struct MysoPriceSnapshot {
    pub usd: f64,
    pub fetched_at: Instant,
    /// Age reported by the price oracle, when available.
    pub oracle_age_secs: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct MysoPriceClient {
    base_url: String,
    http: reqwest::Client,
}

#[derive(Debug, Deserialize)]
struct ReadyResponse {
    last_price: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct StatusResponse {
    oracle_state: Option<StatusOracleState>,
}

#[derive(Debug, Deserialize)]
struct StatusOracleState {
    last_price: Option<serde_json::Value>,
    last_update_ago_seconds: Option<u64>,
}

impl MysoPriceClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into().trim_end_matches('/').to_string(),
            http: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("reqwest client"),
        }
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }

    pub async fn fetch_latest(&self) -> Result<MysoPriceSnapshot> {
        match self.fetch_ready().await {
            Ok(snapshot) => Ok(snapshot),
            Err(ready_err) => {
                tracing::debug!(error = %ready_err, "price oracle /ready failed, trying /status");
                self.fetch_status()
                    .await
                    .with_context(|| format!("/ready failed: {ready_err}"))
            }
        }
    }

    async fn fetch_ready(&self) -> Result<MysoPriceSnapshot> {
        let url = format!("{}/ready", self.base_url);
        let body: ReadyResponse = self
            .http
            .get(&url)
            .send()
            .await
            .context("price oracle /ready request")?
            .error_for_status()
            .context("price oracle /ready status")?
            .json()
            .await
            .context("price oracle /ready json")?;
        let usd = parse_price_value(body.last_price.as_ref())?;
        Ok(MysoPriceSnapshot {
            usd,
            fetched_at: Instant::now(),
            oracle_age_secs: None,
        })
    }

    async fn fetch_status(&self) -> Result<MysoPriceSnapshot> {
        let url = format!("{}/status", self.base_url);
        let body: StatusResponse = self
            .http
            .get(&url)
            .send()
            .await
            .context("price oracle /status request")?
            .error_for_status()
            .context("price oracle /status status")?
            .json()
            .await
            .context("price oracle /status json")?;
        let state = body
            .oracle_state
            .context("price oracle /status missing oracle_state")?;
        let usd = parse_price_value(state.last_price.as_ref())?;
        Ok(MysoPriceSnapshot {
            usd,
            fetched_at: Instant::now(),
            oracle_age_secs: state.last_update_ago_seconds,
        })
    }
}

pub fn validate_myso_usd(usd: f64) -> Result<f64> {
    anyhow::ensure!(
        usd.is_finite() && usd >= MIN_MYSO_USD && usd <= MAX_MYSO_USD,
        "myso_usd {usd} out of bounds [{MIN_MYSO_USD}, {MAX_MYSO_USD}]"
    );
    Ok(usd)
}

fn parse_price_value(value: Option<&serde_json::Value>) -> Result<f64> {
    let value = value.context("price oracle response missing last_price")?;
    let raw = match value {
        serde_json::Value::String(s) => s
            .parse::<f64>()
            .with_context(|| format!("invalid last_price string: {s}"))?,
        serde_json::Value::Number(n) => n
            .as_f64()
            .context("invalid last_price number")?,
        other => anyhow::bail!("unexpected last_price type: {other}"),
    };
    validate_myso_usd(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_ready_price_string() {
        let v = serde_json::json!({ "last_price": "0.0045" });
        let ready: ReadyResponse = serde_json::from_value(v).unwrap();
        let usd = parse_price_value(ready.last_price.as_ref()).unwrap();
        assert!((usd - 0.0045).abs() < f64::EPSILON);
    }

    #[test]
    fn parse_status_price_number() {
        let v = serde_json::json!({
            "oracle_state": {
                "last_price": 1.25,
                "last_update_ago_seconds": 42
            }
        });
        let status: StatusResponse = serde_json::from_value(v).unwrap();
        let state = status.oracle_state.unwrap();
        let usd = parse_price_value(state.last_price.as_ref()).unwrap();
        assert!((usd - 1.25).abs() < f64::EPSILON);
    }

    #[test]
    fn rejects_invalid_price() {
        assert!(validate_myso_usd(0.0).is_err());
        assert!(validate_myso_usd(-1.0).is_err());
    }
}
