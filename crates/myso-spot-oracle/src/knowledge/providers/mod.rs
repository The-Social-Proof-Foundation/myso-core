// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub mod entity_seed;
pub mod metrics_observation;
pub mod relationship_seed;

use std::sync::Arc;

use crate::knowledge::KnowledgeProvider;

pub fn all_default_providers() -> Vec<Arc<dyn KnowledgeProvider>> {
    vec![
        Arc::new(entity_seed::EntitySeedProvider),
        Arc::new(relationship_seed::RelationshipSeedProvider),
        Arc::new(metrics_observation::MetricsObservationProvider),
    ]
}
