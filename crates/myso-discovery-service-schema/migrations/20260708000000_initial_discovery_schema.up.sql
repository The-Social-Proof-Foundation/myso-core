-- Discovery service initial schema

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE discovery_sources (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    adapter_type VARCHAR(64) NOT NULL,
    domain VARCHAR(32) NOT NULL DEFAULT 'creative',
    source_url TEXT,
    config JSONB NOT NULL DEFAULT '{}',
    trust_score DOUBLE PRECISION NOT NULL DEFAULT 0.5,
    enabled BOOLEAN NOT NULL DEFAULT true,
    terms_notes TEXT,
    last_polled_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE creator_candidates (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    primary_x_handle VARCHAR(256) UNIQUE,
    identity_hash VARCHAR(128),
    display_name VARCHAR(256),
    aliases JSONB NOT NULL DEFAULT '[]',
    platform_handles JSONB NOT NULL DEFAULT '{}',
    source_urls JSONB NOT NULL DEFAULT '[]',
    creator_confidence DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    work_count INTEGER NOT NULL DEFAULT 0,
    blockchain_hit_count INTEGER NOT NULL DEFAULT 0,
    similarity_hit_count INTEGER NOT NULL DEFAULT 0,
    lifecycle_state VARCHAR(32) NOT NULL DEFAULT 'unresolved',
    merge_target_id UUID REFERENCES creator_candidates(id),
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE discovery_assets (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    source_id UUID REFERENCES discovery_sources(id),
    external_source_url TEXT NOT NULL,
    canonical_metadata JSONB NOT NULL DEFAULT '{}',
    media_type VARCHAR(32) NOT NULL,
    content_hash VARCHAR(128),
    metadata_hash VARCHAR(128),
    lifecycle_state VARCHAR(32) NOT NULL DEFAULT 'discovered',
    source_trust_score DOUBLE PRECISION NOT NULL DEFAULT 0.5,
    work_confidence DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    creator_confidence DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    creator_candidate_id UUID REFERENCES creator_candidates(id),
    active_embedding_version VARCHAR(64),
    related_on_chain_post VARCHAR(128),
    priority_score BIGINT NOT NULL DEFAULT 0,
    discovered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    exclusion_reason TEXT,
    UNIQUE (external_source_url)
);

CREATE INDEX idx_discovery_assets_lifecycle ON discovery_assets(lifecycle_state, priority_score DESC);
CREATE INDEX idx_discovery_assets_creator ON discovery_assets(creator_candidate_id);

CREATE TABLE creator_candidate_assets (
    creator_candidate_id UUID NOT NULL REFERENCES creator_candidates(id) ON DELETE CASCADE,
    discovery_asset_id UUID NOT NULL REFERENCES discovery_assets(id) ON DELETE CASCADE,
    attribution_confidence DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    attribution_source VARCHAR(64) NOT NULL DEFAULT 'auto',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (creator_candidate_id, discovery_asset_id)
);

CREATE TABLE discovery_jobs (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_type VARCHAR(32) NOT NULL,
    discovery_asset_id UUID REFERENCES discovery_assets(id) ON DELETE CASCADE,
    priority_score BIGINT NOT NULL DEFAULT 0,
    status VARCHAR(32) NOT NULL DEFAULT 'pending',
    attempts INTEGER NOT NULL DEFAULT 0,
    last_error TEXT,
    payload JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_discovery_jobs_claim ON discovery_jobs(status, priority_score DESC, created_at);

CREATE TABLE provenance_hits (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    network VARCHAR(20) NOT NULL,
    post_id VARCHAR(128) NOT NULL,
    query_media_id VARCHAR(100),
    discovery_asset_id UUID REFERENCES discovery_assets(id),
    creator_candidate_id UUID REFERENCES creator_candidates(id),
    similarity_score DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    match_type VARCHAR(50),
    work_confidence DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    creator_confidence DOUBLE PRECISION NOT NULL DEFAULT 0.0,
    decision VARCHAR(32) NOT NULL DEFAULT 'pending',
    vault_provisioned BOOLEAN NOT NULL DEFAULT false,
    vault_identity_hash VARCHAR(128),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_provenance_hits_post ON provenance_hits(network, post_id, created_at DESC);

CREATE TABLE discovery_exclusions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    target_type VARCHAR(32) NOT NULL,
    target_id UUID NOT NULL,
    reason TEXT NOT NULL,
    requested_by VARCHAR(256),
    status VARCHAR(32) NOT NULL DEFAULT 'active',
    audit JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE discovery_audit_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    entity_type VARCHAR(32) NOT NULL,
    entity_id UUID NOT NULL,
    action VARCHAR(64) NOT NULL,
    details JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_discovery_audit_entity ON discovery_audit_log(entity_type, entity_id, created_at DESC);
