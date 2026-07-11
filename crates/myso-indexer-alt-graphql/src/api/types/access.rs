// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Enum, SimpleObject};
use myso_indexer_alt_social_reader::{
    MyDataAccessConfigurationKind as DbMyDataAccessConfigurationKind,
    PostAccessKind as DbPostAccessKind, ResolvedPostAccess,
};

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum PostAccessKind {
    Public,
    ProfileSubscription,
    MarketplaceOneTime,
}

impl From<DbPostAccessKind> for PostAccessKind {
    fn from(kind: DbPostAccessKind) -> Self {
        match kind {
            DbPostAccessKind::Public => Self::Public,
            DbPostAccessKind::ProfileSubscription => Self::ProfileSubscription,
            DbPostAccessKind::MarketplaceOneTime => Self::MarketplaceOneTime,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum MyDataAccessConfigurationKind {
    ProfileSubscription,
    MarketplaceOneTime,
    MarketplaceRecurring,
}

impl From<DbMyDataAccessConfigurationKind> for MyDataAccessConfigurationKind {
    fn from(kind: DbMyDataAccessConfigurationKind) -> Self {
        match kind {
            DbMyDataAccessConfigurationKind::ProfileSubscription => Self::ProfileSubscription,
            DbMyDataAccessConfigurationKind::MarketplaceOneTime => Self::MarketplaceOneTime,
            DbMyDataAccessConfigurationKind::MarketplaceRecurring => Self::MarketplaceRecurring,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub(crate) struct PostAccess {
    kind: PostAccessKind,
    subscription_service_id: Option<String>,
    subscription_min_tier_level: Option<i64>,
    mydata_id: Option<String>,
}

impl From<ResolvedPostAccess> for PostAccess {
    fn from(access: ResolvedPostAccess) -> Self {
        Self {
            kind: access.kind.into(),
            subscription_service_id: access.subscription_service_id,
            subscription_min_tier_level: access.subscription_min_tier_level,
            mydata_id: access.mydata_id,
        }
    }
}
