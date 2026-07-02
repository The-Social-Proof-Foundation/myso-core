// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use tokio::sync::{Mutex, RwLock};

use crate::approvals::{approval_covers, ApprovalsCache};
use crate::catalog::{CAP_AI_SPEND, PricingCatalog};
use crate::catalog_sync::{spawn_catalog_sync_worker, startup_catalog_sync};
use crate::chain_balance;
use crate::config::OracleArgs;
use crate::ledger::BalanceLedger;
use crate::myso_price_client::MysoPriceClient;
use crate::openrouter_client::OpenRouterClient;
use crate::price_refresh::{spawn_price_refresh_worker, startup_price_refresh};
use crate::pricing::{PriceBreakdown, PricingEngine, CATALOG_USD_PEG, USAGE_EMBED, USAGE_INFERENCE, USAGE_TOOL};
use crate::receipt::{ReceiptStore, UsageLine};
use crate::settlement_coordinator::{spawn_settlement_worker, SettlementCoordinator, SettlementMode};
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
    pub oracle_args: OracleArgs,
    pub settlement_coordinator: Arc<SettlementCoordinator>,
    pub catalog: Arc<RwLock<PricingCatalog>>,
    pub myso_price_oracle_url: String,
    pub approvals: ApprovalsCache,
    pub workflow: Option<WorkflowClient>,
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
}

#[derive(Debug, serde::Serialize)]
pub struct UsageResponse {
    pub receipt_id: u128,
    pub amount_mist: u64,
    pub settlement_nonce: u64,
    pub signature: String,
    pub receipt: UsageReceipt,
}

pub async fn serve(args: OracleArgs) -> anyhow::Result<()> {
    let signer = ReceiptSigner::from_hex(&args.private_key_hex)?;
    tracing::info!(public_key = %signer.public_key_hex(), "oracle signer ready");

    let catalog_path = if args.pricing_catalog_path.is_absolute() {
        args.pricing_catalog_path.clone()
    } else {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(&args.pricing_catalog_path)
    };
    let catalog = PricingCatalog::load(&catalog_path)?;
    let catalog = Arc::new(RwLock::new(catalog));
    let pricing = Arc::new(RwLock::new(PricingEngine::new(
        catalog.read().await.clone(),
        args.ecosystem_margin_pct,
    )));
    let myso_price_client = MysoPriceClient::new(args.myso_price_oracle_url.clone());
    startup_price_refresh(&args, &pricing, &myso_price_client).await;
    spawn_price_refresh_worker(
        Arc::new(args.clone()),
        pricing.clone(),
        myso_price_client,
    );

    let store = ReceiptStore::load(&args.receipt_store_path)?;
    let store_arc = Arc::new(Mutex::new(store));
    let args_arc = Arc::new(args.clone());

    if args.catalog_sync_active() {
        let openrouter = OpenRouterClient::new(
            args.openrouter_api_url.clone(),
            args.openrouter_api_key.clone().expect("catalog_sync_active implies key"),
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
        oracle_args: args.clone(),
        settlement_coordinator,
        catalog,
        myso_price_oracle_url: args.myso_price_oracle_url.clone(),
        approvals,
        workflow,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/v1/ai-credit/preflight", post(preflight))
        .route("/v1/ai-credit/estimate", post(estimate))
        .route("/v1/ai-credit/usage", post(record_usage))
        .route("/v1/ai-credit/usage-history", get(usage_history))
        .route("/v1/ai-credit/catalog", get(get_catalog))
        .route("/internal/ai-credit/settle", post(trigger_settle))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&args.listen_addr).await?;
    tracing::info!(addr = %args.listen_addr, "myso-ai-credit-oracle listening");
    axum::serve(listener, app).await?;
    Ok(())
}

async fn health() -> StatusCode {
    StatusCode::OK
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
                        .fetch_approved(
                            owner,
                            &balance_resp.balance.balance_id,
                            agent_object_id,
                        )
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
    Json(req): Json<PreflightRequest>,
) -> Json<PreflightResponse> {
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
        return Json(PreflightResponse {
            allowed: false,
            reason: Some("estimated_mist must be > 0".into()),
            estimated_mist: Some(0),
            effective_available_mist: None,
            base_mist: Some(breakdown.base_mist),
            margin_mist: Some(breakdown.margin_mist),
            approval_required: false,
            approval_threshold_mist: None,
        });
    }

    let store = state.store.lock().await;
    match validate_spend_policy(
        &state,
        &req.owner,
        None,
        &req.agent_object_id,
        estimated_mist,
        &store,
    )
    .await
    {
        Ok((effective, _agent)) => Json(PreflightResponse {
            allowed: true,
            reason: None,
            estimated_mist: Some(estimated_mist),
            effective_available_mist: Some(effective),
            base_mist: Some(breakdown.base_mist),
            margin_mist: Some(breakdown.margin_mist),
            approval_required: false,
            approval_threshold_mist: None,
        }),
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
            Json(PreflightResponse {
                allowed: false,
                reason: Some(APPROVAL_REQUIRED_REASON.to_string()),
                estimated_mist: Some(estimated_mist),
                effective_available_mist: None,
                base_mist: Some(breakdown.base_mist),
                margin_mist: Some(breakdown.margin_mist),
                approval_required: true,
                approval_threshold_mist: Some(threshold_mist),
            })
        }
        Err(err) => Json(PreflightResponse {
            allowed: false,
            reason: Some(err.reason()),
            estimated_mist: Some(estimated_mist),
            effective_available_mist: None,
            base_mist: Some(breakdown.base_mist),
            margin_mist: Some(breakdown.margin_mist),
            approval_required: false,
            approval_threshold_mist: None,
        }),
    }
}

