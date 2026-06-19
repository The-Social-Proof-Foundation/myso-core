// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Derive post paywall fields from linked MyData objects.

use std::collections::HashMap;

use anyhow::Result;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::Queryable;
use diesel::QueryDsl;
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

use super::mydata::u64_to_db_i64;
use super::mydata_object::{parse_mydata_object_contents, BcsMyData};
use crate::metrics::SocialMetrics;

/// Paywall-relevant fields extracted from a MyData object at index time.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MyDataPaywallSnapshot {
    pub subscription_price: Option<i64>,
    pub encrypted_content_hash: Option<String>,
}

/// Post paywall columns derived from MyData.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct PostPaywallFields {
    pub requires_subscription: Option<bool>,
    pub subscription_price: Option<i64>,
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
        subscription_price: mydata.subscription_price.map(u64_to_db_i64),
        encrypted_content_hash: compute_encrypted_content_hash(&mydata.encrypted_data),
    }
}

pub(crate) fn paywall_from_mydata(
    subscription_price: Option<i64>,
    encrypted_content_hash: Option<String>,
) -> PostPaywallFields {
    PostPaywallFields {
        requires_subscription: Some(subscription_price.is_some()),
        subscription_price,
        encrypted_content_hash,
    }
}

pub(crate) fn apply_paywall_fields(post: &mut NewPost, fields: &PostPaywallFields) {
    post.requires_subscription = fields.requires_subscription;
    post.subscription_price = fields.subscription_price;
    post.encrypted_content_hash = fields.encrypted_content_hash.clone();
}

pub(crate) fn enrich_post_from_snapshot(post: &mut NewPost, snapshot: &MyDataPaywallSnapshot) {
    let fields = paywall_from_mydata(
        snapshot.subscription_price,
        snapshot.encrypted_content_hash.clone(),
    );
    apply_paywall_fields(post, &fields);
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

#[derive(Debug, Queryable)]
struct MyDataPaywallDbRow {
    subscription_price: Option<i64>,
    encrypted_content_hash: Option<String>,
}

pub(crate) fn post_paywall_needs_db_fallback(post: &NewPost) -> bool {
    post.mydata_id.is_some() && post.requires_subscription.is_none()
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

    let row: Option<MyDataPaywallDbRow> = mydata_data::table
        .filter(mydata_data::mydata_id.eq(mydata_id))
        .select((
            mydata_data::subscription_price,
            mydata_data::encrypted_content_hash,
        ))
        .first(conn)
        .await
        .optional()?;

    let Some(row) = row else {
        return Ok(());
    };

    let fields = paywall_from_mydata(row.subscription_price, row.encrypted_content_hash);
    apply_paywall_fields(post, &fields);
    Ok(())
}

pub(crate) async fn sync_posts_for_mydata<'a>(
    conn: &mut Connection<'a>,
    mydata_id: &str,
    fields: &PostPaywallFields,
) -> Result<usize> {
    let updated = diesel::update(posts::table)
        .filter(posts::mydata_id.eq(mydata_id))
        .set((
            posts::requires_subscription.eq(fields.requires_subscription),
            posts::subscription_price.eq(fields.subscription_price),
            posts::encrypted_content_hash.eq(fields.encrypted_content_hash.clone()),
        ))
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
    fn paywall_from_mydata_subscription_only() {
        let fields = paywall_from_mydata(Some(500), Some("0xabc".to_string()));
        assert_eq!(fields.requires_subscription, Some(true));
        assert_eq!(fields.subscription_price, Some(500));
        assert_eq!(fields.encrypted_content_hash.as_deref(), Some("0xabc"));
    }

    #[test]
    fn paywall_from_mydata_one_time_only() {
        let fields = paywall_from_mydata(None, Some("0xdef".to_string()));
        assert_eq!(fields.requires_subscription, Some(false));
        assert_eq!(fields.subscription_price, None);
        assert_eq!(fields.encrypted_content_hash.as_deref(), Some("0xdef"));
    }

    #[test]
    fn paywall_from_mydata_free_encrypted() {
        let fields = paywall_from_mydata(None, None);
        assert_eq!(fields.requires_subscription, Some(false));
        assert_eq!(fields.subscription_price, None);
        assert_eq!(fields.encrypted_content_hash, None);
    }
}
