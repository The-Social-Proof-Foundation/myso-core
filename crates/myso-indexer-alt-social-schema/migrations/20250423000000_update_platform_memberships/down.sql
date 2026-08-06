-- Revert: drop left_at and restore role (prior hard-delete membership model).

DROP INDEX IF EXISTS idx_platform_memberships_left_at;

ALTER TABLE platform_memberships DROP COLUMN IF EXISTS left_at;

ALTER TABLE platform_memberships
    ADD COLUMN IF NOT EXISTS role TEXT NOT NULL DEFAULT 'member';

COMMENT ON TABLE platform_memberships IS
    'Records of profiles joined to platforms. Records are deleted when a user leaves a platform.';
