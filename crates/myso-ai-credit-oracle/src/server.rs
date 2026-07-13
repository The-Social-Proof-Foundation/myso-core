// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use axum::extract::{Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::routing::{get, post};
use axum::{Json, Router};
use blake2::{Blake2b512, Digest};
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
use crate::reservation::{self, CancelSpendRequest, CaptureSpendRequest, ReserveSpendRequest};
use crate::reservation_ledger::{
    BeginReservation, ClaimedOutboxAction, ReservationLedger, ReservationRecord, ReservationStatus,
};
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
    pub reservation_ledger: ReservationLedger,
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
    #[serde(default)]
    pub system_prompt: Option<String>,
    pub prompt: String,
    #[serde(default)]
    pub max_tokens: Option<u32>,
    pub idempotency_key: String,
}

#[derive(Debug, serde::Serialize)]
pub struct InferenceResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_id: Option<u128>,
    pub amount_mist: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_nonce: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<UsageReceipt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reservation_nonce: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved_mist: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserve_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_digest: Option<String>,
    pub billing_state: String,
    pub tokens_in: u64,
    pub tokens_out: u64,
    pub model_id: String,
    pub content: String,
    pub provider_cost_usd_micros: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream_cost_usd_micros: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_generation_id: Option<String>,
}

#[derive(Debug, Clone)]
struct ProviderCostEvidence {
    provider_cost_usd_micros: u64,
    upstream_cost_usd_micros: Option<u64>,
    generation_id: Option<String>,
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
    let reservation_ledger = ReservationLedger::connect(
        &args.database_url,
        args.database_max_connections,
        args.outbox_lease_secs,
    )
    .await?;
    let incomplete_reservations = reservation_ledger.incomplete_count().await?;
    if incomplete_reservations > 0 {
        tracing::warn!(
            incomplete_reservations,
            "reservation recovery required; duplicate provider calls will fail closed"
        );
    }
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
    if args.legacy_usage_enabled {
        spawn_settlement_worker(settlement_coordinator.clone());
    }

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
        reservation_ledger,
    };

    if args.legacy_usage_enabled {
        spawn_ingest_reconcile_worker(state.clone());
    }
    spawn_reservation_reconcile_worker(state.clone());

    let mut app = Router::new()
        .route("/health", get(health))
        .route("/v1/ai-credit/preflight", post(preflight))
        .route("/v1/ai-credit/estimate", post(estimate))
        .route("/v1/ai-credit/inference", post(run_inference))
        .route("/v1/ai-credit/catalog", get(get_catalog));
    if args.legacy_usage_enabled {
        app = app
            .route("/v1/ai-credit/usage", post(record_usage))
            .route("/v1/ai-credit/usage-history", get(usage_history))
            .route("/internal/ai-credit/settle", post(trigger_settle));
    }
    let app = app.with_state(state);

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
        reservation_database_ready: state.reservation_ledger.probe().await,
    })
}

