// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Wikipedia trusted source. Fetches a Wikipedia REST summary (or any provided Wikipedia URL) as
//! JSON evidence for factual/historical claim verification — the common case for past claims that
//! never had (and never will have) a prediction market, e.g. "Hitler won World War II".

use async_trait::async_trait;

use crate::resolver::{ResolverDefinition, ResolverKind, ResolverSpec};
use crate::sources::direct_fetch;
use crate::sources::source_config::{SourceDomain, SourceHealth, SourceMetadata};
use crate::sources::{SourceEvidence, TrustedSource};

const WIKI_SUMMARY_BASE: &str = "https://en.wikipedia.org/api/rest_v1/page/summary/";

pub struct WikipediaAdapter;

impl WikipediaAdapter {
    pub fn new() -> Self {
        Self
    }

    /// Build a Wikipedia REST summary URL from a subject when the claim has no explicit url.
    fn summary_url(subject: &str) -> String {
        let title = subject.trim().replace(' ', "_");
        format!("{WIKI_SUMMARY_BASE}{}", urlencoding_light(&title))
    }
}

impl Default for WikipediaAdapter {
    fn default() -> Self {
        Self::new()
    }
}

/// Minimal path-segment encoding (Wikipedia titles allow most chars; encode the risky few).
fn urlencoding_light(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            ' ' => out.push_str("%20"),
            '?' => out.push_str("%3F"),
            '#' => out.push_str("%23"),
            '&' => out.push_str("%26"),
            _ => out.push(c),
        }
    }
    out
}

#[async_trait]
impl TrustedSource for WikipediaAdapter {
    fn id(&self) -> &str {
        "wikipedia"
    }

    fn domain(&self) -> SourceDomain {
        SourceDomain::Factual
    }

    fn supports(&self, def: &ResolverDefinition) -> bool {
        matches!(
            def.resolver_kind,
            ResolverKind::CustomHttp | ResolverKind::EventOccurrence
        ) && matches!(
            &def.spec,
            ResolverSpec::CustomHttp { source_id, .. }
                | ResolverSpec::EventOccurrence { source_id, .. }
                if source_id == "wikipedia"
        )
    }

    async fn resolve(&self, def: &ResolverDefinition) -> anyhow::Result<SourceEvidence> {
        match &def.spec {
            ResolverSpec::CustomHttp { url, .. } => {
                let target = if url.is_empty() {
                    anyhow::bail!("wikipedia: missing url");
                } else if url.starts_with("http") {
                    url.clone()
                } else {
                    Self::summary_url(url)
                };
                direct_fetch::fetch_http_json(self.id(), &target).await
            }
            ResolverSpec::EventOccurrence {
                feed_url,
                match_predicate,
                ..
            } => {
                let target = if feed_url.starts_with("http") {
                    feed_url.clone()
                } else if !match_predicate.is_empty() {
                    Self::summary_url(match_predicate)
                } else {
                    anyhow::bail!("wikipedia: missing feed_url/match_predicate");
                };
                direct_fetch::fetch_http_json(self.id(), &target).await
            }
            _ => anyhow::bail!("wikipedia: unsupported resolver spec"),
        }
    }

    async fn health(&self) -> SourceHealth {
        SourceHealth {
            healthy: true,
            message: "wikipedia REST summary".to_string(),
        }
    }

    fn metadata(&self) -> SourceMetadata {
        SourceMetadata {
            id: self.id().to_string(),
            description: "Wikipedia REST summary for factual/historical claim verification"
                .to_string(),
            domain: SourceDomain::Factual,
        }
    }
}
