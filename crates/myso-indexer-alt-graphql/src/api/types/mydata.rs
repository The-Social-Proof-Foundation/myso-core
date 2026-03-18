// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::str::FromStr;

use async_graphql::Object;
use myso_indexer_alt_social_reader::{MyDataPurchaseRow, MyDataRecordRow};

use crate::api::scalars::myso_address::MySoAddress;

fn parse_tags(value: &serde_json::Value) -> Vec<String> {
    value
        .as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default()
}

#[derive(Clone)]
pub(crate) struct MyDataRecord {
    inner: MyDataRecordRow,
}

impl MyDataRecord {
    pub(crate) fn from_row(inner: MyDataRecordRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataRecord {
    /// Unique MyData record identifier.
    async fn mydata_id(&self) -> &str {
        &self.inner.mydata_id
    }

    /// Owner address.
    async fn owner(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.owner)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Media type (e.g. "text", "audio", "image", "video").
    async fn media_type(&self) -> &str {
        &self.inner.media_type
    }

    /// Searchable tags.
    async fn tags(&self) -> Vec<String> {
        parse_tags(&self.inner.tags)
    }

    /// One-time purchase price (null if not for sale).
    async fn one_time_price(&self) -> Option<i64> {
        self.inner.one_time_price
    }

    /// Subscription price (null if not for sale).
    async fn subscription_price(&self) -> Option<i64> {
        self.inner.subscription_price
    }
}

#[derive(Clone)]
pub(crate) struct MyDataPurchase {
    inner: MyDataPurchaseRow,
}

impl MyDataPurchase {
    pub(crate) fn from_row(inner: MyDataPurchaseRow) -> Self {
        Self { inner }
    }
}

#[Object]
impl MyDataPurchase {
    /// MyData record ID this purchase is for.
    async fn mydata_id(&self) -> &str {
        &self.inner.mydata_id
    }

    /// Buyer address.
    async fn buyer(&self) -> MySoAddress {
        MySoAddress::from_str(&self.inner.buyer)
            .unwrap_or_else(|_| MySoAddress::from(myso_types::base_types::MySoAddress::ZERO))
    }

    /// Price paid.
    async fn price(&self) -> i64 {
        self.inner.price
    }

    /// Purchase type ("one_time" or "subscription").
    async fn purchase_type(&self) -> &str {
        &self.inner.purchase_type
    }

    /// Epoch timestamp when the purchase was made.
    async fn purchase_time(&self) -> i64 {
        self.inner.purchase_time
    }

    /// Transaction ID.
    async fn transaction_id(&self) -> &str {
        &self.inner.transaction_id
    }
}
