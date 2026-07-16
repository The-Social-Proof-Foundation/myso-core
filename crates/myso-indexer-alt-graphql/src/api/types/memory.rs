// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;
use myso_indexer_alt_social_reader::{SocialAttributionRow, SubAgentRow};
use myso_indexer_alt_social_schema::models::MemoryAccountRow;

use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::enterprise::{AiCreditAgentBudget, OrgMemoryPermission, OrgRoleAssignment};

#[derive(Clone)]
pub(crate) struct SubAgent {
    inner: SubAgentRow,
}

impl SubAgent {
    pub(crate) fn from_row(inner: SubAgentRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SubAgent {
    async fn agent_object_id(&self) -> &str {
        &self.inner.agent_object_id
    }

    async fn derived_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.derived_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn label(&self) -> &str {
        &self.inner.label
    }

    async fn memory_vault_id(&self, ctx: &async_graphql::Context<'_>) -> Option<String> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_agent_memory_vault_id(&self.inner.agent_object_id)
            .await
            .ok()
            .flatten()
    }

    async fn identity_class(&self) -> i32 {
        i32::from(self.inner.identity_class)
    }

    async fn role_tags(&self) -> i64 {
        self.inner.role_tags
    }

    async fn capabilities(&self) -> i64 {
        self.inner.capabilities
    }

    async fn delegatable_caps(&self) -> i64 {
        self.inner.delegatable_caps
    }

    async fn register_scope(&self) -> i32 {
        i32::from(self.inner.register_scope)
    }

    async fn max_action_spend(&self) -> Option<i64> {
        self.inner.max_action_spend
    }

    async fn platform_scope(&self) -> Option<MySoAddress> {
        self.inner
            .platform_scope
            .as_deref()
            .and_then(|s| MySoAddress::from_str(s).ok())
            .map(Into::into)
    }

    async fn parent_object_id(&self) -> Option<&str> {
        self.inner.parent_object_id.as_deref()
    }

    async fn depth(&self) -> i32 {
        i32::from(self.inner.depth)
    }

    async fn registered_by(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.registered_by)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn active(&self) -> bool {
        self.inner.active
    }

    async fn expires_at(&self) -> Option<i64> {
        self.inner.expires_at_ms
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at_ms
    }

    async fn organization_id(&self) -> Option<&str> {
        self.inner.organization_id.as_deref()
    }

    async fn budget(&self, ctx: &async_graphql::Context<'_>) -> Option<AiCreditAgentBudget> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_agent_budget(&self.inner.agent_object_id)
            .await
            .ok()
            .flatten()
            .map(AiCreditAgentBudget::from_row)
    }

    async fn org_memory_permissions(
        &self,
        ctx: &async_graphql::Context<'_>,
    ) -> Option<Vec<OrgMemoryPermission>> {
        let org_id = self.inner.organization_id.as_deref()?;
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_org_memory_permissions(org_id, Some(&self.inner.derived_address), true)
            .await
            .ok()
            .map(|rows| {
                rows.into_iter()
                    .map(OrgMemoryPermission::from_row)
                    .collect()
            })
    }

    async fn org_roles(&self, ctx: &async_graphql::Context<'_>) -> Option<Vec<OrgRoleAssignment>> {
        let org_id = self.inner.organization_id.as_deref()?;
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_org_role_assignments(org_id, Some(&self.inner.derived_address), true)
            .await
            .ok()
            .map(|rows| rows.into_iter().map(OrgRoleAssignment::from_row).collect())
    }

    async fn children(
        &self,
        ctx: &async_graphql::Context<'_>,
        active_only: Option<bool>,
        limit: Option<u64>,
        offset: Option<u64>,
    ) -> Option<Vec<SubAgent>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let limit = limit.unwrap_or(50).min(100) as i64;
        let offset = offset.unwrap_or(0) as i64;
        reader
            .list_sub_agent_children(
                &self.inner.agent_object_id,
                active_only.unwrap_or(true),
                limit,
                offset,
            )
            .await
            .ok()
            .map(|rows| rows.into_iter().map(SubAgent::from_row).collect())
    }
}

#[derive(Clone)]
pub(crate) struct MemoryAccount {
    inner: MemoryAccountRow,
}

impl MemoryAccount {
    pub(crate) fn from_row(inner: MemoryAccountRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MemoryAccount {
    async fn account_id(&self) -> &str {
        &self.inner.account_id
    }

    async fn principal_owner(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.principal_owner)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn profile_id(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.profile_id)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn active(&self) -> bool {
        self.inner.active
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at_ms
    }

    async fn contract_version(&self) -> i64 {
        self.inner.contract_version
    }
}

#[derive(Clone)]
pub(crate) struct SocialAttribution {
    inner: SocialAttributionRow,
}

impl SocialAttribution {
    pub(crate) fn from_post(owner: &str, row: &myso_indexer_alt_social_reader::PostRow) -> Self {
        let actor = row
            .actor_address
            .clone()
            .unwrap_or_else(|| owner.to_string());
        Self {
            inner: SocialAttributionRow::from_parts(
                Some(actor),
                row.sub_agent_id.clone(),
                row.action_identity_class,
                None,
                row.organization_id.clone(),
                owner,
            ),
        }
    }

    pub(crate) fn from_comment(row: &myso_indexer_alt_social_reader::CommentRow) -> Self {
        Self {
            inner: SocialAttributionRow::from_parts(
                row.actor_address.clone(),
                row.sub_agent_id.clone(),
                row.action_identity_class,
                None,
                row.organization_id.clone(),
                &row.owner,
            ),
        }
    }

    pub(crate) fn from_reaction(row: &myso_indexer_alt_social_reader::ReactionRow) -> Self {
        Self {
            inner: SocialAttributionRow::from_parts(
                row.actor_address.clone(),
                row.sub_agent_id.clone(),
                row.action_identity_class,
                row.principal_owner.clone(),
                row.organization_id.clone(),
                &row.user_address,
            ),
        }
    }

    pub(crate) fn from_repost(row: &myso_indexer_alt_social_reader::RepostRow) -> Self {
        Self {
            inner: SocialAttributionRow::from_parts(
                row.actor_address.clone(),
                row.sub_agent_id.clone(),
                row.action_identity_class,
                None,
                row.organization_id.clone(),
                &row.owner,
            ),
        }
    }
}

#[Object]
impl SocialAttribution {
    async fn actor_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.actor_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn sub_agent_id(&self) -> Option<&str> {
        self.inner.sub_agent_id.as_deref()
    }

    async fn action_identity_class(&self) -> i32 {
        i32::from(self.inner.action_identity_class)
    }

    async fn principal_owner(&self) -> Option<MySoAddress> {
        self.inner
            .principal_owner
            .as_deref()
            .and_then(|s| MySoAddress::from_str(s).ok())
            .map(Into::into)
    }

    async fn organization_id(&self) -> Option<&str> {
        self.inner.organization_id.as_deref()
    }
}
