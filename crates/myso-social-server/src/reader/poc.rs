// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Bool, Nullable, SmallInt, Text};
use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::QueryableByName;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::schema::{
    poc_analysis_results, poc_configuration, poc_dispute_votes,
};

use crate::error::SocialError;
use crate::reader::types::{
    PocAnalysisResultRow, PocBadgeRow, PocConfigRow, PocDisputeRow, PocDisputeVoteRow,
    PocRevenueRedirectionRow,
};
use myso_pg_db::Db;

pub(crate) async fn list_poc_badges(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocBadgeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT badge_id, post_id, media_type, issued_by, issued_at, revoked
        FROM (
            SELECT DISTINCT ON (badge_id) *
            FROM poc_badges
            ORDER BY badge_id, time DESC
        ) sub
        WHERE revoked = false
        ORDER BY issued_at DESC
        LIMIT $1 OFFSET $2
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        badge_id: String,
        #[diesel(sql_type = Text)]
        post_id: String,
        #[diesel(sql_type = SmallInt)]
        media_type: i16,
        #[diesel(sql_type = Text)]
        issued_by: String,
        #[diesel(sql_type = BigInt)]
        issued_at: i64,
        #[diesel(sql_type = Bool)]
        revoked: bool,
    }
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(|r| PocBadgeRow {
            badge_id: r.badge_id,
            post_id: r.post_id,
            media_type: r.media_type,
            issued_by: r.issued_by,
            issued_at: r.issued_at,
            revoked: r.revoked,
        })
        .collect())
}

pub(crate) async fn get_poc_badge_by_id(
    db: &Db,
    badge_id: &str,
) -> Result<Option<PocBadgeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT badge_id, post_id, media_type, issued_by, issued_at, revoked
        FROM (
            SELECT DISTINCT ON (badge_id) *
            FROM poc_badges
            WHERE badge_id = $1
            ORDER BY badge_id, time DESC
        ) sub
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        badge_id: String,
        #[diesel(sql_type = Text)]
        post_id: String,
        #[diesel(sql_type = SmallInt)]
        media_type: i16,
        #[diesel(sql_type = Text)]
        issued_by: String,
        #[diesel(sql_type = BigInt)]
        issued_at: i64,
        #[diesel(sql_type = Bool)]
        revoked: bool,
    }
    let result = diesel::sql_query(query)
        .bind::<Text, _>(badge_id)
        .get_result::<Row>(&mut conn)
        .await
        .optional()?;
    Ok(result.map(|r| PocBadgeRow {
        badge_id: r.badge_id,
        post_id: r.post_id,
        media_type: r.media_type,
        issued_by: r.issued_by,
        issued_at: r.issued_at,
        revoked: r.revoked,
    }))
}

pub(crate) async fn list_poc_revenue_redirections(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocRevenueRedirectionRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT redirection_id, accused_post_id, original_post_id, redirect_percentage,
               similarity_score, created_at, removed
        FROM (
            SELECT DISTINCT ON (redirection_id) *
            FROM poc_revenue_redirections
            ORDER BY redirection_id, time DESC
        ) sub
        WHERE removed = false
        ORDER BY created_at DESC
        LIMIT $1 OFFSET $2
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        redirection_id: String,
        #[diesel(sql_type = Text)]
        accused_post_id: String,
        #[diesel(sql_type = Text)]
        original_post_id: String,
        #[diesel(sql_type = BigInt)]
        redirect_percentage: i64,
        #[diesel(sql_type = BigInt)]
        similarity_score: i64,
        #[diesel(sql_type = BigInt)]
        created_at: i64,
        #[diesel(sql_type = Bool)]
        removed: bool,
    }
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(|r| PocRevenueRedirectionRow {
            redirection_id: r.redirection_id,
            accused_post_id: r.accused_post_id,
            original_post_id: r.original_post_id,
            redirect_percentage: r.redirect_percentage,
            similarity_score: r.similarity_score,
            created_at: r.created_at,
            removed: r.removed,
        })
        .collect())
}

