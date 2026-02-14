// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Social events pipeline: processes myso-social events from checkpoints into social tables.
//!
//! Filters events by MYSO_SOCIAL_PACKAGE_ID, routes by module/event name, and inserts into
//! profiles, social_graph_relationships, social_graph_events, etc.

mod event_handlers;
mod events;

use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use diesel::BoolExpressionMethods;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel_async::RunQueryDsl;
use move_core_types::account_address::AccountAddress;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_framework::FieldCount;
use myso_indexer_alt_social_schema::models::{
    GovernanceRegistryUpdate, NewAnonymousVote, NewBlockedEvent, NewBlockedProfile, NewComment,
    NewCommunityVote, NewDelegate, NewDelegateRating, NewDelegateVote, NewDeletionEvent,
    NewGovernanceEvent, NewGovernanceRegistry, NewModerationEvent, NewNominatedDelegate,
    NewPlatform, NewPlatformBlockedProfile, NewPlatformEvent, NewPlatformMembership,
    NewPlatformModerator, NewPlatformTokenAirdrop, NewPost, NewProfile, NewProfileBadge,
    NewProfileEvent, NewProposal, NewReaction, NewReactionCount, NewReport, NewRepost,
    NewRewardDistribution, NewSocialGraphEvent, NewSocialGraphRelationship, NewTip,
    NewVoteDecryptionFailure, ProfileUpdateSet, ProposalUpdateSet,
};
use myso_indexer_alt_social_schema::schema::{
    anonymous_votes, blocked_events, blocked_profiles, comments, community_votes, delegate_ratings,
    delegate_votes, delegates, governance_events, governance_registries, nominated_delegates,
    platform_blocked_profiles, platform_events, platform_memberships, platform_moderators,
    platform_token_airdrops, platforms, posts, posts_deletion_events, posts_moderation_events,
    posts_reports, profile_badges, profile_events, profiles, proposals, reaction_counts, reactions,
    reposts, reward_distributions, social_graph_events, social_graph_relationships, tips,
    vote_decryption_failures,
};
use myso_types::base_types::ObjectID;
use myso_types::MYSO_SOCIAL_PACKAGE_ID;
use tracing::debug;

use self::events::parse_event_contents;

fn is_social_package_event(package_id: &ObjectID, type_address: &AccountAddress) -> bool {
    use std::ops::Deref;
    *package_id == MYSO_SOCIAL_PACKAGE_ID || type_address == MYSO_SOCIAL_PACKAGE_ID.deref()
}

pub struct SocialEvents;

