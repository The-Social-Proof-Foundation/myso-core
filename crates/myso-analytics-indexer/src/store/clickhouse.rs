// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::io::ErrorKind;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use clickhouse_native_client::column::nullable::ColumnNullable;
use clickhouse_native_client::column::numeric::ColumnUInt64;
use clickhouse_native_client::column::numeric::{ColumnInt64, ColumnUInt8, ColumnUInt32};
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
    Execute {
        query: String,
        tx: oneshot::Sender<ClientResult<()>>,
    },
    Query {
        query: String,
        tx: oneshot::Sender<ClientResult<QueryResponse>>,
    },
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
    http_user: String,
    http_password: String,
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
        let http_user = opts.user.clone();
        let http_password = opts.password.clone();
        let http_client = ReqwestClient::builder()
            .timeout(OPERATION_TIMEOUT)
            .build()
            .map_err(|e| clickhouse_native_client::Error::Connection(e.to_string()))?;

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
                        let response = r.map(|result| QueryResponse {
                            blocks: result.blocks,
                        });
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
            http_user,
            http_password,
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
        let lines = block_to_json_each_row(block)?;
        if lines.is_empty() {
            return Ok(());
        }
        let body = lines.join("\n");
        let query = format!("INSERT INTO {} FORMAT JSONEachRow", table);
        let url = format!("{}/", self.http_base);
        let mut req = self
            .http_client
            .post(&url)
            .query(&[("query", query)])
            .body(body);
        if !self.http_password.is_empty() {
            req = req.basic_auth(&self.http_user, Some(&self.http_password));
        }
        let resp = req
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

fn extract_column_value(
    col: &std::sync::Arc<dyn clickhouse_native_client::column::Column>,
    row: usize,
) -> serde_json::Value {
    let any = col.as_ref().as_any();
    if let Some(c) = any.downcast_ref::<ColumnUInt64>() {
        serde_json::json!(c.get(row).copied().unwrap_or(0))
    } else if let Some(c) = any.downcast_ref::<ColumnInt64>() {
        serde_json::json!(c.get(row).copied().unwrap_or(0))
    } else if let Some(c) = any.downcast_ref::<ColumnUInt8>() {
        serde_json::json!(c.get(row).copied().unwrap_or(0))
    } else if let Some(c) = any.downcast_ref::<ColumnUInt32>() {
        serde_json::json!(c.get(row).copied().unwrap_or(0))
    } else if let Some(c) = any.downcast_ref::<ColumnString>() {
        serde_json::json!(c.get(row).map(|s| s.to_string()).unwrap_or_default())
    } else if let Some(nullable) = any.downcast_ref::<ColumnNullable>() {
        if nullable.is_null(row) {
            serde_json::Value::Null
        } else {
            extract_column_value(&nullable.nested_ref(), row)
        }
    } else {
        serde_json::Value::Null
    }
}

fn block_to_json_each_row(block: &Block) -> Result<Vec<String>> {
    let rows = block.row_count();
    if rows == 0 {
        return Ok(vec![]);
    }
    let col_count = block.column_count();
    let mut col_names = Vec::with_capacity(col_count);
    let mut col_refs = Vec::with_capacity(col_count);
    for i in 0..col_count {
        let name = block
            .column_name(i)
            .ok_or_else(|| anyhow::anyhow!("missing column name at index {}", i))?
            .to_string();
        let col = block
            .column(i)
            .ok_or_else(|| anyhow::anyhow!("missing column at index {}", i))?;
        col_names.push(name);
        col_refs.push(col);
    }
    let mut lines = Vec::with_capacity(rows);
    for row in 0..rows {
        let mut obj = serde_json::Map::new();
        for (name, col) in col_names.iter().zip(col_refs.iter()) {
            let val = extract_column_value(col, row);
            obj.insert(name.clone(), val);
        }
        lines.push(serde_json::Value::Object(obj).to_string());
    }
    Ok(lines)
}

