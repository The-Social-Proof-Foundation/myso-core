use crate::models::orderbook_margin::margin_manager::LoanRepaidEvent;
use myso_indexer_alt_orderbook_schema::models::LoanRepaid;

define_handler! {
    name: LoanRepaidHandler,
    processor_name: "loan_repaid",
    event_type: LoanRepaidEvent,
    db_model: LoanRepaid,
    table: loan_repaid,
    map_event: |event, meta| LoanRepaid {
        event_digest: meta.event_digest(),
        digest: meta.digest(),
        sender: meta.sender(),
        checkpoint: meta.checkpoint(),
        checkpoint_timestamp_ms: meta.checkpoint_timestamp_ms(),
        package: meta.package(),
        margin_manager_id: event.margin_manager_id.to_string(),
        margin_pool_id: event.margin_pool_id.to_string(),
        repay_amount: event.repay_amount as i64,
        repay_shares: event.repay_shares as i64,
        onchain_timestamp: event.timestamp as i64,
    }
}
