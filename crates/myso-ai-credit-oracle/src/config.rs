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

    /// PostgreSQL is the authoritative inference ledger and transactional outbox.
    /// Every replica must use the same database.
    #[arg(long, env = "AI_CREDIT_DATABASE_URL")]
    pub database_url: String,

    #[arg(long, env = "AI_CREDIT_DATABASE_MAX_CONNECTIONS", default_value = "10")]
    pub database_max_connections: u32,

    #[arg(long, env = "AI_CREDIT_OUTBOX_LEASE_SECS", default_value = "60")]
    pub outbox_lease_secs: u64,

    /// Declared deployment replica count. Multi-replica mode requires the legacy
    /// file-backed usage/settlement API to be disabled.
    #[arg(long, env = "AI_CREDIT_REPLICA_COUNT", default_value = "1")]
    pub replica_count: u32,

    #[arg(long, env = "AI_CREDIT_LEGACY_USAGE_ENABLED", default_value = "true")]
    pub legacy_usage_enabled: bool,

    #[arg(long, env = "AI_CREDIT_ORACLE_PRIVATE_KEY_HEX")]
    pub private_key_hex: String,

    #[arg(long, env = "AI_CREDIT_SETTLEMENT_SECRET")]
    pub settlement_secret: Option<String>,

    #[arg(
        long,
        env = "AI_CREDIT_MYSO_RPC",
        default_value = "http://127.0.0.1:9000"
    )]
    pub myso_rpc: String,

    #[arg(
        long,
        env = "AI_CREDIT_RECEIPT_STORE",
        default_value = "ai_credit_receipts.json"
    )]
    pub receipt_store_path: PathBuf,

    #[arg(long, env = "AI_CREDIT_CONFIG_OBJECT_ID")]
    pub config_object_id: Option<String>,

    #[arg(long, env = "AI_CREDIT_SETTLEMENT_KEY_HEX")]
    pub settlement_key_hex: Option<String>,

    /// Extra headroom applied to the deterministic, bounded provider envelope.
    /// This protects against catalog/route price movement between reserve and capture;
    /// it is not a substitute for enforcing request token limits.
    #[arg(
        long,
        env = "AI_CREDIT_RESERVATION_PRICE_BUFFER_BPS",
        default_value = "2500"
    )]
    pub reservation_price_buffer_bps: u64,

    #[arg(
        long,
        env = "AI_CREDIT_RESERVATION_CAPTURE_WINDOW_SECS",
        default_value = "600"
    )]
    pub reservation_capture_window_secs: u64,

    #[arg(
        long,
        env = "AI_CREDIT_RESERVATION_HARD_EXPIRY_SECS",
        default_value = "1800"
    )]
    pub reservation_hard_expiry_secs: u64,

    #[arg(long, env = "AI_CREDIT_SETTLEMENT_INTERVAL_SECS", default_value = "60")]
    pub settlement_interval_secs: u64,

    /// Per-balance unsettled MIST that triggers settlement (default 10 MYSO).
    #[arg(
        long,
        env = "AI_CREDIT_SETTLE_THRESHOLD_MIST",
        default_value = "10000000000"
    )]
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

    #[arg(
        long,
        env = "AI_CREDIT_SOCIAL_SERVER_URL",
        default_value = "http://127.0.0.1:9126"
    )]
    pub social_server_url: String,

    #[arg(
        long,
        env = "AI_CREDIT_PRICING_CATALOG_PATH",
        default_value = "config/pricing_catalog.toml"
    )]
    pub pricing_catalog_path: PathBuf,

    #[arg(long, env = "AI_CREDIT_ECOSYSTEM_MARGIN_PCT", default_value = "0.125")]
    pub ecosystem_margin_pct: f64,

    #[arg(
        long,
        env = "AI_CREDIT_GRAPHQL_URL",
        default_value = "http://127.0.0.1:9125/graphql"
    )]
    pub graphql_url: String,

    #[arg(
        long,
        env = "AI_CREDIT_MARKUP_REFRESH_INTERVAL_SECS",
        default_value = "300"
    )]
    pub markup_refresh_interval_secs: u64,

    #[arg(long, env = "AI_CREDIT_MARKUP_GRAPHQL_ENABLED", default_value = "true")]
    pub markup_graphql_enabled: bool,

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

    #[arg(
        long,
        env = "AI_CREDIT_PRICE_REFRESH_INTERVAL_SECS",
        default_value = "60"
    )]
    pub price_refresh_interval_secs: u64,

    #[arg(
        long,
        env = "AI_CREDIT_MYSO_PRICE_MAX_STALE_SECS",
        default_value = "300"
    )]
    pub myso_price_max_stale_secs: u64,

    #[arg(long, env = "AI_CREDIT_MYSO_PRICE_ENABLED", default_value = "true")]
    pub myso_price_enabled: bool,

    #[arg(long, env = "AI_CREDIT_OPENROUTER_API_KEY")]
    pub openrouter_api_key: Option<String>,

    #[arg(long, env = "AI_CREDIT_CATALOG_SYNC_ENABLED", default_value = "false")]
    pub catalog_sync_enabled: bool,

    #[arg(
        long,
        env = "AI_CREDIT_CATALOG_SYNC_INTERVAL_SECS",
        default_value = "86400"
    )]
    pub catalog_sync_interval_secs: u64,

    #[arg(
        long,
        env = "AI_CREDIT_CATALOG_SYNC_ON_STARTUP",
        default_value = "true"
    )]
    pub catalog_sync_on_startup: bool,

    #[arg(
        long,
        env = "AI_CREDIT_OPENROUTER_API_URL",
        default_value = "https://openrouter.ai/api/v1/models"
    )]
    pub openrouter_api_url: String,

    #[arg(
        long,
        env = "AI_CREDIT_OPENROUTER_CHAT_URL",
        default_value = "https://openrouter.ai/api/v1/chat/completions"
    )]
    pub openrouter_chat_url: String,

    /// Proxy live LLM calls via OpenRouter (`POST /v1/ai-credit/inference`).
    #[arg(long, env = "AI_CREDIT_INFERENCE_ENABLED", default_value = "false")]
    pub inference_enabled: bool,

    #[arg(long, env = "AI_CREDIT_CATALOG_MAX_DRIFT_PCT", default_value = "50.0")]
    pub catalog_max_drift_pct: f64,

    /// Enforce `require_approval_above_mist` with reject-before-sign gating.
    /// Deploy with `false`, flip once the indexer is caught up on approval events.
    #[arg(long, env = "AI_CREDIT_APPROVALS_ENABLED", default_value = "false")]
    pub approvals_enabled: bool,

    #[arg(long, env = "AI_CREDIT_APPROVAL_LOOKUP_TTL_SECS", default_value = "5")]
    pub approval_lookup_ttl_secs: u64,

    /// Don't accept an allowance that expires sooner than this (must outlive the
    /// settlement window, `settle_max_age_secs`).
    #[arg(
        long,
        env = "AI_CREDIT_APPROVAL_MIN_REMAINING_SECS",
        default_value = "180"
    )]
    pub approval_min_remaining_secs: u64,

    /// Workflow relayer base URL for ApprovalRequest inbox items (unset = disabled).
    #[arg(long, env = "AI_CREDIT_WORKFLOW_RELAYER_URL")]
    pub workflow_relayer_url: Option<String>,

    #[arg(long, env = "AI_CREDIT_WORKFLOW_SYNC_SECRET")]
    pub workflow_sync_secret: Option<String>,

    /// Shared secret for `POST /internal/audit/logs` on social-server.
    #[arg(long, env = "AI_CREDIT_AUDIT_SYNC_SECRET")]
    pub audit_sync_secret: Option<String>,

    /// Required on all `/v1/ai-credit/*` and `/usage-history` requests (`x-ai-credit-oracle-secret`).
    #[arg(long, env = "AI_CREDIT_ORACLE_API_SECRET")]
    pub oracle_api_secret: Option<String>,

    /// Require `AI_CREDIT_ORACLE_API_SECRET` at startup (set false for local dev).
    #[arg(long, env = "AI_CREDIT_REQUIRE_SECRETS", default_value = "true")]
    pub require_secrets: bool,

    /// Verify agent signatures on `POST /v1/ai-credit/usage` (set false for local dev).
    #[arg(long, env = "AI_CREDIT_AGENT_AUTH_ENABLED", default_value = "true")]
    pub agent_auth_enabled: bool,

    #[arg(long, env = "AI_CREDIT_AGENT_AUTH_TTL_SECS", default_value = "300")]
    pub agent_auth_ttl_secs: i64,

    /// Require `AI_CREDIT_SETTLEMENT_SECRET` at startup and on `/internal/ai-credit/settle`.
    #[arg(
        long,
        env = "AI_CREDIT_REQUIRE_SETTLEMENT_SECRET",
        default_value = "true"
    )]
    pub require_settlement_secret: bool,

    /// After corrupt receipt JSON backup, allow empty store on load.
    #[arg(long, env = "AI_CREDIT_RECEIPT_STORE_RECOVER", default_value = "false")]
    pub receipt_store_recover: bool,

    #[arg(
        long,
        env = "AI_CREDIT_INGEST_RECONCILE_INTERVAL_SECS",
        default_value = "30"
    )]
    pub ingest_reconcile_interval_secs: u64,

    #[arg(
        long,
        env = "AI_CREDIT_INGEST_BACKLOG_WARN_AGE_SECS",
        default_value = "300"
    )]
    pub ingest_backlog_warn_age_secs: u64,

    /// Bearer token for OpenAI-compatible `/v1/{models,chat/completions,responses}`
    /// (OpenClaw / Hermes). When unset, those routes are not mounted.
    #[arg(long, env = "AI_CREDIT_PROVIDER_TOKEN")]
    pub provider_token: Option<String>,

    /// Owner address mapped from the OpenAI provider bearer token.
    #[arg(long, env = "AI_CREDIT_PROVIDER_OWNER")]
    pub provider_owner: Option<String>,

    /// On-chain AI credit balance object mapped from the provider bearer token.
    #[arg(long, env = "AI_CREDIT_PROVIDER_BALANCE_ID")]
    pub provider_balance_id: Option<String>,

    /// MemoryAccount object mapped from the provider bearer token.
    #[arg(long, env = "AI_CREDIT_PROVIDER_MEMORY_ACCOUNT_ID")]
    pub provider_memory_account_id: Option<String>,

    /// Sub-agent object used for CAP_AI_SPEND / reservation metering.
    #[arg(long, env = "AI_CREDIT_PROVIDER_AGENT_OBJECT_ID")]
    pub provider_agent_object_id: Option<String>,

    /// Comma-separated model ids advertised by `GET /v1/models`.
    /// When empty, the pricing catalog aliases are used.
    #[arg(long, env = "AI_CREDIT_PROVIDER_MODELS")]
    pub provider_models: Option<String>,
}

