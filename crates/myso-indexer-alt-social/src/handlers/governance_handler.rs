// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Governance pipeline: indexes governance module events.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::ExpressionMethods;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Int2, SmallInt, Text};
use diesel_async::RunQueryDsl;
use move_core_types::ident_str;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::types::full_checkpoint_content::{
    Checkpoint, ExecutedTransaction, ObjectSet,
};
use myso_indexer_alt_social_schema::PROPOSAL_TYPE_PLATFORM;
use myso_indexer_alt_social_schema::models::{
    GovernanceRegistryPanelBoundaryUpdate, GovernanceRegistryUpdate, NewAnonymousVote,
    NewCommunityVote, NewDelegate, NewDelegateRating, NewDelegateVote, NewGovernanceEvent,
    NewGovernanceRegistry, NewNominatedDelegate, NewProposal, NewRewardDistribution,
    NewVoteDecryptionFailure, ProposalUpdateSet,
};
use myso_indexer_alt_social_schema::schema::{
    anonymous_votes, community_votes, delegate_ratings, delegate_votes, delegates,
    governance_events, governance_registries, nominated_delegates, platforms, proposals,
    reward_distributions, vote_decryption_failures,
};
use myso_types::MYSO_SOCIAL_ADDRESS;
use myso_types::object::ID_END_INDEX;
use myso_types::storage::ObjectKey;

use super::common;
use super::events;
use super::governance;

const GOVERNANCE_MODULES: &[&str] = &["governance"];

#[derive(QueryableByName)]
struct LatestProposalGovernanceMeta {
    #[diesel(sql_type = SmallInt)]
    proposal_type: i16,
    #[diesel(sql_type = Text)]
    governance_registry_id: String,
}

async fn load_latest_proposal_governance_meta(
    conn: &mut Connection<'_>,
    proposal_id: &str,
) -> Option<LatestProposalGovernanceMeta> {
    diesel::sql_query(
        r#"SELECT proposal_type, governance_registry_id FROM proposals
           WHERE id = $1 AND time = (SELECT max(time) FROM proposals p2 WHERE p2.id = $1)"#,
    )
    .bind::<Text, _>(proposal_id)
    .get_result::<LatestProposalGovernanceMeta>(conn)
    .await
    .ok()
}

#[derive(QueryableByName)]
struct LatestNomineeVotes {
    #[diesel(sql_type = BigInt)]
    upvotes: i64,
    #[diesel(sql_type = BigInt)]
    downvotes: i64,
}

/// Latest [`nominated_delegates`] snapshot for this address (max `time`).
async fn load_latest_nominee_vote_totals(
    conn: &mut Connection<'_>,
    address: &str,
    registry_type: i16,
    governance_registry_id: &str,
) -> Option<(i64, i64)> {
    diesel::sql_query(
        r#"SELECT upvotes, downvotes FROM nominated_delegates
           WHERE address = $1 AND registry_type = $2 AND governance_registry_id = $3
             AND time = (SELECT MAX(time) FROM nominated_delegates
                         WHERE address = $1 AND registry_type = $2 AND governance_registry_id = $3)"#,
    )
    .bind::<Text, _>(address)
    .bind::<SmallInt, _>(registry_type)
    .bind::<Text, _>(governance_registry_id)
    .get_result::<LatestNomineeVotes>(conn)
    .await
    .ok()
    .map(|r| (r.upvotes, r.downvotes))
}

/// When the election JSON has no vote counts (0/0), use the latest nominee row if it has votes.
fn apply_nominee_vote_carryover_to_delegate_row(
    mut delegate: NewDelegate,
    nominee_upvotes: i64,
    nominee_downvotes: i64,
) -> NewDelegate {
    if delegate.upvotes == 0
        && delegate.downvotes == 0
        && (nominee_upvotes != 0 || nominee_downvotes != 0)
    {
        tracing::debug!(
            address = %delegate.address,
            registry_type = delegate.registry_type,
            nominee_upvotes,
            nominee_downvotes,
            "DelegateElected JSON had 0/0; applied vote counts from latest nominated_delegates row"
        );
        delegate.upvotes = nominee_upvotes;
        delegate.downvotes = nominee_downvotes;
    }
    delegate
}

#[derive(Debug, Clone)]
pub enum GovernanceRow {
    GovernanceRegistry(NewGovernanceRegistry),
    GovernanceRegistryUpdate(GovernanceRegistryUpdate),
    GovernanceRegistryPanelBoundary(GovernanceRegistryPanelBoundaryUpdate),
    NominatedDelegate(NewNominatedDelegate),
    Delegate(NewDelegate),
    Proposal(NewProposal),
    ProposalUpdate {
        proposal_id: String,
        set: ProposalUpdateSet,
        governance_event: Option<(String, serde_json::Value, String)>,
        submitter_filter: Option<String>,
    },
    DelegateRating(NewDelegateRating),
    DelegateVote(NewDelegateVote),
    CommunityVote(NewCommunityVote),
    RewardDistribution(NewRewardDistribution),
    GovernanceEvent(NewGovernanceEvent),
    GovernanceEventFromProposal {
        proposal_id: String,
        event_type: String,
        event_data: serde_json::Value,
        event_id: String,
        anonymous_voting_related: Option<bool>,
    },
    AnonymousVote(NewAnonymousVote),
    VoteDecryptionFailure(NewVoteDecryptionFailure),
    NominatedDelegateStatusUpdate {
        address: String,
        registry_type: i16,
        status: i16,
        governance_registry_id: String,
    },
    DelegateVoteCountsUpdate {
        target_address: String,
        registry_type: i16,
        is_active_delegate: bool,
        upvotes: i64,
        downvotes: i64,
        governance_registry_id: String,
    },
    ProposalDelegateVoteIncrement {
        proposal_id: String,
        approve: bool,
    },
    DelegateProposalsReviewedIncrement {
        proposal_id: String,
        delegate_address: String,
    },
    ProposalCommunityVoteUpdate {
        proposal_id: String,
        votes_for_delta: i64,
        votes_against_delta: i64,
        reward_pool_delta: i64,
    },
    ProposalOutcomeApplyDelegateSidedUpdates {
        proposal_id: String,
        approvers_win: bool,
    },
    DelegateProposalsSubmittedIncrement {
        proposal_id: String,
        submitter: String,
    },
    ProposalAnonymousVotersIncrement {
        proposal_id: String,
    },
}

