// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;

pub fn handle_subscription_event(
    event_name: &str,
    _data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "ProfileSubscriptionServiceCreatedEvent" => process_subscription_service_created_event(),
        "ProfileSubscriptionCreatedEvent" => process_subscription_created_event(),
        "ProfileSubscriptionRenewedEvent" => process_subscription_renewed_event(),
        "ProfileSubscriptionCancelledEvent" => process_subscription_cancelled_event(),
        "ProfileSubscriptionUpdatedEvent" => process_subscription_updated_event(),
        "RenewalBalanceFundedEvent" => process_renewal_balance_funded_event(),
        "ProfileSubscriptionServiceDeactivatedEvent" => {
            process_subscription_service_deactivated_event()
        }
        _ => None,
    }
}

fn process_subscription_service_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_subscription_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_subscription_renewed_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_subscription_cancelled_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_subscription_updated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_renewal_balance_funded_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_subscription_service_deactivated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
