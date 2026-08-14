// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Phase 4 post-level application enforcement events (bindings, decisions, denials, manifests).

use serde::Deserialize;

use super::common;
use super::SocialEventRow;
use myso_indexer_alt_social_schema::models::{
    NewPostUsageDecisionEvent, NewRevenueManifestRecord,
};

use super::media_asset::{chain_time, transaction_id_from_event_id};

fn u64_from_json(value: &serde_json::Value) -> Option<u64> {
    value
        .as_u64()
        .or_else(|| value.as_str().and_then(|s| s.parse().ok()))
}

fn binding_json(
    binding_id: i64,
    source_asset_id: &str,
    usage_class: u8,
    stem: u8,
    media_component: u8,
    recorded_at: u64,
) -> serde_json::Value {
    serde_json::json!({
        "binding_id": binding_id,
        "source_asset_id": source_asset_id,
        "usage_class": usage_class,
        "stem": stem,
        "media_component": media_component,
        "recorded_at": recorded_at,
    })
}

fn decision_json(
    binding_id: i64,
    policy_playback_permitted: bool,
    playback_permitted: bool,
    policy_reason_code: u8,
    policy_version: i64,
) -> serde_json::Value {
    serde_json::json!({
        "binding_id": binding_id,
        "policy_playback_permitted": policy_playback_permitted,
        "playback_permitted": playback_permitted,
        "policy_reason_code": policy_reason_code,
        "policy_version_at_decision": policy_version,
    })
}

fn denial_json(binding_id: i64, denial_scope: u8, recorded_at: u64) -> serde_json::Value {
    serde_json::json!({
        "binding_id": binding_id,
        "denial_scope": denial_scope,
        "recorded_at": recorded_at,
    })
}

#[derive(Debug, Deserialize)]
struct EmbeddedBindingJson {
    binding_id: serde_json::Value,
    source_asset_id: String,
    usage_class: u8,
    stem: u8,
    media_component: u8,
}

