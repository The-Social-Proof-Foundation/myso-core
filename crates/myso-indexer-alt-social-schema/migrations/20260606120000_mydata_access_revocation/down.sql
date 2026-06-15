DROP FUNCTION IF EXISTS user_has_access(TEXT, TEXT, BIGINT) CASCADE;

DROP INDEX IF EXISTS idx_mydata_subscriptions_active;
DROP INDEX IF EXISTS idx_mydata_purchases_active;

ALTER TABLE mydata_subscriptions
    DROP COLUMN IF EXISTS revoked_by,
    DROP COLUMN IF EXISTS revoked_at,
    DROP COLUMN IF EXISTS revoked;

ALTER TABLE mydata_purchases
    DROP COLUMN IF EXISTS revoked_by,
    DROP COLUMN IF EXISTS revoked_at,
    DROP COLUMN IF EXISTS revoked;
