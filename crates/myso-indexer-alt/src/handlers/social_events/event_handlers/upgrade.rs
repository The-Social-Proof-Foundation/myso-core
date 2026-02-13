// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use super::SocialEventRow;

pub fn handle_upgrade_event(
    event_name: &str,
    _data: &serde_json::Value,
    _event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "UpgradeEvent" => process_upgrade_event(),
        "ObjectMigratedEvent" => process_object_migrated_event(),
        _ => None,
    }
}

fn process_upgrade_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
fn process_object_migrated_event() -> Option<Vec<SocialEventRow>> {
    Some(vec![])
}
