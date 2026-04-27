-- Drop min-on-chain-age eligibility columns (registry + platform).
ALTER TABLE governance_registries DROP COLUMN IF EXISTS min_on_chain_age_days;
ALTER TABLE platforms DROP COLUMN IF EXISTS min_on_chain_age_days;

-- Greenfield-oriented: require GovernanceDAO object id on delegate-scoped rows; fix stats / aggregates.

DROP VIEW IF EXISTS governance_stats CASCADE;

DROP MATERIALIZED VIEW IF EXISTS delegate_ratings_daily CASCADE;

DROP INDEX IF EXISTS idx_ratings_target_voter_registry_regid_time;

DROP INDEX IF EXISTS idx_delegates_address_type_regid_time;
DROP INDEX IF EXISTS idx_delegates_governance_registry_list;

DROP INDEX IF EXISTS idx_nominees_address_type_regid_time;
DROP INDEX IF EXISTS idx_nominees_governance_registry_list;

ALTER TABLE delegates
    ALTER COLUMN governance_registry_id SET NOT NULL;

ALTER TABLE delegate_ratings
    ALTER COLUMN governance_registry_id SET NOT NULL;

ALTER TABLE nominated_delegates
    ALTER COLUMN governance_registry_id SET NOT NULL;

ALTER TABLE proposals
    ALTER COLUMN governance_registry_id SET NOT NULL;

DROP INDEX IF EXISTS idx_proposals_governance_registry_list;

CREATE INDEX IF NOT EXISTS idx_proposals_governance_registry_list
    ON proposals (governance_registry_id, proposal_type, time);

CREATE UNIQUE INDEX IF NOT EXISTS idx_delegates_address_type_regid_time
    ON delegates (address, registry_type, governance_registry_id, time);

CREATE INDEX IF NOT EXISTS idx_delegates_governance_registry_list
    ON delegates (governance_registry_id, registry_type, time);

CREATE UNIQUE INDEX IF NOT EXISTS idx_nominees_address_type_regid_time
    ON nominated_delegates (address, registry_type, governance_registry_id, time);

CREATE INDEX IF NOT EXISTS idx_nominees_governance_registry_list
    ON nominated_delegates (governance_registry_id, registry_type, status, time);

CREATE UNIQUE INDEX IF NOT EXISTS idx_ratings_target_voter_registry_regid_time
    ON delegate_ratings (target_address, voter_address, registry_type, governance_registry_id, time);

-- Historical hypertable rows: only the latest row per delegate key should stay active.
UPDATE delegates d
SET is_active = false
WHERE (d.id, d.time) NOT IN (
    SELECT id, time
    FROM (
        SELECT DISTINCT ON (address, registry_type, governance_registry_id)
            id,
            time
        FROM delegates
        ORDER BY address, registry_type, governance_registry_id, time DESC, id DESC
    ) latest
);

CREATE VIEW governance_stats AS
SELECT
    g.registry_id,
    g.registry_type,
    COUNT(DISTINCT d.id) AS active_delegates,
    COUNT(DISTINCT n.id) AS pending_nominees,
    COUNT(DISTINCT p.id) AS submitted_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 1) AS in_review_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 2) AS voting_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status IN (3, 5)) AS approved_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 4) AS rejected_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 5) AS implemented_proposals,
    COUNT(DISTINCT p.id) FILTER (WHERE p.status = 6) AS rescinded_proposals
FROM governance_registries g
LEFT JOIN (
    SELECT DISTINCT ON (address, registry_type, governance_registry_id)
        *
    FROM delegates
    ORDER BY address, registry_type, governance_registry_id, time DESC, id DESC
) d
    ON g.registry_type = d.registry_type
    AND d.is_active = true
    AND d.governance_registry_id = g.registry_id
LEFT JOIN nominated_delegates n
    ON g.registry_type = n.registry_type
    AND n.status = 0
    AND n.governance_registry_id = g.registry_id
LEFT JOIN (
    SELECT DISTINCT ON (id) *
    FROM proposals
    ORDER BY id, time DESC
) p ON g.registry_type = p.proposal_type
    AND p.governance_registry_id = g.registry_id
GROUP BY g.registry_id, g.registry_type;

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
            governance_registry_id,
            target_address,
            SUM(CASE WHEN vote_kind = 1 THEN 1 ELSE 0 END) AS upvotes,
            SUM(CASE WHEN vote_kind = 0 THEN 1 ELSE 0 END) AS downvotes,
            SUM(CASE WHEN vote_kind = 2 THEN 1 ELSE 0 END) AS clears,
            COUNT(*) AS total_ratings
        FROM delegate_ratings
        GROUP BY day, registry_type, governance_registry_id, target_address
        WITH NO DATA
        $sql$;

        PERFORM add_continuous_aggregate_policy('delegate_ratings_daily',
            start_offset => INTERVAL '30 days',
            end_offset => INTERVAL '1 hour',
            schedule_interval => INTERVAL '1 day');
    END IF;
END
$$;

DROP VIEW IF EXISTS delegate_performance CASCADE;

CREATE VIEW delegate_performance AS
SELECT
    d.address,
    d.governance_registry_id,
    d.registry_type,
    d.upvotes,
    d.downvotes,
    d.proposals_reviewed,
    d.proposals_submitted,
    d.sided_winning_proposals,
    d.sided_losing_proposals,
    d.term_start,
    d.term_end,
    d.is_active,
    CASE
        WHEN d.proposals_reviewed > 0 THEN
            d.sided_winning_proposals::FLOAT / NULLIF(d.proposals_reviewed, 0)
        ELSE NULL
    END AS winning_rate,
    COUNT(DISTINCT dv.proposal_id) AS recent_votes,
    SUM(CASE WHEN dv.approve THEN 1 ELSE 0 END) AS recent_approvals,
    SUM(CASE WHEN NOT dv.approve THEN 1 ELSE 0 END) AS recent_rejections
FROM
    delegates d
LEFT JOIN
    delegate_votes dv ON d.address = dv.delegate_address
                      AND dv.time > NOW() - INTERVAL '30 days'
GROUP BY
    d.id, d.address, d.governance_registry_id, d.registry_type, d.upvotes, d.downvotes,
    d.proposals_reviewed, d.proposals_submitted, d.sided_winning_proposals,
    d.sided_losing_proposals, d.term_start, d.term_end, d.is_active;
