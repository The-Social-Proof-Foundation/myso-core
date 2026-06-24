// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::username_registry;

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
