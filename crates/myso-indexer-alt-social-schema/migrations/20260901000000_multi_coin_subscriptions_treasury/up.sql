-- Multi-coin subscription plans and platform treasury holdings.

ALTER TABLE profile_subscription_plans
    ADD COLUMN IF NOT EXISTS coin_type TEXT NOT NULL DEFAULT '';

ALTER TABLE profile_subscriptions
    ADD COLUMN IF NOT EXISTS coin_type TEXT NOT NULL DEFAULT '';

ALTER TABLE subscription_revenue
    ADD COLUMN IF NOT EXISTS coin_type TEXT NOT NULL DEFAULT '';

CREATE TABLE IF NOT EXISTS platform_treasury_coin_balances (
    platform_id TEXT NOT NULL,
    coin_type TEXT NOT NULL,
    balance BIGINT NOT NULL DEFAULT 0,
    last_funded_at BIGINT,
    last_withdrawn_at BIGINT,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (platform_id, coin_type)
);

ALTER TABLE platform_treasury_withdrawals
    ADD COLUMN IF NOT EXISTS coin_type TEXT NOT NULL DEFAULT '';

CREATE INDEX IF NOT EXISTS idx_subscription_revenue_coin_type
    ON subscription_revenue (coin_type, time DESC);

CREATE INDEX IF NOT EXISTS idx_profile_subscription_plans_coin_type
    ON profile_subscription_plans (service_id, coin_type);

CREATE INDEX IF NOT EXISTS idx_platform_treasury_coin_balances_platform
    ON platform_treasury_coin_balances (platform_id);
