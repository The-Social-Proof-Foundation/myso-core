// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Posts pipeline: indexes post, comment, reaction, repost, tip, and
//! `proof_of_creativity` / `poc` module events in **checkpoint transaction order** in a single
//! commit batch. That ordering satisfies DB triggers on `poc_*` tables that require a matching
//! `posts.post_id` row (see `20250620000000_create_poc_tables`).

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::sql_types::{BigInt, Text};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    NewComment, NewDeletionEvent, NewModerationEvent, NewPocAnalysisResult, NewPocBadge,
    NewPocConfiguration, NewPocCreatorIdentityLink, NewPocDispute, NewPocDisputeVote,
    NewPocRevenueRedirection, NewPocUsernameBeneficiary, NewPocUsernameBeneficiaryEvent,
    NewPocVaultClaim, NewPocVaultDeposit, NewPost, NewPostTransfer, NewPromotedPost,
    NewPromotionBudgetEvent, NewPromotionStatusEvent, NewPromotionView, NewReaction,
    NewReactionCount, NewReport, NewRepost, NewSubscriptionAccessLog, NewTip, NewUnifiedRevenue,
    REVENUE_TYPE_PROMOTION_ECOSYSTEM_FEE, REVENUE_TYPE_PROMOTION_PLATFORM_FEE,
    REVENUE_TYPE_PROMOTION_VIEWER_NET,
};
use myso_indexer_alt_social_schema::schema::{
    comments, ecosystem_treasury, poc_analysis_results, poc_badges, poc_config,
    poc_creator_identity_links, poc_dispute_votes, poc_disputes, poc_revenue_redirections,
    poc_username_beneficiary_events, poc_vault_claims, poc_vault_deposits, post_config, posts,
    promoted_posts, promotion_budget_events, promotion_status_events, promotion_views,
    reaction_counts, reactions, reposts, subscription_access_logs, tips,
};
use myso_indexer_alt_social_schema::schema::{
    posts_deletion_events, posts_moderation_events, posts_reports, posts_transfers, profiles,
    unified_revenue,
};
use myso_types::transaction::TransactionDataAPI;

use super::common;
use super::events;
use super::organization_stats::{
    apply_org_outbound_spend, apply_org_revenue, resolve_organization_id_for_derived_address,
    resolve_organization_id_for_post, resolve_organization_id_for_sub_agent,
    stamp_and_count_social_action, OrgStatColumn,
};
use super::poc;
use super::post;
use super::post_mydata;

const POST_MODULES: &[&str] = &["post", "comment", "reaction", "repost", "tip"];
const POC_MODULES: &[&str] = &[
    "poc",
    "proof_of_creativity",
    "poc_vault",
    "poc_username_beneficiary",
];

#[derive(Debug, Clone)]
pub enum PostRow {
    Post(NewPost),
    Comment(NewComment),
    Reaction(NewReaction),
    RemoveReaction {
        object_id: String,
        user_address: String,
        reaction_text: String,
        is_post: bool,
    },
    Repost(NewRepost),
    RepostRemoved {
        repost_id: String,
        original_id: String,
        is_original_post: bool,
    },
    Tip(NewTip),
    ModerationEvent(NewModerationEvent),
    Report(NewReport),
    DeletionEvent(NewDeletionEvent),
    PostCommentCountIncrement {
        post_id: String,
        delta: i64,
    },
    PostCommentCountDecrementByComment {
        comment_id: String,
        owner: String,
    },
    ProfilePostCountIncrement {
        owner_address: String,
    },
    ProfilePostCountDecrement {
        owner_address: String,
    },
    PostRepostCountIncrement {
        original_id: String,
        is_original_post: bool,
    },
    PostTipsReceivedIncrement {
        object_id: String,
        recipient: String,
        amount: i64,
        is_post: bool,
    },
    PostModerationUpdate {
        object_id: String,
        removed: bool,
        moderated_by: String,
    },
    PostDeletedAtUpdate {
        object_id: String,
        owner: String,
        deleted_at: i64,
    },
    CommentDeletedAtUpdate {
        object_id: String,
        owner: String,
        deleted_at: i64,
    },
    PostContentUpdate {
        object_id: String,
        content: String,
        media_urls: Option<serde_json::Value>,
        mentions: Option<serde_json::Value>,
        metadata_json: Option<serde_json::Value>,
        is_post: bool,
        updated_at: i64,
    },
    PostOwnerUpdate {
        object_id: String,
        new_owner: String,
        is_post: bool,
    },
    PostTransfer(NewPostTransfer),
    PostConfig {
        updated_by: String,
        max_content_length: i64,
        max_media_urls: i64,
        max_mentions: i64,
        max_metadata_size: i64,
        max_description_length: i64,
        max_reaction_length: i64,
        commenter_tip_percentage: i64,
        repost_tip_percentage: i64,
        min_promotion_amount: i64,
        max_promotion_amount: i64,
        min_view_duration_ms: i64,
        platform_fee_bps: i64,
        ecosystem_fee_bps: i64,
        version: Option<i64>,
        updated_at: i64,
        transaction_id: String,
    },
    PromotedPost {
        post_id: String,
        owner: String,
        profile_id: String,
        payment_per_view: i64,
        total_budget: i64,
        created_at: i64,
        transaction_id: String,
    },
    PromotionView {
        post_id: String,
        promotion_id: String,
        viewer: String,
        payment_amount: i64,
        platform_fee: i64,
        ecosystem_fee: i64,
        recipient_amount: i64,
        view_duration: i64,
        platform_id: String,
        timestamp: i64,
        transaction_id: String,
    },
    PromotionStatusEvent {
        promotion_id: String,
        toggled_by: String,
        new_status: bool,
        timestamp: i64,
        transaction_id: String,
    },
    PromotionBudgetEvent {
        promotion_id: String,
        owner: String,
        withdrawn_amount: i64,
        timestamp: i64,
        transaction_id: String,
    },
    SubscriptionAccessLog(NewSubscriptionAccessLog),
    UnifiedRevenue(NewUnifiedRevenue),
    PocBadge(NewPocBadge),
    PocAnalysisResult(NewPocAnalysisResult),
    PocRevenueRedirection(NewPocRevenueRedirection),
    PocDispute(NewPocDispute),
    PostPocDisputesSubmitted {
        post_id: String,
        poc_disputes_submitted: i16,
    },
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
    PostPocResultApplied {
        post_id: String,
        poc_outcome: i16,
        poc_redirection_kind: i16,
        similarity_detected: bool,
        timestamp_ms: i64,
    },
    PostPocBadgePointer {
        post_id: String,
        poc_badge_object_id: String,
    },
    PocBeneficiaryVaultDeposit {
        vault_id: String,
        vault_routing_key: String,
        coin_type: String,
        amount: i64,
        source_post_id: Option<String>,
        timestamp_ms: i64,
        transaction_id: String,
    },
    PocBeneficiaryVaultClaimed {
        vault_id: String,
        vault_routing_key: String,
        coin_type: String,
        referrer_address: Option<String>,
        treasury_amount: i64,
        referrer_amount: i64,
        beneficiary_amount: i64,
        join_referral_applied: bool,
        claim_kind: Option<String>,
        timestamp_ms: i64,
        transaction_id: String,
    },
    PocUsernameBeneficiary(NewPocUsernameBeneficiary),
    PocUsernameBeneficiaryClaimed {
        beneficiary_id: String,
        username: String,
        profile_id: String,
        claimed_by: String,
        wallet: String,
        oracle_evidence_hash: String,
        claimed_at_ms: i64,
        transaction_id: String,
    },
    PocUsernameBeneficiaryEnded {
        beneficiary_id: String,
        username: String,
        ended_by: String,
        end_reason_code: i16,
        swept_mys_amount: i64,
        ended_at_ms: i64,
        transaction_id: String,
    },
    PocUsernameBeneficiaryJoinReferralPaid {
        vault_id: String,
        join_referrer: Option<String>,
        join_referral_paid_at_ms: i64,
        transaction_id: String,
    },
    PocCreatorIdentityLink(NewPocCreatorIdentityLink),
    PocUsernameBeneficiaryEvent(NewPocUsernameBeneficiaryEvent),
    PostRevenueRedirectUpdate {
        post_id: String,
        revenue_redirect_to: String,
        revenue_redirect_percentage: i64,
        poc_redirection_kind: i16,
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
        quorum_met: bool,
    },
    PocVoteRewardClaimed {
        dispute_id: String,
        voter: String,
        reward_amount: i64,
    },
}

