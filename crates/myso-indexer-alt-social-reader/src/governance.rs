// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{Array, BigInt, Bool, Nullable, SmallInt, Text, Timestamptz};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{
    AnonymousVoteRow, AnonymousVotingStatsRow, AnonymousVotingTrendRow, CommunityVoteRow,
    DelegateRatingRow, DelegateRow, DelegateVoteRow, GovernanceEventRow, GovernanceRegistryRow,
    GovernanceStatsRow, NominatedDelegateRow, ProposalRow, RewardDistributionRow,
    VoteDecryptionFailureRow,
};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

/// Target row for batch lookup of the viewer's latest delegate or nominee rating (`delegate_ratings`).
#[derive(Debug, Clone)]
pub struct DelegateRatingViewerTarget {
    pub target_address: String,
    pub registry_type: i16,
    pub governance_registry_id: Option<String>,
    pub is_active_delegate: bool,
}

/// Stable key for [`DelegateRatingViewerTarget`], aligned with list row scope (registry + platform DAO).
pub fn delegate_rating_viewer_lookup_key(t: &DelegateRatingViewerTarget) -> String {
    format!(
        "{}|{}|{}|{}",
        t.target_address,
        t.registry_type,
        t.governance_registry_id.as_deref().unwrap_or(""),
        t.is_active_delegate as u8
    )
}

fn delegate_rating_key_from_sql_row(
    target_address: &str,
    registry_type: i16,
    governance_registry_id: &str,
    is_active_delegate: bool,
) -> String {
    format!(
        "{}|{}|{}|{}",
        target_address,
        registry_type,
        governance_registry_id,
        is_active_delegate as u8
    )
}

/// SQL for listing latest proposal rows with optional status, proposal type, submitter, and platform registry filters.
/// Bind order: status ($1), proposal_type ($2), submitter ($3), limit ($4), offset ($5), governance_registry_id ($6).
/// When `platform_id` is provided to `list_proposals`, `$2` is the platform's registry type (or -1) and `$6` is the platform's on-chain registry id.
const LIST_PROPOSALS_SQL: &str = "
        SELECT id, title, description, proposal_type, reference_id, metadata_json, submitter,
               submission_time, delegate_approval_count, delegate_rejection_count,
               community_votes_for, community_votes_against, status, voting_start_time,
               voting_end_time, reward_pool, implemented_description, implementation_time,
               rescind_time, rejection_time, anonymous_voters_count, governance_registry_id
        FROM (SELECT DISTINCT ON (id) * FROM proposals ORDER BY id, time DESC) p
        WHERE ($1::smallint IS NULL OR status = $1)
          AND ($2::smallint IS NULL OR proposal_type = $2)
          AND ($3::text IS NULL OR submitter = $3)
          AND ($6::text IS NULL OR governance_registry_id IS NOT DISTINCT FROM $6)
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

    let (effective_proposal_type, platform_registry_scope) =
        if let Some(pid) = platform_id {
            resolve_platform_proposal_list_scope(conn, pid).await?
        } else {
            (proposal_type, None)
        };

    let results = diesel::sql_query(LIST_PROPOSALS_SQL)
        .bind::<Nullable<SmallInt>, _>(status)
        .bind::<Nullable<SmallInt>, _>(effective_proposal_type)
        .bind::<Nullable<Text>, _>(submitter)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .bind::<Nullable<Text>, _>(platform_registry_scope.as_deref())
        .load::<ProposalRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

