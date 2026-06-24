-- Legacy: drop blockchain_username_id from old `usernames` table when present.
-- Greenfield installs only create `username_registry`; this is a no-op in that case.
ALTER TABLE IF EXISTS usernames DROP COLUMN IF EXISTS blockchain_username_id;
