// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Subscription pipeline: indexes subscription and profile_subscription module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewProfileSubscription, NewProfileSubscriptionPlan, NewProfileSubscriptionService,
    NewSubscriptionConfig, NewSubscriptionEvent, NewSubscriptionRevenue, NewUnifiedRevenue,
    REVENUE_TYPE_SUBSCRIPTION_CREATOR_AMOUNT, REVENUE_TYPE_SUBSCRIPTION_ECOSYSTEM_FEE,
    REVENUE_TYPE_SUBSCRIPTION_PLATFORM_FEE,
};
use myso_indexer_alt_social_schema::schema::{
    ecosystem_treasury, profile_subscription_plans, profile_subscription_services,
    profile_subscriptions, profiles, subscription_config, subscription_events,
    subscription_revenue, unified_revenue,
};

use super::common;
use super::events;
use super::subscription;
use super::subscription_object;
use crate::metrics::SocialMetrics;

const SUBSCRIPTION_MODULES: &[&str] = &["subscription"];

#[derive(Debug, Clone)]
pub enum SubscriptionRow {
    ProfileSubscriptionService(NewProfileSubscriptionService),
    ProfileSubscriptionPlan(NewProfileSubscriptionPlan),
    ProfileSubscription(NewProfileSubscription),
    SubscriptionEvent(NewSubscriptionEvent),
    SubscriptionConfig(NewSubscriptionConfig),
    ProfileSubscriptionServiceSubscriberIncrement {
        service_id: String,
    },
    ProfileSubscriptionServicePlanCountIncrement {
        service_id: String,
    },
    ProfileSubscriptionPlanUpdate {
        plan_id: String,
        title: String,
        description: Option<String>,
        price: i64,
        duration_ms: i64,
        tier_level: Option<i64>,
        platform_id: Option<String>,
        active: bool,
        updated_at: i64,
    },
    ProfileSubscriptionPlanDeactivate {
        plan_id: String,
        updated_at: i64,
    },
    ProfileSubscriptionUpdate {
        subscription_id: String,
        expires_at: i64,
        renewal_count: i64,
        plan_id: Option<String>,
        tier_level: Option<i64>,
        platform_id: Option<String>,
        price: Option<i64>,
        duration_ms: Option<i64>,
    },
    ProfileSubscriptionCancel {
        subscription_id: String,
    },
    ProfileSubscriptionRenewalBalanceUpdate {
        subscription_id: String,
        new_balance: i64,
    },
    ProfileSubscriptionServiceDeactivate {
        service_id: String,
        updated_at: i64,
    },
    ProfileSubscriptionServiceSubscriberDecrementBySubscription {
        subscription_id: String,
    },
    SubscriptionRevenueFromCreated {
        service_id: String,
        subscription_id: String,
        from_address: String,
        amount: i64,
        platform_fee: i64,
        ecosystem_fee: i64,
        creator_amount: i64,
        platform_address: Option<String>,
        revenue_type: String,
        payment_time: i64,
        transaction_id: String,
    },
    SubscriptionRevenueFromRefund {
        subscription_id: String,
        subscriber: String,
        refunded_amount: i64,
        transaction_id: String,
    },
    SubscriptionRevenueFromRenewal {
        subscription_id: String,
        subscriber: String,
        new_expires_at: i64,
        renewal_count: i64,
        auto_renewed: bool,
        price: i64,
        duration_ms: i64,
        platform_fee: i64,
        ecosystem_fee: i64,
        creator_amount: i64,
        platform_address: Option<String>,
        transaction_id: String,
    },
}

