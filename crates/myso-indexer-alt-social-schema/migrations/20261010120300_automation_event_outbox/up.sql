-- Task market events waiting to be delivered to the automation engine.
--
-- The task indexer writes rows here in the same transaction that applies the event to
-- `agent_tasks`, so an event is either both indexed and queued or neither. The social server's
-- forwarder drains the table in order, posts each row to the engine and stamps `sent_at`. The engine
-- deduplicates on `dedup_key`, which makes redelivery after a crash harmless.
--
-- One row per party that may need to react, so a job scoped to one agent or account only
-- ever sees its own events.
CREATE TABLE IF NOT EXISTS automation_event_outbox (
    id BIGSERIAL PRIMARY KEY,
    dedup_key TEXT NOT NULL UNIQUE,
    event_family TEXT NOT NULL,
    event_type TEXT NOT NULL,
    -- The target party's MemoryAccount and, when it acts through one, its SubAgent.
    account_id TEXT NOT NULL,
    agent_object_id TEXT,
    payload JSONB NOT NULL,
    occurred_at_ms BIGINT NOT NULL,
    source_event_id TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    sent_at TIMESTAMPTZ,
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    next_attempt_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_automation_event_outbox_pending
    ON automation_event_outbox (next_attempt_at, id) WHERE sent_at IS NULL;
