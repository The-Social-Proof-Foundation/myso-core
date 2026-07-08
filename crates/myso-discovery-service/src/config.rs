// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use clap::Parser;
use figment::{providers::Env, providers::Format, providers::Toml, Figment};
use serde::Deserialize;

#[derive(Debug, Clone, Parser)]
pub struct DiscoveryArgs {
    #[arg(long, env = "DISCOVERY_LISTEN", default_value = "0.0.0.0:8096")]
    pub listen_addr: String,

    #[arg(
        long,
        env = "DISCOVERY_DATABASE_URL",
        default_value = "postgresql://poc:poc@127.0.0.1:5433/discovery"
    )]
    pub database_url: String,

    #[arg(
        long,
        env = "DISCOVERY_EMBED_ENDPOINT",
        default_value = "http://127.0.0.1:8000/internal/discovery/embed"
    )]
    pub embed_endpoint: String,

    #[arg(long, env = "DISCOVERY_EMBED_SECRET")]
    pub embed_secret: Option<String>,

    #[arg(long, env = "DISCOVERY_ADMIN_SECRET")]
    pub admin_secret: Option<String>,

    #[arg(long, env = "DISCOVERY_SCHEDULER_POLL_INTERVAL_SECONDS", default_value = "300")]
    pub scheduler_poll_interval_secs: u64,

    #[arg(long, env = "DISCOVERY_WORKER_CONCURRENCY", default_value = "2")]
    pub worker_concurrency: usize,

    #[arg(long, env = "DISCOVERY_MAX_EMBEDS_PER_HOUR", default_value = "100")]
    pub max_embeds_per_hour: u64,

    #[arg(long, env = "DISCOVERY_MAX_CONCURRENT_FETCHES", default_value = "4")]
    pub max_concurrent_fetches: usize,

    #[arg(long, env = "DISCOVERY_MAX_RETRIES", default_value = "5")]
    pub max_retries: i32,

    #[arg(
        long,
        env = "DISCOVERY_SOURCES_CONFIG",
        default_value = "config/discovery/sources.localnet.yaml"
    )]
    pub sources_config: PathBuf,

    #[arg(
        long,
        env = "DISCOVERY_ACTIVE_EMBEDDING_VERSION",
        default_value = "clip-vit-b32-v1"
    )]
    pub active_embedding_version: String,

    #[arg(long, env = "DISCOVERY_X_HANDLE_CONFIDENCE_THRESHOLD", default_value = "0.85")]
    pub x_handle_confidence_threshold: f64,

    #[arg(long, env = "DISCOVERY_WORK_CONFIDENCE_THRESHOLD", default_value = "0.95")]
    pub work_confidence_threshold: f64,

    #[arg(long, env = "DISCOVERY_ENABLED", default_value = "true")]
    pub enabled: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiscoverySettings {
    pub listen_addr: String,
    pub database_url: String,
}

impl DiscoveryArgs {
    pub fn load_figment(&self) -> Figment {
        Figment::new()
            .merge(Toml::file("config/default.toml").nested())
            .merge(Env::prefixed("DISCOVERY_").split("_"))
    }
}
