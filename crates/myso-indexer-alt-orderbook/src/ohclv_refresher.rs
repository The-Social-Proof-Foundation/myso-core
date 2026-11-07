//! Keeps `ohclv_1m` / `ohclv_1d` filled from `order_fills`.
//!
//! The `update_all_ohclv` procedure (see the `add_ohclv` migration) only ran once at migration time,
//! so candles stayed empty. This task backfills all history on startup, then re-aggregates recent
//! buckets on an interval.

use std::time::Duration;

use diesel::sql_types::BigInt;
use diesel_async::RunQueryDsl;
use myso_pg_db::Db;
use tracing::{info, warn};

const REFRESH_INTERVAL: Duration = Duration::from_secs(60);
const DAY_MS: i64 = 86_400_000;

/// Spawns the background refresher. Errors are logged and retried on the next tick.
pub fn spawn(db: Db) {
    tokio::spawn(async move {
        // First pass: everything since genesis.
        let mut start_ms: i64 = 0;
        let mut ticker = tokio::time::interval(REFRESH_INTERVAL);
        loop {
            ticker.tick().await;
            let now_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_millis() as i64)
                .unwrap_or_default();
            match refresh(&db, start_ms, now_ms).await {
                Ok(()) => {
                    if start_ms == 0 {
                        info!("OHCLV backfill complete");
                    }
                    // Procedures replace each bucket's volume, so always recompute whole buckets:
                    // from the start of yesterday (UTC) covers every 1m bucket and both open day buckets.
                    start_ms = (now_ms / DAY_MS - 1) * DAY_MS;
                }
                Err(e) => warn!("OHCLV refresh failed: {e:#}"),
            }
        }
    });
}

async fn refresh(db: &Db, start_ms: i64, end_ms: i64) -> anyhow::Result<()> {
    let mut conn = db.connect().await?;
    diesel::sql_query("CALL update_all_ohclv($1, $2)")
        .bind::<BigInt, _>(start_ms)
        .bind::<BigInt, _>(end_ms)
        .execute(&mut conn)
        .await?;
    Ok(())
}
