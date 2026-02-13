// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel_migrations::embed_migrations;
use diesel_migrations::EmbeddedMigrations;

pub mod schema;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