#[derive(Debug, serde::Serialize)]
struct HealthResponse {
    price_stale: bool,
    pending_receipts: u64,
    ingest_backlog: u64,
    settlement_enabled: bool,
    store_writable: bool,
    reservation_database_ready: bool,
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

fn hash32(parts: &[&[u8]]) -> Vec<u8> {
    let mut hasher = Blake2b512::new();
    for part in parts {
        hasher.update((part.len() as u64).to_le_bytes());
        hasher.update(part);
    }
    hasher.finalize()[..32].to_vec()
}

fn apply_reservation_buffer(amount: u64, buffer_bps: u64) -> Result<u64, StatusCode> {
    let numerator = (amount as u128)
        .checked_mul(10_000u128 + buffer_bps as u128)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let buffered = numerator.div_ceil(10_000);
    u64::try_from(buffered).map_err(|_| StatusCode::BAD_REQUEST)
}

fn inference_response_from_reservation(
    record: &ReservationRecord,
) -> Result<InferenceResponse, StatusCode> {
    if !matches!(
        record.status,
        ReservationStatus::Captured | ReservationStatus::Cancelled
    ) {
        return Err(StatusCode::CONFLICT);
    }
    Ok(InferenceResponse {
        receipt_id: Some(derive_receipt_id(
            &record.idempotency_key,
            &record.balance_id,
            &record.agent_object_id,
        )),
        amount_mist: record.amount_mist.unwrap_or(0),
        settlement_nonce: None,
        signature: None,
        receipt: None,
        reservation_nonce: Some(record.reservation_nonce),
        reserved_mist: Some(record.max_amount_mist),
        reserve_digest: record.reserve_digest.clone(),
        capture_digest: record.capture_digest.clone(),
        billing_state: match record.status {
            ReservationStatus::Captured => "captured".to_string(),
            ReservationStatus::Cancelled => "cancelled_zero_cost".to_string(),
            _ => unreachable!(),
        },
        tokens_in: record.tokens_in.unwrap_or(0),
        tokens_out: record.tokens_out.unwrap_or(0),
        model_id: record.model_id.clone(),
        content: record.content.clone().unwrap_or_default(),
        provider_cost_usd_micros: record.provider_cost_usd_micros.unwrap_or(0),
        upstream_cost_usd_micros: record.upstream_cost_usd_micros,
        provider_generation_id: record.provider_generation_id.clone(),
    })
}

async fn update_reservation_record(
    state: &AppState,
    balance_id: &str,
    agent_object_id: &str,
    idempotency_key: &str,
    update: impl FnOnce(&mut ReservationRecord),
) -> Result<ReservationRecord, StatusCode> {
    state
        .reservation_ledger
        .update(balance_id, agent_object_id, idempotency_key, update)
        .await
        .map_err(|error| {
            tracing::error!(%error, "failed to persist reservation ledger transition");
            StatusCode::INTERNAL_SERVER_ERROR
        })
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

fn spawn_reservation_reconcile_worker(state: AppState) {
    tokio::spawn(async move {
        // Do not race normal request finalization immediately after process startup.
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(30)).await;
            run_reservation_reconcile_cycle(&state).await;
        }
    });
}

async fn run_reservation_reconcile_cycle(state: &AppState) {
    let recoverable = match state.reservation_ledger.claim_pending(32).await {
        Ok(records) => records,
        Err(error) => {
            tracing::error!(%error, "failed to claim inference outbox work");
            return;
        }
    };

    for work in recoverable {
        let ClaimedOutboxAction { action, record } = work;
        let now_ms = chrono::Utc::now().timestamp_millis() as u64;
        if now_ms >= record.hard_expiry_ms {
            tracing::error!(
                balance_id = %record.balance_id,
                reservation_nonce = record.reservation_nonce,
                "provider charge missed the capture window; manual reconciliation required"
            );
            let _ = state
                .reservation_ledger
                .retry(&record, &action, "reservation hard expiry passed")
                .await;
            continue;
        }
        let result = process_outbox_action(state, &record, &action, now_ms).await;

        match result {
            Ok((status, digest)) => {
                let update = state
                    .reservation_ledger
                    .complete_action(
                        &record.balance_id,
                        &record.agent_object_id,
                        &record.idempotency_key,
                        &action,
                        &digest,
                        |current| {
                            current.status = status;
                            match action.as_str() {
                                "reserve" => current.reserve_digest = Some(digest.clone()),
                                "capture" => current.capture_digest = Some(digest.clone()),
                                "cancel" => {
                                    current.cancel_digest = Some(digest.clone());
                                    current.amount_mist = Some(0);
                                }
                                _ => {}
                            }
                            current.last_error = None;
                        },
                    )
                    .await;
                if update.is_ok() {
                    tracing::info!(
                        reservation_nonce = record.reservation_nonce,
                        %action,
                        ?status,
                        "recovered unfinished inference billing action"
                    );
                }
            }
            Err(error) => {
                let error_text = error.to_string();
                let _ = update_reservation_record(
                    state,
                    &record.balance_id,
                    &record.agent_object_id,
                    &record.idempotency_key,
                    |current| current.last_error = Some(error_text.clone()),
                )
                .await;
                let _ = state
                    .reservation_ledger
                    .retry(&record, &action, &error_text)
                    .await;
                tracing::warn!(
                    %error,
                    %action,
                    reservation_nonce = record.reservation_nonce,
                    "reservation outbox retry failed"
                );
            }
        }
    }
}