pub fn create_client_options(
    host: &str,
    port: u16,
    user: &str,
    password: Option<&str>,
) -> ClientOptions {
    let mut opts = ClientOptions::new(host.to_string(), port)
        .database("default")
        .user(user.to_string())
        .compression(None);
    if let Some(pw) = password {
        opts = opts.password(pw);
    }
    opts
}

pub fn transaction_rows_to_block(checkpoints: &[CheckpointRows], schema: &[&str]) -> Result<Block> {
    let n: usize = checkpoints.iter().map(|c| c.len()).sum();
    if n == 0 {
        return Ok(Block::new());
    }

    let mut checkpoint_sequence_number =
        clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut transaction_digest = ColumnString::new(Type::string());
    let mut sender = ColumnString::new(Type::string());
    let mut timestamp_ms = ColumnInt64::with_capacity(n);
    let mut tx_kind = ColumnString::new(Type::string());
    let mut gas_computation_cost =
        clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut gas_storage_cost =
        clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut gas_storage_rebate =
        clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
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
    block.append_column(
        "checkpoint_sequence_number",
        std::sync::Arc::new(checkpoint_sequence_number),
    )?;
    block.append_column(
        "transaction_digest",
        std::sync::Arc::new(transaction_digest),
    )?;
    block.append_column("sender", std::sync::Arc::new(sender))?;
    block.append_column("timestamp_ms", std::sync::Arc::new(timestamp_ms))?;
    block.append_column("tx_kind", std::sync::Arc::new(tx_kind))?;
    block.append_column(
        "gas_computation_cost",
        std::sync::Arc::new(gas_computation_cost),
    )?;
    block.append_column("gas_storage_cost", std::sync::Arc::new(gas_storage_cost))?;
    block.append_column(
        "gas_storage_rebate",
        std::sync::Arc::new(gas_storage_rebate),
    )?;
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

pub fn event_rows_to_block(checkpoints: &[CheckpointRows], schema: &[&str]) -> Result<Block> {
    let n: usize = checkpoints.iter().map(|c| c.len()).sum();
    if n == 0 {
        return Ok(Block::new());
    }

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

    let mut checkpoint_sequence_number =
        clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut transaction_digest = ColumnString::new(Type::string());
    let mut event_index = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut epoch = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut timestamp_ms = ColumnInt64::with_capacity(n);
    let mut sender = ColumnString::new(Type::string());
    let mut package = ColumnString::new(Type::string());
    let mut module = ColumnString::new(Type::string());
    let mut event_type = ColumnString::new(Type::string());
    let mut event_json = ColumnString::new(Type::string());
    let mut bcs_length = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);

    for cp in checkpoints {
        for row in cp.iter() {
            let checkpoint = get_u64(row, schema, "checkpoint");
            let digest = get_str(row, schema, "transaction_digest");
            let ev_idx = get_u64(row, schema, "event_index");
            let ep = get_u64(row, schema, "epoch");
            let ts = get_u64(row, schema, "timestamp_ms") as i64;
            let s = get_str(row, schema, "sender");
            let pkg = get_str(row, schema, "package");
            let mod_ = get_str(row, schema, "module");
            let ev_type = get_str(row, schema, "event_type");
            let ev_json = get_str(row, schema, "event_json");
            let bcs_len = get_u64(row, schema, "bcs_length");

            checkpoint_sequence_number.append(checkpoint);
            transaction_digest.append(digest);
            event_index.append(ev_idx);
            epoch.append(ep);
            timestamp_ms.append(ts);
            sender.append(s);
            package.append(pkg);
            module.append(mod_);
            event_type.append(ev_type);
            event_json.append(ev_json);
            bcs_length.append(bcs_len);
        }
    }

    let mut block = Block::new();
    block.append_column(
        "checkpoint_sequence_number",
        std::sync::Arc::new(checkpoint_sequence_number),
    )?;
    block.append_column(
        "transaction_digest",
        std::sync::Arc::new(transaction_digest),
    )?;
    block.append_column("event_index", std::sync::Arc::new(event_index))?;
    block.append_column("epoch", std::sync::Arc::new(epoch))?;
    block.append_column("timestamp_ms", std::sync::Arc::new(timestamp_ms))?;
    block.append_column("sender", std::sync::Arc::new(sender))?;
    block.append_column("package", std::sync::Arc::new(package))?;
    block.append_column("module", std::sync::Arc::new(module))?;
    block.append_column("event_type", std::sync::Arc::new(event_type))?;
    block.append_column("event_json", std::sync::Arc::new(event_json))?;
    block.append_column("bcs_length", std::sync::Arc::new(bcs_length))?;

    Ok(block)
}

