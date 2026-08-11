-- Repair profile schema drift in long-lived social indexer databases.
--
-- Fresh databases already receive these columns from the canonical profile
-- schema and dynamic ecosystem migration. Older testnet databases can still
-- have `current_location` and no `contract_version`, while the current Diesel
-- Profile model selects `location` and `contract_version` for every profile.

DO $$ BEGIN
  IF EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_schema = current_schema()
      AND table_name = 'profiles'
      AND column_name = 'current_location'
  ) AND NOT EXISTS (
    SELECT 1 FROM information_schema.columns
    WHERE table_schema = current_schema()
      AND table_name = 'profiles'
      AND column_name = 'location'
  ) THEN
    ALTER TABLE profiles RENAME COLUMN current_location TO location;
  END IF;
END $$;

ALTER TABLE profiles
  ADD COLUMN IF NOT EXISTS location TEXT,
  ADD COLUMN IF NOT EXISTS contract_version BIGINT NOT NULL DEFAULT 0;
