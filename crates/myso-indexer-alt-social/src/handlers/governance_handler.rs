// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Governance pipeline: indexes governance module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Bool, Int2, Text};
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    GovernanceRegistryUpdate, NewAnonymousVote, NewCommunityVote, NewDelegate, NewDelegateRating,
    NewDelegateVote, NewGovernanceEvent, NewGovernanceRegistry, NewNominatedDelegate, NewProposal,
    NewRewardDistribution, NewVoteDecryptionFailure, ProposalUpdateSet,
};
use myso_indexer_alt_social_schema::schema::{
    anonymous_votes, community_votes, delegate_ratings, delegate_votes, delegates,
    governance_events, governance_registries, nominated_delegates, proposals, reward_distributions,
    vote_decryption_failures,
};

use super::common;
use super::events;
use super::governance;

const GOVERNANCE_MODULES: &[&str] = &["governance"];

#[derive(Debug, Clone)]
pub enum GovernanceRow {
    GovernanceRegistry(NewGovernanceRegistry),
    GovernanceRegistryUpdate(GovernanceRegistryUpdate),
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
    },
    DelegateVoteCountsUpdate {
        target_address: String,
        registry_type: i16,
        is_active_delegate: bool,
        upvotes: i64,
        downvotes: i64,
    },
    ProposalDelegateVoteIncrement {
        proposal_id: String,
        approve: bool,
    },
    DelegateProposalsReviewedIncrement {
        address: String,
    },
    ProposalCommunityVoteUpdate {
        proposal_id: String,
        votes_for_delta: i64,
        votes_against_delta: i64,
    },
    ProposalOutcomeApplyDelegateSidedUpdates {
        proposal_id: String,
        approvers_win: bool,
    },
    DelegateProposalsSubmittedIncrement {
        address: String,
        registry_type: i16,
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
            } => Some(GovernanceRow::NominatedDelegateStatusUpdate {
                address,
                registry_type,
                status,
            }),
            crate::handlers::SocialEventRow::DelegateVoteCountsUpdate {
                target_address,
                registry_type,
                is_active_delegate,
                upvotes,
                downvotes,
            } => Some(GovernanceRow::DelegateVoteCountsUpdate {
                target_address,
                registry_type,
                is_active_delegate,
                upvotes,
                downvotes,
            }),
            crate::handlers::SocialEventRow::ProposalDelegateVoteIncrement {
                proposal_id,
                approve,
            } => Some(GovernanceRow::ProposalDelegateVoteIncrement {
                proposal_id,
                approve,
            }),
            crate::handlers::SocialEventRow::DelegateProposalsReviewedIncrement { address } => {
                Some(GovernanceRow::DelegateProposalsReviewedIncrement { address })
            }
            crate::handlers::SocialEventRow::ProposalCommunityVoteUpdate {
                proposal_id,
                votes_for_delta,
                votes_against_delta,
            } => Some(GovernanceRow::ProposalCommunityVoteUpdate {
                proposal_id,
                votes_for_delta,
                votes_against_delta,
            }),
            crate::handlers::SocialEventRow::ProposalOutcomeApplyDelegateSidedUpdates {
                proposal_id,
                approvers_win,
            } => Some(GovernanceRow::ProposalOutcomeApplyDelegateSidedUpdates {
                proposal_id,
                approvers_win,
            }),
            crate::handlers::SocialEventRow::DelegateProposalsSubmittedIncrement {
                address,
                registry_type,
            } => Some(GovernanceRow::DelegateProposalsSubmittedIncrement {
                address,
                registry_type,
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
                if let Some(rows) =
                    governance::handle_governance_event(event_name, &event_data, &event_id)
                {
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
                    let registry_id = reg.registry_id.clone();
                    let exists = governance_registries::table
                        .filter(governance_registries::registry_id.eq(&registry_id))
                        .count()
                        .get_result::<i64>(conn)
                        .await
                        .unwrap_or(0)
                        > 0;
                    if !exists {
                        let delegate_count = reg.delegate_count;
                        let delegate_term_epochs = reg.delegate_term_epochs;
                        let proposal_submission_cost = reg.proposal_submission_cost;
                        let max_votes_per_user = reg.max_votes_per_user;
                        let quadratic_base_cost = reg.quadratic_base_cost;
                        let voting_period_ms = reg.voting_period_ms;
                        let quorum_votes = reg.quorum_votes;
                        let updated_at = reg.updated_at;
                        total += diesel::insert_into(governance_registries::table)
                            .values(reg)
                            .on_conflict(governance_registries::registry_type)
                            .do_update()
                            .set((
                                governance_registries::registry_id.eq(registry_id),
                                governance_registries::delegate_count.eq(delegate_count),
                                governance_registries::delegate_term_epochs
                                    .eq(delegate_term_epochs),
                                governance_registries::proposal_submission_cost
                                    .eq(proposal_submission_cost),
                                governance_registries::max_votes_per_user.eq(max_votes_per_user),
                                governance_registries::quadratic_base_cost.eq(quadratic_base_cost),
                                governance_registries::voting_period_ms.eq(voting_period_ms),
                                governance_registries::quorum_votes.eq(quorum_votes),
                                governance_registries::updated_at.eq(updated_at),
                            ))
                            .execute(conn)
                            .await?;
                    }
                }
                GovernanceRow::GovernanceRegistryUpdate(up) => {
                    total += diesel::update(governance_registries::table)
                        .filter(governance_registries::registry_type.eq(up.registry_type))
                        .set((
                            governance_registries::delegate_count.eq(up.delegate_count),
                            governance_registries::delegate_term_epochs.eq(up.delegate_term_epochs),
                            governance_registries::proposal_submission_cost
                                .eq(up.proposal_submission_cost),
                            governance_registries::max_votes_per_user.eq(up.max_votes_per_user),
                            governance_registries::quadratic_base_cost.eq(up.quadratic_base_cost),
                            governance_registries::voting_period_ms.eq(up.voting_period_ms),
                            governance_registries::quorum_votes.eq(up.quorum_votes),
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
                GovernanceRow::Delegate(d) => {
                    total += diesel::insert_into(delegates::table)
                        .values(d)
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
                        let proposal_type: Option<i16> = proposals::table
                            .filter(proposals::id.eq(proposal_id))
                            .select(proposals::proposal_type)
                            .limit(1)
                            .load::<i16>(conn)
                            .await
                            .ok()
                            .and_then(|v| v.into_iter().next());
                        if let Some(registry_type) = proposal_type {
                            let gov_ev = NewGovernanceEvent {
                                event_type: event_type.clone(),
                                registry_type,
                                event_data: event_data.clone(),
                                event_id: event_id.clone(),
                                created_at: chrono::Utc::now(),
                                anonymous_voting_related: None,
                            };
                            total += diesel::insert_into(governance_events::table)
                                .values(&gov_ev)
                                .execute(conn)
                                .await?;
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
                    let proposal_type: Option<i16> = proposals::table
                        .filter(proposals::id.eq(proposal_id))
                        .select(proposals::proposal_type)
                        .limit(1)
                        .load::<i16>(conn)
                        .await
                        .ok()
                        .and_then(|v| v.into_iter().next());
                    if let Some(registry_type) = proposal_type {
                        let gov_ev = NewGovernanceEvent {
                            event_type: event_type.clone(),
                            registry_type,
                            event_data: event_data.clone(),
                            event_id: event_id.clone(),
                            created_at: chrono::Utc::now(),
                            anonymous_voting_related: *anonymous_voting_related,
                        };
                        total += diesel::insert_into(governance_events::table)
                            .values(&gov_ev)
                            .execute(conn)
                            .await?;
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
                } => {
                    let upd = diesel::sql_query(
                        "UPDATE nominated_delegates SET status = $1 WHERE address = $2 AND registry_type = $3 AND time = (SELECT max(time) FROM nominated_delegates WHERE address = $2 AND registry_type = $3)",
                    )
                    .bind::<Int2, _>(*status)
                    .bind::<Text, _>(address)
                    .bind::<Int2, _>(*registry_type);
                    total += upd.execute(conn).await?;
                }
                GovernanceRow::DelegateVoteCountsUpdate {
                    target_address,
                    registry_type,
                    is_active_delegate,
                    upvotes,
                    downvotes,
                } => {
                    if *is_active_delegate {
                        let upd = diesel::sql_query(
                            "UPDATE delegates SET upvotes = $1, downvotes = $2 WHERE address = $3 AND registry_type = $4 AND time = (SELECT max(time) FROM delegates WHERE address = $3 AND registry_type = $4)",
                        )
                        .bind::<BigInt, _>(*upvotes)
                        .bind::<BigInt, _>(*downvotes)
                        .bind::<Text, _>(target_address)
                        .bind::<Int2, _>(*registry_type);
                        total += upd.execute(conn).await?;
                    } else {
                        let upd = diesel::sql_query(
                            "UPDATE nominated_delegates SET upvotes = $1, downvotes = $2 WHERE address = $3 AND registry_type = $4 AND time = (SELECT max(time) FROM nominated_delegates WHERE address = $3 AND registry_type = $4)",
                        )
                        .bind::<BigInt, _>(*upvotes)
                        .bind::<BigInt, _>(*downvotes)
                        .bind::<Text, _>(target_address)
                        .bind::<Int2, _>(*registry_type);
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
                GovernanceRow::DelegateProposalsReviewedIncrement { address } => {
                    let upd = diesel::sql_query(
                        "UPDATE delegates SET proposals_reviewed = proposals_reviewed + 1 WHERE address = $1 AND is_active = true",
                    )
                    .bind::<Text, _>(address);
                    total += upd.execute(conn).await?;
                }
                GovernanceRow::ProposalCommunityVoteUpdate {
                    proposal_id,
                    votes_for_delta,
                    votes_against_delta,
                } => {
                    let upd = diesel::sql_query(
                        "UPDATE proposals SET community_votes_for = community_votes_for + $1, community_votes_against = community_votes_against + $2 WHERE id = $3 AND time = (SELECT max(time) FROM proposals WHERE id = $3)",
                    )
                    .bind::<BigInt, _>(*votes_for_delta)
                    .bind::<BigInt, _>(*votes_against_delta)
                    .bind::<Text, _>(proposal_id);
                    total += upd.execute(conn).await?;
                }
                GovernanceRow::ProposalOutcomeApplyDelegateSidedUpdates {
                    proposal_id,
                    approvers_win,
                } => {
                    let subq = "SELECT DISTINCT ON (delegate_address) delegate_address, approve FROM delegate_votes WHERE proposal_id = $1 ORDER BY delegate_address, time DESC";
                    let win_sql = format!(
                        "UPDATE delegates d SET sided_winning_proposals = sided_winning_proposals + 1 FROM ({}) dv WHERE d.address = dv.delegate_address AND dv.approve = $2",
                        subq
                    );
                    let lose_sql = format!(
                        "UPDATE delegates d SET sided_losing_proposals = sided_losing_proposals + 1 FROM ({}) dv WHERE d.address = dv.delegate_address AND dv.approve = $2",
                        subq
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
                    address,
                    registry_type,
                } => {
                    let upd = diesel::sql_query(
                        "UPDATE delegates SET proposals_submitted = proposals_submitted + 1 WHERE address = $1 AND registry_type = $2 AND is_active = true",
                    )
                    .bind::<Text, _>(address)
                    .bind::<Int2, _>(*registry_type);
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
