// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use myso_indexer_alt_social_schema::models::{NewObjectMigratedEvent, NewUpgradeEvent};

use super::SocialEventRow;

pub fn handle_upgrade_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    match event_name {
        "UpgradeEvent" => process_upgrade_event(data, event_id),
        "ObjectMigratedEvent" => process_object_migrated_event(data, event_id),
        _ => None,
    }
}

fn process_upgrade_event(data: &serde_json::Value, event_id: &str) -> Option<Vec<SocialEventRow>> {
    let package_id = data.get("package_id")?.as_str()?.to_string();
    let version = data.get("version")?.as_u64().unwrap_or(0) as i64;
    let transaction_id = event_id.split(':').next().unwrap_or(event_id).to_string();

    tracing::info!(
        "Processed UpgradeEvent: package {} upgraded to version {} (event: {})",
        package_id,
        version,
        event_id
    );

    let ev = NewUpgradeEvent {
        package_id,
        version,
        event_id: event_id.to_string(),
        transaction_id,
        created_at: chrono::Utc::now(),
    };

    Some(vec![SocialEventRow::UpgradeEvent(ev)])
}

fn process_object_migrated_event(
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let object_id = data.get("object_id")?.as_str()?.to_string();
    let object_type = data.get("object_type")?.as_str()?.to_string();
    let old_version = data.get("old_version")?.as_u64().unwrap_or(0) as i64;
    let new_version = data.get("new_version")?.as_u64().unwrap_or(0) as i64;
    let migrated_by = data.get("migrated_by")?.as_str()?.to_string();
    let transaction_id = event_id.split(':').next().unwrap_or(event_id).to_string();

    tracing::info!(
        "Processed ObjectMigratedEvent: {} ({}) migrated from v{} to v{} by {} (event: {})",
        object_id,
        object_type,
        old_version,
        new_version,
        migrated_by,
        event_id
    );

    let ev = NewObjectMigratedEvent {
        object_id,
        object_type,
        old_version,
        new_version,
        migrated_by,
        event_id: event_id.to_string(),
        transaction_id,
        created_at: chrono::Utc::now(),
    };

    Some(vec![SocialEventRow::ObjectMigratedEvent(ev)])
}