impl GovernanceRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::GovernanceRegistry(reg) => {
                Some(GovernanceRow::GovernanceRegistry(reg))
            }
            crate::handlers::SocialEventRow::GovernanceRegistryUpdate(up) => {
                Some(GovernanceRow::GovernanceRegistryUpdate(up))
            }
            crate::handlers::SocialEventRow::GovernanceRegistryPanelBoundary(up) => {
                Some(GovernanceRow::GovernanceRegistryPanelBoundary(up))
            }
            crate::handlers::SocialEventRow::NominatedDelegate(n) => {
                Some(GovernanceRow::NominatedDelegate(n))
            }
            crate::handlers::SocialEventRow::Delegate(d) => Some(GovernanceRow::Delegate(d)),
            crate::handlers::SocialEventRow::Proposal(p) => Some(GovernanceRow::Proposal(p)),
            crate::handlers::SocialEventRow::ProposalUpdate {
                proposal_id,
                set,
                governance_event,
                submitter_filter,
            } => Some(GovernanceRow::ProposalUpdate {
                proposal_id,
                set,
                governance_event,
                submitter_filter,
            }),
            crate::handlers::SocialEventRow::DelegateRating(r) => {
                Some(GovernanceRow::DelegateRating(r))
            }
            crate::handlers::SocialEventRow::DelegateVote(v) => {
                Some(GovernanceRow::DelegateVote(v))
            }
            crate::handlers::SocialEventRow::CommunityVote(v) => {
                Some(GovernanceRow::CommunityVote(v))
            }
            crate::handlers::SocialEventRow::RewardDistribution(r) => {
                Some(GovernanceRow::RewardDistribution(r))
            }
            crate::handlers::SocialEventRow::GovernanceEvent(ev) => {
                Some(GovernanceRow::GovernanceEvent(ev))
            }
            crate::handlers::SocialEventRow::GovernanceEventFromProposal {
                proposal_id,
                event_type,
                event_data,
                event_id,
                anonymous_voting_related,
            } => Some(GovernanceRow::GovernanceEventFromProposal {
                proposal_id,
                event_type,
                event_data,
                event_id,
                anonymous_voting_related,
            }),
            crate::handlers::SocialEventRow::AnonymousVote(v) => {
                Some(GovernanceRow::AnonymousVote(v))
            }
            crate::handlers::SocialEventRow::VoteDecryptionFailure(f) => {
                Some(GovernanceRow::VoteDecryptionFailure(f))
            }
            crate::handlers::SocialEventRow::NominatedDelegateStatusUpdate {
                address,
                registry_type,
                status,
                governance_registry_id,
            } => Some(GovernanceRow::NominatedDelegateStatusUpdate {
                address,
                registry_type,
                status,
                governance_registry_id,
            }),
            crate::handlers::SocialEventRow::DelegateVoteCountsUpdate {
                target_address,
                registry_type,
                is_active_delegate,
                upvotes,
                downvotes,
                governance_registry_id,
            } => Some(GovernanceRow::DelegateVoteCountsUpdate {
                target_address,
                registry_type,
                is_active_delegate,
                upvotes,
                downvotes,
                governance_registry_id,
            }),
            crate::handlers::SocialEventRow::ProposalDelegateVoteIncrement {
                proposal_id,
                approve,
            } => Some(GovernanceRow::ProposalDelegateVoteIncrement {
                proposal_id,
                approve,
            }),
            crate::handlers::SocialEventRow::DelegateProposalsReviewedIncrement {
                proposal_id,
                delegate_address,
            } => Some(GovernanceRow::DelegateProposalsReviewedIncrement {
                proposal_id,
                delegate_address,
            }),
            crate::handlers::SocialEventRow::ProposalCommunityVoteUpdate {
                proposal_id,
                votes_for_delta,
                votes_against_delta,
                reward_pool_delta,
            } => Some(GovernanceRow::ProposalCommunityVoteUpdate {
                proposal_id,
                votes_for_delta,
                votes_against_delta,
                reward_pool_delta,
            }),
            crate::handlers::SocialEventRow::ProposalOutcomeApplyDelegateSidedUpdates {
                proposal_id,
                approvers_win,
            } => Some(GovernanceRow::ProposalOutcomeApplyDelegateSidedUpdates {
                proposal_id,
                approvers_win,
            }),
            crate::handlers::SocialEventRow::DelegateProposalsSubmittedIncrement {
                proposal_id,
                submitter,
            } => Some(GovernanceRow::DelegateProposalsSubmittedIncrement {
                proposal_id,
                submitter,
            }),
            crate::handlers::SocialEventRow::ProposalAnonymousVotersIncrement { proposal_id } => {
                Some(GovernanceRow::ProposalAnonymousVotersIncrement { proposal_id })
            }
            _ => None,
        }
    }
}

