// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Access-model parsing for post [`PostAccess`] and mydata [`AccessConfiguration`].

use move_core_types::account_address::AccountAddress;
use myso_types::collection_types::Table;
use serde::{Deserialize, Serialize};

/// Move `myso::object::ID` BCS layout (`bytes: address`).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BcsMoveObjectId {
    bytes: AccountAddress,
}

fn addr_to_string(addr: &AccountAddress) -> String {
    format!("0x{}", hex::encode(addr))
}

fn move_object_id_to_string(id: &BcsMoveObjectId) -> String {
    addr_to_string(&id.bytes)
}

fn optional_move_object_id_string(id: &Option<BcsMoveObjectId>) -> Option<String> {
    id.as_ref().map(move_object_id_to_string)
}

/// Move `post::PostAccess` BCS layout.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BcsPostAccess {
    Public,
    ProfileSubscription {
        service_id: BcsMoveObjectId,
        mydata_id: Option<BcsMoveObjectId>,
        min_tier_level: Option<u64>,
    },
    MarketplaceOneTime {
        mydata_id: BcsMoveObjectId,
    },
}

/// Move `mydata::AccessConfiguration` BCS layout.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum BcsAccessConfiguration {
    ProfileSubscription,
    MarketplaceOneTime {
        price: u64,
        purchasers: Table,
    },
    MarketplaceRecurring {
        price: u64,
        duration_days: u64,
        subscribers: Table,
    },
}

pub const POST_ACCESS_KIND_PUBLIC: &str = "public";
pub const POST_ACCESS_KIND_PROFILE_SUB: &str = "profile_subscription";
pub const POST_ACCESS_KIND_MARKETPLACE_ONE_TIME: &str = "marketplace_one_time";

pub const POST_ACCESS_TAG_PUBLIC: u8 = 1;
pub const POST_ACCESS_TAG_PROFILE_SUB: u8 = 2;
pub const POST_ACCESS_TAG_MARKETPLACE_ONE_TIME: u8 = 3;

pub const ACCESS_CONFIG_KIND_PROFILE: &str = "profile";
pub const ACCESS_CONFIG_KIND_ONE_TIME: &str = "one_time";
pub const ACCESS_CONFIG_KIND_RECURRING: &str = "recurring";

pub const ACCESS_CONFIG_TAG_PROFILE: u8 = 1;
pub const ACCESS_CONFIG_TAG_ONE_TIME: u8 = 2;
pub const ACCESS_CONFIG_TAG_RECURRING: u8 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PostAccessFields {
    pub post_access_kind: String,
    pub mydata_id: Option<String>,
    pub subscription_service_id: Option<String>,
    pub subscription_min_tier_level: Option<i64>,
    pub requires_subscription: Option<bool>,
}

impl PostAccessFields {
    pub fn public() -> Self {
        Self {
            post_access_kind: POST_ACCESS_KIND_PUBLIC.to_string(),
            mydata_id: None,
            subscription_service_id: None,
            subscription_min_tier_level: None,
            requires_subscription: Some(false),
        }
    }
}

pub fn post_access_kind_from_tag(tag: u8) -> Option<&'static str> {
    match tag {
        POST_ACCESS_TAG_PUBLIC => Some(POST_ACCESS_KIND_PUBLIC),
        POST_ACCESS_TAG_PROFILE_SUB => Some(POST_ACCESS_KIND_PROFILE_SUB),
        POST_ACCESS_TAG_MARKETPLACE_ONE_TIME => Some(POST_ACCESS_KIND_MARKETPLACE_ONE_TIME),
        _ => None,
    }
}

pub fn post_access_tag_from_kind(kind: &str) -> Option<u8> {
    match kind {
        POST_ACCESS_KIND_PUBLIC => Some(POST_ACCESS_TAG_PUBLIC),
        POST_ACCESS_KIND_PROFILE_SUB | "profile_sub" => Some(POST_ACCESS_TAG_PROFILE_SUB),
        POST_ACCESS_KIND_MARKETPLACE_ONE_TIME => Some(POST_ACCESS_TAG_MARKETPLACE_ONE_TIME),
        _ => None,
    }
}

