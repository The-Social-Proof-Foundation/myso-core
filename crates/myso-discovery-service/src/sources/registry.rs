// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use super::DiscoverySource;

pub struct SourceRegistry {
    adapters: Vec<Arc<dyn DiscoverySource>>,
}

impl SourceRegistry {
    pub fn new(adapters: Vec<Arc<dyn DiscoverySource>>) -> Self {
        Self { adapters }
    }

    pub fn all(&self) -> &[Arc<dyn DiscoverySource>] {
        &self.adapters
    }

    pub fn get(&self, id: &str) -> Option<Arc<dyn DiscoverySource>> {
        self.adapters.iter().find(|a| a.id() == id).cloned()
    }
}

pub fn build_default_registry() -> SourceRegistry {
    use super::adapters::{
        manual_curated::ManualCuratedAdapter, stub::StubAdapter,
    };
    use super::types::DiscoveryDomain;

    let adapters: Vec<Arc<dyn DiscoverySource>> = vec![
        Arc::new(ManualCuratedAdapter),
        Arc::new(StubAdapter::new("spotify", DiscoveryDomain::Creative)),
        Arc::new(StubAdapter::new("youtube", DiscoveryDomain::Creative)),
        Arc::new(StubAdapter::new("musicbrainz", DiscoveryDomain::Creative)),
        Arc::new(StubAdapter::new("instagram_metadata", DiscoveryDomain::Creative)),
        Arc::new(StubAdapter::new("rss", DiscoveryDomain::Factual)),
        Arc::new(StubAdapter::new("sec_edgar", DiscoveryDomain::Factual)),
        Arc::new(StubAdapter::new("noaa", DiscoveryDomain::Factual)),
        Arc::new(StubAdapter::new("fec", DiscoveryDomain::Factual)),
        Arc::new(StubAdapter::new("github_releases", DiscoveryDomain::Factual)),
        Arc::new(StubAdapter::new("http_official", DiscoveryDomain::Factual)),
    ];
    SourceRegistry::new(adapters)
}
