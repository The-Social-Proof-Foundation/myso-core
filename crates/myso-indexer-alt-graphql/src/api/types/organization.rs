// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::api::scalars::date_time::DateTime as GqlDateTime;
use async_graphql::{Context, Enum, Object, SimpleObject};
use chrono::Utc;
use myso_indexer_alt_social_reader::{
    OrganizationCategoryInfo, OrganizationLeaderboardEntry as LeaderboardEntryRow,
    OrganizationLeaderboardResult, OrganizationLeaderboardSort as DbOrganizationLeaderboardSort,
    OrganizationStatistics, OrganizationStatsWindow as DbOrganizationStatsWindow,
};
use myso_indexer_alt_social_schema::models::{
    AgenticOrganizationRow, ORG_TYPE_BRAND, ORG_TYPE_COMMUNITY, ORG_TYPE_COMPANY,
    ORG_TYPE_EDUCATION, ORG_TYPE_GOVERNMENT, ORG_TYPE_HEALTHCARE, ORG_TYPE_INVESTMENT_FUND,
    ORG_TYPE_MEDIA, ORG_TYPE_NONPROFIT, ORG_TYPE_OTHER, ORG_TYPE_RESEARCH, ORG_TYPE_SPORTS,
    ORG_TYPE_STARTUP, ORG_TYPE_STEWARDSHIP,
};

use crate::api::scalars::big_int::BigInt;
use crate::api::types::enterprise::{
    AgentSpendBreakdown, AuditLogConnection, AuditLogEntry, AuditLogFilterInput, OrgInvitation,
    OrgMemoryPermission, OrgRole, OrgRoleAssignment, SpendApproval, resolve_stats_window,
};
use crate::api::types::memory::SubAgent;

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum OrganizationType {
    Company,
    Startup,
    InvestmentFund,
    Nonprofit,
    Research,
    Government,
    Media,
    Stewardship,
    Brand,
    Community,
    Sports,
    Education,
    Healthcare,
    Other,
}

impl OrganizationType {
    pub(crate) fn to_i16(self) -> i16 {
        match self {
            Self::Company => ORG_TYPE_COMPANY,
            Self::Startup => ORG_TYPE_STARTUP,
            Self::InvestmentFund => ORG_TYPE_INVESTMENT_FUND,
            Self::Nonprofit => ORG_TYPE_NONPROFIT,
            Self::Research => ORG_TYPE_RESEARCH,
            Self::Government => ORG_TYPE_GOVERNMENT,
            Self::Media => ORG_TYPE_MEDIA,
            Self::Stewardship => ORG_TYPE_STEWARDSHIP,
            Self::Brand => ORG_TYPE_BRAND,
            Self::Community => ORG_TYPE_COMMUNITY,
            Self::Sports => ORG_TYPE_SPORTS,
            Self::Education => ORG_TYPE_EDUCATION,
            Self::Healthcare => ORG_TYPE_HEALTHCARE,
            Self::Other => ORG_TYPE_OTHER,
        }
    }