impl FieldCount for GovernanceRow {
    const FIELD_COUNT: usize = 50;
}

pub struct GovernanceHandler;

/// `registry_type` / `proposal_type` from parsed governance event JSON, when present.
fn governance_event_registry_type_hint(event_name: &str, data: &serde_json::Value) -> Option<u8> {
    if event_name == "GovernanceRegistryCreatedEvent" {
        return None;
    }
    if let Some(v) = data
        .get("registry_type")
        .and_then(serde_json::Value::as_u64)
    {
        return u8::try_from(v).ok();
    }
    if let Some(v) = data
        .get("proposal_type")
        .and_then(serde_json::Value::as_u64)
    {
        return u8::try_from(v).ok();
    }
    None
}

/// After a Move object's `id: UID` (32 bytes), `GovernanceDAO` stores `registry_type: u8`.
fn governance_dao_registry_type_from_contents(contents: &[u8]) -> Option<u8> {
    contents.get(ID_END_INDEX).copied()
}

fn collect_governance_dao_candidates(
    object_set: &ObjectSet,
    tx: &ExecutedTransaction,
) -> BTreeMap<String, Option<u8>> {
    let mut candidates: BTreeMap<String, Option<u8>> = BTreeMap::new();
    for ((oid, version, _), _owner, _write_kind) in tx.effects.all_changed_objects() {
        let Some(obj) = object_set.get(&ObjectKey(oid, version)) else {
            continue;
        };
        let Some(t) = obj.type_() else {
            continue;
        };
        if t.address() != MYSO_SOCIAL_ADDRESS {
            continue;
        }
        if t.module() == ident_str!("governance") && t.name() == ident_str!("GovernanceDAO") {
            let oid_s = oid.to_string();
            let reg_ty = obj
                .as_inner()
                .data
                .try_as_move()
                .and_then(|m| governance_dao_registry_type_from_contents(m.contents()));
            candidates.insert(oid_s, reg_ty);
        }
    }
    candidates
}

/// Picks the DAO object id when `candidates` maps registry object id -> parsed `registry_type` byte.
fn pick_governance_registry_id(
    candidates: BTreeMap<String, Option<u8>>,
    event_registry_type: Option<u8>,
) -> Option<String> {
    match candidates.len() {
        0 => None,
        1 => candidates.into_keys().next(),
        n => {
            if let Some(filter) = event_registry_type {
                let matched: Vec<String> = candidates
                    .iter()
                    .filter(|(_, rt)| **rt == Some(filter))
                    .map(|(id, _)| id.clone())
                    .collect();
                if matched.len() == 1 {
                    return Some(matched[0].clone());
                }
                if matched.is_empty() {
                    tracing::warn!(
                        count = n,
                        registry_type = filter,
                        "GovernanceDAO writes in tx: none matched event registry_type; omitting governance_registry_id"
                    );
                } else {
                    tracing::warn!(
                        count = matched.len(),
                        registry_type = filter,
                        "GovernanceDAO writes in tx: multiple matched event registry_type; omitting governance_registry_id"
                    );
                }
                None
            } else {
                tracing::warn!(
                    count = n,
                    "ambiguous GovernanceDAO writes in tx; omitting governance_registry_id"
                );
                None
            }
        }
    }
}

/// Identifies the `GovernanceDAO` object for this transaction when possible (mutated, created, or unwrapped).
fn resolve_governance_registry_id_from_tx(
    object_set: &ObjectSet,
    tx: &ExecutedTransaction,
    event_registry_type: Option<u8>,
) -> Option<String> {
    let candidates = collect_governance_dao_candidates(object_set, tx);
    pick_governance_registry_id(candidates, event_registry_type)
}

/// Collects `(registry_id, registry_type)` from any governance event in the transaction whose BCS
/// payload includes `registry_id` (e.g. `DelegatePanelRefreshedEvent` after the elected events).
/// Used as a fallback when `resolve_governance_registry_id_from_tx` does not see a `GovernanceDAO`
/// write (or returns ambiguous) but events still carry the DAO object id.
fn collect_governance_registry_id_hints(tx: &ExecutedTransaction) -> Vec<(String, Option<u8>)> {
    let Some(events) = &tx.events else {
        return Vec::new();
    };
    let mut hints = Vec::new();
    for ev in &events.data {
        if !common::is_social_package_event(&ev.package_id, &ev.type_.address) {
            continue;
        }
        let module = ev.type_.module.as_str();
        if !GOVERNANCE_MODULES.contains(&module) {
            continue;
        }
        let event_name = ev.type_.name.as_str();
        let Ok(value) = events::parse_event_contents(module, event_name, &ev.contents) else {
            continue;
        };
        let Some(s) = value.get("registry_id").and_then(serde_json::Value::as_str) else {
            continue;
        };
        let id = s.to_string();
        let registry_type = value
            .get("registry_type")
            .and_then(serde_json::Value::as_u64)
            .and_then(|n| u8::try_from(n).ok());
        hints.push((id, registry_type));
    }
    hints
}

/// Picks a single `registry_id` from per-transaction event hints, optionally filtered by
/// `registry_type` when multiple distinct DAOs appear in the same transaction.
fn pick_registry_id_from_event_hints(
    hints: &[(String, Option<u8>)],
    event_registry_type: Option<u8>,
) -> Option<String> {
    if hints.is_empty() {
        return None;
    }
    let distinct_ids: BTreeSet<&str> = hints.iter().map(|(id, _)| id.as_str()).collect();
    if distinct_ids.len() == 1 {
        return Some(distinct_ids.iter().next().unwrap().to_string());
    }
    let filter = event_registry_type?;
    let matched: BTreeSet<&str> = hints
        .iter()
        .filter(|(_, rt)| *rt == Some(filter))
        .map(|(id, _)| id.as_str())
        .collect();
    (matched.len() == 1).then(|| matched.iter().next().unwrap().to_string())
}

