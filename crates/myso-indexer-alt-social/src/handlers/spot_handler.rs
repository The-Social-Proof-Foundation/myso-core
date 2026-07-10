// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Spot pipeline: indexes social_proof_of_truth / spot module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel::upsert::excluded;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewSpotBet, NewSpotBetWithdrawal, NewSpotClaim, NewSpotConfig, NewSpotCreatorPayout,
    NewSpotEventLog, NewSpotMarket, NewSpotPayout, NewSpotPostLink, NewSpotRecord, NewSpotRefund,
    NewSpotResolution,
};
use myso_indexer_alt_social_schema::schema::{
    posts, spot_bet_withdrawals, spot_bets, spot_claims, spot_config, spot_creator_earnings_daily,
    spot_creator_payouts, spot_events, spot_markets, spot_payouts, spot_post_links, spot_records,
    spot_refunds, spot_resolutions,
};

use super::common;
use super::events;
use super::organization_stats::{
    apply_spot_bet_stats, resolve_organization_id_for_derived_address,
    resolve_organization_id_for_post,
};
use super::spot;

const SPOT_MODULES: &[&str] = &["social_proof_of_truth", "spot"];

#[derive(Debug, Clone)]
pub enum SpotRow {
    SpotBet(NewSpotBet),
    SpotResolution(NewSpotResolution),
    SpotPayout(NewSpotPayout),
    SpotRefund(NewSpotRefund),
    SpotEventLog(NewSpotEventLog),
    SpotConfig(NewSpotConfig),
    SpotBetWithdrawal(NewSpotBetWithdrawal),
    SpotRecordUpsert(NewSpotRecord),
    SpotRecordUpdate {
        post_id: String,
        status: i16,
        outcome: Option<i16>,
        last_resolution_at_ms: i64,
        claim_object_id: Option<String>,
        market_object_id: Option<String>,
        creator_fee_total: Option<i64>,
    },
    SpotRecordGovernanceUpdate {
        spot_record_id: String,
        post_id: String,
        active_proposal_id: Option<String>,
        oracle_proposed_outcome: Option<i16>,
        proposed_outcome: Option<i16>,
        dao_escalated_at_ms: Option<i64>,
        status: Option<i16>,
    },
    SpotClaimUpsert(NewSpotClaim),
    SpotMarketUpsert(NewSpotMarket),
    SpotPostLinkUpsert(NewSpotPostLink),
    SpotCreatorPayoutUpsert(NewSpotCreatorPayout),
    SpotCreatorPayoutStatusUpdate {
        market_object_id: String,
        payout_id: i64,
        status: String,
        claimed_at_ms: Option<i64>,
        reclaimed_at_ms: Option<i64>,
        ecosystem_amount: Option<i64>,
        platform_amount: Option<i64>,
    },
    SpotCreatorEarningsDailyUpsert {
        creator_address: String,
        day: chrono::NaiveDate,
        amount: i64,
    },
    SpotMarketUpdate {
        market_object_id: String,
        status: i16,
        outcome: Option<i16>,
        last_resolution_at_ms: Option<i64>,
        resolution_timestamp_ms: Option<i64>,
        creator_fee_total: Option<i64>,
    },
    PostSpotFieldsUpdate {
        post_id: String,
        spot_id: Option<String>,
        spot_claim_id: Option<String>,
    },
}