    pub(crate) fn from_i16(value: i16) -> Self {
        match value {
            ORG_TYPE_COMPANY => Self::Company,
            ORG_TYPE_STARTUP => Self::Startup,
            ORG_TYPE_INVESTMENT_FUND => Self::InvestmentFund,
            ORG_TYPE_NONPROFIT => Self::Nonprofit,
            ORG_TYPE_RESEARCH => Self::Research,
            ORG_TYPE_GOVERNMENT => Self::Government,
            ORG_TYPE_MEDIA => Self::Media,
            ORG_TYPE_STEWARDSHIP => Self::Stewardship,
            ORG_TYPE_BRAND => Self::Brand,
            ORG_TYPE_COMMUNITY => Self::Community,
            ORG_TYPE_SPORTS => Self::Sports,
            ORG_TYPE_EDUCATION => Self::Education,
            ORG_TYPE_HEALTHCARE => Self::Healthcare,
            _ => Self::Other,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug, Default)]
pub(crate) enum OrganizationStatsWindow {
    #[default]
    All,
    Days7,
    Days30,
    Days180,
    Days365,
}

impl From<OrganizationStatsWindow> for DbOrganizationStatsWindow {
    fn from(value: OrganizationStatsWindow) -> Self {
        match value {
            OrganizationStatsWindow::All => Self::All,
            OrganizationStatsWindow::Days7 => Self::Days7,
            OrganizationStatsWindow::Days30 => Self::Days30,
            OrganizationStatsWindow::Days180 => Self::Days180,
            OrganizationStatsWindow::Days365 => Self::Days365,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum OrganizationLeaderboardSort {
    HighestNetCashFlow,
    FastestGrowing,
    HighestAccuracy,
    MostActive,
    HighestRevenue,
    LargestEstimatedAum,
}

impl From<OrganizationLeaderboardSort> for DbOrganizationLeaderboardSort {
    fn from(value: OrganizationLeaderboardSort) -> Self {
        match value {
            OrganizationLeaderboardSort::HighestNetCashFlow => Self::HighestNetCashFlow,
            OrganizationLeaderboardSort::FastestGrowing => Self::FastestGrowing,
            OrganizationLeaderboardSort::HighestAccuracy => Self::HighestAccuracy,
            OrganizationLeaderboardSort::MostActive => Self::MostActive,
            OrganizationLeaderboardSort::HighestRevenue => Self::HighestRevenue,
            OrganizationLeaderboardSort::LargestEstimatedAum => Self::LargestEstimatedAum,
        }
    }
}

#[derive(Clone)]
pub(crate) struct AgenticOrganization {
    inner: AgenticOrganizationRow,
}

impl AgenticOrganization {
    pub(crate) fn from_row(inner: AgenticOrganizationRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl AgenticOrganization {
    async fn organization_id(&self) -> &str {
        &self.inner.organization_id
    }

    async fn name(&self) -> Option<&str> {
        self.inner.name.as_deref()
    }

    async fn description(&self) -> Option<&str> {
        self.inner.description.as_deref()
    }

    async fn org_type(&self) -> OrganizationType {
        OrganizationType::from_i16(self.inner.org_type)
    }

    async fn active(&self) -> bool {
        self.inner.active
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at_ms
    }

    async fn age_ms(&self) -> BigInt {
        let now_ms = Utc::now().timestamp_millis();
        BigInt::from(now_ms.saturating_sub(self.inner.created_at_ms))
    }

    async fn root_agent(&self, ctx: &Context<'_>) -> Option<SubAgent> {
        let root_agent_id = self.inner.root_agent_id.as_deref()?;
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .get_sub_agent_by_object_id(root_agent_id)
            .await
            .ok()
            .flatten()
            .map(SubAgent::from_row)
    }

    async fn statistics(
        &self,
        ctx: &Context<'_>,
        window: Option<OrganizationStatsWindow>,
    ) -> Option<GraphOrganizationStatistics> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let window = window.unwrap_or_default().into();
        reader
            .get_organization_statistics(&self.inner.organization_id, window)
            .await
            .ok()
            .flatten()
            .map(GraphOrganizationStatistics::from_stats)
    }

    async fn agent_spend_breakdown(
        &self,
        ctx: &Context<'_>,
        window: Option<OrganizationStatsWindow>,
        #[graphql(default = 20)] limit: i32,
    ) -> Option<Vec<AgentSpendBreakdown>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_agent_spend_breakdown(
                &self.inner.organization_id,
                resolve_stats_window(window),
                limit as i64,
            )
            .await
            .ok()
            .map(|rows| {
                rows.into_iter()
                    .map(AgentSpendBreakdown::from_entry)
                    .collect()
            })
    }

    async fn roles(&self, ctx: &Context<'_>) -> Option<Vec<OrgRole>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_org_roles(&self.inner.organization_id)
            .await
            .ok()
            .map(|rows| rows.into_iter().map(OrgRole::from_row).collect())
    }

    async fn role_assignments(
        &self,
        ctx: &Context<'_>,
        member: Option<String>,
        active_only: Option<bool>,
    ) -> Option<Vec<OrgRoleAssignment>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_org_role_assignments(
                &self.inner.organization_id,
                member.as_deref(),
                active_only.unwrap_or(true),
            )
            .await
            .ok()
            .map(|rows| rows.into_iter().map(OrgRoleAssignment::from_row).collect())
    }

    async fn memory_permissions(
        &self,
        ctx: &Context<'_>,
        member: Option<String>,
        active_only: Option<bool>,
    ) -> Option<Vec<OrgMemoryPermission>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_org_memory_permissions(
                &self.inner.organization_id,
                member.as_deref(),
                active_only.unwrap_or(true),
            )
            .await
            .ok()
            .map(|rows| {
                rows.into_iter()
                    .map(OrgMemoryPermission::from_row)
                    .collect()
            })
    }

    async fn invitations(
        &self,
        ctx: &Context<'_>,
        invitee: Option<String>,
        status: Option<String>,
    ) -> Option<Vec<OrgInvitation>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_org_invitations(
                &self.inner.organization_id,
                invitee.as_deref(),
                status.as_deref(),
            )
            .await
            .ok()
            .map(|rows| rows.into_iter().map(OrgInvitation::from_row).collect())
    }

    async fn spend_approvals(
        &self,
        ctx: &Context<'_>,
        status: Option<String>,
        agent: Option<String>,
        #[graphql(default = 20)] limit: i32,
    ) -> Option<Vec<SpendApproval>> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        reader
            .list_spend_approvals_by_org(
                &self.inner.organization_id,
                status.as_deref(),
                agent.as_deref(),
                limit as i64,
            )
            .await
            .ok()
            .map(|rows| rows.into_iter().map(SpendApproval::from_row).collect())
    }

    async fn audit_log(
        &self,
        ctx: &Context<'_>,
        filter: Option<AuditLogFilterInput>,
        #[graphql(default = 20)] limit: i32,
        #[graphql(default = 0)] offset: i32,
    ) -> Option<AuditLogConnection> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let filter: AuditLogFilterInput = filter.unwrap_or_default();
        reader
            .list_audit_logs_for_org(
                &self.inner.organization_id,
                &filter.into(),
                limit as i64,
                offset as i64,
            )
            .await
            .ok()
            .map(|rows| AuditLogConnection {
                entries: rows.into_iter().map(AuditLogEntry::from_row).collect(),
            })
    }
}