async fn process_outbox_action(
    state: &AppState,
    record: &ReservationRecord,
    action: &str,
    now_ms: u64,
) -> anyhow::Result<(ReservationStatus, String)> {
    match action {
        "reserve" => reservation::reserve_spend(
            &state.oracle_args,
            &state.signer,
            &ReserveSpendRequest {
                balance_id: record.balance_id.clone(),
                memory_account_id: record.memory_account_id.clone(),
                agent_object_id: record.agent_object_id.clone(),
                reservation_nonce: record.reservation_nonce,
                max_amount_mist: record.max_amount_mist,
                provider_envelope_hash: hex::decode(&record.provider_envelope_hash_hex)?,
                request_hash: hex::decode(&record.request_hash_hex)?,
                fx_quote_id: hex::decode(&record.fx_quote_id_hex)?,
                myso_usd_e8: record.myso_usd_e8,
                markup_bps: record.markup_bps,
                timestamp_ms: record.created_at_ms,
                capture_deadline_ms: record.capture_deadline_ms,
                hard_expiry_ms: record.hard_expiry_ms,
            },
        )
        .await
        .map(|digest| (ReservationStatus::Reserved, digest)),
        "cancel" => reservation::cancel_spend(
            &state.oracle_args,
            &state.signer,
            &CancelSpendRequest {
                balance_id: record.balance_id.clone(),
                agent_object_id: record.agent_object_id.clone(),
                reservation_nonce: record.reservation_nonce,
                timestamp_ms: now_ms,
            },
        )
        .await
        .map(|digest| (ReservationStatus::Cancelled, digest)),
        "capture" => {
            let amount = record.amount_mist.unwrap_or(0);
            let provider_cost = record.provider_cost_usd_micros.unwrap_or(0);
            let generation_hash = hash32(&[
                record
                    .provider_generation_id
                    .as_deref()
                    .unwrap_or("")
                    .as_bytes(),
                record.content.as_deref().unwrap_or("").as_bytes(),
                &record.tokens_in.unwrap_or(0).to_le_bytes(),
                &record.tokens_out.unwrap_or(0).to_le_bytes(),
                &provider_cost.to_le_bytes(),
            ]);
            reservation::capture_spend(
                &state.oracle_args,
                &state.signer,
                &CaptureSpendRequest {
                    balance_id: record.balance_id.clone(),
                    agent_object_id: record.agent_object_id.clone(),
                    reservation_nonce: record.reservation_nonce,
                    amount_mist: amount,
                    provider_cost_usd_micros: provider_cost,
                    provider_generation_hash: generation_hash,
                    timestamp_ms: now_ms,
                },
            )
            .await
            .map(|digest| (ReservationStatus::Captured, digest))
        }
        _ => anyhow::bail!("unsupported inference outbox action {action}"),
    }
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
                settled: false,
                settlement_tx: None,
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
        return Err(denied("insufficient_ai_balance".into()));
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

    let usage = record_usage_core(&state, req, serde_json::Map::new(), None).await?;
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

    const MAX_INFERENCE_OUTPUT_TOKENS: u32 = 32_768;
    const MAX_PROMPT_BYTES: usize = 1_048_576;

    let system_prompt = req.system_prompt.as_deref().unwrap_or("");
    let combined_prompt_bytes = req.prompt.len().saturating_add(system_prompt.len());
    if combined_prompt_bytes > MAX_PROMPT_BYTES {
        return Err(StatusCode::PAYLOAD_TOO_LARGE);
    }
    let max_tokens = req.max_tokens.unwrap_or(64);
    if max_tokens == 0 || max_tokens > MAX_INFERENCE_OUTPUT_TOKENS {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Completed retries return the original result. Every nonterminal state fails closed:
    // in particular, no retry can duplicate provider spend after a crash or timeout.
    if let Some(existing) = state
        .reservation_ledger
        .find(&req.balance_id, &req.agent_object_id, &req.idempotency_key)
        .await
        .map_err(|error| {
            tracing::error!(%error, "failed to read inference idempotency ledger");
            StatusCode::SERVICE_UNAVAILABLE
        })?
    {
        return Ok(Json(inference_response_from_reservation(&existing)?));
    }

    let now_ms = chrono::Utc::now().timestamp_millis() as u64;
    let (max_amount_mist, myso_usd_e8, markup_bps, catalog_version) = {
        let pricing = state.pricing.read().await;
        if price_unavailable_for_usage(&state, &pricing) {
            tracing::warn!("inference rejected: MYSO/USD price unavailable or stale");
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
        let envelope = pricing.inference_breakdown(
            &req.model_id,
            combined_prompt_bytes.max(1) as u64,
            max_tokens as u64,
        );
        (
            apply_reservation_buffer(
                envelope.amount_mist,
                state.oracle_args.reservation_price_buffer_bps,
            )?,
            pricing.myso_usd_e8(),
            pricing.oracle_markup_bps(),
            pricing.catalog_version().to_string(),
        )
    };

    // This indexed preflight produces the UX-friendly reason/approval workflow. The Move
    // reserve call repeats capability, revocation, allowance, balance, and cap checks against
    // finalized chain state and remains authoritative.
    let authorized_agent = {
        let store = state.store.lock().await;
        match validate_spend_policy(
            &state,
            &req.owner,
            Some(&req.balance_id),
            &req.agent_object_id,
            max_amount_mist,
            &store,
        )
        .await
        {
            Ok((_available, agent)) => agent,
            Err(SpendPolicyError::ApprovalRequired { .. }) => {
                return Err(StatusCode::PAYMENT_REQUIRED)
            }
            Err(error) => {
                tracing::warn!(reason = %error.reason(), "inference reservation rejected");
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    };

    let provider_envelope_hash = hash32(&[
        b"openrouter-chat-v1",
        req.model_id.as_bytes(),
        &(combined_prompt_bytes.max(1) as u64).to_le_bytes(),
        &(max_tokens as u64).to_le_bytes(),
        catalog_version.as_bytes(),
        &state.oracle_args.reservation_price_buffer_bps.to_le_bytes(),
    ]);
    let request_hash = hash32(&[
        req.owner.as_bytes(),
        req.balance_id.as_bytes(),
        req.memory_account_id.as_bytes(),
        req.agent_object_id.as_bytes(),
        req.model_id.as_bytes(),
        system_prompt.as_bytes(),
        req.prompt.as_bytes(),
        &(max_tokens as u64).to_le_bytes(),
        req.idempotency_key.as_bytes(),
    ]);
    let fx_quote_id = hash32(&[
        state.myso_price_oracle_url.as_bytes(),
        &myso_usd_e8.to_le_bytes(),
        &markup_bps.to_le_bytes(),
        catalog_version.as_bytes(),
        &now_ms.to_le_bytes(),
    ]);
    let capture_deadline_ms = now_ms
        .checked_add(state.oracle_args.reservation_capture_window_secs * 1000)
        .ok_or(StatusCode::BAD_REQUEST)?;
    let hard_expiry_ms = now_ms
        .checked_add(state.oracle_args.reservation_hard_expiry_secs * 1000)
        .ok_or(StatusCode::BAD_REQUEST)?;

    let (latest_nonce, _) = chain_balance::fetch_on_chain_reservation_state(
        &state.oracle_args.myso_rpc,
        &req.balance_id,
    )
    .await
    .map_err(|error| {
        tracing::warn!(%error, "cannot read canonical reservation nonce");
        StatusCode::SERVICE_UNAVAILABLE
    })?;
    let record = ReservationRecord {
        idempotency_key: req.idempotency_key.clone(),
        owner: req.owner.clone(),
        balance_id: req.balance_id.clone(),
        memory_account_id: req.memory_account_id.clone(),
        agent_object_id: req.agent_object_id.clone(),
        model_id: req.model_id.clone(),
        reservation_nonce: 0,
        max_amount_mist,
        provider_envelope_hash_hex: hex::encode(&provider_envelope_hash),
        request_hash_hex: hex::encode(&request_hash),
        fx_quote_id_hex: hex::encode(&fx_quote_id),
        myso_usd_e8,
        markup_bps,
        capture_deadline_ms,
        hard_expiry_ms,
        status: ReservationStatus::Preparing,
        reserve_digest: None,
        capture_digest: None,
        cancel_digest: None,
        amount_mist: None,
        provider_cost_usd_micros: None,
        upstream_cost_usd_micros: None,
        provider_generation_id: None,
        content: None,
        tokens_in: None,
        tokens_out: None,
        created_at_ms: now_ms,
        updated_at_ms: now_ms,
        last_error: None,
    };
    let record = match state.reservation_ledger.begin(record, latest_nonce).await {
        Ok(BeginReservation::Created(record)) => record,
        Ok(BeginReservation::Existing(existing)) => {
            if existing.request_hash_hex != hex::encode(&request_hash) {
                return Err(StatusCode::CONFLICT);
            }
            return Ok(Json(inference_response_from_reservation(&existing)?));
        }
        Err(error) => {
            tracing::error!(%error, "failed to create inference reservation ledger entry");
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
    };
    let reservation_nonce = record.reservation_nonce;
    if !state
        .reservation_ledger
        .claim_action(&record, "reserve")
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
    {
        return Err(StatusCode::CONFLICT);
    }
    let reserve_result = process_outbox_action(&state, &record, "reserve", now_ms)
        .await
        .map(|(_, digest)| digest);
    let reserve_digest = match reserve_result {
        Ok(digest) => digest,
        Err(error) => {
            let error_text = error.to_string();
            let _ = update_reservation_record(
                &state,
                &req.balance_id,
                &req.agent_object_id,
                &req.idempotency_key,
                |record| record.last_error = Some(error_text.clone()),
            )
            .await;
            let _ = state
                .reservation_ledger
                .retry(&record, "reserve", &error_text)
                .await;
            tracing::warn!(%error, reservation_nonce, "on-chain inference reservation failed");
            return Err(StatusCode::PAYMENT_REQUIRED);
        }
    };

    state
        .reservation_ledger
        .complete_action(
            &req.balance_id,
            &req.agent_object_id,
            &req.idempotency_key,
            "reserve",
            &reserve_digest,
            |record| {
                record.status = ReservationStatus::Reserved;
                record.reserve_digest = Some(reserve_digest.clone());
            },
        )
        .await
        .map_err(|error| {
            tracing::error!(%error, "failed to commit reserve outbox delivery");
            StatusCode::SERVICE_UNAVAILABLE
        })?;

    let mut messages = Vec::with_capacity(if system_prompt.is_empty() { 1 } else { 2 });
    if !system_prompt.is_empty() {
        messages.push(crate::openrouter_client::ChatMessage {
            role: "system",
            content: system_prompt,
        });
    }
    messages.push(crate::openrouter_client::ChatMessage {
        role: "user",
        content: &req.prompt,
    });
    let completion = match openrouter
        .chat_completions(&req.model_id, &messages, max_tokens)
        .await
    {
        Ok(completion) => completion,
        Err(error) => {
            // A transport/parse failure is ambiguous: OpenRouter may have generated and
            // charged. Never issue an unsafe cancellation. Hard expiry releases funds while
            // the durable record prevents duplicate inference for this key.
            let error_text = error.to_string();
            let _ = update_reservation_record(
                &state,
                &req.balance_id,
                &req.agent_object_id,
                &req.idempotency_key,
                |record| {
                    record.status = ReservationStatus::AmbiguousProviderFailure;
                    record.last_error = Some(error_text.clone());
                },
            )
            .await;
            tracing::warn!(%error, reservation_nonce, "OpenRouter failed after finalized reservation");
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

    let provider = ProviderCostEvidence {
        provider_cost_usd_micros: completion.provider_cost_usd_micros,
        upstream_cost_usd_micros: completion.upstream_cost_usd_micros,
        generation_id: completion.generation_id.clone(),
    };
    let actual_amount = PricingEngine::provider_cost_breakdown_at_quote(
        provider.provider_cost_usd_micros,
        myso_usd_e8,
        markup_bps,
    )
    .amount_mist;
    if actual_amount > max_amount_mist {
        update_reservation_record(
            &state,
            &req.balance_id,
            &req.agent_object_id,
            &req.idempotency_key,
            |record| {
                record.status = ReservationStatus::AmbiguousProviderFailure;
                record.amount_mist = Some(actual_amount);
                record.provider_cost_usd_micros = Some(provider.provider_cost_usd_micros);
                record.upstream_cost_usd_micros = provider.upstream_cost_usd_micros;
                record.provider_generation_id = provider.generation_id.clone();
                record.tokens_in = Some(completion.prompt_tokens);
                record.tokens_out = Some(completion.completion_tokens);
                record.last_error = Some(format!(
                    "actual charge {actual_amount} exceeded reserved maximum {max_amount_mist}"
                ));
            },
        )
        .await?;
        tracing::error!(
            actual_amount,
            max_amount_mist,
            reservation_nonce,
            "provider price exceeded locked envelope"
        );
        return Err(StatusCode::BAD_GATEWAY);
    }

    let provider_record = update_reservation_record(
        &state,
        &req.balance_id,
        &req.agent_object_id,
        &req.idempotency_key,
        |record| {
            record.status = ReservationStatus::ProviderSucceeded;
            record.amount_mist = Some(actual_amount);
            record.provider_cost_usd_micros = Some(provider.provider_cost_usd_micros);
            record.upstream_cost_usd_micros = provider.upstream_cost_usd_micros;
            record.provider_generation_id = provider.generation_id.clone();
            record.content = Some(completion.content.clone());
            record.tokens_in = Some(completion.prompt_tokens);
            record.tokens_out = Some(completion.completion_tokens);
        },
    )
    .await?;

    let finalization_action = if provider.provider_cost_usd_micros == 0 || actual_amount == 0 {
        "cancel"
    } else {
        "capture"
    };
    if !state
        .reservation_ledger
        .claim_action(&provider_record, finalization_action)
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
    {
        return Err(StatusCode::CONFLICT);
    }
    let (terminal_status, finalization_digest) = match process_outbox_action(
        &state,
        &provider_record,
        finalization_action,
        chrono::Utc::now().timestamp_millis() as u64,
    )
    .await
    {
        Ok(result) => result,
        Err(error) => {
            let error_text = error.to_string();
            let _ = update_reservation_record(
                &state,
                &req.balance_id,
                &req.agent_object_id,
                &req.idempotency_key,
                |record| record.last_error = Some(error_text.clone()),
            )
            .await;
            let _ = state
                .reservation_ledger
                .retry(&provider_record, finalization_action, &error_text)
                .await;
            tracing::error!(%error, reservation_nonce, %finalization_action, "provider succeeded but finalization failed; outbox reconciliation required");
            return Err(StatusCode::SERVICE_UNAVAILABLE);
        }
    };
    let finalized = state
        .reservation_ledger
        .complete_action(
            &req.balance_id,
            &req.agent_object_id,
            &req.idempotency_key,
            finalization_action,
            &finalization_digest,
            |record| {
                record.status = terminal_status;
                if terminal_status == ReservationStatus::Captured {
                    record.capture_digest = Some(finalization_digest.clone());
                } else {
                    record.cancel_digest = Some(finalization_digest.clone());
                    record.amount_mist = Some(0);
                }
                record.last_error = None;
            },
        )
        .await
        .map_err(|error| {
            tracing::error!(%error, "failed to commit finalization outbox delivery");
            StatusCode::SERVICE_UNAVAILABLE
        })?;

    // Usage analytics are downstream of finalized billing. Failure here never reopens or
    // repeats the provider call; chain reservation events remain canonical.
    let receipt_id = derive_receipt_id(&req.idempotency_key, &req.balance_id, &req.agent_object_id);
    let metadata = serde_json::json!({
        "billing_authority": "openrouter_usage_cost",
        "billing_state": if finalized.status == ReservationStatus::Captured { "captured" } else { "cancelled_zero_cost" },
        "reservation_nonce": reservation_nonce,
        "reserved_mist": max_amount_mist,
        "reserve_digest": reserve_digest,
        "capture_digest": finalized.capture_digest,
        "provider_cost_usd_micros": provider.provider_cost_usd_micros,
        "upstream_cost_usd_micros": provider.upstream_cost_usd_micros,
        "provider_generation_id": provider.generation_id,
        "myso_usd_e8": myso_usd_e8,
        "markup_bps": markup_bps,
        "tokens_in": completion.prompt_tokens,
        "tokens_out": completion.completion_tokens,
    });
    if let Err(error) = state
        .social
        .ingest_usage_line_with_retries(
            &IngestUsageLineRequest {
                receipt_id: receipt_id.to_string(),
                balance_id: req.balance_id.clone(),
                agent_object_id: req.agent_object_id.clone(),
                usage_kind: USAGE_INFERENCE as i16,
                amount_mist: actual_amount as i64,
                model_id: Some(req.model_id.clone()),
                tool_id: None,
                metadata: Some(metadata),
                organization_id: authorized_agent.organization_id,
                settled: true,
                settlement_tx: finalized
                    .capture_digest
                    .clone()
                    .or_else(|| finalized.cancel_digest.clone()),
            },
            3,
        )
        .await
    {
        tracing::warn!(%error, receipt_id, "captured inference analytics ingest failed");
    }

    Ok(Json(inference_response_from_reservation(&finalized)?))
}

async fn record_usage_core(
    state: &AppState,
    req: UsageRequest,
    mut extra_metadata: serde_json::Map<String, serde_json::Value>,
    provider_cost: Option<ProviderCostEvidence>,
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
    let breakdown = match provider_cost.as_ref() {
        Some(evidence) => pricing.provider_cost_breakdown(evidence.provider_cost_usd_micros),
        None => match compute_usage_breakdown(state, &pricing, &catalog, &req) {
            Ok(b) => b,
            Err(reason) => {
                tracing::warn!(reason = %reason, "usage rejected");
                return Err(StatusCode::BAD_REQUEST);
            }
        },
    };
    let amount_mist = breakdown.amount_mist;
    let catalog_version = pricing.catalog_version().to_string();
    let ecosystem_margin_pct = pricing.ecosystem_margin_pct();
    let myso_usd = pricing.myso_usd();
    let myso_usd_e8 = pricing.myso_usd_e8();
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
        metadata.insert("myso_usd_e8".to_string(), serde_json::json!(myso_usd_e8));
        metadata.insert(
            "billing_authority".to_string(),
            serde_json::Value::String(if provider_cost.is_some() {
                "openrouter_usage_cost".to_string()
            } else {
                "reported_usage_catalog".to_string()
            }),
        );
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
            settled: false,
            settlement_tx: None,
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
