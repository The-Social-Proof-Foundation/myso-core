DROP INDEX IF EXISTS idx_platform_treasury_coin_balances_platform;
DROP INDEX IF EXISTS idx_profile_subscription_plans_coin_type;
DROP INDEX IF EXISTS idx_subscription_revenue_coin_type;

ALTER TABLE platform_treasury_withdrawals DROP COLUMN IF EXISTS coin_type;

DROP TABLE IF EXISTS platform_treasury_coin_balances;

ALTER TABLE subscription_revenue DROP COLUMN IF EXISTS coin_type;
ALTER TABLE profile_subscriptions DROP COLUMN IF EXISTS coin_type;
ALTER TABLE profile_subscription_plans DROP COLUMN IF EXISTS coin_type;
