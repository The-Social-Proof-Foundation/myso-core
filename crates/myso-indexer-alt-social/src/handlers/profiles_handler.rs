// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Profiles pipeline: indexes profile module events (including EcosystemTreasury from profile).
//! For greenfield `create_profile`, also indexes memory and ai_credit bootstrap events in
//! transaction order so profile rows can be inserted with linked account and balance ids.

use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Text, Timestamp};
use diesel::upsert::excluded;
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewAiCreditBalance, NewEcosystemTreasury, NewMemoryAccount, NewProfile, NewProfileBadge,
    NewProfileConfig, NewProfileEvent, NewProfileOffer, NewProfileSaleFee, NewUsernameRegistry,
    NewVestingEvent, NewVestingWallet, ProfileUpdateSet, default_profile_config,
    merge_profile_config,
};
use myso_indexer_alt_social_schema::schema::{
    ai_credit_balances, ecosystem_treasury, memory_accounts, profile_badges, profile_config,
    profile_events, profile_offers, profile_sale_fees, profiles, username_registry, vesting_events,
    vesting_wallets,
};

use super::ai_credit;
use super::common;
use super::events;
use super::memory;
use super::profile;
use super::ProfileUpdate;

const PROFILE_MODULES: &[&str] = &["profile"];
const PROFILE_BOOTSTRAP_MODULES: &[&str] = &["memory", "ai_credit", "profile"];

