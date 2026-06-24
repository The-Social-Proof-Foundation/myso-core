// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Shared utilities for social event processing across pipelines.

use chrono::{DateTime, TimeZone, Utc};
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

/// Resolves chain-sourced milliseconds: prefer positive on-chain event ms, else checkpoint ms.
pub fn chain_timestamp_ms(event_ms: Option<i64>, checkpoint_timestamp_ms: u64) -> i64 {
    if let Some(ms) = event_ms.filter(|&ms| ms > 0) {
        return ms;
    }
    if checkpoint_timestamp_ms > 0 {
        checkpoint_timestamp_ms as i64
    } else {
        0
    }
}

pub fn chain_time_from_ms(ms: i64) -> DateTime<Utc> {
    Utc.timestamp_millis_opt(ms).single().unwrap_or_else(Utc::now)
}

pub fn json_field_as_i64(v: Option<&serde_json::Value>) -> Option<i64> {
    v.and_then(|val| {
        val.as_i64()
            .or_else(|| val.as_u64().and_then(|u| u.try_into().ok()))
            .or_else(|| val.as_str().and_then(|s| s.parse().ok()))
    })
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
