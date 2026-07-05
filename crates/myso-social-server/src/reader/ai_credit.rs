// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    AiCreditAgentBudgetRow, AiCreditBalanceRow, AiCreditConfigRow, AiCreditUsageLineRow,
    NewAiCreditUsageLine,
};
use myso_indexer_alt_social_schema::schema::{
    ai_credit_agent_budgets, ai_credit_balances, ai_credit_config, ai_credit_usage_lines,
};
use myso_pg_db::Db;
use serde::{Deserialize, Serialize};

use crate::error::SocialError;

pub(crate) async fn get_ai_credit_balance_by_owner(
    db: &Db,
    owner: &str,
) -> Result<Option<AiCreditBalanceRow>, SocialError> {
    let mut conn = db.connect().await?;
    ai_credit_balances::table
        .filter(ai_credit_balances::principal_owner.eq(owner))
        .select(AiCreditBalanceRow::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(Into::into)
}

pub(crate) async fn list_agent_budgets(
    db: &Db,
    balance_id: &str,
) -> Result<Vec<AiCreditAgentBudgetRow>, SocialError> {
    let mut conn = db.connect().await?;
    ai_credit_agent_budgets::table
        .filter(ai_credit_agent_budgets::balance_id.eq(balance_id))
        .select(AiCreditAgentBudgetRow::as_select())
        .load(&mut conn)
        .await
        .map_err(Into::into)
}

pub(crate) async fn list_usage_lines(
    db: &Db,
    balance_id: &str,
    limit: i64,
) -> Result<Vec<AiCreditUsageLineRow>, SocialError> {
    let mut conn = db.connect().await?;
    ai_credit_usage_lines::table
        .filter(ai_credit_usage_lines::balance_id.eq(balance_id))
        .order(ai_credit_usage_lines::created_at.desc())
        .limit(limit)
        .select(AiCreditUsageLineRow::as_select())
        .load(&mut conn)
        .await
        .map_err(Into::into)
}

pub(crate) async fn get_ai_credit_config(
    db: &Db,
) -> Result<Option<AiCreditConfigRow>, SocialError> {
    let mut conn = db.connect().await?;
    ai_credit_config::table
        .order(ai_credit_config::time.desc())
        .select(AiCreditConfigRow::as_select())
        .first(&mut conn)
        .await
        .optional()
        .map_err(Into::into)
}

#[derive(Debug, Clone, Deserialize)]
pub struct IngestUsageLineRequest {
    pub receipt_id: String,
    pub balance_id: String,
    pub agent_object_id: String,
    pub usage_kind: i16,
    pub amount_mist: i64,
    pub model_id: Option<String>,
    pub tool_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub organization_id: Option<String>,
}

pub(crate) async fn ingest_usage_line(
    db: &Db,
    req: IngestUsageLineRequest,
) -> Result<(), SocialError> {
    let mut conn = db.connect().await?;
    diesel::insert_into(ai_credit_usage_lines::table)
        .values(NewAiCreditUsageLine {
            receipt_id: req.receipt_id,
            balance_id: req.balance_id,
            agent_object_id: req.agent_object_id,
            usage_kind: req.usage_kind,
            amount_mist: req.amount_mist,
            model_id: req.model_id,
            tool_id: req.tool_id,
            metadata: req.metadata,
            settled: false,
            settlement_tx: None,
            created_at: chrono::Utc::now(),
            organization_id: req.organization_id,
        })
        .on_conflict(ai_credit_usage_lines::receipt_id)
        .do_nothing()
        .execute(&mut conn)
        .await?;
    Ok(())
}

#[cfg(test)]
mod ingest_tests {
    use diesel::QueryDsl;
    use diesel_async::RunQueryDsl;
    use myso_indexer_alt_social_schema::schema::ai_credit_usage_lines;
    use myso_pg_db::temp::TempDb;
    use myso_pg_db::{Db, DbArgs};

    use super::*;

    async fn setup_usage_lines_table(db: &Db) {
        let mut conn = db.connect().await.unwrap();
        diesel::sql_query(
            r#"
            CREATE TABLE IF NOT EXISTS ai_credit_usage_lines (
                id BIGSERIAL PRIMARY KEY,
                receipt_id TEXT NOT NULL UNIQUE,
                balance_id TEXT NOT NULL,
                agent_object_id TEXT NOT NULL,
                usage_kind SMALLINT NOT NULL,
                amount_mist BIGINT NOT NULL,
                model_id TEXT,
                tool_id TEXT,
                metadata JSONB,
                settled BOOLEAN NOT NULL DEFAULT FALSE,
                settlement_tx TEXT,
                created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                organization_id TEXT
            )
            "#,
        )
        .execute(&mut conn)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn ingest_usage_line_requires_write_pool() {
        let temp_db = TempDb::new().unwrap();
        let url = temp_db.database().url();
        let write_db = Db::for_write(url.clone(), DbArgs::default()).await.unwrap();
        setup_usage_lines_table(&write_db).await;
        let read_db = Db::for_read(url.clone(), DbArgs::default()).await.unwrap();

        let req = IngestUsageLineRequest {
            receipt_id: "1".to_string(),
            balance_id: "0xabc".to_string(),
            agent_object_id: "0xdef".to_string(),
            usage_kind: 1,
            amount_mist: 100,
            model_id: None,
            tool_id: None,
            metadata: None,
            organization_id: None,
        };

        let read_err = ingest_usage_line(&read_db, req.clone()).await.unwrap_err();
        assert!(
            read_err.to_string().contains("read-only"),
            "expected read-only error, got: {read_err}"
        );

        ingest_usage_line(&write_db, req).await.unwrap();

        let mut conn = write_db.connect().await.unwrap();
        let count: i64 = ai_credit_usage_lines::table
            .count()
            .get_result(&mut conn)
            .await
            .unwrap();
        assert_eq!(count, 1);
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AiCreditBalanceResponse {
    pub balance: AiCreditBalanceRow,
    pub credits: i64,
    pub agent_budgets: Vec<AiCreditAgentBudgetRow>,
}
