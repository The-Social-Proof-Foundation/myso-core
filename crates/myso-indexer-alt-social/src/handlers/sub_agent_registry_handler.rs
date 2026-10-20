// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Dedicated watermark pipeline for sub-agent registry rows (`sub_agents`, `sub_agent_events`).

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::upsert::excluded;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewCredibility, NewReputationAggregate, NewSubAgent, NewSubAgentEvent,
};
use myso_indexer_alt_social_schema::schema::{
    credibility, memory_accounts, reputation_aggregates, sub_agent_events, sub_agents,
};

use super::common;
use super::events;
use super::memory::{self, json_str};
use crate::metrics::SocialMetrics;
use chrono::Utc;

const MEMORY_MODULE: &str = "memory";

const SUB_AGENT_REGISTRY_EVENTS: &[&str] = &[
    "SubAgentRegistered",
    "SubAgentUpdated",
    "SubAgentDeactivated",
    "SubAgentRevoked",
    "SubAgentsClearedOnTransfer",
    "ReputationUpdated",
    "CredibilityUpdated",
];

#[derive(Debug, Clone)]
pub enum SubAgentRegistryRow {
    Upsert(NewSubAgent),
    Deactivate {
        agent_object_id: String,
        deactivated_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    Revoke {
        agent_object_id: String,
        revoked_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    BulkClear {
        account_id: String,
        new_principal_owner: String,
        revoked_at_ms: i64,
        event_id: String,
        transaction_id: String,
    },
    Audit(NewSubAgentEvent),
    /// Aggregate after one review; later reviews carry a larger `review_count`.
    Reputation(NewReputationAggregate),
    /// Principal credibility inputs; later settlements carry a larger `settled_count`.
    Credibility(NewCredibility),
}

impl FieldCount for SubAgentRegistryRow {
    const FIELD_COUNT: usize = 20;
}

fn str_field(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key).and_then(|v| v.as_str()).map(String::from)
}

fn opt_str_field(data: &serde_json::Value, key: &str) -> Option<String> {
    data.get(key)
        .filter(|v| !v.is_null())
        .and_then(|v| v.as_str())
        .map(String::from)
}

fn i64_field(data: &serde_json::Value, key: &str) -> Option<i64> {
    let v = data.get(key)?;
    v.as_i64()
        .or_else(|| v.as_u64().and_then(|u| i64::try_from(u).ok()))
}

/// `u128` sums arrive as decimal strings (serde_json numbers stop at `u64`); anything else is
/// rejected so a malformed event never stores a bogus aggregate.
fn decimal_field(data: &serde_json::Value, key: &str) -> Option<String> {
    let s = data.get(key)?.as_str()?;
    (!s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())).then(|| s.to_string())
}

/// Pure builder for `memory::ReputationUpdated`.
pub fn reputation_row(
    data: &serde_json::Value,
    event_id: &str,
    transaction_id: &str,
) -> Option<NewReputationAggregate> {
    let updated_at_ms = i64_field(data, "updated_at_ms")?;
    Some(NewReputationAggregate {
        subject_id: str_field(data, "subject_id")?,
        role: i64_field(data, "role")? as i16,
        subject_kind: i64_field(data, "subject_kind")? as i16,
        principal_owner: str_field(data, "principal_owner")?,
        organization_id: opt_str_field(data, "organization_id"),
        review_count: i64_field(data, "review_count")?,
        star_1: i64_field(data, "star_1")?,
        star_2: i64_field(data, "star_2")?,
        star_3: i64_field(data, "star_3")?,
        star_4: i64_field(data, "star_4")?,
        star_5: i64_field(data, "star_5")?,
        weight_sum_bps: decimal_field(data, "weight_sum_bps")?,
        weighted_star_sum: decimal_field(data, "weighted_star_sum")?,
        bayesian_score_milli: i64_field(data, "bayesian_score_milli")?,
        confidence_bps: i64_field(data, "confidence_bps")?,
        last_receipt_id: str_field(data, "receipt_id")?,
        updated_at_ms,
        event_id: event_id.to_string(),
        transaction_id: transaction_id.to_string(),
        time: common::chain_time_from_ms(updated_at_ms),
    })
}

