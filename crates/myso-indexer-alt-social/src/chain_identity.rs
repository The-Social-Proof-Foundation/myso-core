// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use anyhow::Context;
use diesel::QueryableByName;
use diesel::sql_query;
use diesel::sql_types::BigInt;
use diesel::sql_types::Nullable;
use diesel::sql_types::Text;
use diesel_async::RunQueryDsl;
use myso_pg_db::Connection;
use myso_pg_db::Db;
use myso_rpc::proto::myso::rpc::v2::GetServiceInfoRequest;
use tracing::info;
use tracing::warn;

const ADD_CHAIN_ID_COLUMN: &str =
    "ALTER TABLE watermarks ADD COLUMN IF NOT EXISTS chain_id TEXT";

/// Compare the fullnode chain id to the one stored with these watermarks.
///
/// A different chain id means genesis changed, so checkpoint cursors from the previous chain are
/// dropped and ingestion starts at 0. When no chain id has been recorded yet, cursors past the
/// fullnode tip are dropped too: those checkpoints cannot exist on the chain just identified.
///
/// `grpc_url` is the RPC URL, or the streaming URL when RPC is unset. Local checkpoint ingestion
/// has neither, and this check is skipped.
pub async fn sync_chain_identity(store: &Db, grpc_url: Option<&str>) -> anyhow::Result<()> {
    let Some(grpc_url) = grpc_url else {
        info!("No fullnode gRPC URL; leaving indexer watermarks unchanged");
        return Ok(());
    };

    let rpc = myso_rpc::Client::new(grpc_url.to_owned())
        .context("Failed to build fullnode client for chain identity")?;
    let info = rpc
        .ledger_client()
        .get_service_info(GetServiceInfoRequest::default())
        .await
        .context("GetServiceInfo failed")?
        .into_inner();
    let chain_id = info
        .chain_id
        .filter(|id| !id.is_empty())
        .context("Fullnode returned no chain id")?;
    let tip = info
        .checkpoint_height
        .context("Fullnode returned no checkpoint height")?;

    let mut conn = store
        .connect()
        .await
        .context("Failed to connect while syncing chain identity")?;
    sql_query(ADD_CHAIN_ID_COLUMN)
        .execute(&mut conn)
        .await
        .context("Failed to ensure watermarks.chain_id")?;

    let stored = sql_query(
        "SELECT chain_id FROM watermarks WHERE chain_id IS NOT NULL LIMIT 1",
    )
    .get_result::<ChainIdRow>(&mut conn)
    .await
    .optional()
    .context("Failed to read watermarks.chain_id")?
    .map(|row| row.chain_id);
    let max_hi = sql_query(
        "SELECT MAX(checkpoint_hi_inclusive) AS max_checkpoint_hi FROM watermarks",
    )
    .get_result::<MaxCheckpointRow>(&mut conn)
    .await
    .context("Failed to read watermark cursors")?
    .max_checkpoint_hi
    .and_then(|hi| u64::try_from(hi).ok());

    if !should_reset_watermarks(stored.as_deref(), &chain_id, max_hi, tip) {
        if stored.is_none() {
            save_chain_id(&mut conn, &chain_id).await?;
            info!(chain_id, tip, "Recorded indexer chain id");
        } else {
            info!(chain_id, tip, "Indexer chain id unchanged");
        }
        return Ok(());
    }

    let deleted = sql_query("DELETE FROM watermarks")
        .execute(&mut conn)
        .await
        .context("Failed to reset watermarks for chain change")?;
    warn!(
        previous_chain_id = stored.as_deref().unwrap_or(""),
        chain_id,
        tip,
        max_checkpoint_hi = max_hi.unwrap_or(0),
        deleted,
        "Chain changed or cursors are past the tip; reset indexer watermarks to checkpoint 0"
    );
    Ok(())
}

fn should_reset_watermarks(
    stored: Option<&str>,
    current: &str,
    max_checkpoint_hi: Option<u64>,
    tip: u64,
) -> bool {
    match stored {
        Some(stored) => stored != current,
        None => max_checkpoint_hi.is_some_and(|hi| hi > tip),
    }
}

async fn save_chain_id(conn: &mut Connection<'_>, chain_id: &str) -> anyhow::Result<()> {
    sql_query("UPDATE watermarks SET chain_id = $1")
        .bind::<Text, _>(chain_id)
        .execute(conn)
        .await
        .context("Failed to store indexer chain id")?;
    Ok(())
}

#[derive(QueryableByName)]
struct ChainIdRow {
    #[diesel(sql_type = Text)]
    chain_id: String,
}

#[derive(QueryableByName)]
struct MaxCheckpointRow {
    #[diesel(sql_type = Nullable<BigInt>)]
    max_checkpoint_hi: Option<i64>,
}

trait OptionalResult<T> {
    fn optional(self) -> anyhow::Result<Option<T>>;
}

impl<T> OptionalResult<T> for Result<T, diesel::result::Error> {
    fn optional(self) -> anyhow::Result<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(diesel::result::Error::NotFound) => Ok(None),
            Err(error) => Err(error.into()),
        }
    }
}
