DROP INDEX IF EXISTS idx_spot_markets_graph_dedup;
DROP INDEX IF EXISTS idx_spot_markets_outcome_identity;

ALTER TABLE spot_markets
    DROP COLUMN IF EXISTS deadline_day,
    DROP COLUMN IF EXISTS outcome_identity_hash,
    DROP COLUMN IF EXISTS metric_ref,
    DROP COLUMN IF EXISTS event_ref,
    DROP COLUMN IF EXISTS competition_ref,
    DROP COLUMN IF EXISTS entity_ref;

DROP INDEX IF EXISTS idx_knowledge_relationships_subject;
DROP TABLE IF EXISTS knowledge_relationships;

DROP INDEX IF EXISTS idx_knowledge_observations_lookup;
DROP TABLE IF EXISTS knowledge_observations;

DROP TABLE IF EXISTS knowledge_metrics;

DROP INDEX IF EXISTS idx_knowledge_events_keywords;
DROP TABLE IF EXISTS knowledge_events;

DROP TABLE IF EXISTS knowledge_competitions;

DROP INDEX IF EXISTS idx_knowledge_entities_aliases;
DROP TABLE IF EXISTS knowledge_entities;
