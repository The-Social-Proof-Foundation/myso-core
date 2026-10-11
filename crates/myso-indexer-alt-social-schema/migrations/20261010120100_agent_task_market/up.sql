-- Agent task market (agent_task module events). One row per task, updated in event order by
-- the sequential TaskMarketHandler, so plain UPDATEs are safe.
CREATE TABLE IF NOT EXISTS agent_tasks (
    task_id TEXT PRIMARY KEY,
    buyer_principal TEXT NOT NULL,
    buyer_agent_id TEXT,
    buyer_memory_account_id TEXT NOT NULL,
    seller_principal TEXT NOT NULL,
    seller_agent_id TEXT NOT NULL,
    seller_memory_account_id TEXT NOT NULL,
    organization_id TEXT,
    room_group_id TEXT,
    platform_id TEXT,
    title TEXT NOT NULL,
    scope_digest TEXT NOT NULL,
    amount_mist BIGINT NOT NULL,
    accept_window_ms BIGINT NOT NULL,
    delivery_window_ms BIGINT NOT NULL,
    settle_window_ms BIGINT NOT NULL,
    platform_fee_bps BIGINT NOT NULL,
    ecosystem_fee_bps BIGINT NOT NULL,
    terms_digest TEXT NOT NULL,
    -- 0 offered, 1 accepted, 2 delivered, 3 settled, 4 refunded, 5 cancelled
    status SMALLINT NOT NULL DEFAULT 0,
    close_kind SMALLINT,
    settlement_kind SMALLINT,
    refund_kind SMALLINT,
    created_at_ms BIGINT NOT NULL,
    accept_deadline_ms BIGINT NOT NULL,
    accepted_at_ms BIGINT,
    delivery_deadline_ms BIGINT,
    delivered_at_ms BIGINT,
    settle_deadline_ms BIGINT,
    settled_at_ms BIGINT,
    refunded_at_ms BIGINT,
    cancelled_at_ms BIGINT,
    review_deadline_ms BIGINT,
    deliverable_digest TEXT,
    deliverable_uri TEXT,
    platform_fee_mist BIGINT,
    ecosystem_fee_mist BIGINT,
    net_amount_mist BIGINT,
    -- 0 none, 1 buyer, 2 seller
    refund_proposer SMALLINT NOT NULL DEFAULT 0,
    refund_proposed_at_ms BIGINT,
    buyer_credibility_bps BIGINT,
    seller_credibility_bps BIGINT,
    buyer_review_stars SMALLINT,
    seller_review_stars SMALLINT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_agent_tasks_buyer ON agent_tasks (buyer_principal, created_at_ms DESC);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_buyer_agent ON agent_tasks (buyer_agent_id, created_at_ms DESC)
    WHERE buyer_agent_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_agent_tasks_seller_agent ON agent_tasks (seller_agent_id, created_at_ms DESC);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_seller_principal ON agent_tasks (seller_principal, created_at_ms DESC);
CREATE INDEX IF NOT EXISTS idx_agent_tasks_organization ON agent_tasks (organization_id, created_at_ms DESC)
    WHERE organization_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_agent_tasks_room ON agent_tasks (room_group_id)
    WHERE room_group_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_agent_tasks_status ON agent_tasks (status, created_at_ms DESC);

-- One receipt per settled task (settled by the buyer or auto-released).
CREATE TABLE IF NOT EXISTS task_receipts (
    task_id TEXT PRIMARY KEY,
    receipt_id TEXT NOT NULL,
    buyer_principal TEXT NOT NULL,
    buyer_agent_id TEXT,
    seller_principal TEXT NOT NULL,
    seller_agent_id TEXT NOT NULL,
    organization_id TEXT,
    room_group_id TEXT,
    settlement_kind SMALLINT NOT NULL,
    amount_mist BIGINT NOT NULL,
    platform_fee_mist BIGINT NOT NULL,
    ecosystem_fee_mist BIGINT NOT NULL,
    net_amount_mist BIGINT NOT NULL,
    deliverable_digest TEXT,
    deliverable_uri TEXT,
    buyer_credibility_bps BIGINT NOT NULL,
    seller_credibility_bps BIGINT NOT NULL,
    settle_deadline_ms BIGINT NOT NULL,
    review_deadline_ms BIGINT NOT NULL,
    settled_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_task_receipts_seller_agent ON task_receipts (seller_agent_id, settled_at_ms DESC);
CREATE INDEX IF NOT EXISTS idx_task_receipts_buyer_agent ON task_receipts (buyer_agent_id, settled_at_ms DESC)
    WHERE buyer_agent_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_task_receipts_buyer ON task_receipts (buyer_principal, settled_at_ms DESC);

-- At most one review per direction per task: the key mirrors the on-chain once-per-direction rule.
CREATE TABLE IF NOT EXISTS task_reviews (
    task_id TEXT NOT NULL,
    -- 1 buyer reviewed the seller, 2 seller reviewed the buyer
    rater_role SMALLINT NOT NULL,
    receipt_id TEXT NOT NULL,
    rater_principal TEXT NOT NULL,
    rater_agent_id TEXT,
    reviewee_principal TEXT NOT NULL,
    reviewee_agent_id TEXT,
    stars SMALLINT NOT NULL,
    rater_credibility_bps BIGINT NOT NULL,
    value_weight_bps BIGINT NOT NULL,
    base_weight_bps BIGINT NOT NULL,
    prior_pair_reviews BIGINT NOT NULL,
    weight_bps BIGINT NOT NULL,
    reviewed_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (task_id, rater_role)
);

CREATE INDEX IF NOT EXISTS idx_task_reviews_reviewee_agent ON task_reviews (reviewee_agent_id, reviewed_at_ms DESC)
    WHERE reviewee_agent_id IS NOT NULL;

-- Every TaskConfigUpdated, append-only; `task_config` is the latest row.
CREATE TABLE IF NOT EXISTS task_config_history (
    id BIGSERIAL PRIMARY KEY,
    updated_by TEXT NOT NULL,
    platform_fee_bps BIGINT NOT NULL,
    ecosystem_fee_bps BIGINT NOT NULL,
    ecosystem_fee_recipient TEXT NOT NULL,
    min_escrow_mist BIGINT NOT NULL,
    accept_window_min_ms BIGINT NOT NULL,
    accept_window_default_ms BIGINT NOT NULL,
    accept_window_max_ms BIGINT NOT NULL,
    delivery_window_min_ms BIGINT NOT NULL,
    delivery_window_max_ms BIGINT NOT NULL,
    settle_window_min_ms BIGINT NOT NULL,
    settle_window_max_ms BIGINT NOT NULL,
    release_grace_ms BIGINT NOT NULL,
    review_window_ms BIGINT NOT NULL,
    max_title_bytes BIGINT NOT NULL,
    max_uri_bytes BIGINT NOT NULL,
    active BOOLEAN NOT NULL,
    prior_mean_milli BIGINT NOT NULL,
    prior_weight_bps BIGINT NOT NULL,
    value_ref_mist BIGINT NOT NULL,
    min_value_weight_bps BIGINT NOT NULL,
    max_value_weight_bps BIGINT NOT NULL,
    cred_base_bps BIGINT NOT NULL,
    cred_step_bps BIGINT NOT NULL,
    cred_cap_n BIGINT NOT NULL,
    cred_max_bps BIGINT NOT NULL,
    overall_seller_weight_bps BIGINT NOT NULL,
    overall_buyer_weight_bps BIGINT NOT NULL,
    timestamp_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL UNIQUE,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE VIEW task_config AS
    SELECT * FROM task_config_history ORDER BY id DESC LIMIT 1;
