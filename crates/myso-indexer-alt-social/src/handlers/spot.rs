// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewSpotBet, NewSpotBetWithdrawal, NewSpotConfig, NewSpotEventLog, NewSpotPayout, NewSpotRecord,
    NewSpotRefund, NewSpotResolution, STATUS_OPEN, STATUS_RESOLVED,
};

fn transaction_id_from_event_id(event_id: &str) -> String {
    event_id.split(':').next().unwrap_or(event_id).to_string()
}

fn json_to_i64(v: &serde_json::Value) -> i64 {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
        .unwrap_or(0)
}

fn json_opt_i64(v: &serde_json::Value) -> Option<i64> {
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| u.try_into().ok()))
}

pub fn handle_spot_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
    _epoch: u64,
    timestamp_ms: u64,
) -> Option<Vec<SocialEventRow>> {
    let transaction_id = transaction_id_from_event_id(event_id);
    let now = chrono::Utc::now();
    match event_name {
        "SpotBetPlacedEvent" | "BetPlacedEvent" => {
            process_spot_bet_placed_event(data, event_id, timestamp_ms, &transaction_id, now)
        }
        "SpotResolvedEvent" | "ResolvedEvent" => {
            process_spot_resolved_event(data, event_id, timestamp_ms, &transaction_id, now)
        }
        "SpotDaoRequiredEvent" | "DaoRequiredEvent" => {
            process_spot_dao_required_event(data, event_id, &transaction_id, now)
        }
        "SpotPayoutEvent" | "PayoutEvent" => {
            process_spot_payout_event(data, event_id, timestamp_ms, &transaction_id, now)
        }
        "SpotRefundEvent" | "RefundEvent" => {
            process_spot_refund_event(data, event_id, timestamp_ms, &transaction_id, now)
        }
        "SpotConfigUpdatedEvent" | "ConfigUpdatedEvent" => {
            process_spot_config_updated_event(data, event_id, timestamp_ms, &transaction_id, now)
        }
        "SpotRecordCreatedEvent" | "RecordCreatedEvent" => {
            process_spot_record_created_event(data, event_id, &transaction_id, now)
        }
        "SpotBetWithdrawnEvent" | "BetWithdrawnEvent" => {
            process_spot_bet_withdrawn_event(data, event_id, timestamp_ms, &transaction_id, now)
        }
        _ => None,
    }
}

