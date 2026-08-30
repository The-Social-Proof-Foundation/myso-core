-- Reservation volume charts: (pool_id, time) lookup + 5-minute real-time CAGG.
-- Raw events stay on spt_reservations; this view only speeds FIVE_MIN reads.

CREATE INDEX IF NOT EXISTS idx_spt_reservations_pool_time
    ON spt_reservations (pool_id, time DESC);

DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates
        WHERE view_name = 'spt_reservation_volume_5m'
    ) INTO view_exists;

    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW spt_reservation_volume_5m
        WITH (timescaledb.continuous) AS
        SELECT
            time_bucket('5 minutes', time) AS bucket,
            pool_id,
            SUM(CASE WHEN amount > 0 THEN amount ELSE 0 END) AS deposit_volume,
            SUM(CASE WHEN amount < 0 THEN -amount ELSE 0 END) AS withdrawal_volume,
            COUNT(*) FILTER (WHERE amount > 0) AS deposit_count,
            COUNT(*) FILTER (WHERE amount < 0) AS withdrawal_count,
            MIN(time) AS earliest_at,
            MAX(time) AS latest_at
        FROM spt_reservations
        GROUP BY 1, 2
        WITH NO DATA
        $sql$;

        ALTER MATERIALIZED VIEW spt_reservation_volume_5m
            SET (timescaledb.materialized_only = false);

        PERFORM add_continuous_aggregate_policy(
            'spt_reservation_volume_5m',
            start_offset => INTERVAL '3 hours',
            end_offset => INTERVAL '5 minutes',
            schedule_interval => INTERVAL '5 minutes'
        );
    END IF;
END $$;
