-- Align profiles with Move profile: keep x_username only for external social handle fields.
ALTER TABLE profiles
  DROP COLUMN IF EXISTS instagram_username,
  DROP COLUMN IF EXISTS facebook_username,
  DROP COLUMN IF EXISTS github_username,
  DROP COLUMN IF EXISTS linkedin_username,
  DROP COLUMN IF EXISTS reddit_username,
  DROP COLUMN IF EXISTS twitch_username;
