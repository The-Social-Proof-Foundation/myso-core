// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use clap::Parser;
use myso_indexer_alt_framework::ingestion::ingestion_client::IngestionClientArgs;
use myso_indexer_alt_framework::ingestion::streaming_client::StreamingClientArgs;
use myso_indexer_alt_framework::ingestion::{ClientArgs, IngestionConfig};
use myso_indexer_alt_framework::{Indexer, IndexerArgs};
use myso_indexer_alt_metrics::db::DbConnectionStatsCollector;
use myso_indexer_alt_metrics::{MetricsArgs, MetricsService};
use myso_indexer_alt_social_schema::MIGRATIONS;
use myso_pg_db::{Db, DbArgs};
use prometheus::Registry;
use social_indexer::{
    BlockingHandler, GovernanceHandler, InsuranceHandler, MyDataHandler, PlatformHandler,
    PostsHandler, ProfilesHandler, SocialEnv, SocialGraphHandler, SpotHandler, SptHandler,
    SubscriptionHandler, UpgradeHandler,
};
use std::net::SocketAddr;
use std::path::PathBuf;
use url::Url;

#[derive(Parser)]
#[clap(rename_all = "kebab-case", author, version)]
struct Args {
    #[command(flatten)]
    db_args: DbArgs,
    #[command(flatten)]
    indexer_args: IndexerArgs,
    #[command(flatten)]
    streaming_args: StreamingClientArgs,
    #[clap(env, long, default_value = "0.0.0.0:9185")]
    metrics_address: SocketAddr,
    #[clap(
        env,
        long,
        default_value = "postgres://postgres:postgrespw@localhost:5432/social"
    )]
    database_url: Url,
    #[clap(env, long)]
    env: SocialEnv,
    /// Path to local checkpoint directory (for local dev). Overrides --env when set.
    #[clap(long)]
    local_ingestion_path: Option<PathBuf>,
    /// Fullnode gRPC URL to fetch checkpoints (for local dev). Overrides --env when set.
    #[clap(long)]
    rpc_api_url: Option<Url>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let Args {
        db_args,
        indexer_args,
        streaming_args,
        metrics_address,
        database_url,
        env,
        local_ingestion_path,
        rpc_api_url,
    } = Args::parse();

    let ingestion_args = if let Some(path) = local_ingestion_path {
        IngestionClientArgs {
            local_ingestion_path: Some(path),
            ..Default::default()
        }
    } else if let Some(url) = rpc_api_url {
        IngestionClientArgs {
            rpc_api_url: Some(url),
            ..Default::default()
        }
    } else {
        IngestionClientArgs {
            remote_store_url: Some(env.remote_store_url()),
            ..Default::default()
        }
    };

    let registry = Registry::new_custom(Some("social".into()), None)
        .context("Failed to create Prometheus registry.")?;
    let metrics = MetricsService::new(MetricsArgs { metrics_address }, registry.clone());

    let store = Db::for_write(database_url, db_args)
        .await
        .context("Failed to connect to database")?;

    store
        .run_migrations(Some(&MIGRATIONS))
        .await
        .context("Failed to run pending migrations")?;

    registry.register(Box::new(DbConnectionStatsCollector::new(
        Some("social_indexer_db"),
        store.clone(),
    )))?;

    let social_metrics = social_indexer::metrics::SocialMetrics::new(&registry)
        .context("SocialMetrics registration")?;
    social_indexer::metrics::SocialMetrics::init(social_metrics);

    let mut indexer = Indexer::new(
        store,
        indexer_args,
        ClientArgs {
            ingestion: ingestion_args,
            streaming: streaming_args,
        },
        IngestionConfig::default(),
        None,
        metrics.registry(),
    )
    .await?;

    indexer
        .concurrent_pipeline(BlockingHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(GovernanceHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(UpgradeHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(SocialGraphHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(PlatformHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(MyDataHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(InsuranceHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(SpotHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(SptHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(SubscriptionHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(ProfilesHandler, Default::default())
        .await?;
    indexer
        .concurrent_pipeline(PostsHandler, Default::default())
        .await?;

    let s_indexer = indexer.run().await?;
    let s_metrics = metrics.run().await?;

    s_indexer.attach(s_metrics).main().await?;
    Ok(())
}
