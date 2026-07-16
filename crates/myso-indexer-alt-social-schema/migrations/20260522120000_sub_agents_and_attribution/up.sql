-- Sub-agent registry + social action attribution (actor vs human principal).
-- owner/profile_id on posts & comments = human principal; sub_agent_id links delegated actors.

CREATE TABLE IF NOT EXISTS memory_accounts (
    account_id TEXT NOT NULL PRIMARY KEY,
    principal_owner TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    contract_version BIGINT NOT NULL DEFAULT 0,
    created_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_memory_accounts_principal_owner ON memory_accounts (principal_owner);
CREATE INDEX IF NOT EXISTS idx_memory_accounts_profile_id ON memory_accounts (profile_id);

CREATE TABLE IF NOT EXISTS sub_agents (
    agent_object_id TEXT NOT NULL PRIMARY KEY,
    derived_address TEXT NOT NULL UNIQUE,
    account_id TEXT NOT NULL,
    label TEXT NOT NULL,
    identity_class SMALLINT NOT NULL DEFAULT 0,
    role_tags BIGINT NOT NULL DEFAULT 0,
    capabilities BIGINT NOT NULL DEFAULT 0,
    delegatable_caps BIGINT NOT NULL DEFAULT 0,
    register_scope SMALLINT NOT NULL DEFAULT 0,
    approval_required_caps BIGINT NOT NULL DEFAULT 0,
    platform_scope TEXT,
    parent_object_id TEXT,
    depth SMALLINT NOT NULL DEFAULT 1,
    registered_by TEXT NOT NULL,
    expires_at_ms BIGINT,
    max_action_spend BIGINT,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at_ms BIGINT NOT NULL,
    deactivated_at_ms BIGINT,
    revoked_at_ms BIGINT,
    updated_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_sub_agents_account_active ON sub_agents (account_id, active);
CREATE INDEX IF NOT EXISTS idx_sub_agents_parent ON sub_agents (parent_object_id);
CREATE INDEX IF NOT EXISTS idx_sub_agents_derived_address ON sub_agents (derived_address);

CREATE TABLE IF NOT EXISTS agent_memory_vaults (
    vault_id TEXT NOT NULL PRIMARY KEY,
    agent_object_id TEXT NOT NULL UNIQUE,
    memory_account_id TEXT NOT NULL,
    created_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_agent_memory_vaults_account
    ON agent_memory_vaults (memory_account_id);

CREATE TABLE IF NOT EXISTS sub_agent_events (
    id SERIAL NOT NULL,
    event_type TEXT NOT NULL,
    account_id TEXT,
    principal_owner TEXT,
    profile_id TEXT,
    agent_object_id TEXT,
    derived_address TEXT,
    label TEXT,
    identity_class SMALLINT,
    role_tags BIGINT,
    capabilities BIGINT,
    delegatable_caps BIGINT,
    register_scope SMALLINT,
    approval_required_caps BIGINT,
    platform_scope TEXT,
    parent_object_id TEXT,
    depth SMALLINT,
    registered_by TEXT,
    expires_at_ms BIGINT,
    max_action_spend BIGINT,
    active BOOLEAN,
    created_at_ms BIGINT,
    revoked_count BIGINT,
    previous_owner TEXT,
    new_owner TEXT,
    migration_from_version BIGINT,
    migration_to_version BIGINT,
    registry_id TEXT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, time)
);

SELECT create_hypertable('sub_agent_events', 'time', if_not_exists => TRUE);

CREATE INDEX IF NOT EXISTS idx_sub_agent_events_derived ON sub_agent_events (derived_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_sub_agent_events_principal ON sub_agent_events (principal_owner, time DESC);
CREATE INDEX IF NOT EXISTS idx_sub_agent_events_agent_object ON sub_agent_events (agent_object_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_sub_agent_events_registry
    ON sub_agent_events (registry_id, time DESC);

ALTER TABLE profiles ADD COLUMN IF NOT EXISTS memory_account_id TEXT;
CREATE INDEX IF NOT EXISTS idx_profiles_memory_account_id ON profiles (memory_account_id);

ALTER TABLE posts ADD COLUMN IF NOT EXISTS sub_agent_id TEXT;
ALTER TABLE posts ADD COLUMN IF NOT EXISTS action_identity_class SMALLINT;

CREATE INDEX IF NOT EXISTS idx_posts_owner_sub_agent_time ON posts (owner, sub_agent_id, time DESC);

ALTER TABLE comments ADD COLUMN IF NOT EXISTS actor_address TEXT;
ALTER TABLE comments ADD COLUMN IF NOT EXISTS sub_agent_id TEXT;
ALTER TABLE comments ADD COLUMN IF NOT EXISTS action_identity_class SMALLINT;

CREATE INDEX IF NOT EXISTS idx_comments_post_sub_agent ON comments (post_id, sub_agent_id);

ALTER TABLE reposts ADD COLUMN IF NOT EXISTS actor_address TEXT;
ALTER TABLE reposts ADD COLUMN IF NOT EXISTS sub_agent_id TEXT;
ALTER TABLE reposts ADD COLUMN IF NOT EXISTS action_identity_class SMALLINT;

ALTER TABLE reactions ADD COLUMN IF NOT EXISTS principal_owner TEXT;
ALTER TABLE reactions ADD COLUMN IF NOT EXISTS actor_address TEXT;
ALTER TABLE reactions ADD COLUMN IF NOT EXISTS sub_agent_id TEXT;
ALTER TABLE reactions ADD COLUMN IF NOT EXISTS action_identity_class SMALLINT;
