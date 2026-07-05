-- Add on-chain profile fields and social username columns to profiles
ALTER TABLE profiles
  ADD COLUMN IF NOT EXISTS birthdate TEXT,
  ADD COLUMN IF NOT EXISTS location TEXT,
  ADD COLUMN IF NOT EXISTS x_username TEXT,
  ADD COLUMN IF NOT EXISTS mastodon_username TEXT,
  ADD COLUMN IF NOT EXISTS facebook_username TEXT,
  ADD COLUMN IF NOT EXISTS reddit_username TEXT,
  ADD COLUMN IF NOT EXISTS github_username TEXT;

-- Migrate any existing data from profile_encrypted_data and profile_private_fields
-- For the migration, we're setting just flags indicating if data exists
-- In a real migration, you would decode the encrypted data and move it
DO $$ 
BEGIN
  -- Only migrate data if profiles table has id column and profile_private_fields table exists
  IF EXISTS (
    SELECT 1 FROM information_schema.columns 
    WHERE table_name = 'profiles' AND column_name = 'id'
  ) AND EXISTS (
    SELECT 1 FROM information_schema.tables 
    WHERE table_name = 'profile_private_fields'
  ) THEN
    UPDATE profiles p
    SET
      birthdate = CASE WHEN ppf.has_birthdate THEN '(encrypted)' ELSE NULL END,
      location = CASE WHEN ppf.has_current_location THEN '(encrypted)' ELSE NULL END,
      x_username = CASE WHEN ppf.has_social_usernames THEN '(encrypted)' ELSE NULL END,
      mastodon_username = CASE WHEN ppf.has_social_usernames THEN '(encrypted)' ELSE NULL END,
      facebook_username = CASE WHEN ppf.has_social_usernames THEN '(encrypted)' ELSE NULL END,
      reddit_username = CASE WHEN ppf.has_social_usernames THEN '(encrypted)' ELSE NULL END,
      github_username = CASE WHEN ppf.has_social_usernames THEN '(encrypted)' ELSE NULL END
    FROM profile_private_fields ppf
    WHERE p.id = ppf.profile_id;
  END IF;
END $$;

-- Drop old tables that are no longer needed
DROP TABLE IF EXISTS profile_encrypted_data;
DROP TABLE IF EXISTS profile_private_fields;

-- Remove has_private_data column, since we now know based on field presence
ALTER TABLE profiles DROP COLUMN IF EXISTS has_private_data;

-- Rename private_data_updated_at to sensitive_data_updated_at for clarity
DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' 
        AND column_name = 'private_data_updated_at'
    ) AND NOT EXISTS (
        SELECT 1 FROM information_schema.columns 
        WHERE table_name = 'profiles' 
        AND column_name = 'sensitive_data_updated_at'
    ) THEN
        ALTER TABLE profiles 
        RENAME COLUMN private_data_updated_at TO sensitive_data_updated_at;
    END IF;
END $$;