impl OracleArgs {
    pub fn catalog_sync_active(&self) -> bool {
        self.catalog_sync_enabled && self.openrouter_api_key.is_some()
    }

    pub fn inference_active(&self) -> bool {
        self.inference_enabled && self.openrouter_api_key.is_some()
    }

    pub fn openai_provider_configured(&self) -> bool {
        self.nonempty_opt(&self.provider_token)
            && self.nonempty_opt(&self.provider_owner)
            && self.nonempty_opt(&self.provider_balance_id)
            && self.nonempty_opt(&self.provider_memory_account_id)
            && self.nonempty_opt(&self.provider_agent_object_id)
    }

    pub fn provider_model_ids(&self) -> Vec<String> {
        self.provider_models
            .as_deref()
            .map(|s| {
                s.split(',')
                    .map(str::trim)
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect::<Vec<_>>()
            })
            .filter(|v| !v.is_empty())
            .unwrap_or_default()
    }

    fn nonempty_opt(&self, value: &Option<String>) -> bool {
        value
            .as_deref()
            .is_some_and(|value| !value.trim().is_empty())
    }

    pub fn validate_startup(&self) -> anyhow::Result<()> {
        if self.database_url.trim().is_empty() {
            anyhow::bail!("AI_CREDIT_DATABASE_URL is required");
        }
        if self.database_max_connections == 0 {
            anyhow::bail!("AI_CREDIT_DATABASE_MAX_CONNECTIONS must be greater than zero");
        }
        if !(15..=300).contains(&self.outbox_lease_secs) {
            anyhow::bail!("AI_CREDIT_OUTBOX_LEASE_SECS must be between 15 and 300");
        }
        if self.replica_count == 0 {
            anyhow::bail!("AI_CREDIT_REPLICA_COUNT must be greater than zero");
        }
        if self.replica_count > 1 && self.legacy_usage_enabled {
            anyhow::bail!(
                "AI_CREDIT_LEGACY_USAGE_ENABLED must be false when AI_CREDIT_REPLICA_COUNT > 1"
            );
        }
        if self.require_secrets && self.oracle_api_secret.is_none() {
            anyhow::bail!(
                "AI_CREDIT_ORACLE_API_SECRET is required when AI_CREDIT_REQUIRE_SECRETS=true"
            );
        }
        if self.require_settlement_secret && self.settlement_secret.is_none() {
            anyhow::bail!(
                "AI_CREDIT_SETTLEMENT_SECRET is required when AI_CREDIT_REQUIRE_SETTLEMENT_SECRET=true"
            );
        }
        if self.inference_enabled {
            if self
                .openrouter_api_key
                .as_deref()
                .is_none_or(|value| value.trim().is_empty())
            {
                anyhow::bail!(
                    "AI_CREDIT_OPENROUTER_API_KEY is required when AI_CREDIT_INFERENCE_ENABLED=true"
                );
            }
            if self
                .config_object_id
                .as_deref()
                .is_none_or(|value| value.trim().is_empty())
                || self
                    .settlement_key_hex
                    .as_deref()
                    .is_none_or(|value| value.trim().is_empty())
            {
                anyhow::bail!(
                    "AI_CREDIT_CONFIG_OBJECT_ID and AI_CREDIT_SETTLEMENT_KEY_HEX are required when AI_CREDIT_INFERENCE_ENABLED=true"
                );
            }
            if self.reservation_capture_window_secs == 0
                || self.reservation_capture_window_secs > 15 * 60
            {
                anyhow::bail!(
                    "AI_CREDIT_RESERVATION_CAPTURE_WINDOW_SECS must be between 1 and 900"
                );
            }
            if self.reservation_hard_expiry_secs <= self.reservation_capture_window_secs
                || self.reservation_hard_expiry_secs > 30 * 60
            {
                anyhow::bail!(
                    "AI_CREDIT_RESERVATION_HARD_EXPIRY_SECS must be greater than capture window and at most 1800"
                );
            }
            if self.reservation_price_buffer_bps > 10_000 {
                anyhow::bail!("AI_CREDIT_RESERVATION_PRICE_BUFFER_BPS must be at most 10000");
            }
        }
        let provider_fields = [
            self.provider_token.as_deref(),
            self.provider_owner.as_deref(),
            self.provider_balance_id.as_deref(),
            self.provider_memory_account_id.as_deref(),
            self.provider_agent_object_id.as_deref(),
        ];
        let any_provider = provider_fields
            .iter()
            .any(|value| value.is_some_and(|value| !value.trim().is_empty()));
        if any_provider && !self.openai_provider_configured() {
            anyhow::bail!(
                "OpenAI provider requires AI_CREDIT_PROVIDER_TOKEN, _OWNER, _BALANCE_ID, _MEMORY_ACCOUNT_ID, and _AGENT_OBJECT_ID"
            );
        }
        if self.openai_provider_configured() && !self.inference_active() {
            anyhow::bail!(
                "OpenAI provider requires AI_CREDIT_INFERENCE_ENABLED=true and AI_CREDIT_OPENROUTER_API_KEY"
            );
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn blank_args() -> OracleArgs {
        OracleArgs {
            listen_addr: "0.0.0.0:8095".into(),
            database_url: "postgres://localhost/test".into(),
            database_max_connections: 10,
            outbox_lease_secs: 60,
            replica_count: 1,
            legacy_usage_enabled: true,
            private_key_hex: "00".repeat(32),
            settlement_secret: None,
            myso_rpc: "http://127.0.0.1:9000".into(),
            receipt_store_path: std::path::PathBuf::from("test.json"),
            config_object_id: None,
            settlement_key_hex: None,
            reservation_price_buffer_bps: 2500,
            reservation_capture_window_secs: 600,
            reservation_hard_expiry_secs: 1800,
            settlement_interval_secs: 60,
            settle_threshold_mist: 10_000_000_000,
            settle_max_age_secs: 180,
            settle_min_count: 8,
            settle_warn_age_secs: 240,
            social_server_url: "http://127.0.0.1:9126".into(),
            pricing_catalog_path: std::path::PathBuf::from("config/pricing_catalog.toml"),
            ecosystem_margin_pct: 0.125,
            graphql_url: "http://127.0.0.1:9125/graphql".into(),
            markup_refresh_interval_secs: 300,
            markup_graphql_enabled: false,
            usage_sync_secret: None,
            strict_catalog: false,
            myso_price_oracle_url: "https://example.invalid".into(),
            price_refresh_interval_secs: 60,
            myso_price_max_stale_secs: 300,
            myso_price_enabled: false,
            openrouter_api_key: None,
            catalog_sync_enabled: false,
            catalog_sync_interval_secs: 86400,
            catalog_sync_on_startup: true,
            openrouter_api_url: "https://openrouter.ai/api/v1/models".into(),
            openrouter_chat_url: "https://openrouter.ai/api/v1/chat/completions".into(),
            inference_enabled: false,
            catalog_max_drift_pct: 50.0,
            approvals_enabled: false,
            approval_lookup_ttl_secs: 5,
            approval_min_remaining_secs: 180,
            workflow_relayer_url: None,
            workflow_sync_secret: None,
            audit_sync_secret: None,
            oracle_api_secret: None,
            require_secrets: false,
            agent_auth_enabled: false,
            agent_auth_ttl_secs: 300,
            require_settlement_secret: false,
            receipt_store_recover: false,
            ingest_reconcile_interval_secs: 30,
            ingest_backlog_warn_age_secs: 300,
            provider_token: None,
            provider_owner: None,
            provider_balance_id: None,
            provider_memory_account_id: None,
            provider_agent_object_id: None,
            provider_models: None,
        }
    }

    #[test]
    fn openai_provider_requires_complete_mapping() {
        let mut args = blank_args();
        args.provider_token = Some("t".into());
        assert!(!args.openai_provider_configured());
        assert!(args.validate_startup().is_err());

        args.provider_owner = Some("0xo".into());
        args.provider_balance_id = Some("0xb".into());
        args.provider_memory_account_id = Some("0xm".into());
        args.provider_agent_object_id = Some("0xa".into());
        assert!(args.openai_provider_configured());
        // Still fails because inference is not active.
        assert!(args.validate_startup().is_err());

        args.inference_enabled = true;
        args.openrouter_api_key = Some("sk".into());
        args.config_object_id = Some("0xc".into());
        args.settlement_key_hex = Some("11".repeat(32));
        assert!(args.validate_startup().is_ok());
        assert_eq!(args.provider_model_ids(), Vec::<String>::new());
        args.provider_models = Some("openai/gpt-4o-mini, openai/gpt-4o".into());
        assert_eq!(
            args.provider_model_ids(),
            vec![
                "openai/gpt-4o-mini".to_string(),
                "openai/gpt-4o".to_string()
            ]
        );
    }
}
