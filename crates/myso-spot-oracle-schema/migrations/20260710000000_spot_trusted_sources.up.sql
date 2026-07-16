-- SPoT-owned trusted source registry. Replaces co-hosted discovery_sources.
-- Greenfield: spot_oracle DB no longer requires discovery schema migrations.

CREATE TABLE IF NOT EXISTS spot_trusted_sources (
    id UUID PRIMARY KEY,
    source_key VARCHAR(128) NOT NULL UNIQUE,
    adapter_type VARCHAR(64) NOT NULL,
    domain VARCHAR(32) NOT NULL DEFAULT 'factual',
    trust_score DOUBLE PRECISION NOT NULL DEFAULT 0.5,
    enabled BOOLEAN NOT NULL DEFAULT true,
    config JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_spot_trusted_sources_enabled
    ON spot_trusted_sources(enabled, trust_score DESC);

-- Rename legacy SpotRecord column to SpotMarket object id.
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'markets' AND column_name = 'spot_record_id'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'markets' AND column_name = 'spot_market_object_id'
    ) THEN
        ALTER TABLE markets RENAME COLUMN spot_record_id TO spot_market_object_id;
    END IF;
END $$;

DROP INDEX IF EXISTS idx_markets_spot_record;
CREATE INDEX IF NOT EXISTS idx_markets_spot_market_object
    ON markets(spot_market_object_id)
    WHERE spot_market_object_id IS NOT NULL;
