// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Insurance pipeline: indexes insurance module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::pg::upsert::excluded;
use diesel::prelude::OptionalExtension;
use diesel::sql_types::{BigInt, Bool, Int2, Text, Timestamptz};
use diesel::ExpressionMethods;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewInsuranceConfig, NewInsuranceCoverageRoute, NewInsuranceEventLog,
    NewInsuranceMarketExposure, NewInsurancePolicy, NewInsurancePolicyEvent, NewInsuranceRouteFill,
    NewInsuranceRouterConfig, NewInsuranceUserExposure, NewInsuranceVault,
    NewInsuranceVaultTransaction, UpdateInsuranceVaultStatus,
};
use myso_indexer_alt_social_schema::schema::{
    insurance_config, insurance_coverage_routes, insurance_events, insurance_market_exposures,
    insurance_policies, insurance_policy_events, insurance_route_fills, insurance_router_config,
    insurance_user_exposures, insurance_vault_transactions, insurance_vaults,
};

use super::common;
use super::events;
use super::insurance::{self, InsuranceConfigSnapshot};
use crate::metrics::SocialMetrics;

const INSURANCE_MODULES: &[&str] = &["insurance"];

#[derive(Debug, Clone)]
pub enum InsuranceRow {
    InsuranceConfig(InsuranceConfigSnapshot),
    InsuranceRouterConfig(NewInsuranceRouterConfig),
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
        updated_at: i64,
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
            crate::handlers::SocialEventRow::InsuranceConfig(snapshot) => {
                Some(InsuranceRow::InsuranceConfig(snapshot))
            }
            crate::handlers::SocialEventRow::InsuranceRouterConfig(c) => {
                Some(InsuranceRow::InsuranceRouterConfig(c))
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
                updated_at: timestamp_ms,
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
                        Err(e) => {
                            tracing::warn!(
                                tx_digest = %tx_digest,
                                module,
                                event_name,
                                error = %e,
                                hex_preview = %e.contents_hex_preview(48),
                                "insurance pipeline: event contents parse failed; skipping event"
                            );
                            SocialMetrics::record_event_bcs_parse_failed(module, event_name);
                            continue;
                        }
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

#[derive(QueryableByName)]
struct LatestInsuranceConfigRow {
    #[diesel(sql_type = Text)]
    updated_by: String,
    #[diesel(sql_type = Bool)]
    insurance_enabled: bool,
    #[diesel(sql_type = BigInt)]
    min_coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    max_coverage_bps: i64,
    #[diesel(sql_type = BigInt)]
    max_duration_ms: i64,
    #[diesel(sql_type = BigInt)]
    fee_bps: i64,
    #[diesel(sql_type = BigInt)]
    version: i64,
    #[diesel(sql_type = BigInt)]
    updated_at: i64,
    #[diesel(sql_type = Timestamptz)]
    time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    transaction_id: String,
    #[diesel(sql_type = BigInt)]
    min_spot_total_liquidity: i64,
    #[diesel(sql_type = BigInt)]
    max_coverage_fraction_of_option_bps: i64,
    #[diesel(sql_type = BigInt)]
    max_risk_multiplier_bps: i64,
    #[diesel(sql_type = BigInt)]
    min_premium_amount: i64,
    #[diesel(sql_type = BigInt)]
    spot_smoothing_per_option: i64,
    #[diesel(sql_type = BigInt)]
    implied_prob_floor_bps: i64,
    #[diesel(sql_type = Bool)]
    odds_floor_1x: bool,
    #[diesel(sql_type = BigInt)]
    odds_cap_bps: i64,
    #[diesel(sql_type = BigInt)]
    liq_cap_bps: i64,
    #[diesel(sql_type = BigInt)]
    liq_ref_amount: i64,
    #[diesel(sql_type = BigInt)]
    exposure_cap_bps: i64,
    #[diesel(sql_type = BigInt)]
    exposure_k_bps: i64,
    #[diesel(sql_type = BigInt)]
    odds_base_bps: i64,
}

fn latest_row_to_new(row: LatestInsuranceConfigRow) -> NewInsuranceConfig {
    NewInsuranceConfig {
        updated_by: row.updated_by,
        insurance_enabled: row.insurance_enabled,
        min_coverage_bps: row.min_coverage_bps,
        max_coverage_bps: row.max_coverage_bps,
        max_duration_ms: row.max_duration_ms,
        fee_bps: row.fee_bps,
        version: row.version,
        updated_at: row.updated_at,
        time: row.time,
        transaction_id: row.transaction_id,
        min_spot_total_liquidity: row.min_spot_total_liquidity,
        max_coverage_fraction_of_option_bps: row.max_coverage_fraction_of_option_bps,
        max_risk_multiplier_bps: row.max_risk_multiplier_bps,
        min_premium_amount: row.min_premium_amount,
        spot_smoothing_per_option: row.spot_smoothing_per_option,
        implied_prob_floor_bps: row.implied_prob_floor_bps,
        odds_floor_1x: row.odds_floor_1x,
        odds_cap_bps: row.odds_cap_bps,
        liq_cap_bps: row.liq_cap_bps,
        liq_ref_amount: row.liq_ref_amount,
        exposure_cap_bps: row.exposure_cap_bps,
        exposure_k_bps: row.exposure_k_bps,
        odds_base_bps: row.odds_base_bps,
    }
}

async fn load_latest_insurance_config(
    conn: &mut Connection<'_>,
) -> Result<Option<NewInsuranceConfig>> {
    let query = "
        SELECT updated_by, insurance_enabled, min_coverage_bps, max_coverage_bps, max_duration_ms,
               fee_bps, version, updated_at, time, transaction_id,
               min_spot_total_liquidity, max_coverage_fraction_of_option_bps,
               max_risk_multiplier_bps, min_premium_amount, spot_smoothing_per_option,
               implied_prob_floor_bps, odds_floor_1x, odds_cap_bps, liq_cap_bps, liq_ref_amount,
               exposure_cap_bps, exposure_k_bps, odds_base_bps
        FROM insurance_config
        ORDER BY time DESC
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .get_result::<LatestInsuranceConfigRow>(conn)
        .await
        .optional()?;
    Ok(result.map(latest_row_to_new))
}

fn finalize_insurance_config(
    prev: &NewInsuranceConfig,
    snapshot: &InsuranceConfigSnapshot,
) -> NewInsuranceConfig {
    match snapshot {
        InsuranceConfigSnapshot::Initialized(config) => config.clone(),
        InsuranceConfigSnapshot::Updated(update) => NewInsuranceConfig {
            updated_by: update.updated_by.clone(),
            insurance_enabled: update.insurance_enabled,
            min_coverage_bps: update.min_coverage_bps,
            max_coverage_bps: update.max_coverage_bps,
            max_duration_ms: update.max_duration_ms,
            fee_bps: update.fee_bps,
            version: update.version,
            updated_at: update.updated_at,
            time: update.time,
            transaction_id: update.transaction_id.clone(),
            min_spot_total_liquidity: prev.min_spot_total_liquidity,
            max_coverage_fraction_of_option_bps: prev.max_coverage_fraction_of_option_bps,
            max_risk_multiplier_bps: prev.max_risk_multiplier_bps,
            min_premium_amount: prev.min_premium_amount,
            spot_smoothing_per_option: prev.spot_smoothing_per_option,
            implied_prob_floor_bps: prev.implied_prob_floor_bps,
            odds_floor_1x: prev.odds_floor_1x,
            odds_cap_bps: prev.odds_cap_bps,
            liq_cap_bps: prev.liq_cap_bps,
            liq_ref_amount: prev.liq_ref_amount,
            exposure_cap_bps: prev.exposure_cap_bps,
            exposure_k_bps: prev.exposure_k_bps,
            odds_base_bps: update.odds_base_bps,
        },
        InsuranceConfigSnapshot::RiskPricingUpdated(update) => NewInsuranceConfig {
            updated_by: update.updated_by.clone(),
            insurance_enabled: prev.insurance_enabled,
            min_coverage_bps: prev.min_coverage_bps,
            max_coverage_bps: prev.max_coverage_bps,
            max_duration_ms: prev.max_duration_ms,
            fee_bps: prev.fee_bps,
            version: prev.version,
            updated_at: update.updated_at,
            time: update.time,
            transaction_id: update.transaction_id.clone(),
            min_spot_total_liquidity: update.min_spot_total_liquidity,
            max_coverage_fraction_of_option_bps: update.max_coverage_fraction_of_option_bps,
            max_risk_multiplier_bps: update.max_risk_multiplier_bps,
            min_premium_amount: update.min_premium_amount,
            spot_smoothing_per_option: update.spot_smoothing_per_option,
            implied_prob_floor_bps: update.implied_prob_floor_bps,
            odds_floor_1x: update.odds_floor_1x,
            odds_cap_bps: update.odds_cap_bps,
            liq_cap_bps: update.liq_cap_bps,
            liq_ref_amount: update.liq_ref_amount,
            exposure_cap_bps: update.exposure_cap_bps,
            exposure_k_bps: update.exposure_k_bps,
            odds_base_bps: prev.odds_base_bps,
        },
    }
}

#[async_trait]
impl Handler for InsuranceHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        let mut running_latest = load_latest_insurance_config(conn)
            .await?
            .unwrap_or_else(insurance::new_insurance_config_with_defaults);

        for row in values {
            match row {
                InsuranceRow::InsuranceConfig(snapshot) => {
                    let merged = finalize_insurance_config(&running_latest, snapshot);
                    total += diesel::insert_into(insurance_config::table)
                        .values(&merged)
                        .execute(conn)
                        .await?;
                    running_latest = merged;
                }
                InsuranceRow::InsuranceRouterConfig(c) => {
                    total += diesel::insert_into(insurance_router_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                InsuranceRow::InsuranceVault(v) => {
                    total += diesel::insert_into(insurance_vaults::table)
                        .values(v)
                        .on_conflict(insurance_vaults::vault_id)
                        .do_update()
                        .set((
                            insurance_vaults::underwriter
                                .eq(excluded(insurance_vaults::underwriter)),
                            insurance_vaults::base_rate_bps_per_day
                                .eq(excluded(insurance_vaults::base_rate_bps_per_day)),
                            insurance_vaults::utilization_multiplier_bps
                                .eq(excluded(insurance_vaults::utilization_multiplier_bps)),
                            insurance_vaults::max_exposure_per_market
                                .eq(excluded(insurance_vaults::max_exposure_per_market)),
                            insurance_vaults::max_exposure_per_user
                                .eq(excluded(insurance_vaults::max_exposure_per_user)),
                            insurance_vaults::max_exposure_per_option
                                .eq(excluded(insurance_vaults::max_exposure_per_option)),
                            insurance_vaults::enabled.eq(excluded(insurance_vaults::enabled)),
                            insurance_vaults::paused.eq(excluded(insurance_vaults::paused)),
                            insurance_vaults::version.eq(excluded(insurance_vaults::version)),
                            insurance_vaults::updated_at.eq(excluded(insurance_vaults::updated_at)),
                            insurance_vaults::transaction_id
                                .eq(excluded(insurance_vaults::transaction_id)),
                        ))
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
                    updated_at,
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
                            timestamp_ms: *updated_at,
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
