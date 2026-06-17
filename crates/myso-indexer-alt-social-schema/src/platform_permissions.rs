// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

pub const PLATFORM_BLOCK_ADMIN: &str = "PlatformBlockAdmin";
pub const PLATFORM_BADGE_ADMIN: &str = "PlatformBadgeAdmin";
pub const PLATFORM_TREASURY_ADMIN: &str = "PlatformTreasuryAdmin";
pub const PLATFORM_CONTENT_MODERATOR: &str = "PlatformContentModerator";
pub const PLATFORM_PROMOTION_ADMIN: &str = "PlatformPromotionAdmin";

pub const ALL_MODERATOR_EXTENSION_PERMISSIONS: &[&str] = &[
    PLATFORM_BLOCK_ADMIN,
    PLATFORM_BADGE_ADMIN,
    PLATFORM_TREASURY_ADMIN,
    PLATFORM_CONTENT_MODERATOR,
    PLATFORM_PROMOTION_ADMIN,
];

pub fn is_valid_moderator_permission(name: &str) -> bool {
    ALL_MODERATOR_EXTENSION_PERMISSIONS.contains(&name)
}

pub fn normalize_platform_permission(name: &str) -> Option<&'static str> {
    match name {
        PLATFORM_BLOCK_ADMIN => Some(PLATFORM_BLOCK_ADMIN),
        PLATFORM_BADGE_ADMIN => Some(PLATFORM_BADGE_ADMIN),
        PLATFORM_TREASURY_ADMIN => Some(PLATFORM_TREASURY_ADMIN),
        PLATFORM_CONTENT_MODERATOR => Some(PLATFORM_CONTENT_MODERATOR),
        PLATFORM_PROMOTION_ADMIN => Some(PLATFORM_PROMOTION_ADMIN),
        _ => None,
    }
}
