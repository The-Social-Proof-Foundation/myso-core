// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use clap::Parser;
use myso_spot_oracle::config::OracleArgs;
use myso_spot_oracle::runtime;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = OracleArgs::parse();
    runtime::serve(args).await?;
    Ok(())
}
