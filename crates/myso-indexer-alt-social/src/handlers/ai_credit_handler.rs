// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Nullable, Text, Timestamptz};
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewAiCreditAgentBudget, NewAiCreditBalance, NewAiCreditConfig, NewAiCreditEvent,
    NewAiCreditSpendApproval, NewAuditLog,
};
use myso_indexer_alt_social_schema::schema::{
    ai_credit_agent_budgets, ai_credit_balances, ai_credit_config, ai_credit_events,
    ai_credit_spend_approvals, ai_credit_usage_lines, profiles, sub_agents,
};

use super::ai_credit;
use super::common;
use super::events;
use super::organization_stats::apply_org_ai_credit_spend;
use crate::metrics::SocialMetrics;

const AI_CREDIT_MODULE: &str = "ai_credit";

#[derive(Debug, Clone)]
pub enum AiCreditRow {
    BalanceUpsert(NewAiCreditBalance),
    BalanceBalanceUpdate {
        balance_id: String,
        balance_mist: i64,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    BalanceCapsUpdate {
        balance_id: String,
        daily_cap_mist: Option<i64>,
        monthly_cap_mist: Option<i64>,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    BalanceSettlementUpdate {
        balance_id: String,
        settlement_nonce: i64,
        spent_increment_mist: i64,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    BalanceActiveUpdate {
        balance_id: String,
        active: bool,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    AgentBudgetUpsert(NewAiCreditAgentBudget),
    AgentBudgetDisable {
        balance_id: String,
        agent_object_id: String,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    AgentBudgetSpendUpdate {
        balance_id: String,
        agent_object_id: String,
        spent_increment_mist: i64,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    UsageLineSettle {
        receipt_id: String,
        settlement_tx: String,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    ProfileAiCreditBalanceLink {
        profile_id: String,
        ai_credit_balance_id: String,
    },
    ConfigUpsert(NewAiCreditConfig),
    ConfigLimitsUpdate {
        updated_by: String,
        max_single_settlement_mist: i64,
        receipt_ttl_ms: i64,
        updated_at: i64,
        time: chrono::DateTime<chrono::Utc>,
        event_id: String,
        transaction_id: String,
    },
    ConfigPubkeyUpdate {
        updated_by: String,
        oracle_pubkey_hex: String,
        updated_at: i64,
        time: chrono::DateTime<chrono::Utc>,
        event_id: String,
        transaction_id: String,
    },
    ConfigMarkupUpdate {
        updated_by: String,
        oracle_markup_bps: i64,
        updated_at: i64,
        time: chrono::DateTime<chrono::Utc>,
        event_id: String,
        transaction_id: String,
    },
    ConfigMinDepositUpdate {
        updated_by: String,
        min_deposit_mist: i64,
        updated_at: i64,
        time: chrono::DateTime<chrono::Utc>,
        event_id: String,
        transaction_id: String,
    },
    Event(NewAiCreditEvent),
    SpendApprovalUpsert(NewAiCreditSpendApproval),
    SpendApprovalStatus {
        balance_id: String,
        agent_object_id: String,
        status: String,
        consumed_amount_mist: Option<i64>,
        event_id: String,
    },
    OrgSpendFromAgent {
        agent_object_id: String,
        amount_mist: i64,
        receipt_id: Option<String>,
        activity_at_ms: i64,
    },
    AuditLog(NewAuditLog),
}

impl AiCreditRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::AiCreditBalanceUpsert(b) => {
                Some(AiCreditRow::BalanceUpsert(b))
            }
            crate::handlers::SocialEventRow::AiCreditBalanceBalanceUpdate {
                balance_id,
                balance_mist,
                updated_at_ms,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::BalanceBalanceUpdate {
                balance_id,
                balance_mist,
                updated_at_ms,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditBalanceCapsUpdate {
                balance_id,
                daily_cap_mist,
                monthly_cap_mist,
                updated_at_ms,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::BalanceCapsUpdate {
                balance_id,
                daily_cap_mist,
                monthly_cap_mist,
                updated_at_ms,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditBalanceSettlementUpdate {
                balance_id,
                settlement_nonce,
                spent_increment_mist,
                updated_at_ms,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::BalanceSettlementUpdate {
                balance_id,
                settlement_nonce,
                spent_increment_mist,
                updated_at_ms,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditBalanceActiveUpdate {
                balance_id,
                active,
                updated_at_ms,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::BalanceActiveUpdate {
                balance_id,
                active,
                updated_at_ms,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditAgentBudgetUpsert(b) => {
                Some(AiCreditRow::AgentBudgetUpsert(b))
            }
            crate::handlers::SocialEventRow::AiCreditAgentBudgetDisable {
                balance_id,
                agent_object_id,
                updated_at_ms,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::AgentBudgetDisable {
                balance_id,
                agent_object_id,
                updated_at_ms,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditAgentBudgetSpendUpdate {
                balance_id,
                agent_object_id,
                spent_increment_mist,
                updated_at_ms,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::AgentBudgetSpendUpdate {
                balance_id,
                agent_object_id,
                spent_increment_mist,
                updated_at_ms,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditUsageLineSettle {
                receipt_id,
                settlement_tx,
                updated_at_ms,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::UsageLineSettle {
                receipt_id,
                settlement_tx,
                updated_at_ms,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::ProfileAiCreditBalanceLink {
                profile_id,
                ai_credit_balance_id,
            } => Some(AiCreditRow::ProfileAiCreditBalanceLink {
                profile_id,
                ai_credit_balance_id,
            }),
            crate::handlers::SocialEventRow::AiCreditConfigUpsert(c) => {
                Some(AiCreditRow::ConfigUpsert(c))
            }
            crate::handlers::SocialEventRow::AiCreditConfigLimitsUpdate {
                updated_by,
                max_single_settlement_mist,
                receipt_ttl_ms,
                updated_at,
                time,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::ConfigLimitsUpdate {
                updated_by,
                max_single_settlement_mist,
                receipt_ttl_ms,
                updated_at,
                time,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditConfigPubkeyUpdate {
                updated_by,
                oracle_pubkey_hex,
                updated_at,
                time,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::ConfigPubkeyUpdate {
                updated_by,
                oracle_pubkey_hex,
                updated_at,
                time,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditConfigMarkupUpdate {
                updated_by,
                oracle_markup_bps,
                updated_at,
                time,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::ConfigMarkupUpdate {
                updated_by,
                oracle_markup_bps,
                updated_at,
                time,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditConfigMinDepositUpdate {
                updated_by,
                min_deposit_mist,
                updated_at,
                time,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::ConfigMinDepositUpdate {
                updated_by,
                min_deposit_mist,
                updated_at,
                time,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditEvent(e) => Some(AiCreditRow::Event(e)),
            crate::handlers::SocialEventRow::AiCreditSpendApprovalUpsert(a) => {
                Some(AiCreditRow::SpendApprovalUpsert(a))
            }
            crate::handlers::SocialEventRow::AiCreditSpendApprovalStatus {
                balance_id,
                agent_object_id,
                status,
                consumed_amount_mist,
                event_id,
            } => Some(AiCreditRow::SpendApprovalStatus {
                balance_id,
                agent_object_id,
                status,
                consumed_amount_mist,
                event_id,
            }),
            crate::handlers::SocialEventRow::AiCreditOrgSpendFromAgent {
                agent_object_id,
                amount_mist,
                receipt_id,
                activity_at_ms,
            } => Some(AiCreditRow::OrgSpendFromAgent {
                agent_object_id,
                amount_mist,
                receipt_id,
                activity_at_ms,
            }),
            crate::handlers::SocialEventRow::AuditLog(a) => Some(AiCreditRow::AuditLog(a)),
            _ => None,
        }
    }
}

impl FieldCount for AiCreditRow {
    const FIELD_COUNT: usize = 8;
}

#[derive(QueryableByName)]
struct LatestAiCreditConfigRow {
    #[diesel(sql_type = Text)]
    updated_by: String,
    #[diesel(sql_type = Text)]
    oracle_pubkey_hex: String,
    #[diesel(sql_type = Text)]
    treasury_address: String,
    #[diesel(sql_type = BigInt)]
    min_deposit_mist: i64,
    #[diesel(sql_type = BigInt)]
    max_single_settlement_mist: i64,
    #[diesel(sql_type = BigInt)]
    receipt_ttl_ms: i64,
    #[diesel(sql_type = BigInt)]
    oracle_markup_bps: i64,
    #[diesel(sql_type = Nullable<Text>)]
    catalog_version: Option<String>,
    #[diesel(sql_type = BigInt)]
    version: i64,
    #[diesel(sql_type = BigInt)]
    updated_at: i64,
    #[diesel(sql_type = Text)]
    event_id: String,
    #[diesel(sql_type = Text)]
    transaction_id: String,
    #[diesel(sql_type = Timestamptz)]
    time: chrono::DateTime<chrono::Utc>,
}

fn latest_row_to_new(row: LatestAiCreditConfigRow) -> NewAiCreditConfig {
    NewAiCreditConfig {
        updated_by: row.updated_by,
        oracle_pubkey_hex: row.oracle_pubkey_hex,
        treasury_address: row.treasury_address,
        min_deposit_mist: row.min_deposit_mist,
        max_single_settlement_mist: row.max_single_settlement_mist,
        receipt_ttl_ms: row.receipt_ttl_ms,
        oracle_markup_bps: row.oracle_markup_bps,
        catalog_version: row.catalog_version,
        version: row.version,
        updated_at: row.updated_at,
        event_id: row.event_id,
        transaction_id: row.transaction_id,
        time: row.time,
    }
}

async fn load_latest_ai_credit_config(
    conn: &mut Connection<'_>,
) -> Result<Option<NewAiCreditConfig>> {
    let query = "
        SELECT updated_by, oracle_pubkey_hex, treasury_address, min_deposit_mist,
               max_single_settlement_mist, receipt_ttl_ms, oracle_markup_bps, catalog_version,
               version, updated_at, event_id, transaction_id, time
        FROM ai_credit_config
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .get_result::<LatestAiCreditConfigRow>(conn)
        .await
        .optional()?;
    Ok(result.map(latest_row_to_new))
}

fn next_ai_credit_version(prev: &NewAiCreditConfig, incoming_version: Option<i64>) -> i64 {
    incoming_version.unwrap_or(prev.version + 1)
}

fn finalize_ai_credit_config(prev: &NewAiCreditConfig, row: &AiCreditRow) -> Option<NewAiCreditConfig> {
    match row {
        AiCreditRow::ConfigUpsert(config) => Some(config.clone()),
        AiCreditRow::ConfigLimitsUpdate {
            updated_by,
            max_single_settlement_mist,
            receipt_ttl_ms,
            updated_at,
            time,
            event_id,
            transaction_id,
        } => Some(NewAiCreditConfig {
            updated_by: updated_by.clone(),
            max_single_settlement_mist: *max_single_settlement_mist,
            receipt_ttl_ms: *receipt_ttl_ms,
            version: next_ai_credit_version(prev, None),
            updated_at: *updated_at,
            event_id: event_id.clone(),
            transaction_id: transaction_id.clone(),
            time: *time,
            ..prev.clone()
        }),
        AiCreditRow::ConfigPubkeyUpdate {
            updated_by,
            oracle_pubkey_hex,
            updated_at,
            time,
            event_id,
            transaction_id,
        } => Some(NewAiCreditConfig {
            updated_by: updated_by.clone(),
            oracle_pubkey_hex: oracle_pubkey_hex.clone(),
            version: next_ai_credit_version(prev, None),
            updated_at: *updated_at,
            event_id: event_id.clone(),
            transaction_id: transaction_id.clone(),
            time: *time,
            ..prev.clone()
        }),
        AiCreditRow::ConfigMarkupUpdate {
            updated_by,
            oracle_markup_bps,
            updated_at,
            time,
            event_id,
            transaction_id,
        } => Some(NewAiCreditConfig {
            updated_by: updated_by.clone(),
            oracle_markup_bps: *oracle_markup_bps,
            version: next_ai_credit_version(prev, None),
            updated_at: *updated_at,
            event_id: event_id.clone(),
            transaction_id: transaction_id.clone(),
            time: *time,
            ..prev.clone()
        }),
        AiCreditRow::ConfigMinDepositUpdate {
            updated_by,
            min_deposit_mist,
            updated_at,
            time,
            event_id,
            transaction_id,
        } => Some(NewAiCreditConfig {
            updated_by: updated_by.clone(),
            min_deposit_mist: *min_deposit_mist,
            version: next_ai_credit_version(prev, None),
            updated_at: *updated_at,
            event_id: event_id.clone(),
            transaction_id: transaction_id.clone(),
            time: *time,
            ..prev.clone()
        }),
        _ => None,
    }
}

pub struct AiCreditHandler;

#[async_trait]
impl Processor for AiCreditHandler {
    const NAME: &'static str = "ai_credit";

    type Value = AiCreditRow;

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
                if ev.type_.module.as_str() != AI_CREDIT_MODULE {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data = match events::parse_event_contents(
                    AI_CREDIT_MODULE,
                    event_name,
                    &ev.contents,
                ) {
                    Ok(v) => v,
                    Err(e) => {
                        tracing::warn!(
                            tx_digest = %tx_digest,
                            module = AI_CREDIT_MODULE,
                            event_name,
                            error = %e,
                            hex_preview = %e.contents_hex_preview(48),
                            "ai_credit pipeline: event contents parse failed; skipping event"
                        );
                        SocialMetrics::record_event_bcs_parse_failed(AI_CREDIT_MODULE, event_name);
                        continue;
                    }
                };
                if let Some(rows) =
                    ai_credit::handle_ai_credit_event(event_name, &event_data, &event_id)
                {
                    for row in rows {
                        if let Some(r) = AiCreditRow::from_social(row) {
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
impl Handler for AiCreditHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        let mut running_latest = load_latest_ai_credit_config(conn)
            .await?
            .unwrap_or_else(ai_credit::new_ai_credit_config_with_defaults);

        for row in values {
            if let Some(merged) = finalize_ai_credit_config(&running_latest, row) {
                total += diesel::insert_into(ai_credit_config::table)
                    .values(&merged)
                    .execute(conn)
                    .await?;
                running_latest = merged;
                continue;
            }

            match row {
                AiCreditRow::BalanceUpsert(b) => {
                    total += diesel::insert_into(ai_credit_balances::table)
                        .values(b)
                        .on_conflict(ai_credit_balances::balance_id)
                        .do_update()
                        .set((
                            ai_credit_balances::memory_account_id.eq(b.memory_account_id.clone()),
                            ai_credit_balances::principal_owner.eq(b.principal_owner.clone()),
                            ai_credit_balances::profile_id.eq(b.profile_id.clone()),
                            ai_credit_balances::balance_mist.eq(b.balance_mist),
                            ai_credit_balances::spent_total_mist.eq(b.spent_total_mist),
                            ai_credit_balances::daily_cap_mist.eq(b.daily_cap_mist),
                            ai_credit_balances::monthly_cap_mist.eq(b.monthly_cap_mist),
                            ai_credit_balances::spent_day_mist.eq(b.spent_day_mist),
                            ai_credit_balances::spent_month_mist.eq(b.spent_month_mist),
                            ai_credit_balances::settlement_nonce.eq(b.settlement_nonce),
                            ai_credit_balances::active.eq(b.active),
                            ai_credit_balances::updated_at_ms.eq(b.updated_at_ms),
                            ai_credit_balances::event_id.eq(b.event_id.clone()),
                            ai_credit_balances::transaction_id.eq(b.transaction_id.clone()),
                            ai_credit_balances::time.eq(b.time),
                        ))
                        .execute(conn)
                        .await?;
                }
                AiCreditRow::BalanceBalanceUpdate {
                    balance_id,
                    balance_mist,
                    updated_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    let affected = diesel::update(
                        ai_credit_balances::table
                            .filter(ai_credit_balances::balance_id.eq(balance_id)),
                    )
                    .set((
                        ai_credit_balances::balance_mist.eq(*balance_mist),
                        ai_credit_balances::updated_at_ms.eq(*updated_at_ms),
                        ai_credit_balances::event_id.eq(event_id),
                        ai_credit_balances::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    if affected == 0 {
                        tracing::warn!(
                            balance_id = %balance_id,
                            event_id = %event_id,
                            transaction_id = %transaction_id,
                            "ai_credit balance balance_mist update matched no rows"
                        );
                    }
                    total += affected;
                }
                AiCreditRow::BalanceCapsUpdate {
                    balance_id,
                    daily_cap_mist,
                    monthly_cap_mist,
                    updated_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        ai_credit_balances::table
                            .filter(ai_credit_balances::balance_id.eq(balance_id)),
                    )
                    .set((
                        ai_credit_balances::daily_cap_mist.eq(*daily_cap_mist),
                        ai_credit_balances::monthly_cap_mist.eq(*monthly_cap_mist),
                        ai_credit_balances::updated_at_ms.eq(*updated_at_ms),
                        ai_credit_balances::event_id.eq(event_id),
                        ai_credit_balances::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                AiCreditRow::BalanceSettlementUpdate {
                    balance_id,
                    settlement_nonce,
                    spent_increment_mist,
                    updated_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    let affected = diesel::update(
                        ai_credit_balances::table.filter(
                            ai_credit_balances::balance_id
                                .eq(balance_id)
                                .and(ai_credit_balances::settlement_nonce.lt(*settlement_nonce)),
                        ),
                    )
                    .set((
                        ai_credit_balances::settlement_nonce.eq(*settlement_nonce),
                        ai_credit_balances::spent_total_mist
                            .eq(ai_credit_balances::spent_total_mist + *spent_increment_mist),
                        ai_credit_balances::spent_day_mist
                            .eq(ai_credit_balances::spent_day_mist + *spent_increment_mist),
                        ai_credit_balances::spent_month_mist
                            .eq(ai_credit_balances::spent_month_mist + *spent_increment_mist),
                        ai_credit_balances::updated_at_ms.eq(*updated_at_ms),
                        ai_credit_balances::event_id.eq(event_id),
                        ai_credit_balances::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    if affected == 0 {
                        tracing::warn!(
                            balance_id = %balance_id,
                            settlement_nonce = %settlement_nonce,
                            event_id = %event_id,
                            transaction_id = %transaction_id,
                            "ai_credit balance settlement update matched no rows"
                        );
                    }
                    total += affected;
                }
                AiCreditRow::BalanceActiveUpdate {
                    balance_id,
                    active,
                    updated_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        ai_credit_balances::table
                            .filter(ai_credit_balances::balance_id.eq(balance_id)),
                    )
                    .set((
                        ai_credit_balances::active.eq(*active),
                        ai_credit_balances::updated_at_ms.eq(*updated_at_ms),
                        ai_credit_balances::event_id.eq(event_id),
                        ai_credit_balances::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                AiCreditRow::AgentBudgetUpsert(b) => {
                    total += diesel::insert_into(ai_credit_agent_budgets::table)
                        .values(b)
                        .on_conflict((
                            ai_credit_agent_budgets::balance_id,
                            ai_credit_agent_budgets::agent_object_id,
                        ))
                        .do_update()
                        .set((
                            ai_credit_agent_budgets::budget_mist.eq(b.budget_mist),
                            ai_credit_agent_budgets::daily_cap_mist.eq(b.daily_cap_mist),
                            ai_credit_agent_budgets::monthly_cap_mist.eq(b.monthly_cap_mist),
                            ai_credit_agent_budgets::require_approval_above_mist
                                .eq(b.require_approval_above_mist),
                            ai_credit_agent_budgets::enabled.eq(b.enabled),
                            ai_credit_agent_budgets::updated_at_ms.eq(b.updated_at_ms),
                            ai_credit_agent_budgets::event_id.eq(b.event_id.clone()),
                            ai_credit_agent_budgets::transaction_id.eq(b.transaction_id.clone()),
                            ai_credit_agent_budgets::time.eq(b.time),
                        ))
                        .execute(conn)
                        .await?;
                }
                AiCreditRow::AgentBudgetDisable {
                    balance_id,
                    agent_object_id,
                    updated_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        ai_credit_agent_budgets::table
                            .filter(ai_credit_agent_budgets::balance_id.eq(balance_id))
                            .filter(ai_credit_agent_budgets::agent_object_id.eq(agent_object_id)),
                    )
                    .set((
                        ai_credit_agent_budgets::enabled.eq(false),
                        ai_credit_agent_budgets::updated_at_ms.eq(*updated_at_ms),
                        ai_credit_agent_budgets::event_id.eq(event_id),
                        ai_credit_agent_budgets::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                AiCreditRow::AgentBudgetSpendUpdate {
                    balance_id,
                    agent_object_id,
                    spent_increment_mist,
                    updated_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    let affected = diesel::update(
                        ai_credit_agent_budgets::table
                            .filter(ai_credit_agent_budgets::balance_id.eq(balance_id))
                            .filter(ai_credit_agent_budgets::agent_object_id.eq(agent_object_id)),
                    )
                    .set((
                        ai_credit_agent_budgets::spent_mist
                            .eq(ai_credit_agent_budgets::spent_mist + *spent_increment_mist),
                        ai_credit_agent_budgets::updated_at_ms.eq(*updated_at_ms),
                        ai_credit_agent_budgets::event_id.eq(event_id),
                        ai_credit_agent_budgets::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    if affected == 0 {
                        tracing::warn!(
                            balance_id = %balance_id,
                            agent_object_id = %agent_object_id,
                            event_id = %event_id,
                            transaction_id = %transaction_id,
                            "ai_credit agent budget spend update matched no rows"
                        );
                    }
                    total += affected;
                }
                AiCreditRow::UsageLineSettle {
                    receipt_id,
                    settlement_tx,
                    updated_at_ms: _,
                    event_id,
                    transaction_id,
                } => {
                    let affected = diesel::update(
                        ai_credit_usage_lines::table
                            .filter(ai_credit_usage_lines::receipt_id.eq(receipt_id))
                            .filter(ai_credit_usage_lines::settled.eq(false)),
                    )
                    .set((
                        ai_credit_usage_lines::settled.eq(true),
                        ai_credit_usage_lines::settlement_tx.eq(settlement_tx),
                    ))
                    .execute(conn)
                    .await?;
                    if affected == 0 {
                        tracing::warn!(
                            receipt_id = %receipt_id,
                            event_id = %event_id,
                            transaction_id = %transaction_id,
                            "ai_credit usage line settle matched no rows (ingest may lag settlement)"
                        );
                    }
                    total += affected;
                }
                AiCreditRow::ProfileAiCreditBalanceLink {
                    profile_id,
                    ai_credit_balance_id,
                } => {
                    total +=
                        diesel::update(profiles::table.filter(profiles::profile_id.eq(profile_id)))
                            .set(profiles::ai_credit_balance_id.eq(ai_credit_balance_id))
                            .execute(conn)
                            .await?;
                }
                AiCreditRow::ConfigUpsert(_)
                | AiCreditRow::ConfigLimitsUpdate { .. }
                | AiCreditRow::ConfigPubkeyUpdate { .. }
                | AiCreditRow::ConfigMarkupUpdate { .. }
                | AiCreditRow::ConfigMinDepositUpdate { .. } => {}
                AiCreditRow::Event(e) => {
                    total += diesel::insert_into(ai_credit_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                AiCreditRow::SpendApprovalUpsert(a) => {
                    total += diesel::insert_into(ai_credit_spend_approvals::table)
                        .values(a)
                        .on_conflict((
                            ai_credit_spend_approvals::balance_id,
                            ai_credit_spend_approvals::agent_object_id,
                        ))
                        .do_update()
                        .set((
                            ai_credit_spend_approvals::status.eq(a.status.clone()),
                            ai_credit_spend_approvals::approval_nonce.eq(a.approval_nonce),
                            ai_credit_spend_approvals::max_amount_mist.eq(a.max_amount_mist),
                            ai_credit_spend_approvals::expires_at_ms.eq(a.expires_at_ms),
                            ai_credit_spend_approvals::approved_by.eq(a.approved_by.clone()),
                            ai_credit_spend_approvals::approved_by_agent_id
                                .eq(a.approved_by_agent_id.clone()),
                            ai_credit_spend_approvals::organization_id
                                .eq(a.organization_id.clone()),
                            ai_credit_spend_approvals::updated_at.eq(a.updated_at),
                            ai_credit_spend_approvals::event_id.eq(a.event_id.clone()),
                        ))
                        .execute(conn)
                        .await?;
                }
                AiCreditRow::SpendApprovalStatus {
                    balance_id,
                    agent_object_id,
                    status,
                    consumed_amount_mist,
                    event_id,
                } => {
                    let affected = diesel::update(
                        ai_credit_spend_approvals::table
                            .filter(ai_credit_spend_approvals::balance_id.eq(balance_id))
                            .filter(ai_credit_spend_approvals::agent_object_id.eq(agent_object_id)),
                    )
                    .set((
                        ai_credit_spend_approvals::status.eq(status),
                        ai_credit_spend_approvals::consumed_amount_mist.eq(*consumed_amount_mist),
                        ai_credit_spend_approvals::updated_at.eq(chrono::Utc::now()),
                        ai_credit_spend_approvals::event_id.eq(Some(event_id.clone())),
                    ))
                    .execute(conn)
                    .await?;
                    if affected == 0 {
                        tracing::warn!(
                            balance_id = %balance_id,
                            agent_object_id = %agent_object_id,
                            status = %status,
                            "ai_credit spend approval status update matched no rows"
                        );
                    }
                    total += affected;
                }
                AiCreditRow::OrgSpendFromAgent {
                    agent_object_id,
                    amount_mist,
                    receipt_id,
                    activity_at_ms,
                } => {
                    let org_id = sub_agents::table
                        .filter(sub_agents::agent_object_id.eq(agent_object_id))
                        .select(sub_agents::organization_id)
                        .first::<Option<String>>(conn)
                        .await
                        .ok()
                        .flatten();
                    if let Some(organization_id) = &org_id {
                        apply_org_ai_credit_spend(
                            conn,
                            Some(organization_id),
                            *amount_mist,
                            *activity_at_ms,
                        )
                        .await?;
                        // Backfill org attribution on the usage line when the oracle
                        // ingest predates the agent's org registration.
                        if let Some(receipt_id) = receipt_id {
                            diesel::update(
                                ai_credit_usage_lines::table
                                    .filter(ai_credit_usage_lines::receipt_id.eq(receipt_id))
                                    .filter(ai_credit_usage_lines::organization_id.is_null()),
                            )
                            .set(
                                ai_credit_usage_lines::organization_id
                                    .eq(Some(organization_id.clone())),
                            )
                            .execute(conn)
                            .await?;
                        }
                    }
                    total += 1;
                }
                AiCreditRow::AuditLog(a) => {
                    total += diesel::insert_into(
                        myso_indexer_alt_social_schema::schema::audit_log::table,
                    )
                    .values(a)
                    .execute(conn)
                    .await?;
                }
            }
        }
        Ok(total)
    }
}
