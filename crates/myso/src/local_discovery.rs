// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Local discovery service for `myso start --with-spot` / `--with-poc`.

use std::net::SocketAddr;
use std::path::PathBuf;
use std::time::Duration;

use anyhow::Context;
use colored::Colorize;
use tokio::process::Child;

use crate::local_sidecar_util::{
    docker_compose_up, http_base, myso_core_root, resolve_workspace_binary, spawn_with_env,
    wait_http_ok,
};

pub const DEFAULT_DISCOVERY_PORT: u16 = 8096;
pub const DEFAULT_DISCOVERY_METRICS_PORT: u16 = 9286;
pub const LOCAL_EMBED_SECRET: &str = "discovery-secret";
pub const LOCAL_ADMIN_SECRET: &str = "local-discovery-admin";

#[derive(Debug, Clone, Copy)]
pub enum DiscoveryCorpus {
    Factual,
    Media,
    Combined,
}

impl DiscoveryCorpus {
    pub fn from_flags(with_spot: bool, with_poc: bool) -> Option<Self> {
        match (with_spot, with_poc) {
            (true, false) => Some(Self::Factual),
            (false, true) => Some(Self::Media),
            (true, true) => Some(Self::Combined),
            (false, false) => None,
        }
    }

    fn sources_rel_path(self) -> &'static str {
        match self {
            Self::Factual => {
                "crates/myso-discovery-service/config/discovery/sources.factual.localnet.yaml"
            }
            Self::Media => {
                "crates/myso-discovery-service/config/discovery/sources.media.localnet.yaml"
            }
            Self::Combined => {
                "crates/myso-discovery-service/config/discovery/sources.combined.localnet.yaml"
            }
        }
    }

    fn use_manual_curated(self) -> bool {
        matches!(self, Self::Media | Self::Combined)
    }
}

pub struct DiscoveryLocalInfo {
    pub listen: SocketAddr,
    pub base_url: String,
    pub corpus: DiscoveryCorpus,
    pub embed_enabled: bool,
    pub embed_endpoint: Option<String>,
    pub embed_secret: Option<String>,
    pub admin_secret: String,
    pub sources_config: PathBuf,
}

pub async fn start_discovery_postgres() -> anyhow::Result<()> {
    let compose = myso_core_root().join("crates/myso-discovery-service/docker-compose.yml");
    docker_compose_up(&compose, &["discovery-postgres"], None, &[]).await
}

pub async fn spawn_discovery(
    listen: SocketAddr,
    corpus: DiscoveryCorpus,
    embed_enabled: bool,
    poc_api_port: u16,
) -> anyhow::Result<(DiscoveryLocalInfo, Child)> {
    start_discovery_postgres().await?;

    let root = myso_core_root();
    let sources_config = root.join(corpus.sources_rel_path());
    let bin = resolve_workspace_binary("myso-discovery-service")?;
    let metrics = format!("127.0.0.1:{DEFAULT_DISCOVERY_METRICS_PORT}");
    let db_url = "postgresql://poc:poc@127.0.0.1:5434/discovery".to_string();
    let listen_str = listen.to_string();
    let embed_endpoint = if embed_enabled {
        Some(format!(
            "http://127.0.0.1:{poc_api_port}/internal/discovery/embed"
        ))
    } else {
        None
    };

    let mut envs = vec![
        ("DISCOVERY_DATABASE_URL", db_url),
        ("DISCOVERY_LISTEN", listen_str),
        ("DISCOVERY_METRICS_ADDRESS", metrics),
        (
            "DISCOVERY_SOURCES_CONFIG",
            sources_config.display().to_string(),
        ),
        (
            "DISCOVERY_EMBED_ENABLED",
            if embed_enabled {
                "true".to_string()
            } else {
                "false".to_string()
            },
        ),
        ("DISCOVERY_ADMIN_SECRET", LOCAL_ADMIN_SECRET.to_string()),
        ("DISCOVERY_ENABLED", "true".to_string()),
        ("RUST_LOG", "info".to_string()),
    ];
    if corpus.use_manual_curated() {
        envs.push(("DISCOVERY_USE_MANUAL_CURATED", "1".to_string()));
    }
    if let Some(ref ep) = embed_endpoint {
        envs.push(("DISCOVERY_EMBED_ENDPOINT", ep.clone()));
        envs.push(("DISCOVERY_EMBED_SECRET", LOCAL_EMBED_SECRET.to_string()));
    }

    let child = spawn_with_env(&bin, &envs, Some(&root))?;
    let base_url = http_base(listen);
    wait_http_ok(
        &format!("{base_url}/health"),
        Duration::from_secs(120),
        "discovery service",
    )
    .await
    .context("waiting for discovery /health")?;

    let info = DiscoveryLocalInfo {
        listen,
        base_url,
        corpus,
        embed_enabled,
        embed_endpoint,
        embed_secret: embed_enabled.then(|| LOCAL_EMBED_SECRET.to_string()),
        admin_secret: LOCAL_ADMIN_SECRET.to_string(),
        sources_config,
    };
    Ok((info, child))
}

pub fn log_discovery_once(info: &DiscoveryLocalInfo) {
    let corpus = match info.corpus {
        DiscoveryCorpus::Factual => "factual",
        DiscoveryCorpus::Media => "media",
        DiscoveryCorpus::Combined => "combined",
    };
    println!(
        "{}",
        format!(
            r"Discovery service (local):
  listen:          {} ({})
  corpus:          {corpus}
  embed_enabled:   {}
  admin_secret:    {}
  sources:         {:?}
",
            info.base_url,
            info.listen,
            info.embed_enabled,
            info.admin_secret,
            info.sources_config
        )
        .green()
    );
    if let (Some(ep), Some(secret)) = (&info.embed_endpoint, &info.embed_secret) {
        println!(
            "{}",
            format!("  embed_endpoint:  {ep}\n  embed_secret:    {secret}\n").green()
        );
    }
}