#[derive(Debug, Clone)]
pub enum SocialEventRow {
    Profile(NewProfile),
    ProfileUpdate(ProfileUpdate),
    SocialGraphRelationship(NewSocialGraphRelationship),
    SocialGraphEvent(NewSocialGraphEvent),
    SocialGraphUnfollow {
        follower_address: String,
        following_address: String,
    },
    BlockedEvent(NewBlockedEvent),
    BlockedProfile(NewBlockedProfile),
    BlockedProfileDelete {
        blocker_address: String,
        blocked_address: String,
    },
    ProfileEvent(NewProfileEvent),
    ProfileBadge(NewProfileBadge),
    ProfileBadgeRevoke {
        profile_id: String,
        badge_id: String,
        revoked_at: i64,
        revoked_by: String,
    },
    GovernanceRegistry(NewGovernanceRegistry),
    GovernanceRegistryUpdate(GovernanceRegistryUpdate),
    NominatedDelegate(NewNominatedDelegate),
    Delegate(NewDelegate),
    Proposal(NewProposal),
    ProposalUpdate {
        proposal_id: String,
        set: ProposalUpdateSet,
        governance_event: Option<(String, serde_json::Value, String)>,
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
    Post(NewPost),
    Comment(NewComment),
    Reaction(NewReaction),
    ReactionCount(NewReactionCount),
    RemoveReaction {
        object_id: String,
        user_address: String,
        reaction_text: String,
        is_post: bool,
    },
    Repost(NewRepost),
    Tip(NewTip),
    ModerationEvent(NewModerationEvent),
    Report(NewReport),
    DeletionEvent(NewDeletionEvent),
    PostCommentCountIncrement {
        post_id: String,
        delta: i64,
    },
    Platform(NewPlatform),
    PlatformUpdate {
        platform_id: String,
        name: String,
        tagline: String,
        description: Option<String>,
        terms_of_service: Option<String>,
        privacy_policy: Option<String>,
        platform_names: Option<serde_json::Value>,
        links: Option<serde_json::Value>,
        status: i16,
        release_date: Option<String>,
        shutdown_date: Option<String>,
        updated_at: chrono::NaiveDateTime,
        primary_category: String,
        secondary_category: Option<String>,
    },
    PlatformApprovalChange {
        platform_id: String,
        is_approved: bool,
        approved_by: String,
        changed_at: chrono::NaiveDateTime,
    },
    PlatformModerator(NewPlatformModerator),
    PlatformModeratorRemove {
        platform_id: String,
        moderator_address: String,
    },
    PlatformBlockedProfile(NewPlatformBlockedProfile),
    PlatformBlockedProfileRemove {
        platform_id: String,
        wallet_address: String,
    },
    PlatformMembership(NewPlatformMembership),
    PlatformMembershipRemove {
        platform_id: String,
        wallet_address: String,
    },
    PlatformTokenAirdrop(NewPlatformTokenAirdrop),
    PlatformEvent(NewPlatformEvent),
    PlatformDeleted {
        platform_id: String,
        deleted_at: chrono::NaiveDateTime,
    },
}

#[derive(Debug, Clone)]
pub struct ProfileUpdate {
    pub profile_id: String,
    pub owner_address: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub profile_photo: Option<String>,
    pub cover_photo: Option<String>,
    pub birthdate: Option<String>,
    pub current_location: Option<String>,
    pub raised_location: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub gender: Option<String>,
    pub political_view: Option<String>,
    pub religion: Option<String>,
    pub education: Option<String>,
    pub primary_language: Option<String>,
    pub relationship_status: Option<String>,
    pub x_username: Option<String>,
    pub facebook_username: Option<String>,
    pub reddit_username: Option<String>,
    pub github_username: Option<String>,
    pub instagram_username: Option<String>,
    pub linkedin_username: Option<String>,
    pub twitch_username: Option<String>,
    pub min_offer_amount: Option<i64>,
    pub username: Option<String>,
    pub selected_badge_id: Option<String>,
    pub paid_messaging_enabled: Option<bool>,
    pub paid_messaging_min_cost: Option<i64>,
}

impl FieldCount for SocialEventRow {
    const FIELD_COUNT: usize = 62;
}

#[async_trait]
impl Processor for SocialEvents {
    const NAME: &'static str = "social_events";

    type Value = SocialEventRow;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> Result<Vec<Self::Value>> {
        let Checkpoint { transactions, .. } = checkpoint.as_ref();
        let mut values = Vec::new();

        for tx in transactions.iter() {
            let tx_digest = tx.transaction.digest().to_string();

            let Some(events) = &tx.events else {
                continue;
            };

            for (event_seq, ev) in events.data.iter().enumerate() {
                let package_matches = is_social_package_event(&ev.package_id, &ev.type_.address);
                if !package_matches {
                    if ev.type_.module.as_str() == "profile"
                        || ev.type_.module.as_str() == "governance"
                    {
                        debug!(
                            package_id = %ev.package_id,
                            type_address = %ev.type_.address,
                            module = %ev.type_.module,
                            "skipping event: package mismatch (expected 0x50c1)"
                        );
                    }
                    continue;
                }

                let module = ev.type_.module.as_str();
                let event_name = ev.type_.name.as_str();
                let event_id = format!("{}:{}", tx_digest, event_seq);

                let event_data = match parse_event_contents(&ev.contents) {
                    Some(v) => v,
                    None => {
                        debug!(
                            module = %module,
                            event_name = %event_name,
                            event_id = %event_id,
                            contents_len = ev.contents.len(),
                            "skipping event: failed to parse BCS contents (layout may have changed)"
                        );
                        continue;
                    }
                };

                if let Some(rows) =
                    event_handlers::route_event(module, event_name, &event_data, &event_id)
                {
                    values.extend(rows);
                } else {
                    debug!(
                        module = %module,
                        event_name = %event_name,
                        event_id = %event_id,
                        "skipping event: no handler for this module/event"
                    );
                }
            }
        }

        Ok(values)
    }
}

#[async_trait]
impl Handler for SocialEvents {
    const MIN_EAGER_ROWS: usize = 50;
    const MAX_PENDING_ROWS: usize = 5000;

