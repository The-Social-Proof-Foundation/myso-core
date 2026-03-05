// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

mod block_conv;
mod handlers;
mod native_bridge;
mod store;

use anyhow::{Result, bail};
use clap::Parser;
use myso_indexer_alt_framework::{
    Indexer, IndexerArgs,
    ingestion::{ClientArgs, IngestionConfig},
    pipeline::{concurrent::ConcurrentConfig, CommitterConfig},
    service::Error,
};

use handlers::TxDigests;
use store::ClickHouseStore;

#[derive(clap::Parser, Debug, Clone)]
struct Args {
    #[clap(flatten)]
    pub clickhouse: ClickHouseArgs,

    #[clap(subcommand)]
    pub command: Command,
}

#[derive(clap::Args, Debug, Clone)]
struct ClickHouseArgs {
    /// ClickHouse host for native protocol. Overridden by CLICKHOUSE_HOST env var.
    #[clap(long, env = "CLICKHOUSE_HOST", default_value = "localhost")]
    pub clickhouse_host: String,

    /// ClickHouse native protocol port. Overridden by CLICKHOUSE_PORT env var.
    #[clap(long, env = "CLICKHOUSE_PORT", default_value = "9000")]
    pub clickhouse_port: u16,
}

#[derive(clap::Subcommand, Debug, Clone)]
enum Command {
    /// Run the indexer (default)
    Run {
        #[clap(flatten)]
        indexer_args: IndexerArgs,

        #[clap(flatten)]
        client_args: ClientArgs,
    },
    /// Drop watermarks and transactions tables for a clean reset. Use when switching
    /// checkpoint ranges or migrating from Docker to native ClickHouse.
    Reset,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    rustls::crypto::ring::default_provider()
        .install_default()
        .expect("Failed to install crypto provider");

    let args = Args::parse();

    let clickhouse_host = &args.clickhouse.clickhouse_host;
    let clickhouse_port = args.clickhouse.clickhouse_port;
    let clickhouse_user = std::env::var("CLICKHOUSE_USER").unwrap_or_else(|_| "default".to_string());

    println!(
        "Connecting to ClickHouse at {}:{} (user: {})",
        clickhouse_host, clickhouse_port, clickhouse_user
    );

    let store = ClickHouseStore::new(clickhouse_host, clickhouse_port, &clickhouse_user);

    match args.command {
        Command::Reset => {
            store.reset_tables().await?;
            println!("Dropped watermarks and transactions tables. Run the indexer with: cargo run -- run --remote-store-url ... --first-checkpoint=<N>");
            Ok(())
        }
        Command::Run { indexer_args, client_args } => {
            run_indexer(store, indexer_args, client_args).await
        }
    }
}

async fn run_indexer(
    store: ClickHouseStore,
    indexer_args: IndexerArgs,
    client_args: ClientArgs,
) -> Result<()> {
    // Ensure the database tables are created before starting the indexer
    store.create_tables_if_not_exists().await?;

    // Manually build the indexer with our custom ClickHouse store
    // This is the key difference from basic-myso-indexer which uses IndexerCluster::builder()
    let ingestion_config = IngestionConfig {
        ingest_concurrency: 50, // Lower than default (200) to avoid overwhelming testnet fullnode
        streaming_statement_timeout_ms: 60_000, // 60s - testnet may produce checkpoints slowly
        ..IngestionConfig::default()
    };
    let mut indexer = Indexer::new(
        store.clone(),
        indexer_args,
        client_args,
        ingestion_config,
        None,                // No metrics prefix
        &Default::default(), // Empty prometheus registry
    )
    .await?;

    // Single writer, small batches, frequent flush for reliable inserts
    let pipeline_config = ConcurrentConfig {
        committer: CommitterConfig {
            write_concurrency: 1,
            collect_interval_ms: 50,
            ..CommitterConfig::default()
        },
        ..ConcurrentConfig::default()
    };
    indexer
        .concurrent_pipeline(TxDigests, pipeline_config)
        .await?;

    println!("Starting ClickHouse MySo indexer...");
    println!("Tip: Use RUST_LOG=info to see commit progress (e.g. RUST_LOG=info cargo run ...)");

    // Start the indexer and wait for it to complete
    match indexer.run().await?.main().await {
        Ok(()) | Err(Error::Terminated) => Ok(()),
        Err(Error::Aborted) => {
            bail!("Indexer aborted due to an unexpected error")
        }
        Err(Error::Task(e)) => {
            bail!(e)
        }
    }
}