async fn resolve_platform_proposal_list_scope(
    conn: &mut Connection<'_>,
    platform_id: &str,
) -> anyhow::Result<(Option<i16>, Option<String>)> {
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
        return Ok((Some(-1), None));
    };

    #[derive(QueryableByName)]
    struct RegistryType {
        #[diesel(sql_type = SmallInt)]
        registry_type: i16,
    }

    let reg = diesel::sql_query(
        "SELECT registry_type FROM governance_registries WHERE registry_id = $1 LIMIT 1",
    )
    .bind::<Text, _>(&reg_id)
    .get_result::<RegistryType>(conn)
    .await
    .optional()?;

    let Some(r) = reg else {
        return Ok((Some(-1), None));
    };

    Ok((Some(r.registry_type), Some(reg_id)))
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
               rescind_time, rejection_time, anonymous_voters_count, governance_registry_id
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
        SELECT address, registry_type, governance_registry_id, upvotes, downvotes, proposals_reviewed, proposals_submitted,
               sided_winning_proposals, sided_losing_proposals, term_start, term_end, is_active
        FROM (SELECT DISTINCT ON (address, registry_type, COALESCE(governance_registry_id, '')) * FROM delegates ORDER BY address, registry_type, COALESCE(governance_registry_id, ''), time DESC) d
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
    registry_type: Option<i16>,
    governance_registry_id: Option<&str>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<DelegateRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT address, registry_type, governance_registry_id, upvotes, downvotes, proposals_reviewed, proposals_submitted,
               sided_winning_proposals, sided_losing_proposals, term_start, term_end, is_active
        FROM (SELECT DISTINCT ON (address, registry_type, COALESCE(governance_registry_id, '')) * FROM delegates ORDER BY address, registry_type, COALESCE(governance_registry_id, ''), time DESC) d
        WHERE address = $1
          AND ($2::smallint IS NULL OR registry_type = $2)
          AND ($3::text IS NULL OR governance_registry_id IS NOT DISTINCT FROM $3)
        ORDER BY registry_type, governance_registry_id NULLS FIRST
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(address)
        .bind::<Nullable<SmallInt>, _>(registry_type)
        .bind::<Nullable<Text>, _>(governance_registry_id)
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
               rescind_time, rejection_time, anonymous_voters_count, governance_registry_id
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
        SELECT target_address, voter_address, registry_type, governance_registry_id, is_active_delegate, vote_kind, rated_at
        FROM (
            SELECT DISTINCT ON (target_address, voter_address, registry_type, COALESCE(governance_registry_id, ''), is_active_delegate)
                   target_address, voter_address, registry_type, governance_registry_id, is_active_delegate, vote_kind, rated_at
            FROM delegate_ratings
            WHERE target_address = $1
            ORDER BY target_address, voter_address, registry_type, COALESCE(governance_registry_id, ''), is_active_delegate, time DESC
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

/// Latest `vote_kind` per target row for any `voter_address` in `viewer_refs`, batched in one query.
pub(crate) async fn batch_viewer_latest_delegate_rating_vote_kind(
    conn: &mut Connection<'_>,
    viewer_refs: &[String],
    targets: &[DelegateRatingViewerTarget],
    metrics: &DbReaderMetrics,
) -> anyhow::Result<HashMap<String, i16>> {
    if targets.is_empty()
        || viewer_refs.is_empty()
        || viewer_refs.iter().all(|s| s.is_empty())
    {
        return Ok(HashMap::new());
    }
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let addrs: Vec<String> = targets.iter().map(|t| t.target_address.clone()).collect();
    let rts: Vec<i16> = targets.iter().map(|t| t.registry_type).collect();
    let gids: Vec<String> = targets
        .iter()
        .map(|t| t.governance_registry_id.clone().unwrap_or_default())
        .collect();
    let active_i: Vec<i16> = targets
        .iter()
        .map(|t| if t.is_active_delegate { 1 } else { 0 })
        .collect();
    let refs: Vec<String> = viewer_refs.to_vec();

    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        target_address: String,
        #[diesel(sql_type = SmallInt)]
        registry_type: i16,
        #[diesel(sql_type = Text)]
        governance_registry_id: String,
        #[diesel(sql_type = Bool)]
        is_active_delegate: bool,
        #[diesel(sql_type = SmallInt)]
        vote_kind: i16,
    }

    let query = r#"
        WITH targets AS (
            SELECT u.target_address, u.registry_type, u.governance_registry_id, (u.is_active_i <> 0) AS is_active_delegate
            FROM unnest($1::text[], $2::smallint[], $3::text[], $4::smallint[])
                AS u(target_address, registry_type, governance_registry_id, is_active_i)
        ),
        latest AS (
            SELECT DISTINCT ON (dr.target_address, dr.registry_type, COALESCE(dr.governance_registry_id, ''), dr.is_active_delegate)
                dr.target_address, dr.registry_type, dr.governance_registry_id, dr.is_active_delegate, dr.vote_kind
            FROM delegate_ratings dr
            WHERE dr.voter_address = ANY($5::text[])
            ORDER BY dr.target_address, dr.registry_type, COALESCE(dr.governance_registry_id, ''), dr.is_active_delegate, dr.time DESC
        )
        SELECT t.target_address, t.registry_type, t.governance_registry_id, t.is_active_delegate, l.vote_kind
        FROM targets t
        JOIN latest l ON l.target_address = t.target_address
            AND l.registry_type = t.registry_type
            AND l.is_active_delegate = t.is_active_delegate
            AND COALESCE(l.governance_registry_id, '') = COALESCE(NULLIF(t.governance_registry_id, ''), '')
    "#;

    let rows: Vec<Row> = diesel::sql_query(query)
        .bind::<Array<Text>, _>(&addrs)
        .bind::<Array<SmallInt>, _>(&rts)
        .bind::<Array<Text>, _>(&gids)
        .bind::<Array<SmallInt>, _>(&active_i)
        .bind::<Array<Text>, _>(&refs)
        .load(conn)
        .await?;

    let out: HashMap<String, i16> = rows
        .into_iter()
        .map(|r| {
            (
                delegate_rating_key_from_sql_row(
                    &r.target_address,
                    r.registry_type,
                    &r.governance_registry_id,
                    r.is_active_delegate,
                ),
                r.vote_kind,
            )
        })
        .collect();

    metrics.requests_succeeded.inc();
    Ok(out)
}

