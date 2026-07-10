// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Direct trusted-source fetch helpers (no Discovery proxy).

use chrono::Utc;

use crate::sources::http_fetch::HttpFetchClient;
use crate::sources::rate_limit::RateLimiter;
use crate::sources::SourceEvidence;

static RATE_LIMITER: std::sync::OnceLock<RateLimiter> = std::sync::OnceLock::new();

fn rate_limiter() -> &'static RateLimiter {
    RATE_LIMITER.get_or_init(RateLimiter::new)
}

fn client() -> HttpFetchClient {
    HttpFetchClient::new()
}

/// Fetch a CoinGecko-style price and return evidence with `{asset: {quote: price}}` payload.
pub async fn fetch_coingecko_price(
    adapter_id: &str,
    asset: &str,
    quote: &str,
) -> anyhow::Result<SourceEvidence> {
    rate_limiter().throttle(adapter_id).await;
    let url = format!(
        "https://api.coingecko.com/api/v3/simple/price?ids={asset}&vs_currencies={quote}"
    );
    let fetched = client().get_text(&url).await?;
    let payload: serde_json::Value = serde_json::from_str(&fetched.body)?;
    Ok(SourceEvidence {
        adapter_id: adapter_id.to_string(),
        source_url: url,
        content_hash: fetched.content_hash,
        raw_response: Some(fetched.body),
        fetched_at: Utc::now(),
        payload,
    })
}

/// Fetch Coinbase spot price; normalize to `{asset: {quote: price}}`.
pub async fn fetch_coinbase_price(
    adapter_id: &str,
    asset: &str,
    quote: &str,
) -> anyhow::Result<SourceEvidence> {
    rate_limiter().throttle(adapter_id).await;
    let pair = format!("{}-{}", asset.to_uppercase(), quote.to_uppercase());
    let url = format!("https://api.coinbase.com/v2/prices/{pair}/spot");
    let fetched = client().get_text(&url).await?;
    let raw: serde_json::Value = serde_json::from_str(&fetched.body)?;
    let amount = raw
        .get("data")
        .and_then(|d| d.get("amount"))
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse::<f64>().ok())
        .ok_or_else(|| anyhow::anyhow!("coinbase: missing data.amount"))?;
    let mut inner = serde_json::Map::new();
    inner.insert(quote.to_string(), serde_json::json!(amount));
    let mut outer = serde_json::Map::new();
    outer.insert(asset.to_string(), serde_json::Value::Object(inner));
    Ok(SourceEvidence {
        adapter_id: adapter_id.to_string(),
        source_url: url,
        content_hash: fetched.content_hash,
        raw_response: Some(fetched.body),
        fetched_at: Utc::now(),
        payload: serde_json::Value::Object(outer),
    })
}

/// Fetch GitHub latest release.
pub async fn fetch_github_release(
    adapter_id: &str,
    owner: &str,
    repo: &str,
) -> anyhow::Result<SourceEvidence> {
    rate_limiter().throttle(adapter_id).await;
    let url = format!("https://api.github.com/repos/{owner}/{repo}/releases/latest");
    let fetched = client().get_text(&url).await?;
    let payload: serde_json::Value = serde_json::from_str(&fetched.body)?;
    let tag = payload
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    Ok(SourceEvidence {
        adapter_id: adapter_id.to_string(),
        source_url: url,
        content_hash: fetched.content_hash,
        raw_response: Some(fetched.body),
        fetched_at: Utc::now(),
        payload: serde_json::json!({
            "tag_name": tag,
            "owner": owner,
            "repo": repo,
        }),
    })
}

/// Fetch an RSS/Atom feed snapshot as event evidence.
pub async fn fetch_rss_events(
    adapter_id: &str,
    feed_url: &str,
) -> anyhow::Result<SourceEvidence> {
    rate_limiter().throttle(adapter_id).await;
    let fetched = client().get_text(feed_url).await?;
    let events = vec![serde_json::json!({
        "title": "feed snapshot",
        "url": feed_url,
        "summary": format!("{} bytes", fetched.body.len()),
    })];
    Ok(SourceEvidence {
        adapter_id: adapter_id.to_string(),
        source_url: feed_url.to_string(),
        content_hash: fetched.content_hash,
        raw_response: Some(fetched.body),
        fetched_at: Utc::now(),
        payload: serde_json::Value::Array(events),
    })
}

/// Fetch an arbitrary HTTP JSON endpoint and return the raw JSON as payload.
pub async fn fetch_http_json(
    adapter_id: &str,
    url: &str,
) -> anyhow::Result<SourceEvidence> {
    rate_limiter().throttle(adapter_id).await;
    let fetched = client().get_text(url).await?;
    let payload: serde_json::Value =
        serde_json::from_str(&fetched.body).unwrap_or(serde_json::Value::Null);
    Ok(SourceEvidence {
        adapter_id: adapter_id.to_string(),
        source_url: url.to_string(),
        content_hash: fetched.content_hash,
        raw_response: Some(fetched.body),
        fetched_at: Utc::now(),
        payload,
    })
}
