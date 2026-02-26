-- Fix watermarks table if it was created with the wrong schema.
-- The indexer framework requires pipeline, epoch_hi_inclusive, checkpoint_hi_inclusive, etc.
-- CASCADE on DROP avoids lock contention from dependent objects.

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.tables
        WHERE table_schema = 'public' AND table_name = 'watermarks'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_schema = 'public' AND table_name = 'watermarks'
        AND column_name = 'epoch_hi_inclusive'
    ) THEN
        DROP TABLE IF EXISTS watermarks CASCADE;
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
