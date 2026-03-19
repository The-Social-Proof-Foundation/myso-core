// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;
use chrono::Utc;
use myso_indexer_alt_social_schema::models::{
    NewInsuranceConfig, NewInsuranceEventLog, NewInsuranceMarketExposure, NewInsurancePolicy,
    NewInsurancePolicyEvent, NewInsuranceUserExposure, NewInsuranceVault,
    NewInsuranceVaultTransaction, STATUS_ACTIVE, STATUS_CANCELLED, STATUS_CLAIMED, STATUS_EXPIRED,
};

fn transaction_id_from_event_id(event_id: &str) -> String {
    event_id.split(':').next().unwrap_or(event_id).to_string()
}

fn json_to_i64(v: &serde_json::Value) -> i64 {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
        .unwrap_or(0)
}

fn json_str(v: &serde_json::Value) -> String {
    v.as_str().unwrap_or_default().to_string()
}

fn new_insurance_event_log(
    event_type: &str,
    event_data: &serde_json::Value,
    event_id: &str,
) -> NewInsuranceEventLog {
    NewInsuranceEventLog {
        event_type: event_type.to_string(),
        event_data: event_data.clone(),
        event_id: event_id.to_string(),
        created_at: Utc::now(),
    }
}

pub fn handle_insurance_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
    timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let tx = transaction_id_from_event_id(event_id);
    let timestamp_ms_i64 = timestamp_ms as i64;
    match event_name {
        "ConfigInitializedEvent" => {
            process_config_initialized_event(data, &tx, event_id, timestamp_ms_i64)
        }
        "ConfigUpdatedEvent" => process_config_updated_event(data, &tx, event_id, timestamp_ms_i64),
        "UnderwriterVaultCreatedEvent" => process_vault_created_event(data, &tx, event_id),
        "UnderwriterVaultDepositedEvent" => {
            process_vault_deposited_event(data, &tx, event_id, timestamp_ms_i64)
        }
        "UnderwriterVaultWithdrawnEvent" => {
            process_vault_withdrawn_event(data, &tx, event_id, timestamp_ms_i64)
        }
        "CoveragePurchasedEvent" => {
            process_coverage_purchased_event(data, &tx, event_id, timestamp_ms_i64)
        }
        "CoverageCancelledEvent" => {
            process_coverage_cancelled_event(data, &tx, event_id, timestamp_ms_i64)
        }
        "CoverageClaimedEvent" => {
            process_coverage_claimed_event(data, &tx, event_id, timestamp_ms_i64)
        }
        "PolicyExpiredEvent" => process_policy_expired_event(data, &tx, event_id, timestamp_ms_i64),
        _ => None,
    }
}

fn process_config_initialized_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let admin = json_str(data.get("admin")?);
    let min_coverage_bps = json_to_i64(data.get("min_coverage_bps")?);
    let max_coverage_bps = json_to_i64(data.get("max_coverage_bps")?);
    let max_duration_ms = json_to_i64(data.get("max_duration_ms")?);
    let fee_bps = json_to_i64(data.get("fee_bps")?);

    let config = NewInsuranceConfig {
        updated_by: admin,
        enable_flag: false,
        min_coverage_bps,
        max_coverage_bps,
        max_duration_ms,
        fee_bps,
        version: 1,
        timestamp_ms,
        time: Utc::now(),
        transaction_id: tx.to_string(),
    };

    Some(vec![
        SocialEventRow::InsuranceConfig(config),
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "ConfigInitializedEvent",
            data,
            event_id,
        )),
    ])
}

fn process_config_updated_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    default_timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let updated_by = json_str(data.get("updated_by")?);
    let enable_flag = data.get("enable_flag")?.as_bool().unwrap_or(false);
    let min_coverage_bps = json_to_i64(data.get("min_coverage_bps")?);
    let max_coverage_bps = json_to_i64(data.get("max_coverage_bps")?);
    let max_duration_ms = json_to_i64(data.get("max_duration_ms")?);
    let fee_bps = json_to_i64(data.get("fee_bps")?);
    let timestamp_ms = data
        .get("timestamp")
        .map(json_to_i64)
        .filter(|t| *t > 0)
        .unwrap_or(default_timestamp_ms);

    let config = NewInsuranceConfig {
        updated_by,
        enable_flag,
        min_coverage_bps,
        max_coverage_bps,
        max_duration_ms,
        fee_bps,
        version: 1,
        timestamp_ms,
        time: Utc::now(),
        transaction_id: tx.to_string(),
    };

    Some(vec![
        SocialEventRow::InsuranceConfig(config),
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "ConfigUpdatedEvent",
            data,
            event_id,
        )),
    ])
}

