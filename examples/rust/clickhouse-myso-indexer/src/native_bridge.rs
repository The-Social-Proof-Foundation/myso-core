// Copyright (c) Mysten Labs, Inc.
// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Bridge to run clickhouse_native_client (non-Send) from async_trait (requires Send).
//! Spawns a dedicated thread per connection that owns the Client and processes requests via channel.
//! Reconnects automatically on connection errors (Broken pipe, timeout, etc.) and retries the
//! operation once before returning an error, so the indexer recovers instead of cascading.

use clickhouse_native_client::{Block, Client, ClientOptions};
use tracing::warn;
use std::io::ErrorKind;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use tokio::sync::oneshot;

/// Timeout for execute/query/insert. ClickHouse inserts can be slow (merges, sync);
/// 5 min allows writes to complete before we give up.
const OPERATION_TIMEOUT: Duration = Duration::from_secs(300);

type ClientResult<T> = Result<T, clickhouse_native_client::Error>;

enum Request {
    Execute { query: String, tx: oneshot::Sender<ClientResult<()>> },
    Query { query: String, tx: oneshot::Sender<ClientResult<QueryResponse>> },
    Insert { table: String, block: Block, tx: oneshot::Sender<ClientResult<()>> },
}

pub struct QueryResponse {
    pub blocks: Vec<Block>,
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
}

impl NativeClientBridge {
    pub fn new(opts: ClientOptions) -> ClientResult<Self> {
        let opts_for_reconnect = opts.clone();
        let (req_tx, req_rx) = mpsc::channel();

        thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .expect("runtime");
            let mut client = match rt.block_on(Client::connect(opts_for_reconnect.clone())) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("ClickHouse native connect failed: {e}");
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
                    Request::Insert { table, block, tx } => {
                        let r = rt.block_on(client.insert(&table, block.clone()));
                        let (reconnect, result) = if let Err(ref e) = r {
                            if is_recoverable(e) {
                                match rt.block_on(Client::connect(opts_for_reconnect.clone())) {
                                    Ok(c) => {
                                        client = c;
                                        let retry = rt.block_on(client.insert(&table, block));
                                        let retry_failed_recoverable =
                                            retry.as_ref().err().is_some_and(is_recoverable);
                                        (retry_failed_recoverable, retry)
                                    }
                                    Err(connect_err) => {
                                        warn!("ClickHouse reconnect failed: {connect_err}");
                                        (true, r)
                                    }
                                }
                            } else {
                                (false, r)
                            }
                        } else {
                            (false, r)
                        };
                        let _ = tx.send(result);
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

        Ok(Self { tx: req_tx })
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

    pub async fn insert(&self, table: &str, block: Block) -> ClientResult<()> {
        let (tx, rx) = oneshot::channel();
        self.tx
            .send(Request::Insert {
                table: table.to_string(),
                block,
                tx,
            })
            .map_err(|_| clickhouse_native_client::Error::Connection("channel closed".into()))?;
        tokio::time::timeout(OPERATION_TIMEOUT, rx)
            .await
            .map_err(|_| clickhouse_native_client::Error::Connection("operation timeout".into()))?
            .map_err(|_| clickhouse_native_client::Error::Connection("channel closed".into()))?
    }
}
