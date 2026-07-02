// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    sub_agent_organization_counterparties, sub_agent_organization_events,
    sub_agent_organization_stats, sub_agent_organization_stats_daily, sub_agent_organizations,
};

pub const ORG_TYPE_COMPANY: i16 = 0;
pub const ORG_TYPE_STARTUP: i16 = 1;
pub const ORG_TYPE_INVESTMENT_FUND: i16 = 2;
pub const ORG_TYPE_NONPROFIT: i16 = 3;
pub const ORG_TYPE_RESEARCH: i16 = 4;
pub const ORG_TYPE_GOVERNMENT: i16 = 5;
pub const ORG_TYPE_MEDIA: i16 = 6;
pub const ORG_TYPE_STEWARDSHIP: i16 = 7;
pub const ORG_TYPE_BRAND: i16 = 8;
pub const ORG_TYPE_COMMUNITY: i16 = 9;
pub const ORG_TYPE_SPORTS: i16 = 10;
pub const ORG_TYPE_EDUCATION: i16 = 11;
pub const ORG_TYPE_HEALTHCARE: i16 = 12;
pub const ORG_TYPE_OTHER: i16 = 13;
pub const ORG_TYPE_COUNT: i16 = 14;

pub const MAX_ORGANIZATIONS_PER_USER: u8 = 8;

pub const MAX_ORG_NAME_LENGTH: usize = 100;
pub const MAX_ORG_DESCRIPTION_LENGTH: usize = 1200;

pub const SPOT_ACCURACY_DISPLAY_MIN_RESOLVED: i64 = 5;
pub const SPOT_ACCURACY_LEADERBOARD_MIN_RESOLVED: i64 = 25;
pub const AUM_LEADERBOARD_MIN_ATTRIBUTION_COVERAGE_BPS: i32 = 5000;

pub const EVENT_TYPE_ORG_CREATED: &str = "created";
pub const EVENT_TYPE_ORG_UPDATED: &str = "updated";
pub const EVENT_TYPE_ORG_CATEGORY_UPDATED: &str = "category_updated";
pub const EVENT_TYPE_ORG_DEACTIVATED: &str = "deactivated";
pub const EVENT_TYPE_ORG_ROOT_AGENT_SET: &str = "root_agent_set";

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_organizations)]
pub struct NewAgenticOrganization {
    pub organization_id: String,
    pub account_id: String,
    pub principal_owner: String,
    pub profile_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub org_type: i16,
    pub root_agent_id: Option<String>,
    pub active: bool,
    pub created_at_ms: i64,
    pub deactivated_at_ms: Option<i64>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_organizations)]
pub struct AgenticOrganizationRow {
    pub organization_id: String,
    pub account_id: String,
    pub principal_owner: String,
    pub profile_id: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub org_type: i16,
    pub root_agent_id: Option<String>,
    pub active: bool,
    pub created_at_ms: i64,
    pub deactivated_at_ms: Option<i64>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_organization_events)]
pub struct NewOrganizationEvent {
    pub event_type: String,
    pub organization_id: Option<String>,
    pub account_id: Option<String>,
    pub principal_owner: Option<String>,
    pub profile_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub org_type: Option<i16>,
    pub previous_org_type: Option<i16>,
    pub root_agent_id: Option<String>,
    pub agent_object_id: Option<String>,
    pub active: Option<bool>,
    pub created_at_ms: Option<i64>,
    pub deactivated_at_ms: Option<i64>,
    pub updated_at_ms: Option<i64>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_organization_stats)]
