// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Parser;
use figment::{providers::Env, providers::Format, providers::Toml, Figment};
use serde::Deserialize;

/// On-chain `myso-social` package ID (localnet / framework default). Used by the
/// blockchain module to resolve `social_proof_of_truth` entrypoints.
pub const SOCIAL_PACKAGE_ID: &str = "0x50c1";

#[derive(Debug, Clone, Parser)]
pub struct OracleArgs {
    #[arg(long, env = "SPOT_ORACLE_LISTEN", default_value = "0.0.0.0:8097")]
    pub listen_addr: String,

    #[arg(
        long,
        env = "SPOT_ORACLE_METRICS_ADDRESS",
        default_value = "0.0.0.0:9187"
    )]
    pub metrics_addr: SocketAddr,

    /// Postgres DSN for the `spot_oracle` DB (runs discovery + spot schemas).
    #[arg(
        long,
        env = "SPOT_ORACLE_DATABASE_URL",
        default_value = "postgresql://spot:spot@127.0.0.1:5435/spot_oracle"
    )]
    pub database_url: String,

    #[arg(long, env = "SPOT_ORACLE_DB_MAX_CONNECTIONS", default_value = "10")]
    pub db_max_connections: u32,

    /// Path to the discovery source-registration YAML (where to fetch, not fake data).
    #[arg(
        long,
        env = "SPOT_ORACLE_SOURCES_CONFIG",
        default_value = "crates/myso-spot-oracle/config/discovery/sources.localnet.yaml"
    )]
    pub sources_config: PathBuf,

    /// MySo RPC URL for PTB submission (create / resolve / refund).
    #[arg(long, env = "SPOT_ORACLE_MYSO_RPC", default_value = "http://127.0.0.1:9000")]
    pub myso_rpc: String,

    /// Oracle signer private key (hex). Required to submit `oracle_resolve` / create PTBs.
    #[arg(long, env = "SPOT_ORACLE_PRIVATE_KEY_HEX")]
    pub private_key_hex: Option<String>,

    /// `SpotConfig` shared object ID (required for `create_spot_record_for_post`).
    #[arg(long, env = "SPOT_ORACLE_SPOT_CONFIG_OBJECT_ID")]
    pub spot_config_object_id: Option<String>,

    /// `SpotOracleAdminCap` object ID (required for oracle-only entrypoints).
    #[arg(long, env = "SPOT_ORACLE_ADMIN_CAP_OBJECT_ID")]
    pub admin_cap_object_id: Option<String>,

    /// Shared `SpotClaimRegistry` object ID (required for claim/market PTBs).
    #[arg(long, env = "SPOT_ORACLE_REGISTRY_OBJECT_ID")]
    pub spot_registry_object_id: Option<String>,

    /// Shared `Platform` object ID (required for on-chain `oracle_resolve`).
    #[arg(long, env = "SPOT_ORACLE_PLATFORM_OBJECT_ID")]
    pub platform_object_id: Option<String>,

    /// Shared `EcosystemTreasury` object ID (required for on-chain `oracle_resolve`).
    #[arg(long, env = "SPOT_ORACLE_ECOSYSTEM_TREASURY_OBJECT_ID")]
    pub ecosystem_treasury_object_id: Option<String>,

    /// Social-server base URL (pending-posts ingestion + indexed SPoT reads).
    #[arg(
        long,
        env = "SPOT_ORACLE_SOCIAL_SERVER_URL",
        default_value = "http://127.0.0.1:9126"
    )]
    pub social_server_url: String,

    /// Shared secret for `GET /spot/pending-posts` on social-server.
    #[arg(long, env = "SPOT_ORACLE_SOCIAL_SYNC_SECRET")]
    pub social_sync_secret: Option<String>,

    /// Secret-gated admin API header (`x-spot-oracle-admin-secret`).
    #[arg(long, env = "SPOT_ORACLE_ADMIN_SECRET")]
    pub admin_secret: Option<String>,

    /// OpenRouter LLM provider (NLU extraction only — never approves/resolves).
    #[arg(long, env = "SPOT_ORACLE_OPENROUTER_API_URL", default_value = "https://openrouter.ai/api/v1/chat/completions")]
    pub openrouter_api_url: String,
    #[arg(long, env = "SPOT_ORACLE_OPENROUTER_API_KEY")]
    pub openrouter_api_key: Option<String>,
    #[arg(long, env = "SPOT_ORACLE_LLM_MODEL", default_value = "openai/gpt-4o-mini")]
    pub llm_model: String,

    // Worker cadence (seconds).
    #[arg(long, env = "SPOT_ORACLE_REVIEW_POLL_INTERVAL_SECS", default_value = "15")]
    pub review_poll_interval_secs: u64,
    #[arg(long, env = "SPOT_ORACLE_SCHEDULER_POLL_INTERVAL_SECS", default_value = "10")]
    pub scheduler_poll_interval_secs: u64,
    #[arg(long, env = "SPOT_ORACLE_RESOLVER_CONCURRENCY", default_value = "4")]
    pub resolver_concurrency: usize,
    #[arg(long, env = "SPOT_ORACLE_RSS_POLL_INTERVAL_SECS", default_value = "60")]
    pub rss_poll_interval_secs: u64,
    #[arg(long, env = "SPOT_ORACLE_CHAIN_SUBMIT_INTERVAL_SECS", default_value = "10")]
    pub chain_submit_interval_secs: u64,
    #[arg(long, env = "SPOT_ORACLE_RECONCILE_INTERVAL_SECS", default_value = "120")]
    pub reconcile_interval_secs: u64,

    /// Confidence threshold (bps) matching on-chain SpotConfig default (70%).
    #[arg(long, env = "SPOT_ORACLE_CONFIDENCE_THRESHOLD_BPS", default_value = "7000")]
    pub confidence_threshold_bps: u64,

    /// Persist raw evidence bodies (config: `SPOT_ORACLE_STORE_RAW_EVIDENCE=true`).
    #[arg(long, env = "SPOT_ORACLE_STORE_RAW_EVIDENCE", default_value = "true")]
    pub store_raw_evidence: bool,

    /// Enable live external source calls in tests/smoke runs.
    #[arg(long, env = "SPOT_ORACLE_LIVE_SOURCES", default_value = "false")]
    pub live_sources: bool,

    /// Master switch for background workers (API + metrics still serve when false).
    #[arg(long, env = "SPOT_ORACLE_ENABLED", default_value = "true")]
    pub enabled: bool,

    /// gRPC endpoint for `SubscribeCheckpoints` claim ingest.
    #[arg(long, env = "SPOT_ORACLE_STREAMING_URL")]
    pub streaming_url: Option<String>,

    /// Claim ingest mode: `checkpoint`, `http`, or `both`.
    #[arg(long, env = "SPOT_ORACLE_INGEST_MODE", default_value = "checkpoint")]
    pub ingest_mode: String,

    /// Discovery service base URL for factual `/v1/*` settlement fetches.
    #[arg(long, env = "SPOT_ORACLE_DISCOVERY_CLIENT_URL")]
    pub discovery_client_url: Option<String>,

    /// Secret for `x-discovery-client-secret` on Discovery factual API.
    #[arg(long, env = "SPOT_ORACLE_DISCOVERY_CLIENT_SECRET")]
    pub discovery_client_secret: Option<String>,

    /// On-chain social package id for filtering `PostCreatedEvent`.
    #[arg(long, env = "SPOT_ORACLE_SOCIAL_PACKAGE_ID", default_value = "0x50c1")]
    pub social_package_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct OracleSettings {
    pub listen_addr: String,
    pub database_url: String,
}

impl OracleArgs {
    /// Figment layering: `config/default.toml` < `config/local.toml` < `SPOT_ORACLE_*` env.
    pub fn load_figment(&self) -> Figment {
        Figment::new()
            .merge(Toml::file("config/default.toml").nested())
            .merge(Toml::file("config/local.toml").nested())
            .merge(Env::prefixed("SPOT_ORACLE_").split("_"))
    }

    pub fn uses_checkpoint_ingest(&self) -> bool {
        matches!(self.ingest_mode.as_str(), "checkpoint" | "both")
    }

    pub fn uses_http_ingest(&self) -> bool {
        matches!(self.ingest_mode.as_str(), "http" | "both")
    }

    pub fn uses_discovery_client(&self) -> bool {
        self.discovery_client_url.is_some()
    }
}
