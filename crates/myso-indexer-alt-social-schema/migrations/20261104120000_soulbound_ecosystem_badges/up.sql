-- Ecosystem badges are soulbound objects minted into wallets (platform::EcosystemBadge).
-- Profile-stored ('owned') badges no longer exist on-chain.
DELETE FROM profile_badges WHERE badge_kind = 'owned';

ALTER TABLE profile_badges
  ADD COLUMN IF NOT EXISTS badge_object_id TEXT NULL,
  -- What the badge attests to (canonical X handle for Verified X).
  ADD COLUMN IF NOT EXISTS badge_subject TEXT NULL;

ALTER TABLE profile_badges ALTER COLUMN badge_kind SET DEFAULT 'ecosystem';

ALTER TABLE profile_badges DROP CONSTRAINT IF EXISTS profile_badges_kind_identity;
ALTER TABLE profile_badges ADD CONSTRAINT profile_badges_kind_identity CHECK (
  (badge_kind = 'ecosystem' AND wallet_address IS NOT NULL AND profile_id IS NULL AND badge_object_id IS NOT NULL)
  OR
  (badge_kind = 'platform' AND wallet_address IS NOT NULL AND profile_id IS NULL)
);

CREATE INDEX IF NOT EXISTS profile_badges_object_id
  ON profile_badges (badge_object_id)
  WHERE badge_object_id IS NOT NULL;

CREATE INDEX IF NOT EXISTS profile_badges_wallet_ecosystem_active
  ON profile_badges (wallet_address, badge_id)
  WHERE revoked = false AND badge_kind = 'ecosystem';

-- PoC username beneficiaries: the creator's recipient wallet (never the oracle) and the
-- wallet a username is reserved for when the claim found an existing profile.
ALTER TABLE poc_username_beneficiaries
  ADD COLUMN IF NOT EXISTS linked_wallet TEXT NULL,
  ADD COLUMN IF NOT EXISTS reserved_for TEXT NULL;

CREATE INDEX IF NOT EXISTS poc_username_beneficiaries_linked_wallet
  ON poc_username_beneficiaries (LOWER(linked_wallet))
  WHERE linked_wallet IS NOT NULL;
