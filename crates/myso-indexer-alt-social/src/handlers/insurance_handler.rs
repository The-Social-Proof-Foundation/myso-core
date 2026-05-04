// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Insurance pipeline: indexes insurance module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Int2, Text};
use diesel::ExpressionMethods;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewInsuranceConfig, NewInsuranceCoverageRoute, NewInsuranceEventLog, NewInsuranceMarketExposure,
    NewInsurancePolicy, NewInsurancePolicyEvent, NewInsuranceRouteFill, NewInsuranceUserExposure,
    NewInsuranceVault, NewInsuranceVaultTransaction, UpdateInsuranceVaultStatus,
};
use myso_indexer_alt_social_schema::schema::{
    insurance_config, insurance_coverage_routes, insurance_events, insurance_market_exposures,
    insurance_policies, insurance_policy_events, insurance_route_fills, insurance_user_exposures,
    insurance_vault_transactions, insurance_vaults,
};

use super::common;
use super::events;
use super::insurance;

const INSURANCE_MODULES: &[&str] = &["insurance"];

#[derive(Debug, Clone)]
pub enum InsuranceRow {
    InsuranceConfig(NewInsuranceConfig),
    InsuranceVault(NewInsuranceVault),
    InsuranceVaultTransaction(NewInsuranceVaultTransaction),
    InsuranceVaultBalanceUpdate {
        vault_id: String,
        new_balance: i64,
    },
    InsurancePolicy(NewInsurancePolicy),
    InsurancePolicyEvent(NewInsurancePolicyEvent),
    InsuranceMarketExposure(NewInsuranceMarketExposure),
    InsuranceUserExposure(NewInsuranceUserExposure),
    InsuranceEventLog(NewInsuranceEventLog),
    InsurancePolicyStatusUpdate {
        policy_id: String,
        status: i16,
    },
    InsurancePolicyEventFromPolicy {
        policy_id: String,
        event_type: String,
        refunded_amount: Option<i64>,
        fee_paid: Option<i64>,
        payout: Option<i64>,
        reserve_released: Option<i64>,
        timestamp_ms: i64,
        transaction_id: String,
    },
    InsuranceCoverageRoute(NewInsuranceCoverageRoute),
    InsuranceRouteFill(NewInsuranceRouteFill),
    InsuranceVaultOperationalUpdate {
        vault_id: String,
        max_exposure_per_option: i64,
        enabled: bool,
        paused: bool,
        max_exposure_per_market: i64,
        max_exposure_per_user: i64,
        base_rate_bps_per_day: i64,
        utilization_multiplier_bps: i64,
    },
}