pub fn move_call_rows_to_block(checkpoints: &[CheckpointRows], schema: &[&str]) -> Result<Block> {
    let n: usize = checkpoints.iter().map(|c| c.len()).sum();
    if n == 0 {
        return Ok(Block::new());
    }

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

    let mut checkpoint_sequence_number =
        clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut transaction_digest = ColumnString::new(Type::string());
    let mut cmd_idx = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut epoch = clickhouse_native_client::column::numeric::ColumnUInt64::with_capacity(n);
    let mut timestamp_ms = ColumnInt64::with_capacity(n);
    let mut package = ColumnString::new(Type::string());
    let mut module = ColumnString::new(Type::string());
    let mut function = ColumnString::new(Type::string());

    for cp in checkpoints {
        for row in cp.iter() {
            let checkpoint = get_u64(row, schema, "checkpoint");
            let digest = get_str(row, schema, "transaction_digest");
            let cmd = get_u64(row, schema, "cmd_idx");
            let ep = get_u64(row, schema, "epoch");
            let ts = get_u64(row, schema, "timestamp_ms") as i64;
            let pkg = get_str(row, schema, "package");
            let mod_ = get_str(row, schema, "module");
            let func = get_str(row, schema, "function");

            checkpoint_sequence_number.append(checkpoint);
            transaction_digest.append(digest);
            cmd_idx.append(cmd);
            epoch.append(ep);
            timestamp_ms.append(ts);
            package.append(pkg);
            module.append(mod_);
            function.append(func);
        }
    }

    let mut block = Block::new();
    block.append_column(
        "checkpoint_sequence_number",
        std::sync::Arc::new(checkpoint_sequence_number),
    )?;
    block.append_column(
        "transaction_digest",
        std::sync::Arc::new(transaction_digest),
    )?;
    block.append_column("cmd_idx", std::sync::Arc::new(cmd_idx))?;
    block.append_column("epoch", std::sync::Arc::new(epoch))?;
    block.append_column("timestamp_ms", std::sync::Arc::new(timestamp_ms))?;
    block.append_column("package", std::sync::Arc::new(package))?;
    block.append_column("module", std::sync::Arc::new(module))?;
    block.append_column("function", std::sync::Arc::new(function))?;

    Ok(block)
}

