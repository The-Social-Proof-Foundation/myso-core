// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Derive post paywall fields from linked MyData objects.

use std::collections::HashMap;

use anyhow::Result;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use fastcrypto::hash::{Blake2b256, HashFunction};
use move_core_types::account_address::AccountAddress;
use move_core_types::ident_str;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_social_schema::models::NewPost;
use myso_indexer_alt_social_schema::schema::{mydata_data, posts};
use myso_types::storage::ObjectKey;
use myso_types::MYSO_SOCIAL_ADDRESS;

use super::access::POST_ACCESS_KIND_PROFILE_SUB;
use super::mydata_object::{parse_mydata_object_contents, BcsMyData};
use crate::metrics::SocialMetrics;

/// Paywall-relevant fields extracted from a MyData object at index time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MyDataPaywallSnapshot {
    pub encrypted_content_hash: Option<String>,
}

/// blake2b256 digest of ciphertext, `0x`-prefixed hex (matches Move `myso::hash::blake2b256`).
pub(crate) fn compute_encrypted_content_hash(data: &[u8]) -> Option<String> {
    if data.is_empty() {
        return None;
    }
    let digest = Blake2b256::digest(data);
    Some(format!("0x{}", hex::encode(digest.digest)))
}

pub(crate) fn paywall_snapshot_from_bcs(mydata: &BcsMyData) -> MyDataPaywallSnapshot {
    MyDataPaywallSnapshot {
        encrypted_content_hash: compute_encrypted_content_hash(&mydata.encrypted_data),
    }
}

/// A linked MyData object supplies encrypted bytes only. Post entitlement and pricing are
/// determined by `PostAccess`: profile subscriptions come from `subscription.move`, while
/// one-time prices remain on the linked MyData record.
fn enrich_post_from_snapshot(post: &mut NewPost, snapshot: &MyDataPaywallSnapshot) {
    post.encrypted_content_hash = snapshot.encrypted_content_hash.clone();
}

pub(crate) fn enrich_post_from_mydata_id(
    post: &mut NewPost,
    mydata_id: &str,
    snapshots: &HashMap<String, MyDataPaywallSnapshot>,
) {
    if let Some(snapshot) = snapshots.get(mydata_id) {
        enrich_post_from_snapshot(post, snapshot);
    }
}

fn is_mydata_object_type(type_address: &AccountAddress, module: &str, name: &str) -> bool {
    type_address == &MYSO_SOCIAL_ADDRESS
        && module == ident_str!("mydata").as_str()
        && name == ident_str!("MyData").as_str()
}

/// Last MyData object write per id in the checkpoint wins.
pub(crate) fn build_checkpoint_mydata_snapshots(
    checkpoint: &Checkpoint,
) -> HashMap<String, MyDataPaywallSnapshot> {
    let mut snapshots = HashMap::new();
    for tx in &checkpoint.transactions {
        let tx_digest = tx.transaction.digest().to_string();
        for ((oid, version, _), _owner, _write_kind) in tx.effects.all_changed_objects() {
            let Some(obj) = checkpoint.object_set.get(&ObjectKey(oid, version)) else {
                continue;
            };
            let Some(t) = obj.type_() else {
                continue;
            };
            if !is_mydata_object_type(&t.address(), t.module().as_str(), t.name().as_str()) {
                continue;
            }
            let Some(move_obj) = obj.as_inner().data.try_as_move() else {
                continue;
            };
            let parsed = match parse_mydata_object_contents(move_obj.contents()) {
                Ok(v) => v,
                Err(e) => {
                    tracing::warn!(
                        tx_digest = %tx_digest,
                        object_id = %oid,
                        error = %e,
                        "post_mydata: failed to parse MyData object BCS for paywall snapshot"
                    );
                    SocialMetrics::record_event_bcs_parse_failed("mydata", "MyDataObject");
                    continue;
                }
            };
            snapshots.insert(oid.to_string(), paywall_snapshot_from_bcs(&parsed));
        }
    }
    snapshots
}

#[derive(Debug, QueryableByName)]
struct MinPlanPriceRow {
    #[diesel(sql_type = diesel::sql_types::Nullable<diesel::sql_types::BigInt>)]
    min_price: Option<i64>,
}

pub(crate) fn post_paywall_needs_db_fallback(post: &NewPost) -> bool {
    post.mydata_id.is_some() && post.encrypted_content_hash.is_none()
}

