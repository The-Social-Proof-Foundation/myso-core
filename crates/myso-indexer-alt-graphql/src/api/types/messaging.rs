// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::{Context, Enum, Object};
use std::str::FromStr;

use myso_indexer_alt_social_reader::{MessagingAgentGroupRow, PaidMessageEscrowRow};

use crate::api::scalars::myso_address::MySoAddress;
use crate::api::resolve_profile::resolve_profile_summary;
use crate::api::types::profile_summary::ProfileSummary;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
#[graphql(rename_items = "SCREAMING_SNAKE_CASE")]
pub(crate) enum PaidMessageEscrowStatusGql {
    Escrowed,
    Claimed,
    Settled,
    Refunded,
}

impl PaidMessageEscrowStatusGql {
    fn from_status(status: &str) -> Self {
        match status {
            "claimed" => Self::Claimed,
            "settled" => Self::Settled,
            "refunded" => Self::Refunded,
            _ => Self::Escrowed,
        }
    }
}

#[derive(Clone)]
pub(crate) struct PaidMessageEscrow {
    inner: PaidMessageEscrowRow,
}

impl PaidMessageEscrow {
    pub(crate) fn from_row(inner: PaidMessageEscrowRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PaidMessageEscrow {
    async fn group_id(&self) -> &str {
        &self.inner.group_id
    }

    async fn seq(&self) -> i64 {
        self.inner.seq
    }

    async fn payer(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.payer)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn payer_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        if self.inner.payer.is_empty() {
            return None;
        }
        resolve_profile_summary(ctx, &self.inner.payer).await
    }

    async fn recipient(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.recipient)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn recipient_profile(&self, ctx: &Context<'_>) -> Option<ProfileSummary> {
        if self.inner.recipient.is_empty() {
            return None;
        }
        resolve_profile_summary(ctx, &self.inner.recipient).await
    }

    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    async fn status(&self) -> PaidMessageEscrowStatusGql {
        PaidMessageEscrowStatusGql::from_status(&self.inner.status)
    }

    async fn platform_fee(&self) -> Option<i64> {
        self.inner.platform_fee
    }

    async fn treasury_fee(&self) -> Option<i64> {
        self.inner.treasury_fee
    }

    async fn net_amount(&self) -> Option<i64> {
        self.inner.net_amount
    }

    async fn created_at_ms(&self) -> i64 {
        self.inner.created_at_ms
    }

    async fn claimed_at_ms(&self) -> Option<i64> {
        self.inner.claimed_at_ms
    }

    async fn refunded_at_ms(&self) -> Option<i64> {
        self.inner.refunded_at_ms
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}

#[derive(Clone)]
pub(crate) struct MessagingAgentGroup {
    inner: MessagingAgentGroupRow,
}

impl MessagingAgentGroup {
    pub(crate) fn from_row(inner: MessagingAgentGroupRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MessagingAgentGroup {
    async fn group_id(&self) -> &str {
        &self.inner.group_id
    }

    async fn creator_actor(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.creator_actor)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn creator_principal(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.creator_principal)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn creator_sub_agent_id(&self) -> Option<&str> {
        self.inner.creator_sub_agent_id.as_deref()
    }

    async fn creator_identity_class(&self) -> i64 {
        self.inner.creator_identity_class
    }

    async fn organization_id(&self) -> Option<&str> {
        self.inner.organization_id.as_deref()
    }

    async fn group_name(&self) -> &str {
        &self.inner.group_name
    }

    async fn group_uuid(&self) -> &str {
        &self.inner.group_uuid
    }

    async fn created_at_ms(&self) -> i64 {
        self.inner.created_at_ms
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}
