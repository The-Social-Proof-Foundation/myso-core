use crate::models::orderbook::state::StakeEvent;
use myso_indexer_alt_orderbook_schema::models::Stakes;

define_handler! {
    name: StakesHandler,
    processor_name: "stakes",
    event_type: StakeEvent,
    db_model: Stakes,
    table: stakes,
    map_event: |event, meta| Stakes {
        event_digest: meta.event_digest(),
        digest: meta.digest(),
        sender: meta.sender(),
        checkpoint: meta.checkpoint(),
        checkpoint_timestamp_ms: meta.checkpoint_timestamp_ms(),
        package: meta.package(),
        pool_id: event.pool_id.to_string(),
        balance_manager_id: event.balance_manager_id.to_string(),
        epoch: event.epoch as i64,
        amount: event.amount as i64,
        stake: event.stake,
    }
}
