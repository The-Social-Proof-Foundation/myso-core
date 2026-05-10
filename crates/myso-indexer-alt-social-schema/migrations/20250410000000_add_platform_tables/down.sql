-- Drop platform events table
DROP TABLE IF EXISTS platform_events;

-- Drop blocked profiles table
DROP TABLE IF EXISTS platform_blocked_profiles;

DROP INDEX IF EXISTS idx_platform_moderator_permissions_platform_moderator;
DROP INDEX IF EXISTS idx_platform_moderator_permissions_platform_id;
DROP TABLE IF EXISTS platform_moderator_permissions;

ALTER TABLE platform_moderators DROP COLUMN IF EXISTS updated_at;

-- Drop moderators table
DROP TABLE IF EXISTS platform_moderators;

-- Drop platforms table
DROP TABLE IF EXISTS platforms;