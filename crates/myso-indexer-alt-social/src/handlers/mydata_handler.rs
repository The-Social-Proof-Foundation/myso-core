// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! MyData pipeline: indexes `mydata` and `my_ip` module events (social_contracts query marketplace emits from `mydata`).

use std::collections::HashSet;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::pg::upsert::excluded;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewMyDataAccessLog, NewMyDataBroadPool, NewMyDataClaim, NewMyDataConfig, NewMyDataData,
    NewMyDataDistributionRound, NewMyDataListingSubPool, NewMyDataMerkleRoot, NewMyDataPurchase,
    NewMyDataRegistry, NewMyDataRevenue, NewMyDataSnapshotAnchor, NewMyDataSubPool,
    NewMyDataSubscription,
};
use myso_indexer_alt_social_schema::schema::{
    mydata_access_logs, mydata_broad_pools, mydata_claims, mydata_config, mydata_data,
    mydata_distribution_rounds, mydata_listing_sub_pools, mydata_merkle_roots, mydata_purchases,
    mydata_registry, mydata_revenue, mydata_snapshot_anchors, mydata_sub_pools,
    mydata_subscriptions,
};

use super::common;
use super::events;
use super::mydata;
use super::mydata_object;
use super::organization_stats::{
    apply_org_outbound_spend, resolve_organization_id_for_derived_address,
};
use super::post_mydata::{self, paywall_from_mydata};
use crate::metrics::SocialMetrics;

const MYDATA_MODULES: &[&str] = &["mydata", "my_ip"];
const MILLISECONDS_PER_DAY: i64 = 86_400_000;

#[derive(Debug, Clone)]
pub enum MyDataRow {
    MyDataData(NewMyDataData),
    MyDataPurchase(NewMyDataPurchase),
    MyDataSubscription(NewMyDataSubscription),
    MyDataRevenue(NewMyDataRevenue),
    MyDataAccessLog(NewMyDataAccessLog),
    MyDataRegistry(NewMyDataRegistry),
    MyDataRegistryUpdate {
        mydata_id: String,
        owner: String,
        unregistered_at: i64,
        transaction_id: String,
    },
    MyDataConfig(NewMyDataConfig),
    MyDataContentUpdate {
        mydata_id: String,
        last_updated: i64,
        transaction_id: String,
    },
    MyDataAccessRevoke {
        mydata_id: String,
        user: String,
        access_type: String,
        revoked_at: i64,
        revoked_by: String,
        transaction_id: String,
    },
    MyDataBroadPool(NewMyDataBroadPool),
    MyDataSubPool(NewMyDataSubPool),
    MyDataListingSubPoolsReplace {
        listing_id: String,
        rows: Vec<NewMyDataListingSubPool>,
    },
    MyDataSnapshotAnchor(NewMyDataSnapshotAnchor),
    MyDataDistributionRound(NewMyDataDistributionRound),
    MyDataMerkleRoot(NewMyDataMerkleRoot),
    MyDataClaim(NewMyDataClaim),
}

