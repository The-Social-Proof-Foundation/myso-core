-- Fix spt_revenue transaction_type CHECK constraint to accept 'BUY' and 'SELL'
-- (uppercase) to match spt_transactions and the indexer model constants.
-- The original constraint used lowercase ('buy', 'sell') which caused insert failures.
-- 'RESERVATION' / 'RESERVATION_WITHDRAW' match NewSptRevenue::from_reservation_event (spt.rs).

ALTER TABLE spt_revenue DROP CONSTRAINT IF EXISTS spt_revenue_transaction_type_check;

ALTER TABLE spt_revenue ADD CONSTRAINT spt_revenue_transaction_type_check
    CHECK (transaction_type IN ('BUY', 'SELL', 'RESERVATION', 'RESERVATION_WITHDRAW'));
