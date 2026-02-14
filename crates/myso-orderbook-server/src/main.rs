// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use myso_orderbook_server::server::run_server;
use std::net::SocketAddr;
use myso_pg_db::DbArgs;
use url::Url;

#[derive(Parser)]
#[clap(rename_all = "kebab-case", author, version)]
struct Args {
    #[command(flatten)]
    db_args: DbArgs,
    #[clap(env, long, default_value_t = 9008)]
    server_port: u16,
    #[clap(env, long, default_value = "0.0.0.0:9184")]
    metrics_address: SocketAddr,
    #[clap(
        env,
        long,
        default_value = "postgres://postgres:postgrespw@localhost:5432/orderbook"
    )]
    database_url: Url,
    #[clap(env, long, default_value = "https://fullnode.mainnet.mysocial.network:443")]
    rpc_url: Url,
    #[clap(
        env,
        long,
        default_value = "0x2c8d603bc51326b8c13cef9dd07031a408a48dddb541963357661df5d3204809"
    )]
    orderbook_package_id: String,
    #[clap(
        env,
        long,
        default_value = "0xdeeb7a4662eec9f2f3def03fb937a663dddaa2e215b8078a284d026b7946c270"
    )]
    myso_token_package_id: String,
    #[clap(
        env,
        long,
        default_value = "0x032abf8948dda67a271bcc18e776dbbcfb0d58c8d288a700ff0d5521e57a1ffe"
    )]
    myso_treasury_id: String,

    // Margin metrics polling configuration
    #[clap(env, long, default_value_t = 30)]
    margin_poll_interval_secs: u64,
    #[clap(env, long)]
    margin_package_id: Option<String>,
    /// Comma-separated list of valid admin bearer tokens
    #[clap(env = "ADMIN_TOKENS", long)]
    admin_tokens: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let _guard = telemetry_subscribers::TelemetryConfig::new()
        .with_env()
        .init();

    let Args {
        db_args,
        server_port,
        metrics_address,
        database_url,
        rpc_url,
        orderbook_package_id,
        myso_token_package_id,
        myso_treasury_id,
        margin_poll_interval_secs,
        margin_package_id,
        admin_tokens,
    } = Args::parse();

    run_server(
        server_port,
        database_url,
        db_args,
        rpc_url,
        metrics_address,
        orderbook_package_id,
        myso_token_package_id,
        myso_treasury_id,
        margin_poll_interval_secs,
        margin_package_id,
        admin_tokens,
    )
    .await?;

    Ok(())
}