fn process_vault_created_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let vault_id = json_str(data.get("vault_id")?);
    let underwriter = json_str(data.get("underwriter")?);
    let base_rate_bps_per_day = json_to_i64(data.get("base_rate_bps_per_day")?);
    let utilization_multiplier_bps = json_to_i64(data.get("utilization_multiplier_bps")?);
    let max_exposure_per_market = json_to_i64(data.get("max_exposure_per_market")?);
    let max_exposure_per_user = json_to_i64(data.get("max_exposure_per_user")?);

    let now = Utc::now().naive_utc();
    let vault = NewInsuranceVault {
        vault_id: vault_id.clone(),
        underwriter,
        capital_balance: 0,
        reserved: 0,
        base_rate_bps_per_day,
        utilization_multiplier_bps,
        max_exposure_per_market,
        max_exposure_per_user,
        version: 1,
        created_at: now,
        updated_at: now,
        transaction_id: tx.to_string(),
    };

    Some(vec![
        SocialEventRow::InsuranceVault(vault),
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "UnderwriterVaultCreatedEvent",
            data,
            event_id,
        )),
    ])
}

fn process_vault_deposited_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let vault_id = json_str(data.get("vault_id")?);
    let amount = json_to_i64(data.get("amount")?);
    let new_balance = json_to_i64(data.get("new_balance")?);

    let transaction = NewInsuranceVaultTransaction {
        vault_id: vault_id.clone(),
        transaction_type: "DEPOSIT".to_string(),
        amount,
        balance_after: new_balance,
        timestamp_ms,
        time: Utc::now(),
        transaction_id: tx.to_string(),
    };

    Some(vec![
        SocialEventRow::InsuranceVaultTransaction(transaction),
        SocialEventRow::InsuranceVaultBalanceUpdate {
            vault_id,
            new_balance,
        },
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "UnderwriterVaultDepositedEvent",
            data,
            event_id,
        )),
    ])
}

fn process_vault_withdrawn_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let vault_id = json_str(data.get("vault_id")?);
    let amount = json_to_i64(data.get("amount")?);
    let new_balance = json_to_i64(data.get("new_balance")?);

    let transaction = NewInsuranceVaultTransaction {
        vault_id: vault_id.clone(),
        transaction_type: "WITHDRAWAL".to_string(),
        amount,
        balance_after: new_balance,
        timestamp_ms,
        time: Utc::now(),
        transaction_id: tx.to_string(),
    };

    Some(vec![
        SocialEventRow::InsuranceVaultTransaction(transaction),
        SocialEventRow::InsuranceVaultBalanceUpdate {
            vault_id,
            new_balance,
        },
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "UnderwriterVaultWithdrawnEvent",
            data,
            event_id,
        )),
    ])
}

