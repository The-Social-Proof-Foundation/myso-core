-- Indexes for profile P&L queries (recipient/sender + time range on hypertables).

CREATE INDEX IF NOT EXISTS idx_unified_revenue_recipient_time
    ON unified_revenue (recipient_address, time DESC);

CREATE INDEX IF NOT EXISTS idx_spt_transactions_sender_time
    ON spt_transactions (sender, time DESC);

CREATE INDEX IF NOT EXISTS idx_spt_reservations_reserver_time
    ON spt_reservations (reserver_address, time DESC);

-- Dedupes replayed indexer batches: transaction_id is "{digest}:{event_seq}", time is chain event ms.
CREATE UNIQUE INDEX IF NOT EXISTS idx_spt_reservations_txn_time_unique
    ON spt_reservations (transaction_id, time);