impl MyDataRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::MyDataData(d) => Some(MyDataRow::MyDataData(d)),
            crate::handlers::SocialEventRow::MyDataPurchase(p) => {
                Some(MyDataRow::MyDataPurchase(p))
            }
            crate::handlers::SocialEventRow::MyDataSubscription(s) => {
                Some(MyDataRow::MyDataSubscription(s))
            }
            crate::handlers::SocialEventRow::MyDataRevenue(r) => Some(MyDataRow::MyDataRevenue(r)),
            crate::handlers::SocialEventRow::MyDataAccessLog(a) => {
                Some(MyDataRow::MyDataAccessLog(a))
            }
            crate::handlers::SocialEventRow::MyDataRegistry(reg) => {
                Some(MyDataRow::MyDataRegistry(reg))
            }
            crate::handlers::SocialEventRow::MyDataRegistryUpdate {
                mydata_id,
                owner,
                unregistered_at,
                transaction_id,
            } => Some(MyDataRow::MyDataRegistryUpdate {
                mydata_id,
                owner,
                unregistered_at,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::MyDataConfig(c) => Some(MyDataRow::MyDataConfig(c)),
            crate::handlers::SocialEventRow::MyDataContentUpdate {
                mydata_id,
                last_updated,
                transaction_id,
            } => Some(MyDataRow::MyDataContentUpdate {
                mydata_id,
                last_updated,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::MyDataAccessRevoke {
                mydata_id,
                user,
                access_type,
                revoked_at,
                revoked_by,
                transaction_id,
            } => Some(MyDataRow::MyDataAccessRevoke {
                mydata_id,
                user,
                access_type,
                revoked_at,
                revoked_by,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::MyDataBroadPool(b) => {
                Some(MyDataRow::MyDataBroadPool(b))
            }
            crate::handlers::SocialEventRow::MyDataSubPool(s) => Some(MyDataRow::MyDataSubPool(s)),
            crate::handlers::SocialEventRow::MyDataListingSubPoolsReplace { listing_id, rows } => {
                Some(MyDataRow::MyDataListingSubPoolsReplace { listing_id, rows })
            }
            crate::handlers::SocialEventRow::MyDataSnapshotAnchor(a) => {
                Some(MyDataRow::MyDataSnapshotAnchor(a))
            }
            crate::handlers::SocialEventRow::MyDataDistributionRound(d) => {
                Some(MyDataRow::MyDataDistributionRound(d))
            }
            crate::handlers::SocialEventRow::MyDataMerkleRoot(m) => {
                Some(MyDataRow::MyDataMerkleRoot(m))
            }
            crate::handlers::SocialEventRow::MyDataClaim(c) => Some(MyDataRow::MyDataClaim(c)),
            _ => None,
        }
    }
}

impl FieldCount for MyDataRow {
    const FIELD_COUNT: usize = 21;
}

pub struct MyDataHandler;

#[async_trait]
impl Processor for MyDataHandler {
    const NAME: &'static str = "mydata";

    type Value = MyDataRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let mut values = mydata_object::process_mydata_objects_from_checkpoint(checkpoint);

        let mut object_mydata_ids: HashSet<String> = values
            .iter()
            .filter_map(|row| match row {
                MyDataRow::MyDataData(d) => Some(d.mydata_id.clone()),
                _ => None,
            })
            .collect();

        let mut registry_mydata_ids: HashSet<String> = values
            .iter()
            .filter_map(|row| match row {
                MyDataRow::MyDataRegistry(r) => Some(r.mydata_id.clone()),
                _ => None,
            })
            .collect();

        for tx in &checkpoint.transactions {
            let tx_digest = tx.transaction.digest().to_string();
            let Some(events) = &tx.events else {
                continue;
            };
            for (event_seq, ev) in events.data.iter().enumerate() {
                if !common::is_social_package_event(&ev.package_id, &ev.type_.address) {
                    continue;
                }
                let module = ev.type_.module.as_str();
                if !MYDATA_MODULES.contains(&module) {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data =
                    match events::parse_event_contents(module, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(e) => {
                            tracing::warn!(
                                tx_digest = %tx_digest,
                                module,
                                event_name,
                                error = %e,
                                hex_preview = %e.contents_hex_preview(48),
                                "mydata pipeline: event contents parse failed; skipping event"
                            );
                            SocialMetrics::record_event_bcs_parse_failed(module, event_name);
                            continue;
                        }
                    };
                if let Some(rows) = mydata::handle_mydata_event(event_name, &event_data, &event_id)
                {
                    for row in rows {
                        let skip = match &row {
                            crate::handlers::SocialEventRow::MyDataData(d) => {
                                object_mydata_ids.contains(&d.mydata_id)
                            }
                            crate::handlers::SocialEventRow::MyDataRegistry(r) => {
                                registry_mydata_ids.contains(&r.mydata_id)
                            }
                            _ => false,
                        };
                        if skip {
                            continue;
                        }
                        if let Some(r) = MyDataRow::from_social(row) {
                            if let MyDataRow::MyDataData(d) = &r {
                                object_mydata_ids.insert(d.mydata_id.clone());
                            }
                            if let MyDataRow::MyDataRegistry(r) = &r {
                                registry_mydata_ids.insert(r.mydata_id.clone());
                            }
                            values.push(r);
                        }
                    }
                }
            }
        }
        Ok(values)
    }
}

#[async_trait]
impl Handler for MyDataHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                MyDataRow::MyDataData(d) => {
                    let owner = d.owner.clone();
                    let media_type = d.media_type.clone();
                    let tags = d.tags.clone();
                    let platform_id = d.platform_id.clone();
                    let timestamp_start = d.timestamp_start;
                    let timestamp_end = d.timestamp_end;
                    let created_at = d.created_at;
                    let one_time_price = d.one_time_price;
                    let subscription_price = d.subscription_price;
                    let subscription_duration_days = d.subscription_duration_days;
                    let geographic_region = d.geographic_region.clone();
                    let data_quality = d.data_quality.clone();
                    let sample_size = d.sample_size;
                    let collection_method = d.collection_method.clone();
                    let is_updating = d.is_updating;
                    let update_frequency = d.update_frequency.clone();
                    let version = d.version;
                    let last_updated = d.last_updated;
                    let encrypted_content_hash = d.encrypted_content_hash.clone();
                    let transaction_id = d.transaction_id.clone();
                    let mydata_id = d.mydata_id.clone();
                    total += diesel::insert_into(mydata_data::table)
                        .values(d)
                        .on_conflict(mydata_data::mydata_id)
                        .do_update()
                        .set((
                            mydata_data::owner.eq(owner),
                            mydata_data::media_type.eq(media_type),
                            mydata_data::tags.eq(tags),
                            mydata_data::platform_id.eq(platform_id),
                            mydata_data::timestamp_start.eq(timestamp_start),
                            mydata_data::timestamp_end.eq(timestamp_end),
                            mydata_data::created_at.eq(created_at),
                            mydata_data::one_time_price.eq(one_time_price),
                            mydata_data::subscription_price.eq(subscription_price),
                            mydata_data::subscription_duration_days.eq(subscription_duration_days),
                            mydata_data::geographic_region.eq(geographic_region),
                            mydata_data::data_quality.eq(data_quality),
                            mydata_data::sample_size.eq(sample_size),
                            mydata_data::collection_method.eq(collection_method),
                            mydata_data::is_updating.eq(is_updating),
                            mydata_data::update_frequency.eq(update_frequency),
                            mydata_data::version.eq(version),
                            mydata_data::last_updated.eq(last_updated),
                            mydata_data::encrypted_content_hash.eq(encrypted_content_hash.clone()),
                            mydata_data::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                    let paywall = paywall_from_mydata(subscription_price, encrypted_content_hash);
                    total += post_mydata::sync_posts_for_mydata(conn, &mydata_id, &paywall).await?;
                }
                MyDataRow::MyDataPurchase(p) => {
                    let mut purchase = p.clone();
                    if purchase.organization_id.is_none() {
                        purchase.organization_id =
                            resolve_organization_id_for_derived_address(conn, &purchase.buyer)
                                .await?;
                    }
                    total += diesel::insert_into(mydata_purchases::table)
                        .values(&purchase)
                        .execute(conn)
                        .await?;
                    apply_org_outbound_spend(
                        conn,
                        purchase.organization_id.as_deref(),
                        purchase.price,
                        None,
                        purchase.purchase_time,
                    )
                    .await?;
                }
                MyDataRow::MyDataSubscription(s) => {
                    let duration_days = mydata_data::table
                        .filter(mydata_data::mydata_id.eq(&s.mydata_id))
                        .select(mydata_data::subscription_duration_days)
                        .first::<i64>(conn)
                        .await
                        .unwrap_or(30);
                    let subscription_end = if s.subscription_end > 0 {
                        s.subscription_end
                    } else {
                        s.subscription_start + duration_days * MILLISECONDS_PER_DAY
                    };
                    let row = NewMyDataSubscription {
                        subscription_end,
                        ..s.clone()
                    };
                    total += diesel::insert_into(mydata_subscriptions::table)
                        .values(&row)
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataRevenue(r) => {
                    let mut to_address = r.to_address.clone();
                    if to_address.is_empty() {
                        to_address = mydata_data::table
                            .filter(mydata_data::mydata_id.eq(&r.mydata_id))
                            .select(mydata_data::owner)
                            .first::<String>(conn)
                            .await
                            .unwrap_or_else(|_| "unknown".to_string());
                    }
                    let row = NewMyDataRevenue {
                        mydata_id: r.mydata_id.clone(),
                        from_address: r.from_address.clone(),
                        to_address,
                        amount: r.amount,
                        revenue_type: r.revenue_type.clone(),
                        revenue_time: r.revenue_time,
                        transaction_id: r.transaction_id.clone(),
                    };
                    total += diesel::insert_into(mydata_revenue::table)
                        .values(&row)
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataAccessLog(a) => {
                    total += diesel::insert_into(mydata_access_logs::table)
                        .values(a)
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataRegistry(reg) => {
                    let owner = reg.owner.clone();
                    let registered_at = reg.registered_at;
                    let transaction_id = reg.transaction_id.clone();
                    total += diesel::insert_into(mydata_registry::table)
                        .values(reg)
                        .on_conflict(mydata_registry::mydata_id)
                        .do_update()
                        .set((
                            mydata_registry::owner.eq(owner),
                            mydata_registry::registered_at.eq(registered_at),
                            mydata_registry::unregistered_at.eq(None::<i64>),
                            mydata_registry::is_active.eq(true),
                            mydata_registry::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataRegistryUpdate {
                    mydata_id,
                    owner,
                    unregistered_at,
                    transaction_id,
                } => {
                    total += diesel::update(mydata_registry::table)
                        .filter(mydata_registry::mydata_id.eq(mydata_id))
                        .filter(mydata_registry::owner.eq(owner))
                        .filter(mydata_registry::is_active.eq(true))
                        .set((
                            mydata_registry::unregistered_at.eq(Some(*unregistered_at)),
                            mydata_registry::is_active.eq(false),
                            mydata_registry::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataConfig(c) => {
                    total += diesel::insert_into(mydata_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataContentUpdate {
                    mydata_id,
                    last_updated,
                    transaction_id,
                } => {
                    total += diesel::update(mydata_data::table)
                        .filter(mydata_data::mydata_id.eq(mydata_id))
                        .set((
                            mydata_data::last_updated.eq(*last_updated),
                            mydata_data::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataAccessRevoke {
                    mydata_id,
                    user,
                    access_type,
                    revoked_at,
                    revoked_by,
                    transaction_id: _,
                } => {
                    if access_type == "one_time" || access_type == "all" {
                        total += diesel::update(mydata_purchases::table)
                            .filter(mydata_purchases::mydata_id.eq(mydata_id))
                            .filter(mydata_purchases::buyer.eq(user))
                            .filter(mydata_purchases::purchase_type.eq("one_time"))
                            .filter(mydata_purchases::revoked.eq(false))
                            .set((
                                mydata_purchases::revoked.eq(true),
                                mydata_purchases::revoked_at.eq(Some(*revoked_at)),
                                mydata_purchases::revoked_by.eq(Some(revoked_by.clone())),
                            ))
                            .execute(conn)
                            .await?;
                    }
                    if access_type == "subscription" || access_type == "all" {
                        total += diesel::update(mydata_subscriptions::table)
                            .filter(mydata_subscriptions::mydata_id.eq(mydata_id))
                            .filter(mydata_subscriptions::subscriber.eq(user))
                            .filter(mydata_subscriptions::revoked.eq(false))
                            .set((
                                mydata_subscriptions::revoked.eq(true),
                                mydata_subscriptions::revoked_at.eq(Some(*revoked_at)),
                                mydata_subscriptions::revoked_by.eq(Some(revoked_by.clone())),
                            ))
                            .execute(conn)
                            .await?;
                    }
                }
                MyDataRow::MyDataBroadPool(b) => {
                    total += diesel::insert_into(mydata_broad_pools::table)
                        .values(b)
                        .on_conflict(mydata_broad_pools::pool_id)
                        .do_update()
                        .set((
                            mydata_broad_pools::name.eq(excluded(mydata_broad_pools::name)),
                            mydata_broad_pools::created_at_ms
                                .eq(excluded(mydata_broad_pools::created_at_ms)),
                            mydata_broad_pools::event_id.eq(excluded(mydata_broad_pools::event_id)),
                            mydata_broad_pools::transaction_id
                                .eq(excluded(mydata_broad_pools::transaction_id)),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataSubPool(s) => {
                    total += diesel::insert_into(mydata_sub_pools::table)
                        .values(s)
                        .on_conflict(mydata_sub_pools::sub_pool_id)
                        .do_update()
                        .set((
                            mydata_sub_pools::broad_pool_id
                                .eq(excluded(mydata_sub_pools::broad_pool_id)),
                            mydata_sub_pools::name.eq(excluded(mydata_sub_pools::name)),
                            mydata_sub_pools::created_at_ms
                                .eq(excluded(mydata_sub_pools::created_at_ms)),
                            mydata_sub_pools::event_id.eq(excluded(mydata_sub_pools::event_id)),
                            mydata_sub_pools::transaction_id
                                .eq(excluded(mydata_sub_pools::transaction_id)),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataListingSubPoolsReplace { listing_id, rows } => {
                    total += diesel::delete(mydata_listing_sub_pools::table)
                        .filter(mydata_listing_sub_pools::listing_id.eq(listing_id))
                        .execute(conn)
                        .await?;
                    if !rows.is_empty() {
                        total += diesel::insert_into(mydata_listing_sub_pools::table)
                            .values(rows)
                            .execute(conn)
                            .await?;
                    }
                }
                MyDataRow::MyDataSnapshotAnchor(a) => {
                    total += diesel::insert_into(mydata_snapshot_anchors::table)
                        .values(a)
                        .on_conflict((
                            mydata_snapshot_anchors::event_id,
                            mydata_snapshot_anchors::time,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataDistributionRound(d) => {
                    total += diesel::insert_into(mydata_distribution_rounds::table)
                        .values(d)
                        .on_conflict(mydata_distribution_rounds::snapshot_id)
                        .do_update()
                        .set((
                            mydata_distribution_rounds::total_amount
                                .eq(excluded(mydata_distribution_rounds::total_amount)),
                            mydata_distribution_rounds::contributor_count
                                .eq(excluded(mydata_distribution_rounds::contributor_count)),
                            mydata_distribution_rounds::merkle_root
                                .eq(excluded(mydata_distribution_rounds::merkle_root)),
                            mydata_distribution_rounds::published_at_ms
                                .eq(excluded(mydata_distribution_rounds::published_at_ms)),
                            mydata_distribution_rounds::event_id
                                .eq(excluded(mydata_distribution_rounds::event_id)),
                            mydata_distribution_rounds::transaction_id
                                .eq(excluded(mydata_distribution_rounds::transaction_id)),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataMerkleRoot(m) => {
                    total += diesel::insert_into(mydata_merkle_roots::table)
                        .values(m)
                        .on_conflict(mydata_merkle_roots::snapshot_id)
                        .do_update()
                        .set((
                            mydata_merkle_roots::root_hash
                                .eq(excluded(mydata_merkle_roots::root_hash)),
                            mydata_merkle_roots::published_at_ms
                                .eq(excluded(mydata_merkle_roots::published_at_ms)),
                            mydata_merkle_roots::event_id
                                .eq(excluded(mydata_merkle_roots::event_id)),
                            mydata_merkle_roots::transaction_id
                                .eq(excluded(mydata_merkle_roots::transaction_id)),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataClaim(c) => {
                    total += diesel::insert_into(mydata_claims::table)
                        .values(c)
                        .on_conflict((mydata_claims::event_id, mydata_claims::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
