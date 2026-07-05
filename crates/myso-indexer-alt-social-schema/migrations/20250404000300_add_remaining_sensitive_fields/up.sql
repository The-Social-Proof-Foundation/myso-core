-- Ensure on-chain profile fields exist on profiles (website, birthdate, location)
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                  WHERE table_name='profiles' AND column_name='birthdate') THEN
        ALTER TABLE profiles ADD COLUMN birthdate TEXT;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                  WHERE table_name='profiles' AND column_name='location') THEN
        ALTER TABLE profiles ADD COLUMN location TEXT;
    END IF;

    -- Handle website field - make sure we have just one website field
    IF EXISTS (SELECT 1 FROM information_schema.columns
               WHERE table_name='profiles' AND column_name='website_url')
       AND NOT EXISTS (SELECT 1 FROM information_schema.columns
               WHERE table_name='profiles' AND column_name='website') THEN
        ALTER TABLE profiles RENAME COLUMN website_url TO website;
    END IF;

    IF EXISTS (SELECT 1 FROM information_schema.columns
              WHERE table_name='profiles' AND column_name='website_url')
       AND EXISTS (SELECT 1 FROM information_schema.columns
              WHERE table_name='profiles' AND column_name='website') THEN
        UPDATE profiles SET website = website_url WHERE website IS NULL AND website_url IS NOT NULL;
        ALTER TABLE profiles DROP COLUMN website_url;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                  WHERE table_name='profiles' AND column_name='website')
       AND NOT EXISTS (SELECT 1 FROM information_schema.columns
                  WHERE table_name='profiles' AND column_name='website_url') THEN
        ALTER TABLE profiles ADD COLUMN website TEXT;
    END IF;

    -- Social platform usernames (indexed separately from on-chain profile fields)
    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                  WHERE table_name='profiles' AND column_name='x_username') THEN
        ALTER TABLE profiles ADD COLUMN x_username TEXT;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                  WHERE table_name='profiles' AND column_name='mastodon_username') THEN
        ALTER TABLE profiles ADD COLUMN mastodon_username TEXT;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                  WHERE table_name='profiles' AND column_name='facebook_username') THEN
        ALTER TABLE profiles ADD COLUMN facebook_username TEXT;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                  WHERE table_name='profiles' AND column_name='reddit_username') THEN
        ALTER TABLE profiles ADD COLUMN reddit_username TEXT;
    END IF;

    IF NOT EXISTS (SELECT 1 FROM information_schema.columns
                  WHERE table_name='profiles' AND column_name='github_username') THEN
        ALTER TABLE profiles ADD COLUMN github_username TEXT;
    END IF;
END $$;
