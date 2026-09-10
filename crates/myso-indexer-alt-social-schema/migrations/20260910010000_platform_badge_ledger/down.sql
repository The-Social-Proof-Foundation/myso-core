DROP TABLE IF EXISTS wallet_badge_selections;
DROP INDEX IF EXISTS profile_badges_wallet_active;
ALTER TABLE profile_badges DROP CONSTRAINT IF EXISTS profile_badges_kind_identity;
ALTER TABLE profile_badges DROP COLUMN IF EXISTS wallet_address;
ALTER TABLE profile_badges DROP COLUMN IF EXISTS badge_kind;
ALTER TABLE profile_badges DROP COLUMN IF EXISTS expires_at;
ALTER TABLE profile_badges ALTER COLUMN profile_id SET NOT NULL;
ALTER TABLE platforms DROP COLUMN IF EXISTS badge_ledger_id;
