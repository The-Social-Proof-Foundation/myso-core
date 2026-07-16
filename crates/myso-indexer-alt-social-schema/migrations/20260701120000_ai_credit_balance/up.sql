ALTER TABLE profiles ADD COLUMN IF NOT EXISTS ai_credit_balance_id TEXT;
CREATE INDEX IF NOT EXISTS idx_profiles_ai_credit_balance_id
    ON profiles (ai_credit_balance_id);

CREATE TABLE IF NOT EXISTS ai_credit_balances (
    balance_id TEXT NOT NULL PRIMARY KEY,
    memory_account_id TEXT NOT NULL REFERENCES memory_accounts(account_id),
    principal_owner TEXT NOT NULL,
    profile_id TEXT NOT NULL,
    balance_mist BIGINT NOT NULL DEFAULT 0,
    spent_total_mist BIGINT NOT NULL DEFAULT 0,
    daily_cap_mist BIGINT,
    monthly_cap_mist BIGINT,
    spent_day_mist BIGINT NOT NULL DEFAULT 0,
    spent_month_mist BIGINT NOT NULL DEFAULT 0,
    settlement_nonce BIGINT NOT NULL DEFAULT 0,
    active BOOLEAN NOT NULL DEFAULT TRUE,
    contract_version BIGINT NOT NULL DEFAULT 0,
    updated_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ai_credit_balances_principal_owner
    ON ai_credit_balances (principal_owner);
CREATE INDEX IF NOT EXISTS idx_ai_credit_balances_profile_id
    ON ai_credit_balances (profile_id);
CREATE INDEX IF NOT EXISTS idx_ai_credit_balances_memory_account
    ON ai_credit_balances (memory_account_id);

CREATE TABLE IF NOT EXISTS ai_credit_agent_budgets (
    balance_id TEXT NOT NULL REFERENCES ai_credit_balances(balance_id),
    agent_object_id TEXT NOT NULL REFERENCES sub_agents(agent_object_id),
    budget_mist BIGINT,
    spent_mist BIGINT NOT NULL DEFAULT 0,
    daily_cap_mist BIGINT,
    monthly_cap_mist BIGINT,
    require_approval_above_mist BIGINT,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    updated_at_ms BIGINT NOT NULL,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (balance_id, agent_object_id)
);

CREATE TABLE IF NOT EXISTS ai_credit_events (
    id SERIAL NOT NULL,
    event_type TEXT NOT NULL,
    balance_id TEXT,
    memory_account_id TEXT,
    principal_owner TEXT,
    profile_id TEXT,
    agent_object_id TEXT,
    amount_mist BIGINT,
    new_balance_mist BIGINT,
    credits BIGINT,
    receipt_id TEXT,
    usage_kind SMALLINT,
    settlement_nonce BIGINT,
    remaining_mist BIGINT,
    credits_remaining BIGINT,
    daily_cap_mist BIGINT,
    monthly_cap_mist BIGINT,
    budget_mist BIGINT,
    require_approval_above_mist BIGINT,
    event_id TEXT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id, time)
);

SELECT create_hypertable('ai_credit_events', 'time', if_not_exists => TRUE);

CREATE INDEX IF NOT EXISTS idx_ai_credit_events_balance_time
    ON ai_credit_events (balance_id, time DESC);
CREATE INDEX IF NOT EXISTS idx_ai_credit_events_type_time
    ON ai_credit_events (event_type, time DESC);

CREATE TABLE IF NOT EXISTS ai_credit_usage_lines (
    id BIGSERIAL PRIMARY KEY,
    receipt_id TEXT NOT NULL UNIQUE,
    balance_id TEXT NOT NULL,
    agent_object_id TEXT NOT NULL,
    usage_kind SMALLINT NOT NULL,
    amount_mist BIGINT NOT NULL,
    model_id TEXT,
    tool_id TEXT,
    metadata JSONB,
    settled BOOLEAN NOT NULL DEFAULT FALSE,
    settlement_tx TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_ai_credit_usage_lines_balance
    ON ai_credit_usage_lines (balance_id, created_at DESC);
CREATE INDEX IF NOT EXISTS idx_ai_credit_usage_lines_unsettled
    ON ai_credit_usage_lines (balance_id, settled, created_at DESC)
    WHERE settled = FALSE;

CREATE TABLE IF NOT EXISTS ai_credit_config (
    id SERIAL NOT NULL,
    updated_by TEXT NOT NULL DEFAULT '',
    oracle_pubkey_hex TEXT NOT NULL,
    treasury_address TEXT NOT NULL,
    min_deposit_mist BIGINT NOT NULL,
    max_single_settlement_mist BIGINT NOT NULL,
    receipt_ttl_ms BIGINT NOT NULL,
    catalog_version TEXT,
    version BIGINT NOT NULL DEFAULT 0,
    updated_at BIGINT NOT NULL,
    transaction_id TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

UPDATE ai_credit_config SET time = to_timestamp(updated_at / 1000) WHERE time = NOW();
CREATE OR REPLACE FUNCTION update_ai_credit_config_time()
RETURNS TRIGGER AS $$
BEGIN
    NEW.time = to_timestamp(NEW.updated_at / 1000);
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
DROP TRIGGER IF EXISTS set_ai_credit_config_time ON ai_credit_config;
CREATE TRIGGER set_ai_credit_config_time
BEFORE INSERT ON ai_credit_config
FOR EACH ROW
EXECUTE FUNCTION update_ai_credit_config_time();
SELECT create_hypertable('ai_credit_config', 'time', if_not_exists => TRUE,
                          create_default_indexes => FALSE,
                          chunk_time_interval => INTERVAL '1 month');
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM pg_constraint WHERE conname = 'ai_credit_config_pkey'
    ) THEN
        ALTER TABLE ai_credit_config ADD PRIMARY KEY (id, time);
    END IF;
END $$;
CREATE INDEX IF NOT EXISTS idx_ai_credit_config_time ON ai_credit_config(time DESC);
CREATE INDEX IF NOT EXISTS idx_ai_credit_config_transaction_id ON ai_credit_config(transaction_id);
COMMENT ON TABLE ai_credit_config IS 'Append-only AI credit configuration history. Latest row is current config.';

ALTER TABLE profiles ADD CONSTRAINT fk_profiles_ai_credit_balance
    FOREIGN KEY (ai_credit_balance_id) REFERENCES ai_credit_balances(balance_id);
