// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Upgrade pipeline: indexes upgrade module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{NewObjectMigratedEvent, NewUpgradeEvent};
use myso_indexer_alt_social_schema::schema::{object_migrated_events, upgrade_events};

use super::common;
use super::events;
use super::upgrade;

const UPGRADE_MODULES: &[&str] = &["upgrade"];

#[derive(Debug, Clone)]
pub enum UpgradeRow {
    UpgradeEvent(NewUpgradeEvent),
    ObjectMigratedEvent(NewObjectMigratedEvent),
}

impl UpgradeRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::UpgradeEvent(ev) => Some(UpgradeRow::UpgradeEvent(ev)),
            crate::handlers::SocialEventRow::ObjectMigratedEvent(ev) => {
                Some(UpgradeRow::ObjectMigratedEvent(ev))
            }
            _ => None,
        }
    }
}

impl FieldCount for UpgradeRow {
    const FIELD_COUNT: usize = 20;
}

pub struct UpgradeHandler;

#[async_trait]
impl Processor for UpgradeHandler {
    const NAME: &'static str = "upgrade";

    type Value = UpgradeRow;

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
                if !UPGRADE_MODULES.contains(&module) {
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
                    upgrade::handle_upgrade_event(event_name, &event_data, &event_id)
                {
                    for row in rows {
                        if let Some(r) = UpgradeRow::from_social(row) {
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
impl Handler for UpgradeHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                UpgradeRow::UpgradeEvent(ev) => {
                    total += diesel::insert_into(upgrade_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                UpgradeRow::ObjectMigratedEvent(ev) => {
                    total += diesel::insert_into(object_migrated_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
