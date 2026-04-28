-- Create spt_events table for SPT event history (kill switch, pool created, config updates, etc.)
-- The indexer inserts SocialProofTokensEvent rows (e.g. EmergencyKillSwitchEvent, TokenPoolCreatedEvent) into this table.

CREATE TABLE IF NOT EXISTS spt_events (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR NOT NULL,
    event_data JSONB NOT NULL DEFAULT '{}'::jsonb,
    event_id VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_spt_events_event_type ON spt_events(event_type);
CREATE INDEX IF NOT EXISTS idx_spt_events_created_at ON spt_events(created_at DESC);
