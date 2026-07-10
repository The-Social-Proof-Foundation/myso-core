-- Pluggable event provider registry and discovered scheduled events.

CREATE TABLE IF NOT EXISTS spot_event_providers (
    id UUID PRIMARY KEY,
    provider_key VARCHAR(128) NOT NULL UNIQUE,
    provider_type VARCHAR(64) NOT NULL,
    enabled BOOLEAN NOT NULL DEFAULT true,
    poll_interval_secs INTEGER NOT NULL DEFAULT 3600,
    config JSONB NOT NULL DEFAULT '{}',
    last_sync_at TIMESTAMPTZ,
    last_sync_status VARCHAR(32),
    health_healthy BOOLEAN,
    health_message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_spot_event_providers_enabled
    ON spot_event_providers(enabled, poll_interval_secs);

CREATE TABLE IF NOT EXISTS spot_scheduled_events (
    id UUID PRIMARY KEY,
    provider_key VARCHAR(128) NOT NULL,
    external_id VARCHAR(256) NOT NULL,
    label TEXT NOT NULL,
    category VARCHAR(32) NOT NULL DEFAULT 'other',
    start_at_ms BIGINT,
    end_at_ms BIGINT NOT NULL,
    keywords TEXT[] NOT NULL DEFAULT '{}',
    entities JSONB NOT NULL DEFAULT '[]',
    feed_url TEXT,
    match_predicate TEXT,
    preferred_source_keys TEXT[] NOT NULL DEFAULT '{}',
    priority INTEGER NOT NULL DEFAULT 0,
    enabled BOOLEAN NOT NULL DEFAULT true,
    provenance JSONB NOT NULL DEFAULT '{}',
    admin_override JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (provider_key, external_id)
);

CREATE INDEX IF NOT EXISTS idx_spot_scheduled_events_active
    ON spot_scheduled_events(enabled, end_at_ms DESC);

CREATE INDEX IF NOT EXISTS idx_spot_scheduled_events_keywords
    ON spot_scheduled_events USING GIN (keywords);
