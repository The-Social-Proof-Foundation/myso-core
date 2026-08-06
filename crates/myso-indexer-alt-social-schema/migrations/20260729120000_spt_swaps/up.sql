-- SPT SWAP SUMMARIES
-- Records atomic SPT→SPT swaps emitted by `TokenSwappedEvent`.
-- This table is SUMMARY ONLY: holdings/supply/price/revenue are handled by the
-- underlying `TokenSoldEvent` + `TokenBoughtEvent` legs, not by this table.

-- ============================================================================
-- 1. CREATE SWAP SUMMARY TABLE
-- ============================================================================

CREATE TABLE IF NOT EXISTS spt_swaps (
    id BIGSERIAL NOT NULL,
    transaction_id TEXT NOT NULL,
    trader TEXT NOT NULL,
    source_pool_id TEXT NOT NULL,
    dest_pool_id TEXT NOT NULL,
    sell_amount BIGINT NOT NULL DEFAULT 0,   -- nano-SPT sold from the source pool
    dest_amount BIGINT NOT NULL DEFAULT 0,   -- nano-SPT bought into the dest pool
    sell_myso_gross BIGINT NOT NULL DEFAULT 0,
    buy_myso_gross BIGINT NOT NULL DEFAULT 0,
    sell_fee_amount BIGINT NOT NULL DEFAULT 0,
    buy_fee_amount BIGINT NOT NULL DEFAULT 0,
    sell_creator_fee BIGINT NOT NULL DEFAULT 0,
    sell_platform_fee BIGINT NOT NULL DEFAULT 0,
    sell_treasury_fee BIGINT NOT NULL DEFAULT 0,
    buy_creator_fee BIGINT NOT NULL DEFAULT 0,
    buy_platform_fee BIGINT NOT NULL DEFAULT 0,
    buy_treasury_fee BIGINT NOT NULL DEFAULT 0,
    leftover_myso BIGINT NOT NULL DEFAULT 0,
    source_new_price BIGINT NOT NULL DEFAULT 0,
    dest_new_price BIGINT NOT NULL DEFAULT 0,
    organization_id TEXT,
    created_at BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT pk_spt_swaps PRIMARY KEY (id, time)
);

-- Create TimescaleDB hypertable
SELECT create_hypertable('spt_swaps', 'time', if_not_exists => TRUE, migrate_data => TRUE);

-- Enable compression on swaps table
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
        AND c.relname = 'spt_swaps'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE spt_swaps SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'source_pool_id,dest_pool_id,trader',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- ============================================================================
-- 2. CREATE INDEXES
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_spt_swaps_transaction_id ON spt_swaps(transaction_id);
CREATE INDEX IF NOT EXISTS idx_spt_swaps_trader_time ON spt_swaps (trader, time DESC);
CREATE INDEX IF NOT EXISTS idx_spt_swaps_source_pool_time ON spt_swaps (source_pool_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_spt_swaps_dest_pool_time ON spt_swaps (dest_pool_id, time DESC);

-- ============================================================================
-- 3. SET UP AUTOMATIC COMPRESSION POLICY
-- ============================================================================

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression'
        AND hypertable_schema = 'public'
        AND hypertable_name = 'spt_swaps'
    ) THEN
        PERFORM add_compression_policy('spt_swaps', INTERVAL '7 days');
    END IF;
END $$;

-- ============================================================================
-- 4. MARK SWAP LEGS ON EXISTING TRANSACTIONS TABLE
-- ============================================================================

ALTER TABLE spt_transactions ADD COLUMN IF NOT EXISTS counterparty_pool_id TEXT;
ALTER TABLE spt_transactions ADD COLUMN IF NOT EXISTS is_swap_leg BOOLEAN NOT NULL DEFAULT FALSE;
