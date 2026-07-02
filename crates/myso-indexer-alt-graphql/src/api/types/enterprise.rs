// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Context, Enum, InputObject, Object, SimpleObject};
use std::str::FromStr;
use myso_indexer_alt_social_reader::{
    AgentSpendBreakdownEntry, AuditLogFilter, OrganizationStatsWindow,
};
use myso_indexer_alt_social_schema::models::{
    AiCreditAgentBudgetRow, AiCreditSpendApprovalRow, AuditLogRow, OrgMemoryPermissionRow,
    OrgRoleAssignmentRow, OrgRoleRow,
};

use crate::api::scalars::big_int::BigInt;
use crate::api::scalars::date_time::DateTime as GqlDateTime;
use crate::api::scalars::json::Json;
use crate::api::scalars::myso_address::MySoAddress;
use crate::api::types::memory::SubAgent;
use crate::api::types::organization::OrganizationStatsWindowGql;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum SpendApprovalStatusGql {
    Requested,
    Approved,
    Consumed,
    Revoked,
    Expired,
}

impl SpendApprovalStatusGql {
    fn from_db(status: &str) -> Self {
        match status {
            "approved" => Self::Approved,
            "consumed" => Self::Consumed,
            "revoked" => Self::Revoked,
            "expired" => Self::Expired,
            _ => Self::Requested,
        }
    }
}

#[derive(Clone)]
pub(crate) struct SpendApproval {
    inner: AiCreditSpendApprovalRow,
}

