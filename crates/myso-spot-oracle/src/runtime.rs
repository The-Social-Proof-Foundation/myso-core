// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use myso_futures::service::Service;
use myso_indexer_alt_metrics::{MetricsArgs, MetricsService};
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::api;
use crate::api::AppState;
use crate::config::OracleArgs;
use crate::metrics::OracleMetrics;
use crate::store::OracleStore;

pub async fn serve(args: OracleArgs) -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(args.db_max_connections)
        .connect(&args.database_url)
        .await?;
    myso_discovery_service_schema::run_migrations(&pool).await?;
    myso_spot_oracle_schema::run_migrations(&pool).await?;
    info!("spot_oracle DB migrations complete (discovery + spot schemas)");

    let store = Arc::new(OracleStore::new(pool));
    let metrics = Arc::new(OracleMetrics::build());
    let cancel = CancellationToken::new();
    let workers_enabled = args.enabled;
    let args = Arc::new(args);

    let sources = myso_discovery_service_core::sources::config_loader::load_sources_config(
        &args.sources_config,
    )?;
    store.upsert_source_rows(&sources).await?;
    info!(sources = sources.len(), "registered discovery sources");

    let enabled_rows = store.list_enabled_sources().await?;
    let default_registry = crate::sources::build_default_registry();
    let registry = crate::sources::build_registry_from_sources(&default_registry, &enabled_rows);
    info!(
        enabled = registry.len(),
        total_impls = default_registry.len(),
        "resolver registry built"
    );

    let state = Arc::new(AppState {
        store: store.clone(),
        args: args.clone(),
        metrics: metrics.clone(),
        sources: Arc::new(registry),
        cancel: cancel.clone(),
    });

    let metrics_service = MetricsService::new(
        MetricsArgs {
            metrics_address: args.metrics_addr,
        },
        metrics.registry.clone(),
    )
    .run()
    .await?;

    let listener = tokio::net::TcpListener::bind(&args.listen_addr).await?;
    info!(listen = %args.listen_addr, "myso-spot-oracle listening");

    let (stx, srx) = tokio::sync::oneshot::channel::<()>();
    let cancel_for_shutdown = cancel.clone();
    let state_for_api = state.clone();
    let state_for_workers = state.clone();
    let cancel_for_workers = cancel.clone();

    let mut service = Service::new()
        .attach(metrics_service)
        .with_shutdown_signal(async move {
            cancel_for_shutdown.cancel();
        })
        .with_shutdown_signal(async move {
            let _ = stx.send(());
        })
        .spawn(async move {
            axum::serve(listener, api::router(state_for_api))
                .with_graceful_shutdown(async move {
                    let _ = srx.await;
                })
                .await?;
            Ok(())
        });

    if workers_enabled {
        service = service.merge(spawn_workers(state_for_workers, cancel_for_workers));
    }

    service.main().await?;
    Ok(())
}

fn spawn_workers(state: Arc<AppState>, cancel: CancellationToken) -> Service {
    Service::new()
        .spawn(run_review_worker(state.clone(), cancel.clone()))
        .spawn(run_scheduler_worker(state.clone(), cancel.clone()))
        .spawn(run_rss_watcher(state.clone(), cancel.clone()))
        .spawn(run_reconcile_worker(state.clone(), cancel.clone()))
}

async fn run_review_worker(state: Arc<AppState>, cancel: CancellationToken) -> anyhow::Result<()> {
    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(
        state.args.review_poll_interval_secs,
    ));
    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            _ = ticker.tick() => {
                if let Err(err) = crate::review::ingest::run_post_poller(state.clone()).await {
                    tracing::warn!(error = %err, "post poller failed");
                }
            }
        }
    }
    Ok(())
}

async fn run_scheduler_worker(state: Arc<AppState>, cancel: CancellationToken) -> anyhow::Result<()> {
    let state_clone = state.clone();
    let cancel_clone = cancel.clone();
    tokio::spawn(async move {
        if let Err(err) = crate::scheduler::run_scheduler_loop(state_clone).await {
            tracing::error!(error = %err, "scheduler loop exited");
        }
        cancel_clone.cancel();
    });
    cancel.cancelled().await;
    Ok(())
}

async fn run_rss_watcher(state: Arc<AppState>, cancel: CancellationToken) -> anyhow::Result<()> {
    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(
        state.args.rss_poll_interval_secs,
    ));
    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            _ = ticker.tick() => {
                if let Err(err) = crate::rss::run_rss_watcher_loop(state.clone()).await {
                    tracing::warn!(error = %err, "rss watcher failed");
                }
            }
        }
    }
    Ok(())
}

async fn run_reconcile_worker(
    state: Arc<AppState>,
    cancel: CancellationToken,
) -> anyhow::Result<()> {
    let mut ticker = tokio::time::interval(std::time::Duration::from_secs(
        state.args.reconcile_interval_secs,
    ));
    loop {
        tokio::select! {
            _ = cancel.cancelled() => break,
            _ = ticker.tick() => {
                if let Err(err) = crate::jobs::run_reconcile_loop(state.clone()).await {
                    tracing::warn!(error = %err, "reconcile failed");
                }
            }
        }
    }
    Ok(())
}
