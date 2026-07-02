SELECT remove_retention_policy('audit_log', if_exists => true);
SELECT remove_compression_policy('audit_log', if_exists => true);

DROP TABLE IF EXISTS org_invitations;

ALTER TABLE sub_agent_organizations
    DROP COLUMN IF EXISTS org_memory_group_id;
