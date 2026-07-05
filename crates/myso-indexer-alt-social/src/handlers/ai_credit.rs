// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::Utc;

use super::memory::chain_audit_row;
use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    AuditAction, NewAiCreditAgentBudget, NewAiCreditBalance, NewAiCreditConfig, NewAiCreditEvent,
    NewAiCreditSpendApproval, APPROVAL_STATUS_APPROVED, APPROVAL_STATUS_CONSUMED,
    APPROVAL_STATUS_REVOKED,
};

pub(crate) fn json_to_i64(v: &serde_json::Value) -> i64 {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
        .unwrap_or(0)
}

fn json_opt_i64(v: &serde_json::Value) -> Option<i64> {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
}

fn json_str(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key).and_then(|v| v.as_str()).map(String::from)
}

pub(crate) fn new_ai_credit_config_with_defaults() -> NewAiCreditConfig {
    NewAiCreditConfig {
        updated_by: String::new(),
        oracle_pubkey_hex: String::new(),
        treasury_address: String::new(),
        min_deposit_mist: 0,
        max_single_settlement_mist: 0,
        receipt_ttl_ms: 0,
        oracle_markup_bps: 1500,
        catalog_version: None,
        version: 0,
        updated_at: 0,
        transaction_id: String::new(),
        time: Utc::now(),
    }
}

fn config_updated_at(data: &serde_json::Value, now: chrono::DateTime<chrono::Utc>) -> i64 {
    data.get("timestamp")
        .or_else(|| data.get("updated_at"))
        .and_then(json_opt_i64)
        .unwrap_or_else(|| now.timestamp_millis())
}

fn config_updated_by(data: &serde_json::Value) -> String {
    json_str(data, "updated_by")
        .or_else(|| json_str(data, "admin"))
        .unwrap_or_default()
}

pub(crate) fn json_receipt_id(data: &serde_json::Value) -> Option<String> {
    let v = data.get("receipt_id")?;
    if let Some(s) = v.as_str() {
        return Some(s.to_string());
    }
    v.as_u64().map(|n| n.to_string())
}

