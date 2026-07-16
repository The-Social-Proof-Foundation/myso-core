// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

//! Resolve post and MyData access from indexed kind tags with legacy fallbacks.

use crate::post::PostRow;

pub const POST_ACCESS_KIND_PUBLIC: &str = "public";
pub const POST_ACCESS_KIND_PROFILE_SUBSCRIPTION: &str = "profile_subscription";
pub const POST_ACCESS_KIND_MARKETPLACE_ONE_TIME: &str = "marketplace_one_time";

pub const MYDATA_ACCESS_KIND_PROFILE_SUBSCRIPTION: &str = "profile_subscription";
pub const MYDATA_ACCESS_KIND_MARKETPLACE_ONE_TIME: &str = "marketplace_one_time";
pub const MYDATA_ACCESS_KIND_MARKETPLACE_RECURRING: &str = "marketplace_recurring";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostAccessKind {
    Public,
    ProfileSubscription,
    MarketplaceOneTime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MyDataAccessConfigurationKind {
    ProfileSubscription,
    MarketplaceOneTime,
    MarketplaceRecurring,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedPostAccess {
    pub kind: PostAccessKind,
    pub subscription_service_id: Option<String>,
    pub subscription_min_tier_level: Option<i64>,
    pub mydata_id: Option<String>,
}

fn parse_post_access_kind_tag(tag: &str) -> Option<PostAccessKind> {
    match tag {
        "1" | POST_ACCESS_KIND_PUBLIC => Some(PostAccessKind::Public),
        "2" | POST_ACCESS_KIND_PROFILE_SUBSCRIPTION => Some(PostAccessKind::ProfileSubscription),
        "3" | POST_ACCESS_KIND_MARKETPLACE_ONE_TIME => Some(PostAccessKind::MarketplaceOneTime),
        _ => None,
    }
}

fn parse_mydata_access_configuration_kind_tag(tag: &str) -> Option<MyDataAccessConfigurationKind> {
    match tag {
        "1" | MYDATA_ACCESS_KIND_PROFILE_SUBSCRIPTION => {
            Some(MyDataAccessConfigurationKind::ProfileSubscription)
        }
        "2" | MYDATA_ACCESS_KIND_MARKETPLACE_ONE_TIME => {
            Some(MyDataAccessConfigurationKind::MarketplaceOneTime)
        }
        "3" | MYDATA_ACCESS_KIND_MARKETPLACE_RECURRING => {
            Some(MyDataAccessConfigurationKind::MarketplaceRecurring)
        }
        _ => None,
    }
}

fn build_post_access(
    kind: PostAccessKind,
    subscription_service_id: Option<&str>,
    subscription_min_tier_level: Option<i64>,
    mydata_id: Option<&str>,
) -> ResolvedPostAccess {
    match kind {
        PostAccessKind::Public => ResolvedPostAccess {
            kind,
            subscription_service_id: None,
            subscription_min_tier_level: None,
            mydata_id: None,
        },
        PostAccessKind::ProfileSubscription => ResolvedPostAccess {
            kind,
            subscription_service_id: subscription_service_id.map(str::to_owned),
            subscription_min_tier_level,
            mydata_id: mydata_id.map(str::to_owned),
        },
        PostAccessKind::MarketplaceOneTime => ResolvedPostAccess {
            kind,
            subscription_service_id: None,
            subscription_min_tier_level: None,
            mydata_id: mydata_id.map(str::to_owned),
        },
    }
}

pub fn resolve_post_access(
    post_access_kind: Option<&str>,
    requires_subscription: Option<bool>,
    subscription_service_id: Option<&str>,
    subscription_min_tier_level: Option<i64>,
    mydata_id: Option<&str>,
) -> ResolvedPostAccess {
    if let Some(tag) = post_access_kind {
        if let Some(kind) = parse_post_access_kind_tag(tag) {
            return build_post_access(
                kind,
                subscription_service_id,
                subscription_min_tier_level,
                mydata_id,
            );
        }
    }

    if requires_subscription == Some(true) || subscription_service_id.is_some() {
        return build_post_access(
            PostAccessKind::ProfileSubscription,
            subscription_service_id,
            subscription_min_tier_level,
            mydata_id,
        );
    }

    if mydata_id.is_some() {
        return build_post_access(PostAccessKind::MarketplaceOneTime, None, None, mydata_id);
    }

    build_post_access(PostAccessKind::Public, None, None, None)
}

pub fn resolve_mydata_access_configuration_kind(
    access_configuration_kind: Option<&str>,
    one_time_price: Option<i64>,
    subscription_price: Option<i64>,
) -> MyDataAccessConfigurationKind {
    if let Some(tag) = access_configuration_kind {
        if let Some(kind) = parse_mydata_access_configuration_kind_tag(tag) {
            return kind;
        }
    }

    if one_time_price.is_some() {
        return MyDataAccessConfigurationKind::MarketplaceOneTime;
    }

    if subscription_price.is_some() {
        return MyDataAccessConfigurationKind::MarketplaceRecurring;
    }

    MyDataAccessConfigurationKind::ProfileSubscription
}

impl PostRow {
    pub fn resolve_post_access(&self) -> ResolvedPostAccess {
        resolve_post_access(
            self.post_access_kind.as_deref(),
            self.requires_subscription,
            self.subscription_service_id.as_deref(),
            self.subscription_min_tier_level,
            self.mydata_id.as_deref(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_access_from_indexed_kind() {
        let access = resolve_post_access(
            Some("profile_subscription"),
            None,
            Some("svc-1"),
            Some(2),
            Some("md-1"),
        );
        assert_eq!(access.kind, PostAccessKind::ProfileSubscription);
        assert_eq!(access.subscription_service_id.as_deref(), Some("svc-1"));
        assert_eq!(access.subscription_min_tier_level, Some(2));
        assert_eq!(access.mydata_id.as_deref(), Some("md-1"));
    }

    #[test]
    fn post_access_legacy_subscription_gate() {
        let access = resolve_post_access(None, Some(true), Some("svc-1"), None, None);
        assert_eq!(access.kind, PostAccessKind::ProfileSubscription);
        assert_eq!(access.subscription_service_id.as_deref(), Some("svc-1"));
    }

    #[test]
    fn mydata_access_from_indexed_kind() {
        let kind =
            resolve_mydata_access_configuration_kind(Some("marketplace_recurring"), None, None);
        assert_eq!(kind, MyDataAccessConfigurationKind::MarketplaceRecurring);
    }

    #[test]
    fn mydata_access_legacy_prices() {
        let kind = resolve_mydata_access_configuration_kind(None, Some(100), None);
        assert_eq!(kind, MyDataAccessConfigurationKind::MarketplaceOneTime);

        let kind = resolve_mydata_access_configuration_kind(None, None, Some(50));
        assert_eq!(kind, MyDataAccessConfigurationKind::MarketplaceRecurring);
    }
}
