// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::resolver::{MaturitySchedule, ResolverDefinition, ResolverSpec};
use crate::review::canonicalize::OutcomeType;
use crate::review::CanonicalClaimFields;
use crate::types::ResolverKind;

#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct OracleReviewRow {
    pub id: Uuid,
    pub post_id: String,
    pub canonical_claim_id: Option<Uuid>,
    pub decision: String,
    pub reject_reason: Option<String>,
    pub reviewer: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ExtractedClaim {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metric: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub comparison: Option<crate::types::ComparisonOp>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub threshold: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<DateTime<Utc>>,
    pub outcome_type: OutcomeType,
    #[serde(default)]
    pub suggested_sources: Vec<String>,
    #[serde(default)]
    pub suggested_options: Vec<String>,
    #[serde(default)]
    pub claim_category: crate::types::ClaimCategory,
    /// future | past | unsupported. Future opens a market; past is verified; unsupported skipped.
    #[serde(default)]
    pub time_class: crate::types::TimeClass,
    #[serde(default)]
    pub resolver_hints: crate::types::ResolverHints,
}

pub async fn insert_llm_extraction(
    pool: &PgPool,
    post_id: &str,
    raw_response: Option<&str>,
    parsed: &ExtractedClaim,
    model: &str,
) -> anyhow::Result<Uuid> {
    let parsed_json = serde_json::to_value(parsed)?;
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO llm_extractions (post_id, raw_response, parsed, model)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind(post_id)
    .bind(raw_response)
    .bind(parsed_json)
    .bind(model)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn get_or_insert_canonical_claim(
    pool: &PgPool,
    llm_extraction_id: Uuid,
    fields: &CanonicalClaimFields,
    semantic_hash: &str,
    market_key_hash: &str,
) -> anyhow::Result<Uuid> {
    if let Some(row) = sqlx::query_as::<_, (Uuid,)>(
        "SELECT id FROM canonical_claims WHERE semantic_claim_hash = $1",
    )
    .bind(semantic_hash)
    .fetch_optional(pool)
    .await?
    {
        return Ok(row.0);
    }
    let fields_json = serde_json::to_value(fields)?;
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO canonical_claims (llm_extraction_id, normalized_fields, claim_hash, semantic_claim_hash, market_key_hash)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id
        "#,
    )
    .bind(llm_extraction_id)
    .bind(fields_json)
    .bind(market_key_hash)
    .bind(semantic_hash)
    .bind(market_key_hash)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn insert_oracle_review(
    pool: &PgPool,
    post_id: &str,
    canonical_claim_id: Option<Uuid>,
    decision: &str,
    reject_reason: Option<&str>,
) -> anyhow::Result<Uuid> {
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO oracle_reviews (post_id, canonical_claim_id, decision, reject_reason)
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    .bind(post_id)
    .bind(canonical_claim_id)
    .bind(decision)
    .bind(reject_reason)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn get_review(pool: &PgPool, review_id: Uuid) -> anyhow::Result<Option<OracleReviewRow>> {
    let row = sqlx::query_as::<_, OracleReviewRow>(
        r#"
        SELECT id, post_id, canonical_claim_id, decision, reject_reason, reviewer, created_at
        FROM oracle_reviews WHERE id = $1
        "#,
    )
    .bind(review_id)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

pub async fn insert_resolver_definition(
    pool: &PgPool,
    canonical_claim_id: Uuid,
    def: &ResolverDefinition,
    compile_fingerprint: &str,
) -> anyhow::Result<Uuid> {
    let spec_json = serde_json::to_value(&def.spec)?;
    let source_ids_json = serde_json::to_value(&def.source_ids)?;
    let betting_options_json = serde_json::to_value(&def.betting_options)?;
    let maturity_json = serde_json::to_value(&def.maturity_schedule)?;
    let kind = match def.resolver_kind {
        ResolverKind::PriceThreshold => "price_threshold",
        ResolverKind::EventOccurrence => "event_occurrence",
        ResolverKind::ReleasePublished => "release_published",
        ResolverKind::CustomHttp => "custom_http",
    };
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO resolver_definitions
            (id, canonical_claim_id, resolver_kind, spec, source_ids, betting_options, maturity_schedule, compile_fingerprint)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
    )
    .bind(def.id)
    .bind(canonical_claim_id)
    .bind(kind)
    .bind(spec_json)
    .bind(source_ids_json)
    .bind(betting_options_json)
    .bind(maturity_json)
    .bind(compile_fingerprint)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}

pub async fn get_resolver_definition(
    pool: &PgPool,
    def_id: Uuid,
) -> anyhow::Result<Option<ResolverDefinition>> {
    #[derive(sqlx::FromRow)]
    struct Row {
        id: Uuid,
        resolver_kind: String,
        spec: serde_json::Value,
        source_ids: serde_json::Value,
        betting_options: serde_json::Value,
        maturity_schedule: serde_json::Value,
    }
    let row = sqlx::query_as::<_, Row>(
        r#"
        SELECT id, resolver_kind, spec, source_ids, betting_options, maturity_schedule
        FROM resolver_definitions WHERE id = $1
        "#,
    )
    .bind(def_id)
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(None);
    };

    let resolver_kind = match row.resolver_kind.as_str() {
        "price_threshold" => crate::types::ResolverKind::PriceThreshold,
        "event_occurrence" => crate::types::ResolverKind::EventOccurrence,
        "release_published" => crate::types::ResolverKind::ReleasePublished,
        _ => crate::types::ResolverKind::CustomHttp,
    };
    Ok(Some(ResolverDefinition {
        id: row.id,
        resolver_kind,
        spec: serde_json::from_value::<ResolverSpec>(row.spec)?,
        source_ids: serde_json::from_value(row.source_ids)?,
        betting_options: serde_json::from_value(row.betting_options)?,
        maturity_schedule: serde_json::from_value::<MaturitySchedule>(row.maturity_schedule)?,
    }))
}
