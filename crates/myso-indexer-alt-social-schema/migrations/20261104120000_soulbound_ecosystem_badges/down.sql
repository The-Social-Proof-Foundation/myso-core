DROP INDEX IF EXISTS poc_username_beneficiaries_linked_wallet;
ALTER TABLE poc_username_beneficiaries
  DROP COLUMN IF EXISTS reserved_for,
  DROP COLUMN IF EXISTS linked_wallet;

DROP INDEX IF EXISTS profile_badges_wallet_ecosystem_active;
DROP INDEX IF EXISTS profile_badges_object_id;
DELETE FROM profile_badges WHERE badge_kind = 'ecosystem';
ALTER TABLE profile_badges DROP CONSTRAINT IF EXISTS profile_badges_kind_identity;
ALTER TABLE profile_badges ADD CONSTRAINT profile_badges_kind_identity CHECK (
  (badge_kind = 'owned' AND profile_id IS NOT NULL AND wallet_address IS NULL)
  OR
  (badge_kind = 'platform' AND wallet_address IS NOT NULL AND profile_id IS NULL)
);
ALTER TABLE profile_badges ALTER COLUMN badge_kind SET DEFAULT 'owned';
ALTER TABLE profile_badges DROP COLUMN IF EXISTS badge_subject;
ALTER TABLE profile_badges DROP COLUMN IF EXISTS badge_object_id;
