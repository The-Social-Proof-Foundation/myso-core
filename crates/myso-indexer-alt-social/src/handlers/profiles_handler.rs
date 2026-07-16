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
    default_profile_config, merge_profile_config, NewAiCreditBalance, NewEcosystemTreasury,
    NewMemoryAccount, NewProfile, NewProfileBadge, NewProfileConfig, NewProfileEvent,
    NewUnifiedRevenue, NewUsernameListing, NewUsernameOffer, NewUsernameRegistry,
    NewUsernameReservation, NewUsernameSaleFee, NewVestingEvent, NewVestingWallet,
    ProfileUpdateSet, REVENUE_TYPE_USERNAME_MARKETPLACE_ECOSYSTEM_FEE,
    REVENUE_TYPE_USERNAME_MARKETPLACE_SELLER_NET, USERNAME_RESERVATION_STATUS_ACTIVE,
    USERNAME_RESERVATION_STATUS_RELEASED,
};
use myso_indexer_alt_social_schema::schema::{
    ai_credit_balances, ecosystem_treasury, memory_accounts, profile_badges, profile_config,
    profile_events, profiles, unified_revenue, username_listings, username_offers,
    username_registry, username_reservations, username_sale_fees, vesting_events, vesting_wallets,
};

use super::ai_credit;
use super::common;
use super::events;
use super::memory;
use super::organization_stats::{apply_org_revenue, resolve_organization_id_for_derived_address};
use super::profile;
use super::ProfileUpdate;
use crate::metrics::SocialMetrics;

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
    UsernameReservation(NewUsernameReservation),
    UsernameReservationRelease {
        username: String,
        reason: i16,
        released_by: String,
        released_at: i64,
        release_transaction_id: String,
    },
    ProfileUsernameSet {
        profile_id: String,
        username: String,
        owner_address: Option<String>,
    },
    ProfileEvent(NewProfileEvent),
    UsernameListing(NewUsernameListing),
    UsernameListingStatusUpdate {
        username: String,
        status: String,
        cancelled_at: Option<i64>,
        transaction_id: String,
    },
    UsernameOffer(NewUsernameOffer),
    UsernameSaleFee(NewUsernameSaleFee),
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
            crate::handlers::SocialEventRow::UsernameReservation(reservation) => {
                Some(ProfileRow::UsernameReservation(reservation))
            }
            crate::handlers::SocialEventRow::UsernameReservationRelease {
                username,
                reason,
                released_by,
                released_at,
                release_transaction_id,
            } => Some(ProfileRow::UsernameReservationRelease {
                username,
                reason,
                released_by,
                released_at,
                release_transaction_id,
            }),
            crate::handlers::SocialEventRow::ProfileUsernameSet {
                profile_id,
                username,
                owner_address,
            } => Some(ProfileRow::ProfileUsernameSet {
                profile_id,
                username,
                owner_address,
            }),
            crate::handlers::SocialEventRow::ProfileEvent(e) => Some(ProfileRow::ProfileEvent(e)),
            crate::handlers::SocialEventRow::UsernameListing(l) => {
                Some(ProfileRow::UsernameListing(l))
            }
            crate::handlers::SocialEventRow::UsernameListingStatusUpdate {
                username,
                status,
                cancelled_at,
                transaction_id,
            } => Some(ProfileRow::UsernameListingStatusUpdate {
                username,
                status,
                cancelled_at,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::UsernameOffer(o) => Some(ProfileRow::UsernameOffer(o)),
            crate::handlers::SocialEventRow::UsernameSaleFee(f) => {
                Some(ProfileRow::UsernameSaleFee(f))
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
                        Err(e) => {
                            tracing::warn!(
                                tx_digest = %tx_digest,
                                module,
                                event_name,
                                error = %e,
                                hex_preview = %e.contents_hex_preview(48),
                                "profiles pipeline: event contents parse failed; skipping event"
                            );
                            SocialMetrics::record_event_bcs_parse_failed(module, event_name);
                            continue;
                        }
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

/// Commit pass for registry/profile username rows. Marketplace offer/fee rows commit before
/// registry mutations so accept/reject snapshots persist even if registry writes fail on legacy DBs.
const COMMIT_PASS_MARKETPLACE: u8 = 2;
const COMMIT_PASS_REGISTRY_REASSIGN: u8 = 3;
const COMMIT_PASS_REGISTRY_UPSERT: u8 = 4;
const COMMIT_PASS_USERNAME_SET: u8 = 5;
const COMMIT_PASS_OTHER: u8 = 6;

fn profile_row_commit_pass(row: &ProfileRow) -> u8 {
    match row {
        ProfileRow::UsernameOffer(_)
        | ProfileRow::UsernameSaleFee(_)
        | ProfileRow::UsernameListing(_)
        | ProfileRow::UsernameListingStatusUpdate { .. }
        | ProfileRow::UsernameReservation(_)
        | ProfileRow::UsernameReservationRelease { .. } => COMMIT_PASS_MARKETPLACE,
        ProfileRow::UsernameRegistryReassign { .. } | ProfileRow::UsernameRegistryDelete { .. } => {
            COMMIT_PASS_REGISTRY_REASSIGN
        }
        ProfileRow::UsernameRegistryUpsert(_) => COMMIT_PASS_REGISTRY_UPSERT,
        ProfileRow::ProfileUsernameSet { .. } => COMMIT_PASS_USERNAME_SET,
        ProfileRow::Profile(_)
        | ProfileRow::MemoryAccountBootstrap(_)
        | ProfileRow::AiCreditBalanceBootstrap(_) => 0,
        _ => COMMIT_PASS_OTHER,
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
        for pass in [
            COMMIT_PASS_MARKETPLACE,
            COMMIT_PASS_REGISTRY_REASSIGN,
            COMMIT_PASS_REGISTRY_UPSERT,
            COMMIT_PASS_USERNAME_SET,
            COMMIT_PASS_OTHER,
        ] {
            for row in values {
                if profile_row_commit_pass(row) == pass {
                    total += commit_profile_row(row, conn).await?;
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

async fn load_latest_profile_config(conn: &mut Connection<'_>) -> Result<Option<NewProfileConfig>> {
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
            let updated = diesel::update(username_registry::table)
                .filter(username_registry::username.eq(username))
                .set((
                    username_registry::profile_id.eq(new_profile_id),
                    username_registry::transaction_id.eq(transaction_id),
                ))
                .execute(conn)
                .await?;
            if updated == 0 {
                tracing::warn!(
                    username = %username,
                    new_profile_id = %new_profile_id,
                    "UsernameRegistryReassign updated 0 rows"
                );
            }
            total += updated;
        }
        ProfileRow::ProfileUsernameSet {
            profile_id,
            username,
            owner_address,
        } => {
            let now = chrono::Utc::now().naive_utc();
            let profile_id_norm = common::normalize_hex_address(profile_id);
            // Username is NOT NULL + UNIQUE. Admin reassign emits destination Set before the
            // source Claimed rename; free any other holder first so idx_profiles_username allows
            // the transfer (park on a unique placeholder until their replacement Set lands).
            total += diesel::sql_query(
                "UPDATE profiles \
                 SET username = '__releasing__' || COALESCE(profile_id, owner_address, id::text), \
                     updated_at = $1 \
                 WHERE username = $2 \
                   AND COALESCE(profile_id, '') IS DISTINCT FROM $3",
            )
            .bind::<Timestamp, _>(now)
            .bind::<Text, _>(username)
            .bind::<Text, _>(&profile_id_norm)
            .execute(conn)
            .await?;
            let updated = if let Some(owner) = owner_address.as_ref() {
                let owner_norm = common::normalize_hex_address(owner);
                diesel::update(profiles::table)
                    .filter(
                        profiles::profile_id
                            .eq(&profile_id_norm)
                            .or(profiles::owner_address.eq(&owner_norm)),
                    )
                    .set((
                        profiles::username.eq(username),
                        profiles::updated_at.eq(now),
                    ))
                    .execute(conn)
                    .await?
            } else {
                diesel::update(profiles::table)
                    .filter(profiles::profile_id.eq(&profile_id_norm))
                    .set((
                        profiles::username.eq(username),
                        profiles::updated_at.eq(now),
                    ))
                    .execute(conn)
                    .await?
            };
            if updated == 0 {
                tracing::error!(
                    profile_id = %profile_id_norm,
                    username = %username,
                    owner_address = ?owner_address,
                    "ProfileUsernameSet updated 0 rows — profiles.username out of sync with registry"
                );
            }
            total += updated;
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
        ProfileRow::UsernameListing(listing) => {
            total += diesel::insert_into(username_listings::table)
                .values(listing)
                .execute(conn)
                .await?;
        }
        ProfileRow::UsernameReservation(reservation) => {
            let existing_tx: Option<String> = username_reservations::table
                .filter(username_reservations::username.eq(&reservation.username))
                .filter(username_reservations::reason.eq(reservation.reason))
                .filter(username_reservations::status.eq(USERNAME_RESERVATION_STATUS_ACTIVE))
                .select(username_reservations::reserve_transaction_id)
                .first(conn)
                .await
                .optional()?;
            if let Some(tx) = existing_tx {
                if tx == reservation.reserve_transaction_id {
                    // Idempotent replay of the same reserve event.
                } else {
                    tracing::warn!(
                        username = %reservation.username,
                        reason = reservation.reason,
                        existing_transaction_id = %tx,
                        new_transaction_id = %reservation.reserve_transaction_id,
                        "UsernameReservation active row already exists for username/reason"
                    );
                }
            } else {
                total += diesel::insert_into(username_reservations::table)
                    .values(reservation)
                    .execute(conn)
                    .await?;
            }
        }
        ProfileRow::UsernameReservationRelease {
            username,
            reason,
            released_by,
            released_at,
            release_transaction_id,
        } => {
            let updated = diesel::update(username_reservations::table)
                .filter(username_reservations::username.eq(username))
                .filter(username_reservations::reason.eq(*reason))
                .filter(username_reservations::status.eq(USERNAME_RESERVATION_STATUS_ACTIVE))
                .set((
                    username_reservations::status.eq(USERNAME_RESERVATION_STATUS_RELEASED),
                    username_reservations::released_by.eq(released_by),
                    username_reservations::released_at.eq(*released_at),
                    username_reservations::release_transaction_id.eq(release_transaction_id),
                ))
                .execute(conn)
                .await?;
            if updated == 0 {
                tracing::warn!(
                    username = %username,
                    reason = reason,
                    "UsernameReservationRelease updated 0 rows"
                );
            }
            total += updated;
        }
        ProfileRow::UsernameListingStatusUpdate {
            username,
            status,
            cancelled_at,
            transaction_id,
        } => {
            let _ = diesel::update(username_listings::table)
                .filter(username_listings::username.eq(username))
                .filter(username_listings::status.eq("active"))
                .set((
                    username_listings::status.eq(status),
                    username_listings::cancelled_at.eq(*cancelled_at),
                    username_listings::transaction_id.eq(transaction_id),
                ))
                .execute(conn)
                .await;
        }
        ProfileRow::UsernameOffer(offer) => {
            total += diesel::insert_into(username_offers::table)
                .values(offer)
                .execute(conn)
                .await?;
        }
        ProfileRow::UsernameSaleFee(fee) => {
            total += diesel::insert_into(username_sale_fees::table)
                .values(fee)
                .execute(conn)
                .await?;
            total += insert_username_marketplace_unified_revenue(conn, fee).await?;
            let seller_net = fee.sale_amount.saturating_sub(fee.fee_amount);
            if seller_net > 0 {
                let org_id =
                    resolve_organization_id_for_derived_address(conn, &fee.seller_address).await?;
                apply_org_revenue(
                    conn,
                    org_id.as_deref(),
                    seller_net,
                    Some(fee.buyer_address.as_str()),
                    fee.timestamp,
                )
                .await?;
            }
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

async fn insert_username_marketplace_unified_revenue(
    conn: &mut Connection<'_>,
    fee: &NewUsernameSaleFee,
) -> Result<usize> {
    let seller_net = fee.sale_amount.saturating_sub(fee.fee_amount);
    let mut total = 0usize;
    if seller_net > 0 {
        total += diesel::insert_into(unified_revenue::table)
            .values(NewUnifiedRevenue::from_username_marketplace(
                REVENUE_TYPE_USERNAME_MARKETPLACE_SELLER_NET.to_string(),
                fee.seller_address.clone(),
                None,
                seller_net,
                fee.username.clone(),
                fee.buyer_address.clone(),
                fee.seller_address.clone(),
                fee.timestamp,
                fee.transaction_id.clone(),
            ))
            .execute(conn)
            .await?;
    }
    if fee.fee_amount > 0 {
        total += diesel::insert_into(unified_revenue::table)
            .values(NewUnifiedRevenue::from_username_marketplace(
                REVENUE_TYPE_USERNAME_MARKETPLACE_ECOSYSTEM_FEE.to_string(),
                fee.seller_address.clone(),
                None,
                fee.fee_amount,
                fee.username.clone(),
                fee.buyer_address.clone(),
                fee.fee_recipient_address.clone(),
                fee.timestamp,
                fee.transaction_id.clone(),
            ))
            .execute(conn)
            .await?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use diesel::OptionalExtension;
    use move_core_types::account_address::AccountAddress;
    use myso_indexer_alt_framework::postgres::handler::Handler;
    use myso_indexer_alt_social_schema::models::{
        NewProfile, NewUsernameRegistry, NewUsernameSaleFee,
        REVENUE_TYPE_USERNAME_MARKETPLACE_ECOSYSTEM_FEE,
        REVENUE_TYPE_USERNAME_MARKETPLACE_SELLER_NET, USERNAME_RESERVATION_STATUS_ACTIVE,
        USERNAME_RESERVATION_STATUS_RELEASED,
    };
    use myso_indexer_alt_social_schema::schema::{
        profile_events, profiles, unified_revenue, username_registry, username_reservations,
    };
    use myso_indexer_alt_social_schema::MIGRATIONS;
    use myso_pg_db::temp::TempDb;
    use myso_pg_db::Db;

    use super::common;
    use super::profile;
    use super::profile_row_commit_pass;
    use super::ProfileRow;
    use super::ProfilesHandler;
    use super::COMMIT_PASS_MARKETPLACE;
    use super::COMMIT_PASS_OTHER;
    use super::COMMIT_PASS_REGISTRY_REASSIGN;
    use super::COMMIT_PASS_REGISTRY_UPSERT;
    use super::COMMIT_PASS_USERNAME_SET;

    fn addr(id: u8) -> AccountAddress {
        let mut bytes = [0u8; 32];
        bytes[31] = id;
        AccountAddress::new(bytes)
    }

    fn addr_hex(id: u8) -> String {
        addr(id).to_canonical_string(true)
    }

    fn sample_profile(owner: &str, profile_id: &str, username: &str) -> NewProfile {
        let now = DateTime::from_timestamp(1_700_000_000, 0)
            .unwrap()
            .naive_utc();
        NewProfile {
            owner_address: owner.to_string(),
            username: username.to_string(),
            display_name: None,
            bio: None,
            profile_photo: None,
            website: None,
            created_at: now,
            updated_at: now,
            cover_photo: None,
            profile_id: Some(profile_id.to_string()),
            followers_count: 0,
            following_count: 0,
            blocked_count: 0,
            post_count: 0,
            birthdate: None,
            location: None,
            x_username: None,
            social_proof_token_address: None,
            reservation_pool_address: None,
            selected_badge_id: None,
            selected_ecosystem_badge_id: None,
            memory_account_id: None,
            ai_credit_balance_id: None,
            contract_version: 0,
        }
    }

    async fn setup_temp_db() -> Option<Db> {
        let temp_db = TempDb::new().ok()?;
        let store = Db::for_write(temp_db.database().url().clone(), Default::default())
            .await
            .ok()?;
        {
            let mut probe = store.connect().await.ok()?;
            use diesel_async::RunQueryDsl;
            diesel::sql_query("CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE")
                .execute(&mut probe)
                .await
                .ok()?;
        }
        store.run_migrations(Some(&MIGRATIONS)).await.ok()?;
        Some(store)
    }

    #[test]
    fn username_sale_rows_use_registry_safe_commit_passes() {
        let seller_profile_id = addr_hex(1);
        let buyer_profile_id = addr_hex(2);
        let rows = vec![
            ProfileRow::UsernameListingStatusUpdate {
                username: "premium1".to_string(),
                status: "sold".to_string(),
                cancelled_at: None,
                transaction_id: "tx:0".to_string(),
            },
            ProfileRow::UsernameRegistryDelete {
                username: "buyer1".to_string(),
            },
            ProfileRow::UsernameRegistryReassign {
                username: "premium1".to_string(),
                new_profile_id: buyer_profile_id.clone(),
                transaction_id: "tx:0".to_string(),
            },
            ProfileRow::UsernameRegistryUpsert(NewUsernameRegistry {
                username: "seller1".to_string(),
                profile_id: seller_profile_id.clone(),
                transaction_id: "tx:0".to_string(),
            }),
            ProfileRow::ProfileUsernameSet {
                profile_id: seller_profile_id,
                username: "seller1".to_string(),
                owner_address: None,
            },
            ProfileRow::ProfileUsernameSet {
                profile_id: buyer_profile_id,
                username: "premium1".to_string(),
                owner_address: None,
            },
        ];
        let passes: Vec<u8> = rows.iter().map(profile_row_commit_pass).collect();
        let first_marketplace = passes
            .iter()
            .position(|pass| *pass == COMMIT_PASS_MARKETPLACE)
            .expect("marketplace pass");
        let first_reassign = passes
            .iter()
            .position(|pass| *pass == COMMIT_PASS_REGISTRY_REASSIGN)
            .expect("reassign pass");
        let first_upsert = passes
            .iter()
            .position(|pass| *pass == COMMIT_PASS_REGISTRY_UPSERT)
            .expect("upsert pass");
        let first_profile_set = passes
            .iter()
            .position(|pass| *pass == COMMIT_PASS_USERNAME_SET)
            .expect("profile username set pass");
        assert!(first_marketplace < first_reassign);
        assert!(first_reassign < first_upsert);
        assert!(first_upsert < first_profile_set);
    }

    #[test]
    fn username_sale_commit_passes_sets_after_registry() {
        assert!(COMMIT_PASS_REGISTRY_UPSERT < COMMIT_PASS_USERNAME_SET);
        assert!(COMMIT_PASS_USERNAME_SET < COMMIT_PASS_OTHER);
    }

    #[tokio::test]
    async fn username_sale_commit_maintains_one_username_per_profile() {
        let Some(store) = setup_temp_db().await else {
            return;
        };

        let seller_profile_id = addr_hex(11);
        let buyer_profile_id = addr_hex(12);
        let seller_owner = addr_hex(21);
        let buyer_owner = addr_hex(22);

        let mut conn = store.connect().await.expect("connection");
        diesel::insert_into(profiles::table)
            .values(sample_profile(
                &seller_owner,
                &seller_profile_id,
                "premium1",
            ))
            .execute(&mut conn)
            .await
            .expect("insert seller profile");
        diesel::insert_into(profiles::table)
            .values(sample_profile(&buyer_owner, &buyer_profile_id, "buyer1"))
            .execute(&mut conn)
            .await
            .expect("insert buyer profile");
        diesel::insert_into(username_registry::table)
            .values(NewUsernameRegistry {
                username: "premium1".to_string(),
                profile_id: seller_profile_id.clone(),
                transaction_id: "tx:seed:0".to_string(),
            })
            .execute(&mut conn)
            .await
            .expect("insert listed username");
        diesel::insert_into(username_registry::table)
            .values(NewUsernameRegistry {
                username: "buyer1".to_string(),
                profile_id: buyer_profile_id.clone(),
                transaction_id: "tx:seed:1".to_string(),
            })
            .execute(&mut conn)
            .await
            .expect("insert buyer username");

        let settle_json = serde_json::json!({
            "listed_username": "premium1",
            "replacement_username": "seller1",
            "seller": seller_owner,
            "seller_profile_id": seller_profile_id,
            "buyer": buyer_owner,
            "buyer_profile_id": buyer_profile_id,
            "amount": "5000000000",
            "settled_at": "1783238216000",
            "prior_buyer_username": "buyer1",
        });

        let mut rows = Vec::new();
        for row in
            profile::handle_profile_event("UsernameSaleSettledEvent", &settle_json, "tx:0", 0)
                .expect("settle handler")
        {
            if let Some(r) = ProfileRow::from_social(row) {
                rows.push(r);
            }
        }

        ProfilesHandler::commit(&rows, &mut conn)
            .await
            .expect("commit username sale rows");

        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel_async::RunQueryDsl;

        let listed_owner: String = username_registry::table
            .filter(username_registry::username.eq("premium1"))
            .select(username_registry::profile_id)
            .first(&mut conn)
            .await
            .expect("listed registry row");
        assert_eq!(listed_owner, buyer_profile_id);

        let replacement_owner: String = username_registry::table
            .filter(username_registry::username.eq("seller1"))
            .select(username_registry::profile_id)
            .first(&mut conn)
            .await
            .expect("replacement registry row");
        assert_eq!(replacement_owner, seller_profile_id);

        let buyer_prior: Option<String> = username_registry::table
            .filter(username_registry::username.eq("buyer1"))
            .select(username_registry::profile_id)
            .first(&mut conn)
            .await
            .optional()
            .expect("buyer prior lookup");
        assert!(
            buyer_prior.is_none(),
            "buyer prior username should be deleted from registry"
        );

        let settled_events: i64 = profile_events::table
            .filter(profile_events::event_type.eq("UsernameSaleSettled"))
            .count()
            .get_result(&mut conn)
            .await
            .expect("settled profile events");
        assert_eq!(settled_events, 1);

        let seller_username: String = profiles::table
            .filter(profiles::profile_id.eq(&seller_profile_id))
            .select(profiles::username)
            .first(&mut conn)
            .await
            .expect("seller profile username");
        assert_eq!(seller_username, "seller1");

        let buyer_username: String = profiles::table
            .filter(profiles::profile_id.eq(&buyer_profile_id))
            .select(profiles::username)
            .first(&mut conn)
            .await
            .expect("buyer profile username");
        assert_eq!(buyer_username, "premium1");
    }

    /// Single-profile admin rename: delete prior, upsert new name, set profiles.username.
    #[tokio::test]
    async fn username_reassign_commit_updates_single_profile() {
        let Some(store) = setup_temp_db().await else {
            return;
        };

        let profile_id = addr_hex(31);
        let other_profile_id = addr_hex(32);
        let owner = addr_hex(41);
        let other_owner = addr_hex(42);

        let mut conn = store.connect().await.expect("connection");
        diesel::insert_into(profiles::table)
            .values(sample_profile(&owner, &profile_id, "user2"))
            .execute(&mut conn)
            .await
            .expect("insert target profile");
        diesel::insert_into(profiles::table)
            .values(sample_profile(&other_owner, &other_profile_id, "user1"))
            .execute(&mut conn)
            .await
            .expect("insert other profile");
        diesel::insert_into(username_registry::table)
            .values(NewUsernameRegistry {
                username: "user2".to_string(),
                profile_id: profile_id.clone(),
                transaction_id: "tx:seed:0".to_string(),
            })
            .execute(&mut conn)
            .await
            .expect("insert prior username");
        diesel::insert_into(username_registry::table)
            .values(NewUsernameRegistry {
                username: "user1".to_string(),
                profile_id: other_profile_id.clone(),
                transaction_id: "tx:seed:1".to_string(),
            })
            .execute(&mut conn)
            .await
            .expect("insert other username");

        let reassign_json = serde_json::json!({
            "username": "brandnew",
            "profile_id": profile_id,
            "admin": "0x1",
            "reason_code": 2,
            "prior_username": "user2",
        });
        let checkpoint_timestamp_ms = 1_783_238_216_000;

        let mut rows = Vec::new();
        for row in profile::handle_profile_event(
            "UsernameReassignedEvent",
            &reassign_json,
            "tx:reassign:0",
            checkpoint_timestamp_ms,
        )
        .expect("handler")
        {
            if let Some(r) = ProfileRow::from_social(row) {
                rows.push(r);
            }
        }

        ProfilesHandler::commit(&rows, &mut conn)
            .await
            .expect("commit single-profile admin rename");

        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel_async::RunQueryDsl;

        let new_owner: String = username_registry::table
            .filter(username_registry::username.eq("brandnew"))
            .select(username_registry::profile_id)
            .first(&mut conn)
            .await
            .expect("new registry row");
        assert_eq!(new_owner, profile_id);

        let prior: Option<String> = username_registry::table
            .filter(username_registry::username.eq("user2"))
            .select(username_registry::profile_id)
            .first(&mut conn)
            .await
            .optional()
            .expect("prior lookup");
        assert!(
            prior.is_none(),
            "prior username should be deleted from registry"
        );

        let other_still: String = username_registry::table
            .filter(username_registry::username.eq("user1"))
            .select(username_registry::profile_id)
            .first(&mut conn)
            .await
            .expect("other registry row");
        assert_eq!(other_still, other_profile_id);

        let renamed: String = profiles::table
            .filter(profiles::profile_id.eq(&profile_id))
            .select(profiles::username)
            .first(&mut conn)
            .await
            .expect("renamed profile username");
        assert_eq!(renamed, "brandnew");

        let other_username: String = profiles::table
            .filter(profiles::profile_id.eq(&other_profile_id))
            .select(profiles::username)
            .first(&mut conn)
            .await
            .expect("other profile username");
        assert_eq!(other_username, "user1");

        let audit_events: Vec<(
            String,
            String,
            serde_json::Value,
            Option<String>,
            chrono::NaiveDateTime,
            chrono::NaiveDateTime,
        )> = profile_events::table
            .filter(profile_events::event_type.eq("UsernameReassigned"))
            .select((
                profile_events::event_type,
                profile_events::profile_id,
                profile_events::event_data,
                profile_events::event_id,
                profile_events::created_at,
                profile_events::updated_at,
            ))
            .load(&mut conn)
            .await
            .expect("reassign profile events");
        assert_eq!(audit_events.len(), 1);
        let (event_type, event_profile_id, event_data, event_id, created_at, updated_at) =
            &audit_events[0];
        let expected_time = common::chain_time_from_ms(checkpoint_timestamp_ms as i64).naive_utc();
        assert_eq!(event_type, "UsernameReassigned");
        assert_eq!(event_profile_id, &profile_id);
        assert_eq!(
            event_data,
            &serde_json::json!({
                "username": "brandnew",
                "profile_id": profile_id,
                "admin": "0x1",
                "reason_code": 2,
                "prior_username": "user2",
            })
        );
        assert_eq!(event_id.as_deref(), Some("tx:reassign:0"));
        assert_eq!(created_at, &expected_time);
        assert_eq!(updated_at, &expected_time);
    }

    #[tokio::test]
    async fn username_accept_tx_commit_updates_profiles_username() {
        let Some(store) = setup_temp_db().await else {
            return;
        };

        let seller_profile_id = addr_hex(11);
        let buyer_profile_id = addr_hex(12);
        let seller_owner = addr_hex(21);
        let buyer_owner = addr_hex(22);

        let mut conn = store.connect().await.expect("connection");
        diesel::insert_into(profiles::table)
            .values(sample_profile(
                &seller_owner,
                &seller_profile_id,
                "premium1",
            ))
            .execute(&mut conn)
            .await
            .expect("insert seller profile");
        diesel::insert_into(profiles::table)
            .values(sample_profile(&buyer_owner, &buyer_profile_id, "buyer1"))
            .execute(&mut conn)
            .await
            .expect("insert buyer profile");
        diesel::insert_into(username_registry::table)
            .values(NewUsernameRegistry {
                username: "premium1".to_string(),
                profile_id: seller_profile_id.clone(),
                transaction_id: "tx:seed:0".to_string(),
            })
            .execute(&mut conn)
            .await
            .expect("insert listed username");
        diesel::insert_into(username_registry::table)
            .values(NewUsernameRegistry {
                username: "buyer1".to_string(),
                profile_id: buyer_profile_id.clone(),
                transaction_id: "tx:seed:1".to_string(),
            })
            .execute(&mut conn)
            .await
            .expect("insert buyer username");

        let settle_json = serde_json::json!({
            "listed_username": "premium1",
            "replacement_username": "seller1",
            "seller": seller_owner,
            "seller_profile_id": seller_profile_id,
            "buyer": buyer_owner,
            "buyer_profile_id": buyer_profile_id,
            "amount": "5000000000",
            "settled_at": "1783238216000",
            "prior_buyer_username": "buyer1",
        });
        let accept_json = serde_json::json!({
            "username": "premium1",
            "replacement_username": "seller1",
            "seller": seller_owner,
            "seller_profile_id": seller_profile_id,
            "buyer": buyer_owner,
            "buyer_profile_id": buyer_profile_id,
            "amount": "5000000000",
            "accepted_at": "1783238216000",
        });

        let mut rows = Vec::new();
        for event in [
            ("UsernameSaleSettledEvent", &settle_json),
            ("UsernameOfferAcceptedEvent", &accept_json),
        ] {
            for row in profile::handle_profile_event(event.0, event.1, "tx:accept:0", 0)
                .unwrap_or_else(|| panic!("handler for {}", event.0))
            {
                if let Some(r) = ProfileRow::from_social(row) {
                    rows.push(r);
                }
            }
        }

        ProfilesHandler::commit(&rows, &mut conn)
            .await
            .expect("commit full accept tx rows");

        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel_async::RunQueryDsl;

        let seller_username: String = profiles::table
            .filter(profiles::profile_id.eq(&seller_profile_id))
            .select(profiles::username)
            .first(&mut conn)
            .await
            .expect("seller profile username");
        assert_eq!(seller_username, "seller1");

        let buyer_username: String = profiles::table
            .filter(profiles::profile_id.eq(&buyer_profile_id))
            .select(profiles::username)
            .first(&mut conn)
            .await
            .expect("buyer profile username");
        assert_eq!(buyer_username, "premium1");

        let buyer_prior: Option<String> = username_registry::table
            .filter(username_registry::username.eq("buyer1"))
            .select(username_registry::profile_id)
            .first(&mut conn)
            .await
            .optional()
            .expect("buyer prior lookup");
        assert!(
            buyer_prior.is_none(),
            "buyer prior username should be absent from username_registry after settle"
        );
    }

    #[tokio::test]
    async fn username_sale_fee_commit_inserts_unified_revenue_rows() {
        let Some(store) = setup_temp_db().await else {
            return;
        };

        let seller_owner = addr_hex(31);
        let treasury = addr_hex(99);
        let fee = NewUsernameSaleFee {
            username: "premium1".to_string(),
            seller_address: seller_owner.clone(),
            seller_profile_id: addr_hex(41),
            buyer_address: addr_hex(32),
            buyer_profile_id: addr_hex(42),
            sale_amount: 5_000_000_000,
            fee_amount: 250_000_000,
            fee_recipient_address: treasury.clone(),
            timestamp: 1_783_237_181_000,
            transaction_id: "tx:fee:0".to_string(),
        };

        let mut conn = store.connect().await.expect("connection");
        ProfilesHandler::commit(&[ProfileRow::UsernameSaleFee(fee)], &mut conn)
            .await
            .expect("commit sale fee");

        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel_async::RunQueryDsl;

        let seller_net: i64 = unified_revenue::table
            .filter(unified_revenue::recipient_address.eq(&seller_owner))
            .filter(unified_revenue::revenue_type.eq(REVENUE_TYPE_USERNAME_MARKETPLACE_SELLER_NET))
            .select(unified_revenue::amount)
            .first(&mut conn)
            .await
            .expect("seller net unified revenue");
        assert_eq!(seller_net, 4_750_000_000);

        let ecosystem_fee: i64 = unified_revenue::table
            .filter(unified_revenue::recipient_address.eq(&treasury))
            .filter(
                unified_revenue::revenue_type.eq(REVENUE_TYPE_USERNAME_MARKETPLACE_ECOSYSTEM_FEE),
            )
            .select(unified_revenue::amount)
            .first(&mut conn)
            .await
            .expect("ecosystem fee unified revenue");
        assert_eq!(ecosystem_fee, 250_000_000);

        let unified_row_count: i64 = unified_revenue::table
            .filter(unified_revenue::transaction_id.eq("tx:fee:0"))
            .count()
            .get_result(&mut conn)
            .await
            .expect("unified revenue row count");
        assert_eq!(unified_row_count, 2);
    }

    #[tokio::test]
    async fn username_reservation_commit_tracks_lock_state_without_registry_mutation() {
        let Some(store) = setup_temp_db().await else {
            return;
        };

        let profile_id = addr_hex(41);
        let owner = addr_hex(42);
        let reserved_by = addr_hex(43);
        let mut conn = store.connect().await.expect("connection");

        diesel::insert_into(profiles::table)
            .values(sample_profile(&owner, &profile_id, "creator1"))
            .execute(&mut conn)
            .await
            .expect("insert profile");

        let reserve_json = serde_json::json!({
            "username": "locked_name",
            "reason": 1,
            "reserved_by": reserved_by,
        });
        let release_json = serde_json::json!({
            "username": "locked_name",
            "reason": 1,
            "released_by": reserved_by,
        });

        let mut reserve_rows = Vec::new();
        for row in profile::handle_profile_event(
            "UsernameReservedEvent",
            &reserve_json,
            "tx:reserve:0",
            1_700_000_000_000,
        )
        .expect("reserve handler")
        {
            if let Some(r) = ProfileRow::from_social(row) {
                reserve_rows.push(r);
            }
        }
        ProfilesHandler::commit(&reserve_rows, &mut conn)
            .await
            .expect("commit reserve rows");

        use diesel::ExpressionMethods;
        use diesel::QueryDsl;
        use diesel_async::RunQueryDsl;

        let registry_count: i64 = username_registry::table
            .count()
            .get_result(&mut conn)
            .await
            .expect("registry count");
        assert_eq!(registry_count, 0);

        let profile_username: String = profiles::table
            .filter(profiles::profile_id.eq(&profile_id))
            .select(profiles::username)
            .first(&mut conn)
            .await
            .expect("profile username");
        assert_eq!(profile_username, "creator1");

        let (status, reserve_tx): (String, String) = username_reservations::table
            .filter(username_reservations::username.eq("locked_name"))
            .select((
                username_reservations::status,
                username_reservations::reserve_transaction_id,
            ))
            .first(&mut conn)
            .await
            .expect("active reservation");
        assert_eq!(status, USERNAME_RESERVATION_STATUS_ACTIVE);
        assert_eq!(reserve_tx, "tx:reserve:0");

        let audit_count: i64 = profile_events::table
            .filter(profile_events::event_type.eq("UsernameReserved"))
            .count()
            .get_result(&mut conn)
            .await
            .expect("reserve audit count");
        assert_eq!(audit_count, 1);

        let mut release_rows = Vec::new();
        for row in profile::handle_profile_event(
            "UsernameReleasedEvent",
            &release_json,
            "tx:release:0",
            1_700_000_001_000,
        )
        .expect("release handler")
        {
            if let Some(r) = ProfileRow::from_social(row) {
                release_rows.push(r);
            }
        }
        ProfilesHandler::commit(&release_rows, &mut conn)
            .await
            .expect("commit release rows");

        let (status, release_tx): (String, Option<String>) = username_reservations::table
            .filter(username_reservations::username.eq("locked_name"))
            .select((
                username_reservations::status,
                username_reservations::release_transaction_id,
            ))
            .first(&mut conn)
            .await
            .expect("released reservation");
        assert_eq!(status, USERNAME_RESERVATION_STATUS_RELEASED);
        assert_eq!(release_tx.as_deref(), Some("tx:release:0"));

        let release_audit_count: i64 = profile_events::table
            .filter(profile_events::event_type.eq("UsernameReleased"))
            .count()
            .get_result(&mut conn)
            .await
            .expect("release audit count");
        assert_eq!(release_audit_count, 1);
    }
}