fn resolve_effective_governance_registry_id(
    object_set: &ObjectSet,
    tx: &ExecutedTransaction,
    event_registry_type: Option<u8>,
    tx_registry_hints: &[(String, Option<u8>)],
) -> Option<String> {
    resolve_governance_registry_id_from_tx(object_set, tx, event_registry_type)
        .or_else(|| pick_registry_id_from_event_hints(tx_registry_hints, event_registry_type))
}

#[async_trait]
impl Processor for GovernanceHandler {
    const NAME: &'static str = "governance";

    type Value = GovernanceRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let mut values = Vec::new();
        for tx in &checkpoint.transactions {
            let tx_digest = tx.transaction.digest().to_string();
            let Some(events) = &tx.events else {
                continue;
            };
            let tx_registry_hints = collect_governance_registry_id_hints(tx);
            for (event_seq, ev) in events.data.iter().enumerate() {
                if !common::is_social_package_event(&ev.package_id, &ev.type_.address) {
                    continue;
                }
                let module = ev.type_.module.as_str();
                if !GOVERNANCE_MODULES.contains(&module) {
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
                                hex_preview = %e.contents_hex_preview(32),
                                "governance event parse failed"
                            );
                            continue;
                        }
                    };
                let hint = governance_event_registry_type_hint(event_name, &event_data);
                let governance_registry_id = resolve_effective_governance_registry_id(
                    &checkpoint.object_set,
                    tx,
                    hint,
                    &tx_registry_hints,
                );
                if let Some(rows) = governance::handle_governance_event(
                    event_name,
                    &event_data,
                    &event_id,
                    governance_registry_id,
                ) {
                    for row in rows {
                        if let Some(r) = GovernanceRow::from_social(row) {
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
impl Handler for GovernanceHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                GovernanceRow::GovernanceRegistry(reg) => {
                    let registry_type = reg.registry_type;
                    let registry_id = reg.registry_id.clone();
                    let delegate_count = reg.delegate_count;
                    let delegate_term_epochs = reg.delegate_term_epochs;
                    let proposal_submission_cost = reg.proposal_submission_cost;
                    let max_votes_per_user = reg.max_votes_per_user;
                    let quadratic_base_cost = reg.quadratic_base_cost;
                    let voting_period_ms = reg.voting_period_ms;
                    let quorum_votes = reg.quorum_votes;
                    let updated_at = reg.updated_at;
                    let transaction_id = reg.transaction_id.clone();
                    total += diesel::insert_into(governance_registries::table)
                        .values(reg.clone())
                        .on_conflict(governance_registries::registry_id)
                        .do_update()
                        .set((
                            governance_registries::registry_type.eq(registry_type),
                            governance_registries::delegate_count.eq(delegate_count),
                            governance_registries::delegate_term_epochs.eq(delegate_term_epochs),
                            governance_registries::proposal_submission_cost
                                .eq(proposal_submission_cost),
                            governance_registries::max_votes_per_user.eq(max_votes_per_user),
                            governance_registries::quadratic_base_cost.eq(quadratic_base_cost),
                            governance_registries::voting_period_ms.eq(voting_period_ms),
                            governance_registries::quorum_votes.eq(quorum_votes),
                            governance_registries::updated_at.eq(updated_at),
                            governance_registries::transaction_id.eq(transaction_id),
                            governance_registries::registry_id.eq(registry_id),
                        ))
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::GovernanceRegistryUpdate(up) => {
                    total += if let Some(ref rid) = up.registry_id {
                        diesel::update(governance_registries::table)
                            .filter(governance_registries::registry_id.eq(rid))
                            .set((
                                governance_registries::delegate_count.eq(up.delegate_count),
                                governance_registries::delegate_term_epochs
                                    .eq(up.delegate_term_epochs),
                                governance_registries::proposal_submission_cost
                                    .eq(up.proposal_submission_cost),
                                governance_registries::max_votes_per_user.eq(up.max_votes_per_user),
                                governance_registries::quadratic_base_cost
                                    .eq(up.quadratic_base_cost),
                                governance_registries::voting_period_ms.eq(up.voting_period_ms),
                                governance_registries::quorum_votes.eq(up.quorum_votes),
                                governance_registries::updated_at.eq(up.updated_at),
                                governance_registries::transaction_id.eq(up.transaction_id.clone()),
                            ))
                            .execute(conn)
                            .await?
                    } else {
                        diesel::update(governance_registries::table)
                            .filter(governance_registries::registry_type.eq(up.registry_type))
                            .set((
                                governance_registries::delegate_count.eq(up.delegate_count),
                                governance_registries::delegate_term_epochs
                                    .eq(up.delegate_term_epochs),
                                governance_registries::proposal_submission_cost
                                    .eq(up.proposal_submission_cost),
                                governance_registries::max_votes_per_user.eq(up.max_votes_per_user),
                                governance_registries::quadratic_base_cost
                                    .eq(up.quadratic_base_cost),
                                governance_registries::voting_period_ms.eq(up.voting_period_ms),
                                governance_registries::quorum_votes.eq(up.quorum_votes),
                                governance_registries::updated_at.eq(up.updated_at),
                                governance_registries::transaction_id.eq(up.transaction_id.clone()),
                            ))
                            .execute(conn)
                            .await?
                    };

                    if up.registry_type == PROPOSAL_TYPE_PLATFORM {
                        if let Some(ref rid) = up.registry_id {
                            let now = chrono::Utc::now().naive_utc();
                            total += diesel::update(platforms::table)
                                .filter(platforms::governance_registry_id.eq(rid))
                                .filter(platforms::deleted_at.is_null())
                                .set((
                                    platforms::delegate_count.eq(up.delegate_count),
                                    platforms::delegate_term_epochs.eq(up.delegate_term_epochs),
                                    platforms::proposal_submission_cost
                                        .eq(up.proposal_submission_cost),
                                    platforms::max_votes_per_user.eq(up.max_votes_per_user),
                                    platforms::quadratic_base_cost.eq(up.quadratic_base_cost),
                                    platforms::voting_period_epochs.eq(up.voting_period_ms),
                                    platforms::quorum_votes.eq(up.quorum_votes),
                                    platforms::updated_at.eq(now),
                                ))
                                .execute(conn)
                                .await?;
                        }
                    }
                }
                GovernanceRow::GovernanceRegistryPanelBoundary(up) => {
                    total += diesel::update(governance_registries::table)
                        .filter(governance_registries::registry_id.eq(&up.registry_id))
                        .set((
                            governance_registries::last_delegate_panel_boundary_epoch
                                .eq(Some(up.last_delegate_panel_boundary_epoch)),
                            governance_registries::updated_at.eq(up.updated_at),
                            governance_registries::transaction_id.eq(up.transaction_id.clone()),
                        ))
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::NominatedDelegate(n) => {
                    total += diesel::insert_into(nominated_delegates::table)
                        .values(n)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::Delegate(d_src) => {
                    let mut d = d_src.clone();
                    if d.upvotes == 0 && d.downvotes == 0 {
                        if let Some((nu, nd)) = load_latest_nominee_vote_totals(
                            conn,
                            &d.address,
                            d.registry_type,
                            &d.governance_registry_id,
                        )
                        .await
                        {
                            d = apply_nominee_vote_carryover_to_delegate_row(d, nu, nd);
                        }
                    }
                    total += diesel::sql_query(
                        "UPDATE delegates SET is_active = false \
                         WHERE address = $1 AND registry_type = $2 AND governance_registry_id = $3",
                    )
                    .bind::<Text, _>(&d.address)
                    .bind::<SmallInt, _>(d.registry_type)
                    .bind::<Text, _>(&d.governance_registry_id)
                    .execute(conn)
                    .await?;
                    total += diesel::insert_into(delegates::table)
                        .values(&d)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::Proposal(p) => {
                    total += diesel::insert_into(proposals::table)
                        .values(p)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::ProposalUpdate {
                    proposal_id,
                    set,
                    governance_event,
                    submitter_filter,
                } => {
                    total += if let Some(ref s) = submitter_filter {
                        diesel::update(proposals::table)
                            .filter(proposals::id.eq(proposal_id))
                            .filter(proposals::submitter.eq(s))
                            .set(set)
                            .execute(conn)
                            .await?
                    } else {
                        diesel::update(proposals::table)
                            .filter(proposals::id.eq(proposal_id))
                            .set(set)
                            .execute(conn)
                            .await?
                    };
                    if let Some((event_type, event_data, event_id)) = governance_event {
                        if let Some(meta) =
                            load_latest_proposal_governance_meta(conn, proposal_id).await
                        {
                            let gov_ev = NewGovernanceEvent {
                                event_type: event_type.clone(),
                                registry_type: meta.proposal_type,
                                event_data: event_data.clone(),
                                event_id: event_id.clone(),
                                created_at: chrono::Utc::now(),
                                anonymous_voting_related: None,
                                governance_registry_id: Some(meta.governance_registry_id),
                                proposal_id: Some(proposal_id.clone()),
                            };
                            total += diesel::insert_into(governance_events::table)
                                .values(&gov_ev)
                                .execute(conn)
                                .await?;
                        } else {
                            tracing::warn!(
                                proposal_id = %proposal_id,
                                event_type = %event_type,
                                "governance_events insert skipped: missing proposal row for lifecycle event"
                            );
                        }
                    }
                }
                GovernanceRow::DelegateRating(r) => {
                    total += diesel::insert_into(delegate_ratings::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::DelegateVote(v) => {
                    total += diesel::insert_into(delegate_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::CommunityVote(v) => {
                    total += diesel::insert_into(community_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::RewardDistribution(r) => {
                    total += diesel::insert_into(reward_distributions::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::GovernanceEvent(ev) => {
                    total += diesel::insert_into(governance_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::GovernanceEventFromProposal {
                    proposal_id,
                    event_type,
                    event_data,
                    event_id,
                    anonymous_voting_related,
                } => {
                    if let Some(meta) =
                        load_latest_proposal_governance_meta(conn, proposal_id).await
                    {
                        let gov_ev = NewGovernanceEvent {
                            event_type: event_type.clone(),
                            registry_type: meta.proposal_type,
                            event_data: event_data.clone(),
                            event_id: event_id.clone(),
                            created_at: chrono::Utc::now(),
                            anonymous_voting_related: *anonymous_voting_related,
                            governance_registry_id: Some(meta.governance_registry_id),
                            proposal_id: Some(proposal_id.clone()),
                        };
                        total += diesel::insert_into(governance_events::table)
                            .values(&gov_ev)
                            .execute(conn)
                            .await?;
                    } else {
                        tracing::warn!(
                            proposal_id = %proposal_id,
                            event_type = %event_type,
                            "governance_events insert skipped: missing proposal row for proposal-linked event"
                        );
                    }
                }
                GovernanceRow::AnonymousVote(v) => {
                    total += diesel::insert_into(anonymous_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::VoteDecryptionFailure(f) => {
                    total += diesel::insert_into(vote_decryption_failures::table)
                        .values(f)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::NominatedDelegateStatusUpdate {
                    address,
                    registry_type,
                    status,
                    governance_registry_id,
                } => {
                    let upd = diesel::sql_query(
                        "UPDATE nominated_delegates SET status = $1 WHERE address = $2 AND registry_type = $3 AND governance_registry_id IS NOT DISTINCT FROM $4 AND time = (SELECT max(time) FROM nominated_delegates WHERE address = $2 AND registry_type = $3 AND governance_registry_id IS NOT DISTINCT FROM $4)",
                    )
                    .bind::<Int2, _>(*status)
                    .bind::<Text, _>(address)
                    .bind::<Int2, _>(*registry_type)
                    .bind::<Text, _>(governance_registry_id);
                    total += upd.execute(conn).await?;
                }
                GovernanceRow::DelegateVoteCountsUpdate {
                    target_address,
                    registry_type,
                    is_active_delegate,
                    upvotes,
                    downvotes,
                    governance_registry_id,
                } => {
                    if *is_active_delegate {
                        let upd = diesel::sql_query(
                            "UPDATE delegates SET upvotes = $1, downvotes = $2 WHERE address = $3 AND registry_type = $4 AND governance_registry_id IS NOT DISTINCT FROM $5 AND time = (SELECT max(time) FROM delegates WHERE address = $3 AND registry_type = $4 AND governance_registry_id IS NOT DISTINCT FROM $5)",
                        )
                        .bind::<BigInt, _>(*upvotes)
                        .bind::<BigInt, _>(*downvotes)
                        .bind::<Text, _>(target_address)
                        .bind::<Int2, _>(*registry_type)
                        .bind::<Text, _>(governance_registry_id);
                        total += upd.execute(conn).await?;
                    } else {
                        let upd = diesel::sql_query(
                            "UPDATE nominated_delegates SET upvotes = $1, downvotes = $2 WHERE address = $3 AND registry_type = $4 AND governance_registry_id IS NOT DISTINCT FROM $5 AND time = (SELECT max(time) FROM nominated_delegates WHERE address = $3 AND registry_type = $4 AND governance_registry_id IS NOT DISTINCT FROM $5)",
                        )
                        .bind::<BigInt, _>(*upvotes)
                        .bind::<BigInt, _>(*downvotes)
                        .bind::<Text, _>(target_address)
                        .bind::<Int2, _>(*registry_type)
                        .bind::<Text, _>(governance_registry_id);
                        total += upd.execute(conn).await?;
                    }
                }
                GovernanceRow::ProposalDelegateVoteIncrement {
                    proposal_id,
                    approve,
                } => {
                    let sql = if *approve {
                        "UPDATE proposals SET delegate_approval_count = delegate_approval_count + 1 WHERE id = $1 AND time = (SELECT max(time) FROM proposals WHERE id = $1)"
                    } else {
                        "UPDATE proposals SET delegate_rejection_count = delegate_rejection_count + 1 WHERE id = $1 AND time = (SELECT max(time) FROM proposals WHERE id = $1)"
                    };
                    total += diesel::sql_query(sql)
                        .bind::<Text, _>(proposal_id)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::DelegateProposalsReviewedIncrement {
                    proposal_id,
                    delegate_address,
                } => {
                    let upd = diesel::sql_query(
                        "UPDATE delegates d SET proposals_reviewed = proposals_reviewed + 1 \
                         FROM proposals p \
                         WHERE p.id = $1 AND p.time = (SELECT max(time) FROM proposals pm WHERE pm.id = p.id) \
                         AND d.address = $2 AND d.registry_type = p.proposal_type \
                         AND d.is_active = true \
                         AND d.governance_registry_id IS NOT DISTINCT FROM p.governance_registry_id \
                         AND d.time = (SELECT max(time) FROM delegates d2 \
                         WHERE d2.address = d.address AND d2.registry_type = d.registry_type \
                         AND d2.governance_registry_id IS NOT DISTINCT FROM d.governance_registry_id)",
                    )
                    .bind::<Text, _>(proposal_id)
                    .bind::<Text, _>(delegate_address);
                    total += upd.execute(conn).await?;
                }
                GovernanceRow::ProposalCommunityVoteUpdate {
                    proposal_id,
                    votes_for_delta,
                    votes_against_delta,
                    reward_pool_delta,
                } => {
                    let upd = diesel::sql_query(
                        "UPDATE proposals SET community_votes_for = community_votes_for + $1, community_votes_against = community_votes_against + $2, reward_pool = reward_pool + $3 WHERE id = $4 AND time = (SELECT max(time) FROM proposals WHERE id = $4)",
                    )
                    .bind::<BigInt, _>(*votes_for_delta)
                    .bind::<BigInt, _>(*votes_against_delta)
                    .bind::<BigInt, _>(*reward_pool_delta)
                    .bind::<Text, _>(proposal_id);
                    total += upd.execute(conn).await?;
                }
                GovernanceRow::ProposalOutcomeApplyDelegateSidedUpdates {
                    proposal_id,
                    approvers_win,
                } => {
                    let subq = "SELECT DISTINCT ON (dv.delegate_address) dv.delegate_address, dv.approve, p.proposal_type, p.governance_registry_id \
                         FROM delegate_votes dv \
                         INNER JOIN proposals p ON p.id = dv.proposal_id AND p.time = (SELECT max(time) FROM proposals pm WHERE pm.id = p.id) \
                         WHERE dv.proposal_id = $1 \
                         ORDER BY dv.delegate_address, dv.time DESC";
                    let win_sql = format!(
                        "UPDATE delegates d SET sided_winning_proposals = sided_winning_proposals + 1 FROM ({}) x \
                         WHERE d.address = x.delegate_address AND d.registry_type = x.proposal_type AND d.is_active = true \
                         AND d.governance_registry_id IS NOT DISTINCT FROM x.governance_registry_id \
                         AND d.time = (SELECT max(time) FROM delegates d2 WHERE d2.address = d.address \
                         AND d2.registry_type = d.registry_type \
                         AND d2.governance_registry_id IS NOT DISTINCT FROM d.governance_registry_id) \
                         AND x.approve = $2",
                        subq,
                    );
                    let lose_sql = format!(
                        "UPDATE delegates d SET sided_losing_proposals = sided_losing_proposals + 1 FROM ({}) x \
                         WHERE d.address = x.delegate_address AND d.registry_type = x.proposal_type AND d.is_active = true \
                         AND d.governance_registry_id IS NOT DISTINCT FROM x.governance_registry_id \
                         AND d.time = (SELECT max(time) FROM delegates d2 WHERE d2.address = d.address \
                         AND d2.registry_type = d.registry_type \
                         AND d2.governance_registry_id IS NOT DISTINCT FROM d.governance_registry_id) \
                         AND x.approve = $2",
                        subq,
                    );
                    total += diesel::sql_query(&win_sql)
                        .bind::<Text, _>(proposal_id)
                        .bind::<Bool, _>(*approvers_win)
                        .execute(conn)
                        .await?;
                    total += diesel::sql_query(&lose_sql)
                        .bind::<Text, _>(proposal_id)
                        .bind::<Bool, _>(!*approvers_win)
                        .execute(conn)
                        .await?;
                }
                GovernanceRow::DelegateProposalsSubmittedIncrement {
                    proposal_id,
                    submitter,
                } => {
                    let upd = diesel::sql_query(
                        "UPDATE delegates d SET proposals_submitted = proposals_submitted + 1 \
                         FROM proposals p \
                         WHERE p.id = $1 AND p.time = (SELECT max(time) FROM proposals pm WHERE pm.id = p.id) \
                         AND d.address = $2 AND d.registry_type = p.proposal_type \
                         AND d.is_active = true \
                         AND d.governance_registry_id IS NOT DISTINCT FROM p.governance_registry_id \
                         AND d.time = (SELECT max(time) FROM delegates d2 \
                         WHERE d2.address = d.address AND d2.registry_type = d.registry_type \
                         AND d2.governance_registry_id IS NOT DISTINCT FROM d.governance_registry_id)",
                    )
                    .bind::<Text, _>(proposal_id)
                    .bind::<Text, _>(submitter);
                    total += upd.execute(conn).await?;
                }
                GovernanceRow::ProposalAnonymousVotersIncrement { proposal_id } => {
                    let upd = diesel::sql_query(
                        "UPDATE proposals SET anonymous_voters_count = COALESCE(anonymous_voters_count, 0) + 1 WHERE id = $1 AND time = (SELECT max(time) FROM proposals WHERE id = $1)",
                    )
                    .bind::<Text, _>(proposal_id);
                    total += upd.execute(conn).await?;
                }
            }
        }
        Ok(total)
    }
}

#[cfg(test)]
mod resolve_governance_registry_tests {
    use std::collections::BTreeMap;

    use fastcrypto::ed25519::Ed25519KeyPair;
    use move_core_types::identifier::Identifier;
    use move_core_types::language_storage::StructTag;
    use myso_types::MYSO_SOCIAL_ADDRESS;
    use myso_types::base_types::{MoveObjectType, MySoAddress, ObjectID, SequenceNumber};
    use myso_types::crypto::{AccountKeyPair, get_key_pair_from_rng};
    use myso_types::digests::{ObjectDigest, TransactionDigest};
    use myso_types::effects::{TestEffectsBuilder, TransactionEffectsAPI};
    use myso_types::full_checkpoint_content::{ExecutedTransaction, ObjectSet};
    use myso_types::object::{MoveObject, Object, Owner};
    use myso_types::programmable_transaction_builder::ProgrammableTransactionBuilder;
    use myso_types::transaction::{Transaction, TransactionData};
    use myso_types::utils::to_sender_signed_transaction;
    use rand::SeedableRng;
    use rand::rngs::StdRng;

    use super::{
        ID_END_INDEX, collect_governance_dao_candidates, pick_governance_registry_id,
        pick_registry_id_from_event_hints, resolve_governance_registry_id_from_tx,
    };

    #[test]
    fn pick_disambiguates_two_daos_by_event_registry_type() {
        let mut m = BTreeMap::new();
        m.insert("0xaaa".to_string(), Some(0u8));
        m.insert("0xbbb".to_string(), Some(1u8));
        assert_eq!(
            pick_governance_registry_id(m.clone(), Some(0)),
            Some("0xaaa".to_string())
        );
        assert_eq!(
            pick_governance_registry_id(m, Some(1)),
            Some("0xbbb".to_string())
        );
    }

    #[test]
    fn event_hints_single_id_without_type() {
        let hints = vec![("0xreg".to_string(), Some(0u8))];
        assert_eq!(
            pick_registry_id_from_event_hints(&hints, None),
            Some("0xreg".to_string())
        );
    }

    #[test]
    fn event_hints_two_ids_disambiguate_by_type() {
        let hints = vec![
            ("0xaaa".to_string(), Some(0u8)),
            ("0xbbb".to_string(), Some(1u8)),
        ];
        assert_eq!(pick_registry_id_from_event_hints(&hints, None), None);
        assert_eq!(
            pick_registry_id_from_event_hints(&hints, Some(0u8)),
            Some("0xaaa".to_string())
        );
        assert_eq!(
            pick_registry_id_from_event_hints(&hints, Some(1u8)),
            Some("0xbbb".to_string())
        );
    }

    fn make_test_transaction(
        sender: MySoAddress,
        keypair: &AccountKeyPair,
        gas_object_id: ObjectID,
    ) -> Transaction {
        let pt = ProgrammableTransactionBuilder::new();
        let gas_object = (
            gas_object_id,
            SequenceNumber::from(1),
            ObjectDigest::random(),
        );
        let tx_data =
            TransactionData::new_programmable(sender, vec![gas_object], pt.finish(), 1000, 1);
        to_sender_signed_transaction(tx_data, keypair)
    }

    fn fake_governance_dao_object(
        id: ObjectID,
        version: SequenceNumber,
        registry_type: u8,
    ) -> Object {
        let mut contents = vec![0u8; 128];
        contents[0..32].copy_from_slice(id.as_ref());
        contents[ID_END_INDEX] = registry_type;
        let tag = StructTag {
            address: MYSO_SOCIAL_ADDRESS.into(),
            module: Identifier::new("governance").unwrap(),
            name: Identifier::new("GovernanceDAO").unwrap(),
            type_params: vec![],
        };
        let mo = unsafe {
            MoveObject::new_from_execution_with_limit(
                MoveObjectType::from(tag),
                false,
                version,
                contents,
                1024,
            )
            .unwrap()
        };
        Object::new_move(
            mo,
            Owner::Shared {
                initial_shared_version: SequenceNumber::from(1),
            },
            TransactionDigest::genesis_marker(),
        )
    }

    #[test]
    fn resolve_finds_two_created_governance_daos_like_bootstrap() {
        let mut rng = StdRng::from_seed([7u8; 32]);
        let (sender, keypair): (MySoAddress, AccountKeyPair) =
            get_key_pair_from_rng::<Ed25519KeyPair, _>(&mut rng);
        let gas_object_id = ObjectID::random();
        let tx = make_test_transaction(sender, &keypair, gas_object_id);

        let id_eco = ObjectID::random();
        let id_poc = ObjectID::random();

        let effects = TestEffectsBuilder::new(tx.data())
            .with_created_objects(vec![
                (id_eco, Owner::AddressOwner(sender)),
                (id_poc, Owner::AddressOwner(sender)),
            ])
            .build();

        let lamport = effects.lamport_version();
        let mut object_set = ObjectSet::default();
        object_set.insert(fake_governance_dao_object(id_eco, lamport, 0));
        object_set.insert(fake_governance_dao_object(id_poc, lamport, 1));

        let executed = ExecutedTransaction {
            transaction: tx.data().intent_message().value.clone(),
            signatures: tx.data().tx_signatures().to_vec(),
            effects,
            events: None,
            unchanged_loaded_runtime_objects: Vec::new(),
        };

        let candidates = collect_governance_dao_candidates(&object_set, &executed);
        assert_eq!(
            candidates.len(),
            2,
            "expected only GovernanceDAO objects in set"
        );

        assert_eq!(
            resolve_governance_registry_id_from_tx(&object_set, &executed, Some(0)),
            Some(id_eco.to_string())
        );
        assert_eq!(
            resolve_governance_registry_id_from_tx(&object_set, &executed, Some(1)),
            Some(id_poc.to_string())
        );
    }
}

#[cfg(test)]
mod delegate_vote_carryover_tests {
    use myso_indexer_alt_social_schema::models::NewDelegate;

    use super::apply_nominee_vote_carryover_to_delegate_row;

    fn sample_delegate(upvotes: i64, downvotes: i64) -> NewDelegate {
        NewDelegate {
            address: "0x1".to_string(),
            registry_type: 0,
            governance_registry_id: "0xreg".to_string(),
            upvotes,
            downvotes,
            proposals_reviewed: 0,
            proposals_submitted: 0,
            sided_winning_proposals: 0,
            sided_losing_proposals: 0,
            term_start: 1,
            term_end: 2,
            is_active: true,
            created_at: 0,
            updated_at: 0,
            transaction_id: "tx".to_string(),
        }
    }

    #[test]
    fn carryover_fills_zero_zero_from_nominee_snapshot() {
        let d = sample_delegate(0, 0);
        let out = apply_nominee_vote_carryover_to_delegate_row(d, 42, 5);
        assert_eq!(out.upvotes, 42);
        assert_eq!(out.downvotes, 5);
    }

    #[test]
    fn carryover_preserves_nonzero_event_counts() {
        let d = sample_delegate(10, 1);
        let out = apply_nominee_vote_carryover_to_delegate_row(d, 99, 99);
        assert_eq!(out.upvotes, 10);
        assert_eq!(out.downvotes, 1);
    }

    #[test]
    fn carryover_noop_when_nominee_snapshot_also_zero() {
        let d = sample_delegate(0, 0);
        let out = apply_nominee_vote_carryover_to_delegate_row(d, 0, 0);
        assert_eq!(out.upvotes, 0);
        assert_eq!(out.downvotes, 0);
    }
}
