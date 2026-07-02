-- FX2: on-chain org membership invitations indexed from memory.move OrgInvitation* events.

CREATE TABLE IF NOT EXISTS org_invitations (
    organization_id TEXT NOT NULL,
    invitee_address TEXT NOT NULL,
    role_name TEXT,
    permissions_mask BIGINT NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'pending',
    invited_by TEXT NOT NULL,
    created_at_ms BIGINT NOT NULL,
    expires_at_ms BIGINT,
    responded_at_ms BIGINT,
    responded_by TEXT,
    granted_mask BIGINT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (organization_id, invitee_address)
);

CREATE INDEX IF NOT EXISTS idx_org_invitations_org_status
    ON org_invitations (organization_id, status);

CREATE INDEX IF NOT EXISTS idx_org_invitations_invitee_status
    ON org_invitations (invitee_address, status);

-- ============================================================
-- FX4: audit_log Timescale compression + retention
-- ============================================================

ALTER TABLE audit_log SET (
    timescaledb.compress,
    timescaledb.compress_segmentby = 'organization_id',
    timescaledb.compress_orderby = 'time DESC'
);

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_compression'
          AND hypertable_schema = 'public'
          AND hypertable_name = 'audit_log'
    ) THEN
        PERFORM add_compression_policy('audit_log', INTERVAL '7 days');
    END IF;

    IF NOT EXISTS (
        SELECT 1 FROM timescaledb_information.jobs
        WHERE proc_name = 'policy_retention'
          AND hypertable_schema = 'public'
          AND hypertable_name = 'audit_log'
    ) THEN
        PERFORM add_retention_policy('audit_log', INTERVAL '90 days');
    END IF;
END $$;

-- Wave 0: index the org memory share group id on the org row so services can
-- fetch it from social-server instead of re-deriving derived_object addresses.
ALTER TABLE sub_agent_organizations
    ADD COLUMN IF NOT EXISTS org_memory_group_id TEXT;
