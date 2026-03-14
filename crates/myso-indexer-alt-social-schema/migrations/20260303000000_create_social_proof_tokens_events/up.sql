-- Create social_proof_tokens_events table for SPT event history (kill switch, config updates, etc.)
-- The indexer inserts SocialProofTokensEvent rows (e.g. EmergencyKillSwitchEvent) into this table.
-- Previously only token_exchange_events existed; this table is what the schema and handler expect.

CREATE TABLE IF NOT EXISTS social_proof_tokens_events (
    id SERIAL PRIMARY KEY,
    event_type VARCHAR NOT NULL,
    event_data JSONB NOT NULL DEFAULT '{}'::jsonb,
    event_id VARCHAR NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_social_proof_tokens_events_event_type ON social_proof_tokens_events(event_type);
CREATE INDEX IF NOT EXISTS idx_social_proof_tokens_events_created_at ON social_proof_tokens_events(created_at DESC);
