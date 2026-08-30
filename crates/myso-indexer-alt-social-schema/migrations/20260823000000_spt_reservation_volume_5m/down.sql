DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM timescaledb_information.continuous_aggregates
        WHERE view_name = 'spt_reservation_volume_5m'
    ) THEN
        PERFORM remove_continuous_aggregate_policy(
            'spt_reservation_volume_5m',
            if_exists => true
        );
    END IF;
END $$;

DROP MATERIALIZED VIEW IF EXISTS spt_reservation_volume_5m CASCADE;
DROP INDEX IF EXISTS idx_spt_reservations_pool_time;
