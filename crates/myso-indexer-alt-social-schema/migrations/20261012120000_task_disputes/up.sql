-- Task disputes (agent_task dispute events). One row per disputed task: a task can be disputed
-- once, so the row is created by `TaskDisputed` and updated in event order by the sequential
-- TaskMarketHandler. Kept apart from agent_tasks so that table's column list stays unchanged;
-- the task's own status (6 = disputed) is tracked on agent_tasks.
CREATE TABLE IF NOT EXISTS task_disputes (
    task_id TEXT PRIMARY KEY,
    case_id TEXT NOT NULL,
    opened_by_role SMALLINT NOT NULL,
    -- 1 open, 2 cancelled before a draw, 3 resolved
    state SMALLINT NOT NULL DEFAULT 1,
    fee_each_mist BIGINT NOT NULL,
    buyer_paid_mist BIGINT NOT NULL DEFAULT 0,
    seller_paid_mist BIGINT NOT NULL DEFAULT 0,
    opened_at_ms BIGINT NOT NULL,
    -- Set when the jury failed: the settlement window ends here, then the 50/50 split runs.
    neutral_window_ends_ms BIGINT,
    -- Pending mutual agreement (0 = none).
    agreement_proposer SMALLINT NOT NULL DEFAULT 0,
    agreement_outcome SMALLINT,
    resolved_outcome SMALLINT,
    -- 1 verdict, 2 agreement, 3 neutral split
    resolved_source SMALLINT,
    resolved_at_ms BIGINT,
    fees_settled BOOLEAN NOT NULL DEFAULT FALSE,
    buyer_fee_refund_mist BIGINT,
    seller_fee_refund_mist BIGINT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL
);

CREATE INDEX IF NOT EXISTS task_disputes_case_idx ON task_disputes (case_id);
CREATE INDEX IF NOT EXISTS task_disputes_open_idx ON task_disputes (state) WHERE state = 1;