impl PostRow {
    fn from_social(row: crate::handlers::SocialEventRow) -> Option<Self> {
        use crate::handlers::SocialEventRow;
        match row {
            SocialEventRow::Post(p) => Some(PostRow::Post(p)),
            SocialEventRow::Comment(c) => Some(PostRow::Comment(c)),
            SocialEventRow::Reaction(r) => Some(PostRow::Reaction(r)),
            SocialEventRow::RemoveReaction {
                object_id,
                user_address,
                reaction_text,
                is_post,
            } => Some(PostRow::RemoveReaction {
                object_id,
                user_address,
                reaction_text,
                is_post,
            }),
            SocialEventRow::Repost(r) => Some(PostRow::Repost(r)),
            SocialEventRow::RepostRemoved {
                repost_id,
                original_id,
                is_original_post,
            } => Some(PostRow::RepostRemoved {
                repost_id,
                original_id,
                is_original_post,
            }),
            SocialEventRow::Tip(t) => Some(PostRow::Tip(t)),
            SocialEventRow::ModerationEvent(m) => Some(PostRow::ModerationEvent(m)),
            SocialEventRow::Report(r) => Some(PostRow::Report(r)),
            SocialEventRow::DeletionEvent(d) => Some(PostRow::DeletionEvent(d)),
            SocialEventRow::PostCommentCountIncrement { post_id, delta } => {
                Some(PostRow::PostCommentCountIncrement { post_id, delta })
            }
            SocialEventRow::PostCommentCountDecrementByComment { comment_id, owner } => {
                Some(PostRow::PostCommentCountDecrementByComment { comment_id, owner })
            }
            SocialEventRow::ProfilePostCountIncrement { owner_address } => {
                Some(PostRow::ProfilePostCountIncrement { owner_address })
            }
            SocialEventRow::ProfilePostCountDecrement { owner_address } => {
                Some(PostRow::ProfilePostCountDecrement { owner_address })
            }
            SocialEventRow::PostRepostCountIncrement {
                original_id,
                is_original_post,
            } => Some(PostRow::PostRepostCountIncrement {
                original_id,
                is_original_post,
            }),
            SocialEventRow::PostTipsReceivedIncrement {
                object_id,
                recipient,
                amount,
                is_post,
            } => Some(PostRow::PostTipsReceivedIncrement {
                object_id,
                recipient,
                amount,
                is_post,
            }),
            SocialEventRow::PostModerationUpdate {
                object_id,
                removed,
                moderated_by,
            } => Some(PostRow::PostModerationUpdate {
                object_id,
                removed,
                moderated_by,
            }),
            SocialEventRow::PostDeletedAtUpdate {
                object_id,
                owner,
                deleted_at,
            } => Some(PostRow::PostDeletedAtUpdate {
                object_id,
                owner,
                deleted_at,
            }),
            SocialEventRow::CommentDeletedAtUpdate {
                object_id,
                owner,
                deleted_at,
            } => Some(PostRow::CommentDeletedAtUpdate {
                object_id,
                owner,
                deleted_at,
            }),
            SocialEventRow::PostContentUpdate {
                object_id,
                content,
                media_urls,
                mentions,
                metadata_json,
                is_post,
                updated_at,
            } => Some(PostRow::PostContentUpdate {
                object_id,
                content,
                media_urls,
                mentions,
                metadata_json,
                is_post,
                updated_at,
            }),
            SocialEventRow::PostOwnerUpdate {
                object_id,
                new_owner,
                is_post,
            } => Some(PostRow::PostOwnerUpdate {
                object_id,
                new_owner,
                is_post,
            }),
            SocialEventRow::SubscriptionAccessLog(log) => Some(PostRow::SubscriptionAccessLog(log)),
            SocialEventRow::PostTransfer(t) => Some(PostRow::PostTransfer(t)),
            SocialEventRow::PostConfig {
                updated_by,
                max_content_length,
                max_media_urls,
                max_mentions,
                max_metadata_size,
                max_description_length,
                max_reaction_length,
                commenter_tip_percentage,
                repost_tip_percentage,
                min_promotion_amount,
                max_promotion_amount,
                min_view_duration_ms,
                platform_fee_bps,
                ecosystem_fee_bps,
                version,
                updated_at,
                transaction_id,
            } => Some(PostRow::PostConfig {
                updated_by,
                max_content_length,
                max_media_urls,
                max_mentions,
                max_metadata_size,
                max_description_length,
                max_reaction_length,
                commenter_tip_percentage,
                repost_tip_percentage,
                min_promotion_amount,
                max_promotion_amount,
                min_view_duration_ms,
                platform_fee_bps,
                ecosystem_fee_bps,
                version,
                updated_at,
                transaction_id,
            }),
            SocialEventRow::PromotedPost {
                post_id,
                owner,
                profile_id,
                payment_per_view,
                total_budget,
                created_at,
                transaction_id,
            } => Some(PostRow::PromotedPost {
                post_id,
                owner,
                profile_id,
                payment_per_view,
                total_budget,
                created_at,
                transaction_id,
            }),
            SocialEventRow::PromotionView {
                post_id,
                promotion_id,
                viewer,
                payment_amount,
                platform_fee,
                ecosystem_fee,
                recipient_amount,
                view_duration,
                platform_id,
                timestamp,
                transaction_id,
            } => Some(PostRow::PromotionView {
                post_id,
                promotion_id,
                viewer,
                payment_amount,
                platform_fee,
                ecosystem_fee,
                recipient_amount,
                view_duration,
                platform_id,
                timestamp,
                transaction_id,
            }),
            SocialEventRow::PromotionStatusEvent {
                promotion_id,
                toggled_by,
                new_status,
                timestamp,
                transaction_id,
            } => Some(PostRow::PromotionStatusEvent {
                promotion_id,
                toggled_by,
                new_status,
                timestamp,
                transaction_id,
            }),
            SocialEventRow::PromotionBudgetEvent {
                promotion_id,
                owner,
                withdrawn_amount,
                timestamp,
                transaction_id,
            } => Some(PostRow::PromotionBudgetEvent {
                promotion_id,
                owner,
                withdrawn_amount,
                timestamp,
                transaction_id,
            }),
            SocialEventRow::UnifiedRevenue(r) => Some(PostRow::UnifiedRevenue(r)),
            SocialEventRow::PocBadge(p) => Some(PostRow::PocBadge(p)),
            SocialEventRow::PocAnalysisResult(r) => Some(PostRow::PocAnalysisResult(r)),
            SocialEventRow::PocRevenueRedirection(r) => Some(PostRow::PocRevenueRedirection(r)),
            SocialEventRow::PocDispute(d) => Some(PostRow::PocDispute(d)),
            SocialEventRow::PostPocDisputesSubmitted {
                post_id,
                poc_disputes_submitted,
            } => Some(PostRow::PostPocDisputesSubmitted {
                post_id,
                poc_disputes_submitted,
            }),
            SocialEventRow::PocDisputeVote(v) => Some(PostRow::PocDisputeVote(v)),
            SocialEventRow::PocConfiguration(c) => Some(PostRow::PocConfiguration(c)),
            SocialEventRow::PostPocUpdate {
                post_id,
                poc_reasoning,
                poc_evidence_urls,
                poc_similarity_score,
                poc_media_type,
                poc_oracle_address,
                poc_analyzed_at,
            } => Some(PostRow::PostPocUpdate {
                post_id,
                poc_reasoning,
                poc_evidence_urls,
                poc_similarity_score,
                poc_media_type,
                poc_oracle_address,
                poc_analyzed_at,
            }),
            SocialEventRow::PostPocResultApplied {
                post_id,
                poc_outcome,
                poc_redirection_kind,
                similarity_detected,
                timestamp_ms,
            } => Some(PostRow::PostPocResultApplied {
                post_id,
                poc_outcome,
                poc_redirection_kind,
                similarity_detected,
                timestamp_ms,
            }),
            SocialEventRow::PostPocBadgePointer {
                post_id,
                poc_badge_object_id,
            } => Some(PostRow::PostPocBadgePointer {
                post_id,
                poc_badge_object_id,
            }),
            SocialEventRow::PocBeneficiaryVaultDeposit {
                vault_id,
                vault_routing_key,
                coin_type,
                amount,
                source_post_id,
                timestamp_ms,
                transaction_id,
            } => Some(PostRow::PocBeneficiaryVaultDeposit {
                vault_id,
                vault_routing_key,
                coin_type,
                amount,
                source_post_id,
                timestamp_ms,
                transaction_id,
            }),
            SocialEventRow::PocBeneficiaryVaultClaimed {
                vault_id,
                vault_routing_key,
                coin_type,
                referrer_address,
                treasury_amount,
                referrer_amount,
                beneficiary_amount,
                join_referral_applied,
                claim_kind,
                timestamp_ms,
                transaction_id,
            } => Some(PostRow::PocBeneficiaryVaultClaimed {
                vault_id,
                vault_routing_key,
                coin_type,
                referrer_address,
                treasury_amount,
                referrer_amount,
                beneficiary_amount,
                join_referral_applied,
                claim_kind,
                timestamp_ms,
                transaction_id,
            }),
            SocialEventRow::PocUsernameBeneficiary(row) => {
                Some(PostRow::PocUsernameBeneficiary(row))
            }
            SocialEventRow::PocUsernameBeneficiaryClaimed {
                beneficiary_id,
                username,
                profile_id,
                claimed_by,
                wallet,
                oracle_evidence_hash,
                claimed_at_ms,
                transaction_id,
            } => Some(PostRow::PocUsernameBeneficiaryClaimed {
                beneficiary_id,
                username,
                profile_id,
                claimed_by,
                wallet,
                oracle_evidence_hash,
                claimed_at_ms,
                transaction_id,
            }),
            SocialEventRow::PocUsernameBeneficiaryEnded {
                beneficiary_id,
                username,
                ended_by,
                end_reason_code,
                swept_mys_amount,
                ended_at_ms,
                transaction_id,
            } => Some(PostRow::PocUsernameBeneficiaryEnded {
                beneficiary_id,
                username,
                ended_by,
                end_reason_code,
                swept_mys_amount,
                ended_at_ms,
                transaction_id,
            }),
            SocialEventRow::PocUsernameBeneficiaryJoinReferralPaid {
                vault_id,
                join_referrer,
                join_referral_paid_at_ms,
                transaction_id,
            } => Some(PostRow::PocUsernameBeneficiaryJoinReferralPaid {
                vault_id,
                join_referrer,
                join_referral_paid_at_ms,
                transaction_id,
            }),
            SocialEventRow::PocCreatorIdentityLink(row) => {
                Some(PostRow::PocCreatorIdentityLink(row))
            }
            SocialEventRow::PocUsernameBeneficiaryEvent(row) => {
                Some(PostRow::PocUsernameBeneficiaryEvent(row))
            }
            SocialEventRow::PostRevenueRedirectUpdate {
                post_id,
                revenue_redirect_to,
                revenue_redirect_percentage,
                poc_redirection_kind,
            } => Some(PostRow::PostRevenueRedirectUpdate {
                post_id,
                revenue_redirect_to,
                revenue_redirect_percentage,
                poc_redirection_kind,
            }),
            SocialEventRow::PocDisputeResolved {
                dispute_id,
                post_id,
                resolution,
                winning_side,
                total_winning_stake,
                total_losing_stake,
                resolved_at,
                badge_revoked,
                redirection_removed,
                quorum_met,
            } => Some(PostRow::PocDisputeResolved {
                dispute_id,
                post_id,
                resolution,
                winning_side,
                total_winning_stake,
                total_losing_stake,
                resolved_at,
                badge_revoked,
                redirection_removed,
                quorum_met,
            }),
            SocialEventRow::PocVoteRewardClaimed {
                dispute_id,
                voter,
                reward_amount,
            } => Some(PostRow::PocVoteRewardClaimed {
                dispute_id,
                voter,
                reward_amount,
            }),
            _ => None,
        }
    }
}

impl FieldCount for PostRow {
    const FIELD_COUNT: usize = 91;
}

pub struct PostsHandler;

#[async_trait]
impl Processor for PostsHandler {
    const NAME: &'static str = "posts";

    type Value = PostRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let checkpoint_timestamp_ms = checkpoint.summary.timestamp_ms;
        let mydata_snapshots = post_mydata::build_checkpoint_mydata_snapshots(checkpoint);
        let mut values = Vec::new();
        for tx in &checkpoint.transactions {
            let tx_digest = tx.transaction.digest().to_string();
            let tx_sender = tx.transaction.sender().to_string();
            let Some(events) = &tx.events else {
                continue;
            };
            for (event_seq, ev) in events.data.iter().enumerate() {
                if !common::is_social_package_event(&ev.package_id, &ev.type_.address) {
                    continue;
                }
                let module = ev.type_.module.as_str();
                let is_post_module = POST_MODULES.contains(&module);
                let is_poc_module = POC_MODULES.contains(&module);
                if !is_post_module && !is_poc_module {
                    continue;
                }
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);
                let event_data =
                    match events::parse_event_contents(module, event_name, &ev.contents) {
                        Ok(v) => v,
                        Err(e) => {
                            crate::metrics::SocialMetrics::record_event_bcs_parse_failed(
                                module, event_name,
                            );
                            tracing::warn!(
                                tx_digest = %tx_digest,
                                module,
                                event_name = event_name,
                                error = %e,
                                hex_preview = %e.contents_hex_preview(32),
                                "post or PoC module event BCS parse failed"
                            );
                            continue;
                        }
                    };
                if is_post_module {
                    if let Some(rows) = post::handle_post_event(
                        event_name,
                        &event_data,
                        &event_id,
                        &mydata_snapshots,
                        checkpoint_timestamp_ms,
                    ) {
                        for row in rows {
                            if let Some(r) = PostRow::from_social(row) {
                                values.push(r);
                            }
                        }
                    }
                } else if let Some(rows) =
                    poc::handle_poc_event(event_name, &event_data, &event_id, Some(&tx_sender))
                {
                    for row in rows {
                        if let Some(r) = PostRow::from_social(row) {
                            values.push(r);
                        }
                    }
                } else {
                    tracing::warn!(
                        tx_digest = %tx_digest,
                        module,
                        event_name = event_name,
                        "poc event handler returned no rows (validation or unknown event?)"
                    );
                }
            }
        }
        Ok(values)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ReactionApplyKind {
    New,
    Swap { previous: String },
    Replay,
}