impl InsuranceRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::InsuranceConfig(c) => {
                Some(InsuranceRow::InsuranceConfig(c))
            }
            crate::handlers::SocialEventRow::InsuranceVault(v) => {
                Some(InsuranceRow::InsuranceVault(v))
            }
            crate::handlers::SocialEventRow::InsuranceVaultTransaction(t) => {
                Some(InsuranceRow::InsuranceVaultTransaction(t))
            }
            crate::handlers::SocialEventRow::InsuranceVaultBalanceUpdate {
                vault_id,
                new_balance,
            } => Some(InsuranceRow::InsuranceVaultBalanceUpdate {
                vault_id,
                new_balance,
            }),
            crate::handlers::SocialEventRow::InsurancePolicy(p) => {
                Some(InsuranceRow::InsurancePolicy(p))
            }
            crate::handlers::SocialEventRow::InsurancePolicyEvent(pe) => {
                Some(InsuranceRow::InsurancePolicyEvent(pe))
            }
            crate::handlers::SocialEventRow::InsuranceMarketExposure(me) => {
                Some(InsuranceRow::InsuranceMarketExposure(me))
            }
            crate::handlers::SocialEventRow::InsuranceUserExposure(ue) => {
                Some(InsuranceRow::InsuranceUserExposure(ue))
            }
            crate::handlers::SocialEventRow::InsuranceEventLog(log) => {
                Some(InsuranceRow::InsuranceEventLog(log))
            }
            crate::handlers::SocialEventRow::InsurancePolicyStatusUpdate { policy_id, status } => {
                Some(InsuranceRow::InsurancePolicyStatusUpdate { policy_id, status })
            }
            crate::handlers::SocialEventRow::InsurancePolicyEventFromPolicy {
                policy_id,
                event_type,
                refunded_amount,
                fee_paid,
                payout,
                reserve_released,
                timestamp_ms,
                transaction_id,
            } => Some(InsuranceRow::InsurancePolicyEventFromPolicy {
                policy_id,
                event_type,
                refunded_amount,
                fee_paid,
                payout,
                reserve_released,
                timestamp_ms,
                transaction_id,
            }),
            crate::handlers::SocialEventRow::InsuranceCoverageRoute(r) => {
                Some(InsuranceRow::InsuranceCoverageRoute(r))
            }
            crate::handlers::SocialEventRow::InsuranceRouteFill(r) => {
                Some(InsuranceRow::InsuranceRouteFill(r))
            }
            crate::handlers::SocialEventRow::InsuranceVaultOperationalUpdate {
                vault_id,
                max_exposure_per_option,
                enabled,
                paused,
                max_exposure_per_market,
                max_exposure_per_user,
                base_rate_bps_per_day,
                utilization_multiplier_bps,
            } => Some(InsuranceRow::InsuranceVaultOperationalUpdate {
                vault_id,
                max_exposure_per_option,
                enabled,
                paused,
                max_exposure_per_market,
                max_exposure_per_user,
                base_rate_bps_per_day,
                utilization_multiplier_bps,
            }),
            _ => None,
        }
    }
}

impl FieldCount for InsuranceRow {
    const FIELD_COUNT: usize = 40;
}

pub struct InsuranceHandler;

#[async_trait]
impl Processor for InsuranceHandler {
    const NAME: &'static str = "insurance";