impl SpotRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::SpotBet(bet) => Some(SpotRow::SpotBet(bet)),
            crate::handlers::SocialEventRow::SpotResolution(r) => Some(SpotRow::SpotResolution(r)),
            crate::handlers::SocialEventRow::SpotPayout(p) => Some(SpotRow::SpotPayout(p)),
            crate::handlers::SocialEventRow::SpotRefund(r) => Some(SpotRow::SpotRefund(r)),
            crate::handlers::SocialEventRow::SpotEventLog(log) => Some(SpotRow::SpotEventLog(log)),
            crate::handlers::SocialEventRow::SpotConfig(c) => Some(SpotRow::SpotConfig(c)),
            crate::handlers::SocialEventRow::SpotBetWithdrawal(w) => {
                Some(SpotRow::SpotBetWithdrawal(w))
            }
            crate::handlers::SocialEventRow::SpotRecordUpsert(record) => {
                Some(SpotRow::SpotRecordUpsert(record))
            }
            crate::handlers::SocialEventRow::SpotRecordUpdate {
                post_id,
                status,
                outcome,
                last_resolution_at_ms,
                claim_object_id,
                market_object_id,
                creator_fee_total,
            } => Some(SpotRow::SpotRecordUpdate {
                post_id,
                status,
                outcome,
                last_resolution_at_ms,
                claim_object_id,
                market_object_id,
                creator_fee_total,
            }),
            crate::handlers::SocialEventRow::SpotRecordGovernanceUpdate {
                spot_record_id,
                post_id,
                active_proposal_id,
                oracle_proposed_outcome,
                proposed_outcome,
                dao_escalated_at_ms,
                status,
            } => Some(SpotRow::SpotRecordGovernanceUpdate {
                spot_record_id,
                post_id,
                active_proposal_id,
                oracle_proposed_outcome,
                proposed_outcome,
                dao_escalated_at_ms,
                status,
            }),
            crate::handlers::SocialEventRow::SpotClaimUpsert(claim) => {
                Some(SpotRow::SpotClaimUpsert(claim))
            }
            crate::handlers::SocialEventRow::SpotMarketUpsert(market) => {
                Some(SpotRow::SpotMarketUpsert(market))
            }
            crate::handlers::SocialEventRow::SpotPostLinkUpsert(link) => {
                Some(SpotRow::SpotPostLinkUpsert(link))
            }
            crate::handlers::SocialEventRow::SpotCreatorPayoutUpsert(payout) => {
                Some(SpotRow::SpotCreatorPayoutUpsert(payout))
            }
            crate::handlers::SocialEventRow::SpotCreatorPayoutStatusUpdate {
                market_object_id,
                payout_id,
                status,
                claimed_at_ms,
                reclaimed_at_ms,
                ecosystem_amount,
                platform_amount,
            } => Some(SpotRow::SpotCreatorPayoutStatusUpdate {
                market_object_id,
                payout_id,
                status,
                claimed_at_ms,
                reclaimed_at_ms,
                ecosystem_amount,
                platform_amount,
            }),
            crate::handlers::SocialEventRow::SpotCreatorEarningsDailyUpsert {
                creator_address,
                day,
                amount,
            } => Some(SpotRow::SpotCreatorEarningsDailyUpsert {
                creator_address,
                day,
                amount,
            }),
            crate::handlers::SocialEventRow::SpotMarketUpdate {
                market_object_id,
                status,
                outcome,
                last_resolution_at_ms,
                resolution_timestamp_ms,
                creator_fee_total,
            } => Some(SpotRow::SpotMarketUpdate {
                market_object_id,
                status,
                outcome,
                last_resolution_at_ms,
                resolution_timestamp_ms,
                creator_fee_total,
            }),
            crate::handlers::SocialEventRow::PostSpotFieldsUpdate {
                post_id,
                spot_id,
                spot_claim_id,
            } => Some(SpotRow::PostSpotFieldsUpdate {
                post_id,
                spot_id,
                spot_claim_id,
            }),
            _ => None,
        }
    }
}

impl FieldCount for SpotRow {
    const FIELD_COUNT: usize = 19;
}

pub struct SpotHandler;

#[async_trait]
impl Processor for SpotHandler {
    const NAME: &'static str = "spot";

