// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct DiscoveryArgs {
    #[arg(long, env = "DISCOVERY_LISTEN", default_value = "0.0.0.0:8096")]
    pub listen_addr: String,

    #[arg(
        long,
        env = "DISCOVERY_METRICS_ADDRESS",
        default_value = "0.0.0.0:9286"
    )]
    pub metrics_addr: SocketAddr,

    #[arg(
        long,
        env = "DISCOVERY_DATABASE_URL",
        default_value = "postgresql://poc:poc@127.0.0.1:5434/discovery"
    )]
    pub database_url: String,

    #[arg(
        long,
        env = "DISCOVERY_EMBED_ENDPOINT",
        default_value = "http://127.0.0.1:8000/internal/discovery/embed"
    )]
    pub embed_endpoint: String,

    /// When false, discovery fetch/indexing runs without calling the PoC embed worker.
    /// Local dev and discovery-runnable E2E do not require embed; set true when the
    /// proof-of-creativity stack is running.
    #[arg(long, env = "DISCOVERY_EMBED_ENABLED", default_value = "false")]
    pub embed_enabled: bool,

    #[arg(long, env = "DISCOVERY_EMBED_SECRET")]
    pub embed_secret: Option<String>,

    /// Secret for `x-discovery-admin-secret` on `/admin/*` routes. Required for admin.
    #[arg(long, env = "DISCOVERY_ADMIN_SECRET")]
    pub admin_secret: Option<String>,

    #[arg(long, env = "DISCOVERY_SCHEDULER_POLL_INTERVAL_SECONDS", default_value = "300")]
    pub scheduler_poll_interval_secs: u64,

    #[arg(long, env = "DISCOVERY_WORKER_CONCURRENCY", default_value = "2")]
    pub worker_concurrency: usize,

    /// Max embed attempts before dead-letter (also used as job max_attempts default).
    #[arg(long, env = "DISCOVERY_MAX_RETRIES", default_value = "5")]
    pub max_retries: i32,

    #[arg(
        long,
        env = "DISCOVERY_SOURCES_CONFIG",
        default_value = "crates/myso-discovery-service/config/discovery/sources.localnet.yaml"
    )]
    pub sources_config: PathBuf,

    #[arg(
        long,
        env = "DISCOVERY_ACTIVE_EMBEDDING_VERSION",
        default_value = "clip-vit-b32-v1"
    )]
    pub active_embedding_version: String,

    #[arg(long, env = "DISCOVERY_ENABLED", default_value = "true")]
    pub enabled: bool,

    /// Secret for `x-discovery-client-secret` on `/v1/*` factual query routes.
    #[arg(long, env = "DISCOVERY_CLIENT_SECRET")]
    pub client_secret: Option<String>,

    /// Factual cache TTL in seconds for `/v1/*` responses.
    #[arg(long, env = "DISCOVERY_CACHE_TTL_SECS", default_value = "300")]
    pub cache_ttl_secs: i64,

    /// Run one scheduler poll + worker drain then exit (cold-start bootstrap).
    #[arg(long, env = "DISCOVERY_BOOTSTRAP_ONLY", default_value = "false")]
    pub bootstrap_only: bool,
}
