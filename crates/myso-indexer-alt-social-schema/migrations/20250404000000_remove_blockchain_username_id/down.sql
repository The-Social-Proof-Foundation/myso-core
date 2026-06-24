-- Restore column on legacy `usernames` table when present.
ALTER TABLE IF EXISTS usernames ADD COLUMN IF NOT EXISTS blockchain_username_id TEXT;