pub(crate) async fn enrich_post_paywall_from_db<'a>(
    post: &mut NewPost,
    conn: &mut Connection<'a>,
) -> Result<()> {
    let Some(mydata_id) = post.mydata_id.as_deref() else {
        return Ok(());
    };
    if !post_paywall_needs_db_fallback(post) {
        return Ok(());
    }

    let encrypted_content_hash: Option<Option<String>> = mydata_data::table
        .filter(mydata_data::mydata_id.eq(mydata_id))
        .select(mydata_data::encrypted_content_hash)
        .first(conn)
        .await
        .optional()?;

    if let Some(hash) = encrypted_content_hash {
        post.encrypted_content_hash = hash;
    }
    Ok(())
}

pub(crate) async fn enrich_post_subscription_price_from_db<'a>(
    post: &mut NewPost,
    conn: &mut Connection<'a>,
) -> Result<()> {
    if post.subscription_price.is_some() {
        return Ok(());
    }
    if post.post_access_kind.as_deref() != Some(POST_ACCESS_KIND_PROFILE_SUB) {
        return Ok(());
    }
    let Some(service_id) = post.subscription_service_id.as_deref() else {
        return Ok(());
    };
    let min_price: Option<i64> = diesel::sql_query(
        "SELECT MIN(price) AS min_price FROM profile_subscription_plans \
         WHERE service_id = $1 \
           AND active = true \
           AND COALESCE(tier_level, 0) >= COALESCE($2, 0) \
           AND (platform_id IS NULL OR platform_id = $3)",
    )
    .bind::<diesel::sql_types::Text, _>(service_id)
    .bind::<diesel::sql_types::Nullable<diesel::sql_types::BigInt>, _>(
        post.subscription_min_tier_level,
    )
    .bind::<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>(post.platform_id.as_deref())
    .get_result::<MinPlanPriceRow>(conn)
    .await
    .optional()?
    .and_then(|row| row.min_price);
    post.subscription_price = min_price;
    Ok(())
}

/// Recompute the cached price for every profile-subscription post using a service.
/// This runs after plan create/update/deactivate events so the compatibility field
/// cannot retain an old plan price.
pub(crate) async fn refresh_post_subscription_prices_for_service<'a>(
    conn: &mut Connection<'a>,
    service_id: &str,
) -> Result<usize> {
    let updated = diesel::sql_query(
        "UPDATE posts AS p \
         SET subscription_price = ( \
             SELECT MIN(plan.price) \
             FROM profile_subscription_plans AS plan \
             WHERE plan.service_id = p.subscription_service_id \
               AND plan.active = true \
               AND COALESCE(plan.tier_level, 0) >= COALESCE(p.subscription_min_tier_level, 0) \
               AND (plan.platform_id IS NULL OR plan.platform_id = p.platform_id) \
         ) \
         WHERE p.subscription_service_id = $1 \
           AND p.post_access_kind IN ('2', 'profile_sub', 'profile_subscription')",
    )
    .bind::<diesel::sql_types::Text, _>(service_id)
    .execute(conn)
    .await?;
    Ok(updated)
}

pub(crate) async fn sync_posts_for_mydata<'a>(
    conn: &mut Connection<'a>,
    mydata_id: &str,
    encrypted_content_hash: Option<String>,
) -> Result<usize> {
    let updated = diesel::update(posts::table)
        .filter(posts::mydata_id.eq(mydata_id))
        .set(posts::encrypted_content_hash.eq(encrypted_content_hash))
        .execute(conn)
        .await?;
    Ok(updated)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_encrypted_content_hash_empty_is_none() {
        assert_eq!(compute_encrypted_content_hash(&[]), None);
    }

    #[test]
    fn compute_encrypted_content_hash_non_empty_is_prefixed_hex() {
        let hash = compute_encrypted_content_hash(&[1, 2, 3]).expect("hash");
        assert!(hash.starts_with("0x"));
        assert_eq!(hash.len(), 2 + 64);
    }

    #[test]
    fn mydata_snapshot_contains_only_encrypted_content_hash() {
        let snapshot = MyDataPaywallSnapshot {
            encrypted_content_hash: Some("0xabc".to_string()),
        };
        assert_eq!(snapshot.encrypted_content_hash.as_deref(), Some("0xabc"));
    }
}
