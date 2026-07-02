DROP TABLE IF EXISTS memory_usage_stats;

ALTER TABLE sub_agent_organization_stats_daily
    DROP COLUMN IF EXISTS ai_credit_spent_mist,
    DROP COLUMN IF EXISTS memory_bytes;

ALTER TABLE sub_agent_organization_stats
    DROP COLUMN IF EXISTS ai_credit_spent_mist,
    DROP COLUMN IF EXISTS ai_credit_usage_events,
    DROP COLUMN IF EXISTS memory_entries,
    DROP COLUMN IF EXISTS memory_bytes,
    DROP COLUMN IF EXISTS org_shared_memory_entries;

DROP INDEX IF EXISTS idx_ai_credit_usage_lines_organization;
ALTER TABLE ai_credit_usage_lines DROP COLUMN IF EXISTS organization_id;

DROP TABLE IF EXISTS ai_credit_spend_approvals;
DROP TABLE IF EXISTS audit_log;
DROP TABLE IF EXISTS org_role_assignments;
DROP TABLE IF EXISTS org_roles;
DROP TABLE IF EXISTS org_memory_permissions;
