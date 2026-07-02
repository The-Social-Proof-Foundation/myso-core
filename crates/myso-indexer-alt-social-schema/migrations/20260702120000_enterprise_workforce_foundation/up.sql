-- Enterprise AI workforce foundation: org memory permissions, org roles, unified audit log,
-- AI-credit spend approvals, org attribution on usage lines, org AI-credit/memory stats,
-- and relayer-pushed memory usage stats.

-- ============================================================
-- Org memory share permissions (from memory.move OrgMemoryPermission* events)
-- ============================================================

CREATE TABLE IF NOT EXISTS org_memory_permissions (
    organization_id TEXT NOT NULL,
    member_address TEXT NOT NULL,
    -- Bit from the fixed org permission set (1=memory_read, 2=memory_write, 4=agent_manager,
    -- 8=budget_manager, 16=spend_approver, 32=dashboard_viewer, 64=auditor).
    permission_kind BIGINT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    granted_by TEXT NOT NULL,
    group_id TEXT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (organization_id, member_address, permission_kind)
);

CREATE INDEX IF NOT EXISTS idx_org_memory_permissions_member
    ON org_memory_permissions (member_address, active);
CREATE INDEX IF NOT EXISTS idx_org_memory_permissions_org_active
    ON org_memory_permissions (organization_id, active);

-- ============================================================
-- Org roles (definitions + assignments; from OrgRole* events)
-- ============================================================

CREATE TABLE IF NOT EXISTS org_roles (
    organization_id TEXT NOT NULL,
    role_name TEXT NOT NULL,
    mask BIGINT NOT NULL,
    is_builtin BOOLEAN NOT NULL DEFAULT FALSE,
    defined_by TEXT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    updated_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (organization_id, role_name)
);

CREATE TABLE IF NOT EXISTS org_role_assignments (
    organization_id TEXT NOT NULL,
    member_address TEXT NOT NULL,
    role_name TEXT NOT NULL,
    -- Full role mask at assignment time.
    role_mask BIGINT NOT NULL,
    -- Delta actually granted by the assignment (used for exact revocation).
    assigned_mask BIGINT NOT NULL,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    assigned_by TEXT NOT NULL,
    assigned_at_ms BIGINT NOT NULL,
    revoked_at_ms BIGINT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (organization_id, member_address, role_name)
);

CREATE INDEX IF NOT EXISTS idx_org_role_assignments_member
    ON org_role_assignments (member_address, active);
CREATE INDEX IF NOT EXISTS idx_org_role_assignments_org_active
    ON org_role_assignments (organization_id, active);

-- ============================================================
-- Unified enterprise audit log (chain + off-chain services)
-- ============================================================

CREATE TABLE IF NOT EXISTS audit_log (
    id BIGSERIAL NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- 'chain' | 'oracle' | 'memory_relayer' | 'workflow_relayer' | 'scheduler'
    source TEXT NOT NULL,
    actor_address TEXT NOT NULL,
    -- 'human' | 'agent' | 'service'
    actor_type TEXT NOT NULL,
    action TEXT NOT NULL,
    target_type TEXT NOT NULL,
    target_id TEXT NOT NULL,
    organization_id TEXT,
    account_id TEXT,
    prev_state JSONB,
    new_state JSONB,
    tx_digest TEXT,
    event_id TEXT,
    -- Off-chain idempotency key (nullable for chain rows which dedupe by event_id).
    idempotency_key TEXT,
    metadata JSONB,
    PRIMARY KEY (id, time)
);

SELECT create_hypertable('audit_log', 'time', if_not_exists => TRUE);

CREATE INDEX IF NOT EXISTS idx_audit_log_org_time
    ON audit_log (organization_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_audit_log_actor_time
    ON audit_log (actor_address, time DESC);
CREATE INDEX IF NOT EXISTS idx_audit_log_target
    ON audit_log (target_type, target_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_audit_log_action_time
    ON audit_log (action, time DESC);
CREATE UNIQUE INDEX IF NOT EXISTS idx_audit_log_event_dedupe
    ON audit_log (event_id, time)
    WHERE event_id IS NOT NULL;
CREATE UNIQUE INDEX IF NOT EXISTS idx_audit_log_idempotency
    ON audit_log (idempotency_key, time)
    WHERE idempotency_key IS NOT NULL;

-- ============================================================
-- AI-credit spend approvals (lifecycle: requested -> approved -> consumed|revoked|expired)
-- ============================================================

CREATE TABLE IF NOT EXISTS ai_credit_spend_approvals (
    balance_id TEXT NOT NULL,
    agent_object_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'requested'
        CHECK (status IN ('requested', 'approved', 'consumed', 'revoked', 'expired')),
    -- Off-chain: what the oracle estimated when it rejected the request.
    requested_amount_mist BIGINT,
    threshold_mist BIGINT,
    -- On-chain allowance fields (present once approved).
    approval_nonce BIGINT,
    max_amount_mist BIGINT,
    expires_at_ms BIGINT,
    approved_by TEXT,
    approved_by_agent_id TEXT,
    organization_id TEXT,
    consumed_amount_mist BIGINT,
    requested_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    event_id TEXT,
    PRIMARY KEY (balance_id, agent_object_id)
);

CREATE INDEX IF NOT EXISTS idx_ai_credit_spend_approvals_status
    ON ai_credit_spend_approvals (status, updated_at DESC);
CREATE INDEX IF NOT EXISTS idx_ai_credit_spend_approvals_agent
    ON ai_credit_spend_approvals (agent_object_id, status);
CREATE INDEX IF NOT EXISTS idx_ai_credit_spend_approvals_org
    ON ai_credit_spend_approvals (organization_id, status);

-- ============================================================
-- Org attribution on usage lines
-- ============================================================

ALTER TABLE ai_credit_usage_lines ADD COLUMN IF NOT EXISTS organization_id TEXT;

UPDATE ai_credit_usage_lines l
SET organization_id = a.organization_id
FROM sub_agents a
WHERE l.agent_object_id = a.agent_object_id
  AND l.organization_id IS NULL
  AND a.organization_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_ai_credit_usage_lines_organization
    ON ai_credit_usage_lines (organization_id, created_at DESC);

-- ============================================================
-- Org stats: AI-credit spend + memory usage columns
-- ============================================================

ALTER TABLE sub_agent_organization_stats
    ADD COLUMN IF NOT EXISTS ai_credit_spent_mist BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS ai_credit_usage_events BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS memory_entries BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS memory_bytes BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS org_shared_memory_entries BIGINT NOT NULL DEFAULT 0;

ALTER TABLE sub_agent_organization_stats_daily
    ADD COLUMN IF NOT EXISTS ai_credit_spent_mist BIGINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS memory_bytes BIGINT NOT NULL DEFAULT 0;

-- ============================================================
-- Memory usage stats (pushed by the memory relayer via internal ingest)
-- ============================================================

CREATE TABLE IF NOT EXISTS memory_usage_stats (
    agent_object_id TEXT NOT NULL PRIMARY KEY,
    organization_id TEXT,
    account_id TEXT,
    entries BIGINT NOT NULL DEFAULT 0,
    bytes BIGINT NOT NULL DEFAULT 0,
    org_shared_entries BIGINT NOT NULL DEFAULT 0,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_memory_usage_stats_org
    ON memory_usage_stats (organization_id);
CREATE INDEX IF NOT EXISTS idx_memory_usage_stats_account
    ON memory_usage_stats (account_id);