    type Value = SpotRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let epoch = checkpoint.summary.epoch;
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
                if !SPOT_MODULES.contains(&module) {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data =
                    match events::parse_event_contents(module, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                if let Some(rows) =
                    spot::handle_spot_event(event_name, &event_data, &event_id, epoch, timestamp_ms)
                {
                    for row in rows {
                        if let Some(r) = SpotRow::from_social(row) {
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
impl Handler for SpotHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                SpotRow::SpotBet(bet) => {
                    let mut bet = bet.clone();
                    if bet.organization_id.is_none() {
                        bet.organization_id =
                            resolve_organization_id_for_derived_address(conn, &bet.user_address)
                                .await?;
                    }
                    total += diesel::insert_into(spot_bets::table)
                        .values(&bet)
                        .execute(conn)
                        .await?;
                    let post_org = resolve_organization_id_for_post(conn, &bet.post_id).await?;
                    apply_spot_bet_stats(
                        conn,
                        bet.organization_id.as_deref(),
                        post_org.as_deref(),
                        bet.escrow_amount,
                        &bet.user_address,
                        bet.timestamp_ms,
                    )
                    .await?;
                }
                SpotRow::SpotResolution(r) => {
                    total += diesel::insert_into(spot_resolutions::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotPayout(p) => {
                    total += diesel::insert_into(spot_payouts::table)
                        .values(p)
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotRefund(r) => {
                    total += diesel::insert_into(spot_refunds::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotEventLog(log) => {
                    total += diesel::insert_into(spot_events::table)
                        .values(log)
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotConfig(c) => {
                    total += diesel::insert_into(spot_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotBetWithdrawal(w) => {
                    total += diesel::insert_into(spot_bet_withdrawals::table)
                        .values(w)
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotRecordUpsert(record) => {
                    let betting_options = record
                        .betting_options
                        .clone()
                        .unwrap_or_else(|| serde_json::json!([]));
                    let resolution_window_ms = record.resolution_window_ms;
                    let max_resolution_window_ms = record.max_resolution_window_ms;
                    let record_object_id = record.record_object_id.clone();
                    let claim_object_id = record.claim_object_id.clone();
                    let market_object_id = record.market_object_id.clone();
                    let primary_post_id = record.primary_post_id.clone();
                    let market_key_hash = record.market_key_hash.clone();
                    total += diesel::insert_into(spot_records::table)
                        .values(record)
                        .on_conflict(spot_records::post_id)
                        .do_update()
                        .set((
                            spot_records::betting_options.eq(betting_options),
                            spot_records::resolution_window_ms.eq(resolution_window_ms),
                            spot_records::max_resolution_window_ms.eq(max_resolution_window_ms),
                            spot_records::record_object_id.eq(record_object_id),
                            spot_records::claim_object_id.eq(claim_object_id),
                            spot_records::market_object_id.eq(market_object_id),
                            spot_records::primary_post_id.eq(primary_post_id),
                            spot_records::market_key_hash.eq(market_key_hash),
                            spot_records::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotRecordUpdate {
                    post_id,
                    status,
                    outcome,
                    last_resolution_at_ms,
                    claim_object_id,
                    market_object_id,
                    creator_fee_total,
                } => {
                    total += diesel::update(spot_records::table)
                        .filter(spot_records::post_id.eq(post_id))
                        .set((
                            spot_records::status.eq(*status),
                            spot_records::outcome.eq(*outcome),
                            spot_records::last_resolution_at_ms.eq(Some(*last_resolution_at_ms)),
                            spot_records::claim_object_id.eq(claim_object_id),
                            spot_records::market_object_id.eq(market_object_id),
                            spot_records::creator_fee_total.eq(*creator_fee_total),
                            spot_records::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotRecordGovernanceUpdate {
                    spot_record_id: _,
                    post_id,
                    active_proposal_id,
                    oracle_proposed_outcome,
                    proposed_outcome,
                    dao_escalated_at_ms,
                    status,
                } => {
                    total += diesel::update(spot_records::table)
                        .filter(spot_records::post_id.eq(post_id))
                        .set((
                            spot_records::active_proposal_id.eq(active_proposal_id),
                            spot_records::oracle_proposed_outcome.eq(oracle_proposed_outcome),
                            spot_records::proposed_outcome.eq(proposed_outcome),
                            spot_records::dao_escalated_at_ms.eq(dao_escalated_at_ms),
                            spot_records::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                    if let Some(s) = status {
                        total += diesel::update(spot_records::table)
                            .filter(spot_records::post_id.eq(post_id))
                            .set(spot_records::status.eq(*s))
                            .execute(conn)
                            .await?;
                    }
                }
                SpotRow::SpotClaimUpsert(claim) => {
                    total += diesel::insert_into(spot_claims::table)
                        .values(claim)
                        .on_conflict(spot_claims::claim_object_id)
                        .do_update()
                        .set((
                            spot_claims::semantic_claim_hash
                                .eq(excluded(spot_claims::semantic_claim_hash)),
                            spot_claims::created_at_ms.eq(excluded(spot_claims::created_at_ms)),
                            spot_claims::transaction_id.eq(excluded(spot_claims::transaction_id)),
                        ))
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotMarketUpsert(market) => {
                    total += diesel::insert_into(spot_markets::table)
                        .values(market)
                        .on_conflict(spot_markets::market_object_id)
                        .do_update()
                        .set((
                            spot_markets::status.eq(excluded(spot_markets::status)),
                            spot_markets::outcome.eq(excluded(spot_markets::outcome)),
                            spot_markets::betting_options.eq(excluded(spot_markets::betting_options)),
                            spot_markets::option_escrow.eq(excluded(spot_markets::option_escrow)),
                            spot_markets::resolution_window_ms
                                .eq(excluded(spot_markets::resolution_window_ms)),
                            spot_markets::max_resolution_window_ms
                                .eq(excluded(spot_markets::max_resolution_window_ms)),
                            spot_markets::resolution_at_ms
                                .eq(excluded(spot_markets::resolution_at_ms)),
                            spot_markets::last_resolution_at_ms
                                .eq(excluded(spot_markets::last_resolution_at_ms)),
                            spot_markets::resolution_timestamp_ms
                                .eq(excluded(spot_markets::resolution_timestamp_ms)),
                            spot_markets::creator_fee_total
                                .eq(excluded(spot_markets::creator_fee_total)),
                            spot_markets::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotPostLinkUpsert(link) => {
                    total += diesel::insert_into(spot_post_links::table)
                        .values(link)
                        .on_conflict(spot_post_links::post_id)
                        .do_update()
                        .set((
                            spot_post_links::claim_object_id
                                .eq(excluded(spot_post_links::claim_object_id)),
                            spot_post_links::market_object_id
                                .eq(excluded(spot_post_links::market_object_id)),
                            spot_post_links::link_kind.eq(excluded(spot_post_links::link_kind)),
                            spot_post_links::transaction_id
                                .eq(excluded(spot_post_links::transaction_id)),
                        ))
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotCreatorPayoutUpsert(payout) => {
                    total += diesel::insert_into(spot_creator_payouts::table)
                        .values(payout)
                        .on_conflict((
                            spot_creator_payouts::market_object_id,
                            spot_creator_payouts::payout_id,
                        ))
                        .do_update()
                        .set((
                            spot_creator_payouts::amount.eq(excluded(spot_creator_payouts::amount)),
                            spot_creator_payouts::expires_at_ms
                                .eq(excluded(spot_creator_payouts::expires_at_ms)),
                            spot_creator_payouts::status.eq(excluded(spot_creator_payouts::status)),
                            spot_creator_payouts::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotCreatorPayoutStatusUpdate {
                    market_object_id,
                    payout_id,
                    status,
                    claimed_at_ms,
                    reclaimed_at_ms,
                    ecosystem_amount,
                    platform_amount,
                } => {
                    total += diesel::update(spot_creator_payouts::table)
                        .filter(spot_creator_payouts::market_object_id.eq(market_object_id))
                        .filter(spot_creator_payouts::payout_id.eq(*payout_id))
                        .set((
                            spot_creator_payouts::status.eq(status),
                            spot_creator_payouts::claimed_at_ms.eq(claimed_at_ms),
                            spot_creator_payouts::reclaimed_at_ms.eq(reclaimed_at_ms),
                            spot_creator_payouts::ecosystem_amount.eq(ecosystem_amount),
                            spot_creator_payouts::platform_amount.eq(platform_amount),
                            spot_creator_payouts::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotCreatorEarningsDailyUpsert {
                    creator_address,
                    day,
                    amount,
                } => {
                    total += diesel::insert_into(spot_creator_earnings_daily::table)
                        .values((
                            spot_creator_earnings_daily::creator_address.eq(creator_address),
                            spot_creator_earnings_daily::day.eq(day),
                            spot_creator_earnings_daily::amount.eq(amount),
                            spot_creator_earnings_daily::updated_at
                                .eq(chrono::Utc::now().naive_utc()),
                        ))
                        .on_conflict((
                            spot_creator_earnings_daily::creator_address,
                            spot_creator_earnings_daily::day,
                        ))
                        .do_update()
                        .set((
                            spot_creator_earnings_daily::amount.eq(
                                spot_creator_earnings_daily::amount
                                    + excluded(spot_creator_earnings_daily::amount),
                            ),
                            spot_creator_earnings_daily::updated_at
                                .eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotMarketUpdate {
                    market_object_id,
                    status,
                    outcome,
                    last_resolution_at_ms,
                    resolution_timestamp_ms,
                    creator_fee_total,
                } => {
                    total += diesel::update(spot_markets::table)
                        .filter(spot_markets::market_object_id.eq(market_object_id))
                        .set((
                            spot_markets::status.eq(status),
                            spot_markets::outcome.eq(outcome),
                            spot_markets::last_resolution_at_ms.eq(last_resolution_at_ms),
                            spot_markets::resolution_timestamp_ms.eq(resolution_timestamp_ms),
                            spot_markets::creator_fee_total.eq(creator_fee_total),
                            spot_markets::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SpotRow::PostSpotFieldsUpdate {
                    post_id,
                    spot_id,
                    spot_claim_id,
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set((
                            posts::spot_id.eq(spot_id),
                            posts::spot_claim_id.eq(spot_claim_id),
                        ))
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
