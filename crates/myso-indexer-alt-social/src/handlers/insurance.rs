// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;
use chrono::Utc;
use myso_indexer_alt_social_schema::models::{
    NewInsuranceConfig, NewInsuranceCoverageRoute, NewInsuranceEventLog,
    NewInsuranceMarketExposure, NewInsurancePolicy, NewInsurancePolicyEvent, NewInsuranceRouteFill,
    NewInsuranceRouterConfig, NewInsuranceUserExposure, NewInsuranceVault,
    NewInsuranceVaultTransaction, DEFAULT_EXPOSURE_CAP_BPS, DEFAULT_EXPOSURE_K_BPS,
    DEFAULT_IMPLIED_PROB_FLOOR_BPS, DEFAULT_LIQ_CAP_BPS, DEFAULT_LIQ_REF_AMOUNT,
    DEFAULT_MAX_COVERAGE_BPS, DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS, DEFAULT_MAX_DURATION_MS,
    DEFAULT_MAX_RISK_MULTIPLIER_BPS, DEFAULT_MIN_COVERAGE_BPS, DEFAULT_MIN_PREMIUM_AMOUNT,
    DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY, DEFAULT_ODDS_CAP_BPS, DEFAULT_SPOT_SMOOTHING_PER_OPTION,
    INSURANCE_DEFAULT_FEE_BPS, STATUS_ACTIVE, STATUS_CANCELLED, STATUS_CLAIMED, STATUS_EXPIRED,
};

const DEFAULT_ODDS_BASE_BPS: i64 = 5000;

#[derive(Debug, Clone)]
pub enum InsuranceConfigSnapshot {
    Initialized(NewInsuranceConfig),
    Updated(NewInsuranceConfig),
    RiskPricingUpdated(NewInsuranceConfig),
}

