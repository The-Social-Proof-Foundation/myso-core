// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_trait::async_trait;

use crate::events::types::ProviderHealth;
use crate::knowledge::types::{
    DiscoveredEntity, DiscoveredKnowledge, KnowledgeProvider, ProviderContext,
};

pub struct EntitySeedProvider;

#[async_trait]
impl KnowledgeProvider for EntitySeedProvider {
    fn id(&self) -> &str {
        "entity_seed"
    }

    async fn discover(&self, ctx: &ProviderContext) -> anyhow::Result<DiscoveredKnowledge> {
        let path = ctx
            .config
            .get("entities_file")
            .and_then(|v| v.as_str())
            .unwrap_or("crates/myso-spot-oracle/config/entity_seed.localnet.yaml");
        let raw = std::fs::read_to_string(path)?;
        let parsed: EntitySeedFile = serde_yaml::from_str(&raw)?;
        let entities = parsed
            .entities
            .into_iter()
            .map(|e| DiscoveredEntity {
                id: e.id,
                kind: e.kind,
                name: e.name,
                aliases: e.aliases,
                domain: e.domain,
                external_refs: e.external_refs.unwrap_or_default(),
                provenance: serde_json::json!({
                    "provider": ctx.provider_key,
                    "source": "entity_seed",
                }),
            })
            .collect();
        Ok(DiscoveredKnowledge {
            entities,
            ..Default::default()
        })
    }

    async fn health(&self) -> ProviderHealth {
        ProviderHealth::ok("entity_seed ready")
    }
}

#[derive(Debug, serde::Deserialize)]
struct EntitySeedFile {
    entities: Vec<EntitySeedRow>,
}

#[derive(Debug, serde::Deserialize)]
struct EntitySeedRow {
    id: String,
    kind: String,
    name: String,
    #[serde(default)]
    aliases: Vec<String>,
    #[serde(default = "default_domain")]
    domain: String,
    #[serde(default)]
    external_refs: Option<serde_json::Value>,
}

fn default_domain() -> String {
    "unknown".to_string()
}
