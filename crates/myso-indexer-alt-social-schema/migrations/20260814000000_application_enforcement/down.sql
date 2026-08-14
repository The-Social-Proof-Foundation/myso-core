DROP INDEX IF EXISTS idx_post_usage_decisions_post;
DROP TABLE IF EXISTS post_usage_decision_events;
ALTER TABLE posts DROP COLUMN IF EXISTS usage_denials;
ALTER TABLE posts DROP COLUMN IF EXISTS usage_decisions;
ALTER TABLE posts DROP COLUMN IF EXISTS embedded_bindings;
