// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Local SPoT oracle for `myso start --with-spot`.

use std::net::SocketAddr;
use std::time::Duration;

use anyhow::Context;
use colored::Colorize;
use tokio::process::Child;

use crate::local_sidecar_util::{
    docker_compose_up, http_base, myso_core_root, resolve_workspace_binary, spawn_with_env,
    wait_http_ok,
};

pub const DEFAULT_SPOT_ORACLE_PORT: u16 = 8097;
pub const DEFAULT_SPOT_METRICS_PORT: u16 = 9187;
pub const DEFAULT_DISCOVERY_PORT: u16 = 8096;

pub struct SpotOracleLocalInfo {
    pub listen: SocketAddr,
    pub base_url: String,
    pub myso_rpc: String,
    pub social_server_url: String,
    pub discovery_client_url: String,
    pub streaming_url: String,
}

pub async fn start_spot_postgres() -> anyhow::Result<()> {
    let compose = myso_core_root().join("crates/myso-spot-oracle/docker-compose.yml");
    docker_compose_up(&compose, &["spot-oracle-postgres"], None, &[]).await
}

pub async fn spawn_spot_oracle(
    listen: SocketAddr,
    fullnode_rpc_url: &str,
    social_server_url: &str,
) -> anyhow::Result<(SpotOracleLocalInfo, Child)> {
    start_spot_postgres().await?;

    let root = myso_core_root();
    let bin = resolve_workspace_binary("myso-spot-oracle")?;
    let listen_str = listen.to_string();
    let metrics = format!("127.0.0.1:{DEFAULT_SPOT_METRICS_PORT}");
    let discovery_client_url = format!("http://127.0.0.1:{DEFAULT_DISCOVERY_PORT}");
    let discovery_client_secret = "local-discovery-client".to_string();

    let envs = [
        (
            "SPOT_ORACLE_DATABASE_URL",
            "postgresql://spot:spot@127.0.0.1:5435/spot_oracle".to_string(),
        ),
        ("SPOT_ORACLE_LISTEN", listen_str),
        ("SPOT_ORACLE_METRICS_ADDRESS", metrics),
        ("SPOT_ORACLE_MYSO_RPC", fullnode_rpc_url.to_string()),
        (
            "SPOT_ORACLE_SOCIAL_SERVER_URL",
            social_server_url.to_string(),
        ),
        ("SPOT_ORACLE_STREAMING_URL", fullnode_rpc_url.to_string()),
        ("SPOT_ORACLE_INGEST_MODE", "checkpoint".to_string()),
        ("SPOT_ORACLE_DISCOVERY_CLIENT_URL", discovery_client_url.clone()),
        (
            "SPOT_ORACLE_DISCOVERY_CLIENT_SECRET",
            discovery_client_secret.clone(),
        ),
        ("SPOT_ORACLE_ENABLED", "true".to_string()),
        ("SPOT_ORACLE_LIVE_SOURCES", "true".to_string()),
        (
            "SPOT_ORACLE_SYNC_SECRET",
            "local-spot-oracle-sync".to_string(),
        ),
        ("RUST_LOG", "info".to_string()),
    ];

    let child = spawn_with_env(&bin, &envs, Some(&root))?;
    let base_url = http_base(listen);
    wait_http_ok(
        &format!("{base_url}/health"),
        Duration::from_secs(120),
        "spot-oracle",
    )
    .await
    .context("waiting for spot-oracle /health")?;

    let info = SpotOracleLocalInfo {
        listen,
        base_url,
        myso_rpc: fullnode_rpc_url.to_string(),
        social_server_url: social_server_url.to_string(),
        discovery_client_url,
        streaming_url: fullnode_rpc_url.to_string(),
    };
    Ok((info, child))
}

pub fn log_spot_once(info: &SpotOracleLocalInfo) {
    println!(
        "{}",
        format!(
            r"SPoT oracle (local):
  listen:              {} ({})
  myso_rpc:            {}
  streaming_url:       {}
  discovery_client:    {}
  social_server:       {}
",
            info.base_url,
            info.listen,
            info.myso_rpc,
            info.streaming_url,
            info.discovery_client_url,
            info.social_server_url
        )
        .green()
    );
}
