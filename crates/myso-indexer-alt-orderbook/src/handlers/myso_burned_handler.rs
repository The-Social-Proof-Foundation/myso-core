use crate::handlers::{is_orderbook_tx, try_extract_move_call_package};
use crate::models::orderbook::pool::MysoBurned as MysoBurnedEvent;
use crate::models::myso::myso::MYSO;
use crate::traits::MoveStruct;
use crate::OrderbookEnv;
use async_trait::async_trait;
use myso_indexer_alt_orderbook_schema::models::MysoBurned;
use myso_indexer_alt_orderbook_schema::schema::myso_burned;
use diesel_async::RunQueryDsl;
use std::sync::Arc;
use myso_indexer_alt_framework::pipeline::Processor;
use myso_indexer_alt_framework::postgres::handler::Handler;
use myso_indexer_alt_framework::postgres::Connection;
use myso_indexer_alt_framework::types::full_checkpoint_content::Checkpoint;
use myso_types::transaction::TransactionDataAPI;
use tracing::debug;

pub struct MysoBurnedHandler {
    env: OrderbookEnv,
}

impl MysoBurnedHandler {
    pub fn new(env: OrderbookEnv) -> Self {
        Self { env }
    }
}

#[async_trait]
impl Processor for MysoBurnedHandler {
    const NAME: &'static str = "myso_burned";
    type Value = MysoBurned;

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

            for (index, ev) in events.data.iter().enumerate() {
                // Match base type (ignore type parameters)
                if !MysoBurnedEvent::<MYSO, MYSO>::matches_event_type(&ev.type_, self.env) {
                    continue;
                }

                // Can use <MYSO,MYSO> since it doesn't affect deserialization
                let event: MysoBurnedEvent<MYSO, MYSO> = bcs::from_bytes(&ev.contents)?;
                let data = MysoBurned {
                    digest: digest.to_string(),
                    event_digest: format!("{digest}{index}"),
                    sender: tx.transaction.sender().to_string(),
                    checkpoint: checkpoint_seq,
                    checkpoint_timestamp_ms,
                    package: package.clone(),
                    pool_id: event.pool_id.to_string(),
                    burned_amount: event.myso_burned as i64,
                };
                debug!("Observed Orderbook MysoBurned {:?}", data);
                results.push(data);
            }
        }
        Ok(results)
    }
}

#[async_trait]
impl Handler for MysoBurnedHandler {
    async fn commit<'a>(
        values: &[Self::Value],
        conn: &mut Connection<'a>,
    ) -> anyhow::Result<usize> {
        Ok(diesel::insert_into(myso_burned::table)
            .values(values)
            .on_conflict_do_nothing()
            .execute(conn)
            .await?)
    }
}
