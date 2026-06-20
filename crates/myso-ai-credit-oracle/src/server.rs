// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use tokio::sync::Mutex;

use crate::catalog::{CAP_AI_SPEND, PricingCatalog};
use crate::config::OracleArgs;
use crate::ledger::BalanceLedger;
use crate::pricing::{PriceBreakdown, PricingEngine, USAGE_EMBED, USAGE_INFERENCE, USAGE_TOOL};
use crate::receipt::{ReceiptStore, UsageLine};
use crate::settlement_coordinator::{spawn_settlement_worker, SettlementCoordinator, SettlementMode};
use crate::signing::{parse_object_id_hex, ReceiptSigner, UsageReceipt};
use crate::social_client::{IngestUsageLineRequest, SocialClient};

#[derive(Clone)]
pub struct AppState {
    pub signer: ReceiptSigner,
    pub pricing: PricingEngine,
    pub ledger: BalanceLedger,
    pub social: SocialClient,
    pub store: Arc<Mutex<ReceiptStore>>,
    pub store_path: std::path::PathBuf,
    pub settlement_secret: Option<String>,
    pub oracle_args: OracleArgs,
    pub settlement_coordinator: Arc<SettlementCoordinator>,
    pub pricing_catalog: PricingCatalog,
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
    let pricing = PricingEngine::new(catalog.clone(), args.ecosystem_margin_pct);
    let social = SocialClient::new(
        args.social_server_url.clone(),
        args.usage_sync_secret.clone(),
    );
    let ledger = BalanceLedger::new(social.clone());

