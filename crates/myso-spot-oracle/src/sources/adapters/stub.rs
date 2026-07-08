// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Generic `TrustedSource` stub. V1 adapter ids are registered as live placeholders
//! (`disabled` health, `supports` = false, `resolve` = err) until the v1-adapters
//! phase replaces them with real HTTP/API impls. Scaffolded non-V1 adapters stay
//! stubs and are never enabled in V1 E2E.

use async_trait::async_trait;
use myso_discovery_service_core::sources::{DiscoveryDomain, SourceHealth, SourceMetadata};

use crate::resolver::ResolverDefinition;
use crate::sources::{SourceEvidence, TrustedSource};

pub struct StubTrustedSource {
    id: String,
    description: String,
    domain: DiscoveryDomain,
}

impl StubTrustedSource {
    /// V1 adapter id whose live impl lands in the v1-adapters phase.
    pub fn new_live_placeholder(id: &'static str) -> Self {
        Self {
            id: id.to_string(),
            description: format!("{id} live adapter (pending v1-adapters phase)"),
            domain: DiscoveryDomain::Factual,
        }
    }

    /// Non-V1 adapter id, scaffolded only.
    pub fn new_scaffolded(id: &'static str, domain: DiscoveryDomain) -> Self {
        Self {
            id: id.to_string(),
            description: format!("{id} scaffolded stub (not in V1 E2E)"),
            domain,
        }
    }
}

#[async_trait]
impl TrustedSource for StubTrustedSource {
    fn id(&self) -> &str {
        &self.id
    }

    fn domain(&self) -> DiscoveryDomain {
        self.domain
    }

    fn supports(&self, _def: &ResolverDefinition) -> bool {
        false
    }

    async fn resolve(&self, _def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        anyhow::bail!("trusted source '{}' not implemented (stub)", self.id)
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: false,
            message: format!("{} stub — disabled", self.id),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id.clone(),
            description: self.description.clone(),
            domain: self.domain,
        }
    }
}
