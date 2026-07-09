// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use crate::sources::{DiscoveryAssetRecord, RawDiscoveryRecord};

pub fn normalize_record(raw: &RawDiscoveryRecord) -> DiscoveryAssetRecord {
    let creator_confidence = if raw.creator_x_handle.is_some() {
        (raw.trust_score * 0.9).min(1.0)
    } else {
        0.0
    };

    DiscoveryAssetRecord {
        external_source_url: raw.external_source_url.clone(),
        media_type: raw.media_type.clone(),
        content_kind: raw.content_kind,
        canonical_metadata: serde_json::json!({
            "title": raw.title,
            "creator_x_handle": raw.creator_x_handle,
            "source_metadata": raw.metadata,
        }),
        source_trust_score: raw.trust_score,
        creator_confidence,
        creator_x_handle: raw.creator_x_handle.clone(),
        content_hash: raw.content_hash.clone(),
    }
}