pub fn object_rows_to_block(checkpoints: &[CheckpointRows], schema: &[&str]) -> Result<Block> {
    let n: usize = checkpoints.iter().map(|c| c.len()).sum();
    if n == 0 {
        return Ok(Block::new());
    }

    fn col_idx(schema: &[&str], name: &str) -> Option<usize> {
        schema.iter().position(|s| *s == name)
    }

    fn get_u64(row: &dyn Row, schema: &[&str], name: &str) -> u64 {
        col_idx(schema, name)
            .and_then(|i| row.get_column(i).ok())
            .and_then(|v| match v {
                ColumnValue::U64(x) => Some(x),
                ColumnValue::OptionU64(Some(x)) => Some(x),
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

    let mut object_id = ColumnString::new(Type::string());
    let mut version = ColumnUInt64::with_capacity(n);
    let mut digest = ColumnString::new(Type::string());
    let mut type_ = ColumnString::new(Type::string());
    let mut checkpoint_sequence_number = ColumnUInt64::with_capacity(n);
    let mut epoch = ColumnUInt64::with_capacity(n);
    let mut timestamp_ms = ColumnInt64::with_capacity(n);
    let mut owner_type = ColumnString::new(Type::string());
    let mut owner_address = ColumnString::new(Type::string());
    let mut object_status = ColumnString::new(Type::string());
    let mut initial_shared_version = ColumnUInt64::with_capacity(n);
    let mut previous_transaction = ColumnString::new(Type::string());
    let mut has_public_transfer = ColumnUInt8::with_capacity(n);
    let mut is_consensus = ColumnUInt8::with_capacity(n);
    let mut storage_rebate = ColumnUInt64::with_capacity(n);
    let mut bcs = ColumnString::new(Type::string());
    let mut coin_type = ColumnString::new(Type::string());
    let mut coin_balance = ColumnUInt64::with_capacity(n);
    let mut struct_tag = ColumnString::new(Type::string());
    let mut object_json = ColumnString::new(Type::string());
    let mut bcs_length = ColumnUInt64::with_capacity(n);

    for cp in checkpoints {
        for row in cp.iter() {
            object_id.append(get_str(row, schema, "object_id"));
            version.append(get_u64(row, schema, "version"));
            digest.append(get_str(row, schema, "digest"));
            type_.append(get_str(row, schema, "type_"));
            checkpoint_sequence_number.append(get_u64(row, schema, "checkpoint"));
            epoch.append(get_u64(row, schema, "epoch"));
            timestamp_ms.append(get_u64(row, schema, "timestamp_ms") as i64);
            owner_type.append(get_str(row, schema, "owner_type"));
            owner_address.append(get_str(row, schema, "owner_address"));
            object_status.append(get_str(row, schema, "object_status"));
            initial_shared_version.append(get_u64(row, schema, "initial_shared_version"));
            previous_transaction.append(get_str(row, schema, "previous_transaction"));
            has_public_transfer.append(get_bool(row, schema, "has_public_transfer") as u8);
            is_consensus.append(get_bool(row, schema, "is_consensus") as u8);
            storage_rebate.append(get_u64(row, schema, "storage_rebate"));
            bcs.append(get_str(row, schema, "bcs"));
            coin_type.append(get_str(row, schema, "coin_type"));
            coin_balance.append(get_u64(row, schema, "coin_balance"));
            struct_tag.append(get_str(row, schema, "struct_tag"));
            object_json.append(get_str(row, schema, "object_json"));
            bcs_length.append(get_u64(row, schema, "bcs_length"));
        }
    }

    let mut block = Block::new();
    block.append_column("object_id", std::sync::Arc::new(object_id))?;
    block.append_column("version", std::sync::Arc::new(version))?;
    block.append_column("digest", std::sync::Arc::new(digest))?;
    block.append_column("type_", std::sync::Arc::new(type_))?;
    block.append_column(
        "checkpoint_sequence_number",
        std::sync::Arc::new(checkpoint_sequence_number),
    )?;
    block.append_column("epoch", std::sync::Arc::new(epoch))?;
    block.append_column("timestamp_ms", std::sync::Arc::new(timestamp_ms))?;
    block.append_column("owner_type", std::sync::Arc::new(owner_type))?;
    block.append_column("owner_address", std::sync::Arc::new(owner_address))?;
    block.append_column("object_status", std::sync::Arc::new(object_status))?;
    block.append_column(
        "initial_shared_version",
        std::sync::Arc::new(initial_shared_version),
    )?;
    block.append_column(
        "previous_transaction",
        std::sync::Arc::new(previous_transaction),
    )?;
    block.append_column("has_public_transfer", std::sync::Arc::new(has_public_transfer))?;
    block.append_column("is_consensus", std::sync::Arc::new(is_consensus))?;
    block.append_column("storage_rebate", std::sync::Arc::new(storage_rebate))?;
    block.append_column("bcs", std::sync::Arc::new(bcs))?;
    block.append_column("coin_type", std::sync::Arc::new(coin_type))?;
    block.append_column("coin_balance", std::sync::Arc::new(coin_balance))?;
    block.append_column("struct_tag", std::sync::Arc::new(struct_tag))?;
    block.append_column("object_json", std::sync::Arc::new(object_json))?;
    block.append_column("bcs_length", std::sync::Arc::new(bcs_length))?;

    Ok(block)
}

pub fn balance_change_rows_to_block(
    checkpoints: &[CheckpointRows],
    schema: &[&str],
) -> Result<Block> {
    let n: usize = checkpoints.iter().map(|c| c.len()).sum();
    if n == 0 {
        return Ok(Block::new());
    }

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

    let mut checkpoint_sequence_number = ColumnUInt64::with_capacity(n);
    let mut transaction_digest = ColumnString::new(Type::string());
    let mut epoch = ColumnUInt64::with_capacity(n);
    let mut timestamp_ms = ColumnInt64::with_capacity(n);
    let mut owner = ColumnString::new(Type::string());
    let mut coin_type = ColumnString::new(Type::string());
    let mut amount = ColumnInt64::with_capacity(n);

    for cp in checkpoints {
        for row in cp.iter() {
            checkpoint_sequence_number.append(get_u64(row, schema, "checkpoint"));
            transaction_digest.append(get_str(row, schema, "transaction_digest"));
            epoch.append(get_u64(row, schema, "epoch"));
            timestamp_ms.append(get_u64(row, schema, "timestamp_ms") as i64);
            owner.append(get_str(row, schema, "owner"));
            coin_type.append(get_str(row, schema, "coin_type"));
            amount.append(get_i64(row, schema, "amount"));
        }
    }

    let mut block = Block::new();
    block.append_column(
        "checkpoint_sequence_number",
        std::sync::Arc::new(checkpoint_sequence_number),
    )?;
    block.append_column(
        "transaction_digest",
        std::sync::Arc::new(transaction_digest),
    )?;
    block.append_column("epoch", std::sync::Arc::new(epoch))?;
    block.append_column("timestamp_ms", std::sync::Arc::new(timestamp_ms))?;
    block.append_column("owner", std::sync::Arc::new(owner))?;
    block.append_column("coin_type", std::sync::Arc::new(coin_type))?;
    block.append_column("amount", std::sync::Arc::new(amount))?;

    Ok(block)
}

#[cfg(test)]
mod tests {
    use crate::handlers::CheckpointRows;
    use crate::schema::RowSchema;
    use crate::tables::{EventRow, MoveCallRow, ObjectRow, ObjectStatus, OwnerType};

    use super::{event_rows_to_block, move_call_rows_to_block, object_rows_to_block};

    fn make_event_checkpoint_rows() -> Vec<CheckpointRows> {
        let rows = vec![
            EventRow {
                transaction_digest: "digest1".to_string(),
                event_index: 0,
                checkpoint: 100,
                epoch: 1,
                timestamp_ms: 1000,
                sender: "0xabc".to_string(),
                package: "0xpkg".to_string(),
                module: "mymodule".to_string(),
                event_type: "mydata::ProfileCreated".to_string(),
                bcs: "".to_string(),
                event_json: r#"{"user":"alice"}"#.to_string(),
                bcs_length: 0,
            },
            EventRow {
                transaction_digest: "digest1".to_string(),
                event_index: 1,
                checkpoint: 100,
                epoch: 1,
                timestamp_ms: 1000,
                sender: "0xabc".to_string(),
                package: "0xpkg".to_string(),
                module: "mymodule".to_string(),
                event_type: "mydata::PostLiked".to_string(),
                bcs: "".to_string(),
                event_json: r#"{"post_id":"0x123"}"#.to_string(),
                bcs_length: 0,
            },
        ];
        vec![CheckpointRows::from_rows(100, 1, rows)]
    }

    #[test]
    fn test_event_rows_to_block() {
        let checkpoints = make_event_checkpoint_rows();
        let schema = EventRow::schema();
        let block = event_rows_to_block(&checkpoints, schema).unwrap();
        assert_eq!(block.row_count(), 2);
        assert_eq!(block.column_count(), 11);
        assert!(block.column_by_name("checkpoint_sequence_number").is_some());
        assert!(block.column_by_name("transaction_digest").is_some());
        assert!(block.column_by_name("event_index").is_some());
        assert!(block.column_by_name("event_type").is_some());
        assert!(block.column_by_name("event_json").is_some());
    }

    fn make_move_call_checkpoint_rows() -> Vec<CheckpointRows> {
        let rows = vec![
            MoveCallRow {
                transaction_digest: "digest2".to_string(),
                cmd_idx: 0,
                checkpoint: 200,
                epoch: 2,
                timestamp_ms: 2000,
                package: "0xpkg".to_string(),
                module: "mydata".to_string(),
                function: "like_post".to_string(),
            },
            MoveCallRow {
                transaction_digest: "digest2".to_string(),
                cmd_idx: 1,
                checkpoint: 200,
                epoch: 2,
                timestamp_ms: 2000,
                package: "0xpkg".to_string(),
                module: "mydata".to_string(),
                function: "create_profile".to_string(),
            },
        ];
        vec![CheckpointRows::from_rows(200, 2, rows)]
    }

    #[test]
    fn test_move_call_rows_to_block() {
        let checkpoints = make_move_call_checkpoint_rows();
        let schema = MoveCallRow::schema();
        let block = move_call_rows_to_block(&checkpoints, schema).unwrap();
        assert_eq!(block.row_count(), 2);
        assert_eq!(block.column_count(), 8);
        assert!(block.column_by_name("checkpoint_sequence_number").is_some());
        assert!(block.column_by_name("transaction_digest").is_some());
        assert!(block.column_by_name("cmd_idx").is_some());
        assert!(block.column_by_name("module").is_some());
        assert!(block.column_by_name("function").is_some());
    }

    #[test]
    fn test_event_rows_to_block_empty() {
        let checkpoints: Vec<CheckpointRows> = vec![];
        let schema = EventRow::schema();
        let block = event_rows_to_block(&checkpoints, schema).unwrap();
        assert_eq!(block.row_count(), 0);
    }

    #[test]
    fn test_move_call_rows_to_block_empty() {
        let checkpoints: Vec<CheckpointRows> = vec![];
        let schema = MoveCallRow::schema();
        let block = move_call_rows_to_block(&checkpoints, schema).unwrap();
        assert_eq!(block.row_count(), 0);
    }

    fn make_object_checkpoint_rows() -> Vec<CheckpointRows> {
        let rows = vec![
            ObjectRow {
                object_id: "0xobj1".to_string(),
                version: 1,
                digest: "dig1".to_string(),
                type_: Some("0x2::coin::Coin<0x2::myso::MYSO>".to_string()),
                checkpoint: 100,
                epoch: 1,
                timestamp_ms: 1000,
                owner_type: Some(OwnerType::AddressOwner),
                owner_address: Some("0xabc".to_string()),
                object_status: ObjectStatus::Mutated,
                initial_shared_version: None,
                previous_transaction: "tx1".to_string(),
                has_public_transfer: true,
                is_consensus: false,
                storage_rebate: Some(100),
                bcs: "".to_string(),
                coin_type: Some("0x2::myso::MYSO".to_string()),
                coin_balance: Some(1000),
                struct_tag: None,
                object_json: None,
                bcs_length: 0,
            },
        ];
        vec![CheckpointRows::from_rows(100, 1, rows)]
    }

    #[test]
    fn test_object_rows_to_block() {
        let checkpoints = make_object_checkpoint_rows();
        let schema = ObjectRow::schema();
        let block = object_rows_to_block(&checkpoints, schema).unwrap();
        assert_eq!(block.row_count(), 1);
        assert!(block.column_by_name("object_id").is_some());
        assert!(block.column_by_name("checkpoint_sequence_number").is_some());
        assert!(block.column_by_name("coin_type").is_some());
        assert!(block.column_by_name("coin_balance").is_some());
    }

    #[test]
    fn test_object_rows_to_block_empty() {
        let checkpoints: Vec<CheckpointRows> = vec![];
        let schema = ObjectRow::schema();
        let block = object_rows_to_block(&checkpoints, schema).unwrap();
        assert_eq!(block.row_count(), 0);
    }
}
