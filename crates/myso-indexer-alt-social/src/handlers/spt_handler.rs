// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! SPT pipeline: indexes social_proof_tokens / spt module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Nullable, Text};
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewSptExchangeConfig, NewSptHolding, NewSptPool, NewSptPriceHistory, NewSptReservation,
    NewSptReservationPool, NewSptRevenue, NewSptTransaction, NewSocialProofTokensConfig,
    NewSocialProofTokensEvent, NewUnifiedRevenue, ProfileUpdateSet,
    RESERVATION_POOL_STATUS_ACTIVE, RESERVATION_POOL_STATUS_THRESHOLD_MET,
    REVENUE_TYPE_SPT_CREATOR_FEE, REVENUE_TYPE_SPT_PLATFORM_FEE, REVENUE_TYPE_SPT_TREASURY_FEE,
    TOKEN_TYPE_POST,
};
use myso_indexer_alt_social_schema::schema::{
    ecosystem_treasury, posts, profiles, social_proof_tokens_config, social_proof_tokens_events,
    spt_exchange_config, spt_holdings, spt_pools, spt_price_history, spt_reservation_pools,
    spt_reservations, spt_revenue, spt_transactions, unified_revenue,
};

use super::common;
use super::events;
use super::spt;
use super::ProfileUpdate;

const SPT_MODULES: &[&str] = &["social_proof_tokens", "spt"];

#[derive(Debug, Clone)]
pub enum SptRow {
    SptPool(NewSptPool),
    SptTransaction(NewSptTransaction),
    SptHolding(NewSptHolding),
    SptPoolSupplyUpdate {
        pool_id: String,
        delta: i64,
    },
    SptPriceHistory(NewSptPriceHistory),
    SptReservationPool(NewSptReservationPool),
    SptReservation {
        associated_id: String,
        reservation: NewSptReservation,
        token_type: i16,
        total_reserved: i64,
        threshold_met: bool,
        created_at: i64,
    },
    SptReservationPoolUpdate {
        pool_id: String,
        associated_id: String,
        total_reserved: i64,
        status: Option<String>,
        required_threshold: Option<i64>,
    },
    SptExchangeConfig(NewSptExchangeConfig),
    SocialProofTokensConfig(NewSocialProofTokensConfig),
    SocialProofTokensEvent(NewSocialProofTokensEvent),
    SptBuySellRevenueData {
        pool_id: String,
        associated_id: String,
        token_type: i16,
        trader: String,
        transaction_type: String,
        creator_fee: i64,
        platform_fee: i64,
        treasury_fee: i64,
        amount: i64,
        myso_amount: i64,
        token_price: i64,
        revenue_time: i64,
        transaction_id: String,
    },
    ProfileUpdate(ProfileUpdate),
    PostRevenueRedirectUpdate {
        post_id: String,
        revenue_redirect_to: String,
        revenue_redirect_percentage: i64,
    },
}

impl SptRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::SptPool(p) => Some(SptRow::SptPool(p)),
            crate::handlers::SocialEventRow::SptTransaction(t) => Some(SptRow::SptTransaction(t)),
            crate::handlers::SocialEventRow::SptHolding(h) => Some(SptRow::SptHolding(h)),
            crate::handlers::SocialEventRow::SptPoolSupplyUpdate { pool_id, delta } => {
                Some(SptRow::SptPoolSupplyUpdate { pool_id, delta })
            }
            crate::handlers::SocialEventRow::SptPriceHistory(ph) => {
                Some(SptRow::SptPriceHistory(ph))
            }
            crate::handlers::SocialEventRow::SptReservationPool(rp) => {
                Some(SptRow::SptReservationPool(rp))
            }
            crate::handlers::SocialEventRow::SptReservation {
                associated_id,
                reservation,
                token_type,
                total_reserved,
                threshold_met,
                created_at,
            } => Some(SptRow::SptReservation {
                associated_id,
                reservation,
                token_type,
                total_reserved,
                threshold_met,
                created_at,
            }),
            crate::handlers::SocialEventRow::SptReservationPoolUpdate {
                pool_id,
                associated_id,
                total_reserved,
                status,
                required_threshold,
            } => Some(SptRow::SptReservationPoolUpdate {
                pool_id,
                associated_id,
                total_reserved,
                status,
                required_threshold,
            }),
            crate::handlers::SocialEventRow::SptExchangeConfig(c) => {
                Some(SptRow::SptExchangeConfig(c))
            }
            crate::handlers::SocialEventRow::SocialProofTokensConfig(c) => {
                Some(SptRow::SocialProofTokensConfig(c))
            }
            crate::handlers::SocialEventRow::SocialProofTokensEvent(e) => {
                Some(SptRow::SocialProofTokensEvent(e))
            }
            crate::handlers::SocialEventRow::SptBuySellRevenueData {
                pool_id,
                associated_id,
                token_type,
                trader,
                transaction_type,
                creator_fee,
                platform_fee,
                treasury_fee,
                amount,
                myso_amount,
                token_price,
                revenue_time,
                transaction_id,
            } => Some(SptRow::SptBuySellRevenueData {
                pool_id,
                associated_id,
                token_type,
                trader,
                transaction_type,
                creator_fee,
                platform_fee,
                treasury_fee,
                amount,
                myso_amount,
                token_price,
                revenue_time,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::ProfileUpdate(up) => Some(SptRow::ProfileUpdate(up)),
            crate::handlers::SocialEventRow::PostRevenueRedirectUpdate {
                post_id,
                revenue_redirect_to,
                revenue_redirect_percentage,
            } => Some(SptRow::PostRevenueRedirectUpdate {
                post_id,
                revenue_redirect_to,
                revenue_redirect_percentage,
            }),
            _ => None,
        }
    }
}

