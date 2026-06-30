// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::path::PathBuf;

use clap::Parser;

/// On-chain `social_contracts` package ID (localnet / framework default).
pub const SOCIAL_PACKAGE_ID: &str = "0x50c1";

#[derive(Debug, Clone, Parser)]
pub struct OracleArgs {
    #[arg(long, env = "AI_CREDIT_ORACLE_LISTEN", default_value = "0.0.0.0:8095")]
    pub listen_addr: String,

    #[arg(long, env = "AI_CREDIT_ORACLE_PRIVATE_KEY_HEX")]
    pub private_key_hex: String,

    #[arg(long, env = "AI_CREDIT_SETTLEMENT_SECRET")]
    pub settlement_secret: Option<String>,

    #[arg(long, env = "AI_CREDIT_MYSO_RPC", default_value = "http://127.0.0.1:9001")]
    pub myso_rpc: String,

    #[arg(long, env = "AI_CREDIT_RECEIPT_STORE", default_value = "ai_credit_receipts.json")]
    pub receipt_store_path: PathBuf,

    #[arg(long, env = "AI_CREDIT_CONFIG_OBJECT_ID")]
    pub config_object_id: Option<String>,

    #[arg(long, env = "AI_CREDIT_SETTLEMENT_KEY_HEX")]
    pub settlement_key_hex: Option<String>,

    #[arg(long, env = "AI_CREDIT_SETTLEMENT_INTERVAL_SECS", default_value = "60")]
    pub settlement_interval_secs: u64,

    /// Per-balance unsettled MIST that triggers settlement (default 10 MYSO).
    #[arg(long, env = "AI_CREDIT_SETTLE_THRESHOLD_MIST", default_value = "10000000000")]
    pub settle_threshold_mist: u64,

    /// Max age of oldest unsettled receipt before settlement (seconds).
    #[arg(long, env = "AI_CREDIT_SETTLE_MAX_AGE_SECS", default_value = "180")]
    pub settle_max_age_secs: u64,

    /// Unsettled receipt count per balance that triggers settlement.
    #[arg(long, env = "AI_CREDIT_SETTLE_MIN_COUNT", default_value = "8")]
    pub settle_min_count: u64,

    /// Log warning when oldest pending receipt exceeds this age (seconds).
    #[arg(long, env = "AI_CREDIT_SETTLE_WARN_AGE_SECS", default_value = "240")]
    pub settle_warn_age_secs: u64,

    #[arg(long, env = "AI_CREDIT_SOCIAL_SERVER_URL", default_value = "http://127.0.0.1:9126")]
    pub social_server_url: String,

    #[arg(
        long,
        env = "AI_CREDIT_PRICING_CATALOG_PATH",
        default_value = "config/pricing_catalog.toml"
    )]
    pub pricing_catalog_path: PathBuf,

    #[arg(long, env = "AI_CREDIT_ECOSYSTEM_MARGIN_PCT", default_value = "0.125")]
    pub ecosystem_margin_pct: f64,

    #[arg(long, env = "AI_CREDIT_USAGE_SYNC_SECRET")]
    pub usage_sync_secret: Option<String>,

    /// Reject record_usage when model_id / tool_id is not in the pricing catalog.
    #[arg(long, env = "AI_CREDIT_STRICT_CATALOG", default_value = "false")]
    pub strict_catalog: bool,

    #[arg(
        long,
        env = "AI_CREDIT_MYSO_PRICE_ORACLE_URL",
        default_value = "https://myso-price-oracle-testnet.up.railway.app"
    )]
    pub myso_price_oracle_url: String,

    #[arg(long, env = "AI_CREDIT_PRICE_REFRESH_INTERVAL_SECS", default_value = "60")]
    pub price_refresh_interval_secs: u64,

    #[arg(long, env = "AI_CREDIT_MYSO_PRICE_MAX_STALE_SECS", default_value = "300")]
    pub myso_price_max_stale_secs: u64,

    #[arg(long, env = "AI_CREDIT_MYSO_PRICE_ENABLED", default_value = "true")]
    pub myso_price_enabled: bool,
}
