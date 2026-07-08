// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::collections::HashMap;
use std::sync::Arc;

use crate::sources::adapters::{
    github_releases::GithubReleasesAdapter, http_official::HttpOfficialAdapter,
    manual_curated::ManualCuratedAdapter, rss::RssAdapter, stub::StubAdapter,
};
use crate::sources::{DiscoveryDomain, DiscoverySource, SourceConfig};

/// Holds `DiscoverySource` impls by adapter type. Renamed from `SourceRegistry` so
/// the name matches the responsibility (discovery crawl), distinct from spot-oracle's
/// `ResolverRegistry` which holds `TrustedSource` impls (settlement evidence).
#[derive(Clone, Default)]
pub struct DiscoveryRegistry {
    by_type: HashMap<String, Arc<dyn DiscoverySource>>,
}

impl DiscoveryRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, source: Arc<dyn DiscoverySource>) {
        // Key by adapter_type string (e.g. "rss", "github_releases") so a single
        // impl serves many `SourceConfig`s that share a type.
        self.by_type.insert(source.metadata().id.to_string(), source);
    }

    /// Look up an adapter by its id (which equals `adapter_type` for all impls).
    /// Matches the previous `SourceRegistry::get` signature.
    pub fn get(&self, id: &str) -> Option<Arc<dyn DiscoverySource>> {
        self.by_type.get(id).cloned()
    }

    pub fn lookup(&self, adapter_type: &str) -> Option<Arc<dyn DiscoverySource>> {
        self.get(adapter_type)
    }

    pub fn supports(&self, config: &SourceConfig) -> bool {
        self.by_type.contains_key(&config.adapter_type)
    }

    pub fn all(&self) -> Vec<Arc<dyn DiscoverySource>> {
        self.by_type.values().cloned().collect()
    }
}

/// Build the default registry of real `DiscoverySource` adapters. Creative sources
/// not in V1 scope remain stubbed and disabled unless explicitly enabled.
pub fn build_default_registry() -> DiscoveryRegistry {
    let mut reg = DiscoveryRegistry::new();
    // Real factual adapters.
    reg.register(Arc::new(RssAdapter::new()));
    reg.register(Arc::new(GithubReleasesAdapter::new()));
    reg.register(Arc::new(HttpOfficialAdapter::new()));
    // Test-only curated source (gated behind DISCOVERY_USE_MANUAL_CURATED=1 at use sites).
    reg.register(Arc::new(ManualCuratedAdapter));
    // Creative + out-of-scope factual stubs (disabled unless enabled in config).
    reg.register(Arc::new(StubAdapter::new("spotify", DiscoveryDomain::Creative)));
    reg.register(Arc::new(StubAdapter::new("youtube", DiscoveryDomain::Creative)));
    reg.register(Arc::new(StubAdapter::new("musicbrainz", DiscoveryDomain::Creative)));
    reg.register(Arc::new(StubAdapter::new("instagram", DiscoveryDomain::Creative)));
    reg.register(Arc::new(StubAdapter::new("sec_edgar", DiscoveryDomain::Factual)));
    reg.register(Arc::new(StubAdapter::new("noaa", DiscoveryDomain::Factual)));
    reg.register(Arc::new(StubAdapter::new("fec", DiscoveryDomain::Factual)));
    reg
}
