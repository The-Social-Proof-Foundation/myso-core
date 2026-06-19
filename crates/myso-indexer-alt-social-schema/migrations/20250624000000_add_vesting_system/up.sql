-- ADD TOKEN VESTING SYSTEM
-- Piecewise vesting schedules with lump cliffs, per-segment curves, and 0.1% claim threshold

-- ============================================================================
-- 1. CREATE VESTING WALLET TABLE (Regular Table)
-- ============================================================================

CREATE TABLE IF NOT EXISTS vesting_wallets (
    wallet_id TEXT PRIMARY KEY,
    owner_address TEXT NOT NULL,
    total_amount BIGINT NOT NULL,
    start_time BIGINT NOT NULL,
    schedule_end BIGINT NOT NULL,
    pieces JSONB NOT NULL DEFAULT '[]',
    claimed_amount BIGINT NOT NULL DEFAULT 0,
    remaining_balance BIGINT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL
);

-- ============================================================================
-- 2. CREATE VESTING EVENTS TABLE (TimescaleDB Hypertable)
-- ============================================================================

CREATE TABLE IF NOT EXISTS vesting_events (
    id SERIAL NOT NULL,
    wallet_id TEXT NOT NULL,
    event_type TEXT NOT NULL,
    owner_address TEXT NOT NULL,
    amount BIGINT NOT NULL,
    remaining_balance BIGINT,
    start_time BIGINT,
    schedule_end BIGINT,
    pieces JSONB,
    event_time BIGINT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    transaction_id TEXT NOT NULL,
    CONSTRAINT pk_vesting_events PRIMARY KEY (id, time)
);

SELECT create_hypertable('vesting_events', 'time', if_not_exists => TRUE, migrate_data => TRUE);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_class c
        JOIN pg_namespace n ON n.oid = c.relnamespace
        WHERE n.nspname = 'public'
        AND c.relname = 'vesting_events'
        AND c.reloptions IS NOT NULL
        AND array_to_string(c.reloptions, ',') LIKE '%compress=true%'
    ) THEN
        ALTER TABLE vesting_events SET (
            timescaledb.compress,
            timescaledb.compress_segmentby = 'wallet_id,owner_address,event_type',
            timescaledb.compress_orderby = 'time DESC'
        );
    END IF;
END $$;

-- ============================================================================
-- 3. CREATE INDEXES
-- ============================================================================

CREATE INDEX IF NOT EXISTS idx_vesting_wallets_owner_address ON vesting_wallets(owner_address);
CREATE INDEX IF NOT EXISTS idx_vesting_wallets_start_time ON vesting_wallets(start_time);
CREATE INDEX IF NOT EXISTS idx_vesting_wallets_schedule_end ON vesting_wallets(schedule_end);
CREATE INDEX IF NOT EXISTS idx_vesting_wallets_created_at ON vesting_wallets(created_at);

CREATE INDEX IF NOT EXISTS idx_vesting_events_wallet_id ON vesting_events(wallet_id);
CREATE INDEX IF NOT EXISTS idx_vesting_events_owner_address ON vesting_events(owner_address);
CREATE INDEX IF NOT EXISTS idx_vesting_events_event_type ON vesting_events(event_type);
CREATE INDEX IF NOT EXISTS idx_vesting_events_event_time ON vesting_events(event_time);

-- ============================================================================
-- 4. SET UP AUTOMATIC COMPRESSION POLICIES
-- ============================================================================

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression'
        AND hypertable_schema = 'public'
        AND hypertable_name = 'vesting_events'
    ) THEN
        PERFORM add_compression_policy('vesting_events', INTERVAL '7 days');
    END IF;
END $$;

-- ============================================================================
-- 5. CREATE VESTING UTILITY FUNCTIONS
-- ============================================================================

