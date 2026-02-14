-- Fix watermarks table if it was created with the wrong schema (worker_id, stream_name).
-- The indexer framework expects pipeline, epoch_hi_inclusive, etc.

DO $$
BEGIN
    IF EXISTS (
        SELECT 1
        FROM information_schema.columns
        WHERE table_name = 'watermarks'
        AND column_name = 'worker_id'
        AND table_schema = 'public'
    ) THEN
        DROP TABLE IF EXISTS watermarks;
        CREATE TABLE watermarks
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
    END IF;
END $$;