fn process_coverage_purchased_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let policy_id = json_str(data.get("policy_id")?);
    let market_id = json_str(data.get("market_id")?);
    let insured = json_str(data.get("insured")?);
    let option_id = data.get("option_id")?.as_u64().unwrap_or(0) as i16;
    let covered_amount = json_to_i64(data.get("covered_amount")?);
    let coverage_bps = json_to_i64(data.get("coverage_bps")?);
    let premium_paid = json_to_i64(data.get("premium_paid")?);
    let reserve_locked = json_to_i64(data.get("reserve_locked")?);
    let expiry_time_ms = json_to_i64(data.get("expiry_time_ms")?);

    let vault_id = data
        .get("vault_id")
        .and_then(|v| v.as_str())
        .map(String::from)
        .unwrap_or_else(|| market_id.clone());

    let now = Utc::now().naive_utc();
    let policy = NewInsurancePolicy {
        policy_id: policy_id.clone(),
        market_id: market_id.clone(),
        insured: insured.clone(),
        option_id,
        covered_amount,
        coverage_bps,
        premium_paid,
        start_time_ms: timestamp_ms,
        expiry_time_ms,
        vault_id: vault_id.clone(),
        status: STATUS_ACTIVE,
        created_at: now,
        updated_at: now,
        transaction_id: tx.to_string(),
    };

    let policy_event = NewInsurancePolicyEvent {
        policy_id: policy_id.clone(),
        event_type: "PURCHASED".to_string(),
        market_id: market_id.clone(),
        insured: insured.clone(),
        option_id,
        covered_amount,
        coverage_bps,
        premium_paid,
        reserve_locked,
        refunded_amount: None,
        fee_paid: None,
        payout: None,
        timestamp_ms,
        time: Utc::now(),
        transaction_id: tx.to_string(),
    };

    let market_exposure = NewInsuranceMarketExposure {
        vault_id: vault_id.clone(),
        market_id: market_id.clone(),
        option_id,
        reserved_amount: reserve_locked,
        timestamp_ms,
        time: Utc::now(),
        transaction_id: tx.to_string(),
    };

    let user_exposure = NewInsuranceUserExposure {
        vault_id,
        insured: insured.clone(),
        reserved_amount: reserve_locked,
        timestamp_ms,
        time: Utc::now(),
        transaction_id: tx.to_string(),
    };

    Some(vec![
        SocialEventRow::InsurancePolicy(policy),
        SocialEventRow::InsurancePolicyEvent(policy_event),
        SocialEventRow::InsuranceMarketExposure(market_exposure),
        SocialEventRow::InsuranceUserExposure(user_exposure),
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "CoveragePurchasedEvent",
            data,
            event_id,
        )),
    ])
}

fn process_coverage_cancelled_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    _timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let policy_id = json_str(data.get("policy_id")?);
    let _insured = json_str(data.get("insured")?);
    let refunded_amount = json_to_i64(data.get("refunded_amount")?);
    let fee_paid = json_to_i64(data.get("fee_paid")?);

    Some(vec![
        SocialEventRow::InsurancePolicyStatusUpdate {
            policy_id: policy_id.clone(),
            status: STATUS_CANCELLED,
        },
        SocialEventRow::InsurancePolicyEventFromPolicy {
            policy_id,
            event_type: "CANCELLED".to_string(),
            refunded_amount: Some(refunded_amount),
            fee_paid: Some(fee_paid),
            payout: None,
            reserve_released: None,
            timestamp_ms: _timestamp_ms,
            transaction_id: tx.to_string(),
        },
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "CoverageCancelledEvent",
            data,
            event_id,
        )),
    ])
}

fn process_coverage_claimed_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let policy_id = json_str(data.get("policy_id")?);
    let payout = json_to_i64(data.get("payout")?);

    Some(vec![
        SocialEventRow::InsurancePolicyStatusUpdate {
            policy_id: policy_id.clone(),
            status: STATUS_CLAIMED,
        },
        SocialEventRow::InsurancePolicyEventFromPolicy {
            policy_id,
            event_type: "CLAIMED".to_string(),
            refunded_amount: None,
            fee_paid: None,
            payout: Some(payout),
            reserve_released: None,
            timestamp_ms,
            transaction_id: tx.to_string(),
        },
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "CoverageClaimedEvent",
            data,
            event_id,
        )),
    ])
}

fn process_policy_expired_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let policy_id = json_str(data.get("policy_id")?);
    let reserve_released = data.get("reserve_released").map(json_to_i64);

    Some(vec![
        SocialEventRow::InsurancePolicyStatusUpdate {
            policy_id: policy_id.clone(),
            status: STATUS_EXPIRED,
        },
        SocialEventRow::InsurancePolicyEventFromPolicy {
            policy_id,
            event_type: "EXPIRED".to_string(),
            refunded_amount: None,
            fee_paid: None,
            payout: None,
            reserve_released,
            timestamp_ms,
            transaction_id: tx.to_string(),
        },
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "PolicyExpiredEvent",
            data,
            event_id,
        )),
    ])
}
