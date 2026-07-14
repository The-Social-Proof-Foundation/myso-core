-- The data correction is intentionally not reversed: legacy MyData-derived post
-- subscription prices were not valid profile-subscription entitlements.

COMMENT ON COLUMN mydata_config.marketplace_enabled IS
    'Whether the MyData marketplace is enabled (default: false)';

COMMENT ON COLUMN posts.subscription_price IS
    'Indexed subscription price associated with the post';
