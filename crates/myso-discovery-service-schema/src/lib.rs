// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! SQLx migrations for the discovery service operational database.
//!
//! When the discovery schema shares a database with other migration sets (e.g.
//! `spot_oracle` also runs `myso-spot-oracle-schema`), `ignore_missing` allows
//! other crates' versions in `_sqlx_migrations` without failing validation.

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
