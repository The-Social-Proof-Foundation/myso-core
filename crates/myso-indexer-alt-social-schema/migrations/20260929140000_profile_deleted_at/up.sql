ALTER TABLE profiles
  ADD COLUMN IF NOT EXISTS deleted_at TIMESTAMP;

DROP INDEX IF EXISTS idx_profiles_username;

CREATE UNIQUE INDEX IF NOT EXISTS idx_profiles_username
  ON profiles (username)
  WHERE deleted_at IS NULL;
