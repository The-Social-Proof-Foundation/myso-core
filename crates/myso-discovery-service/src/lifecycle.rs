// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AssetLifecycleState {
    Discovered,
    Normalized,
    Queued,
    Acquiring,
    Embedded,
    Indexed,
    Matched,
    ProvenanceConfirmed,
    VaultEligible,
    VaultCreated,
    Claimed,
    Failed,
    Excluded,
    Stale,
    Superseded,
}

impl AssetLifecycleState {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discovered => "discovered",
            Self::Normalized => "normalized",
            Self::Queued => "queued",
            Self::Acquiring => "acquiring",
            Self::Embedded => "embedded",
            Self::Indexed => "indexed",
            Self::Matched => "matched",
            Self::ProvenanceConfirmed => "provenance_confirmed",
            Self::VaultEligible => "vault_eligible",
            Self::VaultCreated => "vault_created",
            Self::Claimed => "claimed",
            Self::Failed => "failed",
            Self::Excluded => "excluded",
            Self::Stale => "stale",
            Self::Superseded => "superseded",
        }
    }

    pub fn parse(value: &str) -> Option<Self> {
        match value {
            "discovered" => Some(Self::Discovered),
            "normalized" => Some(Self::Normalized),
            "queued" => Some(Self::Queued),
            "acquiring" => Some(Self::Acquiring),
            "embedded" => Some(Self::Embedded),
            "indexed" => Some(Self::Indexed),
            "matched" => Some(Self::Matched),
            "provenance_confirmed" => Some(Self::ProvenanceConfirmed),
            "vault_eligible" => Some(Self::VaultEligible),
            "vault_created" => Some(Self::VaultCreated),
            "claimed" => Some(Self::Claimed),
            "failed" => Some(Self::Failed),
            "excluded" => Some(Self::Excluded),
            "stale" => Some(Self::Stale),
            "superseded" => Some(Self::Superseded),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleEvent {
    Normalize,
    Enqueue,
    StartAcquire,
    EmbedComplete,
    IndexComplete,
    MatchDetected,
    ProvenanceConfirmed,
    VaultEligible,
    VaultCreated,
    Claimed,
    Fail,
    Exclude,
    MarkStale,
    Supersede,
}

#[derive(Debug, Error)]
pub enum LifecycleError {
    #[error("invalid transition from {from:?} via {event:?}")]
    InvalidTransition {
        from: AssetLifecycleState,
        event: LifecycleEvent,
    },
}

pub fn transition(
    from: AssetLifecycleState,
    event: LifecycleEvent,
) -> Result<AssetLifecycleState, LifecycleError> {
    use AssetLifecycleState as S;
    use LifecycleEvent as E;

    let next = match (from, event) {
        (S::Discovered, E::Normalize) => S::Normalized,
        (S::Normalized, E::Enqueue) => S::Queued,
        (S::Queued, E::StartAcquire) => S::Acquiring,
        (S::Acquiring, E::EmbedComplete) => S::Embedded,
        (S::Embedded, E::IndexComplete) => S::Indexed,
        (S::Indexed, E::MatchDetected) => S::Matched,
        (S::Matched, E::ProvenanceConfirmed) => S::ProvenanceConfirmed,
        (S::ProvenanceConfirmed, E::VaultEligible) => S::VaultEligible,
        (S::VaultEligible, E::VaultCreated) => S::VaultCreated,
        (S::VaultCreated, E::Claimed) => S::Claimed,
        (S::Indexed, E::Supersede) => S::Superseded,
        (_, E::Fail) => S::Failed,
        (_, E::Exclude) => S::Excluded,
        (_, E::MarkStale) => S::Stale,
        _ => {
            return Err(LifecycleError::InvalidTransition { from, event });
        }
    };
    Ok(next)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path_to_indexed() {
        let mut state = AssetLifecycleState::Discovered;
        state = transition(state, LifecycleEvent::Normalize).unwrap();
        state = transition(state, LifecycleEvent::Enqueue).unwrap();
        state = transition(state, LifecycleEvent::StartAcquire).unwrap();
        state = transition(state, LifecycleEvent::EmbedComplete).unwrap();
        state = transition(state, LifecycleEvent::IndexComplete).unwrap();
        assert_eq!(state, AssetLifecycleState::Indexed);
    }
}
