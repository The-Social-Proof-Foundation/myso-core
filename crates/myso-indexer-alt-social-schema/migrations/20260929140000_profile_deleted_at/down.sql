DROP INDEX IF EXISTS idx_profiles_username;

CREATE UNIQUE INDEX IF NOT EXISTS idx_profiles_username ON profiles (username);

ALTER TABLE profiles DROP COLUMN IF EXISTS deleted_at;
