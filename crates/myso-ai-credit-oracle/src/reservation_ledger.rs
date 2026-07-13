// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! PostgreSQL-backed inference reservation ledger and transactional outbox.
//!
//! The database is authoritative for gateway idempotency. Per-balance advisory
//! locks serialize reservation nonce allocation across replicas. Outbox rows are
//! claimed with leases and `SKIP LOCKED`, so only one replica submits a given
//! reserve, capture, or cancel action at a time.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{postgres::PgPoolOptions, FromRow, PgPool};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReservationStatus {
    Preparing,
    Reserved,
    ProviderSucceeded,
    Captured,
    Cancelled,
    AmbiguousProviderFailure,
}

impl ReservationStatus {
    fn as_str(self) -> &'static str {
        match self {
            Self::Preparing => "preparing",
            Self::Reserved => "reserved",
            Self::ProviderSucceeded => "provider_succeeded",
            Self::Captured => "captured",
            Self::Cancelled => "cancelled",
            Self::AmbiguousProviderFailure => "ambiguous_provider_failure",
        }
    }

    fn parse(value: &str) -> Result<Self> {
        Ok(match value {
            "preparing" => Self::Preparing,
            "reserved" => Self::Reserved,
            "provider_succeeded" => Self::ProviderSucceeded,
            "captured" => Self::Captured,
            "cancelled" => Self::Cancelled,
            "ambiguous_provider_failure" => Self::AmbiguousProviderFailure,
            _ => anyhow::bail!("invalid reservation status {value}"),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReservationRecord {
    pub idempotency_key: String,
    pub owner: String,
    pub balance_id: String,
    pub memory_account_id: String,
    pub agent_object_id: String,
    pub model_id: String,
    pub reservation_nonce: u64,
    pub max_amount_mist: u64,
    pub provider_envelope_hash_hex: String,
    pub request_hash_hex: String,
    pub fx_quote_id_hex: String,
    pub myso_usd_e8: u64,
    pub markup_bps: u64,
    pub capture_deadline_ms: u64,
    pub hard_expiry_ms: u64,
    pub status: ReservationStatus,
    pub reserve_digest: Option<String>,
    pub capture_digest: Option<String>,
    pub cancel_digest: Option<String>,
    pub amount_mist: Option<u64>,
    pub provider_cost_usd_micros: Option<u64>,
    pub upstream_cost_usd_micros: Option<u64>,
    pub provider_generation_id: Option<String>,
    pub content: Option<String>,
    pub tokens_in: Option<u64>,
    pub tokens_out: Option<u64>,
    pub created_at_ms: u64,
    pub updated_at_ms: u64,
    pub last_error: Option<String>,
}

#[derive(Debug, FromRow)]
struct ReservationRow {
    idempotency_key: String,
    owner_address: String,
    balance_id: String,
    memory_account_id: String,
    agent_object_id: String,
    model_id: String,
    reservation_nonce: i64,
    max_amount_mist: i64,
    provider_envelope_hash_hex: String,
    request_hash_hex: String,
    fx_quote_id_hex: String,
    myso_usd_e8: i64,
    markup_bps: i64,
    capture_deadline_ms: i64,
    hard_expiry_ms: i64,
    status: String,
    reserve_digest: Option<String>,
    capture_digest: Option<String>,
    cancel_digest: Option<String>,
    amount_mist: Option<i64>,
    provider_cost_usd_micros: Option<i64>,
    upstream_cost_usd_micros: Option<i64>,
    provider_generation_id: Option<String>,
    content: Option<String>,
    tokens_in: Option<i64>,
    tokens_out: Option<i64>,
    created_at_ms: i64,
    updated_at_ms: i64,
    last_error: Option<String>,
}

fn to_u64(value: i64, field: &str) -> Result<u64> {
    u64::try_from(value).with_context(|| format!("negative {field} in inference ledger"))
}

fn to_i64(value: u64, field: &str) -> Result<i64> {
    i64::try_from(value).with_context(|| format!("{field} exceeds PostgreSQL BIGINT"))
}

impl TryFrom<ReservationRow> for ReservationRecord {
    type Error = anyhow::Error;

    fn try_from(row: ReservationRow) -> Result<Self> {
        Ok(Self {
            idempotency_key: row.idempotency_key,
            owner: row.owner_address,
            balance_id: row.balance_id,
            memory_account_id: row.memory_account_id,
            agent_object_id: row.agent_object_id,
            model_id: row.model_id,
            reservation_nonce: to_u64(row.reservation_nonce, "reservation_nonce")?,
            max_amount_mist: to_u64(row.max_amount_mist, "max_amount_mist")?,
            provider_envelope_hash_hex: row.provider_envelope_hash_hex,
            request_hash_hex: row.request_hash_hex,
            fx_quote_id_hex: row.fx_quote_id_hex,
            myso_usd_e8: to_u64(row.myso_usd_e8, "myso_usd_e8")?,
            markup_bps: to_u64(row.markup_bps, "markup_bps")?,
            capture_deadline_ms: to_u64(row.capture_deadline_ms, "capture_deadline_ms")?,
            hard_expiry_ms: to_u64(row.hard_expiry_ms, "hard_expiry_ms")?,
            status: ReservationStatus::parse(&row.status)?,
            reserve_digest: row.reserve_digest,
            capture_digest: row.capture_digest,
            cancel_digest: row.cancel_digest,
            amount_mist: row
                .amount_mist
                .map(|v| to_u64(v, "amount_mist"))
                .transpose()?,
            provider_cost_usd_micros: row
                .provider_cost_usd_micros
                .map(|v| to_u64(v, "provider_cost_usd_micros"))
                .transpose()?,
            upstream_cost_usd_micros: row
                .upstream_cost_usd_micros
                .map(|v| to_u64(v, "upstream_cost_usd_micros"))
                .transpose()?,
            provider_generation_id: row.provider_generation_id,
            content: row.content,
            tokens_in: row.tokens_in.map(|v| to_u64(v, "tokens_in")).transpose()?,
            tokens_out: row
                .tokens_out
                .map(|v| to_u64(v, "tokens_out"))
                .transpose()?,
            created_at_ms: to_u64(row.created_at_ms, "created_at_ms")?,
            updated_at_ms: to_u64(row.updated_at_ms, "updated_at_ms")?,
            last_error: row.last_error,
        })
    }
}

const SELECT_COLUMNS: &str = "idempotency_key, owner_address, balance_id, memory_account_id, agent_object_id, model_id, reservation_nonce, max_amount_mist, provider_envelope_hash_hex, request_hash_hex, fx_quote_id_hex, myso_usd_e8, markup_bps, capture_deadline_ms, hard_expiry_ms, status, reserve_digest, capture_digest, cancel_digest, amount_mist, provider_cost_usd_micros, upstream_cost_usd_micros, provider_generation_id, content, tokens_in, tokens_out, created_at_ms, updated_at_ms, last_error";

#[derive(Debug, Clone)]
pub struct ClaimedOutboxAction {
    pub action: String,
    pub record: ReservationRecord,
}

#[derive(Debug)]
pub enum BeginReservation {
    Created(ReservationRecord),
    Existing(ReservationRecord),
}

#[derive(Clone)]
pub struct ReservationLedger {
    pool: PgPool,
    worker_id: String,
    lease_secs: u64,
}

impl ReservationLedger {
    pub async fn connect(
        database_url: &str,
        max_connections: u32,
        lease_secs: u64,
    ) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(max_connections)
            .connect(database_url)
            .await
            .context("connect AI credit PostgreSQL ledger")?;
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .context("migrate AI credit PostgreSQL ledger")?;
        Ok(Self {
            pool,
            worker_id: uuid::Uuid::new_v4().to_string(),
            lease_secs,
        })
    }

    pub async fn probe(&self) -> bool {
        sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .is_ok()
    }

    pub async fn incomplete_count(&self) -> Result<i64> {
        sqlx::query_scalar("SELECT COUNT(*) FROM ai_inference_reservations WHERE status NOT IN ('captured','cancelled')")
            .fetch_one(&self.pool)
            .await
            .context("count incomplete inference reservations")
    }

    pub async fn find(
        &self,
        balance_id: &str,
        agent_object_id: &str,
        idempotency_key: &str,
    ) -> Result<Option<ReservationRecord>> {
        let query = format!("SELECT {SELECT_COLUMNS} FROM ai_inference_reservations WHERE balance_id=$1 AND agent_object_id=$2 AND idempotency_key=$3");
        sqlx::query_as::<_, ReservationRow>(&query)
            .bind(balance_id)
            .bind(agent_object_id)
            .bind(idempotency_key)
            .fetch_optional(&self.pool)
            .await
            .context("read inference reservation")?
            .map(TryInto::try_into)
            .transpose()
    }

    pub async fn begin(
        &self,
        mut record: ReservationRecord,
        chain_latest_nonce: u64,
    ) -> Result<BeginReservation> {
        let mut tx = self.pool.begin().await?;
        sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 0))")
            .bind(&record.balance_id)
            .execute(&mut *tx)
            .await?;
        let existing_query = format!("SELECT {SELECT_COLUMNS} FROM ai_inference_reservations WHERE balance_id=$1 AND agent_object_id=$2 AND idempotency_key=$3");
        if let Some(existing) = sqlx::query_as::<_, ReservationRow>(&existing_query)
            .bind(&record.balance_id)
            .bind(&record.agent_object_id)
            .bind(&record.idempotency_key)
            .fetch_optional(&mut *tx)
            .await?
        {
            tx.commit().await?;
            return Ok(BeginReservation::Existing(existing.try_into()?));
        }
        let database_nonce: i64 = sqlx::query_scalar("SELECT COALESCE(MAX(reservation_nonce), 0) FROM ai_inference_reservations WHERE balance_id=$1")
            .bind(&record.balance_id)
            .fetch_one(&mut *tx)
            .await?;
        record.reservation_nonce = chain_latest_nonce
            .max(to_u64(database_nonce, "database reservation nonce")?)
            .checked_add(1)
            .context("reservation nonce overflow")?;
        let scope = idempotency_scope(&record);
        sqlx::query("INSERT INTO ai_inference_reservations (idempotency_scope,idempotency_key,owner_address,balance_id,memory_account_id,agent_object_id,model_id,reservation_nonce,max_amount_mist,provider_envelope_hash_hex,request_hash_hex,fx_quote_id_hex,myso_usd_e8,markup_bps,capture_deadline_ms,hard_expiry_ms,status,created_at_ms,updated_at_ms) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19)")
            .bind(&scope).bind(&record.idempotency_key).bind(&record.owner).bind(&record.balance_id)
            .bind(&record.memory_account_id).bind(&record.agent_object_id).bind(&record.model_id)
            .bind(to_i64(record.reservation_nonce, "reservation_nonce")?)
            .bind(to_i64(record.max_amount_mist, "max_amount_mist")?)
            .bind(&record.provider_envelope_hash_hex).bind(&record.request_hash_hex).bind(&record.fx_quote_id_hex)
            .bind(to_i64(record.myso_usd_e8, "myso_usd_e8")?).bind(to_i64(record.markup_bps, "markup_bps")?)
            .bind(to_i64(record.capture_deadline_ms, "capture_deadline_ms")?).bind(to_i64(record.hard_expiry_ms, "hard_expiry_ms")?)
            .bind(record.status.as_str()).bind(to_i64(record.created_at_ms, "created_at_ms")?).bind(to_i64(record.updated_at_ms, "updated_at_ms")?)
            .execute(&mut *tx).await?;
        insert_outbox(&mut tx, &scope, "reserve", &record).await?;
        tx.commit().await?;
        Ok(BeginReservation::Created(record))
    }

    pub async fn update<F>(
        &self,
        balance_id: &str,
        agent_object_id: &str,
        idempotency_key: &str,
        update: F,
    ) -> Result<ReservationRecord>
    where
        F: FnOnce(&mut ReservationRecord),
    {
        let mut tx = self.pool.begin().await?;
        let query = format!("SELECT {SELECT_COLUMNS} FROM ai_inference_reservations WHERE balance_id=$1 AND agent_object_id=$2 AND idempotency_key=$3 FOR UPDATE");
        let row = sqlx::query_as::<_, ReservationRow>(&query)
            .bind(balance_id)
            .bind(agent_object_id)
            .bind(idempotency_key)
            .fetch_one(&mut *tx)
            .await?;
        let mut record: ReservationRecord = row.try_into()?;
        let old_status = record.status;
        update(&mut record);
        record.updated_at_ms = chrono::Utc::now().timestamp_millis() as u64;
        let scope = idempotency_scope(&record);
        save_record(&mut tx, &scope, &record).await?;
        if old_status != ReservationStatus::ProviderSucceeded
            && record.status == ReservationStatus::ProviderSucceeded
        {
            let action = if record.amount_mist.unwrap_or(0) == 0
                || record.provider_cost_usd_micros.unwrap_or(0) == 0
            {
                "cancel"
            } else {
                "capture"
            };
            insert_outbox(&mut tx, &scope, action, &record).await?;
        }
        tx.commit().await?;
        Ok(record)
    }

    /// Atomically records a finalized chain side effect and delivers its outbox row.
    pub async fn complete_action<F>(
        &self,
        balance_id: &str,
        agent_object_id: &str,
        idempotency_key: &str,
        action: &str,
        digest: &str,
        update: F,
    ) -> Result<ReservationRecord>
    where
        F: FnOnce(&mut ReservationRecord),
    {
        let mut tx = self.pool.begin().await?;
        let query = format!("SELECT {SELECT_COLUMNS} FROM ai_inference_reservations WHERE balance_id=$1 AND agent_object_id=$2 AND idempotency_key=$3 FOR UPDATE");
        let row = sqlx::query_as::<_, ReservationRow>(&query)
            .bind(balance_id)
            .bind(agent_object_id)
            .bind(idempotency_key)
            .fetch_one(&mut *tx)
            .await?;
        let mut record: ReservationRecord = row.try_into()?;
        update(&mut record);
        record.updated_at_ms = chrono::Utc::now().timestamp_millis() as u64;
        let scope = idempotency_scope(&record);
        save_record(&mut tx, &scope, &record).await?;
        let delivered = sqlx::query("UPDATE ai_inference_outbox SET state='delivered',delivered_digest=$3,lease_owner=NULL,lease_expires_at=NULL,last_error=NULL,updated_at=NOW() WHERE idempotency_scope=$1 AND action=$2 AND state <> 'delivered'")
            .bind(&scope).bind(action).bind(digest).execute(&mut *tx).await?;
        anyhow::ensure!(
            delivered.rows_affected() == 1,
            "outbox action was not claimable"
        );
        tx.commit().await?;
        Ok(record)
    }

    pub async fn claim_action(&self, record: &ReservationRecord, action: &str) -> Result<bool> {
        let result = sqlx::query("UPDATE ai_inference_outbox SET state='processing',lease_owner=$3,lease_expires_at=NOW()+($4::BIGINT * INTERVAL '1 second'),attempt_count=attempt_count+1,updated_at=NOW() WHERE idempotency_scope=$1 AND action=$2 AND state <> 'delivered' AND (state='pending' OR lease_expires_at < NOW())")
            .bind(idempotency_scope(record)).bind(action).bind(&self.worker_id).bind(to_i64(self.lease_secs, "outbox lease")?)
            .execute(&self.pool).await?;
        Ok(result.rows_affected() == 1)
    }

    pub async fn claim_pending(&self, limit: i64) -> Result<Vec<ClaimedOutboxAction>> {
        let mut tx = self.pool.begin().await?;
        let rows: Vec<(String, String)> = sqlx::query_as("WITH candidates AS (SELECT id FROM ai_inference_outbox WHERE state <> 'delivered' AND next_attempt_at <= NOW() AND (state='pending' OR lease_expires_at < NOW()) ORDER BY id FOR UPDATE SKIP LOCKED LIMIT $1) UPDATE ai_inference_outbox o SET state='processing',lease_owner=$2,lease_expires_at=NOW()+($3::BIGINT * INTERVAL '1 second'),attempt_count=attempt_count+1,updated_at=NOW() FROM candidates c WHERE o.id=c.id RETURNING o.idempotency_scope,o.action")
            .bind(limit).bind(&self.worker_id).bind(to_i64(self.lease_secs, "outbox lease")?).fetch_all(&mut *tx).await?;
        let mut claimed = Vec::with_capacity(rows.len());
        for (scope, action) in rows {
            let query = format!(
                "SELECT {SELECT_COLUMNS} FROM ai_inference_reservations WHERE idempotency_scope=$1"
            );
            let row = sqlx::query_as::<_, ReservationRow>(&query)
                .bind(&scope)
                .fetch_one(&mut *tx)
                .await?;
            claimed.push(ClaimedOutboxAction {
                action,
                record: row.try_into()?,
            });
        }
        tx.commit().await?;
        Ok(claimed)
    }

    pub async fn retry(&self, record: &ReservationRecord, action: &str, error: &str) -> Result<()> {
        sqlx::query("UPDATE ai_inference_outbox SET state='pending',lease_owner=NULL,lease_expires_at=NULL,next_attempt_at=NOW()+(LEAST(300,POWER(2,LEAST(attempt_count,8)))::BIGINT * INTERVAL '1 second'),last_error=$3,updated_at=NOW() WHERE idempotency_scope=$1 AND action=$2 AND state <> 'delivered'")
            .bind(idempotency_scope(record)).bind(action).bind(error).execute(&self.pool).await?;
        Ok(())
    }
}

