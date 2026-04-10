// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! MyData pipeline: indexes `mydata` and `my_ip` module events (social_contracts query marketplace emits from `mydata`).

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::pg::upsert::excluded;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewMyDataAccessLog, NewMyDataConfig, NewMyDataData, NewMyDataPurchase, NewMyDataQueryBroadPool,
    NewMyDataQueryClaim, NewMyDataQueryDistributionRound, NewMyDataQueryListingSubPool,
    NewMyDataQueryMerkleRoot, NewMyDataQuerySnapshotAnchor, NewMyDataQuerySubPool,
    NewMyDataRegistry, NewMyDataRevenue,
    NewMyDataSubscription,
};
use myso_indexer_alt_social_schema::schema::{
    mydata_access_logs, mydata_config, mydata_data, mydata_purchases, mydata_query_broad_pools,
    mydata_query_claims, mydata_query_distribution_rounds, mydata_query_listing_sub_pools,
    mydata_query_merkle_roots, mydata_query_snapshot_anchors, mydata_query_sub_pools,
    mydata_registry, mydata_revenue,
    mydata_subscriptions,
};

use super::common;
use super::events;
use super::mydata;

const MYDATA_MODULES: &[&str] = &["mydata", "my_ip"];

#[derive(Debug, Clone)]
pub enum MyDataRow {
    MyDataData(NewMyDataData),
    MyDataPurchase(NewMyDataPurchase),
    MyDataSubscription(NewMyDataSubscription),
    MyDataRevenue(NewMyDataRevenue),
    MyDataAccessLog(NewMyDataAccessLog),
    MyDataRegistry(NewMyDataRegistry),
    MyDataRegistryUpdate {
        ip_id: String,
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
    MyDataQueryBroadPool(NewMyDataQueryBroadPool),
    MyDataQuerySubPool(NewMyDataQuerySubPool),
    MyDataQueryListingSubPoolsReplace {
        listing_id: String,
        rows: Vec<NewMyDataQueryListingSubPool>,
    },
    MyDataQuerySnapshotAnchor(NewMyDataQuerySnapshotAnchor),
    MyDataQueryDistributionRound(NewMyDataQueryDistributionRound),
    MyDataQueryMerkleRoot(NewMyDataQueryMerkleRoot),
    MyDataQueryClaim(NewMyDataQueryClaim),
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
                ip_id,
                owner,
                unregistered_at,
                transaction_id,
            } => Some(MyDataRow::MyDataRegistryUpdate {
                ip_id,
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
            crate::handlers::SocialEventRow::MyDataQueryBroadPool(b) => {
                Some(MyDataRow::MyDataQueryBroadPool(b))
            }
            crate::handlers::SocialEventRow::MyDataQuerySubPool(s) => {
                Some(MyDataRow::MyDataQuerySubPool(s))
            }
            crate::handlers::SocialEventRow::MyDataQueryListingSubPoolsReplace {
                listing_id,
                rows,
            } => Some(MyDataRow::MyDataQueryListingSubPoolsReplace { listing_id, rows }),
            crate::handlers::SocialEventRow::MyDataQuerySnapshotAnchor(a) => {
                Some(MyDataRow::MyDataQuerySnapshotAnchor(a))
            }
            crate::handlers::SocialEventRow::MyDataQueryDistributionRound(d) => {
                Some(MyDataRow::MyDataQueryDistributionRound(d))
            }
            crate::handlers::SocialEventRow::MyDataQueryMerkleRoot(m) => {
                Some(MyDataRow::MyDataQueryMerkleRoot(m))
            }
            crate::handlers::SocialEventRow::MyDataQueryClaim(c) => {
                Some(MyDataRow::MyDataQueryClaim(c))
            }
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
        let mut values = Vec::new();
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
                        Err(_) => continue,
                    };
                if let Some(rows) = mydata::handle_mydata_event(event_name, &event_data, &event_id)
                {
                    for row in rows {
                        if let Some(r) = MyDataRow::from_social(row) {
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
                    let one_time_price = d.one_time_price;
                    let subscription_price = d.subscription_price;
                    let last_updated = d.last_updated;
                    let transaction_id = d.transaction_id.clone();
                    total += diesel::insert_into(mydata_data::table)
                        .values(d)
                        .on_conflict(mydata_data::mydata_id)
                        .do_update()
                        .set((
                            mydata_data::owner.eq(owner),
                            mydata_data::media_type.eq(media_type),
                            mydata_data::tags.eq(tags),
                            mydata_data::platform_id.eq(platform_id),
                            mydata_data::one_time_price.eq(one_time_price),
                            mydata_data::subscription_price.eq(subscription_price),
                            mydata_data::last_updated.eq(last_updated),
                            mydata_data::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataPurchase(p) => {
                    total += diesel::insert_into(mydata_purchases::table)
                        .values(p)
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataSubscription(s) => {
                    total += diesel::insert_into(mydata_subscriptions::table)
                        .values(s)
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
                        .on_conflict(mydata_registry::ip_id)
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
                    ip_id,
                    owner,
                    unregistered_at,
                    transaction_id,
                } => {
                    total += diesel::update(mydata_registry::table)
                        .filter(mydata_registry::ip_id.eq(ip_id))
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
                MyDataRow::MyDataQueryBroadPool(b) => {
                    total += diesel::insert_into(mydata_query_broad_pools::table)
                        .values(b)
                        .on_conflict(mydata_query_broad_pools::pool_id)
                        .do_update()
                        .set((
                            mydata_query_broad_pools::name.eq(excluded(mydata_query_broad_pools::name)),
                            mydata_query_broad_pools::created_at_ms
                                .eq(excluded(mydata_query_broad_pools::created_at_ms)),
                            mydata_query_broad_pools::event_id
                                .eq(excluded(mydata_query_broad_pools::event_id)),
                            mydata_query_broad_pools::transaction_id
                                .eq(excluded(mydata_query_broad_pools::transaction_id)),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataQuerySubPool(s) => {
                    total += diesel::insert_into(mydata_query_sub_pools::table)
                        .values(s)
                        .on_conflict(mydata_query_sub_pools::sub_pool_id)
                        .do_update()
                        .set((
                            mydata_query_sub_pools::broad_pool_id
                                .eq(excluded(mydata_query_sub_pools::broad_pool_id)),
                            mydata_query_sub_pools::name.eq(excluded(mydata_query_sub_pools::name)),
                            mydata_query_sub_pools::created_at_ms
                                .eq(excluded(mydata_query_sub_pools::created_at_ms)),
                            mydata_query_sub_pools::event_id
                                .eq(excluded(mydata_query_sub_pools::event_id)),
                            mydata_query_sub_pools::transaction_id
                                .eq(excluded(mydata_query_sub_pools::transaction_id)),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataQueryListingSubPoolsReplace { listing_id, rows } => {
                    total += diesel::delete(mydata_query_listing_sub_pools::table)
                        .filter(mydata_query_listing_sub_pools::listing_id.eq(listing_id))
                        .execute(conn)
                        .await?;
                    if !rows.is_empty() {
                        total += diesel::insert_into(mydata_query_listing_sub_pools::table)
                            .values(rows)
                            .execute(conn)
                            .await?;
                    }
                }
                MyDataRow::MyDataQuerySnapshotAnchor(a) => {
                    total += diesel::insert_into(mydata_query_snapshot_anchors::table)
                        .values(a)
                        .on_conflict((
                            mydata_query_snapshot_anchors::event_id,
                            mydata_query_snapshot_anchors::time,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataQueryDistributionRound(d) => {
                    total += diesel::insert_into(mydata_query_distribution_rounds::table)
                        .values(d)
                        .on_conflict(mydata_query_distribution_rounds::snapshot_id)
                        .do_update()
                        .set((
                            mydata_query_distribution_rounds::total_amount
                                .eq(excluded(mydata_query_distribution_rounds::total_amount)),
                            mydata_query_distribution_rounds::contributor_count.eq(excluded(
                                mydata_query_distribution_rounds::contributor_count,
                            )),
                            mydata_query_distribution_rounds::merkle_root
                                .eq(excluded(mydata_query_distribution_rounds::merkle_root)),
                            mydata_query_distribution_rounds::published_at_ms.eq(excluded(
                                mydata_query_distribution_rounds::published_at_ms,
                            )),
                            mydata_query_distribution_rounds::event_id
                                .eq(excluded(mydata_query_distribution_rounds::event_id)),
                            mydata_query_distribution_rounds::transaction_id.eq(excluded(
                                mydata_query_distribution_rounds::transaction_id,
                            )),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataQueryMerkleRoot(m) => {
                    total += diesel::insert_into(mydata_query_merkle_roots::table)
                        .values(m)
                        .on_conflict(mydata_query_merkle_roots::snapshot_id)
                        .do_update()
                        .set((
                            mydata_query_merkle_roots::root_hash
                                .eq(excluded(mydata_query_merkle_roots::root_hash)),
                            mydata_query_merkle_roots::published_at_ms
                                .eq(excluded(mydata_query_merkle_roots::published_at_ms)),
                            mydata_query_merkle_roots::event_id
                                .eq(excluded(mydata_query_merkle_roots::event_id)),
                            mydata_query_merkle_roots::transaction_id
                                .eq(excluded(mydata_query_merkle_roots::transaction_id)),
                        ))
                        .execute(conn)
                        .await?;
                }
                MyDataRow::MyDataQueryClaim(c) => {
                    total += diesel::insert_into(mydata_query_claims::table)
                        .values(c)
                        .on_conflict((
                            mydata_query_claims::event_id,
                            mydata_query_claims::time,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
