// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use myso_futures::service::Service;
use myso_indexer_alt_metrics::{MetricsArgs, MetricsService};
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::admin;
use crate::api;
use crate::config::DiscoveryArgs;
use crate::embed_client::EmbedClient;
use crate::factual_api;
use crate::jobs::run_reconciliation_loop;
use crate::metrics::DiscoveryMetrics;
use crate::rate_limit::RateLimiter;
use crate::scheduler::{run_scheduler_loop, run_worker_loop};
use crate::sources::{build_default_registry, DiscoveryRegistry, SourceConfig};
use crate::store::DiscoveryStore;

pub struct AppState {
    pub store: Arc<DiscoveryStore>,
    pub args: Arc<DiscoveryArgs>,
    pub metrics: Arc<DiscoveryMetrics>,
    pub registry: Arc<DiscoveryRegistry>,
    pub sources: Vec<SourceConfig>,
    pub rate_limiter: Arc<RateLimiter>,
}

pub async fn serve(args: DiscoveryArgs) -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&args.database_url)
        .await?;
    myso_discovery_service_schema::run_migrations(&pool).await?;

    let store = Arc::new(DiscoveryStore::new(pool));
    let registry = Arc::new(build_default_registry());
    let metrics = Arc::new(DiscoveryMetrics::build());
    let rate_limiter = Arc::new(RateLimiter::new());
    let embed_client = Arc::new(EmbedClient::new(
        args.embed_endpoint.clone(),
        args.embed_secret.clone(),
    ));
    let args = Arc::new(args);

    // Real sources only — fail fast if config missing/empty (no silent manual_curated stub).
    let sources = myso_discovery_service_core::sources::config_loader::load_sources_config(
        &args.sources_config,
    )?;

    if args.bootstrap_only {
        let bootstrap_store = store.clone();
        let bootstrap_args = args.clone();
        let bootstrap_metrics = metrics.clone();
        crate::bootstrap::run_bootstrap(
            bootstrap_store,
            sources.clone(),
            bootstrap_args,
            bootstrap_metrics,
        )
        .await?;
        info!("bootstrap-only mode complete; exiting");
        return Ok(());
    }

    let cancel = CancellationToken::new();
    if args.enabled {
        let sched_store = store.clone();
        let sched_registry = registry.clone();
        let sched_sources = sources.clone();
        let poll = args.scheduler_poll_interval_secs;
        let embed_enabled = args.embed_enabled;
        let max_retries = args.max_retries;
        let sched_metrics = metrics.clone();
        let c = cancel.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = run_scheduler_loop(
                    sched_store,
                    sched_registry,
                    sched_sources,
                    poll,
                    embed_enabled,
                    max_retries,
                    sched_metrics,
                ) => {}
                _ = c.cancelled() => {}
            }
        });

        if args.embed_enabled {
            let worker_store = store.clone();
            let worker_embed = embed_client.clone();
            let worker_args = args.clone();
            let worker_concurrency = args.worker_concurrency;
            let worker_metrics = metrics.clone();
            let c = cancel.clone();
            tokio::spawn(async move {
                tokio::select! {
                    _ = run_worker_loop(
                        worker_store,
                        worker_embed,
                        worker_args,
                        worker_concurrency,
                        worker_metrics,
                    ) => {}
                    _ = c.cancelled() => {}
                }
            });
        } else {
            info!("embed worker disabled (DISCOVERY_EMBED_ENABLED=false); assets remain normalized without PoC indexing");
        }

        let recon_store = store.clone();
        let recon_metrics = metrics.clone();
        let c = cancel.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = run_reconciliation_loop(recon_store, recon_metrics) => {}
                _ = c.cancelled() => {}
            }
        });
    }

    let state = Arc::new(AppState {
        store: store.clone(),
        args: args.clone(),
        metrics: metrics.clone(),
        registry: registry.clone(),
        sources: sources.clone(),
        rate_limiter: rate_limiter.clone(),
    });

    let metrics_service = MetricsService::new(
        MetricsArgs {
            metrics_address: args.metrics_addr,
        },
        metrics.registry.clone(),
    )
    .run()
    .await?;

    let app = Router::new()
        .route("/health", get(api::health))
        .route("/ready", get(api::ready))
        .route("/discovery/stats", get(api::stats))
        .route("/v1/sources", get(factual_api::list_sources))
        .route("/v1/sources/health", get(factual_api::all_sources_health))
        .route("/v1/sources/:id/health", get(factual_api::source_health))
        .route("/v1/prices", get(factual_api::get_price))
        .route("/v1/releases", get(factual_api::get_release))
        .route("/v1/events", get(factual_api::get_events))
        .route("/v1/refresh", post(factual_api::refresh_source))
        .route("/internal/lifecycle", post(api::lifecycle_callback))
        .route("/internal/provenance-hit", post(api::provenance_hit))
        .route("/admin/exclude-asset", post(admin::exclude_asset))
        .route("/admin/replay-source", post(admin::replay_source))
        .with_state(state);

    info!(listen = %args.listen_addr, "discovery service listening");
    let listener = tokio::net::TcpListener::bind(&args.listen_addr).await?;

    let (stx, srx) = tokio::sync::oneshot::channel::<()>();
    let cancel_for_shutdown = cancel.clone();

    Service::new()
        .attach(metrics_service)
        .with_shutdown_signal(async move {
            cancel_for_shutdown.cancel();
        })
        .with_shutdown_signal(async move {
            let _ = stx.send(());
        })
        .spawn(async move {
            axum::serve(listener, app)
                .with_graceful_shutdown(async move {
                    let _ = srx.await;
                })
                .await?;
            Ok(())
        })
        .main()
        .await?;
    Ok(())
}
