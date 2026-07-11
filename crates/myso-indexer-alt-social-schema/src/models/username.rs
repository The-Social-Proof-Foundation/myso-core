// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use chrono::{DateTime, Utc};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{username_registry, username_reservations};

pub const USERNAME_RESERVATION_STATUS_ACTIVE: &str = "active";
pub const USERNAME_RESERVATION_STATUS_RELEASED: &str = "released";

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = username_registry)]
pub struct UsernameRegistryRow {
    pub username: String,
    pub profile_id: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = username_registry)]
pub struct NewUsernameRegistry {
    pub username: String,
    pub profile_id: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = username_reservations)]
pub struct UsernameReservation {
    pub id: i32,
    pub username: String,
    pub reason: i16,
    pub reserved_by: String,
    pub reserved_at: i64,
    pub released_by: Option<String>,
    pub released_at: Option<i64>,
    pub status: String,
    pub reserve_transaction_id: String,
    pub release_transaction_id: Option<String>,
    pub time: DateTime<Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = username_reservations)]
pub struct NewUsernameReservation {
    pub username: String,
    pub reason: i16,
    pub reserved_by: String,
    pub reserved_at: i64,
    pub released_by: Option<String>,
    pub released_at: Option<i64>,
    pub status: String,
    pub reserve_transaction_id: String,
    pub release_transaction_id: Option<String>,
    pub time: DateTime<Utc>,
}
