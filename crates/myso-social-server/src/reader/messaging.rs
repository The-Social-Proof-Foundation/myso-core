// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::WalletMessagingPolicy;
use myso_indexer_alt_social_schema::schema::wallet_messaging_policies;

use crate::error::SocialError;
use crate::reader::types::WalletMessagingPolicyResponse;
use myso_pg_db::Db;

pub(crate) async fn get_wallet_messaging_policy(
    db: &Db,
    address: &str,
) -> Result<Option<WalletMessagingPolicyResponse>, SocialError> {
    let mut conn = db.connect().await?;
    let policy = wallet_messaging_policies::table
        .filter(wallet_messaging_policies::wallet_address.eq(address))
        .select(WalletMessagingPolicy::as_select())
        .first(&mut conn)
        .await
        .optional()?;
    Ok(policy.map(WalletMessagingPolicyResponse::from))
}