pub(crate) async fn list_governance_events(
    conn: &mut Connection<'_>,
    limit: i64,
    offset: i64,
    governance_registry_id: Option<&str>,
    registry_type: Option<i16>,
    event_type: Option<&str>,
    proposal_id: Option<&str>,
    created_after: Option<chrono::DateTime<chrono::Utc>>,
    created_before: Option<chrono::DateTime<chrono::Utc>>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<GovernanceEventRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, event_type, registry_type, event_data, event_id, created_at,
               governance_registry_id, proposal_id
        FROM governance_events
        WHERE ($3::text IS NULL OR governance_registry_id IS NOT DISTINCT FROM $3)
          AND ($4::smallint IS NULL OR registry_type = $4)
          AND ($5::text IS NULL OR event_type = $5)
          AND ($6::text IS NULL OR proposal_id = $6)
          AND ($7::timestamptz IS NULL OR created_at >= $7)
          AND ($8::timestamptz IS NULL OR created_at <= $8)
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    ";

    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .bind::<Nullable<Text>, _>(governance_registry_id)
        .bind::<Nullable<SmallInt>, _>(registry_type)
        .bind::<Nullable<Text>, _>(event_type)
        .bind::<Nullable<Text>, _>(proposal_id)
        .bind::<Nullable<Timestamptz>, _>(created_after)
        .bind::<Nullable<Timestamptz>, _>(created_before)
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
        SELECT address, registry_type, governance_registry_id, upvotes, downvotes, scheduled_term_start_epoch,
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
        SELECT address, registry_type, governance_registry_id, upvotes, downvotes, scheduled_term_start_epoch,
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

pub(crate) async fn get_governance_stats_by_registry_id(
    conn: &mut Connection<'_>,
    registry_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<GovernanceStatsRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT registry_id, registry_type, active_delegates, pending_nominees, submitted_proposals,
               in_review_proposals, voting_proposals, approved_proposals, rejected_proposals,
               implemented_proposals, rescinded_proposals
        FROM governance_stats
        WHERE registry_id = $1
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(registry_id)
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
    fn list_proposals_sql_filters_governance_registry_at_sixth_bind() {
        assert!(
            LIST_PROPOSALS_SQL.contains("governance_registry_id IS NOT DISTINCT FROM $6"),
            "platform list must scope proposals by governance_registry_id bind $6"
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

    #[test]
    fn list_nominated_delegates_sql_selects_governance_registry_id() {
        assert!(
            LIST_NOMINATED_DELEGATES_PLATFORM_SQL.contains("governance_registry_id, upvotes"),
            "platform nominee list must return governance_registry_id for viewer rating scope"
        );
        assert!(
            LIST_NOMINATED_DELEGATES_LEGACY_SQL.contains("governance_registry_id, upvotes"),
            "legacy nominee list must return governance_registry_id column (null for ecosystem/PoC)"
        );
    }
}

#[cfg(test)]
mod delegate_rating_viewer_lookup_tests {
    use super::DelegateRatingViewerTarget;
    use super::delegate_rating_viewer_lookup_key;

    #[test]
    fn lookup_key_separates_delegate_and_nominee_same_address() {
        let addr = "0xabc".to_string();
        let d = DelegateRatingViewerTarget {
            target_address: addr.clone(),
            registry_type: 2,
            governance_registry_id: Some("0xdao".to_string()),
            is_active_delegate: true,
        };
        let n = DelegateRatingViewerTarget {
            target_address: addr,
            registry_type: 2,
            governance_registry_id: Some("0xdao".to_string()),
            is_active_delegate: false,
        };
        assert_ne!(
            delegate_rating_viewer_lookup_key(&d),
            delegate_rating_viewer_lookup_key(&n)
        );
    }

    #[test]
    fn lookup_key_separates_platform_daos() {
        let t1 = DelegateRatingViewerTarget {
            target_address: "0xtarget".to_string(),
            registry_type: 2,
            governance_registry_id: Some("0xdao1".to_string()),
            is_active_delegate: true,
        };
        let t2 = DelegateRatingViewerTarget {
            target_address: "0xtarget".to_string(),
            registry_type: 2,
            governance_registry_id: Some("0xdao2".to_string()),
            is_active_delegate: true,
        };
        assert_ne!(
            delegate_rating_viewer_lookup_key(&t1),
            delegate_rating_viewer_lookup_key(&t2)
        );
    }

    #[test]
    fn lookup_key_null_governance_registry_matches_empty_string_scope() {
        let a = DelegateRatingViewerTarget {
            target_address: "0xa".to_string(),
            registry_type: 0,
            governance_registry_id: None,
            is_active_delegate: true,
        };
        let b = DelegateRatingViewerTarget {
            target_address: "0xa".to_string(),
            registry_type: 0,
            governance_registry_id: Some(String::new()),
            is_active_delegate: true,
        };
        assert_eq!(delegate_rating_viewer_lookup_key(&a), delegate_rating_viewer_lookup_key(&b));
    }
}
