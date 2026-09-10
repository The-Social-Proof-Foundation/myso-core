ALTER TABLE platforms
  ADD COLUMN IF NOT EXISTS badge_ledger_id TEXT NULL;

ALTER TABLE profile_badges
  ADD COLUMN IF NOT EXISTS wallet_address TEXT NULL,
  ADD COLUMN IF NOT EXISTS badge_kind TEXT NOT NULL DEFAULT 'owned',
  ADD COLUMN IF NOT EXISTS expires_at BIGINT NULL;

ALTER TABLE profile_badges ALTER COLUMN profile_id DROP NOT NULL;

ALTER TABLE profile_badges DROP CONSTRAINT IF EXISTS profile_badges_kind_identity;
ALTER TABLE profile_badges ADD CONSTRAINT profile_badges_kind_identity CHECK (
  (badge_kind = 'owned' AND profile_id IS NOT NULL AND wallet_address IS NULL)
  OR
  (badge_kind = 'platform' AND wallet_address IS NOT NULL AND profile_id IS NULL)
);

CREATE INDEX IF NOT EXISTS profile_badges_wallet_active
  ON profile_badges (wallet_address, badge_id)
  WHERE revoked = false AND badge_kind = 'platform';

CREATE TABLE IF NOT EXISTS wallet_badge_selections (
  wallet_address TEXT PRIMARY KEY,
  badge_id TEXT NULL,
  ecosystem_badge_id TEXT NULL,
  selected_at BIGINT NOT NULL,
  ecosystem_selected_at BIGINT NULL
);