async fn save_record(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    scope: &str,
    record: &ReservationRecord,
) -> Result<()> {
    sqlx::query("UPDATE ai_inference_reservations SET status=$2,reserve_digest=$3,capture_digest=$4,cancel_digest=$5,amount_mist=$6,provider_cost_usd_micros=$7,upstream_cost_usd_micros=$8,provider_generation_id=$9,content=$10,tokens_in=$11,tokens_out=$12,updated_at_ms=$13,last_error=$14 WHERE idempotency_scope=$1")
        .bind(scope).bind(record.status.as_str()).bind(&record.reserve_digest).bind(&record.capture_digest).bind(&record.cancel_digest)
        .bind(record.amount_mist.map(|v| to_i64(v, "amount_mist")).transpose()?)
        .bind(record.provider_cost_usd_micros.map(|v| to_i64(v, "provider_cost_usd_micros")).transpose()?)
        .bind(record.upstream_cost_usd_micros.map(|v| to_i64(v, "upstream_cost_usd_micros")).transpose()?)
        .bind(&record.provider_generation_id).bind(&record.content)
        .bind(record.tokens_in.map(|v| to_i64(v, "tokens_in")).transpose()?)
        .bind(record.tokens_out.map(|v| to_i64(v, "tokens_out")).transpose()?)
        .bind(to_i64(record.updated_at_ms, "updated_at_ms")?).bind(&record.last_error)
        .execute(&mut **tx).await?;
    Ok(())
}

