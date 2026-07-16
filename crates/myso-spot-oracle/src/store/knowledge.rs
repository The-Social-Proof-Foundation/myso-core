// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use sqlx::PgPool;

use crate::knowledge::types::DiscoveredKnowledge;

pub async fn upsert_discovered_knowledge(
    pool: &PgPool,
    _provider_key: &str,
    knowledge: &DiscoveredKnowledge,
) -> anyhow::Result<usize> {
    let mut count = 0usize;

    for ent in &knowledge.entities {
        sqlx::query(
            r#"
            INSERT INTO knowledge_entities (id, kind, name, aliases, domain, external_refs, provenance)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                kind = EXCLUDED.kind,
                name = EXCLUDED.name,
                aliases = EXCLUDED.aliases,
                domain = EXCLUDED.domain,
                external_refs = EXCLUDED.external_refs,
                provenance = EXCLUDED.provenance,
                updated_at = NOW()
            "#,
        )
        .bind(&ent.id)
        .bind(&ent.kind)
        .bind(&ent.name)
        .bind(&ent.aliases)
        .bind(&ent.domain)
        .bind(&ent.external_refs)
        .bind(&ent.provenance)
        .execute(pool)
        .await?;
        count += 1;
    }

    for comp in &knowledge.competitions {
        sqlx::query(
            r#"
            INSERT INTO knowledge_competitions (id, kind, label, domain, recurrence_rule, provenance)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE SET
                kind = EXCLUDED.kind,
                label = EXCLUDED.label,
                domain = EXCLUDED.domain,
                recurrence_rule = EXCLUDED.recurrence_rule,
                provenance = EXCLUDED.provenance,
                updated_at = NOW()
            "#,
        )
        .bind(&comp.id)
        .bind(&comp.kind)
        .bind(&comp.label)
        .bind(&comp.domain)
        .bind(&comp.recurrence_rule)
        .bind(&comp.provenance)
        .execute(pool)
        .await?;
        count += 1;
    }

    for ev in &knowledge.events {
        sqlx::query(
            r#"
            INSERT INTO knowledge_events (
                id, competition_id, label, domain, start_at, end_at, keywords, entities,
                feed_url, match_predicate, preferred_source_keys, priority, provenance
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            ON CONFLICT (id) DO UPDATE SET
                competition_id = EXCLUDED.competition_id,
                label = EXCLUDED.label,
                domain = EXCLUDED.domain,
                start_at = EXCLUDED.start_at,
                end_at = EXCLUDED.end_at,
                keywords = EXCLUDED.keywords,
                entities = EXCLUDED.entities,
                feed_url = EXCLUDED.feed_url,
                match_predicate = EXCLUDED.match_predicate,
                preferred_source_keys = EXCLUDED.preferred_source_keys,
                priority = EXCLUDED.priority,
                provenance = EXCLUDED.provenance,
                updated_at = NOW()
            "#,
        )
        .bind(&ev.id)
        .bind(&ev.competition_id)
        .bind(&ev.label)
        .bind(&ev.domain)
        .bind(ev.start_at)
        .bind(ev.end_at)
        .bind(&ev.keywords)
        .bind(serde_json::to_value(&ev.entities)?)
        .bind(&ev.feed_url)
        .bind(&ev.match_predicate)
        .bind(&ev.preferred_source_keys)
        .bind(ev.priority)
        .bind(&ev.provenance)
        .execute(pool)
        .await?;
        count += 1;
    }

    for metric in &knowledge.metrics {
        sqlx::query(
            r#"
            INSERT INTO knowledge_metrics (id, key, unit, domain, aggregation, schema, provenance)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            ON CONFLICT (id) DO UPDATE SET
                key = EXCLUDED.key,
                unit = EXCLUDED.unit,
                domain = EXCLUDED.domain,
                aggregation = EXCLUDED.aggregation,
                schema = EXCLUDED.schema,
                provenance = EXCLUDED.provenance,
                updated_at = NOW()
            "#,
        )
        .bind(&metric.id)
        .bind(&metric.key)
        .bind(&metric.unit)
        .bind(&metric.domain)
        .bind(&metric.aggregation)
        .bind(&metric.schema)
        .bind(&metric.provenance)
        .execute(pool)
        .await?;
        count += 1;
    }

    for obs in &knowledge.observations {
        sqlx::query(
            r#"
            INSERT INTO knowledge_observations (metric_id, entity_id, event_id, observed_at, value, provenance)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(&obs.metric_id)
        .bind(&obs.entity_id)
        .bind(&obs.event_id)
        .bind(obs.observed_at)
        .bind(&obs.value)
        .bind(&obs.provenance)
        .execute(pool)
        .await?;
        count += 1;
    }

    for rel in &knowledge.relationships {
        sqlx::query(
            r#"
            INSERT INTO knowledge_relationships (subject_id, object_id, rel_type, valid_from, valid_to, provenance)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
        )
        .bind(&rel.subject_id)
        .bind(&rel.object_id)
        .bind(&rel.rel_type)
        .bind(rel.valid_from)
        .bind(rel.valid_to)
        .bind(&rel.provenance)
        .execute(pool)
        .await?;
        count += 1;
    }

    Ok(count)
}
