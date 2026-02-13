// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;

pub fn handle_mydata_event(
    event_name: &str,
    _data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "MyDataRegisteredEvent" | "IPRegisteredEvent" => process_mydata_registered_event(),
        "MyDataUnregisteredEvent" | "IPUnregisteredEvent" => process_mydata_unregistered_event(),
        "MyDataCreatedEvent" | "DataCreatedEvent" => process_mydata_created_event(),
        "PurchaseEvent" | "DataPurchasedEvent" => process_mydata_purchase_event(),
        "AccessGrantedEvent" | "DataAccessGrantedEvent" => process_mydata_access_granted_event(),
        "MyDataConfigUpdatedEvent" | "ConfigUpdatedEvent" => process_mydata_config_updated_event(),
        _ => None,
    }
}

fn process_mydata_registered_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_mydata_unregistered_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_mydata_created_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_mydata_purchase_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_mydata_access_granted_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_mydata_config_updated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