pub(crate) fn new_insurance_config_with_defaults() -> NewInsuranceConfig {
    NewInsuranceConfig {
        updated_by: String::new(),
        insurance_enabled: false,
        min_coverage_bps: DEFAULT_MIN_COVERAGE_BPS,
        max_coverage_bps: DEFAULT_MAX_COVERAGE_BPS,
        max_duration_ms: DEFAULT_MAX_DURATION_MS,
        fee_bps: INSURANCE_DEFAULT_FEE_BPS,
        version: 0,
        updated_at: 0,
        time: Utc::now(),
        transaction_id: String::new(),
        min_spot_total_liquidity: DEFAULT_MIN_SPOT_TOTAL_LIQUIDITY,
        max_coverage_fraction_of_option_bps: DEFAULT_MAX_COVERAGE_FRACTION_OF_OPTION_BPS,
        max_risk_multiplier_bps: DEFAULT_MAX_RISK_MULTIPLIER_BPS,
        min_premium_amount: DEFAULT_MIN_PREMIUM_AMOUNT,
        spot_smoothing_per_option: DEFAULT_SPOT_SMOOTHING_PER_OPTION,
        implied_prob_floor_bps: DEFAULT_IMPLIED_PROB_FLOOR_BPS,
        odds_floor_1x: true,
        odds_cap_bps: DEFAULT_ODDS_CAP_BPS,
        liq_cap_bps: DEFAULT_LIQ_CAP_BPS,
        liq_ref_amount: DEFAULT_LIQ_REF_AMOUNT,
        exposure_cap_bps: DEFAULT_EXPOSURE_CAP_BPS,
        exposure_k_bps: DEFAULT_EXPOSURE_K_BPS,
        odds_base_bps: DEFAULT_ODDS_BASE_BPS,
    }
}

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
        "RouterConfigUpdatedEvent" => {
            process_router_config_updated_event(data, &tx, event_id, timestamp_ms_i64)
        }
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
        "RiskPricingConfigUpdatedEvent" => {
            process_risk_pricing_config_updated_event(data, &tx, event_id, timestamp_ms_i64)
        }
        "CoverageCancelledEvent" => {
            process_coverage_cancelled_event(data, &tx, event_id, timestamp_ms_i64)
        }
        "CoverageClaimedEvent" => {
            process_coverage_claimed_event(data, &tx, event_id, timestamp_ms_i64)
        }
        "PolicyExpiredEvent" => process_policy_expired_event(data, &tx, event_id, timestamp_ms_i64),
        "VaultStatusUpdatedEvent" => {
            process_vault_status_updated_event(data, &tx, event_id, timestamp_ms_i64)
        }
        "CoverageRoutedEvent" => process_coverage_routed_event(data, &tx, event_id),
        "RouteFillEvent" => process_route_fill_event(data, &tx, event_id, timestamp_ms_i64),
        "BackstopUsedEvent" => Some(vec![SocialEventRow::InsuranceEventLog(
            new_insurance_event_log("BackstopUsedEvent", data, event_id),
        )]),
        "BackstopTreasuryDepositEvent" => Some(vec![SocialEventRow::InsuranceEventLog(
            new_insurance_event_log("BackstopTreasuryDepositEvent", data, event_id),
        )]),
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

    let mut config = new_insurance_config_with_defaults();
    config.updated_by = admin;
    config.insurance_enabled = false;
    config.min_coverage_bps = min_coverage_bps;
    config.max_coverage_bps = max_coverage_bps;
    config.max_duration_ms = max_duration_ms;
    config.fee_bps = fee_bps;
    config.version = 0;
    config.updated_at = timestamp_ms;
    config.time = Utc::now();
    config.transaction_id = tx.to_string();

    Some(vec![
        SocialEventRow::InsuranceConfig(InsuranceConfigSnapshot::Initialized(config)),
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
    let insurance_enabled = data.get("insurance_enabled")?.as_bool().unwrap_or(false);
    let min_coverage_bps = json_to_i64(data.get("min_coverage_bps")?);
    let max_coverage_bps = json_to_i64(data.get("max_coverage_bps")?);
    let max_duration_ms = json_to_i64(data.get("max_duration_ms")?);
    let fee_bps = json_to_i64(data.get("fee_bps")?);
    let odds_base_bps = json_to_i64(data.get("odds_base_bps")?);
    let timestamp_ms = data
        .get("timestamp")
        .map(json_to_i64)
        .filter(|t| *t > 0)
        .unwrap_or(default_timestamp_ms);

    let mut config = new_insurance_config_with_defaults();
    config.updated_by = updated_by;
    config.insurance_enabled = insurance_enabled;
    config.min_coverage_bps = min_coverage_bps;
    config.max_coverage_bps = max_coverage_bps;
    config.max_duration_ms = max_duration_ms;
    config.fee_bps = fee_bps;
    config.odds_base_bps = odds_base_bps;
    config.version = 0;
    config.updated_at = timestamp_ms;
    config.time = Utc::now();
    config.transaction_id = tx.to_string();

    Some(vec![
        SocialEventRow::InsuranceConfig(InsuranceConfigSnapshot::Updated(config)),
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "ConfigUpdatedEvent",
            data,
            event_id,
        )),
    ])
}

fn process_router_config_updated_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    default_timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let updated_by = json_str(data.get("updated_by")?);
    let paused = data
        .get("paused")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);
    let max_route_reserve_market = json_to_i64(data.get("max_route_reserve_market")?);
    let max_route_reserve_user = json_to_i64(data.get("max_route_reserve_user")?);
    let max_route_reserve_option = json_to_i64(data.get("max_route_reserve_option")?);
    let max_vault_concentration_bps = json_to_i64(data.get("max_vault_concentration_bps")?);
    let min_vault_health_factor_bps = json_to_i64(data.get("min_vault_health_factor_bps")?);
    let max_route_legs = json_to_i64(data.get("max_route_legs")?);
    let timestamp_ms = data
        .get("timestamp")
        .map(json_to_i64)
        .filter(|t| *t > 0)
        .unwrap_or(default_timestamp_ms);

    let config = NewInsuranceRouterConfig {
        updated_by,
        paused,
        max_route_reserve_market,
        max_route_reserve_user,
        max_route_reserve_option,
        max_vault_concentration_bps,
        min_vault_health_factor_bps,
        max_route_legs,
        version: 0,
        updated_at: timestamp_ms,
        time: Utc::now(),
        transaction_id: tx.to_string(),
    };

    Some(vec![
        SocialEventRow::InsuranceRouterConfig(config),
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "RouterConfigUpdatedEvent",
            data,
            event_id,
        )),
    ])
}

