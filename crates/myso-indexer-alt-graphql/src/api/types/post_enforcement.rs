// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Object;
use serde_json::Value as JsonValue;

const MEDIA_COMPONENT_AUDIO: i64 = 3;
const MEDIA_COMPONENT_VIDEO: i64 = 2;

#[derive(Clone, Default)]
pub(crate) struct PlaybackPolicy {
    audio_muted: bool,
    video_restricted: bool,
}

#[Object]
impl PlaybackPolicy {
    async fn audio_muted(&self) -> bool {
        self.audio_muted
    }

    async fn video_restricted(&self) -> bool {
        self.video_restricted
    }
}

#[derive(Clone)]
pub(crate) struct EmbeddedAssetBinding {
    binding_id: i64,
    source_asset_id: String,
    usage_class: i16,
    stem: i16,
    media_component: i16,
}

#[Object]
impl EmbeddedAssetBinding {
    async fn binding_id(&self) -> i64 {
        self.binding_id
    }

    async fn source_asset_id(&self) -> &str {
        &self.source_asset_id
    }

    async fn usage_class(&self) -> i16 {
        self.usage_class
    }

    async fn stem(&self) -> i16 {
        self.stem
    }

    async fn media_component(&self) -> i16 {
        self.media_component
    }
}

#[derive(Clone)]
pub(crate) struct UsageDecisionSnapshot {
    binding_id: i64,
    policy_playback_permitted: bool,
    playback_permitted: bool,
    policy_reason_code: i16,
    policy_version_at_decision: i64,
}

#[Object]
impl UsageDecisionSnapshot {
    async fn binding_id(&self) -> i64 {
        self.binding_id
    }

    async fn policy_playback_permitted(&self) -> bool {
        self.policy_playback_permitted
    }

    async fn playback_permitted(&self) -> bool {
        self.playback_permitted
    }

    async fn policy_reason_code(&self) -> i16 {
        self.policy_reason_code
    }

    async fn policy_version_at_decision(&self) -> i64 {
        self.policy_version_at_decision
    }
}

#[derive(Clone)]
pub(crate) struct ContainerUsageDenial {
    binding_id: i64,
    denial_scope: i16,
}

#[Object]
impl ContainerUsageDenial {
    async fn binding_id(&self) -> i64 {
        self.binding_id
    }

    async fn denial_scope(&self) -> i16 {
        self.denial_scope
    }
}

fn json_u64(value: &JsonValue, key: &str) -> Option<i64> {
    value
        .get(key)
        .and_then(|v| {
            v.as_u64()
                .or_else(|| v.as_str().and_then(|s| s.parse().ok()))
        })
        .map(|v| v as i64)
}

fn json_u16(value: &JsonValue, key: &str) -> i16 {
    json_u64(value, key).unwrap_or(0) as i16
}

fn json_bool(value: &JsonValue, key: &str, default: bool) -> bool {
    value.get(key).and_then(|v| v.as_bool()).unwrap_or(default)
}

pub(crate) fn parse_embedded_bindings(value: &JsonValue) -> Vec<EmbeddedAssetBinding> {
    let Some(arr) = value.as_array() else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|item| {
            Some(EmbeddedAssetBinding {
                binding_id: json_u64(item, "binding_id")?,
                source_asset_id: item.get("source_asset_id")?.as_str()?.to_string(),
                usage_class: json_u16(item, "usage_class"),
                stem: json_u16(item, "stem"),
                media_component: json_u16(item, "media_component"),
            })
        })
        .collect()
}

pub(crate) fn parse_usage_decisions(value: &JsonValue) -> Vec<UsageDecisionSnapshot> {
    let Some(arr) = value.as_array() else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|item| {
            let binding_id = json_u64(item, "binding_id")?;
            let playback = json_bool(item, "playback_permitted", false);
            Some(UsageDecisionSnapshot {
                binding_id,
                policy_playback_permitted: json_bool(item, "policy_playback_permitted", playback),
                playback_permitted: playback,
                policy_reason_code: json_u16(item, "policy_reason_code"),
                policy_version_at_decision: json_u64(item, "policy_version_at_decision")
                    .unwrap_or(0),
            })
        })
        .collect()
}

pub(crate) fn parse_usage_denials(value: &JsonValue) -> Vec<ContainerUsageDenial> {
    let Some(arr) = value.as_array() else {
        return Vec::new();
    };
    arr.iter()
        .filter_map(|item| {
            Some(ContainerUsageDenial {
                binding_id: json_u64(item, "binding_id")?,
                denial_scope: json_u16(item, "denial_scope"),
            })
        })
        .collect()
}

pub(crate) fn derive_playback_policy(
    bindings: Option<&JsonValue>,
    decisions: Option<&JsonValue>,
) -> PlaybackPolicy {
    let bindings_arr = bindings.and_then(|v| v.as_array());
    let decisions_arr = decisions.and_then(|v| v.as_array());
    let Some(bindings_arr) = bindings_arr else {
        return PlaybackPolicy::default();
    };

    let mut policy = PlaybackPolicy::default();
    for binding in bindings_arr {
        let Some(binding_id) = json_u64(binding, "binding_id") else {
            continue;
        };
        let media_component = json_u64(binding, "media_component").unwrap_or(0);
        let decision = decisions_arr.and_then(|arr| {
            arr.iter()
                .find(|d| json_u64(d, "binding_id") == Some(binding_id))
        });
        let playback_permitted = match decision {
            Some(d) => json_bool(d, "playback_permitted", false),
            None => false,
        };
        if !playback_permitted && media_component == MEDIA_COMPONENT_AUDIO {
            policy.audio_muted = true;
        }
        if !playback_permitted && media_component == MEDIA_COMPONENT_VIDEO {
            policy.video_restricted = true;
        }
    }
    policy
}
