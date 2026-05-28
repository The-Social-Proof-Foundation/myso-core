// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{MemoryAccountRow, SubAgentRow};
use myso_indexer_alt_social_schema::schema::{agent_memory_vaults, memory_accounts, profiles, sub_agents};
use myso_pg_db::Connection;
use serde::Serialize;

use crate::metrics::DbReaderMetrics;

#[derive(Debug, Clone, Serialize)]
pub struct SubAgentListResult {
    pub sub_agents: Vec<SubAgentRow>,
    pub total_count: i64,
}

pub(crate) async fn get_memory_account_by_owner(
    conn: &mut Connection<'_>,
    owner: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MemoryAccountRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = memory_accounts::table
        .filter(memory_accounts::principal_owner.eq(owner))
        .select(MemoryAccountRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub(crate) async fn get_memory_account_by_profile_id(
    conn: &mut Connection<'_>,
    profile_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<MemoryAccountRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = memory_accounts::table
        .filter(memory_accounts::profile_id.eq(profile_id))
        .select(MemoryAccountRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub(crate) async fn list_sub_agents(
    conn: &mut Connection<'_>,
    principal_owner: &str,
    active_only: bool,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<SubAgentListResult> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let mut count_query = sub_agents::table
        .inner_join(memory_accounts::table)
        .filter(memory_accounts::principal_owner.eq(principal_owner))
        .into_boxed();
    let mut list_query = sub_agents::table
        .inner_join(memory_accounts::table)
        .filter(memory_accounts::principal_owner.eq(principal_owner))
        .into_boxed();
    if active_only {
        count_query = count_query.filter(sub_agents::active.eq(true));
        list_query = list_query.filter(sub_agents::active.eq(true));
    }
    let total_count = count_query.count().get_result::<i64>(conn).await?;
    let sub_agents = list_query
        .order(sub_agents::depth.asc())
        .then_order_by(sub_agents::created_at_ms.desc())
        .limit(limit)
        .offset(offset)
        .select(SubAgentRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(SubAgentListResult {
        sub_agents,
        total_count,
    })
}

pub(crate) async fn get_sub_agent(
    conn: &mut Connection<'_>,
    derived_address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SubAgentRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = sub_agents::table
        .filter(sub_agents::derived_address.eq(derived_address))
        .select(SubAgentRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub(crate) async fn get_sub_agent_by_object_id(
    conn: &mut Connection<'_>,
    agent_object_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<SubAgentRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let row = sub_agents::table
        .filter(sub_agents::agent_object_id.eq(agent_object_id))
        .select(SubAgentRow::as_select())
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(row)
}

pub(crate) async fn list_sub_agent_children(
    conn: &mut Connection<'_>,
    parent_object_id: &str,
    active_only: bool,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<SubAgentRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let mut query = sub_agents::table
        .filter(sub_agents::parent_object_id.eq(parent_object_id))
        .into_boxed();
    if active_only {
        query = query.filter(sub_agents::active.eq(true));
    }
    let rows = query
        .order(sub_agents::created_at_ms.desc())
        .limit(limit)
        .offset(offset)
        .select(SubAgentRow::as_select())
        .load(conn)
        .await?;
    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn get_agent_memory_vault_id(
    conn: &mut Connection<'_>,
    agent_object_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<String>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let vault_id = agent_memory_vaults::table
        .filter(agent_memory_vaults::agent_object_id.eq(agent_object_id))
        .select(agent_memory_vaults::vault_id)
        .first(conn)
        .await
        .optional()?;
    metrics.requests_succeeded.inc();
    Ok(vault_id)
}

pub(crate) async fn get_profile_memory_account_id(
    conn: &mut Connection<'_>,
    profile_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<String>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();
    let denormalized = profiles::table
        .filter(profiles::profile_id.eq(profile_id))
        .select(profiles::memory_account_id)
        .first::<Option<String>>(conn)
        .await
        .optional()?
        .flatten();
    let registry_account_id = if denormalized.is_some() {
        None
    } else {
        memory_accounts::table
            .filter(memory_accounts::profile_id.eq(profile_id))
            .select(memory_accounts::account_id)
            .first(conn)
            .await
            .optional()?
    };
    metrics.requests_succeeded.inc();
    Ok(resolve_profile_memory_account_id(denormalized, registry_account_id))
}

fn resolve_profile_memory_account_id(
    denormalized: Option<String>,
    registry_account_id: Option<String>,
) -> Option<String> {
    denormalized.or(registry_account_id)
}

#[derive(Debug, Clone, Serialize)]
pub struct SocialAttributionRow {
    pub actor_address: String,
    pub sub_agent_id: Option<String>,
    pub action_identity_class: i16,
    pub principal_owner: Option<String>,
}

impl SocialAttributionRow {
    pub fn from_parts(
        actor_address: Option<String>,
        sub_agent_id: Option<String>,
        action_identity_class: Option<i16>,
        principal_owner: Option<String>,
        fallback_actor: &str,
    ) -> Self {
        Self {
            actor_address: actor_address.unwrap_or_else(|| fallback_actor.to_string()),
            sub_agent_id,
            action_identity_class: action_identity_class.unwrap_or(0),
            principal_owner,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::resolve_profile_memory_account_id;

    #[test]
    fn profile_memory_account_id_resolution_prefers_denormalized_column() {
        assert_eq!(
            resolve_profile_memory_account_id(
                Some("0xdenorm".to_string()),
                Some("0xregistry".to_string()),
            ),
            Some("0xdenorm".to_string()),
        );
    }

    #[test]
    fn profile_memory_account_id_resolution_falls_back_to_registry() {
        assert_eq!(
            resolve_profile_memory_account_id(None, Some("0xregistry".to_string())),
            Some("0xregistry".to_string()),
        );
        assert_eq!(resolve_profile_memory_account_id(None, None), None);
    }
}
