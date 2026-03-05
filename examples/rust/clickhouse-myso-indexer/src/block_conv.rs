// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Result;
use clickhouse_native_client::column::nullable::ColumnNullable;
use clickhouse_native_client::column::numeric::{ColumnInt64, ColumnUInt64, ColumnUInt32, ColumnUInt8};
use clickhouse_native_client::column::string::ColumnString;
use clickhouse_native_client::types::Type;
use clickhouse_native_client::Block;
use std::sync::Arc;

use crate::handlers::StoredTransaction;

/// Convert a slice of StoredTransaction to a Block for INSERT into the transactions table.
pub fn stored_transactions_to_block(values: &[StoredTransaction]) -> Result<Block> {
    let n = values.len();
    let mut checkpoint_sequence_number = ColumnUInt64::with_capacity(n);
    let mut transaction_digest = ColumnString::new(Type::string());
    let mut sender = ColumnString::new(Type::string());
    let mut timestamp_ms = ColumnInt64::with_capacity(n);
    let mut tx_kind = ColumnString::new(Type::string());
    let mut gas_computation_cost = ColumnUInt64::with_capacity(n);
    let mut gas_storage_cost = ColumnUInt64::with_capacity(n);
    let mut gas_storage_rebate = ColumnUInt64::with_capacity(n);
    let mut status = ColumnUInt8::with_capacity(n);
    let mut epoch = ColumnUInt64::with_capacity(n);
    let mut gas_price = ColumnUInt64::with_capacity(n);
    let mut gas_budget = ColumnUInt64::with_capacity(n);
    let mut gas_owner = ColumnString::new(Type::string());
    let mut is_sponsored = ColumnUInt8::with_capacity(n);
    let mut created_objects = ColumnUInt32::with_capacity(n);
    let mut mutated_objects = ColumnUInt32::with_capacity(n);
    let mut execution_error = ColumnNullable::new(Type::nullable(Type::string()));

    for tx in values {
        checkpoint_sequence_number.append(tx.checkpoint_sequence_number);
        transaction_digest.append(tx.transaction_digest.clone());
        sender.append(tx.sender.clone());
        timestamp_ms.append(tx.timestamp_ms);
        tx_kind.append(tx.tx_kind.clone());
        gas_computation_cost.append(tx.gas_computation_cost);
        gas_storage_cost.append(tx.gas_storage_cost);
        gas_storage_rebate.append(tx.gas_storage_rebate);
        status.append(tx.status);
        epoch.append(tx.epoch);
        gas_price.append(tx.gas_price);
        gas_budget.append(tx.gas_budget);
        gas_owner.append(tx.gas_owner.clone());
        is_sponsored.append(tx.is_sponsored);
        created_objects.append(tx.created_objects);
        mutated_objects.append(tx.mutated_objects);
        match &tx.execution_error {
            None => execution_error.append_null(),
            Some(s) => {
                execution_error.append_non_null();
                execution_error
                    .nested_mut::<ColumnString>()
                    .append(s.clone());
            }
        }
    }

    let mut block = Block::new();
    block.append_column("checkpoint_sequence_number", Arc::new(checkpoint_sequence_number))?;
    block.append_column("transaction_digest", Arc::new(transaction_digest))?;
    block.append_column("sender", Arc::new(sender))?;
    block.append_column("timestamp_ms", Arc::new(timestamp_ms))?;
    block.append_column("tx_kind", Arc::new(tx_kind))?;
    block.append_column("gas_computation_cost", Arc::new(gas_computation_cost))?;
    block.append_column("gas_storage_cost", Arc::new(gas_storage_cost))?;
    block.append_column("gas_storage_rebate", Arc::new(gas_storage_rebate))?;
    block.append_column("status", Arc::new(status))?;
    block.append_column("epoch", Arc::new(epoch))?;
    block.append_column("gas_price", Arc::new(gas_price))?;
    block.append_column("gas_budget", Arc::new(gas_budget))?;
    block.append_column("gas_owner", Arc::new(gas_owner))?;
    block.append_column("is_sponsored", Arc::new(is_sponsored))?;
    block.append_column("created_objects", Arc::new(created_objects))?;
    block.append_column("mutated_objects", Arc::new(mutated_objects))?;
    block.append_column("execution_error", Arc::new(execution_error))?;

    Ok(block)
}

/// Watermark row for init and set_committer_watermark.
pub struct WatermarkRow {
    pub pipeline: String,
    pub epoch_hi_inclusive: u64,
    pub checkpoint_hi_inclusive: u64,
    pub tx_hi: u64,
    pub timestamp_ms_hi_inclusive: u64,
    pub reader_lo: u64,
    pub pruner_hi: u64,
    pub pruner_timestamp: u64,
}

