// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Upgrade pipeline: indexes upgrade module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{NewObjectMigratedEvent, NewUpgradeEvent};
use myso_indexer_alt_social_schema::schema::{
    ai_credit_balances, insurance_coverage_routes, insurance_policies, insurance_vaults,
    memory_accounts, object_migrated_events, platforms, posts, profiles, upgrade_events,
};

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

async fn update_latest_config_version<'a>(
    table: &str,
    new_version: i64,
    conn: &mut Connection<'a>,
) -> Result<()> {
    diesel::sql_query(format!(
        "UPDATE {table} SET version = $1 WHERE (id, time) = (
            SELECT id, time FROM {table} ORDER BY time DESC LIMIT 1
        )"
    ))
    .bind::<diesel::sql_types::BigInt, _>(new_version)
    .execute(conn)
    .await?;
    Ok(())
}

async fn fanout_object_migration<'a>(
    ev: &NewObjectMigratedEvent,
    conn: &mut Connection<'a>,
) -> Result<()> {
    let object_id = &ev.object_id;
    let new_version = ev.new_version;
    match ev.object_type.as_str() {
        "Platform" => {
            diesel::update(platforms::table.filter(platforms::platform_id.eq(object_id)))
                .set(platforms::version.eq(new_version))
                .execute(conn)
                .await?;
        }
        "AiCreditBalance" => {
            diesel::update(
                ai_credit_balances::table.filter(ai_credit_balances::balance_id.eq(object_id)),
            )
            .set(ai_credit_balances::contract_version.eq(new_version))
            .execute(conn)
            .await?;
        }
        "AiCreditConfig" => {
            update_latest_config_version("ai_credit_config", new_version, conn).await?;
        }
        "InsuranceConfig" => {
            update_latest_config_version("insurance_config", new_version, conn).await?;
        }
        "UnderwriterVault" => {
            diesel::update(insurance_vaults::table.filter(insurance_vaults::vault_id.eq(object_id)))
                .set(insurance_vaults::version.eq(new_version))
                .execute(conn)
                .await?;
        }
        "CoveragePolicy" => {
            diesel::update(
                insurance_policies::table.filter(insurance_policies::policy_id.eq(object_id)),
            )
            .set(insurance_policies::contract_version.eq(new_version))
            .execute(conn)
            .await?;
        }
        "CoverageRoute" => {
            diesel::update(
                insurance_coverage_routes::table
                    .filter(insurance_coverage_routes::route_id.eq(object_id)),
            )
            .set(insurance_coverage_routes::contract_version.eq(new_version))
            .execute(conn)
            .await?;
        }
        "MemoryAccount" => {
            diesel::update(
                memory_accounts::table.filter(memory_accounts::account_id.eq(object_id)),
            )
            .set(memory_accounts::contract_version.eq(new_version))
            .execute(conn)
            .await?;
        }
        "MemoryConfig" => {
            update_latest_config_version("memory_config", new_version, conn).await?;
        }
        "Profile" => {
            diesel::update(profiles::table.filter(profiles::profile_id.eq(object_id)))
                .set(profiles::contract_version.eq(new_version))
                .execute(conn)
                .await?;
        }
        "Post" => {
            diesel::update(posts::table.filter(posts::post_id.eq(object_id)))
                .set(posts::contract_version.eq(new_version))
                .execute(conn)
                .await?;
        }
        _ => {}
    }
    Ok(())
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
                    fanout_object_migration(ev, conn).await?;
                }
            }
        }
        Ok(total)
    }
}
