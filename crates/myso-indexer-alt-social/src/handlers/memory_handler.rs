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
    NewAgentMemoryVault, NewAgenticOrganization, NewAuditLog, NewMemoryAccount, NewMemoryConfig,
    NewOrgInvitation, NewOrgMemoryPermission, NewOrgRole, NewOrgRoleAssignment,
    NewOrganizationEvent, NewSubAgentEvent,
};
use myso_indexer_alt_social_schema::schema::{
    audit_log, memory_accounts, memory_config, org_invitations, org_memory_permissions,
    org_role_assignments, org_roles, profiles, sub_agent_events, sub_agent_memory_vaults,
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
    MemoryConfig(NewMemoryConfig),
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
    AgenticOrganizationMemoryGroupSet {
        organization_id: String,
        group_id: String,
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
    OrgMemoryPermissionUpsert(NewOrgMemoryPermission),
    OrgRoleUpsert(NewOrgRole),
    OrgRoleAssignmentUpsert(NewOrgRoleAssignment),
    OrgRoleAssignmentRevoke {
        organization_id: String,
        member_address: String,
        role_name: String,
        revoked_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    OrgInvitationUpsert(NewOrgInvitation),
    OrgInvitationRespond {
        organization_id: String,
        invitee_address: String,
        status: String,
        responded_at_ms: i64,
        responded_by: String,
        granted_mask: Option<i64>,
        event_id: String,
        transaction_id: String,
    },
    AuditLog(NewAuditLog),
}

impl MemoryRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::MemoryAccount(a) => Some(MemoryRow::MemoryAccount(a)),
            crate::handlers::SocialEventRow::MemoryConfig(c) => Some(MemoryRow::MemoryConfig(c)),
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
            crate::handlers::SocialEventRow::AgenticOrganizationMemoryGroupSet {
                organization_id,
                group_id,
            } => Some(MemoryRow::AgenticOrganizationMemoryGroupSet {
                organization_id,
                group_id,
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
            crate::handlers::SocialEventRow::OrgMemoryPermissionUpsert(p) => {
                Some(MemoryRow::OrgMemoryPermissionUpsert(p))
            }
            crate::handlers::SocialEventRow::OrgRoleUpsert(r) => Some(MemoryRow::OrgRoleUpsert(r)),
            crate::handlers::SocialEventRow::OrgRoleAssignmentUpsert(a) => {
                Some(MemoryRow::OrgRoleAssignmentUpsert(a))
            }
            crate::handlers::SocialEventRow::OrgRoleAssignmentRevoke {
                organization_id,
                member_address,
                role_name,
                revoked_at_ms,
                event_id,
                transaction_id,
            } => Some(MemoryRow::OrgRoleAssignmentRevoke {
                organization_id,
                member_address,
                role_name,
                revoked_at_ms,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::OrgInvitationUpsert(i) => {
                Some(MemoryRow::OrgInvitationUpsert(i))
            }
            crate::handlers::SocialEventRow::OrgInvitationRespond {
                organization_id,
                invitee_address,
                status,
                responded_at_ms,
                responded_by,
                granted_mask,
                event_id,
                transaction_id,
            } => Some(MemoryRow::OrgInvitationRespond {
                organization_id,
                invitee_address,
                status,
                responded_at_ms,
                responded_by,
                granted_mask,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AuditLog(a) => Some(MemoryRow::AuditLog(a)),
            _ => None,
        }
    }
}

impl FieldCount for MemoryRow {
    const FIELD_COUNT: usize = 11;
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
                MemoryRow::MemoryConfig(c) => {
                    total += diesel::insert_into(memory_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
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
                MemoryRow::AgenticOrganizationMemoryGroupSet {
                    organization_id,
                    group_id,
                } => {
                    total += diesel::update(
                        sub_agent_organizations::table
                            .filter(sub_agent_organizations::organization_id.eq(organization_id)),
                    )
                    .set(sub_agent_organizations::org_memory_group_id.eq(group_id))
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
                MemoryRow::OrgMemoryPermissionUpsert(p) => {
                    total += diesel::insert_into(org_memory_permissions::table)
                        .values(p)
                        .on_conflict((
                            org_memory_permissions::organization_id,
                            org_memory_permissions::member_address,
                            org_memory_permissions::permission_kind,
                        ))
                        .do_update()
                        .set((
                            org_memory_permissions::active.eq(p.active),
                            org_memory_permissions::granted_by.eq(p.granted_by.clone()),
                            org_memory_permissions::group_id.eq(p.group_id.clone()),
                            org_memory_permissions::event_id.eq(p.event_id.clone()),
                            org_memory_permissions::transaction_id.eq(p.transaction_id.clone()),
                            org_memory_permissions::time.eq(p.time),
                        ))
                        .execute(conn)
                        .await?;
                }
                MemoryRow::OrgRoleUpsert(r) => {
                    total += diesel::insert_into(org_roles::table)
                        .values(r)
                        .on_conflict((org_roles::organization_id, org_roles::role_name))
                        .do_update()
                        .set((
                            org_roles::mask.eq(r.mask),
                            org_roles::is_builtin.eq(r.is_builtin),
                            org_roles::defined_by.eq(r.defined_by.clone()),
                            org_roles::active.eq(r.active),
                            org_roles::updated_at_ms.eq(r.updated_at_ms),
                            org_roles::event_id.eq(r.event_id.clone()),
                            org_roles::transaction_id.eq(r.transaction_id.clone()),
                            org_roles::time.eq(r.time),
                        ))
                        .execute(conn)
                        .await?;
                }
                MemoryRow::OrgRoleAssignmentUpsert(a) => {
                    total += diesel::insert_into(org_role_assignments::table)
                        .values(a)
                        .on_conflict((
                            org_role_assignments::organization_id,
                            org_role_assignments::member_address,
                            org_role_assignments::role_name,
                        ))
                        .do_update()
                        .set((
                            org_role_assignments::role_mask.eq(a.role_mask),
                            org_role_assignments::assigned_mask.eq(a.assigned_mask),
                            org_role_assignments::active.eq(a.active),
                            org_role_assignments::assigned_by.eq(a.assigned_by.clone()),
                            org_role_assignments::assigned_at_ms.eq(a.assigned_at_ms),
                            org_role_assignments::revoked_at_ms.eq(a.revoked_at_ms),
                            org_role_assignments::event_id.eq(a.event_id.clone()),
                            org_role_assignments::transaction_id.eq(a.transaction_id.clone()),
                            org_role_assignments::time.eq(a.time),
                        ))
                        .execute(conn)
                        .await?;
                }
                MemoryRow::OrgRoleAssignmentRevoke {
                    organization_id,
                    member_address,
                    role_name,
                    revoked_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        org_role_assignments::table
                            .filter(org_role_assignments::organization_id.eq(organization_id))
                            .filter(org_role_assignments::member_address.eq(member_address))
                            .filter(org_role_assignments::role_name.eq(role_name)),
                    )
                    .set((
                        org_role_assignments::active.eq(false),
                        org_role_assignments::revoked_at_ms.eq(Some(*revoked_at_ms)),
                        org_role_assignments::event_id.eq(event_id),
                        org_role_assignments::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                MemoryRow::OrgInvitationUpsert(i) => {
                    total += diesel::insert_into(org_invitations::table)
                        .values(i)
                        .on_conflict((
                            org_invitations::organization_id,
                            org_invitations::invitee_address,
                        ))
                        .do_update()
                        .set((
                            org_invitations::role_name.eq(i.role_name.clone()),
                            org_invitations::permissions_mask.eq(i.permissions_mask),
                            org_invitations::status.eq(i.status.clone()),
                            org_invitations::invited_by.eq(i.invited_by.clone()),
                            org_invitations::created_at_ms.eq(i.created_at_ms),
                            org_invitations::expires_at_ms.eq(i.expires_at_ms),
                            org_invitations::responded_at_ms.eq(i.responded_at_ms),
                            org_invitations::responded_by.eq(i.responded_by.clone()),
                            org_invitations::granted_mask.eq(i.granted_mask),
                            org_invitations::event_id.eq(i.event_id.clone()),
                            org_invitations::transaction_id.eq(i.transaction_id.clone()),
                            org_invitations::time.eq(i.time),
                        ))
                        .execute(conn)
                        .await?;
                }
                MemoryRow::OrgInvitationRespond {
                    organization_id,
                    invitee_address,
                    status,
                    responded_at_ms,
                    responded_by,
                    granted_mask,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        org_invitations::table
                            .filter(org_invitations::organization_id.eq(organization_id))
                            .filter(org_invitations::invitee_address.eq(invitee_address)),
                    )
                    .set((
                        org_invitations::status.eq(status),
                        org_invitations::responded_at_ms.eq(Some(*responded_at_ms)),
                        org_invitations::responded_by.eq(Some(responded_by.clone())),
                        org_invitations::granted_mask.eq(*granted_mask),
                        org_invitations::event_id.eq(event_id),
                        org_invitations::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                MemoryRow::AuditLog(a) => {
                    total += diesel::insert_into(audit_log::table)
                        .values(a)
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
