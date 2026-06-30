ALTER TABLE profiles DROP CONSTRAINT IF EXISTS fk_profiles_ai_credit_balance;
ALTER TABLE profiles DROP COLUMN IF EXISTS ai_credit_balance_id;
DROP INDEX IF EXISTS idx_profiles_ai_credit_balance_id;

DROP TABLE IF EXISTS ai_credit_usage_lines;
DROP TABLE IF EXISTS ai_credit_events;
DROP TABLE IF EXISTS ai_credit_agent_budgets;
DROP TABLE IF EXISTS ai_credit_config;
DROP TABLE IF EXISTS ai_credit_balances;
