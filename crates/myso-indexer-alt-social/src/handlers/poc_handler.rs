// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Proof of Creativity pipeline: indexes poc and proof_of_creativity module events.

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Bool, Int2, Nullable, Text};
use diesel::ExpressionMethods;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewPocAnalysisResult, NewPocBadge, NewPocConfiguration, NewPocDispute, NewPocDisputeVote,
    NewPocRevenueRedirection,
};
use myso_indexer_alt_social_schema::schema::{
    poc_analysis_results, poc_badges, poc_configuration, poc_dispute_votes, poc_disputes,
    poc_revenue_redirections, posts,
};

use super::common;
use super::events;
use super::poc;

const POC_MODULES: &[&str] = &["poc", "proof_of_creativity"];

#[derive(Debug, Clone)]
pub enum PocRow {
    PocBadge(NewPocBadge),
    PocAnalysisResult(NewPocAnalysisResult),
    PocRevenueRedirection(NewPocRevenueRedirection),
    PocDispute(NewPocDispute),
    PocDisputeVote(NewPocDisputeVote),
    PocConfiguration(NewPocConfiguration),
    PostPocUpdate {
        post_id: String,
        poc_reasoning: Option<String>,
        poc_evidence_urls: Option<serde_json::Value>,
        poc_similarity_score: Option<i64>,
        poc_media_type: Option<i16>,
        poc_oracle_address: Option<String>,
        poc_analyzed_at: Option<i64>,
    },
    PostRevenueRedirectUpdate {
        post_id: String,
        revenue_redirect_to: String,
        revenue_redirect_percentage: i64,
    },
    PocDisputeResolved {
        dispute_id: String,
        post_id: String,
        resolution: i16,
        winning_side: i16,
        total_winning_stake: i64,
        total_losing_stake: i64,
        resolved_at: i64,
        badge_revoked: bool,
        redirection_removed: bool,
    },
    PocVoteRewardClaimed {
        dispute_id: String,
        voter: String,
        reward_amount: i64,
    },
}

impl PocRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        match row {
            crate::handlers::SocialEventRow::PocBadge(badge) => Some(PocRow::PocBadge(badge)),
            crate::handlers::SocialEventRow::PocAnalysisResult(r) => {
                Some(PocRow::PocAnalysisResult(r))
            }
            crate::handlers::SocialEventRow::PocRevenueRedirection(r) => {
                Some(PocRow::PocRevenueRedirection(r))
            }
            crate::handlers::SocialEventRow::PocDispute(d) => Some(PocRow::PocDispute(d)),
            crate::handlers::SocialEventRow::PocDisputeVote(v) => {
                Some(PocRow::PocDisputeVote(v))
            }
            crate::handlers::SocialEventRow::PocConfiguration(c) => {
                Some(PocRow::PocConfiguration(c))
            }
            crate::handlers::SocialEventRow::PostPocUpdate {
                post_id,
                poc_reasoning,
                poc_evidence_urls,
                poc_similarity_score,
                poc_media_type,
                poc_oracle_address,
                poc_analyzed_at,
            } => Some(PocRow::PostPocUpdate {
                post_id,
                poc_reasoning,
                poc_evidence_urls,
                poc_similarity_score,
                poc_media_type,
                poc_oracle_address,
                poc_analyzed_at,
            }),
            crate::handlers::SocialEventRow::PostRevenueRedirectUpdate {
                post_id,
                revenue_redirect_to,
                revenue_redirect_percentage,
            } => Some(PocRow::PostRevenueRedirectUpdate {
                post_id,
                revenue_redirect_to,
                revenue_redirect_percentage,
            }),
            crate::handlers::SocialEventRow::PocDisputeResolved {
                dispute_id,
                post_id,
                resolution,
                winning_side,
                total_winning_stake,
                total_losing_stake,
                resolved_at,
                badge_revoked,
                redirection_removed,
            } => Some(PocRow::PocDisputeResolved {
                dispute_id,
                post_id,
                resolution,
                winning_side,
                total_winning_stake,
                total_losing_stake,
                resolved_at,
                badge_revoked,
                redirection_removed,
            }),
            crate::handlers::SocialEventRow::PocVoteRewardClaimed {
                dispute_id,
                voter,
                reward_amount,
            } => Some(PocRow::PocVoteRewardClaimed {
                dispute_id,
                voter,
                reward_amount,
            }),
            _ => None,
        }
    }
}

impl FieldCount for PocRow {
    const FIELD_COUNT: usize = 35;
}

pub struct PocHandler;

#[async_trait]
impl Processor for PocHandler {
    const NAME: &'static str = "poc";