#[derive(Clone)]
pub(crate) struct GraphOrganizationStatistics {
    inner: OrganizationStatistics,
}

impl GraphOrganizationStatistics {
    fn from_stats(inner: OrganizationStatistics) -> Self {
        Self { inner }
    }
}

#[Object]
impl GraphOrganizationStatistics {
    async fn total_revenue_myso(&self) -> BigInt {
        BigInt::from(self.inner.total_revenue_myso)
    }

    async fn total_outbound_spend_myso(&self) -> BigInt {
        BigInt::from(self.inner.total_outbound_spend_myso)
    }

    async fn net_cash_flow_myso(&self) -> BigInt {
        BigInt::from(self.inner.net_cash_flow_myso)
    }

    async fn total_actions_executed(&self) -> BigInt {
        BigInt::from(self.inner.total_actions_executed)
    }

    async fn total_agents(&self) -> i32 {
        self.inner.total_agents
    }

    async fn active_agents(&self) -> i32 {
        self.inner.active_agents
    }

    async fn total_counterparties(&self) -> BigInt {
        BigInt::from(self.inner.total_counterparties)
    }

    async fn total_posts(&self) -> BigInt {
        BigInt::from(self.inner.total_posts)
    }

    async fn total_engagement(&self) -> BigInt {
        BigInt::from(self.inner.total_engagement)
    }

    async fn total_spot_participation(&self) -> BigInt {
        BigInt::from(self.inner.total_spot_participation)
    }

    async fn spot_accuracy(&self) -> Option<f64> {
        self.inner.spot_accuracy
    }

    async fn spot_bets_resolved(&self) -> BigInt {
        BigInt::from(self.inner.spot_bets_resolved)
    }

    async fn insufficient_sample(&self) -> bool {
        self.inner.insufficient_sample
    }

    async fn originality_score_average(&self) -> Option<f64> {
        self.inner.originality_score_average
    }

    async fn originality_posts_analyzed(&self) -> BigInt {
        BigInt::from(self.inner.originality_posts_analyzed)
    }

    async fn estimated_assets_under_management_myso(&self) -> BigInt {
        BigInt::from(self.inner.estimated_assets_under_management_myso)
    }

    async fn attribution_coverage(&self) -> f64 {
        self.inner.attribution_coverage
    }

    async fn organization_age_ms(&self) -> BigInt {
        BigInt::from(self.inner.organization_age_ms)
    }

    async fn total_transactions(&self) -> BigInt {
        BigInt::from(self.inner.total_transactions)
    }

    async fn stats_rollup_at(&self) -> Option<GqlDateTime> {
        self.inner.stats_rollup_at.map(GqlDateTime::from_chrono)
    }

    async fn ai_credit_spent_mist(&self) -> BigInt {
        BigInt::from(self.inner.ai_credit_spent_mist)
    }

    async fn ai_credit_usage_events(&self) -> BigInt {
        BigInt::from(self.inner.ai_credit_usage_events)
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
}

#[derive(Clone)]
pub(crate) struct AgenticOrganizationLeaderboardEntry {
    inner: LeaderboardEntryRow,
}

impl AgenticOrganizationLeaderboardEntry {
    pub(crate) fn from_row(inner: LeaderboardEntryRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl AgenticOrganizationLeaderboardEntry {
    async fn rank(&self) -> i64 {
        self.inner.rank
    }

    async fn sort_value(&self) -> BigInt {
        BigInt::from(self.inner.sort_value)
    }

    async fn organization(&self) -> AgenticOrganization {
        AgenticOrganization::from_row(self.inner.organization.clone())
    }
}

#[derive(Clone)]
pub(crate) struct AgenticOrganizationLeaderboardResponse {
    pub entries: Vec<AgenticOrganizationLeaderboardEntry>,
    pub total: i64,
}

#[Object]
impl AgenticOrganizationLeaderboardResponse {
    async fn entries(&self) -> &[AgenticOrganizationLeaderboardEntry] {
        &self.entries
    }

    async fn total(&self) -> i64 {
        self.total
    }
}

impl AgenticOrganizationLeaderboardResponse {
    pub(crate) fn from_result(result: OrganizationLeaderboardResult) -> Self {
        Self {
            entries: result
                .entries
                .into_iter()
                .map(AgenticOrganizationLeaderboardEntry::from_row)
                .collect(),
            total: result.total,
        }
    }
}

#[derive(SimpleObject)]
pub(crate) struct OrganizationCategory {
    pub value: i32,
    pub slug: String,
    pub display_name: String,
}

impl From<OrganizationCategoryInfo> for OrganizationCategory {
    fn from(info: OrganizationCategoryInfo) -> Self {
        Self {
            value: i32::from(info.value),
            slug: info.slug,
            display_name: info.display_name,
        }
    }
}
