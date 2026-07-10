DROP INDEX IF EXISTS idx_markets_spot_market_object;

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'markets' AND column_name = 'spot_market_object_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'markets' AND column_name = 'spot_record_id'
    ) THEN
        ALTER TABLE markets RENAME COLUMN spot_market_object_id TO spot_record_id;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_markets_spot_record
    ON markets(spot_record_id)
    WHERE spot_record_id IS NOT NULL;

DROP TABLE IF EXISTS spot_trusted_sources;