/// Convert a WatermarkRow to a Block for INSERT into the watermarks table.
pub fn watermark_row_to_block(row: WatermarkRow) -> Result<Block> {
    let mut pipeline = ColumnString::new(Type::string());
    pipeline.append(row.pipeline);
    let mut epoch_hi_inclusive = ColumnUInt64::new();
    epoch_hi_inclusive.append(row.epoch_hi_inclusive);
    let mut checkpoint_hi_inclusive = ColumnUInt64::new();
    checkpoint_hi_inclusive.append(row.checkpoint_hi_inclusive);
    let mut tx_hi = ColumnUInt64::new();
    tx_hi.append(row.tx_hi);
    let mut timestamp_ms_hi_inclusive = ColumnUInt64::new();
    timestamp_ms_hi_inclusive.append(row.timestamp_ms_hi_inclusive);
    let mut reader_lo = ColumnUInt64::new();
    reader_lo.append(row.reader_lo);
    let mut pruner_hi = ColumnUInt64::new();
    pruner_hi.append(row.pruner_hi);
    let mut pruner_timestamp = ColumnUInt64::new();
    pruner_timestamp.append(row.pruner_timestamp);

    let mut block = Block::new();
    block.append_column("pipeline", Arc::new(pipeline))?;
    block.append_column("epoch_hi_inclusive", Arc::new(epoch_hi_inclusive))?;
    block.append_column("checkpoint_hi_inclusive", Arc::new(checkpoint_hi_inclusive))?;
    block.append_column("tx_hi", Arc::new(tx_hi))?;
    block.append_column("timestamp_ms_hi_inclusive", Arc::new(timestamp_ms_hi_inclusive))?;
    block.append_column("reader_lo", Arc::new(reader_lo))?;
    block.append_column("pruner_hi", Arc::new(pruner_hi))?;
    block.append_column("pruner_timestamp", Arc::new(pruner_timestamp))?;

    Ok(block)
}

/// Extract (epoch_hi, checkpoint_hi, tx_hi, timestamp_hi) from the first block of a query result.
pub fn block_to_committer_watermark(block: &Block) -> Option<(u64, u64, u64, u64)> {
    if block.row_count() == 0 {
        return None;
    }
    let epoch_col = block.column_by_name("epoch_hi_inclusive")?;
    let checkpoint_col = block.column_by_name("checkpoint_hi_inclusive")?;
    let tx_col = block.column_by_name("tx_hi")?;
    let timestamp_col = block.column_by_name("timestamp_ms_hi_inclusive")?;

    let epoch = *epoch_col.as_ref().as_any().downcast_ref::<ColumnUInt64>()?.get(0)?;
    let checkpoint = *checkpoint_col.as_ref().as_any().downcast_ref::<ColumnUInt64>()?.get(0)?;
    let tx = *tx_col.as_ref().as_any().downcast_ref::<ColumnUInt64>()?.get(0)?;
    let timestamp = *timestamp_col.as_ref().as_any().downcast_ref::<ColumnUInt64>()?.get(0)?;

    Some((epoch, checkpoint, tx, timestamp))
}

/// Extract (checkpoint_hi, reader_lo) from the first block.
pub fn block_to_reader_watermark(block: &Block) -> Option<(u64, u64)> {
    if block.row_count() == 0 {
        return None;
    }
    let checkpoint_col = block.column_by_name("checkpoint_hi_inclusive")?;
    let reader_col = block.column_by_name("reader_lo")?;

    let checkpoint = *checkpoint_col.as_ref().as_any().downcast_ref::<ColumnUInt64>()?.get(0)?;
    let reader = *reader_col.as_ref().as_any().downcast_ref::<ColumnUInt64>()?.get(0)?;

    Some((checkpoint, reader))
}

/// Extract (reader_lo, pruner_hi, wait_for_ms) from the first block.
pub fn block_to_pruner_watermark(block: &Block) -> Option<(u64, u64, i64)> {
    if block.row_count() == 0 {
        return None;
    }
    let reader_col = block.column_by_name("reader_lo")?;
    let pruner_col = block.column_by_name("pruner_hi")?;
    let wait_col = block.column_by_name("wait_for_ms")?;

    let reader = *reader_col.as_ref().as_any().downcast_ref::<ColumnUInt64>()?.get(0)?;
    let pruner = *pruner_col.as_ref().as_any().downcast_ref::<ColumnUInt64>()?.get(0)?;
    let wait = *wait_col.as_ref().as_any().downcast_ref::<ColumnInt64>()?.get(0)?;

    Some((reader, pruner, wait))
}

/// Extract checkpoint_hi_inclusive (u64) from the first block.
pub fn block_to_checkpoint(block: &Block) -> Option<u64> {
    if block.row_count() == 0 {
        return None;
    }
    let col = block.column_by_name("checkpoint_hi_inclusive")?;
    Some(*col.as_ref().as_any().downcast_ref::<ColumnUInt64>()?.get(0)?)
}
