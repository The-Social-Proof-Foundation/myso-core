// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Factual query API (`/v1/*`) for SPoT and other Discovery consumers.

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::Json;
use chrono::Utc;
use myso_discovery_service_core::api::{
    FetchProvenance, NormalizedEvent, NormalizedPrice, NormalizedRelease, PriceQuery,
    RefreshRequest, ReleaseQuery, SourceHealthResponse, SourceSummary,
};
use myso_discovery_service_core::sources::http_client::HttpFetchClient;
use myso_discovery_service_core::sources::SourceConfig;
use serde::Deserialize;

use crate::cache;
use crate::runtime::AppState;
use crate::store::DiscoverySourceRow;

const CLIENT_SECRET_HEADER: &str = "x-discovery-client-secret";

fn check_client_auth(state: &AppState, headers: &HeaderMap) -> Result<(), StatusCode> {
    let Some(expected) = &state.args.client_secret else {
        return Ok(());
    };
    let provided = headers
        .get(CLIENT_SECRET_HEADER)
        .and_then(|v| v.to_str().ok());
    if provided == Some(expected.as_str()) {
        Ok(())
    } else {
        Err(StatusCode::UNAUTHORIZED)
    }
}

pub async fn list_sources(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<SourceSummary>>, StatusCode> {
    check_client_auth(&state, &headers)?;
    let mut out = Vec::new();
    for cfg in &state.sources {
        if !cfg.enabled {
            continue;
        }
        let health = if let Some(adapter) = state.registry.get(&cfg.adapter_type) {
            adapter.health().await
        } else {
            myso_discovery_service_core::sources::SourceHealth {
                healthy: false,
                message: "not registered".into(),
            }
        };
        out.push(SourceSummary {
            id: cfg.id.clone(),
            adapter_type: cfg.adapter_type.clone(),
            domain: cfg.domain.as_str().to_string(),
            trust_score: cfg.trust_score,
            enabled: cfg.enabled,
            content_kind: cfg
                .content_kind
                .map(|k| k.as_str().to_string())
                .unwrap_or_else(|| "text".to_string()),
            health_healthy: Some(health.healthy),
        });
    }
    Ok(Json(out))
}

pub async fn all_sources_health(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
) -> Result<Json<Vec<SourceHealthResponse>>, StatusCode> {
    check_client_auth(&state, &headers)?;
    let mut out = Vec::new();
    for cfg in &state.sources {
        if !cfg.enabled {
            continue;
        }
        let health = if let Some(adapter) = state.registry.get(&cfg.adapter_type) {
            adapter.health().await
        } else {
            myso_discovery_service_core::sources::SourceHealth {
                healthy: false,
                message: "adapter not registered".into(),
            }
        };
        out.push(SourceHealthResponse {
            source_id: cfg.id.clone(),
            healthy: health.healthy,
            message: health.message,
        });
    }
    Ok(Json(out))
}

pub async fn source_health(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    axum::extract::Path(source_id): axum::extract::Path<String>,
) -> Result<Json<SourceHealthResponse>, StatusCode> {
    check_client_auth(&state, &headers)?;
    let cfg = find_source(&state.sources, &source_id).ok_or(StatusCode::NOT_FOUND)?;
    let health = if let Some(adapter) = state.registry.get(&cfg.adapter_type) {
        adapter.health().await
    } else {
        myso_discovery_service_core::sources::SourceHealth {
            healthy: false,
            message: "adapter not registered".into(),
        }
    };
    Ok(Json(SourceHealthResponse {
        source_id,
        healthy: health.healthy,
        message: health.message,
    }))
}

#[derive(Debug, Deserialize)]
pub struct PriceQueryParams {
    pub asset: String,
    pub quote: String,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub refresh: Option<bool>,
}

pub async fn get_price(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<PriceQueryParams>,
) -> Result<Json<NormalizedPrice>, StatusCode> {
    check_client_auth(&state, &headers)?;
    let refresh = params.refresh.unwrap_or(false);
    let source_id = params
        .source_id
        .clone()
        .unwrap_or_else(|| "coingecko-simple-price".to_string());
    let cache_key = cache::price_cache_key(&source_id, &params.asset, &params.quote);

    if !refresh {
        if let Ok(Some(entry)) = cache::get(state.store.pool(), &cache_key).await {
            state.metrics.cache_hits_total.inc();
            if let Ok(price) = parse_cached_price(&params.asset, &params.quote, &source_id, &entry, true) {
                return Ok(Json(price));
            }
        }
    }

    state.rate_limiter.throttle(&source_id).await;
    let cfg = find_source(&state.sources, &source_id).ok_or(StatusCode::NOT_FOUND)?;
    let (url, payload, content_hash) = fetch_price_payload(&cfg, &params.asset, &params.quote)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let price = extract_price(&payload, &params.asset, &params.quote)
        .map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;

    let normalized = NormalizedPrice {
        asset: params.asset.clone(),
        quote: params.quote.clone(),
        price,
        observed_at: Utc::now(),
        provenance: FetchProvenance {
            source_id: source_id.clone(),
            source_url: url,
            content_hash,
            fetched_at: Utc::now(),
            cache_hit: false,
        },
    };

    let _ = cache::put(
        state.store.pool(),
        &cache_key,
        Some(crate::store::uuid_v5_named(&source_id)),
        "price",
        &normalized.provenance.source_url,
        &normalized.provenance.content_hash,
        &serde_json::to_value(&normalized).unwrap_or_default(),
        state.args.cache_ttl_secs,
    )
    .await;

    state.metrics.cache_misses_total.inc();
    Ok(Json(normalized))
}

#[derive(Debug, Deserialize)]
pub struct ReleaseQueryParams {
    pub owner: String,
    pub repo: String,
    #[serde(default)]
    pub tag: Option<String>,
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub refresh: Option<bool>,
}

pub async fn get_release(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<ReleaseQueryParams>,
) -> Result<Json<NormalizedRelease>, StatusCode> {
    check_client_auth(&state, &headers)?;
    let refresh = params.refresh.unwrap_or(false);
    let source_id = params
        .source_id
        .clone()
        .unwrap_or_else(|| format!("{}-releases", params.repo));
    let cache_key = cache::release_cache_key(&source_id, &params.owner, &params.repo);

    if !refresh {
        if let Ok(Some(entry)) = cache::get(state.store.pool(), &cache_key).await {
            state.metrics.cache_hits_total.inc();
            if let Ok(release) = parse_cached_release(&params.owner, &params.repo, &source_id, &entry, true) {
                return Ok(Json(release));
            }
        }
    }

    state.rate_limiter.throttle(&source_id).await;
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        params.owner, params.repo
    );
    let client = HttpFetchClient::new();
    let fetched = client.get_text(&url).await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let payload: serde_json::Value =
        serde_json::from_str(&fetched.body).map_err(|_| StatusCode::UNPROCESSABLE_ENTITY)?;
    let tag = payload
        .get("tag_name")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let release = NormalizedRelease {
        owner: params.owner.clone(),
        repo: params.repo.clone(),
        tag,
        published_at: None,
        provenance: FetchProvenance {
            source_id: source_id.clone(),
            source_url: url,
            content_hash: fetched.content_hash,
            fetched_at: Utc::now(),
            cache_hit: false,
        },
    };

    let _ = cache::put(
        state.store.pool(),
        &cache_key,
        None,
        "release",
        &release.provenance.source_url,
        &release.provenance.content_hash,
        &serde_json::to_value(&release).unwrap_or_default(),
        state.args.cache_ttl_secs,
    )
    .await;

    state.metrics.cache_misses_total.inc();
    Ok(Json(release))
}

