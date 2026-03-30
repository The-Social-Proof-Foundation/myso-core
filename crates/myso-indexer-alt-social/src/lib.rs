// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use url::Url;

mod handlers;

pub use handlers::{
    BlockingHandler, GovernanceHandler, InsuranceHandler, MyDataHandler, PlatformHandler,
    PocHandler, PostsHandler, ProfilesHandler, SocialGraphHandler, SpotHandler, SptHandler,
    SubscriptionHandler, UpgradeHandler,
};

pub const MAINNET_REMOTE_STORE_URL: &str = "https://checkpoints.mainnet.mysocial.network";
pub const TESTNET_REMOTE_STORE_URL: &str =
    "https://storage.googleapis.com/mysocial-testnet-checkpoints";

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
        .concurrent_pipeline(BlockingHandler, Default::default())
        .await
        .context("Failed to add BlockingHandler pipeline")?;
    indexer
        .concurrent_pipeline(GovernanceHandler, Default::default())
        .await
        .context("Failed to add GovernanceHandler pipeline")?;
    indexer
        .concurrent_pipeline(UpgradeHandler, Default::default())
        .await
        .context("Failed to add UpgradeHandler pipeline")?;
    indexer
        .concurrent_pipeline(SocialGraphHandler, Default::default())
        .await
        .context("Failed to add SocialGraphHandler pipeline")?;
    indexer
        .concurrent_pipeline(PlatformHandler, Default::default())
        .await
        .context("Failed to add PlatformHandler pipeline")?;
    indexer
        .concurrent_pipeline(MyDataHandler, Default::default())
        .await
        .context("Failed to add MyDataHandler pipeline")?;
    indexer
        .concurrent_pipeline(InsuranceHandler, Default::default())
        .await
        .context("Failed to add InsuranceHandler pipeline")?;
    indexer
        .concurrent_pipeline(SpotHandler, Default::default())
        .await
        .context("Failed to add SpotHandler pipeline")?;
    indexer
        .concurrent_pipeline(SptHandler, Default::default())
        .await
        .context("Failed to add SptHandler pipeline")?;
    indexer
        .concurrent_pipeline(PocHandler, Default::default())
        .await
        .context("Failed to add PocHandler pipeline")?;
    indexer
        .concurrent_pipeline(SubscriptionHandler, Default::default())
        .await
        .context("Failed to add SubscriptionHandler pipeline")?;
    indexer
        .concurrent_pipeline(ProfilesHandler, Default::default())
        .await
        .context("Failed to add ProfilesHandler pipeline")?;
    indexer
        .concurrent_pipeline(PostsHandler, Default::default())
        .await
        .context("Failed to add PostsHandler pipeline")?;

    tracing::info!(
        "Social indexer pipelines registered — blocking, governance, upgrade, social_graph, \
         platform, mydata, insurance, spot, spt, poc, subscription, profiles, posts; \
         resuming from watermarks or checkpoint 0"
    );

    let s_indexer = indexer
        .run()
        .await
        .context("Failed to start social indexer")?;
    let s_metrics = metrics.run().await?;

    Ok(s_indexer.attach(s_metrics))
}

/// Threshold: values below this are likely epoch seconds (e.g. year 2001+ in seconds).
/// Values at or above are likely epoch milliseconds.
const LIKELY_SECONDS_THRESHOLD: i64 = 1_000_000_000_000;

/// Normalize a timestamp to epoch milliseconds.
/// Use when parsing event payloads that may contain either seconds (pre-upgrade) or ms (post-upgrade).
/// If `from_seconds` is true and value is below threshold, multiplies by 1000.
pub fn ensure_epoch_ms(value: i64, from_seconds: bool) -> i64 {
    if from_seconds && value > 0 && value < LIKELY_SECONDS_THRESHOLD {
        value.saturating_mul(1000)
    } else {
        value
    }
}