impl FieldCount for SptRow {
    const FIELD_COUNT: usize = 16;
}

pub struct SptHandler;

#[async_trait]
impl Processor for SptHandler {
    const NAME: &'static str = "spt";

    type Value = SptRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let epoch = checkpoint.summary.epoch;
        let timestamp_ms = checkpoint.summary.timestamp_ms;
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
                if !SPT_MODULES.contains(&module) {
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
                    spt::handle_spt_event(event_name, &event_data, &event_id, epoch, timestamp_ms)
                {
                    for row in rows {
                        if let Some(r) = SptRow::from_social(row) {
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
impl Handler for SptHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        use diesel::dsl::max;

        let mut total = 0;
        for row in values {
            match row {
                SptRow::SptPool(p) => {
                    total += diesel::insert_into(spt_pools::table)
                        .values(p)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptTransaction(t) => {
                    total += diesel::insert_into(spt_transactions::table)
                        .values(t)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptHolding(h) => {
                    total += diesel::insert_into(spt_holdings::table)
                        .values(h)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptPoolSupplyUpdate { pool_id, delta } => {
                    let update_sql =
                        "UPDATE spt_pools SET circulating_supply = circulating_supply + $1 \
                         WHERE pool_id = $2 AND time = (SELECT time FROM spt_pools WHERE pool_id = $2 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(*delta)
                        .bind::<Text, _>(pool_id)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptPriceHistory(ph) => {
                    total += diesel::insert_into(spt_price_history::table)
                        .values(ph)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptReservationPool(rp) => {
                    total += diesel::insert_into(spt_reservation_pools::table)
                        .values(rp)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptReservation {
                    associated_id,
                    reservation,
                    token_type,
                    total_reserved,
                    threshold_met,
                    created_at,
                } => {
                    #[derive(QueryableByName)]
                    struct PoolIdRow {
                        #[diesel(sql_type = Text)]
                        pool_id: String,
                    }
                    let pool_id_row: Option<PoolIdRow> = diesel::sql_query(
                        "SELECT pool_id FROM spt_reservation_pools WHERE associated_id = $1 ORDER BY time DESC LIMIT 1",
                    )
                    .bind::<Text, _>(associated_id)
                    .get_result(conn)
                    .await
                    .optional()?;
                    let pool_id = if let Some(ref row) = pool_id_row {
                        row.pool_id.clone()
                    } else {
                        let synthetic_pool_id = format!("reservation_pool_{}", associated_id);
                        #[derive(QueryableByName)]
                        struct OwnerRow {
                            #[diesel(sql_type = Text)]
                            owner: String,
                        }
                        let owner = if *token_type == TOKEN_TYPE_POST {
                            diesel::sql_query(
                                "SELECT owner FROM posts WHERE post_id = $1 ORDER BY time DESC LIMIT 1",
                            )
                            .bind::<Text, _>(associated_id)
                            .get_result::<OwnerRow>(conn)
                            .await
                            .ok()
                            .map(|r| r.owner)
                        } else {
                            diesel::sql_query(
                                "SELECT owner_address FROM profiles WHERE profile_id = $1 OR owner_address = $1 LIMIT 1",
                            )
                            .bind::<Text, _>(associated_id)
                            .get_result::<OwnerRow>(conn)
                            .await
                            .ok()
                            .map(|r| r.owner)
                        }
                        .unwrap_or_else(|| reservation.reserver_address.clone());
                        let status = if *threshold_met {
                            RESERVATION_POOL_STATUS_THRESHOLD_MET.to_string()
                        } else {
                            RESERVATION_POOL_STATUS_ACTIVE.to_string()
                        };
                        let synthetic_pool = NewSptReservationPool {
                            pool_id: synthetic_pool_id.clone(),
                            associated_id: associated_id.clone(),
                            token_type: *token_type,
                            owner: owner.clone(),
                            total_reserved: *total_reserved,
                            required_threshold: *total_reserved,
                            status,
                            created_at: *created_at,
                            time: reservation.time,
                            transaction_id: reservation.transaction_id.clone(),
                        };
                        total += diesel::insert_into(spt_reservation_pools::table)
                            .values(&synthetic_pool)
                            .execute(conn)
                            .await?;
                        tracing::info!(
                            associated_id = %associated_id,
                            pool_id = %synthetic_pool_id,
                            "created synthetic SptReservationPool (no canonical pool found)"
                        );
                        synthetic_pool_id
                    };
                    let mut r = reservation.clone();
                    r.pool_id = pool_id.clone();
                    total += diesel::insert_into(spt_reservations::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                    tracing::info!(
                        associated_id = %associated_id,
                        pool_id = %pool_id,
                        reserver = %reservation.reserver_address,
                        amount = %reservation.amount,
                        "SptReservation inserted"
                    );
                }
                SptRow::SptReservationPoolUpdate {
                    pool_id: _pool_id,
                    associated_id,
                    total_reserved,
                    status,
                    required_threshold,
                } => {
                    let update_sql =
                        "UPDATE spt_reservation_pools SET total_reserved = $1, \
                         status = COALESCE($2, status), \
                         required_threshold = COALESCE($4, required_threshold) \
                         WHERE associated_id = $3 AND time = (SELECT time FROM spt_reservation_pools WHERE associated_id = $3 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<BigInt, _>(*total_reserved)
                        .bind::<Nullable<Text>, _>(status.as_deref())
                        .bind::<Text, _>(associated_id)
                        .bind::<Nullable<BigInt>, _>(*required_threshold)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptExchangeConfig(c) => {
                    let sync_reservation_pool_thresholds =
                        c.profile_threshold > 0 && c.post_threshold > 0;
                    let profile_threshold = c.profile_threshold;
                    let post_threshold = c.post_threshold;
                    let latest: Option<(i32, chrono::NaiveDateTime)> = spt_exchange_config::table
                        .order(spt_exchange_config::time.desc())
                        .select((spt_exchange_config::id, spt_exchange_config::time))
                        .first(conn)
                        .await
                        .ok();
                    if let Some((id, time)) = latest {
                        total += diesel::update(spt_exchange_config::table)
                            .filter(spt_exchange_config::id.eq(id))
                            .filter(spt_exchange_config::time.eq(time))
                            .set((
                                spt_exchange_config::updated_by.eq(&c.updated_by),
                                spt_exchange_config::post_threshold.eq(c.post_threshold),
                                spt_exchange_config::profile_threshold.eq(c.profile_threshold),
                                spt_exchange_config::max_individual_reservation_bps
                                    .eq(c.max_individual_reservation_bps),
                                spt_exchange_config::total_fee_bps.eq(c.total_fee_bps),
                                spt_exchange_config::creator_fee_bps.eq(c.creator_fee_bps),
                                spt_exchange_config::platform_fee_bps.eq(c.platform_fee_bps),
                                spt_exchange_config::treasury_fee_bps.eq(c.treasury_fee_bps),
                                spt_exchange_config::trading_creator_fee_bps
                                    .eq(c.trading_creator_fee_bps),
                                spt_exchange_config::trading_platform_fee_bps
                                    .eq(c.trading_platform_fee_bps),
                                spt_exchange_config::trading_treasury_fee_bps
                                    .eq(c.trading_treasury_fee_bps),
                                spt_exchange_config::reservation_creator_fee_bps
                                    .eq(c.reservation_creator_fee_bps),
                                spt_exchange_config::reservation_platform_fee_bps
                                    .eq(c.reservation_platform_fee_bps),
                                spt_exchange_config::reservation_treasury_fee_bps
                                    .eq(c.reservation_treasury_fee_bps),
                                spt_exchange_config::max_reservers_per_pool
                                    .eq(c.max_reservers_per_pool),
                                spt_exchange_config::base_price.eq(c.base_price),
                                spt_exchange_config::quadratic_coefficient
                                    .eq(c.quadratic_coefficient),
                                spt_exchange_config::max_hold_percent_bps
                                    .eq(c.max_hold_percent_bps),
                                spt_exchange_config::trading_enabled.eq(c.trading_enabled),
                                spt_exchange_config::updated_at.eq(c.updated_at),
                                spt_exchange_config::transaction_id.eq(&c.transaction_id),
                            ))
                            .execute(conn)
                            .await?;
                    } else {
                        total += diesel::insert_into(spt_exchange_config::table)
                            .values(c)
                            .execute(conn)
                            .await?;
                    }
                    if sync_reservation_pool_thresholds {
                        let sync_sql = r#"
                            UPDATE spt_reservation_pools sp
                            SET required_threshold = CASE sp.token_type
                                WHEN 1 THEN $1
                                WHEN 2 THEN $2
                                ELSE sp.required_threshold
                            END
                            FROM (
                                SELECT DISTINCT ON (pool_id) pool_id, time
                                FROM spt_reservation_pools
                                ORDER BY pool_id, time DESC
                            ) AS latest
                            WHERE sp.pool_id = latest.pool_id AND sp.time = latest.time
                        "#;
                        total += diesel::sql_query(sync_sql)
                            .bind::<BigInt, _>(profile_threshold)
                            .bind::<BigInt, _>(post_threshold)
                            .execute(conn)
                            .await?;
                    }
                }
                SptRow::SocialProofTokensConfig(c) => {
                    let max_id: Option<i32> = social_proof_tokens_config::table
                        .select(max(social_proof_tokens_config::id))
                        .get_result(conn)
                        .await
                        .ok()
                        .flatten();
                    if let Some(id) = max_id {
                        total += diesel::update(social_proof_tokens_config::table)
                            .filter(social_proof_tokens_config::id.eq(id))
                            .set((
                                social_proof_tokens_config::trading_enabled.eq(c.trading_enabled),
                                social_proof_tokens_config::admin_address.eq(&c.admin_address),
                                social_proof_tokens_config::reason.eq(&c.reason),
                                social_proof_tokens_config::timestamp_ms.eq(c.timestamp_ms),
                                social_proof_tokens_config::updated_at.eq(c.updated_at),
                                social_proof_tokens_config::transaction_id.eq(&c.transaction_id),
                            ))
                            .execute(conn)
                            .await?;
                    } else {
                        total += diesel::insert_into(social_proof_tokens_config::table)
                            .values(c)
                            .execute(conn)
                            .await?;
                    }
                }
                SptRow::SocialProofTokensEvent(e) => {
                    total += diesel::insert_into(social_proof_tokens_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                SptRow::SptBuySellRevenueData {
                    pool_id,
                    trader,
                    transaction_type,
                    creator_fee,
                    platform_fee,
                    treasury_fee,
                    amount,
                    myso_amount,
                    token_price,
                    revenue_time,
                    transaction_id,
                    ..
                } => {
                    let pool_row: Option<(String, String, i16)> = spt_pools::table
                        .filter(spt_pools::pool_id.eq(pool_id))
                        .order(spt_pools::time.desc())
                        .select((
                            spt_pools::owner,
                            spt_pools::associated_id,
                            spt_pools::token_type,
                        ))
                        .first::<(String, String, i16)>(conn)
                        .await
                        .ok();

                    let (creator_address, platform_address, treasury_address): (
                        String,
                        String,
                        String,
                    ) = if let Some((owner, _associated_id, _token_type)) = pool_row {
                        let treasury = ecosystem_treasury::table
                            .order(ecosystem_treasury::time.desc())
                            .select(ecosystem_treasury::treasury_address)
                            .first::<String>(conn)
                            .await
                            .ok()
                            .unwrap_or_default();
                        (owner, String::new(), treasury)
                    } else {
                        (String::new(), String::new(), String::new())
                    };

                    if *creator_fee != 0 || *platform_fee != 0 || *treasury_fee != 0 {
                        let spt_rev = if transaction_type == "SELL" {
                            NewSptRevenue::from_sell_event(
                                pool_id.clone(),
                                trader.clone(),
                                creator_address.clone(),
                                platform_address.clone(),
                                treasury_address.clone(),
                                *creator_fee,
                                *platform_fee,
                                *treasury_fee,
                                *amount,
                                *myso_amount,
                                *token_price,
                                *revenue_time,
                                transaction_id.clone(),
                            )
                        } else {
                            NewSptRevenue::from_buy_event(
                                pool_id.clone(),
                                trader.clone(),
                                creator_address.clone(),
                                platform_address.clone(),
                                treasury_address.clone(),
                                *creator_fee,
                                *platform_fee,
                                *treasury_fee,
                                *amount,
                                *myso_amount,
                                *token_price,
                                *revenue_time,
                                transaction_id.clone(),
                            )
                        };
                        total += diesel::insert_into(spt_revenue::table)
                            .values(&spt_rev)
                            .execute(conn)
                            .await?;

                        if *creator_fee > 0 {
                            total += diesel::insert_into(unified_revenue::table)
                                .values(NewUnifiedRevenue::from_spt(
                                    REVENUE_TYPE_SPT_CREATOR_FEE.to_string(),
                                    creator_address.clone(),
                                    Some(platform_address.clone()),
                                    *creator_fee,
                                    pool_id.clone(),
                                    trader.clone(),
                                    creator_address.clone(),
                                    *revenue_time,
                                    transaction_id.clone(),
                                ))
                                .execute(conn)
                                .await?;
                        }
                        if *platform_fee > 0 {
                            total += diesel::insert_into(unified_revenue::table)
                                .values(NewUnifiedRevenue::from_spt(
                                    REVENUE_TYPE_SPT_PLATFORM_FEE.to_string(),
                                    creator_address.clone(),
                                    Some(platform_address.clone()),
                                    *platform_fee,
                                    pool_id.clone(),
                                    trader.clone(),
                                    platform_address.clone(),
                                    *revenue_time,
                                    transaction_id.clone(),
                                ))
                                .execute(conn)
                                .await?;
                        }
                        if *treasury_fee > 0 {
                            total += diesel::insert_into(unified_revenue::table)
                                .values(NewUnifiedRevenue::from_spt(
                                    REVENUE_TYPE_SPT_TREASURY_FEE.to_string(),
                                    creator_address.clone(),
                                    None,
                                    *treasury_fee,
                                    pool_id.clone(),
                                    trader.clone(),
                                    treasury_address.clone(),
                                    *revenue_time,
                                    transaction_id.clone(),
                                ))
                                .execute(conn)
                                .await?;
                        }
                    }
                }
                SptRow::ProfileUpdate(up) => {
                    let now = chrono::Utc::now().naive_utc();
                    let set = ProfileUpdateSet {
                        updated_at: now,
                        display_name: up.display_name.clone().map(Some),
                        bio: up.bio.clone().map(Some),
                        profile_photo: up.profile_photo.clone().map(Some),
                        cover_photo: up.cover_photo.clone().map(Some),
                        birthdate: up.birthdate.clone().map(Some),
                        current_location: up.current_location.clone().map(Some),
                        raised_location: up.raised_location.clone().map(Some),
                        phone: up.phone.clone().map(Some),
                        email: up.email.clone().map(Some),
                        gender: up.gender.clone().map(Some),
                        political_view: up.political_view.clone().map(Some),
                        religion: up.religion.clone().map(Some),
                        education: up.education.clone().map(Some),
                        primary_language: up.primary_language.clone().map(Some),
                        relationship_status: up.relationship_status.clone().map(Some),
                        x_username: up.x_username.clone().map(Some),
                        facebook_username: up.facebook_username.clone().map(Some),
                        reddit_username: up.reddit_username.clone().map(Some),
                        github_username: up.github_username.clone().map(Some),
                        instagram_username: up.instagram_username.clone().map(Some),
                        linkedin_username: up.linkedin_username.clone().map(Some),
                        twitch_username: up.twitch_username.clone().map(Some),
                        min_offer_amount: up.min_offer_amount.map(Some),
                        username: up.username.clone(),
                        selected_badge_id: up.selected_badge_id.clone(),
                        selected_ecosystem_badge_id: up.selected_ecosystem_badge_id.clone(),
                        paid_messaging_enabled: up.paid_messaging_enabled,
                        paid_messaging_min_cost: up.paid_messaging_min_cost.map(Some),
                        reservation_pool_address: up.reservation_pool_address.clone(),
                    };
                    let filter = profiles::profile_id
                        .eq(&up.profile_id)
                        .or(profiles::owner_address.eq(&up.owner_address));
                    total += diesel::update(profiles::table)
                        .filter(filter)
                        .set(set)
                        .execute(conn)
                        .await?;
                }
                SptRow::PostRevenueRedirectUpdate {
                    post_id,
                    revenue_redirect_to,
                    revenue_redirect_percentage,
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set((
                            posts::revenue_redirect_to.eq(Some(revenue_redirect_to)),
                            posts::revenue_redirect_percentage
                                .eq(Some(*revenue_redirect_percentage)),
                        ))
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
