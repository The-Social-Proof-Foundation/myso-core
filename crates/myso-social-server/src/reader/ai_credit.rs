// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::{
    AiCreditAgentBudgetRow, AiCreditBalanceRow, AiCreditUsageLineRow, NewAiCreditUsageLine,
};
use myso_indexer_alt_social_schema::schema::{
    ai_credit_agent_budgets, ai_credit_balances, ai_credit_usage_lines,
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

#[derive(Debug, Deserialize)]
pub struct IngestUsageLineRequest {
    pub receipt_id: String,
    pub balance_id: String,
    pub agent_object_id: String,
    pub usage_kind: i16,
    pub amount_mist: i64,
    pub model_id: Option<String>,
    pub tool_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
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
        })
        .execute(&mut conn)
        .await?;
    Ok(())
}

#[derive(Debug, Clone, Serialize)]
pub struct AiCreditBalanceResponse {
    pub balance: AiCreditBalanceRow,
    pub credits: i64,
    pub agent_budgets: Vec<AiCreditAgentBudgetRow>,
}
