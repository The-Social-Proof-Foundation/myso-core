// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    ai_credit_agent_budgets, ai_credit_balances, ai_credit_config, ai_credit_events,
    ai_credit_spend_approvals, ai_credit_usage_lines, ai_spend_reservations,
};

/// Spend-approval lifecycle states (see `ai_credit_spend_approvals.status`).
pub const APPROVAL_STATUS_REQUESTED: &str = "requested";
pub const APPROVAL_STATUS_APPROVED: &str = "approved";
pub const APPROVAL_STATUS_CONSUMED: &str = "consumed";
pub const APPROVAL_STATUS_REVOKED: &str = "revoked";
pub const APPROVAL_STATUS_EXPIRED: &str = "expired";

pub const RESERVATION_STATUS_RESERVED: &str = "reserved";
pub const RESERVATION_STATUS_CAPTURED: &str = "captured";
pub const RESERVATION_STATUS_CANCELLED: &str = "cancelled";
pub const RESERVATION_STATUS_EXPIRED: &str = "expired";

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_balances)]
pub struct NewAiCreditBalance {
    pub balance_id: String,
    pub memory_account_id: String,
    pub principal_owner: String,
    pub profile_id: String,
    pub balance_mist: i64,
    pub spent_total_mist: i64,
    pub reserved_mist: i64,
    pub daily_cap_mist: Option<i64>,
    pub monthly_cap_mist: Option<i64>,
    pub spent_day_mist: i64,
    pub spent_month_mist: i64,
    pub settlement_nonce: i64,
    pub reservation_nonce: i64,
    pub active: bool,
    pub updated_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_balances)]
pub struct AiCreditBalanceRow {
    pub balance_id: String,
    pub memory_account_id: String,
    pub principal_owner: String,
    pub profile_id: String,
    pub balance_mist: i64,
    pub spent_total_mist: i64,
    pub reserved_mist: i64,
    pub daily_cap_mist: Option<i64>,
    pub monthly_cap_mist: Option<i64>,
    pub spent_day_mist: i64,
    pub spent_month_mist: i64,
    pub settlement_nonce: i64,
    pub reservation_nonce: i64,
    pub active: bool,
    pub updated_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_agent_budgets)]
pub struct NewAiCreditAgentBudget {
    pub balance_id: String,
    pub agent_object_id: String,
    pub budget_mist: Option<i64>,
    pub spent_mist: i64,
    pub reserved_mist: i64,
    pub daily_cap_mist: Option<i64>,
    pub monthly_cap_mist: Option<i64>,
    pub require_approval_above_mist: Option<i64>,
    pub enabled: bool,
    pub updated_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_agent_budgets)]
pub struct AiCreditAgentBudgetRow {
    pub balance_id: String,
    pub agent_object_id: String,
    pub budget_mist: Option<i64>,
    pub spent_mist: i64,
    pub reserved_mist: i64,
    pub daily_cap_mist: Option<i64>,
    pub monthly_cap_mist: Option<i64>,
    pub require_approval_above_mist: Option<i64>,
    pub enabled: bool,
    pub updated_at_ms: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_events)]
pub struct NewAiCreditEvent {
    pub event_type: String,
    pub balance_id: Option<String>,
    pub memory_account_id: Option<String>,
    pub principal_owner: Option<String>,
    pub profile_id: Option<String>,
    pub agent_object_id: Option<String>,
    pub amount_mist: Option<i64>,
    pub new_balance_mist: Option<i64>,
    pub receipt_id: Option<String>,
    pub usage_kind: Option<i16>,
    pub settlement_nonce: Option<i64>,
    pub remaining_mist: Option<i64>,
    pub daily_cap_mist: Option<i64>,
    pub monthly_cap_mist: Option<i64>,
    pub budget_mist: Option<i64>,
    pub require_approval_above_mist: Option<i64>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_usage_lines)]
pub struct NewAiCreditUsageLine {
    pub receipt_id: String,
    pub balance_id: String,
    pub agent_object_id: String,
    pub usage_kind: i16,
    pub amount_mist: i64,
    pub model_id: Option<String>,
    pub tool_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub settled: bool,
    pub settlement_tx: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_usage_lines)]
