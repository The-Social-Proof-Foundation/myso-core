// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use sqlx::postgres::PgPoolOptions;
use tokio_util::sync::CancellationToken;
use tracing::info;

use crate::api;
use crate::config::DiscoveryArgs;
use crate::embed_client::EmbedClient;
use crate::scheduler::{run_scheduler_loop, run_worker_loop};
use crate::sources::build_default_registry;
use crate::store::DiscoveryStore;

pub struct AppState {
    pub store: Arc<DiscoveryStore>,
    pub args: Arc<DiscoveryArgs>,
}

pub async fn serve(args: DiscoveryArgs) -> anyhow::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&args.database_url)
        .await?;
    myso_discovery_service_schema::run_migrations(&pool).await?;

    let store = Arc::new(DiscoveryStore::new(pool));
    let registry = Arc::new(build_default_registry());
    let embed_client = Arc::new(EmbedClient::new(
        args.embed_endpoint.clone(),
        args.embed_secret.clone(),
    ));
    let args = Arc::new(args);

    // Real sources only — fail fast if config missing/empty (no silent manual_curated stub).
    let sources = myso_discovery_service_core::sources::config_loader::load_sources_config(
        &args.sources_config,
    )?;

    let cancel = CancellationToken::new();
    if args.enabled {
        let sched_store = store.clone();
        let sched_registry = registry.clone();
        let sched_sources = sources.clone();
        let poll = args.scheduler_poll_interval_secs;
        let embed_enabled = args.embed_enabled;
        let c = cancel.clone();
        tokio::spawn(async move {
            tokio::select! {
                _ = run_scheduler_loop(sched_store, sched_registry, sched_sources, poll, embed_enabled) => {}
                _ = c.cancelled() => {}
            }
        });

        if args.embed_enabled {
            let worker_store = store.clone();
            let worker_embed = embed_client.clone();
            let worker_args = args.clone();
            let worker_concurrency = args.worker_concurrency;
            let c = cancel.clone();
            tokio::spawn(async move {
                tokio::select! {
                    _ = run_worker_loop(worker_store, worker_embed, worker_args, worker_concurrency) => {}
                    _ = c.cancelled() => {}
                }
            });
        } else {
            info!("embed worker disabled (DISCOVERY_EMBED_ENABLED=false); assets remain normalized without PoC indexing");
        }
    }

    let state = Arc::new(AppState {
        store: store.clone(),
        args: args.clone(),
    });

    let app = Router::new()
        .route("/health", get(api::health))
        .route("/ready", get(api::ready))
        .route("/discovery/stats", get(api::stats))
        .route("/internal/lifecycle", post(api::lifecycle_callback))
        .route("/internal/provenance-hit", post(api::provenance_hit))
        .with_state(state);

    info!(listen = %args.listen_addr, "discovery service listening");
    let listener = tokio::net::TcpListener::bind(&args.listen_addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            wait_for_shutdown_signal().await;
            cancel.cancel();
        })
        .await?;
    Ok(())
}

async fn wait_for_shutdown_signal() {
    let ctrl_c = tokio::signal::ctrl_c();

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {}
        _ = terminate => {}
    }
}