pub fn mydata_access_kind_from_tag(tag: u8) -> Option<&'static str> {
    match tag {
        ACCESS_CONFIG_TAG_PROFILE => Some(ACCESS_CONFIG_KIND_PROFILE),
        ACCESS_CONFIG_TAG_ONE_TIME => Some(ACCESS_CONFIG_KIND_ONE_TIME),
        ACCESS_CONFIG_TAG_RECURRING => Some(ACCESS_CONFIG_KIND_RECURRING),
        _ => None,
    }
}

pub fn mydata_access_tag_from_kind(kind: &str) -> Option<u8> {
    match kind {
        ACCESS_CONFIG_KIND_PROFILE => Some(ACCESS_CONFIG_TAG_PROFILE),
        ACCESS_CONFIG_KIND_ONE_TIME => Some(ACCESS_CONFIG_TAG_ONE_TIME),
        ACCESS_CONFIG_KIND_RECURRING => Some(ACCESS_CONFIG_TAG_RECURRING),
        _ => None,
    }
}

pub fn post_access_fields_from_bcs(access: &BcsPostAccess) -> PostAccessFields {
    match access {
        BcsPostAccess::Public => PostAccessFields::public(),
        BcsPostAccess::ProfileSubscription {
            service_id,
            mydata_id,
            min_tier_level,
        } => PostAccessFields {
            post_access_kind: POST_ACCESS_KIND_PROFILE_SUB.to_string(),
            mydata_id: optional_move_object_id_string(mydata_id),
            subscription_service_id: Some(move_object_id_to_string(service_id)),
            subscription_min_tier_level: min_tier_level.map(|v| v as i64),
            requires_subscription: Some(true),
        },
        BcsPostAccess::MarketplaceOneTime { mydata_id } => PostAccessFields {
            post_access_kind: POST_ACCESS_KIND_MARKETPLACE_ONE_TIME.to_string(),
            mydata_id: Some(move_object_id_to_string(mydata_id)),
            subscription_service_id: None,
            subscription_min_tier_level: None,
            requires_subscription: Some(false),
        },
    }
}

pub fn post_access_json_from_bcs(access: &BcsPostAccess) -> serde_json::Value {
    let fields = post_access_fields_from_bcs(access);
    serde_json::json!({
        "post_access_kind": fields.post_access_kind,
        "post_access_tag": post_access_tag_from_kind(&fields.post_access_kind),
        "mydata_id": fields.mydata_id,
        "subscription_service_id": fields.subscription_service_id,
        "subscription_min_tier_level": fields.subscription_min_tier_level,
        "requires_subscription": fields.requires_subscription,
        "access": access_object_json(&fields),
    })
}

fn access_object_json(fields: &PostAccessFields) -> serde_json::Value {
    let mut obj = serde_json::json!({
        "kind": fields.post_access_kind,
    });
    if let Some(sid) = &fields.subscription_service_id {
        obj["service_id"] = serde_json::Value::String(sid.clone());
    }
    if let Some(mid) = &fields.mydata_id {
        obj["mydata_id"] = serde_json::Value::String(mid.clone());
    }
    if let Some(tier) = fields.subscription_min_tier_level {
        obj["min_tier_level"] = serde_json::Value::from(tier);
    }
    obj
}