pub(crate) async fn list_poc_analysis_results(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocAnalysisResultRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = poc_analysis_results::table
        .order_by(poc_analysis_results::analysis_timestamp.desc())
        .limit(limit)
        .offset(offset)
        .select((
            poc_analysis_results::post_id,
            poc_analysis_results::media_type,
            poc_analysis_results::similarity_detected,
            poc_analysis_results::highest_similarity_score,
            poc_analysis_results::oracle_address,
            poc_analysis_results::analysis_timestamp,
        ))
        .load::<(String, i16, bool, i64, String, i64)>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(
                post_id,
                media_type,
                similarity_detected,
                highest_similarity_score,
                oracle_address,
                analysis_timestamp,
            )| PocAnalysisResultRow {
                post_id,
                media_type,
                similarity_detected,
                highest_similarity_score,
                oracle_address,
                analysis_timestamp,
            },
        )
        .collect())
}

pub(crate) async fn list_poc_disputes(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocDisputeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT dispute_id, post_id, disputer, dispute_type, evidence, status,
               stake_amount, submitted_at, resolved_at
        FROM (
            SELECT DISTINCT ON (dispute_id) *
            FROM poc_disputes
            ORDER BY dispute_id, time DESC
        ) sub
        ORDER BY submitted_at DESC
        LIMIT $1 OFFSET $2
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        dispute_id: String,
        #[diesel(sql_type = Text)]
        post_id: String,
        #[diesel(sql_type = Text)]
        disputer: String,
        #[diesel(sql_type = SmallInt)]
        dispute_type: i16,
        #[diesel(sql_type = Text)]
        evidence: String,
        #[diesel(sql_type = SmallInt)]
        status: i16,
        #[diesel(sql_type = BigInt)]
        stake_amount: i64,
        #[diesel(sql_type = BigInt)]
        submitted_at: i64,
        #[diesel(sql_type = Nullable<BigInt>)]
        resolved_at: Option<i64>,
    }
    let results = diesel::sql_query(query)
        .bind::<BigInt, _>(limit)
        .bind::<BigInt, _>(offset)
        .load::<Row>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(|r| PocDisputeRow {
            dispute_id: r.dispute_id,
            post_id: r.post_id,
            disputer: r.disputer,
            dispute_type: r.dispute_type,
            evidence: r.evidence,
            status: r.status,
            stake_amount: r.stake_amount,
            submitted_at: r.submitted_at,
            resolved_at: r.resolved_at,
        })
        .collect())
}

pub(crate) async fn get_poc_dispute_by_id(
    db: &Db,
    dispute_id: &str,
) -> Result<Option<PocDisputeRow>, SocialError> {
    let mut conn = db.connect().await?;
    let query = "
        SELECT dispute_id, post_id, disputer, dispute_type, evidence, status,
               stake_amount, submitted_at, resolved_at
        FROM (
            SELECT DISTINCT ON (dispute_id) *
            FROM poc_disputes
            WHERE dispute_id = $1
            ORDER BY dispute_id, time DESC
        ) sub
    ";
    #[derive(QueryableByName)]
    struct Row {
        #[diesel(sql_type = Text)]
        dispute_id: String,
        #[diesel(sql_type = Text)]
        post_id: String,
        #[diesel(sql_type = Text)]
        disputer: String,
        #[diesel(sql_type = SmallInt)]
        dispute_type: i16,
        #[diesel(sql_type = Text)]
        evidence: String,
        #[diesel(sql_type = SmallInt)]
        status: i16,
        #[diesel(sql_type = BigInt)]
        stake_amount: i64,
        #[diesel(sql_type = BigInt)]
        submitted_at: i64,
        #[diesel(sql_type = Nullable<BigInt>)]
        resolved_at: Option<i64>,
    }
    let result = diesel::sql_query(query)
        .bind::<Text, _>(dispute_id)
        .get_result::<Row>(&mut conn)
        .await
        .optional()?;
    Ok(result.map(|r| PocDisputeRow {
        dispute_id: r.dispute_id,
        post_id: r.post_id,
        disputer: r.disputer,
        dispute_type: r.dispute_type,
        evidence: r.evidence,
        status: r.status,
        stake_amount: r.stake_amount,
        submitted_at: r.submitted_at,
        resolved_at: r.resolved_at,
    }))
}

