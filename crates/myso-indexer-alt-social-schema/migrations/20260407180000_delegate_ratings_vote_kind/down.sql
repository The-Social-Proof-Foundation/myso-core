DROP MATERIALIZED VIEW IF EXISTS delegate_ratings_daily CASCADE;

ALTER TABLE delegate_ratings ADD COLUMN upvote BOOLEAN;

UPDATE delegate_ratings SET upvote = (vote_kind = 1);

UPDATE delegate_ratings SET upvote = FALSE WHERE vote_kind = 2;

ALTER TABLE delegate_ratings ALTER COLUMN upvote SET NOT NULL;

ALTER TABLE delegate_ratings DROP COLUMN vote_kind;

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
            SUM(CASE WHEN upvote THEN 1 ELSE 0 END) AS upvotes,
            SUM(CASE WHEN NOT upvote THEN 1 ELSE 0 END) AS downvotes,
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
