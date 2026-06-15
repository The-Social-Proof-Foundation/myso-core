// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Indexes `PaidMessagingPolicyUpdated` events from the messaging package.

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
use myso_indexer_alt_social_schema::models::NewWalletMessagingPolicy;
use myso_indexer_alt_social_schema::schema::wallet_messaging_policies;

use super::common;
use super::events;

const PAID_MESSAGING_POLICY_MODULE: &str = "paid_messaging_policy";

#[derive(Debug, Clone)]
pub struct PaidMessagingPolicyRow(NewWalletMessagingPolicy);

impl FieldCount for PaidMessagingPolicyRow {
    const FIELD_COUNT: usize = 4;
}

pub struct PaidMessagingPolicyHandler;

#[async_trait]
impl Processor for PaidMessagingPolicyHandler {
    const NAME: &'static str = "paid_messaging_policy";

    type Value = PaidMessagingPolicyRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_timestamp_ms = checkpoint.summary.timestamp_ms as i64;
        let mut values = Vec::new();
        for tx in &checkpoint.transactions {
            let Some(events) = &tx.events else {
                continue;
            };
            for ev in &events.data {
                if !common::is_messaging_package_event(&ev.package_id, &ev.type_.address) {
                    continue;
                }
                if ev.type_.module.as_str() != PAID_MESSAGING_POLICY_MODULE {
                    continue;
                }
                if ev.type_.name.as_str() != "PaidMessagingPolicyUpdated" {
                    continue;
                }
                let event_data = match events::parse_event_contents(
                    PAID_MESSAGING_POLICY_MODULE,
                    "PaidMessagingPolicyUpdated",
                    &ev.contents,
                ) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let Some(row) = handle_paid_messaging_policy_updated(&event_data, checkpoint_timestamp_ms)
                else {
                    continue;
                };
                values.push(PaidMessagingPolicyRow(row));
            }
        }
        Ok(values)
    }
}

fn handle_paid_messaging_policy_updated(
    data: &serde_json::Value,
    updated_at: i64,
) -> Option<NewWalletMessagingPolicy> {
    let wallet = data.get("wallet")?.as_str()?.to_string();
    let enabled = data.get("enabled")?.as_bool()?;
    let min_cost = data
        .get("min_cost")
        .and_then(|v| {
            if v.is_null() {
                None
            } else {
                v.as_u64().map(|n| n as i64)
            }
        });
    Some(NewWalletMessagingPolicy {
        wallet_address: wallet,
        enabled,
        min_cost,
        updated_at,
    })
}

#[async_trait]
impl Handler for PaidMessagingPolicyHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            let policy = &row.0;
            let enabled = policy.enabled;
            let min_cost = policy.min_cost;
            let updated_at = policy.updated_at;
            total += diesel::insert_into(wallet_messaging_policies::table)
                .values(policy)
                .on_conflict(wallet_messaging_policies::wallet_address)
                .do_update()
                .set((
                    wallet_messaging_policies::enabled.eq(enabled),
                    wallet_messaging_policies::min_cost.eq(min_cost),
                    wallet_messaging_policies::updated_at.eq(updated_at),
                ))
                .execute(conn)
                .await?;
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paid_messaging_policy_updated_bcs_roundtrip() {
        use move_core_types::account_address::AccountAddress;

        let wallet = AccountAddress::from_hex_literal(
            "0x2458950181e415250823d6ce1d55f2b3427826a111939e0d6d38e9a1397411d8",
        )
        .unwrap();
        let ev = events::BcsPaidMessagingPolicyUpdated {
            wallet,
            enabled: true,
            min_cost: Some(1_000_000),
        };
        let bytes = bcs::to_bytes(&ev).unwrap();
        let json = events::parse_event_contents(
            "paid_messaging_policy",
            "PaidMessagingPolicyUpdated",
            &bytes,
        )
        .expect("BCS parse should succeed");
        let row = handle_paid_messaging_policy_updated(&json, 42).expect("handler should accept");
        assert!(row.enabled);
        assert_eq!(row.min_cost, Some(1_000_000));
        assert_eq!(row.updated_at, 42);
        assert!(row.wallet_address.starts_with("0x"));
    }
}
