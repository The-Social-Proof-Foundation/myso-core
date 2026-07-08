// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! SQLx migrations for the SPoT oracle operational database (`spot_oracle`).
//! Run after `myso_discovery_service_schema::run_migrations` so the shared
//! `discovery_sources` table exists in the same DB.
//!
//! The spot_oracle DB shares `_sqlx_migrations` with discovery migrations applied
//! first (version `20260708000000`). Spot migrations use a distinct version and
//! `ignore_missing` so discovery rows in the table are not treated as errors.

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate::Migrator {
        migrations: MIGRATOR.migrations.clone(),
        ignore_missing: true,
        locking: MIGRATOR.locking,
        no_tx: MIGRATOR.no_tx,
    }
    .run(pool)
    .await
}
