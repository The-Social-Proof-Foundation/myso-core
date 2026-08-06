-- SPT TRANSFERS
-- Records P2P SocialToken transfers emitted by `TokenTransferredEvent`.
-- Holdings are mutated via delta rows on `spt_holdings` (from− / to+).
-- Circulating supply, price history, and revenue are NOT changed by transfers.

-- ============================================================================
-- 1. CREATE TRANSFER SUMMARY TABLE
-- ============================================================================

CREATE TABLE IF NOT EXISTS spt_transfers (
    id BIGSERIAL NOT NULL,
    transaction_id TEXT NOT NULL,
    pool_id TEXT NOT NULL,
    from_address TEXT NOT NULL,
    to_address TEXT NOT NULL,
    amount BIGINT NOT NULL DEFAULT 0,   -- nano-SPT
    organization_id TEXT,
    created_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_spt_transfers PRIMARY KEY (id, time)
);

SELECT create_hypertable('spt_transfers', 'time', if_not_exists => TRUE, migrate_data => TRUE);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
        AND c.relname = 'spt_transfers'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_transfers SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'pool_id,from_address,to_address',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- ============================================================================
-- 2. CREATE INDEXES
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_spt_transfers_transaction_id ON spt_transfers(transaction_id);
CREATE INDEX IF NOT EXISTS idx_spt_transfers_pool_time ON spt_transfers (pool_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_spt_transfers_from_time ON spt_transfers (from_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_spt_transfers_to_time ON spt_transfers (to_address, time DESC);

-- ============================================================================
-- 3. SET UP AUTOMATIC COMPRESSION POLICY
-- ============================================================================

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression'
        AND hypertable_schema = 'public'
        AND hypertable_name = 'spt_transfers'
    ) THEN
        PERFORM add_compression_policy('spt_transfers', INTERVAL '7 days');
    END IF;
END $$;
