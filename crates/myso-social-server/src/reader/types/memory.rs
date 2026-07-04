// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::sql_types::{BigInt, SmallInt, Text};
use diesel::QueryableByName;
use serde::Serialize;

#[derive(Debug, Serialize, QueryableByName)]
#[serde(rename_all = "camelCase")]
pub struct MemoryConfigInfo {
    #[diesel(sql_type = Text)]
    pub updated_by: String,
    #[diesel(sql_type = SmallInt)]
    pub max_organizations_per_user: i16,
    #[diesel(sql_type = BigInt)]
    pub org_category_update_cooldown_ms: i64,
    #[diesel(sql_type = SmallInt)]
    pub max_agent_depth: i16,
    #[diesel(sql_type = BigInt)]
    pub max_label_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_org_name_length: i64,
    #[diesel(sql_type = BigInt)]
    pub max_org_description_length: i64,
    #[diesel(sql_type = BigInt)]
    pub version: i64,
    #[diesel(sql_type = BigInt)]
    pub updated_at: i64,
}
