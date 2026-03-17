// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    promotion_budget_events, promotion_status_events, promotion_views, promoted_posts,
};

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = promoted_posts)]
pub struct NewPromotedPost {
    pub promotion_id: String,
    pub post_id: String,
    pub owner: String,
    pub profile_id: String,
    pub payment_per_view: i64,
    pub total_budget: i64,
    pub remaining_budget: i64,
    pub active: bool,
    pub created_at: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = promotion_views)]
pub struct NewPromotionView {
    pub post_id: String,
    pub promotion_id: String,
    pub viewer: String,
    pub payment_amount: i64,
    pub view_duration: i64,
    pub platform_id: String,
    pub timestamp: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = promotion_status_events)]
pub struct NewPromotionStatusEvent {
    pub post_id: String,
    pub promotion_id: String,
    pub event_type: String,
    pub triggered_by: String,
    pub new_status: Option<bool>,
    pub amount: Option<i64>,
    pub timestamp: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = promotion_budget_events)]
pub struct NewPromotionBudgetEvent {
    pub promotion_id: String,
    pub post_id: String,
    pub event_type: String,
    pub amount: i64,
    pub remaining_budget: i64,
    pub timestamp: i64,
    pub time: chrono::DateTime<chrono::Utc>,
    pub transaction_id: String,
}