impl SpendApproval {
    pub(crate) fn from_row(inner: AiCreditSpendApprovalRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl SpendApproval {
    async fn balance_id(&self) -> &str {
        &self.inner.balance_id
    }

    async fn agent_object_id(&self) -> &str {
        &self.inner.agent_object_id
    }

    async fn status(&self) -> SpendApprovalStatusGql {
        SpendApprovalStatusGql::from_db(&self.inner.status)
    }

    async fn requested_amount_mist(&self) -> Option<BigInt> {
        self.inner.requested_amount_mist.map(BigInt::from)
    }

    async fn threshold_mist(&self) -> Option<BigInt> {
        self.inner.threshold_mist.map(BigInt::from)
    }

    async fn max_amount_mist(&self) -> Option<BigInt> {
        self.inner.max_amount_mist.map(BigInt::from)
    }

    async fn expires_at_ms(&self) -> Option<BigInt> {
        self.inner.expires_at_ms.map(BigInt::from)
    }

    async fn approved_by(&self) -> Option<MySoAddress> {
        self.inner
            .approved_by
            .as_deref()
            .and_then(|s| MySoAddress::from_str(s).ok())
            .map(Into::into)
    }

    async fn organization_id(&self) -> Option<&str> {
        self.inner.organization_id.as_deref()
    }

    async fn requested_at(&self) -> GqlDateTime {
        GqlDateTime::from_chrono(self.inner.requested_at)
    }

    async fn updated_at(&self) -> GqlDateTime {
        GqlDateTime::from_chrono(self.inner.updated_at)
    }
}

#[derive(Clone)]
pub(crate) struct AiCreditAgentBudget {
    inner: AiCreditAgentBudgetRow,
}

impl AiCreditAgentBudget {
    pub(crate) fn from_row(inner: AiCreditAgentBudgetRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl AiCreditAgentBudget {
    async fn agent_object_id(&self) -> &str {
        &self.inner.agent_object_id
    }

    async fn budget_mist(&self) -> Option<BigInt> {
        self.inner.budget_mist.map(BigInt::from)
    }

    async fn spent_mist(&self) -> BigInt {
        BigInt::from(self.inner.spent_mist)
    }

    async fn daily_cap_mist(&self) -> Option<BigInt> {
        self.inner.daily_cap_mist.map(BigInt::from)
    }

    async fn monthly_cap_mist(&self) -> Option<BigInt> {
        self.inner.monthly_cap_mist.map(BigInt::from)
    }

    async fn require_approval_above_mist(&self) -> Option<BigInt> {
        self.inner.require_approval_above_mist.map(BigInt::from)
    }

    async fn enabled(&self) -> bool {
        self.inner.enabled
    }
}

#[derive(Clone)]
pub(crate) struct OrgMemoryPermission {
    inner: OrgMemoryPermissionRow,
}

impl OrgMemoryPermission {
    pub(crate) fn from_row(inner: OrgMemoryPermissionRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl OrgMemoryPermission {
    async fn member_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.member_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn permission_kind(&self) -> BigInt {
        BigInt::from(self.inner.permission_kind)
    }

    async fn active(&self) -> bool {
        self.inner.active
    }

    async fn granted_by(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.granted_by)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn granted_at(&self) -> GqlDateTime {
        GqlDateTime::from_chrono(self.inner.time)
    }
}

#[derive(Clone)]
pub(crate) struct OrgRole {
    inner: OrgRoleRow,
}

impl OrgRole {
    pub(crate) fn from_row(inner: OrgRoleRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl OrgRole {
    async fn role_name(&self) -> &str {
        &self.inner.role_name
    }

    async fn mask(&self) -> BigInt {
        BigInt::from(self.inner.mask)
    }

    async fn is_builtin(&self) -> bool {
        self.inner.is_builtin
    }

    async fn defined_by(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.defined_by)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }
}

#[derive(Clone)]
pub(crate) struct OrgRoleAssignment {
    inner: OrgRoleAssignmentRow,
}

impl OrgRoleAssignment {
    pub(crate) fn from_row(inner: OrgRoleAssignmentRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl OrgRoleAssignment {
    async fn member_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.member_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn role_name(&self) -> &str {
        &self.inner.role_name
    }

    async fn assigned_mask(&self) -> BigInt {
        BigInt::from(self.inner.assigned_mask)
    }

    async fn active(&self) -> bool {
        self.inner.active
    }

    async fn assigned_by(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.assigned_by)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn assigned_at_ms(&self) -> BigInt {
        BigInt::from(self.inner.assigned_at_ms)
    }
}

#[derive(InputObject, Default)]
pub(crate) struct AuditLogFilterInput {
    pub action: Option<String>,
    pub actor: Option<String>,
    pub target_type: Option<String>,
    pub source: Option<String>,
}

impl From<AuditLogFilterInput> for AuditLogFilter {
    fn from(value: AuditLogFilterInput) -> Self {
        Self {
            action: value.action,
            actor: value.actor,
            target_type: value.target_type,
            source: value.source,
        }
    }
}

#[derive(Clone)]
pub(crate) struct AuditLogEntry {
    inner: AuditLogRow,
}

impl AuditLogEntry {
    pub(crate) fn from_row(inner: AuditLogRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl AuditLogEntry {
    async fn id(&self) -> BigInt {
        BigInt::from(self.inner.id)
    }

    async fn time(&self) -> GqlDateTime {
        GqlDateTime::from_chrono(self.inner.time)
    }

    async fn source(&self) -> &str {
        &self.inner.source
    }

    async fn actor_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.actor_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn actor_type(&self) -> &str {
        &self.inner.actor_type
    }

    async fn action(&self) -> &str {
        &self.inner.action
    }

    async fn target_type(&self) -> &str {
        &self.inner.target_type
    }

    async fn target_id(&self) -> &str {
        &self.inner.target_id
    }

    async fn organization_id(&self) -> Option<&str> {
        self.inner.organization_id.as_deref()
    }

    async fn account_id(&self) -> Option<&str> {
        self.inner.account_id.as_deref()
    }

    async fn prev_state(&self) -> Option<Json> {
        self.inner
            .prev_state
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    async fn new_state(&self) -> Option<Json> {
        self.inner
            .new_state
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }

    async fn tx_digest(&self) -> Option<&str> {
        self.inner.tx_digest.as_deref()
    }

    async fn metadata(&self) -> Option<Json> {
        self.inner
            .metadata
            .as_ref()
            .and_then(|v| Json::try_from(v.clone()).ok())
    }
}

#[derive(Clone)]
pub(crate) struct AgentSpendBreakdown {
    inner: AgentSpendBreakdownEntry,
}

impl AgentSpendBreakdown {
    pub(crate) fn from_entry(inner: AgentSpendBreakdownEntry) -> Self {
        Self { inner }
    }
}

#[Object]
impl AgentSpendBreakdown {
    async fn agent_object_id(&self) -> &str {
        &self.inner.agent_object_id
    }

    async fn label(&self) -> &str {
        &self.inner.label
    }

    async fn derived_address(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.derived_address)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn spent_mist(&self) -> BigInt {
        BigInt::from(self.inner.spent_mist)
    }

    async fn usage_events(&self) -> BigInt {
        BigInt::from(self.inner.usage_events)
    }

    async fn budget_mist(&self) -> Option<BigInt> {
        self.inner.budget_mist.map(BigInt::from)
    }

    async fn require_approval_above_mist(&self) -> Option<BigInt> {
        self.inner.require_approval_above_mist.map(BigInt::from)
    }

    async fn budget_enabled(&self) -> bool {
        self.inner.budget_enabled
    }

    async fn memory_entries(&self) -> BigInt {
        BigInt::from(self.inner.memory_entries)
    }

    async fn memory_bytes(&self) -> BigInt {
        BigInt::from(self.inner.memory_bytes)
    }

    async fn org_shared_memory_entries(&self) -> BigInt {
        BigInt::from(self.inner.org_shared_memory_entries)
    }

    async fn agent(&self, ctx: &Context<'_>) -> Option<SubAgent> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_sub_agent_by_object_id(&self.inner.agent_object_id)
            .await
            .ok()
            .flatten()
            .map(SubAgent::from_row)
    }
}

pub(crate) fn window_from_gql(window: Option<OrganizationStatsWindowGql>) -> OrganizationStatsWindow {
    window.unwrap_or_default().into()
}

#[derive(SimpleObject)]
pub(crate) struct AuditLogConnection {
    pub entries: Vec<AuditLogEntry>,
}
