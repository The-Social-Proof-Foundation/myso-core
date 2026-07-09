// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod chainlink;
pub mod coingecko;
pub mod coinbase;
pub mod github_releases;
pub mod http_official;
pub mod rss_event;
pub mod stub;

use std::sync::Arc;

use myso_discovery_service_core::sources::DiscoveryDomain;

use crate::sources::discovery_resolve::DiscoveryResolveCtx;
use crate::sources::TrustedSource;

/// All `TrustedSource` impls compiled into the binary.
pub fn all_default_sources(ctx: DiscoveryResolveCtx) -> Vec<Arc<dyn TrustedSource>> {
    let live: Vec<Arc<dyn TrustedSource>> = vec![
        Arc::new(coingecko::CoingeckoAdapter::new(ctx.clone())),
        Arc::new(coinbase::CoinbaseAdapter::new(ctx.clone())),
        Arc::new(chainlink::ChainlinkAdapter::new(ctx.clone())),
        Arc::new(rss_event::RssEventAdapter::new(ctx.clone())),
        Arc::new(http_official::HttpOfficialAdapter::new(ctx.clone())),
        Arc::new(github_releases::GithubReleasesAdapter::new(ctx.clone())),
    ];
    let scaffolded: [&'static str; 16] = [
        "sec_edgar",
        "fec",
        "fed",
        "bls",
        "noaa",
        "nasa",
        "usgs",
        "congress",
        "supreme_court",
        "app_store",
        "google_play",
        "steam",
        "imdb",
        "billboard",
        "sports_api",
        "election_api",
    ];
    let mut out = live;
    for id in scaffolded {
        out.push(Arc::new(stub::StubTrustedSource::new_scaffolded(
            id,
            DiscoveryDomain::Factual,
        )));
    }
    out
}
