// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;

pub fn handle_insurance_event(
    event_name: &str,
    _data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "ConfigInitializedEvent" => process_config_initialized_event(),
        "ConfigUpdatedEvent" => process_config_updated_event(),
        "UnderwriterVaultCreatedEvent" => process_vault_created_event(),
        "UnderwriterVaultDepositedEvent" => process_vault_deposited_event(),
        "UnderwriterVaultWithdrawnEvent" => process_vault_withdrawn_event(),
        "CoveragePurchasedEvent" => process_coverage_purchased_event(),
        "CoverageCancelledEvent" => process_coverage_cancelled_event(),
        "CoverageClaimedEvent" => process_coverage_claimed_event(),
        "PolicyExpiredEvent" => process_policy_expired_event(),
        _ => None,
    }
}

fn process_config_initialized_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_config_updated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_vault_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_vault_deposited_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_vault_withdrawn_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_coverage_purchased_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_coverage_cancelled_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_coverage_claimed_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_policy_expired_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
