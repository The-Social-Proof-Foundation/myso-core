// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Helpers for `TrustedSource` adapters to resolve via Discovery client when configured.

use std::sync::Arc;

use chrono::Utc;
use myso_discovery_service_core::api::DiscoveryClient;
use myso_discovery_service_core::api::{EventsQuery, PriceQuery, ReleaseQuery};

use crate::sources::SourceEvidence;

pub type SharedDiscoveryClient = Arc<dyn DiscoveryClient>;

#[derive(Clone, Default)]
pub struct DiscoveryResolveCtx {
    pub client: Option<SharedDiscoveryClient>,
}

impl DiscoveryResolveCtx {
    pub fn new(client: Option<SharedDiscoveryClient>) -> Self {
        Self { client }
    }

    pub fn uses_discovery(&self) -> bool {
        self.client.is_some()
    }
}

pub fn evidence_from_price(
    adapter_id: &str,
    price: myso_discovery_service_core::api::NormalizedPrice,
    raw: Option<String>,
) -> SourceEvidence {
    let mut inner = serde_json::Map::new();
    inner.insert(
        price.quote.clone(),
        serde_json::json!(price.price),
    );
    let mut outer = serde_json::Map::new();
    outer.insert(price.asset.clone(), serde_json::Value::Object(inner));
    SourceEvidence {
        adapter_id: adapter_id.to_string(),
        source_url: price.provenance.source_url,
        content_hash: price.provenance.content_hash,
        raw_response: raw,
        fetched_at: Utc::now(),
        payload: serde_json::Value::Object(outer),
    }
}

pub fn evidence_from_release(
    adapter_id: &str,
    release: myso_discovery_service_core::api::NormalizedRelease,
    raw: Option<String>,
) -> SourceEvidence {
    SourceEvidence {
        adapter_id: adapter_id.to_string(),
        source_url: release.provenance.source_url,
        content_hash: release.provenance.content_hash,
        raw_response: raw,
        fetched_at: Utc::now(),
        payload: serde_json::json!({
            "tag_name": release.tag,
            "owner": release.owner,
            "repo": release.repo,
        }),
    }
}

pub fn evidence_from_events(
    adapter_id: &str,
    events: Vec<myso_discovery_service_core::api::NormalizedEvent>,
) -> SourceEvidence {
    let first = events.first();
    SourceEvidence {
        adapter_id: adapter_id.to_string(),
        source_url: first
            .map(|e| e.provenance.source_url.clone())
            .unwrap_or_default(),
        content_hash: first
            .map(|e| e.provenance.content_hash.clone())
            .unwrap_or_default(),
        raw_response: None,
        fetched_at: Utc::now(),
        payload: serde_json::to_value(&events).unwrap_or_default(),
    }
}

pub async fn fetch_price(
    ctx: &DiscoveryResolveCtx,
    adapter_id: &str,
    source_id: &str,
    asset: &str,
    quote: &str,
) -> anyhow::Result<SourceEvidence> {
    let client = ctx
        .client
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("SPOT_ORACLE_DISCOVERY_CLIENT_URL required for factual settlement"))?;
    let price = client
        .get_price(&PriceQuery {
            asset: asset.to_string(),
            quote: quote.to_string(),
            source_id: Some(source_id.to_string()),
            refresh: true,
        })
        .await?;
    Ok(evidence_from_price(adapter_id, price, None))
}

pub async fn fetch_release(
    ctx: &DiscoveryResolveCtx,
    adapter_id: &str,
    source_id: &str,
    owner: &str,
    repo: &str,
) -> anyhow::Result<SourceEvidence> {
    let client = ctx
        .client
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("SPOT_ORACLE_DISCOVERY_CLIENT_URL required for factual settlement"))?;
    let release = client
        .get_release(&ReleaseQuery {
            owner: owner.to_string(),
            repo: repo.to_string(),
            tag: None,
            source_id: Some(source_id.to_string()),
            refresh: true,
        })
        .await?;
    Ok(evidence_from_release(adapter_id, release, None))
}

pub async fn fetch_events(
    ctx: &DiscoveryResolveCtx,
    adapter_id: &str,
    source_id: &str,
    feed: &str,
) -> anyhow::Result<SourceEvidence> {
    let client = ctx
        .client
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("SPOT_ORACLE_DISCOVERY_CLIENT_URL required for factual settlement"))?;
    let events = client
        .get_events(&EventsQuery {
            source_id: Some(source_id.to_string()),
            feed: Some(feed.to_string()),
            since: None,
            query: None,
            refresh: true,
        })
        .await?;
    Ok(evidence_from_events(adapter_id, events))
}
