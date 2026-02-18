CREATE TABLE upgrade_events (
    id SERIAL PRIMARY KEY,
    package_id VARCHAR(66) NOT NULL,
    version BIGINT NOT NULL,
    event_id VARCHAR(128) NOT NULL,
    transaction_id VARCHAR(128) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE object_migrated_events (
    id SERIAL PRIMARY KEY,
    object_id VARCHAR(66) NOT NULL,
    object_type VARCHAR(255) NOT NULL,
    old_version BIGINT NOT NULL,
    new_version BIGINT NOT NULL,
    migrated_by VARCHAR(66) NOT NULL,
    event_id VARCHAR(128) NOT NULL,
    transaction_id VARCHAR(128) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_upgrade_events_package_id ON upgrade_events(package_id);
CREATE INDEX idx_object_migrated_events_object_id ON object_migrated_events(object_id);
