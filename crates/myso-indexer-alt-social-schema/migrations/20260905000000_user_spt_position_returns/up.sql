-- Personal SPT investment performance: current WAC ledger + event-driven ownership snapshots.
-- Snapshots are written only when ownership or cost basis changes (never on price ticks).
-- Mark value, unrealized ROI, and supply % are computed at read time.

-- ============================================================================
-- 1. CURRENT POSITION LEDGER (not a hypertable)
-- ============================================================================

CREATE TABLE IF NOT EXISTS user_spt_position_state (
    holder_address TEXT NOT NULL,
    pool_id TEXT NOT NULL,
    token_balance BIGINT NOT NULL DEFAULT 0,
    cost_basis_myso BIGINT NOT NULL DEFAULT 0,
    total_invested_myso BIGINT NOT NULL DEFAULT 0,
    total_returned_myso BIGINT NOT NULL DEFAULT 0,
    realized_myso BIGINT NOT NULL DEFAULT 0,
    disposed_cost_basis_myso BIGINT NOT NULL DEFAULT 0,
    sold_token_qty BIGINT NOT NULL DEFAULT 0,
    exit_proceeds_myso BIGINT NOT NULL DEFAULT 0,
    reservation_cost_myso BIGINT NOT NULL DEFAULT 0,
    cost_basis_unknown BOOLEAN NOT NULL DEFAULT FALSE,
    last_event_time TIMESTAMPTZ,
    last_tx_id TEXT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (holder_address, pool_id)
);

CREATE INDEX IF NOT EXISTS idx_user_spt_position_state_pool
    ON user_spt_position_state (pool_id);

CREATE INDEX IF NOT EXISTS idx_user_spt_position_state_holder
    ON user_spt_position_state (holder_address);

CREATE INDEX IF NOT EXISTS idx_user_spt_position_state_realized
    ON user_spt_position_state (realized_myso DESC)
    WHERE token_balance > 0 OR sold_token_qty > 0;

CREATE INDEX IF NOT EXISTS idx_user_spt_position_state_invested
    ON user_spt_position_state (total_invested_myso DESC)
    WHERE token_balance > 0 OR sold_token_qty > 0;

COMMENT ON TABLE user_spt_position_state IS
    'Weighted-average cost ledger per wallet+pool. Realized ROI uses disposed_cost_basis_myso.';

-- ============================================================================
-- 2. EVENT IDEMPOTENCY (checkpoint replay safety)
-- ============================================================================

CREATE TABLE IF NOT EXISTS user_spt_position_events (
    transaction_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    holder_address TEXT NOT NULL,
    pool_id TEXT NOT NULL,
    applied_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (transaction_id, event_type, holder_address, pool_id)
);

-- ============================================================================
-- 3. OWNERSHIP SNAPSHOTS (hypertable, event-driven only)
-- ============================================================================

CREATE TABLE IF NOT EXISTS user_spt_position_snapshots (
    id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    holder_address TEXT NOT NULL,
    pool_id TEXT NOT NULL,
    token_balance BIGINT NOT NULL DEFAULT 0,
    circulating_supply BIGINT NOT NULL DEFAULT 0,
    cost_basis_myso BIGINT NOT NULL DEFAULT 0,
    realized_myso BIGINT NOT NULL DEFAULT 0,
    disposed_cost_basis_myso BIGINT NOT NULL DEFAULT 0,
    reservation_cost_myso BIGINT NOT NULL DEFAULT 0,
    event_type TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_user_spt_position_snapshots PRIMARY KEY (id, time)
);

SELECT create_hypertable('user_spt_position_snapshots', 'time', if_not_exists => TRUE, migrate_data => TRUE);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
          AND c.relname = 'user_spt_position_snapshots'
          AND c.reloptions IS NOT NULL
          AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE user_spt_position_snapshots SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'holder_address,pool_id',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_user_spt_position_snapshots_holder_pool_time
    ON user_spt_position_snapshots (holder_address, pool_id, time DESC);

CREATE INDEX IF NOT EXISTS idx_user_spt_position_snapshots_tx
    ON user_spt_position_snapshots (transaction_id, event_type);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression'
          AND hypertable_schema = 'public'
          AND hypertable_name = 'user_spt_position_snapshots'
    ) THEN
        PERFORM add_compression_policy('user_spt_position_snapshots', INTERVAL '7 days');
    END IF;
END $$;

-- Last ownership snapshot per day (long-range charts). Mark/unrealized computed at read.
CREATE MATERIALIZED VIEW IF NOT EXISTS user_spt_position_snapshots_daily
WITH (timescaledb.continuous) AS
SELECT
    time_bucket('1 day', time) AS bucket,
    holder_address,
    pool_id,
    last(token_balance, time) AS token_balance,
    last(circulating_supply, time) AS circulating_supply,
    last(cost_basis_myso, time) AS cost_basis_myso,
    last(realized_myso, time) AS realized_myso,
    last(disposed_cost_basis_myso, time) AS disposed_cost_basis_myso,
    last(reservation_cost_myso, time) AS reservation_cost_myso
FROM user_spt_position_snapshots
GROUP BY bucket, holder_address, pool_id
WITH NO DATA;

SELECT add_continuous_aggregate_policy(
    'user_spt_position_snapshots_daily',
    start_offset => INTERVAL '30 days',
    end_offset => INTERVAL '1 hour',
    schedule_interval => INTERVAL '1 hour',
    if_not_exists => TRUE
);