fn classify_reaction(prior: Option<&str>, new: &str) -> ReactionApplyKind {
    match prior {
        None => ReactionApplyKind::New,
        Some(prev) if prev == new => ReactionApplyKind::Replay,
        Some(prev) => ReactionApplyKind::Swap {
            previous: prev.to_string(),
        },
    }
}

async fn resolve_content_owner(
    conn: &mut Connection<'_>,
    object_id: &str,
    is_post: bool,
) -> Result<Option<String>> {
    if is_post {
        Ok(posts::table
            .filter(posts::post_id.eq(object_id))
            .select(posts::owner)
            .first::<String>(conn)
            .await
            .optional()?)
    } else {
        Ok(comments::table
            .filter(comments::comment_id.eq(object_id))
            .select(comments::owner)
            .first::<String>(conn)
            .await
            .optional()?)
    }
}

async fn resolve_organization_id_for_comment(
    conn: &mut Connection<'_>,
    comment_id: &str,
) -> Result<Option<String>> {
    let comment_org = comments::table
        .filter(comments::comment_id.eq(comment_id))
        .select(comments::organization_id)
        .first::<Option<String>>(conn)
        .await
        .optional()?;
    if let Some(Some(org_id)) = comment_org {
        return Ok(Some(org_id));
    }
    let post_id = comments::table
        .filter(comments::comment_id.eq(comment_id))
        .select(comments::post_id)
        .first::<String>(conn)
        .await
        .optional()?;
    match post_id {
        Some(ref pid) => resolve_organization_id_for_post(conn, pid).await,
        None => Ok(None),
    }
}

async fn resolve_tip_recipient_org(
    conn: &mut Connection<'_>,
    tip: &NewTip,
) -> Result<Option<String>> {
    let content_owner = resolve_content_owner(conn, &tip.object_id, tip.is_post).await?;
    let owner_is_recipient = content_owner.as_deref() == Some(tip.recipient.as_str());
    if owner_is_recipient {
        if tip.is_post {
            resolve_organization_id_for_post(conn, &tip.object_id).await
        } else {
            resolve_organization_id_for_comment(conn, &tip.object_id).await
        }
    } else {
        resolve_organization_id_for_derived_address(conn, &tip.recipient).await
    }
}

async fn resolve_tip_organization_id(
    conn: &mut Connection<'_>,
    tip: &NewTip,
) -> Result<Option<String>> {
    if tip.is_post {
        return resolve_organization_id_for_post(conn, &tip.object_id).await;
    }
    let content_owner = resolve_content_owner(conn, &tip.object_id, false).await?;
    if content_owner.as_deref() != Some(tip.recipient.as_str()) {
        return resolve_organization_id_for_derived_address(conn, &tip.recipient).await;
    }
    resolve_organization_id_for_comment(conn, &tip.object_id).await
}

async fn apply_tips_received_increment(
    conn: &mut Connection<'_>,
    object_id: &str,
    recipient: &str,
    amount: i64,
    is_post: bool,
) -> Result<()> {
    use diesel::sql_query;
    let Some(owner) = resolve_content_owner(conn, object_id, is_post).await? else {
        return Ok(());
    };
    if !owner.eq_ignore_ascii_case(recipient) {
        return Ok(());
    }
    if is_post {
        let _ = sql_query("UPDATE posts SET tips_received = tips_received + $1 WHERE post_id = $2")
            .bind::<BigInt, _>(amount)
            .bind::<Text, _>(object_id)
            .execute(conn)
            .await;
    } else {
        let _ = sql_query(
            "UPDATE comments SET tips_received = tips_received + $1 WHERE comment_id = $2",
        )
        .bind::<BigInt, _>(amount)
        .bind::<Text, _>(object_id)
        .execute(conn)
        .await;
    }
    Ok(())
}

async fn apply_total_tip_volume_increment(
    conn: &mut Connection<'_>,
    post_id: &str,
    amount: i64,
) -> Result<()> {
    use diesel::sql_query;
    let _ =
        sql_query("UPDATE posts SET total_tip_volume = total_tip_volume + $1 WHERE post_id = $2")
            .bind::<BigInt, _>(amount)
            .bind::<Text, _>(post_id)
            .execute(conn)
            .await;
    Ok(())
}

async fn resolve_attribution_organization_id(
    conn: &mut Connection<'_>,
    organization_id: &mut Option<String>,
    sub_agent_id: &Option<String>,
) -> Result<()> {
    if organization_id.is_none() {
        if let Some(sub_agent_id) = sub_agent_id {
            *organization_id = resolve_organization_id_for_sub_agent(conn, sub_agent_id).await?;
        }
    }
    Ok(())
}

async fn load_latest_poc_thresholds(conn: &mut Connection<'_>) -> poc::PocThresholds {
    use diesel::sql_types::BigInt;
    use diesel::QueryableByName;

    #[derive(QueryableByName)]
    struct ThresholdRow {
        #[diesel(sql_type = BigInt)]
        image_threshold: i64,
        #[diesel(sql_type = BigInt)]
        video_threshold: i64,
        #[diesel(sql_type = BigInt)]
        audio_threshold: i64,
    }

    diesel::sql_query(
        "SELECT image_threshold, video_threshold, audio_threshold \
         FROM poc_config ORDER BY time DESC LIMIT 1",
    )
    .get_result::<ThresholdRow>(conn)
    .await
    .map(|row| poc::PocThresholds {
        image_threshold: row.image_threshold,
        video_threshold: row.video_threshold,
        audio_threshold: row.audio_threshold,
    })
    .unwrap_or_default()
}

#[async_trait]
impl Handler for PostsHandler {
    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        use diesel::sql_query;
        use diesel::sql_types::{Bool, Int2, Nullable};

