// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;

pub fn handle_spt_event(
    event_name: &str,
    _data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "TokenPoolCreatedEvent" | "PoolCreatedEvent" => process_token_pool_created_event(),
        "TokenBoughtEvent" | "BuyEvent" => process_token_bought_event(),
        "TokenSoldEvent" | "SellEvent" => process_token_sold_event(),
        "ReservationPoolCreatedEvent" => process_reservation_pool_created_event(),
        "ReservationCreatedEvent" => process_reservation_created_event(),
        "ReservationWithdrawnEvent" => process_reservation_withdrawn_event(),
        "ThresholdMetEvent" => process_threshold_met_event(),
        "ConfigUpdatedEvent" => process_spt_config_updated_event(),
        "EmergencyKillSwitchEvent" => process_emergency_kill_switch_event(),
        "SocialProofInitPoolEvent" | "InitPoolEvent" => process_social_proof_init_pool_event(),
        "SocialProofBuyEvent" => process_social_proof_buy_event(),
        "SocialProofSellEvent" => process_social_proof_sell_event(),
        "TokensAddedEvent" => process_tokens_added_event(),
        "PocRedirectionUpdatedEvent" => process_poc_redirection_updated_event(),
        _ => None,
    }
}

fn process_token_pool_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_token_bought_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_token_sold_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_reservation_pool_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_reservation_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_reservation_withdrawn_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_threshold_met_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_spt_config_updated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_emergency_kill_switch_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_social_proof_init_pool_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_social_proof_buy_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_social_proof_sell_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_tokens_added_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_poc_redirection_updated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
