// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;

use crate::catalog::{CatalogSyncReport, PricingCatalog};
use crate::config::OracleArgs;
use crate::openrouter_client::OpenRouterClient;
use crate::pricing::PricingEngine;

pub async fn sync_catalog_once(
    args: &OracleArgs,
    catalog_path: &Path,
    catalog: &Arc<RwLock<PricingCatalog>>,
    pricing: &Arc<RwLock<PricingEngine>>,
    openrouter: &OpenRouterClient,
) -> Result<CatalogSyncReport, String> {
    let remote = openrouter
        .fetch_model_rates()
        .await
        .map_err(|e| e.to_string())?;

    let mut cat = catalog.write().await;
    let report = cat.apply_openrouter_rates(&remote, args.catalog_max_drift_pct);

    if report.updated > 0 {
        cat.save(catalog_path).map_err(|e| e.to_string())?;
        tracing::info!(
            checked = report.checked,
            updated = report.updated,
            skipped_drift = report.skipped_drift,
            unchanged = report.unchanged,
            version = %cat.version,
            "applied OpenRouter catalog rate updates"
        );
        let updated = cat.clone();
        drop(cat);
        pricing.write().await.replace_catalog(updated);
    } else {
        tracing::debug!(
            checked = report.checked,
            skipped_drift = report.skipped_drift,
            unchanged = report.unchanged,
            "OpenRouter catalog sync: no rate changes applied"
        );
    }

    Ok(report)
}

pub async fn startup_catalog_sync(
    args: &OracleArgs,
    catalog_path: &Path,
    catalog: Arc<RwLock<PricingCatalog>>,
    pricing: Arc<RwLock<PricingEngine>>,
    openrouter: OpenRouterClient,
) {
    if !args.catalog_sync_active() {
        if args.catalog_sync_enabled && args.openrouter_api_key.is_none() {
            tracing::error!(
                "AI_CREDIT_CATALOG_SYNC_ENABLED=true but AI_CREDIT_OPENROUTER_API_KEY is unset"
            );
        }
        return;
    }

    if !args.catalog_sync_on_startup {
        tracing::info!("OpenRouter catalog sync on startup disabled");
        return;
    }

    match sync_catalog_once(args, catalog_path, &catalog, &pricing, &openrouter).await {
        Ok(report) => {
            tracing::info!(
                checked = report.checked,
                updated = report.updated,
                "startup OpenRouter catalog sync complete"
            );
        }
        Err(err) => {
            tracing::warn!(
                error = %err,
                "startup OpenRouter catalog sync failed; keeping loaded catalog"
            );
        }
    }
}

pub fn spawn_catalog_sync_worker(
    args: Arc<OracleArgs>,
    catalog_path: PathBuf,
    catalog: Arc<RwLock<PricingCatalog>>,
    pricing: Arc<RwLock<PricingEngine>>,
    openrouter: OpenRouterClient,
) {
    if !args.catalog_sync_active() {
        return;
    }

    let interval_secs = args.catalog_sync_interval_secs;
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(Duration::from_secs(interval_secs));
        ticker.tick().await;
        loop {
            ticker.tick().await;
            match sync_catalog_once(
                args.as_ref(),
                catalog_path.as_path(),
                &catalog,
                &pricing,
                &openrouter,
            )
            .await
            {
                Ok(report) if report.updated > 0 => {
                    tracing::info!(
                        checked = report.checked,
                        updated = report.updated,
                        "periodic OpenRouter catalog sync applied updates"
                    );
                }
                Ok(_) => {}
                Err(err) => {
                    tracing::warn!(
                        error = %err,
                        "periodic OpenRouter catalog sync failed; keeping last good catalog"
                    );
                }
            }
        }
    });
}
