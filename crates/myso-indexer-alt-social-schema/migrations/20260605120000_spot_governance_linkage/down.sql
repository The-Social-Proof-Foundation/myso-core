DROP INDEX IF EXISTS idx_spot_records_record_object_id;
DROP INDEX IF EXISTS idx_spot_records_active_proposal_id;

ALTER TABLE spot_records
    DROP COLUMN IF EXISTS record_object_id,
    DROP COLUMN IF EXISTS active_proposal_id,
    DROP COLUMN IF EXISTS oracle_proposed_outcome,
    DROP COLUMN IF EXISTS proposed_outcome,
    DROP COLUMN IF EXISTS dao_escalated_at_ms;

ALTER TABLE spot_config
    DROP COLUMN IF EXISTS spot_governance_registry_id;

-- Reverse registry/proposal type swap: Platform = 2, SPoT = 3.
UPDATE governance_registries SET registry_type = 99 WHERE registry_type = 3;
UPDATE governance_registries SET registry_type = 3 WHERE registry_type = 2;
UPDATE governance_registries SET registry_type = 2 WHERE registry_type = 99;

UPDATE proposals SET proposal_type = 99 WHERE proposal_type = 3;
UPDATE proposals SET proposal_type = 3 WHERE proposal_type = 2;
UPDATE proposals SET proposal_type = 2 WHERE proposal_type = 99;

UPDATE delegates SET registry_type = 99 WHERE registry_type = 3;
UPDATE delegates SET registry_type = 3 WHERE registry_type = 2;
UPDATE delegates SET registry_type = 2 WHERE registry_type = 99;

UPDATE nominated_delegates SET registry_type = 99 WHERE registry_type = 3;
UPDATE nominated_delegates SET registry_type = 3 WHERE registry_type = 2;
UPDATE nominated_delegates SET registry_type = 2 WHERE registry_type = 99;

UPDATE delegate_ratings SET registry_type = 99 WHERE registry_type = 3;
UPDATE delegate_ratings SET registry_type = 3 WHERE registry_type = 2;
UPDATE delegate_ratings SET registry_type = 2 WHERE registry_type = 99;

UPDATE governance_events SET registry_type = 99 WHERE registry_type = 3;
UPDATE governance_events SET registry_type = 3 WHERE registry_type = 2;
UPDATE governance_events SET registry_type = 2 WHERE registry_type = 99;
