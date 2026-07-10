ALTER TABLE spot_markets
    ADD COLUMN IF NOT EXISTS resolution_at_ms BIGINT;

COMMENT ON COLUMN spot_markets.resolution_at_ms IS 'Immutable claim deadline (UTC ms) set at market creation';

ALTER TABLE spot_records
    ADD COLUMN IF NOT EXISTS resolution_at_ms BIGINT;

COMMENT ON COLUMN spot_records.resolution_at_ms IS 'Immutable claim deadline (UTC ms) mirrored from SpotMarket';
