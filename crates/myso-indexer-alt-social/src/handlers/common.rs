// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Shared utilities for social event processing across pipelines.

use move_core_types::account_address::AccountAddress;
use myso_types::base_types::ObjectID;
use myso_types::{MYSO_MESSAGING_PACKAGE_ID, MYSO_SOCIAL_PACKAGE_ID};
use serde::de::DeserializeOwned;

/// Deserializes `data` into `T` after Stage A (`events::parse_event_contents`); on failure records
/// metrics and logs (no silent drops).
pub fn deserialize_social_event_json<T: DeserializeOwned>(
    module: &str,
    event_type: &str,
    event_id: &str,
    data: &serde_json::Value,
    warn_message: &'static str,
) -> Option<T> {
    match serde_json::from_value(data.clone()) {
        Ok(v) => Some(v),
        Err(e) => {
            crate::metrics::SocialMetrics::record_event_json_deserialize_failed(module, event_type);
            let keys: Vec<String> = data
                .as_object()
                .map(|m| m.keys().cloned().collect())
                .unwrap_or_default();
            tracing::warn!(
                event_id = %event_id,
                error = %e,
                json_keys = ?keys,
                "{}",
                warn_message
            );
            None
        }
    }
}

/// Returns true if the event belongs to the myso-social package.
pub fn is_social_package_event(package_id: &ObjectID, type_address: &AccountAddress) -> bool {
    use std::ops::Deref;
    *package_id == MYSO_SOCIAL_PACKAGE_ID || *type_address == *MYSO_SOCIAL_PACKAGE_ID.deref()
}

/// Returns true if the event belongs to the messaging package.
pub fn is_messaging_package_event(package_id: &ObjectID, type_address: &AccountAddress) -> bool {
    use std::ops::Deref;
    *package_id == MYSO_MESSAGING_PACKAGE_ID
        || *type_address == *MYSO_MESSAGING_PACKAGE_ID.deref()
}
