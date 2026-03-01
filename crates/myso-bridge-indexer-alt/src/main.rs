// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0
use anyhow::Context;
use clap::Parser;
use myso_bridge_indexer_alt::handlers::error_handler::ErrorTransactionHandler;
use myso_bridge_indexer_alt::handlers::governance_action_handler::GovernanceActionHandler;
use myso_bridge_indexer_alt::handlers::token_transfer_data_handler::TokenTransferDataHandler;
use myso_bridge_indexer_alt::handlers::token_transfer_handler::TokenTransferHandler;
use myso_bridge_indexer_alt::metrics::BridgeIndexerMetrics;
use myso_bridge_schema::MIGRATIONS;
use myso_indexer_alt_framework::ingestion::{
    streaming_client::StreamingClientArgs, ClientArgs, ingestion_client::IngestionClientArgs,
};
use myso_indexer_alt_framework::postgres::DbArgs;
use myso_indexer_alt_framework::service::Error;
use myso_indexer_alt_framework::{Indexer, IndexerArgs};
use myso_indexer_alt_metrics::{MetricsArgs, MetricsService};
use prometheus::Registry;
use std::net::SocketAddr;
use url::Url;

#[derive(Parser)]
#[clap(rename_all = "kebab-case", author, version)]
struct Args {
    #[command(flatten)]
    db_args: DbArgs,
    #[command(flatten)]
    indexer_args: IndexerArgs,
    #[clap(env, long, default_value = "0.0.0.0:9184")]
    metrics_address: SocketAddr,
    #[clap(
        env,
        long,
        default_value = "postgres://postgres:postgrespw@localhost:5432/bridge"
    )]
    database_url: Url,
    #[clap(
        env,
        long,
        default_value = "https://checkpoints.mainnet.mysocial.network"
    )]
    remote_store_url: Url,
    /// gRPC endpoint for streaming checkpoints (optional, enables faster ingestion)
    #[clap(env, long)]
    streaming_url: Option<Url>,
}
#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let Args {
        db_args,
        indexer_args,
        metrics_address,
        database_url,
        remote_store_url,
        streaming_url,
    } = Args::parse();

    let is_bounded_job = indexer_args.last_checkpoint.is_some();
    let registry = Registry::new_custom(Some("bridge".into()), None)
        .context("Failed to create Prometheus registry.")?;

    // Initialize bridge-specific metrics
    let bridge_metrics = BridgeIndexerMetrics::new(&registry);

    let metrics = MetricsService::new(MetricsArgs { metrics_address }, registry);

    // When streaming is configured, use gRPC GetCheckpoint for ingestion fallback (when outside
    // buffer). This avoids depending on checkpoint-blob-indexer/remote store. When streaming is not set, use remote_store_url for HTTP ingestion.
    let ingestion_args = if let Some(ref url) = streaming_url {
        IngestionClientArgs {
            rpc_api_url: Some(url.clone()),
            ..Default::default()
        }
    } else {
        IngestionClientArgs {
            remote_store_url: Some(remote_store_url),
            ..Default::default()
        }
    };

    let metrics_prefix = None;
    let mut indexer = Indexer::new_from_pg(
        database_url,
        db_args,
        indexer_args,
        ClientArgs {
            ingestion: ingestion_args,
            streaming: StreamingClientArgs {
                streaming_url: streaming_url.and_then(|u| u.as_str().parse().ok()),
            },
        },
        Default::default(),
        Some(&MIGRATIONS),
        metrics_prefix,
        metrics.registry(),
    )
    .await?;

    indexer
        .concurrent_pipeline(
            TokenTransferHandler::new(bridge_metrics.clone()),
            Default::default(),
        )
        .await?;

    indexer
        .concurrent_pipeline(TokenTransferDataHandler::default(), Default::default())
        .await?;

    indexer
        .concurrent_pipeline(
            GovernanceActionHandler::new(bridge_metrics.clone()),
            Default::default(),
        )
        .await?;

    indexer
        .concurrent_pipeline(ErrorTransactionHandler, Default::default())
        .await?;

    let s_indexer = indexer.run().await?;
    let s_metrics = metrics.run().await?;

    match s_indexer.attach(s_metrics).main().await {
        Ok(()) => Ok(()),
        Err(Error::Terminated) => {
            if is_bounded_job {
                std::process::exit(1);
            } else {
                Ok(())
            }
        }
        Err(Error::Aborted) => {
            std::process::exit(1);
        }
        Err(Error::Task(_)) => {
            std::process::exit(2);
        }
    }
}
