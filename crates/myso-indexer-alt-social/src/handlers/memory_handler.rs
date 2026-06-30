// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Memory module pipeline: memory accounts, org stats, vaults, and migration audit events.
//!
//! Sub-agent registry state (`sub_agents`) is indexed by [`SubAgentRegistryHandler`].
//! This pipeline still writes migration audit rows to `sub_agent_events` when applicable.

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
use myso_indexer_alt_social_schema::models::{
    NewAgentMemoryVault, NewAgenticOrganization, NewMemoryAccount, NewOrganizationEvent,
    NewSubAgentEvent,
};
use myso_indexer_alt_social_schema::schema::{
    memory_accounts, profiles, sub_agent_events, sub_agent_memory_vaults,
    sub_agent_organization_events, sub_agent_organizations, sub_agents,
};

use super::common;
use super::events;
use super::memory;
use super::organization_stats::{
    apply_sub_agent_active_delta, apply_sub_agent_registration_stats, init_org_stats,
    set_org_root_agent,
};
use crate::metrics::SocialMetrics;

const MEMORY_MODULE: &str = "memory";

#[derive(Debug, Clone)]
pub enum MemoryRow {
    MemoryAccount(NewMemoryAccount),
    ProfileMemoryAccountLink {
        profile_id: String,
        memory_account_id: String,
    },
    MemoryAccountActiveUpdate {
        account_id: String,
        active: bool,
    },
    AgentMemoryVault(NewAgentMemoryVault),
    SubAgentEvent(NewSubAgentEvent),
    AgenticOrganizationUpsert(NewAgenticOrganization),
    AgenticOrganizationMetadataUpdate {
        organization_id: String,
        name: Option<String>,
        description: Option<String>,
    },
    AgenticOrganizationCategoryUpdate {
        organization_id: String,
        org_type: i16,
        previous_org_type: i16,
        updated_at_ms: i64,
    },
    AgenticOrganizationDeactivate {
        organization_id: String,
        deactivated_at_ms: i64,
    },
    OrganizationEvent(NewOrganizationEvent),
    OrganizationStatsInit {
        organization_id: String,
        activity_at_ms: i64,
    },
    OrganizationAgentRegistered {
        organization_id: String,
        active: bool,
        depth: i16,
        parent_object_id: Option<String>,
        agent_object_id: String,
        activity_at_ms: i64,
    },
    OrganizationAgentActiveDelta {
        agent_object_id: String,
        active_delta: i32,
        activity_at_ms: i64,
    },
}

impl MemoryRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::MemoryAccount(a) => Some(MemoryRow::MemoryAccount(a)),
            crate::handlers::SocialEventRow::ProfileMemoryAccountLink {
                profile_id,
                memory_account_id,
            } => Some(MemoryRow::ProfileMemoryAccountLink {
                profile_id,
                memory_account_id,
            }),
            crate::handlers::SocialEventRow::MemoryAccountActiveUpdate { account_id, active } => {
                Some(MemoryRow::MemoryAccountActiveUpdate { account_id, active })
            }
            crate::handlers::SocialEventRow::AgentMemoryVault(v) => {
                Some(MemoryRow::AgentMemoryVault(v))
            }
            crate::handlers::SocialEventRow::SubAgentEvent(e) => Some(MemoryRow::SubAgentEvent(e)),
            crate::handlers::SocialEventRow::AgenticOrganizationUpsert(o) => {
                Some(MemoryRow::AgenticOrganizationUpsert(o))
            }
            crate::handlers::SocialEventRow::AgenticOrganizationMetadataUpdate {
                organization_id,
                name,
                description,
            } => Some(MemoryRow::AgenticOrganizationMetadataUpdate {
                organization_id,
                name,
                description,
            }),
            crate::handlers::SocialEventRow::AgenticOrganizationCategoryUpdate {
                organization_id,
                org_type,
                previous_org_type,
                updated_at_ms,
            } => Some(MemoryRow::AgenticOrganizationCategoryUpdate {
                organization_id,
                org_type,
                previous_org_type,
                updated_at_ms,
            }),
            crate::handlers::SocialEventRow::AgenticOrganizationDeactivate {
                organization_id,
                deactivated_at_ms,
            } => Some(MemoryRow::AgenticOrganizationDeactivate {
                organization_id,
                deactivated_at_ms,
            }),
            crate::handlers::SocialEventRow::OrganizationEvent(e) => {
                Some(MemoryRow::OrganizationEvent(e))
            }
            crate::handlers::SocialEventRow::OrganizationStatsInit {
                organization_id,
                activity_at_ms,
            } => Some(MemoryRow::OrganizationStatsInit {
                organization_id,
                activity_at_ms,
            }),
            crate::handlers::SocialEventRow::OrganizationAgentRegistered {
                organization_id,
                active,
                depth,
                parent_object_id,
                agent_object_id,
                activity_at_ms,
            } => Some(MemoryRow::OrganizationAgentRegistered {
                organization_id,
                active,
                depth,
                parent_object_id,
                agent_object_id,
                activity_at_ms,
            }),
            crate::handlers::SocialEventRow::OrganizationAgentActiveDelta {
                agent_object_id,
                active_delta,
                activity_at_ms,
            } => Some(MemoryRow::OrganizationAgentActiveDelta {
                agent_object_id,
                active_delta,
                activity_at_ms,
            }),
            _ => None,
        }
    }
}

