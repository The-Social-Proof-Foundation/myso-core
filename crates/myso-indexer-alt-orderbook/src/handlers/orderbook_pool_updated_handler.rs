use crate::models::orderbook_margin::margin_pool::OrderbookPoolUpdated;
use myso_indexer_alt_orderbook_schema::models::OrderbookPoolUpdated as OrderbookPoolUpdatedModel;

define_handler! {
    name: OrderbookPoolUpdatedHandler,
    processor_name: "orderbook_pool_updated",
    event_type: OrderbookPoolUpdated,
    db_model: OrderbookPoolUpdatedModel,
    table: orderbook_pool_updated,
    map_event: |event, meta| OrderbookPoolUpdatedModel {
        event_digest: meta.event_digest(),
        digest: meta.digest(),
        sender: meta.sender(),
        checkpoint: meta.checkpoint(),
        checkpoint_timestamp_ms: meta.checkpoint_timestamp_ms(),
        package: meta.package(),
        margin_pool_id: event.margin_pool_id.to_string(),
        orderbook_pool_id: event.orderbook_pool_id.to_string(),
        pool_cap_id: event.pool_cap_id.to_string(),
        enabled: event.enabled,
        onchain_timestamp: event.timestamp as i64,
    }
}
