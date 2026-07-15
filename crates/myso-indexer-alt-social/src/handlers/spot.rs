// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::{SocialEventRow, SpotFinalizeProjection};
use myso_indexer_alt_social_schema::models::{
    CREATOR_PAYOUT_STATUS_ACCRUED, CREATOR_PAYOUT_STATUS_CLAIMED, CREATOR_PAYOUT_STATUS_RECLAIMED,
    NewSpotBet, NewSpotBetWithdrawal, NewSpotClaim, NewSpotClaimVerdict, NewSpotConfig,
    NewSpotCreatorPayout, NewSpotEventLog, NewSpotMarket, NewSpotPayout, NewSpotPostLink,
    NewSpotRecord, NewSpotRefund, NewSpotResolution, SPOT_LINK_KIND_PRIMARY, STATUS_DAO_REQUIRED,
    STATUS_OPEN, STATUS_RESOLVED,
};

/// A related-market address of all zeros (0x0) encodes "no related market".
fn non_zero_addr(s: &str) -> Option<String> {
    let hex = s.trim_start_matches("0x");
    if hex.chars().all(|c| c == '0') {
        None
    } else {
        Some(s.to_string())
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
        "SpotGovernanceProposalLinkedEvent" => {
            process_spot_governance_proposal_linked_event(data, event_id, &transaction_id, now)
        }
        "SpotGovernanceProposalClearedEvent" => {
            process_spot_governance_proposal_cleared_event(data, event_id, &transaction_id, now)
        }
        "SpotClaimCreatedEvent" => {
            process_spot_claim_created_event(data, event_id, &transaction_id, now)
        }
        "SpotMarketCreatedEvent" => {
            process_spot_market_created_event(data, event_id, &transaction_id, now)
        }
        "SpotPostLinkedEvent" => {
            process_spot_post_linked_event(data, event_id, &transaction_id, now)
        }
        "SpotClaimsFinalizedForPost" => {
            process_spot_claims_finalized_event(data, event_id, now)
        }
        "SpotCreatorPayoutAccruedEvent" => {
            process_spot_creator_payout_accrued_event(data, event_id, timestamp_ms, &transaction_id, now)
        }
        "SpotCreatorPayoutClaimedEvent" => {
            process_spot_creator_payout_claimed_event(data, event_id, timestamp_ms, &transaction_id, now)
        }
        "SpotCreatorPayoutReclaimedEvent" => {
            process_spot_creator_payout_reclaimed_event(data, event_id, timestamp_ms, &transaction_id, now)
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
    let market_object_id = data
        .get("market_id")
        .and_then(|v| v.as_str())
        .map(String::from);
    let referrer_post_id = data
        .get("referrer_post_id")
        .and_then(|v| v.as_str())
        .map(String::from);
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
        organization_id: None,
        market_object_id,
        referrer_post_id,
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
    let claim_object_id = data
        .get("claim_id")
        .and_then(|v| v.as_str())
        .map(String::from);
    let market_object_id = data
        .get("market_id")
        .and_then(|v| v.as_str())
        .map(String::from);
    let outcome = data.get("outcome")?.as_u64().unwrap_or(0) as i16;
    let total_escrow = json_to_i64(data.get("total_escrow")?);
    let fee_taken = json_to_i64(data.get("fee_taken")?);
    let creator_fee_total = data.get("creator_fee_total").and_then(json_opt_i64);
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
        claim_object_id: claim_object_id.clone(),
        market_object_id: market_object_id.clone(),
        creator_fee_total,
    };

    let log = new_event_log("SpotResolvedEvent", &post_id, data, event_id, now);

    let mut rows = vec![
        SocialEventRow::SpotResolution(resolution),
        SocialEventRow::SpotRecordUpdate {
            post_id: post_id.clone(),
            status: STATUS_RESOLVED,
            outcome: Some(outcome),
            last_resolution_at_ms: resolved_at_ms,
            claim_object_id,
            market_object_id: market_object_id.clone(),
            creator_fee_total,
        },
        SocialEventRow::SpotEventLog(log),
    ];
    if let Some(market_id) = market_object_id {
        rows.push(SocialEventRow::SpotMarketUpdate {
            market_object_id: market_id,
            status: STATUS_RESOLVED,
            outcome: Some(outcome),
            last_resolution_at_ms: Some(resolved_at_ms),
            resolution_timestamp_ms: Some(resolved_at_ms),
            creator_fee_total,
        });
    }
    Some(rows)
}

fn process_spot_dao_required_event(
    data: &serde_json::Value,
    event_id: &str,
    _transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let log = new_event_log("SpotDaoRequiredEvent", &post_id, data, event_id, now);
    Some(vec![
        SocialEventRow::SpotRecordGovernanceUpdate {
            spot_record_id: data
                .get("spot_record_id")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            post_id: post_id.clone(),
            active_proposal_id: None,
            oracle_proposed_outcome: data
                .get("oracle_proposed_outcome")
                .and_then(|v| v.as_u64())
                .map(|v| v as i16),
            proposed_outcome: None,
            dao_escalated_at_ms: data.get("dao_escalated_at_ms").and_then(json_opt_i64),
            status: Some(STATUS_DAO_REQUIRED),
        },
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_governance_proposal_linked_event(
    data: &serde_json::Value,
    event_id: &str,
    _transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let spot_record_id = data.get("spot_record_id")?.as_str()?.to_string();
    let proposal_id = data.get("proposal_id")?.as_str()?.to_string();
    let proposed_outcome = data.get("proposed_outcome")?.as_u64().map(|v| v as i16);
    let log = new_event_log(
        "SpotGovernanceProposalLinkedEvent",
        &post_id,
        data,
        event_id,
        now,
    );
    Some(vec![
        SocialEventRow::SpotRecordGovernanceUpdate {
            spot_record_id,
            post_id: post_id.clone(),
            active_proposal_id: Some(proposal_id),
            oracle_proposed_outcome: None,
            proposed_outcome,
            dao_escalated_at_ms: None,
            status: Some(STATUS_DAO_REQUIRED),
        },
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_governance_proposal_cleared_event(
    data: &serde_json::Value,
    event_id: &str,
    _transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let spot_record_id = data.get("spot_record_id")?.as_str()?.to_string();
    let log = new_event_log(
        "SpotGovernanceProposalClearedEvent",
        &post_id,
        data,
        event_id,
        now,
    );
    Some(vec![
        SocialEventRow::SpotRecordGovernanceUpdate {
            spot_record_id,
            post_id: post_id.clone(),
            active_proposal_id: None,
            oracle_proposed_outcome: None,
            proposed_outcome: None,
            dao_escalated_at_ms: None,
            status: None,
        },
        SocialEventRow::SpotEventLog(log),
    ])
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
    let truth_enabled = data.get("truth_enabled")?.as_bool().unwrap_or(false);
    let confidence_threshold_bps = json_to_i64(data.get("confidence_threshold_bps")?);
    let resolution_window_ms = json_to_i64(data.get("resolution_window_ms")?);
    let max_resolution_window_ms = json_to_i64(data.get("max_resolution_window_ms")?);
    let payout_delay_ms = json_to_i64(data.get("payout_delay_ms")?);
    let platform_fee_bps = json_to_i64(data.get("platform_fee_bps")?);
    let ecosystem_fee_bps = json_to_i64(data.get("ecosystem_fee_bps")?);
    let creator_fee_bps = data.get("creator_fee_bps").and_then(json_opt_i64);
    let creator_claim_window_ms = data.get("creator_claim_window_ms").and_then(json_opt_i64);
    let expired_creator_ecosystem_bps = data
        .get("expired_creator_ecosystem_bps")
        .and_then(json_opt_i64);
    let min_betting_options = json_to_i64(data.get("min_betting_options")?);
    let max_betting_options = json_to_i64(data.get("max_betting_options")?);
    let min_reasoning_length = json_to_i64(data.get("min_reasoning_length")?);
    let max_reasoning_length = json_to_i64(data.get("max_reasoning_length")?);
    let max_evidence_urls = json_to_i64(data.get("max_evidence_urls")?);
    let oracle_address = data
        .get("oracle_address")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let max_single_bet = json_to_i64(data.get("max_single_bet")?);
    let max_bets_per_record = data
        .get("max_bets_per_record")
        .and_then(json_opt_i64)
        .unwrap_or(10_000);
    let max_claim_per_post = data
        .get("max_claim_per_post")
        .and_then(json_opt_i64)
        .unwrap_or(10);
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
        truth_enabled,
        confidence_threshold_bps,
        resolution_window_ms,
        max_resolution_window_ms,
        payout_delay_ms,
        platform_fee_bps,
        ecosystem_fee_bps,
        creator_fee_bps,
        creator_claim_window_ms,
        expired_creator_ecosystem_bps,
        min_betting_options,
        max_betting_options,
        min_reasoning_length,
        max_reasoning_length,
        max_evidence_urls,
        oracle_address,
        max_single_bet,
        max_bets_per_record,
        max_claim_per_post,
        version,
        updated_at: event_timestamp_ms,
        time: now,
        transaction_id: transaction_id.to_string(),
        spot_governance_registry_id: data
            .get("spot_governance_registry_id")
            .and_then(|v| v.as_str())
            .map(String::from),
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
    let record_object_id = data
        .get("record_id")
        .and_then(|v| v.as_str())
        .map(String::from);
    let created_at_ms = json_to_i64(data.get("created_at_ms")?);
    let betting_options = data
        .get("betting_options")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_else(|| serde_json::json!([]));
    let resolution_window_ms = data.get("resolution_window_ms").and_then(json_opt_i64);
    let max_resolution_window_ms = data.get("max_resolution_window_ms").and_then(json_opt_i64);
    let resolution_at_ms = data.get("resolution_at_ms").and_then(json_opt_i64);

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
        resolution_at_ms,
        created_at_ms,
        last_resolution_at_ms: None,
        version: 1,
        created_at: now_naive,
        updated_at: now_naive,
        transaction_id: transaction_id.to_string(),
        record_object_id,
        active_proposal_id: None,
        oracle_proposed_outcome: None,
        proposed_outcome: None,
        dao_escalated_at_ms: None,
        claim_object_id: None,
        market_object_id: None,
        primary_post_id: Some(post_id.clone()),
        market_key_hash: None,
        creator_fee_total: None,
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

fn process_spot_claim_created_event(
    data: &serde_json::Value,
    event_id: &str,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let claim_object_id = data.get("claim_id")?.as_str()?.to_string();
    let semantic_claim_hash = data.get("semantic_claim_hash")?.as_str()?.to_string();
    let created_at_ms = json_to_i64(data.get("created_at_ms")?);

    let claim = NewSpotClaim {
        claim_object_id: claim_object_id.clone(),
        semantic_claim_hash,
        created_at_ms,
        transaction_id: transaction_id.to_string(),
        created_at: now.naive_utc(),
    };

    let log = new_event_log("SpotClaimCreatedEvent", "", data, event_id, now);

    Some(vec![
        SocialEventRow::SpotClaimUpsert(claim),
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_market_created_event(
    data: &serde_json::Value,
    event_id: &str,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let market_object_id = data.get("market_id")?.as_str()?.to_string();
    let claim_object_id = data.get("claim_id")?.as_str()?.to_string();
    let market_key_hash = data.get("market_key_hash")?.as_str()?.to_string();
    let primary_post_id = data.get("primary_post_id")?.as_str()?.to_string();
    let created_at_ms = json_to_i64(data.get("created_at_ms")?);
    let betting_options = data
        .get("betting_options")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_else(|| serde_json::json!([]));
    let resolution_at_ms = data.get("resolution_at_ms").and_then(json_opt_i64);
    let max_resolution_window_ms = data.get("max_resolution_window_ms").and_then(json_opt_i64);
    let now_naive = now.naive_utc();

    let market = NewSpotMarket {
        market_object_id: market_object_id.clone(),
        claim_object_id: claim_object_id.clone(),
        market_key_hash: market_key_hash.clone(),
        primary_post_id: primary_post_id.clone(),
        primary_creator: None,
        status: STATUS_OPEN,
        outcome: None,
        betting_options: betting_options.clone(),
        option_escrow: serde_json::json!({}),
        resolution_window_ms: None,
        max_resolution_window_ms,
        resolution_at_ms,
        created_at_ms,
        last_resolution_at_ms: None,
        resolution_timestamp_ms: None,
        creator_fee_total: None,
        transaction_id: transaction_id.to_string(),
        created_at: now_naive,
        updated_at: now_naive,
    };

    let record = NewSpotRecord {
        post_id: primary_post_id.clone(),
        status: STATUS_OPEN,
        outcome: None,
        amm_split_bps_used: 0,
        betting_options: Some(betting_options),
        option_escrow: Some(serde_json::json!({})),
        resolution_window_ms: None,
        max_resolution_window_ms,
        resolution_at_ms,
        created_at_ms,
        last_resolution_at_ms: None,
        version: 1,
        created_at: now_naive,
        updated_at: now_naive,
        transaction_id: transaction_id.to_string(),
        record_object_id: Some(market_object_id.clone()),
        active_proposal_id: None,
        oracle_proposed_outcome: None,
        proposed_outcome: None,
        dao_escalated_at_ms: None,
        claim_object_id: Some(claim_object_id.clone()),
        market_object_id: Some(market_object_id.clone()),
        primary_post_id: Some(primary_post_id.clone()),
        market_key_hash: Some(market_key_hash),
        creator_fee_total: None,
    };

    let link = NewSpotPostLink {
        post_id: primary_post_id.clone(),
        claim_object_id: claim_object_id.clone(),
        market_object_id: Some(market_object_id.clone()),
        link_kind: SPOT_LINK_KIND_PRIMARY.to_string(),
        transaction_id: transaction_id.to_string(),
        created_at: now_naive,
        claim_index: data.get("claim_index").and_then(|v| v.as_i64()).unwrap_or(0),
        policy_hash: data
            .get("resolution_policy_hash")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
    };

    let log = new_event_log(
        "SpotMarketCreatedEvent",
        &primary_post_id,
        data,
        event_id,
        now,
    );

    Some(vec![
        SocialEventRow::SpotMarketUpsert(market),
        SocialEventRow::SpotRecordUpsert(record),
        SocialEventRow::SpotPostLinkUpsert(link),
        SocialEventRow::SpotEventLog(log),
    ])
}

/// Atomic multi-claim finalize projection: rewrite the `posts` analysis denorm + `spot_post_analyses`
/// sidecar, upsert past `spot_claim_verdicts`, and append the forensic event log.
fn process_spot_claims_finalized_event(
    data: &serde_json::Value,
    event_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let transaction_id = transaction_id_from_event_id(event_id);
    let geti = |k: &str| data.get(k).map(json_to_i64).unwrap_or(0);
    let gets = |k: &str| data.get(k).and_then(|v| v.as_str()).map(String::from);
    let arr = |k: &str| data.get(k).cloned().unwrap_or_else(|| serde_json::json!([]));

    let projection = SpotFinalizeProjection {
        post_id: post_id.clone(),
        status: geti("status") as i16,
        detected_claim_count: geti("detected_claim_count"),
        rejected_claim_count: geti("rejected_claim_count"),
        truncated_claim_count: geti("truncated_claim_count"),
        future_accepted_count: geti("future_accepted_count"),
        past_verified_count: geti("past_verified_count"),
        max_claim_per_post_applied: geti("max_claim_per_post_applied"),
        claim_indexes: arr("future_claim_indexes"),
        claim_ids: arr("future_claim_ids"),
        market_ids: arr("future_market_ids"),
        claim_manifest_hash: gets("claim_manifest_hash"),
        veracity_manifest_hash: gets("veracity_manifest_hash"),
        finalize_tx_digest: Some(transaction_id.clone()),
        updated_at: now,
    };

    let mut rows = vec![SocialEventRow::SpotFinalize(Box::new(projection))];

    // Past verdicts: parallel arrays (claim_index, verdict, related_market, evidence_hash).
    let p_indexes = data
        .get("past_claim_indexes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let p_verdicts = data
        .get("past_verdicts")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let p_markets = data
        .get("past_related_market_ids")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    let p_evidence = data
        .get("past_evidence_hashes")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();
    for i in 0..p_indexes.len() {
        rows.push(SocialEventRow::SpotClaimVerdictUpsert(NewSpotClaimVerdict {
            post_id: post_id.clone(),
            claim_index: p_indexes[i].as_i64().unwrap_or(0),
            time_class: "past".to_string(),
            verdict: p_verdicts.get(i).and_then(|v| v.as_i64()).unwrap_or(0) as i16,
            semantic_claim_hash: None,
            policy_hash: String::new(),
            evidence_manifest_hash: p_evidence
                .get(i)
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string(),
            related_market_object_id: p_markets
                .get(i)
                .and_then(|v| v.as_str())
                .and_then(non_zero_addr),
            related_claim_object_id: None,
            evidence_urls: serde_json::json!([]),
            summary: None,
            transaction_id: transaction_id.clone(),
            created_at: now,
        }));
    }

    let log = new_event_log("SpotClaimsFinalizedForPost", &post_id, data, event_id, now);
    rows.push(SocialEventRow::SpotEventLog(log));
    Some(rows)
}

fn process_spot_post_linked_event(
    data: &serde_json::Value,
    event_id: &str,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let post_id = data.get("post_id")?.as_str()?.to_string();
    let claim_object_id = data.get("claim_id")?.as_str()?.to_string();
    let market_object_id = data
        .get("market_id")
        .and_then(|v| v.as_str())
        .map(String::from);

    let link = NewSpotPostLink {
        post_id: post_id.clone(),
        claim_object_id: claim_object_id.clone(),
        market_object_id: market_object_id.clone(),
        link_kind: "linked".to_string(),
        transaction_id: transaction_id.to_string(),
        created_at: now.naive_utc(),
        claim_index: data.get("claim_index").and_then(|v| v.as_i64()).unwrap_or(0),
        policy_hash: data
            .get("policy_hash")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
    };

    let log = new_event_log("SpotPostLinkedEvent", &post_id, data, event_id, now);

    Some(vec![
        SocialEventRow::SpotPostLinkUpsert(link),
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_creator_payout_accrued_event(
    data: &serde_json::Value,
    event_id: &str,
    _checkpoint_timestamp_ms: u64,
    transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let market_object_id = data.get("market_id")?.as_str()?.to_string();
    let payout_id = json_to_i64(data.get("payout_id")?);
    let creator_address = data.get("creator")?.as_str()?.to_string();
    let referrer_post_id = data.get("referrer_post_id")?.as_str()?.to_string();
    let amount = json_to_i64(data.get("amount")?);
    let expires_at_ms = json_to_i64(data.get("expires_at_ms")?);
    let now_naive = now.naive_utc();

    let payout = NewSpotCreatorPayout {
        market_object_id: market_object_id.clone(),
        payout_id,
        creator_address,
        referrer_post_id: referrer_post_id.clone(),
        amount,
        expires_at_ms,
        status: CREATOR_PAYOUT_STATUS_ACCRUED.to_string(),
        ecosystem_amount: None,
        platform_amount: None,
        claimed_at_ms: None,
        reclaimed_at_ms: None,
        transaction_id: transaction_id.to_string(),
        created_at: now_naive,
        updated_at: now_naive,
    };

    let log = new_event_log(
        "SpotCreatorPayoutAccruedEvent",
        &referrer_post_id,
        data,
        event_id,
        now,
    );

    Some(vec![
        SocialEventRow::SpotCreatorPayoutUpsert(payout),
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_creator_payout_claimed_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    _transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let market_object_id = data.get("market_id")?.as_str()?.to_string();
    let payout_id = json_to_i64(data.get("payout_id")?);
    let creator_address = data.get("creator")?.as_str()?.to_string();
    let amount = json_to_i64(data.get("amount")?);
    let day = chrono::DateTime::from_timestamp_millis(checkpoint_timestamp_ms as i64)
        .map(|dt| dt.date_naive())
        .unwrap_or_else(|| now.date_naive());
    let log = new_event_log(
        "SpotCreatorPayoutClaimedEvent",
        &market_object_id,
        data,
        event_id,
        now,
    );

    Some(vec![
        SocialEventRow::SpotCreatorPayoutStatusUpdate {
            market_object_id: market_object_id.clone(),
            payout_id,
            status: CREATOR_PAYOUT_STATUS_CLAIMED.to_string(),
            claimed_at_ms: Some(checkpoint_timestamp_ms as i64),
            reclaimed_at_ms: None,
            ecosystem_amount: None,
            platform_amount: None,
        },
        SocialEventRow::SpotCreatorEarningsDailyUpsert {
            creator_address,
            day,
            amount,
        },
        SocialEventRow::SpotEventLog(log),
    ])
}

fn process_spot_creator_payout_reclaimed_event(
    data: &serde_json::Value,
    event_id: &str,
    checkpoint_timestamp_ms: u64,
    _transaction_id: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> Option<Vec<SocialEventRow>> {
    let market_object_id = data.get("market_id")?.as_str()?.to_string();
    let payout_id = json_to_i64(data.get("payout_id")?);
    let ecosystem_amount = json_to_i64(data.get("ecosystem_amount")?);
    let platform_amount = json_to_i64(data.get("platform_amount")?);
    let log = new_event_log(
        "SpotCreatorPayoutReclaimedEvent",
        &market_object_id,
        data,
        event_id,
        now,
    );

    Some(vec![
        SocialEventRow::SpotCreatorPayoutStatusUpdate {
            market_object_id,
            payout_id,
            status: CREATOR_PAYOUT_STATUS_RECLAIMED.to_string(),
            claimed_at_ms: None,
            reclaimed_at_ms: Some(checkpoint_timestamp_ms as i64),
            ecosystem_amount: Some(ecosystem_amount),
            platform_amount: Some(platform_amount),
        },
        SocialEventRow::SpotEventLog(log),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::handlers::SocialEventRow;

    /// SpotConfig fee fields map directly from the Move event platform/ecosystem bps.
    #[test]
    fn spot_config_fee_breakout_maps_to_row() {
        let data = serde_json::json!({
            "updated_by": "0x9cc886f94db2b2a41b1f8d7c20c7fc0960e1f9eb34ce2c0c7f309",
            "truth_enabled": true,
            "confidence_threshold_bps": 6500,
            "resolution_window_ms": 86400000,
            "max_resolution_window_ms": 604800000,
            "payout_delay_ms": 12000,
            "platform_fee_bps": 50,
            "ecosystem_fee_bps": 50,
            "min_betting_options": 2,
            "max_betting_options": 10,
            "min_reasoning_length": 10,
            "max_reasoning_length": 5000,
            "max_evidence_urls": 10,
            "oracle_address": "0x2f41b4f43f505d427e8777c511461de8e50eac26558a996627dded27dce50918",
            "max_single_bet": 1000000000,
            "max_bets_per_record": 100,
            "max_claim_per_post": 10,
            "spot_governance_registry_id": "0xabcdef",
            "timestamp": 1700000000,
        });
        let rows = handle_spot_event("SpotConfigUpdatedEvent", &data, "tx:0", 0, 0)
            .expect("handler should produce rows");
        let cfg = rows
            .iter()
            .find_map(|r| match r {
                SocialEventRow::SpotConfig(cfg) => Some(cfg),
                _ => None,
            })
            .expect("SpotConfig row should be emitted");
        assert_eq!(cfg.platform_fee_bps, 50);
        assert_eq!(cfg.ecosystem_fee_bps, 50);
        assert_eq!(cfg.min_betting_options, 2);
        assert_eq!(cfg.max_betting_options, 10);
        assert_eq!(cfg.max_reasoning_length, 5000);
        assert_eq!(cfg.max_evidence_urls, 10);
        assert_eq!(cfg.max_bets_per_record, 100);
        assert_eq!(cfg.max_claim_per_post, 10);
        assert_eq!(
            cfg.spot_governance_registry_id.as_deref(),
            Some("0xabcdef")
        );
    }
}
