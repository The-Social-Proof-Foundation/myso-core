-- Per-registry scoping for elected delegates (matches nominated_delegates; required for platform DAO registry_type = 2).

ALTER TABLE delegates
    ADD COLUMN IF NOT EXISTS governance_registry_id TEXT;

DROP INDEX IF EXISTS idx_delegates_address_type_time;

CREATE UNIQUE INDEX IF NOT EXISTS idx_delegates_address_type_regid_time
    ON delegates (address, registry_type, COALESCE(governance_registry_id, ''), time);

CREATE INDEX IF NOT EXISTS idx_delegates_governance_registry_list
    ON delegates (governance_registry_id, registry_type, time)
    WHERE governance_registry_id IS NOT NULL;

-- Align delegate join with nominated_delegates: one row per governance registry instance for platform DAOs.
CREATE OR REPLACE VIEW governance_stats AS
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
