// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, Nullable, Text, Timestamptz};
use diesel::QueryableByName;
use serde::Serialize;

#[derive(Debug, Serialize, QueryableByName)]
pub struct UnifiedRevenueRow {
    #[diesel(sql_type = Text)]
    pub revenue_source: String,
    #[diesel(sql_type = Text)]
    pub revenue_type: String,
    #[diesel(sql_type = Text)]
    pub creator_address: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub platform_address: Option<String>,
    #[diesel(sql_type = BigInt)]
    pub amount: i64,
    #[diesel(sql_type = Text)]
    pub currency: String,
    #[diesel(sql_type = Nullable<Text>)]
    pub content_id: Option<String>,
    #[diesel(sql_type = Nullable<Text>)]
    pub content_type: Option<String>,
    #[diesel(sql_type = Text)]
    pub payer_address: String,
    #[diesel(sql_type = Text)]
    pub recipient_address: String,
    #[diesel(sql_type = BigInt)]
    pub revenue_time: i64,
    #[diesel(sql_type = Timestamptz)]
    pub time: chrono::DateTime<chrono::Utc>,
    #[diesel(sql_type = Text)]
    pub transaction_id: String,
}
