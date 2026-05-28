-- MyData marketplace (event-backed): broad/sub pools, listing assignments, anchors, merkle roots, claims.

CREATE TABLE IF NOT EXISTS mydata_broad_pools (
    pool_id TEXT NOT NULL PRIMARY KEY,
    name TEXT NOT NULL,
    created_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_mydata_broad_pools_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at_ms / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_mydata_broad_pools_time ON mydata_broad_pools;
CREATE TRIGGER set_mydata_broad_pools_time
BEFORE INSERT OR UPDATE ON mydata_broad_pools
FOR EACH ROW
EXECUTE FUNCTION update_mydata_broad_pools_time();

CREATE INDEX IF NOT EXISTS idx_mydata_broad_pools_time ON mydata_broad_pools (time DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_broad_pools_event_id ON mydata_broad_pools (event_id);

CREATE TABLE IF NOT EXISTS mydata_sub_pools (
    sub_pool_id TEXT NOT NULL PRIMARY KEY,
    broad_pool_id TEXT NOT NULL,
    name TEXT NOT NULL,
    created_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_mydata_sub_pools_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at_ms / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_mydata_sub_pools_time ON mydata_sub_pools;
CREATE TRIGGER set_mydata_sub_pools_time
BEFORE INSERT OR UPDATE ON mydata_sub_pools
FOR EACH ROW
EXECUTE FUNCTION update_mydata_sub_pools_time();

CREATE INDEX IF NOT EXISTS idx_mydata_sub_pools_broad ON mydata_sub_pools (broad_pool_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_mydata_sub_pools_time ON mydata_sub_pools (time DESC);

CREATE TABLE IF NOT EXISTS mydata_listing_sub_pools (
    listing_id TEXT NOT NULL,
    sub_pool_id TEXT NOT NULL,
    assigned_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (listing_id, sub_pool_id)
);

CREATE OR REPLACE FUNCTION update_mydata_listing_sub_pools_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.assigned_at_ms / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_mydata_listing_sub_pools_time ON mydata_listing_sub_pools;
CREATE TRIGGER set_mydata_listing_sub_pools_time
BEFORE INSERT OR UPDATE ON mydata_listing_sub_pools
FOR EACH ROW
EXECUTE FUNCTION update_mydata_listing_sub_pools_time();

CREATE INDEX IF NOT EXISTS idx_mydata_listing_sub_pools_sub ON mydata_listing_sub_pools (sub_pool_id);
CREATE INDEX IF NOT EXISTS idx_mydata_listing_sub_pools_listing ON mydata_listing_sub_pools (listing_id);

CREATE TABLE IF NOT EXISTS mydata_merkle_roots (
    snapshot_id TEXT NOT NULL PRIMARY KEY,
    root_hash TEXT NOT NULL,
    published_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_mydata_merkle_roots_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.published_at_ms / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_mydata_merkle_roots_time ON mydata_merkle_roots;
CREATE TRIGGER set_mydata_merkle_roots_time
BEFORE INSERT OR UPDATE ON mydata_merkle_roots
FOR EACH ROW
EXECUTE FUNCTION update_mydata_merkle_roots_time();

CREATE UNIQUE INDEX IF NOT EXISTS idx_mydata_merkle_roots_event_id ON mydata_merkle_roots (event_id);

CREATE TABLE IF NOT EXISTS mydata_snapshot_anchors (
    id SERIAL NOT NULL,
    snapshot_id TEXT NOT NULL,
    buyer_address TEXT NOT NULL,
    price_paid BIGINT NOT NULL,
    created_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    manifest_hash TEXT,
    payment_reference TEXT
);

CREATE OR REPLACE FUNCTION update_mydata_snapshot_anchors_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.created_at_ms / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_mydata_snapshot_anchors_time ON mydata_snapshot_anchors;
CREATE TRIGGER set_mydata_snapshot_anchors_time
BEFORE INSERT ON mydata_snapshot_anchors
FOR EACH ROW
EXECUTE FUNCTION update_mydata_snapshot_anchors_time();

SELECT create_hypertable('mydata_snapshot_anchors', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'mydata_snapshot_anchors_pkey'
    ) THEN
        ALTER TABLE mydata_snapshot_anchors ADD PRIMARY KEY (id, time);
    END IF;
END $$;

CREATE UNIQUE INDEX IF NOT EXISTS idx_mydata_snapshot_anchors_event_time
    ON mydata_snapshot_anchors (event_id, time);

CREATE INDEX IF NOT EXISTS idx_mydata_snapshot_anchors_snapshot_time
    ON mydata_snapshot_anchors (snapshot_id, time DESC);

CREATE TABLE IF NOT EXISTS mydata_distribution_rounds (
    snapshot_id TEXT NOT NULL PRIMARY KEY,
    total_amount BIGINT NOT NULL,
    contributor_count BIGINT NOT NULL,
    merkle_root TEXT NOT NULL,
    published_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_mydata_distribution_rounds_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.published_at_ms / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_mydata_distribution_rounds_time ON mydata_distribution_rounds;
CREATE TRIGGER set_mydata_distribution_rounds_time
BEFORE INSERT OR UPDATE ON mydata_distribution_rounds
FOR EACH ROW
EXECUTE FUNCTION update_mydata_distribution_rounds_time();

CREATE UNIQUE INDEX IF NOT EXISTS idx_mydata_distribution_rounds_event_id
    ON mydata_distribution_rounds (event_id);

CREATE INDEX IF NOT EXISTS idx_mydata_distribution_rounds_time
    ON mydata_distribution_rounds (time DESC);

CREATE TABLE IF NOT EXISTS mydata_claims (
    id SERIAL NOT NULL,
    snapshot_id TEXT NOT NULL,
    claimant TEXT NOT NULL,
    amount BIGINT NOT NULL,
    claimed_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE OR REPLACE FUNCTION update_mydata_claims_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.claimed_at_ms / 1000.0);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS set_mydata_claims_time ON mydata_claims;
CREATE TRIGGER set_mydata_claims_time
BEFORE INSERT ON mydata_claims
FOR EACH ROW
EXECUTE FUNCTION update_mydata_claims_time();

SELECT create_hypertable('mydata_claims', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'mydata_claims_pkey'
    ) THEN
        ALTER TABLE mydata_claims ADD PRIMARY KEY (id, time);
    END IF;
END $$;

CREATE UNIQUE INDEX IF NOT EXISTS idx_mydata_claims_event_time
    ON mydata_claims (event_id, time);

CREATE INDEX IF NOT EXISTS idx_mydata_claims_snapshot_time
    ON mydata_claims (snapshot_id, time DESC);

COMMENT ON TABLE mydata_broad_pools IS 'Marketplace broad pools from BroadPoolCreatedEvent';
COMMENT ON TABLE mydata_sub_pools IS 'Marketplace sub pools from SubPoolCreatedEvent';
COMMENT ON TABLE mydata_listing_sub_pools IS 'Listing to sub-pool assignments; state replaced per MyDataAssignedToSubPoolEvent';
COMMENT ON TABLE mydata_snapshot_anchors IS 'Snapshot anchor records from SnapshotAnchorRecordedEvent (manifest_hash and payment_reference nullable for legacy events)';
COMMENT ON TABLE mydata_merkle_roots IS 'Published Merkle roots from MerkleRootPublishedEvent';
COMMENT ON TABLE mydata_distribution_rounds IS 'Contributor distribution rounds from DistributionRecordedEvent';
COMMENT ON TABLE mydata_claims IS 'Claim payouts from ClaimExecutedEvent';
