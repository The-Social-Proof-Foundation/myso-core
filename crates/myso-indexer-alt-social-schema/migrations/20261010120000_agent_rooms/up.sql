-- Agent rooms: agent-created messaging groups tagged as rooms (AgentRoomBound events).
-- One row per group; the event is emitted exactly once per room at creation.
CREATE TABLE IF NOT EXISTS agent_rooms (
    group_id TEXT PRIMARY KEY,
    room_kind SMALLINT NOT NULL,
    task_id TEXT,
    creator_actor TEXT NOT NULL,
    creator_principal TEXT NOT NULL,
    organization_id TEXT,
    created_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_agent_rooms_creator_principal
    ON agent_rooms (creator_principal, created_at_ms DESC);
CREATE INDEX IF NOT EXISTS idx_agent_rooms_organization
    ON agent_rooms (organization_id, created_at_ms DESC)
    WHERE organization_id IS NOT NULL;
CREATE INDEX IF NOT EXISTS idx_agent_rooms_task
    ON agent_rooms (task_id)
    WHERE task_id IS NOT NULL;

COMMENT ON TABLE agent_rooms IS 'Agent rooms indexed from messaging::AgentRoomBound events.';
