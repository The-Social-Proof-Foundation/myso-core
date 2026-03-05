// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use async_trait::async_trait;
use chrono::Utc;
use clickhouse_native_client::ClientOptions;
use scoped_futures::ScopedBoxFuture;
use std::sync::Arc;
use std::time::Duration;
use myso_indexer_alt_framework::store::{
    CommitterWatermark, Connection, PrunerWatermark, ReaderWatermark, Store, TransactionalStore,
};

use crate::block_conv::{block_to_checkpoint, block_to_committer_watermark, block_to_reader_watermark,
                        block_to_pruner_watermark, watermark_row_to_block, WatermarkRow};
use crate::native_bridge::NativeClientBridge;

#[derive(Clone)]
pub struct ClickHouseStore {
    bridge: Arc<NativeClientBridge>,
}

pub struct ClickHouseConnection {
    pub bridge: Arc<NativeClientBridge>,
}

impl ClickHouseStore {
    pub fn new(host: &str, port: u16, user: &str) -> Self {
        let opts = Self::client_options(host, port, user);
        let bridge = NativeClientBridge::new(opts).expect("ClickHouse connection");
        Self {
            bridge: Arc::new(bridge),
        }
    }

    fn client_options(host: &str, port: u16, user: &str) -> ClientOptions {
        ClientOptions::new(host.to_string(), port)
            .database("default")
            .user(user.to_string())
            .compression(None)
    }

    /// Create tables if they don't exist
    pub async fn create_tables_if_not_exists(&self) -> Result<()> {
        self.bridge
            .execute(
                "
                CREATE TABLE IF NOT EXISTS watermarks
                (
                    pipeline String,
                    epoch_hi_inclusive UInt64,
                    checkpoint_hi_inclusive UInt64,
                    tx_hi UInt64,
                    timestamp_ms_hi_inclusive UInt64,
                    reader_lo UInt64,
                    pruner_hi UInt64,
                    pruner_timestamp UInt64
                )
                ENGINE = MergeTree()
                ORDER BY pipeline
                ",
            )
            .await?;

        self.bridge
            .execute(
                "
                CREATE TABLE IF NOT EXISTS transactions
                (
                    checkpoint_sequence_number UInt64,
                    transaction_digest String,
                    sender String,
                    timestamp_ms Int64,
                    tx_kind LowCardinality(String),
                    gas_computation_cost UInt64,
                    gas_storage_cost UInt64,
                    gas_storage_rebate UInt64,
                    status UInt8,
                    epoch UInt64,
                    gas_price UInt64,
                    gas_budget UInt64,
                    gas_owner String,
                    is_sponsored UInt8,
                    created_objects UInt32,
                    mutated_objects UInt32,
                    execution_error Nullable(String),
                    indexed_at DateTime64(3, 'UTC') DEFAULT now()
                )
                ENGINE = MergeTree()
                ORDER BY (checkpoint_sequence_number, transaction_digest)
                ",
            )
            .await?;

        Ok(())
    }

    /// Drop watermarks and transactions tables for a clean reset.
    pub async fn reset_tables(&self) -> Result<()> {
        self.bridge.execute("DROP TABLE IF EXISTS watermarks").await?;
        self.bridge.execute("DROP TABLE IF EXISTS transactions").await?;
        Ok(())
    }
}

#[async_trait]
impl Store for ClickHouseStore {
    type Connection<'c> = ClickHouseConnection;

    async fn connect<'c>(&'c self) -> Result<Self::Connection<'c>> {
        Ok(ClickHouseConnection {
            bridge: self.bridge.clone(),
        })
    }
}

#[async_trait]
impl TransactionalStore for ClickHouseStore {
    async fn transaction<'a, R, F>(&self, f: F) -> anyhow::Result<R>
    where
        R: Send + 'a,
        F: Send + 'a,
        F: for<'r> FnOnce(
            &'r mut Self::Connection<'_>,
        ) -> ScopedBoxFuture<'a, 'r, anyhow::Result<R>>,
    {
        let mut conn = self.connect().await?;
        f(&mut conn).await
    }
}

fn escape_sql_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('\'', "\\'")
}

#[async_trait]
impl Connection for ClickHouseConnection {
    async fn init_watermark(
        &mut self,
        pipeline_task: &str,
        default_next_checkpoint: u64,
    ) -> anyhow::Result<Option<u64>> {
        let existing = self.committer_watermark(pipeline_task).await?;

        let Some(checkpoint_hi_inclusive) = default_next_checkpoint.checked_sub(1) else {
            return Ok(existing.map(|w| w.checkpoint_hi_inclusive));
        };

        if let Some(existing) = existing {
            return Ok(Some(existing.checkpoint_hi_inclusive));
        }

        let block = watermark_row_to_block(WatermarkRow {
            pipeline: pipeline_task.to_string(),
            epoch_hi_inclusive: 0,
            checkpoint_hi_inclusive,
            tx_hi: 0,
            timestamp_ms_hi_inclusive: 0,
            reader_lo: default_next_checkpoint,
            pruner_hi: default_next_checkpoint,
            pruner_timestamp: 0,
        })?;
        self.bridge.insert("watermarks", block).await?;
        Ok(Some(checkpoint_hi_inclusive))
    }

