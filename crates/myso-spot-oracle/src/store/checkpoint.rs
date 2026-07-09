// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use sqlx::PgPool;

pub struct CheckpointIngestStore {
    pool: PgPool,
}

impl CheckpointIngestStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_watermark(&self) -> anyhow::Result<u64> {
        let row: (i64,) =
            sqlx::query_as("SELECT last_checkpoint_seq FROM checkpoint_ingest_state WHERE id = 1")
                .fetch_one(&self.pool)
                .await?;
        Ok(row.0 as u64)
    }

    pub async fn set_watermark(&self, seq: u64) -> anyhow::Result<()> {
        sqlx::query(
            "UPDATE checkpoint_ingest_state SET last_checkpoint_seq = $1, updated_at = NOW() WHERE id = 1",
        )
        .bind(seq as i64)
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}
