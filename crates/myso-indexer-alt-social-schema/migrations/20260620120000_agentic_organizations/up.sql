-- Sub-agent organizations: org registry, audit events, stats rollups, and attribution spine.
-- org_type: 0=Company .. 13=Other (ORG_TYPE_OTHER).

ALTER TABLE agent_memory_vaults RENAME TO sub_agent_memory_vaults;
ALTER INDEX idx_agent_memory_vaults_account RENAME TO idx_sub_agent_memory_vaults_account;

CREATE TABLE IF NOT EXISTS sub_agent_organizations (
    organization_id TEXT NOT NULL PRIMARY KEY,
    account_id TEXT NOT NULL,
    principal_owner TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    name TEXT,
    description TEXT,
    org_type SMALLINT NOT NULL CHECK (org_type >= 0 AND org_type <= 13),
    root_agent_id TEXT UNIQUE,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at_ms BIGINT NOT NULL,
    deactivated_at_ms BIGINT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_sub_agent_organizations_type
    ON sub_agent_organizations (org_type, active);
CREATE INDEX IF NOT EXISTS idx_sub_agent_organizations_account
    ON sub_agent_organizations (account_id, active);
CREATE INDEX IF NOT EXISTS idx_sub_agent_organizations_principal
    ON sub_agent_organizations (principal_owner, active);
CREATE INDEX IF NOT EXISTS idx_sub_agent_organizations_profile
    ON sub_agent_organizations (profile_id, active);

CREATE TABLE IF NOT EXISTS sub_agent_organization_events (
    id SERIAL NOT NULL,
    event_type TEXT NOT NULL,
    organization_id TEXT,
    account_id TEXT,
    principal_owner TEXT,
    profile_id TEXT,
    name TEXT,
    description TEXT,
    org_type SMALLINT,
    previous_org_type SMALLINT,
    root_agent_id TEXT,
    agent_object_id TEXT,
    active BOOLEAN,
    created_at_ms BIGINT,
    deactivated_at_ms BIGINT,
    updated_at_ms BIGINT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, time)
);

SELECT create_hypertable('sub_agent_organization_events', 'time', if_not_exists => TRUE);

