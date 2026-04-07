// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{
    AnonymousVoteRow, AnonymousVotingStatsRow, AnonymousVotingTrendRow, CommunityVoteRow,
    DelegateRatingRow, DelegateRow, DelegateVoteRow, GovernanceEventRow, GovernanceRegistryRow,
    GovernanceStatsRow, NominatedDelegateRow, ProposalRow, RewardDistributionRow,
    VoteDecryptionFailureRow,
};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

/// SQL for listing latest proposal rows with optional status, proposal type, and submitter filters.
/// Bind order: status ($1), proposal_type ($2), submitter ($3), limit ($4), offset ($5).
/// When `platform_id` is provided to `list_proposals`, `$2` is replaced by the platform's registry type (or -1), not the caller's `proposal_type`.
const LIST_PROPOSALS_SQL: &str = "
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

/// List proposals from the social DB. When `platform_id` is set, filters by that platform's governance
/// registry type and ignores `proposal_type`.
pub(crate) async fn list_proposals(
    conn: &mut Connection<'_>,
    platform_id: Option<&str>,
    status: Option<i16>,
    proposal_type: Option<i16>,
    submitter: Option<&str>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<ProposalRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let effective_proposal_type = if let Some(pid) = platform_id {
        let registry_type = resolve_registry_type_for_platform(conn, pid).await?;
        registry_type.or(Some(-1))
    } else {
        proposal_type
    };

    let results = diesel::sql_query(LIST_PROPOSALS_SQL)
        .bind::<Nullable<SmallInt>, _>(status)
        .bind::<Nullable<SmallInt>, _>(effective_proposal_type)
        .bind::<Nullable<Text>, _>(submitter)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<ProposalRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

async fn resolve_registry_type_for_platform(
    conn: &mut Connection<'_>,
    platform_id: &str,
) -> anyhow::Result<Option<i16>> {
    #[derive(QueryableByName)]
    struct PlatformRegistry {
        #[diesel(sql_type = Nullable<Text>)]
        governance_registry_id: Option<String>,
    }

    let platform = diesel::sql_query(
        "SELECT governance_registry_id FROM platforms WHERE platform_id = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind::<Text, _>(platform_id)
    .get_result::<PlatformRegistry>(conn)
    .await
    .optional()?;

    let Some(reg_id) = platform.and_then(|p| p.governance_registry_id) else {
        return Ok(None);
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

    Ok(reg.map(|r| r.registry_type))
}

pub(crate) async fn get_proposal_by_id(
    conn: &mut Connection<'_>,
    id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<ProposalRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

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
        .get_result::<ProposalRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_delegates(
    conn: &mut Connection<'_>,
    registry_type: Option<i16>,
    is_active: Option<bool>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<DelegateRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

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
        .load::<DelegateRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_delegate_by_address(
    conn: &mut Connection<'_>,
    address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<DelegateRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT address, registry_type, upvotes, downvotes, proposals_reviewed, proposals_submitted,
               sided_winning_proposals, sided_losing_proposals, term_start, term_end, is_active
        FROM (SELECT DISTINCT ON (address, registry_type) * FROM delegates ORDER BY address, registry_type, time DESC) d
        WHERE address = $1
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .get_result::<DelegateRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_governance_registries(
    conn: &mut Connection<'_>,
    registry_type: Option<i16>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<GovernanceRegistryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT registry_type, registry_id, delegate_count, delegate_term_epochs,
               proposal_submission_cost, min_on_chain_age_days, max_votes_per_user,
               quadratic_base_cost, voting_period_ms, quorum_votes
        FROM governance_registries
        WHERE ($1::smallint IS NULL OR registry_type = $1)
    ";

    let results = diesel::sql_query(query)
        .bind::<Nullable<SmallInt>, _>(registry_type)
        .load::<GovernanceRegistryRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_governance_registry_by_type(
    conn: &mut Connection<'_>,
    registry_type: i16,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<GovernanceRegistryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT registry_type, registry_id, delegate_count, delegate_term_epochs,
               proposal_submission_cost, min_on_chain_age_days, max_votes_per_user,
               quadratic_base_cost, voting_period_ms, quorum_votes
        FROM governance_registries
        WHERE registry_type = $1
        ORDER BY registry_id ASC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .bind::<SmallInt, _>(registry_type)
        .get_result::<GovernanceRegistryRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_governance_registry_by_platform_id(
    conn: &mut Connection<'_>,
    platform_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<GovernanceRegistryRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    #[derive(QueryableByName)]
    struct PlatformRegistry {
        #[diesel(sql_type = Nullable<Text>)]
        governance_registry_id: Option<String>,
    }

    let platform = diesel::sql_query(
        "SELECT governance_registry_id FROM platforms WHERE platform_id = $1 AND deleted_at IS NULL LIMIT 1",
    )
    .bind::<Text, _>(platform_id)
    .get_result::<PlatformRegistry>(conn)
    .await
    .optional()?;

    let Some(reg_id) = platform.and_then(|p| p.governance_registry_id) else {
        metrics.requests_succeeded.inc();
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
        .get_result::<GovernanceRegistryRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_proposal_delegate_votes(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<DelegateVoteRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT proposal_id, delegate_address, approve, vote_time, reason
        FROM delegate_votes
        WHERE proposal_id = $1
        ORDER BY vote_time DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<DelegateVoteRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_proposal_community_votes_count(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<i64> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        count: i64,
    }

    let query = "
        SELECT COUNT(*)::bigint FROM community_votes WHERE proposal_id = $1
    ";
    let row = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .get_result::<CountRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(row.count)
}

pub(crate) async fn get_proposal_community_votes(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<CommunityVoteRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

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
        .load::<CommunityVoteRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_proposal_reward_distributions(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<RewardDistributionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT proposal_id, recipient_address, amount, distribution_time, distribution_type
        FROM reward_distributions
        WHERE proposal_id = $1
        ORDER BY distribution_time DESC
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .load::<RewardDistributionRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_delegate_proposals(
    conn: &mut Connection<'_>,
    address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<ProposalRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

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
        .load::<ProposalRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_delegate_ratings(
    conn: &mut Connection<'_>,
    address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<DelegateRatingRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT target_address, voter_address, registry_type, is_active_delegate, vote_kind, rated_at
        FROM (
            SELECT DISTINCT ON (target_address, voter_address, registry_type, is_active_delegate)
                   target_address, voter_address, registry_type, is_active_delegate, vote_kind, rated_at
            FROM delegate_ratings
            WHERE target_address = $1
            ORDER BY target_address, voter_address, registry_type, is_active_delegate, time DESC
        ) latest
        ORDER BY rated_at DESC
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .load::<DelegateRatingRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn list_governance_events(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<GovernanceEventRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, event_type, registry_type, event_data, event_id, created_at
        FROM governance_events
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    ";

    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<GovernanceEventRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_proposal_anonymous_stats(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<AnonymousVotingStatsRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

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
        .get_result::<AnonymousVotingStatsRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_proposal_anonymous_votes(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<AnonymousVoteRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

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
        .load::<AnonymousVoteRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_proposal_decryption_failures(
    conn: &mut Connection<'_>,
    proposal_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<VoteDecryptionFailureRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT proposal_id, voter_address, failure_reason, attempted_at
        FROM vote_decryption_failures
        WHERE proposal_id = $1
        ORDER BY attempted_at DESC
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(proposal_id)
        .load::<VoteDecryptionFailureRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_anonymous_voting_trends(
    conn: &mut Connection<'_>,
    limit: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<AnonymousVotingTrendRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

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
        .load::<AnonymousVotingTrendRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

/// Latest nominees for a single platform governance registry. Binds: registry_type ($1), status ($2),
/// governance_registry_id ($3), limit ($4), offset ($5).
const LIST_NOMINATED_DELEGATES_PLATFORM_SQL: &str = "
        SELECT address, registry_type, upvotes, downvotes, scheduled_term_start_epoch,
               nomination_time, status
        FROM (
            SELECT DISTINCT ON (address, registry_type, COALESCE(governance_registry_id, '')) *
            FROM nominated_delegates
            ORDER BY address, registry_type, COALESCE(governance_registry_id, ''), nomination_time DESC, time DESC
        ) n
        WHERE registry_type = $1
          AND ($2::smallint IS NULL OR status = $2)
          AND governance_registry_id = $3
        ORDER BY upvotes DESC
        LIMIT $4 OFFSET $5
    ";

/// Ecosystem / PoC nominees only (`governance_registry_id` NULL). Optional omnibus excludes `registry_type = 2`.
/// Binds: registry_type ($1), status ($2), omnibus_exclude_type_2 ($3), limit ($4), offset ($5).
const LIST_NOMINATED_DELEGATES_LEGACY_SQL: &str = "
        SELECT address, registry_type, upvotes, downvotes, scheduled_term_start_epoch,
               nomination_time, status
        FROM (
            SELECT DISTINCT ON (address, registry_type, COALESCE(governance_registry_id, '')) *
            FROM nominated_delegates
            ORDER BY address, registry_type, COALESCE(governance_registry_id, ''), nomination_time DESC, time DESC
        ) n
        WHERE ($1::smallint IS NULL OR registry_type = $1)
          AND ($2::smallint IS NULL OR status = $2)
          AND governance_registry_id IS NULL
          AND ($3::bool = false OR registry_type <> 2)
        ORDER BY upvotes DESC
        LIMIT $4 OFFSET $5
    ";

pub(crate) async fn list_nominated_delegates(
    conn: &mut Connection<'_>,
    platform_id: Option<&str>,
    registry_type: Option<i16>,
    status: Option<i16>,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<NominatedDelegateRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let results = if let Some(pid) = platform_id {
        let Some(reg) = get_governance_registry_by_platform_id(conn, pid, metrics).await? else {
            metrics.requests_succeeded.inc();
            return Ok(vec![]);
        };
        diesel::sql_query(LIST_NOMINATED_DELEGATES_PLATFORM_SQL)
            .bind::<SmallInt, _>(reg.registry_type)
            .bind::<Nullable<SmallInt>, _>(status)
            .bind::<Text, _>(&reg.registry_id)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<NominatedDelegateRow>(conn)
            .await?
    } else {
        let omnibus = registry_type.is_none();
        diesel::sql_query(LIST_NOMINATED_DELEGATES_LEGACY_SQL)
            .bind::<Nullable<SmallInt>, _>(registry_type)
            .bind::<Nullable<SmallInt>, _>(status)
            .bind::<Bool, _>(omnibus)
            .bind::<BigInt, _>(limit)
            .bind::<BigInt, _>(offset)
            .load::<NominatedDelegateRow>(conn)
            .await?
    };

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_governance_stats_by_registry_type(
    conn: &mut Connection<'_>,
    registry_type: i16,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<GovernanceStatsRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT registry_type, active_delegates, pending_nominees, submitted_proposals,
               in_review_proposals, voting_proposals, approved_proposals, rejected_proposals,
               implemented_proposals, rescinded_proposals
        FROM governance_stats
        WHERE registry_type = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<SmallInt, _>(registry_type)
        .get_result::<GovernanceStatsRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

#[cfg(test)]
mod list_proposals_sql_tests {
    use super::LIST_NOMINATED_DELEGATES_LEGACY_SQL;
    use super::LIST_NOMINATED_DELEGATES_PLATFORM_SQL;
    use super::LIST_PROPOSALS_SQL;

    #[test]
    fn list_proposals_sql_filters_proposal_type_at_second_bind() {
        assert!(
            LIST_PROPOSALS_SQL.contains("proposal_type = $2"),
            "registryType / proposal_type filter must use bind $2 for list_proposals"
        );
    }

    #[test]
    fn list_nominated_delegates_platform_sql_binds_registry_type_first() {
        assert!(
            LIST_NOMINATED_DELEGATES_PLATFORM_SQL.contains("registry_type = $1")
                && LIST_NOMINATED_DELEGATES_PLATFORM_SQL.contains("governance_registry_id = $3"),
            "platform nominee list must filter by registry type and governance_registry_id"
        );
    }

    #[test]
    fn list_nominated_delegates_legacy_sql_requires_null_governance_registry_id() {
        assert!(
            LIST_NOMINATED_DELEGATES_LEGACY_SQL.contains("governance_registry_id IS NULL"),
            "legacy nominee list must only include ecosystem/PoC rows"
        );
    }
}