/// Pure builder for `memory::CredibilityUpdated`.
pub fn credibility_row(
    data: &serde_json::Value,
    event_id: &str,
    transaction_id: &str,
) -> Option<NewCredibility> {
    let updated_at_ms = i64_field(data, "updated_at_ms")?;
    Some(NewCredibility {
        account_id: str_field(data, "account_id")?,
        principal_owner: str_field(data, "principal_owner")?,
        settled_count: i64_field(data, "settled_count")?,
        volume_mist: decimal_field(data, "volume_mist")?,
        credibility_bps: i64_field(data, "credibility_bps")?,
        updated_at_ms,
        event_id: event_id.to_string(),
        transaction_id: transaction_id.to_string(),
        time: common::chain_time_from_ms(updated_at_ms),
    })
}

pub struct SubAgentRegistryHandler;

fn handle_sub_agent_registry_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SubAgentRegistryRow>> {
    let transaction_id = event_id.split(':').next().unwrap_or(event_id).to_string();
    let now = Utc::now();
    match event_name {
        "SubAgentRegistered" | "SubAgentUpdated" => {
            let row = memory::sub_agent_from_event(data, event_id, &transaction_id, now)?;
            let audit =
                memory::sub_agent_audit_event(event_name, data, event_id, &transaction_id, now);
            Some(vec![
                SubAgentRegistryRow::Upsert(row),
                SubAgentRegistryRow::Audit(audit),
            ])
        }
        "SubAgentDeactivated" => {
            let agent_object_id = json_str(data, "agent_object_id")?;
            let audit =
                memory::sub_agent_audit_event(event_name, data, event_id, &transaction_id, now);
            Some(vec![
                SubAgentRegistryRow::Deactivate {
                    agent_object_id,
                    deactivated_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id,
                },
                SubAgentRegistryRow::Audit(audit),
            ])
        }
        "SubAgentRevoked" => {
            let agent_object_id = json_str(data, "agent_object_id")?;
            let audit =
                memory::sub_agent_audit_event(event_name, data, event_id, &transaction_id, now);
            Some(vec![
                SubAgentRegistryRow::Revoke {
                    agent_object_id,
                    revoked_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id,
                },
                SubAgentRegistryRow::Audit(audit),
            ])
        }
        "SubAgentsClearedOnTransfer" => {
            let account_id = json_str(data, "account_id")?;
            let new_owner = json_str(data, "new_owner")?;
            let audit =
                memory::sub_agent_audit_event(event_name, data, event_id, &transaction_id, now);
            Some(vec![
                SubAgentRegistryRow::BulkClear {
                    account_id,
                    new_principal_owner: new_owner,
                    revoked_at_ms: now.timestamp_millis(),
                    event_id: event_id.to_string(),
                    transaction_id,
                },
                SubAgentRegistryRow::Audit(audit),
            ])
        }
        "ReputationUpdated" => reputation_row(data, event_id, &transaction_id)
            .map(|r| vec![SubAgentRegistryRow::Reputation(r)]),
        "CredibilityUpdated" => credibility_row(data, event_id, &transaction_id)
            .map(|r| vec![SubAgentRegistryRow::Credibility(r)]),
        _ => None,
    }
}

#[async_trait]
impl Processor for SubAgentRegistryHandler {
    const NAME: &'static str = "sub_agent_registry";

    type Value = SubAgentRegistryRow;

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
                if ev.type_.module.as_str() != MEMORY_MODULE {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                if !SUB_AGENT_REGISTRY_EVENTS.contains(&event_name) {
                    continue;
                }
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data =
                    match events::parse_event_contents(MEMORY_MODULE, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(e) => {
                            tracing::warn!(
                                tx_digest = %tx_digest,
                                module = MEMORY_MODULE,
                                event_name,
                                error = %e,
                                "sub_agent_registry: event parse failed; skipping"
                            );
                            SocialMetrics::record_event_bcs_parse_failed(MEMORY_MODULE, event_name);
                            continue;
                        }
                    };
                if let Some(rows) =
                    handle_sub_agent_registry_event(event_name, &event_data, &event_id)
                {
                    values.extend(rows);
                }
            }
        }
        Ok(values)
    }
}