#[derive(Debug, Deserialize)]
struct EmbeddedBindingRecordedEvent {
    post_id: String,
    #[serde(default)]
    bindings: Vec<EmbeddedBindingJson>,
    #[serde(default)]
    embedded_bindings: Option<Vec<EmbeddedBindingJson>>,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct UsageDecisionRefreshedEvent {
    post_id: String,
    binding_id: serde_json::Value,
    #[serde(default)]
    policy_playback_permitted: Option<bool>,
    playback_permitted: bool,
    policy_reason_code: u8,
    policy_version_at_decision: serde_json::Value,
    #[serde(default)]
    embedded_bindings: Option<Vec<EmbeddedBindingJson>>,
    #[serde(default)]
    usage_decisions: Option<Vec<serde_json::Value>>,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct ContainerUsageDeniedEvent {
    post_id: String,
    binding_id: serde_json::Value,
    denial_scope: u8,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct ContainerUsageDenialLiftedEvent {
    post_id: String,
    binding_id: serde_json::Value,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

#[derive(Debug, Deserialize)]
struct CandidateManifestSubmittedEvent {
    post_id: String,
    manifest_version: serde_json::Value,
    #[serde(default)]
    entries: Option<serde_json::Value>,
    #[serde(default)]
    entries_json: Option<serde_json::Value>,
    #[serde(deserialize_with = "super::poc::deserialize_u64")]
    timestamp: u64,
}

fn bindings_to_json(bindings: &[EmbeddedBindingJson], recorded_at: u64) -> serde_json::Value {
    let items: Vec<_> = bindings
        .iter()
        .filter_map(|b| {
            let binding_id = u64_from_json(&b.binding_id)? as i64;
            Some(binding_json(
                binding_id,
                &b.source_asset_id,
                b.usage_class,
                b.stem,
                b.media_component,
                recorded_at,
            ))
        })
        .collect();
    serde_json::Value::Array(items)
}

pub fn handle_post_enforcement_event(
    event_name: &str,
    data: &serde_json::Value,
    event_id: &str,
) -> Option<Vec<SocialEventRow>> {
    let tx_id = transaction_id_from_event_id(event_id);
    match event_name {
        "EmbeddedBindingRecordedEvent" => {
            let ev: EmbeddedBindingRecordedEvent = common::deserialize_social_event_json(
                "post",
                event_name,
                event_id,
                data,
                "EmbeddedBindingRecordedEvent parse failed",
            )?;
            let bindings = if !ev.bindings.is_empty() {
                &ev.bindings
            } else {
                ev.embedded_bindings.as_deref()?
            };
            Some(vec![SocialEventRow::PostEnforcementUpdate {
                post_id: ev.post_id,
                embedded_bindings: Some(bindings_to_json(bindings, ev.timestamp)),
                usage_decisions: None,
                usage_denials: None,
            }])
        }
        "UsageDecisionRefreshedEvent" => {
            let ev: UsageDecisionRefreshedEvent = common::deserialize_social_event_json(
                "post",
                event_name,
                event_id,
                data,
                "UsageDecisionRefreshedEvent parse failed",
            )?;
            let binding_id = u64_from_json(&ev.binding_id)? as i64;
            let policy_version = u64_from_json(&ev.policy_version_at_decision)? as i64;
            let policy_playback = ev.policy_playback_permitted.unwrap_or(ev.playback_permitted);

            let embedded_bindings = ev
                .embedded_bindings
                .as_ref()
                .map(|b| bindings_to_json(b, ev.timestamp));
            let usage_decisions = if let Some(decisions) = ev.usage_decisions {
                Some(serde_json::Value::Array(decisions))
            } else {
                Some(serde_json::json!([decision_json(
                    binding_id,
                    policy_playback,
                    ev.playback_permitted,
                    ev.policy_reason_code,
                    policy_version,
                )]))
            };

            Some(vec![
                SocialEventRow::PostUsageDecisionEvent(NewPostUsageDecisionEvent {
                    post_id: ev.post_id.clone(),
                    binding_id,
                    playback_permitted: ev.playback_permitted,
                    payout_permitted: true,
                    policy_reason_code: i16::from(ev.policy_reason_code),
                    policy_version,
                    transaction_id: tx_id.clone(),
                    time: chain_time(ev.timestamp),
                }),
                SocialEventRow::PostEnforcementUpdate {
                    post_id: ev.post_id,
                    embedded_bindings,
                    usage_decisions,
                    usage_denials: None,
                },
            ])
        }
        "ContainerUsageDeniedEvent" => {
            let ev: ContainerUsageDeniedEvent = common::deserialize_social_event_json(
                "post",
                event_name,
                event_id,
                data,
                "ContainerUsageDeniedEvent parse failed",
            )?;
            let binding_id = u64_from_json(&ev.binding_id)? as i64;
            Some(vec![SocialEventRow::PostEnforcementUpdate {
                post_id: ev.post_id,
                embedded_bindings: None,
                usage_decisions: None,
                usage_denials: Some(serde_json::json!([denial_json(
                    binding_id,
                    ev.denial_scope,
                    ev.timestamp,
                )])),
            }])
        }
        "ContainerUsageDenialLiftedEvent" => {
            let ev: ContainerUsageDenialLiftedEvent = common::deserialize_social_event_json(
                "post",
                event_name,
                event_id,
                data,
                "ContainerUsageDenialLiftedEvent parse failed",
            )?;
            let binding_id = u64_from_json(&ev.binding_id)? as i64;
            let _recorded_at = ev.timestamp;
            Some(vec![SocialEventRow::PostEnforcementUpdateDenialLift {
                post_id: ev.post_id,
                binding_id,
            }])
        }
        "CandidateManifestSubmittedEvent" => {
            let ev: CandidateManifestSubmittedEvent = common::deserialize_social_event_json(
                "post",
                event_name,
                event_id,
                data,
                "CandidateManifestSubmittedEvent parse failed",
            )?;
            let manifest_version = u64_from_json(&ev.manifest_version)? as i64;
            let entries = ev
                .entries_json
                .or(ev.entries)
                .unwrap_or(serde_json::json!([]));
            Some(vec![SocialEventRow::RevenueManifestRecord(
                NewRevenueManifestRecord {
                    post_id: ev.post_id,
                    manifest_version,
                    entries_json: entries,
                    transaction_id: tx_id,
                    time: chain_time(ev.timestamp),
                },
            )])
        }
        _ => None,
    }
}
