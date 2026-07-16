// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::events::types::ProviderHealth;
use crate::knowledge::types::{
    DiscoveredKnowledge, DiscoveredRelationship, KnowledgeProvider, ProviderContext,
};

pub struct RelationshipSeedProvider;

#[async_trait]
impl KnowledgeProvider for RelationshipSeedProvider {
    fn id(&self) -> &str {
        "relationship_seed"
    }

    async fn discover(&self, ctx: &ProviderContext) -> anyhow::Result<DiscoveredKnowledge> {
        let path = ctx
            .config
            .get("relationships_file")
            .and_then(|v| v.as_str())
            .unwrap_or("crates/myso-spot-oracle/config/relationship_seed.localnet.yaml");
        let raw = std::fs::read_to_string(path)?;
        let parsed: RelationshipSeedFile = serde_yaml::from_str(&raw)?;
        let relationships = parsed
            .relationships
            .into_iter()
            .map(|r| DiscoveredRelationship {
                subject_id: r.subject_id,
                object_id: r.object_id,
                rel_type: r.rel_type,
                valid_from: None,
                valid_to: None,
                provenance: serde_json::json!({
                    "provider": ctx.provider_key,
                    "source": "relationship_seed",
                }),
            })
            .collect();
        Ok(DiscoveredKnowledge {
            relationships,
            ..Default::default()
        })
    }

    async fn health(&self) -> ProviderHealth {
        ProviderHealth::ok("relationship_seed ready")
    }
}

#[derive(Debug, serde::Deserialize)]
struct RelationshipSeedFile {
    relationships: Vec<RelationshipSeedRow>,
}

#[derive(Debug, serde::Deserialize)]
struct RelationshipSeedRow {
    subject_id: String,
    object_id: String,
    rel_type: String,
}
