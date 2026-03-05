// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::io::ErrorKind;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use clickhouse_native_client::column::nullable::ColumnNullable;
use clickhouse_native_client::column::numeric::{ColumnInt64, ColumnUInt32, ColumnUInt8};
use clickhouse_native_client::column::numeric::ColumnUInt64;
use clickhouse_native_client::column::string::ColumnString;
use clickhouse_native_client::types::Type;
use clickhouse_native_client::{Block, Client, ClientOptions};
use reqwest::Client as ReqwestClient;
use tokio::sync::oneshot;

use crate::handlers::CheckpointRows;
use crate::handlers::Row;
use crate::schema::ColumnValue;

const OPERATION_TIMEOUT: Duration = Duration::from_secs(300);

type ClientResult<T> = Result<T, clickhouse_native_client::Error>;

pub struct QueryResponse {
    pub blocks: Vec<Block>,
}

enum Request {
    Execute { query: String, tx: oneshot::Sender<ClientResult<()>> },
    Query { query: String, tx: oneshot::Sender<ClientResult<QueryResponse>> },
}

fn is_recoverable(e: &clickhouse_native_client::Error) -> bool {
    use clickhouse_native_client::Error;
    match e {
        Error::Connection(_) => true,
        Error::Io(io) => matches!(
            io.kind(),
            ErrorKind::BrokenPipe
                | ErrorKind::ConnectionReset
                | ErrorKind::ConnectionAborted
                | ErrorKind::TimedOut
        ),
        _ => false,
    }
}

pub struct NativeClientBridge {
    tx: mpsc::Sender<Request>,
    http_client: ReqwestClient,
    http_base: String,
}

fn http_port(native_port: u16) -> u16 {
    if native_port == 9000 {
        8123
    } else {
        native_port
    }
}

