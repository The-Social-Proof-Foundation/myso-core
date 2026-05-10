// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Platform pipeline: indexes platform module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::dsl::count_star;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewPlatform, NewPlatformBlockedProfile, NewPlatformEvent, NewPlatformMembership,
    NewPlatformModerator, NewPlatformModeratorPermission, NewPlatformTokenAirdrop,
};
use myso_indexer_alt_social_schema::schema::{
    platform_blocked_profiles, platform_events, platform_memberships,
    platform_moderator_permissions, platform_moderators, platform_token_airdrops, platforms,
};

use super::common;
use super::events;
use super::platform;

const PLATFORM_MODULES: &[&str] = &["platform"];

#[derive(Debug, Clone)]
pub enum PlatformRow {
    Platform(NewPlatform),
    PlatformUpdate {
        platform_id: String,
        name: String,
        tagline: String,
        description: Option<String>,
        terms_of_service: Option<String>,
        privacy_policy: Option<String>,
        platform_names: Option<serde_json::Value>,
        links: Option<serde_json::Value>,
        status: i16,
        release_date: Option<String>,
        shutdown_date: Option<String>,
        updated_at: chrono::NaiveDateTime,
        primary_category: String,
        secondary_category: Option<String>,
    },
    PlatformApprovalChange {
        platform_id: String,
        is_approved: bool,
        approved_by: String,
        changed_at: chrono::NaiveDateTime,
    },
    ModeratorPermissionsUpdated {
        platform_id: String,
        moderator_address: String,
        granted: Vec<String>,
        revoked: Vec<String>,
        updated_by: String,
        changed_at: chrono::NaiveDateTime,
    },
    PlatformBlockedProfile(NewPlatformBlockedProfile),
    PlatformBlockedProfileRemove {
        platform_id: String,
        wallet_address: String,
    },
    PlatformMembership(NewPlatformMembership),
    PlatformMembershipRemove {
        platform_id: String,
        wallet_address: String,
    },
    PlatformTokenAirdrop(NewPlatformTokenAirdrop),
    PlatformEvent(NewPlatformEvent),
    PlatformDeleted {
        platform_id: String,
        deleted_at: chrono::NaiveDateTime,
    },
}

impl PlatformRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::Platform(p) => Some(PlatformRow::Platform(p)),
            crate::handlers::SocialEventRow::PlatformUpdate {
                platform_id,
                name,
                tagline,
                description,
                terms_of_service,
                privacy_policy,
                platform_names,
                links,
                status,
                release_date,
                shutdown_date,
                updated_at,
                primary_category,
                secondary_category,
            } => Some(PlatformRow::PlatformUpdate {
                platform_id,
                name,
                tagline,
                description,
                terms_of_service,
                privacy_policy,
                platform_names,
                links,
                status,
                release_date,
                shutdown_date,
                updated_at,
                primary_category,
                secondary_category,
            }),
            crate::handlers::SocialEventRow::PlatformApprovalChange {
                platform_id,
                is_approved,
                approved_by,
                changed_at,
            } => Some(PlatformRow::PlatformApprovalChange {
                platform_id,
                is_approved,
                approved_by,
                changed_at,
            }),
            crate::handlers::SocialEventRow::ModeratorPermissionsUpdated {
                platform_id,
                moderator_address,
                granted,
                revoked,
                updated_by,
                changed_at,
            } => Some(PlatformRow::ModeratorPermissionsUpdated {
                platform_id,
                moderator_address,
                granted,
                revoked,
                updated_by,
                changed_at,
            }),
            crate::handlers::SocialEventRow::PlatformBlockedProfile(b) => {
                Some(PlatformRow::PlatformBlockedProfile(b))
            }
            crate::handlers::SocialEventRow::PlatformBlockedProfileRemove {
                platform_id,
                wallet_address,
            } => Some(PlatformRow::PlatformBlockedProfileRemove {
                platform_id,
                wallet_address,
            }),
            crate::handlers::SocialEventRow::PlatformMembership(m) => {
                Some(PlatformRow::PlatformMembership(m))
            }
            crate::handlers::SocialEventRow::PlatformMembershipRemove {
                platform_id,
                wallet_address,
            } => Some(PlatformRow::PlatformMembershipRemove {
                platform_id,
                wallet_address,
            }),
            crate::handlers::SocialEventRow::PlatformTokenAirdrop(a) => {
                Some(PlatformRow::PlatformTokenAirdrop(a))
            }
            crate::handlers::SocialEventRow::PlatformEvent(e) => {
                Some(PlatformRow::PlatformEvent(e))
            }
            crate::handlers::SocialEventRow::PlatformDeleted {
                platform_id,
                deleted_at,
            } => Some(PlatformRow::PlatformDeleted {
                platform_id,
                deleted_at,
            }),
            _ => None,
        }
    }
}

impl FieldCount for PlatformRow {
    const FIELD_COUNT: usize = 25;
}

pub struct PlatformHandler;

#[async_trait]
impl Processor for PlatformHandler {
    const NAME: &'static str = "platform";

