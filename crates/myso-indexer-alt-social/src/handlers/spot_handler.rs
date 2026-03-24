// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Spot pipeline: indexes social_proof_of_truth / spot module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewSpotBet, NewSpotBetWithdrawal, NewSpotConfig, NewSpotEventLog, NewSpotPayout, NewSpotRecord,
    NewSpotRefund, NewSpotResolution,
};
use myso_indexer_alt_social_schema::schema::{
    spot_bet_withdrawals, spot_bets, spot_config, spot_events, spot_payouts, spot_records,
    spot_refunds, spot_resolutions,
};

use super::common;
use super::events;
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
        last_resolution_epoch: i64,
    },
}

impl SpotRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::SpotBet(bet) => Some(SpotRow::SpotBet(bet)),
            crate::handlers::SocialEventRow::SpotResolution(r) => Some(SpotRow::SpotResolution(r)),
            crate::handlers::SocialEventRow::SpotPayout(p) => Some(SpotRow::SpotPayout(p)),
            crate::handlers::SocialEventRow::SpotRefund(r) => Some(SpotRow::SpotRefund(r)),
            crate::handlers::SocialEventRow::SpotEventLog(log) => {
                Some(SpotRow::SpotEventLog(log))
            }
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
                last_resolution_epoch,
            } => Some(SpotRow::SpotRecordUpdate {
                post_id,
                status,
                outcome,
                last_resolution_epoch,
            }),
            _ => None,
        }
    }
}

impl FieldCount for SpotRow {
    const FIELD_COUNT: usize = 9;
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
                    total += diesel::insert_into(spot_bets::table)
                        .values(bet)
                        .execute(conn)
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
                    let resolution_window_epochs = record.resolution_window_epochs;
                    let max_resolution_window_epochs = record.max_resolution_window_epochs;
                    total += diesel::insert_into(spot_records::table)
                        .values(record)
                        .on_conflict(spot_records::post_id)
                        .do_update()
                        .set((
                            spot_records::betting_options.eq(betting_options),
                            spot_records::resolution_window_epochs.eq(resolution_window_epochs),
                            spot_records::max_resolution_window_epochs
                                .eq(max_resolution_window_epochs),
                            spot_records::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
                SpotRow::SpotRecordUpdate {
                    post_id,
                    status,
                    outcome,
                    last_resolution_epoch,
                } => {
                    total += diesel::update(spot_records::table)
                        .filter(spot_records::post_id.eq(post_id))
                        .set((
                            spot_records::status.eq(*status),
                            spot_records::outcome.eq(*outcome),
                            spot_records::last_resolution_epoch.eq(Some(*last_resolution_epoch)),
                            spot_records::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