fn config_audit_event(
    event_name: &str,
    event_id: &str,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> NewAiCreditEvent {
    NewAiCreditEvent {
        event_type: event_name.to_string(),
        balance_id: None,
        memory_account_id: None,
        principal_owner: None,
        profile_id: None,
        agent_object_id: None,
        amount_mist: None,
        new_balance_mist: None,
        credits: None,
        receipt_id: None,
        usage_kind: None,
        settlement_nonce: None,
        remaining_mist: None,
        credits_remaining: None,
        daily_cap_mist: None,
        monthly_cap_mist: None,
        budget_mist: None,
        require_approval_above_mist: None,
        event_id: event_id.to_string(),
        transaction_id: transaction_id.to_string(),
        time: now,
    }
}

pub fn handle_ai_credit_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = event_id.split(':').next().unwrap_or(event_id).to_string();
    let now = Utc::now();
    match event_name {
        "AiCreditBalanceCreated" => {
            let balance_id = json_str(data, "balance_id")?;
            let memory_account_id = json_str(data, "memory_account_id")?;
            let principal_owner = json_str(data, "principal_owner")?;
            let profile_id = json_str(data, "profile_id")?;
            Some(vec![
                SocialEventRow::AiCreditBalanceUpsert(NewAiCreditBalance {
                    balance_id: balance_id.clone(),
                    memory_account_id: memory_account_id.clone(),
                    principal_owner: principal_owner.clone(),
                    profile_id: profile_id.clone(),
                    balance_mist: 0,
                    spent_total_mist: 0,
                    daily_cap_mist: None,
                    monthly_cap_mist: None,
                    spent_day_mist: 0,
                    spent_month_mist: 0,
                    settlement_nonce: 0,
                    active: true,
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                    time: now,
                }),
                SocialEventRow::ProfileAiCreditBalanceLink {
                    profile_id: profile_id.clone(),
                    ai_credit_balance_id: balance_id.clone(),
                },
                SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                    event_type: event_name.to_string(),
                    balance_id: Some(balance_id),
                    memory_account_id: Some(memory_account_id),
                    principal_owner: Some(principal_owner),
                    profile_id: Some(profile_id),
                    agent_object_id: None,
                    amount_mist: None,
                    new_balance_mist: Some(0),
                    credits: Some(0),
                    receipt_id: None,
                    usage_kind: None,
                    settlement_nonce: None,
                    remaining_mist: None,
                    credits_remaining: Some(0),
                    daily_cap_mist: None,
                    monthly_cap_mist: None,
                    budget_mist: None,
                    require_approval_above_mist: None,
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AiCreditDeposited" => {
            let balance_id = json_str(data, "balance_id")?;
            let amount_mist =
                json_to_i64(data.get("amount_mist").unwrap_or(&serde_json::Value::Null));
            let new_balance_mist = json_to_i64(
                data.get("new_balance_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let credits = json_to_i64(data.get("credits").unwrap_or(&serde_json::Value::Null));
            Some(vec![
                SocialEventRow::AiCreditBalanceBalanceUpdate {
                    balance_id: balance_id.clone(),
                    balance_mist: new_balance_mist,
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                    event_type: event_name.to_string(),
                    balance_id: Some(balance_id),
                    memory_account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    agent_object_id: None,
                    amount_mist: Some(amount_mist),
                    new_balance_mist: Some(new_balance_mist),
                    credits: Some(credits),
                    receipt_id: None,
                    usage_kind: None,
                    settlement_nonce: None,
                    remaining_mist: Some(new_balance_mist),
                    credits_remaining: Some(credits),
                    daily_cap_mist: None,
                    monthly_cap_mist: None,
                    budget_mist: None,
                    require_approval_above_mist: None,
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AiCreditWithdrawn" => {
            let balance_id = json_str(data, "balance_id")?;
            let amount_mist =
                json_to_i64(data.get("amount_mist").unwrap_or(&serde_json::Value::Null));
            let new_balance_mist = json_to_i64(
                data.get("new_balance_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            Some(vec![
                SocialEventRow::AiCreditBalanceBalanceUpdate {
                    balance_id: balance_id.clone(),
                    balance_mist: new_balance_mist,
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                    event_type: event_name.to_string(),
                    balance_id: Some(balance_id),
                    memory_account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    agent_object_id: None,
                    amount_mist: Some(amount_mist),
                    new_balance_mist: Some(new_balance_mist),
                    credits: None,
                    receipt_id: None,
                    usage_kind: None,
                    settlement_nonce: None,
                    remaining_mist: Some(new_balance_mist),
                    credits_remaining: None,
                    daily_cap_mist: None,
                    monthly_cap_mist: None,
                    budget_mist: None,
                    require_approval_above_mist: None,
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AiCreditAccountCapsUpdated" => {
            let balance_id = json_str(data, "balance_id")?;
            let daily_cap_mist = json_opt_i64(
                data.get("daily_cap_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let monthly_cap_mist = json_opt_i64(
                data.get("monthly_cap_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            Some(vec![
                SocialEventRow::AiCreditBalanceCapsUpdate {
                    balance_id: balance_id.clone(),
                    daily_cap_mist,
                    monthly_cap_mist,
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                    event_type: event_name.to_string(),
                    balance_id: Some(balance_id),
                    memory_account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    agent_object_id: None,
                    amount_mist: None,
                    new_balance_mist: None,
                    credits: None,
                    receipt_id: None,
                    usage_kind: None,
                    settlement_nonce: None,
                    remaining_mist: None,
                    credits_remaining: None,
                    daily_cap_mist,
                    monthly_cap_mist,
                    budget_mist: None,
                    require_approval_above_mist: None,
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AiCreditAgentBudgetUpdated" => {
            let balance_id = json_str(data, "balance_id")?;
            let agent_object_id = json_str(data, "agent_object_id")?;
            let budget_mist =
                json_opt_i64(data.get("budget_mist").unwrap_or(&serde_json::Value::Null));
            let daily_cap_mist = json_opt_i64(
                data.get("daily_cap_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let monthly_cap_mist = json_opt_i64(
                data.get("monthly_cap_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let require_approval_above_mist = json_opt_i64(
                data.get("require_approval_above_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            Some(vec![
                SocialEventRow::AiCreditAgentBudgetUpsert(NewAiCreditAgentBudget {
                    balance_id: balance_id.clone(),
                    agent_object_id: agent_object_id.clone(),
                    budget_mist,
                    spent_mist: 0,
                    daily_cap_mist,
                    monthly_cap_mist,
                    require_approval_above_mist,
                    enabled: true,
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                    time: now,
                }),
                SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                    event_type: event_name.to_string(),
                    balance_id: Some(balance_id),
                    memory_account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    agent_object_id: Some(agent_object_id),
                    amount_mist: None,
                    new_balance_mist: None,
                    credits: None,
                    receipt_id: None,
                    usage_kind: None,
                    settlement_nonce: None,
                    remaining_mist: None,
                    credits_remaining: None,
                    daily_cap_mist,
                    monthly_cap_mist,
                    budget_mist,
                    require_approval_above_mist,
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AiCreditAgentBudgetDisabled" => {
            let balance_id = json_str(data, "balance_id")?;
            let agent_object_id = json_str(data, "agent_object_id")?;
            Some(vec![
                SocialEventRow::AiCreditAgentBudgetDisable {
                    balance_id: balance_id.clone(),
                    agent_object_id: agent_object_id.clone(),
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                    event_type: event_name.to_string(),
                    balance_id: Some(balance_id),
                    memory_account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    agent_object_id: Some(agent_object_id),
                    amount_mist: None,
                    new_balance_mist: None,
                    credits: None,
                    receipt_id: None,
                    usage_kind: None,
                    settlement_nonce: None,
                    remaining_mist: None,
                    credits_remaining: None,
                    daily_cap_mist: None,
                    monthly_cap_mist: None,
                    budget_mist: None,
                    require_approval_above_mist: None,
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AiCreditUsageSettled" => {
            let balance_id = json_str(data, "balance_id")?;
            let agent_object_id = json_str(data, "agent_object_id")?;
            let amount_mist =
                json_to_i64(data.get("amount_mist").unwrap_or(&serde_json::Value::Null));
            let remaining_mist = json_to_i64(
                data.get("remaining_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let credits_remaining = json_to_i64(
                data.get("credits_remaining")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let settlement_nonce = json_to_i64(
                data.get("settlement_nonce")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let usage_kind = data
                .get("usage_kind")
                .and_then(|v| v.as_u64())
                .and_then(|n| i16::try_from(n).ok());
            let receipt_id = json_receipt_id(data);
            let mut rows = vec![
                SocialEventRow::AiCreditBalanceBalanceUpdate {
                    balance_id: balance_id.clone(),
                    balance_mist: remaining_mist,
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditBalanceSettlementUpdate {
                    balance_id: balance_id.clone(),
                    settlement_nonce,
                    spent_increment_mist: amount_mist,
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditAgentBudgetSpendUpdate {
                    balance_id: balance_id.clone(),
                    agent_object_id: agent_object_id.clone(),
                    spent_increment_mist: amount_mist,
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                },
            ];
            if let Some(receipt_id) = receipt_id.clone() {
                rows.push(SocialEventRow::AiCreditUsageLineSettle {
                    receipt_id,
                    settlement_tx: transaction_id.clone(),
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                });
            }
            // Tier 1 org spend attribution (org resolved via sub_agents at commit time).
            rows.push(SocialEventRow::AiCreditOrgSpendFromAgent {
                agent_object_id: agent_object_id.clone(),
                amount_mist,
                receipt_id: receipt_id.clone(),
                activity_at_ms: now.timestamp_millis(),
            });
            rows.push(SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                event_type: event_name.to_string(),
                balance_id: Some(balance_id),
                memory_account_id: None,
                principal_owner: None,
                profile_id: None,
                agent_object_id: Some(agent_object_id),
                amount_mist: Some(amount_mist),
                new_balance_mist: None,
                credits: None,
                receipt_id,
                usage_kind,
                settlement_nonce: Some(settlement_nonce),
                remaining_mist: Some(remaining_mist),
                credits_remaining: Some(credits_remaining),
                daily_cap_mist: None,
                monthly_cap_mist: None,
                budget_mist: None,
                require_approval_above_mist: None,
                event_id: event_id.to_string(),
                transaction_id,
                time: now,
            }));
            Some(rows)
        }
        "AiCreditBalancePaused" | "AiCreditBalanceReactivated" => {
            let balance_id = json_str(data, "balance_id")?;
            let active = event_name == "AiCreditBalanceReactivated";
            Some(vec![
                SocialEventRow::AiCreditBalanceActiveUpdate {
                    balance_id: balance_id.clone(),
                    active,
                    updated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                    event_type: event_name.to_string(),
                    balance_id: Some(balance_id),
                    memory_account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    agent_object_id: None,
                    amount_mist: None,
                    new_balance_mist: None,
                    credits: None,
                    receipt_id: None,
                    usage_kind: None,
                    settlement_nonce: None,
                    remaining_mist: None,
                    credits_remaining: None,
                    daily_cap_mist: None,
                    monthly_cap_mist: None,
                    budget_mist: None,
                    require_approval_above_mist: None,
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AiCreditBalanceDepleted" => {
            let balance_id = json_str(data, "balance_id")?;
            Some(vec![SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                event_type: event_name.to_string(),
                balance_id: Some(balance_id),
                memory_account_id: None,
                principal_owner: None,
                profile_id: None,
                agent_object_id: None,
                amount_mist: None,
                new_balance_mist: Some(0),
                credits: Some(0),
                receipt_id: None,
                usage_kind: None,
                settlement_nonce: None,
                remaining_mist: Some(0),
                credits_remaining: Some(0),
                daily_cap_mist: None,
                monthly_cap_mist: None,
                budget_mist: None,
                require_approval_above_mist: None,
                event_id: event_id.to_string(),
                transaction_id,
                time: now,
            })])
        }
        "AiCreditAgentBudgetChanged" => {
            let balance_id = json_str(data, "balance_id")?;
            let agent_object_id = json_str(data, "agent_object_id")?;
            let enabled = data
                .get("enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            let set_by = json_str(data, "set_by")?;
            let set_by_agent_id = json_str(data, "set_by_agent_id");
            let organization_id = json_str(data, "organization_id");
            let had_previous = data
                .get("had_previous_entry")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let prev_state = if had_previous {
                Some(serde_json::json!({
                    "budget_mist": data.get("prev_budget_mist"),
                    "daily_cap_mist": data.get("prev_daily_cap_mist"),
                    "monthly_cap_mist": data.get("prev_monthly_cap_mist"),
                    "require_approval_above_mist": data.get("prev_require_approval_above_mist"),
                    "enabled": data.get("prev_enabled"),
                }))
            } else {
                None
            };
            let new_state = serde_json::json!({
                "budget_mist": data.get("budget_mist"),
                "daily_cap_mist": data.get("daily_cap_mist"),
                "monthly_cap_mist": data.get("monthly_cap_mist"),
                "require_approval_above_mist": data.get("require_approval_above_mist"),
                "enabled": enabled,
                "set_by_agent_id": set_by_agent_id,
            });
            let mut audit = chain_audit_row(
                if enabled {
                    AuditAction::AgentBudgetChange
                } else {
                    AuditAction::AgentBudgetDisable
                },
                set_by,
                "agent_budget",
                agent_object_id,
                organization_id,
                None,
                prev_state,
                Some(new_state),
                event_id,
                &transaction_id,
                now,
            );
            audit.metadata = Some(serde_json::json!({ "balance_id": balance_id }));
            Some(vec![SocialEventRow::AuditLog(audit)])
        }
        "AiCreditSpendApproved" => {
            let balance_id = json_str(data, "balance_id")?;
            let agent_object_id = json_str(data, "agent_object_id")?;
            let approval_nonce = json_to_i64(
                data.get("approval_nonce")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let max_amount_mist = json_to_i64(
                data.get("max_amount_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let expires_at_ms = json_to_i64(
                data.get("expires_at_ms")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let approved_by = json_str(data, "approved_by")?;
            let approved_by_agent_id = json_str(data, "approved_by_agent_id");
            let organization_id = json_str(data, "organization_id");
            Some(vec![
                SocialEventRow::AiCreditSpendApprovalUpsert(NewAiCreditSpendApproval {
                    balance_id: balance_id.clone(),
                    agent_object_id: agent_object_id.clone(),
                    status: APPROVAL_STATUS_APPROVED.to_string(),
                    requested_amount_mist: None,
                    threshold_mist: None,
                    approval_nonce: Some(approval_nonce),
                    max_amount_mist: Some(max_amount_mist),
                    expires_at_ms: Some(expires_at_ms),
                    approved_by: Some(approved_by.clone()),
                    approved_by_agent_id: approved_by_agent_id.clone(),
                    organization_id: organization_id.clone(),
                    consumed_amount_mist: None,
                    requested_at: now,
                    updated_at: now,
                    event_id: Some(event_id.to_string()),
                }),
                SocialEventRow::AuditLog(chain_audit_row(
                    AuditAction::SpendApprovalApprove,
                    approved_by,
                    "spend_approval",
                    agent_object_id,
                    organization_id,
                    None,
                    None,
                    Some(serde_json::json!({
                        "balance_id": balance_id,
                        "approval_nonce": approval_nonce,
                        "max_amount_mist": max_amount_mist,
                        "expires_at_ms": expires_at_ms,
                        "approved_by_agent_id": approved_by_agent_id,
                    })),
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        "AiCreditSpendApprovalRevoked" => {
            let balance_id = json_str(data, "balance_id")?;
            let agent_object_id = json_str(data, "agent_object_id")?;
            let approval_nonce = json_to_i64(
                data.get("approval_nonce")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let revoked_by = json_str(data, "revoked_by")?;
            Some(vec![
                SocialEventRow::AiCreditSpendApprovalStatus {
                    balance_id: balance_id.clone(),
                    agent_object_id: agent_object_id.clone(),
                    status: APPROVAL_STATUS_REVOKED.to_string(),
                    consumed_amount_mist: None,
                    event_id: event_id.to_string(),
                },
                SocialEventRow::AuditLog(chain_audit_row(
                    AuditAction::SpendApprovalRevoke,
                    revoked_by,
                    "spend_approval",
                    agent_object_id,
                    None,
                    None,
                    Some(serde_json::json!({ "approval_nonce": approval_nonce })),
                    None,
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        "AiCreditSpendApprovalConsumed" => {
            let balance_id = json_str(data, "balance_id")?;
            let agent_object_id = json_str(data, "agent_object_id")?;
            let approval_nonce = json_to_i64(
                data.get("approval_nonce")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let amount_mist =
                json_to_i64(data.get("amount_mist").unwrap_or(&serde_json::Value::Null));
            let approved_by = json_str(data, "approved_by")?;
            Some(vec![
                SocialEventRow::AiCreditSpendApprovalStatus {
                    balance_id: balance_id.clone(),
                    agent_object_id: agent_object_id.clone(),
                    status: APPROVAL_STATUS_CONSUMED.to_string(),
                    consumed_amount_mist: Some(amount_mist),
                    event_id: event_id.to_string(),
                },
                SocialEventRow::AuditLog(chain_audit_row(
                    AuditAction::SpendApprovalConsume,
                    approved_by,
                    "spend_approval",
                    agent_object_id,
                    None,
                    None,
                    None,
                    Some(serde_json::json!({
                        "balance_id": balance_id,
                        "approval_nonce": approval_nonce,
                        "amount_mist": amount_mist,
                    })),
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        "AiCreditConfigInitialized" => {
            let updated_by = config_updated_by(data);
            let oracle_pubkey_hex = json_str(data, "oracle_pubkey_hex")?;
            let treasury_address = json_str(data, "treasury_address")?;
            let min_deposit_mist = json_to_i64(
                data.get("min_deposit_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let max_single_settlement_mist = json_to_i64(
                data.get("max_single_settlement_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let receipt_ttl_ms = json_to_i64(
                data.get("receipt_ttl_ms")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let oracle_markup_bps = json_to_i64(
                data.get("oracle_markup_bps")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let version = data
                .get("version")
                .and_then(json_opt_i64)
                .unwrap_or(0);
            let updated_at = config_updated_at(data, now);
            Some(vec![
                SocialEventRow::AiCreditConfigUpsert(NewAiCreditConfig {
                    updated_by,
                    oracle_pubkey_hex: oracle_pubkey_hex.clone(),
                    treasury_address: treasury_address.clone(),
                    min_deposit_mist,
                    max_single_settlement_mist,
                    receipt_ttl_ms,
                    oracle_markup_bps,
                    catalog_version: None,
                    version,
                    updated_at,
                    transaction_id: transaction_id.clone(),
                    time: now,
                }),
                SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                    event_type: event_name.to_string(),
                    balance_id: None,
                    memory_account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    agent_object_id: None,
                    amount_mist: None,
                    new_balance_mist: None,
                    credits: None,
                    receipt_id: None,
                    usage_kind: None,
                    settlement_nonce: None,
                    remaining_mist: None,
                    credits_remaining: None,
                    daily_cap_mist: None,
                    monthly_cap_mist: None,
                    budget_mist: None,
                    require_approval_above_mist: None,
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AiCreditSettlementLimitsUpdated" => {
            let updated_by = config_updated_by(data);
            let max_single_settlement_mist = json_to_i64(
                data.get("max_single_settlement_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let receipt_ttl_ms = json_to_i64(
                data.get("receipt_ttl_ms")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let updated_at = config_updated_at(data, now);
            Some(vec![
                SocialEventRow::AiCreditConfigLimitsUpdate {
                    updated_by,
                    max_single_settlement_mist,
                    receipt_ttl_ms,
                    updated_at,
                    time: now,
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditEvent(NewAiCreditEvent {
                    event_type: event_name.to_string(),
                    balance_id: None,
                    memory_account_id: None,
                    principal_owner: None,
                    profile_id: None,
                    agent_object_id: None,
                    amount_mist: None,
                    new_balance_mist: None,
                    credits: None,
                    receipt_id: None,
                    usage_kind: None,
                    settlement_nonce: None,
                    remaining_mist: None,
                    credits_remaining: None,
                    daily_cap_mist: None,
                    monthly_cap_mist: None,
                    budget_mist: None,
                    require_approval_above_mist: None,
                    event_id: event_id.to_string(),
                    transaction_id,
                    time: now,
                }),
            ])
        }
        "AiCreditOraclePubkeyUpdated" => {
            let updated_by = config_updated_by(data);
            let oracle_pubkey_hex = json_str(data, "new_pubkey_hex")?;
            let updated_at = config_updated_at(data, now);
            Some(vec![
                SocialEventRow::AiCreditConfigPubkeyUpdate {
                    updated_by,
                    oracle_pubkey_hex,
                    updated_at,
                    time: now,
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditEvent(config_audit_event(
                    event_name,
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        "AiCreditMarkupUpdated" => {
            let updated_by = config_updated_by(data);
            let oracle_markup_bps = json_to_i64(
                data.get("oracle_markup_bps")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let updated_at = config_updated_at(data, now);
            Some(vec![
                SocialEventRow::AiCreditConfigMarkupUpdate {
                    updated_by,
                    oracle_markup_bps,
                    updated_at,
                    time: now,
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditEvent(config_audit_event(
                    event_name,
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        "AiCreditMinDepositUpdated" => {
            let updated_by = config_updated_by(data);
            let min_deposit_mist = json_to_i64(
                data.get("min_deposit_mist")
                    .unwrap_or(&serde_json::Value::Null),
            );
            let updated_at = config_updated_at(data, now);
            Some(vec![
                SocialEventRow::AiCreditConfigMinDepositUpdate {
                    updated_by,
                    min_deposit_mist,
                    updated_at,
                    time: now,
                    transaction_id: transaction_id.clone(),
                },
                SocialEventRow::AiCreditEvent(config_audit_event(
                    event_name,
                    event_id,
                    &transaction_id,
                    now,
                )),
            ])
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::SocialEventRow;

    #[test]
    fn usage_settled_emits_balance_usage_line_and_audit_rows() {
        let data = serde_json::json!({
            "balance_id": "0x2f41b4f43f505d427e8777c511461de8e50eac26558a996627dded27dce50918",
            "agent_object_id": "0x124043762fbf1db8d8ba247c69a66e71702bebfc4f22ac5663a9b089bde73620",
            "receipt_id": "132625655239685005677817396617643760670",
            "amount_mist": 222222223,
            "usage_kind": 1,
            "settlement_nonce": 1,
            "remaining_mist": 4677777777_i64,
            "credits_remaining": 4
        });
        let rows = handle_ai_credit_event("AiCreditUsageSettled", &data, "tx:0")
            .expect("handler should produce rows");
        assert_eq!(rows.len(), 6);
        assert!(rows.iter().any(|r| {
            matches!(
                r,
                SocialEventRow::AiCreditOrgSpendFromAgent {
                    amount_mist: 222222223,
                    ..
                }
            )
        }));
        assert!(rows.iter().any(|r| {
            matches!(
                r,
                SocialEventRow::AiCreditBalanceSettlementUpdate {
                    settlement_nonce: 1,
                    ..
                }
            )
        }));
        assert!(rows.iter().any(|r| {
            matches!(
                r,
                SocialEventRow::AiCreditUsageLineSettle {
                    receipt_id,
                    settlement_tx,
                    ..
                } if receipt_id == "132625655239685005677817396617643760670"
                    && settlement_tx == "tx"
            )
        }));
        assert!(rows.iter().any(|r| {
            matches!(
                r,
                SocialEventRow::AiCreditEvent(e) if e.event_type == "AiCreditUsageSettled"
                    && e.receipt_id.as_deref() == Some("132625655239685005677817396617643760670")
            )
        }));
    }

    #[test]
    fn json_receipt_id_parses_string_and_u64() {
        let from_str = json_receipt_id(&serde_json::json!({ "receipt_id": "999" }));
        assert_eq!(from_str.as_deref(), Some("999"));
        let from_u64 = json_receipt_id(&serde_json::json!({ "receipt_id": 42 }));
        assert_eq!(from_u64.as_deref(), Some("42"));
    }

    #[test]
    fn oracle_pubkey_updated_emits_pubkey_update_row() {
        let data = serde_json::json!({
            "updated_by": "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
            "new_pubkey_hex": "deadbeef01234567",
        });
        let rows = handle_ai_credit_event("AiCreditOraclePubkeyUpdated", &data, "tx:0")
            .expect("handler should produce rows");
        assert!(rows.iter().any(|r| {
            matches!(
                r,
                SocialEventRow::AiCreditConfigPubkeyUpdate {
                    oracle_pubkey_hex,
                    ..
                } if oracle_pubkey_hex == "deadbeef01234567"
            )
        }));
    }

    #[test]
    fn markup_updated_emits_markup_update_row() {
        let data = serde_json::json!({
            "updated_by": "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
            "oracle_markup_bps": 250,
        });
        let rows = handle_ai_credit_event("AiCreditMarkupUpdated", &data, "tx:0")
            .expect("handler should produce rows");
        assert!(rows.iter().any(|r| {
            matches!(
                r,
                SocialEventRow::AiCreditConfigMarkupUpdate {
                    oracle_markup_bps: 250,
                    ..
                }
            )
        }));
    }
}
