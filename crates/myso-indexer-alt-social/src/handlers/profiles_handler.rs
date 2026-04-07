// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Profiles pipeline: indexes profile module events (including EcosystemTreasury from profile).

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Text, Timestamp};
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewEcosystemTreasury, NewProfile, NewProfileBadge, NewProfileEvent, NewProfileOffer,
    NewProfileSaleFee, NewVestingEvent, NewVestingWallet, ProfileUpdateSet,
};
use myso_indexer_alt_social_schema::schema::{
    ecosystem_treasury, profile_badges, profile_events, profile_offers, profile_sale_fees,
    profiles, vesting_events, vesting_wallets,
};

use super::common;
use super::events;
use super::profile;
use super::ProfileUpdate;

const PROFILE_MODULES: &[&str] = &["profile"];

#[derive(Debug, Clone)]
pub enum ProfileRow {
    Profile(NewProfile),
    ProfileUpdate(ProfileUpdate),
    ProfileXUsernameUpdate {
        profile_id: String,
        owner_address: String,
        x_username: Option<String>,
    },
    ProfileEvent(NewProfileEvent),
    ProfileOffer(NewProfileOffer),
    ProfileOfferStatusUpdate {
        profile_id: String,
        offeror_address: String,
        status: String,
        resolved_at: i64,
        updated_at: i64,
        transaction_id: String,
    },
    ProfileSaleFee(NewProfileSaleFee),
    ProfileBadge(NewProfileBadge),
    ProfileBadgeRevoke {
        profile_id: String,
        badge_id: String,
        revoked_at: i64,
        revoked_by: String,
    },
    EcosystemTreasury(NewEcosystemTreasury),
    VestingWallet(NewVestingWallet),
    VestingEvent(NewVestingEvent),
    VestingWalletClaimUpdate {
        wallet_id: String,
        claimed_amount: i64,
        remaining_balance: i64,
    },
    VestingWalletDelete {
        wallet_id: String,
    },
}

impl ProfileRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::Profile(p) => Some(ProfileRow::Profile(p)),
            crate::handlers::SocialEventRow::ProfileUpdate(u) => Some(ProfileRow::ProfileUpdate(u)),
            crate::handlers::SocialEventRow::ProfileXUsernameUpdate {
                profile_id,
                owner_address,
                x_username,
            } => Some(ProfileRow::ProfileXUsernameUpdate {
                profile_id,
                owner_address,
                x_username,
            }),
            crate::handlers::SocialEventRow::ProfileEvent(e) => Some(ProfileRow::ProfileEvent(e)),
            crate::handlers::SocialEventRow::ProfileOffer(o) => Some(ProfileRow::ProfileOffer(o)),
            crate::handlers::SocialEventRow::ProfileOfferStatusUpdate {
                profile_id,
                offeror_address,
                status,
                resolved_at,
                updated_at,
                transaction_id,
            } => Some(ProfileRow::ProfileOfferStatusUpdate {
                profile_id,
                offeror_address,
                status,
                resolved_at,
                updated_at,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::ProfileSaleFee(f) => {
                Some(ProfileRow::ProfileSaleFee(f))
            }
            crate::handlers::SocialEventRow::ProfileBadge(b) => Some(ProfileRow::ProfileBadge(b)),
            crate::handlers::SocialEventRow::ProfileBadgeRevoke {
                profile_id,
                badge_id,
                revoked_at,
                revoked_by,
            } => Some(ProfileRow::ProfileBadgeRevoke {
                profile_id,
                badge_id,
                revoked_at,
                revoked_by,
            }),
            crate::handlers::SocialEventRow::EcosystemTreasury(c) => {
                Some(ProfileRow::EcosystemTreasury(c))
            }
            crate::handlers::SocialEventRow::VestingWallet(w) => Some(ProfileRow::VestingWallet(w)),
            crate::handlers::SocialEventRow::VestingEvent(e) => Some(ProfileRow::VestingEvent(e)),
            crate::handlers::SocialEventRow::VestingWalletClaimUpdate {
                wallet_id,
                claimed_amount,
                remaining_balance,
            } => Some(ProfileRow::VestingWalletClaimUpdate {
                wallet_id,
                claimed_amount,
                remaining_balance,
            }),
            crate::handlers::SocialEventRow::VestingWalletDelete { wallet_id } => {
                Some(ProfileRow::VestingWalletDelete { wallet_id })
            }
            _ => None,
        }
    }
}

impl FieldCount for ProfileRow {
    const FIELD_COUNT: usize = 20;
}

pub struct ProfilesHandler;

#[async_trait]
impl Processor for ProfilesHandler {
    const NAME: &'static str = "profiles";

