// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! SQLx repositories for SPoT-specific tables including `spot_trusted_sources`.
//! The PG job-queue pattern (`FOR UPDATE SKIP LOCKED` + priority + attempts) is
//! implemented against `spot_jobs`.

pub mod checkpoint;
pub mod claims;
pub mod events;
pub mod evidence;
pub mod jobs;
pub mod knowledge;
pub mod markets;
pub mod reviews;
pub mod transactions;

use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::events::config::EventProviderConfig;
use crate::events::types::DiscoveredEvent;
use crate::resolver::ResolverDefinition;
use crate::sources::SourceConfig;
use crate::store::reviews::ExtractedClaim;

pub use events::{EventProviderRow, ScheduledEventRow};

/// Deterministic UUID v5 namespace for `spot_trusted_sources` rows.
const NAMESPACE: Uuid = Uuid::from_bytes([
    0x6d, 0x79, 0x73, 0x6f, 0x2d, 0x73, 0x70, 0x6f, 0x74, 0x2d, 0x73, 0x72, 0x63, 0x2d, 0x76, 0x31,
]);

pub fn uuid_v5_named(source_id: &str) -> Uuid {
    Uuid::new_v5(&NAMESPACE, source_id.as_bytes())
}

pub struct OracleStore {
    pool: PgPool,
}

