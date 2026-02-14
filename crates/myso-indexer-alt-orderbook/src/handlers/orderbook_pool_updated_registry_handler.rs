use crate::models::orderbook_margin::margin_registry::OrderbookPoolUpdated;
use myso_indexer_alt_orderbook_schema::models::OrderbookPoolUpdatedRegistry;

define_handler! {
    name: OrderbookPoolUpdatedRegistryHandler,
    processor_name: "orderbook_pool_updated_registry",
    event_type: OrderbookPoolUpdated,
    db_model: OrderbookPoolUpdatedRegistry,
    table: orderbook_pool_updated_registry,
    map_event: |event, meta| OrderbookPoolUpdatedRegistry {
        event_digest: meta.event_digest(),
        digest: meta.digest(),
        sender: meta.sender(),
        checkpoint: meta.checkpoint(),
        checkpoint_timestamp_ms: meta.checkpoint_timestamp_ms(),
        package: meta.package(),
        pool_id: event.pool_id.to_string(),
        enabled: event.enabled,
        onchain_timestamp: event.timestamp as i64,
    }
}
