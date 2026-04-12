-- Per-registry scoping for platform DAO nominees (on-chain governance registry object id).

-- Multiple platform GovernanceDAO objects share registry_type = 2; key rows by on-chain object id.
DROP INDEX IF EXISTS idx_governance_registries_type;
DROP INDEX IF EXISTS idx_governance_registries_registry_id;

CREATE UNIQUE INDEX IF NOT EXISTS idx_governance_registries_registry_id_unique
    ON governance_registries (registry_id);

CREATE INDEX IF NOT EXISTS idx_governance_registries_type
    ON governance_registries (registry_type);

ALTER TABLE nominated_delegates
    ADD COLUMN IF NOT EXISTS governance_registry_id TEXT;

DROP INDEX IF EXISTS idx_nominees_address_type_time;

CREATE UNIQUE INDEX IF NOT EXISTS idx_nominees_address_type_regid_time
    ON nominated_delegates (address, registry_type, COALESCE(governance_registry_id, ''), time);

CREATE INDEX IF NOT EXISTS idx_nominees_governance_registry_list
    ON nominated_delegates (governance_registry_id, registry_type, status, time)
    WHERE governance_registry_id IS NOT NULL;

-- Pending nominees: ecosystem/PoC rows typically have NULL governance_registry_id; platform rows match registry_id.
-- Proposals: use latest hypertable row per id (matches list_proposals). submitted_proposals = lifetime total;
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
LEFT JOIN delegates d ON g.registry_type = d.registry_type AND d.is_active = true
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