#[async_trait]
impl Handler for SubAgentRegistryHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                SubAgentRegistryRow::Upsert(s) => {
                    let organization_id = s.organization_id.clone();
                    let label = s.label.clone();
                    let identity_class = s.identity_class;
                    let role_tags = s.role_tags;
                    let capabilities = s.capabilities;
                    let delegatable_caps = s.delegatable_caps;
                    let register_scope = s.register_scope;
                    let approval_required_caps = s.approval_required_caps;
                    let max_action_spend = s.max_action_spend;
                    let platform_scope = s.platform_scope.clone();
                    let parent_object_id = s.parent_object_id.clone();
                    let depth = s.depth;
                    let registered_by = s.registered_by.clone();
                    let expires_at_ms = s.expires_at_ms;
                    let active = s.active;
                    let updated_at_ms = s.updated_at_ms;
                    let event_id = s.event_id.clone();
                    let transaction_id = s.transaction_id.clone();
                    let time = s.time;
                    total += diesel::insert_into(sub_agents::table)
                        .values(s)
                        .on_conflict(sub_agents::agent_object_id)
                        .do_update()
                        .set((
                            sub_agents::derived_address.eq(s.derived_address.clone()),
                            sub_agents::account_id.eq(s.account_id.clone()),
                            sub_agents::organization_id.eq(organization_id),
                            sub_agents::label.eq(label),
                            sub_agents::identity_class.eq(identity_class),
                            sub_agents::role_tags.eq(role_tags),
                            sub_agents::capabilities.eq(capabilities),
                            sub_agents::delegatable_caps.eq(delegatable_caps),
                            sub_agents::register_scope.eq(register_scope),
                            sub_agents::approval_required_caps.eq(approval_required_caps),
                            sub_agents::max_action_spend.eq(max_action_spend),
                            sub_agents::platform_scope.eq(platform_scope),
                            sub_agents::parent_object_id.eq(parent_object_id),
                            sub_agents::depth.eq(depth),
                            sub_agents::registered_by.eq(registered_by),
                            sub_agents::expires_at_ms.eq(expires_at_ms),
                            sub_agents::active.eq(active),
                            sub_agents::updated_at_ms.eq(updated_at_ms),
                            sub_agents::event_id.eq(event_id),
                            sub_agents::transaction_id.eq(transaction_id),
                            sub_agents::time.eq(time),
                            sub_agents::deactivated_at_ms.eq(None::<i64>),
                            sub_agents::revoked_at_ms.eq(None::<i64>),
                        ))
                        .execute(conn)
                        .await?;
                }
                SubAgentRegistryRow::Deactivate {
                    agent_object_id,
                    deactivated_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        sub_agents::table.filter(sub_agents::agent_object_id.eq(agent_object_id)),
                    )
                    .set((
                        sub_agents::active.eq(false),
                        sub_agents::deactivated_at_ms.eq(*deactivated_at_ms),
                        sub_agents::updated_at_ms.eq(*deactivated_at_ms),
                        sub_agents::event_id.eq(event_id),
                        sub_agents::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                SubAgentRegistryRow::Revoke {
                    agent_object_id,
                    revoked_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        sub_agents::table.filter(sub_agents::agent_object_id.eq(agent_object_id)),
                    )
                    .set((
                        sub_agents::active.eq(false),
                        sub_agents::revoked_at_ms.eq(*revoked_at_ms),
                        sub_agents::updated_at_ms.eq(*revoked_at_ms),
                        sub_agents::event_id.eq(event_id),
                        sub_agents::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                }
                SubAgentRegistryRow::BulkClear {
                    account_id,
                    new_principal_owner,
                    revoked_at_ms,
                    event_id,
                    transaction_id,
                } => {
                    total += diesel::update(
                        sub_agents::table
                            .filter(sub_agents::account_id.eq(account_id))
                            .filter(sub_agents::active.eq(true)),
                    )
                    .set((
                        sub_agents::active.eq(false),
                        sub_agents::revoked_at_ms.eq(*revoked_at_ms),
                        sub_agents::updated_at_ms.eq(*revoked_at_ms),
                        sub_agents::event_id.eq(event_id),
                        sub_agents::transaction_id.eq(transaction_id),
                    ))
                    .execute(conn)
                    .await?;
                    total += diesel::update(
                        memory_accounts::table.filter(memory_accounts::account_id.eq(account_id)),
                    )
                    .set(memory_accounts::principal_owner.eq(new_principal_owner))
                    .execute(conn)
                    .await?;
                }
                SubAgentRegistryRow::Audit(e) => {
                    total += diesel::insert_into(sub_agent_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                SubAgentRegistryRow::Reputation(r) => {
                    // Concurrent pipeline: never let an older aggregate overwrite a newer one.
                    let upsert = diesel::insert_into(reputation_aggregates::table)
                        .values(r)
                        .on_conflict((
                            reputation_aggregates::subject_id,
                            reputation_aggregates::role,
                        ))
                        .do_update()
                        .set(r);
                    // Called by path: importing `FilterDsl` would make `.filter` on plain tables
                    // ambiguous with `QueryDsl::filter` in this file.
                    total += diesel::query_dsl::methods::FilterDsl::filter(
                        upsert,
                        reputation_aggregates::review_count
                            .lt(excluded(reputation_aggregates::review_count)),
                    )
                    .execute(conn)
                    .await?;
                }
                SubAgentRegistryRow::Credibility(c) => {
                    let upsert = diesel::insert_into(credibility::table)
                        .values(c)
                        .on_conflict(credibility::account_id)
                        .do_update()
                        .set(c);
                    total += diesel::query_dsl::methods::FilterDsl::filter(
                        upsert,
                        credibility::settled_count.lt(excluded(credibility::settled_count)),
                    )
                    .execute(conn)
                    .await?;
                }
            }
        }
        Ok(total)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sub_agent_registered_produces_upsert_and_audit_rows() {
        let data = serde_json::json!({
            "agent_object_id": "0xagent",
            "derived_address": "0xderived",
            "account_id": "0xaccount",
            "label": "bot",
            "registered_by": "0xowner",
            "active": true,
        });
        let rows = handle_sub_agent_registry_event("SubAgentRegistered", &data, "tx-digest:1")
            .expect("SubAgentRegistered should produce rows");
        assert_eq!(rows.len(), 2);
        match (&rows[0], &rows[1]) {
            (SubAgentRegistryRow::Upsert(s), SubAgentRegistryRow::Audit(e))
            | (SubAgentRegistryRow::Audit(e), SubAgentRegistryRow::Upsert(s)) => {
                assert_eq!(s.agent_object_id, "0xagent");
                assert_eq!(e.event_type, "SubAgentRegistered");
            }
            _ => panic!("expected Upsert and Audit rows"),
        }
    }

    #[test]
    fn sub_agent_deactivated_produces_deactivate_and_audit_rows() {
        let data = serde_json::json!({
            "agent_object_id": "0xagent",
            "account_id": "0xaccount",
        });
        let rows = handle_sub_agent_registry_event("SubAgentDeactivated", &data, "tx-digest:2")
            .expect("SubAgentDeactivated should produce rows");
        assert_eq!(rows.len(), 2);
        match (&rows[0], &rows[1]) {
            (
                SubAgentRegistryRow::Deactivate {
                    agent_object_id, ..
                },
                SubAgentRegistryRow::Audit(e),
            )
            | (
                SubAgentRegistryRow::Audit(e),
                SubAgentRegistryRow::Deactivate {
                    agent_object_id, ..
                },
            ) => {
                assert_eq!(agent_object_id, "0xagent");
                assert_eq!(e.event_type, "SubAgentDeactivated");
            }
            _ => panic!("expected Deactivate and Audit rows"),
        }
    }

    #[test]
    fn sub_agents_cleared_on_transfer_produces_bulk_clear_and_audit_rows() {
        let data = serde_json::json!({
            "account_id": "0xaccount",
            "new_owner": "0xnew_owner",
        });
        let rows =
            handle_sub_agent_registry_event("SubAgentsClearedOnTransfer", &data, "tx-digest:3")
                .expect("SubAgentsClearedOnTransfer should produce rows");
        assert_eq!(rows.len(), 2);
        match (&rows[0], &rows[1]) {
            (
                SubAgentRegistryRow::BulkClear {
                    account_id,
                    new_principal_owner,
                    ..
                },
                SubAgentRegistryRow::Audit(e),
            )
            | (
                SubAgentRegistryRow::Audit(e),
                SubAgentRegistryRow::BulkClear {
                    account_id,
                    new_principal_owner,
                    ..
                },
            ) => {
                assert_eq!(account_id, "0xaccount");
                assert_eq!(new_principal_owner, "0xnew_owner");
                assert_eq!(e.event_type, "SubAgentsClearedOnTransfer");
            }
            _ => panic!("expected BulkClear and Audit rows"),
        }
    }

    fn parsed(event_name: &str, bytes: &[u8]) -> serde_json::Value {
        events::parse_event_contents(MEMORY_MODULE, event_name, bytes)
            .expect("BCS parse should succeed")
    }

    fn id(hex: &str) -> events::BcsMoveObjectId {
        events::BcsMoveObjectId::from_address(
            move_core_types::account_address::AccountAddress::from_hex_literal(hex).unwrap(),
        )
    }

    fn address(hex: &str) -> move_core_types::account_address::AccountAddress {
        move_core_types::account_address::AccountAddress::from_hex_literal(hex).unwrap()
    }

    #[test]
    fn reputation_updated_becomes_an_aggregate_row_with_u128_sums_as_strings() {
        let ev = events::BcsReputationUpdated {
            subject_kind: 1,
            subject_id: id("0x5ea"),
            principal_owner: address("0x5e"),
            organization_id: Some(id("0x0e9")),
            role: 1,
            review_count: 2,
            star_1: 0,
            star_2: 0,
            star_3: 0,
            star_4: 0,
            star_5: 2,
            // Beyond u64: must survive as an exact decimal string.
            weight_sum_bps: 18_446_744_073_709_551_616u128,
            weighted_star_sum: 340_282_366_920_938_463_463_374_607_431_768_211_455u128,
            bayesian_score_milli: 3122,
            confidence_bps: 609,
            receipt_id: id("0x7a5c"),
            updated_at_ms: 1_700_000_000_000,
        };
        let json = parsed("ReputationUpdated", &bcs::to_bytes(&ev).unwrap());
        let rows = handle_sub_agent_registry_event("ReputationUpdated", &json, "digest:4")
            .expect("handler should accept");
        let [SubAgentRegistryRow::Reputation(row)] = rows.as_slice() else {
            panic!("expected one Reputation row");
        };
        assert_eq!((row.role, row.subject_kind), (1, 1));
        assert_eq!(row.review_count, 2);
        assert_eq!(row.star_5, 2);
        assert_eq!(row.weight_sum_bps, "18446744073709551616");
        assert_eq!(
            row.weighted_star_sum,
            "340282366920938463463374607431768211455"
        );
        assert_eq!(row.bayesian_score_milli, 3122);
        assert!(row.organization_id.is_some());
        assert_eq!(row.transaction_id, "digest");
    }

    #[test]
    fn a_human_buyer_aggregate_has_no_organization() {
        let ev = events::BcsReputationUpdated {
            subject_kind: 3,
            subject_id: id("0xacc"),
            principal_owner: address("0xb0b"),
            organization_id: None,
            role: 2,
            review_count: 1,
            star_1: 0,
            star_2: 0,
            star_3: 0,
            star_4: 1,
            star_5: 0,
            weight_sum_bps: 2000,
            weighted_star_sum: 8_000_000,
            bayesian_score_milli: 3038,
            confidence_bps: 384,
            receipt_id: id("0x7a5c"),
            updated_at_ms: 5,
        };
        let json = parsed("ReputationUpdated", &bcs::to_bytes(&ev).unwrap());
        let rows = handle_sub_agent_registry_event("ReputationUpdated", &json, "d:0").unwrap();
        let [SubAgentRegistryRow::Reputation(row)] = rows.as_slice() else {
            panic!("expected one Reputation row");
        };
        assert_eq!((row.role, row.subject_kind), (2, 3));
        assert!(row.organization_id.is_none());
    }

    #[test]
    fn credibility_updated_becomes_a_credibility_row() {
        let ev = events::BcsCredibilityUpdated {
            account_id: id("0xacc"),
            principal_owner: address("0xb0b"),
            settled_count: 3,
            volume_mist: 3_000_000_000,
            credibility_bps: 3000,
            updated_at_ms: 9,
        };
        let json = parsed("CredibilityUpdated", &bcs::to_bytes(&ev).unwrap());
        let rows = handle_sub_agent_registry_event("CredibilityUpdated", &json, "d:1").unwrap();
        let [SubAgentRegistryRow::Credibility(row)] = rows.as_slice() else {
            panic!("expected one Credibility row");
        };
        assert_eq!(row.settled_count, 3);
        assert_eq!(row.volume_mist, "3000000000");
        assert_eq!(row.credibility_bps, 3000);
    }

    #[test]
    fn a_non_decimal_sum_is_rejected_rather_than_stored() {
        let data = serde_json::json!({
            "subject_id": "0x1", "role": 1, "subject_kind": 1, "principal_owner": "0x2",
            "organization_id": null, "review_count": 1, "star_1": 0, "star_2": 0, "star_3": 0,
            "star_4": 0, "star_5": 1, "weight_sum_bps": "12x", "weighted_star_sum": "5",
            "bayesian_score_milli": 3000, "confidence_bps": 1, "receipt_id": "0x3",
            "updated_at_ms": 1
        });
        assert!(reputation_row(&data, "d:0", "d").is_none());
    }

    #[test]
    fn every_memory_reputation_event_is_in_the_registry_event_list() {
        let move_source =
            include_str!("../../../myso-framework/packages/myso-social/sources/memory.move");
        for event in ["ReputationUpdated", "CredibilityUpdated"] {
            assert!(
                move_source.contains(&format!("public struct {event} has copy, drop")),
                "{event} is not declared in memory.move"
            );
            assert!(
                SUB_AGENT_REGISTRY_EVENTS.contains(&event),
                "{event} has no handler entry"
            );
        }
    }
}