    async fn committer_watermark(&mut self, pipeline: &str) -> Result<Option<CommitterWatermark>> {
        let escaped = escape_sql_string(pipeline);
        let query = format!(
            "SELECT epoch_hi_inclusive, checkpoint_hi_inclusive, tx_hi, timestamp_ms_hi_inclusive
             FROM watermarks
             WHERE pipeline = '{}'
             ORDER BY pruner_timestamp DESC
             LIMIT 1",
            escaped
        );
        let result = self.bridge.query(&query).await?;
        for block in &result.blocks {
            if let Some((epoch_hi, checkpoint_hi, tx_hi, timestamp_hi)) =
                block_to_committer_watermark(block)
            {
                return Ok(Some(CommitterWatermark {
                    epoch_hi_inclusive: epoch_hi,
                    checkpoint_hi_inclusive: checkpoint_hi,
                    tx_hi,
                    timestamp_ms_hi_inclusive: timestamp_hi,
                }));
            }
        }
        Ok(None)
    }

    async fn reader_watermark(
        &mut self,
        pipeline: &'static str,
    ) -> Result<Option<ReaderWatermark>> {
        let escaped = escape_sql_string(pipeline);
        let query = format!(
            "SELECT checkpoint_hi_inclusive, reader_lo
             FROM watermarks
             WHERE pipeline = '{}'
             ORDER BY pruner_timestamp DESC
             LIMIT 1",
            escaped
        );
        let result = self.bridge.query(&query).await?;
        for block in &result.blocks {
            if let Some((checkpoint_hi, reader_lo)) = block_to_reader_watermark(block) {
                return Ok(Some(ReaderWatermark {
                    checkpoint_hi_inclusive: checkpoint_hi,
                    reader_lo,
                }));
            }
        }
        Ok(None)
    }

    async fn pruner_watermark(
        &mut self,
        pipeline: &'static str,
        delay: Duration,
    ) -> Result<Option<PrunerWatermark>> {
        let delay_ms = delay.as_millis() as i64;
        let escaped = escape_sql_string(pipeline);
        let query = format!(
            "SELECT reader_lo, pruner_hi,
                    toInt64({} + (pruner_timestamp - toUnixTimestamp64Milli(now64()))) as wait_for_ms
             FROM watermarks
             WHERE pipeline = '{}'
             ORDER BY pruner_timestamp DESC
             LIMIT 1",
            delay_ms, escaped
        );
        let result = self.bridge.query(&query).await?;
        for block in &result.blocks {
            if let Some((reader_lo, pruner_hi, wait_for_ms)) = block_to_pruner_watermark(block) {
                return Ok(Some(PrunerWatermark {
                    wait_for_ms,
                    reader_lo,
                    pruner_hi,
                }));
            }
        }
        Ok(None)
    }

    async fn set_committer_watermark(
        &mut self,
        pipeline: &str,
        watermark: CommitterWatermark,
    ) -> Result<bool> {
        let escaped = escape_sql_string(pipeline);
        let check_query = format!(
            "SELECT checkpoint_hi_inclusive FROM watermarks WHERE pipeline = '{}' LIMIT 1",
            escaped
        );
        let result = self.bridge.query(&check_query).await?;
        let existing_checkpoint = result.blocks.first().and_then(block_to_checkpoint);

        if let Some(existing_checkpoint) = existing_checkpoint {
            if existing_checkpoint < watermark.checkpoint_hi_inclusive {
                let update = format!(
                    "ALTER TABLE watermarks UPDATE
                     epoch_hi_inclusive = {},
                     checkpoint_hi_inclusive = {},
                     tx_hi = {},
                     timestamp_ms_hi_inclusive = {}
                 WHERE pipeline = '{}'",
                    watermark.epoch_hi_inclusive,
                    watermark.checkpoint_hi_inclusive,
                    watermark.tx_hi,
                    watermark.timestamp_ms_hi_inclusive,
                    escaped
                );
                self.bridge.execute(&update).await?;
            }
        } else {
            let block = watermark_row_to_block(WatermarkRow {
                pipeline: pipeline.to_string(),
                epoch_hi_inclusive: watermark.epoch_hi_inclusive,
                checkpoint_hi_inclusive: watermark.checkpoint_hi_inclusive,
                tx_hi: watermark.tx_hi,
                timestamp_ms_hi_inclusive: watermark.timestamp_ms_hi_inclusive,
                reader_lo: 0,
                pruner_hi: 0,
                pruner_timestamp: Utc::now().timestamp_millis() as u64,
            })?;
            self.bridge.insert("watermarks", block).await?;
        }

        Ok(true)
    }

    async fn set_reader_watermark(
        &mut self,
        pipeline: &'static str,
        reader_lo: u64,
    ) -> Result<bool> {
        let escaped = escape_sql_string(pipeline);
        let update = format!(
            "ALTER TABLE watermarks
             UPDATE reader_lo = {}, pruner_timestamp = toUnixTimestamp64Milli(now64())
             WHERE pipeline = '{}' AND reader_lo < {}",
            reader_lo, escaped, reader_lo
        );
        self.bridge.execute(&update).await?;
        Ok(true)
    }

    async fn set_pruner_watermark(
        &mut self,
        pipeline: &'static str,
        pruner_hi: u64,
    ) -> Result<bool> {
        let escaped = escape_sql_string(pipeline);
        let update = format!(
            "ALTER TABLE watermarks UPDATE pruner_hi = {} WHERE pipeline = '{}'",
            pruner_hi, escaped
        );
        self.bridge.execute(&update).await?;
        Ok(true)
    }
}
