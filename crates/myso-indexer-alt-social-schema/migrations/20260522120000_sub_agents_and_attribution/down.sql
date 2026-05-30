ALTER TABLE reactions DROP COLUMN IF EXISTS action_identity_class;
ALTER TABLE reactions DROP COLUMN IF EXISTS sub_agent_id;
ALTER TABLE reactions DROP COLUMN IF EXISTS actor_address;
ALTER TABLE reactions DROP COLUMN IF EXISTS principal_owner;

ALTER TABLE reposts DROP COLUMN IF EXISTS action_identity_class;
ALTER TABLE reposts DROP COLUMN IF EXISTS sub_agent_id;
ALTER TABLE reposts DROP COLUMN IF EXISTS actor_address;

ALTER TABLE comments DROP COLUMN IF EXISTS action_identity_class;
ALTER TABLE comments DROP COLUMN IF EXISTS sub_agent_id;
ALTER TABLE comments DROP COLUMN IF EXISTS actor_address;

ALTER TABLE posts DROP COLUMN IF EXISTS action_identity_class;
ALTER TABLE posts DROP COLUMN IF EXISTS sub_agent_id;
ALTER TABLE posts DROP COLUMN IF EXISTS actor_address;

DROP INDEX IF EXISTS idx_profiles_memory_account_id;
ALTER TABLE profiles DROP COLUMN IF EXISTS memory_account_id;

DROP TABLE IF EXISTS agent_memory_vaults;
DROP TABLE IF EXISTS sub_agent_events;
DROP TABLE IF EXISTS sub_agents;
DROP TABLE IF EXISTS memory_accounts;
