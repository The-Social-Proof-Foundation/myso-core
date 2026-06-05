-- SPoT governance linkage: contested markets bind to SPoT DAO proposals (registry_type = 2).
-- Also swap governance registry types: SPoT = 2, Platform = 3 (was Platform = 2, SPoT = 3).

-- Swap registry/proposal type ids (sentinel 99 avoids collisions during two-step swap).
UPDATE governance_registries SET registry_type = 99 WHERE registry_type = 2;
UPDATE governance_registries SET registry_type = 2 WHERE registry_type = 3;
UPDATE governance_registries SET registry_type = 3 WHERE registry_type = 99;

UPDATE proposals SET proposal_type = 99 WHERE proposal_type = 2;
UPDATE proposals SET proposal_type = 2 WHERE proposal_type = 3;
UPDATE proposals SET proposal_type = 3 WHERE proposal_type = 99;

UPDATE delegates SET registry_type = 99 WHERE registry_type = 2;
UPDATE delegates SET registry_type = 2 WHERE registry_type = 3;
UPDATE delegates SET registry_type = 3 WHERE registry_type = 99;

UPDATE nominated_delegates SET registry_type = 99 WHERE registry_type = 2;
UPDATE nominated_delegates SET registry_type = 2 WHERE registry_type = 3;
UPDATE nominated_delegates SET registry_type = 3 WHERE registry_type = 99;

UPDATE delegate_ratings SET registry_type = 99 WHERE registry_type = 2;
UPDATE delegate_ratings SET registry_type = 2 WHERE registry_type = 3;
UPDATE delegate_ratings SET registry_type = 3 WHERE registry_type = 99;

UPDATE governance_events SET registry_type = 99 WHERE registry_type = 2;
UPDATE governance_events SET registry_type = 2 WHERE registry_type = 3;
UPDATE governance_events SET registry_type = 3 WHERE registry_type = 99;

ALTER TABLE spot_records
    ADD COLUMN IF NOT EXISTS record_object_id TEXT NULL,
    ADD COLUMN IF NOT EXISTS active_proposal_id TEXT NULL,
    ADD COLUMN IF NOT EXISTS oracle_proposed_outcome SMALLINT NULL,
    ADD COLUMN IF NOT EXISTS proposed_outcome SMALLINT NULL,
    ADD COLUMN IF NOT EXISTS dao_escalated_at_ms BIGINT NULL;

ALTER TABLE spot_config
    ADD COLUMN IF NOT EXISTS spot_governance_registry_id TEXT NULL;

CREATE INDEX IF NOT EXISTS idx_spot_records_record_object_id
    ON spot_records (record_object_id)
    WHERE record_object_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_spot_records_active_proposal_id
    ON spot_records (active_proposal_id)
    WHERE active_proposal_id IS NOT NULL;

COMMENT ON COLUMN spot_records.active_proposal_id IS 'Linked SPoT governance proposal object ID while debate is open';
COMMENT ON COLUMN spot_records.oracle_proposed_outcome IS 'Oracle-suggested outcome when escalated to DAO_REQUIRED';
COMMENT ON COLUMN spot_records.proposed_outcome IS 'Outcome under community ratification in the active proposal';
COMMENT ON COLUMN spot_records.dao_escalated_at_ms IS 'Wall-clock ms when oracle escalated to DAO_REQUIRED';
COMMENT ON COLUMN spot_config.spot_governance_registry_id IS 'Shared SPoT GovernanceDAO object ID (registry_type = 2)';
