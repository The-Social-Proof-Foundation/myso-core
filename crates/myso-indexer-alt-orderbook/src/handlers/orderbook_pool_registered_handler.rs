use crate::models::orderbook_margin::margin_registry::OrderbookPoolRegistered;
use myso_indexer_alt_orderbook_schema::models::OrderbookPoolRegistered as OrderbookPoolRegisteredModel;

define_handler! {
    name: OrderbookPoolRegisteredHandler,
    processor_name: "orderbook_pool_registered",
    event_type: OrderbookPoolRegistered,
    db_model: OrderbookPoolRegisteredModel,
    table: orderbook_pool_registered,
    map_event: |event, meta| OrderbookPoolRegisteredModel {
        event_digest: meta.event_digest(),
        digest: meta.digest(),
        sender: meta.sender(),
        checkpoint: meta.checkpoint(),
        checkpoint_timestamp_ms: meta.checkpoint_timestamp_ms(),
        package: meta.package(),
        pool_id: event.pool_id.to_string(),
        config_json: Some(serde_json::to_value(&event.config).unwrap()),
        onchain_timestamp: event.timestamp as i64,
    }
}
