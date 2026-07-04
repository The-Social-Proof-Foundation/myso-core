// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::OptionalExtension;
use diesel::sql_types::{BigInt, Text};
use diesel_async::RunQueryDsl;

use crate::metrics::standalone_reader_metrics;
use myso_indexer_alt_social_schema::models::{
    POC_VAULT_LEGACY_AGGREGATE_COIN_TYPE, PocAnalysisResultRow, PocBadgeRow,
    PocBeneficiaryVaultRow, PocConfigRow, PocDisputeRow, PocDisputeVoteRow,
    PocRevenueRedirectionRow, PocVaultClaimRow, PocVaultCoinBalanceRow, PocVaultDepositRow,
};
use myso_pg_db::Connection;

use crate::metrics::DbReaderMetrics;

pub(crate) async fn get_poc_analysis_for_post(
    conn: &mut Connection<'_>,
    post_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocAnalysisResultRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT post_id, similarity_detected, highest_similarity_score, media_type,
               oracle_address, original_creator, analysis_timestamp,
               reasoning, evidence_urls
        FROM (
            SELECT DISTINCT ON (post_id) *
            FROM poc_analysis_results
            WHERE post_id = $1
            ORDER BY post_id, time DESC
        ) sub
    ";

    let result = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .get_result::<PocAnalysisResultRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn get_poc_badges_for_post(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocBadgeRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT badge_id, post_id, media_type, issued_by, issued_at, revoked,
               beneficiary_address, matched_anchor_id, media_index
        FROM (
            SELECT DISTINCT ON (badge_id) *
            FROM poc_badges
            WHERE post_id = $1
            ORDER BY badge_id, time DESC
        ) sub
        WHERE revoked = false
        ORDER BY issued_at DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocBadgeRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_post_revenue_redirections(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocRevenueRedirectionRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT redirection_id, accused_post_id, original_post_id, redirect_percentage,
               similarity_score, created_at, removed
        FROM (
            SELECT DISTINCT ON (redirection_id) *
            FROM poc_revenue_redirections
            WHERE accused_post_id = $1 OR original_post_id = $1
            ORDER BY redirection_id, time DESC
        ) sub
        WHERE removed = false
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocRevenueRedirectionRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_poc_disputes_for_post(
    conn: &mut Connection<'_>,
    post_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocDisputeRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT dispute_id, post_id, disputer, dispute_type, evidence, status,
               resolution, stake_amount, voting_start_ms, voting_end_ms,
               winning_side, total_winning_stake, total_losing_stake,
               submitted_at, resolved_at, dispute_round, effective_dispute_fee,
               required_total_stake_quorum, quorum_met
        FROM (
            SELECT DISTINCT ON (dispute_id) *
            FROM poc_disputes
            WHERE post_id = $1
            ORDER BY dispute_id, time DESC
        ) sub
        ORDER BY submitted_at DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(post_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocDisputeRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_poc_dispute_votes(
    conn: &mut Connection<'_>,
    dispute_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocDisputeVoteRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT dispute_id, voter, vote_choice, stake_amount, voted_at, reward_claimed, reward_amount
        FROM (
            SELECT DISTINCT ON (dispute_id, voter) *
            FROM poc_dispute_votes
            WHERE dispute_id = $1
            ORDER BY dispute_id, voter, time DESC
        ) sub
        ORDER BY voted_at DESC
        LIMIT $2 OFFSET $3
    ";

    let results = diesel::sql_query(query)
        .bind::<Text, _>(dispute_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocDisputeVoteRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(results)
}

pub(crate) async fn get_poc_configuration(
    conn: &mut Connection<'_>,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocConfigRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT image_threshold, video_threshold, audio_threshold,
               revenue_redirect_percentage, dispute_cost,
               min_vote_stake, max_vote_stake, voting_duration_ms,
               max_reasoning_length, max_evidence_urls, max_votes_per_dispute,
               oracle_address, claim_treasury_fee_bps, max_referral_bps,
               video_embedded_audio_redirect_bps,
               dispute_quorum_base_stake,
               dispute_second_round_fee_multiplier_bps,
               dispute_second_round_quorum_multiplier_bps,
               username_beneficiary_join_referral_bps,
               max_disputes_per_post, min_vault_deposit_amount,
               dispute_governance_registry_id,
               updated_by, updated_at, transaction_id, version, time
        FROM poc_configuration
        ORDER BY time DESC
        LIMIT 1
    ";

    let result = diesel::sql_query(query)
        .get_result::<PocConfigRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(result)
}

pub(crate) async fn list_poc_vault_coin_balances_for_vault(
    conn: &mut Connection<'_>,
    vault_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocVaultCoinBalanceRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT vault_id, coin_type, balance, updated_at_ms
        FROM poc_vault_coin_balances
        WHERE vault_id = $1 AND balance <> 0
          AND coin_type <> $2
        ORDER BY coin_type ASC
    ";

    let rows = diesel::sql_query(query)
        .bind::<Text, _>(vault_id)
        .bind::<Text, _>(POC_VAULT_LEGACY_AGGREGATE_COIN_TYPE)
        .load::<PocVaultCoinBalanceRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn get_poc_beneficiary_vault_by_vault_id(
    conn: &mut Connection<'_>,
    vault_id: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocBeneficiaryVaultRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT vault_id, vault_routing_key, updated_at_ms, transaction_id
        FROM (
            SELECT DISTINCT ON (vault_id) *
            FROM poc_beneficiary_vaults
            WHERE vault_id = $1
            ORDER BY vault_id, time DESC
        ) sub
    ";

    let row = diesel::sql_query(query)
        .bind::<Text, _>(vault_id)
        .get_result::<PocBeneficiaryVaultRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(row)
}

pub(crate) async fn get_poc_beneficiary_vault_by_beneficiary_address(
    conn: &mut Connection<'_>,
    beneficiary_address: &str,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Option<PocBeneficiaryVaultRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT vault_id, vault_routing_key, updated_at_ms, transaction_id
        FROM poc_beneficiary_vaults
        WHERE vault_routing_key = $1
        ORDER BY time DESC
        LIMIT 1
    ";

    let row = diesel::sql_query(query)
        .bind::<Text, _>(beneficiary_address)
        .get_result::<PocBeneficiaryVaultRow>(conn)
        .await
        .optional()?;

    metrics.requests_succeeded.inc();
    Ok(row)
}

pub(crate) async fn list_poc_vault_deposits_for_vault(
    conn: &mut Connection<'_>,
    vault_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocVaultDepositRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, vault_id, vault_routing_key, amount, coin_type,
               source_post_id, occurred_at_ms, transaction_id
        FROM poc_vault_deposits
        WHERE vault_id = $1
        ORDER BY occurred_at_ms DESC, id DESC
        LIMIT $2 OFFSET $3
    ";

    let rows = diesel::sql_query(query)
        .bind::<Text, _>(vault_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocVaultDepositRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(rows)
}

pub(crate) async fn list_poc_vault_claims_for_vault(
    conn: &mut Connection<'_>,
    vault_id: &str,
    limit: i64,
    offset: i64,
    metrics: &DbReaderMetrics,
) -> anyhow::Result<Vec<PocVaultClaimRow>> {
    metrics.requests_received.inc();
    let _guard = metrics.latency.start_timer();

    let query = "
        SELECT id, vault_id, vault_routing_key, coin_type, referrer_address,
               treasury_amount, referrer_amount, beneficiary_amount,
               occurred_at_ms, transaction_id, claim_kind, gross_amount
        FROM poc_vault_claims
        WHERE vault_id = $1
        ORDER BY occurred_at_ms DESC, id DESC
        LIMIT $2 OFFSET $3
    ";

    let rows = diesel::sql_query(query)
        .bind::<Text, _>(vault_id)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<PocVaultClaimRow>(conn)
        .await?;

    metrics.requests_succeeded.inc();
    Ok(rows)
}

/// Resolve a PoC beneficiary vault row by on-chain vault object id.
pub async fn get_poc_beneficiary_vault_by_vault_id_for_conn(
    conn: &mut Connection<'_>,
    vault_id: &str,
) -> anyhow::Result<Option<PocBeneficiaryVaultRow>> {
    get_poc_beneficiary_vault_by_vault_id(conn, vault_id, standalone_reader_metrics()).await
}

/// Resolve a PoC beneficiary vault row by beneficiary wallet address.
pub async fn get_poc_beneficiary_vault_by_beneficiary_address_for_conn(
    conn: &mut Connection<'_>,
    beneficiary_address: &str,
) -> anyhow::Result<Option<PocBeneficiaryVaultRow>> {
    get_poc_beneficiary_vault_by_beneficiary_address(
        conn,
        beneficiary_address,
        standalone_reader_metrics(),
    )
    .await
}

pub async fn list_poc_vault_coin_balances_for_vault_for_conn(
    conn: &mut Connection<'_>,
    vault_id: &str,
) -> anyhow::Result<Vec<PocVaultCoinBalanceRow>> {
    list_poc_vault_coin_balances_for_vault(conn, vault_id, standalone_reader_metrics()).await
}

pub async fn list_poc_vault_deposits_for_vault_for_conn(
    conn: &mut Connection<'_>,
    vault_id: &str,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<PocVaultDepositRow>> {
    list_poc_vault_deposits_for_vault(conn, vault_id, limit, offset, standalone_reader_metrics())
        .await
}

pub async fn list_poc_vault_claims_for_vault_for_conn(
    conn: &mut Connection<'_>,
    vault_id: &str,
    limit: i64,
    offset: i64,
) -> anyhow::Result<Vec<PocVaultClaimRow>> {
    list_poc_vault_claims_for_vault(conn, vault_id, limit, offset, standalone_reader_metrics())
        .await
}
