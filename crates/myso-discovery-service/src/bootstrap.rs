// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use tracing::info;

use crate::config::DiscoveryArgs;
use crate::embed_client::EmbedClient;
use crate::metrics::DiscoveryMetrics;
use crate::scheduler::{drain_embed_jobs, poll_sources_once};
use crate::sources::{build_default_registry, SourceConfig};
use crate::store::DiscoveryStore;

/// One-shot cold-start: poll sources once, drain embed jobs, then return.
pub async fn run_bootstrap(
    store: Arc<DiscoveryStore>,
    sources: Vec<SourceConfig>,
    args: Arc<DiscoveryArgs>,
    metrics: Arc<DiscoveryMetrics>,
) -> anyhow::Result<()> {
    let registry = Arc::new(build_default_registry());
    let embed_client = Arc::new(EmbedClient::new(
        args.embed_endpoint.clone(),
        args.embed_secret.clone(),
    ));

    info!("discovery bootstrap: polling sources once");
    poll_sources_once(
        store.clone(),
        registry,
        sources,
        args.embed_enabled,
        args.max_retries,
        metrics.clone(),
    )
    .await;

    if args.embed_enabled {
        info!("discovery bootstrap: draining embed jobs");
        drain_embed_jobs(
            store,
            embed_client,
            args.clone(),
            metrics.clone(),
            500,
        )
        .await;
    }

    info!("discovery bootstrap complete");
    Ok(())
}
