-- Knowledge Graph + Registry foundation and spot_markets dedup columns.

CREATE TABLE IF NOT EXISTS knowledge_entities (
    id VARCHAR(128) PRIMARY KEY,
    kind VARCHAR(64) NOT NULL,
    name TEXT NOT NULL,
    aliases TEXT[] NOT NULL DEFAULT '{}',
    domain VARCHAR(64) NOT NULL DEFAULT 'unknown',
    external_refs JSONB NOT NULL DEFAULT '{}',
    provenance JSONB NOT NULL DEFAULT '{}',
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_knowledge_entities_aliases
    ON knowledge_entities USING GIN (aliases);

CREATE TABLE IF NOT EXISTS knowledge_competitions (
    id VARCHAR(128) PRIMARY KEY,
    kind VARCHAR(64) NOT NULL,
    label TEXT NOT NULL,
    domain VARCHAR(64) NOT NULL DEFAULT 'unknown',
    recurrence_rule TEXT,
    provenance JSONB NOT NULL DEFAULT '{}',
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS knowledge_events (
    id VARCHAR(128) PRIMARY KEY,
    competition_id VARCHAR(128) REFERENCES knowledge_competitions(id) ON DELETE SET NULL,
    label TEXT NOT NULL,
    domain VARCHAR(64) NOT NULL DEFAULT 'unknown',
    start_at TIMESTAMPTZ,
    end_at TIMESTAMPTZ NOT NULL,
    keywords TEXT[] NOT NULL DEFAULT '{}',
    entities JSONB NOT NULL DEFAULT '[]',
    feed_url TEXT,
    match_predicate TEXT,
    preferred_source_keys TEXT[] NOT NULL DEFAULT '{}',
    priority INT NOT NULL DEFAULT 0,
    provenance JSONB NOT NULL DEFAULT '{}',
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_knowledge_events_keywords
    ON knowledge_events USING GIN (keywords);

CREATE TABLE IF NOT EXISTS knowledge_metrics (
    id VARCHAR(128) PRIMARY KEY,
    key VARCHAR(128) NOT NULL,
    unit VARCHAR(64),
    domain VARCHAR(64) NOT NULL DEFAULT 'unknown',
    aggregation VARCHAR(64),
    schema JSONB NOT NULL DEFAULT '{}',
    provenance JSONB NOT NULL DEFAULT '{}',
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS knowledge_observations (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    metric_id VARCHAR(128) NOT NULL REFERENCES knowledge_metrics(id) ON DELETE CASCADE,
    entity_id VARCHAR(128) REFERENCES knowledge_entities(id) ON DELETE SET NULL,
    event_id VARCHAR(128) REFERENCES knowledge_events(id) ON DELETE SET NULL,
    observed_at TIMESTAMPTZ NOT NULL,
    value JSONB NOT NULL,
    provenance JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_knowledge_observations_lookup
    ON knowledge_observations (metric_id, entity_id, observed_at DESC);

CREATE TABLE IF NOT EXISTS knowledge_relationships (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    subject_id VARCHAR(128) NOT NULL,
    object_id VARCHAR(128) NOT NULL,
    rel_type VARCHAR(64) NOT NULL,
    valid_from TIMESTAMPTZ,
    valid_to TIMESTAMPTZ,
    provenance JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_knowledge_relationships_subject
    ON knowledge_relationships (subject_id, rel_type);

ALTER TABLE spot_markets
    ADD COLUMN IF NOT EXISTS entity_ref VARCHAR(128),
    ADD COLUMN IF NOT EXISTS competition_ref VARCHAR(128),
    ADD COLUMN IF NOT EXISTS event_ref VARCHAR(128),
    ADD COLUMN IF NOT EXISTS metric_ref VARCHAR(128),
    ADD COLUMN IF NOT EXISTS outcome_identity_hash VARCHAR(128),
    ADD COLUMN IF NOT EXISTS deadline_day VARCHAR(16);

CREATE INDEX IF NOT EXISTS idx_spot_markets_outcome_identity
    ON spot_markets(outcome_identity_hash)
    WHERE outcome_identity_hash IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_spot_markets_graph_dedup
    ON spot_markets(event_ref, entity_ref, deadline_day)
    WHERE event_ref IS NOT NULL AND entity_ref IS NOT NULL AND deadline_day IS NOT NULL;
