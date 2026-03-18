// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;

use crate::api::scalars::myso_address::MySoAddress;

use myso_indexer_alt_social_reader::TipRow;

#[derive(Clone)]
pub(crate) struct TipSummary {
    inner: TipRow,
}

impl TipSummary {
    pub(crate) fn from_row(inner: TipRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl TipSummary {
    async fn tipper(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.tipper)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn recipient(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.recipient)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    async fn amount(&self) -> i64 {
        self.inner.amount
    }

    async fn created_at(&self) -> i64 {
        self.inner.created_at
    }
}
