// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::embed_migrations;

pub mod models;

pub const GOVERNANCE_STATUS_SUBMITTED: i16 = 0;
pub const GOVERNANCE_STATUS_DELEGATE_REVIEW: i16 = 1;
pub const GOVERNANCE_STATUS_COMMUNITY_VOTING: i16 = 2;
pub const GOVERNANCE_STATUS_APPROVED: i16 = 3;
pub const GOVERNANCE_STATUS_REJECTED: i16 = 4;
pub const GOVERNANCE_STATUS_IMPLEMENTED: i16 = 5;
pub const GOVERNANCE_STATUS_OWNER_RESCINDED: i16 = 6;

/// Proposal types (must match governance.move)
pub const PROPOSAL_TYPE_ECOSYSTEM: i16 = 0;
pub const PROPOSAL_TYPE_PROOF_OF_CREATIVITY: i16 = 1;
pub const PROPOSAL_TYPE_PLATFORM: i16 = 3;

pub const NOMINEE_STATUS_PENDING: i16 = 0;
pub const NOMINEE_STATUS_ELECTED: i16 = 1;

pub const ANONYMOUS_VOTE_STATUS_PENDING: i16 = 0;
pub const ANONYMOUS_VOTE_STATUS_SUCCESS: i16 = 1;
pub const ANONYMOUS_VOTE_STATUS_FAILED: i16 = 2;

pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
