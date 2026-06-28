// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use myso_indexer_alt_social_schema::models::PocUsernameBeneficiaryRow;

use crate::error::SocialError;
use myso_pg_db::Db;

pub(crate) async fn list_poc_username_beneficiaries(
    db: &Db,
    status: Option<i16>,
    limit: i64,
    offset: i64,
) -> Result<Vec<PocUsernameBeneficiaryRow>, SocialError> {
    let mut conn = db.connect().await?;
    myso_indexer_alt_social_reader::list_username_beneficiaries_for_conn(
        &mut conn,
        status,
        limit,
        offset,
    )
    .await
    .map_err(|e| SocialError::internal(e.to_string()))
}

pub(crate) async fn get_poc_username_beneficiary_by_username(
    db: &Db,
    username: &str,
) -> Result<Option<PocUsernameBeneficiaryRow>, SocialError> {
    let mut conn = db.connect().await?;
    myso_indexer_alt_social_reader::get_poc_username_beneficiary_by_username_for_conn(
        &mut conn,
        username,
    )
    .await
    .map_err(|e| SocialError::internal(e.to_string()))
}

pub(crate) async fn get_poc_username_beneficiary_by_id(
    db: &Db,
    beneficiary_id: &str,
) -> Result<Option<PocUsernameBeneficiaryRow>, SocialError> {
    let mut conn = db.connect().await?;
    myso_indexer_alt_social_reader::get_poc_username_beneficiary_by_id_for_conn(
        &mut conn,
        beneficiary_id,
    )
    .await
    .map_err(|e| SocialError::internal(e.to_string()))
}