    let store = ReceiptStore::load(&args.receipt_store_path)?;
    let store_arc = Arc::new(Mutex::new(store));
    let args_arc = Arc::new(args.clone());
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
        pricing_catalog: catalog,
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

fn estimate_breakdown(state: &AppState, req: &EstimateRequest) -> PriceBreakdown {
    let usage_kind = req.usage_kind.unwrap_or(USAGE_INFERENCE);
    if usage_kind == USAGE_TOOL {
        return state
            .pricing
            .tool_breakdown(req.tool_id.as_deref().unwrap_or("default"));
    }
    if usage_kind == USAGE_EMBED {
        return state
            .pricing
            .embedding_breakdown(
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
        let infer = state.pricing.inference_breakdown(model, tokens_in, tokens_out);
        let embed = state
            .pricing
            .embedding_breakdown("text-embedding-3-small", facts * 200);
        return combine_breakdowns(infer, embed);
    }
    if operation == "remember" || operation == "embed" {
        return state.pricing.embedding_breakdown(model, tokens_in.max(1));
    }
    if operation == "ask" {
        let embed = state.pricing.embedding_breakdown(model, tokens_in.max(1));
        let infer = state.pricing.inference_breakdown(model, tokens_in, tokens_out);
        return combine_breakdowns(embed, infer);
    }
    state.pricing.inference_breakdown(model, tokens_in, tokens_out)
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

fn compute_usage_breakdown(state: &AppState, req: &UsageRequest) -> Result<PriceBreakdown, String> {
    if state.oracle_args.strict_catalog {
        match req.usage_kind {
            USAGE_TOOL => {
                let tool_id = req.tool_id.as_deref().unwrap_or("default");
                if !state.pricing_catalog.is_known_tool(tool_id) {
                    return Err("unknown_tool_id".into());
                }
            }
            USAGE_EMBED => {
                let model = req.model_id.as_deref().unwrap_or("text-embedding-3-small");
                if !state.pricing_catalog.is_known_embedding_model(model) {
                    return Err("unknown_embedding_model_id".into());
                }
            }
            _ => {
                let model = req.model_id.as_deref().unwrap_or("openai/gpt-4o-mini");
                if !state.pricing_catalog.is_known_inference_model(model) {
                    return Err("unknown_model_id".into());
                }
            }
        }
    }

    Ok(match req.usage_kind {
        USAGE_TOOL => state
            .pricing
            .tool_breakdown(req.tool_id.as_deref().unwrap_or("default")),
        USAGE_EMBED => state
            .pricing
            .embedding_breakdown(
                req.model_id.as_deref().unwrap_or("text-embedding-3-small"),
                req.tokens_in.unwrap_or(0),
            ),
        _ => state.pricing.inference_breakdown(
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
) -> Result<u64, String> {
    let balance_resp = state
        .ledger
        .fetch_balance(owner)
        .await
        .map_err(|e| format!("balance_fetch_failed: {e}"))?
        .ok_or_else(|| "no_ai_credit_balance".to_string())?;

    if !balance_resp.balance.active {
        return Err("balance_inactive".into());
    }

    if let Some(expected) = balance_id {
        if balance_resp.balance.balance_id != expected {
            return Err("balance_id_mismatch".into());
        }
    }

    let effective = BalanceLedger::effective_available_mist(&balance_resp, store);
    if amount_mist > effective {
        return Err("insufficient_ai_credits".into());
    }

    let agent = state
        .social
        .get_sub_agent_by_object_id(agent_object_id)
        .await
        .map_err(|e| format!("agent_fetch_failed: {e}"))?;

    if !agent.active || agent.revoked_at_ms.is_some() {
        return Err("sub_agent_not_active".into());
    }
    if agent.capabilities & CAP_AI_SPEND as i64 != CAP_AI_SPEND as i64 {
        return Err("missing_cap_ai_spend".into());
    }
    if let Some(exp) = agent.expires_at_ms {
        if chrono::Utc::now().timestamp_millis() > exp {
            return Err("sub_agent_expired".into());
        }
    }

    if let Some(daily) = balance_resp.balance.daily_cap_mist {
        if balance_resp.balance.spent_day_mist + amount_mist as i64 > daily {
            return Err("daily_cap_exceeded".into());
        }
    }
    if let Some(monthly) = balance_resp.balance.monthly_cap_mist {
        if balance_resp.balance.spent_month_mist + amount_mist as i64 > monthly {
            return Err("monthly_cap_exceeded".into());
        }
    }

    if let Some(budget) = balance_resp
        .agent_budgets
        .iter()
        .find(|b| b.agent_object_id == agent_object_id)
    {
        if !budget.enabled {
            return Err("agent_budget_disabled".into());
        }
        if let Some(max) = budget.budget_mist {
            if budget.spent_mist + amount_mist as i64 > max {
                return Err("agent_budget_exceeded".into());
            }
        }
    }

    Ok(effective)
}

async fn preflight(
    State(state): State<AppState>,
    Json(req): Json<PreflightRequest>,
) -> Json<PreflightResponse> {
    let breakdown = estimate_breakdown(
        &state,
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
        Ok(effective) => Json(PreflightResponse {
            allowed: true,
            reason: None,
            estimated_mist: Some(estimated_mist),
            effective_available_mist: Some(effective),
            base_mist: Some(breakdown.base_mist),
            margin_mist: Some(breakdown.margin_mist),
        }),
        Err(reason) => Json(PreflightResponse {
            allowed: false,
            reason: Some(reason),
            estimated_mist: Some(estimated_mist),
            effective_available_mist: None,
            base_mist: Some(breakdown.base_mist),
            margin_mist: Some(breakdown.margin_mist),
        }),
    }
}

async fn estimate(
    State(state): State<AppState>,
    Json(req): Json<EstimateRequest>,
) -> Json<EstimateResponse> {
    let breakdown = estimate_breakdown(&state, &req);
    Json(EstimateResponse {
        estimated_mist: breakdown.amount_mist,
        estimated_credits: state.pricing.credits_from_mist(breakdown.amount_mist),
        base_mist: breakdown.base_mist,
        margin_mist: breakdown.margin_mist,
        ecosystem_margin_pct: state.pricing.ecosystem_margin_pct(),
        catalog_version: state.pricing.catalog_version().to_string(),
    })
}

async fn record_usage(
    State(state): State<AppState>,
    Json(req): Json<UsageRequest>,
) -> Result<Json<UsageResponse>, StatusCode> {
    let breakdown = match compute_usage_breakdown(&state, &req) {
        Ok(b) => b,
        Err(reason) => {
            tracing::warn!(reason = %reason, "usage rejected");
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let amount_mist = breakdown.amount_mist;

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
        if let Err(reason) = validate_spend_policy(
            &state,
            &req.owner,
            Some(&req.balance_id),
            &req.agent_object_id,
            amount_mist,
            &store_guard,
        )
        .await
        {
            tracing::warn!(reason = %reason, "usage rejected");
            return Err(StatusCode::BAD_REQUEST);
        }

        let settlement_nonce =
            BalanceLedger::next_settlement_nonce(&balance_resp, &store_guard);

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
            "catalog_version": state.pricing.catalog_version(),
            "tokens_in": req.tokens_in,
            "tokens_out": req.tokens_out,
            "base_mist": breakdown.base_mist,
            "margin_mist": breakdown.margin_mist,
            "ecosystem_margin_pct": state.pricing.ecosystem_margin_pct(),
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
    Json(state.pricing_catalog.to_response())
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