    type Value = PlatformRow;

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
                if !PLATFORM_MODULES.contains(&module) {
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
                    platform::handle_platform_event(event_name, &event_data, &event_id)
                {
                    for row in rows {
                        if let Some(r) = PlatformRow::from_social(row) {
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
impl Handler for PlatformHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                PlatformRow::Platform(p) => {
                    total += diesel::insert_into(platforms::table)
                        .values(p)
                        .on_conflict(platforms::platform_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                PlatformRow::PlatformUpdate {
                    platform_id,
                    name,
                    tagline,
                    description,
                    terms_of_service,
                    privacy_policy,
                    platform_names,
                    links,
                    status,
                    release_date,
                    shutdown_date,
                    updated_at,
                    primary_category,
                    secondary_category,
                } => {
                    total += diesel::update(platforms::table)
                        .filter(platforms::platform_id.eq(platform_id))
                        .set((
                            platforms::name.eq(name),
                            platforms::tagline.eq(tagline),
                            platforms::description.eq(description),
                            platforms::terms_of_service.eq(terms_of_service),
                            platforms::privacy_policy.eq(privacy_policy),
                            platforms::platform_names.eq(platform_names),
                            platforms::links.eq(links),
                            platforms::status.eq(status),
                            platforms::release_date.eq(release_date),
                            platforms::shutdown_date.eq(shutdown_date),
                            platforms::updated_at.eq(updated_at),
                            platforms::primary_category.eq(primary_category),
                            platforms::secondary_category.eq(secondary_category),
                        ))
                        .execute(conn)
                        .await?;
                }
                PlatformRow::PlatformApprovalChange {
                    platform_id,
                    is_approved,
                    approved_by,
                    changed_at,
                } => {
                    total += diesel::update(platforms::table)
                        .filter(platforms::platform_id.eq(platform_id))
                        .set((
                            platforms::is_approved.eq(is_approved),
                            platforms::approval_changed_at.eq(Some(changed_at)),
                            platforms::approved_by.eq(Some(approved_by)),
                            platforms::updated_at.eq(changed_at),
                        ))
                        .execute(conn)
                        .await?;
                }
                PlatformRow::ModeratorPermissionsUpdated {
                    platform_id,
                    moderator_address,
                    granted,
                    revoked,
                    updated_by,
                    changed_at,
                } => {
                    if !revoked.is_empty() {
                        total += diesel::delete(platform_moderator_permissions::table)
                            .filter(platform_moderator_permissions::platform_id.eq(platform_id))
                            .filter(
                                platform_moderator_permissions::moderator_address
                                    .eq(moderator_address),
                            )
                            .filter(platform_moderator_permissions::permission.eq_any(revoked))
                            .execute(conn)
                            .await?;
                    }
                    for perm in granted {
                        total += diesel::insert_into(platform_moderator_permissions::table)
                            .values(NewPlatformModeratorPermission {
                                platform_id: platform_id.clone(),
                                moderator_address: moderator_address.clone(),
                                permission: perm.clone(),
                                created_at: *changed_at,
                            })
                            .on_conflict((
                                platform_moderator_permissions::platform_id,
                                platform_moderator_permissions::moderator_address,
                                platform_moderator_permissions::permission,
                            ))
                            .do_nothing()
                            .execute(conn)
                            .await?;
                    }
                    let remaining: i64 = platform_moderator_permissions::table
                        .filter(platform_moderator_permissions::platform_id.eq(platform_id))
                        .filter(
                            platform_moderator_permissions::moderator_address.eq(moderator_address),
                        )
                        .select(count_star())
                        .first(conn)
                        .await?;
                    if remaining == 0 {
                        total += diesel::delete(platform_moderators::table)
                            .filter(platform_moderators::platform_id.eq(platform_id))
                            .filter(platform_moderators::moderator_address.eq(moderator_address))
                            .execute(conn)
                            .await?;
                    } else {
                        total += diesel::insert_into(platform_moderators::table)
                            .values(NewPlatformModerator {
                                platform_id: platform_id.clone(),
                                moderator_address: moderator_address.clone(),
                                added_by: updated_by.clone(),
                                created_at: *changed_at,
                            })
                            .on_conflict((
                                platform_moderators::platform_id,
                                platform_moderators::moderator_address,
                            ))
                            .do_update()
                            .set(platform_moderators::updated_at.eq(Some(*changed_at)))
                            .execute(conn)
                            .await?;
                    }
                }
                PlatformRow::PlatformBlockedProfile(b) => {
                    total += diesel::insert_into(platform_blocked_profiles::table)
                        .values(b)
                        .on_conflict((
                            platform_blocked_profiles::platform_id,
                            platform_blocked_profiles::wallet_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                PlatformRow::PlatformBlockedProfileRemove {
                    platform_id,
                    wallet_address,
                } => {
                    let _ = diesel::delete(platform_blocked_profiles::table)
                        .filter(platform_blocked_profiles::platform_id.eq(platform_id))
                        .filter(platform_blocked_profiles::wallet_address.eq(wallet_address))
                        .execute(conn)
                        .await;
                }
                PlatformRow::PlatformMembership(m) => {
                    total += diesel::insert_into(platform_memberships::table)
                        .values(m)
                        .on_conflict((
                            platform_memberships::platform_id,
                            platform_memberships::wallet_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                PlatformRow::PlatformMembershipRemove {
                    platform_id,
                    wallet_address,
                } => {
                    let _ = diesel::delete(platform_memberships::table)
                        .filter(platform_memberships::platform_id.eq(platform_id))
                        .filter(platform_memberships::wallet_address.eq(wallet_address))
                        .execute(conn)
                        .await;
                }
                PlatformRow::PlatformTokenAirdrop(a) => {
                    total += diesel::insert_into(platform_token_airdrops::table)
                        .values(a)
                        .execute(conn)
                        .await?;
                }
                PlatformRow::PlatformEvent(e) => {
                    total += diesel::insert_into(platform_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                PlatformRow::PlatformDeleted {
                    platform_id,
                    deleted_at,
                } => {
                    total += diesel::update(platforms::table)
                        .filter(platforms::platform_id.eq(platform_id))
                        .set((
                            platforms::deleted_at.eq(Some(deleted_at)),
                            platforms::updated_at.eq(deleted_at),
                        ))
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
