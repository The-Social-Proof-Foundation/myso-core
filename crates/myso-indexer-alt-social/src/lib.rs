// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use url::Url;

pub mod handlers;

pub const MAINNET_REMOTE_STORE_URL: &str = "https://checkpoints.mainnet.mysocial.network";
pub const TESTNET_REMOTE_STORE_URL: &str = "https://checkpoints.testnet.mysocial.network";

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum SocialEnv {
    Mainnet,
    Testnet,
}

impl SocialEnv {
    pub fn remote_store_url(&self) -> Url {
        let url = match self {
            SocialEnv::Mainnet => MAINNET_REMOTE_STORE_URL,
            SocialEnv::Testnet => TESTNET_REMOTE_STORE_URL,
        };
        Url::parse(url).unwrap()
    }
}

/// Set up the social indexer and return a Service that can be merged into a larger service.
/// Uses the provided client_args (e.g. with local_ingestion_path for local dev).
pub async fn setup_social_indexer(
    database_url: Url,
    db_args: myso_pg_db::DbArgs,
    indexer_args: myso_indexer_alt_framework::IndexerArgs,
    client_args: myso_indexer_alt_framework::ingestion::ClientArgs,
    metrics_args: myso_indexer_alt_metrics::MetricsArgs,
    registry: &prometheus::Registry,
) -> anyhow::Result<myso_indexer_alt_framework::service::Service> {
    use anyhow::Context;
    use myso_indexer_alt_framework::ingestion::IngestionConfig;
    use myso_indexer_alt_framework::Indexer;
    use myso_indexer_alt_metrics::db::DbConnectionStatsCollector;
    use myso_indexer_alt_metrics::MetricsService;
    use myso_indexer_alt_social_schema::MIGRATIONS;
    use myso_pg_db::Db;

    let store = Db::for_write(database_url, db_args)
        .await
        .context("Failed to connect to social database")?;

    store
        .run_migrations(Some(&MIGRATIONS))
        .await
        .context("Failed to run social migrations")?;

    registry.register(Box::new(DbConnectionStatsCollector::new(
        Some("social_indexer_db"),
        store.clone(),
    )))?;

    let metrics = MetricsService::new(metrics_args, registry.clone());

    let mut indexer = Indexer::new(
        store,
        indexer_args,
        client_args,
        IngestionConfig::default(),
        Some("social"),
        registry,
    )
    .await
    .context("Failed to create social indexer")?;

    indexer
        .concurrent_pipeline(handlers::social_events::SocialEvents, Default::default())
        .await
        .context("Failed to add SocialEvents pipeline")?;

    tracing::info!(
        pipeline = "social_events",
        "Social pipeline registered — indexes profiles, governance, platforms, posts; \
         resuming from watermark or checkpoint 0"
    );

    let s_indexer = indexer
        .run()
        .await
        .context("Failed to start social indexer")?;
    let s_metrics = metrics.run().await?;

    Ok(s_indexer.attach(s_metrics))
}
