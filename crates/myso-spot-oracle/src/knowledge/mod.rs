// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Knowledge Graph + Registry — entities, competitions, events, metrics, observations, relationships.

pub mod config;
pub mod graph;
pub mod providers;
pub mod sync;
pub mod types;

pub use graph::KnowledgeGraph;
pub use types::{
    DiscoveredCompetition, DiscoveredEntity, DiscoveredEvent, DiscoveredKnowledge,
    DiscoveredMetric, DiscoveredObservation, DiscoveredRelationship, KnowledgeProvider,
    ProviderContext,
};

use std::collections::HashMap;
use std::sync::Arc;

/// Holds `KnowledgeProvider` impls compiled into the binary.
#[derive(Clone, Default)]
pub struct KnowledgeProviderRegistry {
    by_type: HashMap<String, Arc<dyn KnowledgeProvider>>,
}

impl KnowledgeProviderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, provider: Arc<dyn KnowledgeProvider>) {
        self.by_type.insert(provider.id().to_string(), provider);
    }

    pub fn get(&self, provider_type: &str) -> Option<Arc<dyn KnowledgeProvider>> {
        self.by_type.get(provider_type).cloned()
    }
}

pub fn build_default_knowledge_provider_registry() -> KnowledgeProviderRegistry {
    let mut reg = KnowledgeProviderRegistry::new();
    for provider in providers::all_default_providers() {
        reg.register(provider);
    }
    reg
}
