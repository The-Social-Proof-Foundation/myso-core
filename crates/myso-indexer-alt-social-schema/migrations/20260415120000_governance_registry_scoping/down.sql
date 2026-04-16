-- Restore pre-scoping governance_stats (grouped by registry_type only).
-- Must DROP first: migrated view leads with registry_id; CREATE OR REPLACE cannot swap column order.
DROP VIEW IF EXISTS governance_stats CASCADE;
CREATE VIEW governance_stats AS
SELECT
    g.registry_type,
    COUNT(DISTINCT d.id) AS active_delegates,
    COUNT(DISTINCT n.id) AS pending_nominees,
    COUNT(DISTINCT p.id) AS submitted_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 1) AS in_review_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 2) AS voting_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 3) AS approved_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 4) AS rejected_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 5) AS implemented_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 6) AS rescinded_proposals
FROM governance_registries g
LEFT JOIN delegates d
    ON g.registry_type = d.registry_type
    AND d.is_active = true
    AND (
        (g.registry_type <> 2 AND d.governance_registry_id IS NULL)
        OR (g.registry_type = 2 AND d.governance_registry_id = g.registry_id)
    )
LEFT JOIN nominated_delegates n
    ON g.registry_type = n.registry_type
    AND n.status = 0
    AND (
        (g.registry_type <> 2 AND n.governance_registry_id IS NULL)
        OR (g.registry_type = 2 AND n.governance_registry_id = g.registry_id)
    )
LEFT JOIN (
    SELECT DISTINCT ON (id) *
    FROM proposals
    ORDER BY id, time DESC
) p ON g.registry_type = p.proposal_type
GROUP BY g.registry_type;

DROP MATERIALIZED VIEW IF EXISTS delegate_ratings_daily CASCADE;

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

DROP INDEX IF EXISTS idx_ratings_governance_registry_list;
DROP INDEX IF EXISTS idx_ratings_target_voter_registry_regid_time;

CREATE UNIQUE INDEX IF NOT EXISTS idx_ratings_target_voter_registry_time
    ON delegate_ratings (target_address, voter_address, registry_type, time);

ALTER TABLE delegate_ratings DROP COLUMN IF EXISTS governance_registry_id;

DROP INDEX IF EXISTS idx_governance_events_proposal_id;
DROP INDEX IF EXISTS idx_governance_events_gov_registry_created;

ALTER TABLE governance_events DROP COLUMN IF EXISTS proposal_id;
ALTER TABLE governance_events DROP COLUMN IF EXISTS governance_registry_id;

DROP INDEX IF EXISTS idx_proposals_governance_registry_list;

ALTER TABLE proposals DROP COLUMN IF EXISTS governance_registry_id;
