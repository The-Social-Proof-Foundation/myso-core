// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text};
use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_pg_db::Db;

use crate::error::SocialError;
use crate::reader::types::*;

pub(crate) async fn list_proposals(
    db: &Db,
    limit: i64,
    offset: i64,
    status: Option<i16>,
    proposal_type: Option<i16>,
    platform_id: Option<&str>,
    submitter: Option<&str>,
) -> Result<Vec<ProposalRow>, SocialError> {
    let mut conn = db.connect().await?;

    let effective_proposal_type = if let Some(pid) = platform_id {
        resolve_registry_type_for_platform(&mut conn, pid).await?
    } else {
        proposal_type
    };

    let query = "
        SELECT id, title, description, proposal_type, reference_id, metadata_json, submitter,
               submission_time, delegate_approval_count, delegate_rejection_count,
               community_votes_for, community_votes_against, status, voting_start_time,
               voting_end_time, reward_pool, implemented_description, implementation_time,
               rescind_time, rejection_time, anonymous_voters_count
        FROM (SELECT DISTINCT ON (id) * FROM proposals ORDER BY id, time DESC) p
        WHERE ($1::smallint IS NULL OR status = $1)
          AND ($2::smallint IS NULL OR proposal_type = $2)
          AND ($3::text IS NULL OR submitter = $3)
        ORDER BY submission_time DESC
        LIMIT $4 OFFSET $5
    ";
    let results = diesel::sql_query(query)
        .bind::<Nullable<SmallInt>, _>(status)
        .bind::<Nullable<SmallInt>, _>(effective_proposal_type)
        .bind::<Nullable<Text>, _>(submitter)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProposalRow>(&mut conn)
        .await?;
    Ok(results)
}

async fn resolve_registry_type_for_platform<C>(
    conn: &mut C,
    platform_id: &str,
) -> Result<Option<i16>, SocialError>
where
    C: diesel_async::AsyncConnection<Backend = diesel::pg::Pg> + Send,
{
    #[derive(QueryableByName)]
    struct PlatformRegistry {
        #[diesel(sql_type = Nullable<Text>)]
        governance_registry_id: Option<String>,
    }

    use diesel::OptionalExtension;
    let platform = diesel::sql_query(
        "SELECT governance_registry_id FROM platforms WHERE platform_id = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind::<Text, _>(platform_id)
    .get_result::<PlatformRegistry>(conn)
    .await
    .optional()?;

    let Some(reg_id) = platform.and_then(|p| p.governance_registry_id) else {
        return Ok(Some(-1));
    };

    #[derive(QueryableByName)]
    struct RegistryType {
        #[diesel(sql_type = SmallInt)]
        registry_type: i16,
    }

    let reg = diesel::sql_query(
        "SELECT registry_type FROM governance_registries WHERE registry_id = $1 LIMIT 1",
    )
    .bind::<Text, _>(reg_id)
    .get_result::<RegistryType>(conn)
    .await
    .optional()?;

    Ok(reg.map(|r| r.registry_type).or(Some(-1)))
}

pub(crate) async fn get_proposal_by_id(
    db: &Db,
    id: &str,
) -> Result<Option<ProposalRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT id, title, description, proposal_type, reference_id, metadata_json, submitter,
               submission_time, delegate_approval_count, delegate_rejection_count,
               community_votes_for, community_votes_against, status, voting_start_time,
               voting_end_time, reward_pool, implemented_description, implementation_time,
               rescind_time, rejection_time, anonymous_voters_count
        FROM proposals
        WHERE id = $1 AND time = (SELECT max(time) FROM proposals WHERE id = $1)
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(id)
        .get_result::<ProposalRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_proposal_delegate_votes(
    db: &Db,
    proposal_id: &str,
) -> Result<Vec<DelegateVoteRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT proposal_id, delegate_address, approve, vote_time, reason
        FROM delegate_votes
        WHERE proposal_id = $1
        ORDER BY vote_time DESC
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .load::<DelegateVoteRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_proposal_community_votes_count(
    db: &Db,
    proposal_id: &str,
) -> Result<i64, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT COUNT(*)::bigint FROM community_votes WHERE proposal_id = $1
    ";
    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        count: i64,
    }
    let row = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .get_result::<CountRow>(&mut conn)
        .await?;
    Ok(row.count)
}

