// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Dedicated watermark pipeline for sub-agent registry rows (`sub_agents`, `sub_agent_events`).

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
use myso_indexer_alt_social_schema::models::{NewSubAgent, NewSubAgentEvent};
use myso_indexer_alt_social_schema::schema::{memory_accounts, sub_agent_events, sub_agents};

use super::common;
use super::events;
use super::memory::{self, json_str};
use crate::metrics::SocialMetrics;
use chrono::Utc;

const MEMORY_MODULE: &str = "memory";

const SUB_AGENT_REGISTRY_EVENTS: &[&str] = &[
    "SubAgentRegistered",
    "SubAgentUpdated",
    "SubAgentDeactivated",
    "SubAgentRevoked",
    "SubAgentsClearedOnTransfer",
];

#[derive(Debug, Clone)]
pub enum SubAgentRegistryRow {
    Upsert(NewSubAgent),
    Deactivate {
        agent_object_id: String,
        deactivated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    Revoke {
        agent_object_id: String,
        revoked_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    BulkClear {
        account_id: String,
        new_principal_owner: String,
        revoked_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    Audit(NewSubAgentEvent),
}

impl FieldCount for SubAgentRegistryRow {
    const FIELD_COUNT: usize = 8;
}

pub struct SubAgentRegistryHandler;

fn handle_sub_agent_registry_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SubAgentRegistryRow>> {
    let transaction_id = event_id.split(':').next().unwrap_or(event_id).to_string();
    let now = Utc::now();
    match event_name {
        "SubAgentRegistered" | "SubAgentUpdated" => {
            let row = memory::sub_agent_from_event(data, event_id, &transaction_id, now)?;
            let audit =
                memory::sub_agent_audit_event(event_name, data, event_id, &transaction_id, now);
            Some(vec![
                SubAgentRegistryRow::Upsert(row),
                SubAgentRegistryRow::Audit(audit),
            ])
        }
        "SubAgentDeactivated" => {
            let agent_object_id = json_str(data, "agent_object_id")?;
            let audit =
                memory::sub_agent_audit_event(event_name, data, event_id, &transaction_id, now);
            Some(vec![
                SubAgentRegistryRow::Deactivate {
                    agent_object_id,
                    deactivated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id,
                },
                SubAgentRegistryRow::Audit(audit),
            ])
        }
        "SubAgentRevoked" => {
            let agent_object_id = json_str(data, "agent_object_id")?;
            let audit =
                memory::sub_agent_audit_event(event_name, data, event_id, &transaction_id, now);
            Some(vec![
                SubAgentRegistryRow::Revoke {
                    agent_object_id,
                    revoked_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id,
                },
                SubAgentRegistryRow::Audit(audit),
            ])
        }
        "SubAgentsClearedOnTransfer" => {
            let account_id = json_str(data, "account_id")?;
            let new_owner = json_str(data, "new_owner")?;
            let audit =
                memory::sub_agent_audit_event(event_name, data, event_id, &transaction_id, now);
            Some(vec![
                SubAgentRegistryRow::BulkClear {
                    account_id,
                    new_principal_owner: new_owner,
                    revoked_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id,
                },
                SubAgentRegistryRow::Audit(audit),
            ])
        }
        _ => None,
    }
}

#[async_trait]
impl Processor for SubAgentRegistryHandler {
    const NAME: &'static str = "sub_agent_registry";

    type Value = SubAgentRegistryRow;

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
                if ev.type_.module.as_str() != MEMORY_MODULE {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                if !SUB_AGENT_REGISTRY_EVENTS.contains(&event_name) {
                    continue;
                }
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data =
                    match events::parse_event_contents(MEMORY_MODULE, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(e) => {
                            tracing::warn!(
                                tx_digest = %tx_digest,
                                module = MEMORY_MODULE,
                                event_name,
                                error = %e,
                                "sub_agent_registry: event parse failed; skipping"
                            );
                            SocialMetrics::record_event_bcs_parse_failed(MEMORY_MODULE, event_name);
                            continue;
                        }
                    };
                if let Some(rows) =
                    handle_sub_agent_registry_event(event_name, &event_data, &event_id)
                {
                    values.extend(rows);
                }
            }
        }
        Ok(values)
    }
}

#[async_trait]
impl Handler for SubAgentRegistryHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                SubAgentRegistryRow::Upsert(s) => {
                    let organization_id = s.organization_id.clone();
                    let label = s.label.clone();
                    let identity_class = s.identity_class;
                    let role_tags = s.role_tags;
                    let capabilities = s.capabilities;
                    let delegatable_caps = s.delegatable_caps;
                    let register_scope = s.register_scope;
                    let approval_required_caps = s.approval_required_caps;
                    let max_action_spend = s.max_action_spend;
                    let platform_scope = s.platform_scope.clone();
                    let parent_object_id = s.parent_object_id.clone();
                    let depth = s.depth;
                    let registered_by = s.registered_by.clone();
                    let expires_at_ms = s.expires_at_ms;
                    let active = s.active;
                    let updated_at_ms = s.updated_at_ms;
                    let event_id = s.event_id.clone();
                    let transaction_id = s.transaction_id.clone();
                    let time = s.time;
                    total += diesel::insert_into(sub_agents::table)
                        .values(s)
                        .on_conflict(sub_agents::agent_object_id)
                        .do_update()
                        .set((
                            sub_agents::derived_address.eq(s.derived_address.clone()),
                            sub_agents::account_id.eq(s.account_id.clone()),
                            sub_agents::organization_id.eq(organization_id),
                            sub_agents::label.eq(label),
                            sub_agents::identity_class.eq(identity_class),
                            sub_agents::role_tags.eq(role_tags),
                            sub_agents::capabilities.eq(capabilities),
                            sub_agents::delegatable_caps.eq(delegatable_caps),
                            sub_agents::register_scope.eq(register_scope),
                            sub_agents::approval_required_caps.eq(approval_required_caps),
                            sub_agents::max_action_spend.eq(max_action_spend),
                            sub_agents::platform_scope.eq(platform_scope),
                            sub_agents::parent_object_id.eq(parent_object_id),
                            sub_agents::depth.eq(depth),
                            sub_agents::registered_by.eq(registered_by),
                            sub_agents::expires_at_ms.eq(expires_at_ms),
                            sub_agents::active.eq(active),
                            sub_agents::updated_at_ms.eq(updated_at_ms),
                            sub_agents::event_id.eq(event_id),
                            sub_agents::transaction_id.eq(transaction_id),
                            sub_agents::time.eq(time),
                            sub_agents::deactivated_at_ms.eq(None::<i64>),
                            sub_agents::revoked_at_ms.eq(None::<i64>),
                        ))
                        .execute(conn)
                        .await?;
                }
                SubAgentRegistryRow::Deactivate {
                    agent_object_id,
                    deactivated_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        sub_agents::table.filter(sub_agents::agent_object_id.eq(agent_object_id)),
                    )
                    .set((
                        sub_agents::active.eq(false),
                        sub_agents::deactivated_at_ms.eq(*deactivated_at_ms),
                        sub_agents::updated_at_ms.eq(*deactivated_at_ms),
                        sub_agents::event_id.eq(event_id),
                        sub_agents::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                SubAgentRegistryRow::Revoke {
                    agent_object_id,
                    revoked_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        sub_agents::table.filter(sub_agents::agent_object_id.eq(agent_object_id)),
                    )
                    .set((
                        sub_agents::active.eq(false),
                        sub_agents::revoked_at_ms.eq(*revoked_at_ms),
                        sub_agents::updated_at_ms.eq(*revoked_at_ms),
                        sub_agents::event_id.eq(event_id),
                        sub_agents::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                SubAgentRegistryRow::BulkClear {
                    account_id,
                    new_principal_owner,
                    revoked_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        sub_agents::table
                            .filter(sub_agents::account_id.eq(account_id))
                            .filter(sub_agents::active.eq(true)),
                    )
                    .set((
                        sub_agents::active.eq(false),
                        sub_agents::revoked_at_ms.eq(*revoked_at_ms),
                        sub_agents::updated_at_ms.eq(*revoked_at_ms),
                        sub_agents::event_id.eq(event_id),
                        sub_agents::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    total += diesel::update(
                        memory_accounts::table.filter(memory_accounts::account_id.eq(account_id)),
                    )
                    .set(memory_accounts::principal_owner.eq(new_principal_owner))
                    .execute(conn)
                    .await?;
                }
                SubAgentRegistryRow::Audit(e) => {
                    total += diesel::insert_into(sub_agent_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_agent_registered_produces_upsert_and_audit_rows() {
        let data = serde_json::json!({
            "agent_object_id": "0xagent",
            "derived_address": "0xderived",
            "account_id": "0xaccount",
            "label": "bot",
            "registered_by": "0xowner",
            "active": true,
        });
        let rows = handle_sub_agent_registry_event("SubAgentRegistered", &data, "tx-digest:1")
            .expect("SubAgentRegistered should produce rows");
        assert_eq!(rows.len(), 2);
        match (&rows[0], &rows[1]) {
            (SubAgentRegistryRow::Upsert(s), SubAgentRegistryRow::Audit(e))
            | (SubAgentRegistryRow::Audit(e), SubAgentRegistryRow::Upsert(s)) => {
                assert_eq!(s.agent_object_id, "0xagent");
                assert_eq!(e.event_type, "SubAgentRegistered");
            }
            _ => panic!("expected Upsert and Audit rows"),
        }
    }

    #[test]
    fn sub_agent_deactivated_produces_deactivate_and_audit_rows() {
        let data = serde_json::json!({
            "agent_object_id": "0xagent",
            "account_id": "0xaccount",
        });
        let rows = handle_sub_agent_registry_event("SubAgentDeactivated", &data, "tx-digest:2")
            .expect("SubAgentDeactivated should produce rows");
        assert_eq!(rows.len(), 2);
        match (&rows[0], &rows[1]) {
            (
                SubAgentRegistryRow::Deactivate {
                    agent_object_id, ..
                },
                SubAgentRegistryRow::Audit(e),
            )
            | (
                SubAgentRegistryRow::Audit(e),
                SubAgentRegistryRow::Deactivate {
                    agent_object_id, ..
                },
            ) => {
                assert_eq!(agent_object_id, "0xagent");
                assert_eq!(e.event_type, "SubAgentDeactivated");
            }
            _ => panic!("expected Deactivate and Audit rows"),
        }
    }

    #[test]
    fn sub_agents_cleared_on_transfer_produces_bulk_clear_and_audit_rows() {
        let data = serde_json::json!({
            "account_id": "0xaccount",
            "new_owner": "0xnew_owner",
        });
        let rows =
            handle_sub_agent_registry_event("SubAgentsClearedOnTransfer", &data, "tx-digest:3")
                .expect("SubAgentsClearedOnTransfer should produce rows");
        assert_eq!(rows.len(), 2);
        match (&rows[0], &rows[1]) {
            (
                SubAgentRegistryRow::BulkClear {
                    account_id,
                    new_principal_owner,
                    ..
                },
                SubAgentRegistryRow::Audit(e),
            )
            | (
                SubAgentRegistryRow::Audit(e),
                SubAgentRegistryRow::BulkClear {
                    account_id,
                    new_principal_owner,
                    ..
                },
            ) => {
                assert_eq!(account_id, "0xaccount");
                assert_eq!(new_principal_owner, "0xnew_owner");
                assert_eq!(e.event_type, "SubAgentsClearedOnTransfer");
            }
            _ => panic!("expected BulkClear and Audit rows"),
        }
    }
}
