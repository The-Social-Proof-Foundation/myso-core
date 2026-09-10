// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Resolve `ProfileSubscription` object IDs and fields from checkpoint tx effects.

use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use myso_indexer_alt_framework::types::full_checkpoint_content::{ExecutedTransaction, ObjectSet};
use myso_types::id::UID;
use myso_types::collection_types::Bag;
use myso_types::storage::ObjectKey;
use myso_types::storage::WriteKind;
use myso_types::MYSO_SOCIAL_ADDRESS;
use serde::{Deserialize, Serialize};

use super::common;

/// Context resolved from a newly created `ProfileSubscription` object in the same tx as the create event.
#[derive(Debug, Clone)]
pub struct SubscriptionCreateContext {
    pub renewal_balance: u64,
    pub created_at_ms: i64,
}

#[derive(Debug, Deserialize, Serialize)]
struct BcsTypeName {
    name: String,
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
    coin_type: BcsTypeName,
    renewal_balances: Bag,
    renewal_count: u64,
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

/// Look up a newly created `ProfileSubscription` by the event's object ID.
/// Enrichment only — missing or unparseable objects return None and must not drop the event.
pub(crate) fn find_created_profile_subscription(
    object_set: &ObjectSet,
    tx: &ExecutedTransaction,
    subscription_id: &str,
) -> Option<SubscriptionCreateContext> {
    let expected_id = common::normalize_hex_address(subscription_id);

    for ((oid, version, _), _owner, write_kind) in tx.effects.all_changed_objects() {
        if !matches!(write_kind, WriteKind::Create | WriteKind::Unwrap) {
            continue;
        }
        if common::normalize_hex_address(&oid.to_string()) != expected_id {
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
                return None;
            }
        };
        return Some(SubscriptionCreateContext {
            renewal_balance: changed_renewal_balance(object_set, tx, &parsed).unwrap_or(0),
            created_at_ms: parsed.created_at as i64,
        });
    }

    tracing::warn!(
        tx_digest = %tx.transaction.digest(),
        subscription_id = %subscription_id,
        "subscription pipeline: ProfileSubscriptionCreatedEvent object not found for enrichment"
    );
    None
}

/// The bag stores Field<RenewalBalanceKey<Coin>, Balance<Coin>> children.
/// Only consume the balance for the subscription's current billing coin.
fn changed_renewal_balance(
    object_set: &ObjectSet,
    tx: &ExecutedTransaction,
    subscription: &BcsProfileSubscription,
) -> Option<u64> {
    use move_core_types::language_storage::TypeTag;
    use myso_types::{base_types::ObjectID, dynamic_field::Field, object::Owner};
    for ((oid, version, _), owner, _) in tx.effects.all_changed_objects() {
        let Owner::ObjectOwner(parent) = owner else { continue };
        if ObjectID::from(parent) != *subscription.renewal_balances.id.object_id() { continue; }
        let Some(obj) = object_set.get(&ObjectKey(oid, version)) else { continue };
        let Some(t) = obj.type_() else { continue };
        let params = t.type_params();
        let Some(TypeTag::Struct(key)) = params.first().map(|p| p.as_ref()) else { continue };
        if key.address != MYSO_SOCIAL_ADDRESS || key.module.as_str() != "subscription"
            || key.name.as_str() != "RenewalBalanceKey"
            || !key.type_params.first().is_some_and(|coin| coin.to_canonical_string(false) == subscription.coin_type.name)
        { continue; }
        let Some(obj) = obj.as_inner().data.try_as_move() else { continue };
        // Move empty structs carry a single dummy bool; Balance is a single u64.
        if let Ok(field) = bcs::from_bytes::<Field<bool, u64>>(obj.contents()) {
            return Some(field.value);
        }
    }
    None
}

pub(crate) fn renewal_balance_updates_from_tx(
    object_set: &ObjectSet,
    tx: &ExecutedTransaction,
) -> Vec<(String, u64)> {
    let mut updates = Vec::new();
    for ((oid, version, _), _, _) in tx.effects.all_changed_objects() {
        let Some(obj) = object_set.get(&ObjectKey(oid, version)) else { continue };
        let Some(t) = obj.type_() else { continue };
        if !is_profile_subscription_type(&t.address(), t.module().as_str(), t.name().as_str()) { continue; }
        let Some(obj) = obj.as_inner().data.try_as_move() else { continue };
        if let Ok(subscription) = parse_profile_subscription_contents(obj.contents()) {
            if let Some(balance) = changed_renewal_balance(object_set, tx, &subscription) {
                updates.push((oid.to_string(), balance));
            }
        }
    }
    updates
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
            plan_id: AccountAddress::from_hex_literal("0x123").unwrap(),
            tier_level: Some(1),
            platform_id: None,
            subscriber: AccountAddress::from_hex_literal("0xdef").unwrap(),
            created_at: 1_700_000_000_000,
            expires_at: 1_700_002_592_000_000,
            auto_renew: true,
            coin_type: BcsTypeName { name: "2::myso::MYSO".to_string() },
            renewal_balances: Bag::default(),
            renewal_count: 0,
        };
        let bytes = bcs::to_bytes(&sub).unwrap();
        let parsed = parse_profile_subscription_contents(&bytes).unwrap();
        assert_eq!(parsed.created_at, sub.created_at);
        assert_eq!(parsed.coin_type.name, "2::myso::MYSO");
        assert_eq!(parsed.renewal_balances.size, 0);
    }
}