pub(crate) async fn get_poc_dispute_votes(
    db: &Db,
    dispute_id: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocDisputeVoteRow>, SocialError> {
    let mut conn = db.connect().await?;
    let results = poc_dispute_votes::table
        .filter(poc_dispute_votes::dispute_id.eq(dispute_id))
        .order_by(poc_dispute_votes::voted_at.desc())
        .limit(limit)
        .offset(offset)
        .select((
            poc_dispute_votes::dispute_id,
            poc_dispute_votes::voter,
            poc_dispute_votes::vote_choice,
            poc_dispute_votes::stake_amount,
            poc_dispute_votes::voted_at,
        ))
        .load::<(String, String, i16, i64, i64)>(&mut conn)
        .await?;
    Ok(results
        .into_iter()
        .map(
            |(dispute_id, voter, vote_choice, stake_amount, voted_at)| PocDisputeVoteRow {
                dispute_id,
                voter,
                vote_choice,
                stake_amount,
                voted_at,
            },
        )
        .collect())
}

pub(crate) async fn get_poc_analytics(db: &Db) -> Result<serde_json::Value, SocialError> {
    let mut conn = db.connect().await?;
    #[derive(QueryableByName)]
    struct CountRow {
        #[diesel(sql_type = BigInt)]
        count: i64,
    }
    let badges_count: i64 = diesel::sql_query(
        "SELECT COUNT(DISTINCT badge_id)::bigint as count FROM poc_badges WHERE revoked = false",
    )
    .get_result::<CountRow>(&mut conn)
    .await
    .map(|r| r.count)?;
    let disputes_count: i64 =
        diesel::sql_query("SELECT COUNT(DISTINCT dispute_id)::bigint as count FROM poc_disputes")
            .get_result::<CountRow>(&mut conn)
            .await
            .map(|r| r.count)?;
    let redirections_count: i64 = diesel::sql_query(
        "SELECT COUNT(DISTINCT redirection_id)::bigint as count FROM poc_revenue_redirections WHERE removed = false",
    )
    .get_result::<CountRow>(&mut conn)
    .await
    .map(|r| r.count)?;
    Ok(serde_json::json!({
        "total_badges": badges_count,
        "total_disputes": disputes_count,
        "total_revenue_redirections": redirections_count,
    }))
}

pub(crate) async fn get_poc_configuration(db: &Db) -> Result<Option<PocConfigRow>, SocialError> {
    let mut conn = db.connect().await?;
    let result = poc_configuration::table
        .order_by(poc_configuration::updated_at.desc())
        .limit(1)
        .select((
            poc_configuration::image_threshold,
            poc_configuration::video_threshold,
            poc_configuration::audio_threshold,
            poc_configuration::revenue_redirect_percentage,
            poc_configuration::dispute_cost,
            poc_configuration::dispute_protocol_fee,
            poc_configuration::min_vote_stake,
            poc_configuration::max_vote_stake,
            poc_configuration::voting_duration_epochs,
            poc_configuration::max_reasoning_length,
            poc_configuration::max_evidence_urls,
            poc_configuration::max_votes_per_dispute,
            poc_configuration::oracle_address,
            poc_configuration::updated_by,
            poc_configuration::updated_at,
            poc_configuration::transaction_id,
        ))
        .first::<(
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            i64,
            Option<String>,
            String,
            i64,
            String,
        )>(&mut conn)
        .await
        .optional()?;
    Ok(result.map(
        |(
            image_threshold,
            video_threshold,
            audio_threshold,
            revenue_redirect_percentage,
            dispute_cost,
            dispute_protocol_fee,
            min_vote_stake,
            max_vote_stake,
            voting_duration_epochs,
            max_reasoning_length,
            max_evidence_urls,
            max_votes_per_dispute,
            oracle_address,
            updated_by,
            updated_at,
            transaction_id,
        )| PocConfigRow {
            image_threshold,
            video_threshold,
            audio_threshold,
            revenue_redirect_percentage,
            dispute_cost,
            dispute_protocol_fee,
            min_vote_stake,
            max_vote_stake,
            voting_duration_epochs,
            max_reasoning_length,
            max_evidence_urls,
            max_votes_per_dispute,
            oracle_address,
            updated_by,
            updated_at,
            transaction_id,
        },
    ))
}