fn idempotency_scope(record: &ReservationRecord) -> String {
    use blake2::{Blake2b512, Digest};
    let mut hash = Blake2b512::new();
    for part in [
        &record.balance_id,
        &record.agent_object_id,
        &record.idempotency_key,
    ] {
        hash.update((part.len() as u64).to_le_bytes());
        hash.update(part.as_bytes());
    }
    hex::encode(&hash.finalize()[..32])
}

async fn insert_outbox(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    scope: &str,
    action: &str,
    record: &ReservationRecord,
) -> Result<()> {
    let payload = json!({
        "balanceId": record.balance_id,
        "agentObjectId": record.agent_object_id,
        "reservationNonce": record.reservation_nonce,
        "maxAmountMist": record.max_amount_mist,
        "requestHash": record.request_hash_hex,
        "providerEnvelopeHash": record.provider_envelope_hash_hex,
        "fxQuoteId": record.fx_quote_id_hex,
        "amountMist": record.amount_mist,
        "providerCostUsdMicros": record.provider_cost_usd_micros,
    });
    sqlx::query("INSERT INTO ai_inference_outbox (idempotency_scope,action,payload) VALUES ($1,$2,$3) ON CONFLICT (idempotency_scope,action) DO NOTHING")
        .bind(scope).bind(action).bind(payload).execute(&mut **tx).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_is_bound_to_balance_agent_and_key() {
        let mut record = ReservationRecord {
            idempotency_key: "request-1".into(),
            owner: "0x1".into(),
            balance_id: "0x2".into(),
            memory_account_id: "0x3".into(),
            agent_object_id: "0x4".into(),
            model_id: "model".into(),
            reservation_nonce: 1,
            max_amount_mist: 1,
            provider_envelope_hash_hex: "00".repeat(32),
            request_hash_hex: "11".repeat(32),
            fx_quote_id_hex: "22".repeat(32),
            myso_usd_e8: 1,
            markup_bps: 1500,
            capture_deadline_ms: 2,
            hard_expiry_ms: 3,
            status: ReservationStatus::Preparing,
            reserve_digest: None,
            capture_digest: None,
            cancel_digest: None,
            amount_mist: None,
            provider_cost_usd_micros: None,
            upstream_cost_usd_micros: None,
            provider_generation_id: None,
            content: None,
            tokens_in: None,
            tokens_out: None,
            created_at_ms: 1,
            updated_at_ms: 1,
            last_error: None,
        };
        let original = idempotency_scope(&record);
        record.agent_object_id = "0x5".into();
        assert_ne!(original, idempotency_scope(&record));
    }
}
