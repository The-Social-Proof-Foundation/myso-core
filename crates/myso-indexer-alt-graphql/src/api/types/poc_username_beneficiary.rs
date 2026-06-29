// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use async_graphql::Context;
use async_graphql::Object;
use myso_indexer_alt_social_reader::PocUsernameBeneficiaryRow;

use crate::api::types::poc::PocBeneficiaryVault;

#[derive(Clone)]
pub struct PocUsernameBeneficiary {
    inner: PocUsernameBeneficiaryRow,
}

impl PocUsernameBeneficiary {
    pub(crate) fn from_row(inner: PocUsernameBeneficiaryRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl PocUsernameBeneficiary {
    async fn beneficiary_id(&self) -> &str {
        &self.inner.beneficiary_id
    }

    async fn username(&self) -> &str {
        &self.inner.username
    }

    /// 1=active, 2=claimed, 3=ended.
    async fn status(&self) -> i16 {
        self.inner.status
    }

    async fn creator_identity_source(&self) -> i16 {
        self.inner.creator_identity_source
    }

    async fn creator_identity_hash(&self) -> &str {
        &self.inner.creator_identity_hash
    }

    /// Identity-derived vault directory lookup key (not a user wallet).
    async fn vault_routing_key(&self) -> &str {
        &self.inner.vault_routing_key
    }

    /// Deprecated alias for [`vault_routing_key`](Self::vault_routing_key).
    async fn beneficiary_address(&self) -> &str {
        &self.inner.vault_routing_key
    }

    async fn vault_id(&self) -> &str {
        &self.inner.vault_id
    }

    async fn required_x_handle(&self) -> &str {
        &self.inner.required_x_handle
    }

    async fn oracle_evidence_hash(&self) -> &str {
        &self.inner.oracle_evidence_hash
    }

    async fn provisioned_at_ms(&self) -> i64 {
        self.inner.provisioned_at_ms
    }

    async fn provisioned_by(&self) -> &str {
        &self.inner.provisioned_by
    }

    async fn claimed_profile_id(&self) -> Option<&str> {
        self.inner.claimed_profile_id.as_deref()
    }

    async fn claimed_by(&self) -> Option<&str> {
        self.inner.claimed_by.as_deref()
    }

    async fn claimed_at_ms(&self) -> Option<i64> {
        self.inner.claimed_at_ms
    }

    async fn ended_at_ms(&self) -> Option<i64> {
        self.inner.ended_at_ms
    }

    async fn ended_by(&self) -> Option<&str> {
        self.inner.ended_by.as_deref()
    }

    async fn end_reason_code(&self) -> Option<i16> {
        self.inner.end_reason_code
    }

    async fn join_referrer(&self) -> Option<&str> {
        self.inner.join_referrer.as_deref()
    }

    async fn join_referral_paid(&self) -> bool {
        self.inner.join_referral_paid
    }

    async fn join_referral_paid_at_ms(&self) -> Option<i64> {
        self.inner.join_referral_paid_at_ms
    }

    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }

    async fn poc_beneficiary_vault(&self, ctx: &Context<'_>) -> Option<PocBeneficiaryVault> {
        let reader_opt = ctx
            .data_opt::<std::sync::Arc<Option<myso_indexer_alt_social_reader::SocialPgReader>>>()?;
        let reader = reader_opt.as_ref().as_ref()?;
        let row = reader
            .get_poc_beneficiary_vault_by_vault_id(&self.inner.vault_id)
            .await
            .ok()??;
        Some(PocBeneficiaryVault::from_row(row))
    }
}