fn new_event_log(
    event_type: &str,
    post_id: &str,
    event_data: &serde_json::Value,
    event_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> NewSpotEventLog {
    NewSpotEventLog {
        event_type: event_type.to_string(),
        post_id: post_id.to_string(),
        event_data: event_data.clone(),
        event_id: event_id.to_string(),
        created_at: now,
    }
}

fn process_spot_bet_placed_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let user = data.get("user")?.as_str()?.to_string();
    let option_id = data.get("option_id")?.as_u64().unwrap_or(0) as i16;
    let amount = json_to_i64(data.get("amount")?);
    let ts = data
        .get("timestamp_ms")
        .and_then(json_opt_i64)
        .unwrap_or(checkpoint_timestamp_ms as i64);

    let bet = NewSpotBet {
        post_id: post_id.clone(),
        user_address: user,
        option_id,
        escrow_amount: amount,
        amm_amount: 0,
        timestamp_ms: ts,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let log = new_event_log("SpotBetPlacedEvent", &post_id, data, event_id, now);

    Some(vec![
        SocialEventRow::SpotBet(bet),
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_resolved_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let outcome = data.get("outcome")?.as_u64().unwrap_or(0) as i16;
    let total_escrow = json_to_i64(data.get("total_escrow")?);
    let fee_taken = json_to_i64(data.get("fee_taken")?);
    let reasoning = data.get("reasoning")?.as_str().unwrap_or("").to_string();
    let evidence_urls = data
        .get("evidence_urls")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_else(|| serde_json::json!([]));

    let resolved_at_ms = checkpoint_timestamp_ms as i64;

    let resolution = NewSpotResolution {
        post_id: post_id.clone(),
        outcome,
        total_escrow,
        fee_taken,
        resolved_at_ms,
        time: now,
        transaction_id: transaction_id.to_string(),
        reasoning,
        evidence_urls,
    };

    let log = new_event_log("SpotResolvedEvent", &post_id, data, event_id, now);

    Some(vec![
        SocialEventRow::SpotResolution(resolution),
        SocialEventRow::SpotRecordUpdate {
            post_id: post_id.clone(),
            status: STATUS_RESOLVED,
            outcome: Some(outcome),
            last_resolution_at_ms: resolved_at_ms,
        },
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_dao_required_event(
    data: &serde_json::Value,
    event_id: &str,
    _transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let log = new_event_log("SpotDaoRequiredEvent", &post_id, data, event_id, now);
    Some(vec![SocialEventRow::SpotEventLog(log)])
}

fn process_spot_payout_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let user = data.get("user")?.as_str()?.to_string();
    let amount = json_to_i64(data.get("amount")?);

    let payout = NewSpotPayout {
        post_id: post_id.clone(),
        user_address: user,
        amount,
        timestamp_ms: checkpoint_timestamp_ms as i64,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let log = new_event_log("SpotPayoutEvent", &post_id, data, event_id, now);

    Some(vec![
        SocialEventRow::SpotPayout(payout),
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_refund_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let user = data.get("user")?.as_str()?.to_string();
    let amount = json_to_i64(data.get("amount")?);

    let refund = NewSpotRefund {
        post_id: post_id.clone(),
        user_address: user,
        amount,
        timestamp_ms: checkpoint_timestamp_ms as i64,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let log = new_event_log("SpotRefundEvent", &post_id, data, event_id, now);

    Some(vec![
        SocialEventRow::SpotRefund(refund),
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_config_updated_event(
    data: &serde_json::Value,
    event_id: &str,
    timestamp_ms: u64,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let updated_by = data.get("updated_by")?.as_str()?.to_string();
    let enable_flag = data.get("enable_flag")?.as_bool().unwrap_or(false);
    let confidence_threshold_bps = json_to_i64(data.get("confidence_threshold_bps")?);
    let resolution_window_ms = json_to_i64(data.get("resolution_window_ms")?);
    let max_resolution_window_ms = json_to_i64(data.get("max_resolution_window_ms")?);
    let payout_delay_ms = json_to_i64(data.get("payout_delay_ms")?);
    let fee_bps = json_to_i64(data.get("fee_bps")?);
    let fee_split_bps_platform = json_to_i64(data.get("fee_split_bps_platform")?);
    let oracle_address = data
        .get("oracle_address")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let max_single_bet = json_to_i64(data.get("max_single_bet")?);
    let version = data
        .get("version")
        .and_then(|v| json_opt_i64(v))
        .unwrap_or(0);
    let event_timestamp_ms = data
        .get("timestamp")
        .and_then(|v| json_opt_i64(v))
        .unwrap_or(timestamp_ms as i64);

    let config = NewSpotConfig {
        updated_by,
        enable_flag,
        confidence_threshold_bps,
        resolution_window_ms,
        max_resolution_window_ms,
        payout_delay_ms,
        fee_bps,
        fee_split_bps_platform,
        oracle_address,
        max_single_bet,
        version,
        timestamp_ms: event_timestamp_ms,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let log = new_event_log("SpotConfigUpdatedEvent", "", data, event_id, now);

    Some(vec![
        SocialEventRow::SpotConfig(config),
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_record_created_event(
    data: &serde_json::Value,
    event_id: &str,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let created_at_ms = json_to_i64(data.get("created_at_ms")?);
    let betting_options = data
        .get("betting_options")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_else(|| serde_json::json!([]));
    let resolution_window_ms = data.get("resolution_window_ms").and_then(json_opt_i64);
    let max_resolution_window_ms = data.get("max_resolution_window_ms").and_then(json_opt_i64);

    let now_naive = now.naive_utc();
    let record = NewSpotRecord {
        post_id: post_id.clone(),
        status: STATUS_OPEN,
        outcome: None,
        amm_split_bps_used: 0,
        betting_options: Some(betting_options),
        option_escrow: Some(serde_json::json!({})),
        resolution_window_ms,
        max_resolution_window_ms,
        created_at_ms,
        last_resolution_at_ms: None,
        version: 1,
        created_at: now_naive,
        updated_at: now_naive,
        transaction_id: transaction_id.to_string(),
    };

    let log = new_event_log("SpotRecordCreatedEvent", &post_id, data, event_id, now);

    Some(vec![
        SocialEventRow::SpotRecordUpsert(record),
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_bet_withdrawn_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let user = data.get("user")?.as_str()?.to_string();
    let option_id = data.get("option_id")?.as_u64().unwrap_or(0) as i16;
    let amount = json_to_i64(data.get("amount")?);
    let fee_taken = json_to_i64(data.get("fee_taken")?);

    let withdrawal = NewSpotBetWithdrawal {
        post_id: post_id.clone(),
        user_address: user,
        option_id,
        amount,
        fee_taken,
        timestamp_ms: checkpoint_timestamp_ms as i64,
        time: now,
        transaction_id: transaction_id.to_string(),
    };

    let log = new_event_log("SpotBetWithdrawnEvent", &post_id, data, event_id, now);

    Some(vec![
        SocialEventRow::SpotBetWithdrawal(withdrawal),
        SocialEventRow::SpotEventLog(log),
    ])
}