pub fn post_access_fields_from_json(data: &serde_json::Value) -> PostAccessFields {
    if let Some(access) = data.get("access") {
        if let Some(parsed) = post_access_fields_from_access_object(access) {
            return parsed;
        }
    }

    if let Some(kind) = data
        .get("post_access_kind")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
    {
        return PostAccessFields {
            post_access_kind: normalize_post_access_kind(kind),
            mydata_id: json_opt_string(data, "mydata_id"),
            subscription_service_id: json_opt_string(data, "subscription_service_id")
                .or_else(|| json_opt_string(data, "service_id")),
            subscription_min_tier_level: json_opt_i64(data, "subscription_min_tier_level")
                .or_else(|| json_opt_i64(data, "min_tier_level")),
            requires_subscription: json_opt_bool(data, "requires_subscription")
                .or_else(|| {
                    Some(kind == POST_ACCESS_KIND_PROFILE_SUB || kind == "profile_sub")
                }),
        };
    }

    if let Some(tag) = json_opt_u8(data, "access_kind")
        .or_else(|| json_opt_u8(data, "post_access_kind"))
    {
        if let Some(kind) = post_access_kind_from_tag(tag) {
            if post_access_tag_from_kind(kind) == Some(tag) {
                return PostAccessFields {
                    post_access_kind: kind.to_string(),
                    mydata_id: json_opt_string(data, "mydata_id"),
                    subscription_service_id: json_opt_string(data, "subscription_service_id")
                        .or_else(|| json_opt_string(data, "service_id")),
                    subscription_min_tier_level: json_opt_i64(data, "subscription_min_tier_level")
                        .or_else(|| json_opt_i64(data, "min_tier_level")),
                    requires_subscription: Some(kind == POST_ACCESS_KIND_PROFILE_SUB),
                };
            }
        }
    }

    if let Some(mydata_id) = json_opt_string(data, "mydata_id") {
        return PostAccessFields {
            post_access_kind: POST_ACCESS_KIND_MARKETPLACE_ONE_TIME.to_string(),
            mydata_id: Some(mydata_id),
            subscription_service_id: None,
            subscription_min_tier_level: None,
            requires_subscription: Some(false),
        };
    }

    PostAccessFields::public()
}

fn post_access_fields_from_access_object(access: &serde_json::Value) -> Option<PostAccessFields> {
    let kind = access
        .get("kind")
        .and_then(|v| {
            v.as_str()
                .map(String::from)
                .or_else(|| v.as_u64().and_then(|n| post_access_kind_from_tag(n as u8).map(str::to_string)))
        })?;

    let kind = normalize_post_access_kind(&kind);

    let mydata_id = access
        .get("mydata_id")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from);
    let subscription_service_id = access
        .get("service_id")
        .or_else(|| access.get("subscription_service_id"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(String::from);

    Some(PostAccessFields {
        post_access_kind: kind.clone(),
        mydata_id,
        subscription_service_id,
        subscription_min_tier_level: access
            .get("min_tier_level")
            .or_else(|| access.get("subscription_min_tier_level"))
            .and_then(|v| v.as_i64()),
        requires_subscription: Some(kind == POST_ACCESS_KIND_PROFILE_SUB),
    })
}

fn normalize_post_access_kind(kind: &str) -> String {
    if kind == "profile_sub" {
        POST_ACCESS_KIND_PROFILE_SUB.to_string()
    } else {
        kind.to_string()
    }
}

pub fn mydata_access_kind_from_json(data: &serde_json::Value) -> Option<String> {
    if let Some(kind) = data
        .get("access_configuration_kind")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
    {
        return Some(kind.to_string());
    }
    if let Some(tag) = json_opt_u8(data, "access_configuration_kind") {
        if let Some(kind) = mydata_access_kind_from_tag(tag) {
            if mydata_access_tag_from_kind(kind) == Some(tag) {
                return Some(kind.to_string());
            }
        }
    }
    None
}

pub fn mydata_access_kind_from_bcs(access: &BcsAccessConfiguration) -> &'static str {
    match access {
        BcsAccessConfiguration::ProfileSubscription => ACCESS_CONFIG_KIND_PROFILE,
        BcsAccessConfiguration::MarketplaceOneTime { .. } => ACCESS_CONFIG_KIND_ONE_TIME,
        BcsAccessConfiguration::MarketplaceRecurring { .. } => ACCESS_CONFIG_KIND_RECURRING,
    }
}

pub fn mydata_deprecated_prices_from_bcs(
    access: &BcsAccessConfiguration,
) -> (Option<u64>, Option<u64>, u64) {
    match access {
        BcsAccessConfiguration::ProfileSubscription => (None, None, 30),
        BcsAccessConfiguration::MarketplaceOneTime { price, .. } => (Some(*price), None, 30),
        BcsAccessConfiguration::MarketplaceRecurring {
            price,
            duration_days,
            ..
        } => (None, Some(*price), *duration_days),
    }
}