impl SubscriptionRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::ProfileSubscriptionService(s) => {
                Some(SubscriptionRow::ProfileSubscriptionService(s))
            }
            crate::handlers::SocialEventRow::ProfileSubscriptionPlan(p) => {
                Some(SubscriptionRow::ProfileSubscriptionPlan(p))
            }
            crate::handlers::SocialEventRow::ProfileSubscription(s) => {
                Some(SubscriptionRow::ProfileSubscription(s))
            }
            crate::handlers::SocialEventRow::SubscriptionEvent(ev) => {
                Some(SubscriptionRow::SubscriptionEvent(ev))
            }
            crate::handlers::SocialEventRow::SubscriptionConfig(c) => {
                Some(SubscriptionRow::SubscriptionConfig(c))
            }
            crate::handlers::SocialEventRow::ProfileSubscriptionServiceSubscriberIncrement {
                service_id,
            } => Some(SubscriptionRow::ProfileSubscriptionServiceSubscriberIncrement {
                service_id,
            }),
            crate::handlers::SocialEventRow::ProfileSubscriptionServicePlanCountIncrement {
                service_id,
            } => Some(SubscriptionRow::ProfileSubscriptionServicePlanCountIncrement {
                service_id,
            }),
            crate::handlers::SocialEventRow::ProfileSubscriptionPlanUpdate {
                plan_id,
                title,
                description,
                price,
                duration_ms,
                tier_level,
                platform_id,
                active,
                updated_at,
            } => Some(SubscriptionRow::ProfileSubscriptionPlanUpdate {
                plan_id,
                title,
                description,
                price,
                duration_ms,
                tier_level,
                platform_id,
                active,
                updated_at,
            }),
            crate::handlers::SocialEventRow::ProfileSubscriptionPlanDeactivate {
                plan_id,
                updated_at,
            } => Some(SubscriptionRow::ProfileSubscriptionPlanDeactivate {
                plan_id,
                updated_at,
            }),
            crate::handlers::SocialEventRow::ProfileSubscriptionUpdate {
                subscription_id,
                expires_at,
                renewal_count,
                plan_id,
                tier_level,
                platform_id,
                price,
                duration_ms,
            } => Some(SubscriptionRow::ProfileSubscriptionUpdate {
                subscription_id,
                expires_at,
                renewal_count,
                plan_id,
                tier_level,
                platform_id,
                price,
                duration_ms,
            }),
            crate::handlers::SocialEventRow::ProfileSubscriptionCancel { subscription_id } => {
                Some(SubscriptionRow::ProfileSubscriptionCancel {
                    subscription_id,
                })
            }
            crate::handlers::SocialEventRow::ProfileSubscriptionRenewalBalanceUpdate {
                subscription_id,
                new_balance,
            } => Some(SubscriptionRow::ProfileSubscriptionRenewalBalanceUpdate {
                subscription_id,
                new_balance,
            }),
            crate::handlers::SocialEventRow::ProfileSubscriptionServiceDeactivate {
                service_id,
                updated_at,
            } => Some(SubscriptionRow::ProfileSubscriptionServiceDeactivate {
                service_id,
                updated_at,
            }),
            crate::handlers::SocialEventRow::ProfileSubscriptionServiceSubscriberDecrementBySubscription {
                subscription_id,
            } => Some(
                SubscriptionRow::ProfileSubscriptionServiceSubscriberDecrementBySubscription {
                    subscription_id,
                },
            ),
            crate::handlers::SocialEventRow::SubscriptionRevenueFromCreated {
                service_id,
                subscription_id,
                from_address,
                amount,
                platform_fee,
                ecosystem_fee,
                creator_amount,
                platform_address,
                revenue_type,
                payment_time,
                transaction_id,
            } => Some(SubscriptionRow::SubscriptionRevenueFromCreated {
                service_id,
                subscription_id,
                from_address,
                amount,
                platform_fee,
                ecosystem_fee,
                creator_amount,
                platform_address,
                revenue_type,
                payment_time,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::SubscriptionRevenueFromRefund {
                subscription_id,
                subscriber,
                refunded_amount,
                transaction_id,
            } => Some(SubscriptionRow::SubscriptionRevenueFromRefund {
                subscription_id,
                subscriber,
                refunded_amount,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::SubscriptionRevenueFromRenewal {
                subscription_id,
                subscriber,
                new_expires_at,
                renewal_count,
                auto_renewed,
                price,
                duration_ms,
                platform_fee,
                ecosystem_fee,
                creator_amount,
                platform_address,
                transaction_id,
            } => Some(SubscriptionRow::SubscriptionRevenueFromRenewal {
                subscription_id,
                subscriber,
                new_expires_at,
                renewal_count,
                auto_renewed,
                price,
                duration_ms,
                platform_fee,
                ecosystem_fee,
                creator_amount,
                platform_address,
                transaction_id,
            }),
            _ => None,
        }
    }
}

impl FieldCount for SubscriptionRow {
    const FIELD_COUNT: usize = 40;
}

pub struct SubscriptionHandler;

#[async_trait]
impl Processor for SubscriptionHandler {
    const NAME: &'static str = "subscription";

    type Value = SubscriptionRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_timestamp_ms = checkpoint.summary.timestamp_ms;
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
                if !SUBSCRIPTION_MODULES.contains(&module) {
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
                                "subscription pipeline: event contents parse failed; skipping event"
                            );
                            SocialMetrics::record_event_bcs_parse_failed(module, event_name);
                            continue;
                        }
                    };
                let create_context = if event_name == "ProfileSubscriptionCreatedEvent" {
                    let service_id = event_data
                        .get("service_id")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    let subscriber = event_data
                        .get("subscriber")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default();
                    subscription_object::find_created_profile_subscription(
                        &checkpoint.object_set,
                        tx,
                        service_id,
                        subscriber,
                    )
                } else {
                    None
                };
                if let Some(rows) = subscription::handle_subscription_event(
                    event_name,
                    &event_data,
                    &event_id,
                    checkpoint_timestamp_ms,
                    create_context.as_ref(),
                ) {
                    for row in rows {
                        if let Some(r) = SubscriptionRow::from_social(row) {
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
impl Handler for SubscriptionHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                SubscriptionRow::SubscriptionConfig(c) => {
                    total += diesel::insert_into(subscription_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                SubscriptionRow::ProfileSubscriptionService(s) => {
                    total += diesel::insert_into(profile_subscription_services::table)
                        .values(s)
                        .on_conflict(profile_subscription_services::service_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    let _ = diesel::update(profiles::table)
                        .filter(profiles::owner_address.eq(&s.profile_owner))
                        .set((
                            profiles::subscription_service_id.eq(Some(s.service_id.clone())),
                            profiles::subscription_enabled.eq(true),
                        ))
                        .execute(conn)
                        .await;
                }
                SubscriptionRow::ProfileSubscriptionPlan(p) => {
                    total += diesel::insert_into(profile_subscription_plans::table)
                        .values(p)
                        .on_conflict(profile_subscription_plans::plan_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SubscriptionRow::ProfileSubscription(s) => {
                    total += diesel::insert_into(profile_subscriptions::table)
                        .values(s)
                        .on_conflict((
                            profile_subscriptions::subscription_id,
                            profile_subscriptions::time,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SubscriptionRow::SubscriptionEvent(ev) => {
                    total += diesel::insert_into(subscription_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SubscriptionRow::ProfileSubscriptionServiceSubscriberIncrement { service_id } => {
                    let _ = diesel::update(profile_subscription_services::table)
                        .filter(profile_subscription_services::service_id.eq(service_id))
                        .set(
                            profile_subscription_services::subscriber_count
                                .eq(profile_subscription_services::subscriber_count + 1),
                        )
                        .execute(conn)
                        .await;
                }
                SubscriptionRow::ProfileSubscriptionServicePlanCountIncrement { service_id } => {
                    let _ = diesel::update(profile_subscription_services::table)
                        .filter(profile_subscription_services::service_id.eq(service_id))
                        .set(
                            profile_subscription_services::plan_count
                                .eq(profile_subscription_services::plan_count + 1),
                        )
                        .execute(conn)
                        .await;
                }
                SubscriptionRow::ProfileSubscriptionPlanUpdate {
                    plan_id,
                    title,
                    description,
                    price,
                    duration_ms,
                    tier_level,
                    platform_id,
                    active,
                    updated_at,
                } => {
                    total += diesel::update(profile_subscription_plans::table)
                        .filter(profile_subscription_plans::plan_id.eq(plan_id))
                        .set((
                            profile_subscription_plans::title.eq(title),
                            profile_subscription_plans::description.eq(description),
                            profile_subscription_plans::price.eq(price),
                            profile_subscription_plans::duration_ms.eq(duration_ms),
                            profile_subscription_plans::tier_level.eq(tier_level),
                            profile_subscription_plans::platform_id.eq(platform_id),
                            profile_subscription_plans::active.eq(active),
                            profile_subscription_plans::updated_at.eq(Some(updated_at)),
                        ))
                        .execute(conn)
                        .await?;
                }
                SubscriptionRow::ProfileSubscriptionPlanDeactivate {
                    plan_id,
                    updated_at,
                } => {
                    total += diesel::update(profile_subscription_plans::table)
                        .filter(profile_subscription_plans::plan_id.eq(plan_id))
                        .set((
                            profile_subscription_plans::active.eq(false),
                            profile_subscription_plans::updated_at.eq(Some(updated_at)),
                        ))
                        .execute(conn)
                        .await?;
                }
                SubscriptionRow::ProfileSubscriptionServiceSubscriberDecrementBySubscription {
                    subscription_id,
                } => {
                    let service_id: Option<String> = profile_subscriptions::table
                        .filter(profile_subscriptions::subscription_id.eq(subscription_id))
                        .order(profile_subscriptions::time.desc())
                        .select(profile_subscriptions::service_id)
                        .first(conn)
                        .await
                        .ok();
                    if let Some(sid) = service_id {
                        let _ = diesel::update(profile_subscription_services::table)
                            .filter(profile_subscription_services::service_id.eq(&sid))
                            .set(
                                profile_subscription_services::subscriber_count
                                    .eq(profile_subscription_services::subscriber_count - 1),
                            )
                            .execute(conn)
                            .await;
                    }
                }
                SubscriptionRow::ProfileSubscriptionUpdate {
                    subscription_id,
                    expires_at,
                    renewal_count,
                    plan_id,
                    tier_level,
                    platform_id,
                    price,
                    duration_ms,
                } => {
                    let update_sql = "UPDATE profile_subscriptions SET \
                        expires_at = $1, renewal_count = $2, \
                        plan_id = COALESCE($3, plan_id), \
                        tier_level = COALESCE($4, tier_level), \
                        platform_id = COALESCE($5, platform_id), \
                        price = COALESCE($6, price), \
                        duration_ms = COALESCE($7, duration_ms) \
                        WHERE subscription_id = $8 AND time = (SELECT time FROM profile_subscriptions WHERE subscription_id = $8 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(expires_at)
                        .bind::<BigInt, _>(renewal_count)
                        .bind::<Nullable<Text>, _>(plan_id.as_deref())
                        .bind::<Nullable<BigInt>, _>(tier_level)
                        .bind::<Nullable<Text>, _>(platform_id.as_deref())
                        .bind::<Nullable<BigInt>, _>(price)
                        .bind::<Nullable<BigInt>, _>(duration_ms)
                        .bind::<Text, _>(subscription_id)
                        .execute(conn)
                        .await?;
                }
                SubscriptionRow::ProfileSubscriptionCancel { subscription_id } => {
                    let now = chrono::Utc::now().timestamp_millis();
                    let update_sql = "UPDATE profile_subscriptions SET cancelled_at = $1 \
                        WHERE subscription_id = $2 AND time = (SELECT time FROM profile_subscriptions WHERE subscription_id = $2 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<Nullable<BigInt>, _>(Some(now))
                        .bind::<Text, _>(subscription_id)
                        .execute(conn)
                        .await?;
                }
                SubscriptionRow::ProfileSubscriptionRenewalBalanceUpdate {
                    subscription_id,
                    new_balance,
                } => {
                    let update_sql = "UPDATE profile_subscriptions SET renewal_balance = $1 \
                        WHERE subscription_id = $2 AND time = (SELECT time FROM profile_subscriptions WHERE subscription_id = $2 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(new_balance)
                        .bind::<Text, _>(subscription_id)
                        .execute(conn)
                        .await?;
                }
                SubscriptionRow::ProfileSubscriptionServiceDeactivate {
                    service_id,
                    updated_at,
                } => {
                    total += diesel::update(profile_subscription_services::table)
                        .filter(profile_subscription_services::service_id.eq(service_id))
                        .set((
                            profile_subscription_services::active.eq(false),
                            profile_subscription_services::updated_at.eq(Some(updated_at)),
                        ))
                        .execute(conn)
                        .await?;
                    let profile_owner: Option<String> = profile_subscription_services::table
                        .filter(profile_subscription_services::service_id.eq(service_id))
                        .select(profile_subscription_services::profile_owner)
                        .first(conn)
                        .await
                        .ok();
                    if let Some(owner) = profile_owner {
                        let _ = diesel::update(profiles::table)
                            .filter(profiles::owner_address.eq(owner))
                            .set(profiles::subscription_enabled.eq(false))
                            .execute(conn)
                            .await;
                    }
                }
                SubscriptionRow::SubscriptionRevenueFromCreated {
                    service_id,
                    subscription_id,
                    from_address,
                    amount,
                    platform_fee,
                    ecosystem_fee,
                    creator_amount,
                    platform_address,
                    revenue_type,
                    payment_time,
                    transaction_id,
                } => {
                    let profile_owner: Option<String> = profile_subscription_services::table
                        .filter(profile_subscription_services::service_id.eq(service_id))
                        .select(profile_subscription_services::profile_owner)
                        .first(conn)
                        .await
                        .ok();
                    if let Some(to_address) = profile_owner {
                        let creator_net = if *creator_amount > 0 {
                            *creator_amount
                        } else {
                            *amount - *platform_fee - *ecosystem_fee
                        };
                        let revenue = NewSubscriptionRevenue {
                            service_id: service_id.clone(),
                            subscription_id: Some(subscription_id.clone()),
                            from_address: from_address.clone(),
                            to_address: to_address.clone(),
                            amount: *amount,
                            platform_fee: *platform_fee,
                            ecosystem_fee: *ecosystem_fee,
                            creator_amount: creator_net,
                            platform_address: platform_address.clone(),
                            revenue_type: revenue_type.clone(),
                            payment_time: *payment_time,
                            time: chrono::Utc::now(),
                            transaction_id: transaction_id.clone(),
                            processing_success: true,
                            processing_error: None,
                        };
                        total += diesel::insert_into(subscription_revenue::table)
                            .values(&revenue)
                            .execute(conn)
                            .await?;
                        total += insert_subscription_unified_revenue(
                            conn,
                            service_id,
                            from_address,
                            &to_address,
                            platform_address.as_deref(),
                            creator_net,
                            *platform_fee,
                            *ecosystem_fee,
                            revenue_type,
                            *payment_time,
                            transaction_id,
                        )
                        .await?;
                    }
                }
                SubscriptionRow::SubscriptionRevenueFromRefund {
                    subscription_id,
                    subscriber,
                    refunded_amount,
                    transaction_id,
                } => {
                    let sub_row: Option<(String, String)> = profile_subscriptions::table
                        .filter(profile_subscriptions::subscription_id.eq(subscription_id))
                        .order(profile_subscriptions::time.desc())
                        .select((
                            profile_subscriptions::service_id,
                            profile_subscriptions::subscriber,
                        ))
                        .first(conn)
                        .await
                        .ok();
                    if let Some((service_id, _)) = sub_row {
                        let profile_owner: Option<String> = profile_subscription_services::table
                            .filter(profile_subscription_services::service_id.eq(&service_id))
                            .select(profile_subscription_services::profile_owner)
                            .first(conn)
                            .await
                            .ok();
                        if let Some(profile_owner) = profile_owner {
                            let revenue = NewSubscriptionRevenue {
                                service_id,
                                subscription_id: Some(subscription_id.clone()),
                                from_address: profile_owner,
                                to_address: subscriber.clone(),
                                amount: -(*refunded_amount),
                                platform_fee: 0,
                                ecosystem_fee: 0,
                                creator_amount: -(*refunded_amount),
                                platform_address: None,
                                revenue_type: "refund".to_string(),
                                payment_time: chrono::Utc::now().timestamp_millis(),
                                time: chrono::Utc::now(),
                                transaction_id: transaction_id.clone(),
                                processing_success: true,
                                processing_error: None,
                            };
                            total += diesel::insert_into(subscription_revenue::table)
                                .values(&revenue)
                                .execute(conn)
                                .await?;
                        }
                    }
                }
                SubscriptionRow::SubscriptionRevenueFromRenewal {
                    subscription_id,
                    subscriber,
                    new_expires_at,
                    renewal_count: _,
                    auto_renewed,
                    price,
                    duration_ms,
                    platform_fee,
                    ecosystem_fee,
                    creator_amount,
                    platform_address,
                    transaction_id,
                } => {
                    let sub_row: Option<(String, i64)> = profile_subscriptions::table
                        .filter(profile_subscriptions::subscription_id.eq(subscription_id))
                        .order(profile_subscriptions::time.desc())
                        .select((
                            profile_subscriptions::service_id,
                            profile_subscriptions::renewal_balance,
                        ))
                        .first(conn)
                        .await
                        .ok();
                    if let Some((service_id, _)) = sub_row {
                        let profile_owner: Option<String> = profile_subscription_services::table
                            .filter(profile_subscription_services::service_id.eq(&service_id))
                            .select(profile_subscription_services::profile_owner)
                            .first(conn)
                            .await
                            .ok();
                        if let Some(to_address) = profile_owner {
                            let revenue_type = if *auto_renewed {
                                "auto_renewal"
                            } else {
                                "renewal"
                            };
                            let payment_time = *new_expires_at - *duration_ms;
                            let creator_net = if *creator_amount > 0 {
                                *creator_amount
                            } else {
                                *price - *platform_fee - *ecosystem_fee
                            };
                            let revenue = NewSubscriptionRevenue {
                                service_id: service_id.clone(),
                                subscription_id: Some(subscription_id.clone()),
                                from_address: subscriber.clone(),
                                to_address: to_address.clone(),
                                amount: *price,
                                platform_fee: *platform_fee,
                                ecosystem_fee: *ecosystem_fee,
                                creator_amount: creator_net,
                                platform_address: platform_address.clone(),
                                revenue_type: revenue_type.to_string(),
                                payment_time,
                                time: chrono::Utc::now(),
                                transaction_id: transaction_id.clone(),
                                processing_success: true,
                                processing_error: None,
                            };
                            total += diesel::insert_into(subscription_revenue::table)
                                .values(&revenue)
                                .execute(conn)
                                .await?;
                            total += insert_subscription_unified_revenue(
                                conn,
                                &service_id,
                                subscriber,
                                &to_address,
                                platform_address.as_deref(),
                                creator_net,
                                *platform_fee,
                                *ecosystem_fee,
                                revenue_type,
                                payment_time,
                                transaction_id,
                            )
                            .await?;
                        }
                    }
                }
            }
        }
        Ok(total)
    }
}

async fn load_ecosystem_treasury_address(conn: &mut Connection<'_>) -> Option<String> {
    ecosystem_treasury::table
        .order(ecosystem_treasury::time.desc())
        .select(ecosystem_treasury::treasury_address)
        .first(conn)
        .await
        .ok()
}

async fn insert_subscription_unified_revenue(
    conn: &mut Connection<'_>,
    service_id: &str,
    payer_address: &str,
    creator_address: &str,
    platform_address: Option<&str>,
    creator_amount: i64,
    platform_fee: i64,
    ecosystem_fee: i64,
    revenue_type: &str,
    revenue_time: i64,
    transaction_id: &str,
) -> Result<usize> {
    let mut total = 0usize;
    if creator_amount > 0 {
        total += diesel::insert_into(unified_revenue::table)
            .values(NewUnifiedRevenue::from_subscription(
                REVENUE_TYPE_SUBSCRIPTION_CREATOR_AMOUNT.to_string(),
                creator_address.to_string(),
                platform_address.map(str::to_string),
                creator_amount,
                service_id.to_string(),
                payer_address.to_string(),
                creator_address.to_string(),
                revenue_time,
                transaction_id.to_string(),
            ))
            .execute(conn)
            .await?;
    }
    if platform_fee > 0 {
        if let Some(platform) = platform_address {
            total += diesel::insert_into(unified_revenue::table)
                .values(NewUnifiedRevenue::from_subscription(
                    REVENUE_TYPE_SUBSCRIPTION_PLATFORM_FEE.to_string(),
                    creator_address.to_string(),
                    Some(platform.to_string()),
                    platform_fee,
                    service_id.to_string(),
                    payer_address.to_string(),
                    platform.to_string(),
                    revenue_time,
                    transaction_id.to_string(),
                ))
                .execute(conn)
                .await?;
        }
    }
    if ecosystem_fee > 0 {
        if let Some(treasury) = load_ecosystem_treasury_address(conn).await {
            total += diesel::insert_into(unified_revenue::table)
                .values(NewUnifiedRevenue::from_subscription(
                    REVENUE_TYPE_SUBSCRIPTION_ECOSYSTEM_FEE.to_string(),
                    creator_address.to_string(),
                    None,
                    ecosystem_fee,
                    service_id.to_string(),
                    payer_address.to_string(),
                    treasury,
                    revenue_time,
                    transaction_id.to_string(),
                ))
                .execute(conn)
                .await?;
        }
    }
    let _ = revenue_type;
    Ok(total)
}
