// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::QueryableByName;
use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text};
use diesel_async::RunQueryDsl;

use myso_indexer_alt_social_schema::models::{
    DelegateRow, GovernanceRegistryRow, GovernanceStatsRow, ProposalRow,
};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

pub(crate) async fn list_proposals(
    conn: &mut Connection<'_>,
    platform_id: Option<&str>,
    status: Option<i16>,
    proposal_type: Option<i16>,
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

    let query = "
        SELECT id, title, description, proposal_type, reference_id, metadata_json, submitter,
               submission_time, delegate_approval_count, delegate_rejection_count,
               community_votes_for, community_votes_against, status, voting_start_time,
               voting_end_time, reward_pool, implemented_description, implementation_time,
               rescind_time, anonymous_voters_count
        FROM (SELECT DISTINCT ON (id) * FROM proposals ORDER BY id, time DESC) p
        WHERE ($1::smallint IS NULL OR status = $1)
          AND ($2::smallint IS NULL OR proposal_type = $2)
        ORDER BY submission_time DESC
        LIMIT $3 OFFSET $4
    ";

    let results = diesel::sql_query(query)
        .bind::<Nullable<SmallInt>, _>(status)
        .bind::<Nullable<SmallInt>, _>(effective_proposal_type)
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
               rescind_time, anonymous_voters_count
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
               proposal_submission_cost, max_votes_per_user, voting_period_ms, quorum_votes
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
               proposal_submission_cost, max_votes_per_user, voting_period_ms, quorum_votes
        FROM governance_registries
        WHERE registry_type = $1
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
               proposal_submission_cost, max_votes_per_user, voting_period_ms, quorum_votes
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
