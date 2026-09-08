// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    user_spt_position_events, user_spt_position_snapshots, user_spt_position_state,
};

pub const POSITION_EVENT_BUY: &str = "buy";
pub const POSITION_EVENT_SELL: &str = "sell";
pub const POSITION_EVENT_RESERVATION: &str = "reservation";
pub const POSITION_EVENT_LAUNCH: &str = "launch";
pub const POSITION_EVENT_TRANSFER_IN: &str = "transfer_in";
pub const POSITION_EVENT_TRANSFER_OUT: &str = "transfer_out";

#[derive(Debug, Clone, Queryable, Selectable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = user_spt_position_state)]
pub struct UserSptPositionState {
    pub holder_address: String,
    pub pool_id: String,
    pub token_balance: i64,
    pub cost_basis_myso: i64,
    pub total_invested_myso: i64,
    pub total_returned_myso: i64,
    pub realized_myso: i64,
    pub disposed_cost_basis_myso: i64,
    pub sold_token_qty: i64,
    pub exit_proceeds_myso: i64,
    pub reservation_cost_myso: i64,
    pub cost_basis_unknown: bool,
    pub last_event_time: Option<chrono::DateTime<chrono::Utc>>,
    pub last_tx_id: Option<String>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl UserSptPositionState {
    pub fn empty(holder_address: String, pool_id: String) -> Self {
        Self {
            holder_address,
            pool_id,
            token_balance: 0,
            cost_basis_myso: 0,
            total_invested_myso: 0,
            total_returned_myso: 0,
            realized_myso: 0,
            disposed_cost_basis_myso: 0,
            sold_token_qty: 0,
            exit_proceeds_myso: 0,
            reservation_cost_myso: 0,
            cost_basis_unknown: false,
            last_event_time: None,
            last_tx_id: None,
            updated_at: chrono::Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = user_spt_position_events)]
pub struct NewUserSptPositionEvent {
    pub transaction_id: String,
    pub event_type: String,
    pub holder_address: String,
    pub pool_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = user_spt_position_snapshots)]
pub struct NewUserSptPositionSnapshot {
    pub time: chrono::DateTime<chrono::Utc>,
    pub holder_address: String,
    pub pool_id: String,
    pub token_balance: i64,
    pub circulating_supply: i64,
    pub cost_basis_myso: i64,
    pub realized_myso: i64,
    pub disposed_cost_basis_myso: i64,
    pub reservation_cost_myso: i64,
    pub event_type: String,
    pub transaction_id: String,
}
