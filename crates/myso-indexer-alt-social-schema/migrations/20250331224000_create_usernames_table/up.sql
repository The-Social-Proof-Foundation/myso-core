CREATE TABLE IF NOT EXISTS username_registry (
    username TEXT PRIMARY KEY,
    profile_id TEXT NOT NULL UNIQUE,
    transaction_id TEXT NOT NULL
);

-- Backfill profiles.username from registry when profile rows predate UsernameClaimed indexing order fix.
UPDATE profiles p
SET username = ur.username, updated_at = NOW()
FROM username_registry ur
WHERE p.profile_id = ur.profile_id
  AND (p.username IS NULL OR p.username = '');
