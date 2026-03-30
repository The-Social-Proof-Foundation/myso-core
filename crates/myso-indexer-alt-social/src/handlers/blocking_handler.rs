// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Blocking pipeline: indexes block_list and blocking module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{NewBlockedEvent, NewBlockedProfile, NewProfileEvent};
use myso_indexer_alt_social_schema::schema::{
    blocked_events, blocked_profiles, profile_events, social_graph_relationships,
};

use super::common;
use super::events;

const BLOCKING_MODULES: &[&str] = &["block_list", "blocking"];

#[derive(Debug, Clone)]
pub enum BlockingRow {
    BlockedEvent(NewBlockedEvent),
    BlockedProfile(NewBlockedProfile),
    BlockedProfileDelete {
        blocker_address: String,
        blocked_address: String,
    },
    ProfileEvent(NewProfileEvent),
    SocialGraphUnfollow {
        follower_address: String,
        following_address: String,
    },
}

impl BlockingRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::BlockedEvent(ev) => {
                Some(BlockingRow::BlockedEvent(ev))
            }
            crate::handlers::SocialEventRow::BlockedProfile(bp) => {
                Some(BlockingRow::BlockedProfile(bp))
            }
            crate::handlers::SocialEventRow::BlockedProfileDelete {
                blocker_address,
                blocked_address,
            } => Some(BlockingRow::BlockedProfileDelete {
                blocker_address,
                blocked_address,
            }),
            crate::handlers::SocialEventRow::ProfileEvent(ev) => {
                Some(BlockingRow::ProfileEvent(ev))
            }
            crate::handlers::SocialEventRow::SocialGraphUnfollow {
                follower_address,
                following_address,
            } => Some(BlockingRow::SocialGraphUnfollow {
                follower_address,
                following_address,
            }),
            _ => None,
        }
    }
}

impl FieldCount for BlockingRow {
    const FIELD_COUNT: usize = 30;
}

pub struct BlockingHandler;

#[async_trait]
impl Processor for BlockingHandler {
    const NAME: &'static str = "blocking";

    type Value = BlockingRow;

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
                if !BLOCKING_MODULES.contains(&module) {
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
                    super::blocking::handle_blocking_event(event_name, &event_data, &event_id)
                {
                    for row in rows {
                        if let Some(r) = BlockingRow::from_social(row) {
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
impl Handler for BlockingHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                BlockingRow::BlockedEvent(ev) => {
                    total += diesel::insert_into(blocked_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                BlockingRow::BlockedProfile(bp) => {
                    let last_blocked_at = bp.last_blocked_at;
                    let blocked_profile_id = bp.blocked_profile_id.clone();
                    let blocked_username = bp.blocked_username.clone();
                    let blocked_display_name = bp.blocked_display_name.clone();
                    let blocked_profile_photo = bp.blocked_profile_photo.clone();
                    total += diesel::insert_into(blocked_profiles::table)
                        .values(bp)
                        .on_conflict((
                            blocked_profiles::blocker_address,
                            blocked_profiles::blocked_address,
                        ))
                        .do_update()
                        .set((
                            blocked_profiles::blocked_profile_id.eq(blocked_profile_id),
                            blocked_profiles::blocked_username.eq(blocked_username),
                            blocked_profiles::blocked_display_name.eq(blocked_display_name),
                            blocked_profiles::blocked_profile_photo.eq(blocked_profile_photo),
                            blocked_profiles::last_blocked_at.eq(last_blocked_at),
                            blocked_profiles::total_block_count
                                .eq(blocked_profiles::total_block_count + 1),
                        ))
                        .execute(conn)
                        .await?;
                }
                BlockingRow::BlockedProfileDelete {
                    blocker_address,
                    blocked_address,
                } => {
                    total += diesel::delete(blocked_profiles::table)
                        .filter(blocked_profiles::blocker_address.eq(blocker_address))
                        .filter(blocked_profiles::blocked_address.eq(blocked_address))
                        .execute(conn)
                        .await?;
                }
                BlockingRow::ProfileEvent(ev) => {
                    total += diesel::insert_into(profile_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                BlockingRow::SocialGraphUnfollow {
                    follower_address,
                    following_address,
                } => {
                    total += diesel::delete(social_graph_relationships::table)
                        .filter(social_graph_relationships::follower_address.eq(follower_address))
                        .filter(social_graph_relationships::following_address.eq(following_address))
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
