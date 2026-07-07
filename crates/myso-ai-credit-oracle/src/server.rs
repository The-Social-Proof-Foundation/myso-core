// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use tokio::sync::{Mutex, RwLock};

use crate::agent_auth::{
    agent_auth_error_to_status, check_oracle_api_secret, derive_receipt_id, verify_agent_usage_auth,
};
use crate::approvals::{approval_covers, ApprovalsCache};
use crate::catalog::{PricingCatalog, CAP_AI_SPEND};
use crate::catalog_sync::{spawn_catalog_sync_worker, startup_catalog_sync};
use crate::chain_balance;
use crate::config::OracleArgs;
use crate::graphql_client::MarkupConfigClient;
use crate::ledger::BalanceLedger;
use crate::markup_refresh::{spawn_markup_refresh_worker, startup_markup_refresh};
use crate::myso_price_client::MysoPriceClient;
use crate::openrouter_client::OpenRouterClient;
use crate::price_refresh::{spawn_price_refresh_worker, startup_price_refresh};
use crate::pricing::{
    PriceBreakdown, PricingEngine, CATALOG_USD_PEG, DEFAULT_ORACLE_MARKUP_BPS, USAGE_EMBED,
    USAGE_INFERENCE, USAGE_TOOL,
};
use crate::receipt::{ReceiptStore, UsageLine};
use crate::settlement_coordinator::{
    spawn_settlement_worker, SettlementCoordinator, SettlementMode,
};
use crate::signing::{parse_object_id_hex, ReceiptSigner, UsageReceipt};
use crate::social_client::{
    IngestApprovalRequest, IngestAuditLogEntry, IngestUsageLineRequest, SocialClient,
    SocialSubAgent,
};
use crate::workflow_client::{approval_idempotency_key, WorkflowClient, WorkflowItemIngest};

pub const APPROVAL_REQUIRED_REASON: &str = "approval_required";

#[derive(Clone)]
pub struct AppState {
    pub signer: ReceiptSigner,
    pub pricing: Arc<RwLock<PricingEngine>>,
    pub ledger: BalanceLedger,
    pub social: SocialClient,
    pub store: Arc<Mutex<ReceiptStore>>,
    pub store_path: std::path::PathBuf,
    pub settlement_secret: Option<String>,
    pub oracle_api_secret: Option<String>,
    pub oracle_args: OracleArgs,
    pub settlement_coordinator: Arc<SettlementCoordinator>,
    pub catalog: Arc<RwLock<PricingCatalog>>,
    pub myso_price_oracle_url: String,
    pub approvals: ApprovalsCache,
    pub workflow: Option<WorkflowClient>,
    pub openrouter: Option<OpenRouterClient>,
}

/// Structured spend-policy rejection so approval gating can carry context to the caller
/// and the side-effect pipeline (requested-approval ingest, inbox item, audit entry).
#[derive(Debug, Clone)]
pub enum SpendPolicyError {
    Denied(String),
    ApprovalRequired {
        balance_id: String,
        threshold_mist: u64,
        organization_id: Option<String>,
    },
}

