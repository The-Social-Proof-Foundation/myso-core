-- Replace boolean upvote with vote_kind: 0 = down, 1 = up, 2 = cleared (withdrawn vote).
-- Recreate delegate_ratings_daily continuous aggregate to count only up/down; track clears separately.

DROP MATERIALIZED VIEW IF EXISTS delegate_ratings_daily CASCADE;

ALTER TABLE delegate_ratings ADD COLUMN vote_kind SMALLINT;

UPDATE delegate_ratings SET vote_kind = CASE WHEN upvote THEN 1 ELSE 0 END;

ALTER TABLE delegate_ratings ALTER COLUMN vote_kind SET NOT NULL;

ALTER TABLE delegate_ratings DROP COLUMN upvote;

DO $$
DECLARE
    view_exists BOOLEAN;
BEGIN
    SELECT EXISTS(
        SELECT 1 FROM timescaledb_information.continuous_aggregates
        WHERE view_name = 'delegate_ratings_daily'
    ) INTO view_exists;

    IF NOT view_exists THEN
        EXECUTE $sql$
        CREATE MATERIALIZED VIEW delegate_ratings_daily
        WITH (timescaledb.continuous, timescaledb.materialized_only=false) AS
        SELECT
            time_bucket('1 day', time) AS day,
            registry_type,
            target_address,
            SUM(CASE WHEN vote_kind = 1 THEN 1 ELSE 0 END) AS upvotes,
            SUM(CASE WHEN vote_kind = 0 THEN 1 ELSE 0 END) AS downvotes,
            SUM(CASE WHEN vote_kind = 2 THEN 1 ELSE 0 END) AS clears,
            COUNT(*) AS total_ratings
        FROM delegate_ratings
        GROUP BY day, registry_type, target_address
        WITH NO DATA
        $sql$;

        PERFORM add_continuous_aggregate_policy('delegate_ratings_daily',
            start_offset => INTERVAL '30 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 day');
    END IF;
END
$$;