    async fn commit<'a>(values: &[Self::Value], conn: &mut Connection<'a>) -> Result<usize> {
        let mut total = 0;

        for row in values {
            match row {
                SocialEventRow::Profile(profile) => {
                    total += diesel::insert_into(profiles::table)
                        .values(profile)
                        .on_conflict(profiles::owner_address)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileUpdate(up) => {
                    let now = chrono::Utc::now().naive_utc();
                    let set = ProfileUpdateSet {
                        updated_at: now,
                        display_name: up.display_name.clone().map(Some),
                        bio: up.bio.clone().map(Some),
                        profile_photo: up.profile_photo.clone().map(Some),
                        cover_photo: up.cover_photo.clone().map(Some),
                        birthdate: up.birthdate.clone().map(Some),
                        current_location: up.current_location.clone().map(Some),
                        raised_location: up.raised_location.clone().map(Some),
                        phone: up.phone.clone().map(Some),
                        email: up.email.clone().map(Some),
                        gender: up.gender.clone().map(Some),
                        political_view: up.political_view.clone().map(Some),
                        religion: up.religion.clone().map(Some),
                        education: up.education.clone().map(Some),
                        primary_language: up.primary_language.clone().map(Some),
                        relationship_status: up.relationship_status.clone().map(Some),
                        x_username: up.x_username.clone().map(Some),
                        facebook_username: up.facebook_username.clone().map(Some),
                        reddit_username: up.reddit_username.clone().map(Some),
                        github_username: up.github_username.clone().map(Some),
                        instagram_username: up.instagram_username.clone().map(Some),
                        linkedin_username: up.linkedin_username.clone().map(Some),
                        twitch_username: up.twitch_username.clone().map(Some),
                        min_offer_amount: up.min_offer_amount.map(Some),
                        username: up.username.clone(),
                        selected_badge_id: up.selected_badge_id.clone().map(Some),
                        paid_messaging_enabled: up.paid_messaging_enabled,
                        paid_messaging_min_cost: up.paid_messaging_min_cost.map(Some),
                    };
                    let filter = profiles::profile_id
                        .eq(&up.profile_id)
                        .or(profiles::owner_address.eq(&up.owner_address));
                    total += diesel::update(profiles::table)
                        .filter(filter)
                        .set(set)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::BlockedEvent(ev) => {
                    total += diesel::insert_into(blocked_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::BlockedProfile(bp) => {
                    let last_blocked_at = bp.last_blocked_at;
                    let blocked_profile_id = bp.blocked_profile_id.clone();
                    let blocked_username = bp.blocked_username.clone();
                    let blocked_display_name = bp.blocked_display_name.clone();
                    let blocked_profile_photo = bp.blocked_profile_photo.clone();
                    total += diesel::insert_into(blocked_profiles::table)
                        .values(bp)
                        .on_conflict((
                            blocked_profiles::blocker_address,
                            blocked_profiles::blocked_address,
                        ))
                        .do_update()
                        .set((
                            blocked_profiles::blocked_profile_id.eq(blocked_profile_id),
                            blocked_profiles::blocked_username.eq(blocked_username),
                            blocked_profiles::blocked_display_name.eq(blocked_display_name),
                            blocked_profiles::blocked_profile_photo.eq(blocked_profile_photo),
                            blocked_profiles::last_blocked_at.eq(last_blocked_at),
                            blocked_profiles::total_block_count
                                .eq(blocked_profiles::total_block_count + 1),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::BlockedProfileDelete {
                    blocker_address,
                    blocked_address,
                } => {
                    total += diesel::delete(blocked_profiles::table)
                        .filter(blocked_profiles::blocker_address.eq(blocker_address))
                        .filter(blocked_profiles::blocked_address.eq(blocked_address))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileEvent(ev) => {
                    total += diesel::insert_into(profile_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileBadge(badge) => {
                    total += diesel::insert_into(profile_badges::table)
                        .values(badge)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProfileBadgeRevoke {
                    profile_id,
                    badge_id,
                    revoked_at,
                    revoked_by,
                } => {
                    total += diesel::update(profile_badges::table)
                        .filter(profile_badges::profile_id.eq(profile_id))
                        .filter(profile_badges::badge_id.eq(badge_id))
                        .filter(profile_badges::revoked.eq(false))
                        .set((
                            profile_badges::revoked.eq(true),
                            profile_badges::revoked_at.eq(Some(*revoked_at)),
                            profile_badges::revoked_by.eq(Some(revoked_by.clone())),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SocialGraphRelationship(rel) => {
                    total += diesel::insert_into(social_graph_relationships::table)
                        .values(rel)
                        .on_conflict((
                            social_graph_relationships::follower_address,
                            social_graph_relationships::following_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SocialGraphEvent(ev) => {
                    total += diesel::insert_into(social_graph_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::SocialGraphUnfollow {
                    follower_address,
                    following_address,
                } => {
                    total += diesel::delete(social_graph_relationships::table)
                        .filter(social_graph_relationships::follower_address.eq(follower_address))
                        .filter(social_graph_relationships::following_address.eq(following_address))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::GovernanceRegistry(reg) => {
                    let registry_id = reg.registry_id.clone();
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
                            governance_registries::delegate_term_epochs.eq(delegate_term_epochs),
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
                SocialEventRow::GovernanceRegistryUpdate(up) => {
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
                SocialEventRow::NominatedDelegate(n) => {
                    total += diesel::insert_into(nominated_delegates::table)
                        .values(n)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Delegate(d) => {
                    total += diesel::insert_into(delegates::table)
                        .values(d)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Proposal(p) => {
                    total += diesel::insert_into(proposals::table)
                        .values(p)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ProposalUpdate {
                    proposal_id,
                    set,
                    governance_event,
                } => {
                    total += diesel::update(proposals::table)
                        .filter(proposals::id.eq(proposal_id))
                        .set(set)
                        .execute(conn)
                        .await?;
                    if let Some((event_type, event_data, event_id)) = governance_event {
                        let proposal_type: Option<i16> = proposals::table
                            .filter(proposals::id.eq(&proposal_id))
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
                SocialEventRow::DelegateRating(r) => {
                    total += diesel::insert_into(delegate_ratings::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::DelegateVote(v) => {
                    total += diesel::insert_into(delegate_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::CommunityVote(v) => {
                    total += diesel::insert_into(community_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::RewardDistribution(r) => {
                    total += diesel::insert_into(reward_distributions::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::GovernanceEvent(ev) => {
                    total += diesel::insert_into(governance_events::table)
                        .values(ev)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::GovernanceEventFromProposal {
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
                SocialEventRow::AnonymousVote(v) => {
                    total += diesel::insert_into(anonymous_votes::table)
                        .values(v)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::VoteDecryptionFailure(f) => {
                    total += diesel::insert_into(vote_decryption_failures::table)
                        .values(f)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Post(p) => {
                    total += diesel::insert_into(posts::table)
                        .values(p)
                        .on_conflict((posts::id, posts::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Comment(c) => {
                    total += diesel::insert_into(comments::table)
                        .values(c)
                        .on_conflict((comments::id, comments::time))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Reaction(r) => {
                    total += diesel::insert_into(reactions::table)
                        .values(r)
                        .on_conflict_do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ReactionCount(rc) => {
                    total += diesel::insert_into(reaction_counts::table)
                        .values(rc)
                        .on_conflict((reaction_counts::object_id, reaction_counts::reaction_text))
                        .do_update()
                        .set(reaction_counts::count.eq(reaction_counts::count + 1))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::RemoveReaction {
                    object_id,
                    user_address,
                    reaction_text,
                    is_post: _,
                } => {
                    let _ = diesel::delete(reactions::table)
                        .filter(reactions::object_id.eq(object_id))
                        .filter(reactions::user_address.eq(user_address))
                        .filter(reactions::reaction_text.eq(reaction_text))
                        .execute(conn)
                        .await;
                    let _ = diesel::update(reaction_counts::table)
                        .filter(reaction_counts::object_id.eq(object_id))
                        .filter(reaction_counts::reaction_text.eq(reaction_text))
                        .set(reaction_counts::count.eq(reaction_counts::count - 1))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::Repost(r) => {
                    total += diesel::insert_into(reposts::table)
                        .values(r)
                        .on_conflict(reposts::id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Tip(t) => {
                    total += diesel::insert_into(tips::table)
                        .values(t)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::ModerationEvent(m) => {
                    total += diesel::insert_into(posts_moderation_events::table)
                        .values(m)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::Report(r) => {
                    total += diesel::insert_into(posts_reports::table)
                        .values(r)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::DeletionEvent(d) => {
                    total += diesel::insert_into(posts_deletion_events::table)
                        .values(d)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PostCommentCountIncrement { post_id, delta } => {
                    let _ = diesel::update(posts::table)
                        .filter(posts::post_id.eq(post_id))
                        .set(posts::comment_count.eq(posts::comment_count + delta))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::Platform(p) => {
                    total += diesel::insert_into(platforms::table)
                        .values(p)
                        .on_conflict(platforms::platform_id)
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformUpdate {
                    platform_id,
                    name,
                    tagline,
                    description,
                    terms_of_service,
                    privacy_policy,
                    platform_names,
                    links,
                    status,
                    release_date,
                    shutdown_date,
                    updated_at,
                    primary_category,
                    secondary_category,
                } => {
                    total += diesel::update(platforms::table)
                        .filter(platforms::platform_id.eq(platform_id))
                        .set((
                            platforms::name.eq(name),
                            platforms::tagline.eq(tagline),
                            platforms::description.eq(description),
                            platforms::terms_of_service.eq(terms_of_service),
                            platforms::privacy_policy.eq(privacy_policy),
                            platforms::platform_names.eq(platform_names),
                            platforms::links.eq(links),
                            platforms::status.eq(status),
                            platforms::release_date.eq(release_date),
                            platforms::shutdown_date.eq(shutdown_date),
                            platforms::updated_at.eq(updated_at),
                            platforms::primary_category.eq(primary_category),
                            platforms::secondary_category.eq(secondary_category),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformApprovalChange {
                    platform_id,
                    is_approved,
                    approved_by,
                    changed_at,
                } => {
                    total += diesel::update(platforms::table)
                        .filter(platforms::platform_id.eq(platform_id))
                        .set((
                            platforms::is_approved.eq(is_approved),
                            platforms::approval_changed_at.eq(Some(changed_at)),
                            platforms::approved_by.eq(Some(approved_by)),
                            platforms::updated_at.eq(changed_at),
                        ))
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformModerator(m) => {
                    total += diesel::insert_into(platform_moderators::table)
                        .values(m)
                        .on_conflict((
                            platform_moderators::platform_id,
                            platform_moderators::moderator_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformModeratorRemove {
                    platform_id,
                    moderator_address,
                } => {
                    let _ = diesel::delete(platform_moderators::table)
                        .filter(platform_moderators::platform_id.eq(platform_id))
                        .filter(platform_moderators::moderator_address.eq(moderator_address))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::PlatformBlockedProfile(b) => {
                    total += diesel::insert_into(platform_blocked_profiles::table)
                        .values(b)
                        .on_conflict((
                            platform_blocked_profiles::platform_id,
                            platform_blocked_profiles::wallet_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformBlockedProfileRemove {
                    platform_id,
                    wallet_address,
                } => {
                    let _ = diesel::delete(platform_blocked_profiles::table)
                        .filter(platform_blocked_profiles::platform_id.eq(platform_id))
                        .filter(platform_blocked_profiles::wallet_address.eq(wallet_address))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::PlatformMembership(m) => {
                    total += diesel::insert_into(platform_memberships::table)
                        .values(m)
                        .on_conflict((
                            platform_memberships::platform_id,
                            platform_memberships::wallet_address,
                        ))
                        .do_nothing()
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformMembershipRemove {
                    platform_id,
                    wallet_address,
                } => {
                    let _ = diesel::delete(platform_memberships::table)
                        .filter(platform_memberships::platform_id.eq(platform_id))
                        .filter(platform_memberships::wallet_address.eq(wallet_address))
                        .execute(conn)
                        .await;
                }
                SocialEventRow::PlatformTokenAirdrop(a) => {
                    total += diesel::insert_into(platform_token_airdrops::table)
                        .values(a)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformEvent(e) => {
                    total += diesel::insert_into(platform_events::table)
                        .values(e)
                        .execute(conn)
                        .await?;
                }
                SocialEventRow::PlatformDeleted {
                    platform_id,
                    deleted_at,
                } => {
                    total += diesel::update(platforms::table)
                        .filter(platforms::platform_id.eq(platform_id))
                        .set((
                            platforms::deleted_at.eq(Some(deleted_at)),
                            platforms::updated_at.eq(deleted_at),
                        ))
                        .execute(conn)
                        .await?;
                }
            }
        }

        Ok(total)
    }
}