pub struct AiCreditUsageLineRow {
    pub id: i64,
    pub receipt_id: String,
    pub balance_id: String,
    pub agent_object_id: String,
    pub usage_kind: i16,
    pub amount_mist: i64,
    pub model_id: Option<String>,
    pub tool_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    pub settled: bool,
    pub settlement_tx: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub organization_id: Option<String>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = ai_spend_reservations)]
pub struct NewAiSpendReservation {
    pub balance_id: String,
    pub reservation_nonce: i64,
    pub agent_object_id: String,
    pub status: String,
    pub max_amount_mist: i64,
    pub captured_mist: Option<i64>,
    pub released_mist: Option<i64>,
    pub provider_envelope_hash_hex: String,
    pub request_hash_hex: String,
    pub fx_quote_id_hex: String,
    pub myso_usd_e8: i64,
    pub markup_bps: i64,
    pub provider_cost_usd_micros: Option<i64>,
    pub provider_generation_hash_hex: Option<String>,
    pub capture_deadline_ms: i64,
    pub hard_expiry_ms: i64,
    pub available_mist: i64,
    pub reserve_event_id: String,
    pub reserve_transaction_id: String,
    pub terminal_event_id: Option<String>,
    pub terminal_transaction_id: Option<String>,
    pub terminal_at_ms: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = ai_spend_reservations)]
pub struct AiSpendReservationRow {
    pub balance_id: String,
    pub reservation_nonce: i64,
    pub agent_object_id: String,
    pub status: String,
    pub max_amount_mist: i64,
    pub captured_mist: Option<i64>,
    pub released_mist: Option<i64>,
    pub provider_envelope_hash_hex: String,
    pub request_hash_hex: String,
    pub fx_quote_id_hex: String,
    pub myso_usd_e8: i64,
    pub markup_bps: i64,
    pub provider_cost_usd_micros: Option<i64>,
    pub provider_generation_hash_hex: Option<String>,
    pub capture_deadline_ms: i64,
    pub hard_expiry_ms: i64,
    pub available_mist: i64,
    pub reserve_event_id: String,
    pub reserve_transaction_id: String,
    pub terminal_event_id: Option<String>,
    pub terminal_transaction_id: Option<String>,
    pub terminal_at_ms: Option<i64>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_spend_approvals)]
pub struct NewAiCreditSpendApproval {
    pub balance_id: String,
    pub agent_object_id: String,
    pub status: String,
    pub requested_amount_mist: Option<i64>,
    pub threshold_mist: Option<i64>,
    pub approval_nonce: Option<i64>,
    pub max_amount_mist: Option<i64>,
    pub expires_at_ms: Option<i64>,
    pub approved_by: Option<String>,
    pub approved_by_agent_id: Option<String>,
    pub organization_id: Option<String>,
    pub consumed_amount_mist: Option<i64>,
    pub requested_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub event_id: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_spend_approvals)]
pub struct AiCreditSpendApprovalRow {
    pub balance_id: String,
    pub agent_object_id: String,
    pub status: String,
    pub requested_amount_mist: Option<i64>,
    pub threshold_mist: Option<i64>,
    pub approval_nonce: Option<i64>,
    pub max_amount_mist: Option<i64>,
    pub expires_at_ms: Option<i64>,
    pub approved_by: Option<String>,
    pub approved_by_agent_id: Option<String>,
    pub organization_id: Option<String>,
    pub consumed_amount_mist: Option<i64>,
    pub requested_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub event_id: Option<String>,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_config)]
pub struct NewAiCreditConfig {
    pub updated_by: String,
    pub oracle_pubkey_hex: String,
    pub treasury_address: String,
    pub min_deposit_mist: i64,
    pub max_single_settlement_mist: i64,
    pub receipt_ttl_ms: i64,
    pub oracle_markup_bps: i64,
    pub catalog_version: Option<String>,
    pub version: i64,
    pub updated_at: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_config)]
pub struct AiCreditConfigRow {
    pub id: i32,
    pub updated_by: String,
    pub oracle_pubkey_hex: String,
    pub treasury_address: String,
    pub min_deposit_mist: i64,
    pub max_single_settlement_mist: i64,
    pub receipt_ttl_ms: i64,
    pub oracle_markup_bps: i64,
    pub catalog_version: Option<String>,
    pub version: i64,
    pub updated_at: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}
