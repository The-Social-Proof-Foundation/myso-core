// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Resolve `ProfileSubscription` object IDs and fields from checkpoint tx effects.

use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use myso_indexer_alt_framework::types::full_checkpoint_content::{ExecutedTransaction, ObjectSet};
use myso_types::id::UID;
use myso_types::storage::ObjectKey;
use myso_types::storage::WriteKind;
use myso_types::MYSO_SOCIAL_ADDRESS;
use serde::{Deserialize, Serialize};

use super::common;

/// Context resolved from a newly created `ProfileSubscription` object in the same tx as the create event.
#[derive(Debug, Clone)]
pub struct SubscriptionCreateContext {
    pub subscription_id: String,
    pub renewal_balance: u64,
    pub created_at_ms: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct BcsBalance {
    value: u64,
}

/// BCS layout for `social_contracts::subscription::ProfileSubscription` (field order must match Move).
#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct BcsProfileSubscription {
    _id: UID,
    service_id: AccountAddress,
    plan_id: AccountAddress,
    tier_level: Option<u64>,
    platform_id: Option<AccountAddress>,
    subscriber: AccountAddress,
    created_at: u64,
    expires_at: u64,
    auto_renew: bool,
    renewal_balance: BcsBalance,
    renewal_count: u64,
}

fn addr_to_string(addr: &AccountAddress) -> String {
    format!("0x{}", hex::encode(addr))
}

fn is_profile_subscription_type(type_address: &AccountAddress, module: &str, name: &str) -> bool {
    type_address == &MYSO_SOCIAL_ADDRESS
        && module == ident_str!("subscription").as_str()
        && name == ident_str!("ProfileSubscription").as_str()
}

pub(crate) fn parse_profile_subscription_contents(
    contents: &[u8],
) -> Result<BcsProfileSubscription, bcs::Error> {
    bcs::from_bytes(contents)
}

/// Scan `tx` for newly created `ProfileSubscription` objects matching event `service_id` + `subscriber`.
pub(crate) fn find_created_profile_subscription(
    object_set: &ObjectSet,
    tx: &ExecutedTransaction,
    service_id: &str,
    subscriber: &str,
) -> Option<SubscriptionCreateContext> {
    let expected_service = common::normalize_hex_address(service_id);
    let expected_subscriber = common::normalize_hex_address(subscriber);
    let mut matches = Vec::new();

    for ((oid, version, _), _owner, write_kind) in tx.effects.all_changed_objects() {
        if !matches!(write_kind, WriteKind::Create | WriteKind::Unwrap) {
            continue;
        }
        let Some(obj) = object_set.get(&ObjectKey(oid, version)) else {
            continue;
        };
        let Some(t) = obj.type_() else {
            continue;
        };
        if !is_profile_subscription_type(&t.address(), t.module().as_str(), t.name().as_str()) {
            continue;
        }
        let Some(move_obj) = obj.as_inner().data.try_as_move() else {
            continue;
        };
        let parsed = match parse_profile_subscription_contents(move_obj.contents()) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!(
                    tx_digest = %tx.transaction.digest(),
                    object_id = %oid,
                    error = %e,
                    "subscription pipeline: failed to parse ProfileSubscription object BCS"
                );
                continue;
            }
        };

        let obj_service = common::normalize_hex_address(&addr_to_string(&parsed.service_id));
        let obj_subscriber = common::normalize_hex_address(&addr_to_string(&parsed.subscriber));
        if obj_service == expected_service && obj_subscriber == expected_subscriber {
            matches.push(SubscriptionCreateContext {
                subscription_id: oid.to_string(),
                renewal_balance: parsed.renewal_balance.value,
                created_at_ms: parsed.created_at as i64,
            });
        }
    }

    match matches.len() {
        0 => {
            tracing::warn!(
                tx_digest = %tx.transaction.digest(),
                service_id = %service_id,
                subscriber = %subscriber,
                "subscription pipeline: ProfileSubscriptionCreatedEvent with no matching created object"
            );
            None
        }
        1 => Some(matches.remove(0)),
        n => {
            tracing::warn!(
                tx_digest = %tx.transaction.digest(),
                service_id = %service_id,
                subscriber = %subscriber,
                match_count = n,
                "subscription pipeline: ambiguous ProfileSubscription create matches; skipping"
            );
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_profile_subscription_bcs_roundtrip() {
        use myso_types::base_types::ObjectID;
        let sub = BcsProfileSubscription {
            _id: UID::new(ObjectID::random()),
            service_id: AccountAddress::from_hex_literal("0xabc").unwrap(),
            plan_id: AccountAddress::from_hex_literal("0xplan").unwrap(),
            tier_level: Some(1),
            platform_id: None,
            subscriber: AccountAddress::from_hex_literal("0xdef").unwrap(),
            created_at: 1_700_000_000_000,
            expires_at: 1_700_002_592_000_000,
            auto_renew: true,
            renewal_balance: BcsBalance {
                value: 5_000_000_000,
            },
            renewal_count: 0,
        };
        let bytes = bcs::to_bytes(&sub).unwrap();
        let parsed = parse_profile_subscription_contents(&bytes).unwrap();
        assert_eq!(parsed.created_at, sub.created_at);
        assert_eq!(parsed.renewal_balance.value, 5_000_000_000);
    }
}
