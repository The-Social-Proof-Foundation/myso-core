// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;

use crate::config::OracleArgs;
use crate::myso_price_client::MysoPriceClient;
use crate::pricing::{PricingEngine, DEFAULT_MYSO_USD_FALLBACK};

pub async fn refresh_pricing_once(
    pricing: &Arc<RwLock<PricingEngine>>,
    client: &MysoPriceClient,
) -> Result<f64, String> {
    let snapshot = client.fetch_latest().await.map_err(|e| e.to_string())?;
    let mut engine = pricing.write().await;
    engine.set_myso_usd(snapshot.usd, snapshot.fetched_at);
    Ok(snapshot.usd)
}

async fn handle_price_refresh_failure(
    pricing: &Arc<RwLock<PricingEngine>>,
    err: &str,
    phase: &str,
) {
    let mut engine = pricing.write().await;
    if engine.price_ever_fetched() {
        tracing::warn!(
            error = %err,
            phase,
            "MYSO/USD price refresh failed; keeping last good price"
        );
    } else {
        engine.apply_fallback_myso_usd();
        tracing::warn!(
            error = %err,
            phase,
            myso_usd = DEFAULT_MYSO_USD_FALLBACK,
            "MYSO/USD price refresh failed; using hardcoded fallback"
        );
    }
}

pub fn spawn_price_refresh_worker(
    args: Arc<OracleArgs>,
    pricing: Arc<RwLock<PricingEngine>>,
    client: MysoPriceClient,
) {
    if !args.myso_price_enabled {
        tracing::info!("MYSO/USD price refresh disabled (AI_CREDIT_MYSO_PRICE_ENABLED=false)");
        return;
    }

    let interval_secs = args.price_refresh_interval_secs;
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));
        loop {
            ticker.tick().await;
            match refresh_pricing_once(&pricing, &client).await {
                Ok(usd) => {
                    tracing::info!(
                        myso_usd = usd,
                        url = %client.base_url(),
                        "refreshed MYSO/USD price"
                    );
                }
                Err(err) => {
                    handle_price_refresh_failure(&pricing, &err, "interval").await;
                }
            }
        }
    });
}

pub async fn startup_price_refresh(
    args: &OracleArgs,
    pricing: &Arc<RwLock<PricingEngine>>,
    client: &MysoPriceClient,
) {
    if !args.myso_price_enabled {
        return;
    }
    match refresh_pricing_once(pricing, client).await {
        Ok(usd) => {
            tracing::info!(
                myso_usd = usd,
                url = %client.base_url(),
                "initial MYSO/USD price loaded"
            );
        }
        Err(err) => {
            handle_price_refresh_failure(pricing, &err, "startup").await;
        }
    }
}
