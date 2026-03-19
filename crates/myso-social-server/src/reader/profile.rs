// Copyright (c) The Social Proof Foundation, LLC.
// SPDX-License-Identifier: Apache-2.0

use diesel::ExpressionMethods;
use diesel::OptionalExtension;
use diesel::QueryDsl;
use diesel::SelectableHelper;
use diesel_async::RunQueryDsl;
use myso_indexer_alt_social_schema::models::Profile;
use myso_indexer_alt_social_schema::schema::{
    profile_offers, profile_sale_fees, profiles, wallet_social_graph,
};

use crate::error::SocialError;
use crate::reader::social_graph::enrich_users_with_universal_data;
use crate::reader::types::{ProfileByAddressResponse, UniversalUserResult};
use crate::reader::WalletOnlyProfile;
use myso_indexer_alt_social_schema::models::{ProfileOffer, ProfileSaleFee};
use myso_pg_db::Db;

pub(crate) async fn get_profiles(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<Profile>, SocialError> {
    let mut conn = db.connect().await?;
    let results = profiles::table
        .order_by(profiles::id.desc())
        .limit(limit)
        .offset(offset)
        .select(Profile::as_select())
        .load::<Profile>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn get_profiles_enriched(
    db: &Db,
    limit: i64,
    offset: i64,
) -> Result<Vec<UniversalUserResult>, SocialError> {
    let profiles = get_profiles(db, limit, offset).await?;
    if profiles.is_empty() {
        return Ok(vec![]);
    }
    let wallet_addresses: Vec<String> = profiles.iter().map(|p| p.owner_address.clone()).collect();
    let mut conn = db.connect().await?;
    let enriched = enrich_users_with_universal_data(&mut conn, wallet_addresses).await?;
    let result: Vec<UniversalUserResult> = profiles
        .iter()
        .filter_map(|p| enriched.get(&p.owner_address).cloned())
        .collect();
    Ok(result)
}

pub(crate) async fn get_profile_count(db: &Db) -> Result<i64, SocialError> {
    let mut conn = db.connect().await?;
    let count: i64 = profiles::table.count().get_result(&mut conn).await?;
    Ok(count)
}

pub(crate) async fn get_profile_by_address(
    db: &Db,
    address: &str,
) -> Result<Option<Profile>, SocialError> {
    let mut conn = db.connect().await?;
    let result = profiles::table
        .filter(profiles::owner_address.eq(address))
        .select(Profile::as_select())
        .first::<Profile>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

/// Get profile by address, or fall back to wallet_social_graph for wallet-only addresses.
/// Returns unified ProfileByAddressResponse with full profile fields when found in profiles table;
/// otherwise wallet-only response with counts from wallet_social_graph (or zero counts if not in WSG).
pub(crate) async fn get_profile_or_wallet_by_address(
    db: &Db,
    address: &str,
) -> Result<ProfileByAddressResponse, SocialError> {
    let mut conn = db.connect().await?;
    let profile_result = profiles::table
        .filter(profiles::owner_address.eq(address))
        .select(Profile::as_select())
        .first::<Profile>(&mut conn)
        .await;

    match profile_result {
        Ok(profile) => {
            let enriched =
                enrich_users_with_universal_data(&mut conn, vec![address.to_string()]).await?;
            let mut response = ProfileByAddressResponse::from(profile);
            if let Some(e) = enriched.get(address) {
                response = response.with_enrichment(e);
            }
            Ok(response)
        }
        Err(diesel::result::Error::NotFound) => {
            let wallet_result = wallet_social_graph::table
                .filter(wallet_social_graph::wallet_address.eq(address))
                .select((
                    wallet_social_graph::followers_count,
                    wallet_social_graph::following_count,
                    wallet_social_graph::blocked_count,
                    wallet_social_graph::created_at,
                    wallet_social_graph::updated_at,
                ))
                .first::<(i32, i32, i32, chrono::NaiveDateTime, chrono::NaiveDateTime)>(&mut conn)
                .await;

            let wallet_only = match wallet_result {
                Ok((fc, fg, bc, created_at, updated_at)) => WalletOnlyProfile::new(
                    address.to_string(),
                    fc,
                    fg,
                    bc,
                    Some(created_at),
                    Some(updated_at),
                ),
                Err(_) => WalletOnlyProfile::new(address.to_string(), 0, 0, 0, None, None),
            };
            Ok(ProfileByAddressResponse::from(wallet_only))
        }
        Err(e) => Err(e.into()),
    }
}

pub(crate) async fn get_profile_by_username(
    db: &Db,
    username: &str,
) -> Result<Option<Profile>, SocialError> {
    let mut conn = db.connect().await?;
    let result = profiles::table
        .filter(profiles::username.eq(username))
        .select(Profile::as_select())
        .first::<Profile>(&mut conn)
        .await
        .optional()?;
    Ok(result)
}

async fn resolve_profile_id(db: &Db, address: &str) -> Result<String, SocialError> {
    let mut conn = db.connect().await?;
    let profile_id: Option<String> = profiles::table
        .filter(profiles::owner_address.eq(address))
        .select(profiles::profile_id)
        .first(&mut conn)
        .await
        .optional()?
        .flatten();
    Ok(profile_id.unwrap_or_else(|| address.to_string()))
}

pub(crate) async fn list_profile_offers(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileOffer>, SocialError> {
    let profile_id = resolve_profile_id(db, address).await?;
    let mut conn = db.connect().await?;
    let results = profile_offers::table
        .filter(profile_offers::profile_id.eq(&profile_id))
        .order_by(profile_offers::created_at.desc())
        .limit(limit)
        .offset(offset)
        .select(ProfileOffer::as_select())
        .load::<ProfileOffer>(&mut conn)
        .await?;
    Ok(results)
}

pub(crate) async fn list_profile_sale_fees(
    db: &Db,
    address: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<ProfileSaleFee>, SocialError> {
    let profile_id = resolve_profile_id(db, address).await?;
    let mut conn = db.connect().await?;
    let results = profile_sale_fees::table
        .filter(profile_sale_fees::profile_id.eq(&profile_id))
        .order_by(profile_sale_fees::timestamp.desc())
        .limit(limit)
        .offset(offset)
        .select(ProfileSaleFee::as_select())
        .load::<ProfileSaleFee>(&mut conn)
        .await?;
    Ok(results)
}
