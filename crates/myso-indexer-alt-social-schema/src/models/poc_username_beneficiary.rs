// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::schema::{
    poc_creator_identity_links, poc_username_beneficiaries, poc_username_beneficiary_events,
};

pub const USERNAME_BENEFICIARY_STATUS_ACTIVE: i16 = 1;
pub const USERNAME_BENEFICIARY_STATUS_CLAIMED: i16 = 2;
pub const USERNAME_BENEFICIARY_STATUS_ENDED: i16 = 3;

pub const IDENTITY_SOURCE_X: i16 = 1;

pub const END_REASON_ADMIN: i16 = 1;

pub const EVENT_TYPE_PROVISIONED: &str = "provisioned";
pub const EVENT_TYPE_CLAIMED: &str = "claimed";
pub const EVENT_TYPE_ENDED: &str = "ended";
pub const EVENT_TYPE_CONFLICT: &str = "conflict";
pub const EVENT_TYPE_CREATOR_IDENTITY_WALLET_LINKED: &str = "creator_identity_wallet_linked";

pub const VAULT_CLAIM_KIND_STANDARD: &str = "standard";
pub const VAULT_CLAIM_KIND_JOIN_REFERRAL: &str = "join_referral";

#[derive(Debug, Clone, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = poc_username_beneficiaries)]
pub struct NewPocUsernameBeneficiary {
    pub beneficiary_id: String,
    pub username: String,
    pub status: i16,
    pub creator_identity_source: i16,
    pub creator_identity_hash: String,
    pub beneficiary_address: String,
    pub vault_id: String,
    pub required_x_handle: String,
    pub oracle_evidence_hash: String,
    pub provisioned_at_ms: i64,
    pub provisioned_by: String,
    pub claimed_profile_id: Option<String>,
    pub claimed_by: Option<String>,
    pub claimed_at_ms: Option<i64>,
    pub ended_at_ms: Option<i64>,
    pub ended_by: Option<String>,
    pub end_reason_code: Option<i16>,
    pub join_referrer: Option<String>,
    pub join_referral_paid: bool,
    pub join_referral_paid_at_ms: Option<i64>,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = poc_username_beneficiaries)]
pub struct PocUsernameBeneficiaryRow {
    pub beneficiary_id: String,
    pub username: String,
    pub status: i16,
    pub creator_identity_source: i16,
    pub creator_identity_hash: String,
    pub beneficiary_address: String,
    pub vault_id: String,
    pub required_x_handle: String,
    pub oracle_evidence_hash: String,
    pub provisioned_at_ms: i64,
    pub provisioned_by: String,
    pub claimed_profile_id: Option<String>,
    pub claimed_by: Option<String>,
    pub claimed_at_ms: Option<i64>,
    pub ended_at_ms: Option<i64>,
    pub ended_by: Option<String>,
    pub end_reason_code: Option<i16>,
    pub join_referrer: Option<String>,
    pub join_referral_paid: bool,
    pub join_referral_paid_at_ms: Option<i64>,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_creator_identity_links)]
pub struct NewPocCreatorIdentityLink {
    pub creator_identity_source: i16,
    pub creator_identity_hash: String,
    pub wallet_address: String,
    pub beneficiary_id: String,
    pub linked_at_ms: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = poc_creator_identity_links)]
pub struct PocCreatorIdentityLinkRow {
    pub creator_identity_source: i16,
    pub creator_identity_hash: String,
    pub wallet_address: String,
    pub beneficiary_id: String,
    pub linked_at_ms: i64,
    pub transaction_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Insertable, Serialize, Deserialize)]
#[diesel(table_name = poc_username_beneficiary_events)]
pub struct NewPocUsernameBeneficiaryEvent {
    pub event_type: String,
    pub beneficiary_id: Option<String>,
    pub username: Option<String>,
    pub payload_json: serde_json::Value,
    pub transaction_id: String,
    pub event_id: String,
    pub time: chrono::DateTime<chrono::Utc>,
}
