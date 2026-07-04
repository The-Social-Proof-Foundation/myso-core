// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;

use crate::config::OracleArgs;
use crate::graphql_client::MarkupConfigClient;
use crate::pricing::{PricingEngine, DEFAULT_ORACLE_MARKUP_BPS};

pub async fn refresh_markup_once(
    pricing: &Arc<RwLock<PricingEngine>>,
    client: &MarkupConfigClient,
) -> Result<u64, String> {
    let bps = client.fetch_oracle_markup_bps().await?;
    let mut engine = pricing.write().await;
    engine.set_oracle_markup_bps(bps);
    Ok(bps)
}

async fn handle_markup_refresh_failure(
    pricing: &Arc<RwLock<PricingEngine>>,
    args: &OracleArgs,
    err: &str,
    phase: &str,
) {
    let mut engine = pricing.write().await;
    if engine.markup_ever_fetched() {
        tracing::warn!(
            error = %err,
            phase,
            oracle_markup_bps = engine.oracle_markup_bps(),
            "oracle markup refresh failed; keeping last good value"
        );
    } else if !args.markup_graphql_enabled {
        engine.set_ecosystem_margin_pct(args.ecosystem_margin_pct);
        tracing::warn!(
            error = %err,
            phase,
            margin_pct = engine.ecosystem_margin_pct(),
            "markup refresh failed with GraphQL disabled; using AI_CREDIT_ECOSYSTEM_MARGIN_PCT"
        );
    } else {
        engine.apply_fallback_markup();
        tracing::warn!(
            error = %err,
            phase,
            oracle_markup_bps = DEFAULT_ORACLE_MARKUP_BPS,
            "oracle markup refresh failed; using default fallback"
        );
    }
}

pub fn spawn_markup_refresh_worker(
    args: Arc<OracleArgs>,
    pricing: Arc<RwLock<PricingEngine>>,
    client: MarkupConfigClient,
) {
    if !args.markup_graphql_enabled {
        tracing::info!(
            "oracle markup GraphQL refresh disabled (AI_CREDIT_MARKUP_GRAPHQL_ENABLED=false)"
        );
        return;
    }

    let interval_secs = args.markup_refresh_interval_secs;
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));
        loop {
            ticker.tick().await;
            match refresh_markup_once(&pricing, &client).await {
                Ok(bps) => {
                    let margin_pct = bps as f64 / 10_000.0;
                    tracing::info!(
                        oracle_markup_bps = bps,
                        margin_pct,
                        url = %client.graphql_url(),
                        "refreshed oracle markup"
                    );
                }
                Err(err) => {
                    handle_markup_refresh_failure(&pricing, &args, &err, "interval").await;
                }
            }
        }
    });
}

pub async fn startup_markup_refresh(
    args: &OracleArgs,
    pricing: &Arc<RwLock<PricingEngine>>,
    client: &MarkupConfigClient,
) {
    if !args.markup_graphql_enabled {
        let mut engine = pricing.write().await;
        engine.set_ecosystem_margin_pct(args.ecosystem_margin_pct);
        tracing::info!(
            margin_pct = engine.ecosystem_margin_pct(),
            "oracle markup using AI_CREDIT_ECOSYSTEM_MARGIN_PCT (GraphQL disabled)"
        );
        return;
    }

    match refresh_markup_once(pricing, client).await {
        Ok(bps) => {
            let margin_pct = bps as f64 / 10_000.0;
            tracing::info!(
                oracle_markup_bps = bps,
                margin_pct,
                url = %client.graphql_url(),
                "initial oracle markup loaded"
            );
        }
        Err(err) => {
            handle_markup_refresh_failure(pricing, args, &err, "startup").await;
        }
    }
}