    type Value = InsuranceRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let timestamp_ms = checkpoint.summary.timestamp_ms;
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
                let module = ev.type_.module.as_str();
                if !INSURANCE_MODULES.contains(&module) {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data =
                    match events::parse_event_contents(module, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                if let Some(rows) = insurance::handle_insurance_event(
                    event_name,
                    &event_data,
                    &event_id,
                    timestamp_ms,
                ) {
                    for row in rows {
                        if let Some(r) = InsuranceRow::from_social(row) {
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
impl Handler for InsuranceHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                InsuranceRow::InsuranceConfig(c) => {
                    total += diesel::insert_into(insurance_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsuranceVault(v) => {
                    total += diesel::insert_into(insurance_vaults::table)
                        .values(v)
                        .on_conflict(insurance_vaults::vault_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsuranceVaultTransaction(t) => {
                    total += diesel::insert_into(insurance_vault_transactions::table)
                        .values(t)
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsuranceVaultBalanceUpdate {
                    vault_id,
                    new_balance,
                } => {
                    let now = chrono::Utc::now().naive_utc();
                    total += diesel::update(insurance_vaults::table)
                        .filter(insurance_vaults::vault_id.eq(vault_id))
                        .set((
                            insurance_vaults::capital_balance.eq(*new_balance),
                            insurance_vaults::updated_at.eq(now),
                        ))
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsurancePolicy(p) => {
                    total += diesel::insert_into(insurance_policies::table)
                        .values(p)
                        .on_conflict(insurance_policies::policy_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsurancePolicyEvent(pe) => {
                    total += diesel::insert_into(insurance_policy_events::table)
                        .values(pe)
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsuranceMarketExposure(me) => {
                    total += diesel::insert_into(insurance_market_exposures::table)
                        .values(me)
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsuranceUserExposure(ue) => {
                    total += diesel::insert_into(insurance_user_exposures::table)
                        .values(ue)
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsuranceEventLog(log) => {
                    total += diesel::insert_into(insurance_events::table)
                        .values(log)
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsurancePolicyStatusUpdate { policy_id, status } => {
                    let now = chrono::Utc::now().naive_utc();
                    total += diesel::update(insurance_policies::table)
                        .filter(insurance_policies::policy_id.eq(policy_id))
                        .set((
                            insurance_policies::status.eq(*status),
                            insurance_policies::updated_at.eq(now),
                        ))
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsurancePolicyEventFromPolicy {
                    policy_id,
                    event_type,
                    refunded_amount,
                    fee_paid,
                    payout,
                    reserve_released,
                    timestamp_ms,
                    transaction_id,
                } => {
                    #[derive(QueryableByName)]
                    struct PolicyRow {
                        #[diesel(sql_type = Text)]
                        market_id: String,
                        #[diesel(sql_type = Int2)]
                        option_id: i16,
                        #[diesel(sql_type = BigInt)]
                        covered_amount: i64,
                        #[diesel(sql_type = BigInt)]
                        coverage_bps: i64,
                        #[diesel(sql_type = BigInt)]
                        premium_paid: i64,
                        #[diesel(sql_type = Text)]
                        insured: String,
                    }
                    let policy_row: Option<PolicyRow> = diesel::sql_query(
                        "SELECT market_id, option_id, covered_amount, coverage_bps, premium_paid, insured FROM insurance_policies WHERE policy_id = $1",
                    )
                    .bind::<Text, _>(policy_id)
                    .get_result(conn)
                    .await
                    .ok();
                    if let Some(row) = policy_row {
                        let reserve_locked = reserve_released.unwrap_or_else(|| {
                            ((row.covered_amount as i128 * row.coverage_bps as i128) / 10000i128)
                                as i64
                        });
                        let policy_event = NewInsurancePolicyEvent {
                            policy_id: policy_id.clone(),
                            event_type: event_type.clone(),
                            market_id: row.market_id,
                            insured: row.insured,
                            option_id: row.option_id,
                            covered_amount: row.covered_amount,
                            coverage_bps: row.coverage_bps,
                            premium_paid: row.premium_paid,
                            reserve_locked,
                            premium_raw: None,
                            implied_probability_bps: None,
                            risk_multiplier_bps: None,
                            base_premium: None,
                            market_total_amount: None,
                            option_escrow_amount: None,
                            refunded_amount: *refunded_amount,
                            fee_paid: *fee_paid,
                            payout: *payout,
                            timestamp_ms: *timestamp_ms,
                            time: chrono::Utc::now(),
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(insurance_policy_events::table)
                            .values(&policy_event)
                            .execute(conn)
                            .await?;
                    }
                }
                InsuranceRow::InsuranceCoverageRoute(r) => {
                    total += diesel::insert_into(insurance_coverage_routes::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsuranceRouteFill(r) => {
                    total += diesel::insert_into(insurance_route_fills::table)
                        .values(r)
                        .on_conflict(insurance_route_fills::event_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsuranceVaultOperationalUpdate {
                    vault_id,
                    max_exposure_per_option,
                    enabled,
                    paused,
                    max_exposure_per_market,
                    max_exposure_per_user,
                    base_rate_bps_per_day,
                    utilization_multiplier_bps,
                } => {
                    let now = chrono::Utc::now().naive_utc();
                    let u = UpdateInsuranceVaultStatus {
                        max_exposure_per_option: *max_exposure_per_option,
                        enabled: *enabled,
                        paused: *paused,
                        max_exposure_per_market: *max_exposure_per_market,
                        max_exposure_per_user: *max_exposure_per_user,
                        base_rate_bps_per_day: *base_rate_bps_per_day,
                        utilization_multiplier_bps: *utilization_multiplier_bps,
                        updated_at: now,
                    };
                    total += diesel::update(insurance_vaults::table)
                        .filter(insurance_vaults::vault_id.eq(vault_id))
                        .set(&u)
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