fn process_risk_pricing_config_updated_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    default_timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let updated_by = json_str(data.get("updated_by")?);
    let timestamp_ms = data
        .get("timestamp")
        .map(json_to_i64)
        .filter(|t| *t > 0)
        .unwrap_or(default_timestamp_ms);

    let mut config = new_insurance_config_with_defaults();
    config.updated_by = updated_by;
    config.updated_at = timestamp_ms;
    config.time = Utc::now();
    config.transaction_id = tx.to_string();
    config.min_spot_total_liquidity = json_to_i64(data.get("min_spot_total_liquidity")?);
    config.max_coverage_fraction_of_option_bps =
        json_to_i64(data.get("max_coverage_fraction_of_option_bps")?);
    config.max_risk_multiplier_bps = json_to_i64(data.get("max_risk_multiplier_bps")?);
    config.min_premium_amount = json_to_i64(data.get("min_premium_amount")?);
    config.spot_smoothing_per_option = json_to_i64(data.get("spot_smoothing_per_option")?);
    config.implied_prob_floor_bps = json_to_i64(data.get("implied_prob_floor_bps")?);
    config.odds_floor_1x = data.get("odds_floor_1x")?.as_bool().unwrap_or(true);
    config.odds_cap_bps = json_to_i64(data.get("odds_cap_bps")?);
    config.liq_cap_bps = json_to_i64(data.get("liq_cap_bps")?);
    config.liq_ref_amount = json_to_i64(data.get("liq_ref_amount")?);
    config.exposure_cap_bps = json_to_i64(data.get("exposure_cap_bps")?);
    config.exposure_k_bps = json_to_i64(data.get("exposure_k_bps")?);

    Some(vec![
        SocialEventRow::InsuranceConfig(InsuranceConfigSnapshot::RiskPricingUpdated(config)),
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "RiskPricingConfigUpdatedEvent",
            data,
            event_id,
        )),
    ])
}

fn purchase_route_fields(data: &serde_json::Value) -> (Option<String>, Option<i16>) {
    let route_id = match data.get("route_id") {
        Some(v) if v.is_null() => None,
        Some(v) => {
            let s = json_str(v);
            if s.is_empty() {
                None
            } else {
                Some(s)
            }
        }
        None => None,
    };
    let route_leg_index = if route_id.is_none() {
        None
    } else {
        Some(
            data.get("route_leg_index")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as i16,
        )
    };
    (route_id, route_leg_index)
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
    let max_exposure_per_option = data
        .get("max_exposure_per_option")
        .map(json_to_i64)
        .unwrap_or(0);
    let enabled = data
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    let paused = data
        .get("paused")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

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
        max_exposure_per_option,
        enabled,
        paused,
        version: 0,
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
    let premium_raw = json_to_i64(data.get("premium_raw")?);
    let implied_probability_bps = json_to_i64(data.get("implied_probability_bps")?);
    let risk_multiplier_bps = json_to_i64(data.get("risk_multiplier_bps")?);
    let base_premium = json_to_i64(data.get("base_premium")?);
    let market_total_amount = json_to_i64(data.get("market_total_amount")?);
    let option_escrow_amount = json_to_i64(data.get("option_escrow_amount")?);
    let expiry_time_ms = json_to_i64(data.get("expiry_time_ms")?);
    let backstop_sweep_amount = data
        .get("backstop_sweep_amount")
        .map(json_to_i64)
        .unwrap_or(0);
    let (route_id, route_leg_index) = purchase_route_fields(data);

    let vault_id = json_str(data.get("vault_id")?);

    let now = Utc::now().naive_utc();
    let policy = NewInsurancePolicy {
        policy_id: policy_id.clone(),
        market_id: market_id.clone(),
        insured: insured.clone(),
        option_id,
        covered_amount,
        coverage_bps,
        premium_paid,
        premium_raw,
        implied_probability_bps,
        risk_multiplier_bps,
        base_premium,
        market_total_amount,
        option_escrow_amount,
        start_time_ms: timestamp_ms,
        expiry_time_ms,
        vault_id: vault_id.clone(),
        status: STATUS_ACTIVE,
        route_id,
        route_leg_index,
        backstop_sweep_amount,
        contract_version: 0,
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
        premium_raw: Some(premium_raw),
        implied_probability_bps: Some(implied_probability_bps),
        risk_multiplier_bps: Some(risk_multiplier_bps),
        base_premium: Some(base_premium),
        market_total_amount: Some(market_total_amount),
        option_escrow_amount: Some(option_escrow_amount),
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

fn process_vault_status_updated_event(
    data: &serde_json::Value,
    _tx: &str,
    event_id: &str,
    _timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let vault_id = json_str(data.get("vault_id")?);
    let max_exposure_per_option = json_to_i64(data.get("max_exposure_per_option")?);
    let enabled = data.get("enabled")?.as_bool()?;
    let paused = data.get("paused")?.as_bool()?;
    let max_exposure_per_market = json_to_i64(data.get("max_exposure_per_market")?);
    let max_exposure_per_user = json_to_i64(data.get("max_exposure_per_user")?);
    let base_rate_bps_per_day = json_to_i64(data.get("base_rate_bps_per_day")?);
    let utilization_multiplier_bps = json_to_i64(data.get("utilization_multiplier_bps")?);

    Some(vec![
        SocialEventRow::InsuranceVaultOperationalUpdate {
            vault_id,
            max_exposure_per_option,
            enabled,
            paused,
            max_exposure_per_market,
            max_exposure_per_user,
            base_rate_bps_per_day,
            utilization_multiplier_bps,
        },
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "VaultStatusUpdatedEvent",
            data,
            event_id,
        )),
    ])
}