impl OracleStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn upsert_source_rows(&self, sources: &[SourceConfig]) -> anyhow::Result<()> {
        for cfg in sources {
            let config_json = serde_json::to_value(&cfg.config)?;
            let row: (Uuid,) = sqlx::query_as(
                r#"
                INSERT INTO spot_trusted_sources
                    (id, source_key, adapter_type, domain, trust_score, enabled, config)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                ON CONFLICT (id) DO UPDATE SET
                    source_key = EXCLUDED.source_key,
                    adapter_type = EXCLUDED.adapter_type,
                    domain = EXCLUDED.domain,
                    trust_score = EXCLUDED.trust_score,
                    enabled = EXCLUDED.enabled,
                    config = EXCLUDED.config,
                    updated_at = NOW()
                RETURNING id
                "#,
            )
            .bind(uuid_v5_named(&cfg.id))
            .bind(&cfg.id)
            .bind(&cfg.adapter_type)
            .bind(cfg.domain.as_str())
            .bind(cfg.trust_score)
            .bind(cfg.enabled)
            .bind(&config_json)
            .fetch_one(&self.pool)
            .await?;
            tracing::debug!(source_id = %cfg.id, db_id = %row.0, "registered trusted source");
        }
        Ok(())
    }

    // --- event providers ---
    pub async fn upsert_event_provider_rows(
        &self,
        providers: &[EventProviderConfig],
    ) -> anyhow::Result<()> {
        events::upsert_provider_rows(&self.pool, providers).await
    }

    pub async fn list_enabled_event_providers(&self) -> anyhow::Result<Vec<EventProviderRow>> {
        events::list_enabled_providers(&self.pool).await
    }

    pub async fn list_all_event_providers(&self) -> anyhow::Result<Vec<EventProviderRow>> {
        events::list_all_providers(&self.pool).await
    }

    pub async fn get_event_provider(
        &self,
        provider_key: &str,
    ) -> anyhow::Result<Option<EventProviderRow>> {
        events::get_provider_by_key(&self.pool, provider_key).await
    }

    pub async fn update_event_provider_sync_status(
        &self,
        provider_key: &str,
        status: &str,
        healthy: bool,
        message: &str,
    ) -> anyhow::Result<()> {
        events::update_provider_sync_status(&self.pool, provider_key, status, healthy, message)
            .await
    }

    pub async fn upsert_discovered_events(
        &self,
        provider_key: &str,
        discovered: &[DiscoveredEvent],
    ) -> anyhow::Result<usize> {
        events::upsert_discovered_events(&self.pool, provider_key, discovered).await
    }

    pub async fn list_active_scheduled_events(&self) -> anyhow::Result<Vec<ScheduledEventRow>> {
        events::list_active_scheduled_events(&self.pool).await
    }

    pub async fn list_scheduled_events(
        &self,
        active_only: bool,
        limit: i64,
    ) -> anyhow::Result<Vec<ScheduledEventRow>> {
        events::list_scheduled_events(&self.pool, active_only, limit).await
    }

    pub async fn patch_event_override(
        &self,
        event_id: Uuid,
        override_json: &serde_json::Value,
    ) -> anyhow::Result<bool> {
        events::patch_event_override(&self.pool, event_id, override_json).await
    }

    pub async fn list_enabled_sources(&self) -> anyhow::Result<Vec<SpotTrustedSourceRow>> {
        let rows = sqlx::query_as::<_, SpotTrustedSourceRow>(
            r#"
            SELECT id, source_key, adapter_type, domain, trust_score, enabled, config
            FROM spot_trusted_sources
            WHERE enabled = true
            ORDER BY trust_score DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(rows)
    }

    // --- markets ---
    pub async fn market_exists(&self, post_id: &str) -> anyhow::Result<bool> {
        markets::market_exists(&self.pool, post_id).await
    }

    pub async fn insert_market(
        &self,
        post_id: &str,
        creator: &str,
        claim_text: &str,
    ) -> anyhow::Result<Uuid> {
        markets::insert_market(&self.pool, post_id, creator, claim_text).await
    }

    pub async fn get_market_by_post_id(
        &self,
        post_id: &str,
    ) -> anyhow::Result<Option<markets::MarketRow>> {
        markets::get_market_by_post_id(&self.pool, post_id).await
    }

    pub async fn get_market(&self, market_id: Uuid) -> anyhow::Result<Option<markets::MarketRow>> {
        markets::get_market(&self.pool, market_id).await
    }

    pub async fn set_market_resolution_timing(
        &self,
        market_id: Uuid,
        resolution_window_ms: i64,
        max_resolution_window_ms: i64,
    ) -> anyhow::Result<()> {
        markets::set_market_resolution_timing(
            &self.pool,
            market_id,
            resolution_window_ms,
            max_resolution_window_ms,
        )
        .await
    }

    pub async fn list_markets(
        &self,
        status: Option<&str>,
        limit: i64,
    ) -> anyhow::Result<Vec<markets::MarketRow>> {
        markets::list_markets(&self.pool, status, limit).await
    }

    pub async fn apply_market_transition(
        &self,
        market_id: Uuid,
        event: &crate::claim::lifecycle::LifecycleEvent,
        ctx: &crate::claim::lifecycle::TransitionContext,
    ) -> anyhow::Result<crate::types::MarketStatus> {
        markets::apply_market_transition(&self.pool, market_id, event, ctx).await
    }

    pub async fn transition_with_metadata(
        &self,
        market_id: Uuid,
        event: crate::claim::lifecycle::LifecycleEvent,
        review_id: Option<Uuid>,
        resolver_definition_id: Option<Uuid>,
        betting_options: Option<&serde_json::Value>,
        ctx: crate::claim::lifecycle::TransitionContext,
    ) -> anyhow::Result<()> {
        markets::transition_with_metadata(
            &self.pool,
            market_id,
            event,
            review_id,
            resolver_definition_id,
            betting_options,
            ctx,
        )
        .await
    }

    pub async fn update_market_status(
        &self,
        market_id: Uuid,
        status: &str,
        review_id: Option<Uuid>,
        resolver_definition_id: Option<Uuid>,
        betting_options: Option<&serde_json::Value>,
    ) -> anyhow::Result<()> {
        markets::update_market_status(
            &self.pool,
            market_id,
            status,
            review_id,
            resolver_definition_id,
            betting_options,
        )
        .await
    }

    pub async fn set_spot_market_object_id_on_market(
        &self,
        market_id: Uuid,
        spot_market_object_id: &str,
        ctx: crate::claim::lifecycle::TransitionContext,
    ) -> anyhow::Result<()> {
        markets::set_spot_market_object_id(&self.pool, market_id, spot_market_object_id, ctx).await
    }

    /// Legacy alias — stores the on-chain SpotMarket object id.
    pub async fn set_spot_record_id(
        &self,
        market_id: Uuid,
        spot_market_object_id: &str,
    ) -> anyhow::Result<()> {
        let mut ctx = crate::claim::lifecycle::default_context_for(
            &crate::claim::lifecycle::LifecycleEvent::CreateTxConfirmed,
        );
        ctx.on_chain_status = Some(1);
        self.set_spot_market_object_id_on_market(market_id, spot_market_object_id, ctx)
            .await
    }

    // --- jobs ---
    pub async fn enqueue_job(
        &self,
        job_type: &str,
        market_id: Option<Uuid>,
        resolver_definition_id: Option<Uuid>,
        priority_score: i64,
        run_after: DateTime<Utc>,
        payload: serde_json::Value,
    ) -> anyhow::Result<Uuid> {
        jobs::enqueue_job(
            &self.pool,
            job_type,
            market_id,
            resolver_definition_id,
            priority_score,
            run_after,
            payload,
        )
        .await
    }

    pub async fn claim_next_job(&self) -> anyhow::Result<Option<jobs::SpotJob>> {
        jobs::claim_next_job(&self.pool).await
    }

    pub async fn complete_job(
        &self,
        job_id: Uuid,
        status: &str,
        error: Option<&str>,
    ) -> anyhow::Result<()> {
        jobs::complete_job(&self.pool, job_id, status, error).await
    }

    pub async fn requeue_job(
        &self,
        job_id: Uuid,
        run_after: DateTime<Utc>,
        error: &str,
    ) -> anyhow::Result<()> {
        jobs::requeue_job(&self.pool, job_id, run_after, error).await
    }

    pub async fn list_jobs(
        &self,
        status: Option<&str>,
        limit: i64,
    ) -> anyhow::Result<Vec<jobs::SpotJob>> {
        jobs::list_jobs(&self.pool, status, limit).await
    }

    // --- reviews ---
    pub async fn insert_llm_extraction(
        &self,
        post_id: &str,
        raw_response: Option<&str>,
        parsed: &ExtractedClaim,
        model: &str,
    ) -> anyhow::Result<Uuid> {
        reviews::insert_llm_extraction(&self.pool, post_id, raw_response, parsed, model).await
    }

    pub async fn get_or_insert_canonical_claim(
        &self,
        llm_extraction_id: Uuid,
        fields: &crate::review::CanonicalClaimFields,
        semantic_hash: &str,
        market_key_hash: &str,
    ) -> anyhow::Result<Uuid> {
        reviews::get_or_insert_canonical_claim(
            &self.pool,
            llm_extraction_id,
            fields,
            semantic_hash,
            market_key_hash,
        )
        .await
    }

    pub async fn get_claim_by_id(
        &self,
        claim_id: Uuid,
    ) -> anyhow::Result<Option<claims::SpotClaimRow>> {
        claims::get_claim_by_id(&self.pool, claim_id).await
    }

    pub async fn get_spot_market_by_id(
        &self,
        spot_market_id: Uuid,
    ) -> anyhow::Result<Option<claims::SpotMarketRow>> {
        claims::get_spot_market_by_id(&self.pool, spot_market_id).await
    }

    pub async fn set_spot_claim_object_id(
        &self,
        claim_id: Uuid,
        object_id: &str,
    ) -> anyhow::Result<()> {
        claims::set_spot_claim_object_id(&self.pool, claim_id, object_id).await
    }

    pub async fn set_spot_market_object_id(
        &self,
        spot_market_id: Uuid,
        object_id: &str,
    ) -> anyhow::Result<()> {
        claims::set_spot_market_object_id(&self.pool, spot_market_id, object_id).await
    }

    // --- reviews (continued) ---
    pub async fn insert_oracle_review(
        &self,
        post_id: &str,
        canonical_claim_id: Option<Uuid>,
        decision: &str,
        reject_reason: Option<&str>,
    ) -> anyhow::Result<Uuid> {
        reviews::insert_oracle_review(
            &self.pool,
            post_id,
            canonical_claim_id,
            decision,
            reject_reason,
        )
        .await
    }

    pub async fn get_review(
        &self,
        review_id: Uuid,
    ) -> anyhow::Result<Option<reviews::OracleReviewRow>> {
        reviews::get_review(&self.pool, review_id).await
    }

    pub async fn insert_resolver_definition(
        &self,
        canonical_claim_id: Uuid,
        def: &ResolverDefinition,
        compile_fingerprint: &str,
    ) -> anyhow::Result<Uuid> {
        reviews::insert_resolver_definition(
            &self.pool,
            canonical_claim_id,
            def,
            compile_fingerprint,
        )
        .await
    }

    pub async fn get_resolver_definition(
        &self,
        def_id: Uuid,
    ) -> anyhow::Result<Option<ResolverDefinition>> {
        reviews::get_resolver_definition(&self.pool, def_id).await
    }

    // --- evidence ---
    pub async fn insert_evidence_bundle(
        &self,
        bundle: &crate::evidence::EvidenceBundle,
    ) -> anyhow::Result<Uuid> {
        evidence::insert_evidence_bundle(&self.pool, bundle).await
    }

    pub async fn list_bundles_for_market(
        &self,
        market_id: Uuid,
    ) -> anyhow::Result<Vec<evidence::EvidenceBundleRow>> {
        evidence::list_bundles_for_market(&self.pool, market_id).await
    }

    pub async fn list_evidence_for_market(
        &self,
        market_id: Uuid,
    ) -> anyhow::Result<Vec<evidence::EvidenceRow>> {
        evidence::list_evidence_for_market(&self.pool, market_id).await
    }

    pub async fn upsert_resolver_state(
        &self,
        market_id: Uuid,
        outcome_draft: Option<&str>,
        confidence_bps: Option<i32>,
    ) -> anyhow::Result<()> {
        evidence::upsert_resolver_state(&self.pool, market_id, outcome_draft, confidence_bps).await
    }

    // --- transactions ---
    pub async fn insert_transaction(
        &self,
        market_id: Option<Uuid>,
        tx_kind: &str,
        nonce: &str,
    ) -> anyhow::Result<Uuid> {
        transactions::insert_transaction(&self.pool, market_id, tx_kind, nonce).await
    }

    pub async fn update_transaction_status(
        &self,
        tx_id: Uuid,
        status: &str,
        digest: Option<&str>,
        error: Option<&str>,
    ) -> anyhow::Result<()> {
        transactions::update_transaction_status(&self.pool, tx_id, status, digest, error).await
    }

    pub async fn list_pending_transactions(
        &self,
        limit: i64,
    ) -> anyhow::Result<Vec<transactions::TransactionRow>> {
        transactions::list_pending_transactions(&self.pool, limit).await
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct SpotTrustedSourceRow {
    pub id: Uuid,
    pub source_key: String,
    pub adapter_type: String,
    pub domain: String,
    pub trust_score: f64,
    pub enabled: bool,
    pub config: serde_json::Value,
}
