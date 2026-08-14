// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! **`media_asset` module:** asset resolution, fingerprint linking, and usage graph events.

use serde::Deserialize;

use super::common;
use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewCompositionAnalysisRecord, NewFingerprintObservation, NewMediaAsset, NewMediaAssetUsage,
};

pub(super) fn transaction_id_from_event_id(event_id: &str) -> String {
    event_id.split(':').next().unwrap_or(event_id).to_string()
}

pub(super) fn bytes_from_json(value: &serde_json::Value) -> Option<Vec<u8>> {
    match value {
        serde_json::Value::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for item in items {
                let n = item.as_u64().and_then(|x| u8::try_from(x).ok())?;
                out.push(n);
            }
            Some(out)
        }
        serde_json::Value::String(s) => hex::decode(s.trim_start_matches("0x")).ok(),
        _ => None,
    }
}

pub(super) fn chain_time(timestamp_ms: u64) -> chrono::DateTime<chrono::Utc> {
    common::chain_time_from_ms(timestamp_ms as i64)
}

#[derive(Debug, Deserialize)]
struct MediaAssetResolvedEvent {
    media_asset_id: String,
    #[serde(default)]
    linked_existing: bool,
    content_commitment: serde_json::Value,
    media_type: u8,
    #[serde(default)]
    asset_kind: u8,
    #[serde(default)]
    registered_by: Option<String>,
    #[serde(default)]
    originality_status: u8,
    lineage_parent_id: Option<String>,
    #[serde(default)]
    oracle_address: Option<String>,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct FingerprintLinkedEvent {
    media_asset_id: String,
    fingerprint_commitment: serde_json::Value,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct MediaAssetUsedEvent {
    container_id: String,
    container_type: u8,
    asset_id: String,
    usage_class: u8,
    #[serde(default)]
    position: u8,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct PostCompositionAnalyzedEvent {
    post_id: String,
    composition_status: u8,
    monetization_status: u8,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

pub fn handle_media_asset_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
    tx_sender: Option<&str>,
) -> Option<Vec<SocialEventRow>> {
    let tx_id = transaction_id_from_event_id(event_id);
    match event_name {
        "MediaAssetResolvedEvent" => process_media_asset_resolved_event(data, event_id, &tx_id, tx_sender),
        "FingerprintLinkedEvent" => process_fingerprint_linked_event(data, event_id, &tx_id),
        "MediaAssetUsedEvent" => process_media_asset_used_event(data, event_id, &tx_id),
        "PostCompositionAnalyzedEvent" => {
            process_post_composition_analyzed_event(data, event_id, &tx_id)
        }
        "MediaAssetRightsUpdatedEvent" => {
            super::media_asset_rights::handle_media_asset_rights_updated_event(data, event_id)
        }
        other => {
            if let Some(rows) =
                super::post_enforcement::handle_post_enforcement_event(other, data, event_id)
            {
                return Some(rows);
            }
            if let Some(rows) = super::media_asset_graph::handle_discovery_event(
                other,
                data,
                event_id,
                tx_sender,
            ) {
                return Some(rows);
            }
            super::media_asset_graph::handle_graph_media_asset_event(other, data, event_id)
        }
    }
}

fn process_media_asset_resolved_event(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
    tx_sender: Option<&str>,
) -> Option<Vec<SocialEventRow>> {
    let ev: MediaAssetResolvedEvent = common::deserialize_social_event_json(
        "media_asset",
        "MediaAssetResolvedEvent",
        event_id,
        data,
        "media_asset MediaAssetResolvedEvent JSON did not match MediaAssetResolvedEvent",
    )?;
    if ev.media_asset_id.is_empty() {
        return None;
    }
    if ev.linked_existing {
        // Existing asset was linked; `FingerprintLinkedEvent` indexes the observation.
        return None;
    }
    let content_commitment = bytes_from_json(&ev.content_commitment)?;
    let registered_by = ev
        .registered_by
        .filter(|s| !s.is_empty())
        .or(ev.oracle_address)
        .filter(|s| !s.is_empty())
        .or_else(|| tx_sender.filter(|s| !s.is_empty()).map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string());
    let now = chain_time(ev.timestamp);
    Some(vec![SocialEventRow::MediaAsset(NewMediaAsset {
        media_asset_id: ev.media_asset_id,
        content_commitment,
        media_type: i16::from(ev.media_type),
        asset_kind: i16::from(ev.asset_kind),
        originality_status: i16::from(ev.originality_status),
        provenance_status: 0,
        lineage_parent_id: ev.lineage_parent_id,
        rights_version: 1,
        economics_version: 1,
        registered_by,
        registered_at: ev.timestamp as i64,
        verified_at: Some(ev.timestamp as i64),
        transaction_id: tx_id.to_string(),
        time: now,
    })])
}

fn process_fingerprint_linked_event(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: FingerprintLinkedEvent = common::deserialize_social_event_json(
        "media_asset",
        "FingerprintLinkedEvent",
        event_id,
        data,
        "media_asset FingerprintLinkedEvent JSON did not match FingerprintLinkedEvent",
    )?;
    let fingerprint_commitment = bytes_from_json(&ev.fingerprint_commitment)?;
    if ev.media_asset_id.is_empty() {
        return None;
    }
    Some(vec![SocialEventRow::FingerprintObservation(
        NewFingerprintObservation {
            fingerprint_commitment,
            media_asset_id: ev.media_asset_id,
            linked_at: ev.timestamp as i64,
            transaction_id: tx_id.to_string(),
            time: chain_time(ev.timestamp),
        },
    )])
}

fn process_media_asset_used_event(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: MediaAssetUsedEvent = common::deserialize_social_event_json(
        "media_asset",
        "MediaAssetUsedEvent",
        event_id,
        data,
        "media_asset MediaAssetUsedEvent JSON did not match MediaAssetUsedEvent",
    )?;
    if ev.container_id.is_empty() || ev.asset_id.is_empty() {
        return None;
    }
    Some(vec![SocialEventRow::MediaAssetUsage(NewMediaAssetUsage {
        container_id: ev.container_id,
        container_type: i16::from(ev.container_type),
        asset_id: ev.asset_id,
        usage_class: i16::from(ev.usage_class),
        position: i16::from(ev.position),
        transaction_id: tx_id.to_string(),
        time: chain_time(ev.timestamp),
    })])
}

fn process_post_composition_analyzed_event(
    data: &serde_json::Value,
    event_id: &str,
    tx_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let ev: PostCompositionAnalyzedEvent = common::deserialize_social_event_json(
        "media_asset",
        "PostCompositionAnalyzedEvent",
        event_id,
        data,
        "media_asset PostCompositionAnalyzedEvent JSON did not match PostCompositionAnalyzedEvent",
    )?;
    if ev.post_id.is_empty() {
        return None;
    }
    let timestamp = ev.timestamp as i64;
    Some(vec![
        SocialEventRow::PostCompositionUpdate {
            post_id: ev.post_id.clone(),
            composition_status: i16::from(ev.composition_status),
            monetization_status: i16::from(ev.monetization_status),
            analyzed_at: timestamp,
        },
        SocialEventRow::CompositionAnalysisRecord(NewCompositionAnalysisRecord {
            post_id: ev.post_id,
            analyzed_at: timestamp,
            usage_context: 0,
            composition_status: i16::from(ev.composition_status),
            monetization_status: i16::from(ev.monetization_status),
            analysis_json: serde_json::json!({}),
            transaction_id: tx_id.to_string(),
            time: chain_time(ev.timestamp),
        }),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn media_asset_used_event_indexes_usage_row() {
        let data = serde_json::json!({
            "container_id": "0xpost1",
            "container_type": 1,
            "asset_id": "0xasset1",
            "usage_class": 1,
            "position": 0,
            "timestamp": 1_700_000_000_000_u64,
        });
        let rows = handle_media_asset_event(
            "MediaAssetUsedEvent",
            &data,
            "tx:0",
            None,
        )
        .expect("rows");
        assert_eq!(rows.len(), 1);
        assert!(matches!(
            &rows[0],
            SocialEventRow::MediaAssetUsage(u)
                if u.container_id == "0xpost1" && u.asset_id == "0xasset1"
        ));
    }
}