#[derive(Debug, Clone)]
pub enum ProfileRow {
    Profile(NewProfile),
    ProfileUpdate(ProfileUpdate),
    ProfileXUsernameUpdate {
        profile_id: String,
        owner_address: String,
        x_username: Option<String>,
    },
    UsernameRegistryUpsert(NewUsernameRegistry),
    UsernameRegistryDelete {
        username: String,
    },
    UsernameRegistryReassign {
        username: String,
        new_profile_id: String,
        transaction_id: String,
    },
    ProfileUsernameSet {
        profile_id: String,
        username: String,
    },
    ProfileUsernameClear {
        profile_id: String,
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
    ProfileConfig(NewProfileConfig),
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
    MemoryAccountBootstrap(NewMemoryAccount),
    AiCreditBalanceBootstrap(NewAiCreditBalance),
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
            crate::handlers::SocialEventRow::UsernameRegistryUpsert(row) => {
                Some(ProfileRow::UsernameRegistryUpsert(row))
            }
            crate::handlers::SocialEventRow::UsernameRegistryDelete { username } => {
                Some(ProfileRow::UsernameRegistryDelete { username })
            }
            crate::handlers::SocialEventRow::UsernameRegistryReassign {
                username,
                new_profile_id,
                transaction_id,
            } => Some(ProfileRow::UsernameRegistryReassign {
                username,
                new_profile_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::ProfileUsernameSet {
                profile_id,
                username,
            } => Some(ProfileRow::ProfileUsernameSet {
                profile_id,
                username,
            }),
            crate::handlers::SocialEventRow::ProfileUsernameClear { profile_id } => {
                Some(ProfileRow::ProfileUsernameClear { profile_id })
            }
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
            crate::handlers::SocialEventRow::ProfileConfig(c) => Some(ProfileRow::ProfileConfig(c)),
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
    const FIELD_COUNT: usize = 22;
}

pub struct ProfilesHandler;

#[async_trait]
impl Processor for ProfilesHandler {
    const NAME: &'static str = "profiles";

    type Value = ProfileRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_timestamp_ms = checkpoint.summary.timestamp_ms;
        let mut values = Vec::new();
        for tx in &checkpoint.transactions {
            let tx_digest = tx.transaction.digest().to_string();
            let Some(events) = &tx.events else {
                continue;
            };
            let mut memory_by_profile: HashMap<String, String> = HashMap::new();
            let mut balance_by_profile: HashMap<String, String> = HashMap::new();
            for (event_seq, ev) in events.data.iter().enumerate() {
                if !common::is_social_package_event(&ev.package_id, &ev.type_.address) {
                    continue;
                }
                let module = ev.type_.module.as_str();
                if !PROFILE_BOOTSTRAP_MODULES.contains(&module) {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data =
                    match events::parse_event_contents(module, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                match module {
                    "memory" if event_name == "MemoryAccountCreated" => {
                        if let Some(rows) =
                            memory::handle_memory_event(event_name, &event_data, &event_id)
                        {
                            for row in rows {
                                if let crate::handlers::SocialEventRow::MemoryAccount(a) = row {
                                    memory_by_profile
                                        .insert(a.profile_id.clone(), a.account_id.clone());
                                    values.push(ProfileRow::MemoryAccountBootstrap(a));
                                }
                            }
                        }
                    }
                    "ai_credit" if event_name == "AiCreditBalanceCreated" => {
                        if let Some(rows) =
                            ai_credit::handle_ai_credit_event(event_name, &event_data, &event_id)
                        {
                            for row in rows {
                                if let crate::handlers::SocialEventRow::AiCreditBalanceUpsert(b) =
                                    row
                                {
                                    balance_by_profile
                                        .insert(b.profile_id.clone(), b.balance_id.clone());
                                    values.push(ProfileRow::AiCreditBalanceBootstrap(b));
                                }
                            }
                        }
                    }
                    "profile" if PROFILE_MODULES.contains(&module) => {
                        if let Some(rows) = profile::handle_profile_event(
                            event_name,
                            &event_data,
                            &event_id,
                            checkpoint_timestamp_ms,
                        ) {
                            for row in rows {
                                if let Some(mut r) = ProfileRow::from_social(row) {
                                    if let ProfileRow::Profile(ref mut p) = r {
                                        if let Some(ref profile_id) = p.profile_id {
                                            profile::enrich_new_profile_bootstrap(
                                                p,
                                                memory_by_profile.get(profile_id).cloned(),
                                                balance_by_profile.get(profile_id).cloned(),
                                            );
                                        }
                                    }
                                    values.push(r);
                                }
                            }
                        }
                    }
                    _ => {}
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
                ProfileRow::MemoryAccountBootstrap(a) => {
                    total += commit_memory_account_bootstrap(a, conn).await?;
                }
                ProfileRow::AiCreditBalanceBootstrap(b) => {
                    total += commit_ai_credit_balance_bootstrap(b, conn).await?;
                }
                _ => {}
            }
        }
        // Insert profiles before UsernameClaimed updates (Move emits UsernameClaimedEvent before
        // ProfileCreatedEvent in the same transaction).
        for row in values {
            if let ProfileRow::Profile(profile) = row {
                total += commit_profile_insert(profile, conn).await?;
            }
        }
        for row in values {
            match row {
                ProfileRow::Profile(_)
                | ProfileRow::MemoryAccountBootstrap(_)
                | ProfileRow::AiCreditBalanceBootstrap(_) => {}
                other => {
                    total += commit_profile_row(other, conn).await?;
                }
            }
        }
        Ok(total)
    }
}

async fn commit_memory_account_bootstrap<'a>(
    a: &NewMemoryAccount,
    conn: &mut Connection<'a>,
) -> Result<usize> {
    let principal_owner = a.principal_owner.clone();
    let profile_id = a.profile_id.clone();
    let active = a.active;
    let created_at_ms = a.created_at_ms;
    let event_id = a.event_id.clone();
    let transaction_id = a.transaction_id.clone();
    let time = a.time;
    Ok(diesel::insert_into(memory_accounts::table)
        .values(a)
        .on_conflict(memory_accounts::account_id)
        .do_update()
        .set((
            memory_accounts::principal_owner.eq(principal_owner),
            memory_accounts::profile_id.eq(profile_id),
            memory_accounts::active.eq(active),
            memory_accounts::created_at_ms.eq(created_at_ms),
            memory_accounts::event_id.eq(event_id),
            memory_accounts::transaction_id.eq(transaction_id),
            memory_accounts::time.eq(time),
        ))
        .execute(conn)
        .await?)
}

async fn commit_ai_credit_balance_bootstrap<'a>(
    b: &NewAiCreditBalance,
    conn: &mut Connection<'a>,
) -> Result<usize> {
    Ok(diesel::insert_into(ai_credit_balances::table)
        .values(b)
        .on_conflict(ai_credit_balances::balance_id)
        .do_update()
        .set((
            ai_credit_balances::memory_account_id.eq(b.memory_account_id.clone()),
            ai_credit_balances::principal_owner.eq(b.principal_owner.clone()),
            ai_credit_balances::profile_id.eq(b.profile_id.clone()),
            ai_credit_balances::balance_mist.eq(b.balance_mist),
            ai_credit_balances::spent_total_mist.eq(b.spent_total_mist),
            ai_credit_balances::daily_cap_mist.eq(b.daily_cap_mist),
            ai_credit_balances::monthly_cap_mist.eq(b.monthly_cap_mist),
            ai_credit_balances::spent_day_mist.eq(b.spent_day_mist),
            ai_credit_balances::spent_month_mist.eq(b.spent_month_mist),
            ai_credit_balances::settlement_nonce.eq(b.settlement_nonce),
            ai_credit_balances::active.eq(b.active),
            ai_credit_balances::updated_at_ms.eq(b.updated_at_ms),
            ai_credit_balances::event_id.eq(b.event_id.clone()),
            ai_credit_balances::transaction_id.eq(b.transaction_id.clone()),
            ai_credit_balances::time.eq(b.time),
        ))
        .execute(conn)
        .await?)
}

async fn commit_profile_insert<'a>(
    profile: &NewProfile,
    conn: &mut Connection<'a>,
) -> Result<usize> {
    let mut total = diesel::insert_into(profiles::table)
        .values(profile)
        .on_conflict(profiles::owner_address)
        .do_nothing()
        .execute(conn)
        .await?;
    if let Some(ref profile_id) = profile.profile_id {
        if profile.memory_account_id.is_none() {
            if let Some(account_id) = memory_accounts::table
                .filter(memory_accounts::profile_id.eq(profile_id))
                .select(memory_accounts::account_id)
                .first::<String>(conn)
                .await
                .optional()?
            {
                total += diesel::update(
                    profiles::table
                        .filter(profiles::profile_id.eq(profile_id))
                        .filter(profiles::memory_account_id.is_null()),
                )
                .set(profiles::memory_account_id.eq(account_id))
                .execute(conn)
                .await?;
            }
        }
        if profile.ai_credit_balance_id.is_none() {
            if let Some(balance_id) = ai_credit_balances::table
                .filter(ai_credit_balances::profile_id.eq(profile_id))
                .select(ai_credit_balances::balance_id)
                .first::<String>(conn)
                .await
                .optional()?
            {
                total += diesel::update(
                    profiles::table
                        .filter(profiles::profile_id.eq(profile_id))
                        .filter(profiles::ai_credit_balance_id.is_null()),
                )
                .set(profiles::ai_credit_balance_id.eq(balance_id))
                .execute(conn)
                .await?;
            }
        }
    }
    Ok(total)
}

fn default_ecosystem_treasury() -> NewEcosystemTreasury {
    NewEcosystemTreasury {
        treasury_address: String::new(),
        updated_by: String::new(),
        updated_at: 0,
        time: chrono::Utc::now(),
        transaction_id: String::new(),
        version: 0,
    }
}

fn merge_ecosystem_treasury(
    prev: &NewEcosystemTreasury,
    incoming: &NewEcosystemTreasury,
) -> NewEcosystemTreasury {
    let version = if incoming.version > 0 {
        incoming.version
    } else {
        prev.version + 1
    };
    NewEcosystemTreasury {
        treasury_address: if incoming.treasury_address.is_empty() {
            prev.treasury_address.clone()
        } else {
            incoming.treasury_address.clone()
        },
        updated_by: incoming.updated_by.clone(),
        updated_at: incoming.updated_at,
        time: incoming.time,
        transaction_id: incoming.transaction_id.clone(),
        version,
    }
}

async fn load_latest_profile_config(
    conn: &mut Connection<'_>,
) -> Result<Option<NewProfileConfig>> {
    profile_config::table
        .order(profile_config::time.desc())
        .select(NewProfileConfig::as_select())
        .first(conn)
        .await
        .optional()
        .map_err(Into::into)
}

async fn load_latest_ecosystem_treasury(
    conn: &mut Connection<'_>,
) -> Result<Option<NewEcosystemTreasury>> {
    ecosystem_treasury::table
        .order(ecosystem_treasury::time.desc())
        .select((
            ecosystem_treasury::treasury_address,
            ecosystem_treasury::updated_by,
            ecosystem_treasury::updated_at,
            ecosystem_treasury::time,
            ecosystem_treasury::transaction_id,
            ecosystem_treasury::version,
        ))
        .first::<(
            String,
            String,
            i64,
            chrono::DateTime<chrono::Utc>,
            String,
            i64,
        )>(conn)
        .await
        .optional()
        .map(|opt| {
            opt.map(
                |(treasury_address, updated_by, updated_at, time, transaction_id, version)| {
                    NewEcosystemTreasury {
                        treasury_address,
                        updated_by,
                        updated_at,
                        time,
                        transaction_id,
                        version,
                    }
                },
            )
        })
        .map_err(Into::into)
}

async fn commit_profile_row<'a>(row: &ProfileRow, conn: &mut Connection<'a>) -> Result<usize> {
    let mut total = 0;
    match row {
        ProfileRow::Profile(profile) => return commit_profile_insert(profile, conn).await,
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
        ProfileRow::UsernameRegistryUpsert(row) => {
            total += diesel::insert_into(username_registry::table)
                .values(row)
                .on_conflict(username_registry::username)
                .do_update()
                .set((
                    username_registry::profile_id.eq(excluded(username_registry::profile_id)),
                    username_registry::transaction_id
                        .eq(excluded(username_registry::transaction_id)),
                ))
                .execute(conn)
                .await?;
        }
        ProfileRow::UsernameRegistryDelete { username } => {
            total += diesel::delete(username_registry::table)
                .filter(username_registry::username.eq(username))
                .execute(conn)
                .await?;
        }
        ProfileRow::UsernameRegistryReassign {
            username,
            new_profile_id,
            transaction_id,
        } => {
            total += diesel::update(username_registry::table)
                .filter(username_registry::username.eq(username))
                .set((
                    username_registry::profile_id.eq(new_profile_id),
                    username_registry::transaction_id.eq(transaction_id),
                ))
                .execute(conn)
                .await?;
        }
        ProfileRow::ProfileUsernameSet {
            profile_id,
            username,
        } => {
            let now = chrono::Utc::now().naive_utc();
            let updated = diesel::update(profiles::table)
                .filter(profiles::profile_id.eq(profile_id))
                .set((
                    profiles::username.eq(username),
                    profiles::updated_at.eq(now),
                ))
                .execute(conn)
                .await?;
            if updated == 0 {
                tracing::warn!(
                    profile_id = %profile_id,
                    username = %username,
                    "ProfileUsernameSet updated 0 rows after profile insert pass"
                );
            }
            total += updated;
        }
        ProfileRow::ProfileUsernameClear { profile_id } => {
            let now = chrono::Utc::now().naive_utc();
            total += diesel::update(profiles::table)
                .filter(profiles::profile_id.eq(profile_id))
                .set((profiles::username.eq(""), profiles::updated_at.eq(now)))
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
                website: up.website.clone().map(Some),
                birthdate: up.birthdate.clone().map(Some),
                location: up.location.clone().map(Some),
                x_username: up.x_username.clone().map(Some),
                min_offer_amount: up.min_offer_amount.map(Some),
                username: up.username.clone(),
                selected_badge_id: up.selected_badge_id.clone(),
                selected_ecosystem_badge_id: up.selected_ecosystem_badge_id.clone(),
                reservation_pool_address: up.reservation_pool_address.clone(),
                social_proof_token_address: up.social_proof_token_address.clone(),
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
            let prev = load_latest_ecosystem_treasury(conn)
                .await?
                .unwrap_or_else(default_ecosystem_treasury);
            let merged = merge_ecosystem_treasury(&prev, c);
            total += diesel::insert_into(ecosystem_treasury::table)
                .values(&merged)
                .execute(conn)
                .await?;
        }
        ProfileRow::ProfileConfig(c) => {
            let prev = load_latest_profile_config(conn)
                .await?
                .unwrap_or_else(default_profile_config);
            let merged = merge_profile_config(&prev, c);
            total += diesel::insert_into(profile_config::table)
                .values(&merged)
                .execute(conn)
                .await?;
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
            claimed_amount: _,
            remaining_balance,
        } => {
            let now = chrono::Utc::now().naive_utc();
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
        ProfileRow::MemoryAccountBootstrap(_) | ProfileRow::AiCreditBalanceBootstrap(_) => {}
    }
    Ok(total)
}