#[derive(Debug, Deserialize)]
pub struct EventsQueryParams {
    #[serde(default)]
    pub source_id: Option<String>,
    #[serde(default)]
    pub feed: Option<String>,
    #[serde(default)]
    pub refresh: Option<bool>,
}

pub async fn get_events(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Query(params): Query<EventsQueryParams>,
) -> Result<Json<Vec<NormalizedEvent>>, StatusCode> {
    check_client_auth(&state, &headers)?;
    let source_id = params
        .source_id
        .clone()
        .unwrap_or_else(|| "coindesk-rss".to_string());
    let feed = params
        .feed
        .clone()
        .or_else(|| find_source(&state.sources, &source_id).and_then(first_feed_url))
        .ok_or(StatusCode::BAD_REQUEST)?;
    let refresh = params.refresh.unwrap_or(false);
    let cache_key = cache::events_cache_key(&source_id, &feed);

    if !refresh {
        if let Ok(Some(entry)) = cache::get(state.store.pool(), &cache_key).await {
            state.metrics.cache_hits_total.inc();
            if let Ok(events) = parse_cached_events(&entry, true) {
                return Ok(Json(events));
            }
        }
    }

    state.rate_limiter.throttle(&source_id).await;
    let client = HttpFetchClient::new();
    let fetched = client.get_text(&feed).await.map_err(|_| StatusCode::BAD_GATEWAY)?;

    let events = vec![NormalizedEvent {
        title: "feed snapshot".to_string(),
        url: feed.clone(),
        published_at: Some(Utc::now()),
        summary: Some(format!("{} bytes", fetched.body.len())),
        provenance: FetchProvenance {
            source_id: source_id.clone(),
            source_url: feed.clone(),
            content_hash: fetched.content_hash,
            fetched_at: Utc::now(),
            cache_hit: false,
        },
    }];

    let _ = cache::put(
        state.store.pool(),
        &cache_key,
        None,
        "events",
        &feed,
        &events[0].provenance.content_hash,
        &serde_json::to_value(&events).unwrap_or_default(),
        state.args.cache_ttl_secs,
    )
    .await;

    state.metrics.cache_misses_total.inc();
    Ok(Json(events))
}

