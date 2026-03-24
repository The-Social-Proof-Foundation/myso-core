// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Social graph pipeline: indexes social_graph module events.

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
use myso_indexer_alt_social_schema::models::{NewSocialGraphEvent, NewSocialGraphRelationship};
use myso_indexer_alt_social_schema::schema::{social_graph_events, social_graph_relationships};

use super::common;
use super::events;
use super::social_graph;

const SOCIAL_GRAPH_MODULES: &[&str] = &["social_graph"];

#[derive(Debug, Clone)]
pub enum SocialGraphRow {
    SocialGraphRelationship(NewSocialGraphRelationship),
    SocialGraphEvent(NewSocialGraphEvent),
    SocialGraphUnfollow {
        follower_address: String,
        following_address: String,
    },
}

impl SocialGraphRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::SocialGraphRelationship(rel) => {
                Some(SocialGraphRow::SocialGraphRelationship(rel))
            }
            crate::handlers::SocialEventRow::SocialGraphEvent(ev) => {
                Some(SocialGraphRow::SocialGraphEvent(ev))
            }
            crate::handlers::SocialEventRow::SocialGraphUnfollow {
                follower_address,
                following_address,
            } => Some(SocialGraphRow::SocialGraphUnfollow {
                follower_address,
                following_address,
            }),
            _ => None,
        }
    }
}

impl FieldCount for SocialGraphRow {
    const FIELD_COUNT: usize = 15;
}

pub struct SocialGraphHandler;

#[async_trait]
impl Processor for SocialGraphHandler {
    const NAME: &'static str = "social_graph";

    type Value = SocialGraphRow;

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
                if !SOCIAL_GRAPH_MODULES.contains(&module) {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data = match events::parse_event_contents(module, event_name, &ev.contents) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                if let Some(rows) =
                    social_graph::handle_social_graph_event(event_name, &event_data, &event_id)
                {
                    for row in rows {
                        if let Some(r) = SocialGraphRow::from_social(row) {
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
impl Handler for SocialGraphHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                SocialGraphRow::SocialGraphRelationship(rel) => {
                    total += diesel::insert_into(social_graph_relationships::table)
                        .values(rel)
                        .on_conflict((
                            social_graph_relationships::follower_address,
                            social_graph_relationships::following_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialGraphRow::SocialGraphEvent(ev) => {
                    total += diesel::insert_into(social_graph_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialGraphRow::SocialGraphUnfollow {
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
