// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use uuid::Uuid;

pub fn canonical_x_handle(raw: &str) -> String {
    raw.trim().trim_start_matches('@').to_lowercase()
}

pub fn identity_hash_from_x_handle(handle: &str) -> String {
    let canonical = canonical_x_handle(handle);
    format!("0x{}", hex::encode(canonical.as_bytes()))
}

pub fn creator_confidence_from_signals(
    explicit_handle: bool,
    trust_score: f64,
    linked_works: i32,
) -> f64 {
    let mut score = if explicit_handle { trust_score * 0.85 } else { 0.0 };
    score += (linked_works as f64 * 0.01).min(0.15);
    score.min(1.0)
}

pub async fn resolve_or_create_candidate(
    pool: &sqlx::PgPool,
    x_handle: &str,
    creator_confidence: f64,
) -> anyhow::Result<Uuid> {
    let canonical = canonical_x_handle(x_handle);
    let identity_hash = identity_hash_from_x_handle(&canonical);
    let row: (Uuid,) = sqlx::query_as(
        r#"
        INSERT INTO creator_candidates (primary_x_handle, identity_hash, creator_confidence, lifecycle_state)
        VALUES ($1, $2, $3, 'resolved')
        ON CONFLICT (primary_x_handle) DO UPDATE SET
            creator_confidence = GREATEST(creator_candidates.creator_confidence, EXCLUDED.creator_confidence),
            identity_hash = COALESCE(creator_candidates.identity_hash, EXCLUDED.identity_hash),
            updated_at = NOW()
        RETURNING id
        "#,
    )
    .bind(&canonical)
    .bind(&identity_hash)
    .bind(creator_confidence)
    .fetch_one(pool)
    .await?;
    Ok(row.0)
}