        let poc_thresholds = load_latest_poc_thresholds(conn).await;
        let mut total = 0;
        for row in values {
            match row {
                PostRow::Post(p) => {
                    let mut post = p.clone();
                    resolve_attribution_organization_id(
                        conn,
                        &mut post.organization_id,
                        &post.sub_agent_id,
                    )
                    .await?;
                    post_mydata::enrich_post_paywall_from_db(&mut post, conn).await?;
                    post_mydata::enrich_post_subscription_price_from_db(&mut post, conn).await?;
                    total += diesel::insert_into(posts::table)
                        .values(&post)
                        .on_conflict((posts::post_id, posts::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    stamp_and_count_social_action(
                        conn,
                        post.organization_id.as_deref(),
                        OrgStatColumn::TotalPosts,
                        post.created_at,
                        None,
                    )
                    .await?;
                }
                PostRow::Comment(c) => {
                    let mut comment = c.clone();
                    resolve_attribution_organization_id(
                        conn,
                        &mut comment.organization_id,
                        &comment.sub_agent_id,
                    )
                    .await?;
                    total += diesel::insert_into(comments::table)
                        .values(&comment)
                        .on_conflict((comments::id, comments::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    stamp_and_count_social_action(
                        conn,
                        comment.organization_id.as_deref(),
                        OrgStatColumn::TotalComments,
                        comment.created_at,
                        None,
                    )
                    .await?;
                }
                PostRow::Reaction(r) => {
                    let mut reaction = r.clone();
                    resolve_attribution_organization_id(
                        conn,
                        &mut reaction.organization_id,
                        &reaction.sub_agent_id,
                    )
                    .await?;
                    let is_post = reaction.is_post;
                    let object_id = reaction.object_id.clone();
                    let user_address = reaction.user_address.clone();
                    let new_reaction_text = reaction.reaction_text.clone();
                    let created_at = reaction.created_at;

                    let prior: Option<String> = reactions::table
                        .filter(reactions::object_id.eq(&object_id))
                        .filter(reactions::user_address.eq(&user_address))
                        .order(reactions::created_at.desc())
                        .select(reactions::reaction_text)
                        .first::<String>(conn)
                        .await
                        .ok();

                    match classify_reaction(prior.as_deref(), &new_reaction_text) {
                        ReactionApplyKind::Replay => {
                            let _ = diesel::insert_into(reactions::table)
                                .values(&reaction)
                                .on_conflict_do_nothing()
                                .execute(conn)
                                .await;
                        }
                        ReactionApplyKind::Swap { previous } => {
                            let _ = diesel::delete(reactions::table)
                                .filter(reactions::object_id.eq(&object_id))
                                .filter(reactions::user_address.eq(&user_address))
                                .execute(conn)
                                .await;
                            let _ = diesel::update(reaction_counts::table)
                                .filter(reaction_counts::object_id.eq(&object_id))
                                .filter(reaction_counts::reaction_text.eq(&previous))
                                .set(reaction_counts::count.eq(reaction_counts::count - 1))
                                .execute(conn)
                                .await;
                            total += diesel::insert_into(reactions::table)
                                .values(&reaction)
                                .execute(conn)
                                .await?;
                            total += diesel::insert_into(reaction_counts::table)
                                .values(NewReactionCount {
                                    object_id: object_id.clone(),
                                    reaction_text: new_reaction_text,
                                    count: 1,
                                })
                                .on_conflict((
                                    reaction_counts::object_id,
                                    reaction_counts::reaction_text,
                                ))
                                .do_update()
                                .set(reaction_counts::count.eq(reaction_counts::count + 1))
                                .execute(conn)
                                .await?;
                        }
                        ReactionApplyKind::New => {
                            total += diesel::insert_into(reactions::table)
                                .values(&reaction)
                                .execute(conn)
                                .await?;
                            total += diesel::insert_into(reaction_counts::table)
                                .values(NewReactionCount {
                                    object_id: object_id.clone(),
                                    reaction_text: new_reaction_text,
                                    count: 1,
                                })
                                .on_conflict((
                                    reaction_counts::object_id,
                                    reaction_counts::reaction_text,
                                ))
                                .do_update()
                                .set(reaction_counts::count.eq(reaction_counts::count + 1))
                                .execute(conn)
                                .await?;
                            stamp_and_count_social_action(
                                conn,
                                reaction.organization_id.as_deref(),
                                OrgStatColumn::TotalReactions,
                                created_at,
                                None,
                            )
                            .await?;
                            if is_post {
                                let _ = diesel::update(posts::table)
                                    .filter(posts::post_id.eq(&object_id))
                                    .set(posts::reaction_count.eq(posts::reaction_count + 1))
                                    .execute(conn)
                                    .await;
                            } else {
                                let _ = diesel::update(comments::table)
                                    .filter(comments::comment_id.eq(&object_id))
                                    .set(comments::reaction_count.eq(comments::reaction_count + 1))
                                    .execute(conn)
                                    .await;
                            }
                        }
                    }
                }
                PostRow::RemoveReaction {
                    object_id,
                    user_address,
                    reaction_text,
                    is_post,
                } => {
                    let _ = diesel::delete(reactions::table)
                        .filter(reactions::object_id.eq(&object_id))
                        .filter(reactions::user_address.eq(&user_address))
                        .filter(reactions::reaction_text.eq(&reaction_text))
                        .execute(conn)
                        .await;
                    let _ = diesel::update(reaction_counts::table)
                        .filter(reaction_counts::object_id.eq(&object_id))
                        .filter(reaction_counts::reaction_text.eq(&reaction_text))
                        .set(reaction_counts::count.eq(reaction_counts::count - 1))
                        .execute(conn)
                        .await;
                    if *is_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(&object_id))
                            .set(posts::reaction_count.eq(posts::reaction_count - 1))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(&object_id))
                            .set(comments::reaction_count.eq(comments::reaction_count - 1))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::RepostRemoved {
                    repost_id,
                    original_id,
                    is_original_post,
                } => {
                    let _ = diesel::delete(reposts::table)
                        .filter(reposts::repost_id.eq(repost_id))
                        .execute(conn)
                        .await;
                    if *is_original_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(original_id))
                            .set(posts::repost_count.eq(posts::repost_count - 1))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(original_id))
                            .set(comments::repost_count.eq(comments::repost_count - 1))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::Repost(r) => {
                    let mut repost = r.clone();
                    resolve_attribution_organization_id(
                        conn,
                        &mut repost.organization_id,
                        &repost.sub_agent_id,
                    )
                    .await?;
                    total += diesel::insert_into(reposts::table)
                        .values(&repost)
                        .on_conflict((reposts::repost_id, reposts::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                    stamp_and_count_social_action(
                        conn,
                        repost.organization_id.as_deref(),
                        OrgStatColumn::TotalReposts,
                        repost.created_at,
                        None,
                    )
                    .await?;
                }
                PostRow::Tip(t) => {
                    let mut t = t.clone();
                    t.organization_id = resolve_tip_organization_id(conn, &t).await?;
                    total += diesel::insert_into(tips::table)
                        .values(&t)
                        .execute(conn)
                        .await?;
                    let tipper_org =
                        resolve_organization_id_for_derived_address(conn, &t.tipper).await?;
                    let recipient_org = resolve_tip_recipient_org(conn, &t).await?;
                    apply_org_outbound_spend(
                        conn,
                        tipper_org.as_deref(),
                        t.amount,
                        Some(&t.recipient),
                        t.created_at,
                    )
                    .await?;
                    apply_org_revenue(
                        conn,
                        recipient_org.as_deref(),
                        t.amount,
                        Some(&t.tipper),
                        t.created_at,
                    )
                    .await?;
                    if t.is_post {
                        apply_total_tip_volume_increment(conn, &t.object_id, t.amount).await?;
                    }
                }
                PostRow::ModerationEvent(m) => {
                    total += diesel::insert_into(posts_moderation_events::table)
                        .values(m)
                        .execute(conn)
                        .await?;
                }
                PostRow::Report(r) => {
                    total += diesel::insert_into(posts_reports::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                PostRow::DeletionEvent(d) => {
                    total += diesel::insert_into(posts_deletion_events::table)
                        .values(d)
                        .execute(conn)
                        .await?;
                }
                PostRow::PostCommentCountIncrement { post_id, delta } => {
                    let _ = diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set(posts::comment_count.eq(posts::comment_count + delta))
                        .execute(conn)
                        .await;
                }
                PostRow::PostCommentCountDecrementByComment { comment_id, owner } => {
                    let _ = sql_query(
                        "UPDATE posts SET comment_count = comment_count - 1 WHERE post_id = (SELECT post_id FROM comments WHERE comment_id = $1 AND owner = $2 LIMIT 1)",
                    )
                    .bind::<Text, _>(comment_id)
                    .bind::<Text, _>(owner)
                    .execute(conn)
                    .await;
                }
                PostRow::ProfilePostCountIncrement { owner_address } => {
                    let _ = diesel::update(profiles::table)
                        .filter(profiles::owner_address.eq(owner_address))
                        .set(profiles::post_count.eq(profiles::post_count + 1))
                        .execute(conn)
                        .await;
                }
                PostRow::ProfilePostCountDecrement { owner_address } => {
                    let _ = diesel::update(profiles::table)
                        .filter(profiles::owner_address.eq(owner_address))
                        .set(profiles::post_count.eq(profiles::post_count - 1))
                        .execute(conn)
                        .await;
                }
                PostRow::PostRepostCountIncrement {
                    original_id,
                    is_original_post,
                } => {
                    if *is_original_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(original_id))
                            .set(posts::repost_count.eq(posts::repost_count + 1))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(original_id))
                            .set(comments::repost_count.eq(comments::repost_count + 1))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::PostTipsReceivedIncrement {
                    object_id,
                    recipient,
                    amount,
                    is_post,
                } => {
                    apply_tips_received_increment(conn, object_id, recipient, *amount, *is_post)
                        .await?;
                }
                PostRow::PostModerationUpdate {
                    object_id,
                    removed,
                    moderated_by,
                } => {
                    let post_updated = diesel::update(posts::table)
                        .filter(posts::post_id.eq(object_id))
                        .set((
                            posts::removed_from_platform.eq(*removed),
                            posts::removed_by.eq(Some(moderated_by.clone())),
                        ))
                        .execute(conn)
                        .await
                        .unwrap_or(0);
                    if post_updated == 0 {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set((
                                comments::removed_from_platform.eq(*removed),
                                comments::removed_by.eq(Some(moderated_by.clone())),
                            ))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::SubscriptionAccessLog(log) => {
                    total += diesel::insert_into(subscription_access_logs::table)
                        .values(log)
                        .on_conflict_do_nothing()
                        .execute(conn)
                        .await?;
                }
                PostRow::PostDeletedAtUpdate {
                    object_id,
                    owner,
                    deleted_at,
                } => {
                    let _ = diesel::update(posts::table)
                        .filter(posts::post_id.eq(object_id))
                        .filter(posts::owner.eq(owner))
                        .set(posts::deleted_at.eq(Some(*deleted_at)))
                        .execute(conn)
                        .await;
                }
                PostRow::CommentDeletedAtUpdate {
                    object_id,
                    owner,
                    deleted_at,
                } => {
                    let _ = diesel::update(comments::table)
                        .filter(comments::comment_id.eq(object_id))
                        .filter(comments::owner.eq(owner))
                        .set(comments::deleted_at.eq(Some(*deleted_at)))
                        .execute(conn)
                        .await;
                }
                PostRow::PostContentUpdate {
                    object_id,
                    content,
                    media_urls,
                    mentions,
                    metadata_json,
                    is_post,
                    updated_at,
                } => {
                    if *is_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(object_id))
                            .set((
                                posts::content.eq(content),
                                posts::media_urls.eq(media_urls),
                                posts::mentions.eq(mentions),
                                posts::metadata_json.eq(metadata_json),
                                posts::updated_at.eq(Some(*updated_at)),
                            ))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set((
                                comments::content.eq(content),
                                comments::media_urls.eq(media_urls),
                                comments::mentions.eq(mentions),
                                comments::metadata_json.eq(metadata_json),
                                comments::updated_at.eq(Some(*updated_at)),
                            ))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::PostOwnerUpdate {
                    object_id,
                    new_owner,
                    is_post,
                } => {
                    if *is_post {
                        let _ = diesel::update(posts::table)
                            .filter(posts::post_id.eq(object_id))
                            .set(posts::owner.eq(new_owner))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = diesel::update(comments::table)
                            .filter(comments::comment_id.eq(object_id))
                            .set(comments::owner.eq(new_owner))
                            .execute(conn)
                            .await;
                    }
                }
                PostRow::PostTransfer(transfer) => {
                    total += diesel::insert_into(posts_transfers::table)
                        .values(transfer)
                        .execute(conn)
                        .await?;
                }
                PostRow::PostConfig {
                    updated_by,
                    max_content_length,
                    max_media_urls,
                    max_mentions,
                    max_metadata_size,
                    max_description_length,
                    max_reaction_length,
                    commenter_tip_percentage,
                    repost_tip_percentage,
                    min_promotion_amount,
                    max_promotion_amount,
                    min_view_duration_ms,
                    platform_fee_bps,
                    ecosystem_fee_bps,
                    version,
                    updated_at,
                    transaction_id,
                } => {
                    let version_val = version.unwrap_or(-1);
                    if version_val >= 0 {
                        let _ = diesel::insert_into(post_config::table)
                            .values((
                                post_config::updated_by.eq(updated_by),
                                post_config::max_content_length.eq(max_content_length),
                                post_config::max_media_urls.eq(max_media_urls),
                                post_config::max_mentions.eq(max_mentions),
                                post_config::max_metadata_size.eq(max_metadata_size),
                                post_config::max_description_length.eq(max_description_length),
                                post_config::max_reaction_length.eq(max_reaction_length),
                                post_config::commenter_tip_percentage.eq(commenter_tip_percentage),
                                post_config::repost_tip_percentage.eq(repost_tip_percentage),
                                post_config::min_promotion_amount.eq(min_promotion_amount),
                                post_config::max_promotion_amount.eq(max_promotion_amount),
                                post_config::min_view_duration_ms.eq(min_view_duration_ms),
                                post_config::platform_fee_bps.eq(platform_fee_bps),
                                post_config::ecosystem_fee_bps.eq(ecosystem_fee_bps),
                                post_config::version.eq(version_val),
                                post_config::updated_at.eq(updated_at),
                                post_config::transaction_id.eq(transaction_id),
                            ))
                            .execute(conn)
                            .await;
                    } else {
                        let _ = sql_query(
                            r#"INSERT INTO post_config (updated_by, max_content_length, max_media_urls, max_mentions, max_metadata_size, max_description_length, max_reaction_length, commenter_tip_percentage, repost_tip_percentage, min_promotion_amount, max_promotion_amount, min_view_duration_ms, platform_fee_bps, ecosystem_fee_bps, version, updated_at, transaction_id)
                               SELECT $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, COALESCE((SELECT MAX(version) FROM post_config), 0) + 1, $15, $16"#,
                        )
                        .bind::<Text, _>(updated_by)
                        .bind::<BigInt, _>(max_content_length)
                        .bind::<BigInt, _>(max_media_urls)
                        .bind::<BigInt, _>(max_mentions)
                        .bind::<BigInt, _>(max_metadata_size)
                        .bind::<BigInt, _>(max_description_length)
                        .bind::<BigInt, _>(max_reaction_length)
                        .bind::<BigInt, _>(commenter_tip_percentage)
                        .bind::<BigInt, _>(repost_tip_percentage)
                        .bind::<BigInt, _>(min_promotion_amount)
                        .bind::<BigInt, _>(max_promotion_amount)
                        .bind::<BigInt, _>(min_view_duration_ms)
                        .bind::<BigInt, _>(platform_fee_bps)
                        .bind::<BigInt, _>(ecosystem_fee_bps)
                        .bind::<BigInt, _>(updated_at)
                        .bind::<Text, _>(transaction_id)
                        .execute(conn)
                        .await;
                    }
                }
                PostRow::PromotedPost {
                    post_id,
                    owner,
                    profile_id,
                    payment_per_view,
                    total_budget,
                    created_at,
                    transaction_id,
                } => {
                    let promotion_id_opt: Option<String> = posts::table
                        .filter(posts::post_id.eq(&post_id))
                        .order(posts::time.desc())
                        .select(posts::promotion_id)
                        .first::<Option<String>>(conn)
                        .await
                        .ok()
                        .flatten();
                    if let Some(promotion_id) = promotion_id_opt {
                        let time = chrono::DateTime::from_timestamp(created_at / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotedPost {
                            promotion_id,
                            post_id: post_id.clone(),
                            owner: owner.clone(),
                            profile_id: profile_id.clone(),
                            payment_per_view: *payment_per_view,
                            total_budget: *total_budget,
                            remaining_budget: *total_budget,
                            active: false,
                            created_at: *created_at,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promoted_posts::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                    }
                }
                PostRow::PromotionView {
                    post_id,
                    promotion_id,
                    viewer,
                    payment_amount,
                    platform_fee,
                    ecosystem_fee,
                    recipient_amount,
                    view_duration,
                    platform_id,
                    timestamp,
                    transaction_id,
                } => {
                    let looked_up_post_id: Option<String> = if post_id.is_empty() {
                        promoted_posts::table
                            .filter(promoted_posts::promotion_id.eq(promotion_id))
                            .order(promoted_posts::time.desc())
                            .select(promoted_posts::post_id)
                            .first::<String>(conn)
                            .await
                            .ok()
                    } else {
                        None
                    };
                    let resolved_post_id = if !post_id.is_empty() {
                        Some(post_id.clone())
                    } else {
                        looked_up_post_id
                    };
                    if let Some(resolved_post_id) = resolved_post_id {
                        let time = chrono::DateTime::from_timestamp(*timestamp / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotionView {
                            post_id: resolved_post_id.clone(),
                            promotion_id: promotion_id.clone(),
                            viewer: viewer.clone(),
                            payment_amount: *payment_amount,
                            platform_fee: *platform_fee,
                            ecosystem_fee: *ecosystem_fee,
                            recipient_amount: *recipient_amount,
                            view_duration: *view_duration,
                            platform_id: platform_id.clone(),
                            timestamp: *timestamp,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promotion_views::table)
                            .values(&row)
                            .execute(conn)
                            .await?;

                        let budget_row: Option<(i64, i64)> = promoted_posts::table
                            .filter(promoted_posts::promotion_id.eq(promotion_id))
                            .order(promoted_posts::time.desc())
                            .select((
                                promoted_posts::remaining_budget,
                                promoted_posts::payment_per_view,
                            ))
                            .first(conn)
                            .await
                            .optional()?;
                        if let Some((remaining_budget, payment_per_view)) = budget_row {
                            let new_remaining = remaining_budget.saturating_sub(*payment_amount);
                            let still_active = new_remaining >= payment_per_view;
                            let budget_event = NewPromotionBudgetEvent {
                                promotion_id: promotion_id.clone(),
                                post_id: resolved_post_id.clone(),
                                event_type: "view_payment".to_string(),
                                amount: *payment_amount,
                                remaining_budget: new_remaining,
                                timestamp: *timestamp,
                                time,
                                transaction_id: transaction_id.clone(),
                            };
                            total += diesel::insert_into(promotion_budget_events::table)
                                .values(&budget_event)
                                .execute(conn)
                                .await?;
                            total += diesel::update(promoted_posts::table)
                                .filter(promoted_posts::promotion_id.eq(promotion_id))
                                .set((
                                    promoted_posts::remaining_budget.eq(new_remaining),
                                    promoted_posts::active.eq(still_active),
                                ))
                                .execute(conn)
                                .await?;
                        }

                        total += insert_promotion_view_unified_revenue(
                            conn,
                            promotion_id,
                            &resolved_post_id,
                            viewer,
                            platform_id,
                            *payment_amount,
                            *platform_fee,
                            *ecosystem_fee,
                            *recipient_amount,
                            *timestamp,
                            transaction_id,
                        )
                        .await?;
                    } else {
                        tracing::warn!(
                            promotion_id = %promotion_id,
                            transaction_id = %transaction_id,
                            "PromotionView missing post_id and promoted_posts lookup failed; skipping"
                        );
                    }
                }
                PostRow::PromotionStatusEvent {
                    promotion_id,
                    toggled_by,
                    new_status,
                    timestamp,
                    transaction_id,
                } => {
                    let post_id_opt: Option<String> = promoted_posts::table
                        .filter(promoted_posts::promotion_id.eq(promotion_id))
                        .order(promoted_posts::time.desc())
                        .select(promoted_posts::post_id)
                        .first::<String>(conn)
                        .await
                        .ok();
                    if let Some(post_id) = post_id_opt {
                        let time = chrono::DateTime::from_timestamp(*timestamp / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotionStatusEvent {
                            post_id,
                            promotion_id: promotion_id.clone(),
                            event_type: "status_toggled".to_string(),
                            triggered_by: toggled_by.clone(),
                            new_status: Some(*new_status),
                            amount: None,
                            timestamp: *timestamp,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promotion_status_events::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                        total += diesel::update(promoted_posts::table)
                            .filter(promoted_posts::promotion_id.eq(promotion_id))
                            .set(promoted_posts::active.eq(*new_status))
                            .execute(conn)
                            .await?;
                    }
                }
                PostRow::PromotionBudgetEvent {
                    promotion_id,
                    owner: _,
                    withdrawn_amount,
                    timestamp,
                    transaction_id,
                } => {
                    let post_id_opt: Option<String> = promoted_posts::table
                        .filter(promoted_posts::promotion_id.eq(promotion_id))
                        .order(promoted_posts::time.desc())
                        .select(promoted_posts::post_id)
                        .first::<String>(conn)
                        .await
                        .ok();
                    if let Some(post_id) = post_id_opt {
                        let time = chrono::DateTime::from_timestamp(*timestamp / 1000, 0)
                            .unwrap_or_else(chrono::Utc::now);
                        let row = NewPromotionBudgetEvent {
                            promotion_id: promotion_id.clone(),
                            post_id,
                            event_type: "withdrawal".to_string(),
                            amount: *withdrawn_amount,
                            remaining_budget: 0,
                            timestamp: *timestamp,
                            time,
                            transaction_id: transaction_id.clone(),
                        };
                        total += diesel::insert_into(promotion_budget_events::table)
                            .values(&row)
                            .execute(conn)
                            .await?;
                        total += diesel::update(promoted_posts::table)
                            .filter(promoted_posts::promotion_id.eq(promotion_id))
                            .set((
                                promoted_posts::remaining_budget.eq(0),
                                promoted_posts::active.eq(false),
                            ))
                            .execute(conn)
                            .await?;
                    }
                }
                PostRow::UnifiedRevenue(r) => {
                    let mut revenue = r.clone();
                    if revenue.organization_id.is_none() {
                        revenue.organization_id = resolve_organization_id_for_derived_address(
                            conn,
                            &revenue.payer_address,
                        )
                        .await?;
                    }
                    total += diesel::insert_into(unified_revenue::table)
                        .values(&revenue)
                        .execute(conn)
                        .await?;
                    apply_org_revenue(
                        conn,
                        revenue.organization_id.as_deref(),
                        revenue.amount,
                        Some(&revenue.payer_address),
                        revenue.revenue_time,
                    )
                    .await?;
                }
                PostRow::PocBadge(badge) => {
                    total += diesel::insert_into(poc_badges::table)
                        .values(badge)
                        .execute(conn)
                        .await?;
                }
                PostRow::PocAnalysisResult(r) => {
                    let mut row = r.clone();
                    row.similarity_detected = poc::poc_similarity_detected(
                        row.media_type,
                        row.highest_similarity_score,
                        &poc_thresholds,
                    );
                    total += diesel::insert_into(poc_analysis_results::table)
                        .values(&row)
                        .execute(conn)
                        .await?;
                }
                PostRow::PocRevenueRedirection(r) => {
                    total += diesel::insert_into(poc_revenue_redirections::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                    total += diesel::sql_query(
                        "UPDATE poc_analysis_results \
                         SET original_creator = $1 \
                         WHERE post_id = $2 AND transaction_id = $3 AND original_creator IS NULL",
                    )
                    .bind::<Text, _>(&r.original_post_id)
                    .bind::<Text, _>(&r.accused_post_id)
                    .bind::<Text, _>(&r.transaction_id)
                    .execute(conn)
                    .await?;
                }
                PostRow::PocDispute(d) => {
                    total += diesel::insert_into(poc_disputes::table)
                        .values(d)
                        .execute(conn)
                        .await?;
                }
                PostRow::PostPocDisputesSubmitted {
                    post_id,
                    poc_disputes_submitted,
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set(posts::poc_disputes_submitted.eq(*poc_disputes_submitted))
                        .execute(conn)
                        .await?;
                }
                PostRow::PocDisputeVote(v) => {
                    total += diesel::insert_into(poc_dispute_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                PostRow::PocConfiguration(c) => {
                    total += diesel::insert_into(poc_config::table)
                        .values(c)
                        .execute(conn)
                        .await?;
                }
                PostRow::PostPocUpdate {
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
                PostRow::PostPocResultApplied {
                    post_id,
                    poc_outcome,
                    poc_redirection_kind,
                    ..
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set((
                            posts::poc_outcome.eq(Some(*poc_outcome)),
                            posts::poc_redirection_kind.eq(Some(*poc_redirection_kind)),
                        ))
                        .execute(conn)
                        .await?;
                }
                PostRow::PostPocBadgePointer {
                    post_id,
                    poc_badge_object_id,
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set(posts::poc_id.eq(Some(poc_badge_object_id.clone())))
                        .execute(conn)
                        .await?;
                }
                PostRow::PocBeneficiaryVaultDeposit {
                    vault_id,
                    vault_routing_key,
                    coin_type,
                    amount,
                    source_post_id,
                    timestamp_ms,
                    transaction_id,
                } => {
                    let deposit = NewPocVaultDeposit {
                        vault_id: vault_id.clone(),
                        vault_routing_key: vault_routing_key.clone(),
                        amount: *amount,
                        coin_type: coin_type.clone(),
                        source_post_id: source_post_id.clone(),
                        occurred_at_ms: *timestamp_ms,
                        transaction_id: transaction_id.clone(),
                    };
                    total += diesel::insert_into(poc_vault_deposits::table)
                        .values(&deposit)
                        .execute(conn)
                        .await?;
                    let vault_meta_sql = "INSERT INTO poc_beneficiary_vaults (vault_id, vault_routing_key, updated_at_ms, transaction_id, time) \
                        VALUES ($1, $2, $3, $4, NOW()) \
                        ON CONFLICT (vault_id) DO UPDATE SET \
                        vault_routing_key = EXCLUDED.vault_routing_key, \
                        updated_at_ms = EXCLUDED.updated_at_ms, \
                        transaction_id = EXCLUDED.transaction_id, \
                        time = NOW()";
                    total += diesel::sql_query(vault_meta_sql)
                        .bind::<Text, _>(vault_id.clone())
                        .bind::<Text, _>(vault_routing_key.clone())
                        .bind::<BigInt, _>(*timestamp_ms)
                        .bind::<Text, _>(transaction_id.clone())
                        .execute(conn)
                        .await?;
                    let coin_bal_sql = "INSERT INTO poc_vault_coin_balances (vault_id, coin_type, balance, updated_at_ms, transaction_id, time) \
                        VALUES ($1, $2, $3, $4, $5, NOW()) \
                        ON CONFLICT (vault_id, coin_type) DO UPDATE SET \
                        balance = poc_vault_coin_balances.balance + EXCLUDED.balance, \
                        updated_at_ms = EXCLUDED.updated_at_ms, \
                        transaction_id = EXCLUDED.transaction_id, \
                        time = NOW()";
                    total += diesel::sql_query(coin_bal_sql)
                        .bind::<Text, _>(vault_id)
                        .bind::<Text, _>(coin_type.clone())
                        .bind::<BigInt, _>(*amount)
                        .bind::<BigInt, _>(*timestamp_ms)
                        .bind::<Text, _>(transaction_id)
                        .execute(conn)
                        .await?;
                }
                PostRow::PocBeneficiaryVaultClaimed {
                    vault_id,
                    vault_routing_key,
                    coin_type,
                    referrer_address,
                    treasury_amount,
                    referrer_amount,
                    beneficiary_amount,
                    join_referral_applied: _,
                    claim_kind,
                    timestamp_ms,
                    transaction_id,
                } => {
                    let gross_i64: i64 = (*treasury_amount as i128
                        + *referrer_amount as i128
                        + *beneficiary_amount as i128)
                        .try_into()
                        .expect("PoC vault claim gross fits i64");
                    let row = NewPocVaultClaim {
                        vault_id: vault_id.clone(),
                        vault_routing_key: vault_routing_key.clone(),
                        coin_type: coin_type.clone(),
                        referrer_address: referrer_address.clone(),
                        treasury_amount: *treasury_amount,
                        referrer_amount: *referrer_amount,
                        beneficiary_amount: *beneficiary_amount,
                        occurred_at_ms: *timestamp_ms,
                        transaction_id: transaction_id.clone(),
                        claim_kind: claim_kind.clone(),
                        gross_amount: gross_i64,
                    };
                    total += diesel::insert_into(poc_vault_claims::table)
                        .values(&row)
                        .execute(conn)
                        .await?;
                    let deduct_sql = "UPDATE poc_vault_coin_balances SET balance = balance - $3, updated_at_ms = $4, transaction_id = $5, time = NOW() \
                        WHERE vault_id = $1 AND coin_type = $2";
                    total += diesel::sql_query(deduct_sql)
                        .bind::<Text, _>(vault_id.clone())
                        .bind::<Text, _>(coin_type.clone())
                        .bind::<BigInt, _>(gross_i64)
                        .bind::<BigInt, _>(*timestamp_ms)
                        .bind::<Text, _>(transaction_id)
                        .execute(conn)
                        .await?;
                    let vault_touch_sql = "UPDATE poc_beneficiary_vaults SET updated_at_ms = $2, transaction_id = $3, time = NOW() WHERE vault_id = $1";
                    total += diesel::sql_query(vault_touch_sql)
                        .bind::<Text, _>(vault_id)
                        .bind::<BigInt, _>(*timestamp_ms)
                        .bind::<Text, _>(&row.transaction_id)
                        .execute(conn)
                        .await?;
                }
                PostRow::PocUsernameBeneficiary(row) => {
                    use diesel::sql_types::{Bool, Int2, Nullable, Timestamptz};
                    let upsert_sql = "INSERT INTO poc_username_beneficiaries (
                        beneficiary_id, username, status, creator_identity_source, creator_identity_hash,
                        vault_routing_key, vault_id, required_x_handle, oracle_evidence_hash,
                        provisioned_at_ms, provisioned_by, claimed_profile_id, claimed_by, claimed_at_ms,
                        ended_at_ms, ended_by, end_reason_code, join_referrer, join_referral_paid,
                        join_referral_paid_at_ms, transaction_id, time
                    ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21, $22)
                    ON CONFLICT (beneficiary_id) DO UPDATE SET
                        username = EXCLUDED.username,
                        status = EXCLUDED.status,
                        creator_identity_source = EXCLUDED.creator_identity_source,
                        creator_identity_hash = EXCLUDED.creator_identity_hash,
                        vault_routing_key = EXCLUDED.vault_routing_key,
                        vault_id = EXCLUDED.vault_id,
                        required_x_handle = EXCLUDED.required_x_handle,
                        oracle_evidence_hash = EXCLUDED.oracle_evidence_hash,
                        provisioned_at_ms = EXCLUDED.provisioned_at_ms,
                        provisioned_by = EXCLUDED.provisioned_by,
                        claimed_profile_id = EXCLUDED.claimed_profile_id,
                        claimed_by = EXCLUDED.claimed_by,
                        claimed_at_ms = EXCLUDED.claimed_at_ms,
                        ended_at_ms = EXCLUDED.ended_at_ms,
                        ended_by = EXCLUDED.ended_by,
                        end_reason_code = EXCLUDED.end_reason_code,
                        join_referrer = EXCLUDED.join_referrer,
                        join_referral_paid = EXCLUDED.join_referral_paid,
                        join_referral_paid_at_ms = EXCLUDED.join_referral_paid_at_ms,
                        transaction_id = EXCLUDED.transaction_id,
                        time = EXCLUDED.time";
                    total += diesel::sql_query(upsert_sql)
                        .bind::<Text, _>(&row.beneficiary_id)
                        .bind::<Text, _>(&row.username)
                        .bind::<Int2, _>(row.status)
                        .bind::<Int2, _>(row.creator_identity_source)
                        .bind::<Text, _>(&row.creator_identity_hash)
                        .bind::<Text, _>(&row.vault_routing_key)
                        .bind::<Text, _>(&row.vault_id)
                        .bind::<Text, _>(&row.required_x_handle)
                        .bind::<Text, _>(&row.oracle_evidence_hash)
                        .bind::<BigInt, _>(row.provisioned_at_ms)
                        .bind::<Text, _>(&row.provisioned_by)
                        .bind::<Nullable<Text>, _>(row.claimed_profile_id.as_ref())
                        .bind::<Nullable<Text>, _>(row.claimed_by.as_ref())
                        .bind::<Nullable<BigInt>, _>(row.claimed_at_ms)
                        .bind::<Nullable<BigInt>, _>(row.ended_at_ms)
                        .bind::<Nullable<Text>, _>(row.ended_by.as_ref())
                        .bind::<Nullable<Int2>, _>(row.end_reason_code)
                        .bind::<Nullable<Text>, _>(row.join_referrer.as_ref())
                        .bind::<Bool, _>(row.join_referral_paid)
                        .bind::<Nullable<BigInt>, _>(row.join_referral_paid_at_ms)
                        .bind::<Text, _>(&row.transaction_id)
                        .bind::<Timestamptz, _>(row.time)
                        .execute(conn)
                        .await?;
                    let vault_meta_sql = "INSERT INTO poc_beneficiary_vaults (vault_id, vault_routing_key, updated_at_ms, transaction_id, time) \
                        VALUES ($1, $2, $3, $4, NOW()) \
                        ON CONFLICT (vault_id) DO UPDATE SET \
                        vault_routing_key = EXCLUDED.vault_routing_key, \
                        updated_at_ms = EXCLUDED.updated_at_ms, \
                        transaction_id = EXCLUDED.transaction_id, \
                        time = NOW()";
                    total += diesel::sql_query(vault_meta_sql)
                        .bind::<Text, _>(&row.vault_id)
                        .bind::<Text, _>(&row.vault_routing_key)
                        .bind::<BigInt, _>(row.provisioned_at_ms)
                        .bind::<Text, _>(&row.transaction_id)
                        .execute(conn)
                        .await?;
                }
                PostRow::PocUsernameBeneficiaryClaimed {
                    beneficiary_id,
                    profile_id,
                    claimed_by,
                    oracle_evidence_hash,
                    claimed_at_ms,
                    transaction_id,
                    ..
                } => {
                    use diesel::sql_types::Int2;
                    use myso_indexer_alt_social_schema::models::USERNAME_BENEFICIARY_STATUS_CLAIMED;
                    let update_sql = "UPDATE poc_username_beneficiaries SET status = $2, claimed_profile_id = $3, claimed_by = $4, \
                        oracle_evidence_hash = $5, claimed_at_ms = $6, transaction_id = $7, time = NOW() \
                        WHERE beneficiary_id = $1";
                    total += diesel::sql_query(update_sql)
                        .bind::<Text, _>(beneficiary_id)
                        .bind::<Int2, _>(USERNAME_BENEFICIARY_STATUS_CLAIMED)
                        .bind::<Text, _>(profile_id)
                        .bind::<Text, _>(claimed_by)
                        .bind::<Text, _>(oracle_evidence_hash)
                        .bind::<BigInt, _>(*claimed_at_ms)
                        .bind::<Text, _>(transaction_id)
                        .execute(conn)
                        .await?;
                }
                PostRow::PocUsernameBeneficiaryEnded {
                    beneficiary_id,
                    ended_by,
                    end_reason_code,
                    ended_at_ms,
                    transaction_id,
                    ..
                } => {
                    use diesel::sql_types::{Int2, Nullable};
                    use myso_indexer_alt_social_schema::models::USERNAME_BENEFICIARY_STATUS_ENDED;
                    let update_sql = "UPDATE poc_username_beneficiaries SET status = $2, ended_by = $3, end_reason_code = $4, \
                        ended_at_ms = $5, transaction_id = $6, time = NOW() \
                        WHERE beneficiary_id = $1";
                    total += diesel::sql_query(update_sql)
                        .bind::<Text, _>(beneficiary_id)
                        .bind::<Int2, _>(USERNAME_BENEFICIARY_STATUS_ENDED)
                        .bind::<Text, _>(ended_by)
                        .bind::<Nullable<Int2>, _>(Some(*end_reason_code))
                        .bind::<Nullable<BigInt>, _>(Some(*ended_at_ms))
                        .bind::<Text, _>(transaction_id)
                        .execute(conn)
                        .await?;
                }
                PostRow::PocUsernameBeneficiaryJoinReferralPaid {
                    vault_id,
                    join_referrer,
                    join_referral_paid_at_ms,
                    transaction_id,
                } => {
                    use diesel::sql_types::{Bool, Nullable};
                    let update_sql = "UPDATE poc_username_beneficiaries SET join_referral_paid = $2, join_referrer = $3, \
                        join_referral_paid_at_ms = $4, transaction_id = $5, time = NOW() \
                        WHERE vault_id = $1";
                    total += diesel::sql_query(update_sql)
                        .bind::<Text, _>(vault_id)
                        .bind::<Bool, _>(true)
                        .bind::<Nullable<Text>, _>(join_referrer.clone())
                        .bind::<Nullable<BigInt>, _>(Some(*join_referral_paid_at_ms))
                        .bind::<Text, _>(transaction_id)
                        .execute(conn)
                        .await?;
                }
                PostRow::PocCreatorIdentityLink(row) => {
                    total += diesel::insert_into(poc_creator_identity_links::table)
                        .values(row)
                        .on_conflict((
                            poc_creator_identity_links::creator_identity_source,
                            poc_creator_identity_links::creator_identity_hash,
                        ))
                        .do_update()
                        .set((
                            poc_creator_identity_links::wallet_address.eq(
                                diesel::upsert::excluded(
                                    poc_creator_identity_links::wallet_address,
                                ),
                            ),
                            poc_creator_identity_links::beneficiary_id.eq(
                                diesel::upsert::excluded(
                                    poc_creator_identity_links::beneficiary_id,
                                ),
                            ),
                            poc_creator_identity_links::linked_at_ms.eq(diesel::upsert::excluded(
                                poc_creator_identity_links::linked_at_ms,
                            )),
                            poc_creator_identity_links::transaction_id.eq(
                                diesel::upsert::excluded(
                                    poc_creator_identity_links::transaction_id,
                                ),
                            ),
                            poc_creator_identity_links::time
                                .eq(diesel::upsert::excluded(poc_creator_identity_links::time)),
                        ))
                        .execute(conn)
                        .await?;
                }
                PostRow::PocUsernameBeneficiaryEvent(row) => {
                    total += diesel::insert_into(poc_username_beneficiary_events::table)
                        .values(row)
                        .on_conflict(poc_username_beneficiary_events::event_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                PostRow::PostRevenueRedirectUpdate {
                    post_id,
                    revenue_redirect_to,
                    revenue_redirect_percentage,
                    poc_redirection_kind,
                } => {
                    total += diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set((
                            posts::revenue_redirect_to.eq(Some(revenue_redirect_to.clone())),
                            posts::revenue_redirect_percentage
                                .eq(Some(*revenue_redirect_percentage)),
                            posts::poc_redirection_kind.eq(Some(*poc_redirection_kind)),
                        ))
                        .execute(conn)
                        .await?;
                }
                PostRow::PocDisputeResolved {
                    dispute_id,
                    post_id,
                    resolution,
                    winning_side,
                    total_winning_stake,
                    total_losing_stake,
                    resolved_at,
                    badge_revoked,
                    redirection_removed,
                    quorum_met,
                } => {
                    let update_sql = "UPDATE poc_disputes SET status = $1, resolution = $2, winning_side = $3, total_winning_stake = $4, total_losing_stake = $5, resolved_at = $6, quorum_met = $7 \
                        WHERE dispute_id = $8 AND time = (SELECT time FROM poc_disputes WHERE dispute_id = $8 ORDER BY time DESC LIMIT 1)";
                    total += diesel::sql_query(update_sql)
                        .bind::<Int2, _>(*resolution)
                        .bind::<Nullable<Int2>, _>(Some(*resolution))
                        .bind::<Nullable<Int2>, _>(Some(*winning_side))
                        .bind::<Nullable<BigInt>, _>(Some(*total_winning_stake))
                        .bind::<Nullable<BigInt>, _>(Some(*total_losing_stake))
                        .bind::<Nullable<BigInt>, _>(Some(*resolved_at))
                        .bind::<Bool, _>(*quorum_met)
                        .bind::<Text, _>(dispute_id)
                        .execute(conn)
                        .await?;

                    if *badge_revoked {
                        let revoke_sql = "UPDATE poc_badges SET revoked = TRUE, revoked_at = $1 \
                            WHERE post_id = $2 AND time = (SELECT time FROM poc_badges WHERE post_id = $2 ORDER BY time DESC LIMIT 1)";
                        total += diesel::sql_query(revoke_sql)
                            .bind::<Nullable<BigInt>, _>(Some(*resolved_at))
                            .bind::<Text, _>(post_id)
                            .execute(conn)
                            .await?;

                        total += diesel::update(posts::table)
                            .filter(posts::post_id.eq(post_id))
                            .set((
                                posts::poc_outcome.eq(None::<i16>),
                                posts::poc_redirection_kind.eq(None::<i16>),
                                posts::poc_id.eq(None::<String>),
                                posts::poc_reasoning.eq(None::<String>),
                                posts::poc_evidence_urls.eq(None::<serde_json::Value>),
                                posts::poc_similarity_score.eq(None::<i64>),
                                posts::poc_media_type.eq(None::<i16>),
                                posts::poc_oracle_address.eq(None::<String>),
                                posts::poc_analyzed_at.eq(None::<i64>),
                                posts::revenue_redirect_to.eq(None::<String>),
                                posts::revenue_redirect_percentage.eq(None::<i64>),
                            ))
                            .execute(conn)
                            .await?;
                    }

                    if *redirection_removed {
                        let remove_sql = "UPDATE poc_revenue_redirections SET removed = TRUE, removed_at = $1 \
                            WHERE accused_post_id = $2 AND time = (SELECT time FROM poc_revenue_redirections WHERE accused_post_id = $2 ORDER BY time DESC LIMIT 1)";
                        total += diesel::sql_query(remove_sql)
                            .bind::<Nullable<BigInt>, _>(Some(*resolved_at))
                            .bind::<Text, _>(post_id)
                            .execute(conn)
                            .await?;

                        if !*badge_revoked {
                            total += diesel::update(posts::table)
                                .filter(posts::post_id.eq(post_id))
                                .set((
                                    posts::revenue_redirect_to.eq(None::<String>),
                                    posts::revenue_redirect_percentage.eq(None::<i64>),
                                    posts::poc_redirection_kind.eq(None::<i16>),
                                ))
                                .execute(conn)
                                .await?;
                        }
                    }
                }
                PostRow::PocVoteRewardClaimed {
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

async fn load_ecosystem_treasury_address(conn: &mut Connection<'_>) -> Option<String> {
    ecosystem_treasury::table
        .order(ecosystem_treasury::time.desc())
        .select(ecosystem_treasury::treasury_address)
        .first(conn)
        .await
        .ok()
}

async fn insert_promotion_view_unified_revenue(
    conn: &mut Connection<'_>,
    promotion_id: &str,
    post_id: &str,
    viewer: &str,
    platform_id: &str,
    _payment_amount: i64,
    platform_fee: i64,
    ecosystem_fee: i64,
    recipient_amount: i64,
    revenue_time: i64,
    transaction_id: &str,
) -> Result<usize> {
    let payer_address: String = promoted_posts::table
        .filter(promoted_posts::promotion_id.eq(promotion_id))
        .order(promoted_posts::time.desc())
        .select(promoted_posts::owner)
        .first(conn)
        .await
        .optional()?
        .unwrap_or_else(|| "0x0".to_string());

    let mut total = 0usize;
    if recipient_amount > 0 {
        total += diesel::insert_into(unified_revenue::table)
            .values(NewUnifiedRevenue::from_post(
                REVENUE_TYPE_PROMOTION_VIEWER_NET.to_string(),
                viewer.to_string(),
                Some(platform_id.to_string()),
                recipient_amount,
                post_id.to_string(),
                payer_address.clone(),
                viewer.to_string(),
                revenue_time,
                transaction_id.to_string(),
            ))
            .execute(conn)
            .await?;
    }
    if platform_fee > 0 {
        total += diesel::insert_into(unified_revenue::table)
            .values(NewUnifiedRevenue::from_post(
                REVENUE_TYPE_PROMOTION_PLATFORM_FEE.to_string(),
                viewer.to_string(),
                Some(platform_id.to_string()),
                platform_fee,
                post_id.to_string(),
                payer_address.clone(),
                platform_id.to_string(),
                revenue_time,
                transaction_id.to_string(),
            ))
            .execute(conn)
            .await?;
    }
    if ecosystem_fee > 0 {
        if let Some(treasury) = load_ecosystem_treasury_address(conn).await {
            total += diesel::insert_into(unified_revenue::table)
                .values(NewUnifiedRevenue::from_post(
                    REVENUE_TYPE_PROMOTION_ECOSYSTEM_FEE.to_string(),
                    viewer.to_string(),
                    Some(platform_id.to_string()),
                    ecosystem_fee,
                    post_id.to_string(),
                    payer_address,
                    treasury,
                    revenue_time,
                    transaction_id.to_string(),
                ))
                .execute(conn)
                .await?;
        }
    }
    Ok(total)
}

#[cfg(test)]
mod post_row_poc_mapping_tests {
    use super::{classify_reaction, PostRow, ReactionApplyKind};
    use crate::handlers::SocialEventRow;
    use myso_indexer_alt_social_schema::models::{NewPocBadge, NewPocUsernameBeneficiary};

    #[test]
    fn classify_reaction_new_when_no_prior() {
        assert_eq!(classify_reaction(None, "👍"), ReactionApplyKind::New);
    }

    #[test]
    fn classify_reaction_swap_when_different_emoji() {
        assert_eq!(
            classify_reaction(Some("👍"), "❤️"),
            ReactionApplyKind::Swap {
                previous: "👍".to_string()
            }
        );
    }

    #[test]
    fn classify_reaction_replay_when_same_emoji() {
        assert_eq!(
            classify_reaction(Some("👍"), "👍"),
            ReactionApplyKind::Replay
        );
    }

    #[test]
    fn post_row_maps_poc_badge_social_event() {
        let b = NewPocBadge {
            badge_id: "0x1".to_string(),
            post_id: "0x2".to_string(),
            media_type: 1,
            issued_by: "0x3".to_string(),
            beneficiary_address: None,
            matched_anchor_id: None,
            media_index: None,
            issued_at: 0,
            revoked: false,
            revoked_at: None,
            transaction_id: "tx".to_string(),
        };
        let r = PostRow::from_social(SocialEventRow::PocBadge(b.clone()));
        assert!(matches!(r, Some(PostRow::PocBadge(x)) if x.badge_id == b.badge_id));
    }

    #[test]
    fn post_row_maps_username_beneficiary_social_event() {
        let row = NewPocUsernameBeneficiary {
            beneficiary_id: "0xb1".to_string(),
            username: "alice".to_string(),
            status: 1,
            creator_identity_source: 1,
            creator_identity_hash: "0xabc".to_string(),
            vault_routing_key: "0xba".to_string(),
            vault_id: "0xv1".to_string(),
            required_x_handle: "alice_x".to_string(),
            oracle_evidence_hash: String::new(),
            provisioned_at_ms: 1000,
            provisioned_by: "0xadmin".to_string(),
            claimed_profile_id: None,
            claimed_by: None,
            claimed_at_ms: None,
            ended_at_ms: None,
            ended_by: None,
            end_reason_code: None,
            join_referrer: None,
            join_referral_paid: false,
            join_referral_paid_at_ms: None,
            transaction_id: "tx".to_string(),
            time: chrono::Utc::now(),
        };
        let mapped = PostRow::from_social(SocialEventRow::PocUsernameBeneficiary(row.clone()));
        assert!(
            matches!(mapped, Some(PostRow::PocUsernameBeneficiary(r)) if r.beneficiary_id == row.beneficiary_id)
        );
    }

    #[test]
    fn poc_vault_claim_gross_amount_is_sum_of_slices() {
        let treasury = 1_000_000i64;
        let referrer = 0i64;
        let beneficiary = 99_000_000i64;
        let gross: i64 = (treasury as i128 + referrer as i128 + beneficiary as i128)
            .try_into()
            .expect("gross fits i64");
        assert_eq!(gross, 100_000_000);
    }

    #[test]
    fn tips_received_increment_matches_owner_case_insensitively() {
        let owner = "0xAbCd00000000000000000000000000000000000000000000000000000001";
        let recipient = "0xabcd00000000000000000000000000000000000000000000000000000001";
        assert!(owner.eq_ignore_ascii_case(recipient));
    }

    #[test]
    fn post_tips_received_increment_row_type_is_distinct_from_tip() {
        use myso_indexer_alt_social_schema::models::NewTip;
        let tip = NewTip {
            tipper: "0x1".to_string(),
            recipient: "0x2".to_string(),
            object_id: "0xpost".to_string(),
            amount: 100,
            coin_type: "0x2::myso::MYSO".to_string(),
            is_post: true,
            created_at: 0,
            time: chrono::Utc::now(),
            transaction_id: "tx".to_string(),
            organization_id: None,
        };
        let tip_row = PostRow::from_social(SocialEventRow::Tip(tip));
        let increment_row = PostRow::from_social(SocialEventRow::PostTipsReceivedIncrement {
            object_id: "0xpost".to_string(),
            recipient: "0x2".to_string(),
            amount: 100,
            is_post: true,
        });
        assert!(matches!(tip_row, Some(PostRow::Tip(_))));
        assert!(matches!(
            increment_row,
            Some(PostRow::PostTipsReceivedIncrement { .. })
        ));
    }
}

#[cfg(test)]
mod promotion_view_commit_tests {
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    use myso_indexer_alt_framework::postgres::handler::Handler;
    use myso_indexer_alt_social_schema::models::NewPromotedPost;
    use myso_indexer_alt_social_schema::schema::{
        promoted_posts, promotion_budget_events, promotion_views, unified_revenue,
    };
    use myso_indexer_alt_social_schema::MIGRATIONS;
    use myso_pg_db::temp::TempDb;
    use myso_pg_db::Db;

    use super::PostRow;
    use super::PostsHandler;

    fn addr_hex(id: u8) -> String {
        format!("0x{:064x}", id)
    }

    async fn setup_temp_db() -> Option<Db> {
        let temp_db = TempDb::new().ok()?;
        let store = Db::for_write(temp_db.database().url().clone(), Default::default())
            .await
            .ok()?;
        {
            let mut probe = store.connect().await.ok()?;
            diesel::sql_query("CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE")
                .execute(&mut probe)
                .await
                .ok()?;
        }
        store.run_migrations(Some(&MIGRATIONS)).await.ok()?;
        Some(store)
    }

    #[tokio::test]
    async fn promotion_view_commit_decrements_remaining_budget() {
        let Some(store) = setup_temp_db().await else {
            return;
        };

        let promotion_id = addr_hex(1);
        let post_id = addr_hex(2);
        let viewer = addr_hex(3);
        let platform_id = addr_hex(4);
        let owner = addr_hex(5);
        let profile_id = addr_hex(6);
        let created_at = 1_700_000_000_000i64;
        let time =
            chrono::DateTime::from_timestamp_millis(created_at).unwrap_or_else(chrono::Utc::now);

        let mut conn = store.connect().await.expect("connection");
        diesel::insert_into(promoted_posts::table)
            .values(NewPromotedPost {
                promotion_id: promotion_id.clone(),
                post_id: post_id.clone(),
                owner: owner.clone(),
                profile_id: profile_id.clone(),
                payment_per_view: 1_000_000,
                total_budget: 3_000_000,
                remaining_budget: 3_000_000,
                active: true,
                created_at,
                time,
                transaction_id: "tx:seed:0".to_string(),
            })
            .execute(&mut conn)
            .await
            .expect("insert promoted post");

        let view_row = PostRow::PromotionView {
            post_id: post_id.clone(),
            promotion_id: promotion_id.clone(),
            viewer: viewer.clone(),
            payment_amount: 1_000_000,
            platform_fee: 100_000,
            ecosystem_fee: 100_000,
            recipient_amount: 800_000,
            view_duration: 3_000,
            platform_id: platform_id.clone(),
            timestamp: created_at + 1_000,
            transaction_id: "tx:view:0".to_string(),
        };
        PostsHandler::commit(&[view_row], &mut conn)
            .await
            .expect("commit promotion view");

        let (remaining_budget, active): (i64, bool) = promoted_posts::table
            .filter(promoted_posts::promotion_id.eq(&promotion_id))
            .select((promoted_posts::remaining_budget, promoted_posts::active))
            .first(&mut conn)
            .await
            .expect("promoted post row");
        assert_eq!(remaining_budget, 2_000_000);
        assert!(active);

        let budget_events: i64 = promotion_budget_events::table
            .filter(promotion_budget_events::event_type.eq("view_payment"))
            .count()
            .get_result(&mut conn)
            .await
            .expect("budget event count");
        assert_eq!(budget_events, 1);

        let views: i64 = promotion_views::table
            .filter(promotion_views::promotion_id.eq(&promotion_id))
            .count()
            .get_result(&mut conn)
            .await
            .expect("view count");
        assert_eq!(views, 1);

        let revenue_rows: Vec<(String, i64, String)> = unified_revenue::table
            .filter(unified_revenue::transaction_id.eq("tx:view:0"))
            .select((
                unified_revenue::revenue_type,
                unified_revenue::amount,
                unified_revenue::recipient_address,
            ))
            .load(&mut conn)
            .await
            .expect("unified revenue");
        assert_eq!(revenue_rows.len(), 2); // viewer net + platform fee (no treasury row seeded)
        assert!(revenue_rows
            .iter()
            .any(|(t, a, r)| { t == "promotion_viewer_net" && *a == 800_000 && r == &viewer }));
        assert!(revenue_rows.iter().any(|(t, a, r)| {
            t == "promotion_platform_fee" && *a == 100_000 && r == &platform_id
        }));
    }
}

#[cfg(test)]
mod poc_analysis_result_commit_tests {
    use chrono::TimeZone;
    use diesel::ExpressionMethods;
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    use myso_indexer_alt_framework::postgres::handler::Handler;
    use myso_indexer_alt_social_schema::models::{
        NewPocAnalysisResult, NewPocRevenueRedirection, NewPost,
    };
    use myso_indexer_alt_social_schema::schema::{
        poc_analysis_results, poc_revenue_redirections, posts,
    };
    use myso_indexer_alt_social_schema::MIGRATIONS;
    use myso_pg_db::temp::TempDb;
    use myso_pg_db::Db;

    use super::PostRow;
    use super::PostsHandler;

    fn addr_hex(id: u8) -> String {
        format!("0x{:064x}", id)
    }

    fn minimal_post(post_id: &str, tx_id: &str) -> NewPost {
        let created_at = 1_700_000_000_000i64;
        let time = chrono::Utc.timestamp_millis_opt(created_at).unwrap();
        NewPost {
            post_id: post_id.to_string(),
            owner: addr_hex(10),
            profile_id: addr_hex(11),
            content: "poc test post".to_string(),
            media_urls: None,
            mentions: None,
            metadata_json: None,
            post_type: "standard".to_string(),
            parent_post_id: None,
            created_at,
            updated_at: None,
            deleted_at: None,
            reaction_count: 0,
            comment_count: 0,
            repost_count: 0,
            tips_received: 0,
            total_tip_volume: 0,
            removed_from_platform: false,
            removed_by: None,
            transaction_id: tx_id.to_string(),
            time,
            mydata_id: None,
            revenue_recipient: None,
            poc_id: None,
            poc_reasoning: None,
            poc_evidence_urls: None,
            poc_similarity_score: None,
            poc_media_type: None,
            poc_oracle_address: None,
            poc_analyzed_at: None,
            poc_outcome: None,
            poc_redirection_kind: None,
            poc_disputes_submitted: 0,
            revenue_redirect_to: None,
            revenue_redirect_percentage: None,
            requires_subscription: None,
            subscription_service_id: None,
            subscription_price: None,
            subscription_min_tier_level: None,
            post_access_kind: Some("public".to_string()),
            encrypted_content_hash: None,
            promotion_id: None,
            enable_spt: false,
            spt_id: None,
            platform_id: None,
            permissions: None,
            sub_agent_id: None,
            action_identity_class: None,
            organization_id: None,
            contract_version: 0,
        }
    }

    async fn setup_temp_db() -> Option<Db> {
        let temp_db = TempDb::new().ok()?;
        let store = Db::for_write(temp_db.database().url().clone(), Default::default())
            .await
            .ok()?;
        {
            let mut probe = store.connect().await.ok()?;
            diesel::sql_query("CREATE EXTENSION IF NOT EXISTS timescaledb CASCADE")
                .execute(&mut probe)
                .await
                .ok()?;
        }
        store.run_migrations(Some(&MIGRATIONS)).await.ok()?;
        Some(store)
    }

    #[tokio::test]
    async fn analysis_commit_derives_similarity_detected_from_score() {
        let Some(store) = setup_temp_db().await else {
            return;
        };

        let post_id = addr_hex(2);
        let tx_id = "tx:analysis:0";
        let mut conn = store.connect().await.expect("connection");
        diesel::insert_into(posts::table)
            .values(minimal_post(&post_id, tx_id))
            .execute(&mut conn)
            .await
            .expect("insert post");

        let analysis_row = PostRow::PocAnalysisResult(NewPocAnalysisResult {
            post_id: post_id.clone(),
            media_type: 1,
            similarity_detected: false,
            highest_similarity_score: 100,
            oracle_address: addr_hex(5),
            original_creator: None,
            analysis_timestamp: 1_700_000_001_000,
            transaction_id: tx_id.to_string(),
            reasoning: None,
            evidence_urls: None,
        });
        PostsHandler::commit(&[analysis_row], &mut conn)
            .await
            .expect("commit analysis");

        let detected: bool = poc_analysis_results::table
            .filter(poc_analysis_results::post_id.eq(&post_id))
            .select(poc_analysis_results::similarity_detected)
            .first(&mut conn)
            .await
            .expect("analysis row");
        assert!(detected);
    }

    #[tokio::test]
    async fn revenue_redirection_commit_populates_original_creator_on_analysis() {
        let Some(store) = setup_temp_db().await else {
            return;
        };

        let accused_post_id = addr_hex(2);
        let original_post_id = addr_hex(3);
        let tx_id = "tx:redirect:0";
        let mut conn = store.connect().await.expect("connection");
        diesel::insert_into(posts::table)
            .values(minimal_post(&accused_post_id, tx_id))
            .execute(&mut conn)
            .await
            .expect("insert accused post");
        diesel::insert_into(posts::table)
            .values(minimal_post(&original_post_id, "tx:original:0"))
            .execute(&mut conn)
            .await
            .expect("insert original post");

        let rows = [
            PostRow::PocAnalysisResult(NewPocAnalysisResult {
                post_id: accused_post_id.clone(),
                media_type: 1,
                similarity_detected: false,
                highest_similarity_score: 100,
                oracle_address: addr_hex(5),
                original_creator: None,
                analysis_timestamp: 1_700_000_001_000,
                transaction_id: tx_id.to_string(),
                reasoning: None,
                evidence_urls: None,
            }),
            PostRow::PocRevenueRedirection(NewPocRevenueRedirection {
                redirection_id: accused_post_id.clone(),
                accused_post_id: accused_post_id.clone(),
                original_post_id: original_post_id.clone(),
                redirect_percentage: 50,
                similarity_score: 100,
                created_at: 1_700_000_002_000,
                removed: false,
                removed_at: None,
                transaction_id: tx_id.to_string(),
            }),
        ];
        PostsHandler::commit(&rows, &mut conn)
            .await
            .expect("commit analysis and redirect");

        let creator: Option<String> = poc_analysis_results::table
            .filter(poc_analysis_results::post_id.eq(&accused_post_id))
            .select(poc_analysis_results::original_creator)
            .first(&mut conn)
            .await
            .expect("analysis row");
        assert_eq!(creator.as_deref(), Some(original_post_id.as_str()));

        let redirect_count: i64 = poc_revenue_redirections::table
            .filter(poc_revenue_redirections::transaction_id.eq(tx_id))
            .count()
            .get_result(&mut conn)
            .await
            .expect("redirect count");
        assert_eq!(redirect_count, 1);
    }
}
