use crate::handlers::{is_orderbook_tx, try_extract_move_call_package};
use crate::models::orderbook::governance::TradeParamsUpdateEvent;
use crate::traits::MoveStruct;
use crate::OrderbookEnv;
use async_trait::async_trait;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_indexer_alt_orderbook_schema::models::TradeParamsUpdate;
use myso_indexer_alt_orderbook_schema::schema::trade_params_update;
use myso_types::transaction::TransactionDataAPI;
use std::sync::Arc;
use tracing::debug;

pub struct TradeParamsUpdateHandler {
    env: OrderbookEnv,
}

impl TradeParamsUpdateHandler {
    pub fn new(env: OrderbookEnv) -> Self {
        Self { env }
    }
}

#[async_trait]
impl Processor for TradeParamsUpdateHandler {
    const NAME: &'static str = "trade_params_update";
    type Value = TradeParamsUpdate;

    async fn process(&self, checkpoint: &Arc<Checkpoint>) -> anyhow::Result<Vec<Self::Value>> {
        let mut results = vec![];
        for tx in &checkpoint.transactions {
            if !is_orderbook_tx(tx, &checkpoint.object_set, self.env) {
                continue;
            }
            let Some(events) = &tx.events else {
                continue;
            };

            let package = try_extract_move_call_package(tx).unwrap_or_default();
            let checkpoint_timestamp_ms = checkpoint.summary.timestamp_ms as i64;
            let checkpoint_seq = checkpoint.summary.sequence_number as i64;
            let digest = tx.transaction.digest();

            // Get package addresses for orderbook
            let orderbook_addresses = self.env.package_addresses();

            let pool = tx
                .input_objects(&checkpoint.object_set)
                .find(|o| matches!(o.data.struct_tag(), Some(struct_tag)
                        if orderbook_addresses.iter().any(|addr| struct_tag.address == *addr) && struct_tag.name.as_str() == "Pool"));
            let pool_id = pool
                .map(|o| o.id().to_hex_uncompressed())
                .unwrap_or("0x0".to_string());

            for (index, ev) in events.data.iter().enumerate() {
                if !TradeParamsUpdateEvent::matches_event_type(&ev.type_, self.env) {
                    continue;
                }
                let event: TradeParamsUpdateEvent = bcs::from_bytes(&ev.contents)?;
                let data = TradeParamsUpdate {
                    digest: digest.to_string(),
                    event_digest: format!("{digest}{index}"),
                    sender: tx.transaction.sender().to_string(),
                    checkpoint: checkpoint_seq,
                    checkpoint_timestamp_ms,
                    package: package.clone(),
                    pool_id: pool_id.clone(),
                    taker_fee: event.taker_fee as i64,
                    maker_fee: event.maker_fee as i64,
                    stake_required: event.stake_required as i64,
                };
                debug!("Observed Orderbook Trade Params Update Event {:?}", data);
                results.push(data);
            }
        }
        Ok(results)
    }
}

#[async_trait]
impl Handler for TradeParamsUpdateHandler {
    async fn commit<'a>(
        values: &[Self::Value],
        conn: &mut Connection<'a>,
    ) -> anyhow::Result<usize> {
        Ok(diesel::insert_into(trade_params_update::table)
            .values(values)
            .on_conflict_do_nothing()
            .execute(conn)
            .await?)
    }
}