impl NativeClientBridge {
    pub fn new(host: &str, port: u16, opts: ClientOptions) -> ClientResult<Self> {
        let opts_for_reconnect = opts.clone();
        let (req_tx, req_rx) = mpsc::channel();
        let http_base = format!("http://{}:{}", host, http_port(port));
        let http_client = ReqwestClient::builder()
            .timeout(OPERATION_TIMEOUT)
            .build()
            .map_err(|e| clickhouse_native_client::Error::Connection(e.to_string().into()))?;

        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("runtime");
            let mut client = match rt.block_on(Client::connect(opts_for_reconnect.clone())) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("ClickHouse connect failed: {e}");
                    return;
                }
            };
            for req in req_rx {
                let should_reconnect = match req {
                    Request::Execute { query, tx } => {
                        let r = rt.block_on(client.execute(query.as_str()));
                        let reconnect = r.as_ref().err().is_some_and(is_recoverable);
                        let _ = tx.send(r);
                        reconnect
                    }
                    Request::Query { query, tx } => {
                        let r = rt.block_on(client.query(query.as_str()));
                        let reconnect = r.as_ref().err().is_some_and(is_recoverable);
                        let response = r.map(|result| QueryResponse { blocks: result.blocks });
                        let _ = tx.send(response);
                        reconnect
                    }
                };
                if should_reconnect
                    && let Ok(c) = rt.block_on(Client::connect(opts_for_reconnect.clone()))
                {
                    client = c;
                }
            }
        });

        Ok(Self {
            tx: req_tx,
            http_client,
            http_base,
        })
    }

    pub async fn query(&self, query: &str) -> ClientResult<QueryResponse> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Request::Query {
                query: query.to_string(),
                tx,
            })
            .map_err(|_| clickhouse_native_client::Error::Connection("channel closed".into()))?;
        tokio::time::timeout(OPERATION_TIMEOUT, rx)
            .await
            .map_err(|_| clickhouse_native_client::Error::Connection("operation timeout".into()))?
            .map_err(|_| clickhouse_native_client::Error::Connection("channel closed".into()))?
    }

    pub async fn execute(&self, query: &str) -> ClientResult<()> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Request::Execute {
                query: query.to_string(),
                tx,
            })
            .map_err(|_| clickhouse_native_client::Error::Connection("channel closed".into()))?;
        tokio::time::timeout(OPERATION_TIMEOUT, rx)
            .await
            .map_err(|_| clickhouse_native_client::Error::Connection("operation timeout".into()))?
            .map_err(|_| clickhouse_native_client::Error::Connection("channel closed".into()))?
    }

    pub async fn insert_http(&self, table: &str, block: &Block) -> Result<()> {
        let rows = block.row_count();
        if rows == 0 {
            return Ok(());
        }
        let mut lines = Vec::with_capacity(rows);
        let cp_col = block
            .column_by_name("checkpoint_sequence_number")
            .ok_or_else(|| anyhow::anyhow!("missing checkpoint_sequence_number"))?;
        let digest_col = block
            .column_by_name("transaction_digest")
            .ok_or_else(|| anyhow::anyhow!("missing transaction_digest"))?;
        let sender_col = block
            .column_by_name("sender")
            .ok_or_else(|| anyhow::anyhow!("missing sender"))?;
        let ts_col = block
            .column_by_name("timestamp_ms")
            .ok_or_else(|| anyhow::anyhow!("missing timestamp_ms"))?;
        let kind_col = block
            .column_by_name("tx_kind")
            .ok_or_else(|| anyhow::anyhow!("missing tx_kind"))?;
        let comp_col = block
            .column_by_name("gas_computation_cost")
            .ok_or_else(|| anyhow::anyhow!("missing gas_computation_cost"))?;
        let stor_col = block
            .column_by_name("gas_storage_cost")
            .ok_or_else(|| anyhow::anyhow!("missing gas_storage_cost"))?;
        let rebate_col = block
            .column_by_name("gas_storage_rebate")
            .ok_or_else(|| anyhow::anyhow!("missing gas_storage_rebate"))?;
        let status_col = block
            .column_by_name("status")
            .ok_or_else(|| anyhow::anyhow!("missing status"))?;
        let epoch_col = block
            .column_by_name("epoch")
            .ok_or_else(|| anyhow::anyhow!("missing epoch"))?;
        let price_col = block
            .column_by_name("gas_price")
            .ok_or_else(|| anyhow::anyhow!("missing gas_price"))?;
        let budget_col = block
            .column_by_name("gas_budget")
            .ok_or_else(|| anyhow::anyhow!("missing gas_budget"))?;
        let owner_col = block
            .column_by_name("gas_owner")
            .ok_or_else(|| anyhow::anyhow!("missing gas_owner"))?;
        let sponsored_col = block
            .column_by_name("is_sponsored")
            .ok_or_else(|| anyhow::anyhow!("missing is_sponsored"))?;
        let created_col = block
            .column_by_name("created_objects")
            .ok_or_else(|| anyhow::anyhow!("missing created_objects"))?;
        let mutated_col = block
            .column_by_name("mutated_objects")
            .ok_or_else(|| anyhow::anyhow!("missing mutated_objects"))?;

        for i in 0..rows {
            let checkpoint = cp_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt64>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let digest = digest_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnString>()
                .map(|c| c.get(i).unwrap_or("").to_string())
                .unwrap_or_default();
            let sender = sender_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnString>()
                .map(|c| c.get(i).unwrap_or("").to_string())
                .unwrap_or_default();
            let ts = ts_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnInt64>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let kind = kind_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnString>()
                .map(|c| c.get(i).unwrap_or("").to_string())
                .unwrap_or_default();
            let comp = comp_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt64>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let stor = stor_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt64>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let rebate = rebate_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt64>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let status = status_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt8>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let epoch = epoch_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt64>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let price = price_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt64>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let budget = budget_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt64>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let owner = owner_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnString>()
                .map(|c| c.get(i).unwrap_or("").to_string())
                .unwrap_or_default();
            let sponsored = sponsored_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt8>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let created = created_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt32>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);
            let mutated = mutated_col
                .as_ref()
                .as_any()
                .downcast_ref::<ColumnUInt32>()
                .and_then(|c| c.get(i))
                .copied()
                .unwrap_or(0);

            let json = serde_json::json!({
                "checkpoint_sequence_number": checkpoint,
                "transaction_digest": digest,
                "sender": sender,
                "timestamp_ms": ts,
                "tx_kind": kind,
                "gas_computation_cost": comp,
                "gas_storage_cost": stor,
                "gas_storage_rebate": rebate,
                "status": status,
                "epoch": epoch,
                "gas_price": price,
                "gas_budget": budget,
                "gas_owner": owner,
                "is_sponsored": sponsored,
                "created_objects": created,
                "mutated_objects": mutated,
                "execution_error": serde_json::Value::Null
            });
            lines.push(json.to_string());
        }

        let body = lines.join("\n");
        let query = format!("INSERT INTO {} FORMAT JSONEachRow", table);
        let url = format!("{}/", self.http_base);
        let resp = self
            .http_client
            .post(&url)
            .query(&[("query", query)])
            .body(body)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("ClickHouse HTTP insert: {}", e))?;
        if !resp.status().is_success() {
            let status = resp.status();
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow::anyhow!(
                "ClickHouse HTTP insert: {} {}",
                status,
                body
            ));
        }
        Ok(())
    }
}

pub fn create_client_options(host: &str, port: u16, user: &str) -> ClientOptions {
    ClientOptions::new(host.to_string(), port)
        .database("default")
        .user(user.to_string())
        .compression(None)
}

