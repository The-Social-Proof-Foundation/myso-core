// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{object_migrated_events, upgrade_events};

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = upgrade_events)]
pub struct NewUpgradeEvent {
    pub package_id: String,
    pub version: i64,
    pub event_id: String,
    pub transaction_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = object_migrated_events)]
pub struct NewObjectMigratedEvent {
    pub object_id: String,
    pub object_type: String,
    pub old_version: i64,
    pub new_version: i64,
    pub migrated_by: String,
    pub event_id: String,
    pub transaction_id: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
