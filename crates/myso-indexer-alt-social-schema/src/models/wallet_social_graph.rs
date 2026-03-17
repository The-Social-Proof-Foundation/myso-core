// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::wallet_social_graph;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = wallet_social_graph)]
pub struct WalletSocialGraph {
    pub wallet_address: String,
    pub followers_count: i32,
    pub following_count: i32,
    pub blocked_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = wallet_social_graph)]
pub struct NewWalletSocialGraph {
    pub wallet_address: String,
    pub followers_count: i32,
    pub following_count: i32,
    pub blocked_count: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
