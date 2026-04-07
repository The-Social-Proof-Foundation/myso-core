-- Restore columns removed in up.sql (for `diesel migration revert` / local rollback).
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS instagram_username TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS facebook_username TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS github_username TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS linkedin_username TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS reddit_username TEXT;
ALTER TABLE profiles ADD COLUMN IF NOT EXISTS twitch_username TEXT;