pub fn transaction_rows_to_block(checkpoints: &[CheckpointRows], schema: &[&str]) -> Result<Block> {
    let n: usize = checkpoints.iter().map(|c| c.len()).sum();
    if n == 0 {
        return Ok(Block::new());
    }

    let mut checkpoint_sequence_number = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut transaction_digest = ColumnString::new(Type::string());
    let mut sender = ColumnString::new(Type::string());
    let mut timestamp_ms = ColumnInt64::with_capacity(n);
    let mut tx_kind = ColumnString::new(Type::string());
    let mut gas_computation_cost = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut gas_storage_cost = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut gas_storage_rebate = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut status = ColumnUInt8::with_capacity(n);
    let mut epoch = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut gas_price = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut gas_budget = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut gas_owner = ColumnString::new(Type::string());
    let mut is_sponsored = ColumnUInt8::with_capacity(n);
    let mut created_objects = ColumnUInt32::with_capacity(n);
    let mut mutated_objects = ColumnUInt32::with_capacity(n);
    let mut execution_error = ColumnNullable::new(Type::nullable(Type::string()));

    fn col_idx(schema: &[&str], name: &str) -> Option<usize> {
        schema.iter().position(|s| *s == name)
    }

    fn get_u64(row: &dyn Row, schema: &[&str], name: &str) -> u64 {
        col_idx(schema, name)
            .and_then(|i| row.get_column(i).ok())
            .and_then(|v| match v {
                ColumnValue::U64(x) => Some(x),
                _ => None,
            })
            .unwrap_or(0)
    }

    fn get_i64(row: &dyn Row, schema: &[&str], name: &str) -> i64 {
        col_idx(schema, name)
            .and_then(|i| row.get_column(i).ok())
            .and_then(|v| match v {
                ColumnValue::I64(x) => Some(x),
                ColumnValue::U64(x) => Some(x as i64),
                _ => None,
            })
            .unwrap_or(0)
    }

    fn get_str(row: &dyn Row, schema: &[&str], name: &str) -> String {
        col_idx(schema, name)
            .and_then(|i| row.get_column(i).ok())
            .map(|v| match v {
                ColumnValue::Str(s) => s.to_string(),
                ColumnValue::OptionStr(Some(s)) => s.to_string(),
                _ => String::new(),
            })
            .unwrap_or_default()
    }

    fn get_bool(row: &dyn Row, schema: &[&str], name: &str) -> bool {
        col_idx(schema, name)
            .and_then(|i| row.get_column(i).ok())
            .and_then(|v| match v {
                ColumnValue::Bool(b) => Some(b),
                _ => None,
            })
            .unwrap_or(false)
    }

    for cp in checkpoints {
        for row in cp.iter() {
            let checkpoint = get_u64(row, schema, "checkpoint");
            let digest = get_str(row, schema, "transaction_digest");
            let s = get_str(row, schema, "sender");
            let ts = get_i64(row, schema, "timestamp_ms");
            let kind = get_str(row, schema, "transaction_kind");
            let comp = get_u64(row, schema, "computation_cost");
            let stor = get_u64(row, schema, "storage_cost");
            let rebate = get_u64(row, schema, "storage_rebate");
            let exec_ok = get_bool(row, schema, "execution_success");
            let ep = get_u64(row, schema, "epoch");
            let price = get_u64(row, schema, "gas_price");
            let budget = get_u64(row, schema, "gas_budget");
            let owner = get_str(row, schema, "gas_owner");
            let sponsored = get_bool(row, schema, "is_sponsored_tx");
            let created = get_u64(row, schema, "created") as u32;
            let mutated = get_u64(row, schema, "mutated") as u32;

            checkpoint_sequence_number.append(checkpoint);
            transaction_digest.append(digest);
            sender.append(s);
            timestamp_ms.append(ts);
            tx_kind.append(kind);
            gas_computation_cost.append(comp);
            gas_storage_cost.append(stor);
            gas_storage_rebate.append(rebate);
            status.append(if exec_ok { 0u8 } else { 1u8 });
            epoch.append(ep);
            gas_price.append(price);
            gas_budget.append(budget);
            gas_owner.append(owner);
            is_sponsored.append(sponsored as u8);
            created_objects.append(created);
            mutated_objects.append(mutated);
            execution_error.append_null();
        }
    }

    let mut block = Block::new();
    block.append_column("checkpoint_sequence_number", std::sync::Arc::new(checkpoint_sequence_number))?;
    block.append_column("transaction_digest", std::sync::Arc::new(transaction_digest))?;
    block.append_column("sender", std::sync::Arc::new(sender))?;
    block.append_column("timestamp_ms", std::sync::Arc::new(timestamp_ms))?;
    block.append_column("tx_kind", std::sync::Arc::new(tx_kind))?;
    block.append_column("gas_computation_cost", std::sync::Arc::new(gas_computation_cost))?;
    block.append_column("gas_storage_cost", std::sync::Arc::new(gas_storage_cost))?;
    block.append_column("gas_storage_rebate", std::sync::Arc::new(gas_storage_rebate))?;
    block.append_column("status", std::sync::Arc::new(status))?;
    block.append_column("epoch", std::sync::Arc::new(epoch))?;
    block.append_column("gas_price", std::sync::Arc::new(gas_price))?;
    block.append_column("gas_budget", std::sync::Arc::new(gas_budget))?;
    block.append_column("gas_owner", std::sync::Arc::new(gas_owner))?;
    block.append_column("is_sponsored", std::sync::Arc::new(is_sponsored))?;
    block.append_column("created_objects", std::sync::Arc::new(created_objects))?;
    block.append_column("mutated_objects", std::sync::Arc::new(mutated_objects))?;
    block.append_column("execution_error", std::sync::Arc::new(execution_error))?;

    Ok(block)
}
