use crate::models::orderbook_margin::margin_registry::OrderbookPoolConfigUpdated;
use myso_indexer_alt_orderbook_schema::models::OrderbookPoolConfigUpdated as OrderbookPoolConfigUpdatedModel;

define_handler! {
    name: OrderbookPoolConfigUpdatedHandler,
    processor_name: "orderbook_pool_config_updated",
    event_type: OrderbookPoolConfigUpdated,
    db_model: OrderbookPoolConfigUpdatedModel,
    table: orderbook_pool_config_updated,
    map_event: |event, meta| OrderbookPoolConfigUpdatedModel {
        event_digest: meta.event_digest(),
        digest: meta.digest(),
        sender: meta.sender(),
        checkpoint: meta.checkpoint(),
        checkpoint_timestamp_ms: meta.checkpoint_timestamp_ms(),
        package: meta.package(),
        pool_id: event.pool_id.to_string(),
        config_json: serde_json::to_value(&event.config).unwrap(),
        onchain_timestamp: event.timestamp as i64,
    }
}
