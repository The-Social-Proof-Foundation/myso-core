// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::wallet_messaging_policies;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = wallet_messaging_policies)]
pub struct WalletMessagingPolicy {
    pub wallet_address: String,
    pub enabled: bool,
    pub min_cost: Option<i64>,
    pub updated_at: i64,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = wallet_messaging_policies)]
pub struct NewWalletMessagingPolicy {
    pub wallet_address: String,
    pub enabled: bool,
    pub min_cost: Option<i64>,
    pub updated_at: i64,
}
