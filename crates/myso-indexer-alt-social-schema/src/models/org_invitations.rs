// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! On-chain org membership invitations (`OrgInvitation*` events from `memory.move`).

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::org_invitations;

pub const ORG_INVITATION_STATUS_PENDING: &str = "pending";
pub const ORG_INVITATION_STATUS_ACCEPTED: &str = "accepted";
pub const ORG_INVITATION_STATUS_DECLINED: &str = "declined";

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = org_invitations)]
pub struct NewOrgInvitation {
    pub organization_id: String,
    pub invitee_address: String,
    pub role_name: Option<String>,
    pub permissions_mask: i64,
    pub status: String,
    pub invited_by: String,
    pub created_at_ms: i64,
    pub expires_at_ms: Option<i64>,
    pub responded_at_ms: Option<i64>,
    pub responded_by: Option<String>,
    pub granted_mask: Option<i64>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = org_invitations)]
pub struct OrgInvitationRow {
    pub organization_id: String,
    pub invitee_address: String,
    pub role_name: Option<String>,
    pub permissions_mask: i64,
    pub status: String,
    pub invited_by: String,
    pub created_at_ms: i64,
    pub expires_at_ms: Option<i64>,
    pub responded_at_ms: Option<i64>,
    pub responded_by: Option<String>,
    pub granted_mask: Option<i64>,
    pub event_id: String,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}
