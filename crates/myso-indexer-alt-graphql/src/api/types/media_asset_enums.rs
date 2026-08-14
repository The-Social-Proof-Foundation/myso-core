// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Enum;

/// Move `media_asset::media_type_*` constants.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum MediaAssetMediaType {
    Unspecified,
    Image,
    Video,
    Audio,
}

impl From<i16> for MediaAssetMediaType {
    fn from(v: i16) -> Self {
        match v {
            1 => Self::Image,
            2 => Self::Video,
            3 => Self::Audio,
            _ => Self::Unspecified,
        }
    }
}

/// Move `media_asset::asset_kind_*` constants.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum MediaAssetKind {
    Unspecified,
    VisualWork,
    MusicalComposition,
    SoundRecording,
}

impl From<i16> for MediaAssetKind {
    fn from(v: i16) -> Self {
        match v {
            1 => Self::VisualWork,
            2 => Self::MusicalComposition,
            3 => Self::SoundRecording,
            _ => Self::Unspecified,
        }
    }
}

/// Move `media_asset::originality_*` constants.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum MediaAssetOriginalityStatus {
    Unresolved,
    Original,
    Derivative,
}

impl From<i16> for MediaAssetOriginalityStatus {
    fn from(v: i16) -> Self {
        match v {
            1 => Self::Original,
            2 => Self::Derivative,
            _ => Self::Unresolved,
        }
    }
}

/// Move `media_asset::provenance_*` constants.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum MediaAssetProvenanceStatus {
    Unverified,
    Verified,
}

impl From<i16> for MediaAssetProvenanceStatus {
    fn from(v: i16) -> Self {
        match v {
            1 => Self::Verified,
            _ => Self::Unverified,
        }
    }
}

/// Move `media_asset::relationship_*` constants.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum DerivativeRelationshipType {
    Unspecified,
    Remix,
    Sample,
    Cover,
    Mashup,
}

impl From<i16> for DerivativeRelationshipType {
    fn from(v: i16) -> Self {
        match v {
            1 => Self::Remix,
            2 => Self::Sample,
            3 => Self::Cover,
            4 => Self::Mashup,
            _ => Self::Unspecified,
        }
    }
}

/// Move `proof_of_creativity` detected relationship status constants.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum DetectedAssetRelationshipStatus {
    Proposed,
    Accepted,
    Rejected,
    Finalized,
}

impl From<i16> for DetectedAssetRelationshipStatus {
    fn from(v: i16) -> Self {
        match v {
            1 => Self::Accepted,
            2 => Self::Rejected,
            3 => Self::Finalized,
            _ => Self::Proposed,
        }
    }
}

/// Move `media_asset::composition_*` constants.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum PostCompositionStatus {
    None,
    Pending,
    Verified,
    Invalid,
    PartiallyRestricted,
}

impl From<i16> for PostCompositionStatus {
    fn from(v: i16) -> Self {
        match v {
            1 => Self::Pending,
            2 => Self::Verified,
            3 => Self::Invalid,
            4 => Self::PartiallyRestricted,
            _ => Self::None,
        }
    }
}

/// Move `media_asset::monetization_*` constants.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum PostMonetizationStatus {
    None,
    Pending,
    Enabled,
    Restricted,
}

impl From<i16> for PostMonetizationStatus {
    fn from(v: i16) -> Self {
        match v {
            1 => Self::Pending,
            2 => Self::Enabled,
            3 => Self::Restricted,
            _ => Self::None,
        }
    }
}

/// Move `governance::STATUS_*` constants.
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum GovernanceProposalStatus {
    Submitted,
    DelegateReview,
    CommunityVoting,
    Approved,
    Rejected,
    Implemented,
    OwnerRescind,
}

impl From<i16> for GovernanceProposalStatus {
    fn from(v: i16) -> Self {
        match v {
            1 => Self::DelegateReview,
            2 => Self::CommunityVoting,
            3 => Self::Approved,
            4 => Self::Rejected,
            5 => Self::Implemented,
            6 => Self::OwnerRescind,
            _ => Self::Submitted,
        }
    }
}