    type Value = ProfileRow;

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
                if !PROFILE_MODULES.contains(&module) {
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
                    profile::handle_profile_event(event_name, &event_data, &event_id)
                {
                    for row in rows {
                        if let Some(r) = ProfileRow::from_social(row) {
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
impl Handler for ProfilesHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                ProfileRow::Profile(profile) => {
                    total += diesel::insert_into(profiles::table)
                        .values(profile)
                        .on_conflict(profiles::owner_address)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                ProfileRow::ProfileXUsernameUpdate {
                    profile_id,
                    owner_address,
                    x_username,
                } => {
                    let now = chrono::Utc::now().naive_utc();
                    let filter = profiles::profile_id
                        .eq(profile_id)
                        .or(profiles::owner_address.eq(owner_address));
                    total += diesel::update(profiles::table)
                        .filter(filter)
                        .set((
                            profiles::x_username.eq(x_username),
                            profiles::updated_at.eq(now),
                        ))
                        .execute(conn)
                        .await?;
                }
                ProfileRow::ProfileUpdate(up) => {
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
                ProfileRow::ProfileEvent(ev) => {
                    total += diesel::insert_into(profile_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                ProfileRow::ProfileOffer(offer) => {
                    total += diesel::insert_into(profile_offers::table)
                        .values(offer)
                        .execute(conn)
                        .await?;
                }
                ProfileRow::ProfileOfferStatusUpdate {
                    profile_id,
                    offeror_address,
                    status,
                    resolved_at,
                    updated_at,
                    transaction_id,
                } => {
                    let _ = diesel::update(profile_offers::table)
                        .filter(profile_offers::profile_id.eq(profile_id))
                        .filter(profile_offers::offeror_address.eq(offeror_address))
                        .filter(profile_offers::status.eq("pending"))
                        .set((
                            profile_offers::status.eq(status),
                            profile_offers::resolved_at.eq(Some(*resolved_at)),
                            profile_offers::updated_at.eq(*updated_at),
                            profile_offers::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await;
                }
                ProfileRow::ProfileSaleFee(fee) => {
                    total += diesel::insert_into(profile_sale_fees::table)
                        .values(fee)
                        .execute(conn)
                        .await?;
                }
                ProfileRow::EcosystemTreasury(c) => {
                    let latest: Option<(i32, chrono::NaiveDateTime)> = ecosystem_treasury::table
                        .order(ecosystem_treasury::time.desc())
                        .select((ecosystem_treasury::id, ecosystem_treasury::time))
                        .first(conn)
                        .await
                        .ok();
                    if let Some((id, time)) = latest {
                        total += diesel::update(ecosystem_treasury::table)
                            .filter(ecosystem_treasury::id.eq(id))
                            .filter(ecosystem_treasury::time.eq(time))
                            .set((
                                ecosystem_treasury::treasury_address.eq(&c.treasury_address),
                                ecosystem_treasury::updated_by.eq(&c.updated_by),
                                ecosystem_treasury::timestamp_ms.eq(c.timestamp_ms),
                                ecosystem_treasury::transaction_id.eq(&c.transaction_id),
                            ))
                            .execute(conn)
                            .await?;
                    } else {
                        total += diesel::insert_into(ecosystem_treasury::table)
                            .values(c)
                            .execute(conn)
                            .await?;
                    }
                }
                ProfileRow::ProfileBadge(badge) => {
                    total += diesel::insert_into(profile_badges::table)
                        .values(badge)
                        .execute(conn)
                        .await?;
                }
                ProfileRow::ProfileBadgeRevoke {
                    profile_id,
                    badge_id,
                    revoked_at,
                    revoked_by,
                } => {
                    total += diesel::update(profile_badges::table)
                        .filter(profile_badges::profile_id.eq(profile_id))
                        .filter(profile_badges::badge_id.eq(badge_id))
                        .filter(profile_badges::revoked.eq(false))
                        .set((
                            profile_badges::revoked.eq(true),
                            profile_badges::revoked_at.eq(Some(*revoked_at)),
                            profile_badges::revoked_by.eq(Some(revoked_by.clone())),
                        ))
                        .execute(conn)
                        .await?;
                }
                ProfileRow::VestingWallet(w) => {
                    total += diesel::insert_into(vesting_wallets::table)
                        .values(w)
                        .on_conflict(vesting_wallets::wallet_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                ProfileRow::VestingEvent(e) => {
                    total += diesel::insert_into(vesting_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                ProfileRow::VestingWalletClaimUpdate {
                    wallet_id,
                    claimed_amount: _, // chain event field is per-claim delta, not cumulative
                    remaining_balance,
                } => {
                    let now = chrono::Utc::now().naive_utc();
                    // Cumulative claimed = total_amount - balance after claim (see profile::VestingWallet).
                    // Idempotent if the same TokensClaimed event is replayed.
                    let upd = diesel::sql_query(
                        "UPDATE vesting_wallets SET \
                         claimed_amount = GREATEST(0, total_amount - $1), \
                         remaining_balance = $1, \
                         updated_at = $2 \
                         WHERE wallet_id = $3",
                    )
                    .bind::<BigInt, _>(*remaining_balance)
                    .bind::<Timestamp, _>(now)
                    .bind::<Text, _>(wallet_id);
                    total += upd.execute(conn).await?;
                }
                ProfileRow::VestingWalletDelete { wallet_id } => {
                    total += diesel::delete(vesting_wallets::table)
                        .filter(vesting_wallets::wallet_id.eq(wallet_id))
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