pub struct NewOrganizationStats {
    pub organization_id: String,
    pub total_agents: i32,
    pub active_agents: i32,
    pub max_tree_depth: i16,
    pub total_posts: i64,
    pub total_comments: i64,
    pub total_reactions: i64,
    pub total_reposts: i64,
    pub total_engagement: i64,
    pub total_revenue_myso: i64,
    pub total_outbound_spend_myso: i64,
    pub net_cash_flow_myso: i64,
    pub estimated_assets_under_management_myso: i64,
    pub attribution_coverage_bps: i32,
    pub total_spot_participation: i64,
    pub spot_bets_placed: i64,
    pub spot_bets_resolved: i64,
    pub spot_bets_correct: i64,
    pub spot_accuracy_bps: Option<i32>,
    pub originality_posts_analyzed: i64,
    pub originality_score_average_bps: Option<i32>,
    pub total_counterparties: i64,
    pub total_actions_executed: i64,
    pub total_transactions: i64,
    pub last_activity_at_ms: Option<i64>,
    pub stats_rollup_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub ai_credit_spent_mist: i64,
    pub ai_credit_usage_events: i64,
    pub memory_entries: i64,
    pub memory_bytes: i64,
    pub org_shared_memory_entries: i64,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_organization_stats)]
pub struct OrganizationStatsRow {
    pub organization_id: String,
    pub total_agents: i32,
    pub active_agents: i32,
    pub max_tree_depth: i16,
    pub total_posts: i64,
    pub total_comments: i64,
    pub total_reactions: i64,
    pub total_reposts: i64,
    pub total_engagement: i64,
    pub total_revenue_myso: i64,
    pub total_outbound_spend_myso: i64,
    pub net_cash_flow_myso: i64,
    pub estimated_assets_under_management_myso: i64,
    pub attribution_coverage_bps: i32,
    pub total_spot_participation: i64,
    pub spot_bets_placed: i64,
    pub spot_bets_resolved: i64,
    pub spot_bets_correct: i64,
    pub spot_accuracy_bps: Option<i32>,
    pub originality_posts_analyzed: i64,
    pub originality_score_average_bps: Option<i32>,
    pub total_counterparties: i64,
    pub total_actions_executed: i64,
    pub total_transactions: i64,
    pub last_activity_at_ms: Option<i64>,
    pub stats_rollup_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub ai_credit_spent_mist: i64,
    pub ai_credit_usage_events: i64,
    pub memory_entries: i64,
    pub memory_bytes: i64,
    pub org_shared_memory_entries: i64,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_organization_stats_daily)]
pub struct NewOrganizationStatsDaily {
    pub organization_id: String,
    pub org_type: i16,
    pub snapshot_date: chrono::NaiveDate,
    pub total_revenue_myso: i64,
    pub net_cash_flow_myso: i64,
    pub total_outbound_spend_myso: i64,
    pub total_counterparties: i64,
    pub active_agents: i32,
    pub total_engagement: i64,
    pub estimated_aum_myso: i64,
    pub total_actions_executed: i64,
    pub growth_score: i64,
    pub spot_accuracy_bps: Option<i32>,
    pub attribution_coverage_bps: i32,
    pub time: chrono::DateTime<chrono::Utc>,
    pub ai_credit_spent_mist: i64,
    pub memory_bytes: i64,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_organization_stats_daily)]
pub struct OrganizationStatsDailyRow {
    pub organization_id: String,
    pub org_type: i16,
    pub snapshot_date: chrono::NaiveDate,
    pub total_revenue_myso: i64,
    pub net_cash_flow_myso: i64,
    pub total_outbound_spend_myso: i64,
    pub total_counterparties: i64,
    pub active_agents: i32,
    pub total_engagement: i64,
    pub estimated_aum_myso: i64,
    pub total_actions_executed: i64,
    pub growth_score: i64,
    pub spot_accuracy_bps: Option<i32>,
    pub attribution_coverage_bps: i32,
    pub time: chrono::DateTime<chrono::Utc>,
    pub ai_credit_spent_mist: i64,
    pub memory_bytes: i64,
}

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_organization_counterparties)]
pub struct NewOrganizationCounterparty {
    pub organization_id: String,
    pub counterparty_address: String,
    pub first_interaction_at_ms: i64,
    pub last_interaction_at_ms: i64,
    pub interaction_count: i64,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = sub_agent_organization_counterparties)]
pub struct OrganizationCounterpartyRow {
    pub organization_id: String,
    pub counterparty_address: String,
    pub first_interaction_at_ms: i64,
    pub last_interaction_at_ms: i64,
    pub interaction_count: i64,
}