CREATE INDEX IF NOT EXISTS idx_sub_agent_organization_events_org
    ON sub_agent_organization_events (organization_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_sub_agent_organization_events_principal
    ON sub_agent_organization_events (principal_owner, time DESC);
CREATE INDEX IF NOT EXISTS idx_sub_agent_organization_events_type
    ON sub_agent_organization_events (event_type, time DESC);

CREATE TABLE IF NOT EXISTS sub_agent_organization_stats (
    organization_id TEXT NOT NULL PRIMARY KEY,

    -- Agents (Tier 1 sync)
    total_agents INT NOT NULL DEFAULT 0,
    active_agents INT NOT NULL DEFAULT 0,
    max_tree_depth SMALLINT NOT NULL DEFAULT 0,

    -- Social (Tier 1 sync + Tier 2 engagement rollup)
    total_posts BIGINT NOT NULL DEFAULT 0,
    total_comments BIGINT NOT NULL DEFAULT 0,
    total_reactions BIGINT NOT NULL DEFAULT 0,
    total_reposts BIGINT NOT NULL DEFAULT 0,
    total_engagement BIGINT NOT NULL DEFAULT 0,

    -- Financial (Tier 1 sync revenue/outbound; Tier 2 net reconcile)
    total_revenue_myso BIGINT NOT NULL DEFAULT 0,
    total_outbound_spend_myso BIGINT NOT NULL DEFAULT 0,
    net_cash_flow_myso BIGINT NOT NULL DEFAULT 0,

    -- Estimated AUM (Tier 2 rollup only)
    estimated_assets_under_management_myso BIGINT NOT NULL DEFAULT 0,
    attribution_coverage_bps INT NOT NULL DEFAULT 0 CHECK (attribution_coverage_bps >= 0 AND attribution_coverage_bps <= 10000),

    -- SPoT (Tier 1 participation sync; Tier 2 accuracy rollup)
    total_spot_participation BIGINT NOT NULL DEFAULT 0,
    spot_bets_placed BIGINT NOT NULL DEFAULT 0,
    spot_bets_resolved BIGINT NOT NULL DEFAULT 0,
    spot_bets_correct BIGINT NOT NULL DEFAULT 0,
    spot_accuracy_bps INT,

    -- Originality (Tier 2 rollup only)
    originality_posts_analyzed BIGINT NOT NULL DEFAULT 0,
    originality_score_average_bps INT,

    -- Counterparties & volume (Tier 1 sync)
    total_counterparties BIGINT NOT NULL DEFAULT 0,
    total_actions_executed BIGINT NOT NULL DEFAULT 0,
    total_transactions BIGINT NOT NULL DEFAULT 0,

    last_activity_at_ms BIGINT,
    stats_rollup_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS sub_agent_organization_stats_daily (
    organization_id TEXT NOT NULL,
    org_type SMALLINT NOT NULL CHECK (org_type >= 0 AND org_type <= 13),
    snapshot_date DATE NOT NULL,
    total_revenue_myso BIGINT NOT NULL DEFAULT 0,
    net_cash_flow_myso BIGINT NOT NULL DEFAULT 0,
    total_outbound_spend_myso BIGINT NOT NULL DEFAULT 0,
    total_counterparties BIGINT NOT NULL DEFAULT 0,
    active_agents INT NOT NULL DEFAULT 0,
    total_engagement BIGINT NOT NULL DEFAULT 0,
    estimated_aum_myso BIGINT NOT NULL DEFAULT 0,
    total_actions_executed BIGINT NOT NULL DEFAULT 0,
    growth_score BIGINT NOT NULL DEFAULT 0,
    spot_accuracy_bps INT,
    attribution_coverage_bps INT NOT NULL DEFAULT 0 CHECK (attribution_coverage_bps >= 0 AND attribution_coverage_bps <= 10000),
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (organization_id, snapshot_date, time)
);

SELECT create_hypertable('sub_agent_organization_stats_daily', 'time', if_not_exists => TRUE);

-- TimescaleDB requires unique indexes to include partitioning column (time)
CREATE UNIQUE INDEX IF NOT EXISTS idx_sub_agent_organization_stats_daily_org_date
    ON sub_agent_organization_stats_daily (organization_id, snapshot_date, time);
CREATE INDEX IF NOT EXISTS idx_sub_agent_organization_stats_daily_type_growth
    ON sub_agent_organization_stats_daily (org_type, growth_score DESC, snapshot_date DESC);
CREATE INDEX IF NOT EXISTS idx_sub_agent_organization_stats_daily_type_revenue
    ON sub_agent_organization_stats_daily (org_type, total_revenue_myso DESC, snapshot_date DESC);

CREATE TABLE IF NOT EXISTS sub_agent_organization_counterparties (
    organization_id TEXT NOT NULL,
    counterparty_address TEXT NOT NULL,
    first_interaction_at_ms BIGINT NOT NULL,
    last_interaction_at_ms BIGINT NOT NULL,
    interaction_count BIGINT NOT NULL DEFAULT 1,
    PRIMARY KEY (organization_id, counterparty_address)
);

CREATE INDEX IF NOT EXISTS idx_sub_agent_organization_counterparties_address
    ON sub_agent_organization_counterparties (counterparty_address);

-- Attribution spine: organization_id on agent registry and social/financial tables.
ALTER TABLE sub_agents ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_sub_agents_organization
    ON sub_agents (organization_id, active);

ALTER TABLE sub_agent_events ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_sub_agent_events_organization
    ON sub_agent_events (organization_id, time DESC);

ALTER TABLE posts ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_posts_organization_time
    ON posts (organization_id, time DESC);

ALTER TABLE comments ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_comments_organization
    ON comments (organization_id, time DESC);

ALTER TABLE reactions ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_reactions_organization
    ON reactions (organization_id, time DESC);

ALTER TABLE reposts ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_reposts_organization
    ON reposts (organization_id, time DESC);

ALTER TABLE spot_bets ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_spot_bets_organization
    ON spot_bets (organization_id, time DESC);

ALTER TABLE spt_transactions ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_spt_transactions_organization
    ON spt_transactions (organization_id, time DESC);

ALTER TABLE spt_reservations ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_spt_reservations_organization
    ON spt_reservations (organization_id, time DESC);

ALTER TABLE mydata_purchases ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_mydata_purchases_organization
    ON mydata_purchases (organization_id, time DESC);

ALTER TABLE unified_revenue ADD COLUMN IF NOT EXISTS organization_id TEXT;
CREATE INDEX IF NOT EXISTS idx_unified_revenue_organization
    ON unified_revenue (organization_id, time DESC);
