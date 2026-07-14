-- Keep post subscription state aligned with the on-chain PostAccess enum:
-- profile subscriptions come from subscription.move, while one-time prices stay on MyData.

COMMENT ON COLUMN mydata_config.marketplace_enabled IS
    'Whether buyers may start new broad-pool/snapshot MyData marketplace rounds; direct MyData listings and purchases are always available';

COMMENT ON COLUMN posts.subscription_price IS
    'Minimum active profile-subscription plan price for PROFILE_SUBSCRIPTION posts; never a MyData one-time or recurring price';

UPDATE posts
SET requires_subscription = CASE
    WHEN post_access_kind IN ('2', 'profile_sub', 'profile_subscription')
    THEN TRUE
    ELSE FALSE
END;

-- Remove legacy values copied from linked MyData one-time/recurring listings.
UPDATE posts SET subscription_price = NULL;

UPDATE posts AS p
SET subscription_price = (
    SELECT MIN(plan.price)
    FROM profile_subscription_plans AS plan
    WHERE plan.service_id = p.subscription_service_id
      AND plan.active = TRUE
      AND COALESCE(plan.tier_level, 0) >= COALESCE(p.subscription_min_tier_level, 0)
      AND (plan.platform_id IS NULL OR plan.platform_id = p.platform_id)
)
WHERE p.post_access_kind IN ('2', 'profile_sub', 'profile_subscription');
