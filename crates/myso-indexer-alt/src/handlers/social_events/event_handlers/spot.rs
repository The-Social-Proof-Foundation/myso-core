// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;

pub fn handle_spot_event(
    event_name: &str,
    _data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "SpotBetPlacedEvent" | "BetPlacedEvent" => process_spot_bet_placed_event(),
        "SpotResolvedEvent" | "ResolvedEvent" => process_spot_resolved_event(),
        "SpotDaoRequiredEvent" | "DaoRequiredEvent" => process_spot_dao_required_event(),
        "SpotPayoutEvent" | "PayoutEvent" => process_spot_payout_event(),
        "SpotRefundEvent" | "RefundEvent" => process_spot_refund_event(),
        "SpotConfigUpdatedEvent" | "ConfigUpdatedEvent" => process_spot_config_updated_event(),
        "SpotRecordCreatedEvent" | "RecordCreatedEvent" => process_spot_record_created_event(),
        "SpotBetWithdrawnEvent" | "BetWithdrawnEvent" => process_spot_bet_withdrawn_event(),
        _ => None,
    }
}

fn process_spot_bet_placed_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_spot_resolved_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_spot_dao_required_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_spot_payout_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_spot_refund_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_spot_config_updated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_spot_record_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_spot_bet_withdrawn_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