pub(crate) async fn get_proposal_community_votes(
    db: &Db,
    proposal_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<CommunityVoteRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT proposal_id, voter_address, vote_weight, approve, vote_time, vote_cost
        FROM community_votes
        WHERE proposal_id = $1
        ORDER BY vote_time DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<CommunityVoteRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_proposal_reward_distributions(
    db: &Db,
    proposal_id: &str,
) -> Result<Vec<RewardDistributionRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT proposal_id, recipient_address, amount, distribution_time, distribution_type
        FROM reward_distributions
        WHERE proposal_id = $1
        ORDER BY distribution_time DESC
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .load::<RewardDistributionRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_delegates(
    db: &Db,
    limit: i64,
    offset: i64,
    registry_type: Option<i16>,
    is_active: Option<bool>,
) -> Result<Vec<DelegateRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT address, registry_type, upvotes, downvotes, proposals_reviewed, proposals_submitted,
               sided_winning_proposals, sided_losing_proposals, term_start, term_end, is_active
        FROM (SELECT DISTINCT ON (address, registry_type) * FROM delegates ORDER BY address, registry_type, time DESC) d
        WHERE ($1::smallint IS NULL OR registry_type = $1)
          AND ($2::bool IS NULL OR is_active = $2)
        ORDER BY upvotes DESC
        LIMIT $3 OFFSET $4
    ";
    let results = diesel::sql_query(query)
        .bind::<Nullable<SmallInt>, _>(registry_type)
        .bind::<Nullable<Bool>, _>(is_active)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<DelegateRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_delegate_by_address(
    db: &Db,
    address: &str,
) -> Result<Option<DelegateRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT address, registry_type, upvotes, downvotes, proposals_reviewed, proposals_submitted,
               sided_winning_proposals, sided_losing_proposals, term_start, term_end, is_active
        FROM (SELECT DISTINCT ON (address, registry_type) * FROM delegates ORDER BY address, registry_type, time DESC) d
        WHERE address = $1
        LIMIT 1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .get_result::<DelegateRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_delegate_proposals(
    db: &Db,
    address: &str,
) -> Result<Vec<ProposalRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT id, title, description, proposal_type, reference_id, metadata_json, submitter,
               submission_time, delegate_approval_count, delegate_rejection_count,
               community_votes_for, community_votes_against, status, voting_start_time,
               voting_end_time, reward_pool, implemented_description, implementation_time,
               rescind_time, rejection_time, anonymous_voters_count
        FROM proposals
        WHERE id IN (SELECT proposal_id FROM delegate_votes WHERE delegate_address = $1)
          AND time = (SELECT max(time) FROM proposals p2 WHERE p2.id = proposals.id)
        ORDER BY submission_time DESC
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .load::<ProposalRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_delegate_ratings(
    db: &Db,
    address: &str,
) -> Result<Vec<DelegateRatingRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT target_address, voter_address, registry_type, is_active_delegate, upvote, rated_at
        FROM (
            SELECT DISTINCT ON (target_address, voter_address, registry_type, is_active_delegate)
                   target_address, voter_address, registry_type, is_active_delegate, upvote, rated_at
            FROM delegate_ratings
            WHERE target_address = $1
            ORDER BY target_address, voter_address, registry_type, is_active_delegate, time DESC
        ) latest
        ORDER BY rated_at DESC
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .load::<DelegateRatingRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_nominees(
    db: &Db,
    limit: i64,
    offset: i64,
    registry_type: Option<i16>,
    status: Option<i16>,
) -> Result<Vec<NominatedDelegateRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT address, registry_type, upvotes, downvotes, scheduled_term_start_epoch,
               nomination_time, status
        FROM (
            SELECT DISTINCT ON (address, registry_type) *
            FROM nominated_delegates
            ORDER BY address, registry_type, nomination_time DESC, time DESC
        ) n
        WHERE ($1::smallint IS NULL OR registry_type = $1)
          AND ($2::smallint IS NULL OR status = $2)
        ORDER BY upvotes DESC
        LIMIT $3 OFFSET $4
    ";
    let results = diesel::sql_query(query)
        .bind::<Nullable<SmallInt>, _>(registry_type)
        .bind::<Nullable<SmallInt>, _>(status)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<NominatedDelegateRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_governance_registries(
    db: &Db,
) -> Result<Vec<GovernanceRegistryRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT registry_type, registry_id, delegate_count, delegate_term_epochs,
               proposal_submission_cost, min_on_chain_age_days, max_votes_per_user,
               quadratic_base_cost, voting_period_ms, quorum_votes
        FROM governance_registries
    ";
    let results = diesel::sql_query(query)
        .load::<GovernanceRegistryRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_governance_registry_by_type(
    db: &Db,
    registry_type: i16,
) -> Result<Option<GovernanceRegistryRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT registry_type, registry_id, delegate_count, delegate_term_epochs,
               proposal_submission_cost, min_on_chain_age_days, max_votes_per_user,
               quadratic_base_cost, voting_period_ms, quorum_votes
        FROM governance_registries
        WHERE registry_type = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<SmallInt, _>(registry_type)
        .get_result::<GovernanceRegistryRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_governance_registry_by_platform_id(
    db: &Db,
    platform_id: &str,
) -> Result<Option<GovernanceRegistryRow>, SocialError> {
    let mut conn = db.connect().await?;

    #[derive(QueryableByName)]
    struct PlatformRegistry {
        #[diesel(sql_type = Nullable<Text>)]
        governance_registry_id: Option<String>,
    }

    let platform = diesel::sql_query(
        "SELECT governance_registry_id FROM platforms WHERE platform_id = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind::<Text, _>(platform_id)
    .get_result::<PlatformRegistry>(&mut conn)
    .await
    .optional()?;

    let Some(reg_id) = platform.and_then(|p| p.governance_registry_id) else {
        return Ok(None);
    };

    let query = "
        SELECT registry_type, registry_id, delegate_count, delegate_term_epochs,
               proposal_submission_cost, min_on_chain_age_days, max_votes_per_user,
               quadratic_base_cost, voting_period_ms, quorum_votes
        FROM governance_registries
        WHERE registry_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(reg_id)
        .get_result::<GovernanceRegistryRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn list_governance_events(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<GovernanceEventRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT id, event_type, registry_type, event_data, event_id, created_at
        FROM governance_events
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<GovernanceEventRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_proposal_anonymous_stats(
    db: &Db,
    proposal_id: &str,
) -> Result<Option<AnonymousVotingStatsRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT
            COUNT(*)::bigint as total_anonymous_votes,
            COUNT(*) FILTER (WHERE decryption_status = 1)::bigint as successfully_decrypted,
            COUNT(*) FILTER (WHERE decryption_status = 2)::bigint as failed_decryptions,
            COUNT(*) FILTER (WHERE decrypted_vote = 1)::bigint as anonymous_votes_for,
            COUNT(*) FILTER (WHERE decrypted_vote = 0)::bigint as anonymous_votes_against,
            COUNT(*) FILTER (WHERE decryption_status = 0)::bigint as pending_decryption
        FROM anonymous_votes
        WHERE proposal_id = $1
    ";
    let result = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .get_result::<AnonymousVotingStatsRow>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

pub(crate) async fn get_proposal_anonymous_votes(
    db: &Db,
    proposal_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<AnonymousVoteRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT proposal_id, voter_address, submitted_at, decryption_status, processing_success
        FROM anonymous_votes
        WHERE proposal_id = $1
        ORDER BY submitted_at DESC
        LIMIT $2 OFFSET $3
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<AnonymousVoteRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_proposal_decryption_failures(
    db: &Db,
    proposal_id: &str,
) -> Result<Vec<VoteDecryptionFailureRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT proposal_id, voter_address, failure_reason, attempted_at
        FROM vote_decryption_failures
        WHERE proposal_id = $1
        ORDER BY attempted_at DESC
    ";
    let results = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .load::<VoteDecryptionFailureRow>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_anonymous_voting_trends(
    db: &Db,
    limit: i64,
) -> Result<Vec<AnonymousVotingTrendRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT day::date as day, SUM(total_anonymous_votes)::bigint as total_votes,
               SUM(successfully_decrypted)::bigint as successful_decryptions,
               SUM(failed_decryptions)::bigint as failed_decryptions
        FROM anonymous_voting_daily_stats
        GROUP BY day
        ORDER BY day DESC
        LIMIT $1
    ";
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .load::<AnonymousVotingTrendRow>(&mut conn)
        .await?;
    Ok(results)
}
