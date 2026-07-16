// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! SQLx migrations for the SPoT oracle operational database (`spot_oracle`).
//! Spot-owned only — does not require Discovery schema migrations.
//!
//! Rebuild this crate (`cargo build -p myso-spot-oracle`) after editing migration files
//! so `sqlx::migrate!` re-embeds them.

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

pub async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), sqlx::migrate::MigrateError> {
    MIGRATOR.run(pool).await
}

/// Fail fast when the knowledge-graph migration was not embedded/applied.
pub async fn verify_knowledge_tables(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let exists: bool = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM information_schema.tables
            WHERE table_schema = 'public' AND table_name = 'knowledge_entities'
        )
        "#,
    )
    .fetch_one(pool)
    .await?;
    if exists {
        return Ok(());
    }
    anyhow::bail!(
        "knowledge_entities table is missing after migrations. \
         Rebuild the schema crate so sqlx embeds the latest migrations, then restart: \
         cargo build -p myso-spot-oracle-schema -p myso-spot-oracle"
    );
}
