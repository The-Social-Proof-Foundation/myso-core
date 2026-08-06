-- Drop role; keep left_at for soft leave (active = left_at IS NULL OR joined_at > left_at).

DO $$
BEGIN
    IF EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'platform_memberships' AND column_name = 'role'
    ) THEN
        ALTER TABLE platform_memberships DROP COLUMN role;
    END IF;
END $$;

DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.columns
        WHERE table_name = 'platform_memberships' AND column_name = 'left_at'
    ) THEN
        ALTER TABLE platform_memberships ADD COLUMN left_at TIMESTAMP NULL;
    END IF;
END $$;

CREATE INDEX IF NOT EXISTS idx_platform_memberships_left_at ON platform_memberships (left_at);

COMMENT ON TABLE platform_memberships IS
    'Platform membership rows are retained. Active member: left_at IS NULL OR joined_at > left_at.';
