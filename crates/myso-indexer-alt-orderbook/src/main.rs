use anyhow::Context;
use clap::Parser;
use myso_indexer_alt_framework::ingestion::ingestion_client::IngestionClientArgs;
use myso_indexer_alt_framework::ingestion::streaming_client::StreamingClientArgs;
use myso_indexer_alt_framework::ingestion::ClientArgs;
use myso_indexer_alt_framework::IndexerArgs;
use myso_indexer_alt_metrics::MetricsArgs;
use myso_pg_db::DbArgs;
use orderbook_indexer::{build_orderbook_indexer, OrderbookEnv, Package};
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
    #[command(flatten)]
    streaming_args: StreamingClientArgs,
    #[clap(env, long, default_value = "0.0.0.0:9184")]
    metrics_address: SocketAddr,
    #[clap(
        env,
        long,
        default_value = "postgres://postgres:postgrespw@localhost:5432/orderbook"
    )]
    database_url: Url,
    /// Checkpoint source / remote store selection for the standalone binary (genesis addresses still come from chain).
    #[clap(env, long)]
    env: OrderbookEnv,
    /// Packages to index events for (can specify multiple)
    #[clap(long, value_enum, default_values = ["orderbook", "orderbook-margin"])]
    packages: Vec<Package>,
    /// HTTP checkpoint store URL (optional). When set, used for ingestion instead of gRPC.
    #[clap(long, env = "REMOTE_STORE_URL")]
    remote_store_url: Option<Url>,
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
        packages,
        remote_store_url,
    } = Args::parse();

    let registry = Registry::new_custom(Some("orderbook".into()), None)
        .context("Failed to create Prometheus registry.")?;

    let ingestion = if let Some(remote_url) = remote_store_url {
        IngestionClientArgs {
            remote_store_url: Some(remote_url),
            ..Default::default()
        }
    } else if let Some(ref u) = streaming_args.streaming_url {
        let rpc_url = Url::parse(&u.to_string()).context("Invalid streaming URL for RPC")?;
        IngestionClientArgs {
            rpc_api_url: Some(rpc_url),
            ..Default::default()
        }
    } else {
        match env {
            OrderbookEnv::Local => anyhow::bail!(
                "Local indexing requires checkpoint bytes: pass --local-ingestion-path <DIR> \
                 (the node's data_ingestion directory, same as `myso start` with --with-indexer), \
                 or use --streaming-url / --remote-store-url."
            ),
            OrderbookEnv::Testnet => anyhow::bail!(
                "Testnet requires --streaming-url or --remote-store-url for checkpoint ingestion \
                 (e.g. --streaming-url http://fullnode.testnet.mysocial.network:9000 or \
                 --remote-store-url https://storage.googleapis.com/mysocial-testnet-checkpoint-blobs)"
            ),
            OrderbookEnv::Mainnet => IngestionClientArgs {
                remote_store_url: Some(env.remote_store_url()),
                ..Default::default()
            },
        }
    };

    let client_args = ClientArgs {
        ingestion,
        streaming: streaming_args,
    };

    let service = build_orderbook_indexer(
        database_url,
        db_args,
        indexer_args,
        client_args,
        MetricsArgs { metrics_address },
        &registry,
        env,
        &packages,
    )
    .await?;

    service.main().await?;

    Ok(())
}
