-- Add watermarks table for indexer framework checkpoint tracking.
-- Uses the same schema as myso-pg-db so the framework's watermark operations work.
-- If the table already exists with the wrong schema (worker_id column), a later
-- migration (20250615000000_fix_watermarks) will fix it.

CREATE TABLE IF NOT EXISTS watermarks
(
    pipeline                    TEXT          PRIMARY KEY,
    epoch_hi_inclusive          BIGINT        NOT NULL,
    checkpoint_hi_inclusive     BIGINT        NOT NULL,
    tx_hi                       BIGINT        NOT NULL,
    timestamp_ms_hi_inclusive   BIGINT        NOT NULL,
    reader_lo                   BIGINT        NOT NULL,
    pruner_timestamp            TIMESTAMP     NOT NULL,
    pruner_hi                   BIGINT        NOT NULL
);
