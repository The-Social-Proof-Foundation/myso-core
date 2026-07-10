// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Generic `TrustedSource` stub for scaffolded non-V1 adapters.

use async_trait::async_trait;

use crate::resolver::ResolverDefinition;
use crate::sources::source_config::{SourceDomain, SourceHealth, SourceMetadata};
use crate::sources::{SourceEvidence, TrustedSource};

pub struct StubTrustedSource {
    id: String,
    description: String,
    domain: SourceDomain,
}

impl StubTrustedSource {
    pub fn new_scaffolded(id: &'static str, domain: SourceDomain) -> Self {
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

    fn domain(&self) -> SourceDomain {
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