pub async fn refresh_source(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<RefreshRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    check_client_auth(&state, &headers)?;
    let cfg = find_source(&state.sources, &body.source_id).ok_or(StatusCode::NOT_FOUND)?;
    let adapter = state
        .registry
        .get(&cfg.adapter_type)
        .ok_or(StatusCode::NOT_FOUND)?;
    state.rate_limiter.throttle(&body.source_id).await;
    let records = adapter
        .discover(cfg)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;
    state.metrics.refresh_total.inc();
    Ok(Json(serde_json::json!({
        "source_id": body.source_id,
        "records": records.len(),
        "refreshed_at": Utc::now(),
    })))
}

fn find_source<'a>(sources: &'a [SourceConfig], id: &str) -> Option<&'a SourceConfig> {
    sources.iter().find(|s| s.id == id)
}

fn first_feed_url(cfg: &SourceConfig) -> Option<String> {
    cfg.config.feed_urls.first().cloned()
}

async fn fetch_price_payload(
    cfg: &SourceConfig,
    asset: &str,
    quote: &str,
) -> anyhow::Result<(String, serde_json::Value, String)> {
    let client = HttpFetchClient::new();
    let url = if cfg.config.api_base_url.is_some() {
        if cfg.config.poll_path.as_deref().is_some_and(|p| p.contains("coingecko")) {
            format!(
                "https://api.coingecko.com/api/v3/simple/price?ids={asset}&vs_currencies={quote}"
            )
        } else {
            let base = cfg.config.api_base_url.as_deref().unwrap_or("");
            let path = cfg.config.poll_path.as_deref().unwrap_or("");
            format!("{base}{path}")
        }
    } else {
        format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={asset}&vs_currencies={quote}"
        )
    };
    let fetched = client.get_text(&url).await?;
    let payload: serde_json::Value = serde_json::from_str(&fetched.body)?;
    Ok((url, payload, fetched.content_hash))
}

fn extract_price(payload: &serde_json::Value, asset: &str, quote: &str) -> anyhow::Result<f64> {
    if let Some(v) = payload
        .get(asset)
        .and_then(|a| a.get(quote))
        .and_then(|v| v.as_f64())
    {
        return Ok(v);
    }
    if let Some(v) = payload
        .get("data")
        .and_then(|d| d.get("amount"))
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
    {
        return Ok(v);
    }
    anyhow::bail!("price not found in payload")
}

fn parse_cached_price(
    asset: &str,
    quote: &str,
    source_id: &str,
    entry: &cache::CacheEntry,
    cache_hit: bool,
) -> anyhow::Result<NormalizedPrice> {
    if let Ok(p) = serde_json::from_value::<NormalizedPrice>(entry.normalized_payload.clone()) {
        return Ok(NormalizedPrice {
            provenance: FetchProvenance {
                cache_hit,
                ..p.provenance
            },
            ..p
        });
    }
    let price = extract_price(&entry.normalized_payload, asset, quote)?;
    Ok(NormalizedPrice {
        asset: asset.to_string(),
        quote: quote.to_string(),
        price,
        observed_at: entry.fetched_at,
        provenance: FetchProvenance {
            source_id: source_id.to_string(),
            source_url: entry.source_url.clone(),
            content_hash: entry.content_hash.clone(),
            fetched_at: entry.fetched_at,
            cache_hit,
        },
    })
}

fn parse_cached_release(
    owner: &str,
    repo: &str,
    source_id: &str,
    entry: &cache::CacheEntry,
    cache_hit: bool,
) -> anyhow::Result<NormalizedRelease> {
    if let Ok(r) = serde_json::from_value::<NormalizedRelease>(entry.normalized_payload.clone()) {
        return Ok(NormalizedRelease {
            provenance: FetchProvenance {
                cache_hit,
                ..r.provenance
            },
            ..r
        });
    }
    let tag = entry
        .normalized_payload
        .get("tag")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();
    Ok(NormalizedRelease {
        owner: owner.to_string(),
        repo: repo.to_string(),
        tag,
        published_at: None,
        provenance: FetchProvenance {
            source_id: source_id.to_string(),
            source_url: entry.source_url.clone(),
            content_hash: entry.content_hash.clone(),
            fetched_at: entry.fetched_at,
            cache_hit,
        },
    })
}

fn parse_cached_events(entry: &cache::CacheEntry, cache_hit: bool) -> anyhow::Result<Vec<NormalizedEvent>> {
    let mut events: Vec<NormalizedEvent> =
        serde_json::from_value(entry.normalized_payload.clone())?;
    for ev in &mut events {
        ev.provenance.cache_hit = cache_hit;
    }
    Ok(events)
}

// Silence unused import warning for API trait types used in docs.
#[allow(dead_code)]
fn _api_types() {
    let _ = std::any::type_name::<PriceQuery>();
    let _ = std::any::type_name::<ReleaseQuery>();
    let _ = std::any::type_name::<DiscoverySourceRow>();
}
