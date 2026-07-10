// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod ical_feed;
pub mod stub;
pub mod yaml_seed;

use std::sync::Arc;

use crate::events::EventProvider;

pub fn all_default_providers() -> Vec<Arc<dyn EventProvider>> {
    vec![
        Arc::new(yaml_seed::YamlSeedProvider),
        Arc::new(ical_feed::IcalFeedProvider),
        Arc::new(stub::StubEventProvider::new()),
    ]
}
