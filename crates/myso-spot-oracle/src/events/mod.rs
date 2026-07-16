// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Pluggable Event Provider framework — discovers future events into Postgres and
//! powers implicit deadline inference at review time.

pub mod calendar;
pub mod config;
pub mod providers;
pub mod registry;
pub mod sync;
pub mod types;

pub use config::{load_event_providers_config, EventProviderConfig};
pub use registry::EventRegistry;
pub use types::{
    DiscoveredEvent, EventCategory, EventEntity, EventResolverHints, ProviderContext,
    ProviderHealth,
};

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

/// Discovers and normalizes future events from external calendars.
#[async_trait]
pub trait EventProvider: Send + Sync {
    fn id(&self) -> &str;
    async fn discover(&self, ctx: &ProviderContext) -> anyhow::Result<Vec<DiscoveredEvent>>;
    async fn health(&self) -> ProviderHealth;
}

/// Holds `EventProvider` impls compiled into the binary, keyed by provider type.
#[derive(Clone, Default)]
pub struct EventProviderRegistry {
    by_type: HashMap<String, Arc<dyn EventProvider>>,
}

impl EventProviderRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, provider: Arc<dyn EventProvider>) {
        self.by_type.insert(provider.id().to_string(), provider);
    }

    pub fn get(&self, provider_type: &str) -> Option<Arc<dyn EventProvider>> {
        self.by_type.get(provider_type).cloned()
    }
}

/// Build the default registry from every `EventProvider` impl in the binary.
pub fn build_default_event_provider_registry() -> EventProviderRegistry {
    let mut reg = EventProviderRegistry::new();
    for provider in providers::all_default_providers() {
        reg.register(provider);
    }
    reg
}
