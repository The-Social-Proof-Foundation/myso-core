CREATE TABLE IF NOT EXISTS username_registry (
    username TEXT PRIMARY KEY CHECK (username ~ '^[a-z0-9_.]+$'),
    profile_id TEXT NOT NULL UNIQUE,
    transaction_id TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_username_registry_profile_id ON username_registry (profile_id);

CREATE TABLE IF NOT EXISTS username_reservations (
    id SERIAL PRIMARY KEY,
    username TEXT NOT NULL CHECK (username ~ '^[a-z0-9_.]+$'),
    reason SMALLINT NOT NULL,
    reserved_by TEXT NOT NULL,
    reserved_at BIGINT NOT NULL,
    released_by TEXT,
    released_at BIGINT,
    status TEXT NOT NULL CHECK (status IN ('active', 'released')),
    reserve_transaction_id TEXT NOT NULL,
    release_transaction_id TEXT,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE UNIQUE INDEX IF NOT EXISTS idx_username_reservations_active
    ON username_reservations (username, reason)
    WHERE status = 'active';

CREATE INDEX IF NOT EXISTS idx_username_reservations_username
    ON username_reservations (username);

CREATE INDEX IF NOT EXISTS idx_username_reservations_reserved_by
    ON username_reservations (reserved_by);

-- Backfill profiles.username from registry when profile rows predate UsernameClaimed indexing order fix.
UPDATE profiles p
SET username = ur.username, updated_at = NOW()
FROM username_registry ur
WHERE p.profile_id = ur.profile_id
  AND (p.username IS NULL OR p.username = '');
