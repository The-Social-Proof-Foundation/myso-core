// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

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
    NewAiCreditAgentBudget, NewAiCreditBalance, NewAiCreditConfig, NewAiCreditEvent,
};
use myso_indexer_alt_social_schema::schema::{
    ai_credit_agent_budgets, ai_credit_balances, ai_credit_config, ai_credit_events, profiles,
};

use super::ai_credit;
use super::common;
use super::events;
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
    ProfileAiCreditBalanceLink {
        profile_id: String,
        ai_credit_balance_id: String,
    },
    ConfigUpsert(NewAiCreditConfig),
    ConfigLimitsUpdate {
        max_single_settlement_mist: i64,
        receipt_ttl_ms: i64,
        updated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    Event(NewAiCreditEvent),
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
                max_single_settlement_mist,
                receipt_ttl_ms,
                updated_at_ms,
                event_id,
                transaction_id,
            } => Some(AiCreditRow::ConfigLimitsUpdate {
                max_single_settlement_mist,
                receipt_ttl_ms,
                updated_at_ms,
                event_id,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::AiCreditEvent(e) => Some(AiCreditRow::Event(e)),
            _ => None,
        }
    }
}

impl FieldCount for AiCreditRow {
    const FIELD_COUNT: usize = 8;
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
                let event_data =
                    match events::parse_event_contents(AI_CREDIT_MODULE, event_name, &ev.contents) {
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
                            SocialMetrics::record_event_bcs_parse_failed(
                                AI_CREDIT_MODULE,
                                event_name,
                            );
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
        for row in values {
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
                    total += diesel::update(
                        ai_credit_balances::table.filter(ai_credit_balances::balance_id.eq(balance_id)),
                    )
                    .set((
                        ai_credit_balances::balance_mist.eq(*balance_mist),
                        ai_credit_balances::updated_at_ms.eq(*updated_at_ms),
                        ai_credit_balances::event_id.eq(event_id),
                        ai_credit_balances::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
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
                        ai_credit_balances::table.filter(ai_credit_balances::balance_id.eq(balance_id)),
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
                    total += diesel::update(
                        ai_credit_balances::table.filter(ai_credit_balances::balance_id.eq(balance_id)),
                    )
                    .set((
                        ai_credit_balances::settlement_nonce.eq(*settlement_nonce),
                        ai_credit_balances::spent_total_mist.eq(
                            ai_credit_balances::spent_total_mist + *spent_increment_mist,
                        ),
                        ai_credit_balances::updated_at_ms.eq(*updated_at_ms),
                        ai_credit_balances::event_id.eq(event_id),
                        ai_credit_balances::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                AiCreditRow::BalanceActiveUpdate {
                    balance_id,
                    active,
                    updated_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        ai_credit_balances::table.filter(ai_credit_balances::balance_id.eq(balance_id)),
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
                    total += diesel::update(
                        ai_credit_agent_budgets::table
                            .filter(ai_credit_agent_budgets::balance_id.eq(balance_id))
                            .filter(ai_credit_agent_budgets::agent_object_id.eq(agent_object_id)),
                    )
                    .set((
                        ai_credit_agent_budgets::spent_mist.eq(
                            ai_credit_agent_budgets::spent_mist + *spent_increment_mist,
                        ),
                        ai_credit_agent_budgets::updated_at_ms.eq(*updated_at_ms),
                        ai_credit_agent_budgets::event_id.eq(event_id),
                        ai_credit_agent_budgets::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                AiCreditRow::ProfileAiCreditBalanceLink {
                    profile_id,
                    ai_credit_balance_id,
                } => {
                    total += diesel::update(
                        profiles::table.filter(profiles::profile_id.eq(profile_id)),
                    )
                    .set(profiles::ai_credit_balance_id.eq(ai_credit_balance_id))
                    .execute(conn)
                    .await?;
                }
                AiCreditRow::ConfigUpsert(c) => {
                    let row = c.clone();
                    total += diesel::insert_into(ai_credit_config::table)
                        .values(&row)
                        .on_conflict(ai_credit_config::id)
                        .do_update()
                        .set((
                            ai_credit_config::oracle_pubkey_hex.eq(&row.oracle_pubkey_hex),
                            ai_credit_config::treasury_address.eq(&row.treasury_address),
                            ai_credit_config::min_deposit_mist.eq(row.min_deposit_mist),
                            ai_credit_config::max_single_settlement_mist
                                .eq(row.max_single_settlement_mist),
                            ai_credit_config::receipt_ttl_ms.eq(row.receipt_ttl_ms),
                            ai_credit_config::catalog_version.eq(&row.catalog_version),
                            ai_credit_config::updated_at_ms.eq(row.updated_at_ms),
                            ai_credit_config::event_id.eq(&row.event_id),
                            ai_credit_config::transaction_id.eq(&row.transaction_id),
                            ai_credit_config::time.eq(row.time),
                        ))
                        .execute(conn)
                        .await?;
                }
                AiCreditRow::ConfigLimitsUpdate {
                    max_single_settlement_mist,
                    receipt_ttl_ms,
                    updated_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(ai_credit_config::table.filter(ai_credit_config::id.eq(1i16)))
                        .set((
                            ai_credit_config::max_single_settlement_mist.eq(*max_single_settlement_mist),
                            ai_credit_config::receipt_ttl_ms.eq(*receipt_ttl_ms),
                            ai_credit_config::updated_at_ms.eq(*updated_at_ms),
                            ai_credit_config::event_id.eq(event_id),
                            ai_credit_config::transaction_id.eq(transaction_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                AiCreditRow::Event(e) => {
                    total += diesel::insert_into(ai_credit_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
