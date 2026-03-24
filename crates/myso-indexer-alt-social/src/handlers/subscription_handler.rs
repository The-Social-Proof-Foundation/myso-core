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
    NewProfileSubscription, NewProfileSubscriptionService, NewSubscriptionEvent,
    NewSubscriptionRevenue,
};
use myso_indexer_alt_social_schema::schema::{
    profile_subscription_services, profile_subscriptions, profiles, subscription_events,
    subscription_revenue,
};

use super::common;
use super::events;
use super::subscription;

const SUBSCRIPTION_MODULES: &[&str] = &["subscription", "profile_subscription"];

#[derive(Debug, Clone)]
pub enum SubscriptionRow {
    ProfileSubscriptionService(NewProfileSubscriptionService),
    ProfileSubscription(NewProfileSubscription),
    SubscriptionEvent(NewSubscriptionEvent),
    ProfileSubscriptionServiceSubscriberIncrement { service_id: String },
    ProfileSubscriptionUpdate {
        subscription_id: String,
        expires_at: i64,
        renewal_count: i64,
    },
    ProfileSubscriptionCancel { subscription_id: String },
    ProfileSubscriptionServiceUpdate {
        service_id: String,
        monthly_fee: i64,
        updated_at: i64,
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
        transaction_id: String,
    },
}

impl SubscriptionRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::ProfileSubscriptionService(s) => {
                Some(SubscriptionRow::ProfileSubscriptionService(s))
            }
            crate::handlers::SocialEventRow::ProfileSubscription(s) => {
                Some(SubscriptionRow::ProfileSubscription(s))
            }
            crate::handlers::SocialEventRow::SubscriptionEvent(ev) => {
                Some(SubscriptionRow::SubscriptionEvent(ev))
            }
            crate::handlers::SocialEventRow::ProfileSubscriptionServiceSubscriberIncrement {
                service_id,
            } => Some(SubscriptionRow::ProfileSubscriptionServiceSubscriberIncrement {
                service_id,
            }),
            crate::handlers::SocialEventRow::ProfileSubscriptionUpdate {
                subscription_id,
                expires_at,
                renewal_count,
            } => Some(SubscriptionRow::ProfileSubscriptionUpdate {
                subscription_id,
                expires_at,
                renewal_count,
            }),
            crate::handlers::SocialEventRow::ProfileSubscriptionCancel { subscription_id } => {
                Some(SubscriptionRow::ProfileSubscriptionCancel {
                    subscription_id,
                })
            }
            crate::handlers::SocialEventRow::ProfileSubscriptionServiceUpdate {
                service_id,
                monthly_fee,
                updated_at,
            } => Some(SubscriptionRow::ProfileSubscriptionServiceUpdate {
                service_id,
                monthly_fee,
                updated_at,
            }),
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
                revenue_type,
                payment_time,
                transaction_id,
            } => Some(SubscriptionRow::SubscriptionRevenueFromCreated {
                service_id,
                subscription_id,
                from_address,
                amount,
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
                transaction_id,
            } => Some(SubscriptionRow::SubscriptionRevenueFromRenewal {
                subscription_id,
                subscriber,
                new_expires_at,
                renewal_count,
                auto_renewed,
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
                        Err(_) => continue,
                    };
                if let Some(rows) =
                    subscription::handle_subscription_event(event_name, &event_data, &event_id)
                {
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
                SubscriptionRow::ProfileSubscriptionService(s) => {
                    let profile_id = profiles::table
                        .filter(profiles::owner_address.eq(&s.profile_owner))
                        .select(profiles::profile_id)
                        .first::<Option<String>>(conn)
                        .await
                        .ok()
                        .flatten()
                        .unwrap_or_else(|| s.profile_owner.clone());
                    let service = NewProfileSubscriptionService {
                        profile_id,
                        ..s.clone()
                    };
                    total += diesel::insert_into(profile_subscription_services::table)
                        .values(&service)
                        .on_conflict(profile_subscription_services::service_id)
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
                } => {
                    let update_sql = "UPDATE profile_subscriptions SET expires_at = $1, renewal_count = $2 \
                        WHERE subscription_id = $3 AND time = (SELECT time FROM profile_subscriptions WHERE subscription_id = $3 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(expires_at)
                        .bind::<BigInt, _>(renewal_count)
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
                SubscriptionRow::ProfileSubscriptionServiceUpdate {
                    service_id,
                    monthly_fee,
                    updated_at,
                } => {
                    total += diesel::update(profile_subscription_services::table)
                        .filter(profile_subscription_services::service_id.eq(service_id))
                        .set((
                            profile_subscription_services::monthly_fee.eq(monthly_fee),
                            profile_subscription_services::updated_at.eq(Some(updated_at)),
                        ))
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
                }
                SubscriptionRow::SubscriptionRevenueFromCreated {
                    service_id,
                    subscription_id,
                    from_address,
                    amount,
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
                        let revenue = NewSubscriptionRevenue {
                            service_id: service_id.clone(),
                            subscription_id: Some(subscription_id.clone()),
                            from_address: from_address.clone(),
                            to_address,
                            amount: *amount,
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
                        let monthly_fee: Option<i64> = profile_subscription_services::table
                            .filter(profile_subscription_services::service_id.eq(&service_id))
                            .select(profile_subscription_services::monthly_fee)
                            .first(conn)
                            .await
                            .ok();
                        if let (Some(to_address), Some(amount)) = (profile_owner, monthly_fee) {
                            let revenue_type = if *auto_renewed {
                                "auto_renewal"
                            } else {
                                "renewal"
                            };
                            let payment_time = *new_expires_at - (30 * 24 * 60 * 60 * 1000);
                            let revenue = NewSubscriptionRevenue {
                                service_id,
                                subscription_id: Some(subscription_id.clone()),
                                from_address: subscriber.clone(),
                                to_address,
                                amount,
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
                        }
                    }
                }
            }
        }
        Ok(total)
    }
}