fn json_opt_string(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key)
        .and_then(|v| {
            if v.is_null() {
                None
            } else {
                v.as_str().map(String::from)
            }
        })
        .filter(|s| !s.is_empty())
}

fn json_opt_bool(data: &serde_json::Value, key: &str) -> Option<bool> {
    data.get(key).and_then(|v| v.as_bool())
}

fn json_opt_u8(data: &serde_json::Value, key: &str) -> Option<u8> {
    data.get(key).and_then(|v| {
        v.as_u64()
            .and_then(|n| u8::try_from(n).ok())
            .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
    })
}

fn json_opt_i64(data: &serde_json::Value, key: &str) -> Option<i64> {
    data.get(key).and_then(|v| v.as_i64())
}

#[cfg(test)]
mod tests {
    use super::*;
    use move_core_types::account_address::AccountAddress;

    fn sample_service_id() -> BcsMoveObjectId {
        BcsMoveObjectId {
            bytes: AccountAddress::from_hex_literal("0x2").unwrap(),
        }
    }

    fn sample_mydata_id() -> BcsMoveObjectId {
        BcsMoveObjectId {
            bytes: AccountAddress::from_hex_literal("0x3").unwrap(),
        }
    }

    #[test]
    fn post_access_public_bcs_roundtrip() {
        let access = BcsPostAccess::Public;
        let bytes = bcs::to_bytes(&access).unwrap();
        let decoded: BcsPostAccess = bcs::from_bytes(&bytes).unwrap();
        let fields = post_access_fields_from_bcs(&decoded);
        assert_eq!(fields.post_access_kind, POST_ACCESS_KIND_PUBLIC);
        assert_eq!(fields.requires_subscription, Some(false));
    }

    #[test]
    fn post_access_profile_subscription_fields() {
        let access = BcsPostAccess::ProfileSubscription {
            service_id: sample_service_id(),
            mydata_id: Some(sample_mydata_id()),
            min_tier_level: Some(2),
        };
        let fields = post_access_fields_from_bcs(&access);
        assert_eq!(fields.post_access_kind, POST_ACCESS_KIND_PROFILE_SUB);
        assert_eq!(fields.requires_subscription, Some(true));
        assert!(fields.subscription_service_id.is_some());
        assert!(fields.mydata_id.is_some());
        assert_eq!(fields.subscription_min_tier_level, Some(2));
    }

    #[test]
    fn post_access_json_parses_access_object() {
        let data = serde_json::json!({
            "access": {
                "kind": "profile_sub",
                "service_id": "0xservice",
                "mydata_id": "0xmydata"
            }
        });
        let fields = post_access_fields_from_json(&data);
        assert_eq!(fields.post_access_kind, POST_ACCESS_KIND_PROFILE_SUB);
        assert_eq!(fields.subscription_service_id.as_deref(), Some("0xservice"));
        assert_eq!(fields.mydata_id.as_deref(), Some("0xmydata"));
    }

    #[test]
    fn mydata_access_kind_from_event_tag() {
        let data = serde_json::json!({ "access_configuration_kind": 2 });
        assert_eq!(
            mydata_access_kind_from_json(&data).as_deref(),
            Some(ACCESS_CONFIG_KIND_ONE_TIME)
        );
    }

    #[test]
    fn post_access_kind_tag_roundtrip() {
        assert_eq!(
            post_access_tag_from_kind(POST_ACCESS_KIND_PUBLIC),
            Some(POST_ACCESS_TAG_PUBLIC)
        );
        assert_eq!(
            post_access_kind_from_tag(POST_ACCESS_TAG_PROFILE_SUB),
            Some(POST_ACCESS_KIND_PROFILE_SUB)
        );
    }

    #[test]
    fn mydata_access_kind_tag_roundtrip() {
        assert_eq!(
            mydata_access_tag_from_kind(ACCESS_CONFIG_KIND_RECURRING),
            Some(ACCESS_CONFIG_TAG_RECURRING)
        );
        assert_eq!(
            mydata_access_kind_from_tag(ACCESS_CONFIG_TAG_ONE_TIME),
            Some(ACCESS_CONFIG_KIND_ONE_TIME)
        );
    }
}