async fn estimate(
    State(state): State<AppState>,
    Json(req): Json<EstimateRequest>,
) -> Json<EstimateResponse> {
    let pricing = state.pricing.read().await;
    let breakdown = estimate_breakdown(&pricing, &req);
    let fx = pricing_fx_from_engine(&state, &pricing);
    Json(EstimateResponse {
        estimated_mist: breakdown.amount_mist,
        estimated_credits: pricing.credits_from_mist(breakdown.amount_mist),
        base_mist: breakdown.base_mist,
        margin_mist: breakdown.margin_mist,
        ecosystem_margin_pct: pricing.ecosystem_margin_pct(),
        catalog_version: pricing.catalog_version().to_string(),
        catalog_usd_peg: fx.catalog_usd_peg,
        myso_usd: fx.myso_usd,
        price_oracle_url: fx.price_oracle_url,
        price_age_secs: fx.price_age_secs,
        price_stale: fx.price_stale,
    })
}

async fn record_usage(
    State(state): State<AppState>,
    Json(req): Json<UsageRequest>,
) -> Result<Json<UsageResponse>, StatusCode> {
    let pricing = state.pricing.read().await;
    if price_unavailable_for_usage(&state, &pricing) {
        tracing::warn!("usage rejected: MYSO/USD price unavailable or stale");
        return Err(StatusCode::SERVICE_UNAVAILABLE);
    }

    let catalog = state.catalog.read().await;
    let breakdown = match compute_usage_breakdown(&state, &pricing, &catalog, &req) {
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
            &state,
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
                // Post-hoc callers may have already burned compute; the spend stays
                // unbilled (never silently bypassed) and is audited for reconciliation.
                tracing::warn!(
                    balance_id = %balance_id,
                    agent_object_id = %req.agent_object_id,
                    amount_mist,
                    threshold_mist,
                    "usage rejected: unbilled_over_threshold (approval required)"
                );
                spawn_approval_request_side_effects(
                    &state,
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

        let on_chain_nonce = chain_balance::fetch_on_chain_settlement_nonce(
            &state.oracle_args.myso_rpc,
            &req.balance_id,
        )
        .await
        .ok();

        let settlement_nonce =
            BalanceLedger::next_settlement_nonce(&balance_resp, &store_guard, on_chain_nonce);

        let receipt_id = uuid::Uuid::new_v4().as_u128();
        let timestamp_ms = chrono::Utc::now().timestamp_millis() as u64;
        let receipt = UsageReceipt {
            balance_id: parse_object_id_hex(&req.balance_id).map_err(|_| StatusCode::BAD_REQUEST)?,
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

        let metadata = serde_json::json!({
            "catalog_version": catalog_version,
            "tokens_in": req.tokens_in,
            "tokens_out": req.tokens_out,
            "base_mist": breakdown.base_mist,
            "margin_mist": breakdown.margin_mist,
            "ecosystem_margin_pct": ecosystem_margin_pct,
            "myso_usd": myso_usd,
        });

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

    if let Err(err) = state.social.ingest_usage_line(&ingest).await {
        tracing::warn!(error = %err, "failed to ingest usage line to social-server");
    }

    let balance_id = req.balance_id.clone();
    let coordinator = Arc::clone(&state.settlement_coordinator);
    tokio::spawn(async move {
        coordinator.request_settle_for_balance(balance_id).await;
    });

    Ok(Json(usage_response))
}

#[derive(Debug, serde::Deserialize)]
pub struct UsageHistoryQuery {
    pub balance_id: Option<String>,
    pub limit: Option<usize>,
}

async fn usage_history(
    State(state): State<AppState>,
    Query(query): Query<UsageHistoryQuery>,
) -> Json<Vec<UsageLine>> {
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
    Json(lines)
}

async fn get_catalog(State(state): State<AppState>) -> Json<crate::catalog::CatalogResponse> {
    let pricing = state.pricing.read().await;
    let catalog = state.catalog.read().await;
    let fx = pricing_fx_from_engine(&state, &pricing);
    Json(catalog.to_response_with_fx(
        fx.catalog_usd_peg,
        fx.myso_usd,
        fx.price_oracle_url,
        fx.price_age_secs,
        fx.price_stale,
    ))
}

async fn trigger_settle(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if let Some(secret) = &state.settlement_secret {
        let provided = headers
            .get("x-ai-credit-settlement-secret")
            .and_then(|v| v.to_str().ok());
        if provided != Some(secret.as_str()) {
            return Err(StatusCode::UNAUTHORIZED);
        }
    }
    let settled = state
        .settlement_coordinator
        .run_cycle(SettlementMode::DueBalances)
        .await;
    Ok(Json(serde_json::json!({ "settled": settled })))
}