impl SpendPolicyError {
    pub fn reason(&self) -> String {
        match self {
            SpendPolicyError::Denied(reason) => reason.clone(),
            SpendPolicyError::ApprovalRequired { .. } => APPROVAL_REQUIRED_REASON.to_string(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PricingFxInfo {
    pub catalog_usd_peg: f64,
    pub myso_usd: f64,
    pub price_oracle_url: String,
    pub price_age_secs: Option<u64>,
    pub price_stale: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct PreflightRequest {
    pub owner: String,
    pub agent_object_id: String,
    pub operation: String,
    pub model_id: Option<String>,
    pub estimated_tokens_in: u64,
    pub estimated_tokens_out: u64,
    pub fact_count: Option<u64>,
}

#[derive(Debug, serde::Serialize)]
pub struct PreflightResponse {
    pub allowed: bool,
    pub reason: Option<String>,
    pub estimated_mist: Option<u64>,
    pub effective_available_mist: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_mist: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_mist: Option<u64>,
    /// True when the spend exceeds the agent's approval threshold and no live allowance
    /// covers it; the owner (or an org spend approver) must approve on-chain first.
    pub approval_required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_threshold_mist: Option<u64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EstimateRequest {
    pub operation: Option<String>,
    pub model_id: Option<String>,
    pub tokens_in: u64,
    pub tokens_out: u64,
    pub usage_kind: Option<u8>,
    pub tool_id: Option<String>,
    pub fact_count: Option<u64>,
}

#[derive(Debug, serde::Serialize)]
pub struct EstimateResponse {
    pub estimated_mist: u64,
    pub estimated_credits: f64,
    pub base_mist: u64,
    pub margin_mist: u64,
    pub ecosystem_margin_pct: f64,
    pub oracle_markup_bps: u64,
    pub catalog_version: String,
    pub catalog_usd_peg: f64,
    pub myso_usd: f64,
    pub price_oracle_url: String,
    pub price_age_secs: Option<u64>,
    pub price_stale: bool,
}

#[derive(Debug, serde::Deserialize)]
pub struct UsageRequest {
    pub owner: String,
    pub balance_id: String,
    pub memory_account_id: String,
    pub agent_object_id: String,
    pub usage_kind: u8,
    pub tokens_in: Option<u64>,
    pub tokens_out: Option<u64>,
    pub tool_id: Option<String>,
    pub model_id: Option<String>,
    pub idempotency_key: String,
}

#[derive(Debug, serde::Serialize)]
pub struct UsageResponse {
    pub receipt_id: u128,
    pub amount_mist: u64,
    pub settlement_nonce: u64,
    pub signature: String,
    pub receipt: UsageReceipt,
}

#[derive(Debug, serde::Deserialize)]
pub struct InferenceRequest {
    pub owner: String,
    pub balance_id: String,
    pub memory_account_id: String,
    pub agent_object_id: String,
    pub model_id: String,
    pub prompt: String,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    pub idempotency_key: String,
}

#[derive(Debug, serde::Serialize)]
pub struct InferenceResponse {
    pub receipt_id: u128,
    pub amount_mist: u64,
    pub settlement_nonce: u64,
    pub signature: String,
    pub receipt: UsageReceipt,
    pub tokens_in: u64,
    pub tokens_out: u64,
    pub model_id: String,
    pub content: String,
}

pub async fn serve(args: OracleArgs) -> anyhow::Result<()> {
    args.validate_startup()?;
    let signer = ReceiptSigner::from_hex(&args.private_key_hex)?;
    tracing::info!(public_key = %signer.public_key_hex(), "oracle signer ready");

    let catalog_path = if args.pricing_catalog_path.is_absolute() {
        args.pricing_catalog_path.clone()
    } else {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&args.pricing_catalog_path)
    };
    let catalog = PricingCatalog::load(&catalog_path)?;
    let catalog = Arc::new(RwLock::new(catalog));
    let initial_margin_pct = DEFAULT_ORACLE_MARKUP_BPS as f64 / 10_000.0;
    let pricing = Arc::new(RwLock::new(PricingEngine::new(
        catalog.read().await.clone(),
        initial_margin_pct,
    )));
    let markup_client =
        MarkupConfigClient::new(args.graphql_url.clone(), args.social_server_url.clone());
    startup_markup_refresh(&args, &pricing, &markup_client).await;
    let myso_price_client = MysoPriceClient::new(args.myso_price_oracle_url.clone());
    startup_price_refresh(&args, &pricing, &myso_price_client).await;
    spawn_price_refresh_worker(Arc::new(args.clone()), pricing.clone(), myso_price_client);

    let store = ReceiptStore::load(&args.receipt_store_path, args.receipt_store_recover)?;
    let store_arc = Arc::new(Mutex::new(store));
    let args_arc = Arc::new(args.clone());
    spawn_markup_refresh_worker(args_arc.clone(), pricing.clone(), markup_client);

    if args.catalog_sync_active() {
        let openrouter = OpenRouterClient::new(
            args.openrouter_api_url.clone(),
            args.openrouter_chat_url.clone(),
            args.openrouter_api_key
                .clone()
                .expect("catalog_sync_active implies key"),
        );
        startup_catalog_sync(
            &args,
            &catalog_path,
            catalog.clone(),
            pricing.clone(),
            openrouter.clone(),
        )
        .await;
        spawn_catalog_sync_worker(
            args_arc.clone(),
            catalog_path.clone(),
            catalog.clone(),
            pricing.clone(),
            openrouter,
        );
    } else if args.catalog_sync_enabled && args.openrouter_api_key.is_none() {
        tracing::error!(
            "AI_CREDIT_CATALOG_SYNC_ENABLED=true but AI_CREDIT_OPENROUTER_API_KEY is unset"
        );
    }

    let openrouter = if args.inference_active() {
        Some(OpenRouterClient::new(
            args.openrouter_api_url.clone(),
            args.openrouter_chat_url.clone(),
            args.openrouter_api_key
                .clone()
                .expect("inference_active implies key"),
        ))
    } else {
        None
    };
    if args.inference_enabled && openrouter.is_none() {
        tracing::warn!(
            "AI_CREDIT_INFERENCE_ENABLED=true but AI_CREDIT_OPENROUTER_API_KEY is unset; /v1/ai-credit/inference disabled"
        );
    } else if openrouter.is_some() {
        tracing::info!("OpenRouter inference proxy enabled");
    }

    let social = SocialClient::new(
        args.social_server_url.clone(),
        args.usage_sync_secret.clone(),
    );
    let ledger = BalanceLedger::new(social.clone());
    let approvals = ApprovalsCache::new(social.clone(), args.approval_lookup_ttl_secs);
    let workflow = WorkflowClient::from_args(
        args.workflow_relayer_url.as_ref(),
        args.workflow_sync_secret.as_ref(),
    );

    let settlement_coordinator = Arc::new(SettlementCoordinator::new(
        args_arc.clone(),
        store_arc.clone(),
        args.receipt_store_path.clone(),
    ));
    spawn_settlement_worker(settlement_coordinator.clone());

    let state = AppState {
        signer,
        pricing,
        ledger,
        social,
        store: store_arc,
        store_path: args.receipt_store_path.clone(),
        settlement_secret: args.settlement_secret.clone(),
        oracle_api_secret: args.oracle_api_secret.clone(),
        oracle_args: args.clone(),
        settlement_coordinator,
        catalog,
        myso_price_oracle_url: args.myso_price_oracle_url.clone(),
        approvals,
        workflow,
        openrouter,
    };

    spawn_ingest_reconcile_worker(state.clone());

    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/ai-credit/preflight", post(preflight))
        .route("/v1/ai-credit/estimate", post(estimate))
        .route("/v1/ai-credit/usage", post(record_usage))
        .route("/v1/ai-credit/inference", post(run_inference))
        .route("/v1/ai-credit/usage-history", get(usage_history))
        .route("/v1/ai-credit/catalog", get(get_catalog))
        .route("/internal/ai-credit/settle", post(trigger_settle))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&args.listen_addr).await?;
    tracing::info!(addr = %args.listen_addr, "myso-ai-credit-oracle listening");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let pricing = state.pricing.read().await;
    let max_stale = state.oracle_args.myso_price_max_stale_secs;
    let price_stale = state.oracle_args.myso_price_enabled && pricing.is_price_stale(max_stale);
    drop(pricing);

    let store = state.store.lock().await;
    let pending_receipts = store.lines.iter().filter(|l| !l.settled && !l.void).count() as u64;
    let ingest_backlog = store.ingest_backlog_count();
    if let Some(oldest) = store.oldest_ingest_backlog_ms() {
        let age_secs = (chrono::Utc::now().timestamp_millis() as u64 - oldest) / 1000;
        if age_secs >= state.oracle_args.ingest_backlog_warn_age_secs {
            tracing::warn!(
                ingest_backlog,
                age_secs,
                warn_age_secs = state.oracle_args.ingest_backlog_warn_age_secs,
                "ingest backlog aging"
            );
        }
    }
    drop(store);

    Json(HealthResponse {
        price_stale,
        pending_receipts,
        ingest_backlog,
        settlement_enabled: state.settlement_secret.is_some(),
        store_writable: ReceiptStore::probe_writable(&state.store_path),
    })
}

#[derive(Debug, serde::Serialize)]
struct HealthResponse {
    price_stale: bool,
    pending_receipts: u64,
    ingest_backlog: u64,
    settlement_enabled: bool,
    store_writable: bool,
}

fn validate_idempotency_key(key: &str) -> Result<(), StatusCode> {
    if key.is_empty() || key.len() > 128 {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(())
}

fn usage_response_from_line(line: &UsageLine) -> Result<UsageResponse, StatusCode> {
    let receipt = UsageReceipt {
        balance_id: parse_object_id_hex(&line.balance_id)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        agent_object_id: parse_object_id_hex(&line.agent_object_id)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        receipt_id: line.receipt_id,
        amount_mist: line.amount_mist,
        usage_kind: line.usage_kind,
        timestamp_ms: line.timestamp_ms,
        settlement_nonce: line.settlement_nonce,
    };
    Ok(UsageResponse {
        receipt_id: line.receipt_id,
        amount_mist: line.amount_mist,
        settlement_nonce: line.settlement_nonce,
        signature: line.signature_hex.clone(),
        receipt,
    })
}

fn inference_response_from_line(line: &UsageLine) -> Result<InferenceResponse, StatusCode> {
    let usage = usage_response_from_line(line)?;
    let metadata = line.metadata.as_ref();
    let content = metadata
        .and_then(|m| m.get("content"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let tokens_in = metadata
        .and_then(|m| m.get("tokens_in"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let tokens_out = metadata
        .and_then(|m| m.get("tokens_out"))
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let model_id = line
        .model_id
        .clone()
        .unwrap_or_else(|| "unknown".to_string());
    Ok(InferenceResponse {
        receipt_id: usage.receipt_id,
        amount_mist: usage.amount_mist,
        settlement_nonce: usage.settlement_nonce,
        signature: usage.signature,
        receipt: usage.receipt,
        tokens_in,
        tokens_out,
        model_id,
        content,
    })
}

fn estimate_prompt_tokens(prompt: &str) -> u64 {
    ((prompt.len() as u64).div_ceil(4)).max(1)
}

fn inference_response_from_usage(
    usage: UsageResponse,
    model_id: &str,
    content: &str,
    tokens_in: u64,
    tokens_out: u64,
) -> InferenceResponse {
    InferenceResponse {
        receipt_id: usage.receipt_id,
        amount_mist: usage.amount_mist,
        settlement_nonce: usage.settlement_nonce,
        signature: usage.signature,
        receipt: usage.receipt,
        tokens_in,
        tokens_out,
        model_id: model_id.to_string(),
        content: content.to_string(),
    }
}

fn spawn_ingest_reconcile_worker(state: AppState) {
    let interval_secs = state.oracle_args.ingest_reconcile_interval_secs;
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(std::time::Duration::from_secs(interval_secs));
        loop {
            ticker.tick().await;
            run_ingest_reconcile_cycle(&state).await;
        }
    });
}

async fn run_ingest_reconcile_cycle(state: &AppState) {
    let pending: Vec<IngestUsageLineRequest> = {
        let store = state.store.lock().await;
        store
            .lines
            .iter()
            .filter(|l| !l.ingest_synced && !l.settled && !l.void)
            .map(|l| IngestUsageLineRequest {
                receipt_id: l.receipt_id.to_string(),
                balance_id: l.balance_id.clone(),
                agent_object_id: l.agent_object_id.clone(),
                usage_kind: l.usage_kind as i16,
                amount_mist: l.amount_mist as i64,
                model_id: l.model_id.clone(),
                tool_id: l.tool_id.clone(),
                metadata: l.metadata.clone(),
                organization_id: l.organization_id.clone(),
            })
            .collect()
    };

    for ingest in pending {
        if state
            .social
            .ingest_usage_line_with_retries(&ingest, 3)
            .await
            .is_ok()
        {
            if let Ok(receipt_id) = ingest.receipt_id.parse::<u128>() {
                let mut store = state.store.lock().await;
                if store.mark_ingest_synced(receipt_id) {
                    if let Err(err) = store.save(&state.store_path) {
                        tracing::warn!(error = %err, receipt_id, "failed to persist ingest_synced flag");
                    }
                }
            }
        }
    }
}

fn pricing_fx_from_engine(state: &AppState, pricing: &PricingEngine) -> PricingFxInfo {
    let max_stale = state.oracle_args.myso_price_max_stale_secs;
    PricingFxInfo {
        catalog_usd_peg: CATALOG_USD_PEG,
        myso_usd: pricing.myso_usd(),
        price_oracle_url: state.myso_price_oracle_url.clone(),
        price_age_secs: pricing.price_age_secs(),
        price_stale: state.oracle_args.myso_price_enabled && pricing.is_price_stale(max_stale),
    }
}

fn price_unavailable_for_usage(state: &AppState, pricing: &PricingEngine) -> bool {
    if !state.oracle_args.myso_price_enabled {
        return false;
    }
    if !pricing.price_ever_fetched() {
        return true;
    }
    pricing.is_price_stale(state.oracle_args.myso_price_max_stale_secs)
}

fn estimate_breakdown(pricing: &PricingEngine, req: &EstimateRequest) -> PriceBreakdown {
    let usage_kind = req.usage_kind.unwrap_or(USAGE_INFERENCE);
    if usage_kind == USAGE_TOOL {
        return pricing.tool_breakdown(req.tool_id.as_deref().unwrap_or("default"));
    }
    if usage_kind == USAGE_EMBED {
        return pricing.embedding_breakdown(
            req.model_id.as_deref().unwrap_or("text-embedding-3-small"),
            req.tokens_in.max(1),
        );
    }

    let model = req.model_id.as_deref().unwrap_or("openai/gpt-4o-mini");
    let mut tokens_in = req.tokens_in;
    let tokens_out = req.tokens_out;
    let operation = req.operation.as_deref().unwrap_or("inference");

    if operation == "analyze" {
        let facts = req.fact_count.unwrap_or(5);
        tokens_in += facts * 500;
        let infer = pricing.inference_breakdown(model, tokens_in, tokens_out);
        let embed = pricing.embedding_breakdown("text-embedding-3-small", facts * 200);
        return combine_breakdowns(infer, embed);
    }
    if operation == "remember" || operation == "embed" {
        return pricing.embedding_breakdown(model, tokens_in.max(1));
    }
    if operation == "ask" {
        let embed = pricing.embedding_breakdown(model, tokens_in.max(1));
        let infer = pricing.inference_breakdown(model, tokens_in, tokens_out);
        return combine_breakdowns(embed, infer);
    }
    pricing.inference_breakdown(model, tokens_in, tokens_out)
}

fn combine_breakdowns(a: PriceBreakdown, b: PriceBreakdown) -> PriceBreakdown {
    let base_mist = a.base_mist + b.base_mist;
    let amount_mist = a.amount_mist + b.amount_mist;
    let margin_mist = amount_mist.saturating_sub(base_mist);
    PriceBreakdown {
        base_mist,
        margin_mist,
        amount_mist,
    }
}

fn compute_usage_breakdown(
    state: &AppState,
    pricing: &PricingEngine,
    catalog: &PricingCatalog,
    req: &UsageRequest,
) -> Result<PriceBreakdown, String> {
    if state.oracle_args.strict_catalog {
        match req.usage_kind {
            USAGE_TOOL => {
                let tool_id = req.tool_id.as_deref().unwrap_or("default");
                if !catalog.is_known_tool(tool_id) {
                    return Err("unknown_tool_id".into());
                }
            }
            USAGE_EMBED => {
                let model = req.model_id.as_deref().unwrap_or("text-embedding-3-small");
                if !catalog.is_known_embedding_model(model) {
                    return Err("unknown_embedding_model_id".into());
                }
            }
            _ => {
                let model = req.model_id.as_deref().unwrap_or("openai/gpt-4o-mini");
                if !catalog.is_known_inference_model(model) {
                    return Err("unknown_model_id".into());
                }
            }
        }
    }

    Ok(match req.usage_kind {
        USAGE_TOOL => pricing.tool_breakdown(req.tool_id.as_deref().unwrap_or("default")),
        USAGE_EMBED => pricing.embedding_breakdown(
            req.model_id.as_deref().unwrap_or("text-embedding-3-small"),
            req.tokens_in.unwrap_or(0),
        ),
        _ => pricing.inference_breakdown(
            req.model_id.as_deref().unwrap_or("openai/gpt-4o-mini"),
            req.tokens_in.unwrap_or(0),
            req.tokens_out.unwrap_or(0),
        ),
    })
}

async fn validate_spend_policy(
    state: &AppState,
    owner: &str,
    balance_id: Option<&str>,
    agent_object_id: &str,
    amount_mist: u64,
    store: &ReceiptStore,
) -> Result<(u64, SocialSubAgent), SpendPolicyError> {
    let denied = SpendPolicyError::Denied;

    let balance_resp = state
        .ledger
        .fetch_balance(owner)
        .await
        .map_err(|e| denied(format!("balance_fetch_failed: {e}")))?
        .ok_or_else(|| denied("no_ai_credit_balance".to_string()))?;

    if !balance_resp.balance.active {
        return Err(denied("balance_inactive".into()));
    }

    if let Some(expected) = balance_id {
        if balance_resp.balance.balance_id != expected {
            return Err(denied("balance_id_mismatch".into()));
        }
    }

    let effective = BalanceLedger::effective_available_mist(&balance_resp, store);
    if amount_mist > effective {
        return Err(denied("insufficient_ai_credits".into()));
    }

    let agent = state
        .social
        .get_sub_agent_by_object_id(agent_object_id)
        .await
        .map_err(|e| denied(format!("agent_fetch_failed: {e}")))?;

    if !agent.active || agent.revoked_at_ms.is_some() {
        return Err(denied("sub_agent_not_active".into()));
    }
    if agent.capabilities & CAP_AI_SPEND as i64 != CAP_AI_SPEND as i64 {
        return Err(denied("missing_cap_ai_spend".into()));
    }
    if let Some(exp) = agent.expires_at_ms {
        if chrono::Utc::now().timestamp_millis() > exp {
            return Err(denied("sub_agent_expired".into()));
        }
    }

    if let Some(daily) = balance_resp.balance.daily_cap_mist {
        if balance_resp.balance.spent_day_mist + amount_mist as i64 > daily {
            return Err(denied("daily_cap_exceeded".into()));
        }
    }
    if let Some(monthly) = balance_resp.balance.monthly_cap_mist {
        if balance_resp.balance.spent_month_mist + amount_mist as i64 > monthly {
            return Err(denied("monthly_cap_exceeded".into()));
        }
    }

    if let Some(budget) = balance_resp
        .agent_budgets
        .iter()
        .find(|b| b.agent_object_id == agent_object_id)
    {
        if !budget.enabled {
            return Err(denied("agent_budget_disabled".into()));
        }
        if let Some(max) = budget.budget_mist {
            if budget.spent_mist + amount_mist as i64 > max {
                return Err(denied("agent_budget_exceeded".into()));
            }
        }

        // Reject-before-sign approval gate: an over-threshold spend never produces a
        // signed receipt (and never consumes a settlement nonce) until a live allowance
        // is indexed — so approvals can never block the sequential nonce queue.
        if state.oracle_args.approvals_enabled {
            if let Some(threshold) = budget.require_approval_above_mist {
                if threshold >= 0 && amount_mist > threshold as u64 {
                    let approval = state
                        .approvals
                        .fetch_approved(owner, &balance_resp.balance.balance_id, agent_object_id)
                        .await
                        .map_err(|e| denied(format!("approval_lookup_failed: {e}")))?;
                    let now_ms = chrono::Utc::now().timestamp_millis();
                    let min_remaining_ms =
                        (state.oracle_args.approval_min_remaining_secs * 1000) as i64;
                    let covered = approval
                        .as_ref()
                        .map(|row| approval_covers(row, amount_mist, now_ms, min_remaining_ms))
                        .unwrap_or(false);
                    if !covered {
                        return Err(SpendPolicyError::ApprovalRequired {
                            balance_id: balance_resp.balance.balance_id.clone(),
                            threshold_mist: threshold as u64,
                            organization_id: agent.organization_id.clone(),
                        });
                    }
                }
            }
        }
    }

    Ok((effective, agent))
}

/// Fire-and-forget side effects when an over-threshold spend is rejected: upsert the
/// `requested` approval row, surface an ApprovalRequest inbox item, and audit the request.
/// Never blocks or fails the caller.
fn spawn_approval_request_side_effects(
    state: &AppState,
    owner: String,
    balance_id: String,
    agent_object_id: String,
    organization_id: Option<String>,
    requested_amount_mist: u64,
    threshold_mist: u64,
) {
    let social = state.social.clone();
    let approvals = state.approvals.clone();
    let workflow = state.workflow.clone();
    let audit_secret = state.oracle_args.audit_sync_secret.clone();
    tokio::spawn(async move {
        let ingest = IngestApprovalRequest {
            balance_id: balance_id.clone(),
            agent_object_id: agent_object_id.clone(),
            requested_amount_mist: Some(requested_amount_mist as i64),
            threshold_mist: Some(threshold_mist as i64),
            organization_id: organization_id.clone(),
        };
        if let Err(err) = social.ingest_requested_approval(&ingest).await {
            tracing::warn!(error = %err, "failed to ingest requested approval");
        }
        approvals.invalidate(&balance_id, &agent_object_id).await;

        if let Some(workflow) = workflow {
            let item = WorkflowItemIngest {
                idempotency_key: approval_idempotency_key(&balance_id, &agent_object_id),
                recipient_address: owner.clone(),
                item_type: "approval_request".to_string(),
                title: "AI spend approval requested".to_string(),
                body: Some(format!(
                    "Agent requested {} MIST (threshold {} MIST)",
                    requested_amount_mist, threshold_mist
                )),
                payload: serde_json::json!({
                    "balance_id": balance_id,
                    "agent_object_id": agent_object_id,
                    "requested_amount_mist": requested_amount_mist,
                    "threshold_mist": threshold_mist,
                    "organization_id": organization_id,
                }),
                organization_id: organization_id.clone(),
                account_id: None,
                source_service: "ai_credit_oracle".to_string(),
                action_deadline_ms: None,
            };
            if let Err(err) = workflow.ingest_item(&item).await {
                tracing::warn!(error = %err, "failed to ingest approval workflow item");
            }
        }

        let now_ms = chrono::Utc::now().timestamp_millis();
        let audit = IngestAuditLogEntry {
            source: "oracle".to_string(),
            actor_address: agent_object_id.clone(),
            actor_type: "agent".to_string(),
            action: "spend_approval_request".to_string(),
            target_type: "spend_approval".to_string(),
            target_id: agent_object_id.clone(),
            organization_id,
            account_id: None,
            prev_state: None,
            new_state: Some(serde_json::json!({
                "requested_amount_mist": requested_amount_mist,
                "threshold_mist": threshold_mist,
                "owner": owner,
            })),
            tx_digest: None,
            // Per-minute dedupe so repeated preflight retries don't spam the log.
            idempotency_key: Some(format!(
                "spend_approval_request:{}:{}:{}",
                balance_id,
                agent_object_id,
                now_ms / 60_000
            )),
            metadata: Some(serde_json::json!({ "balance_id": balance_id })),
        };
        if let Err(err) = social
            .ingest_audit_logs(audit_secret.as_deref(), vec![audit])
            .await
        {
            tracing::warn!(error = %err, "failed to ingest approval audit entry");
        }
    });
}

async fn preflight(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<PreflightRequest>,
) -> Result<Json<PreflightResponse>, StatusCode> {
    check_oracle_api_secret(&headers, &state.oracle_api_secret)?;
    let pricing = state.pricing.read().await;
    let breakdown = estimate_breakdown(
        &pricing,
        &EstimateRequest {
            operation: Some(req.operation.clone()),
            model_id: req.model_id.clone(),
            tokens_in: req.estimated_tokens_in,
            tokens_out: req.estimated_tokens_out,
            usage_kind: Some(USAGE_INFERENCE),
            tool_id: None,
            fact_count: req.fact_count,
        },
    );
    let estimated_mist = breakdown.amount_mist;
    if estimated_mist == 0 {
        return Ok(Json(PreflightResponse {
            allowed: false,
            reason: Some("estimated_mist must be > 0".into()),
            estimated_mist: Some(0),
            effective_available_mist: None,
            base_mist: Some(breakdown.base_mist),
            margin_mist: Some(breakdown.margin_mist),
            approval_required: false,
            approval_threshold_mist: None,
        }));
    }

    let store = state.store.lock().await;
    let response = match validate_spend_policy(
        &state,
        &req.owner,
        None,
        &req.agent_object_id,
        estimated_mist,
        &store,
    )
    .await
    {
        Ok((effective, _agent)) => PreflightResponse {
            allowed: true,
            reason: None,
            estimated_mist: Some(estimated_mist),
            effective_available_mist: Some(effective),
            base_mist: Some(breakdown.base_mist),
            margin_mist: Some(breakdown.margin_mist),
            approval_required: false,
            approval_threshold_mist: None,
        },
        Err(SpendPolicyError::ApprovalRequired {
            balance_id,
            threshold_mist,
            organization_id,
        }) => {
            spawn_approval_request_side_effects(
                &state,
                req.owner.clone(),
                balance_id,
                req.agent_object_id.clone(),
                organization_id,
                estimated_mist,
                threshold_mist,
            );
            PreflightResponse {
                allowed: false,
                reason: Some(APPROVAL_REQUIRED_REASON.to_string()),
                estimated_mist: Some(estimated_mist),
                effective_available_mist: None,
                base_mist: Some(breakdown.base_mist),
                margin_mist: Some(breakdown.margin_mist),
                approval_required: true,
                approval_threshold_mist: Some(threshold_mist),
            }
        }
        Err(err) => PreflightResponse {
            allowed: false,
            reason: Some(err.reason()),
            estimated_mist: Some(estimated_mist),
            effective_available_mist: None,
            base_mist: Some(breakdown.base_mist),
            margin_mist: Some(breakdown.margin_mist),
            approval_required: false,
            approval_threshold_mist: None,
        },
    };
    Ok(Json(response))
}

async fn estimate(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<EstimateRequest>,
) -> Result<Json<EstimateResponse>, StatusCode> {
    check_oracle_api_secret(&headers, &state.oracle_api_secret)?;
    let pricing = state.pricing.read().await;
    let breakdown = estimate_breakdown(&pricing, &req);
    let fx = pricing_fx_from_engine(&state, &pricing);
    Ok(Json(EstimateResponse {
        estimated_mist: breakdown.amount_mist,
        estimated_credits: pricing.credits_from_mist(breakdown.amount_mist),
        base_mist: breakdown.base_mist,
        margin_mist: breakdown.margin_mist,
        ecosystem_margin_pct: pricing.ecosystem_margin_pct(),
        oracle_markup_bps: pricing.oracle_markup_bps(),
        catalog_version: pricing.catalog_version().to_string(),
        catalog_usd_peg: fx.catalog_usd_peg,
        myso_usd: fx.myso_usd,
        price_oracle_url: fx.price_oracle_url,
        price_age_secs: fx.price_age_secs,
        price_stale: fx.price_stale,
    }))
}

async fn record_usage(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<UsageRequest>,
) -> Result<Json<UsageResponse>, StatusCode> {
    check_oracle_api_secret(&headers, &state.oracle_api_secret)?;
    validate_idempotency_key(&req.idempotency_key)?;
    if let Err(err) = verify_agent_usage_auth(
        &headers,
        &req.balance_id,
        &req.agent_object_id,
        req.usage_kind,
        req.tokens_in,
        req.tokens_out,
        &req.model_id,
        &req.tool_id,
        &req.idempotency_key,
        &state.oracle_args,
    )
    .await
    {
        return Err(agent_auth_error_to_status(err));
    }

    let usage = record_usage_core(&state, req, serde_json::Map::new()).await?;
    Ok(Json(usage))
}

async fn run_inference(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(req): Json<InferenceRequest>,
) -> Result<Json<InferenceResponse>, StatusCode> {
    check_oracle_api_secret(&headers, &state.oracle_api_secret)?;
    validate_idempotency_key(&req.idempotency_key)?;

    let openrouter = state.openrouter.as_ref().ok_or_else(|| {
        tracing::warn!("inference rejected: OpenRouter proxy not configured");
        StatusCode::SERVICE_UNAVAILABLE
    })?;

    if req.model_id.trim().is_empty() || req.prompt.trim().is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    {
        let store = state.store.lock().await;
        if let Some(existing) =
            store.find_by_idempotency(&req.balance_id, &req.agent_object_id, &req.idempotency_key)
        {
            return Ok(Json(inference_response_from_line(existing)?));
        }
    }

    let max_tokens = req.max_tokens.unwrap_or(64).max(1);

    let pricing = state.pricing.read().await;
    if price_unavailable_for_usage(&state, &pricing) {
        tracing::warn!("inference rejected: MYSO/USD price unavailable or stale");
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }
    let est_tokens_in = estimate_prompt_tokens(&req.prompt);
    let est_breakdown =
        pricing.inference_breakdown(&req.model_id, est_tokens_in, max_tokens as u64);
    let est_amount = est_breakdown.amount_mist;
    drop(pricing);

    {
        let store = state.store.lock().await;
        match validate_spend_policy(
            &state,
            &req.owner,
            Some(&req.balance_id),
            &req.agent_object_id,
            est_amount,
            &store,
        )
        .await
        {
            Ok(_) => {}
            Err(SpendPolicyError::ApprovalRequired { .. }) => {
                return Err(StatusCode::PAYMENT_REQUIRED);
            }
            Err(err) => {
                tracing::warn!(reason = %err.reason(), "inference preflight rejected");
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    let message = crate::openrouter_client::ChatMessage {
        role: "user",
        content: &req.prompt,
    };
    let completion = openrouter
        .chat_completions(&req.model_id, std::slice::from_ref(&message), max_tokens)
        .await
        .map_err(|err| {
            tracing::warn!(error = %err, model = %req.model_id, "openrouter inference failed");
            StatusCode::BAD_GATEWAY
        })?;

    let tokens_in = completion.prompt_tokens;
    let tokens_out = completion.completion_tokens;

    let pricing = state.pricing.read().await;
    let actual_amount = pricing
        .inference_breakdown(&req.model_id, tokens_in, tokens_out)
        .amount_mist;
    drop(pricing);

    {
        let store = state.store.lock().await;
        match validate_spend_policy(
            &state,
            &req.owner,
            Some(&req.balance_id),
            &req.agent_object_id,
            actual_amount,
            &store,
        )
        .await
        {
            Ok(_) => {}
            Err(SpendPolicyError::ApprovalRequired { .. }) => {
                tracing::warn!(
                    model = %req.model_id,
                    actual_amount,
                    tokens_in,
                    tokens_out,
                    "inference completed but billing rejected (approval required)"
                );
                return Err(StatusCode::PAYMENT_REQUIRED);
            }
            Err(err) => {
                tracing::warn!(
                    reason = %err.reason(),
                    actual_amount,
                    est_amount,
                    "inference completed but billing rejected"
                );
                return Err(StatusCode::PAYMENT_REQUIRED);
            }
        }
    }

    let mut extra_metadata = serde_json::Map::new();
    extra_metadata.insert(
        "content".to_string(),
        serde_json::Value::String(completion.content.clone()),
    );
    extra_metadata.insert(
        "prompt".to_string(),
        serde_json::Value::String(req.prompt.clone()),
    );
    extra_metadata.insert("inference".to_string(), serde_json::Value::Bool(true));

    let usage_req = UsageRequest {
        owner: req.owner,
        balance_id: req.balance_id,
        memory_account_id: req.memory_account_id,
        agent_object_id: req.agent_object_id,
        usage_kind: USAGE_INFERENCE,
        tokens_in: Some(tokens_in),
        tokens_out: Some(tokens_out),
        tool_id: None,
        model_id: Some(req.model_id.clone()),
        idempotency_key: req.idempotency_key,
    };

    let usage = record_usage_core(&state, usage_req, extra_metadata).await?;
    Ok(Json(inference_response_from_usage(
        usage,
        &req.model_id,
        &completion.content,
        tokens_in,
        tokens_out,
    )))
}

async fn record_usage_core(
    state: &AppState,
    req: UsageRequest,
    mut extra_metadata: serde_json::Map<String, serde_json::Value>,
) -> Result<UsageResponse, StatusCode> {
    {
        let store = state.store.lock().await;
        if let Some(existing) =
            store.find_by_idempotency(&req.balance_id, &req.agent_object_id, &req.idempotency_key)
        {
            return usage_response_from_line(existing);
        }
    }

    let pricing = state.pricing.read().await;
    if price_unavailable_for_usage(state, &pricing) {
        tracing::warn!("usage rejected: MYSO/USD price unavailable or stale");
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    let catalog = state.catalog.read().await;
    let breakdown = match compute_usage_breakdown(state, &pricing, &catalog, &req) {
        Ok(b) => b,
        Err(reason) => {
            tracing::warn!(reason = %reason, "usage rejected");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let amount_mist = breakdown.amount_mist;
    let catalog_version = pricing.catalog_version().to_string();
    let ecosystem_margin_pct = pricing.ecosystem_margin_pct();
    let myso_usd = pricing.myso_usd();
    drop(pricing);

    let balance_resp = state
        .ledger
        .fetch_balance(&req.owner)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    if req.balance_id != balance_resp.balance.balance_id {
        return Err(StatusCode::BAD_REQUEST);
    }

    let (usage_response, ingest) = {
        let mut store_guard = state.store.lock().await;
        let agent = match validate_spend_policy(
            state,
            &req.owner,
            Some(&req.balance_id),
            &req.agent_object_id,
            amount_mist,
            &store_guard,
        )
        .await
        {
            Ok((_effective, agent)) => agent,
            Err(SpendPolicyError::ApprovalRequired {
                balance_id,
                threshold_mist,
                organization_id,
            }) => {
                tracing::warn!(
                    balance_id = %balance_id,
                    agent_object_id = %req.agent_object_id,
                    amount_mist,
                    threshold_mist,
                    "usage rejected: unbilled_over_threshold (approval required)"
                );
                spawn_approval_request_side_effects(
                    state,
                    req.owner.clone(),
                    balance_id,
                    req.agent_object_id.clone(),
                    organization_id,
                    amount_mist,
                    threshold_mist,
                );
                return Err(StatusCode::PAYMENT_REQUIRED);
            }
            Err(err) => {
                tracing::warn!(reason = %err.reason(), "usage rejected");
                return Err(StatusCode::BAD_REQUEST);
            }
        };

        let indexed_nonce = balance_resp.balance.settlement_nonce.max(0) as u64;
        let on_chain_nonce = chain_balance::resolve_settlement_nonce(
            &state.oracle_args.myso_rpc,
            &req.balance_id,
            Some(indexed_nonce),
        )
        .await
        .map_err(|err| {
            tracing::warn!(
                balance_id = %req.balance_id,
                error = %err,
                "cannot resolve settlement_nonce; refusing new receipt"
            );
            StatusCode::SERVICE_UNAVAILABLE
        })?;

        let settlement_nonce =
            BalanceLedger::next_settlement_nonce(&balance_resp, &store_guard, Some(on_chain_nonce));

        let receipt_id =
            derive_receipt_id(&req.idempotency_key, &req.balance_id, &req.agent_object_id);
        let timestamp_ms = chrono::Utc::now().timestamp_millis() as u64;
        let receipt = UsageReceipt {
            balance_id: parse_object_id_hex(&req.balance_id)
                .map_err(|_| StatusCode::BAD_REQUEST)?,
            agent_object_id: parse_object_id_hex(&req.agent_object_id)
                .map_err(|_| StatusCode::BAD_REQUEST)?,
            receipt_id,
            amount_mist,
            usage_kind: req.usage_kind,
            timestamp_ms,
            settlement_nonce,
        };
        let signature = state
            .signer
            .sign_receipt(&receipt)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let mut metadata = serde_json::Map::new();
        metadata.insert(
            "catalog_version".to_string(),
            serde_json::Value::String(catalog_version),
        );
        metadata.insert(
            "tokens_in".to_string(),
            serde_json::json!(req.tokens_in.unwrap_or(0)),
        );
        metadata.insert(
            "tokens_out".to_string(),
            serde_json::json!(req.tokens_out.unwrap_or(0)),
        );
        metadata.insert(
            "base_mist".to_string(),
            serde_json::json!(breakdown.base_mist),
        );
        metadata.insert(
            "margin_mist".to_string(),
            serde_json::json!(breakdown.margin_mist),
        );
        metadata.insert(
            "ecosystem_margin_pct".to_string(),
            serde_json::json!(ecosystem_margin_pct),
        );
        metadata.insert("myso_usd".to_string(), serde_json::json!(myso_usd));
        metadata.append(&mut extra_metadata);
        let metadata = serde_json::Value::Object(metadata);

        let line = UsageLine {
            receipt_id,
            balance_id: req.balance_id.clone(),
            memory_account_id: req.memory_account_id.clone(),
            agent_object_id: req.agent_object_id.clone(),
            amount_mist,
            usage_kind: req.usage_kind,
            model_id: req.model_id.clone(),
            tool_id: req.tool_id.clone(),
            metadata: Some(metadata.clone()),
            signature_hex: hex::encode(&signature),
            settlement_nonce,
            timestamp_ms,
            settled: false,
            created_at_ms: timestamp_ms,
            void: false,
            organization_id: agent.organization_id.clone(),
            idempotency_key: Some(req.idempotency_key.clone()),
            ingest_synced: false,
        };

        if store_guard.insert_pending(line).is_err() {
            return Err(StatusCode::CONFLICT);
        }
        store_guard
            .save(&state.store_path)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let ingest = IngestUsageLineRequest {
            receipt_id: receipt_id.to_string(),
            balance_id: req.balance_id.clone(),
            agent_object_id: req.agent_object_id.clone(),
            usage_kind: req.usage_kind as i16,
            amount_mist: amount_mist as i64,
            model_id: req.model_id.clone(),
            tool_id: req.tool_id.clone(),
            metadata: Some(metadata),
            organization_id: agent.organization_id.clone(),
        };

        (
            UsageResponse {
                receipt_id,
                amount_mist,
                settlement_nonce,
                signature: hex::encode(signature),
                receipt,
            },
            ingest,
        )
    };

    if state
        .social
        .ingest_usage_line_with_retries(&ingest, 3)
        .await
        .is_ok()
    {
        let mut store = state.store.lock().await;
        if store.mark_ingest_synced(usage_response.receipt_id) {
            if let Err(err) = store.save(&state.store_path) {
                tracing::warn!(
                    error = %err,
                    receipt_id = usage_response.receipt_id,
                    "failed to persist ingest_synced flag"
                );
            }
        }
    } else {
        tracing::warn!(
            receipt_id = usage_response.receipt_id,
            "usage line ingest failed after retries; reconcile worker will retry"
        );
    }

    let balance_id = req.balance_id.clone();
    let coordinator = Arc::clone(&state.settlement_coordinator);
    tokio::spawn(async move {
        coordinator.request_settle_for_balance(balance_id).await;
    });

    Ok(usage_response)
}

#[derive(Debug, serde::Deserialize)]
pub struct UsageHistoryQuery {
    pub balance_id: Option<String>,
    pub limit: Option<usize>,
}

async fn usage_history(
    State(state): State<AppState>,
    headers: HeaderMap,
    Query(query): Query<UsageHistoryQuery>,
) -> Result<Json<Vec<UsageLine>>, StatusCode> {
    check_oracle_api_secret(&headers, &state.oracle_api_secret)?;
    let limit = query.limit.unwrap_or(50);
    let store = state.store.lock().await;
    let lines: Vec<UsageLine> = store
        .lines
        .iter()
        .filter(|l| {
            query
                .balance_id
                .as_ref()
                .map(|id| l.balance_id == *id)
                .unwrap_or(true)
        })
        .take(limit)
        .cloned()
        .collect();
    Ok(Json(lines))
}

async fn get_catalog(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<crate::catalog::CatalogResponse>, StatusCode> {
    check_oracle_api_secret(&headers, &state.oracle_api_secret)?;
    let pricing = state.pricing.read().await;
    let catalog = state.catalog.read().await;
    let fx = pricing_fx_from_engine(&state, &pricing);
    Ok(Json(catalog.to_response_with_fx(
        fx.catalog_usd_peg,
        fx.myso_usd,
        fx.price_oracle_url,
        fx.price_age_secs,
        fx.price_stale,
    )))
}

async fn trigger_settle(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let secret = state.settlement_secret.as_ref().ok_or_else(|| {
        tracing::warn!("settlement trigger rejected: AI_CREDIT_SETTLEMENT_SECRET unset");
        StatusCode::SERVICE_UNAVAILABLE
    })?;
    let provided = headers
        .get("x-ai-credit-settlement-secret")
        .and_then(|v| v.to_str().ok());
    if provided != Some(secret.as_str()) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let settled = state
        .settlement_coordinator
        .run_cycle(SettlementMode::DueBalances)
        .await;
    Ok(Json(serde_json::json!({ "settled": settled })))
}
