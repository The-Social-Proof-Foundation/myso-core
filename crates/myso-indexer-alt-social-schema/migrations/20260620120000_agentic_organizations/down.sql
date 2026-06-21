DROP INDEX IF EXISTS idx_unified_revenue_organization;
ALTER TABLE unified_revenue DROP COLUMN IF EXISTS organization_id;

DROP INDEX IF EXISTS idx_mydata_purchases_organization;
ALTER TABLE mydata_purchases DROP COLUMN IF EXISTS organization_id;

DROP INDEX IF EXISTS idx_spt_reservations_organization;
ALTER TABLE spt_reservations DROP COLUMN IF EXISTS organization_id;

DROP INDEX IF EXISTS idx_spt_transactions_organization;
ALTER TABLE spt_transactions DROP COLUMN IF EXISTS organization_id;

DROP INDEX IF EXISTS idx_spot_bets_organization;
ALTER TABLE spot_bets DROP COLUMN IF EXISTS organization_id;

DROP INDEX IF EXISTS idx_reposts_organization;
ALTER TABLE reposts DROP COLUMN IF EXISTS organization_id;

DROP INDEX IF EXISTS idx_reactions_organization;
ALTER TABLE reactions DROP COLUMN IF EXISTS organization_id;

DROP INDEX IF EXISTS idx_comments_organization;
ALTER TABLE comments DROP COLUMN IF EXISTS organization_id;

DROP INDEX IF EXISTS idx_posts_organization_time;
ALTER TABLE posts DROP COLUMN IF EXISTS organization_id;

DROP INDEX IF EXISTS idx_sub_agent_events_organization;
ALTER TABLE sub_agent_events DROP COLUMN IF EXISTS organization_id;

DROP INDEX IF EXISTS idx_sub_agents_organization;
ALTER TABLE sub_agents DROP COLUMN IF EXISTS organization_id;

DROP TABLE IF EXISTS sub_agent_organization_counterparties;
DROP TABLE IF EXISTS sub_agent_organization_stats_daily;
DROP TABLE IF EXISTS sub_agent_organization_stats;
DROP TABLE IF EXISTS sub_agent_organization_events;
DROP TABLE IF EXISTS sub_agent_organizations;