fn process_coverage_routed_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let route_id = json_str(data.get("route_id")?);
    let insured = json_str(data.get("insured")?);
    let market_id = json_str(data.get("market_id")?);
    let option_id = data.get("option_id")?.as_u64().unwrap_or(0) as i16;
    let coverage_bps = json_to_i64(data.get("coverage_bps")?);
    let duration_ms = json_to_i64(data.get("duration_ms")?);
    let total_covered = json_to_i64(data.get("total_covered")?);
    let total_premium = json_to_i64(data.get("total_premium")?);
    let total_reserve = json_to_i64(data.get("total_reserve")?);
    let total_backstop_sweep = json_to_i64(data.get("total_backstop_sweep")?);
    let expiry_time_ms = json_to_i64(data.get("expiry_time_ms")?);
    let policy_ids = data.get("policy_ids")?.clone();
    let vault_ids = data.get("vault_ids")?.clone();

    let now = Utc::now().naive_utc();
    let row = NewInsuranceCoverageRoute {
        route_id: route_id.clone(),
        insured,
        market_id,
        option_id,
        coverage_bps,
        duration_ms,
        total_covered,
        total_premium,
        total_reserve,
        total_backstop_sweep,
        expiry_time_ms,
        policy_ids,
        vault_ids,
        contract_version: 0,
        transaction_id: tx.to_string(),
        created_at: now,
    };

    Some(vec![
        SocialEventRow::InsuranceCoverageRoute(row),
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "CoverageRoutedEvent",
            data,
            event_id,
        )),
    ])
}

fn process_route_fill_event(
    data: &serde_json::Value,
    tx: &str,
    event_id: &str,
    timestamp_ms: i64,
) -> Option<Vec<SocialEventRow>> {
    let route_id = json_str(data.get("route_id")?);
    let leg_index = data.get("leg_index")?.as_u64().unwrap_or(0) as i16;
    let vault_id = json_str(data.get("vault_id")?);
    let policy_id = json_str(data.get("policy_id")?);
    let covered_amount = json_to_i64(data.get("covered_amount")?);
    let premium_paid = json_to_i64(data.get("premium_paid")?);
    let reserve_locked = json_to_i64(data.get("reserve_locked")?);
    let backstop_sweep_amount = json_to_i64(data.get("backstop_sweep_amount")?);

    let now = Utc::now().naive_utc();
    let row = NewInsuranceRouteFill {
        route_id,
        leg_index,
        vault_id,
        policy_id,
        covered_amount,
        premium_paid,
        reserve_locked,
        backstop_sweep_amount,
        event_id: event_id.to_string(),
        transaction_id: tx.to_string(),
        timestamp_ms,
        created_at: now,
    };

    Some(vec![
        SocialEventRow::InsuranceRouteFill(row),
        SocialEventRow::InsuranceEventLog(new_insurance_event_log(
            "RouteFillEvent",
            data,
            event_id,
        )),
    ])
}
