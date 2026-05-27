// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{MemoryAccountRow, SubAgentRow};
use myso_indexer_alt_social_schema::schema::{memory_accounts, sub_agents};
use myso_pg_db::Db;
use serde::Serialize;

use crate::error::SocialError;

#[derive(Debug, Clone, Serialize)]
pub struct SubAgentListResponse {
    pub sub_agents: Vec<SubAgentRow>,
    pub total_count: i64,
}

pub(crate) async fn get_memory_account_by_owner(
    db: &Db,
    owner: &str,
) -> Result<Option<MemoryAccountRow>, SocialError> {
    let mut conn = db.connect().await?;
    memory_accounts::table
        .filter(memory_accounts::principal_owner.eq(owner))
        .select(MemoryAccountRow::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(Into::into)
}

pub(crate) async fn get_sub_agent(
    db: &Db,
    derived_address: &str,
) -> Result<Option<SubAgentRow>, SocialError> {
    let mut conn = db.connect().await?;
    sub_agents::table
        .filter(sub_agents::derived_address.eq(derived_address))
        .select(SubAgentRow::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(Into::into)
}

pub(crate) async fn get_sub_agent_by_object_id(
    db: &Db,
    agent_object_id: &str,
) -> Result<Option<SubAgentRow>, SocialError> {
    let mut conn = db.connect().await?;
    sub_agents::table
        .filter(sub_agents::agent_object_id.eq(agent_object_id))
        .select(SubAgentRow::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(Into::into)
}

pub(crate) async fn list_sub_agent_children(
    db: &Db,
    parent_object_id: &str,
    active_only: bool,
    limit: i64,
    offset: i64,
) -> Result<Vec<SubAgentRow>, SocialError> {
    let mut conn = db.connect().await?;
    let mut query = sub_agents::table
        .filter(sub_agents::parent_object_id.eq(parent_object_id))
        .into_boxed();
    if active_only {
        query = query.filter(sub_agents::active.eq(true));
    }
    query
        .order(sub_agents::created_at_ms.desc())
        .limit(limit)
        .offset(offset)
        .select(SubAgentRow::as_select())
        .load(&mut conn)
        .await
        .map_err(Into::into)
}

pub(crate) async fn list_sub_agents(
    db: &Db,
    principal_owner: &str,
    active_only: bool,
    limit: i64,
    offset: i64,
) -> Result<SubAgentListResponse, SocialError> {
    let mut conn = db.connect().await?;
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
    let total_count = count_query.count().get_result(&mut conn).await?;
    let sub_agents = list_query
        .order(sub_agents::depth.asc())
        .then_order_by(sub_agents::created_at_ms.desc())
        .limit(limit)
        .offset(offset)
        .select(SubAgentRow::as_select())
        .load(&mut conn)
        .await?;
    Ok(SubAgentListResponse {
        sub_agents,
        total_count,
    })
}
