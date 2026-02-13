// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel_migrations::EmbeddedMigrations;
use diesel_migrations::embed_migrations;

pub mod models;

pub const GOVERNANCE_STATUS_DELEGATE_REVIEW: i16 = 1;
pub const GOVERNANCE_STATUS_COMMUNITY_VOTING: i16 = 2;
pub const GOVERNANCE_STATUS_APPROVED: i16 = 3;
pub const GOVERNANCE_STATUS_REJECTED: i16 = 4;
pub const GOVERNANCE_STATUS_IMPLEMENTED: i16 = 5;
pub const GOVERNANCE_STATUS_OWNER_RESCINDED: i16 = 6;
pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
