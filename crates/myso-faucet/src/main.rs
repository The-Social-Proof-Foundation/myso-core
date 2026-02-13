// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use myso_config::myso_config_dir;
use myso_faucet::{AppState, create_wallet_context, start_faucet};
use myso_faucet::{FaucetConfig, SimpleFaucet};
use std::env;
use std::path::Path;
use std::sync::Arc;
use tracing::info;

const CONCURRENCY_LIMIT: usize = 30;
const PROM_PORT_ADDR: &str = "0.0.0.0:9184";
const INIT_RETRY_ATTEMPTS: u32 = 5;
const INIT_RETRY_DELAY_MS: u64 = 2000;

// Define the `GIT_REVISION` and `VERSION` consts
bin_version::bin_version!();

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // initialize tracing
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let config: FaucetConfig = FaucetConfig::parse();
    let FaucetConfig {
        wallet_client_timeout_secs,
        ref write_ahead_log,
        ..
    } = config;

    let config_dir = myso_config_dir()?;

    let max_concurrency = env::var("MAX_CONCURRENCY")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(CONCURRENCY_LIMIT);
    info!("Max concurrency: {max_concurrency}.");

    let prom_binding = PROM_PORT_ADDR
        .parse()
        .map_err(|e| anyhow::anyhow!("Invalid Prometheus port: {e}"))?;
    info!("Starting Prometheus HTTP endpoint at {}", prom_binding);
    let registry_service = mysten_metrics::start_prometheus_server(prom_binding);
    let prometheus_registry = registry_service.default_registry();
    prometheus_registry
        .register(mysten_metrics::uptime_metric("faucet", VERSION, "unknown"))
        .map_err(|e| anyhow::anyhow!("Failed to register Prometheus metrics: {e}"))?;

    let faucet = init_faucet_with_retry(
        wallet_client_timeout_secs,
        &config_dir,
        &prometheus_registry,
        write_ahead_log,
        config.clone(),
    )
    .await?;

    let app_state = Arc::new(AppState { faucet, config });

    start_faucet(app_state, max_concurrency, &prometheus_registry).await
}

async fn init_faucet_with_retry(
    wallet_client_timeout_secs: u64,
    config_dir: &Path,
    prometheus_registry: &prometheus::Registry,
    write_ahead_log: &Path,
    config: FaucetConfig,
) -> Result<Arc<SimpleFaucet>, anyhow::Error> {
    let mut last_error = None;
    for attempt in 1..=INIT_RETRY_ATTEMPTS {
        let context = create_wallet_context(wallet_client_timeout_secs, config_dir.to_path_buf())?;
        match SimpleFaucet::new(
            context,
            prometheus_registry,
            write_ahead_log,
            config.clone(),
        )
        .await
        {
            Ok(faucet) => return Ok(faucet),
            Err(e) => {
                last_error = Some(e.to_string());
                if attempt < INIT_RETRY_ATTEMPTS {
                    info!(
                        "Faucet init failed (attempt {}/{}): {}. Retrying in {}ms...",
                        attempt, INIT_RETRY_ATTEMPTS, e, INIT_RETRY_DELAY_MS
                    );
                    tokio::time::sleep(tokio::time::Duration::from_millis(INIT_RETRY_DELAY_MS))
                        .await;
                }
            }
        }
    }
    let err = last_error.unwrap_or_else(|| "Unknown error".to_string());
    Err(anyhow::anyhow!(
        "{}. \
        Ensure the fullnode RPC endpoint in client.yaml is correct and fully operational. \
        Simple HTTP connectivity checks may pass while RPC/API calls fail.",
        err
    ))
}