DROP FUNCTION IF EXISTS calculate_vesting_claimable(BIGINT, BIGINT, BIGINT, BIGINT, BIGINT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS calculate_vesting_claimable(BIGINT, BIGINT, BIGINT, JSONB, BIGINT, BIGINT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_vesting_status(BIGINT, BIGINT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS get_vesting_progress(BIGINT, BIGINT, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS vesting_apply_curve(DOUBLE PRECISION, BIGINT) CASCADE;
DROP FUNCTION IF EXISTS vesting_piece_amount(BIGINT, BIGINT) CASCADE;

CREATE FUNCTION vesting_piece_amount(total_amount_param BIGINT, amount_bps_param BIGINT)
RETURNS BIGINT AS $$
BEGIN
    RETURN (total_amount_param * amount_bps_param) / 10000;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

CREATE FUNCTION vesting_apply_curve(progress_ratio DOUBLE PRECISION, curve_factor_param BIGINT)
RETURNS DOUBLE PRECISION AS $$
DECLARE
    steepness DOUBLE PRECISION;
    quadratic DOUBLE PRECISION;
    sqrt_approx DOUBLE PRECISION;
    precision_factor CONSTANT DOUBLE PRECISION := 1000.0;
BEGIN
    IF curve_factor_param = 0 OR curve_factor_param = 1000 THEN
        RETURN progress_ratio;
    ELSIF curve_factor_param > 1000 THEN
        steepness := curve_factor_param - 1000;
        quadratic := progress_ratio * progress_ratio;
        RETURN (progress_ratio * (precision_factor - steepness) + quadratic * steepness) / precision_factor;
    ELSE
        steepness := 1000 - curve_factor_param;
        sqrt_approx := SQRT(GREATEST(progress_ratio, 0.0));
        RETURN (sqrt_approx * steepness + progress_ratio * (precision_factor - steepness)) / precision_factor;
    END IF;
END;
$$ LANGUAGE plpgsql IMMUTABLE;

CREATE FUNCTION calculate_vesting_claimable(
    total_amount_param BIGINT,
    start_time_param BIGINT,
    schedule_end_param BIGINT,
    pieces_param JSONB,
    claimed_amount_param BIGINT,
    current_time_param BIGINT,
    remaining_balance_param BIGINT
)
RETURNS BIGINT AS $$
DECLARE
    piece JSONB;
    total_released BIGINT := 0;
    piece_alloc BIGINT;
    piece_vested BIGINT;
    kind INT;
    time_offset BIGINT;
    duration BIGINT;
    amount_bps BIGINT;
    curve_factor BIGINT;
    activation_time BIGINT;
    end_time BIGINT;
    elapsed BIGINT;
    progress_ratio DOUBLE PRECISION;
    curved_progress DOUBLE PRECISION;
    capped BIGINT;
    threshold BIGINT;
BEGIN
    IF current_time_param < start_time_param THEN
        RETURN 0;
    END IF;

    IF current_time_param >= schedule_end_param THEN
        RETURN remaining_balance_param;
    END IF;

    FOR piece IN SELECT * FROM jsonb_array_elements(pieces_param) LOOP
        kind := (piece->>'kind')::INT;
        time_offset := (piece->>'time_offset')::BIGINT;
        duration := COALESCE((piece->>'duration')::BIGINT, 0);
        amount_bps := (piece->>'amount_bps')::BIGINT;
        curve_factor := COALESCE((piece->>'curve_factor')::BIGINT, 0);

        piece_alloc := vesting_piece_amount(total_amount_param, amount_bps);
        activation_time := start_time_param + time_offset;
        piece_vested := 0;

        IF current_time_param >= activation_time THEN
            IF kind = 0 THEN
                piece_vested := piece_alloc;
            ELSIF kind = 1 AND duration > 0 THEN
                end_time := activation_time + duration;
                IF current_time_param >= end_time THEN
                    piece_vested := piece_alloc;
                ELSE
                    elapsed := current_time_param - activation_time;
                    progress_ratio := CAST(elapsed AS DOUBLE PRECISION) / CAST(duration AS DOUBLE PRECISION);
                    curved_progress := vesting_apply_curve(progress_ratio, curve_factor);
                    piece_vested := CAST(CAST(piece_alloc AS DOUBLE PRECISION) * curved_progress AS BIGINT);
                END IF;
            END IF;
        END IF;

        total_released := total_released + piece_vested;
    END LOOP;

    IF total_released > total_amount_param THEN
        total_released := total_amount_param;
    END IF;

    capped := GREATEST(total_released - claimed_amount_param, 0);
    IF capped > remaining_balance_param THEN
        capped := remaining_balance_param;
    END IF;

    IF capped = 0 THEN
        RETURN 0;
    END IF;

    threshold := total_amount_param / 1000;
    IF threshold = 0 THEN
        threshold := 1;
    END IF;

    IF capped < threshold AND capped < remaining_balance_param THEN
        RETURN 0;
    END IF;

    RETURN capped;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION get_vesting_status(
    start_time_param BIGINT,
    schedule_end_param BIGINT,
    current_time_param BIGINT
)
RETURNS TEXT AS $$
BEGIN
    IF current_time_param < start_time_param THEN
        RETURN 'not_started';
    ELSIF current_time_param >= schedule_end_param THEN
        RETURN 'completed';
    ELSE
        RETURN 'in_progress';
    END IF;
END;
$$ LANGUAGE plpgsql;

CREATE FUNCTION get_vesting_progress(
    start_time_param BIGINT,
    schedule_end_param BIGINT,
    current_time_param BIGINT
)
RETURNS DOUBLE PRECISION AS $$
DECLARE
    elapsed_time BIGINT;
    total_duration BIGINT;
BEGIN
    IF current_time_param < start_time_param THEN
        RETURN 0.0;
    END IF;

    total_duration := schedule_end_param - start_time_param;
    IF total_duration <= 0 THEN
        RETURN 100.0;
    END IF;

    elapsed_time := current_time_param - start_time_param;

    IF elapsed_time >= total_duration THEN
        RETURN 100.0;
    END IF;

    RETURN (CAST(elapsed_time AS DOUBLE PRECISION) / CAST(total_duration AS DOUBLE PRECISION)) * 100.0;
END;
$$ LANGUAGE plpgsql;
