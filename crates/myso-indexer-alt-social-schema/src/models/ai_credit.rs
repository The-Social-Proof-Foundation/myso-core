// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{ai_credit_agent_budgets, ai_credit_balances, ai_credit_events, ai_credit_usage_lines};

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = ai_credit_balances)]
pub struct NewAiCreditBalance {
    pub balance_id: String,
    pub memory_account_id: String,
    pub principal_owner: String,
    pub profile_id: String,
    pub balance_mist: i64,
    pub reserved_mist: i64,
    pub spent_total_mist: i64,
    pub daily_cap_mist: Option<i64>,
    pub monthly_cap_mist: Option<i64>,
    pub spent_day_mist: i64,
    pub spent_month_mist: i64,
    pub settlement_nonce: i64,
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
    pub reserved_mist: i64,
    pub spent_total_mist: i64,
    pub daily_cap_mist: Option<i64>,
    pub monthly_cap_mist: Option<i64>,
    pub spent_day_mist: i64,
    pub spent_month_mist: i64,
    pub settlement_nonce: i64,
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
    pub credits: Option<i64>,
    pub receipt_id: Option<String>,
    pub usage_kind: Option<i16>,
    pub settlement_nonce: Option<i64>,
    pub remaining_mist: Option<i64>,
    pub credits_remaining: Option<i64>,
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
}