    type Value = PocRow;

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
                if !POC_MODULES.contains(&module) {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data =
                    match events::parse_event_contents(module, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(_) => continue,
                    };
                if let Some(rows) = poc::handle_poc_event(event_name, &event_data, &event_id) {
                    for row in rows {
                        if let Some(r) = PocRow::from_social(row) {
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
impl Handler for PocHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;
        for row in values {
            match row {
                PocRow::PocBadge(badge) => {
                    total += diesel::insert_into(poc_badges::table)
                        .values(badge)
                        .execute(conn)
                        .await?;
                }
                PocRow::PocAnalysisResult(r) => {
                    total += diesel::insert_into(poc_analysis_results::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                PocRow::PocRevenueRedirection(r) => {
                    total += diesel::insert_into(poc_revenue_redirections::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                PocRow::PocDispute(d) => {
                    total += diesel::insert_into(poc_disputes::table)
                        .values(d)
                        .execute(conn)
                        .await?;
                }
                PocRow::PocDisputeVote(v) => {
                    total += diesel::insert_into(poc_dispute_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                PocRow::PocConfiguration(c) => {
                    total += diesel::insert_into(poc_configuration::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                PocRow::PostPocUpdate {
                    post_id,
                    poc_reasoning,
                    poc_evidence_urls,
                    poc_similarity_score,
                    poc_media_type,
                    poc_oracle_address,
                    poc_analyzed_at,
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set((
                            posts::poc_reasoning.eq(poc_reasoning),
                            posts::poc_evidence_urls.eq(poc_evidence_urls),
                            posts::poc_similarity_score.eq(poc_similarity_score),
                            posts::poc_media_type.eq(poc_media_type),
                            posts::poc_oracle_address.eq(poc_oracle_address),
                            posts::poc_analyzed_at.eq(poc_analyzed_at),
                        ))
                        .execute(conn)
                        .await?;
                }
                PocRow::PostRevenueRedirectUpdate {
                    post_id,
                    revenue_redirect_to,
                    revenue_redirect_percentage,
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set((
                            posts::revenue_redirect_to.eq(Some(revenue_redirect_to)),
                            posts::revenue_redirect_percentage
                                .eq(Some(revenue_redirect_percentage)),
                        ))
                        .execute(conn)
                        .await?;
                }
                PocRow::PocDisputeResolved {
                    dispute_id,
                    post_id,
                    resolution,
                    winning_side,
                    total_winning_stake,
                    total_losing_stake,
                    resolved_at,
                    badge_revoked,
                    redirection_removed,
                } => {
                    let update_sql = "UPDATE poc_disputes SET status = $1, resolution = $2, winning_side = $3, total_winning_stake = $4, total_losing_stake = $5, resolved_at = $6 \
                        WHERE dispute_id = $7 AND time = (SELECT time FROM poc_disputes WHERE dispute_id = $7 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<Int2, _>(resolution)
                        .bind::<Nullable<Int2>, _>(Some(resolution))
                        .bind::<Nullable<Int2>, _>(Some(winning_side))
                        .bind::<Nullable<BigInt>, _>(Some(total_winning_stake))
                        .bind::<Nullable<BigInt>, _>(Some(total_losing_stake))
                        .bind::<Nullable<BigInt>, _>(Some(resolved_at))
                        .bind::<Text, _>(dispute_id)
                        .execute(conn)
                        .await?;

                    if *badge_revoked {
                        let revoke_sql = "UPDATE poc_badges SET revoked = TRUE, revoked_at = $1 \
                            WHERE post_id = $2 AND time = (SELECT time FROM poc_badges WHERE post_id = $2 ORDER BY time DESC LIMIT 1)";
                        total += diesel::sql_query(revoke_sql)
                            .bind::<Nullable<BigInt>, _>(Some(resolved_at))
                            .bind::<Text, _>(post_id)
                            .execute(conn)
                            .await?;
                    }

                    if *redirection_removed {
                        let remove_sql = "UPDATE poc_revenue_redirections SET removed = TRUE, removed_at = $1 \
                            WHERE accused_post_id = $2 AND time = (SELECT time FROM poc_revenue_redirections WHERE accused_post_id = $2 ORDER BY time DESC LIMIT 1)";
                        total += diesel::sql_query(remove_sql)
                            .bind::<Nullable<BigInt>, _>(Some(resolved_at))
                            .bind::<Text, _>(post_id)
                            .execute(conn)
                            .await?;
                    }
                }
                PocRow::PocVoteRewardClaimed {
                    dispute_id,
                    voter,
                    reward_amount,
                } => {
                    let update_sql = "UPDATE poc_dispute_votes SET reward_claimed = $1, reward_amount = $2 \
                        WHERE dispute_id = $3 AND voter = $4 AND time = (SELECT time FROM poc_dispute_votes WHERE dispute_id = $3 AND voter = $4 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<Bool, _>(true)
                        .bind::<Nullable<BigInt>, _>(Some(*reward_amount))
                        .bind::<Text, _>(dispute_id)
                        .bind::<Text, _>(voter)
                        .execute(conn)
                        .await?;
                }
            }
        }
        Ok(total)
    }
}