impl FieldCount for MemoryRow {
    const FIELD_COUNT: usize = 10;
}

pub struct MemoryHandler;

#[async_trait]
impl Processor for MemoryHandler {
    const NAME: &'static str = "memory";

    type Value = MemoryRow;

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
                                hex_preview = %e.contents_hex_preview(48),
                                "memory pipeline: event contents parse failed; skipping event"
                            );
                            SocialMetrics::record_event_bcs_parse_failed(MEMORY_MODULE, event_name);
                            continue;
                        }
                    };
                if let Some(rows) = memory::handle_memory_event(event_name, &event_data, &event_id)
                {
                    for row in rows {
                        if let Some(r) = MemoryRow::from_social(row) {
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
impl Handler for MemoryHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                MemoryRow::MemoryAccount(a) => {
                    let principal_owner = a.principal_owner.clone();
                    let profile_id = a.profile_id.clone();
                    let active = a.active;
                    let created_at_ms = a.created_at_ms;
                    let event_id = a.event_id.clone();
                    let transaction_id = a.transaction_id.clone();
                    let time = a.time;
                    total += diesel::insert_into(memory_accounts::table)
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
                        .await?;
                }
                MemoryRow::ProfileMemoryAccountLink {
                    profile_id,
                    memory_account_id,
                } => {
                    let updated =
                        diesel::update(profiles::table.filter(profiles::profile_id.eq(profile_id)))
                            .set(profiles::memory_account_id.eq(memory_account_id))
                            .execute(conn)
                            .await?;
                    if updated == 0 {
                        tracing::debug!(
                            profile_id = %profile_id,
                            memory_account_id = %memory_account_id,
                            "ProfileMemoryAccountLink updated 0 profile rows; ProfilesHandler may backfill after insert"
                        );
                    }
                    total += updated;
                }
                MemoryRow::MemoryAccountActiveUpdate { account_id, active } => {
                    total += diesel::update(
                        memory_accounts::table.filter(memory_accounts::account_id.eq(account_id)),
                    )
                    .set(memory_accounts::active.eq(*active))
                    .execute(conn)
                    .await?;
                }
                MemoryRow::AgentMemoryVault(v) => {
                    let agent_object_id = v.agent_object_id.clone();
                    let memory_account_id = v.memory_account_id.clone();
                    let created_at_ms = v.created_at_ms;
                    let event_id = v.event_id.clone();
                    let transaction_id = v.transaction_id.clone();
                    let time = v.time;
                    total += diesel::insert_into(sub_agent_memory_vaults::table)
                        .values(v)
                        .on_conflict(sub_agent_memory_vaults::vault_id)
                        .do_update()
                        .set((
                            sub_agent_memory_vaults::agent_object_id.eq(agent_object_id),
                            sub_agent_memory_vaults::memory_account_id.eq(memory_account_id),
                            sub_agent_memory_vaults::created_at_ms.eq(created_at_ms),
                            sub_agent_memory_vaults::event_id.eq(event_id),
                            sub_agent_memory_vaults::transaction_id.eq(transaction_id),
                            sub_agent_memory_vaults::time.eq(time),
                        ))
                        .execute(conn)
                        .await?;
                }
                MemoryRow::SubAgentEvent(e) => {
                    total += diesel::insert_into(sub_agent_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                MemoryRow::AgenticOrganizationUpsert(o) => {
                    let account_id = o.account_id.clone();
                    let principal_owner = o.principal_owner.clone();
                    let profile_id = o.profile_id.clone();
                    let name = o.name.clone();
                    let description = o.description.clone();
                    let org_type = o.org_type;
                    let root_agent_id = o.root_agent_id.clone();
                    let active = o.active;
                    let created_at_ms = o.created_at_ms;
                    let deactivated_at_ms = o.deactivated_at_ms;
                    let event_id = o.event_id.clone();
                    let transaction_id = o.transaction_id.clone();
                    let time = o.time;
                    total += diesel::insert_into(sub_agent_organizations::table)
                        .values(o)
                        .on_conflict(sub_agent_organizations::organization_id)
                        .do_update()
                        .set((
                            sub_agent_organizations::account_id.eq(account_id),
                            sub_agent_organizations::principal_owner.eq(principal_owner),
                            sub_agent_organizations::profile_id.eq(profile_id),
                            sub_agent_organizations::name.eq(name),
                            sub_agent_organizations::description.eq(description),
                            sub_agent_organizations::org_type.eq(org_type),
                            sub_agent_organizations::root_agent_id.eq(root_agent_id),
                            sub_agent_organizations::active.eq(active),
                            sub_agent_organizations::created_at_ms.eq(created_at_ms),
                            sub_agent_organizations::deactivated_at_ms.eq(deactivated_at_ms),
                            sub_agent_organizations::event_id.eq(event_id),
                            sub_agent_organizations::transaction_id.eq(transaction_id),
                            sub_agent_organizations::time.eq(time),
                        ))
                        .execute(conn)
                        .await?;
                }
                MemoryRow::AgenticOrganizationMetadataUpdate {
                    organization_id,
                    name,
                    description,
                } => {
                    total += diesel::update(
                        sub_agent_organizations::table
                            .filter(sub_agent_organizations::organization_id.eq(organization_id)),
                    )
                    .set((
                        sub_agent_organizations::name.eq(name),
                        sub_agent_organizations::description.eq(description),
                    ))
                    .execute(conn)
                    .await?;
                }
                MemoryRow::AgenticOrganizationCategoryUpdate {
                    organization_id,
                    org_type,
                    ..
                } => {
                    total += diesel::update(
                        sub_agent_organizations::table
                            .filter(sub_agent_organizations::organization_id.eq(organization_id)),
                    )
                    .set(sub_agent_organizations::org_type.eq(*org_type))
                    .execute(conn)
                    .await?;
                }
                MemoryRow::AgenticOrganizationDeactivate {
                    organization_id,
                    deactivated_at_ms,
                } => {
                    total += diesel::update(
                        sub_agent_organizations::table
                            .filter(sub_agent_organizations::organization_id.eq(organization_id)),
                    )
                    .set((
                        sub_agent_organizations::active.eq(false),
                        sub_agent_organizations::deactivated_at_ms.eq(*deactivated_at_ms),
                    ))
                    .execute(conn)
                    .await?;
                }
                MemoryRow::OrganizationEvent(e) => {
                    total += diesel::insert_into(sub_agent_organization_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                MemoryRow::OrganizationStatsInit {
                    organization_id,
                    activity_at_ms,
                } => {
                    init_org_stats(conn, organization_id, *activity_at_ms).await?;
                    total += 1;
                }
                MemoryRow::OrganizationAgentRegistered {
                    organization_id,
                    active,
                    depth,
                    parent_object_id,
                    agent_object_id,
                    activity_at_ms,
                } => {
                    apply_sub_agent_registration_stats(
                        conn,
                        organization_id,
                        *active,
                        *depth,
                        *activity_at_ms,
                    )
                    .await?;
                    if parent_object_id.is_none() && *depth == 1 {
                        set_org_root_agent(conn, organization_id, agent_object_id).await?;
                    }
                    total += 1;
                }
                MemoryRow::OrganizationAgentActiveDelta {
                    agent_object_id,
                    active_delta,
                    activity_at_ms,
                } => {
                    let org_id = sub_agents::table
                        .filter(sub_agents::agent_object_id.eq(agent_object_id))
                        .select(sub_agents::organization_id)
                        .first::<Option<String>>(conn)
                        .await
                        .ok()
                        .flatten();
                    if let Some(organization_id) = org_id {
                        apply_sub_agent_active_delta(
                            conn,
                            &organization_id,
                            *active_delta,
                            *activity_at_ms,
                        )
                        .await?;
                    }
                    total += 1;
                }
            }
        }
        Ok(total)
    }
}
