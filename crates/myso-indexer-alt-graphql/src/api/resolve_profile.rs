// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use std::sync::Arc;

use async_graphql::Context;
use myso_indexer_alt_social_reader::SocialPgReader;

use crate::api::types::profile_summary::ProfileSummary;

/// Resolve a profile summary by address. Returns None when Social DB is not configured.
pub(crate) async fn resolve_profile_summary(
    ctx: &Context<'_>,
    address: &str,
) -> Option<ProfileSummary> {
    let reader_opt = ctx
        .data_opt::<Arc<Option<SocialPgReader>>>()?;
    let reader = reader_opt.as_ref().as_ref()?;
    let row = reader.get_profile_summary(address).await.ok()?;
    Some(ProfileSummary::from_row(row))
}
